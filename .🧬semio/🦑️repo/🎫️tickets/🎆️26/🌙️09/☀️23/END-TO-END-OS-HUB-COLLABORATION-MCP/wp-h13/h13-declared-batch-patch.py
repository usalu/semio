#!/usr/bin/env python3
"""📏️ H13 session 14c: every batch the document backbone declares legal commits; no size refusal ever reads as transient.

Live root cause (hub 8010 on p24, `transient-probe-8010-1.txt`): a writer's 600-envelope outbox (< 8 192 envelopes / 256 KiB,
declared legal) was refused by the db engine's submit credit (`ARTIFACT_SUBMIT_BATCH_ITEMS` = 256), and `SubmitFuture::submit`
wrapped that size-permanent `LimitExceeded` as `DbError::Unavailable` — so the hub answered `hub.unavailable` and the client
resent the same batch forever. Now (kernel-db, hub-closure-only):
- the engine's submit credits derive from `protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_*` (envelopes, dependencies, target
  segments, bytes, plus the hub's re-keyed document id per envelope), the credit also counts target segments + `observed`, and
  `SubmitFuture::submit` keeps the admission error's own kind (only a capacity shortage is `Unavailable`);
- the hub refuses a `Commands` batch the backbone does not declare legal (the wire's own exact decoder over its canonical
  encoding) PERMANENTLY with the one `HubBatchLimitRefusalMessageV1` (`hub.batch-limit`, level error, schema-first in
  `🚧️refusal`), and maps every engine `LimitExceeded` to the same permanent message.
Laws: engine `a_declared_maximal_batch_commits_through_the_database_and_an_over_declared_one_is_refused_permanently`; hub bin
`a_batch_beyond_its_declaration_is_refused_permanently_and_never_as_transient`; refusal unit
`the_batch_limit_refusal_is_the_declared_schema_message`; fixture `⏳️transient-apply-refusal-v1` gains the batch-limit cases and
the TS Ajv oracle checks them. Idempotent; `--dry-run` writes nothing.
"""
import json
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs"
ENGINE_TESTS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🦀️.rs"
REFUSAL = ROOT / "🌎️hub/🚧️refusal/🦀️.rs"
REFUSAL_TESTS = ROOT / "🌎️hub/🚧️refusal/🧪️tests/🔬️unit/🦀️.rs"
SCHEMA = ROOT / "🌎️hub/🚧️refusal/🧬️schema/🔣️.json"
FIXTURE = ROOT / "🌎️hub/🚧️refusal/🧫️fixtures/⏳️transient-apply-refusal-v1/🔣️.json"
ORACLE = ROOT / "🌎️hub/🧪️tests/🚧️hostile-input/🟦️.ts"
BOOTSTRAP = ROOT / "🌎️hub/🏗️bootstrap/🦀️.rs"
BIN_TESTS = ROOT / "🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"

