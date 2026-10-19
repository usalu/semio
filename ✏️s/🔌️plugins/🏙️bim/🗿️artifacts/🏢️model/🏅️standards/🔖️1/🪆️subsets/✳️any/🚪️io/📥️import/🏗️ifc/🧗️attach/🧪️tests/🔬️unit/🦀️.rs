use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::export_ifc2x3;
use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::testkit::{attic, house, with_depth};
use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::import_ifc2x3;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Axis, Layer, LayerFunction, LocationLine, OpeningKind, Phase, Point2, Profile, Slope, TopConstraint, Vertex, WallSide};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Ceiling, CeilingType, ModelSnapshot, Opening, Slab, Wall, WallSweep};

fn vertex(x: f64, y: f64) -> Vertex {
    Vertex { point: Point2 { x, y }, bulge: 0.0 }
}

#[test]
fn walls_under_ceilings_and_on_slabs_get_their_attach_back_exactly() {
    let model = with_depth();
    let (bytes, written) = export_ifc2x3(&model).expect("the model exports");
    assert!(written.is_empty(), "{written:?}");
    let (back, notes) = import_ifc2x3(&bytes).expect("the file imports");
    assert!(notes.iter().all(|note| !note.contains("target") && !note.contains("base slab")), "{notes:?}");
    for id in ["w-ceil", "w-raked", "w-foot"] {
        assert_eq!(back.walls[id], model.walls[id], "{id}");
    }
}

#[test]
fn export_import_export_of_attached_walls_sweeps_and_reveals_is_byte_stable() {
    let (first, _) = export_ifc2x3(&with_depth()).expect("first export");
    let (back, _) = import_ifc2x3(&first).expect("the import");
    let (second, _) = export_ifc2x3(&back).expect("second export");
    assert_eq!(first, second);
}

#[test]
fn a_wall_under_a_roof_follows_the_imported_roof_and_a_base_slab_that_is_not_imported_leaves_the_base_free() {
    let model = attic();
    let (bytes, _) = export_ifc2x3(&model).expect("the attic exports");
    let (back, notes) = import_ifc2x3(&bytes).expect("the file imports");
    assert!(!back.roofs.is_empty(), "the roofs travel in the file");
    for id in ["w-hip", "w-east", "w-west", "w-south", "w-north"] {
        assert_eq!(back.walls[id].top, model.walls[id].top, "{id}");
    }
    assert!(notes.iter().all(|note| !note.contains("Roof target")), "{notes:?}");
    let layout = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::try_with_inference(None, &model, |inferred| inferred.wall_layout["w-hip"].clone()).expect("the attic infers");
    assert!(layout.height > 0.0);
    if back.slabs.contains_key("sl-slope") {
        assert_eq!(back.walls["w-base"].base_slab.as_deref(), Some("sl-slope"));
    } else {
        assert!(notes.iter().any(|note| note.contains("base slab sl-slope")), "{notes:?}");
        assert_eq!((back.walls["w-base"].base_slab.clone(), back.walls["w-base"].base_offset), (None, 0.0));
    }
}
