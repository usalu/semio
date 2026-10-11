//! 💡️ `wfc3d` inferences — THE SOLVE ITSELF IS AN INFERENCE. `Wfc3dSnapshot` only ever persists the
//! PROBLEM (slots/edges/tiles/rules/seed); the SOLUTION, the contradiction verdict and the
//! pre-propagation entropy map are all derived here via `store::InferredField`, never
//! mutation-authored state. The engine crate `semio_s_plugin_wfc_engine` supplies the internals of
//! these `compute()` bodies.
//!
//! Determinism: `solve_with_job` reads only `snapshot` fields (`seed` included) and drives the same
//! resumable `WfcJob<GraphTopology>` interactive callers use; every step is watchdog-wrapped and
//! explicitly bounded. No ambient randomness enters the inference, so `DepHash` caching over
//! `Wfc3dSolve`/`Wfc3dContradiction`/`Wfc3dEntropy` is sound.
//!
//! Rule semantics: the rules ARE the compatibility table. They compile onto an EMPTY `ModelBuilder`,
//! so a tile pair no rule mentions is FORBIDDEN — an allow-list, the same law every other `wfc`
//! artifact states. `allowed: true` pushes `allow`, `allowed: false` pushes `deny`, and deny always
//! wins over allow at compile time regardless of call order. `relation: None` states the rule for
//! every relation at once. A rule is written for an UNORDERED pair, so both directed orders are
//! pushed. An empty rule set is therefore unsatisfiable the moment two slots are adjacent.

use crate::schema::snapshot::Wfc3dSnapshot;
use semio_s_plugin_wfc_engine as engine;
use std::collections::BTreeMap;

//#region 📦️RetainedPayload



//#endregion 📦️RetainedPayload

//#region 🔖️Compile











/// 🧮️ The compatibility table is `relations × tiles²` bits, so the tile and relation universes are
/// the two admissions that actually bound compile work; slots/edges/rules only bound linear passes.
pub(crate) const MAX_WFC3D_TILES: usize = 256;
pub(crate) const MAX_WFC3D_RELATIONS: usize = 32;
pub(crate) const MAX_WFC3D_SLOTS: usize = 65_536;
pub(crate) const MAX_WFC3D_EDGES: usize = 262_144;
pub(crate) const MAX_WFC3D_RULES: usize = 262_144;
pub(crate) const MAX_WFC3D_ID_BYTES: usize = 1_024;
pub(crate) const MAX_WFC3D_OUTPUT_BYTES: usize = 1 << 20;
/// ⛽️ The headless drive's budget, shared with the sibling `grid3d`. Assembly's inherited
/// `fuel_per_step: 1` buys ONE work unit per session round trip, so a solve spends its wall time in
/// session bookkeeping rather than in the solver; the fuel is raised so one step does as much work as
/// its wall budget admits.
///
/// 📄️ Raising the fuel is only sound because every stage that admits a payload page ENDS its step —
/// see `encode_one`. A `StepContext` grants ONE page per step, and a stage that loops over pages
/// inside a multi-unit step is refused, then cannot even admit its own fault detail, so the job dies
/// as a `StepOutcome::Fault` carrying an empty page.
///
/// ⏱️ The wall budget is deliberately far above `semio_framework_trace`'s shared
/// `INTERACTIVE_STEP_CEILING_US` (8 ms), against which every step is measured whatever budget its
/// config asked for; four consecutive over-ceiling steps quarantine the session. That is safe here
/// because a step is bounded by the preview cadence and by the one-page-per-step encode, not by this
/// deadline — the deadline only stops a solve that would otherwise spin.
pub(crate) const HEADLESS_FUEL_PER_STEP: u64 = 1 << 16;
pub(crate) const HEADLESS_STEP_BUDGET_US: u64 = 250_000;
pub(crate) const PARENT_PREVIEW_UNIT_INTERVAL: u64 = 16;
pub(crate) const PARENT_PREVIEW_TIME_INTERVAL_MS: u64 = 16;





#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Wfc3dInferenceCommit {
    pub assignments: BTreeMap<String, String>,
}

























//#endregion 🔖️Compile

//#region 🔖️Solve
/// 🏁 The solved assignment (slot id → tile id), or `Unsolved` for every non-`Solved` outcome
/// (unsatisfiable/contradiction/budget/cancellation) — see `Wfc3dContradiction` for the dedicated
/// satisfiability verdict.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum Wfc3dSolveResult {
    #[default]
    Unsolved,
    Solved {
        assignments: BTreeMap<String, String>,
    },
}

pub struct Wfc3dSolve;



/// 🏁 The solve as one call, for the editor/viewer preview lane: the inferred assignment is rendered,
/// never persisted.
pub fn solve_assignments(snapshot: &Wfc3dSnapshot) -> BTreeMap<String, String> {
    crate::host::inferences::solve_with_job(snapshot).map(|commit| commit.assignments).unwrap_or_default()
}
//#endregion 🔖️Solve

//#region 🔖️Contradiction
/// 🩺 The satisfiability verdict on its own, so a caller who only needs "is this spec even solvable"
/// never has to decode a full assignment map to find out.
pub struct Wfc3dContradiction;


//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲 Per-slot Shannon entropy of the tile WEIGHT distribution — `0.0` for a pinned slot (fully
/// determined), else the prior entropy over every tile's weight. SCOPE, honestly stated: this is the
/// PRIOR entropy before arc-consistency propagation narrows any slot's domain — a real WFC heuristic
/// (the same weighted-distribution math `engine::weights::WeightTable` encodes), but not the
/// POST-propagation entropy a live "which slot should I collapse next" UI would want.
pub struct Wfc3dEntropy;



pub fn shannon_entropy_over_tiles(snapshot: &Wfc3dSnapshot) -> f64 {
    let weights: Vec<f64> = snapshot.tiles.iter().map(|tile| tile.weight).collect();
    let total: f64 = weights.iter().sum();
    if weights.is_empty() || total <= 0.0 {
        return 0.0;
    }
    -weights.iter().map(|w| w / total).filter(|p| *p > 0.0).map(|p| p * p.ln()).sum::<f64>()
}
//#endregion 🔖️Entropy

//#region 🧪️Tests

//#endregion 🧪️Tests