ENGINE_CONSTS_OLD = """const ARTIFACT_SUBMIT_OPERATION_ITEMS: usize = 64;
const ARTIFACT_SUBMIT_PAGE_BYTES: u64 = 16 * 1024;
const ARTIFACT_SUBMIT_OPERATION_PAGES: u64 = 64;
const ARTIFACT_SUBMIT_OPERATION_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_OPERATION_PAGES;
const ARTIFACT_SUBMIT_TOTAL_PAGES: u64 = 1024;
const ARTIFACT_SUBMIT_TOTAL_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_TOTAL_PAGES;
const ARTIFACT_SUBMIT_BATCH_ITEMS: usize = 256;
const ARTIFACT_SUBMIT_NESTED_ITEMS: usize = 4096;
"""
ENGINE_CONSTS_NEW = """const ARTIFACT_SUBMIT_OPERATION_ITEMS: usize = 64;
const ARTIFACT_SUBMIT_PAGE_BYTES: u64 = 16 * 1024;
/// 📐️ One submit holds any batch the document backbone declares legal (`protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_*`): a
/// writer's whole post-cut outbox arrives as one batch and must commit, so the envelope, nested-item and byte credits are the
/// declaration's own maxima — never a smaller engine-side cap a declared batch could hit.
const ARTIFACT_SUBMIT_BATCH_ITEMS: usize = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES;
const ARTIFACT_SUBMIT_NESTED_ITEMS: usize = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS;
/// 📐️ Owner bytes of the largest declared batch: the credit page, its fixed owners (envelopes, dependency ids, target
/// segments), every declared byte, and one declared identifier per envelope for the document id the hub re-keys.
const ARTIFACT_SUBMIT_DECLARED_BATCH_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES
    + (protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES * size_of::<protocol::MutationEnvelope>()
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES * size_of::<protocol::MutationId>()
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS * size_of::<String>()
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES * protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES) as u64;
const ARTIFACT_SUBMIT_OPERATION_PAGES: u64 = ARTIFACT_SUBMIT_DECLARED_BATCH_BYTES.div_ceil(ARTIFACT_SUBMIT_PAGE_BYTES);
const ARTIFACT_SUBMIT_OPERATION_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_OPERATION_PAGES;
/// 📐️ In-flight submit owners across every document: four declared-maximal batches at once; a fifth waits (`Unavailable`).
const ARTIFACT_SUBMIT_TOTAL_PAGES: u64 = ARTIFACT_SUBMIT_OPERATION_PAGES * 4;
const ARTIFACT_SUBMIT_TOTAL_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_TOTAL_PAGES;
"""

CREDIT_OLD = """        for dependency in &envelope.dependencies {
            bytes = bytes.checked_add(dependency.0.capacity() as u64).ok_or(DbError::LimitExceeded("artifact submit dependency byte credit"))?;
        }
    }
"""
CREDIT_NEW = """        for dependency in &envelope.dependencies {
            bytes = bytes.checked_add(dependency.0.capacity() as u64).ok_or(DbError::LimitExceeded("artifact submit dependency byte credit"))?;
        }
        items = items.checked_add(envelope.target.len()).ok_or(DbError::LimitExceeded("artifact submit nested items"))?;
        if items > ARTIFACT_SUBMIT_NESTED_ITEMS {
            return Err(DbError::LimitExceeded("artifact submit nested item credit"));
        }
        let target_owner_bytes = envelope.target.capacity().checked_mul(size_of::<String>()).ok_or(DbError::LimitExceeded("artifact submit target owner bytes"))?;
        bytes = bytes
            .checked_add(target_owner_bytes as u64)
            .and_then(|value| value.checked_add(envelope.observed.as_ref().map_or(0, |observed| observed.0.capacity() as u64)))
            .ok_or(DbError::LimitExceeded("artifact submit target byte credit"))?;
        for segment in &envelope.target {
            bytes = bytes.checked_add(segment.capacity() as u64).ok_or(DbError::LimitExceeded("artifact submit target byte credit"))?;
        }
    }
"""

