# Describe path review (independent, read-only)

## (a) `OwnedSemioExport::core_type()`
All 36 exports in `🔌️plugin/🧠️interpreter/🔌️owned-semio-boundary/🦀️.rs:200-221` match the guest macro (`📣️macros/⚛️wp4-reactor/🦀️.rs:257-491`): alloc, dealloc, 7 sqlite, restore, packSchemaHash, genesis, printMirror, applyOps, replay `(i32,i32)->i64`; checkpoint, describe, channelVersion `()->i64`; admitActorInput `2xi32+19xi64`; poll, admit/close context, close output, closeJobs 19; start/step/read/ack/cancel job 20; publish/read/ack actor failure 21; borrowJobOutput 5; borrowActorFailure 6; ackJobOutputChunk 22; ackActorFailureChunk 23. The match is exhaustive. No defect in the production code.

Defects (stale tests, both fail today):
- `🏃️runtime/🧪️tests/🧫️fixtures/🌱️channel/🦀️.rs:33-60`: type table (12/11/5/14 i64) and `_ => type 3` give Poll, AdmitActorContext, CloseActorContext, CloseActorOutput, Publish/Read/AckActorFailure, BorrowActorFailure, AckActorFailureChunk, StartJob and AdmitActorInput the wrong signature, so `from_component` rejects the fixture.
- `🧪️tests/🔬️unit/🦀️.rs:17-26`: the `jobs` loop asserts `(i32,i32)->i64` for job exports; they are 19 or 20 `i64` params.

## (b) Caller clock
`🏃️runtime/🔄️owned-runtime/🦀️.rs:81,96`: binds `(budget.retained.work.clock_binding, self.original_clock)`. The runtime-wide clock fn is bound without the genuine-source validation that `bind_original_clock` / `bind_original_clock_from_source` do (`original_source_parent`, issuer binding). The binding is taken from the budget, not from a source loan, so `release_original_clock(binding)` and `original_clock_observation(binding)` see a binding the caller never issued. Acceptable for describe and codec, but it bypasses the caller-authority invariant. Inconsistent: `job_call` (line 266) uses `budget.input.work`, not `retained`. `instantiate_actor` (line 29) still has `None`; `actor_context_call` (line 107) relies on a later bind.

## (c) `native_pack_schema_hash`
No remaining references to `borrowed_record_spec_producer`, `native_record_spec_producer` or `__artifact_borrowed_record_spec_producer` in code. Defects:
- Unused import `BorrowedRecordSpecProducer`: `✏️s/🔌️plugins/🔋️energy/…/📜️carrier/🦀️.rs:5` (its only use was removed) and `🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🦀️.rs:14`. Fails if warnings are denied.
- `🏪️store/📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs:53` registers `unwrap_or([0;32])` while the plugin table returns `None` (and then refuses) for the same snapshot. Zero is a silent sentinel that can equal no real hash.

## (d) Composition routes and identity
Framing and direction (native -> foreign) match `ArtifactIdentityClaim::composition_route`, and the dialect consts match the script's inference (`.…/🪪️identity/🦀️.rs:256-259, 301`). The diagnostics change compiles (Display and `namespace().as_str()` exist). Defects:
- `✏️s/🔌️plugins/🔋️energy/…/🚪️io/🦀️.rs:173`: the idf export entry has `reads: [MODEL_DIALECT, MODEL_JSON_BRIDGE_DIALECT]` with the bridge being `s.stdio.json`. `composer_identity_claims` (identity `🦀️.rs:333`) refuses this with "foreign output routes must read artifact-owned source dialects", whatever the declaration says. The 3 energy rows cannot pass assembly until the bridge is dropped from `reads` or the rule changes. The idf row would also need both routes in one capability if both stayed.
- `composition-route-fix.py` infers the native dialect from the last native row and emits exactly one route per row. It is wrong for any entry with several `reads` (claim set must equal the sorted, deduplicated runtime set). Only energy has such an entry today.
- Plain `dialect` claims for foreign composers remain in remodel, dag, puzzle, draw, playbook, fem2d/3d, sourcing, mathematical, reasoning wires, animate, trinity rewriting, block, note, wfc and forms. None is chained through a real `.composers(...)` call (only doc comments), so they are not checked now. They fail the moment one is chained.

## (e) Emitter sealed recheck
`🖨️describe/🛠️emitter/🟦️.ts` and `🔏️sealed/🟦️.ts` are logically correct and the test covers live drift, tamper and absence. Residual risk: `recheck()` still calls `artifact.recheck()`, which was wrapped as `inputs.recheck(); originalArtifact.recheck()`. `inputs.recheck` re-checks the cargo configuration views of the live workspace (Cargo manifests, locks, config). A concurrent edit of those can still fail the final recheck, unlike source edits. Unverified.
