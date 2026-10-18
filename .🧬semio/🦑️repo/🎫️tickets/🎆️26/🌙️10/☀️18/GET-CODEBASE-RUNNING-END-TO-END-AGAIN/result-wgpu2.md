# Result: WGPU2 (finish the port off the removed job-worker API)

Static only. Nothing was compiled or run (no builds started; `node_modules` is not mounted for vitest). Every edited Rust file parses with `~/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rustfmt --edition 2021 --config skip_children=true --emit stdout` (the `rustfmt` rustup shim hangs on `rust-toolchain.toml`; use the toolchain binary). Edited `.ts` parse with `bun build --no-bundle`, edited `.json` with `json.load`.

`E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `W` = `E/🎯️targets/🧊️wgpu`.

## 1. Sweep (all Rust, framework + `✏️s` + other roots; evidence in `C/🗑️generated/wgpu2/sweep1.txt`, `sweep-ts.txt`)

Removed names: `{WorkerJobSession,BatchJobSession,MountedWorkerJobSession}::try_new`, `try_submit_step`/`pump_one` without grant, `session.close_step(a, b)`, `take_checked_out_outcome`, `WorkerJobSessionAdmissionRejected` as an admission result, `session.step()` / `take_outcome()` without ticket. Callers found and handled:

| Caller | State |
|---|---|
| `W/📐️surface-lane` | ported (section 2.1) |
| `W/🏛️host-base/📥️lifted/🔁️asset-decode-jobs` (`submit_renderer_io`, `RendererIoNode`) | ported (2.2) |
| `W/🧵️frame-job` legacy path | removed, handle ported (2.3) |
| `E/🧪️tests/🔬️wgpu-frame-job-unit`, `…/🔬️wgpu-surface-lane-unit`, `…/🔬️wgpu-renderer-async-boundary` (saturation law), `…/🔬️wgpu-renderer-renderer-io-retained` | ported (2.4) |
| layout export test oracle `run_layout_export_headless_batch` (`✏️s/🔌️plugins/📏️layout/…/⚙️engine/📤️export/⏭️advance-commit-and-codec`) | ported (2.5) |
| `E/…/board-host` size assertions naming `WorkerJobSessionAdmissionRejected` | the two assertions removed |
| layout export test harness (`🧪️tests/🔬️unit/🎟️harness-pdf-and-dispatch-laws`, `🚧️caps-json-checkpoint-pdf-style`) | NOT ported, see section 4 |

No production caller of a removed worker-session function remains anywhere in the repo (second sweep for 2-arg `try_submit_step`/`pump_one`, `session.step()`, `.take_outcome()`, `try_step_on_caller()` found only the layout test harness).

## 2. What changed

### 2.1 Surface lane (`W/📐️surface-lane/🦀️.rs`, crate `host-plumbing`)
- `SurfaceResizeJob` now implements the current `InteractiveJob` (`step -> Result<Option<JobOutcomeBorrow>, OriginalTurnFailure>`, `borrow_outcome`, paid `close_step(cx)` with the four demand functions, `terminal_is_empty`). One scalar per step as before; yield / complete / cancelled outcomes.
- New `SurfaceResizeSession` (job, params, cancel-alias retirement, `MountedWorkerJobSession`, receipt/ack flags). Admission: `admit_original_cancel_alias` + `BatchJobParams` from the initiating context, then `MountedWorkerJobSession::try_admit_owned(&mut job, &mut params, cx)`. A refused admission (`Ok(None)`) keeps job and parameters with the lane and retries next turn (no `Rejected` session variant any more). Outcome custody: pump, take receipt (`consume_retained`), paid acknowledgement, paid hand-back, then resume or terminal. Close ladder: pump pending outcomes, take any pending receipt, `begin_close`, return the cancel alias against the context root (`has_original_cancel_alias_witness` / `return_original_cancel_alias_step`), `close_step(cx)`, pay the terminal-empty item.
- `MountedSurfaceResizeLane::{drive_one, close_step, close_abandoned_step}` and `SurfaceResizeAuthority::{drive_one, close_step}` take `&mut StepContext` and return `Result<_, OriginalTurnFailure>`.
- Callers: `W/🪟️winit-app` `prepare_redraw(cx)` (both `redraw_on_original_caller` and `redraw_offscreen_worker` pass their context), `W/🏠️os-host` retirement ladder `close_abandoned_step(cx)?` / `surface_resize.close_step(cx)?`.
- Test `E/🧪️tests/🔬️wgpu-surface-lane-unit`: job law on the new trait, sealed-candidate close law, lane laws driven by `OriginalJobFixtureCaller`, and a new end-to-end law (`a_resize_request_prepares_through_one_admitted_session_and_closes_every_owner`: 1280x720 at scale 2 gives 640x360, then every owner closes and the caller retires).

### 2.2 Renderer I/O transport (`W/🏛️host-base/📥️lifted/🔁️asset-decode-jobs/🦀️.rs`)
The transport is pumped without a frame owner, so each request now carries its own original authority:
- `RendererIoCaller`: own operation id, generation = slot generation, root cancel token, sequence; `turn` builds a `StepContext` (maintenance budget, `RENDERER_IO_TURN_GRANT` 16 items / 4 MiB / 4 MiB / 16 MiB / depth 64) under that root; `retire_step` retires the root in paid turns after the alias has been returned.
- `RendererIoNode` = unadmitted job + params + alias retirement + `WorkerJobSession` + ticket + result. `submit_renderer_io` admits through `try_admit_owned` (alias from the request's own context); a refused admission gives the same `Err("native I/O retained session registry is full")` result as before and the unadmitted job is retired by the pump (`pump_one` -> `close_unadmitted_step`). The 257th request keeps the exact request in `node.job` (testkit-visible field replaces `node.rejected`).
- Pump: submit with the turn grant (`try_submit_step(pool, Lane::Io, grant)`), `take_outcome`/`take_terminal`, receipt consumed into the turn, paid acknowledgement (4 attempts), resume on yield, result from `NativeIoJob::take_result` on terminal. Detached/finished requests close through `close_session_step` (takes a pending receipt first, then `begin_close`, alias return, `close_step(cx)`). `close_detached_result` uses `NativeIoValue::close_step(grant)`.
- New `close_renderer_io_payload(&mut RetainedJobPayload) -> bool` (bounded paid walk to terminal empty). It replaces the stale payload `close_step(1, JOB_PAYLOAD_PAGE_BYTES)` in `native-asset-stream` (`push_renderer_asset_page`), `scale-bench` (2x), `kernel-pool` (1x); `mailbox-impl` `pump_native_asset_handoff_step` takes one explicit grant per pump.

### 2.3 Frame job (`W/🧵️frame-job/🦀️.rs`)
- Deleted the dead legacy path: both `poll_runtime_and_resubmit` (native + wasm), `admit_active`, `receive_frame_outcome`, `batch_params`/`now_us`, `FrameCompletionWake`, the `rejected`, `cancel`, `last_submitted_generation`, `retained`, `retained_progress` fields and the legacy `close_step`. Nothing in production called them (winit-app / browser worker use `poll_runtime_on_original_caller` + `close_original_frame_step`).
- `FrameBuildHandle::new()` has no argument (plus `Default`); `retire_for_component_surface_close_step(cx) -> Result<bool, _>` now shares the paid ladder `retire_original_frame_step(cx)` with `close_original_frame_step` (which sets `closing`, then also clears the completion waker, see the open item on the old native retirement hang). `W/🏠️os-host` `advance_component_surface_close(cx)`; call in `winit-app::pump_frame_inputs`.
- Doc comments that named `poll_runtime_and_resubmit` now name `poll_runtime_on_original_caller` (frame-job, winit-app, presentation-raster).
- Tests `E/🧪️tests/🔬️wgpu-frame-job-unit`: `compute`, the two retirement laws, the five candidate-return laws and the component-close law run on `OriginalJobFixtureCaller` (`frame_caller`, `retire_phase`, `close_active_frame`); the component-close law now presets `handle.original_job`, retires it transiently, re-admits the same generation through `poll_runtime_on_original_caller`, and finally closes the handle.
- Text laws repointed to the real original-caller owner: `E/🧪️tests/⏱️wgpu-worker-step-budget` (slice `poll_runtime_on_original_caller`, needle `try_step_on_original_caller`), `E/🧪️tests/♻️frame-close` (`pub fn new() -> Self`, `site:"os_renderer_frame_deadlines"`, `try_admit_owned`, no `receive_frame_outcome`), fixture `🖌️wgpu-document-owner-move` `frameWorkerTurn` = `session.try_step_on_original_caller(cx)` (the old needle lived in the deleted wasm path), `🔢️frame-generation-hold` prose, async-boundary text anchors (`poll_runtime_on_original_caller`, `advance_component_surface_close(cx)?`, `retire_for_component_surface_close_step` slice).

### 2.4 Other tests
- `E/🧪️tests/🔬️wgpu-renderer-async-boundary`: saturation law renamed `an_independent_decoder_job_recovers_its_response_from_an_unadmitted_session` (still child-process isolated): 256 `Occupied` sessions via `try_admit_owned`, maximum plus one stays unadmitted with job + params, response recovered, both owners retired in paid turns, then every occupied session closed; the cancellation law uses the current job trait; the payload page close in `native_asset_response…` uses an explicit grant.
- `E/🧪️tests/🔬️wgpu-renderer-renderer-io-retained`: `node.job` instead of `node.rejected`; three `bytes.close_step(1, …)` loops replaced by `close_step(RESULT_PAGE_CLOSE_GRANT)` until terminal empty.

### 2.5 Layout export oracle
`run_layout_export_headless_batch` (`#[cfg(test)]`, `…/📤️export/⏭️advance-commit-and-codec/🦀️.rs`) drives `BatchJobSession::try_admit_owned` + `step_on_original_caller` with `OriginalJobFixtureCaller`, reads the typed outcome view, computes the commit before the close ladder drains the shared chunks, and retires session and caller.

