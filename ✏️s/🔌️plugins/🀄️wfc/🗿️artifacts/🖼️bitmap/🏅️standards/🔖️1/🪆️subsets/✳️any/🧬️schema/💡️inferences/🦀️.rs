//! 💡️ Bitmap inferences — THE SOLVE ITSELF IS AN INFERENCE. `BitmapSnapshot` only ever persists the
//! PROBLEM (input sample, palette, output extent, model parameters, pins, seed); the solved output
//! bitmap, the contradiction verdict and the entropy map are all derived here, never
//! mutation-authored state.
//!
//! The route is the classic overlapping model end to end: `extract_2d` learns `N × N` patterns from
//! the input sample (expanded under the first `model.symmetry` elements of D4), `Grid2dTopology`
//! gives the output its four-neighbour stencil with boundaries taken from `output.periodic`, and the
//! resumable `WfcJob` — the same one every interactive caller drives — collapses it. Determinism:
//! every input is a snapshot field, `seed` included, and no ambient randomness enters, so `DepHash`
//! caching over `BitmapSolve`/`BitmapContradiction`/`BitmapEntropy` is sound.

use crate::schema::snapshot::{BitmapSnapshot};
use semio_s_plugin_wfc_engine as engine;

//#region 📦️RetainedPayload



//#endregion 📦️RetainedPayload

//#region 🔖️Identity














//#endregion 🔖️Identity

//#region 🔖️Admission
/// 🚧️ The largest input sample one solve may learn from — 256 × 256 index bytes.
pub(crate) const MAX_BITMAP_INPUT_CELLS: usize = 65_536;
/// 🚧️ The largest output one solve may collapse — 256 × 256 cells, which also bounds the entropy
/// map the commit carries.
pub(crate) const MAX_BITMAP_OUTPUT_CELLS: usize = 65_536;
/// 🚧️ The largest pattern universe extraction may produce before the compatibility table itself
/// becomes the binding cost.
pub(crate) const MAX_BITMAP_PATTERNS: usize = 4_096;
pub(crate) const MAX_BITMAP_PINS: usize = 65_536;
pub(crate) const MAX_BITMAP_OUTPUT_BYTES: usize = 4 << 20;
/// 📝️ How many base64 characters one encode step appends — well under one payload page, so a step
/// never needs a page it cannot admit.
pub(crate) const BITMAP_ENCODE_CHUNK: usize = 1_024;
/// 🏎️ The headless adapter is a BATCH driver, not an interactive one: a single fuel unit per session
/// step turns one small collapse into tens of thousands of session round trips (a 6 × 4 output took
/// over a minute that way, measured). A large fuel budget per step keeps every step bounded while
/// letting one call do a whole propagation wave.
pub(crate) const HEADLESS_FUEL_PER_STEP: u64 = 4_096;
/// ⏱️ STRICTLY below `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` (8 000 µs). A session step
/// that aims past the ceiling reports a contract violation on EVERY step, and four consecutive
/// violations quarantine the whole session with `job-session.terminal-fault` — which is what a 50 ms
/// budget did to every live solve in the react shell while the native tests stayed green (a test
/// binary installs no monotonic clock, so it records no violation at all). The budget bounds one step,
/// never the whole collapse: the driver resumes until the job is terminal.
pub(crate) const HEADLESS_STEP_BUDGET_US: u64 = 4_000;
pub(crate) const PARENT_PREVIEW_UNIT_INTERVAL: u64 = 16;
pub(crate) const PARENT_PREVIEW_TIME_INTERVAL_MS: u64 = 16;
//#endregion 🔖️Admission

//#region 🔖️Protocol




/// 🏁 What one solve concludes with: the output bitmap as base64 palette indices (row-major), the
/// satisfiability verdict, and the per-cell prior entropy map.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct BitmapInferenceCommit {
    #[value(with = "semio_framework_value::bytes")]
    pub pixels: Vec<u8>,
    pub contradiction: bool,
    pub entropy: Vec<f64>,
}




//#endregion 🔖️Protocol

