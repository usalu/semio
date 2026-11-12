# Typed-ops and reactor-jobs (typedops2)

## Result
- `semio-plugin-instance-typed-ops`: `check --all-targets` PASS (log `typedops2-check-8.log`, rc=0). The package name is `semio-plugin-instance-typed-ops`, run from the repo root workspace (not `🧰️framework`).
- typed-ops tests: first run 0/3 passing (all three `original_typing` tests). Cause: the new decoder prerequisite contract. `OriginalJobDecodeOwner::empty()` no longer exposes `native_receiving_identity()` until its prerequisites are admitted. Fixed in `📥️staging/⌨️typing/🎟️original/🧪️tests/🦀️.rs` by using `caller.admitted_decoder()` (3 sites; the `foreign` decoder stays empty on purpose). Re-run `typedops2-test-2.log` queued behind admission at report time.
- `semio-plugin-reactor-jobs`: BLOCKED upstream by `semio-plugin-instance-initialization` (37 errors, nobody owned it). Migration done (below), re-check `typedops2-init-check-1.log` queued at report time. reactor-jobs itself has not been compiled yet.
- child-operations laws test (coordinator note): NOT done, see section "Laws test".

## Fixes in the typed-ops dependency chain
- `semio-plugin-composition-publication` `🤝️group/🪆️child`: metadata issuer takes the derived `grant`, not the effect.
- `semio-plugin-command`: `ToolLatestWinsKeyCopy::close_granted(grant)` added (effect-lane close of the key copy, uses `close_demands`); `📥️admission` uses `close_demands()` and `close_granted`.
- `semio-plugin-publication-mounted` `📚️members`: `row.close_step` already returns `Result`, dropped the extra `Ok(..)`.
- `semio-plugin-instance-render`: duplicate `InstanceExtension` import removed; `ReservedPreparation::handoff` implemented in `👷️jobs/📥️staging/🏷️reserved` (moves job + completion + wire admission into the receiving slot under one funded unit; the half-migrated `handoff_reserved_producer` called it but it did not exist).
- The `advance_latest_wins_command_one` call site of the stepped `start_typed_command_operation(&mut pending, cx)` was already migrated and compiles.

## Error-type inference (`close_original_retained_step`)
- typed-ops compiles with its existing `Ok::<_,ValueError>` pins; I did NOT change the shared API. Reason: 923 call lines in 427 files, about 590 pin `ValueError`, about 60 pin `OriginalTurnPrimary` or `OriginalTurnFailure`; changing it blind under the current admission queue would break every other agent.
- The constrained form already exists: `semio_plugin_job::original_steps::retired(cx, demand, |effect| -> Result<RetainedCloneStep, ValueError>)` and `retired_poll` (effect-threaded, error fixed to `ValueError`, no annotation needed). All new/migrated code in `semio-plugin-instance-initialization` uses it. Recommended follow-up (framework agent): make `close_original_retained_step` non-generic over `ValueError` (or deprecate it in favour of `retired`) and delete the `::<ValueError>` pins mechanically; then migrate the ~60 non-`ValueError` callers to `close_original_effect_step`.

## instance-initialization migration (new, for reactor-jobs)
- Files: `🫧️ephemeral/🦀️.rs`, `🫧️ephemeral/🧬️declaration/🦀️.rs`, `🏪️stores/🦀️.rs`, `🎟️registration/🦀️.rs`, `🎟️registration/♻️close/🦀️.rs`, `🎟️registration/🪟️windows/🦀️.rs`, root `🦀️.rs`, `Cargo.toml` (+ `semio-plugin-job`).
- All `close_original_retained_step(.., |_| RetainedWorkOutcome{..work:1})` became `retired`; `ControlledRetirement::step(body, effect)` closes use `retired_poll` with `body = cx.retained_grant().maximum_copy_bytes` (the former literal `4096` removed); disposer/peer/window closes use `close_original_effect_step`.
- Decoder identity fields are `(NativeAllocationIdentity, u64)`.
- Registry birth advance/close: new shared helper `semio_plugin_job::original_steps::native_received` (native receiving turn plus work reconciliation). Note: the same logic is duplicated privately in `store/initialization/🏠️owners` and `🔁️replay/🧬️native` (`refuse_scope`/`post_receiving`); they should switch to `native_received`.
- Actor lease clone: `OriginalStoreBirth::stash_actor` and `built_actor` added in `🌱️initialization/🌱️genesis`; used by the instance Store set and the native registration (the clone is admitted under its own effect, then consumed by the construction).
- `NoPresenceRetirementFactory` was removed from `presence_retirement`; declaration now uses `SharedValueRetirementFactory<NoPresence>`.
- Stale outside my scope: plugin apps referencing `semio_plugin::NoPresenceRetirementFactory` (flow, puzzle 2d/5d viewers) need the same replacement.

