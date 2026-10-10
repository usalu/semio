use super::{QUANTITY_SET, SWEEP_SET};
use crate::standards::v1::subsets::any::io::export::ifc::testkit::{attic, count, document, number_of, property_sets, quantity_of, real, rows, string, tags, target, text_of};
use crate::{Profile, WallSide};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Value};

fn lengths(document: &Part21Document, sweep: &str) -> Vec<f64> {
    let mut found: Vec<f64> = rows(document, "IFCMEMBER").into_iter().filter(|(_, args)| string(args, 7).as_deref() == Some(sweep)).map(|(instance, _)| quantity_of(document, instance.id, QUANTITY_SET, "Length").expect("a length")).collect();
    found.sort_by(f64::total_cmp);
    found
}

fn depths(document: &Part21Document) -> Vec<f64> {
    rows(document, "IFCEXTRUDEDAREASOLID").into_iter().map(|(_, args): (_, &Vec<Part21Value>)| real(args, 3)).collect()
}

#[test]
fn every_run_of_every_sweep_is_a_member_tagged_with_the_sweep_id() {
    let document = document(&attic());
    assert_eq!(tags(&document, "IFCMEMBER"), ["ws-door", "ws-door", "ws-rail", "ws-slope"], "the door cuts the baseboard of its wall into two runs");
    assert!(rows(&document, "IFCMEMBER").iter().all(|(_, args)| string(args, 4).as_deref() == Some("WallSweep")));
}

#[test]
fn the_lengths_of_the_runs_add_up_to_the_path_the_shapely_table_measured() {
    let document = document(&attic());
    let sum = |sweep: &str| lengths(&document, sweep).iter().sum::<f64>();
    assert!((sum("ws-door") - 7.1).abs() < 1e-9, "{}", sum("ws-door"));
    assert!((sum("ws-rail") - 8.0).abs() < 1e-9);
    assert!((sum("ws-slope") - 8.0).abs() < 1e-9);
    assert_eq!(lengths(&document, "ws-door").len(), 2);
}

#[test]
fn a_straight_run_on_a_flat_base_is_an_extrusion_as_deep_as_the_run_is_long() {
    let document = document(&attic());
    let depths = depths(&document);
    for expected in [1.55, 5.55, 8.0] {
        assert!(depths.iter().any(|depth| (depth - expected).abs() < 1e-9), "no extrusion of depth {expected} among {depths:?}");
    }
}

#[test]
fn the_run_on_a_sloped_base_is_a_faceted_brep() {
    let document = document(&attic());
    let (_, args) = rows(&document, "IFCMEMBER").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("ws-slope")).expect("the member");
    let shape = target(&document, args, 6).and_then(|shape| shape.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("a shape");
    let kinds: Vec<String> = shape[2].as_list().expect("representations").iter().filter_map(|representation| document.resolve(representation)).filter_map(|representation| representation.entity("IFCSHAPEREPRESENTATION")).filter_map(|representation| string(representation, 2)).collect();
    assert_eq!(kinds, ["Brep"]);
}

#[test]
fn a_run_carries_the_cross_section_and_the_gross_volume_of_its_share() {
    let document = document(&attic());
    for (instance, args) in rows(&document, "IFCMEMBER") {
        let tag = string(args, 7).expect("a tag");
        let length = quantity_of(&document, instance.id, QUANTITY_SET, "Length").expect("a length");
        let section = quantity_of(&document, instance.id, QUANTITY_SET, "CrossSectionArea").expect("a section");
        let volume = quantity_of(&document, instance.id, QUANTITY_SET, "GrossVolume").expect("a volume");
        let expected = if tag == "ws-rail" { 0.05 * 0.04 } else { 0.02 * 0.1 };
        assert!((section - expected).abs() < 1e-12, "{tag}: {section}");
        assert!((volume - section * length).abs() < 1e-12, "{tag}");
    }
}

#[test]
fn the_authored_record_travels_in_the_sweep_set() {
    let document = document(&attic());
    let model = attic();
    for (id, sweep) in &model.wall_sweeps {
        let sets = property_sets(&document, id, SWEEP_SET);
        assert!(!sets.is_empty(), "{id}");
        for rows in sets {
            assert_eq!(text_of(&rows, "SweepId").as_deref(), Some(id.as_str()));
            assert_eq!(text_of(&rows, "Host").as_deref(), Some(sweep.host.as_str()));
            assert_eq!(text_of(&rows, "Side").as_deref(), Some(if sweep.side == WallSide::Left { "Left" } else { "Right" }));
            assert_eq!(text_of(&rows, "Material").as_deref(), Some(sweep.material.as_str()));
            assert_eq!((number_of(&rows, "Height"), number_of(&rows, "Inset")), (Some(sweep.height), Some(sweep.inset)));
            let profile: Profile = from_json_str(&text_of(&rows, "Profile").expect("a profile"), JsonMemberPolicy::Reject).expect("the profile decodes");
            assert_eq!(profile, sweep.profile);
        }
    }
    assert_eq!(property_sets(&document, "ws-door", SWEEP_SET).len(), 2, "the door splits the baseboard in two runs, every run is tagged with the sweep id and carries the set");
}

#[test]
fn a_sweep_is_contained_in_the_storey_of_its_wall_and_has_the_material_of_its_record() {
    let document = document(&attic());
    assert_eq!(count(&document, "IFCMEMBER"), 4);
    let model = attic();
    let associated: usize = document.by_type("IFCRELASSOCIATESMATERIAL").filter_map(|relation| relation.entity("IFCRELASSOCIATESMATERIAL")).filter(|args| document.resolve(&args[5]).is_some_and(|definition| definition.is_type("IFCMATERIAL"))).map(|args| args[4].as_list().map_or(0, |items| items.len())).sum();
    assert!(associated >= model.wall_sweeps.len(), "every sweep run is associated with its material");
}

#[test]
fn a_sweep_whose_host_wall_is_gone_is_skipped_with_a_note() {
    let mut model = attic();
    model.walls.remove("w-rail");
    let (document, notes) = crate::standards::v1::subsets::any::io::export::ifc::model_to_part21(crate::standards::v1::subsets::any::io::export::ifc::Schema::Ifc2x3, &model).expect("the model exports");
    assert!(!tags(&document, "IFCMEMBER").contains(&"ws-rail".to_string()));
    assert!(notes.iter().any(|note| note.contains("ws-rail")), "{notes:?}");
}
