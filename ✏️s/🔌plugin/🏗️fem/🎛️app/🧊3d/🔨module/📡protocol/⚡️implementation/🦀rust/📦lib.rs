//! ⚖️ FEM 3D app — binary command protocol surface + laws (constitutional: protocol).

use fem3d_op::Fem3dOperation;
use protocol::OpBinary;

/// 📦 Encodes a `Fem3dOperation` to its binary command form.
pub fn encode_op(operation: &Fem3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `Fem3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Fem3dOperation, protocol::ProtocolError> {
    Fem3dOperation::decode_op(bytes)
}

// #region 🧪Tests
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
}
// #endregion 🧪Tests
