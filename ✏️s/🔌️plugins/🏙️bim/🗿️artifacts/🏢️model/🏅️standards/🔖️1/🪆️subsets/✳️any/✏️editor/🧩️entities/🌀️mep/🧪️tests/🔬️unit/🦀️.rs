use super::*;
use crate::editor::bim::entities::components::tests::furnished;
use crate::editor::bim::entities::{kind_holding, kind_of};
use crate::ModelInference;
use protocol::Inference;

fn model() -> ModelSnapshot {
    let mut snapshot = furnished();
    let storey = snapshot.storeys.keys().next().cloned().expect("a storey");
    snapshot.mep_elements.insert("m-duct".into(), MepElement { storey, system: MepSystem::Supply, shape: MepShape::Duct { width: 0.3, height: 0.2 }, path: vec![Point3 { x: 0.0, y: 0.0, z: 2.5 }, Point3 { x: 4.0, y: 0.0, z: 2.5 }], name: "Duct 1".into() });
    snapshot
}

fn write(snapshot: &ModelSnapshot, key: &str, value: &str) -> Option<ModelMutation> {
    let field = kind_of("mep-element").expect("kind").fields.iter().find(|field| field.key == key).expect("field");
    (field.write.expect("editable"))(snapshot, "m-duct", value)
}

#[test]
fn the_section_reads_and_writes_as_text_in_metres() {
    assert_eq!(shape_text(&MepShape::Duct { width: 0.3, height: 0.2 }), "duct 0.3 x 0.2");
    assert_eq!(shape_text(&MepShape::Pipe { diameter: 0.1 }), "pipe 0.1");
    assert_eq!(shape_text(&MepShape::Tray { width: 0.3, height: 0.06 }), "tray 0.3 x 0.06");
    for shape in [MepShape::Duct { width: 0.3, height: 0.2 }, MepShape::Pipe { diameter: 0.025 }, MepShape::Tray { width: 0.45, height: 0.06 }] {
        assert_eq!(parse_shape(&shape_text(&shape)), Some(shape));
    }
    assert_eq!(parse_shape(" TRAY 0.3 × 0.06 "), Some(MepShape::Tray { width: 0.3, height: 0.06 }));
    for text in ["duct 0.3", "duct 0 x 0.2", "pipe -1", "pipe", "round 3", "duct a x b", "duct 0.3 x inf"] {
        assert_eq!(parse_shape(text), None, "{text:?} is no section");
    }
    assert_eq!((size_text(&MepShape::Duct { width: 0.3, height: 0.2 }), size_text(&MepShape::Pipe { diameter: 0.1 })), ("300 × 200".to_string(), "Ø100".to_string()));
}

#[test]
fn the_path_reads_and_writes_as_vertices_with_their_elevation() {
    let path = vec![Point3 { x: 0.0, y: 0.0, z: 2.5 }, Point3 { x: 4.0, y: 0.5, z: 2.5 }, Point3 { x: 4.0, y: 0.5, z: 3.1 }];
    assert_eq!(path_text(&path), "0, 0, 2.5; 4, 0.5, 2.5; 4, 0.5, 3.1");
    assert_eq!(parse_path(&path_text(&path)), Some(path));
    for text in ["", "0, 0, 1", "0, 0; 1, 1", "0, 0, 1; 1, 1", "0, 0, 1; 1, 1, 1, 1", "0, 0, 1; a, b, c", "0, 0, 1; 1, 1, inf"] {
        assert_eq!(parse_path(text), None, "{text:?} is no route");
    }
}

#[test]
fn a_new_element_runs_four_metres_at_the_default_elevation_with_the_section_of_its_system() {
    let snapshot = model();
    let storey = snapshot.mep_elements["m-duct"].storey.clone();
    let Ok(ModelMutation::CreateMepElement(create)) = create_mep(&snapshot, "m-new", &storey, "Duct 2") else { panic!("a create-mep-element") };
    assert_eq!((create.mep.system, create.mep.shape.clone(), create.mep.name.as_str()), (MepSystem::Supply, MepShape::Duct { width: 0.3, height: 0.2 }, "Duct 2"));
    assert_eq!(create.mep.path, vec![Point3 { x: 0.0, y: 0.5, z: 2.5 }, Point3 { x: 4.0, y: 0.5, z: 2.5 }]);
    assert_eq!(create_mep(&snapshot, "m-new", "nowhere", "x").err(), Some("bim.create.storey-missing"));
    assert_eq!((default_shape(MepSystem::Waste), default_shape(MepSystem::Data)), (MepShape::Pipe { diameter: 0.1 }, MepShape::Tray { width: 0.3, height: 0.06 }));
}

#[test]
fn an_edited_value_is_exactly_one_sparse_set_mep_element() {
    let snapshot = model();
    let Some(ModelMutation::SetMepElement(leaf)) = write(&snapshot, "shape", "pipe 0.1") else { panic!("a set-mep-element") };
    assert_eq!((leaf.id.as_str(), leaf.shape.clone()), ("m-duct", Some(MepShape::Pipe { diameter: 0.1 })));
    assert!(leaf.system.is_none() && leaf.path.is_none() && leaf.name.is_none() && leaf.storey.is_none());
    let Some(ModelMutation::SetMepElement(leaf)) = write(&snapshot, "system", "waste") else { panic!("a set-mep-element") };
    assert_eq!(leaf.system, Some(MepSystem::Waste));
    let Some(ModelMutation::SetMepElement(leaf)) = write(&snapshot, "path", "0, 0, 1; 2, 0, 1") else { panic!("a set-mep-element") };
    assert_eq!(leaf.path.expect("path").len(), 2);
    for (key, text) in [("shape", "duct 0 x 1"), ("system", "steam"), ("path", "0, 0, 1")] {
        assert!(write(&snapshot, key, text).is_none(), "{key} {text:?} writes nothing");
    }
}

#[test]
fn a_routed_element_is_an_entity_under_its_storey_and_shows_its_section_length_and_findings() {
    let snapshot = model();
    let row = kind_of("mep-element").expect("kind");
    assert_eq!((kind_holding(&snapshot, "m-duct").map(|row| row.kind), (row.name)(&snapshot, "m-duct").as_deref()), (Some("mep-element"), Some("Duct 1")));
    assert_eq!((row.parent)(&snapshot, "m-duct"), Some(snapshot.mep_elements["m-duct"].storey.clone()));
    assert!(row.create.is_some() && row.delete.is_some() && row.rename.is_some() && !row.library);
    let inference = ModelInference::infer(&snapshot).expect("infers");
    let read = |key: &str| (MEP_INFERRED.iter().find(|row| row.key == key).expect("row").read)(&snapshot, &inference, "m-duct");
    assert_eq!(read("section").as_deref(), Some("300 × 200"));
    assert!(read("issues").is_some());
}
