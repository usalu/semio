"""⏯️ C12 prepared patch (window 3, guest freeze): the Rust twin of the TS `sameExecutionTargetV1` resume relation
(`📇️directory/🧬️schema/🦀️.rs`) and its law replaying `🧫️fixtures/📇️directory/⏯️execution-target-resume-v1.json`
(`📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs`). Idempotent. usage: python3 c12-resume-rust-patch.py [--dry-run]"""
import sys
ROOT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory"
DRY = "--dry-run" in sys.argv
SCHEMA = ROOT + "/🧬️schema/🦀️.rs"
TEST = ROOT + "/🔌️client/🧪️tests/🔬️unit/🦀️.rs"
FUNCTION = '''
/// ⏯️ Whether `next` — the fields a reconnect's fresh plan projects — names the SAME execution target under the SAME grant as
/// `current`, the lease of a child a link shortage suspended, so that child resumes instead of reopening. The active `checkpoint`
/// and the `revalidation` witness (its `directory_revision` is the hub directory's global head) legitimately advance while a
/// document stays open and may only move forward; every other field goes through [`same_lease_fields_v1`]. Twin of the TS
/// `sameExecutionTargetV1`, both replaying `🧫️fixtures/📇️directory/⏯️execution-target-resume-v1.json`.
pub fn same_execution_target_v1(current: &DocumentExecutionTargetLeaseFieldsV1, next: &DocumentExecutionTargetLeaseFieldsV1) -> bool {
    fn forward(from: Option<u64>, to: Option<u64>) -> bool {
        match (from, to) {
            (None, None) => true,
            (Some(from), Some(to)) => to >= from,
            _ => false,
        }
    }
    let (from, to) = (&current.checkpoint.baseline_frontier, &next.checkpoint.baseline_frontier);
    next.checkpoint.descriptor_digest_v1 == current.checkpoint.descriptor_digest_v1
        && to.document_id == from.document_id
        && to.head_edit_ordinal >= from.head_edit_ordinal
        && to.last_commit_seq >= from.last_commit_seq
        && next.revalidation.directory_revision >= current.revalidation.directory_revision
        && next.revalidation.membership_generation >= current.revalidation.membership_generation
        && forward(current.revalidation.session_generation, next.revalidation.session_generation)
        && forward(current.revalidation.share_generation, next.revalidation.share_generation)
        && same_lease_fields_v1(current, &DocumentExecutionTargetLeaseFieldsV1 { checkpoint: current.checkpoint.clone(), revalidation: current.revalidation, ..next.clone() })
}
'''
SCHEMA_ANCHOR = '''pub fn same_lease_fields_v1(left: &DocumentExecutionTargetLeaseFieldsV1, right: &DocumentExecutionTargetLeaseFieldsV1) -> bool {
    left == right
}
'''
LAW = '''/// ⏯️ The resume relation, driven by the language-neutral `⏯️execution-target-resume-v1` corpus over the lease corpus's manifest:
/// a suspended child resumes only when the fresh plan names the same target under the same grant, its checkpoint and revalidation
/// witness moved at most forward (twin of the TS `sameExecutionTargetV1` law).
#[test]
fn execution_target_resume_admits_only_a_forward_checkpoint_and_witness() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/📇️directory/⏯️execution-target-resume-v1.json")).expect("resume corpus");
    let lease: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution target lease corpus");
    fn merge(target: &mut serde_json::Value, patch: &serde_json::Value) {
        let Some(entries) = patch.as_object() else {
            *target = patch.clone();
            return;
        };
        if !target.is_object() {
            *target = serde_json::json!({});
        }
        let object = target.as_object_mut().expect("merge target object");
        for (key, value) in entries {
            if value.is_null() {
                object.remove(key);
            } else {
                merge(object.entry(key.clone()).or_insert(serde_json::Value::Null), value);
            }
        }
    }
    let decode = |value: &serde_json::Value| crate::os_pack::json::from_json_str::<DocumentExecutionTargetLeaseFieldsV1>(&serde_json::to_string(value).expect("fields json")).expect("lease fields");
    let current = decode(&lease["manifest"]);
    let cases = corpus["cases"].as_array().expect("resume cases");
    assert!(cases.iter().filter(|row| row["resumes"] == true).count() > 1 && cases.iter().filter(|row| row["resumes"] == false).count() > 1);
    for row in cases {
        let mut next = lease["manifest"].clone();
        merge(&mut next, &row["patch"]);
        assert_eq!(same_execution_target_v1(&current, &decode(&next)), row["resumes"].as_bool().expect("resumes"), "{}", row["case"]);
    }
}

'''
LAW_ANCHOR = "/// 🪪️ The one shared full-field lease relation, driven by the language-neutral\n"
IMPORT_OLD = "use crate::os_directory::schema::{DocumentOpenCheckpointV1, DocumentScope, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1};"
IMPORT_NEW = "use crate::os_directory::schema::{same_execution_target_v1, DocumentOpenCheckpointV1, DocumentScope, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1};"
problems, planned = [], []
schema = open(SCHEMA, encoding="utf-8").read()
if "pub fn same_execution_target_v1(" not in schema:
    if schema.count(SCHEMA_ANCHOR) != 1: problems.append("schema anchor")
    else: planned.append("schema: add same_execution_target_v1"); schema = schema.replace(SCHEMA_ANCHOR, SCHEMA_ANCHOR + FUNCTION)
test = open(TEST, encoding="utf-8").read()
if "fn execution_target_resume_admits_only_a_forward_checkpoint_and_witness" not in test:
    if test.count(LAW_ANCHOR) != 1: problems.append("law anchor")
    else: planned.append("test: add resume law"); test = test.replace(LAW_ANCHOR, LAW + LAW_ANCHOR)
if IMPORT_NEW not in test:
    if test.count(IMPORT_OLD) != 1: problems.append("import anchor")
    else: planned.append("test: import same_execution_target_v1"); test = test.replace(IMPORT_OLD, IMPORT_NEW)
print(f"planned {len(planned)} / problems {len(problems)}: {planned} {problems}")
if problems: sys.exit(1)
if not DRY:
    open(SCHEMA, "w", encoding="utf-8").write(schema)
    open(TEST, "w", encoding="utf-8").write(test)
    print("applied")