## Laws test (`app-typed-command-full-operation/🧩️child-operations/⚖️laws`), not done
Compile errors (`typedops2-laws-check-1.log`, 16):
- 4 `os_spr::protocol_laws` errors (sum-law tests of test-app-mutations): fixed by enabling `protocol-laws` on `semio-os-kernel` in the laws `Cargo.toml`.
- 6 fixture owners (`🫙️owner` x4, `🛫️encoder`, `📢️publication`, `🪪️registry`) implement `ErasedSnapshotRetirement::close_step(grant)`; the trait is now `close_step(&mut self, body, effect) -> OriginalRetirementPoll`.
- The label encode path (`TextEncodingOutput`, `encode_with_inline`, `write_encoding_format`, `label_into`, `read_from_encoding_source`) no longer exists. The replacement is `PagedTextEncodeProducer` plus `PagedTextEncodeFrame` driven by `OriginalSourceTurn` witnesses (`🔨️modules/🌱️value/📝️text/📦️paged/🛫️encode`), a different protocol; the owner fixture and ~30 grant-loop assertions in `⚖️laws/🦀️.rs` (`RetirementStep::Complete` is now a tuple variant) must be redesigned against it. This needs its own focused agent; I did not start it.

## Logs
`🗑️generated/typedops2-*` (check-1..8, jobs-check-1, test-1/2, init-check-1, laws-check-1, init-errors.txt). No `[DEBUG]` lines were added.

# Takeover (typedops3)

## Status
- typed-ops `original_typing` tests and `check --all-targets`: check PASS earlier (`typedops2-check-8.log`). The test run (`typedops2-test-2.log`) was still waiting for admission (25+ requests ahead, swap ceiling) when I stopped; no verdict yet. Re-run: `repo cargo --store=shared --configuration=prototype --parallelism=4 -- test -p semio-plugin-instance-typed-ops` from the repo root.
- `semio-plugin-instance-initialization`: the earlier run (`typedops2-init-check-1.log`) showed only 2 remaining errors (37 before). Fixed both:
  - `🫧️ephemeral/🦀️.rs:7`: `NativeAllocationIdentity` now imported via `semio_framework_value::retained_clone::`.
  - `🏪️stores/🦀️.rs:45`: `owner.close_owned_store_step(effect)` returns an `OriginalRetirementPoll`, so the call uses `retired_poll` instead of `retired`.
  Verification run `typedops3-init-check-1.log` (check --all-targets) queued, not finished.
- `semio-plugin-reactor-jobs`: `typedops3-jobs-check-1.log` queued behind the init check (not compiled yet, still unverified).
- Laws fixtures: `typedops3-laws-check-1.log` queued (see below).

## Laws redesign (partial)
Edits in `app-typed-command-full-operation/🧩️child-operations/{🛫️encoder,🫙️owner,🪪️registry,📢️publication}`:
- The 7 `ErasedSnapshotRetirement` owners now implement `close_step(body, effect) -> OriginalRetirementPoll` plus `next_computational_work`. The previous grant-shaped step logic is kept unchanged as an inherent `advance_granted(grant)`; the new `settled_poll` helper (in the encoder fixture) completes it as one poll with one work unit per progressed step. Same properties, no assertion removed.
- `PagedList::reserve_one(n)` -> `reserve_one_cold(n)`, `release_empty_page(n)` -> `release_empty_page_leased(n)` (cold, fixture-local control, matching the plain `NativeEncodeControl::new` the laws use).
- Not yet re-expressed (honest scope finding): the owner fixture and laws still rely on a whole removed text API generation: `PagedText::read_from_source`, `InlineTextBuffer::read_from_source`/`close_one`, `encode_with_inline`/`TextEncodingOutput`/`write_encoding_format`, `OwnedOperationBytes::close_one(1,4096)`, `retire_one(..)`, `RetirementStep::{Advanced,BudgetExhausted,Bytes,Complete}`. The replacement is `PagedTextDecodeFrame` (read) and `PagedTextEncodeProducer` + `PagedTextEncodeFrame` (encode), which need an `OriginalSourceTurn` witness, recorded span/checkpoint authorities and an effect bank; the working rig is `🧰️framework/🔨️modules/🌱️value/📝️text/📦️paged/🧪️tests/{encoder-rig,staged-owner,native-rig}.rs` (`EncodeBank::run`). Plan: (1) lift that rig into a shared testing crate (not copy), (2) `Text::read_from_source` -> drive `PagedTextDecodeFrame::advance` through the bank, (3) `produce_label` -> drive a `PagedTextEncodeProducer` over a `TextReadSource` (the failing `{Original}` 8194-byte label case becomes a source whose `read_recorded` refuses at the carrier index), (4) re-express the grant-loop assertions with `RetirementStep::Complete(_)` tuple variant and effect-driven `close_step`. Expected 2 more compile iterations after the first laws log; each takes hours under the current queue, hence not finished here.

