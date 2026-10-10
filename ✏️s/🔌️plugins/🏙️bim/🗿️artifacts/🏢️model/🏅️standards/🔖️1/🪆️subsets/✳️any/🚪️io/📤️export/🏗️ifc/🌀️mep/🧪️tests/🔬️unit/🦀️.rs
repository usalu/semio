use super::*;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::{components, count, document, document_in, rows, string, tags, target};
use crate::standards::v1::subsets::any::io::export::ifc::{model_to_part21, Schema};
use crate::Point3;

fn duct(path: Vec<Point3>) -> crate::MepElement {
    crate::MepElement { storey: "st-ground".into(), system: MepSystem::Supply, shape: MepShape::Duct { width: 0.4, height: 0.25 }, path, name: "Extra Duct".into() }
}

#[test]
fn elements_of_one_section_share_one_type_object_and_each_section_has_its_own() {
    let mut model = components();
    model.mep_elements.insert("mep-extra".into(), duct(vec![Point3 { x: 0.0, y: 0.5, z: 2.0 }, Point3 { x: 2.0, y: 0.5, z: 2.0 }]));
    let document = document(&model);
    assert_eq!(count(&document, "IFCDUCTSEGMENTTYPE"), 2);
    let (_, relation) = rows(&document, "IFCRELDEFINESBYTYPE").into_iter().find(|(_, relation)| target(&document, relation, 5).and_then(|kind| kind.entity("IFCDUCTSEGMENTTYPE")).is_some_and(|kind| string(kind, 2).as_deref() == Some("Duct 400x250"))).expect("the 400x250 type");
    assert_eq!(relation[4].as_list().map(<[_]>::len), Some(2));
}

#[test]
fn the_type_objects_carry_the_predefined_type_of_their_kind() {
    let document = document(&components());
    let predefined = |entity: &str| rows(&document, entity).into_iter().filter_map(|(_, args)| args.last().and_then(|value| value.as_enum()).map(str::to_string)).collect::<Vec<_>>();
    assert!(predefined("IFCDUCTSEGMENTTYPE").iter().all(|value| value == "RIGIDSEGMENT"));
    assert!(predefined("IFCPIPESEGMENTTYPE").iter().all(|value| value == "RIGIDSEGMENT"));
    assert!(predefined("IFCCABLECARRIERSEGMENTTYPE").iter().all(|value| value == "CABLETRAYSEGMENT"));
}

#[test]
fn the_ifc4_systems_carry_their_predefined_type() {
    let document = document_in(Schema::Ifc4, &components());
    let mut found: Vec<(String, String)> = rows(&document, "IFCDISTRIBUTIONSYSTEM").into_iter().filter_map(|(_, args)| Some((string(args, 2)?, args.last()?.as_enum()?.to_string()))).collect();
    found.sort();
    assert_eq!(
        found,
        [("DomesticWater", "DOMESTICCOLDWATER"), ("Gas", "GAS"), ("Lighting", "LIGHTING"), ("Power", "ELECTRICAL"), ("Return", "VENTILATION"), ("Supply", "VENTILATION"), ("Waste", "WASTEWATER")].map(|(name, predefined)| (name.to_string(), predefined.to_string()))
    );
}

#[test]
fn a_run_without_length_is_noted_and_not_written() {
    let mut model = components();
    model.mep_elements.insert("mep-point".into(), duct(vec![Point3 { x: 1.0, y: 1.0, z: 1.0 }, Point3 { x: 1.0, y: 1.0, z: 1.0 }]));
    let (document, notes) = model_to_part21(Schema::Ifc2x3, &model).expect("the export");
    assert!(notes.iter().any(|note| note.contains("mep-point")), "{notes:?}");
    assert!(!tags(&document, "IFCFLOWSEGMENT").contains(&"mep-point".to_string()));
}

#[test]
fn a_run_on_a_missing_storey_is_noted_and_not_written() {
    let mut model = components();
    let mut ghost = duct(vec![Point3 { x: 0.0, y: 0.0, z: 1.0 }, Point3 { x: 1.0, y: 0.0, z: 1.0 }]);
    ghost.storey = "st-ghost".into();
    model.mep_elements.insert("mep-ghost".into(), ghost);
    let (document, notes) = model_to_part21(Schema::Ifc2x3, &model).expect("the export");
    assert!(notes.iter().any(|note| note.contains("mep-ghost")), "{notes:?}");
    assert!(!tags(&document, "IFCFLOWSEGMENT").contains(&"mep-ghost".to_string()));
}

#[test]
fn the_names_of_the_systems_are_the_variants_and_the_predefined_types_cover_every_service() {
    use crate::standards::v1::subsets::any::schema::inferences::mep::SYSTEMS;
    for system in SYSTEMS {
        assert_eq!(system_name(system), format!("{system:?}"));
        assert!(!predefined(system).is_empty());
    }
}
