use super::*;
use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView, NoConfig};

const MANIFEST_ADMISSION_FIXTURE: &str = include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧫️fixtures/🔣️manifest-admission.json");

fn contributions(row_id: &str) -> (String, Vec<String>) {
    let fixture: serde_json::Value = serde_json::from_str(MANIFEST_ADMISSION_FIXTURE).expect("manifest admission fixture");
    let row = fixture["rows"].as_array().expect("rows").iter().find(|row| row["id"] == row_id).expect("fixture row");
    let operators = row["operators"].as_array().expect("operators").iter().map(|operator| operator.as_str().expect("operator id").to_string()).collect();
    let json = serde_json::json!([{ "pluginId": fixture["pluginId"], "topicContribution": { "topic": "flow.extension", "payload": { "manifestJson": row["manifestJson"] } } }]).to_string();
    (json, operators)
}

/// ⚖️ LAW: the flow editor's own `setContributions` installs the pushed closure into the registry its
/// catalogue and evaluation read, publishes no store lane (no History row, no config ledger entry) and
/// invalidates the session exactly once per registry generation.
#[test]
fn a_pushed_flow_extension_closure_reaches_the_flow_editor_registry_without_a_store_lane() {
    let (json, operators) = contributions("valid-full");
    assert!(!operators.is_empty(), "the fixture row must contribute an operator, else the law proves nothing");
    let snapshot = FlowSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &NoConfig {}, window: None };
    let mut session = FlowEvalSession::new();
    let emit = handle(&SetContributions { json: json.clone(), page: 0, page_count: 1 }, &doc, &cfg, &mut session).expect("a well-formed closure is admitted");
    assert!(emit.artifact_mutations.is_empty() && emit.config_mutations.is_empty() && emit.draft_mutations.is_empty() && emit.child_emits.is_empty() && emit.effects.is_empty(), "a host contributions push publishes no store lane and no effect");
    let catalogue: serde_json::Value = serde_json::from_str(&flow::flow_neuron_kind_infos_json()).expect("operator catalogue JSON");
    let ids: Vec<&str> = catalogue.as_array().expect("catalogue array").iter().filter_map(|item| item["id"].as_str()).collect();
    for operator in &operators {
        assert!(ids.contains(&operator.as_str()), "contributed operator {operator} must be in the flow registry after setContributions; catalogue={ids:?}");
    }
    assert!(!install(&SetContributions { json, page: 0, page_count: 1 }, &mut session).expect("an unchanged re-push is admitted"), "an unchanged closure keeps the registry generation and owes no re-evaluation");
    flow::uninstall_flow_extension("admission").expect("the law leaves the process-wide registry as it found it");
    session.retire_cold();
}
