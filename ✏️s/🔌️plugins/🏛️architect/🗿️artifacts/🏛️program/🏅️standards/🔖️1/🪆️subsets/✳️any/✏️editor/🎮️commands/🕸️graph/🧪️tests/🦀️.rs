use super::node_graph_edit::{NodeGraphEdit, handle};
use crate::editor::architect::{catalog::new_adjacency, config::ArchitectConfig};
use crate::{EntityId, registers::AdjacencyKind, schema::mutations::ProgramMutation};
use semio_framework_plugin::{ArtifactView, ConfigView};

#[test]
fn graph_tools_follow_the_language_neutral_transaction_law() {
    let fixture = semio_framework_pack_json::parse(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🛠️graph-tool/🔣️.json")), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture");
    for case in fixture.get("cases").unwrap().as_array().unwrap() {
        let mut program = crate::sample_plugin();
        program.elements[0].header.id = EntityId("a".into());
        program.elements[1].header.id = EntityId("b".into());
        program.adjacencies = vec![new_adjacency(&program, &EntityId("a".into()), &EntityId("b".into()), AdjacencyKind::Preferred)];
        program.adjacencies[0].header.id = EntityId("edge".into());
        if case.get("linked").and_then(|value| value.as_bool()) == Some(false) { program.adjacencies.clear(); }
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::with_operation(&program, &history, semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "doc".into(), operation_id: 1, generation: 1, canonical_base_revision: [7; 32], authoring_seed: "seed".into() });
        let config = ArchitectConfig::default();
        let payload = NodeGraphEdit { operations_json: semio_framework_pack_json::to_json_string(case.get("rows").unwrap()) };
        let result = handle(&payload, &doc, &ConfigView { snapshot: &config, window: None });
        if case.get("refused").and_then(|value| value.as_bool()) == Some(true) {
            assert!(result.is_err(), "{}", case.get("name").unwrap());
            continue;
        }
        let emit = result.expect("valid rows dispatch");
        let kinds = emit.artifact_mutations.iter().map(|leaf| match leaf {
            ProgramMutation::ConnectAdjacency(_) => "connect-adjacency",
            ProgramMutation::DisconnectAdjacency(_) => "disconnect-adjacency",
            ProgramMutation::DeleteProgramElement(_) => "delete-program-element",
            other => panic!("unexpected graph leaf {other:?}"),
        }).collect::<Vec<_>>();
        let expected = case.get("mutations").unwrap().as_array().unwrap().iter().map(|kind| kind.as_str().unwrap()).collect::<Vec<_>>();
        assert_eq!(kinds, expected, "{}", case.get("name").unwrap());
        for leaf in &emit.artifact_mutations {
            let (next, outcome) = crate::schema::mutations::apply_program_mutation_outcome(&program, leaf);
            program = next;
            assert!(!outcome.messages().iter().any(|message| message.level == semio_framework_diagnostic::Severity::Fatal), "a graph tool leaf applies on its ordered base");
        }
        assert_eq!(emit.transaction.as_ref().map(|transaction| transaction.tool.as_str()), (!kinds.is_empty()).then_some("s.architect.program@1/*#editor#nodeGraphEdit"));
    }
}