ENGINE_LAW = r'''
/// 📏️ C12 P1 live (hub 8010 on p24: a 600-envelope outbox answered `hub.unavailable` forever): the largest batch the document
/// backbone declares legal (the wire's own exact decoder admits it) commits through a production-profile `Database` →
/// `ArtifactHandle::submit` as one transaction (the hub's socket judges the declared bytes), and a batch of one envelope more
/// than the declared count is refused PERMANENTLY (`LimitExceeded`), never as the transient `Unavailable` a client would resend
/// without end.
#[semio_framework_async_macros::async_test]
async fn a_declared_maximal_batch_commits_through_the_database_and_an_over_declared_one_is_refused_permanently() {
    if !crate::db_storage::process_isolated_law("db_engine::tests::a_declared_maximal_batch_commits_through_the_database_and_an_over_declared_one_is_refused_permanently") {
        return;
    }
    let _history_capacity = db_artifact::history_capacity_test_lock();
    let root = tempdir("declared-maximal-batch").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
    let document = protocol::ArtifactId("declared-maximal".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let mut envelopes = Vec::new();
    let mut bytes = 0usize;
    let next = loop {
        let next = envelope(&format!("{:x}", envelopes.len()), &[], "a", &document, &[]).await;
        let mut encoded = Vec::new();
        protocol::encode_envelope(&next, &mut encoded);
        let count_bytes = if envelopes.len() + 1 < 128 { 1 } else if envelopes.len() + 1 < 16_384 { 2 } else { 3 };
        if envelopes.len() == protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES || count_bytes + bytes + encoded.len() > protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES {
            break next;
        }
        bytes += encoded.len();
        envelopes.push(next);
    };
    assert!(protocol::decode_document_backbone_envelopes_exact(&protocol::encode_envelopes(&envelopes)).is_ok(), "the batch is declared legal by the wire's own decoder");
    let mut beyond = envelopes.clone();
    beyond.push(next);
    assert!(protocol::decode_document_backbone_envelopes_exact(&protocol::encode_envelopes(&beyond)).is_err(), "one more envelope leaves the declaration");
    let mut over = Vec::with_capacity(protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES + 1);
    for index in 0..=protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES {
        over.push(envelope(&format!("over-{index:x}"), &[], "a", &document, &[]).await);
    }
    assert!(envelopes.len() > 256, "the declared-maximal batch ({} envelopes) exceeds the former 256-envelope submit credit", envelopes.len());
    let count = envelopes.len() as u64;
    let receipt = db_actor::block_on(handle.submit(db_artifact::CommandBatch::new(envelopes).await.unwrap(), db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
    assert_eq!(receipt.frontier.head_seq, count, "the whole declared batch commits");
    let refused = db_actor::block_on(handle.submit(db_artifact::CommandBatch { envelopes: over }, db_artifact::SubmitOptions::default())).and_then(std::convert::identity);
    assert!(matches!(refused, Err(DbError::LimitExceeded(_))), "a batch beyond the declaration is a permanent refusal: {refused:?}");
    assert_eq!(handle.frontier().await.unwrap().head_seq, count, "the refused batch applied nothing");
}
'''

REFUSAL_OLD = """/// ✂️ The hub's reason as a `HubTransientApplyRefusalMessageV1.message`: non-empty and within its declared bound.
pub fn hub_transient_apply_refusal_message(reason: &str) -> String {
    let bounded: String = reason.chars().take(HUB_TRANSIENT_APPLY_REFUSAL_MESSAGE_MAX_CHARS).collect();
    if bounded.is_empty() { HUB_TRANSIENT_APPLY_REFUSAL_CODE.to_string() } else { bounded }
}
"""
REFUSAL_NEW = """/// ✂️ The hub's reason as a `HubTransientApplyRefusalMessageV1.message`: non-empty and within its declared bound.
pub fn hub_transient_apply_refusal_message(reason: &str) -> String {
    bounded_apply_refusal_message(reason, HUB_TRANSIENT_APPLY_REFUSAL_MESSAGE_MAX_CHARS, HUB_TRANSIENT_APPLY_REFUSAL_CODE)
}

/// 📏️ `HubBatchLimitRefusalCodeV1`: the `code` of the one message a document batch carries when it exceeds what the document
/// backbone declares legal or what one database operation can ever hold — permanent: the client never resends it unchanged.
pub const HUB_BATCH_LIMIT_REFUSAL_CODE: &str = "hub.batch-limit";

/// 📏️ `HubBatchLimitRefusalMessageV1.message`'s bound in characters.
pub const HUB_BATCH_LIMIT_REFUSAL_MESSAGE_MAX_CHARS: usize = 1024;

/// ✂️ The hub's reason as a `HubBatchLimitRefusalMessageV1.message`: non-empty and within its declared bound.
pub fn hub_batch_limit_refusal_message(reason: &str) -> String {
    bounded_apply_refusal_message(reason, HUB_BATCH_LIMIT_REFUSAL_MESSAGE_MAX_CHARS, HUB_BATCH_LIMIT_REFUSAL_CODE)
}

fn bounded_apply_refusal_message(reason: &str, maximum_chars: usize, empty: &str) -> String {
    let bounded: String = reason.chars().take(maximum_chars).collect();
    if bounded.is_empty() { empty.to_string() } else { bounded }
}
"""

