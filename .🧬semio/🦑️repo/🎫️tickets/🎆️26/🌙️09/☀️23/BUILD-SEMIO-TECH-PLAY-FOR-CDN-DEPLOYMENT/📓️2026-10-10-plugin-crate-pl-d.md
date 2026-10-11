# 2026-10-10 plugin crate pl-d (window-config, SharedUtf8, rest, testkit)

PARKED at usage limit. All edited files parse; no half edit in flight. No cargo running.

## Result lines (private dirs `.🧬semio/🦑️repo/⚡️cache/play-fleet/pl-d/{target,build}`, helpers `chk.sh`/`chk2.sh` there, JSON in `check.json`, errors in `errors.txt`)
- `cargo check -p semio-framework-plugin --lib` native default: 0 errors (success).
- `... --lib --target wasm32-wasip2` (default and `--features component-guest`): Finished, 0 errors.
- `... --lib --features component-guest,artifact-app-testing` native: 0 errors (after testkit fixes below).
- `... --lib --tests --features component-guest,artifact-app-testing`: 1451 errors (test sources not migrated; not started).
- `-p semio-framework-plugin-host --lib` native: 0 errors after my two fixes (build-finished success not re-asserted after the very last run; first verify).
- `-p semio-framework-plugin-describe --lib`: 3 errors, all `E0063 missing field retained in Budget` in `🖨️describe/🛂️descriptor-emission/🦀️.rs` lines ~427, 493, 607 (`semio_framework::kernel::Budget` needs a `retained: RetainedCloneGrant` field; model: host `🖥️host/⏳️runtime/🦀️.rs` `wit_reactor::Budget{retained: ...}` / `budget.retained`).
- `-p semio-framework-os --lib`: 5 errors, all in os-domains territory `🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs` (`MediaType`, `MediaWireFormat`, `MediaForm`, `MediaPortSpec`, `PortMultiplicity` lack `ArtifactCanonicalJsonTree`).
- NOT written: `GREEN os` line (describe + os host + wasm for host/describe/os still open).

