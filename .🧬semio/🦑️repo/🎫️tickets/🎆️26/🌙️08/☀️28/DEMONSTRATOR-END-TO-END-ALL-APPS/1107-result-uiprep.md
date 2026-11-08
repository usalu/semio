STATUS: GREEN lib check, native (uiprep-9 rc 0) and wasm32-wasip2 (uiprep-10 rc 0): semio-ui-prepared-staging, semio-ui-prepared, semio-ui-draw-contract, semio-ui-runtime-presentation, semio-ui-runtime-reconcile-primitives, semio-framework-kernel-ui. GREEN native earlier: semio-framework-job lib (uiprep-3), semio-ui-scene-views (uiprep-5). UNVERIFIED: tests (--tests), the new law, frame-build/app-runtime/frame-presentation callers (not in this run). uiprep-1 (native, `-p semio-ui-prepared-staging -p semio-ui-prepared`) stopped upstream in `semio-framework-job` lib (11 errors: `NativeConstructionScope`, `outcome/return/bytes`, `field/float`; log `C/🗑️generated/1105b/uiprep-1.txt`). No error names a uiprep file yet.

## Effect source (runtime2 rule W1)
- Job turn: the whole ui-prepared close chain runs in a job/frame turn, so the effect comes from the turn: `semio_framework_job::close_original_effect_step` (single debit `receive_original_retained_work`), wrapped once as `prepared_close_turn(cx, quote, |effect| ..)`.
- `PreparedRenderJob::close_step(cx)` (job-completion) calls it. The abandoned-owner closers run inside `frame_before_input_step(.., context: &mut StepContext)` (app-runtime), i.e. also a turn: no wallet there.
- No turn: only the presenter retire step (`retirement-and-present-steps`, render thread, takes `gpu`) and fixtures: `prepared_close_authored(grant, ..)` = `AuthoringWallet::new(grant, 64)`, stock is the grant itself.

## Shape (staging `♻️retirement`)
- `PreparedClose = Result<CloseStep, RetirementPollExit>`; `prepared_close_gate/progress/child` generic over the error (ValueError stays for grant-based leaves); `prepared_close_poll` forwards the child `OriginalRetirementPoll` (lease), `prepared_close_receipt` settles with `RetirementPollExit::settle` (a progressing turn with no recorded work pays exactly one unit) and keeps `Blocked`/`Complete` in side flags because `close_original_effect_step` returns Pending only.
- Only the SceneRaster leaf needed an effect (`SceneRasterLease::close_step(effect)`); atlas pages, raster pages/producer, fixed lists, command pages, draw lists (`retire_granted`), fault payload (`close_step_granted`) stay grant-based and are called with `effect.grant()`; the fault payload's reported work is paid into the effect (`RetirementPollExit::pay`).
- Chain ported to `close_granted/close_step(effect)->PreparedClose`: `PreparedRenderUpload` (staging render-upload, the wgpu caller of the brief), `PreparedRenderPacket`, `PreparedRenderInput`, `PreparedRenderReceiver`, `PreparedRenderJob`, `PreparedRenderJobRejected`, and `prepared_abandoned_close` plus the four `close_abandoned_step(effect)`. `InteractiveJob::close_step(cx)` of the job no longer does its own `consume_retained` + `consume_fuel(1)`.

## Callers ported
- `renderer/app-runtime` abandoned phases (input, job, receiver, packet) -> `frame_prepared_effect_abandonment_complete(cx, next_abandoned_close_demands, close_abandoned_step)` (new, frame-build). Atlas abandonment stays grant-based.
- `frame-build`: `icon_upload`, `upload_rejected`, `resource_input` closes -> `frame_prepared_close`. `frame-preparation` `AppFramePresentation::close_step(effect)`. `frame-presentation` retire step -> `prepared_close_authored` (the old `matches!(frame.close_step(..), Complete)` did not type-check).
- Tests: `🧪️testing` helpers and `physical-job-close/prepared` and world host `reference-and-pick-lane-laws` use `prepared_close_authored`. New law `scene_raster_upload_closes_under_one_authored_effect_per_step` in `staging/🧪️tests` (Rust only; no language-agnostic fixture or third-party oracle exists for the lease ladder).

## Not done / risks
- Nothing compiled. Inference risk: the generic `prepared_close_*` helpers rely on the surrounding return type.
- `frame_original_primitive_close` (frame-build, other owners) still calls `close_original_retained_step` with a `Result` closure, which does not match the job's `RetainedWorkOutcome` signature: not uiprep's, left.
- Stale `SceneRasterLease::close_step(grant)` use in `🔬️targets-wgpu-raster-ownership-unit` (port-c/raster-owner tests) not touched.

## Rerun
`C/1105b-tools/chk.sh uiprep-<n> 🧰️framework/Cargo.toml native -p semio-ui-prepared-staging -p semio-ui-prepared`, then the same with `wasm` (wasm32-wasip2), then `renderer/app-runtime` crates.

## Update: job lib ownership
Coordinator told uiprep to claim the 11 job-lib errors unless runtime2 had taken them. runtime2 replied that jobsplit (acdff90e240d22566) owns `admit_original_construction` and its two stale callers (`📮️return/📦️bytes`, `📥️field/🔢️float`) with the `with_original_effect` design. Claim released in `1105b-claims.md`; no job edits made. Next: wait for the jobsplit FIXED line, then `uiprep-2` native and wasm.