REFUSAL_LAW = r'''
/// 📏️ The Rust batch-limit refusal is `HubBatchLimitRefusalCodeV1` / `HubBatchLimitRefusalMessageV1`: the code is the declared
/// const, distinct from the transient one, and every reason becomes a non-empty message within the declared bound.
#[test]
fn the_batch_limit_refusal_is_the_declared_schema_message() {
    let module: serde_json::Value = serde_json::from_str(HUB_REFUSAL_SCHEMA_JSON).unwrap();
    assert_eq!(module["$defs"]["HubBatchLimitRefusalCodeV1"]["const"], HUB_BATCH_LIMIT_REFUSAL_CODE);
    assert_ne!(HUB_BATCH_LIMIT_REFUSAL_CODE, HUB_TRANSIENT_APPLY_REFUSAL_CODE);
    assert_eq!(module["$defs"]["HubBatchLimitRefusalMessageV1"]["properties"]["level"]["const"], "error");
    let message = &module["$defs"]["HubBatchLimitRefusalMessageV1"]["properties"]["message"];
    assert_eq!(message["maxLength"].as_u64(), Some(HUB_BATCH_LIMIT_REFUSAL_MESSAGE_MAX_CHARS as u64));
    assert_eq!(hub_batch_limit_refusal_message("limit exceeded: envelopes"), "limit exceeded: envelopes");
    assert_eq!(hub_batch_limit_refusal_message(""), HUB_BATCH_LIMIT_REFUSAL_CODE);
    assert_eq!(hub_batch_limit_refusal_message(&"é".repeat(5000)).chars().count(), HUB_BATCH_LIMIT_REFUSAL_MESSAGE_MAX_CHARS);
}
'''

SCHEMA_DEFS = {
    "HubBatchLimitRefusalCodeV1": {
        "description": "The `code` of the one `MutationMessage` a hub puts in a document batch's `ApplyOutcome::Rejected.messages` when the batch exceeds what the document backbone declares legal (`DOCUMENT_BACKBONE_BATCH_MAXIMUM_*`, judged by the wire's own exact decoder) or what one database operation can ever hold. PERMANENT: nothing was applied and the same batch can never be admitted, so the client never resends it unchanged (it splits or rolls it back). A transient refusal carries `HubTransientApplyRefusalCodeV1` instead, never both.",
        "const": "hub.batch-limit",
    },
    "HubBatchLimitRefusalMessageV1": {
        "description": "The batch-limit `MutationMessage` (its `ToValue` wire object): level `error`, the declared code, and the limit the batch exceeded; never a target or an operation index, since the whole batch was refused before any operation.",
        "type": "object",
        "additionalProperties": False,
        "required": ["level", "code", "message"],
        "properties": {
            "level": {"const": "error"},
            "code": {"$ref": "#/$defs/HubBatchLimitRefusalCodeV1"},
            "message": {"type": "string", "minLength": 1, "maxLength": 1024},
        },
    },
}

FIXTURE_FIELDS = {
    "batchLimitCauses": [
        {"cause": "undeclared-batch", "reason": "batch exceeds the document backbone's declaration: limit exceeded: envelopes"},
        {"cause": "undeclared-batch", "reason": "batch exceeds the document backbone's declaration: limit exceeded: batch-bytes"},
        {"cause": "db-limit-exceeded", "reason": "limit exceeded: artifact submit batch item credit"},
    ],
    "batchLimitAnswer": {"level": "error", "code": "hub.batch-limit"},
    "batchLimitInvalidMessages": [
        {"level": "warning", "code": "hub.batch-limit", "message": "limit exceeded: envelopes"},
        {"level": "error", "code": "hub.unavailable", "message": "limit exceeded: envelopes"},
        {"level": "error", "code": "hub.batch-limit", "message": ""},
        {"level": "error", "code": "hub.batch-limit", "message": "limit exceeded: envelopes", "opIndex": 0},
    ],
}
FIXTURE_DESCRIPTION_TAIL = " A batch the hub can NEVER admit carries instead exactly `batchLimitAnswer` (a `HubBatchLimitRefusalMessageV1`, code `hub.batch-limit`, level error) with the cause's reason: `batchLimitCauses` are the batch-size refusals (a batch beyond the document backbone's declaration; a db engine `LimitExceeded`), `batchLimitInvalidMessages` near misses (a warning level, the transient code, an empty message, an operation index)."

