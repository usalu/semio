STATUS: IN PROGRESS (B4 window batch GREEN, test ladder not yet run), UNVERIFIED for tests. r1 ladder (6 batches of 12 crates, `cargo test` without keep-going) all rc 1 = REAL compile errors in the first failing crate of each batch (no deadline hit); no test ran. Next: `check --tests --keep-going` over all 72 (log `C/🗑️generated/1102/testport-c1.txt`), then fix tests only (freeze active, act18 running).

## 1101 carry-over
- `cold-reserve-output.py` (HistoryReplaying root cause) WAS applied in the patch window: `⏩️replay-owned-state-operations/🦀️.rs` now contains the `None if grant==Self::cold_authority_grant()=>{}` match, old `ok_or(VcsError::HistoryReplaying)?;control.admit...` form count 0. Effect on tests-05/sync-tests-2 still unverified (no test ran).
- sc-8 (`C/🗑️generated/1101/testport-sc-8.txt`, check --tests, rc 0, 01:39) was green; the tree drifted afterwards (peers), so r1 found new errors.
- p2-run1 (plugin family, rc 1): `semio-framework-plugin-guest-contract` lib test, 3x `no method progress on RetainedWorkOutcome` (tests `reactor/context/call/{📤️failure,🤝️admission}/🧪️tests`): port to `.result.progress()`/work API. Others in that run not reached.

## Crate table (r1 = `C/🗑️generated/1102/testport-r1-batch-N.txt`)
| batch | first failing crate | errors | cause |
|---|---|---|---|
| 0 | store-cursor (lib test) | 5 | `HistoryPageStack<String>` lacks Serialize/Deserialize in test |
| 1 | durable-group-tests | many | private `binding_step`/`retirement_progress`, `generation()` method, `take_bound` shape (known redesign) |
| 2 | member-open-tests | 41 | `history::dictionary::*` types now private |
| 3 | presence | 7 | `StepContextOwner`, `WorkerJobAdmissionContext`, `try_admit_owned` gone; `step` arity |
| 4 | sync-tests-1 | 2 | `ArtifactCodec.compile_dsl` gone |
| 5 | tests-02 | 1 | `derived_checkpoint_preview_admission_demands` not on instance |

## Window B4 (done 18:42, `win-B4-done`)
- Applied: `cursor-dev-vcs-testkit.py` (store-cursor dev-dep vcs testkit); flowext `directed-dag-facade-exports.py` was already applied by flowext; reactor-jobs Cargo `protocol-laws` on kernel dev-dep; render: the planned `tool-machine` feature edit was obsolete (feature already points at tool-run) and insufficient, real fix = render dev-dependency `semio-framework-plugin-tool-run` with `artifact-app-testing` (cfg(test) of render does not enable the field in tool-run); tests ported: reactor-jobs `🧪️tests/🎟️admission` `.result.unwrap()`, `semio-framework-test-typed-input-law` (`.result.unwrap_or_else`), sync-tests-1 (`codec.reference()?.compile_dsl`, `payload.to_vec_owner()`).
- Check `C/🗑️generated/1102/testport-W-b4c.txt` rc 0: `check --tests` of reactor-jobs, instance-render, typed-input-law green; `testport-b4.txt` run 2: store-cursor, sync-tests-1, dag-host compile (no error).
- Infra: two runs died when the shared build dir was wiped mid-run (`testport-b4-run1-cachewipe.txt`, `W-b4b`): rerun.
- Still not compiling (c1, `testport-c1.txt`, partly stale): durable-group-tests, member-open-tests (dictionary types private), presence, replay-operation (`PagedProjection<BoxedSnapshot>`), tests-02, read/owned-schema-fields (probably fixed by owners since). guest-contract lib test `.progress()` x3.
- time-travel-instance-jobs-lib-test-cycle: HOLD (design).

## 20:50 status
- Ported (test-only, unverified): guest-contract `📤️failure` and `🤝️admission` tests (`descriptor.acknowledge(..).result.unwrap()`).
- Full `check --tests --keep-going` over all 72 (`all.txt`, tag testport-c2) was queued behind the fleet slot semaphore 2h (non-W labels starve behind W- priority slots, K=2); I cancelled the waiter (only my own two pids). Rerun when a slot is free: `C/1102-tools/testport/chkt.sh testport-c2 all.txt`, then `ladder.sh` (6 batches, `t.sh`; switch `t.sh` to the slot.sh wrapper first and use `check --tests --keep-going` before `test`, because `cargo test` stops at the first uncompilable crate).
- Still red by compile (c1): durable-group-tests, member-open-tests, presence, replay-operation, tests-02 (redesign ports); no test has run in 1102, so the passed/failed table is unchanged from 1101 run-5 (338 passed, 86 failed).

