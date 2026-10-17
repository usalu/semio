//! 🧰️ Standalone projections of one inference kind: each reads the model graph through its driver and takes the one table, for tests, oracles and one-shot callers that want a single kind without a session.

use super::{kinds, try_infer_selected};
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use std::collections::BTreeMap;
use crate::standards::v1::subsets::any::schema::inferences::{ModelInference, curtain_layout::CurtainLayout, diagnostics::Diagnostic, element_solids::ElementSolid, opening_frames::OpeningFrame, plan_linework::PlanLinework, quantities::ModelQuantities, ramp_runs::RampRun, spaces::SpaceRoom, stair_runs::StairRun, storey_levels::StoreyLevel, view_linework::ViewLinework, wall_layout::WallLayout};

/// 🪞️ The layout of every curtain wall (the `CurtainLayout` nodes of the model graph).
pub fn compute_curtain_layout(snapshot: &ModelSnapshot) -> BTreeMap<String, CurtainLayout> {
    std::mem::take(&mut try_infer_selected::<{ kinds::CURTAINS }>(snapshot).unwrap_or_else(|error| panic!("{error}")).curtain_layout)
}

/// ⚠️ The findings of the whole model (the `Diagnostics` nodes of the model graph, ordered).
pub fn compute_diagnostics(snapshot: &ModelSnapshot) -> Vec<Diagnostic> {
    std::mem::take(&mut try_infer_selected::<{ kinds::DIAGNOSTICS }>(snapshot).unwrap_or_else(|error| panic!("{error}")).diagnostics)
}

/// 🧊️ The solid of every element that has geometry, keyed by element id (the `Solid` nodes of the model graph).
pub fn compute_element_solids(snapshot: &ModelSnapshot) -> BTreeMap<String, ElementSolid> {
    std::mem::take(&mut try_infer_selected::<{ kinds::SOLIDS }>(snapshot).unwrap_or_else(|error| panic!("{error}")).element_solids)
}

/// 🪟️ The frame of every opening (the `OpeningFrame` nodes of the model graph).
pub fn compute_opening_frames(snapshot: &ModelSnapshot) -> BTreeMap<String, OpeningFrame> {
    std::mem::take(&mut try_infer_selected::<{ kinds::FRAMES }>(snapshot).unwrap_or_else(|error| panic!("{error}")).opening_frames)
}

/// 🗺️ The plan of every storey (the `Plan` nodes of the model graph).
pub fn compute_plan_linework(snapshot: &ModelSnapshot) -> BTreeMap<String, PlanLinework> {
    std::mem::take(&mut try_infer_selected::<{ kinds::PLANS }>(snapshot).unwrap_or_else(|error| panic!("{error}")).plan_linework)
}

/// 🧮️ The take-off of a model (the `Quantity` and `Totals` nodes of the model graph). The inference argument is ignored: the graph derives everything the take-off reads in one run.
pub fn compute_quantities(snapshot: &ModelSnapshot, _inferred: &ModelInference) -> ModelQuantities {
    std::mem::take(&mut try_infer_selected::<{ kinds::QUANTITIES }>(snapshot).unwrap_or_else(|error| panic!("{error}")).quantities)
}

/// 🛝️ The run of every ramp (the `RampRun` nodes of the model graph).
pub fn compute_ramp_runs(snapshot: &ModelSnapshot) -> BTreeMap<String, RampRun> {
    std::mem::take(&mut try_infer_selected::<{ kinds::RAMP_RUNS }>(snapshot).unwrap_or_else(|error| panic!("{error}")).ramp_runs)
}

/// 🏠️ The room of every space (the `Room` nodes of the model graph, flattened).
pub fn compute_spaces(snapshot: &ModelSnapshot) -> BTreeMap<String, SpaceRoom> {
    std::mem::take(&mut try_infer_selected::<{ kinds::ROOMS }>(snapshot).unwrap_or_else(|error| panic!("{error}")).spaces)
}

/// 🪜️ The run of every stair (the `StairRun` nodes of the model graph).
pub fn compute_stair_runs(snapshot: &ModelSnapshot) -> BTreeMap<String, StairRun> {
    std::mem::take(&mut try_infer_selected::<{ kinds::RUNS }>(snapshot).unwrap_or_else(|error| panic!("{error}")).stair_runs)
}

/// 🪜️ The levels of every storey (the `Storey` nodes of the model graph).
pub fn compute_storey_levels(snapshot: &ModelSnapshot) -> BTreeMap<String, StoreyLevel> {
    std::mem::take(&mut try_infer_selected::<{ kinds::LEVELS }>(snapshot).unwrap_or_else(|error| panic!("{error}")).storey_levels)
}

/// 🖼️ The drawing of every view (the `View` nodes of the model graph).
pub fn compute_view_linework(snapshot: &ModelSnapshot) -> BTreeMap<String, ViewLinework> {
    std::mem::take(&mut try_infer_selected::<{ kinds::VIEWS }>(snapshot).unwrap_or_else(|error| panic!("{error}")).view_linework)
}

/// 🧱️ The layout of every wall (the `WallLayout` nodes of the model graph).
pub fn compute_wall_layout(snapshot: &ModelSnapshot) -> BTreeMap<String, WallLayout> {
    std::mem::take(&mut try_infer_selected::<{ kinds::LAYOUTS }>(snapshot).unwrap_or_else(|error| panic!("{error}")).wall_layout)
}