//#region 🔖️Compile
/// 🔄️ The classic `1..=8` overlapping-model symmetry count: the first `count` elements of D4 listed
/// as the reference implementation lists them — rotations and reflections ALTERNATING, `e, m, r,
/// rm, r², r²m, r³, r³m` — not `Transform2d::ALL`'s rotations-then-reflections order. The
/// difference is load-bearing at every count below eight: the alternating order makes `2` a MIRROR
/// (identity plus a horizontal flip), which is what an author reaching for "2" means, while the
/// declaration order would make it identity plus a quarter turn and stand a meadow on its side.
/// `1` learns the sample verbatim and `8` is the full dihedral group either way.
pub const BITMAP_SYMMETRY_ORDER: [engine::symmetry::Transform2d; 8] = [
    engine::symmetry::Transform2d::Identity,
    engine::symmetry::Transform2d::FlipH,
    engine::symmetry::Transform2d::Rot90,
    engine::symmetry::Transform2d::FlipDiag,
    engine::symmetry::Transform2d::Rot180,
    engine::symmetry::Transform2d::FlipV,
    engine::symmetry::Transform2d::Rot270,
    engine::symmetry::Transform2d::FlipAntiDiag,
];

pub fn symmetry_group(count: u32) -> engine::symmetry::SymmetryGroup2d {
    let take = (count.clamp(1, 8)) as usize;
    engine::symmetry::SymmetryGroup2d::Custom(BITMAP_SYMMETRY_ORDER[..take].to_vec())
}

/// 🗺️ The four relation ids `extract_2d` declares, in `Stencil2d::VonNeumann.offsets()` order.
/// `extract_2d` builds its model on a FRESH `ModelBuilder` and calls `declare_stencil_relations`
/// before anything else, so the relations it registers are exactly `0..4` in that order — the
/// coupling this function makes explicit instead of leaving implicit at the topology call site.
pub(crate) fn stencil_relations() -> Vec<engine::ids::RelationId> {
    (0..4).map(engine::ids::RelationId::from_index).collect()
}















/// 🧱 Compiled overlapping collapse ready for an interactive `WfcJob` — shared by the headless
/// oracle and the fill tool run so both drive the same model, topology, pins and decoder.
pub struct BitmapCollapseParts {
    pub model: engine::model::CompiledModel,
    pub topology: engine::grid2d::Grid2dTopology,
    pub fixed: Vec<(engine::ids::NodeId, engine::ids::PatternId)>,
    pub decoder: engine::extract::PatternDecoder2d,
}

/// 🧱 Learn patterns, build the output grid and apply pins for one snapshot.
pub fn compile_bitmap_collapse(snapshot: &BitmapSnapshot) -> Result<BitmapCollapseParts, String> {
    let input_cells = (snapshot.input.width as usize).saturating_mul(snapshot.input.height as usize);
    let output_cells = (snapshot.output.width as usize).saturating_mul(snapshot.output.height as usize);
    if input_cells == 0 || output_cells == 0 || input_cells > MAX_BITMAP_INPUT_CELLS || output_cells > MAX_BITMAP_OUTPUT_CELLS || snapshot.pinned.len() > MAX_BITMAP_PINS || snapshot.input.palette.is_empty() {
        return Err("bitmap-inference-admission-exceeded".into());
    }
    let indices = snapshot.input.indices().ok_or("bitmap-inference-malformed-input")?;
    if indices.iter().any(|index| usize::from(*index) >= snapshot.input.palette.len()) {
        return Err("bitmap-inference-unknown-palette-index".into());
    }
    let tiles = indices.iter().map(|index| engine::ids::TileId(u32::from(*index))).collect();
    let sample = engine::extract::Sample2d::new(snapshot.input.width as usize, snapshot.input.height as usize, tiles);
    let config = engine::extract::Extract2dConfig { window: snapshot.model.pattern_size.max(1) as usize, periodic_input: snapshot.model.periodic_input, symmetry: symmetry_group(snapshot.model.symmetry) };
    let extracted = engine::extract::extract_2d(&[sample], &config).map_err(|error| format!("{error:?}"))?;
    if extracted.model.pattern_count() > MAX_BITMAP_PATTERNS {
        return Err("bitmap-inference-pattern-universe-exceeded".into());
    }
    let decoder = extracted.decoder.clone();
    let boundary = if snapshot.output.periodic { engine::grid2d::Boundary::Wrap } else { engine::grid2d::Boundary::Open };
    let topology = engine::grid2d::Grid2dTopology::new(snapshot.output.width as usize, snapshot.output.height as usize, &engine::grid2d::Stencil2d::VonNeumann, stencil_relations(), boundary, boundary, None).map_err(|error| format!("{error:?}"))?;
    let mut fixed = Vec::new();
    for pin in &snapshot.pinned {
        if pin.x >= snapshot.output.width || pin.y >= snapshot.output.height {
            return Err("bitmap-inference-pin-outside-output".into());
        }
        let pattern = (0..extracted.model.pattern_count()).map(engine::ids::PatternId::from_index).find(|pattern| decoder.anchor_tile(*pattern).get() == pin.color).ok_or("bitmap-inference-unreachable-pin")?;
        let node = engine::ids::NodeId::from_index((pin.y as usize) * (snapshot.output.width as usize) + pin.x as usize);
        fixed.push((node, pattern));
    }
    if let Some(ground) = snapshot.model.ground {
        if let Some(pattern) = (0..extracted.model.pattern_count()).map(engine::ids::PatternId::from_index).find(|pattern| decoder.anchor_tile(*pattern).get() == ground) {
            let width = snapshot.output.width as usize;
            let row = (snapshot.output.height as usize).saturating_sub(1);
            for x in 0..width {
                let node = engine::ids::NodeId::from_index(row * width + x);
                if !fixed.iter().any(|(fixed_node, _)| *fixed_node == node) {
                    fixed.push((node, pattern));
                }
            }
        }
    }
    Ok(BitmapCollapseParts { model: extracted.model, topology, fixed, decoder })
}




