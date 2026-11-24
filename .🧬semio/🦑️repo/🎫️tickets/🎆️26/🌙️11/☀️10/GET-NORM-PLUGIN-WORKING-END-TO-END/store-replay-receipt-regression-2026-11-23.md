# Store replay receipt regression (2026-11-23, 18:30)

Status: NOT green. `semio-os-store-initialization-replay --lib` is 2 passed, 2 failed (clean run, no instrumentation left: `🗑️generated/replay-gate-6.log`).

## Results

| Test | Result |
| --- | --- |
| `retained_initialization_catalog_birth_denial_and_cancellation_preserve_every_original_page` | pass |
| `retained_empty_initialization_catalog_releases_one_exact_native_page_without_ungranted_heap_work` | pass (after approved test fix) |
| `native_replacement_restores_same_payload_and_preserves_original_receipts_at_every_close` | fail at `🧬️replacement/🧪️tests/🦀️.rs:117` (`owner.terminal_is_empty()`) |
| `native_supersession_scope_and_cancellation_match_json_oracle_and_original_physical_receipts` | fail at `🧬️supersession/🧪️tests/🦀️.rs:19:451` (`result.is_err()`) |

## Root causes found

1. Catalog test (not a framework regression). `📚️catalog/🧪️tests/🔬️unit/🦀️.rs` contained a nested duplicate of its denied-grant block. Each outer iteration performed a real page release, the stale `demand` (directory chunk size) was smaller than the next page size, and `close_step` returned pending instead of `DepthLimit`. Removed on the coordinator's approval; every outer assertion is kept.
2. Earlier heap `(200,24)` / `(136,24)` / `(64,0)` against receipt `(0,0)` mismatches were measurement artifacts: `eprintln!` ran inside `observe_heap_allocations_on_this_thread` regions (the `recv refuse tag` and `replacement wire len` prints). Printing allocates. I removed the `replacement wire len` print in `🧳️replacement/⚖️canonical/🦀️.rs` (the other agent's).
3. Replacement advance never funded its step (measured: 252 decoder-admission turns, 109 escrow-reclaim retries, 16 native steps in 3000 turns, then thousands of unfunded retries).
   - `OriginalReplacementReceiving::advance` passed `Default::default()` as the receiving floor, so the scope share (half the turn, 6313 copy bytes) was below the quoted step demand (8456). Fixed: it now passes the real next demand.
   - `advance_native` priced a step at body = whole scope grant, while the quote prices body 4096. Fixed with `STEP_BODY_BYTES = 4096`.
   - `original_receiving_scope_headroom` did not cover the escrow-reclaim copy bytes. Fixed (`max` with `ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES`).
   - When the escrow table was full of acknowledged tombstones (measured: 32 of 32), each turn reclaimed one slot and returned `false`; the slot was consumed by other scopes before the owner's next turn. Fixed: after a successful reclaim, admission is retried once in the same turn.

   After these, `schedule(None)` gets through receiving.

## Remaining failure 1: replacement close (measured)

- In the close loop the owner has `receiver=Some`, `input=None`, `adm=None`, and `close_demands` reports `release_bytes: 4432` (`receiver.allocation_bytes()`), but the test's close turn uses `turn()` with the base stock: release grant 4096. `permits` fails on every turn, so close never completes.
- The receiver box (4432 B) is larger than one 4096-byte release page, the invariant documented on `ARTIFACT_HISTORY_RELEASE_PAGE_BYTES` ("a directory chunk or one value page fits one release page"). Either the receiver grew past a page (most likely via the retirement refactor) or the close turn must be quoted. The test does not quote close, so the framework side must keep the receiver within a page.
- Not done: identifying which field pushed it over. My size probe printed nothing for this receiver in `OriginalJsonReceiving` (so it is probably the binary receiver, `🎮️one-item/…/🧳️receiving/🔢️binary/🦀️.rs`), which I did not instrument.

## Remaining failure 2: supersession (inferred, not measured)

- The foreign caller's `owner.advance` returns `Ok` where the test expects the "changed its source or caller" refusal. That refusal needs `self.binding`, which is set only after the first input build. It is probably still the same funding family but I did not isolate it after the fixes above, because the other agent's concurrent edits repeatedly broke the build or changed behavior (`available_work`, `ArtifactCanonicalEditSealCursor::witness`).

## Changed files

Intended fixes:
- `🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧳️replacement/🦀️.rs` (floor, `STEP_BODY_BYTES`)
- `🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧳️replacement/⚖️canonical/🦀️.rs` (removed another agent's debug print)
- `🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs` (reclaim headroom, same-turn admission retry after reclaim)
- `🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/📚️catalog/🧪️tests/🔬️unit/🦀️.rs` (removed nested duplicate block, approved)

All of my instrumentation is removed (trace `debug_tag`/`debug_turn`, job tags, replacement and pack-json probes). Remaining `[DEBUG]` prints belong to the other agent: `🔨️modules/🧵️job/⏱️context/🦀️.rs` (~lines 328-329), `🛬️decode/🦀️.rs:69`, `📬️outcome/🪙️budget/🦀️.rs:270`, `🌱️value/📋️list/🦀️.rs:590`, `🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs:45`.

## Not run

- Tests of other framework crates I changed (`semio-framework-job-context`, `semio-os-store-operation-wire`): not run, because the workspace kept failing to build from the other agent's in-flight edits.

## Suggested next step

Measure `size_of` of the binary receiving shell and bring it under 4096, for example by boxing the largest `ManuallyDrop<Option<..>>` field inside a receipted birth. Then re-run the supersession test.

## Final (2026-11-24)

Status: `semio-os-store-initialization-replay --lib` is 4 passed, 0 failed (`🗑️generated/replay-gate-9.log`).

### Root causes

1. Replacement close stall (`🧬️replacement/🧪️tests/🦀️.rs:117`). Measured with a non-allocating `size_of` probe: `OriginalJsonReceiving<Vec<String>>` is 38832 B in this configuration (largest parts: `NativeReceivingOriginal` 12256, two admissions 3232 each, two sources 2784 each, close 560). It is the JSON shell, not the binary one; the earlier "4432" figure was stale. The type was never meant to fit one 4096 B release page: its birth is already priced as `capacity_bytes = size_of::<Self>()`, and `close_demands` already prices the symmetric `release_bytes = allocation_bytes()`. The framework therefore keeps the receipts invariant exactly. The defect was the test: its close loop used `turn(..)` (empty quote, release grant = one page) while every advance and the supersession close loop use `quoted_turn(..)`. Shrinking the type below a page would not be honest (the fault field added by the retirement refactor, `Option<ValueError>` = 64 B, is not what matters).
2. Supersession foreign caller (`🧬️supersession/🧪️tests/🦀️.rs:19:451`). Not the funding stall. Measured state at the failing call: only `admission` non-terminal, `binding` None, no source (`position == events.len()`). `NativeSupersessionFold::advance_input` returned `Ok(Some(false))` for `source == None` before any caller check, so a foreign caller could end the stream of an owner whose construction admission was still bound to the original caller. The check `binding` only covers the input phase.

### Fix

- Replacement test: the close loop now runs `quoted_turn(.., owner.close_demands(stock().maximum_copy_bytes).unwrap(), ..)`. All receipt equalities, payload-pointer and `closeTurnsAfterPayloadReturn` assertions are unchanged.
- `NativeSupersessionFold::advance_input`: on `source == None` with a non-terminal admission, call `admission.validate(cx)?` before returning `Some(false)`.
- `OriginalConstructionAdmission::validate` made `pub` with a docstring (`🫆️`).
- Earlier fixes from this ticket stay (replacement floor, `STEP_BODY_BYTES`, reclaim headroom and same-turn retry).

### Changed files (this pass)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧬️replacement/🧪️tests/🦀️.rs` (close quoted)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧬️supersession/🦀️.rs` (validate on end of stream)
- `🧰️framework/🔨️modules/🧵️job/🏭️construction/🦀️.rs` (`validate` pub)
- Probes (`scratch_sizes`, `debug_state`, scratch test) added and removed again; no `[DEBUG]` of mine remains.

### Tests

- `repo cargo ... test -p semio-os-store-initialization-replay --lib --offline`: 4/4 pass (all four test names, including `native_replacement_restores_same_payload_and_preserves_original_receipts_at_every_close` and `native_supersession_scope_and_cancellation_match_json_oracle_and_original_physical_receipts`).
- `semio-framework-job`, `semio-framework-job-context`: `repo cargo test -p` refuses ("requires dev-dependencies and is not a member of the workspace"), so no lib tests can be run for them here.
- `semio-os-store-operation-wire --lib`: does not compile its own tests, independent of my change: `🧳️original/🧪️.rs:33` passes `RetainedWorkPoll<_, OriginalRetirementFailure>` to `effect_fixture::settled`, which expects `JsonBorrowError` (other agent's refactor; not edited).
- `semio-framework-pack-json` direct `-p` build refuses: crate is over its source boundary (268140 of 262144 bytes) before the compiler, also independent of my change.

### Related failures reported by the coordinator

- en1997 editor integration tests (run from `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997`, `-p semio-s-artifact-norm-en1997-editor`): not resolved by this fix. Result: `closes_mid_reserved_commit` passes; `undo_redo_round_trips_through_the_wrapper` (settlement stops after 4098 turns at stage 18, zero demand, `WorkLimit`), `history_edits_end_to_end` ("original report requires its installed semantic and retirement issuers") and `conflict_history_edits_end_to_end` ("archive load did not settle" with a lifecycle fault) still fail. They do not involve a close grant under-quote or the supersession caller check, so they are separate (issuer installation and a stage-18 settlement that makes no demand). No content files edited.
- `scalar close changed Source or Work` (`🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/🌳️source/🦀️.rs:150`, `close_dispatch`): not covered. It compares `invocation.source_turn()` with `effect.work_source_turn()`, a different mechanism from the close quote and the admission caller check; another agent is working there, so I did not touch it.