## Update 2 (11/07 ~11:00)
- FIXED-verified native, no wasm run: `semio-framework-job` lib (uiprep-3 rc 0). Only `📮️return/📦️bytes` was mine: `OriginalContiguousJobBytes` now owns an `OriginalConstructionAdmission` plus candidate and failure custody and calls the 6-arg `admit_original_construction`; a failed construction moves its causes into `failure`, which close retires with a paid step. jobsplit's construction/float edits untouched.
- FIXED-verified native: `semio-ui-scene-views` (uiprep-5 rc 0): `✂️text-splice` `DraftChange`, `DraftChangesOwner`, `DraftChangesJsonCursor` derive `RetireOwned` (manual impls removed).
- uiprep-4 (`-p semio-ui-prepared-staging -p semio-ui-prepared`) now stops in `semio-ui-contract-values` (187 errors, `C/🗑️generated/1105b/uiprep-4.txt`): `🔢️fixed-list`, `🗺️fixed-map-and-bytes/🛬️receive` on the old `RetireOwned::retirement` / `NativeReceiveMap` / `NativeDefault` / recovery API. Nobody claims it; it is the next frontier for my proof.

## Update 3: semio-ui-contract-values (claimed, assessed, released unedited)
187 errors in uiprep-4 (`🛬️receive` 84, `🗺️fixed-map-and-bytes/🛬️receive` 28, `🔢️fixed-list` 22, `🌱️original` 22, `original/🔀️merge` 19, `original/📥️arguments` 13, `🛬️receive/🏟️collections` 6, `collection-arena/🎟️source` 2, `♻️retirement/🌳️typed` 1). One cascade, not mechanical:
- PagedList `reserve_one`/`release_empty_page` and `RetainedCloneClose::begin_granted` take an effect, so `UiFixedList::try_reserve_one(bytes)`, `try_reserve_one_funded(grant)`, `release_empty_page(bytes)` and the UiFixedMap/arena callers must move to effects (and the cold-push page-exact rule from memory must survive).
- `RetireOwned` (`RetirementCursor` + `retirement_seed`), `NativeReceiveMap` (`from_native` only), `NativeDefaultValue` (`default_work_demand`, `default_allocates`, 3-arg `admit_default`).
- `UiValueNativeReceiver` is a hand-written recursive receiver on the removed recovery API (`has_pending_recovery`, `NativeClosePermit`, `NativeRecoveryLoan`) and needs the full select_advance / advance_nested / advance_local / take(effect, reply) rewrite.
Users of `UiFixedList`/`UiFixedMap`/`UiValue` outside this crate (action, scene, plugins) will follow. It needs its own lane; no edits were made, so nothing is half-done. uiprep's ui-prepared proof waits on it.

## Update 4
semio-ui-contract-values is now green (peer lane, W-uivalues-5/6). uiprep-6 (native, `-p semio-ui-prepared-staging -p semio-ui-prepared`) stops one crate earlier in `semio-framework-value-native-decoding` lib, 2 errors in `🛬️decode/🛂️allocation/🦀️.rs:25` (`Option<ValueError>::or(Option<NativeAllocationRefusal>)`), a file a peer edited at 11:39 (log `uiprep-6.txt`). Not mine; not touched. Rerun: `chk.sh uiprep-7 🧰️framework/Cargo.toml native -p semio-ui-prepared-staging -p semio-ui-prepared`, then wasm. Known remaining callers on the old byte-grant UiFixedList forms (present resources, wgpu mounted_layout, builder original retirement) are the contract-values lane's note, not yet ported.

## Update 5 (exclusive: framework-kernel-ui, ui-draw-contract, ui-runtime-presentation)
All EDITED, UNVERIFIED (uiprep-7 stopped upstream in `job-recorded-clock`, peer-claimed; log `uiprep-7.txt`).
- `semio-ui-runtime-presentation` (`🎭️present`): `validate_source` maps the lease error into `OriginalTurnFailure`; `close_native_failure_place` runs through `close_original_effect_step`; the float-free work counter is u128.
- `semio-framework-kernel-ui`: `♻️retirement` `OriginalPatches` closes on the effect (`record_retirement_work` pays each child step, `next_computational_work` 1) and `UiTurnPatches` uses `retirement_seed`; `🛬️receive` `NativeSequenceCollection` gains `Reservation=()`, work demands, the frame-taking `admit_storage` (via `receive_allocation`), `reserve_step` on `prepare_work`/`commit_work`, 5-arg `push_step`; `NativeDefaultValue` gains `default_work_demand`/`default_allocates`/3-arg `admit_default`. Modelled on `UiPatchOps`.
- `semio-ui-draw-contract`: the grant-priced draw-list paths keep their byte grants and use PagedVec's cold/leased forms (`reserve_one_cold`, `reserve_exact_capacity_one_cold`, `release_empty_page_leased`); `RetainedOutputGrant` imported in the crate root. Not audited for the `.expect`/`?` sites flagged at shared-border-finalize:130 and shared-border-clip:152 (those call `release_empty_page_leased`, a `Result`, so they should now type-check).

## Update 6 (proof)
uiprep-9 (native) and uiprep-10 (wasm32-wasip2), `-p semio-framework-kernel-ui -p semio-ui-runtime-presentation -p semio-ui-runtime-reconcile-primitives -p semio-ui-draw-contract -p semio-ui-prepared-staging -p semio-ui-prepared`, rc 0, zero `error` lines (logs in `C/🗑️generated/1105b/`). Last fixes: `OriginalPatches` made `pub` (E0446), reconcile-primitives `records` uses `release_empty_page_leased`.
Not run: `--tests` targets (testing helpers, physical-job-close, world host law, the new scene-raster law), and the renderer callers (`app-runtime`, `frame-build`, `frame-preparation`, `frame-presentation`), which sit above this set.
