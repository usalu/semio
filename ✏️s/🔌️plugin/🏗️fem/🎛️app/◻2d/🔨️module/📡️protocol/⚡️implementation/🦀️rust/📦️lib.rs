//! ⚖️ FEM 2D app — binary command protocol surface + laws (constitutional: protocol).

use fem2d_op::Fem2dOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `Fem2dOperation` to its binary command form.
pub fn encode_op(operation: &Fem2dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Fem2dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Fem2dOperation, protocol::ProtocolError> {
    Fem2dOperation::decode_op(bytes)
}

//#region 🔖️Fem2dCommand
/// 🎯️ B1: `Fem2dPlayApp::Command` — the SOLE dispatch surface for fem2d's own behavior, now covering
/// EVERY declared action (the pre-B1 stringly-typed `{action,args}` `handle_action` channel is gone —
/// see `fem2d_ui`'s `Fem2dPlayApp::handle`). Field shapes mirror each action's real `args` object
/// exactly, with one deliberate upgrade: `AddCombination.terms` moves from a JSON-string blob to the
/// document's own `fem2d::FemCombinationTerm` record type, now that a typed channel makes that
/// possible. `#[derive(dsl::DslOps)]` gives this a binary (`OpBinary`) AND text (`OpText`) codec,
/// matching `Fem2dOperation`'s derive/attribute conventions exactly, even though this enum is never
/// dispatched through `store::DocumentCommand` (it is not a `protocol::Operation` — no
/// `diff`/`backwards` — purely a command-channel wire codec).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Fem2dCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "add-node")]
    AddNode { x: f64, y: f64 },
    #[dsl(key = "add-bar")]
    AddBar { start: String, end: String, material_id: String, section_id: String },
    #[dsl(key = "add-beam")]
    AddBeam { start: String, end: String, material_id: String, section_id: String },
    #[dsl(key = "add-material")]
    AddMaterial { name: String, e: f64 },
    #[dsl(key = "add-section")]
    AddSection { name: String, area: f64, iy: f64 },
    #[dsl(key = "add-support")]
    AddSupport { node_id: String, fixed: Vec<fem2d::FemDof> },
    #[dsl(key = "add-nodal-load")]
    AddNodalLoad { node_id: String, dof: fem2d::FemDof, value: f64, case_id: Option<String> },
    #[dsl(key = "add-member-udl")]
    AddMemberUdl { element_id: String, wx: f64, wy: f64, case_id: Option<String> },
    #[dsl(key = "add-area-load")]
    AddAreaLoad { region_id: String, pressure: f64, case_id: Option<String> },
    #[dsl(key = "add-region")]
    AddRegion { x: f64, y: f64, width: f64, height: f64, material_id: String, thickness: Option<f64>, mesh_size: Option<f64> },
    #[dsl(key = "add-load-case")]
    AddLoadCase { name: String, self_weight: bool },
    #[dsl(key = "add-combination")]
    AddCombination { name: String, terms: Vec<fem2d::FemCombinationTerm> },
    #[dsl(key = "set-self-weight")]
    SetSelfWeight { case_id: String, enabled: bool },
    #[dsl(key = "set-analysis-settings")]
    SetAnalysisSettings { modal_count: Option<u32>, buckling_count: Option<u32>, deformation_scale: Option<f64> },
    #[dsl(key = "remove-selection")]
    RemoveSelection { ids: Vec<String> },
    #[dsl(key = "active-example")]
    SetActiveExample { example_id: String },

    // 👁️ Config-only (was ephemeral `Fem2dPlayApp` `RefCell` state / the deleted `ViewState`) — emit
    // `config_operations`, never document operations.
    #[dsl(key = "camera")]
    SetCamera { x: f64, y: f64, zoom: f64 },
    #[dsl(key = "result-display")]
    SetResultDisplay { source_id: Option<String>, mode: String, mode_index: u32 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}
//#endregion 🔖️Fem2dCommand

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use fem2d::{FemAnalysisSettings, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport};
    use store::{create_document_envelope, DocumentCommand};

    fn simply_supported_beam_doc() -> fem2d::Fem2dDocument {
        fem2d::Fem2dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
            elements: vec![FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = Fem2dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn fem2d_document_text_round_trips_through_the_store() {
        let mut store = fem2d_op::Fem2dStore::new(create_document_envelope(fem2d::FEM_2D_SCHEMA, "fem2d", fem2d_engine::empty_fem2d_projection(), None));
        store
            .dispatch(DocumentCommand::Apply { operations: vec![Fem2dOperation::SetDocument { document: simply_supported_beam_doc() }], description: None })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    // #region 🔖️Fem2dCommand
    #[test]
    fn fem2d_command_binary_round_trips_and_agrees_with_text() {
        let command = Fem2dCommand::AddNode { x: 1.0, y: 2.0 };
        store::test_support::assert_op_text_binary_equivalence(&command);
        let bytes = command.encode_op().expect("encode");
        assert_eq!(Fem2dCommand::decode_op(&bytes).expect("decode"), command);
    }

    #[test]
    fn fem2d_command_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddNode { x: 1.0, y: 2.0 });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddBar { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddMaterial { name: "Steel".into(), e: 2.1e11 });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369 });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddSupport { node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddNodalLoad { node_id: "n1".into(), dof: FemDof::Ty, value: -5000.0, case_id: Some("live".into()) });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddNodalLoad { node_id: "n1".into(), dof: FemDof::Ty, value: -5000.0, case_id: None });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some("dead".into()) });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: Some(0.02), mesh_size: None });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddLoadCase { name: "Live".into(), self_weight: false });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::AddCombination {
            name: "ULS".into(),
            terms: vec![fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, fem2d::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }],
        });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::SetSelfWeight { case_id: "dead".into(), enabled: true });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::RemoveSelection { ids: vec!["n1".into(), "e1".into()] });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::SetActiveExample { example_id: "default".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 });
        store::test_support::assert_op_line_round_trip(&Fem2dCommand::SetLocale { value: "de-DE".into() });
    }
    // #endregion 🔖️Fem2dCommand
}
// #endregion 🧪️Tests
