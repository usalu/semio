use super::clash::{centre_distance, clashes_of, segment_distance};
use super::*;
use crate::{MepElement, Point3};

fn point(x: f64, y: f64, z: f64) -> Point3 {
    Point3 { x, y, z }
}

fn level(elevation: f64) -> StoreyLevel {
    StoreyLevel { elevation, top_elevation: elevation + 3.0, absolute_elevation: elevation, absolute_top_elevation: elevation + 3.0 }
}

fn model(rows: Vec<(&str, MepSystem, MepShape, Vec<Point3>)>) -> ModelSnapshot {
    let mut snapshot = ModelSnapshot::default();
    for (id, system, shape, path) in rows {
        snapshot.mep_elements.insert(id.to_string(), MepElement { storey: "s1".into(), system, shape, path, name: String::new() });
    }
    snapshot
}

fn close(got: f64, want: f64) {
    assert!((got - want).abs() <= 1e-9 * want.abs().max(1.0), "{got} vs {want}");
}

#[test]
fn a_duct_has_the_closed_forms_of_its_section_and_path() {
    let snapshot = model(vec![("d", MepSystem::Supply, MepShape::Duct { width: 0.3, height: 0.2 }, vec![point(0.0, 0.0, 2.5), point(4.0, 0.0, 2.5), point(4.0, 3.0, 3.5)])]);
    let value = mep_of(&snapshot, "d", &level(3.0));
    close(value.section.area, 0.06);
    close(value.section.perimeter, 1.0);
    assert_eq!(value.section.label, "300x200");
    close(value.length, 4.0 + 3.0f64.hypot(1.0));
    close(value.volume, 0.06 * value.length);
    close(value.surface_area, 1.0 * value.length);
    assert_eq!(value.path[0].z, 5.5);
    assert_eq!(value.colour, "#1f77d4");
    assert!(value.issues.is_empty() && value.buildable());
    close(value.bounds.min.z, 5.5 - 0.15);
    close(value.bounds.max.x, 4.0 + 0.15);
}

#[test]
fn a_pipe_is_a_circle_and_its_size_label_uses_the_diameter() {
    let snapshot = model(vec![("p", MepSystem::DomesticWater, MepShape::Pipe { diameter: 0.1 }, vec![point(0.0, 0.0, 0.0), point(0.0, 0.0, 2.0)])]);
    let value = mep_of(&snapshot, "p", &level(0.0));
    close(value.section.area, std::f64::consts::PI * 0.0025);
    assert_eq!(value.section.label, "Ø100");
    assert!(value.segments[0].vertical());
    close(value.volume, std::f64::consts::PI * 0.0025 * 2.0);
    close(value.section.reach(), 0.05);
}

#[test]
fn degenerate_sections_and_runs_are_issues_and_have_no_volume() {
    let snapshot = model(vec![
        ("flat", MepSystem::Supply, MepShape::Duct { width: 0.0, height: 0.2 }, vec![point(0.0, 0.0, 0.0), point(1.0, 0.0, 0.0)]),
        ("short", MepSystem::Supply, MepShape::Pipe { diameter: 0.1 }, vec![point(1.0, 1.0, 1.0), point(1.0, 1.0, 1.0)]),
        ("nan", MepSystem::Supply, MepShape::Pipe { diameter: 0.1 }, vec![point(f64::NAN, 1.0, 1.0), point(1.0, 1.0, 1.0)]),
        ("repeat", MepSystem::Supply, MepShape::Pipe { diameter: 0.1 }, vec![point(0.0, 0.0, 0.0), point(0.0, 0.0, 0.0), point(1.0, 0.0, 0.0)]),
    ]);
    let codes = |id: &str| mep_of(&snapshot, id, &level(0.0)).issues.iter().map(|issue| issue.code).collect::<Vec<_>>();
    assert_eq!(codes("flat"), [MepIssueCode::SectionDegenerate]);
    assert_eq!(codes("short"), [MepIssueCode::PathDegenerate]);
    assert_eq!(codes("nan"), [MepIssueCode::NonFinite]);
    assert_eq!(codes("repeat"), [MepIssueCode::PathDegenerate]);
    assert_eq!(mep_of(&snapshot, "flat", &level(0.0)).volume, 0.0);
    assert!(!mep_of(&snapshot, "nan", &level(0.0)).buildable());
    assert!(mep_of(&snapshot, "repeat", &level(0.0)).buildable(), "the repeated point is dropped, the run keeps its length");
}

#[test]
fn every_system_has_a_distinct_srgb_colour_and_a_key_that_round_trips() {
    let colours: std::collections::BTreeSet<&str> = SYSTEMS.iter().map(|system| colour(*system)).collect();
    assert_eq!(colours.len(), 9);
    for system in SYSTEMS {
        assert_eq!(system_of_key(key(system)), Some(system));
        let [r, g, b] = rgb(system);
        assert!((0.0..=1.0).contains(&r) && (0.0..=1.0).contains(&g) && (0.0..=1.0).contains(&b));
        assert_eq!(part_colour(key(system)), Some(rgb(system)));
    }
    assert_eq!(part_colour("body"), None);
    assert_eq!(colour(MepSystem::Supply), "#1f77d4");
}

#[test]
fn the_distance_of_two_segments_is_the_closed_form() {
    close(segment_distance([0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [2.0, -1.0, 1.0], [2.0, 1.0, 1.0]), 1.0);
    close(segment_distance([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [3.0, 0.0, 0.0], [4.0, 0.0, 0.0]), 2.0);
    close(segment_distance([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 0.0, 0.0], [0.5, 1.0, 0.0]), 0.0);
    close(segment_distance([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 0.0]), 2.0f64.sqrt());
    close(segment_distance([0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [2.0, 1.0, 0.0]), 1.0);
}

#[test]
fn elements_of_different_systems_clash_when_their_centre_lines_come_closer_than_their_reaches() {
    let snapshot = model(vec![
        ("duct", MepSystem::Supply, MepShape::Duct { width: 0.4, height: 0.2 }, vec![point(0.0, 0.0, 2.5), point(6.0, 0.0, 2.5)]),
        ("pipe", MepSystem::DomesticWater, MepShape::Pipe { diameter: 0.1 }, vec![point(3.0, -2.0, 2.6), point(3.0, 2.0, 2.6)]),
        ("far", MepSystem::Power, MepShape::Tray { width: 0.2, height: 0.05 }, vec![point(0.0, 5.0, 2.5), point(6.0, 5.0, 2.5)]),
        ("same", MepSystem::Supply, MepShape::Duct { width: 0.4, height: 0.2 }, vec![point(3.0, -2.0, 2.5), point(3.0, 2.0, 2.5)]),
        ("touch", MepSystem::Waste, MepShape::Pipe { diameter: 0.1 }, vec![point(1.0, -2.0, 2.75), point(1.0, 2.0, 2.75)]),
    ]);
    let values: Vec<(String, MepValue)> = snapshot.mep_elements.keys().map(|id| (id.clone(), mep_of(&snapshot, id, &level(0.0)))).collect();
    let clashes = clashes_of(values.iter().map(|(id, value)| (id.as_str(), value)));
    let pairs: Vec<(&str, &str)> = clashes.pairs.iter().map(|pair| (pair.a.as_str(), pair.b.as_str())).collect();
    assert_eq!(pairs, [("duct", "pipe"), ("pipe", "same")], "touching (0.25 m = 0.2 + 0.05) and equal systems and far elements are no clash");
    close(clashes.pairs[0].distance, 0.1);
    close(clashes.pairs[0].reach, 0.25);
    close(centre_distance(&values[0].1, &values[2].1), 0.1);
}
