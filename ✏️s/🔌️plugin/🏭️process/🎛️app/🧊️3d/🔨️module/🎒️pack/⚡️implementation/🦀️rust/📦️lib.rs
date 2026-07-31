//! 📦️ Process 3d app — binary document surface + laws (constitutional: pack).

use process_3d::Process3dDocument;
use store::PackError;

/// 📦️ Encodes a `Process3dDocument` to its binary pack form.
pub fn encode(document: &Process3dDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `Process3dDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Process3dDocument, PackError> {
    <Process3dDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use process_3d::{empty_process3d_projection, Pose, ProcessMeasure, ProcessStep, SolidSpec, Stock, StepOrigin};

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn drill_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Drill".into(), enabled: true, origin: Some(StepOrigin { module_id: "wood".into(), machine_id: "circularSaw".into(), modification_kind_id: "crosscut".into() }), measure: ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() } }
    }

    fn attach_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Attach".into(), enabled: false, origin: None, measure: ProcessMeasure::Attach { component: SolidSpec::Sphere { radius: 0.05 }, pose: Pose { position: [0.1, -0.2, 0.3], axis: [0.0, 1.0, 0.0], angle: 1.2 } } }
    }

    fn imported_mesh_stock() -> Stock {
        Stock { id: "stock".into(), label: "Imported GLB".into(), solid: SolidSpec::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() }, pose: Pose::default() }
    }

    /// 📜️ A document exercising every `SolidSpec`/`ProcessMeasure` shape and both `origin` states, so
    /// the pack round trip covers the full grammar, not just the happy path.
    fn sample_document() -> Process3dDocument {
        Process3dDocument {
            stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: SolidSpec::Box { width: 2.4, depth: 0.12, height: 0.24 }, pose: Pose { position: [0.0, 0.0, 0.12], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
            steps: vec![cut_step("cut-1"), drill_step("drill-1"), attach_step("attach-1")],
            resolved_up_to: Some(2),
        }
    }

    #[test]
    fn process3d_pack_round_trips() {
        store::test_support::assert_dsl_pack_equivalence(&sample_document());
        store::test_support::assert_dsl_pack_equivalence(&empty_process3d_projection());
        let bytes = encode(&sample_document());
        assert_eq!(decode(&bytes).expect("decode"), sample_document());
    }

    #[test]
    fn process3d_pack_round_trips_imported_solid_shapes() {
        let mut document = sample_document();
        document.stock = imported_mesh_stock();
        document.steps.push(ProcessStep { id: "imported-tool".into(), label: "Imported Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::ImportedSolid { solid_handle: "solid-7".into() }, pose: Pose::default() } });
        store::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[test]
    fn process3d_pack_round_trips_with_no_resolved_cursor() {
        let mut document = sample_document();
        document.resolved_up_to = None;
        store::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[test]
    fn timber_example_fixture_pack_round_trips() {
        let document = process_3d_dsl::parse_dsl(process_3d_dsl::PROCESS_3D_TIMBER_EXAMPLE_TEXT).expect("parse timber example");
        store::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[test]
    fn drilled_plate_example_fixture_pack_round_trips() {
        let document = process_3d_dsl::parse_dsl(process_3d_dsl::PROCESS_3D_PLATE_EXAMPLE_TEXT).expect("parse drilled plate example");
        store::test_support::assert_dsl_pack_equivalence(&document);
    }
}
//#endregion 🧪️Tests
