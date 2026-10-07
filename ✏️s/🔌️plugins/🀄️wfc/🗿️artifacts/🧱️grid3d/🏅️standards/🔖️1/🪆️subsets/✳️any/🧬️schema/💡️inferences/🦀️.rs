//! 💡️ `s.wfc.grid3d` inferences — THE SOLVE ITSELF IS AN INFERENCE. `Grid3dSnapshot` only ever
//! persists the PROBLEM (extent, cell sizes, periodicity, tiles, rules, pins, masks); the
//! ASSIGNMENT, the contradiction verdict and the pre-propagation entropy map are all derived here,
//! never mutation-authored state.
//!
//! 🧱️ The engine wiring: `TiledModelBuilder` + `declare_stencil_relations_3d_tiled(Stencil3d::Face6)`
//! compile the closed adjacency allow-list into a `CompiledModel`, a `Grid3dTopology` carries the
//! mask and the per-axis boundaries, and the SAME resumable `WfcJob` interactive callers drive
//! solves it. Determinism: every input is read off the snapshot (seed included), every step is
//! fuel-bounded and watchdog-wrapped, no ambient randomness enters — so `DepHash` caching over
//! `Grid3dSolve`/`Grid3dContradiction`/`Grid3dEntropy` is sound.

use crate::schema::snapshot::{cell_key, Grid3dSnapshot};
use semio_s_plugin_wfc_engine as engine;
use std::collections::BTreeMap;

//#region 📦️RetainedPayload

//#endregion 📦️RetainedPayload

//#region 🔖️Compile











pub(crate) const MAX_GRID3D_TILES: usize = 4_096;
pub(crate) const MAX_GRID3D_RULES: usize = 262_144;
pub(crate) const MAX_GRID3D_CELLS: usize = 262_144;
pub(crate) const MAX_GRID3D_ID_BYTES: usize = 1_024;
pub(crate) const MAX_GRID3D_OUTPUT_BYTES: usize = 1 << 20;
/// ⛽️ The headless adapter is a BLOCKING call (a render or an `InferredField::compute`), not an
/// interactive turn, so it buys whole solve phases per session step instead of single units. At one
/// unit per step a 48-cell grid spends minutes in session bookkeeping rather than in the solver —
/// measured, not assumed (ticket 26/09/18/EXTRACT-WFC-PLUGIN, A4). The encode stage still yields per
/// page, because page credit is per STEP and not per fuel unit.
pub(crate) const HEADLESS_FUEL_PER_STEP: u64 = 1 << 16;
pub(crate) const HEADLESS_STEP_BUDGET_US: u64 = 250_000;

pub(crate) const PARENT_PREVIEW_UNIT_INTERVAL: u64 = 16;
pub(crate) const PARENT_PREVIEW_TIME_INTERVAL_MS: u64 = 16;





/// 🏁️ One solved cell. Masked cells never appear, so the row count IS the number of cells the grid
/// had to fill.
#[derive(Clone, Debug, Default, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Grid3dAssignment {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub tile_id: String,
}

/// 🏁️ The solve's commit payload — the authoritative assignment list plus the satisfiability
/// verdict. A grid with no consistent assignment is an ANSWER (`satisfiable: false`, no rows), never
/// an inference failure: the engine publishes `wfc-unsatisfiable` as a job FAULT
/// (`⚙️engine/💼️job/🦀️.rs`, `PublicationKind::Fault`), and this job intercepts exactly that detail and
/// turns it back into a verdict.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Grid3dInferenceCommit {
    pub satisfiable: bool,
    pub assignments: Vec<Grid3dAssignment>,
}

impl Default for Grid3dInferenceCommit {
    fn default() -> Self {
        Self { satisfiable: true, assignments: Vec::new() }
    }
}

/// 🩺️ The exact engine fault detail that means "this grid has no consistent assignment" rather than
/// "the solve broke".
pub(crate) const WFC_UNSATISFIABLE: &[u8] = b"wfc-unsatisfiable";





/// 🧊️ The cell count this snapshot declares, saturating so an absurd extent is refused by the
/// admission check rather than overflowing.
pub(crate) fn cell_count(snapshot: &Grid3dSnapshot) -> usize {
    (snapshot.width as usize).saturating_mul(snapshot.height as usize).saturating_mul(snapshot.depth as usize)
}

/// 🧭️ The per-axis boundary a periodicity flag selects.
pub(crate) fn boundary(periodic: bool) -> engine::grid2d::Boundary {
    if periodic {
        engine::grid2d::Boundary::Wrap
    } else {
        engine::grid2d::Boundary::Open
    }
}

/// 🚪️ Runs one owned job through the engine's close ladder and drops it. A `WfcJob` (or a
/// `WfcRestore`) holds retained payload pages, and an ordinary `Drop` on one of those ABORTS the
/// process ("RetainedJobPayload requires one-page close to terminal-empty") — so no child of this
/// parent is ever released any other way.
pub(crate) fn retire(payload: &mut semio_framework_job::RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

/// 🧾️ One retained payload's bytes, concatenated across its pages.
pub(crate) fn payload_bytes(payload: &semio_framework_job::RetainedJobPayload) -> Vec<u8> {
    (0..payload.page_count()).flat_map(|index| payload.page(index).map(<[u8]>::to_vec).unwrap_or_default()).collect()
}





















//#endregion 🔖️Compile

//#region 🔖️Solve
/// 🏁️ The solved assignment, or `Unsolved` for every non-solved outcome (contradiction, budget,
/// cancellation) — see `Grid3dContradiction` for the dedicated satisfiability verdict.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum Grid3dSolveResult {
    #[default]
    Unsolved,
    Solved {
        assignments: Vec<Grid3dAssignment>,
    },
}

pub struct Grid3dSolve;


//#endregion 🔖️Solve

//#region 🔖️Contradiction
/// 🩺️ The satisfiability verdict on its own, so a caller who only needs "is this grid solvable at
/// all" never has to decode a full assignment list to find out.
pub struct Grid3dContradiction;


//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲️ Per-cell Shannon entropy of the tile WEIGHT distribution — `0.0` for a pinned or masked cell
/// (fully determined, or not in the topology at all), else the prior entropy over every tile's
/// weight. SCOPE, honestly stated: this is the PRIOR entropy BEFORE arc consistency narrows any
/// cell's domain — a real WFC heuristic, but not the post-propagation entropy a live "which cell
/// collapses next" overlay would want.
pub struct Grid3dEntropy;



/// 🎲️ Shannon entropy of the tile weight distribution, the same weighted-distribution math
/// `engine::weights::WeightTable` encodes.
pub fn shannon_entropy_over_tiles(snapshot: &Grid3dSnapshot) -> f64 {
    let weights: Vec<f64> = snapshot.tiles.iter().map(|tile| if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 }).collect();
    let total: f64 = weights.iter().sum();
    if weights.is_empty() || total <= 0.0 {
        return 0.0;
    }
    -weights.iter().map(|weight| weight / total).filter(|probability| *probability > 0.0).map(|probability| probability * probability.ln()).sum::<f64>()
}
//#endregion 🔖️Entropy

//#region 🧪️Tests

//#endregion 🧪️Tests
