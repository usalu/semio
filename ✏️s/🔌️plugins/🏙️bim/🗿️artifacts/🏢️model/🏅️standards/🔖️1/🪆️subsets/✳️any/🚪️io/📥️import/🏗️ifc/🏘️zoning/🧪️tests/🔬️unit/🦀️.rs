use crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3;
use crate::standards::v1::subsets::any::io::export::ifc::zoning::tests::zoned;
use crate::standards::v1::subsets::any::io::import::ifc::import_ifc2x3;
use crate::ModelSnapshot;

fn exported(model: &ModelSnapshot) -> String {
    String::from_utf8(export_ifc2x3(model).expect("the model exports").0).expect("a text file")
}

fn edited(text: &str, marker: &str, change: impl Fn(&str) -> String) -> String {
    text.lines().map(|line| if line.contains(marker) { change(line) } else { line.to_string() }).collect::<Vec<_>>().join("\n")
}

#[test]
fn zones_schemes_memberships_and_finishes_survive_the_round_trip_exactly() {
    let model = zoned();
    let (back, notes) = import_ifc2x3(exported(&model).as_bytes()).expect("the file imports");
    assert_eq!(back.zones, model.zones);
    assert_eq!(back.area_schemes, model.area_schemes);
    for space in ["sp-1", "sp-2"] {
        let (before, after) = (&model.spaces[space], &back.spaces[space]);
        assert_eq!((&after.zone, &after.floor_finish, &after.wall_finish, &after.ceiling_finish), (&before.zone, &before.floor_finish, &before.wall_finish, &before.ceiling_finish), "{space}");
    }
    assert!(notes.iter().all(|note| !note.contains("IFCZONE") && !note.contains("IFCGROUP") && !note.contains("Covering")), "{notes:?}");
}

#[test]
fn export_import_export_of_a_zoned_model_is_byte_stable() {
    let mut model = zoned();
    model.roofs.clear();
    model.stairs.clear();
    model.railings.clear();
    model.curtain_walls.clear();
    model.slabs.remove("sl-balcony");
    model.spaces.remove("sp-1");
    let bedroom = model.spaces.get_mut("sp-2").expect("the bedroom");
    (bedroom.wall_finish, bedroom.ceiling_finish) = (Some("m-brick".into()), Some("m-conc".into()));
    let first = exported(&model);
    let (back, _) = import_ifc2x3(first.as_bytes()).expect("the import");
    assert_eq!(first, exported(&back));
}

#[test]
fn a_foreign_zone_without_the_authoring_rows_keeps_its_category_and_starts_without_occupancy() {
    let text = edited(&exported(&zoned()), "'OccupancyDensity'", |line| line.replace("'OccupancyDensity'", "'Density'"));
    let (back, _) = import_ifc2x3(text.as_bytes()).expect("the file imports");
    assert_eq!(back.zones["z-day"].category, "Ventilation");
    assert!(back.zones.values().all(|zone| zone.occupancy_density == 0.0));
    assert_eq!(back.spaces["sp-1"].zone.as_deref(), Some("z-day"), "the membership is the group assignment, not the density");
}

#[test]
fn a_foreign_covering_names_its_material_by_name_when_the_authoring_ids_are_missing() {
    let text = edited(&exported(&zoned()), "'FloorFinish'", |line| line.replace("'FloorFinish'", "'Floor'"));
    let (back, notes) = import_ifc2x3(text.as_bytes()).expect("the file imports");
    assert_eq!(back.spaces["sp-1"].floor_finish.as_deref(), Some("m-wood"), "Oak is found again by its name");
    assert!(notes.iter().all(|note| !note.contains("FloorCovering")), "{notes:?}");
}

#[test]
fn a_covering_that_names_no_material_is_reported_and_leaves_the_surface_unfinished() {
    let text = edited(&exported(&zoned()), "'FloorFinish'", |line| line.replace("'FloorFinish'", "'Floor'"));
    let text = edited(&text, "'FloorCovering'", |line| line.replace("'Oak'", "'Ghost'"));
    let (back, notes) = import_ifc2x3(text.as_bytes()).expect("the file imports");
    assert_eq!(back.spaces["sp-1"].floor_finish, None);
    assert!(notes.iter().any(|note| note.contains("IFCSPACE") && note.contains("FloorCovering names no material")), "{notes:?}");
}

#[test]
fn an_area_scheme_counting_an_unknown_zone_drops_it_and_a_scheme_stays_a_scheme() {
    let text = edited(&exported(&zoned()), "z-day\"]", |line| line.replace("z-day\"]", "z-ghost\"]"));
    let (back, _) = import_ifc2x3(text.as_bytes()).expect("the file imports");
    assert!(back.area_schemes["as-day"].zones.is_empty(), "{:?}", back.area_schemes["as-day"]);
    assert_eq!(back.area_schemes.len(), 2);
    assert_eq!(back.zones.len(), 3);
}

#[test]
fn a_model_without_zoning_imports_without_zoning() {
    let mut model = zoned();
    model.zones.clear();
    model.area_schemes.clear();
    for space in model.spaces.values_mut() {
        (space.zone, space.floor_finish, space.wall_finish, space.ceiling_finish) = (None, None, None, None);
    }
    let (back, _) = import_ifc2x3(exported(&model).as_bytes()).expect("the file imports");
    assert!(back.zones.is_empty() && back.area_schemes.is_empty());
    assert!(back.spaces.values().all(|space| space.zone.is_none() && space.floor_finish.is_none()));
}
