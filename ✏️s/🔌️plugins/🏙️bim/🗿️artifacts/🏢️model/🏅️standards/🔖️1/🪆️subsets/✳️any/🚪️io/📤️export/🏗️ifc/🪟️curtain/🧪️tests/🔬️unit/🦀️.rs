#[allow(unused_imports)]
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::io::export::ifc::brep::{brep_definition, mesh_where};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::label;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::en;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::context::{Export, Quantity};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::curtain_walls::filler_cell;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::parts;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::OpeningKind;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet};
use semio_s_artifact_bim_model_family_tests::standards::v1::subsets::any::io::export::ifc::curtain::{WALL_ROW, TYPE_ID_ROW, TYPE_ROW, OVERRIDES_ROW};
use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, framed, house, real, rows, string};

#[test]
fn a_curtain_wall_aggregates_its_mullions_and_its_panels() {
    let document = document(&house());
    let (wall, args) = rows(&document, "IFCCURTAINWALL").into_iter().next().expect("a curtain wall");
    assert_eq!(string(args, 7).as_deref(), Some("cw-1"));
    let (_, aggregate) = rows(&document, "IFCRELAGGREGATES").into_iter().find(|(_, relation)| relation[4].as_ref_id() == Some(wall.id)).expect("its parts");
    let kinds: Vec<String> = aggregate[5].as_list().expect("parts").iter().filter_map(|part| document.resolve(part)).filter_map(|instance| instance.primary().map(|(name, _)| name.to_string())).collect();
    assert_eq!(kinds, ["IFCMEMBER", "IFCPLATE"]);
    assert_eq!(count(&document, "IFCMEMBER"), 1);
    assert_eq!(count(&document, "IFCPLATE"), 1);
}

#[test]
fn the_curtain_wall_reports_its_length_height_and_side_area() {
    let document = document(&house());
    let (_, quantity) = rows(&document, "IFCELEMENTQUANTITY").into_iter().find(|(_, args)| string(args, 2).as_deref() == Some("Qto_CurtainWallQuantities")).expect("the curtain wall quantities");
    let values: Vec<f64> = quantity[5].as_list().expect("quantities").iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entities.first().map(|(_, args)| real(args, 3))).collect();
    assert_eq!(values.len(), 3);
    assert!((values[0] - 6.0).abs() < 1e-9 && (values[1] - 2.8).abs() < 1e-9 && (values[2] - 6.0 * 2.8).abs() < 1e-9);
}

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::CurtainPanel;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{CurtainPanelOverride, ModelSnapshot};

#[test]
fn a_door_panel_is_an_ifc_door_of_the_curtain_wall_and_an_open_cell_adds_nothing() {
    let document = document(&framed());
    let (wall, _) = rows(&document, "IFCCURTAINWALL").into_iter().next().expect("a curtain wall");
    let (_, aggregate) = rows(&document, "IFCRELAGGREGATES").into_iter().find(|(_, relation)| relation[4].as_ref_id() == Some(wall.id)).expect("its parts");
    let kinds: Vec<String> = aggregate[5].as_list().expect("parts").iter().filter_map(|part| document.resolve(part)).filter_map(|instance| instance.primary().map(|(name, _)| name.to_string())).collect();
    assert_eq!(kinds, ["IFCMEMBER", "IFCPLATE", "IFCDOOR"]);
}

#[test]
fn the_authored_records_travel_as_rows_of_the_authoring_set() {
    let document = document(&framed());
    let names: Vec<String> = rows(&document, "IFCPROPERTYSINGLEVALUE").into_iter().filter_map(|(_, args)| string(args, 0)).collect();
    for row in [WALL_ROW, TYPE_ID_ROW, TYPE_ROW, OVERRIDES_ROW] {
        assert!(names.iter().any(|name| name == row), "{row} is written: {names:?}");
    }
}
