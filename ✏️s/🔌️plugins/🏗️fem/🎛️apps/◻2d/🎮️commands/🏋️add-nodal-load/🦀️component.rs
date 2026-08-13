//! 🏋️ 🏋️ Fem2d play app commands command — `add-nodal-load`.

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::mutations::{add_load, change_load_case_self_weight, create_combination, create_load_case};
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::{FemCombination, FemDof, FemLoad, FemLoadCase};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

/// 🔎️ Resolves the target load case for a load-adding command: the named `case_id` if given and
/// found, else the document's first load case, else `None` — a missing case is not resolved here,
/// the caller decides between `add-load` (existing case) and `create-load-case` (pre-seeded with the
/// new load, synthesized `"case-1"`/`"Load Case 1"`) once it knows which branch it's in.
fn resolve_load_case(doc: &Fem2dSnapshot, case_id: Option<&str>) -> Option<FemLoadCase> {
    case_id.and_then(|id| doc.load_cases.iter().find(|lc| lc.id == id).cloned()).or_else(|| doc.load_cases.first().cloned())
}

/// 🌉️ Shared resolve-or-create gesture behind `add-nodal-load`/`add-member-udl`/`add-area-load`:
/// attaches `load` to the named/first load case via `add-load` if one exists, else synthesizes a
/// fresh `"case-1"`/`"Load Case 1"` case pre-seeded with `load` via `create-load-case`.
fn add_load_mutation(doc: &Fem2dSnapshot, case_id: Option<&str>, load: FemLoad) -> Fem2dMutation {
    match resolve_load_case(doc, case_id) {
        Some(existing) => Fem2dMutation::AddLoad(add_load::mutation::AddLoad { case_id: existing.id, load: Box::new(load) }),
        None => Fem2dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase { load_case: FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: vec![load], self_weight: false } }),
    }
}

/// 🌉️ The load id a new load on the (possibly not-yet-existing) target case should get — reads the
/// existing case's loads for `next_id` continuity, or starts fresh for a synthesized case.
fn next_load_id(doc: &Fem2dSnapshot, case_id: Option<&str>) -> String {
    let loads = resolve_load_case(doc, case_id).map(|lc| lc.loads).unwrap_or_default();
    crate::app_surface::next_id(loads.iter().map(|l| crate::artifacts::fem2d::load_id(l).to_string()), "l")
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-nodal-load")]
pub struct AddNodalLoad {
    pub node_id: String,
    pub dof: FemDof,
    pub value: f64,
    pub case_id: Option<String>,
}

pub fn handle(payload: &AddNodalLoad, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let load_id = next_load_id(doc.snapshot, payload.case_id.as_deref());
    let load = FemLoad::Nodal { id: load_id, node_id: payload.node_id.clone(), dof: payload.dof, value: payload.value };
    Ok(Emit::mutations(vec![add_load_mutation(doc.snapshot, payload.case_id.as_deref(), load)]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem2d::testkit::{dispatch, fem2d_app};
    use crate::apps::fem2d::Fem2dCommand;

    fn with_dead_case(app: &mut crate::apps::fem2d::testkit::Fem2dApp) {
        dispatch(app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Dead".into(), self_weight: false }));
    }

    #[test]
    fn add_load_case_and_combination_emit_ops_2d() {
        let mut app = fem2d_app();
        with_dead_case(&mut app);
        dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }));
        assert_eq!(app.snapshot().expect("snapshot").load_cases.last().expect("case added").name, "Live");

        let dead_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();
        dispatch(&mut app, Fem2dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: dead_id.clone(), factor: 1.35 }] }));
        assert_eq!(app.snapshot().expect("snapshot").combinations.last().expect("combination added").terms, vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: dead_id, factor: 1.35 }]);
    }

    #[test]
    fn add_nodal_load_with_no_existing_case_creates_one_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddNodalLoad(AddNodalLoad { node_id: "n1".into(), dof: FemDof::Ty, value: -5000.0, case_id: None }));
        let snapshot = app.snapshot().expect("snapshot");
        assert_eq!(snapshot.load_cases.len(), 1);
        assert_eq!(snapshot.load_cases[0].id, "case-1");
        assert!(matches!(snapshot.load_cases[0].loads[0], FemLoad::Nodal { .. }));
    }

    #[test]
    fn add_area_load_targets_named_case_2d() {
        let mut app = fem2d_app();
        with_dead_case(&mut app);
        dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }));
        let live_id = app.snapshot().expect("snapshot").load_cases[1].id.clone();
        dispatch(&mut app, Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some(live_id.clone()) }));
        let load_case = app.snapshot().expect("snapshot").load_cases[1].clone();
        assert_eq!(load_case.id, live_id);
        assert!(matches!(load_case.loads[0], FemLoad::Area { .. }));
    }

    #[test]
    fn set_self_weight_toggles_case_2d() {
        let mut app = fem2d_app();
        with_dead_case(&mut app);
        let case_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();
        dispatch(&mut app, Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id, enabled: true }));
        assert!(app.snapshot().expect("snapshot").load_cases[0].self_weight);
    }

    #[test]
    fn set_self_weight_unknown_case_is_a_no_op_2d() {
        let mut app = fem2d_app();
        with_dead_case(&mut app);
        dispatch(&mut app, Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "missing".into(), enabled: true }));
        assert!(!app.snapshot().expect("snapshot").load_cases[0].self_weight);
    }

    #[test]
    fn add_nodal_load_action_targets_named_case_2d() {
        let mut app = fem2d_app();
        with_dead_case(&mut app);
        dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }));
        let live_id = app.snapshot().expect("snapshot").load_cases[1].id.clone();
        dispatch(&mut app, Fem2dCommand::AddNodalLoad(AddNodalLoad { node_id: "n1".into(), dof: FemDof::Ty, value: -5000.0, case_id: Some(live_id.clone()) }));
        let load_case = app.snapshot().expect("snapshot").load_cases[1].clone();
        assert_eq!(load_case.id, live_id);
        assert!(matches!(load_case.loads[0], FemLoad::Nodal { .. }));
    }

    #[test]
    fn add_member_udl_action_emits_op_2d() {
        let mut app = fem2d_app();
        with_dead_case(&mut app);
        dispatch(&mut app, Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None }));
        assert!(matches!(app.snapshot().expect("snapshot").load_cases[0].loads[0], FemLoad::MemberUdl { .. }));
    }
}
//#endregion 🧪️Tests
