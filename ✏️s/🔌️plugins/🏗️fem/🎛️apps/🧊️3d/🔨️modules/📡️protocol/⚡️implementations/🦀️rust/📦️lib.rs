//! ⚖️ FEM 3D app — binary command protocol surface + laws (constitutional: protocol).

use fem3d_op::Fem3dOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `Fem3dOperation` to its binary command form.
pub fn encode_op(operation: &Fem3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Fem3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Fem3dOperation, protocol::ProtocolError> {
    Fem3dOperation::decode_op(bytes)
}

//#region 🔖️Fem3dCommand
/// 🎯️ B1: `Fem3dPlayApp::Command` — the SOLE dispatch surface for fem3d's own behavior, covering EVERY
/// declared action (the pre-B1 stringly-typed `{action,args}` `handle_action` channel is gone — see
/// `fem3d_ui`'s `Fem3dPlayApp::handle`). Field shapes mirror each action's real `args` object exactly;
/// `AddCombination.terms` stays a JSON-string blob (parsed the same way `handle_action` used to) since
/// `fem3d::FemCombination.terms` is a `BTreeMap<String, f64>`, not a dedicated record type like
/// `fem2d::FemCombinationTerm`, and this crate must not touch the `fem3d` document crate to add one.
/// `#[derive(dsl::DslOps)]` gives this a binary (`OpBinary`) AND text (`OpText`) codec, matching
/// `Fem3dOperation`'s derive/attribute conventions exactly, even though this enum is never dispatched
/// through `store::DocumentCommand` (it is not a `protocol::Operation` — no `diff`/`backwards` — purely
/// a command-channel wire codec).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Fem3dCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "add-node")]
    AddNode { x: f64, y: f64, z: f64 },
    #[dsl(key = "add-bar")]
    AddBar { start: String, end: String, material_id: String, section_id: String },
    #[dsl(key = "add-frame")]
    AddFrame { start: String, end: String, material_id: String, section_id: String, roll: f64 },
    #[dsl(key = "add-material")]
    AddMaterial { name: String, e: f64, g: f64 },
    #[dsl(key = "add-section")]
    AddSection { name: String, area: f64, iy: f64, iz: f64, j: f64 },
    #[dsl(key = "add-support")]
    AddSupport { node_id: String, fixed: Vec<fem3d::FemDof> },
    #[dsl(key = "add-nodal-load")]
    AddNodalLoad { node_id: String, dof: fem3d::FemDof, value: f64, case_id: Option<String> },
    #[dsl(key = "add-member-udl")]
    AddMemberUdl { element_id: String, wx: f64, wy: f64, wz: f64, case_id: Option<String> },
    #[dsl(key = "add-area-load")]
    AddAreaLoad { solid_id: String, pressure: f64, case_id: Option<String> },
    #[dsl(key = "add-solid")]
    AddSolid { x: f64, y: f64, width: f64, depth: f64, height: f64, material_id: String, base_z: Option<f64>, layers: Option<u32>, mesh_size: Option<f64> },
    #[dsl(key = "add-load-case")]
    AddLoadCase { name: String, self_weight: bool },
    #[dsl(key = "add-combination")]
    AddCombination { name: String, terms: String },
    #[dsl(key = "set-self-weight")]
    SetSelfWeight { case_id: String, enabled: bool },
    #[dsl(key = "set-analysis-settings")]
    SetAnalysisSettings { modal_count: Option<u32>, buckling_count: Option<u32>, deformation_scale: Option<f64> },
    #[dsl(key = "remove-selection")]
    RemoveSelection { ids: Vec<String> },
    #[dsl(key = "active-example")]
    SetActiveExample { example_id: String },

    // 👁️ Config-only (was ephemeral `Fem3dPlayApp` `RefCell` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "camera")]
    SetCamera { json: String },
    #[dsl(key = "result-display")]
    SetResultDisplay { source_id: Option<String>, mode: String, mode_index: u32 },
}
//#endregion 🔖️Fem3dCommand

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use fem3d::{FemAnalysisSettings, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport};
    use store::{create_document_envelope, DocumentCommand};

    fn cantilever_fixture() -> fem3d::Fem3dDocument {
        fem3d::Fem3dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: 3.0, y: 0.0, z: 0.0 }],
            elements: vec![FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.00000060 }],
            solids: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() }],
            load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -5000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = Fem3dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn fem3d_document_text_round_trips_through_the_store() {
        let mut store = fem3d_op::Fem3dStore::new(create_document_envelope(fem3d::FEM_3D_SCHEMA, "fem3d", fem3d_engine::empty_fem3d_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Fem3dOperation::SetDocument { document: cantilever_fixture() }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    // #region 🔖️Fem3dCommand
    #[test]
    fn fem3d_command_binary_round_trips_and_agrees_with_text() {
        let command = Fem3dCommand::AddNode { x: 1.0, y: 2.0, z: 3.0 };
        store::test_support::assert_op_text_binary_equivalence(&command);
        let bytes = command.encode_op().expect("encode");
        assert_eq!(Fem3dCommand::decode_op(&bytes).expect("decode"), command);
    }

    #[test]
    fn fem3d_command_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddNode { x: 1.0, y: 2.0, z: 3.0 });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddBar { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddFrame { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddMaterial { name: "Steel".into(), e: 2.1e11, g: 8.077e10 });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.0000006 });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddSupport { node_id: "n1".into(), fixed: FemDof::ALL.to_vec() });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddNodalLoad { node_id: "n1".into(), dof: FemDof::Tz, value: -5000.0, case_id: Some("live".into()) });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddNodalLoad { node_id: "n1".into(), dof: FemDof::Tz, value: -5000.0, case_id: None });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -500.0, case_id: None });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddAreaLoad { solid_id: "sol1".into(), pressure: 5000.0, case_id: Some("dead".into()) });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddSolid { x: 0.0, y: 0.0, width: 4.0, depth: 2.0, height: 0.5, material_id: "concrete".into(), base_z: Some(0.0), layers: Some(2), mesh_size: None });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddLoadCase { name: "Live".into(), self_weight: false });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::AddCombination { name: "ULS".into(), terms: "[[\"dead\",1.35],[\"live\",1.5]]".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::SetSelfWeight { case_id: "dead".into(), enabled: true });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::RemoveSelection { ids: vec!["n1".into(), "e1".into()] });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::SetActiveExample { example_id: "default".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::SetCamera { json: "{\"x\":1}".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dCommand::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 });
    }
    // #endregion 🔖️Fem3dCommand

    // #region 🔖️WireBaseline
    // 🚧️ [DEBUG] temporary wire-format baseline dump for the taxonomy migration — delete after diffing.
    #[test]
    fn wire_baseline_dump() {
        fn dump(label: &str, bytes: &[u8]) {
            println!("{label} | {} | {}", bytes.len(), bytes.iter().map(|b| format!("{b:02x}")).collect::<String>());
        }
        dump("Command::AddNode", &Fem3dCommand::AddNode { x: 1.0, y: 2.0, z: 3.0 }.encode_op().unwrap());
        dump("Command::AddBar", &Fem3dCommand::AddBar { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }.encode_op().unwrap());
        dump("Command::AddFrame", &Fem3dCommand::AddFrame { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 }.encode_op().unwrap());
        dump("Command::AddMaterial", &Fem3dCommand::AddMaterial { name: "Steel".into(), e: 2.1e11, g: 8.077e10 }.encode_op().unwrap());
        dump("Command::AddSection", &Fem3dCommand::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.0000006 }.encode_op().unwrap());
        dump("Command::AddSupport", &Fem3dCommand::AddSupport { node_id: "n1".into(), fixed: FemDof::ALL.to_vec() }.encode_op().unwrap());
        dump("Command::AddNodalLoad", &Fem3dCommand::AddNodalLoad { node_id: "n1".into(), dof: FemDof::Tz, value: -5000.0, case_id: Some("live".into()) }.encode_op().unwrap());
        dump("Command::AddNodalLoad(None)", &Fem3dCommand::AddNodalLoad { node_id: "n1".into(), dof: FemDof::Tz, value: -5000.0, case_id: None }.encode_op().unwrap());
        dump("Command::AddMemberUdl", &Fem3dCommand::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -500.0, case_id: None }.encode_op().unwrap());
        dump("Command::AddAreaLoad", &Fem3dCommand::AddAreaLoad { solid_id: "sol1".into(), pressure: 5000.0, case_id: Some("dead".into()) }.encode_op().unwrap());
        dump("Command::AddSolid", &Fem3dCommand::AddSolid { x: 0.0, y: 0.0, width: 4.0, depth: 2.0, height: 0.5, material_id: "concrete".into(), base_z: Some(0.0), layers: Some(2), mesh_size: None }.encode_op().unwrap());
        dump("Command::AddLoadCase", &Fem3dCommand::AddLoadCase { name: "Live".into(), self_weight: false }.encode_op().unwrap());
        dump("Command::AddCombination", &Fem3dCommand::AddCombination { name: "ULS".into(), terms: "[[\"dead\",1.35],[\"live\",1.5]]".into() }.encode_op().unwrap());
        dump("Command::SetSelfWeight", &Fem3dCommand::SetSelfWeight { case_id: "dead".into(), enabled: true }.encode_op().unwrap());
        dump("Command::SetAnalysisSettings", &Fem3dCommand::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) }.encode_op().unwrap());
        dump("Command::RemoveSelection", &Fem3dCommand::RemoveSelection { ids: vec!["n1".into(), "e1".into()] }.encode_op().unwrap());
        dump("Command::SetActiveExample", &Fem3dCommand::SetActiveExample { example_id: "default".into() }.encode_op().unwrap());
        dump("Command::SetCamera", &Fem3dCommand::SetCamera { json: "{\"x\":1}".into() }.encode_op().unwrap());
        dump("Command::SetResultDisplay", &Fem3dCommand::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 }.encode_op().unwrap());
        dump("Operation::SetAnalysisSettings", &encode_op(&Fem3dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } }).unwrap());
    }
    // #endregion 🔖️WireBaseline
}
// #endregion 🧪️Tests
