# Result: WGPU-LAW-FIXES (prefix `wgpu`)

Static only. Nothing was compiled or run: `node_modules` is a dangling symlink into the unmounted dependencies image (vitest and bun module-tree imports fail), and cargo is the coordinator's. Every edited Rust file passes `rustfmt --edition 2021 --config skip_children=true --emit stdout` (parse). I re-implemented the vitest needles of `wgpu-document-owner-move` and `frame-latency` as text checks (all present or absent as the tests require, see "Verification").

## 1. Findings that change the picture

- The asset session, `MountedWorkerJobSession::try_new` (surface-lane), `submit_renderer_io` (host-base lifted) and the legacy part of `FrameBuildHandle` still call a job-worker API that no longer exists (`WorkerJobSession::try_new`, `try_submit_step` without grant, `try_step_on_worker()` without grant, `close_step(grant)`). `fn try_new` is gone from `🔨️modules/🧵️job/👷️worker`. So the asset session was not "one line away from the law": it did not match the job API at all. It is now ported (item 2). The others are listed in section 5.
- `RendererAssetDecodeInvocation` never existed in any commit reachable by `git log -S` on the wgpu tree (the pickaxe over the whole repo is too slow to finish); the law fixture describes a design that has to be built, not restored.

## 2. What changed

All under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` (`E`), tree `🎯️targets/🧊️wgpu` (`W`).

1. frame-latency (`W/🪟️winit-app/🦀️.rs`): `let _latency=…FrameLatencyTimer::start(…renderer_frame(generation.0),…FrameLatencyStage::SnapshotPublish,1)` is the first statement of the `AppPresentStep::Complete` arm of `publish_presented_snapshot_on_original_caller` (original line found with `git show 0c3e78944af`, commit 685 tree; commit 686 dropped it). Qualified through `semio_framework_os_renderer_wgpu_host_base::frame_latency` like the browser worker.
2. Present budget (same file): `present_step` now runs inside `cx.with_original_work_loan(fuel, now + semio_framework_job::INTERACTIVE_STEP_CEILING_US / 2, …)`; the pre-carve code had the same half-ceiling share as a private deadline. This satisfies the unchanged fixture anchor and is the real mechanism (loan restores the caller's deadline and charges the spent fuel).
3. Asset decode session, original authority (`W/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs`, crate `asset-decode`):
   - new `RendererAssetDecodeInvocation::admit(cx)`: the owner's operation, generation, cancellation alias (`cx.admit_original_cancel_alias`), clock and per-step budget as `BatchJobParams`.
   - `RendererAssetDecodeSession::new(probe, wake, invocation)` no longer creates an operation id, a root cancel token, a `WorkerJobSession` or a retirement policy. Admission is `WorkerJobSession::try_admit_owned(&mut job, &mut params, cx)` in `pump_one(cx)`; the unadmitted job/params/alias retire in paid turns (`CancelTokenRetirement`) when the decode is cancelled before admission.
   - `pump_one(cx) -> Result<bool, OriginalTurnFailure>`: native keeps the pool path (`try_submit_step(pool, Lane::Maintenance, cx.retained_grant())`), wasm keeps the inline worker turn (`try_step_on_worker(grant)`); outcomes go through `take_retained_step_receipt` (receipt posted into `cx`), `acknowledge_outcome` (paid, up to 4 attempts, else fault + `begin_close`), then yield/resume or terminal handback by `JobOutcomeKind`. The close ladder is the frame owner's: return the cancel alias against the matching root, `close_step(cx)`, pay the terminal-empty item.
   - removed: `rejected` field, `ui_contract::UI_WORKER_RETIREMENT_POLICY` use (the crate dependency `ui_contract` in its `Cargo.toml` is now unused; left in place so `Cargo.lock` does not change).
4. Callers:
   - `W/🧊️renderer/📮️mailbox-impl/🦀️.rs`: `pump_asset_decode_session_step(closing, cx)`, `asset_decode_step(cx)`, `close_renderer_asset_step(cx)`, new `close_asset_decode_session_step(cx)` and `asset_decode_deferred_turn()`; a probe goes back into its slot when the invocation cannot be admitted.
   - `W/📨️runtime-mailbox/🦀️.rs`: imports the invocation. `W/🧊️renderer/🖥️frame-presentation/🦀️.rs`: `close_renderer_asset_step(cx)?`.
   - `W/🏠️os-host/🦀️.rs` (`OsHost`): new private `AssetDecodeOwner` (root `CancelToken::admit_root`, `JobPayloadAuthority::admit`, sequence) born lazily by `with_asset_decode_context(run)` which builds `StepContext::with_payload_authority` (budget `MAINTENANCE_LANE_FUEL/WALL_US`, declared per-turn grant `ASSET_DECODE_TURN_GRANT`: 8 items, 4 MiB copy, 4 MiB capacity, 16 MiB release, depth 64). `close_asset_decode_step(cx)` closes the session under that root, then retires authority and root (`close_original_retained_step`, `ControlledRetirement<CancelToken>`) in paid turns of the caller. `try_into_retirement` now runs it first and returns `Ok(None)` until done; the field is destructured as `asset_decode_owner: _` afterwards.
   - `W/🪟️winit-app/🦀️.rs` `pump_frame_inputs`: `self.with_asset_decode_context(|runtime, asset_cx| runtime.asset_decode_step(asset_cx))?`.
   - `W/🌐️browser-worker/🦀️.rs`: JS entry `assetDecodeStep` uses `host.with_asset_decode_context` (JSON contract unchanged; deferred answer `{"kind":"pending","pending":true}` from `asset_decode_deferred_turn`); close phase 0 calls `host.close_asset_decode_step(cx)?` then `runtime.close_renderer_asset_step(cx)?`.
   - `E/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` `decoder_worker_boundary`: builds an unbounded `StepContext`, admits the invocation, pumps with `pump_one(&mut cx)`.
5. Frame owner (`W/🧵️frame-job/🦀️.rs`): `admit_active` no longer calls the removed `WorkerJobSession::try_new`; it admits through `try_admit_owned` with a `WorkerJobAdmissionContext` (identity, clock, budget, recipient folded into `retained_progress`) and returns whether the session was admitted; both `poll_runtime_and_resubmit` callers set `last_submitted_generation` only on success (a refused admission is retried instead of being marked submitted).
6. embedded-mount (`W/🧪️tests/🎚️config/🟦️.ts`, the wgpu package vitest config): added the same four wasm engine stub aliases the react config and the composition config already have (`framework-os-node-graph-rs`, `framework-surface-rs`, `framework-editor-rs`, `flow-core` → `wasmEngineStub`). The dev boot imports `…/framework_os_node_graph.js` lazily; without the alias vite cannot resolve it in a checkout where the wasm package is not built. Not an environment excuse: the other two packages that run this test stub it.

No fixture or test expectation was weakened; `🖌️wgpu-document-owner-move` and its fixture are untouched.

## 3. Verification (static)

- `rustfmt` parse of: `🏠️os-host`, `🧊️renderer/🔁️asset-decode-jobs`, `🧊️renderer/📮️mailbox-impl`, `🧊️renderer/🖥️frame-presentation`, `📨️runtime-mailbox`, `🧵️frame-job`, `🪟️winit-app`, `🌐️browser-worker`, `🧪️tests/🔬️wgpu-renderer-async-boundary` — all exit 0.
- Text checks equal to the tests (python over the files): session slice `impl RendererAssetDecodeSession {` … end of file has none of `root_cancel_token()`, `allocate_operation_id()`, `WorkerJobSession::try_new`, `UI_WORKER_RETIREMENT_POLICY` and has `RendererAssetDecodeInvocation`, `try_admit_owned`; renderer files have `RendererAssetDecodeInvocation`, `take_retained_step_receipt`, `acknowledge_outcome` and no `session.try_step_on_worker()`; `os-host` has `CancelToken::admit_root`, `JobPayloadAuthority::admit`, `StepContext::with_payload_authority`; frame-job has `try_admit_owned`, `take_retained_step_receipt`, `acknowledge_outcome`, `session.try_step_on_worker(self.retained)` and none of `owner.take_outcome()`, `WorkerJobSession::try_new`, `drive_deadline_us`; winit has `semio_framework_job::INTERACTIVE_STEP_CEILING_US / 2` and `FrameLatencyStage::SnapshotPublish`. Crate source limits (262144 B per crate closure): asset-decode about 161 KB, host-winit about 113 KB.
- Module-tree reasoning: the renderer facade pulls `asset_decode` through its `📥️mounts/*` `use` lines; `🧵️frame-job` and `🪟️winit-app` are crate-owned, so their trees do not pull host-base (which still has `try_new` in `submit_renderer_io`).
- Not verified: anything that needs a compiler or vitest. The accounting laws (`fits`, paid receipts, alias witness) can only be proven by a run.

## 4. Crates to compile and tests to run

Compile (native, `bun repo cargo --store=shared --configuration=prototype -- check -p <pkg>`; add `--target wasm32-unknown-unknown` for the starred ones):
- `semio-framework-os-renderer-wgpu-asset-decode` (*)
- `semio-framework-os-renderer-wgpu-runtime-mailbox` (*)
- `semio-framework-os-renderer-wgpu-frame-presentation` (*)
- `semio-framework-os-renderer-wgpu-frame-job` (*)
- `semio-framework-os-renderer-wgpu-host-winit` (*)
- `semio-framework-os-renderer-wgpu-browser-worker` (wasm only)
- then `-- test -p semio-framework-os-renderer-wgpu-runtime-mailbox-tests an_independent_decoder_job` (the two `decoder_worker_boundary` tests; the neighbouring tests in that file still call the removed `close_step(1, JOB_PAYLOAD_PAGE_BYTES)` API and will not compile until their owner ports them, see 5).

Run (vitest, wgpu package config, long level):
- `TARGET=🧊️wgpu` run of `🖌️wgpu-document-owner-move` (expect 11 green), `⏱️frame-latency` (3 green), `🪆️embedded-mount` (the boot test; if it still fails read the first vite resolve error, the alias list may need one more engine package).
- Then the whole wgpu package (56 files, 602 tests; expect 0 failing) and the react package once for the `⏱️frame-latency` case (3,184 tests).
- Regression watch: `⏱️wgpu-worker-step-budget` (reads the wasm legacy `poll_runtime_and_resubmit` slice of frame-job; unchanged text except the `if self.admit_active(...)` wrapper) and `♻️frame-close` (bun suite).

## 5. Open (not done, with reason)

- Same stale job-worker API, untouched (not part of the six cases, other owners): `W/📐️surface-lane/🦀️.rs:258/274` (`MountedWorkerJobSession::try_new`, `pump_one` arity), `W/🏛️host-base/📥️lifted/🔁️asset-decode-jobs/🦀️.rs` `submit_renderer_io`/`RendererIoNode` (`WorkerJobSession::try_new`, `try_submit_step` arity), `W/🧵️frame-job/🦀️.rs` legacy `rejected.close_step(self.retained)`, `session.close_step(self.retained)` in `close_step`/`retire_for_component_surface_close_step` and the `rejected` session type, and the Rust test bodies in `🔬️wgpu-renderer-async-boundary` (`close_step(1, JOB_PAYLOAD_PAGE_BYTES)`, `WorkerJobSession::try_new`) and `🔬️wgpu-frame-job-unit`. I did not delete the legacy frame path: `⏱️wgpu-worker-step-budget`, `♻️frame-close`, the frame-job unit tests and `🦑️repo/…/⚖️laws/🔬️interactivity-audit` anchor on it.
- `🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🖥️mounted/🟦️.ts` still requires `WorkerJobSession::try_new` and `session.close_step(1, …)` in frame-job (stale law, contradicts the owner-move law).
- Unused dependency `ui_contract` in `🦴️asset-decode/📦️packages/🦀️rust/Cargo.toml`; remove together with the next lockfile update.
- Native decode steps still run on the pool with the grant of the admission turn (`step_budget_us` = what was left of that turn); if throughput looks poor, the owner can raise `ASSET_DECODE_TURN_GRANT` or the maintenance budget in `OsHost::run_asset_decode_turn`.
- Scratch kept: `C/🗑️generated/wgpu-tools/vt.sh <label> [vitest filters]` (vitest runner writing to `C/🗑️generated/wgpu-out/`; `TARGET=🧊️wgpu` is its default); it needs a mounted `node_modules`.
