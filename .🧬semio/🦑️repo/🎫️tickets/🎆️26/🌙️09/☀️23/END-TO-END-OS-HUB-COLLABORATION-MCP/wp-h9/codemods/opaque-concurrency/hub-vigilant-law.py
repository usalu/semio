#!/usr/bin/env python3
"""⚖️ Hub law + language-agnostic fixture of the opaque-concurrency patch (lands with the db and store halves).

Installs `🌎️hub/🧫️fixtures/⚔️vigilant-concurrent-edit-v1/🔣️.json` and the bin law
`a_vigilant_hub_refuses_an_opaque_write_authored_without_seeing_the_head` that runs every fixture row.

usage: hub-vigilant-law.py [--reverse] [--dry-run]
"""
import pathlib
import sys

ROOT = next(parent for parent in pathlib.Path(__file__).resolve().parents if (parent / ".git").exists())
HERE = pathlib.Path(__file__).resolve().parent
FIXTURE = ROOT / "🌎️hub/🧫️fixtures/⚔️vigilant-concurrent-edit-v1/🔣️.json"
TESTS = ROOT / "🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"
ANCHOR = "/// 🔬️ The refusals a delegation exchange must be unable to distinguish, and the one it must. A"
LAW = r'''/// ⚔️ Two authors write the same opaque document from the same base (fixture `⚔️vigilant-concurrent-edit-v1`): a
/// write whose dependencies miss the document's head was authored without seeing it — `Vigilant` refuses it with the
/// typed `mutation.clamped` warning, `Normal` accepts and reports it, a write that saw the head is accepted.
#[test]
fn a_vigilant_hub_refuses_an_opaque_write_authored_without_seeing_the_head() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚔️vigilant-concurrent-edit-v1/🔣️.json")).expect("vigilant fixture");
        for (index, row) in fixture["rows"].as_array().expect("rows").iter().enumerate() {
            let mut state = test_state().await;
            state.merge_policy = match row["policy"].as_str().expect("policy") {
                "vigilant" => protocol::MergePolicy::Vigilant,
                "normal" => protocol::MergePolicy::Normal,
                other => panic!("undeclared policy {other}"),
            };
            let token = seed_author_token(&state).await;
            let document = format!("vigilant-{index}");
            announce_document_for_test(&state, STUDIO, &document).await;
            let grant = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document.clone())), bearer_headers(&token), State(state.clone())).await.expect("socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let (mut socket, _) = connect_async(document_socket_request(&format!("ws://{addr}/scopes/{STUDIO}%2F{document}/document/ws"), &token)).await.expect("socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
            let opaque = |id: &str, dependencies: Vec<protocol::MutationId>| MutationEnvelope {
                mutation_id: protocol::MutationId(id.to_string()),
                document_id: WireArtifactId(document.clone()),
                actor: ActorId(grant.actor_id.clone()),
                dependencies,
                diff: protocol::ArtifactDiff { schema: protocol::SchemaId("test.v1".into()), payload: id.as_bytes().to_vec() },
                inverse: protocol::InverseMutation { schema: protocol::SchemaId("test.v1".into()), payload: Vec::new() },
                timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
            };
            let mut outcomes = Vec::new();
            for (batch, envelope) in [(1u64, opaque("base", Vec::new())), (2, opaque("first", vec![protocol::MutationId("base".into())])), (3, opaque("second", vec![protocol::MutationId(if row["secondDependsOn"] == "first" { "first".into() } else { "base".into() })]))] {
                socket.send(client_binary(&ClientFrame::Commands { batch_id: batch, envelopes: vec![envelope] }, Lane::Command).await).await.expect("command");
                loop {
                    if let ServerFrame::Ack { batch_id, stages, .. } = next_server_frame_at(&mut socket, "vigilant ack").await {
                        if batch_id == batch {
                            outcomes.push(stages.last().cloned().expect("applied stage"));
                            break;
                        }
                    }
                }
            }
            let name = row["name"].as_str().expect("name");
            let accepted = |stage: &AckStage| matches!(stage, AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Accepted));
            assert!(accepted(&outcomes[0]) && accepted(&outcomes[1]), "{name}: the base and the first write are accepted: {outcomes:?}");
            match row["second"].as_str().expect("second") {
                "accepted" => assert!(accepted(&outcomes[2]), "{name}: {outcomes:?}"),
                "refused" => match &outcomes[2] {
                    AckStage::Applied { outcome } => match outcome.as_ref() {
                        ApplyOutcome::Rejected { messages, .. } => assert!(messages.iter().any(|message| message.code.0 == row["code"].as_str().expect("code") && format!("{:?}", message.level).to_lowercase() == row["level"].as_str().expect("level")), "{name}: {messages:?}"),
                        other => panic!("{name}: expected a typed refusal, got {other:?}"),
                    },
                    other => panic!("{name}: expected an applied stage, got {other:?}"),
                },
                other => panic!("undeclared outcome {other}"),
            }
        }
    });
}

'''


def main() -> int:
    reverse = "--reverse" in sys.argv
    dry = "--dry-run" in sys.argv
    text = TESTS.read_text(encoding="utf-8")
    if reverse:
        if text.count(LAW) != 1:
            print("law not installed exactly once", file=sys.stderr)
            return 1
        if not dry:
            TESTS.write_text(text.replace(LAW, "", 1), encoding="utf-8")
            if FIXTURE.exists():
                FIXTURE.unlink()
                FIXTURE.parent.rmdir()
        print("reverse: ok")
        return 0
    if text.count(ANCHOR) != 1 or LAW in text:
        print(f"anchor found {text.count(ANCHOR)} times; law present: {LAW in text}", file=sys.stderr)
        return 1
    if not dry:
        TESTS.write_text(text.replace(ANCHOR, LAW + ANCHOR, 1), encoding="utf-8")
        FIXTURE.parent.mkdir(exist_ok=True)
        FIXTURE.write_bytes((HERE / "files/vigilant-concurrent-edit-v1.json").read_bytes())
    print("forward: ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
