# semio-plugin-surface-fixture: final green pass (2026-11-27)

Result: 34 of 34 tests pass (each run alone with `--exact`, `RUST_MIN_STACK=128M`, native cargo `-j 2`, target `/Users/ueli/t/src-fixture3`, deleted at the end). Job crate: `original-core-budget` 6/6 (incl. the new cancellation law) and `original-return-and-retirement-contracts` 24/24. Units: both surface-fixture bun laws pass via `repo tools exec bun -- test`.

## 1. Cancelled settles as a typed `Cancelled` outcome (product fix, job crate)
Root cause: the return protocol treated cancellation as a reason not to enter the body. `OriginalJobReturnReceiver::prepare`, `OriginalJobReturnReceiver::enter_body`, `OriginalJobReturnOrigin::admit` and `claim_stop_reason` (return-claim admission) all stopped on `cx.is_cancelled()`. The job body (e.g. `NaturalFileImportJob::step_original`) already answers `admit_cancelled` on entry, but it was never entered, so the receiver never saw a terminal.
Decision: cancellation is a first-class terminal outcome and must travel the same paid return path as every other outcome. Only `should_yield` (budget/deadline) still defers. The source admission itself is unchanged: an origin must still be admitted before work; cancellation merely no longer strands an already-admitted receiving place.
Files (all under `🧰️framework/🔨️modules/🧵️job/`):
- `📬️outcome/⏸️continuation/🎟️origin/🦀️.rs` (`admit`, `prepare`, `enter_body`: dropped `is_cancelled`)
- `📬️outcome/⏸️continuation/🎟️source/🦀️.rs` (`claim_stop_reason`: dropped the `cancelled` stop)
- test: `📬️outcome/⏸️continuation/🧪️tests/🎟️origin/🦀️.rs` (after `cancel_now` the claimed origin now stays ready: `assert!(origin.admit(cx)?)` replaces `assert!(!...)`)
- new unit test `a_cancelled_job_settles_as_a_typed_cancelled_outcome_through_the_original_receiver` in `⏱️budget/🧪️tests/⏱️budget/🦀️.rs` (cancelled caller, `drive_step` returns `Outcome(Cancelled)`, receiver receives and acknowledges it, origin and decoder close empty).
Fixture side: `reserved_request_keeps_all_lanes...` admits the job's return origin before cancelling (`surface_admit_job_origin`), as the host does at job birth. `natural_file_cancel_after_decode...` and `natural_file_reserved_job_accounts...` needed no test change.

SOURCING HUB NOTE: this is product code that is part of the hub wasm (job return path). The hub wasm should be rebuilt before the next hub e2e. Behavioural delta: a job whose `step` does not check `cx.is_cancelled()` will now be stepped once under a cancelled context instead of being skipped. The documented contract (`StepContext::is_cancelled`: "jobs check on entry") already requires the check. I did not audit all ~30 `impl InteractiveJob` sites; the ones that matter for sourcing (natural-file import, reserved tool jobs) check on entry.

## 2. Decoder-owning caller pattern (shared testing caller)
`🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs` (`OriginalJobFixtureCaller`):
- `establish_receipt_in(cx)`: the empty native receiving scope body, now an associated fn.
- `establish_decoder_receipt` delegates to it; new `receipted_decoder(grant)` = `admitted_decoder` + receipt; paired with the existing `close_decoder` and `retire`.
Findings behind the two `child_emission_private_*` tests (diagnosis in the previous report was one layer short):
- Typed lanes: the test replaced the caller's admitted decoder with `OriginalJobDecodeOwner::empty()` and drove `lane.advance` with a standalone `MemberNativeDecodeControl`; the product now requires an original work loan ("original work requires its original caller loan") and a decoder receipt for native retirement. The test now owns a `private_close_caller` (job caller with a receipted decoder), advances lanes through `turn_with_decoder` + `with_native_receiving_work` (macro `lane_advance!`) and closes them through `retirement_turn_with_decoder`. The close loan is the full turn grant (exact-quote grant cannot fund the native retirement scope); the `demand <= grant` law is kept.
- Group rows: stock was too small for `OriginalJobFixtureCaller::admit` (`JobPayloadAuthority::admit` returned `None`). Fixed with a large stock and `retirement_turn`; the owner close now quotes copy 824 / capacity 800 / depth 3, so the close grant copy axis is 1024. Heap-vs-receipt equality adds the bytes settled against the thread birth ledger (`birth_ledger::original_birth_ledger().settled()`): symmetric accounting charges a release to the ledger that recorded the birth, so it leaves the step receipt by design.

## 3. `work_fixture_close_reconciles...` (12152 vs 24): the test's accounting was wrong, not product
The failing assertion was the copied-bytes delta (expected `headerWords * 8 = 24`, observed 12152), not released bytes. The delta of cumulative receipts spans the caller's per-turn Source birth service (return table birth), which `OriginalWorkFixture::close` and the `after` probe turn both performed. Fix: `OriginalWorkFixture::close` now uses `retirement_turn_with_decoder` (retirement never services new return births, same contract as `OriginalJobFixtureCaller::retirement_turn`), and the `before`/`after` probes run with a zero-item grant (no service). Files: `🧪️testing/🎟️caller/👷️work/🦀️.rs`, test `🧵️retained-work-and-child-emission-laws/🦀️.rs`.

## 4. Units manifest
Tool: `$T/regen-surface-fixture-units.ts` (bun, tree-sitter Rust, same hashing as `🟦️.ts`: sha256 of the `function_item` text, assertions by `\bassert(?:_eq|_ne)?!`, file sha256/bytes/assertions; run as `repo tools exec bun -- <script>`). It also resyncs function sets, the aggregate law sha and the macro sha. Regenerated `📦️units/📋️surface-fixture.json`: the renamed codec-table law (sha256 `76d4209d...` before) plus every function the earlier passes edited, 6 added helpers/tests (`close_owner_completely`, `command_from_action`, `retire_completion_birth`, `surface_admit_job_origin`, `private_close_caller`, `buffered_natural_decoder_...`), the aggregate law `committed_wire_witnesses_are_the_canonical_wire`, and file-level hashes. Pre-existing staleness fixed so the law passes: schema `🧬️surface-fixture.json` dependencies 39 -> 41, new edge `semio-plugin-testing-instance-laws` in the owner manifest and `📋️surface-fixture-lock.json`, `🟦️.ts` law count 32 -> 33 (the buffered decoder law exists).

## Peer interference observed
For about 25 minutes `[DEBUG]` `eprintln!`s in `⏱️context/🎟️source/🦀️.rs` close path (allocating 101 bytes inside the Source retirement pump) made 26 of 34 tests fail at `retire_step` (`left (0,0) right (101,0)`). They were removed by their owner at 22:13; the final run is without them. A leftover `dbg_prepare` in the return origin file was also gone by then.

## Open
- Hub wasm rebuild and a hub-level Cancel e2e (see note in section 1).
- `Cargo.lock` was not touched by me; the new `semio-plugin-testing-instance-laws` edge of the fixture is a prior-pass change.
