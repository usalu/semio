# Verify wave 5 (verify9-)

Build pause #4 arrived while `verify9-fw-c.log` (preflight, intrinsic, encode-cursor, view-model) was already running; nothing new was queued after it. Logs: `🗑️generated/verify9-*.log`. No `[DEBUG]` lines remain in source.

## Step 1: typegen and async-tests
`repo run @semio-tech/framework-async-rs:typegen` was cancelled after 3226 s by the graph service: "the service holds 4811 MB of its 4811 MB memory limit". The generated TS file does not exist, so the 4 async-tests were not re-run. Same graph-memory blocker as `describe norm`.

## Step 2: batch A results (verify9-fw-a2.log, fw-b.log)
- diagnostic: 25/25 PASS (was 23/25).
- manifest-view-model: 22/22 PASS in verify9-fw-c.log (was 20/22).
- sqlite snapshot tests: 41 -> 58/64 (see step 3).
- io-control-testing: still aborts: `terminal ticket has no native header lifecycle` (`♻️retirement/🎟️tickets/🔒️close`, an owner in the decoder closure quote has no `native_header_release`). NOT fixed.
- pack-intrinsic: still aborts in close (`retained intrinsic close did not terminate` / `terminal_is_empty` false). The multi-unit `advance` fix below addressed the page-allocation error; close still stalls. Debug output of the last run showed healthy close demands for the first 8 turns; the stall is later. NOT fixed.
- pack-record-encode-cursor: 3 tests fail, document close never terminates. Root cause isolated: the document cursor's deflate branch quotes `release_bytes: 3864, depth: 5` and the turn reports `copied_items: 1, released_bytes: 0` for 300000 turns, i.e. `RetainedDeflateEncoder::close_step` -> `release_empty_page` is Denied although the grant equals the quote (document `close_inner` reports phantom `copied_items:1` for a denied child). I added the missing `copy_bytes` to the frames/chunk_entries/content branches of `retirement_demands` but the deflate head/previous/bytes branch is the one stuck; a quote vs `release_page` mismatch remains (see `🌱️value/📋️list/🦀️.rs` `release_page`). NOT fixed.
- pack-record-preflight: the long test (`record_borrowed_schema_hash_retains_exact_paged_work_and_cancellation`) is not slow per turn (about 30 microseconds). Its thread is runnable (busy) after case 0, turn 2000 and prints nothing for 45+ minutes, so one call spins forever inside the cursor/list code (no loop in the cursor itself; ptrace is denied so no stack). I added a "no progress for an exactly funded quote" assertion to the test. Needs a stack sample or an instrumented rerun after the pause. Also: the lib no longer compiled because `semio_framework_replication_codec::DeflateMeasure` is gated by feature `deflate`; fixed by enabling it in the crate's Cargo.toml (`🫳️preflight/📦️packages/🦀️rust/Cargo.toml`).
- replication, replication-causal: not reached (batches aborted before). The `paged_seal_digest` fix is in source.

## Step 3: production fixes (all with source edits; those marked VERIFIED ran green)
1. ViewModel decoding dropping ordered maps on error (VERIFIED for `an_unresolved_context_names_the_field_it_lost`):
   - derive `FromValue` (`🌱️value/✨️derive/⚙️expansion/🦀️.rs` + `🎯️expand-entries`): every decoded field (struct and enum variant) is now held in a `DecodedValue` guard and released only after all fields decoded, so an error retires the ones already built.
   - `OrderedMap::from_value` (`🗂️ordered/🌐️wire`) guards its partially built map and now implements `retire_decoded`.
   - `ViewModel` serde `Deserialize` is now a manual impl through the one value decoder (`🖼️view-model/🦀️.rs`), because serde's derive drops partial maps.
   - `for_host_armed_action` built a throw-away clone (`armed`) holding ordered maps that was dropped unretired; rewritten to project directly. This was the last failing test (`window_view_context_uses_the_addressed_instance`); VERIFIED 22/22.
2. io-sqlite forwarded-allocation refusal (VERIFIED 41 -> 58 of 64): `NativeDecodeControl::charge` and the encode `charge` are local-only by design. `SqliteSnapshotControl::allocation_stage` now hands the stage a `StageAllocation` (`Local` when the SQL ledger is the only custody, `Forwarded(loan)` when an original port exists) with `decode()`/`encode()` constructors returning controls (`StageDecode`/`StageEncode`, deref to the native controls). Local stages settle their declared bytes into the SQL ledger and keep the "requires the original receiving allocation port" refusal. Callers migrated: sqlite-snapshot transfer/artifact/paged-text, print native, space collection/space preflight, space sqlite, workflow run/workflow, space-history preflight, flow reconstruction, plus 2 sqlite test files. The ✏️s plugins (stdio, forms, procedural, ...) still call the older closure-style `allocation(request.bytes)` API and need their own migration (not touched).
   Remaining 6 sqlite failures: 2 sync tests that deliberately run export with a forwarded original port (retirement test line 43, unit test line 212; need re-expressing over the quoted/receiving path), 3 receiving-path quote assertions (`work > 0`, 48198 vs 21174, 1641078 vs 1614054), and 1 forwarded-limits case (`unwrap_err` on Ok). Fixed a wrong fixture path in `🧵️composition/🧪️tests` (now passes).
3. diagnostic birth layout (VERIFIED): the unboxed `FaultScope` has a `Box<dyn>` cursor whose birth is the sequence size; `retire_decoded` now retires it as `Box<FaultScope>` (the boxed original path the record already uses).
4. pack-intrinsic page allocation occupied: `Materializer::advance` ran several `one()` units in one effect after an allocation had set the effect's actual marker; it now ends the turn once an allocation was recorded (`effect.actual().is_none()` in the loop). The "occupied" error is gone; close stall above remains.
5. preflight >8 min: see step 2 (busy loop, not per-turn cost).
6. replication `paged_seal_digest`: `generous()` used depth 4096 against a stock depth of 256; now `RetainedCloneGrant{maximum_items:1,..STOCK}` (derived from the stock). Not re-run.
7. `retained_diff_sequence` / replication-causal leftover Work action: aborts with the thread-local issuer holding a live action at thread exit in `retained_diff_sequence_cancellation_closes_original_sources_diagnostics_and_candidates`; not located (no run after the pause).

## Steps 4, 5, 6
- Step 4 (untested crates) and step 5 (heap-assertion plugin crates, remodel split check plus `retained_params`, block-3d `retirement_law`, vcs/draw): NOT run (pause).
- Step 6 done in source: `semio-plugin-testing-acceptance` (`🧪️history-edit-acceptance/🦀️.rs`) now gates every `extern crate`, `use`, `pub use` and `mod` on `any(test, feature = "artifact-app-testing")`; the exported macros stay ungated. Not compiled yet.

## Resume
After the pause: rerun typegen (needs graph memory), then the 4 async-tests; rerun fw-a with the preflight busy-loop sampled (run it alone, `--nocapture`); investigate document/deflate close denial and intrinsic close stall; check testing-acceptance without the feature.
