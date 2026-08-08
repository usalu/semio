//! 🏋️ FEM 3D app commands — load cases: nodal/member-UDL/area loads, whole load cases, combinations,
//! and the self-weight toggle.

use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemLoadCase};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🔎️ Resolves the target load case for a load-adding command: the named `case_id` if given and
/// found, else the document's first load case, else a synthesized `"case-1"` placeholder (never
/// inserted — the caller's `SetLoadCase` operation does that). Returns `(index, load_case)`, where
/// `index` is the case's position if it already exists, or `doc.load_cases.len()` if it doesn't (the
/// position a `SetLoadCase` should insert the synthesized case at).
fn resolve_load_case(doc: &Fem3dSnapshot, case_id: Option<&str>) -> (usize, FemLoadCase) {
    let named = case_id.and_then(|id| doc.load_cases.iter().find(|lc| lc.id == id).cloned());
    let load_case = named.or_else(|| doc.load_cases.first().cloned()).unwrap_or_else(|| FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: Vec::new(), self_weight: false });
    let index = doc.load_cases.iter().position(|lc| lc.id == load_case.id).unwrap_or(doc.load_cases.len());
    (index, load_case)
}

//#region 🔖️AddNodalLoad
pub mod add_nodal_load {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    #[dsl(keyword = "add-nodal-load")]
    pub struct AddNodalLoad {
        pub node_id: String,
        pub dof: crate::artifacts::fem3d::FemDof,
        pub value: f64,
        pub case_id: Option<String>,
    }

    pub fn handle(payload: &AddNodalLoad, doc: &DocumentView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
        let (index, mut load_case) = resolve_load_case(doc.snapshot, payload.case_id.as_deref());
        let load_id = crate::app_surface::next_id(load_case.loads.iter().map(|l| crate::artifacts::fem3d::load_id(l).to_string()), "l");
        load_case.loads.push(crate::artifacts::fem3d::FemLoad::Nodal { id: load_id, node_id: payload.node_id.clone(), dof: payload.dof, value: payload.value });
        Ok(Emit::mutations(vec![Fem3dMutation::SetLoadCase { index, load_case }]))
    }
}
//#endregion 🔖️AddNodalLoad

//#region 🔖️AddMemberUdl
pub mod add_member_udl {
    use super::*;

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

    pub fn handle(payload: &AddMemberUdl, doc: &DocumentView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
        let (index, mut load_case) = resolve_load_case(doc.snapshot, payload.case_id.as_deref());
        let load_id = crate::app_surface::next_id(load_case.loads.iter().map(|l| crate::artifacts::fem3d::load_id(l).to_string()), "l");
        load_case.loads.push(crate::artifacts::fem3d::FemLoad::MemberUdl { id: load_id, element_id: payload.element_id.clone(), wx: payload.wx, wy: payload.wy, wz: payload.wz });
        Ok(Emit::mutations(vec![Fem3dMutation::SetLoadCase { index, load_case }]))
    }
}
//#endregion 🔖️AddMemberUdl

//#region 🔖️AddAreaLoad
pub mod add_area_load {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    #[dsl(keyword = "add-area-load")]
    pub struct AddAreaLoad {
        pub solid_id: String,
        pub pressure: f64,
        pub case_id: Option<String>,
    }

    pub fn handle(payload: &AddAreaLoad, doc: &DocumentView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
        let (index, mut load_case) = resolve_load_case(doc.snapshot, payload.case_id.as_deref());
        let load_id = crate::app_surface::next_id(load_case.loads.iter().map(|l| crate::artifacts::fem3d::load_id(l).to_string()), "l");
        load_case.loads.push(crate::artifacts::fem3d::FemLoad::Area { id: load_id, solid_id: payload.solid_id.clone(), pressure: payload.pressure });
        Ok(Emit::mutations(vec![Fem3dMutation::SetLoadCase { index, load_case }]))
    }
}
//#endregion 🔖️AddAreaLoad

//#region 🔖️AddLoadCase
pub mod add_load_case {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    #[dsl(keyword = "add-load-case")]
    pub struct AddLoadCase {
        pub name: String,
        pub self_weight: bool,
    }

    pub fn handle(payload: &AddLoadCase, doc: &DocumentView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = crate::app_surface::next_id(snapshot.load_cases.iter().map(|lc| lc.id.clone()), "case-");
        let index = snapshot.load_cases.len();
        Ok(Emit::mutations(vec![Fem3dMutation::SetLoadCase { index, load_case: FemLoadCase { id, name: payload.name.clone(), loads: Vec::new(), self_weight: payload.self_weight } }]))
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
        /// 📦️ A JSON-encoded `[[caseId, factor], ...]` array — `crate::artifacts::fem3d::FemCombination`'s
        /// `terms` is a `BTreeMap<String, f64>`, not a dedicated record type, so this stays a JSON-string
        /// blob (parsed the same way the pre-migration `handle_action` channel used to) rather than
        /// requiring the DSL engine to grow a `Vec<(String, f64)>` primitive.
        pub terms: String,
    }

