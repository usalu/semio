use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Profile;
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::import::ifc::reader::Section;
use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::Import;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::import::ifc::reader::{opt_text, text, Doc, Loop};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::data::label;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::spatial::{authoring_of, phase_of};
#[allow(unused_imports)]
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Axis, Layer, LayerFunction, Point2, SpaceBoundary, TopConstraint, Vertex};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Beam, BeamType, Column, ColumnType, GridLine, Slab, SlabType, Space};
#[allow(unused_imports)]
use semio_s_artifact_stdio_contract::part21::Part21Value;
use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::elements::{profile_of, profile_top};

#[test]
fn sections_become_profiles_and_report_their_top() {
    let rectangle = Section::Rectangle { centre: [0.0, 0.1], width: 0.3, depth: 0.4 };
    assert_eq!(profile_of(&rectangle), Some(Profile::Rectangle { width: 0.3, depth: 0.4 }));
    assert!((profile_top(&rectangle) - 0.3).abs() < 1e-12);
    assert_eq!(profile_of(&Section::Circle { diameter: 0.5 }), Some(Profile::Circle { diameter: 0.5 }));
    assert_eq!(profile_top(&Section::Circle { diameter: 0.5 }), 0.25);
    let outline = Section::Outline { outer: vec![([0.0, 0.0], 0.0), ([1.0, 0.0], 0.0), ([1.0, 2.0], 0.0)], holes: vec![] };
    assert!(matches!(profile_of(&outline), Some(Profile::Custom { outline }) if outline.len() == 3));
    assert_eq!(profile_top(&outline), 2.0);
}

#[test]
fn unused_ids_count_up_from_the_base() {
    assert_eq!(Import::unused("ct", |_| false), "ct");
    assert_eq!(Import::unused("ct", |candidate| candidate == "ct" || candidate == "ct-2"), "ct-3");
}

#[test]
fn a_leaning_column_and_the_arc_inclined_and_joined_beams_come_back_exactly_from_their_records() {
    use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::export_ifc2x3;
    use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::testkit::leaning;
    use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::import_ifc2x3;
    let model = leaning();
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let (back, notes) = import_ifc2x3(&bytes).expect("the file imports");
    assert!(notes.iter().all(|note| !note.contains("IFCBEAM") && !note.contains("IFCCOLUMN")), "{notes:?}");
    assert_eq!(back.columns["c-lean"], model.columns["c-lean"], "the leaning column is restored from its record");
    assert!(back.columns["c-west"].tilt.is_none() && (back.columns["c-west"].position.x - 40.0).abs() < 1e-9, "a plumb column is read from its swept body");
    for id in ["b-arc", "b-incline", "b-joined"] {
        assert_eq!(back.beams[id], model.beams[id], "{id}");
    }
}