## `close_original_retained_step` removal plan
Facts: 891 unpinned calls + 8 pinned in 437 files. Pins: ~80 `RetainedWorkOutcome::<_,ValueError>`, 26 `Ok::<_,OriginalTurnPrimary>`, 4 `Ok::<_,OriginalTurnFailure>`. Definition: `🧵️job/♻️retirement/🎟️context/🦀️.rs`, generic over `E:Into<OriginalTurnPrimary>`.
- The constrained `retired`/`retired_poll` are NOT a mechanical target: they take an effect (`&mut RetainedAllocationEffect`) closure, whereas the old form takes a grant closure returning `RetainedWorkOutcome{result,work}`. Rewriting bodies is semantic (work is read from the effect), so bulk sed to `retired` is unsafe.
- Mechanical and verifiable alternative: make `close_original_retained_step` non-generic on the error, `advance: FnOnce(RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>` (E defaults to `ValueError`, `RetainedWorkOutcome<T=RetainedCloneStep,E=ValueError>`). Then (a) all ~80 `::<_,ValueError>` pins become redundant and can be sed-stripped (`RetainedWorkOutcome::<_,ValueError>{` -> `RetainedWorkOutcome{`), (b) the 30 `OriginalTurnPrimary/Failure` callers move to the existing non-generic `close_original_retained_turn` (already takes `RetainedWorkOutcome<RetainedCloneStep,OriginalTurnPrimary>`). The 891 unpinned calls need no edit since they return `ValueError`-typed outcomes (that is where inference currently fails).
- Not executed: it changes a signature used in 437 files across 4+ workspaces, none of which can be verified with the current admission queue (hours per workspace check) and other agents are compiling those crates. Recommend: coordinator schedules it as a quiet-fleet step, order: signature change + the 30 non-ValueError callers + pin strip, then `repo cargo check --all-targets --keep-going` per workspace.

## Logs
`🗑️generated/typedops3-{init-check-1,jobs-check-1,laws-check-1}.log` (running/queued at report time). No `[DEBUG]` lines added.

## Build pause (coordinator)
No new `repo cargo`/`repo run` queued since the pause. The four already-queued runs (typedops2-test-2, typedops3-init-check-1, -jobs-check-1, -laws-check-1) are watched by one monitor that fires only on an `rc=`/exit line; on results: fix and re-run one at a time, after the coordinator says resume. Known next source edit for laws (not done): `close_child` in `🫙️owner/🦀️.rs` still calls the grant-shaped `ErasedSnapshotRetirement::close_step(child)` on a `dyn` owner and must forward through `effect.reborrow(..)` like `🧬️retained-clone/♻️close` does.

## Result: instance-initialization (typedops3-init-check-1.log, rc=101)
Lib target compiles. Only the 2 test targets had 5 errors; fixed in source (unverified, builds paused): `🎟️registration/🪟️windows/🧪️tests` uses `semio_plugin_job::original_steps::retired` instead of removed `close_original_retained_step`; `🫧️ephemeral/🧪️tests` now passes an `OriginalStoreBirth::default()` to `OriginalEmptyEphemeralSource::admit` (3 call sites). Open risk: the test must retire that birth if admit leaves it non-empty. Re-run `check -p semio-plugin-instance-initialization --all-targets` after resume, then its tests. reactor-jobs/laws/typed-ops-test logs still pending.

