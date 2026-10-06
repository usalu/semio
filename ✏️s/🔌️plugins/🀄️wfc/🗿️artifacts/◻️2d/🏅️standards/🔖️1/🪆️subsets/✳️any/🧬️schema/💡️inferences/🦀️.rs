//! 💡️ WFC 2D inferences — THE SOLVE ITSELF IS AN INFERENCE. `Wfc2dSnapshot` only ever persists the
//! PROBLEM (slots/edges/tiles/rules/seed); the SOLUTION, the contradiction verdict and the entropy
//! map are all derived here, never mutation-authored state.
//!
//! The compile route is assembly's graph route generalised to NAMED RELATIONS: every distinct
//! `Wfc2dSlotEdge::relation` string becomes its own `RelationId` in a `ModelBuilder`, a rule with
//! `relation: None` applies to all of them, and `allowed: false` compiles to a `deny` (which the
//! model resolves after every `allow`, so a deny always wins). The topology is the engine's
//! production `GraphTopologyBuild`; the solver is the resumable `WfcJob<GraphTopology>` interactive
//! callers drive, stepped here under an explicit fuel/step budget so every step is watchdog-bounded.
//!
//! Determinism: `solve_with_job` reads only snapshot fields (`seed` included) and no ambient
//! randomness enters, so `DepHash` caching over `Wfc2dSolve`/`Wfc2dContradiction`/`Wfc2dEntropy` is
//! sound.

use crate::schema::snapshot::Wfc2dSnapshot;
use std::collections::{BTreeMap, BTreeSet};

//#region 📦️RetainedPayload


//#endregion 📦️RetainedPayload

//#region 🔖️Compile











pub(crate) const MAX_WFC_2D_TILES: usize = 65_536;
pub(crate) const MAX_WFC_2D_SLOTS: usize = 65_536;
pub(crate) const MAX_WFC_2D_RULES: usize = 262_144;
pub(crate) const MAX_WFC_2D_EDGES: usize = 262_144;
pub(crate) const MAX_WFC_2D_ID_BYTES: usize = 1_024;
pub(crate) const MAX_WFC_2D_OUTPUT_BYTES: usize = 1 << 20;
pub(crate) const PARENT_PREVIEW_UNIT_INTERVAL: u64 = 16;
pub(crate) const PARENT_PREVIEW_TIME_INTERVAL_MS: u64 = 16;





/// 🏁 What `s.wfc.wfc2d.solve` commits: the assignment, the satisfiability verdict, and the
/// pre-propagation entropy map. None of it is ever written back into the document.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dInferenceCommit {
    pub assignments: BTreeMap<String, String>,
    pub contradiction: bool,
    pub entropy: BTreeMap<String, f64>,
}







/// 🎲 One slot's PRE-PROPAGATION Shannon entropy over the tile weight distribution — `0.0` for a
/// pinned slot (fully determined). Honest scope: this is the prior, before arc consistency narrows
/// any domain; wiring the post-propagation domain through would need the solver's live state and is
/// a real remaining increment.
pub(crate) fn slot_entropy(snapshot: &Wfc2dSnapshot, slot: &crate::schema::snapshot::Wfc2dSlot) -> f64 {
    if slot.pinned_tile_id.is_some() {
        return 0.0;
    }
    let weights: Vec<f64> = snapshot.tiles.iter().map(|tile| if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 }).collect();
    let total: f64 = weights.iter().sum();
    if weights.is_empty() || total <= 0.0 {
        return 0.0;
    }
    -weights.iter().map(|weight| weight / total).filter(|share| *share > 0.0).map(|share| share * share.ln()).sum::<f64>()
}
















//#endregion 🔖️Compile

//#region 🔖️Solve
/// 🏁 The solved assignment (slot id → tile id), or `Unsolved` for every non-solved outcome.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Wfc2dSolveResult {
    #[default]
    Unsolved,
    Solved {
        assignments: BTreeMap<String, String>,
    },
}

pub struct Wfc2dSolve;


//#endregion 🔖️Solve

//#region 🔖️Contradiction
/// 🩺 The satisfiability verdict on its own, so a caller who only needs "is this spec solvable at
/// all" never has to decode a whole assignment map to find out.
pub struct Wfc2dContradiction;


//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲 Per-slot Shannon entropy of the tile WEIGHT distribution — `0.0` for a pinned slot.
pub struct Wfc2dEntropy;


//#endregion 🔖️Entropy

//#region 🔖️Relations
/// 🔗 The relation universe a solve would compile, ascending — the same list `Relations` walks, made
/// public so the editor's graph window can label an edge's class without rebuilding the model.
pub fn solve_relations(snapshot: &Wfc2dSnapshot) -> Vec<String> {
    let mut names: BTreeSet<String> = snapshot.edges.iter().map(|edge| edge.relation.clone()).collect();
    if names.is_empty() {
        names.insert(crate::schema::snapshot::WFC_2D_DEFAULT_RELATION.to_string());
    }
    names.into_iter().collect()
}
//#endregion 🔖️Relations

//#region 🧪️Tests

//#endregion 🧪️Tests
