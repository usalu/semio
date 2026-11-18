# Native fixtures: shooting lib tests, second pass (still not green)

Command: `SEMIO_UNIT_CACHE_REMOTE=off repo cargo --store=shared --configuration=prototype -- test --no-fail-fast --manifest-path <shooting rust workspace> -p semio-s-artifact-shooting-shooting -p semio-s-artifact-shooting-shooting-shooting -p semio-s-artifact-shooting-shooting-tests --lib` (no `--skip`).

## Final counts (last full run, all `[DEBUG]` probes removed afterwards)

| package | before | after |
| --- | --- | --- |
| sqlite snapshot (`...-shooting-shooting`) | 14 pass / 3 fail | 14 pass / 3 fail |
| editor (`...-shooting-shooting-shooting`) | 22 pass / 45 fail (+1 stack overflow, skipped) | 22 pass / 46 fail, 0 aborts |
| `...-shooting-tests` | 27 pass / 0 fail | 27 pass / 0 fail |

The editor now runs all 68 tests: the 46th failure is `two_instances_converge_disjoint_edits_via_backbone`, which no longer overflows the stack and fails like its siblings. Not green: each fix peeled off one layer and exposed the next unfinished native-migration layer. One cold rebuild after a framework edit costs about 70 minutes, so the passes were few.

## Done

1. Receiving-scope settlement per fixture call (open cause A): the 45 "original receiving scope still owes its enclosing semantic receipt" failures are gone from every run. `OriginalFixtureScope::cx()` ends the previous call's turn (`settle_receiving_scope`) before lending the context; new `fixture_end_turn` is called at the start of `fixture_renew_turn`, `ensure_fixture_cancellation_issuer`, `close_registered_fixture_app`, `render_registered_fixture_app_over` and `release_fixture_render`. This mirrors the host, where the dropped turn context settles its scope (`OriginalJobFixtureCaller::turn_original`).
2. Stack overflow (open cause B): `VcsArtifactApp::with_registry_on_bus_boxed` / `with_registry_boxed` build the roughly 1 MB instance field by field in its final heap allocation (`Box<MaybeUninit<Self>>`); the by-value constructors delegate to them. Fixture constructors `new_registered_app_boxed_with_members`, `new_app_with_registry_boxed(_and_members)`, `paired_registered_apps_boxed_with_members` return boxes, and `assert_two_registered_instances_converge_with_members` holds boxes. The shooting `ShootingApp` fixture holds `Box<VcsArtifactApp<..>>`. No stack was enlarged and the test binary no longer aborts.
3. Cursor work quotes (open cause E): `next_computational_work` added to `ArtifactStoreVcsRetirement`, `ArtifactStoreRevisionAccumulatorRetirement`, `ArtifactStoreCursorRetirement`, `ArtifactStoreMutationDagRetirement`, `ArtifactStoreHistoryLedgerRetirement`, `ArtifactStoreEnvelopeRetirement`, `ArtifactGenesisRetirement`, `ArtifactStoreDisplacedSnapshotRetirement` (one unit per paid step, delegating to the active child). The 12 paid-app-close failures moved from "lacks a work quote" to a later layer (see open).
4. Sqlite `NativeDecodeRecipientCursor` gained `close_step_controlled`; "retirement cursor lacks original controlled closure" is gone and `every_geometry_word...` now fails later ("original snapshot turn failed").
5. Standalone store dispatch Work loan (open cause D): `EntityIdentityAuthority` gained `IdentityWorkLender` / `with_work_lender`; `fixture_identity()` installs a fixture Work authority so every local native identity hop commits under a Work loan. The gumball standalone test path is covered by this; its last run failed earlier on qualification, so it is unverified.
6. `io_run` shared lifetime (open cause C1): new resident installers `NativeDecodeControl::install_construction_recipient`, `NativeEncodeControl::install_work_recipient`, `NativeEncodeControl::install_construction_recipient`; the test harness `io_run` installs all four loans on its controls inside nested loans and closes through `close_decode_installed`. Unverified: no failing test calls it.
7. Tool qualification (new layer found): `OriginalToolQualification::taken()` and `admit_wire` now closes a qualification whose registration already transferred instead of advancing it into the invalid phase-3 `demands` (28 dispatch failures with "tool qualification requires its original catalog, verb and receiving continuation"). That layer is passed; the next one appears.

## Open (evidence from the last run)

