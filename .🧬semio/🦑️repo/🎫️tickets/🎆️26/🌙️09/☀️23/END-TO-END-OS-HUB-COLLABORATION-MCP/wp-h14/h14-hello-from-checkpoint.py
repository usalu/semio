#!/usr/bin/env python3
"""🪢 H14 14c P1 (B2): a frontier-less document hello from a plan whose client seeds its document from the plan's checkpoint
pair (`closed-browser-actor`) resumes the hub's tail at that checkpoint's baseline — the store worker's documented contract
(`forgetMountedDocument`: "the hub answers with the tail from its active checkpoint"), and what the Rust store states
explicitly by naming the pair's baseline in its hello. A genesis checkpoint, a plan without a browser actor or a hello that
names its own frontier keep today's behaviour. Hub bootstrap + bin law. Idempotent; `--dry-run` reports only."""
import sys

PATH = "/Users/ueli/Documents/semio/🌎️hub/🏗️bootstrap/🦀️.rs"
TESTS = "/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"
DRY = "--dry-run" in sys.argv
files = {PATH: open(PATH, encoding="utf-8").read(), TESTS: open(TESTS, encoding="utf-8").read()}
problems, states = [], []


def edit(old, new, label, path=PATH):
    text = files[path]
    if new in text:
        states.append("done")
        return
    if text.count(old) != 1:
        problems.append(f"{label}: expected 1, found {text.count(old)}")
        states.append("problem")
        return
    files[path] = text.replace(old, new)
    states.append("replace")


edit("""    let mut frontier = frontier;
    if let Some(advertised) = frontier.as_mut() {
        if !wire_frontier_to_db(advertised, &document_id, &db_id) {
            let _ = sender.send(error_frame("frontier-document-mismatch", "hello frontier names a different document than this socket").await).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    }
""", """    let mut frontier = frontier;
    if let Some(advertised) = frontier.as_mut() {
        if !wire_frontier_to_db(advertised, &document_id, &db_id) {
            let _ = sender.send(error_frame("frontier-document-mismatch", "hello frontier names a different document than this socket").await).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    }
    let frontier = frontier.or_else(|| plan_seeded_hello_frontier(socket_grant.document_plan.as_deref(), &db_id));
""", "hello frontier")

edit("""fn wire_frontier_to_db(frontier: &mut RuntimeFrontierSummary, document_id: &str, db_id: &ProtocolArtifactId) -> bool {""", """/// @emoji 🪢️ Where a hello that names no frontier resumes: at the baseline of its plan's checkpoint when the plan's client
/// seeds its document from that checkpoint's canonical pair (a `closed-browser-actor` plan) and the checkpoint holds edits —
/// the pair already carries every edit up to it, so the tail starts after it and stays bounded by the checkpoint instead of
/// the document's whole history. `None` (the tail from the start) otherwise. Keyed by the hub's internal document key.
fn plan_seeded_hello_frontier(plan: Option<&DocumentOpenPlanAuthorityV1>, db_id: &ProtocolArtifactId) -> Option<RuntimeFrontierSummary> {
    let plan = plan?;
    let baseline = &plan.checkpoint.baseline_frontier;
    if !matches!(plan.browser_actor, os_directory::DocumentOpenBrowserActorV1::ClosedBrowserActor { .. }) || !baseline.is_edited_for(&plan.scope) {
        return None;
    }
    Some(RuntimeFrontierSummary { document_id: db_id.clone(), head_edit_ordinal: baseline.head_edit_ordinal, head_edit_id: baseline.head_edit_id.clone(), last_commit_seq: baseline.last_commit_seq, chain_hash: baseline.chain_hash.0 })
}

fn wire_frontier_to_db(frontier: &mut RuntimeFrontierSummary, document_id: &str, db_id: &ProtocolArtifactId) -> bool {""", "helper")

edit("""fn document_open_plan_test_authority(fixture: &DocumentOpenPlanLedgerFixture) -> DocumentOpenPlanAuthorityV1 {""", """/// 🪢️ A frontier-less hello resumes at the plan checkpoint's baseline exactly when the plan's client seeds from that
/// checkpoint's pair: a `closed-browser-actor` plan with an edited checkpoint answers the baseline on the hub's internal
/// key; the same plan without a browser actor, or with a genesis checkpoint, answers `None` (the tail from the start).
#[test]
fn a_frontierless_hello_resumes_at_the_checkpoint_its_client_seeds_from() {
    let fixture: DocumentOpenPlanLedgerFixture = directory::os_pack::json::from_json_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open plan fixture");
    let mut plan = document_open_plan_test_authority(&fixture);
    let db_id = db_artifact_id(&plan.scope);
    assert_eq!(plan_seeded_hello_frontier(Some(&plan), &db_id), None, "a plan without a browser actor keeps the whole tail");
    plan.browser_actor = os_directory::DocumentOpenBrowserActorV1::ClosedBrowserActor {
        schema: "semio.browser-actor/v1".into(),
        codegen_policy: "closed".into(),
        sha256: "a".repeat(64),
        source_component_sha256: "b".repeat(64),
        source_descriptor_byte_sha256: "c".repeat(64),
        policy_sha256: "d".repeat(64),
        import_interfaces: Vec::new(),
    };
    let baseline = plan.checkpoint.baseline_frontier.clone();
    assert!(baseline.head_edit_ordinal > 0);
    let resumed = plan_seeded_hello_frontier(Some(&plan), &db_id).expect("a seeding plan resumes at its checkpoint");
    assert_eq!(resumed, RuntimeFrontierSummary { document_id: db_id.clone(), head_edit_ordinal: baseline.head_edit_ordinal, head_edit_id: baseline.head_edit_id.clone(), last_commit_seq: baseline.last_commit_seq, chain_hash: baseline.chain_hash.0 });
    plan.checkpoint.baseline_frontier = directory::os_directory::ArtifactFrontier { document_id: plan.scope.document_id.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: directory::os_directory::ArtifactHash([0; 32]) };
    assert_eq!(plan_seeded_hello_frontier(Some(&plan), &db_id), None, "a genesis checkpoint keeps the tail from the start");
    assert_eq!(plan_seeded_hello_frontier(None, &db_id), None);
}

fn document_open_plan_test_authority(fixture: &DocumentOpenPlanLedgerFixture) -> DocumentOpenPlanAuthorityV1 {""", "law", path=TESTS)

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if DRY:
    print("dry-run clean")
elif "replace" in states:
    for path, text in files.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("nothing to apply")