## typed-ops test result and fix (typedops2-test-2.log: 0/3, rc=101)
Cause: `admitted_decoder()` only admits prerequisites; it never holds a receiving custody (`OriginalJobDecodeOwner::receiving_identity` is the continuation receipt, created by the first original construction). `OriginalTypingCommit::admit` read `cx.native_receiving_identity()` before any construction and refused ("changed its source or receiving custody"); `OriginalToolLedgerRead::admit` (tool-run) does the same ("requires its actual decoder recipient").
Source fixes (unrun, builds paused):
- `📥️staging/⌨️typing/🎟️original/🦀️.rs` `OriginalTypingCommit::admit`: now `birth.built_with(..., |control| ...)` and captures `(control.ledger_identity(), control.retirement_recipient_identity())` from the construction scope itself, the same pattern as `Store genesis` `Source::admit` and `metadata.rs`; later turns still validate against `cx.native_receiving_identity()`, which now exists because that construction created the custody. Fixes both "continuation changed" tests.
- Reader test: the reader consumes the custody and does not create it, so the test first admits a custody anchor (`anchor(..)`: a one-unit `OriginalStoreBirth` construction through the same decoder) and releases it (`release(..)`) before `finish`. Production `OriginalToolLedgerRead` unchanged.
Re-run after resume: `test -p semio-plugin-instance-typed-ops` (also check --all-targets).

## Result: laws check (typedops3-laws-check-1.log, rc=101)
The 7 `close_step` signature errors and the `protocol_laws` errors are gone. 5 resolution errors remain (rustc has not yet type-checked the rest, so more will follow): `TextEncodingOutput` (owner:29, :235), `write_encoding_format` (laws:389 x2, the 8194-byte `{Original}` label-carrier case), and `RetirementStep::Complete` as a unit variant (laws:346; the old `retire_one`/`RetirementStep::{Advanced,BudgetExhausted,Bytes}` capsule-cursor tests at laws:302-346 use a removed cursor generation, so they must be re-expressed against `RetireOwned`/`ControlledRetirement` driven by `retired_poll`, as in `typed-operations/.../original/tests`). Remaining work is exactly the plan above (paged-text rig for label encode/decode; effect-driven retirement loops for the capsule tests).

## Result: reactor-jobs check (typedops3-jobs-check-1.log, rc=101)
`semio-plugin-reactor-jobs` itself is still not compiled: the build stops earlier in dependency `semio-plugin-instance-app` (lib), 17 errors, all API drift (not caused by reactor-jobs or my edits). Locations (under `🧵️instance/`): `🔌️plugin-app-impl/🦀️.rs` 827 (future output `Result<Vec<Effect>,Fault>` vs expected `Result<Option<Vec<Effect>>,OriginalTurnFailure>`), 92 (grant passed where `&mut StepContext` expected), 931 (`RetainedWorkOutcome` error type inference); `🔌️plugin-app-impl/🎞️media-transfer/🦀️.rs` 29-50 (`NativeSnapshotFailure::into_value_error` gone, 3 arg mismatches); `🔌️plugin-app-impl/🔒️close-terminal/🦀️.rs` 15 and `📥️staging/🔒️close-ladder/🦀️.rs` 125-138 (grant-shaped `close_step(grant)` still used; now `close_step(body, effect)` returning `RetainedWorkPoll`; `ToolLatestWinsRegistry::close_step` trait bounds). Owner: whoever holds instance-app / close-ladder (the downstream agent edited ladders/commands per coordination-plan 10:50). Needs an agent; after it compiles, re-run `check -p semio-plugin-reactor-jobs --all-targets` to reach the reactor-jobs `🏛️reserved` resident-admission code.

# typedops4 (post-merge 737-740 rerun)

## 0. instance-construction (coordinator priority)
- `semio-plugin-instance-construction` `check --all-targets`: rc=0 (`typedops4-construction-3.log`). Fixes: `Ok::<_,ValueError>` pins on the `close_original_retained_step` closures (🧺️sources, 🏭️factory); `assemble` no longer captures `self` in a shared closure (progress/cursor computed up front, `initialize_component` called per branch); `close_actor` splits terminal vs live (`close_original_effect_step(cx,d,|effect|owner.step(body,effect))`, effect-shaped `ControlledRetirement::step`); `SharedUtf8::admit(text,grant,&mut None)` (the construction scope accounts the allocation, no effect exists there).

## 1. typed-ops
- `check --all-targets`: rc=0 (after the other agent fixed native-encoding `source_witness`).
- `test`: 0/2 run, third (`original_typing_cancel_keeps...`) SPINS at 97 % CPU (skipped with `--skip`; I killed it once by pid). Source fixes done in tests (unrun against a passing result): denial loop asserts "allocates nothing, publishes no source" (the prerequisite is a separately paid step now, so a capacity-only denial may spend it), `grant()` derived from `original_construction_settled_quote` (the old 1-item grant can never fund `complete.items = demand.items+2+source items`), admit loops until ready (binding, construct, settle are separate turns), anchor uses `u64` demand.
- ROOT BLOCKER (not typed-ops): with `caller.admitted_decoder()` the fixture `OriginalStoreBirth::built` never completes. `[DEBUG]` probe (removed): each of 4096 rounds returns ready=false with an identical per-turn receipt (9 items / 6064 bytes), custody never published, i.e. the construction restarts every turn (receive_prepared_construction stays Pending). Same family as the jr cold-decoder stall (`jr-command-tests-3.md`, `job-crates-tests.md`). The spin in the cancel test is the same loop in `close`. Needs the jr/decoder owner; retest typed-ops after that: `repo cargo ... test -p semio-plugin-instance-typed-ops --lib`.

