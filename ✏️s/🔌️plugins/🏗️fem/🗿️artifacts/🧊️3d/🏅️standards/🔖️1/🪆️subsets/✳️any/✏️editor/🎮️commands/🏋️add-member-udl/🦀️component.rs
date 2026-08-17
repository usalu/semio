//! 🏋️ 🏋️ FEM 3D app commands command — `add-member-udl`.

use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::artifacts::fem3d::mutations::{add_load, change_load_case_self_weight, create_combination, create_load_case};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemLoad, FemLoadCase};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🔎️ Resolves the target load case for a load-adding command: the named `case_id` if given and
/// found, else the document's first load case, else `None` — a missing case is not resolved here,
/// the caller decides between `add-load` (existing case) and `create-load-case` (pre-seeded with the
/// new load, synthesized `"case-1"`/`"Load Case 1"`) once it knows which branch it's in.
fn resolve_load_case(doc: &Fem3dSnapshot, case_id: Option<&str>) -> Option<FemLoadCase> {
    case_id.and_then(|id| doc.load_cases.iter().find(|lc| lc.id == id).cloned()).or_else(|| doc.load_cases.first().cloned())
}

/// 🌉️ Shared resolve-or-create gesture behind `add-nodal-load`/`add-member-udl`/`add-area-load`:
/// attaches `load` to the named/first load case via `add-load` if one exists, else synthesizes a
/// fresh `"case-1"`/`"Load Case 1"` case pre-seeded with `load` via `create-load-case`.
fn add_load_mutation(doc: &Fem3dSnapshot, case_id: Option<&str>, load: FemLoad) -> Fem3dMutation {
    match resolve_load_case(doc, case_id) {
        Some(existing) => Fem3dMutation::AddLoad(add_load::mutation::AddLoad { case_id: existing.id, load: Box::new(load) }),
        None => Fem3dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase { load_case: FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: vec![load], self_weight: false } }),
    }
}

/// 🌉️ The load id a new load on the (possibly not-yet-existing) target case should get — reads the
/// existing case's loads for `next_id` continuity, or starts fresh for a synthesized case.
fn next_load_id(doc: &Fem3dSnapshot, case_id: Option<&str>) -> String {
    let loads = resolve_load_case(doc, case_id).map(|lc| lc.loads).unwrap_or_default();
    crate::app_surface::next_id(loads.iter().map(|l| crate::artifacts::fem3d::load_id(l).to_string()), "l")
}

//#region 🔖️AddNodalLoad
//#endregion 🔖️AddNodalLoad

//#region 🔖️AddMemberUdl
//#endregion 🔖️AddMemberUdl

//#region 🔖️AddAreaLoad
//#endregion 🔖️AddAreaLoad

//#region 🔖️AddLoadCase
//#endregion 🔖️AddLoadCase

//#region 🔖️AddCombination
//#endregion 🔖️AddCombination

//#region 🔖️SetSelfWeight
//#endregion 🔖️SetSelfWeight

// #region 🧪️Tests

// #endregion 🧪️Tests

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-member-udl")]
pub struct AddMemberUdl {
    pub element_id: String,
    pub wx: f64,
    pub wy: f64,
    pub wz: f64,
    pub case_id: Option<String>,
}

pub fn handle(payload: &AddMemberUdl, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let load_id = next_load_id(doc.snapshot, payload.case_id.as_deref());
    let load = FemLoad::MemberUdl { id: load_id, element_id: payload.element_id.clone(), wx: payload.wx, wy: payload.wy, wz: payload.wz };
    Ok(Emit::mutations(vec![add_load_mutation(doc.snapshot, payload.case_id.as_deref(), load)]))
}
