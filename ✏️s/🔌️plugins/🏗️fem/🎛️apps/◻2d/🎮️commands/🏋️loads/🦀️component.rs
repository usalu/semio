//! 🏋️ Fem2d play app commands — load cases, nodal/member-UDL/area loads, self-weight and combinations.

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigOperation};
use crate::artifacts::fem2d::op::Fem2dOperation;
use crate::artifacts::fem2d::{load_id, FemCombination, FemCombinationTerm, FemDof, FemLoad, FemLoadCase};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dDocument = crate::artifacts::fem2d::Fem2dDocument;

/// 🔎️ Finds the load case an incoming load/self-weight edit should target: the named `case_id` if it
/// exists, else the first case, else a freshly synthesized `"case-1"` — shared by `addNodalLoad`,
/// `addMemberUdl`, `addAreaLoad`, and `setSelfWeight` so every load-mutating action resolves its
/// target case the same way. Returns the case's collection index (`load_cases.len()` for a fresh one)
/// alongside an owned clone ready to be mutated and re-emitted via `SetLoadCase`.
fn fem2d_resolve_load_case(doc: &Fem2dDocument, case_id: Option<&str>) -> (usize, FemLoadCase) {
    let named = case_id.and_then(|id| doc.load_cases.iter().find(|lc| lc.id == id).cloned());
    let load_case = named.or_else(|| doc.load_cases.first().cloned()).unwrap_or_else(|| FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: Vec::new(), self_weight: false });
    let index = doc.load_cases.iter().position(|lc| lc.id == load_case.id).unwrap_or(doc.load_cases.len());
    (index, load_case)
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

    pub fn handle(payload: &AddNodalLoad, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let (index, mut load_case) = fem2d_resolve_load_case(doc.projection, payload.case_id.as_deref());
        let new_id = crate::core::shared::next_id(load_case.loads.iter().map(|l| load_id(l).to_string()), "l");
        load_case.loads.push(FemLoad::Nodal { id: new_id, node_id: payload.node_id.clone(), dof: payload.dof, value: payload.value });
        Ok(Emit::operations(vec![Fem2dOperation::SetLoadCase { index, load_case }]))
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

    pub fn handle(payload: &AddMemberUdl, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let (index, mut load_case) = fem2d_resolve_load_case(doc.projection, payload.case_id.as_deref());
        let new_id = crate::core::shared::next_id(load_case.loads.iter().map(|l| load_id(l).to_string()), "l");
        load_case.loads.push(FemLoad::MemberUdl { id: new_id, element_id: payload.element_id.clone(), wx: payload.wx, wy: payload.wy });
        Ok(Emit::operations(vec![Fem2dOperation::SetLoadCase { index, load_case }]))
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

    pub fn handle(payload: &AddAreaLoad, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let (index, mut load_case) = fem2d_resolve_load_case(doc.projection, payload.case_id.as_deref());
        let new_id = crate::core::shared::next_id(load_case.loads.iter().map(|l| load_id(l).to_string()), "l");
        load_case.loads.push(FemLoad::Area { id: new_id, region_id: payload.region_id.clone(), pressure: payload.pressure });
        Ok(Emit::operations(vec![Fem2dOperation::SetLoadCase { index, load_case }]))
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

    pub fn handle(payload: &AddLoadCase, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::core::shared::next_id(projection.load_cases.iter().map(|lc| lc.id.clone()), "case-");
        let index = projection.load_cases.len();
        Ok(Emit::operations(vec![Fem2dOperation::SetLoadCase { index, load_case: FemLoadCase { id, name: payload.name.clone(), loads: Vec::new(), self_weight: payload.self_weight } }]))
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
        pub terms: Vec<FemCombinationTerm>,
    }

    pub fn handle(payload: &AddCombination, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::core::shared::next_id(projection.combinations.iter().map(|c| c.id.clone()), "c");
        let index = projection.combinations.len();
        Ok(Emit::operations(vec![Fem2dOperation::SetCombination { index, combination: FemCombination { id, name: payload.name.clone(), terms: payload.terms.clone() } }]))
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

    pub fn handle(payload: &SetSelfWeight, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let projection = doc.projection;
        match projection.load_cases.iter().position(|lc| lc.id == payload.case_id) {
            Some(index) => {
                let mut load_case = projection.load_cases[index].clone();
                load_case.self_weight = payload.enabled;
                Ok(Emit::operations(vec![Fem2dOperation::SetLoadCase { index, load_case }]))
            }
            None => Ok(Emit::default()),
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
        assert_eq!(app.projection().expect("projection").load_cases.last().expect("case added").name, "Live");

        dispatch(&mut app, Fem2dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }] }));
        assert_eq!(app.projection().expect("projection").combinations.last().expect("combination added").terms, vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }]);
    }

    #[test]
    fn add_area_load_targets_named_case_2d() {
        let mut app = fem2d_app();
        with_dead_case(&mut app);
        dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }));
        let live_id = app.projection().expect("projection").load_cases[1].id.clone();
        dispatch(&mut app, Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some(live_id.clone()) }));
        let load_case = app.projection().expect("projection").load_cases[1].clone();
        assert_eq!(load_case.id, live_id);
        assert!(matches!(load_case.loads[0], FemLoad::Area { .. }));
    }

    #[test]
    fn set_self_weight_toggles_case_2d() {
        let mut app = fem2d_app();
        with_dead_case(&mut app);
        let case_id = app.projection().expect("projection").load_cases[0].id.clone();
        dispatch(&mut app, Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id, enabled: true }));
        assert!(app.projection().expect("projection").load_cases[0].self_weight);
    }

    #[test]
    fn add_nodal_load_action_targets_named_case_2d() {
        let mut app = fem2d_app();
        with_dead_case(&mut app);
        dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }));
        let live_id = app.projection().expect("projection").load_cases[1].id.clone();
        dispatch(&mut app, Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: FemDof::Ty, value: -5000.0, case_id: Some(live_id.clone()) }));
        let load_case = app.projection().expect("projection").load_cases[1].clone();
        assert_eq!(load_case.id, live_id);
        assert!(matches!(load_case.loads[0], FemLoad::Nodal { .. }));
    }

    #[test]
    fn add_member_udl_action_emits_op_2d() {
        let mut app = fem2d_app();
        with_dead_case(&mut app);
        dispatch(&mut app, Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None }));
        assert!(matches!(app.projection().expect("projection").load_cases[0].loads[0], FemLoad::MemberUdl { .. }));
    }
}
//#endregion 🧪️Tests
