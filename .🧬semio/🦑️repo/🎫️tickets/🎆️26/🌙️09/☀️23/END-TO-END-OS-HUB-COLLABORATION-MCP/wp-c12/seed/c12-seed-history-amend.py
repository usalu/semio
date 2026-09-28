"""🌱️ C12 amendment to the landed seed-history law (H13 review): the folded store also carries the replica's OWN edits — one
before and one after the hub tail — so the initializer runs STEP 8's mixed shape (15 remote edits named after their mutation id +
2 local edits). MODIFIES the law in place, region-guarded to `a_document_folded_from_the_hub_tail_initializes_again`; never inserts
a second copy. Idempotent; `--dry-run` prints the plan; exit 1 when the region or an anchor is missing or ambiguous."""
import sys

PATH = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs"
START = "async fn a_document_folded_from_the_hub_tail_initializes_again() {\n"
END = "//#endregion 🔖️HubTailInitialization\n"
DOC_OLD = "/// 🌱️ LAW: a document a replica folded from the hub's tail initializes again through this store initializer."
DOC_NEW = "/// 🌱️ LAW: a document a replica folded from the hub's tail — beside its own local edits, STEP 8's mixed shape — initializes again through this store initializer."
BODY_OLD = """    for value in tail {
        folded.ingest_remote(hub_tail_envelope(value)).await.expect("every operation and transition of the hub tail folds");
    }
    let remote_edits = folded.envelope().vcs.edits.iter().filter(|edit| edit.mutation_meta.first().and_then(|meta| meta.mutation_id.as_ref()).is_some_and(|id| id.0 == edit.id)).count();
    assert_eq!(remote_edits, 15, "every folded operation is an edit named after its own mutation id — the shape SeedHistory must seed once");
"""
BODY_NEW = """    folded.dispatch(store::ArtifactCommand::Apply { mutations: vec![schema::mutations::edit_text("user1 typed before the hub tail".to_string())], description: None }).await.expect("a locally authored edit before the tail");
    for value in tail {
        folded.ingest_remote(hub_tail_envelope(value)).await.expect("every operation and transition of the hub tail folds");
    }
    folded.dispatch(store::ArtifactCommand::Apply { mutations: vec![schema::mutations::edit_text("user1 typed after the hub tail".to_string())], description: None }).await.expect("a locally authored edit after the tail");
    let (remote, local) = folded.envelope().vcs.edits.iter().fold((0, 0), |(remote, local), edit| if edit.mutation_meta.first().and_then(|meta| meta.mutation_id.as_ref()).is_some_and(|id| id.0 == edit.id) { (remote + 1, local) } else { (remote, local + 1) });
    assert_eq!((remote, local), (15, 2), "STEP 8's mixed shape: every folded operation is an edit named after its own mutation id, the replica's own edits are local (entry id + operation id)");
"""

text = open(PATH, encoding="utf-8").read()
if text.count(START) != 1 or text.count(END) != 1:
    print(f"FAIL region: law function ×{text.count(START)}, region end ×{text.count(END)} (want exactly one each)")
    sys.exit(1)
begin = text.rfind("\n//#region 🔖️HubTailInitialization\n", 0, text.index(START))
end = text.index(END) + len(END)
if begin < 0:
    print("FAIL region: no HubTailInitialization region before the law")
    sys.exit(1)
region = text[begin:end]
plan = []
for label, old, new in (("doc", DOC_OLD, DOC_NEW), ("body", BODY_OLD, BODY_NEW)):
    if new in region:
        plan.append(f"present  {label}")
        continue
    if region.count(old) != 1:
        print(f"FAIL {label}: anchor ×{region.count(old)} inside the law region")
        sys.exit(1)
    region = region.replace(old, new)
    plan.append(f"modify   {label}")
print("\n".join(plan))
if "--dry-run" not in sys.argv and any(line.startswith("modify") for line in plan):
    open(PATH, "w", encoding="utf-8").write(text[:begin] + region + text[end:])
    print("written")
