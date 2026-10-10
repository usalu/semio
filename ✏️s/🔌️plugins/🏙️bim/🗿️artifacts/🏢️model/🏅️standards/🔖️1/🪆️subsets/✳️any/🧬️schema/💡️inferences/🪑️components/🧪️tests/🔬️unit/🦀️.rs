use super::*;
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const ROOM: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪑️components/🏠️room/📸️snapshot/🔣️.json");

fn room() -> ModelSnapshot {
    from_json_str(ROOM, JsonMemberPolicy::Reject).expect("the room decodes")
}

fn inferred() -> ModelInference {
    ModelInference::infer(&room()).expect("infers")
}

fn close(got: f64, want: f64) {
    assert!((got - want).abs() <= 1e-9 * want.abs().max(1.0), "{got} vs {want}");
}

fn corners(value: &ComponentValue) -> Vec<(f64, f64)> {
    let mut rounded: Vec<(f64, f64)> = value.footprint.iter().map(|corner| ((corner.x * 1e9).round() / 1e9 + 0.0, (corner.y * 1e9).round() / 1e9 + 0.0)).collect();
    rounded.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    rounded
}

fn codes(value: &ComponentValue) -> Vec<ComponentIssueCode> {
    value.issues.iter().map(|issue| issue.code).collect()
}

#[test]
fn a_free_standing_component_is_its_family_turned_about_its_origin() {
    let inference = inferred();
    let table = &inference.components["c-table"];
    assert_eq!((table.placement.x, table.placement.y, table.placement.z), (2.0, 2.0, 0.0));
    close(table.placement.yaw, std::f64::consts::PI / 6.0);
    close(table.footprint_area, 1.6 * 0.8);
    close(table.volume, 1.6 * 0.8 * 0.74);
    assert_eq!(table.category, Some(FamilyCategory::Furniture));
    assert!(table.issues.is_empty() && table.connector.is_none() && table.overridden.is_empty());
    let (sin, cos) = (std::f64::consts::PI / 6.0).sin_cos();
    let want = [(2.0, 2.0), (2.0 + 1.6 * cos, 2.0 + 1.6 * sin), (2.0 + 1.6 * cos - 0.8 * sin, 2.0 + 1.6 * sin + 0.8 * cos), (2.0 - 0.8 * sin, 2.0 + 0.8 * cos)];
    for corner in want {
        assert!(table.footprint.iter().any(|at| (at.x - corner.0).abs() < 1e-9 && (at.y - corner.1).abs() < 1e-9), "{corner:?} in {:?}", table.footprint);
    }
    assert!(signed_area(&table.footprint) > 0.0, "counter-clockwise");
    close(table.bounds.min.z, 0.0);
    close(table.bounds.max.z, 0.74);
}

#[test]
fn an_override_replaces_the_formula_of_one_instance_and_leaves_the_family_alone() {
    let inference = inferred();
    let wide = &inference.components["c-table-wide"];
    close(wide.footprint_area, 1.8 * 0.8);
    assert_eq!(wide.overridden, ["width"]);
    close(wide.volume, 1.8 * 0.8 * 0.74);
    let plain = &inference.components["c-table"];
    close(plain.volume, 1.6 * 0.8 * 0.74);
    assert_eq!(inference.families["fam-table"].value("width"), Some(&families::ParameterValue::Length { value: 1.6 }));
    assert!(matches!(wide.parameters["width"].value, Some(families::ParameterValue::Length { value }) if (value - 1.8).abs() < 1e-12));
    assert_eq!(wide.parameters["width"].formula, "1.8 m");
    let high = &inference.components["c-chair-high"];
    close(high.bounds.max.z, 0.55 + 0.05 + 0.45);
}

#[test]
fn a_mirrored_component_flips_the_local_x_and_stays_outward() {
    let inference = inferred();
    let chair = &inference.components["c-chair"];
    assert!(chair.placement.mirrored);
    close(chair.footprint_area, 0.45 * 0.45);
    assert_eq!(corners(chair).len(), 4);
    let (min_x, max_x) = (corners(chair).iter().map(|c| c.0).fold(f64::INFINITY, f64::min), corners(chair).iter().map(|c| c.0).fold(f64::NEG_INFINITY, f64::max));
    close(min_x, 2.0);
    close(max_x, 2.45);
    assert!(inference.element_solids["c-chair"].volume > 0.0, "a mirror reverses the winding, the volume stays positive");
    close(inference.element_solids["c-chair"].volume, 0.45 * 0.45 * 0.05 + 0.45 * 0.05 * 0.45);
}

#[test]
fn a_hosted_component_clings_to_the_face_on_the_side_of_its_position() {
    let inference = inferred();
    let south = &inference.components["c-basin-south"];
    let fit = south.placement.host.as_ref().expect("hosted");
    assert_eq!(fit.wall, "w-south");
    close(fit.station, 3.0);
    assert_eq!(fit.side, 1.0);
    close(fit.face.x, 3.0);
    close(fit.face.y, 0.1);
    close(fit.normal.x, 0.0);
    close(fit.normal.y, 1.0);
    close(south.placement.yaw, 0.0);
    close(south.placement.z, 0.85);
    assert_eq!(corners(south), [(3.0, 0.1), (3.0, 0.55), (3.6, 0.1), (3.6, 0.55)]);
    let north = &inference.components["c-basin-north"];
    let fit = north.placement.host.as_ref().expect("hosted");
    close(fit.face.y, 3.9);
    close(fit.normal.y, -1.0);
    assert_eq!(corners(north), [(3.0, 3.45), (3.0, 3.9), (3.6, 3.45), (3.6, 3.9)], "the back plane lies on the face, the basin points into the room");
    let niche = &inference.components["c-niche"];
    let fit = niche.placement.host.as_ref().expect("hosted");
    close(fit.face.x, 5.9);
    close(fit.normal.x, -1.0);
    close(niche.placement.yaw, (-fit.normal.x).atan2(fit.normal.y) + 0.1);
}

