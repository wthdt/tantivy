use Score;
use query::Explanation;
use query::MultiTermAccumulator;

/// Similarity based scoring.
pub trait Similarity: MultiTermAccumulator {
    fn score(&self, ) -> Score;
    fn explain(&self, vals: &[(usize, u32, u32)]) -> Explanation;
}