## 2./3. instance-initialization + reactor-jobs
- Combined batched check with the laws crate (`typedops4-batch-3.log`, --keep-going) was queued when BUILD PAUSE #3 arrived; result appended below if it finished. Earlier compile errors in init (windows test `retired`, ephemeral `OriginalStoreBirth::default()`) were fixed in source; ephemeral test may still need a retire if admit leaves the birth non-empty.

## 4. laws crate `semio-plugin-child-group-wire-laws` (partial port, unverified)
- Findings: the "5 resolution errors" hid a whole removed API generation; resolve errors come first, typeck errors follow after they are fixed.
- Done in source: new `🏦️granted/🦀️.rs` (grant-shaped views via `semio_framework_value_clone_source_testing::drive::drive`: `step`, `bytes`, `drain_bytes`, `retire`, `PAGE`, `NONE`); fixtures (`🫙️owner`, `🛫️encoder`, `🪪️registry`, `📢️publication`) now `advance_granted(grant,effect)` with `close_child` forwarding through `effect.nested(child_grant)`; owner `Text` read/encode ported onto `PagedTextDecodeFrame` (`decode_text`) and `PagedTextEncodeProducer` through the `EncodeBank` rig (`encode_text`; label producers take source + prefix/suffix instead of `label_into`; `write_encoding_format` sites -> `produce_label(..,Original,"","")`); the paged-text `native-rig.rs` is mounted by `#[path]` as `crate::rig` (not copied); `retire_one(1,4096)` capsule tests -> `granted::retire(PAGE, |effect| capsule.retire_one(effect))`; typed-source physical test re-expressed on `ControlledRetirement` + `granted::step`; `release_empty_page` -> `release_empty_page_leased`; Cargo.toml: +5 dev deps, `maxSourceFiles` 32 -> 36 (rig adds 3 files; limit error `exceeds the file count` seen at 32).
- Known semantic changes to review: partial-text retention after cancel/refusal is now retired inside the frame (old assertions about retained partial pages for TEXT no longer hold); label cancellation mid-producer re-expressed as a segment-policy refusal; heap-exact `observe_heap_allocations` around `drive` may differ (drive measures internally). Source bytes budget (262144) not yet measured with the rig.
- Next: first successful typeck run will show the real remaining errors (ScrubLedger, decoder recipient, etc.); run `check -p semio-plugin-child-group-wire-laws --all-targets` then test.

## 5. close_original_retained_step cleanup: NOT executed (host never quiet, builds paused three times). Plan unchanged (typedops3 section). Note: the construction crate again needed per-site `Ok::<_,ValueError>` pins (9 sites), further evidence for making the API non-generic on the error type.

## Logs / hygiene
`🗑️generated/typedops4-*`; scratch scripts in the session scratchpad `typedops4-*.py`. No `[DEBUG]` left (grep clean).

## Final status (end of typedops4)
- Batch check (init + reactor-jobs + laws) FAILED at source admission for the laws crate only: `native-rig.rs exceeds the file count of its crate source boundary (0 files remain)` even at maxSourceFiles=36; raised to 48 (unverified; the admission message gives no count). init and reactor-jobs were therefore NOT compiled in that run; re-run `check -p semio-plugin-instance-initialization -p semio-plugin-reactor-jobs --all-targets` separately (laws separately) after resume, one run each. Owner of the next step: whoever resumes typedops (builds paused by coordinator).

# typedops5