ORACLE_OLD = """  it("the fixture's oversized vector exceeds every declared body limit", () => {"""
ORACLE_NEW = """  it("a batch the hub can never admit carries a valid HubBatchLimitRefusalMessageV1 for every cause, never the transient code, and no near miss is valid", () => {
    const refusals = read(hubRoot, "🚧️refusal", "🧫️fixtures", "⏳️transient-apply-refusal-v1", "🔣️.json");
    const validate = compileDef(refusal, "HubBatchLimitRefusalMessageV1");
    const transient = compileDef(refusal, "HubTransientApplyRefusalMessageV1");
    expect(refusals.batchLimitAnswer.code).toBe(refusal.$defs.HubBatchLimitRefusalCodeV1.const);
    expect(refusals.batchLimitAnswer.code).not.toBe(refusal.$defs.HubTransientApplyRefusalCodeV1.const);
    expect(new Set(refusals.batchLimitCauses.map((cause: any) => cause.cause))).toEqual(new Set(["undeclared-batch", "db-limit-exceeded"]));
    for (const cause of refusals.batchLimitCauses) {
      const message = { level: refusals.batchLimitAnswer.level, code: refusals.batchLimitAnswer.code, message: cause.reason };
      expect(validate(message), JSON.stringify(validate.errors)).toBe(true);
      expect(transient(message)).toBe(false);
    }
    expect(refusals.batchLimitInvalidMessages.length).toBeGreaterThanOrEqual(4);
    for (const message of refusals.batchLimitInvalidMessages) expect(validate(message), JSON.stringify(message)).toBe(false);
  });

  it("the fixture's oversized vector exceeds every declared body limit", () => {"""

BOOT_HANDLER_OLD = """            for envelope in &mut envelopes {
                envelope.document_id = db_id.clone();
            }
            if let Err(error) = admit_writes(gate, principal, tenant, db_id, &envelopes, now_ms().max(0) as u64).await {"""
BOOT_HANDLER_NEW = """            if let Some(outcome) = undeclared_batch_refusal(&envelopes) {
                let frontier = best_effort_frontier(handle).await;
                let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(outcome) }], frontier };
                return sender.send(encode(&ack, document_id).await).await.is_ok();
            }
            for envelope in &mut envelopes {
                envelope.document_id = db_id.clone();
            }
            if let Err(error) = admit_writes(gate, principal, tenant, db_id, &envelopes, now_ms().max(0) as u64).await {"""

BOOT_MESSAGES_OLD = """        db::DbError::Unavailable(reason) => transient_apply_refusal_messages(reason),
        _ => Vec::new(),
    }
}
"""
BOOT_MESSAGES_NEW = """        db::DbError::Unavailable(reason) => transient_apply_refusal_messages(reason),
        db::DbError::LimitExceeded(limit) => batch_limit_refusal_messages(&format!("limit exceeded: {limit}")),
        _ => Vec::new(),
    }
}

/// @emoji 📏️ `ApplyOutcome::Rejected.messages` of a batch the hub can never admit: the one `HubBatchLimitRefusalMessageV1`
/// (`🚧️refusal`), level error, code `hub.batch-limit`, the exceeded limit bounded — permanent, never the transient code.
fn batch_limit_refusal_messages(reason: &str) -> Vec<u8> {
    encode_messages(&[protocol::MutationMessage::error(semio_hub::refusal::HUB_BATCH_LIMIT_REFUSAL_CODE, semio_hub::refusal::hub_batch_limit_refusal_message(reason))])
}

/// @emoji 📏️ The permanent refusal of a `Commands` batch the document backbone does not declare legal
/// (`protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_*`, judged by the wire's own exact decoder over the batch's canonical
/// encoding); `None` for every declared batch, which the socket budget and the db engine admit as one submit.
fn undeclared_batch_refusal(envelopes: &[MutationEnvelope]) -> Option<ApplyOutcome> {
    let limit = protocol::decode_document_backbone_envelopes_exact(&protocol::encode_envelopes(envelopes)).err()?;
    let reason = format!("batch exceeds the document backbone's declaration: {limit}");
    Some(ApplyOutcome::Rejected { messages: batch_limit_refusal_messages(&reason), reason })
}
"""