    pub fn handle(payload: &AddCombination, doc: &DocumentView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        match serde_json::from_str::<Vec<(String, f64)>>(&payload.terms) {
            Ok(parsed) => {
                let terms: std::collections::BTreeMap<String, f64> = parsed.into_iter().collect();
                let id = crate::app_surface::next_id(snapshot.combinations.iter().map(|c| c.id.clone()), "c");
                let index = snapshot.combinations.len();
                Ok(Emit::mutations(vec![Fem3dMutation::SetCombination { index, combination: crate::artifacts::fem3d::FemCombination { id, name: payload.name.clone(), terms } }]))
            }
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️AddCombination

//#region 🔖️SetSelfWeight
pub mod set_self_weight {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    #[dsl(keyword = "set-self-weight")]
    pub struct SetSelfWeight {
        pub case_id: String,
        pub enabled: bool,
    }

    pub fn handle(payload: &SetSelfWeight, doc: &DocumentView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        match snapshot.load_cases.iter().position(|lc| lc.id == payload.case_id) {
            Some(index) => {
                let mut load_case = snapshot.load_cases[index].clone();
                load_case.self_weight = payload.enabled;
                Ok(Emit::mutations(vec![Fem3dMutation::SetLoadCase { index, load_case }]))
            }
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetSelfWeight

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem3d::testkit::{dispatch, fem3d_app, Fem3dApp};
    use crate::apps::fem3d::Fem3dCommand;

    fn app_with_load_case() -> Fem3dApp {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Dead".into(), self_weight: false }));
        app
    }

    #[test]
    fn resolve_load_case_synthesizes_case_when_none_exist() {
        let snapshot = Fem3dSnapshot::default();
        let (index, load_case) = resolve_load_case(&snapshot, None);
        assert_eq!(index, 0);
        assert_eq!(load_case.id, "case-1");
    }

    #[test]
    fn add_member_udl_action_emits_op_3d() {
        let mut app = app_with_load_case();
        dispatch(&mut app, Fem3dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -2000.0, case_id: None }));
        let snapshot = app.snapshot().expect("snapshot");
        let load_case = &snapshot.load_cases[0];
        assert!(matches!(load_case.loads[0], crate::artifacts::fem3d::FemLoad::MemberUdl { .. }));
    }

    #[test]
    fn add_nodal_load_targets_named_case() {
        let mut app = app_with_load_case();
        dispatch(&mut app, Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }));
        let live_case_id = app.snapshot().expect("snapshot").load_cases[1].id.clone();
        dispatch(&mut app, Fem3dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n2".into(), dof: crate::artifacts::fem3d::FemDof::Tz, value: -5000.0, case_id: Some(live_case_id) }));
        let snapshot = app.snapshot().expect("snapshot");
        assert!(snapshot.load_cases[1].loads.iter().any(|l| matches!(l, crate::artifacts::fem3d::FemLoad::Nodal { .. })));
        assert!(snapshot.load_cases[0].loads.is_empty(), "the untargeted case must stay untouched");
    }

    #[test]
    fn set_self_weight_toggles_existing_case() {
        let mut app = app_with_load_case();
        let case_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();
        dispatch(&mut app, Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id, enabled: true }));
        assert!(app.snapshot().expect("snapshot").load_cases[0].self_weight);
    }

    #[test]
    fn set_self_weight_unknown_case_is_a_no_op() {
        let mut app = app_with_load_case();
        dispatch(&mut app, Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "missing".into(), enabled: true }));
        assert!(!app.snapshot().expect("snapshot").load_cases[0].self_weight);
    }

    #[test]
    fn add_combination_parses_terms_json() {
        let mut app = app_with_load_case();
        dispatch(&mut app, Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: "[[\"case-0\",1.35]]".into() }));
        let snapshot = app.snapshot().expect("snapshot");
        assert_eq!(snapshot.combinations.len(), 1);
        assert_eq!(snapshot.combinations[0].terms.get("case-0"), Some(&1.35));
    }

    #[test]
    fn add_combination_invalid_terms_json_is_a_no_op() {
        let mut app = app_with_load_case();
        dispatch(&mut app, Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: "not json".into() }));
        assert!(app.snapshot().expect("snapshot").combinations.is_empty());
    }
}
// #endregion 🧪️Tests