## 1/2. check --all-targets (log `🗑️generated/typedops5-batch-core-1.log`, `-core-2.log`)
- `semio-plugin-instance-initialization`: PASS (earlier run `typedops5-init-check-1.log` had one error, the ephemeral test's `close_original_retained_step` inference, cured by item 5). Tests NOT run (BUILD PAUSE #4).
- `semio-plugin-instance-typed-ops`: check PASS.
- `semio-plugin-reactor-jobs`: lib compiled; 4 test errors fixed: `🏛️reserved/🧪️tests` `birth()` now drives `admit_original_construction_settled` over a quoted turn grant; `🧪️tests/🎟️admission` `JobPayloadAuthority::admit(.., &mut None)` and effect-shaped `close_step` via `semio_framework_value_clone_source_testing::drive::drive` (new dev-dep in its Cargo.toml). check PASS in core-2. Tests NOT run.

## 3. typed-ops original_typing: still 0/3, but the stall is gone
- Root cause of "construction restarts every turn" (found independently of jr8): the test funded turns with the bare settled quote while every turn of a decoder with a return table first pays its origin refresh from the grant, leaving grant depth 0, so `complete.fits(grant)` never held (debug probe, removed). Fix in the test: business grant = `original_construction_settled_quote` and `turn()` uses `caller.refreshed_turn_with_decoder(g, ..)` (refresh paid first, business narrowed by loan); receipts read via `business_receipt()`; admit loops until ready in the cancel test. jr8's `child_work_unreported()` fix landed meanwhile; runs after it no longer spin (0.2 s).
- Remaining failure (all 3, `tests/🦀️.rs:10`): the per-turn heap law `(heap.requested, heap.released) == (receipt.retained_capacity, released)` sees (12480, 0) / (8, 0) vs (0, 0): the construction's allocation (the 12480 B `Box<OriginalTypingCommit>` candidate) is accounted by the original construction/receiving effect, not by the caller's `receipt()`/`business_receipt()`. Needs a fixture accessor that includes construction-effect accounting (or the law re-expressed against it). Not done (builds paused).

## 4. Laws crate split (done in source, laws crates check PASS except re-verification of the last 1-line fix)
- `maxSourceFiles` back to 32. Old `child-operations/⚖️laws` removed; replaced by four domain crates (same dir depth, same Cargo layout): `🎒️wire-laws` (semantic-child-group-wire-laws, 7 laws), `📢️publication-laws` (publication/registry/currencies, 3), `✍️authored-laws` (authored label/operation producers, 3), `♻️source-retention-laws` (typed/paged source physical retention, 3); package names `semio-plugin-child-group-{wire,publication,authored,source-retention}-laws`. Shared pieces mounted, not copied: `🧫️fixture-json` (fixture reader), `🏦️granted` (+ `drain_retained`), `owner::authored`. Workspace `Cargo.toml` members and `📋️build-owners.json` updated; Cargo.lock left for cargo.
- Fixture API drift fixed: `reserve_one_cold`->`reserve_one` on OwnedOperationBytes, `ByteSpan::get` by value, new dev-dep `semio-framework-value-testing-admission` in the two crates mounting test-app-mutations.
- core-2 check: only error was `🧩️projection` `*source.get(..)` (fixed after). Re-verify with one check; laws tests NOT run. Stale: `🧪️tests/📦️units/🟦️.ts` + `📋️child-group-owner.json`/`📋️child-group-lock.json` pin the old single laws file (module hash, 16 laws, dependency list; already stale before the split) and must be regenerated for four targets.

## 5. close_original_retained_step cleanup (done in source)
- Definition is now non-generic (`advance: FnOnce(RetainedCloneGrant)->RetainedWorkOutcome`, error fixed to `ValueError`). Mechanical script (span-aware) over 428 files: 107 `ValueError` pins and 9 turbofish pins stripped, 27 calls with `OriginalTurnPrimary` pins moved to `close_original_retained_turn` (imports added in 2 files, others glob/qualified). Includes 3 old ticket scratch .rs files that matched. Verified: all root-workspace dependency crates of init/reactor-jobs/typed-ops/laws compile (core-1 only failed in the 5 targets). Not verified: remaining ~55 root + 24 framework packages' tests, ✏️s plugin workspaces (~25 pkgs). The big batch hit the 1 h cargo limit (`typedops5-batch-root-1.log`) and was blocked by a feature-unification-only error in `semio-framework-value-snapshot-read` testing/presence-retirement (old `admit_shared_retirement`/`box_close_step` API; unowned, appears only when a snapshot-read test crate joins the batch). Unpinned callers returning non-`ValueError` errors, if any, will surface in the next per-workspace check.

## Hygiene
No `[DEBUG]` left (grep clean). Logs `🗑️generated/typedops5-*`; scripts in session scratchpad.

# typedops6 (BUILD PAUSE #5 hit before any cargo ran; source work only)

## Build status
- Laws check (4 crates, one `repo cargo` run, log `🗑️generated/typedops6-laws-check.log`) sat in the capacity queue ~10 min behind other runs, never started; withdrawn (SIGTERM to its own queued repo process) on BUILD PAUSE #5. Items 1 (check/test laws, TS pin regeneration), 3 (init + reactor-jobs tests) and 4 (per-workspace check batches) are NOT done; they need the next resume. Pin regeneration must go through the generator target via repo (not located yet; pins `child-group-owner.json`/`child-group-lock.json` belong to the old single laws file).

## 2. Construction-effect receipt accessor (source, unbuilt)
- Finding: the 12480 B candidate allocation is posted to the receiving scope (`StepContext::receiving_child_progress()`), not to the callback's business loan.
- `OriginalJobFixtureCaller` (`🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs`) gained field `construction` set per turn from `cx.receiving_child_progress()` and accessor `construction_receipt()`.
- typed-ops `original_typing` `turn()` heap law now compares heap `(requested, released)` to `business_receipt() + construction_receipt()`. If it double counts (scope settled into the loan), the law will show it on first run.

## 5. Retained-command host released-bytes assertion (source, unbuilt)
- `🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs`, lostReservations final close loop: for every close step while the pending command still holds its `Shared<String>` key, any step that releases bytes must release exactly `close_retirement_demands(..).release_bytes` (the close quote). Limitation: the assertion only fires if the key is still open in that loop; no count assertion was added because that is unverified without a run.

## Hygiene
No `[DEBUG]` added. Scratch: none.

# typedops7

Logs: `🗑️generated/typedops7-*`. BUILD PAUSE #6 arrived while `typedops7-laws-test8.log` was running; no repo run was started after it.

## 1. Laws crates and TS pins
- `check --all-targets` for the four laws crates: PASS (`typedops7-laws-check.log`, rc=0, no errors) on the pre-edit sources.
- Tests (`typedops7-laws-test4..7.log`): publication-laws 3/3. wire-laws 5/7, authored-laws and source-retention-laws partial (a foreign test aborts the harness, see below). Fixes that landed: `fixture_path!` depth in `🧫️fixture-json` (3 -> 4 ups); the selection-unit test reads its fixture with `repo_text!` (root-relative; `include_str!` pushed two crates over the 32-file source boundary because include targets count).
- Pins: no repo target existed; the old pins were re-recorded by an ad-hoc ticket script with a `/Users/ueli/...` root. Replaced by a reproducible generator in the domain: `🧩️child-operations/📌️pins/{📜️.ts,🧬️.json,📋️.json,🟦️.ts}`, repo targets `@semio-tech/framework-plugin:child-group-pins` (write) and `:child-group-pins-check` (drift twin). The old four JSON files and their two tests were removed from `📦️units`. Invariants carried over and derived from source: 16 unique obligations == 16 instances (schema const; 32 preserved-from constant), exact 16 law names (test), 0 debug statements, host/original function hashes for the 10 retained-command host and sibling files, per-crate closure, lock edges vs manifest. Per-law diff old vs new assertions (436 -> 425): only two laws differ (`label_producer...retained_source` 23->18, `typed_source_physical8194_grant4096` 36->21); restored pointer identity, construction/terminal/scaffold heap and conservation checks and the drain-equals-allocated law; the remainder is API-intentional (NativeEncodeControl owned_bytes accounting and the partial allocation of a refused encode are no longer observable, old cursor frame size assertions have no equivalent). Pins file is stale after the later source edits and must be regenerated after resume (`child-group-pins`), then `child-group-pins-check` and the bun test (last verified current before the fixture edits: check "current", bun 1/1).

## 2. typed-ops original_typing (typedops7-t3.log)
0/3 still. The heap law shows `(12480, 0)` vs `(0, 0)`: business + construction receipt is 0, so it is NOT a double count; the new `construction_receipt()` is not populated (the 12480 B allocation is not recorded by `cx.receiving_child_progress()` in the fixture caller as assumed). Not root-caused.

## 3. instance-initialization and reactor-jobs tests (typedops7-t3.log)
- initialization 0/4: ephemeral/window birth tests see `copied_items:1, copied_bytes:56` where zero progress is expected; `ordered_windows_keep_original_grants...` fails `source.is_some()`; the ephemeral same-roots turn assertion fails.
- reactor-jobs 21/52: infer, migrate, mutation_plan, reserved registry, original_job_admission_tests and router tests fail.
Not root-caused; looks like the same migration drift as the job-return families.

## 4. close_original_retained_step per-workspace checks
NOT run (never admitted before BUILD PAUSE #6). Still open: root, framework, ✏️s and hub `check --all-targets` batches of at most 10 crates.

## 5. tests-01 retained-command host
NOT run (paused).

## Law failures root-caused after t3 (coordinator request)
- release_bytes 0 vs 8194 (`typed_source_physical8194_grant4096`): API legitimately changed. `ControlledRetirement::next_demand` first quotes the cursor-birth step (release 0); the String release is quoted only after birth. Expectation re-derived from the quote: fund birth steps from the quoted demand until release_bytes > 0, then assert == backing.
- `frame8202` heap: `encode_paged` now runs in a funded encode bank that allocates/releases scratch (13683 released). Expectation re-derived: requested - released == allocated == control.owned_bytes.
- "paged semantic text retains actual physical allocations": production-side behaviour regression in the laws fixture owner: a refused decode dropped its partial PagedText. Fixed: `decode_text` returns the partial owner with the refusal and `Text` retains it. Residual (wire paged_semantic test still dropping a non-terminal Text at test end): suspected the systemic cause from verify-wave-7 (release quotes with `copy_bytes:0`): 8 fixture-owner quotes in `🧩️child-operations` (`🛫️encoder` x2, `🪪️registry`, `📢️publication` x2, `🫙️owner` x3) quoted `release_bytes` without `copy_bytes`; all now quote `copy_bytes: next_release_copy_byte_demand()`. Unverified (pause).
- partial-locale `read_label_from_source` must refuse (production regression, not re-expressed): recorded-source encode controls had no way to carry the caller's cancel. Added `NativeEncodeControl::bind_cancel` / `observe` (consulted at every recorded checkpoint and 256-unit boundary in `checkpoint_recorded`/`advance_recorded`; a control with a bound callback cannot be paused), `EncodeBank::run_cancelable`, `encode_inline_retaining` (keeps the partial owner), and the fixture threads the outer control's callback with cumulative owned bytes. Unverified: the law still failed in test7; re-check after the `copy_bytes` quote fix (a cancelled partial that cannot drain may be involved).
- encoder-rig `assertion failed: success` (authored label tests): not root-caused; the debug probe was removed, no output was captured because a foreign test aborts the harness before libtest prints failures. Plausibly the same release quote cause (producer close in `drive_one`).
- Foreign mounted config test (`test_config_original_record_receiving_preserves_nullable_selection`): fixture-caller usage fix attempted twice (first the grant-shaped `step` without caller loan, then `Authorities::step_effect` in testing-admission), receiver still reports 0 copied items (effect quote not admitted) and its panic aborts the whole test binary ("retained record-body cursor reached Drop before terminal-empty close"), hiding other results. Workaround used only for verification: `-- --skip test_config_original_record_receiving`. The test is mounted into every laws crate through `🖥️test-app-mutations-config`; the clean fix is to stop mounting foreign unit tests into consumers (feature-gate the mounts) or to give it a proper job-turn fixture (`OriginalRetirementPump::turn` + `with_native_decoder_work`, as in the probe-document receiving tests).

## Hygiene
`[DEBUG]` probe added to `🧪️tests/encoder-rig.rs` and removed again (grep clean). Foreign probe seen, not mine: `jr9_probe` with an `[DEBUG]` eprintln in `🏭️construction/📦️effect/🧳️frame/🦀️.rs` (owner: jr9). Source edits unbuilt since test7: the 8 release quotes, pins artifacts, debug removal.

## Resume plan (one cargo run at a time)
1. laws tests (4 crates) -> fix residuals; 2. `child-group-pins` + `-check` + bun test; 3. typed-ops construction_receipt root cause; 4. per-workspace `check --all-targets` batches (root, framework, ✏️s, hub); 5. tests-01 retained-command host.

## typedops7 addendum: laws test8 (foreign test skipped; run predates the 8 release-quote fixes)
- source-retention-laws 16/17: the quote-derived release_bytes law now PASSES (cursor-birth fix verified). Remaining: `paged_source8194_frame8202_physical4096` at `♻️source-retention-laws/🦀️.rs:90` `unwrap()` on `OwnershipLimit "native encoding ownership exceeds caller limit"` (the `control.charge(source.allocated_bytes())` after `encode_paged` for a non-initial4096 mode; allocated bytes exceed the control ceiling - the net-heap expectation edit did not reach it).
- authored-laws 15/17: both label tests still `assertion failed: success` in `encoder-rig.rs` `drive_one` (a producer refusal; message not captured; re-run with the quote fix first).
- wire-laws (test7): 5/7 as above. publication-laws 3/3.
