# 2026-10-10 plugin crate pl-b (job-session / outcome API)

Scope: `semio-framework-plugin` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`), error class "job-session/outcome API" plus the eight `InteractiveJob` impls that still returned `StepOutcome`. Reconstructed forward against the green job crate (`try_admit_owned`, `checkout_outcome`, `checked_out_outcome`, grant-acknowledge, `JobOutcomeBorrow`/`JobOutcomeDescriptor`).

## New shared plugin-local components

- `👷️job/📤️outcomes/🦀️.rs` (`reserved_job_outcomes`):
  - `ReservedJobOutcomes`: one `RetainedJobPublication` (preview, checkpoint, fault) plus one `RetainedPayloadBuilder` (commit output). `advance(kind, source, cx)` pays one frontier per turn and lends the sealed pages; `borrow_outcome` resolves the descriptor; `retire` closes the delivered publication inside the next running step; `close_demands`/`close_step` are the physical close ladder.
  - `BoundedFaultText` (480 byte inline fault source), `receive_step`, `close_result`.
  - `CleanupTurn` / `CleanupPublication` / `CleanupJob` / `drive_turn`: shared step driver for the two runtime cleanup jobs (a job decides a turn, the driver publishes it page by page).
- `👷️job/🚪️session/🦀️.rs` (`reserved_job_session`): `admit_mounted` / `admit_batch` build a `WorkerJobAdmissionContext` from the caller's own preparation grant and call `try_admit_owned`; a refusal leaves the original job and parameters in `RefusedWorkerSource<J>` (same close API the dead `WorkerJobSessionAdmissionRejected` had: `retirement_demands`, `begin_close`, `close_step`, `terminal_is_empty`), closed by paid turns.

## Fixes

- Eight `InteractiveJob` impls migrated to `step -> Result<Option<JobOutcomeBorrow>, ValueError>` + `borrow_outcome` + granted close:
  - `framework_reserved_job!` macro jobs (20 routes): fixed 34 byte state buffer, `pending` outcome kind, commit output = the admitted `raw` envelope (the consumer still checks `retained_payload_eq_slice`), oversized output faults with the static message.
  - `FrameworkConfigurationBinaryJob`, `TypedCommandFullOperationJob`, `NaturalFileImportJob` (leftover half-merged `close_step` tail deleted; `seal`/`close_turn` never existed), `RuntimeLiveCleanupJob`, `RuntimeCloseCleanupJob` (via `drive_turn`).
  - `ChildEmissionPreviewJob`: field `emit` is `Option<Emit>` again (the `RefCell` side was the stale half), stale `close_demand` deleted.
  - `ArtifactStoreInitializationJob` / `ArtifactReservedToolJob` were already migrated by rn-os.
- Session consumers moved to the grant protocol (receipt taken once, outcome read through `checked_out_outcome`, descriptor acknowledged by paid turns, then `resume` or `begin_close`): `drive_framework_reserved_worker`, `drive_agent_lane_preview` (takes the close grant as a parameter now), `ActiveMediaExport` (+ poll), `ActiveArtifactEnvelopeDecode`, `ActiveArtifactStoreReplacement::drive_initializer` (candidate / failure custody is taken after the acknowledge because `checked_out_job_mut` is only available then), typed-operation creation (`admit_mounted`), `RuntimeLiveCleanupPump`/`RuntimeCloseCleanupPump` (`outcome_pending` replaces the outcome slot; job status is read after acknowledge), `reactor/💼️jobs/💡️infer` (`outcome_pending`, `SeenInference` copies payloads before acknowledge).
- `⏯️tool-run`: direct-driven jobs use `StepContextOwner` (born lazily with the turn grant, closed after the job in `ToolRunJobSlot::close_step`), `drive_step` returns a borrow that is reduced to an owned summary before the entry is touched; `pending_job_outcome` slot removed.
- `⚛️reactor/🩹️patches`: `StepContext::new` / `StepBudget::new` with the unfunded `NO_RETAINED_WORK` grant (`SurfaceReconcileJob::drive_one` only spends fuel).
- Removed dead old-shape helpers `retained_job_payload`, `plugin_quoted_demand`, `plugin_outcome_close_demand`.
- `settle_framework_reserved_admission` (pub law helper used by ~20 dependent test crates) no longer calls the reactor job registry (its signature now needs an `OriginalJobAdmission`, `IoRunControl`, `SqliteSnapshotControl`, `StepContext`); it completes the spawn-job with the output the registered `FrameworkReservedBoundedJob` produces (`framework_reserved_job_output`).
- `reactor/💼️jobs/🦀️.rs`: the reserved-job factory is `crate::app::framework_reserved_job_factory`, not `plugin_runtime`.
- `InteractiveJobCloseStep::Refused(kind)` tuple patterns -> struct patterns (root, maintenance-ladder).
- `MountedTypedCommandFullOperation`: `session_rejected` is `RefusedWorkerSource`, stale `terminal_outcome` removed (lease file, composition owner), missing fields added to the composition owner literal; `drive_worker_step(grant)` callers (maintenance-ladder, `drive_typed_operation_worker`) fixed.

## Decisions

- Sessions are admitted with `mounted_policy.preparation`, acknowledged and closed with `mounted_policy.close`; `BatchDriveConfig.retained` follows the admission grant.
- A direct-driven job (tool-run) keeps its own `StepContextOwner`; lent outcomes are never stored, only summarised.
- Terminal outcomes are acknowledged before `begin_close` only where the owner needs the checked-out job afterwards (store initializer, cleanup pumps); otherwise the session close ladder acknowledges.

## Not done / debt

- cfg(test) sources still use the old API (`work_grant`, `StepOutcome`, `ScriptedPreviewJob`, 2-argument `drive_agent_lane_preview`); they are outside `cargo check --lib`.
- `🖥️host` crate (`semio-framework-plugin-host`): `GuestRelayPublication`, `GuestColdRelayJob`, `GuestRelayLifecycleProbeJob`, relay sessions use `StepOutcome`, `WorkerJobSession::try_new`, `take_outcome`. Not touched yet; follows after the lib is green.
- `settle_framework_reserved_admission` bypasses the reactor job registry (see above); a registry-driven law needs a real host context.

## Result lines (private dir `play-fleet/pl-b`)

- `cargo check -p semio-framework-plugin --lib` native: Finished, 0 errors (11:2x, after pl-a/c/d landed their classes).
- `... --target wasm32-wasip2 --lib`: Finished, 0 errors. `... --target wasm32-wasip2 --features component-guest`: Finished, 0 errors.
- `... --features artifact-app-testing` (native): 35 errors, all in the testing harness (`with_authoring_identity` missing, `identity` unbound at lib `8249`, `RetirementDemand`/`ValueError` unimported at `9357/9362`, arity drift in `ingest_operations`, `tick_backbone`, `poll_document_archive_load`, `preview_addressed_action`; `🧪️tests/🧪️history-edit-acceptance`, `🧪️tests/🧪️node-drag-history`). Not job-session class; unowned.
- `-p semio-framework-plugin-describe --lib` / `-p semio-framework-plugin-host --lib`: host fails with 28 errors (describe depends on host). `-p semio-framework-os`: not reached.
- Public API added for plugins: `reactor::jobs::{admit_original_job, original_job_admission_demands}` (see corrections).

## plugin-host first errors (28)

Job class (pl-b, not yet ported): `GuestColdRelayJob` / `GuestRelayLifecycleProbeJob` still `step -> StepOutcome` (`🖥️host/🦀️.rs` 4880, 5077) over `GuestRelayPublication`; mounted relay registry uses removed `WorkerJobSession::try_new`, `try_step_on_caller()` without grant, `WorkerJobOutcome::take_outcome` (5559-5598, 5736, 5980); `⚡️effects` 632/641 `ComputePool::run_job` has the 10-argument admission form. Note: the registry never assigns `session.terminal` / `DrainingForCaller` anywhere, so the terminal handling of a mounted relay has to be designed, not just ported.
Other classes: `shard::IssuedShardTurn` not `Copy` (7 sites in `🧵️shard`), `TurnResult.retained_receipt` missing (`🧵️shard` 2134), `kernel::Budget.retained` missing (`🧬️component-codec` 13), `⏳️runtime` 426 result nesting.

## Parked state (pl-b, usage limit)

All edited files parse and no edit is half done.

### Done since the first report
- `settle_framework_reserved_admission` (veto honoured): now `drive_framework_reserved_registry_job` in plugin `🦀️.rs`; builds a real `OriginalJobAdmission` + `StepContext` (grant = `mounted_policy.preparation`, widened by `job_admission_demands` quotes), `start_job` then `step_job` with real `IoRunControl`/`SqliteSnapshotControl` until `JobStep::Done|Failed`. `framework_reserved_job_output` was deleted.
- plugin-host job class (`🖥️host/🦀️.rs`, `⚡️effects/🦀️.rs`), native `cargo check -p semio-framework-plugin-host --lib`: Finished, 0 errors (after pl-c's parts landed). wasm32 is not applicable: the host depends on wasmtime (`wasmtime-internal-jit-debug` build script fails on wasm32), so there is no wasm target for host/describe.
  - `GuestRelayPublication`: `RetainedPayloadBuilder` + `delivered`; `step` -> `Result<Option<JobOutcomeBorrow>>` (bind, append, retire source, seal, lend preview/complete/fault); `borrow_outcome`; `retire` closes a delivered preview inside the next step; a commit/fault publication stays until the job closes.
  - `GuestColdRelayJob` / `GuestRelayLifecycleProbeJob` on the new trait (`admit_yield`/`admit_cancelled`, `borrow_outcome`); probe job owns an optional publication and closes it first.
  - Mounted registry: `admit_guest_relay_session` (`try_admit_owned` under `drive_policy`); `GuestRelayRefusedSource<J>` replaces the dead `WorkerJobSessionAdmissionRejected`; `step_guest_relay_owner` (`try_step_on_caller(grant)`, `take_outcome`/`take_terminal`, receipt taken once and charged to the wake receiver in the same turn with the checked-out context grant); `pump_outcome` (stages Reading -> Acknowledging; pages copied into `output`; descriptor acknowledged by paid turns).
  - Terminal design (decision): a checked-out outcome is read page by page, acknowledged, then a continuing one (`Yield`/`Preview`/`Checkpoint`) is `resume`d (retried while resume refuses), while `Complete`/`Cancelled`/`Fault` set `session.terminal`, switch the lifecycle to `DrainingForCaller` and `begin_close` the checked-out authority; the existing `pump_close` ladder then closes the session and the caller receives output/error. Before this nothing in the host ever assigned `session.terminal`.
  - `run_router_effect_job` (`⚡️effects`): `ComputePool::prepare_job_params` + `run_job` with `RouterEffectWallet` (a `ComputeRetainedRecipient`) and a `WorkerJobAdmissionContext`; completion closed with `ComputeJobCompletion::close_step`; refused sources closed by `close_unadmitted_router_effect`.

### Last result lines (11:5x-12:xx)
- plugin lib wasm32-wasip2: Finished, 0 errors. plugin-host native: Finished, 0 errors.
- plugin lib native at the last run: 1 error, `🦀️.rs:18728:307` `expected Option<Arc<[u8]>>, found Option<Arc<Vec<u8>>>` (not a job-class site; appeared after other executors' edits; owner unknown, probably pl-d).
- `plugin-describe` native: 3 errors `Budget.retained` missing at `🛂️descriptor-emission` 427/493/607 (pl-c's `kernel::Budget.retained` class).
- `semio-framework-os --lib`: errors only `ArtifactCanonicalJsonTree` missing for `MediaType`, `MediaWireFormat`, `MediaForm`, `MediaPortSpec`, `PortMultiplicity` (framework media types; pack-json/os-domains class, not job).
- Not run after the park request: nothing pending; no cargo is running.

### Next steps
1. Re-check `cargo check -p semio-framework-plugin --lib` native (the 18728 error should be gone once its owner lands), wasm32-wasip2, `--features component-guest`.
2. Re-check `-p semio-framework-plugin-host`, `-p semio-framework-plugin-describe` (after `Budget.retained`), `-p semio-framework-os` (after the media canonical-tree impls).
3. When all pass, write `GREEN os <HH:MM> plugin plugin-host plugin-describe os` to `🗑️generated/coord/os.status` and tell main (host/describe/os are native only).
4. Open debt: `artifact-app-testing` feature harness (35 errors), cfg(test) sources on the old job API, host `GuestRelayRefusedSource` duplicates the plugin-side `RefusedWorkerSource` (the job crate should own one), a never-started cold relay job cannot reach terminal-empty (pre-existing: its `terminal_is_empty` is false and close refuses without a publication).

## Parked state 2 (pl-b): extension export macros

No edit in flight; plugin lib compiles.

### Done
- Release blocker fixed: the stale job glue of `__semio_owned_core_exports!` (`semio_owned_start_job_v1`, `semio_owned_step_job_v1`) and of the WIT `jobs::Guest` impl inside `__semio_actor_exports!` (`start_job`, `step_job`) now calls new guest twins `reactor::jobs::abi::{start, step}` (`⚛️reactor/💼️jobs/🦀️.rs`, region `GuestAbi`).
- Design: the guest funds every job call from the embedding's mounted policy (`PluginRuntime::mounted_owner_policy().preparation`, widened per step by the registry's own `job_admission_demands` quote). `start` builds a real `OriginalJobAdmission` and calls `start_job` (a denied admission is a typed `job.start` fault); `step` builds a `StepContext` (fuel from the host budget, deadline = now + `deadline_ms`), `IoRunControl`/`SqliteSnapshotControl` with always-allow progress callbacks, and calls `step_job`. Operation id = job id, generation constant 0, so start and step see one identity. Faults leave as encoded fault bytes (owned ABI) or `plugin_error` (WIT). `cancel_job` was already current.
- Limitation (decision): the io-run/io-sniff progress callbacks inside the guest do not forward to the host's native allocation port; a host-forwarded variant needs the host to pass a grant in `start-job`/`step-job`.

### Result lines
- `cargo check -p semio-framework-plugin --lib` native: Finished; `--target wasm32-wasip2`: Finished.
- `cargo check --target wasm32-wasip2 --lib --manifest-path ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/Cargo.toml`: Finished, 0 errors (was 10).
- Same for `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/Cargo.toml`: Finished, 0 errors.

### Next steps
1. cfg(test) leftovers: `drive_framework_reserved_spawn_like_host` (plugin `🦀️.rs` ~34691) still calls the old `start_job`/`step_job`; port it onto `reactor::jobs::abi` when tests are revived.
2. Earlier open items (GREEN os gate: plugin-describe `Budget.retained`, os media canonical trees, `artifact-app-testing` harness) remain with their owners; re-run host/describe/os checks after they land, then write `GREEN os` to `os.status`.