## Fixes
- window-config: removed dead old `BoundedWindowConfigPreparation*` (replaced by `RetainedClonePreparationFactory`); `WindowConfigPartition` ctor fields; new `refresh_demands`/`refresh(authority, grant)` on typed owner (mirrors transient); `DirectIngress.addresses`; removed duplicate registry `maintenance_retirements_step`; returned-read retirement in partition close/maintenance; retained pack load: removed duplicate `impl WindowConfigPackLoad` methods, added `WindowConfigPackLoad::close_step`, trait `demand_bytes`, Drop checks `inner.is_none()`, removed conflicting `RetireOwned` derives on `ValueFrame`/`ValueWrapper`; `WindowConfigReplaceCursor` needs `RetireOwned` bounds on `S`/`M`.
- transient: registry `refresh_demands`/`refresh(authority, grant)` (pl-a also listed this name; only one copy exists), ephemeral `advance` returns `ValueError` and `Prepared(checkpoint, progress)`.
- SharedUtf8: `ActionMeta.actor` is `SharedUtf8` (confirmed by the author's mounted-frame retirement using `has_owner`/`close_original_lease`); `instance_actor` stays `String`, ActionMeta sites use `.into()`; `ToolRunEntry.actor` is `SharedUtf8`; composition metadata parts `actor`/`publication_actor` are `SharedUtf8` (retired via `original_allocation_bytes`); unfunded `SharedUtf8::from` Arc births remain in composition metadata `take_ready`, `create_app` actor, tool-machine (follow-up: funded `SharedUtf8::admit`).
- checkpoint `restore` now takes `(identity, NativeSnapshotDecodeOwner)`; `restore_now` builds a local authority (`RESTORE_AUTHORITY_BYTES`) because the WIT `restore(state)` carries no grant.
- rest: `reserved_job_outcomes`/`reserved_job_session`/`extension_invocation_failure` made `pub(crate) mod` (crate-root paths), `drive_candidate_views` grant form, `constructor_receipts` unsafe-index assignment, `MemberOpenOperation` demand method names, replacement poll uses `refusal == Cancelled`, `TimeTravelLedger.member` pub(crate), typo `>>>>>`, `bounded_config_store_one_item_preparation_factory` + `ArtifactCanonicalJsonTree` bound, EntityIdentityAuthority observer coercion (4 sites), inference gateway `axis.as_str()`, composition owner/group fixes, `WorldSunConfig`/`WorldProjectionConfig` derive `RetireOwned + RetainedClone + CanonicalJsonTree`.
- testkit (`artifact-app-testing`): macro `with_authoring_identity!(|identity| body)` (root, `#[macro_export]`, gated test/feature); identity threaded through `artifact_app_laws` (`load_document`, `settle_extension_invocations` gained an `identity` param, `tick_backbone`, `ingest_operations`, `preview_addressed_action` lost its identity arg); history-edit-acceptance harness uses a local `acceptance_policy()` and the macro.
- retained-command: `BoundedArtifactCommandWork::{work_demands, terminal_frame_release_bytes}` implemented.
- host: runtime `call_poll` maps `ActorCallRefusal` via `retained_turn_wire::failure_from_wit`; shard `select_pending_authority` owner is `mut` and budget is `take()`n so `retain_lifecycle_retry(owner)` compiles.

## Next steps
1. Add `RetireOwned` for `app::InteractionHoverState` (main request, not started).
2. Describe: add `retained` to the three `Budget` literals; re-run describe, then host/describe/os-host on `--target wasm32-wasip2`; report os-host errors (os-domains workflow canonical trees) to main; write `GREEN os` only when all pass native and wasm.
3. `--lib --tests` with the testkit features (1451 errors: classify by file, bulk-mechanical first), then run the plugin unit tests.
4. Post corrections line to main: testkit green (done), ActionMeta.actor is SharedUtf8, `settle_extension_invocations` signature gained `identity`, `checkpoint::restore` signature, `with_authoring_identity!` macro exists.

## Resume update
- Plugin lib native 0, wasm 0, testkit-feature lib 0, host lib 0, describe lib 0 (native only). Fixed `manifest` `MediaType` duplicate RetireOwned (leaf impl kept, derive removed by os-domains).
- describe: `describe_budget()` helper with a fixed `RetainedTurnInput`.
- `InteractionHoverState` (alias of `BTreeMap<String, DomainHover>`) already satisfies `RetireOwned` (probed with a compile-time assertion, removed).
- Testkit additions in `artifact_app_laws`: `fixture_mounted_policy()`, `fixture_identity()` (`&mut fixture_identity()` as a trailing call argument), `fixture_retained_turn()`.
- Plugin `--lib --tests --features component-guest,artifact-app-testing`: 1451 -> 568 errors after bulk passes (observer coercion at ~210 identity creation sites; ~210 call sites got `, &mut fixture_identity()` / policy; Budget literals got `retained:`; `ActionMeta{actor: String::new()}` -> `Default::default()`). Host-crate test files only had `JobBudget` reverted and `Budget` literals given an inline `RetainedTurnInput` (host tests are not compiled yet).
- Remaining 568 are job/reactor API: `poll_kernel` (7 args), `step_job` (5), `start_job` (2), `JobStep`/`StepOutcome`/`InteractiveJobCloseStep::Refused{..}` shapes, `StepContext::new` (3/7 args), `build_*_store_owners(grant)` test overrides (decision 9A: delete), `AsyncTask::new` (3 args), `SnapshotRetirementStep`/`PluginCloseStep` leftovers in test fixtures, `close_step(1, bytes)` -> grant, `RuntimeAppCell::new` (2), `plugin_continue_typed_operations` (+identity,+cx), `pump_until` (5), `register_child`/`open_child` forms. Biggest files: plugin-runtime-plugin-builder-contract 173, composition 42, app-typed-command-full-operation 34, interaction dispatch tests 30, jobs migrate/mutation-plan tests 48, extension-retirement 25.
- Plugin unit tests cannot run until the whole test target compiles.

## Test-target migration (parked on main's budget pacing)

- `cargo check -p semio-framework-plugin --lib --tests --features component-guest,artifact-app-testing`: 1451 -> 33 errors (15 files, all test sources; the non-test lib, wasm and testkit were not touched in this stretch and stay green).
- Ported: dispatch close-pump tests (`admit_batch` sessions, `outcome_pending`), jobs migrate/mutation-plan (shared `start_fixture`/`slice_fixture` in the jobs unit tests), child-emission close/step grants, extension-retirement (`echo_step`/`invoked`), composition `RetainedClone` derives + `install_unscheduled_catalog` results, completion/agent-lane/reserved-tool-job/bounded-reload job impls (`borrow_outcome`, `ReservedJobOutcomes`), folder-reload-route wrappers, publication-retirement-authority (default owner hooks, `admit_begin` helper), typed-command (`begin` grant, `advance_..._one(grant)`), query/media/metadata imports. `protocol::{HistoryConflict,HistoryMessage}` re-exported from the spr root for the child-member-registry test.
- Remaining (33): window-config tests (`quoted` helper, 9 errors), tool-run family (3 files, tool-run triage agent), app-declarations-fixture (needs `ArtifactNativeSnapshot` for fixture snapshots, io_run helper over `io_run_with_snapshot_control`, 6-arg SQLite provider owners, `create` fn signature), lib-root test mods (6), 2 in publication-retirement-authority, misc singles.
- `ensure_reserved_emit_bounded`: never called in any history revision (5487a5, 48e475, c44e96 or earlier); deleted, no call restored.
- Plugin unit tests have not been run yet (target does not compile).