### 2.6 Cargo / lock
`W/🦴️asset-decode/📦️packages/🦀️rust/Cargo.toml`: dropped `ui_contract`. Root `Cargo.lock`: removed the single line `"semio-framework-ui-contract",` from the `semio-framework-os-renderer-wgpu-asset-decode` dependency list (verified by diff: that is the only line I changed; the neighbouring `semio-framework-2d-compute` removal in the diff is a peer's). No other lock changes (no dependency added anywhere). `ui_contract` remains reachable through `semio-framework-ui`.

### 2.7 Interactivity-audit laws
`E…/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🖥️mounted/🎞️frame-transaction/🟦️.ts`: the frame-job block no longer requires `WorkerJobSession::try_new`; it requires `WorkerJobSession::try_admit_owned(&mut self.original_job,&mut self.original_params,cx)`, `try_step_on_original_caller(cx)`, one `take_terminal` and one `take_outcome(ticket)`, `take_retained_step_receipt`, `acknowledge_outcome`, the cancel-alias witness/return, `close_step(cx)`, and forbids `try_new`, `WorkerJobSessionAdmissionRejected`, `close_step(self.retained)`, `take_rejected` and the legacy poll function. `…/🖥️mounted/🟦️.ts` (P5c): the ui-engine scheduler law now requires `try_admit_owned(&mut window.layout_job, &mut window.layout_admission.params, cx)` and `pump_one(pool, worker_lane(lane), cx.retained_grant())` and forbids `MountedWorkerJobSession::try_new` (it contradicted `targets-wgpu-mounted-layout-unit`). Every new needle was checked against the current source text.

## 3. Crates to compile and tests to run (coordinator)

`bun repo cargo --store=shared --configuration=prototype -- check -p <pkg>` (native) and the same with `--target wasm32-unknown-unknown` for the starred ones:
- `semio-framework-os-renderer-wgpu-asset-decode` (*) (manifest/lock only; confirms the dropped dependency)
- `semio-framework-os-renderer-wgpu-host-base` (*) (the lifted I/O transport is native-only, wasm must still check)
- `semio-framework-os-renderer-wgpu-host-plumbing` (*) (surface lane)
- `semio-framework-os-renderer-wgpu-frame-job` (*)
- `semio-framework-os-renderer-wgpu-host-winit` (*) (os-host + winit-app)
- `semio-framework-os-renderer-wgpu-browser-worker` (wasm only; unchanged source, depends on the changed `FrameBuildHandle` and `prepare_redraw`)
- `semio-framework-os-renderer-wgpu-runtime-mailbox` (*) (`mailbox-impl`, `native-asset-stream`)
- `semio-framework-os-renderer-wgpu-kernel-pool`, `semio-framework-os-renderer-wgpu-kernel-runtime` (native; `scale-bench` lives in kernel-runtime)

Tests (native): `-- test -p semio-framework-os-renderer-wgpu-host-plumbing surface_lane` (6 laws), `-p semio-framework-os-renderer-wgpu-frame-job` (the unit file, all), `-p semio-framework-os-renderer-wgpu-runtime-mailbox-tests an_independent_decoder_job` (4 laws), the renderer-io retained laws (`mounted_registry_max_plus_one_zero_pump_drop_and_generation_are_exact`, `headless_entrypoint_drives_retained_io_and_matches_the_json_oracle`, `native_app_worker_completes_io_without_a_presentation_host`, `native_kernel_worker_completes_io_without_a_presentation_host`; test host `🧊️renderer/📥️mounts/🌱️wp1-foundation/🧪️test-hosts/🔬️renderer-io`; I could not statically resolve which crate mounts it), and `-p semio-framework-layout-…-export` lib tests only after the harness port (section 4). Vitest: `⏱️wgpu-worker-step-budget`, `♻️frame-close`, `🖌️wgpu-document-owner-move`, `🔬️interactivity-mounted-frame-transaction`, `🔬️targets-wgpu-mounted-layout-unit`.

Things a compiler may reject that I could not prove statically (look here first): the `let Some(x) = self.field.as_mut() else { … self.field … }` borrow shapes in `surface_lane::SurfaceResizeSession::drive`; closure return-type annotations in `RendererIoNode::{admit,close_session_step,receive_outcome,submit_step}`; `WorkerJobSession::poll` used as a path in `map(...)`; `JobOutcomeView::Fault { detail, .. }` in the layout oracle.

## 4. Open (not done, with reason)

- Layout export test harness (`…/📤️export/🧪️tests/🔬️unit/🎟️harness-pdf-and-dispatch-laws/🦀️.rs`, `…/🚧️caps-json-checkpoint-pdf-style/🦀️.rs`): `drive_test_job_then` still calls `BatchJobSession::try_new`, `session.step()`, `take_outcome()`, `outcome.close_step(1, …)` and builds `BatchJobParams` with the removed `work_grant` field; its ~12 callers match on the old owned `StepOutcome` (`job.step(...) == StepOutcome::Yield`, `checkpoint.state`, `StepOutcome::Complete(candidate)`), `LayoutExportPublication::close_step(0, 0) -> JobPayloadCloseStep`, `InteractiveJob::close_step(&mut job, 1, PAGE)`. The production `LayoutExportJob` is already on the new trait, so this is a whole-suite port to the borrowed-outcome model (owner: layout export), not a one-function fix. Until then that crate's lib tests do not compile.
- Other tests in `🔬️wgpu-renderer-async-boundary` still on old non-worker APIs (not touched): `NativeReferenceDecodeJob::close_step(1, …)` is its own (valid) API; `close_step(1, 0)` expectations at ~1399/1475/1484 belong to it. Large `contains(…)` text laws there (`admit_next_frame`, `build_and_publish_snapshot(&mut self)`, `observe_presentation_input_generation(build_generation.0)`) were already stale before this task.
- Stale tool-job laws that still require or build the removed constructor (other owners, not touched): `🔌️plugin/🧪️tests/🔬️tool-job-*` and `⚖️laws/🎯️tool-job-coverage/*` (`MountedWorkerJobSession::try_new(dispatch.job, params)`, `WorkerJobSession::try_new(job, params)` as required needles), `🛎️services/🧪️tests/🧮️compute-retained` (it already forbids `try_new`, correct), `🧵️job/🔭️preview/🎟️drive/🧪️tests` (mention in prose), `✏️s/…/cad-presence-retirement` (`MountedWorkerJobSession::try_new(job, params)` needle). Their Rust targets no longer contain `try_new`, so each law fails until repointed.
- `WorkerJobSessionAdmissionRejected` (worker crate) has no producer any more (only `board-host` named it, now removed). The worker owner can delete the type; the worker crate is being split by a peer right now (`worker/🦀️.rs` shrank to 6 KB during this task), I did not touch it.
- Pre-existing issue found, fixed incidentally: native `FrameBuildHandle::terminal_is_empty` required `completion_waker.is_none()` but nothing cleared the waker on the original-caller close path, so `OsHostRetirementState::close_step` could never finish natively. `close_original_frame_step` now clears it after the ladder. The waker itself (`set_completion_waker`, `HostUserEvent::FrameReady`) is now never invoked, since frames run on the caller; removal would touch winit-app events and the `frame_ready` law, left for the owner.
- `UI_WORKER_RETIREMENT_POLICY` in `ui_contract` is still used by the ui crate; only asset-decode dropped the dependency.
- Throughput note carried over from `result-wgpu.md`: `ASSET_DECODE_TURN_GRANT` / the new `RENDERER_IO_TURN_GRANT` are first estimates; raise them if a run shows starved steps.

Scratch kept: `C/🗑️generated/wgpu2/sweep1.txt`, `sweep-ts.txt` (grep evidence, delete with the coordinator's folder).
