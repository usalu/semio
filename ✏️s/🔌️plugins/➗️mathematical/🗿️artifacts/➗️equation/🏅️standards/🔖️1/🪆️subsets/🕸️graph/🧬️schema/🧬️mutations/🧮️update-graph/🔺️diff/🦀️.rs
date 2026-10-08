//! 🔺️ `update-graph-algorithm` — sparse diff construction: the algorithm and the seed slot that differ.
use crate::diff::EquationOptionalSeed;
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateGraphAlgorithm, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if base.graph.algorithm == payload.new_algorithm && base.graph.algorithm_seed == payload.new_algorithm_seed {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Graph algorithm is already \"{}\".", payload.new_algorithm));
    }
    let diff = EquationDiff {
        algorithm: (base.graph.algorithm != payload.new_algorithm).then(|| payload.new_algorithm.clone()),
        algorithm_seed: (base.graph.algorithm_seed != payload.new_algorithm_seed).then(|| EquationOptionalSeed { value: payload.new_algorithm_seed.clone() }),
        ..Default::default()
    };
    protocol::MutationOutcome::new(diff)
}
//#endregion 🔖️Diff
