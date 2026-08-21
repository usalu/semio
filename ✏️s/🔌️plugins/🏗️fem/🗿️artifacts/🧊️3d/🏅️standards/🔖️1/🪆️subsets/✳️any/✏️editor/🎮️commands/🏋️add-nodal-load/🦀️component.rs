//! 🏋️ 🏋️ FEM 3D app commands command — `add-nodal-load`.

use crate::artifacts::fem3d::mutations::{add_load, create_load_case};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemLoad, FemLoadCase};
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🔎️ Resolves the target load case for a load-adding command: the named `case_id` if given and
/// found, else the document's first load case, else `None` — a missing case is not resolved here,
/// the caller decides between `add-load` (existing case) and `create-load-case` (pre-seeded with the
/// new load, synthesized `"case-1"`/`"Load Case 1"`) once it knows which branch it's in.
async fn resolve_load_case(doc: &Fem3dSnapshot, case_id: Option<&str>) -> Option<FemLoadCase> {
    case_id.and_then(|id| doc.load_cases.iter().find(|lc| lc.id == id).cloned()).or_else(|| doc.load_cases.first().cloned())
}

/// 🌉️ Shared resolve-or-create gesture behind `add-nodal-load`/`add-member-udl`/`add-area-load`:
/// attaches `load` to the named/first load case via `add-load` if one exists, else synthesizes a
/// fresh `"case-1"`/`"Load Case 1"` case pre-seeded with `load` via `create-load-case`.
async fn add_load_mutation(doc: &Fem3dSnapshot, case_id: Option<&str>, load: FemLoad) -> Fem3dMutation {
    match resolve_load_case(doc, case_id) {
        Some(existing) => Fem3dMutation::AddLoad(add_load::mutation::AddLoad { case_id: existing.id, load: Box::new(load) }),
        None => Fem3dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase { load_case: FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: vec![load], self_weight: false } }),
    }
}

/// 🌉️ The load id a new load on the (possibly not-yet-existing) target case should get — reads the
/// existing case's loads for `next_id` continuity, or starts fresh for a synthesized case.
async fn next_load_id(doc: &Fem3dSnapshot, case_id: Option<&str>) -> String {
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
#[dsl(keyword = "add-nodal-load")]
pub struct AddNodalLoad {
    pub node_id: String,
    pub dof: crate::artifacts::fem3d::FemDof,
    pub value: f64,
    pub case_id: Option<String>,
}

pub async fn handle(payload: &AddNodalLoad, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let load_id = next_load_id(doc.snapshot, payload.case_id.as_deref());
    let load = FemLoad::Nodal { id: load_id, node_id: payload.node_id.clone(), dof: payload.dof, value: payload.value };
    Ok(Emit::mutations(vec![add_load_mutation(doc.snapshot, payload.case_id.as_deref(), load)]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem3d::commands::{add_combination, add_load_case, add_member_udl, set_self_weight};
    use crate::editor::fem3d::testkit::{dispatch, fem3d_app, Fem3dApp};
    use crate::editor::fem3d::Fem3dCommand;

    async fn app_with_load_case() -> Fem3dApp {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Dead".into(), self_weight: false }));
        app
    }

    #[semio_framework_async_macros::async_test]
    async fn resolve_load_case_returns_none_when_none_exist() {
        let snapshot = Fem3dSnapshot::default();
        assert!(resolve_load_case(&snapshot, None).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn add_nodal_load_with_no_existing_case_creates_one() {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddNodalLoad(AddNodalLoad { node_id: "n2".into(), dof: crate::artifacts::fem3d::FemDof::Tz, value: -5000.0, case_id: None }));
        let snapshot = app.snapshot().expect("snapshot");
        assert_eq!(snapshot.load_cases.len(), 1);
        assert_eq!(snapshot.load_cases[0].id, "case-1");
        assert!(matches!(snapshot.load_cases[0].loads[0], crate::artifacts::fem3d::FemLoad::Nodal { .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_member_udl_action_emits_op_3d() {
        let mut app = app_with_load_case();
        dispatch(&mut app, Fem3dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -2000.0, case_id: None }));
        let snapshot = app.snapshot().expect("snapshot");
        let load_case = &snapshot.load_cases[0];
        assert!(matches!(load_case.loads[0], crate::artifacts::fem3d::FemLoad::MemberUdl { .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_nodal_load_targets_named_case() {
        let mut app = app_with_load_case();
        dispatch(&mut app, Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }));
        let live_case_id = app.snapshot().expect("snapshot").load_cases[1].id.clone();
        dispatch(&mut app, Fem3dCommand::AddNodalLoad(AddNodalLoad { node_id: "n2".into(), dof: crate::artifacts::fem3d::FemDof::Tz, value: -5000.0, case_id: Some(live_case_id) }));
        let snapshot = app.snapshot().expect("snapshot");
        assert!(snapshot.load_cases[1].loads.iter().any(|l| matches!(l, crate::artifacts::fem3d::FemLoad::Nodal { .. })));
        assert!(snapshot.load_cases[0].loads.is_empty(), "the untargeted case must stay untouched");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_self_weight_toggles_existing_case() {
        let mut app = app_with_load_case();
        let case_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();
        dispatch(&mut app, Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id, enabled: true }));
        assert!(app.snapshot().expect("snapshot").load_cases[0].self_weight);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_self_weight_unknown_case_is_a_no_op() {
        let mut app = app_with_load_case();
        dispatch(&mut app, Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "missing".into(), enabled: true }));
        assert!(!app.snapshot().expect("snapshot").load_cases[0].self_weight);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_combination_parses_terms_json() {
        let mut app = app_with_load_case();
        dispatch(&mut app, Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: "[[\"case-0\",1.35]]".into() }));
        let snapshot = app.snapshot().expect("snapshot");
        assert_eq!(snapshot.combinations.len(), 1);
        assert_eq!(snapshot.combinations[0].terms.get("case-0"), Some(&1.35));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_combination_invalid_terms_json_is_a_no_op() {
        let mut app = app_with_load_case();
        dispatch(&mut app, Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: "not json".into() }));
        assert!(app.snapshot().expect("snapshot").combinations.is_empty());
    }
}
