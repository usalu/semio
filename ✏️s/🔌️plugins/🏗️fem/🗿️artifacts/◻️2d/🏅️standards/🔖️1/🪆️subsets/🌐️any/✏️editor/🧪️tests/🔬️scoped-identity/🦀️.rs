use semio_plugin::{ArtifactView, ComponentTree, ConfigView, HistoryView, NoConfig, TreeWindows, ViewModel};
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::{patch_load, remove_selection};
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::{fem2d_entity_kind, fem2d_load_owner, gumball, Fem2dInteractionSnapshot, FEM2D_GRANULARITY_MATERIAL, FEM2D_GRANULARITY_NODE};
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model as model_window;
use semio_s_artifact_fem_2d_editor_panels::editor::fem2d::panels::{artifact, inspection};
use semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::fem2d_labels;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::{Fem2dSnapshot, Fem2dSnapshotRetiring, Fem2dSnapshotValue};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;

fn identity_fixture() -> (serde_json::Value, Fem2dSnapshotValue) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🎮️commands/🗂️remove-selection/🧪️tests/🔬️unit/🔣️.json")).expect("shared JSON identity scenarios");
    let value = semio_framework_pack_json::from_json_str(&fixture["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authoring snapshot wire");
    (fixture, value)
}

fn sealed(value: Fem2dSnapshotValue) -> Fem2dSnapshotRetiring {
    Fem2dSnapshotRetiring::new(Fem2dSnapshot::from_reference(value).expect("the authoring value seals"))
}

fn layers_json(document: &Fem2dSnapshot, interaction: &Fem2dInteractionSnapshot) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(model_window::fem2d_structure_layers_with(document, "#38bdf8", "#94a3b8", "#f97316", interaction)))
}

#[test]
fn scoped_entity_identity_prevents_cross_kind_and_case_collisions() {
    let _arena = semio_ui_contract_values::testkit::UiArenaFixture::new();
    let (fixture, mut value) = identity_fixture();
    let collision = &fixture["collision"];
    value.materials[0].id = collision["materialId"].as_str().expect("shared identity").into();
    let material_id = value.materials[0].id.clone();
    match &mut value.elements[0] {
        FemElement::Bar { material_id: id, .. } | FemElement::Beam { material_id: id, .. } => *id = material_id,
    }
    let mut live = value.load_cases[0].clone();
    live.id = collision["liveCaseId"].as_str().expect("peer case").into();
    live.name = "Live".into();
    value.load_cases.push(live);
    value.validate_interaction_identities().expect("distinct typed namespaces");
    let document = sealed(value);
    document.validate_interaction_identities().expect("the sealed snapshot admits the same namespaces");
    let target = |name: &str| collision["targets"][name].as_str().expect("canonical target");
    assert_eq!(fem2d_entity_kind(&document, target("node")), Some(FEM2D_GRANULARITY_NODE));
    assert_eq!(fem2d_entity_kind(&document, target("material")), Some(FEM2D_GRANULARITY_MATERIAL));
    assert_eq!(fem2d_entity_kind(&document, "n1"), None, "raw ids are outside the interaction protocol");
    assert_eq!(fem2d_load_owner(&document, target("deadLoad")).map(|(case, _)| case), Some("dead"));
    assert_eq!(fem2d_load_owner(&document, target("liveLoad")).map(|(case, _)| case), Some("live"));
    assert!(gumball::fem2d_transform_targets(&document, &[target("material").into()]).is_empty());
    assert_eq!(gumball::fem2d_transform_targets(&document, &[target("node").into()]).node_ids, ["n1"]);
    let view = ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    let labels = fem2d_labels(&view);
    let material = inspection::render(&document, &Fem2dInteractionSnapshot { selected_ids: vec![target("material").into()], ..Default::default() }, labels, &TreeWindows::unhosted()).expect("typed material inspector");
    let material = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(ComponentTree::new(material)).expect("inspector projection");
    assert!(material.contains("fem2d-play-inspection.material.name.input") && !material.contains("fem2d-play-inspection.node.x.input"));
    for (locale, key) in [(semio_ui_locale::Locale::En, "en"), (semio_ui_locale::Locale::De, "de")] {
        let view = ViewModel::new(locale, semio_ui_locale::Terminology::Native);
        let selected_ids = ["node", "material", "deadLoad", "liveLoad"].iter().map(|key| target(key).to_string()).collect();
        let panel = inspection::render(&document, &Fem2dInteractionSnapshot { selected_ids, ..Default::default() }, fem2d_labels(&view), &TreeWindows::unhosted()).expect("typed selection labels");
        let panel = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(ComponentTree::new(panel)).expect("selection projection");
        for label in collision["selectionLabels"][key].as_array().expect("portable localized selection labels") {
            assert!(panel.contains(label.as_str().unwrap()), "{locale:?}: {panel}");
        }
    }

    let tree = artifact::render(&document, &Fem2dInteractionSnapshot::default(), labels, &TreeWindows::unhosted()).expect("case-owned load tree");
    let tree = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(ComponentTree::new(tree)).expect("tree projection");
    assert!(tree.contains(target("deadLoad")) && tree.contains(target("liveLoad")));
    let history = HistoryView::empty();
    let document_view = ArtifactView::new(&*document, &history);
    let config = NoConfig::default();
    let config_view = ConfigView { snapshot: &config, window: None };
    let patch = patch_load::handle(&patch_load::PatchLoad { id: target("liveLoad").into(), field: "wy".into(), value: "-2000".into() }, &document_view, &config_view).expect("typed load edit");
    let [Fem2dMutation::ChangeLoad(change)] = patch.artifact_mutations.as_slice() else { panic!("one sparse load edit") };
    assert_eq!((&*change.case_id, &*change.id), ("live", "l1"));
    let removed = remove_selection::remove_ids(&document_view, &[target("liveLoad").into()]).expect("case-owned load removal");
    assert_eq!(removed.artifact_mutations.len(), 1);
    let mut after = document.to_reference();
    semio_s_artifact_fem_2d_mutations::central_apply::apply_fem2d_mutation(&mut after, &removed.artifact_mutations[0]).expect("central deletion");
    assert_eq!(after.load_cases[0], document.load_cases[0]);
    assert!(after.load_cases[1].loads.is_empty());
    let highlights = layers_json(&document, &Fem2dInteractionSnapshot { selected_ids: vec![target("material").into()], ..Default::default() });
    assert!(!highlights.contains("sel-node-n1"));
}
