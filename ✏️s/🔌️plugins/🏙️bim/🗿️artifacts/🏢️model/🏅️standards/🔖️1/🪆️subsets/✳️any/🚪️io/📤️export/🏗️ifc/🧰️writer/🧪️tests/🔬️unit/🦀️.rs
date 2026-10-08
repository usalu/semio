use super::*;
use semio_s_artifact_stdio_ifc::part21::{write_part21, Part21Document, Part21Header};

fn finished(ifc: Ifc) -> Part21Document {
    ifc.finish(Part21Header::iso_10303_21_minimum())
}

fn point_of(document: &Part21Document, id: u64) -> [f64; 2] {
    let coordinates = document.instance(id).and_then(|instance| instance.entity("IFCCARTESIANPOINT")).and_then(|args| args[0].as_list().map(|items| items.iter().filter_map(Part21Value::as_real).collect::<Vec<f64>>())).expect("a cartesian point");
    [coordinates[0], coordinates[1]]
}

#[test]
fn a_global_id_has_twenty_two_ifc_alphabet_characters_and_a_two_bit_head() {
    let id = global_id("IFCWALL:w-1");
    assert_eq!(id.len(), 22);
    assert!(id.bytes().all(|byte| ALPHABET.contains(&byte)));
    assert!("0123".contains(&id[..1]), "the first character carries only two bits: {id}");
}

#[test]
fn a_global_id_is_stable_per_key_and_distinct_across_keys() {
    assert_eq!(global_id("IFCWALL:w-1"), global_id("IFCWALL:w-1"));
    let ids: std::collections::BTreeSet<String> = (0..2000).map(|index| global_id(&format!("IFCWALL:w-{index}"))).collect();
    assert_eq!(ids.len(), 2000);
    assert_ne!(global_id("IFCWALL:w-1"), global_id("IFCSLAB:w-1"));
}

#[test]
fn reals_are_rounded_to_a_nanometre_and_zero_is_never_negative() {
    let shown = |value: f64| match real(value) {
        V::Real(decimal) => decimal.to_string(),
        other => panic!("not a real: {other:?}"),
    };
    assert_eq!(shown(0.1 + 0.2), "0.3");
    assert_eq!(shown(-0.0), "0.");
    assert_eq!(shown(2.5e-10), "0.");
    assert_eq!(shown(3.0), "3.");
}

#[test]
fn identical_points_and_directions_are_written_once() {
    let mut ifc = Ifc::new("a", "o", 0.0);
    let (a, b) = (ifc.point3([1.0, 2.0, 3.0]), ifc.point3([1.0, 2.0, 3.0]));
    assert_eq!(a, b);
    assert_ne!(a, ifc.point3([1.0, 2.0, 4.0]));
    assert_eq!(ifc.dir3([0.0, 0.0, 1.0]), ifc.dir3([0.0, 0.0, 1.0]));
}

#[test]
fn rooted_entities_start_with_a_stable_global_id_the_owner_history_name_and_description() {
    let mut ifc = Ifc::new("a", "o", 0.0);
    let owner = ifc.owner;
    let id = ifc.rooted("IFCWALL", "w-1", "Wall", "", vec![unset()]);
    let document = finished(ifc);
    let args = document.instance(id).and_then(|instance| instance.entity("IFCWALL")).expect("the wall");
    assert_eq!(args[0].as_str(), Some(global_id("IFCWALL:w-1").as_str()));
    assert_eq!(args[1].as_ref_id(), Some(owner));
    assert_eq!(args[2].as_str(), Some("Wall"));
    assert!(args[3].is_unset());
}

