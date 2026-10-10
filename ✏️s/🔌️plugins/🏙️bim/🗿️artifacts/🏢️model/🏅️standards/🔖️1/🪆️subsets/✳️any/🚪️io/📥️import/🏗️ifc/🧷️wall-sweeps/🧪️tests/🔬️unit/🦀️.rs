use crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::attic;
use crate::standards::v1::subsets::any::io::import::ifc::attach::tests::with_depth;
use crate::standards::v1::subsets::any::io::import::ifc::import_ifc2x3;

#[test]
fn the_runs_of_every_sweep_merge_into_the_sweep_that_was_written() {
    let model = attic();
    let (bytes, _) = export_ifc2x3(&model).expect("the attic exports");
    let (back, notes) = import_ifc2x3(&bytes).expect("the file imports");
    assert!(notes.iter().all(|note| !note.starts_with("IFCMEMBER")), "{notes:?}");
    assert_eq!(back.wall_sweeps, model.wall_sweeps, "the two runs of the baseboard cut by the door are one sweep again");
}

#[test]
fn sweeps_on_walls_under_ceilings_survive_the_round_trip() {
    let model = with_depth();
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let (back, _) = import_ifc2x3(&bytes).expect("the file imports");
    assert_eq!(back.wall_sweeps, model.wall_sweeps);
    assert_eq!(back.openings["o-rev"], model.openings["o-rev"], "the reveal depth and material come back with the opening");
}

#[test]
fn a_sweep_with_an_unreadable_profile_is_skipped_with_a_note() {
    let (bytes, _) = export_ifc2x3(&attic()).expect("the attic exports");
    let spoiled = String::from_utf8(bytes).expect("a text file").replace("\"Rectangle\"", "\"Pentagon\"");
    let (back, notes) = import_ifc2x3(spoiled.as_bytes()).expect("the file imports");
    assert!(back.wall_sweeps.is_empty());
    assert!(notes.iter().any(|note| note.contains("authored profile is not valid")), "{notes:?}");
}

#[test]
fn a_member_that_is_no_sweep_is_left_to_the_unsupported_class_note() {
    let model = attic();
    let (bytes, _) = export_ifc2x3(&model).expect("the attic exports");
    let spoiled = String::from_utf8(bytes).expect("a text file").replace("'Semio_WallSweep'", "'Another_Set'");
    let (back, notes) = import_ifc2x3(spoiled.as_bytes()).expect("the file imports");
    assert!(back.wall_sweeps.is_empty());
    assert!(notes.iter().any(|note| note.starts_with("IFCMEMBER: 4 not imported")), "{notes:?}");
}
