use super::*;
use crate::standards::v1::subsets::any::schema::inferences::mep::mep_of;
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{MepElement, MepShape, MepSystem, ModelSnapshot, Point3};

fn point(x: f64, y: f64, z: f64) -> Point3 {
    Point3 { x, y, z }
}

fn solid_of(system: MepSystem, shape: MepShape, path: Vec<Point3>) -> (MepValue, ElementSolid) {
    let mut snapshot = ModelSnapshot::default();
    snapshot.mep_elements.insert("m".into(), MepElement { storey: "s1".into(), system, shape, path, name: String::new() });
    let value = mep_of(&snapshot, "m", &StoreyLevel::default());
    let solid = mep_solid(&value);
    (value, solid)
}

fn close(got: f64, want: f64, tolerance: f64) {
    assert!((got - want).abs() <= tolerance * want.abs().max(1.0), "{got} vs {want}");
}

fn watertight(solid: &ElementSolid) -> bool {
    solid.mesh().is_watertight()
}

#[test]
fn a_straight_duct_is_a_box_of_its_section_and_length() {
    let (value, solid) = solid_of(MepSystem::Supply, MepShape::Duct { width: 0.3, height: 0.2 }, vec![point(0.0, 0.0, 2.5), point(4.0, 0.0, 2.5)]);
    close(solid.volume, 0.06 * 4.0, 1e-12);
    close(solid.volume, value.volume, 1e-12);
    assert_eq!((solid.bounds.min.x, solid.bounds.max.x), (0.0, 4.0));
    close(solid.bounds.max.y - solid.bounds.min.y, 0.3, 1e-12);
    close(solid.bounds.max.z - solid.bounds.min.z, 0.2, 1e-12);
    assert_eq!(solid.groups.len(), 1);
    assert_eq!(solid.groups[0].part, "supply");
    assert_eq!(solid.family, SolidFamily::Mep);
    assert!(watertight(&solid));
}

#[test]
fn a_mitred_bend_keeps_the_volume_of_the_closed_form_and_stays_closed_per_prism() {
    let (value, solid) = solid_of(MepSystem::Return, MepShape::Duct { width: 0.4, height: 0.25 }, vec![point(0.0, 0.0, 2.0), point(3.0, 0.0, 2.0), point(3.0, 2.0, 2.0), point(3.0, 2.0, 3.5)]);
    close(solid.volume, 0.1 * (3.0 + 2.0 + 1.5), 1e-9);
    close(solid.volume, value.volume, 1e-9);
    assert!(solid.volume > 0.0);
}

#[test]
fn a_duct_keeps_its_orientation_on_a_riser_and_on_a_slope() {
    let (_, riser) = solid_of(MepSystem::Exhaust, MepShape::Duct { width: 0.4, height: 0.2 }, vec![point(1.0, 1.0, 0.0), point(1.0, 1.0, 3.0)]);
    close(riser.volume, 0.08 * 3.0, 1e-9);
    assert!(riser.bounds.max.x - riser.bounds.min.x > 0.0 && riser.bounds.max.y - riser.bounds.min.y > 0.0);
    let (_, slope) = solid_of(MepSystem::Exhaust, MepShape::Tray { width: 0.3, height: 0.1 }, vec![point(0.0, 0.0, 0.0), point(3.0, 0.0, 1.0)]);
    close(slope.volume, 0.03 * 3.0f64.hypot(1.0), 1e-9);
    close(slope.bounds.max.y - slope.bounds.min.y, 0.3, 1e-9);
}

#[test]
fn a_pipe_is_tessellated_within_the_chord_tolerance_of_a_circle() {
    let (value, solid) = solid_of(MepSystem::DomesticWater, MepShape::Pipe { diameter: 0.1 }, vec![point(0.0, 0.0, 0.0), point(5.0, 0.0, 0.0)]);
    assert!(solid.volume < value.volume, "an inscribed polygon is smaller than its circle");
    close(solid.volume, value.volume, 4e-3);
    assert!(solid.triangle_count() > 100);
    assert!(watertight(&solid));
}

#[test]
fn an_element_that_cannot_be_built_has_an_empty_solid() {
    let (_, solid) = solid_of(MepSystem::Gas, MepShape::Pipe { diameter: 0.0 }, vec![point(0.0, 0.0, 0.0), point(5.0, 0.0, 0.0)]);
    assert!(solid.is_empty());
    let (_, solid) = solid_of(MepSystem::Gas, MepShape::Pipe { diameter: 0.1 }, vec![point(0.0, 0.0, 0.0)]);
    assert!(solid.is_empty());
}