#[test]
fn a_straight_loop_is_a_closed_polyline_and_a_bulged_loop_a_composite_curve() {
    let mut ifc = Ifc::new("a", "o", 0.0);
    let square = ifc.loop_curve(&[([0.0, 0.0], 0.0), ([1.0, 0.0], 0.0), ([1.0, 1.0], 0.0)]);
    let round = ifc.loop_curve(&[([0.0, 0.0], 0.0), ([2.0, 0.0], 0.5), ([2.0, 2.0], 0.0)]);
    let document = finished(ifc);
    let closed = document.instance(square).and_then(|instance| instance.entity("IFCPOLYLINE")).expect("a polyline");
    assert_eq!(closed[0].as_list().map(<[V]>::len), Some(4), "the first point repeats to close the loop");
    let composite = document.instance(round).and_then(|instance| instance.entity("IFCCOMPOSITECURVE")).expect("a composite curve");
    assert_eq!(composite[0].as_list().map(<[V]>::len), Some(3));
    assert_eq!(count_of(&document, "IFCTRIMMEDCURVE"), 1);
}

fn count_of(document: &Part21Document, entity: &str) -> usize {
    document.by_type(entity).count()
}

#[test]
fn a_bulged_edge_is_a_trimmed_circle_through_both_end_points_with_the_right_sense() {
    for bulge in [0.5, -0.5, 1.0, -2.0, 0.1] {
        let mut ifc = Ifc::new("a", "o", 0.0);
        let curve = ifc.edge_curve([0.0, 0.0], [4.0, 0.0], bulge);
        let document = finished(ifc);
        let trimmed = document.instance(curve).and_then(|instance| instance.entity("IFCTRIMMEDCURVE")).expect("a trimmed curve");
        let circle = document.resolve(&trimmed[0]).and_then(|instance| instance.entity("IFCCIRCLE")).expect("its circle");
        let radius = circle[1].as_real().expect("a radius");
        let placement = document.resolve(&circle[0]).and_then(|instance| instance.entity("IFCAXIS2PLACEMENT2D")).expect("its placement");
        let centre = point_of(&document, placement[0].as_ref_id().expect("a location"));
        let sweep = 4.0 * f64::atan(bulge);
        assert!((radius - 4.0 / (2.0 * (sweep / 2.0).sin().abs())).abs() < 1e-8, "bulge {bulge}");
        for end in [[0.0, 0.0], [4.0, 0.0]] {
            assert!(((centre[0] - end[0]).hypot(centre[1] - end[1]) - radius).abs() < 1e-8, "bulge {bulge}: the circle passes through {end:?}");
        }
        assert_eq!(trimmed[3].as_enum(), Some(if bulge > 0.0 { "T" } else { "F" }));
        assert!((centre[1] - bulge.signum() * (sweep / 2.0).cos() * radius).abs() < 1e-8, "the centre lies on the correct side of the chord for bulge {bulge}");
    }
}

#[test]
fn the_context_names_the_true_north_and_three_sub_contexts() {
    let ifc = Ifc::new("a", "o", 0.5);
    let document = finished(ifc);
    let context = document.by_type("IFCGEOMETRICREPRESENTATIONCONTEXT").next().and_then(|instance| instance.entity("IFCGEOMETRICREPRESENTATIONCONTEXT")).expect("the context");
    let north = document.resolve(&context[5]).and_then(|instance| instance.entity("IFCDIRECTION")).expect("the north");
    let components: Vec<f64> = north[0].as_list().expect("components").iter().filter_map(Part21Value::as_real).collect();
    assert!((components[0] + 0.5f64.sin()).abs() < 1e-8 && (components[1] - 0.5f64.cos()).abs() < 1e-8);
    let identifiers: Vec<String> = document.by_type("IFCGEOMETRICREPRESENTATIONSUBCONTEXT").filter_map(|instance| instance.entity("IFCGEOMETRICREPRESENTATIONSUBCONTEXT")).filter_map(|args| args[0].as_str().map(str::to_string)).collect();
    assert_eq!(identifiers, ["Body", "Axis", "FootPrint"]);
}

#[test]
fn the_document_text_survives_a_reparse() {
    let mut ifc = Ifc::new("a", "o", 0.0);
    ifc.rooted("IFCWALL", "w-1", "Quote's wall", "", vec![unset()]);
    let document = finished(ifc);
    let reparsed = semio_s_artifact_stdio_ifc::part21::parse_part21(&write_part21(&document)).expect("the text parses");
    assert_eq!(reparsed.instances, document.instances);
}
