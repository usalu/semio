# Runtime Outcome Host and Wire Parity

## Boundary

This lane preserves durable mutation rejection messages and authoritative conflicts across the native sync wire, the TypeScript worker, and the host backbone document. It does not claim the broader persistence audit closed; the independent frozen-tree audit owns that verdict.

## Contract Changes

- Rust rejected acknowledgements retain the opaque canonical message payload in `ApplyOutcome` and `CommandAckOutcome`.
- TypeScript `WireApplyOutcome.Rejected` and `CommandAckOutcome` require `messages`; their binary encoder, decoder, worker mapping, and fixture assertion preserve the bytes exactly.
- `BackboneDocument` requires `editMessages` and `conflicts` without compatibility defaults. Backbone encode/decode and `OsWorkflowStore::new/document` carry both collections.
- The Rust fixture generator is the sole author of the binary wire fixtures. The rejected fixture now carries `[1, 2, 3]`; its SHA-256 is `75382cdcd7becd56db1769d1952249e9ec6598e6ac9655bf1e65267920bbcf41`.

## Executed Evidence

1. `cargo test -p semio-framework-os-kernel --features sync --lib wire_fixtures_stay_byte_identical_across_rust_and_ts -- --nocapture`
   - Result: 1 passed, 0 failed, 965 filtered.
2. `bun nx run @semio-tech/framework-os:test-quick -- -t 'decodes the Rust-generated binary wire fixtures byte-identically'`
   - Result: exit 0; 2 tests passed, 310 skipped across two selected files.

## Pending Gate

The focused host test `backbone_and_workflow_store_round_trips_preserve_outcomes_and_conflicts` is implemented but not yet executed because the shared stdio dependency is undergoing the repository-wide `MutationOutcome<Diff>` adoption. No host closure is claimed until that test and the host package check pass on the stable combined tree.

The frozen audit found that the first host test fixture used a hand-authored conflict ID that the strict content-addressed validator correctly rejects. The fixture now derives its kind, actor, HLC, mutation IDs, and `ConflictId::new(...)` from the actual persisted edit. The executable host verdict remains pending; the human-readable `.dsl`/`.ops` export also remains lossy for message/conflict history and is assigned to runtime P1 remediation rather than being documented as supported.