//#endregion 🔖️Compile

//#region 🔖️Solve
/// 🏁 The solved output bitmap, or `Unsolved` for every non-`Solved` outcome (contradiction, budget,
/// cancellation) — see [`BitmapContradiction`] for the dedicated satisfiability verdict.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum BitmapSolveResult {
    #[default]
    Unsolved,
    Solved {
        #[value(with = "semio_framework_value::bytes")]
        pixels: Vec<u8>,
    },
}

pub struct BitmapSolve;


//#endregion 🔖️Solve

//#region 🔖️Contradiction
/// 🩺 The satisfiability verdict on its own, so a caller who only needs "can this sample tile that
/// output at all" never has to decode a whole bitmap to find out.
pub struct BitmapContradiction;


//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲 The per-cell PRIOR entropy map, keyed by the cell's row-major index — the same vector the
/// cold-job commit carries, exposed as an ordinary `InferredField` for callers that want it without
/// running a full collapse.
pub struct BitmapEntropy;



/// 🎲 The whole-universe Shannon entropy of the extracted pattern weight distribution — the value
/// every unfixed cell carries before propagation narrows anything.
pub fn pattern_universe_entropy(snapshot: &BitmapSnapshot) -> f64 {
    let Some(indices) = snapshot.input.indices() else { return 0.0 };
    let tiles = indices.iter().map(|index| engine::ids::TileId(u32::from(*index))).collect();
    let sample = engine::extract::Sample2d::new(snapshot.input.width as usize, snapshot.input.height as usize, tiles);
    let config = engine::extract::Extract2dConfig { window: snapshot.model.pattern_size.max(1) as usize, periodic_input: snapshot.model.periodic_input, symmetry: symmetry_group(snapshot.model.symmetry) };
    let Ok(extracted) = engine::extract::extract_2d(&[sample], &config) else { return 0.0 };
    let weights = extracted.model.weights();
    let total: f64 = (0..extracted.model.pattern_count()).map(|index| weights.w(engine::ids::PatternId::from_index(index))).sum();
    if total <= 0.0 {
        return 0.0;
    }
    -(0..extracted.model.pattern_count()).map(|index| weights.w(engine::ids::PatternId::from_index(index)) / total).filter(|share| *share > 0.0).map(|share| share * share.ln()).sum::<f64>()
}
//#endregion 🔖️Entropy

//#region 🧪️Tests

//#endregion 🧪️Tests