BIN_ANCHOR = "/// 🏘️ A member of many spaces reads its space list and its event pages in bounded time, exactly:"
BIN_LAW = r'''/// 📏️ C12 P1 live (a 600-envelope outbox answered `hub.unavailable` forever): a `Commands` batch the document backbone
/// declares legal passes the socket's declaration check, one envelope more is refused PERMANENTLY with the one
/// `HubBatchLimitRefusalMessageV1` (`hub.batch-limit`, level error), and an engine `LimitExceeded` maps to the same permanent
/// message — no size refusal ever carries the transient `hub.unavailable` a client would resend without end.
#[test]
fn a_batch_beyond_its_declaration_is_refused_permanently_and_never_as_transient() {
    run_socket_test(|| async {
        let document = WireArtifactId("p1-declared-batch".to_string());
        let template = sample_envelope("declared", &document).await;
        let mut declared = Vec::new();
        let mut bytes = 0usize;
        let next = loop {
            let next = MutationEnvelope { mutation_id: protocol::MutationId(format!("declared-{:x}", declared.len())), ..template.clone() };
            let mut encoded = Vec::new();
            protocol::encode_envelope(&next, &mut encoded);
            let count_bytes = if declared.len() + 1 < 128 { 1 } else if declared.len() + 1 < 16_384 { 2 } else { 3 };
            if declared.len() == protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES || count_bytes + bytes + encoded.len() > protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES {
                break next;
            }
            bytes += encoded.len();
            declared.push(next);
        };
        let mut over = declared.clone();
        over.push(next);
        assert!(declared.len() > 256, "the declared batch ({} envelopes) exceeds the engine's former 256-envelope credit", declared.len());
        assert!(undeclared_batch_refusal(&declared).is_none(), "a declared batch passes the socket's declaration check");
        let Some(ApplyOutcome::Rejected { reason, messages }) = undeclared_batch_refusal(&over) else { panic!("one envelope beyond the declaration is refused") };
        let messages: serde_json::Value = serde_json::from_slice(&messages).expect("messages are JSON");
        assert_eq!(messages[0]["code"], semio_hub::refusal::HUB_BATCH_LIMIT_REFUSAL_CODE, "{reason}");
        assert_eq!(messages[0]["level"], "error");
        assert_eq!(messages.as_array().unwrap().len(), 1);
        let engine: serde_json::Value = serde_json::from_slice(&messages_for_error(&db::DbError::LimitExceeded("artifact submit batch item credit"))).expect("messages are JSON");
        assert_eq!(engine[0]["code"], semio_hub::refusal::HUB_BATCH_LIMIT_REFUSAL_CODE, "an engine size refusal is permanent");
        assert_ne!(engine[0]["code"], semio_hub::refusal::HUB_TRANSIENT_APPLY_REFUSAL_CODE);
    });
}

'''


def edit(texts, path, old, new, pending, dry, label):
    text = texts[path]
    if new in text:
        print(f"applied already: {label}")
        return 0
    if text.count(old) != 1:
        print(f"PROBLEM ({text.count(old)} matches): {label}")
        return 1
    texts[path] = text.replace(old, new, 1)
    pending.append(label)
    print(f"{'would apply' if dry else 'apply'}: {label}")
    return 0


