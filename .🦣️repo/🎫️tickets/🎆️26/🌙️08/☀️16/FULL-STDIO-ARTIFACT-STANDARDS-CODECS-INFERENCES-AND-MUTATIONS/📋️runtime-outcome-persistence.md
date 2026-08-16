# Runtime Outcome Persistence

## Boundary

This pass makes mutation outcome messages and first-class conflicts authoritative document state rather than process-local observations. It covers serialization, SPR transport, store reconstruction, validation, remote snapshot preflight/merge, and composition-policy execution. It does not claim closure of the umbrella artifact program.

## Implementation

- `ArtifactEnvelope` owns serialized `editMessages` and `conflicts` collections.
- SPR history op metadata carries every mutation message, including severity, code, text, target, and operation index.
- SPR conflict records carry stable identity, kind, status, messages, actors, and timestamp.
- Store construction and replacement rebuild their message index from the authoritative envelope.
- Local dispatch persists accepted outcome messages beside the edit that produced them.
- Remote snapshot merge preflights duplicate message/conflict identities and commits only exact or new rows.
- Validation rejects orphaned message ledgers, invalid operation indices, malformed conflict references, and duplicate non-equivalent identities.
- `ConflictKind::Quarantined` no longer serializes local merge policy into shared state.
- Composition dispatch now applies every preflighted member under the initiator's selected group policy while restoring each member's local policy after dispatch. This closes the former phase-1/phase-2 policy mismatch.

## Runtime Evidence

Temporary runtime evidence was emitted and retained in [`🧪️runtime-outcome-persistence-debug.log`](./🧪️runtime-outcome-persistence-debug.log):

```text
[DEBUG] durable-outcome-round-trip edit=edit-53aaa5ecf4fb4593 messages=1 conflicts=1 sprBytes=534
```

The temporary source log was removed afterward. An exact repository scan found no remaining `durable-outcome-round-trip` debug marker. A broader `[DEBUG]` scan still finds unrelated pre-existing runtime diagnostics elsewhere in the repository, so no repository-wide zero-debug claim is made.

## Verification

All commands used the ticket-local target `🎯️target/runtime-outcome`.

- `cargo check -p semio-framework-os-kernel --lib`: passed.
- `cargo test -p semio-framework-os-kernel --lib spr_round_trip_preserves_edit_messages_and_conflicts -- --nocapture`: 1 passed.
- `cargo test -p semio-framework-os-kernel --lib conflict_ -- --nocapture`: 10 passed.
- `cargo test -p semio-framework-os-kernel --lib op_meta_messages_ -- --nocapture`: 1 passed.
- Initial complete kernel run exposed a real composition-policy mismatch: 934 passed, 1 failed.
- After adding authority-selected `dispatch_wire_with_policy`, the focused regression passed 1/1.
- Final `cargo test -p semio-framework-os-kernel --lib`: 935 passed, 0 failed.

Warnings are existing workspace lint warnings; no warning-free claim is made.

## Remaining Program Work

- Plugin command surfaces must execute set-policy/read-conflicts/resolve-conflict semantically and preserve canonical reports.
- glTF mutation candidates remain unaccepted until mounted Rust vector execution and frozen-audit defects are remediated.
- Typed per-command diffs must replace the remaining generic mutation-diff surface across the artifact program.
