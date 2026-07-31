//! 📜️ Cad app — textual document grammar surface + laws (constitutional: dsl).

use cad_document::CadScene;

/// 📄️ The `default` example scene, handcrafted in the `.cad` DSL — a small structural column with
/// a two-vertex/one-edge/one-wire/one-face/one-shell/one-solid brep, a site-photo reference, and
/// objects across the shape/building/structure-classic panes.
pub const CAD_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/📚️example/📐️default.cad");

/// 📖️ Parses `.cad` DSL text into a `CadScene`.
pub fn parse_dsl(text: &str) -> Result<CadScene, store::TextError> {
    <CadScene as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `CadScene` back to `.cad` DSL text.
pub fn print_dsl(document: &CadScene) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cad_document::{empty_cad_projection, CadEdge, CadEdgeCurve, CadFace, CadGeometry, CadNode, CadObject, CadPaneId, CadPlaneSurface, CadPrimitiveSlot, CadReference, CadShell, CadSolid, CadVertex, CadWire};
    use serde_json::json;

    fn sample_object(id: &str) -> CadObject {
        CadObject {
            id: id.into(),
            label: "Box".into(),
            typology: "spatial.shape.box".into(),
            visible: true,
            locked: false,
            origin: [1.0, 2.0, 3.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: Some([1.0, 1.0, 1.0]),
            mesh_url: Some("https://example.test/mesh.glb".into()),
            extent: Some([2.0, 2.0, 2.0]),
            solid_handle: Some("solid-1".into()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: "solid-1".into(), kind: "solid".into() }],
        }
    }

    fn sample_geometry() -> CadGeometry {
        CadGeometry {
            anchors: vec![json!({ "id": "anchor-1", "position": [0.0, 0.0, 0.0] })],
            vertices: vec![CadVertex { id: "v1".into(), position: [0.0, 0.0, 0.0] }, CadVertex { id: "v2".into(), position: [1.0, 0.0, 0.0] }],
            edges: vec![CadEdge { id: "e1".into(), vertex_ids: vec!["v1".into(), "v2".into()], curve: CadEdgeCurve { kind: "line".into() } }],
            wires: vec![CadWire { id: "w1".into(), edge_ids: vec!["e1".into()] }],
            faces: vec![CadFace { id: "f1".into(), wire_ids: vec!["w1".into()], surface: CadPlaneSurface { kind: "plane".into(), origin: [0.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0] } }],
            shells: vec![CadShell { id: "s1".into(), face_ids: vec!["f1".into()] }],
            solids: vec![CadSolid { id: "sol1".into(), shell_ids: vec!["s1".into()] }],
        }
    }

    fn sample_reference() -> CadReference {
        CadReference {
            id: "ref-1".into(),
            source_url: "https://example.test/plan.png".into(),
            media_kind: "image".into(),
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: Some(1.5),
            width_world: 8.0,
            hidden: false,
            locked: true,
            opacity: Some(0.8),
        }
    }

    fn sample_scene() -> CadScene {
        let mut scene = empty_cad_projection();
        scene.objects.push(sample_object("object-1"));
        scene.building_objects.push(sample_object("object-2"));
        scene.nodes.push(CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() });
        scene.shape_geometry = Some(sample_geometry());
        scene.references_by_model_definition_id.insert(CadPaneId::Shape.model_definition_id().to_string(), vec![sample_reference()]);
        scene.active_model_definition_id = CadPaneId::Shape.model_definition_id().to_string();
        scene
    }

    #[test]
    fn default_example_dsl_round_trips() {
        let document = parse_dsl(CAD_DEFAULT_EXAMPLE_TEXT).expect("parse default .cad example");
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn cad_scene_round_trips_through_dsl_document() {
        store::test_support::assert_dsl_round_trip(&sample_scene());
    }

    #[test]
    fn cad_scene_with_all_geometry_panes_round_trips_through_dsl_document() {
        let mut scene = sample_scene();
        scene.building_geometry = Some(sample_geometry());
        scene.energy_geometry = Some(sample_geometry());
        scene.structure_classic_geometry = Some(sample_geometry());
        store::test_support::assert_dsl_round_trip(&scene);
    }
}
//#endregion 🧪️Tests
