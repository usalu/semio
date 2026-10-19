//! 🧪️ Namespaced interaction targets preserve exact entity and load-case ownership.
use super::*;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_load;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::remove_selection;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::HistoryView;
use semio_framework_plugin::NoConfig;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;

fn collision_document() -> Fem3dSnapshot {
    let fixture: DslValue = semio_framework_pack_json::from_json_str(include_str!("../../../🎮️commands/🗂️remove-selection/🧪️tests/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut doc = Fem3dSnapshot::from_value(fixture.get("snapshot").unwrap().clone()).unwrap();
    doc.nodes[0].id = "same".into();
    doc.materials[0].id = "same".into();
    doc.sections[0].id = "same".into();
    doc.elements[0] = FemElement::Bar { id: "same".into(), start: "same".into(), end: "n2".into(), material_id: "same".into(), section_id: "same".into() };
    doc.solids[0].id = "same".into();
    doc.solids[0].material_id = "same".into();
    doc.supports[0].id = "same".into();
    doc.supports[0].node_id = "same".into();
    doc.load_cases[0].id = "same".into();
    doc.load_cases[0].loads = vec![FemLoad::Nodal { id: "共:1".into(), node_id: "n2".into(), dof: crate::FemDof::Tz, value: -1000.0 }];
    let mut other = doc.load_cases[0].clone();
    other.id = "other:case".into();
    other.loads = vec![FemLoad::Nodal { id: "共:1".into(), node_id: "same".into(), dof: crate::FemDof::Tz, value: -2000.0 }];
    doc.load_cases.push(other);
    doc.combinations[0].id = "same".into();
    doc.combinations[0].terms = [("same".into(), 1.5)].into_iter().collect();
    doc
}

#[test]
fn scoped_targets_separate_all_entity_kinds_and_repeated_load_ids() {
    let fixture: DslValue = semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let doc = collision_document();
    for row in fixture.get("targets").unwrap().as_array().unwrap() {
        let identity = row.get("identity").unwrap().as_str().unwrap();
        let segments = Vec::<String>::from_value(row.get("segments").unwrap().clone()).unwrap();
        assert_eq!(protocol::wire::encode_scoped_identity(&segments.iter().map(String::as_str).collect::<Vec<_>>()).unwrap(), identity);
        assert_eq!(fem3d_entity_kind(&doc, identity), row.get("kind").unwrap().as_str());
        assert_eq!(fem3d_entity_id(identity), segments.last().map(String::as_str));
    }
    for row in fixture.get("invalid").unwrap().as_array().unwrap() { assert!(fem3d_entity_kind(&doc, row.as_str().unwrap()).is_none()); }
    let first = fem3d_load_target("same", "共:1").unwrap();
    let second = fem3d_load_target("other:case", "共:1").unwrap();
    assert_ne!(first, second);
    assert_eq!(fem3d_load_owner(&doc, &first).unwrap().0, "same");
    assert_eq!(fem3d_load_owner(&doc, &second).unwrap().0, "other:case");
    assert_eq!(fem3d_entity_point(&doc, &first), Some([2.0, 0.0, 0.0]));
    assert_eq!(fem3d_entity_point(&doc, &second), Some([0.0; 3]));
    let node = fem3d_entity_target("node", "same").unwrap();
    let material = fem3d_entity_target("material", "same").unwrap();
    assert_eq!(gumball::fem3d_transform_targets(&doc, &[node]), gumball::Fem3dTransformTargets { node_ids: vec!["same".into()], solid_ids: Vec::new() });
    assert!(gumball::fem3d_transform_targets(&doc, &[material]).is_empty());
    eprintln!("[DEBUG] FEM3d canonical targets separate all namespaces and repeated UTF8 load IDs by case");
}

#[test]
fn scoped_load_commands_modify_only_the_addressed_case_and_reject_raw_ids() {
    let doc = collision_document();
    let history = HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let config = NoConfig::default();
    let config = ConfigView { snapshot: &config, window: None };
    let identity = fem3d_load_target("other:case", "共:1").unwrap();
    let patch = patch_load::handle(&patch_load::PatchLoad { id: identity.clone(), field: "value".into(), value: "-3000".into() }, &view, &config).unwrap();
    let after = crate::vcs::apply_mutation(&doc, &patch.artifact_mutations[0]).unwrap().0;
    assert_eq!(after.load_cases[0], doc.load_cases[0]);
    assert!(matches!(after.load_cases[1].loads[0], FemLoad::Nodal { value: -3000.0, .. }));
    let removed = remove_selection::handle(&remove_selection::RemoveSelection { ids: vec![identity] }, &view, &config).unwrap();
    let after = crate::vcs::apply_mutation(&doc, &removed.artifact_mutations[0]).unwrap().0;
    assert_eq!(after.load_cases[0], doc.load_cases[0]);
    assert!(after.load_cases[1].loads.is_empty());
    assert!(patch_load::handle(&patch_load::PatchLoad { id: "共:1".into(), field: "value".into(), value: "1".into() }, &view, &config).is_err());
    for malformed in ["same", "scope:1:7:unknown4:same", "scope:1:4:node4:same1:x"] {
        assert!(remove_selection::handle(&remove_selection::RemoveSelection { ids: vec![fem3d_entity_target("material", "same").unwrap(), malformed.into()] }, &view, &config).is_err());
    }
    eprintln!("[DEBUG] FEM3d scoped inspector and delete commands preserve peer load cases and atomically refuse malformed targets");
}

#[test]
fn viewport_identity_collisions_are_distinct_and_unencodable_ids_refuse_scene() {
    let doc = collision_document();
    let scene = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_scene_parts(&doc, None, 1.0, None).unwrap();
    let instances = semio_framework_pack_json::parse(&scene.1, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let targets: std::collections::HashSet<_> = instances.as_array().unwrap().iter().filter_map(|instance| instance.get("interactionId").or_else(|| instance.get("id")).and_then(semio_framework_pack_json::Value::as_str)).collect();
    for kind in ["node", "element", "solid", "support"] { assert!(targets.contains(fem3d_entity_target(kind, "same").unwrap().as_str())); }
    for case in ["same", "other:case"] { assert!(targets.contains(fem3d_load_target(case, "共:1").unwrap().as_str())); }
    let mut malformed = doc;
    malformed.nodes[0].id.clear();
    assert!(semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_scene_parts(&malformed, None, 1.0, None).is_err());
    eprintln!("[DEBUG] FEM3d viewport hit identities preserve namespace and load ownership with explicit scene admission failure");
}

/// 🛂️ Public cold JSON and metered typed admission enforce the same owner namespace contract.
#[test]
fn public_json_identity_admission_refuses_invalid_owners_and_controlled_scratch_is_metered() {
    let fixture: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owner admission vectors");
    let valid = collision_document();
    let read = |document: &Fem3dSnapshot| semio_framework_pack_json::from_json_str::<Fem3dSnapshot>(&semio_framework_pack_json::to_json_string(document), semio_framework_pack_json::JsonMemberPolicy::Reject);
    assert_eq!(read(&valid).expect("namespace collisions admitted"), valid);
    for row in fixture["invalidOwnerCases"].as_array().expect("invalid owner cases") {
        let mut invalid = valid.clone();
        match row["kind"].as_str().expect("owner case") {
            "emptyNode" => invalid.nodes[0].id.clear(),
            "duplicateNode" => invalid.nodes.push(valid.nodes[0].clone()),
            "duplicateLoad" => invalid.load_cases[0].loads.push(valid.load_cases[0].loads[0].clone()),
            "oversizedNode" => invalid.nodes[0].id = "x".repeat(row["bytes"].as_u64().expect("byte bound") as usize),
            other => panic!("unknown owner admission vector {other}"),
        }
        assert!(read(&invalid).is_err(), "public JSON must refuse {}", row["kind"].as_str().unwrap());
    }
    let mut observe = |_| true;
    let mut granted = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1 << 20, &mut observe);
    valid.validate_interaction_identities_controlled(&mut granted).expect("funded identity scratch");
    assert!(granted.owned_bytes() >= 10 * std::mem::size_of::<(&str, &str, &str)>());
    let mut observe = |_| true;
    let mut unfunded = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(0, &mut observe);
    assert!(valid.validate_interaction_identities_controlled(&mut unfunded).is_err());
    assert_eq!(unfunded.owned_bytes(), 0);
    let mut cancel = |_| false;
    let mut canceled = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1 << 20, &mut cancel);
    assert!(valid.validate_interaction_identities_controlled(&mut canceled).is_err());
    assert_eq!(canceled.owned_bytes(), 0);
    eprintln!("[DEBUG] FEM3d public JSON refused four invalid owner vectors; native identity scratch was explicitly funded, zero-budget refused and cancellation preserved ownership");
}

#[test]
fn snapshot_identity_admission_preserves_namespaces_and_refuses_invalid_owners() {
    use semio_framework_os_kernel::os_store::ArtifactDsl;
    use semio_framework_os_kernel::os_store::ArtifactPack;
    let doc = collision_document();
    assert!(doc.validate_interaction_identities().is_ok());
    assert_eq!(Fem3dSnapshot::decode_pack(&doc.encode_pack()).unwrap(), doc);
    assert_eq!(Fem3dSnapshot::parse_dsl(&doc.print_dsl()).unwrap(), doc);
    let mut duplicate = doc.clone();
    duplicate.nodes.push(doc.nodes[0].clone());
    assert!(duplicate.validate_interaction_identities().is_err());
    assert!(Fem3dSnapshot::decode_pack(&duplicate.encode_pack()).is_err());
    assert!(Fem3dSnapshot::parse_dsl(&duplicate.print_dsl()).is_err());
    let mut duplicate_load = doc.clone();
    duplicate_load.load_cases[0].loads.push(doc.load_cases[0].loads[0].clone());
    assert!(duplicate_load.validate_interaction_identities().is_err());
    let mut oversized = doc.clone();
    oversized.nodes[0].id = "x".repeat(protocol::wire::SCOPED_IDENTITY_MAX_BYTES);
    assert!(oversized.validate_interaction_identities().is_err());
    let invalid_node = semio_s_fem_node::volumetric::FemNodeValue { id: String::new(), x: 1.0, y: 2.0, z: 3.0 };
    let operation = semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation::CreateNode(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node::CreateNode { node: invalid_node, index: None });
    assert_eq!(protocol::Mutation::diff(&operation, &doc).diff(), &Default::default());
    assert_eq!(crate::vcs::apply_mutation(&doc, &operation).unwrap().0, doc);
    eprintln!("[DEBUG] FEM3d snapshot text/binary admission accepts scoped collisions and refuses duplicate, empty or oversized owners");
}
