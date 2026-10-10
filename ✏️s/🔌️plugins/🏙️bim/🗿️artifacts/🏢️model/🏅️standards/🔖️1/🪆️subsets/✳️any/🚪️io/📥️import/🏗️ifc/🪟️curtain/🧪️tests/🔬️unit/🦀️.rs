use super::*;
use crate::standards::v1::subsets::any::io::export::ifc::curtain::tests::framed;
use crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3;
use crate::standards::v1::subsets::any::io::import::ifc::import_ifc2x3;

#[test]
fn a_curtain_wall_with_its_type_and_overrides_survives_the_round_trip_exactly() {
    let model = framed();
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let (back, notes) = import_ifc2x3(&bytes).expect("the file imports");
    assert!(notes.iter().all(|note| !note.contains("IFCCURTAINWALL") && !note.contains("IFCDOOR") && !note.contains("IFCMEMBER") && !note.contains("IFCPLATE")), "{notes:?}");
    assert_eq!(back.curtain_walls, model.curtain_walls);
    assert_eq!(back.curtain_wall_types, model.curtain_wall_types);
    assert_eq!(back.curtain_panel_overrides, model.curtain_panel_overrides);
}

#[test]
fn a_foreign_curtain_wall_without_the_authored_record_is_reported_not_imported() {
    let (bytes, _) = export_ifc2x3(&framed()).expect("the model exports");
    let text = String::from_utf8(bytes).expect("a text file").replace("'CurtainWall'", "'Facade'");
    let (back, notes) = import_ifc2x3(text.as_bytes()).expect("the file imports");
    assert!(back.curtain_walls.is_empty() && back.curtain_panel_overrides.is_empty());
    assert!(notes.iter().any(|note| note.contains("IFCCURTAINWALL") && note.contains("no authored curtain wall record")), "{notes:?}");
}

#[test]
fn a_curtain_wall_outside_a_storey_is_skipped() {
    let mut model = framed();
    model.curtain_walls.get_mut("cw-1").expect("the wall").storey = "st-ghost".into();
    let exported = export_ifc2x3(&model).expect("the model exports").0;
    let (back, _) = import_ifc2x3(&exported).expect("the file imports");
    assert!(back.curtain_walls.is_empty());
}