- Editor, 28 dispatch failures: "native command admission lost its original registered row, wire, verb or decoder" (`OriginalCommandAdmission` after qualification; the admitting branch has the same close-after-transfer shape and needs the same treatment).
- Editor, 14 close failures: "store root issuer is absent" (document store close after the quotes above).
- Editor, 2 failures (`interaction_select...`, `an_id_less_gesture...`): "native encoding refused: original work requires its original caller loan" in `run_framework_reserved_commit_unit`, which resumes the identity from a stored receipt. It needs `cx.with_native_encoder_work` (or an owned Work snapshot) in that identity; `fixture_identity()` is not on this path.
- Editor, 1: "native retirement requires its original producer invocation".
- Sqlite `actual_artifact_declaration_mount_routes_queryable_files`: not a harness lending problem. Backtrace: `sqlite-snapshot::transfer::lex` calls `NativeDecodeControl::copy_text` on a control whose allocation binding is Forwarded (`SqliteSnapshotControl::allocation_stage` relay inside `with_encoding_receiver`), and `charge` refuses Forwarded outside a quoted effect. `lex`/`parse_table` and the other `transfer` allocations must use the effect-based allocation API. Library work.
- Sqlite `independent_complete_geometry_edit_public_io`: "snapshot retirement requires a larger original grant" at `retire_inputs` (demand copy 9, depth 2, quote 1, turn work 0, grants ample). Neither `retire_decode` nor `retire_encode` reported a refusal, so the stalled retire goes through a third effect path; not located.
- Sqlite `every_geometry_word...`: now "original snapshot turn failed" (unlocated).
- Siblings not restored: raster `semio-s-artifact-raster-raster-schema` needs about 59 API migrations across `owned-map`, `image-payload` and `selection/coverage` (not cheap). Procedural `retirement-driver` testkit (3d and the identical 2d file) was rewritten to the effect-based API (`close_step(body, effect)`, `admit_owned/shared_lease` birth helpers via `run_fixture_effect`); not compiled, because it is outside the requested crates.

## Changed files (all under `🧰️framework`, `✏️s`)

Framework:
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📥️staging/🚪️ingress/🦀️.rs`
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🦀️.rs`, and `.../🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs`, `.../🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs`, `.../👯️paired-apps-history-and-convergence-laws/🦀️.rs`, `.../🔎️declared-verb-probing/🦀️.rs`
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🦀️.rs`
- `🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🦀️.rs`
- `🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs`, `.../🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs`
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️typed/🦀️.rs`, `.../🎟️typed/🎟️proof/🦀️.rs`
- `🔨️modules/🌱️value/🛬️decode/🫴️recipient/♻️retirement/🦀️.rs`, `.../🛬️decode/🏭️construction/🦀️.rs`, `.../🛫️encode/🏭️construction/🦀️.rs`, `.../🛫️encode/🎟️action/🦀️.rs`
- `🔨️modules/🧪️test/🛬️original-snapshot-owners/🦀️.rs`

