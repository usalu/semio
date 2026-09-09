use super::*;
use protocol::Inference;

//#region 🧸️Fixtures
/// 🩹️ `FormsSnapshot` composes `structure`/`results` handles (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM) so it no longer deserializes raw step/block JSON
/// directly — `semio_framework_artifact_playbook_playbook::PlaybookSpec` is the SAME `{schema,id,version,title,steps}`
/// camelCase shape, so this fixture deserializes through it instead.
fn step_with_conditional_block() -> FormsSnapshot {
    let json = r#"{
            "schema": "forms.form",
            "id": "forms",
            "version": "1",
            "title": null,
            "steps": [
                {
                    "id": "s1",
                    "title": "Step 1",
                    "blocks": [
                        { "id": "team-size", "label": "Team size", "kind": "slider" },
                        {
                            "id": "team-name",
                            "label": "Team name",
                            "kind": "text",
                            "condition": { "kind": "truthy", "expr": { "kind": "var", "name": "team-size" } }
                        }
                    ]
                }
            ]
        }"#;
    let spec = dsl::os_pack::json::from_json_str::<semio_framework_artifact_playbook_playbook::PlaybookSpec>(json).expect("valid playbook spec json");
    crate::forms_snapshot_with_state(spec.schema, spec.id, spec.version, spec.title, &spec.steps)
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = step_with_conditional_block();
    assert_eq!(FormsInference::infer(&snapshot), FormsInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(FormsInference::infer(&FormsSnapshot::default()), FormsInference::default());
}

#[semio_framework_async_macros::async_test]
async fn topology_orders_the_conditioned_block_after_its_dependency() {
    let snapshot = step_with_conditional_block();
    let inferred = FormsInference::infer(&snapshot);
    let size_index = inferred.topology.topo_order.iter().position(|id| id == "team-size").unwrap();
    let name_index = inferred.topology.topo_order.iter().position(|id| id == "team-name").unwrap();
    assert!(size_index < name_index, "team-name's condition reads team-size, so team-size must precede it");
    assert!(inferred.topology.cycle_free);
}

#[semio_framework_async_macros::async_test]
async fn empty_steps_produce_empty_topology() {
    let snapshot = FormsSnapshot::default();
    let inferred = FormsInference::infer(&snapshot);
    assert!(inferred.topology.topo_order.is_empty());
    assert_eq!(inferred.topology.node_count, 0);
    assert!(inferred.topology.cycle_free);
}
//#endregion 🧪️InferenceLaws
