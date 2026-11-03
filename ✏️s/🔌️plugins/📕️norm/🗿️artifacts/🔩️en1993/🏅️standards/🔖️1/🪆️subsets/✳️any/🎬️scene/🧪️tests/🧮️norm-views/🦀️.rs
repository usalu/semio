//! 🧮️ Scene tests of the wave 3 norm entities: every new row is drawn in 3D with its owner, the instantaneous centre plate draws one force per bolt and the centre, and every new family has its diagram with a table twin; the numbers of the diagrams are those of the checks.

use super::*;
use scene::projection_kit::test_support::{entity, revision};
use scene::Validate;
use std::collections::BTreeSet;

fn showcase() -> En1993Snapshot {
    semio_s_artifact_norm_en1993_showcase::full_showcase()
}

fn scene_of(document: &En1993Snapshot) -> (StructuralScene, CheckReport) {
    let report = checks::check_full_steel_structure(document);
    let scene = project(&SceneInput { document, report: &report, placements: &[], revision: revision() });
    (scene, report)
}

#[test]
fn every_new_row_is_drawn_and_registered_as_an_entity_kind() {
    let document = showcase();
    let (scene, _) = scene_of(&document);
    assert_eq!(scene.validate(), Ok(()));
    for (collection, id) in [("fatigueStresses", "fstress-1"), ("bucklingLengths", "buckling-1"), ("formingGains", "gain-1"), ("fastenedJoints", "fastened-1"), ("materialDuctility", "ductility-1"), ("jointPlates", "plate-ic")] {
        let drawn = scene.entities.iter().filter(|e| e.entity.path.starts_with(&format!("{collection}[id={id}]"))).count();
        assert!(drawn >= 1, "{collection} {id} is not drawn");
        assert!(ENTITY_KINDS.iter().any(|k| k.collection == collection), "{collection} has no entity kind");
        assert_eq!(entity_kind_of(&document, id).map(|k| k.collection), Some(collection));
    }
}

#[test]
fn the_instantaneous_centre_plate_draws_a_force_on_every_bolt_and_the_centre() {
    let document = showcase();
    let (scene, _) = scene_of(&document);
    let plate = document.joint_plates.iter().find(|p| p.id == "plate-ic").expect("plate-ic");
    let forces = scene.entities.iter().filter(|e| e.entity.id.starts_with("plate-ic:bolt-force-")).count();
    assert_eq!(forces, (plate.bolt_rows * plate.bolts_per_row) as usize);
    entity(&scene, "plate-ic:ic");
    let elastic = scene.entities.iter().filter(|e| e.entity.id.starts_with("plate-fin:bolt-force-")).count();
    assert_eq!(elastic, 3, "an elastic plate draws its bolt forces too");
    assert!(scene.entities.iter().all(|e| e.entity.id != "plate-fin:ic"), "no centre without the plastic method");
}

#[test]
fn the_fatigue_range_draws_three_bars_with_the_damage_label() {
    let (scene, _) = scene_of(&showcase());
    for tag in ["raw", "design", "strength"] {
        entity(&scene, &format!("fstress-1:{tag}"));
    }
    assert!(scene.annotations.iter().any(|a| a.id() == "fstress-1:strength-label"));
}

#[test]
fn the_buckling_length_is_dimensioned_against_the_member_length() {
    let (scene, _) = scene_of(&showcase());
    let ids: Vec<&str> = scene.annotations.iter().map(|a| a.id()).collect();
    assert!(ids.contains(&"buckling-1:length") && ids.contains(&"buckling-1:buckling"));
}

#[test]
fn the_screwed_joint_draws_two_sheets_and_one_screw_per_fastener() {
    let document = showcase();
    let (scene, _) = scene_of(&document);
    entity(&scene, "fastened-1:base");
    entity(&scene, "fastened-1:head");
    let screws = scene.entities.iter().filter(|e| e.entity.id.starts_with("fastened-1:fastener-")).count();
    assert_eq!(screws, document.fastened_joints[0].count as usize);
    entity(&scene, "fastened-1:shear");
    entity(&scene, "fastened-1:tension");
}

#[test]
fn every_new_family_has_a_diagram_with_a_table_twin_and_the_number_of_its_check() {
    let document = showcase();
    let report = checks::check_full_steel_structure(&document);
    let list = diagrams(&document, &report, None);
    for id in ["bolt-group:plate-ic", "bolt-group:plate-fin", "fatigue-range:fstress-1", "nomogram:buckling-1", "forming-gain:gain-1", "fastener:fastened-1", "ductility:ductility-1"] {
        let diagram = list.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("missing diagram {id}"));
        assert_eq!(diagram.validate(), Ok(()), "{id}");
        assert!(!diagram.table_twin.columns.is_empty() && diagram.table_twin.columns.iter().all(|c| !c.values.is_empty()), "{id} table twin");
    }
    let nomogram = list.iter().find(|d| d.id == "nomogram:buckling-1").expect("nomogram");
    let k = checks::effective_length_checks::factors(&document.buckling_lengths[0]).expect("factor").2;
    assert!(nomogram.markers.iter().any(|m| (m.at.y - k).abs() < 1e-12), "the marker sits on K");
    let range = list.iter().find(|d| d.id == "fatigue-range:fstress-1").expect("range");
    let check = report.checks.iter().find(|c| c.id == "en1993.1-9.5.range.fstress-1").expect("range check");
    assert!(range.markers.iter().any(|m| m.bound_check.as_deref() == Some(check.id.as_str())));
}

#[test]
fn the_dedicated_diagrams_cover_their_check_families() {
    let drawn: BTreeSet<&str> = kit::DRAWN_FAMILIES.iter().copied().collect();
    for family in ["en1993.1-8.3.12.ic", "en1993.1-9.5.range", "en1993.1-1.6.3.1.3.lcr", "en1993.1-3.3.2.2.fya", "en1993.1-3.8.3.shear", "en1993.3.2.2.strain"] {
        assert!(drawn.contains(family), "{family}");
        assert!(kit::CHECK_OWNERS.iter().any(|(p, _)| *p == family), "{family} has an owner");
    }
}