## Shared build dir wipe trace (for actinfra)
- Run 1 `C/🗑️generated/1102/testport-b4-run1-cachewipe.txt` (finished 17:36, started ~17:30): build dir `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/pa1afdd/b/prototype/build/` vanished mid-compile (ENOENT on `.../semio-framework-hash/45b3a1a63618164b/out/rmeta...`, `failed to write .../fingerprint/invoked.timestamp` for adler2, allocator-api2, anyhow ...). 
- Run 2 `testport-W-b4b.txt` (18:16-18:29): same symptom under `.../fixed/m/u/l/p7f5447/b/prototype/build/` (`syn-18c2f6992cbb2edc...rcgu.o: No such file or directory`, then every dependency's `fingerprint/invoked.timestamp`). 
- Both are whole-`build/` removals while cargo held the lock (dependency fingerprints for all crates fail at once), the pattern of an external cleanup or a peer's prune of `fixed/m/u/l/p*` slot dirs. At 20:48 only six slot dirs exist (`p28d5ee p5e41f4 p7d6edf p8a65ad pa29533 qc5810eacd9d`); `pa1afdd` and `p7f5447` are gone.

## 21:30 redesign ports (UNVERIFIED, no compile yet: testport-c2 queued since 20:51)
Test-side ports done (read the current API, not mechanical):
- presence `♻️retirement/🧪️tests/🔬️unit`: mounted capture test rewritten on `OriginalJobFixtureCaller` (`admit`, `admit_alias`, `turn`, `close_owner`, `retire`) with `WorkerJobSessionAdmission`, `pump_one_for_interactive_turn(cx)`, `acknowledge_checked_out_outcome(cx)->Result<bool>`; job `step` now takes `OriginalJobReturnClaim` and returns `InteractiveJobStep::from_callback`; `ControlledRetirement::step` wrapped in `RetainedWorkOutcome{result,work:1}`. `StepContextOwner`/`WorkerJobAdmissionContext`/`try_admit_owned` are gone. The private `observed_step` import in `value snapshot-read 🧪️testing/👥️presence-retirement` shadowed the glob re-export; fixed (fully qualified use).
- durable-group-tests: `advance_binding!` macro adds the new `NativeEncodeControl` argument (7 sites), `take_bound(grant)` returns `Option<(Bound,progress)>`, `store.generation()` is a method, `snapshot_retirement_factory` lives in `store.owners`, `durable_group_root` in `store.wire`, `Bytes64` takes `RecordBytes::try_from_slice`, imports for `RetainedCloneGrant/Step`, `ValueError`. Remaining: `forwards == ["parent:forward"]` (3 asserts, fixture edit type), E0631 closure at `group-outcome-and-store-fixtures:233`, `ArtifactStore.durable_group_root`.
- tests-? `deferred-reprojection`: helper bound now `StoreMutationFor<DemoSnapshot>` (the missing OpText/Send made `store.replay` unresolvable).
- sync-tests-1: `codec.reference()?.compile_dsl`, `payload.to_vec_owner()`.
Patches for the production side (apply in the next window; all fail-closed, dry-run OK on scratch copies):
- `C/1102-tools/testport/patches/paged-projection-any-output.py`: `PagedProjection` demanded `SnapshotProjection<Output=P>`; every derived projection has `Output=Box<Self>`, but the driver already places `P::Output` generically. Unblocks replay-operation native tests (13 E0599).
- `.../member-open-dictionary-public-types.py`: `MemberHistoryDictionaryOwner` has `pub` fields of private types (`DictionaryOwners`, `PendingByte`, `DictionaryLookup`, index module); makes them `pub` (41 errors in member-open-tests).
- `.../durable-group-test-visibility.py`: restores `pub` on the fields the durable-group laws read (`binding_step`, `retirement_progress`, `decision`, prepared-member `mutation_retirement`/`snapshot_retirement`, commit `value`/`decision_pack`).
- Not diagnosed: tests-02 `derived_checkpoint_preview_admission_demands` "implemented on ArtifactStore<..>" yet not found (probable duplicate crate instance or unmet impl bounds); needs a fresh compile log.

## 23:45 surface-fixture (coordinator priority) COMPILES
`semio-framework-plugin-surface-fixture` `check --tests` rc 0, 0 errors: `C/🗑️generated/1102/testport-W-surface-4.txt` (W-surface-1: 58 errors, -2: 23, -3/-4: 0). Ports, all test-support under `🔌️plugin/🧪️tests/`:
- `InteractiveJob::step` split into `step_body` + trait `step(cx, &mut OriginalJobReturnClaim) -> InteractiveJobStep::from_callback(..)`; `RetainedWorkOutcome` wrappers (`counted_step`) in `mutation-fixtures-job-close`, `.result` on `artifact_retirement_box_close_step` / `admit_owned_retirement`; decoder close via `receive_original_retained_work(cx, decoder.close_step(grant))`.
- `cx` threaded through `consume_media`, `import_media`, `handle_action(.., &mut None, &mut Some(meta), identity, cx)` (rejections are `OriginalTurnPrimary::Fault`), `load_document`, `close_owner`, export via `export_until!` (the instance `export_media` now answers `Ok(None)` until the retained refusal/payload settles).
- `fixture_step(job, cx, origin.receiver())`: a per-job `OriginalJobReturnOrigin` (admit, drive, close) kept in a test-local map in `natural-file-reserved-job-laws`, prepared once before the fuel-debit loop in the retained-work law.
- wire fixtures use the cold document lane (`cold_mutation_apply_preparation_factory`, `factory_constructor_birth_bytes::<ColdMutationApplyFactory>`) with `Mutation + ColdMutation` bounds: SurfaceSnapshot is a plain record, not an `ImmutableRootProjection`, so the paged retained factory cannot serve it.
- publication lane: `advance(member, visibility, body, grant, native)` via `lane_advance!`, `close_step(member, cx, grant)`, `&mut *app.stores.store` deref; `SurfaceDiff` derives `RetireOwned`; surface-fixture Cargo kernel dev-dep gets `protocol-laws` (the `sum-law` test).
Not run: no tests of the crate executed (compile only).

## 00:02 W3 patches applied
Applied `paged-projection-any-output.py`, `member-open-dictionary-public-types.py`, `durable-group-test-visibility.py` (window-open). Wasm `--lib` check of replay-operation, member-open, durable-group, engine-stage: rc 0, 0 errors (`C/🗑️generated/1102/testport-W-testport-w3.txt`). tests-02: a peer already switched line 707 to the engine `ArtifactStore` type (the testing prelude `ArtifactStore` is a newtype, so its associated fn did not resolve); not yet compiled. `testport-c3` (native `check --tests`, 72 crates) is running/queued: `testport-testport-c3.txt`.

## 01:30 ALL 72 CRATES COMPILE THEIR TEST TARGETS
`testport-testport-c3.txt` (native `check --tests --keep-going`, all 72): 3 failing crates (durable-group-tests 13, presence 5, replay-operation 1); fixed (cx/`RetainedWorkOutcome` wrappers, presence Cargo dev-dep `job-worker/testkit`, `NativeEncodeControl` observer `|_| true`, `DurableOwnedThreeStoreCommitV1`/`DurableOwnedMapRecoveryOperationV1` and `DurableOwnedThreeStoreBoundV1.decision_pack` made `pub`, `step_granted(..).result`, `Vec<u8>` borrow); `W-testport-c4` left only durable-group-tests; `W-testport-c5` (durable-group-tests) rc 0, 0 errors. tests-02 and member-open-tests compile (c4). The 3 `verified.*().forwards` assertions of durable-group now check `len()==1` (the verified edits decode into `DemoMutation`, the string labels no longer survive).
Test ladder `r2` started 01:30 (6 batches of 12, slot-wrapped, slow supersede cursor test skipped): logs `C/🗑️generated/1102/testport-r2-batch-N.txt`.

## r2 batch-0 (03:19, `C/🗑️generated/1102/testport-r2-batch-0.txt`, rc 1): 12 crates ran, 34 passed, 4 failed in 2 crates
- store-command (2 failed): bare `NativeDecodeControl::new` lacked the original caller loan ("native decode work requires its original caller loan"); both tests now install a `NativeDecodeRetirementRecipient` (edit made 03:25, unverified).
- store-codec-registry native-snapshot (2 failed): `native_snapshot::original_receiving` `OriginalSnapshotRecordReceiving::admit` through `StepContext::with_native_receiving` fails in `OriginalJobConstructionRecipient::accept` (`job/⏱️context/🦀️.rs:447`): "native construction changed its accepted original Job receipt", progress capacity 26000. Suspect production: the construction grant handed to `admit` is not a subset of the remaining original grant (`retained_is_subset(grant, remaining(original, delta))`). Not fixed, not yet diagnosed to a line.

## 06:25 status (STATUS: PARTIAL)
- All 72 crates compile their test targets (c3 + c4/c5 fixes).
- Native test run: only batch-0 (12 crates) ran: 34 passed, 4 failed (command 2, codec-registry 2; see r2 batch-0 above; command fix applied, unverified). Batches 1-5 (60 crates) NOT run: `r2-batch-1` waited 4 h for a non-priority cargo slot (slots held for hours by W- and other lanes), waiter cancelled at 06:25 (my own pids only). Rerun: `C/1102-tools/testport/ladder.sh` (edit the loop to start at batch-1), logs `testport-r2-batch-N.txt`.
- Production suspect for codec-registry: `job/⏱️context/🦀️.rs:447` `OriginalJobConstructionRecipient::accept`.
- Patches applied in W3: paged-projection-any-output, member-open-dictionary-public-types, durable-group-test-visibility (+ `DurableOwnedThreeStoreCommitV1`, `DurableOwnedMapRecoveryOperationV1` and `Bound.decision_pack` made pub; wasm --lib check of the four crates green).

## 09:30 W4 follow-ups (STATUS: PARTIAL)
1. `accept` fault (job/⏱️context/🦀️.rs:447, `OriginalJobConstructionRecipient::accept`): the refusal is raised only when `admitted` is false, i.e. `!progress.fits(grant) || !retained_is_subset(grant, retained_remaining(self.original, retained_delta(*self.spent, self.start)))`. By reading, delta is zero for the codec-registry call (a quoted metadata receipt {copied_items:1, retained_capacity_bytes:26000} posted through `receive_retained_work` with the same `retained_grant()` that was captured as `original`), so the exact failing sub-condition could NOT be pinned without a run; no W4 patch written (a guess would be unsafe). Law test written instead, which prints issued grant, progress before/after and the result on failure: `job/🛬️receiving/🧪️tests/🦀️.rs` `original_context_accepts_a_quoted_metadata_receipt_of_a_wide_loan_without_native_allocation` (crate `semio-framework-job-receiving`, queued with `store-command` as `testport-r2-w4law`).
2. durable-group `forwards` assertions restored: each verified edit's `serde_json::to_value(forwards)` equals the `forwards` of the independently decoded `DurableBoundOneItemOutcomeV1.edit_canonical_json` of the same member's recovery pack (unverified, not compiled yet).
3. Ladder: `C/1102-tools/testport/chain.sh` = `t.sh r2-w4law w4law.txt` then `ladder.sh 1` (batches 1-5), all through slot.sh. It waited 3 h in the FIFO queue (K=1, 7 W- jobs ahead, queue length 12 constant 08:00-09:26) and was cancelled at 09:30 (own pids only). Requeue with `nohup C/1102-tools/testport/chain.sh &`.

## 10:50 host memory blocker (for the coordinator)
All non-running cargo requests, including my W-priority law test, wait on "not enough free memory (-690 MB spare above the 4096 MB floor)". The resident hog is the macOS `fseventsd` (pid 107, RSS 15.9 GB, uptime 7.5 days); free pages are ~5.5k of 16 KiB. Restarting it is a system action I must not take; the user/coordinator can run `sudo killall fseventsd` (launchd respawns it) to release ~16 GB for the whole fleet.

## 13:55 accept() fault PINNED and fixed (W4)
Cause (law run `testport-W-testport-law3.txt`, accept backtrace + spent ledger +2 items/+52000 B in one scope): `OriginalSnapshotRecordReceiving::step` (`store/📦️codec/📸️native-snapshot/🫴️receiving/🦀️.rs`) delegates to `RetainedRecordReceiving::step` / the binding step, which already post their physical receipt to the Job construction recipient through `native.receive_retained_work`, and then returned that same progress as a plain `Result<RetainedCloneProgress>`; the caller (test helper, any production caller) posted it a second time and `accept` (`job/⏱️context/🦀️.rs:447`) correctly refused the second post ("changed its accepted original Job receipt"). `accept` itself is correct.
Fix: `step` now returns `RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary>`; delegated child outcomes are returned as posted (`Turn::Posted`), only the owner's own effects (`Turn::Fresh`) are posted once via `native.receive_retained_work`; the `OwnedFieldReceiving` trait impl keeps its Result via `flat`. Test helper `receive_posted` added (no re-post). Result: `testport-W-testport-fix2.txt` rc 0, store-codec-registry all tests pass (both formerly failing native-snapshot tests).
