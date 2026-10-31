# Cargo Typing Edit API Review

This is a prospective current-Source interface review before actual52482 compilation. It does not claim a Cargo error has been observed or authorize a speculative behavior edit. Package wiring correctness does not establish implementation compatibility.

## Concrete Parent Interface Mismatches

Current parent `tool-machine/⌨️typing/🦀️.rs:225–233` defines public TypingState fields states/tool/actor/buffer/deadline_ms/transaction/**entries:Vec<(String,M)>**. It has no net field. Newly wired edit implementation uses previous.net in fold_inputs, replaces row.net during exchange, destructures TypingState with net and constructs TypingState with net. Those calls cannot match the actual current state representation unchanged. This is a real field-shape mismatch, not a visibility concern.

Current mounted TypingLedger at typing/🦀️.rs:330–338 owns a BTreeMap<SharedUtf8,RetainedOriginalSlot<TypingState<M>>> and no writing field. The public methods original_write_pending, begin_original_write, end_original_write, original_window_demands, prepare_original_window, original_edit_transform_demands and transform_original_edit are absent from that mounted module. Their only inspected definitions are in authored `typing/🗃️resident/🦀️.rs:17–31`, which the current typing module does not mount. They are public declarations in an unmounted file, so they are unavailable to the standalone edit crate.

Simply mounting resident is not a coherent repair: that file assumes a writing field and Vec<TypingWindow<M>> backing with binary_search_by/insert/index/capacity, unlike the actual BTreeMap ledger. The dependency mismatch crosses representation, resident mutation accounting and state entries, so a compatibility alias or replacement field cast is inappropriate. Wait for the actual registered Cargo error, then repair the owning domain coherently with its existing neutral ownership/cancellation laws.

## Confirmed Available Calls

Parent TypingLedger source_revision (typing line344), open (354) and retained_read_pending (469) are public and present; the edit generic bounds include the Send+Sync/static bounds of retained_read_pending. TypingState other inspected fields are public. ToolStep and TYPING_IDLE_MS are exported by the existing parent typing module. These should not be reported missing.

Replication mutation/edit Source lines20/25 exposes actual public TransactionRef::original_mint_demands and mint_original with the actor/timestamp/tool/grant signatures used here. Existing job Source exposes native_receiving_identity and original_source_parent; OriginalSourceBinding is a real construction-owned type. No missing interface was identified at these inspected calls. This is not an exhaustive Rust semantic proof.

## Dependencies and Tests

Manifest dependencies job/value/replication/tool-machine match the authored imports; dev job testkit/trace/serde_json match actual Original fixture caller/decoder, heap witness and JSON fixture consumption. Existing tests cfg(test) are mounted directly by the edit lib, so they will exercise the actual package and may expose the same parent net/entries mismatch. No fake Original or widened feature is proposed. No Source, tests, Cargo, compiler, Native, process or lease operation was modified or executed during this audit.
