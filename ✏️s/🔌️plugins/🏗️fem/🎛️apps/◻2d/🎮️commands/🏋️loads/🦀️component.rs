//! 🏋️ Fem2d play app commands — load cases, nodal/member-UDL/area loads, self-weight and combinations.

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
pub mod add_nodal_load {
    use super::*;

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
}
//#endregion 🔖️AddNodalLoad

//#region 🔖️AddMemberUdl
pub mod add_member_udl {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-member-udl")]
    pub struct AddMemberUdl {
        pub element_id: String,
        pub wx: f64,
        pub wy: f64,
        pub case_id: Option<String>,
    }

    pub fn handle(payload: &AddMemberUdl, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let load_id = next_load_id(doc.snapshot, payload.case_id.as_deref());
        let load = FemLoad::MemberUdl { id: load_id, element_id: payload.element_id.clone(), wx: payload.wx, wy: payload.wy };
        Ok(Emit::mutations(vec![add_load_mutation(doc.snapshot, payload.case_id.as_deref(), load)]))
    }
}
//#endregion 🔖️AddMemberUdl

//#region 🔖️AddAreaLoad
pub mod add_area_load {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-area-load")]
    pub struct AddAreaLoad {
        pub region_id: String,
        pub pressure: f64,
        pub case_id: Option<String>,
    }

    pub fn handle(payload: &AddAreaLoad, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let load_id = next_load_id(doc.snapshot, payload.case_id.as_deref());
        let load = FemLoad::Area { id: load_id, region_id: payload.region_id.clone(), pressure: payload.pressure };
        Ok(Emit::mutations(vec![add_load_mutation(doc.snapshot, payload.case_id.as_deref(), load)]))
    }
}
//#endregion 🔖️AddAreaLoad

//#region 🔖️AddLoadCase
pub mod add_load_case {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-load-case")]
    pub struct AddLoadCase {
        pub name: String,
        pub self_weight: bool,
    }

    pub fn handle(payload: &AddLoadCase, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = crate::app_surface::next_id(snapshot.load_cases.iter().map(|lc| lc.id.clone()), "case-");
        Ok(Emit::mutations(vec![Fem2dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase { load_case: FemLoadCase { id, name: payload.name.clone(), loads: Vec::new(), self_weight: payload.self_weight } })]))
    }
}
//#endregion 🔖️AddLoadCase

//#region 🔖️AddCombination
pub mod add_combination {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-combination")]
    pub struct AddCombination {
        pub name: String,
        pub terms: Vec<crate::artifacts::fem2d::FemCombinationTerm>,
    }

    pub fn handle(payload: &AddCombination, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = crate::app_surface::next_id(snapshot.combinations.iter().map(|c| c.id.clone()), "c");
        Ok(Emit::mutations(vec![Fem2dMutation::CreateCombination(create_combination::mutation::CreateCombination { combination: FemCombination { id, name: payload.name.clone(), terms: payload.terms.clone() } })]))
    }
}
//#endregion 🔖️AddCombination

//#region 🔖️SetSelfWeight
pub mod set_self_weight {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-self-weight")]
    pub struct SetSelfWeight {
        pub case_id: String,
        pub enabled: bool,
    }

    pub fn handle(payload: &SetSelfWeight, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        match snapshot.load_cases.iter().any(|lc| lc.id == payload.case_id) {
            true => Ok(Emit::mutations(vec![Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::mutation::ChangeLoadCaseSelfWeight { case_id: payload.case_id.clone(), new_self_weight: payload.enabled })])),
            false => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetSelfWeight

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
        dispatch(&mut app, Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: FemDof::Ty, value: -5000.0, case_id: None }));
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
        dispatch(&mut app, Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: FemDof::Ty, value: -5000.0, case_id: Some(live_id.clone()) }));
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