def main() -> int:
    dry = "--dry-run" in sys.argv
    paths = [ENGINE, ENGINE_TESTS, REFUSAL, REFUSAL_TESTS, SCHEMA, FIXTURE, ORACLE, BOOTSTRAP, BIN_TESTS]
    texts = {path: path.read_text(encoding="utf-8") for path in paths}
    pending: list[str] = []
    problems = 0
    problems += edit(texts, ENGINE, ENGINE_CONSTS_OLD, ENGINE_CONSTS_NEW, pending, dry, "engine: declared submit credits")
    problems += edit(texts, ENGINE, CREDIT_OLD, CREDIT_NEW, pending, dry, "engine: credit counts targets + observed")
    problems += edit(texts, ENGINE, "        let admission_error = credit.as_ref().err().map(ToString::to_string);\n", "        let admission_error = credit.as_ref().err().cloned();\n", pending, dry, "engine: admission error keeps its kind (1/2)")
    problems += edit(texts, ENGINE, "Some(Err(DbError::Unavailable(admission_error.unwrap_or_else(|| \"artifact submit admission exhausted\".to_string()))));", "Some(Err(admission_error.unwrap_or_else(|| DbError::Unavailable(\"artifact submit admission exhausted\".to_string()))));", pending, dry, "engine: admission error keeps its kind (2/2)")
    if "fn a_declared_maximal_batch_commits_through_the_database_and_an_over_declared_one_is_refused_permanently" not in texts[ENGINE_TESTS]:
        problems += edit(texts, ENGINE_TESTS, "\nasync fn create_catalog_fixture(", ENGINE_LAW + "\nasync fn create_catalog_fixture(", pending, dry, "engine law")
    problems += edit(texts, REFUSAL, REFUSAL_OLD, REFUSAL_NEW, pending, dry, "refusal: batch-limit code + message")
    if "fn the_batch_limit_refusal_is_the_declared_schema_message" not in texts[REFUSAL_TESTS]:
        texts[REFUSAL_TESTS] = texts[REFUSAL_TESTS].rstrip("\n") + "\n" + REFUSAL_LAW
        pending.append("refusal unit law")
        print(f"{'would append' if dry else 'append'}: refusal unit law")
    schema = json.loads(texts[SCHEMA])
    if "HubBatchLimitRefusalCodeV1" not in schema["$defs"]:
        schema["$defs"].update(SCHEMA_DEFS)
        texts[SCHEMA] = json.dumps(schema, indent=2, ensure_ascii=False) + "\n"
        pending.append("schema defs")
        print(f"{'would add' if dry else 'add'}: schema HubBatchLimitRefusalCodeV1 + HubBatchLimitRefusalMessageV1")
    fixture = json.loads(texts[FIXTURE])
    if "batchLimitAnswer" not in fixture:
        fixture["description"] = fixture["description"] + FIXTURE_DESCRIPTION_TAIL
        fixture.update(FIXTURE_FIELDS)
        texts[FIXTURE] = json.dumps(fixture, indent=2, ensure_ascii=False) + "\n"
        pending.append("fixture batch-limit cases")
        print(f"{'would add' if dry else 'add'}: fixture batch-limit cases")
    if "a batch the hub can never admit carries a valid HubBatchLimitRefusalMessageV1" in texts[ORACLE]:
        print("applied already: TS oracle case (a later case sits between it and its anchor)")
    else:
        problems += edit(texts, ORACLE, ORACLE_OLD, ORACLE_NEW, pending, dry, "TS oracle case")
    if "if let Some(outcome) = undeclared_batch_refusal(&envelopes)" in texts[BOOTSTRAP]:
        print("applied already: hub: socket declaration check (reshaped by H14's ClientFrameStepV1, 21:47)")
    else:
        problems += edit(texts, BOOTSTRAP, BOOT_HANDLER_OLD, BOOT_HANDLER_NEW, pending, dry, "hub: socket declaration check")
    problems += edit(texts, BOOTSTRAP, BOOT_MESSAGES_OLD, BOOT_MESSAGES_NEW, pending, dry, "hub: LimitExceeded → batch-limit message")
    if "fn a_batch_beyond_its_declaration_is_refused_permanently_and_never_as_transient" not in texts[BIN_TESTS]:
        problems += edit(texts, BIN_TESTS, BIN_ANCHOR, BIN_LAW + BIN_ANCHOR, pending, dry, "hub bin law")
    if problems:
        print(f"{problems} problems — nothing written")
        return 1
    if not dry:
        for path, text in texts.items():
            if text != path.read_text(encoding="utf-8"):
                path.write_text(text, encoding="utf-8")
    print(f"{'dry run: ' if dry else ''}{len(pending)} pending")
    return 0


if __name__ == "__main__":
    sys.exit(main())