#[test]
fn the_position_of_a_hosted_component_only_chooses_the_side_and_the_station() {
    let mut snapshot = room();
    snapshot.components.get_mut("c-basin-south").expect("basin").position = Point2 { x: 3.0, y: 2.5 };
    let moved = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(moved.components["c-basin-south"].placement.host, inferred().components["c-basin-south"].placement.host);
    snapshot.components.get_mut("c-basin-south").expect("basin").position = Point2 { x: 3.0, y: -2.0 };
    let outside = ModelInference::infer(&snapshot).expect("infers");
    let fit = outside.components["c-basin-south"].placement.host.as_ref().expect("hosted");
    assert_eq!(fit.side, -1.0);
    close(fit.face.y, -0.1);
    close(fit.normal.y, -1.0);
}

#[test]
fn a_terminal_has_a_connector_at_its_origin_in_the_colour_of_its_system() {
    let inference = inferred();
    let lamp = inference.components["c-lamp"].connector.as_ref().expect("terminal");
    assert_eq!(lamp.system, MepSystem::Lighting);
    assert_eq!(lamp.colour, "#ffbf00");
    assert_eq!((lamp.position.x, lamp.position.y, lamp.position.z), (3.0, 2.0, 2.9));
    assert_eq!(inference.components["c-diffuser-ok"].connector.as_ref().map(|c| c.colour.as_str()), Some("#1f77d4"));
    assert!(inference.components["c-table"].connector.is_none());
}

#[test]
fn the_faults_of_a_placement_are_typed_issues_and_the_component_has_no_geometry() {
    let inference = inferred();
    assert_eq!(codes(&inference.components["c-ghost"]), [ComponentIssueCode::FamilyMissing]);
    assert_eq!(codes(&inference.components["c-profile"]), [ComponentIssueCode::FamilyProfile]);
    assert_eq!(codes(&inference.components["c-nowhere"]), [ComponentIssueCode::HostMissing]);
    for id in ["c-ghost", "c-profile"] {
        assert!(!inference.components[id].solid(), "{id}");
        assert!(inference.element_solids.get(id).is_none_or(|solid| solid.is_empty()));
    }
    assert!(inference.components["c-nowhere"].placement.host.is_none(), "an unusable host falls back to the authored position");
    let bad = &inference.components["c-bad-override"];
    assert!(codes(bad).len() >= 2 && codes(bad).iter().all(|code| *code == ComponentIssueCode::Override));
    let subjects: std::collections::BTreeSet<&str> = bad.issues.iter().map(|issue| issue.subject.as_str()).collect();
    assert_eq!(subjects, ["depth", "s-table", "width"].into(), "the two parameters and the solid that depends on them");
    assert!(bad.issues.iter().all(|issue| issue.family_issue.is_some() && !issue.detail.is_empty()));
    assert!(inference.families["fam-table"].issues.is_empty(), "the family itself is sound");
}

#[test]
fn a_component_without_an_override_shares_the_value_of_its_family() {
    let snapshot = room();
    let family = Arc::new(families::family_of(&snapshot, "fam-table", &BTreeMap::new()));
    let level = StoreyLevel::default();
    let entry = component_of(&snapshot, "c-table", &level, Some(&family), None);
    assert!(Arc::ptr_eq(&entry.family, &family));
    let overridden = component_of(&snapshot, "c-table-wide", &level, Some(&family), None);
    assert!(!Arc::ptr_eq(&overridden.family, &family));
    assert_eq!(overrides_of(&snapshot, "c-table-wide").keys().collect::<Vec<_>>(), ["width"]);
    assert!(overrides_of(&snapshot, "c-table").is_empty());
    assert_eq!(overrides_of(&snapshot, "c-bad-override").len(), 2);
}

#[test]
fn the_dependency_names_the_record_the_overrides_and_the_host_but_not_the_name() {
    let snapshot = room();
    let before = dependency(&snapshot, "c-basin-south");
    let mut renamed = snapshot.clone();
    renamed.components.get_mut("c-basin-south").expect("basin").name = "Renamed".into();
    assert_eq!(dependency(&renamed, "c-basin-south"), before);
    let mut moved = snapshot.clone();
    moved.components.get_mut("c-basin-south").expect("basin").elevation = 0.9;
    assert_ne!(dependency(&moved, "c-basin-south"), before);
    let mut rewalled = snapshot.clone();
    rewalled.walls.get_mut("w-south").expect("wall").storey = "st-first".into();
    assert_ne!(dependency(&rewalled, "c-basin-south"), before);
    let table = dependency(&snapshot, "c-table");
    let mut family_edit = snapshot.clone();
    family_edit.family_parameters.get_mut("fam-table.width").expect("parameter").value = "2 m".into();
    assert_eq!(dependency(&family_edit, "c-table"), table, "without an override the family value is a parent, not a read");
    let wide = dependency(&snapshot, "c-table-wide");
    assert_ne!(dependency(&family_edit, "c-table-wide"), wide, "with an override the instance evaluates the family itself");
}
