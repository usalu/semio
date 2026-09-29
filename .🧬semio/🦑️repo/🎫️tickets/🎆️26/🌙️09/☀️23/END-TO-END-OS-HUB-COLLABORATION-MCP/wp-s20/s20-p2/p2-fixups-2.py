"""🧬️ S20 pass 2 on the p2 overlay — the drift sites outside FH4's families (framework, hub, sourcing, reasoning): apply
rejections name a frozen code (`Invariant` for a rule the diff breaks: unparsable example, child identity, closed or
foreign-owned presence store, fixture overflow/uri), framework-internal reports keep their catalogued framework code
(`MutationMessage::framework_report`), tests name frozen codes. Idempotent. Usage: python3 p2-fixups-2.py"""
from pathlib import Path

O = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2")
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
STORE_TEST = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"
EDITS = [
    ("✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs",
     '.map_err(|error| protocol::MutationApplyError::new("example.unparsable", format!("the committed metabolism example must parse: {error:?}")))',
     ".map_err(|_| protocol::MutationApplyError::new(protocol::MutationCode::Invariant))"),
    ("✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs",
     'self.validate().map_err(|message| protocol::MutationApplyError::new("mutation.child-identity", message))?;',
     "self.validate().map_err(|_| protocol::MutationApplyError::new(protocol::MutationCode::Invariant))?;"),
    ("🌎️hub/🏗️bootstrap/🦀️.rs",
     "encode_messages(&[protocol::MutationMessage::error(semio_hub::refusal::HUB_BATCH_LIMIT_REFUSAL_CODE, semio_hub::refusal::hub_batch_limit_refusal_message(reason))])",
     "encode_messages(&[protocol::MutationMessage::framework_report(protocol::Severity::Error, protocol::FaultCode::new(semio_hub::refusal::HUB_BATCH_LIMIT_REFUSAL_CODE))])"),
    ("🌎️hub/🏗️bootstrap/🦀️.rs",
     "encode_messages(&[protocol::MutationMessage::warn(semio_hub::refusal::HUB_TRANSIENT_APPLY_REFUSAL_CODE, semio_hub::refusal::hub_transient_apply_refusal_message(reason))])",
     "encode_messages(&[protocol::MutationMessage::framework_report(protocol::Severity::Warning, protocol::FaultCode::new(semio_hub::refusal::HUB_TRANSIENT_APPLY_REFUSAL_CODE))])"),
    (STORE, 'return Err(crate::os_spr::MutationApplyError::new("presence.store.closed", "presence store no longer admits local mutations"));',
     "return Err(crate::os_spr::MutationApplyError::new(crate::os_spr::MutationCode::Invariant));"),
    (STORE, 'let previous = self.local_read().map_err(|reason| crate::os_spr::MutationApplyError::new("presence.local.owner", reason))?;',
     "let previous = self.local_read().map_err(|_| crate::os_spr::MutationApplyError::new(crate::os_spr::MutationCode::Invariant))?;"),
    (STORE_TEST, 'messages: vec![crate::os_spr::MutationMessage::info("mutation.duplicate", "first")] },', "messages: vec![crate::os_spr::MutationMessage::info(crate::os_spr::MutationCode::DuplicateId)] },"),
    (STORE_TEST, 'messages: vec![crate::os_spr::MutationMessage::info("mutation.duplicate", "second")] },', "messages: vec![crate::os_spr::MutationMessage::info(crate::os_spr::MutationCode::Cascade)] },"),
    (STORE_TEST, 'messages: vec![crate::os_spr::MutationMessage::fatal("mutation.conflict", "message".repeat(256)).at(["target".repeat(256)])],',
     'messages: vec![crate::os_spr::MutationMessage::fatal(crate::os_spr::MutationCode::Invariant).at(["target".repeat(256)])],'),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🦀️.rs",
     'protocol::MutationApplyError::new("job-test.value-overflow", "job fixture value addition exceeds i32").at(["value"])',
     'protocol::MutationApplyError::new(protocol::MutationCode::Invariant).at(["value"])'),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs",
     '.map_err(|error| protocol::MutationApplyError::new("test-snapshot.declared-child-uri", error))?',
     ".map_err(|_| protocol::MutationApplyError::new(protocol::MutationCode::Invariant))?"),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs",
     ".then(|| protocol::MutationMessage::error(FOREIGN_HISTORY_TRANSITION_CODE, FOREIGN_HISTORY_TRANSITION_MESSAGE))",
     ".then(|| protocol::MutationMessage::framework_report(protocol::Severity::Error, protocol::FaultCode::new(FOREIGN_HISTORY_TRANSITION_CODE)))"),
]
for rel, old, new in EDITS:
    path = O / rel
    text = path.read_text()
    if new in text and old not in text:
        continue
    count = text.count(old)
    assert count >= 1, (rel, old[:70])
    path.write_text(text.replace(old, new))
    print(f"fixed ×{count}", rel.split("/")[-2], new[:70])