Shooting / procedural:
- `✏️s/🔌️plugins/🎥️shooting/.../✏️editor/🧪️tests/🔬️unit/🦀️.rs` and `.../🎮️commands/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs` (boxed `ShootingApp`)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🧰️support/🔬️retirement-driver/🦀️.rs` and the `generation2d` twin

## Process notes

- Two full runs were lost: the disk filled during linking (`No space left on device`, 25 GB free at the start), and the coordinator deleted `🗑️generated` mid-run. Logs now go to `🗑️generated/shooting-run*.txt` and are deleted with the folder when the ticket closes.
- The repo mcp server (`semio`) was unreachable (CONNECT_TIMEOUT), so no `ticket_close`.

# Third pass (coordinator follow-up): final counts and state

Counts of the last full run (`-- --test-threads=4`, the repo runner's own stack): sqlite 14 pass / 3 fail, editor 22 pass / 46 fail, `...-shooting-tests` 27 pass / 0 fail. The counts did not move, but 45 of the editor failures now run through the full dispatch and settle path and fail in teardown only. Not green; I stopped because every full rebuild costs 40 to 70 minutes.

## What this pass fixed (with the evidence that it took effect)

1. Command admission (priority 1): the real cause was a state-machine gap, not a missing registration. After `OriginalCommandAdmission::take_into`, `admit_wire` called `advance` on a phase-2 admission and hit `invalid()`. It now closes a taken admission (`taken()`), as I did for the qualification last pass. The 28 "native command admission lost its original registered row, wire, verb or decoder" failures are gone.
2. Context admission: `ArtifactOwnedContextHandle::admit` is handed `artifact_owned_tool_birth_grant()`, which carries zero copy bytes, while `SealedShared::source_admission_demand` needs copy bytes, so every admission was a silent `Progress(default)` and then a refusal. `admit` now widens the grant component-wise to the real demand. The 29 "context root retains its original projection or Store issuer" failures are gone.
3. Store root issuer (priority 2): `ArcRootRetirement::admit_original/_snapshot` took the issuer out of the caller's slot before checking affordability and dropped it on an early return, so the next call found "store root issuer is absent". `admit` now works on the slot and the wrappers hand an unspent issuer back. Also fixed the genesis quote (`ArtifactGenesisRetirement::demands` now quotes `ArcRootRetirement::admission_demand()` and the box demand for an active child).
4. Reserved commit unit (priority 3): `StepContext::native_encoder_work()` returns an owned `NativeEncoderWork` snapshot (new in the job context crate) and `run_framework_reserved_commit_unit` installs it on the resumed identity through `IdentityWorkLender`, charging the performed work back with `consume_fuel`. The "producer invocation" failure was a fixture gap: the cold close wallet used `run`; `close_step_cold` now uses `run_invoked`, and the io testing harness opens the invocation (`widened`/`invoked`) around its close effects.
5. Sqlite `lex`/`copy_text` (priority 4), library fix: a forwarded `NativeDecodeControl::charge` / `NativeEncodeControl::charge` now performs the historical Admit (`NativeAllocationLoan::begin(.., physical=false)`, take the verdict, `finish`) through the same loan the stage relay already understands, and is a no-op inside a live quoted transaction. This replaces the blanket refusal; `mount_routes` moved past it (its failure is now in the harness close, which the invocation change above addresses).
6. Default quotes: `ErasedSnapshotRetirement::next_computational_work` and `RetirementCursor::next_computational_work` now default to one unit per paid step (`u64::from(!terminal_is_empty())`) instead of refusing, which retires the whole class of "lacks a work quote" failures (60 implementors) in one place. Tradeoff: an owner that nests more than one unit of work must override it; the work accounting then refuses typed, not the quote.
7. `ShootingApp::drop` and the store retirements no longer leak `pending` on the metadata error path of `advance_latest_wins_command_one` (it re-inserts before returning).

## Still failing (evidence)

- Editor, 29 tests: `PendingLatestWinsCommand` Drop ("latest-wins admission must transfer or cursor-retire its exact command/key/raw owners") at the end of `advance_latest_wins_command_one`, on the success path `Ok(true)` of `start_typed_command_operation`: `command_owners_are_empty() && metadata_is_empty()` is false after the start (its `meta`, `original_caller` or `construction` admission is not moved or terminal). Neither the admission-unit failure nor the start failure print, so the dispatch itself succeeds.
- Editor, 13 tests: close ladder stalls on rung `OwnedStores` ("registered fixture did not reach its exact terminal-empty witness", quote copy 0, capacity 784, depth 5, against a grant of depth 4096), i.e. one cursor quotes a step it then refuses with no progress. Editor, 2 tests: "captured shared original changed its Work or destination" (`factory/🦀️.rs` `invalid()`), and 1 fixture test (history edits 0 cases).
- Sqlite `every_geometry_word...` and `actual_artifact_declaration_mount_routes...`: both end in "snapshot effect retains its original encoder settlement" (`NativeSnapshotEffectProgress::reset_completed`), raised after the `charge` change, so a settlement is left open by the new Admit inside an allocating effect or by the harness close. `independent_complete_geometry_edit_public_io`: the retire step polls with an ample grant and returns no progress for `T = (Option<RecordSpec>, Option<RecordValue>, Option<IoPayload>, Option<IoPayload>)`, the `admit_retirement_cursor` of the head owner returns `None` (owner not `retirement_ready`).
- The fast loop: after one full run the editor test binary can be run directly (`RUST_MIN_STACK=67108864 RUST_BACKTRACE=1 <binary> --exact <test>`; without the repo runner's stack the default 2 MiB overflows, which is unrelated to the boxed apps).

## Files changed this pass (all paths under `🧰️framework/` unless marked `✏️s/`)

- `🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️typed/🦀️.rs`, `.../🎟️typed/📥️admission/🦀️.rs`, `.../🎟️typed/🎟️proof/🦀️.rs` (`taken()` and close-after-transfer)
- `🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🛬️lifted/🧬️context/🦀️.rs` (grant widening; `[DEBUG]` probe)
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🧊️reserved-routes/🦀️.rs` (work lender) and `🔨️modules/🧵️job/⏱️context/🦀️.rs` (`NativeEncoderWork`)
- `🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🦀️.rs` (`run_invoked`)
- `🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🏭️factory/🌳️root/🦀️.rs`, `🔨️modules/🌱️value/♻️retirement/🦀️.rs` and `.../🧬️contract/🦀️.rs` (default quotes; `[DEBUG]` probe in `native_retirement_reply!`)
- `🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs`, `.../🧮️cursor-ledger-and-vcs-retirements/🦀️.rs`
- `🔨️modules/🌱️value/🛬️decode/🦀️.rs`, `🔨️modules/🌱️value/🛫️encode/🦀️.rs` (forwarded `charge`)
- `🛍️products/💻️os/🔨️modules/🚪️io/🧰️testing/🦀️.rs` (invoked close), `🔨️modules/🧪️test/🛬️original-snapshot-owners/🦀️.rs` (typed `original snapshot turn failed: <cause>`)
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🖱️inline-interaction-and-latest-wins/🦀️.rs` (re-insert on the error paths; `[DEBUG]` probes)

## Temporary `[DEBUG]` probes left in the tree (remove with the next green run)

`grep -rn "\[DEBUG\]" 🧰️framework`: `native_retirement_reply!` (`no invocation`), `io/⏱️control/🛫️snapshot/🦀️.rs` (`retire stalled`, `retire step ...`), `decl-app .../🧬️context/🦀️.rs` (`context admission refused`), `.../inline-interaction-and-latest-wins/🦀️.rs` (two probes).

# Fourth pass (continuation): sqlite green, editor failures moved past their old layers

Counts of the last probe-bearing build, run from the test binaries (`RUST_MIN_STACK=67108864`, editor one process per test because the 4-thread run leaks the UI arena of a panicked test into later tests on the same thread): sqlite 17 pass / 0 fail, editor 21 pass / 47 fail, `...-shooting-tests` 27 pass / 0 fail. The `[DEBUG]` probes were removed afterwards (`grep -rn --include='*.rs' "\[DEBUG\]" 🧰️framework ✏️s` is empty; every probe-only file is byte-identical to `HEAD` again). The probe-free rebuild was SIGKILLed from outside three times (`repository:command was killed by SIGKILL`, exit 137, after 8 to 40 minutes); the longest attempt compiled 476 crates up to and including `semio-plugin-instance-typed-ops` and `semio-plugin-testing-laws` without an error, and the shooting crates and their tests are unchanged since the probe-bearing build that produced the counts, so the counts are those of that build, not of a probe-free rerun.

## Fixed this pass (with the evidence)

1. Latest-wins drop assertion (29 tests). `PendingLatestWinsCommand::metadata_is_empty` also requires `construction.terminal_is_empty()`. A settled `OriginalConstructionAdmission` keeps its Source `binding` until an explicit `close_step`, so the success path of `advance_latest_wins_command_one` dropped a pending command whose construction still held its binding. The start branch now retires the construction (`semio_plugin_job::original_steps::unbuilt`, one funded close turn, reinserting the pending command until it is terminal) before it calls `start_typed_command_operation`, and the closing branch does the same before it moves the metadata out. The drop assertion is gone from every run.
2. "captured shared original changed its Work or destination" (3 tests: gumball store, wasm store, history acceptance close). Every `StepContext::with_original_effect` opens its own Work issuer and scope, so `OriginalWorkBindingIdentity::matches_effect` (scope and issuer and Source) can never hold across two effects, while `FactorySharedRetirement` is captured by one effect and retired by later ones. New `OriginalWorkBindingIdentity::matches_source` compares the Source dispatch only; `FactorySharedRetirement::close` and `close_original_birth_step` use it.
3. Sqlite `mount_routes` and `every_geometry_word` ("snapshot effect retains its original encoder settlement"). The marker's `retirement_ready` demands `work_claim == None`, but a finished encode effect leaves its claimed work in the marker until the next effect begins. `NativeEncodeEffectState::settle_claimed_work` (claim and report) is called by `NativeSnapshotEffectProgress::reset_completed` first. A temporary state dump showed `work_claim=true actual_work=0 work_reported=0` and nothing else set.
4. Sqlite `public_io` and the two tests above, second layer ("snapshot retirement requires a larger original grant"). `retire_inputs` quoted the next step with `next_copy_byte_demand()` / `next_release_byte_demand()` / `next_depth_demand()` (body 0) but ran `ControlledRetirement::step` with `body = grant.maximum_copy_bytes`; a probe at the idle return showed quote copy 9 against a stepped demand of 516/561. It now quotes `next_demand(grant.maximum_copy_bytes)` once. All 17 sqlite tests pass.
3b. `ArtifactGenesisRetirement::demands` quoted the Arc-lease pack branch with only the birth capacity; it now quotes `arc_lease_admission_demand::<Vec<u8>>()` (copy plus capacity plus depth), which `admit_arc_lease` needs. The ladder rung `OwnedStores` no longer idles; the close now fails typed one layer later (see open).
5. Maintenance stage 0 drove a worker whose producer build was still owed through a one-item maintenance loan (`original_lifecycle_grant` caps `maximum_items` at 1), failing with "original lifecycle receipt exceeds its exact phase loan" for 13 items. `MountedTypedCommandFullOperation::producer_build_pending` (new) keeps `maintenance_tool_selection` away from such an operation; its own publication turn funds the build. The 4 gumball failures with that message are gone.
6. `history_edit_acceptance_law!("shooting", ..., "../..")` rooted the fixture search at the editor folder, which holds no `🦠️mutation` cases (they live under `✳️any/🧪️tests/🧫️fixtures`): "none of the 0 committed and 0 derived editable case(s)". Now `"../../.."`.
7. `wasm::tests::shooting_store_type_alias_constructs_from_an_empty_envelope` had an unbounded `while` over a stalled close; it is bounded like the gumball loop and asserts the terminal witness.

## Open (evidence from the last probe run)

- Editor 21 tests: `settle_registered_typed_operation` fails with "made no progress for its idle turn bound". The mounted operation sits in `Worker` with `session_rejected` (the pending worker source) and `session = None`: `RefusedWorkerSource::admit_pending` -> `WorkerJobSessionAdmission::advance` stage 1 -> `OriginalWorkerJobAdmission::advance` -> `OriginalWorkerPageReservation::admit_step` returns `Ok(false)` without any progress on every turn, grants ample, not cancelled, not yielding. Inside it the first non-progress candidates are `OriginalJobReturnOrigin::admit` (`receiver.prepare`: `cx.admit_original_return_table()` and then `claim_original_sized_return`, which needs a free claim on the Source return budget; the cancellation issuer already holds the one live claim, see cause 7 of the first pass). A try that served the return birth request on the lent context (`OriginalJobReturnBirth::service` before every renewed turn) changed nothing for these 21 tests and made 4 others fail with "original Source clock declined new construction", so it was reverted.
- Editor 13 tests (`original caller paid app close`) and 2 store tests ("the store quotes its close"): "original cursor birth differs from its defining layout" in the close ladder, now reachable because of 3b. The type is not named yet: add a type-name probe in `retirement_cursor_birth_demand` (value `♻️retirement/🦀️.rs`, the declared `retirement_birth_bytes` differs from `size_of::<T::RetirementCursor>()`), the same defect class as `DecodeReport`, `CodecFrame` and the typed snapshot frames of the first pass.
- Editor 2 tests (`history_edit_inputs_resolve`, one more): "original native producer omitted observed native work" (`receive_observed_work`: the close producer's reported work is below the decoder's observed native work).
- `two_instances_converge...`, `ingest_operations...`, `undo_redo...`, `every_example_load...` share the 21-test settle stall.

## Files changed this pass (all under `🧰️framework/` unless marked `✏️s/`)

- `🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🖱️inline-interaction-and-latest-wins/🦀️.rs` (construction retirement on the start and closing branches)
- `🔨️modules/🌱️value/🎟️control/🧬️contract/🎟️work-action/🫴️borrow/🦀️.rs` (`matches_source`), `🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🏭️factory/🦀️.rs` (uses it)
- `🔨️modules/🌱️value/🛫️encode/🎟️effect/🦀️.rs` (`settle_claimed_work`), `🔨️modules/🚪️io/⏱️control/🦀️.rs` (`reset_completed` calls it)
- `🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs` (`retire_inputs` single `next_demand` quote)
- `🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs` (pack branch quote)
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🏭️producer/🦀️.rs` (`producer_build_pending`), `🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/🦀️.rs` (selection skips it)
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (law root `"../../.."`), `.../✏️editor/🌉️wasm/🧪️tests/🔬️unit/🦀️.rs` (bounded close loop)
- All probes are removed; the previous pass's four probe files are clean (`🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs`, `♻️retirement/🦀️.rs` macro, `🧬️context/🦀️.rs`, `🖱️inline-interaction-and-latest-wins/🦀️.rs`).

## Process notes

- A rebuild after a framework edit costs 26 to 61 minutes; the `.cache/b/r/m/u/l` lane is garbage collected after a run, so copy the test binaries out right after `--no-run` (`cargo test --no-run` through `repo cargo` prints their paths) and run them directly. The sqlite tests spawn `bun`: put `.cache/b/t/m/bun/<hash>` on `PATH` and run from the package folder.
- The repo mcp server (`semio`) was unreachable (CONNECT_TIMEOUT) in this session too, so no `ticket_close`.
