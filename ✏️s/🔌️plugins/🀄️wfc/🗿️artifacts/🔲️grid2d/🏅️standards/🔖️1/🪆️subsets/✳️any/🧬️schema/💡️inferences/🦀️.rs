//! 💡️ Resumable inference results for Grid2d. Completed jobs publish derived answers; pure fields expose cheap authored metrics.
use crate::schema::snapshot::{Grid2dSnapshot, WfcDirection2d};

//#region 📦️RetainedPayload

//#endregion 📦️RetainedPayload

//#region 🔖️Identity











pub const MAX_GRID2D_TILES: usize = 4_096;
pub const MAX_GRID2D_RULES: usize = 262_144;
pub const MAX_GRID2D_CELLS: usize = 262_144;
pub const MAX_GRID2D_ID_BYTES: usize = 1_024;
pub const MAX_GRID2D_OUTPUT_BYTES: usize = 1 << 20;
pub const PARENT_PREVIEW_UNIT_INTERVAL: u64 = 16;
pub const PARENT_PREVIEW_TIME_INTERVAL_MS: u64 = 16;

/// 🧭️ The four-neighbour stencil's relation slots, in `Stencil2d::VonNeumann::offsets()` order —
/// `[(1,0), (-1,0), (0,1), (0,-1)]`, i.e. RIGHT, LEFT, BOTTOM, TOP with `y` growing downward.
pub fn relation_slot(direction: WfcDirection2d) -> usize {
    match direction {
        WfcDirection2d::Right => 0,
        WfcDirection2d::Left => 1,
        WfcDirection2d::Bottom => 2,
        WfcDirection2d::Top => 3,
    }
}
//#endregion 🔖️Identity

//#region 🔖️Protocol




/// 🏁 The solve's committed answer: one `[x, y, tileId]` row per unmasked cell, the satisfiability
/// verdict, and the PRE-propagation Shannon entropy of every unmasked cell's tile distribution.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Grid2dInferenceCommit {
    pub assignments: Vec<(u32, u32, String)>,
    pub contradiction: bool,
    pub entropy: Vec<(u32, u32, f64)>,
}








//#endregion 🔖️Protocol

//#region 🔖️Factory









//#endregion 🔖️Factory

//#region 🔖️Headless



//#endregion 🔖️Headless

//#region 🔖️Solve





//#endregion 🔖️Solve

//#region 🔖️Contradiction



//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲 Per-cell prior Shannon entropy over the tile-weight distribution — `0.0` for a pinned or
/// masked cell (fully determined), else the whole-catalogue prior.
pub struct Grid2dEntropy;



/// 🎲 Shannon entropy of the whole tile catalogue's weight distribution.
pub fn shannon_entropy_over_tiles(snapshot: &Grid2dSnapshot) -> f64 {
    let weights: Vec<f64> = snapshot.tiles.iter().map(|tile| if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 }).collect();
    let total: f64 = weights.iter().sum();
    if weights.is_empty() || total <= 0.0 {
        return 0.0;
    }
    -weights.iter().map(|weight| weight / total).filter(|share| *share > 0.0).map(|share| share * share.ln()).sum::<f64>()
}
//#endregion 🔖️Entropy

//#region 🧪️Tests

//#endregion 🧪️Tests
