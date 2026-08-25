# Terra Independent P2c Live Fixed Replay Source/Static Audit

Date: 2026-08-25  
Auditor: Terra  
Verdict: **RED.** The fixed ownership/source predicate is green, but P2c does not mount the required end-to-end production replay route. Phase 2 remains red.

## Scope

This is an independent read-only audit of the exact current tree. It inspected the master plan, P2c repair contract, P2c remediation report, accepted P2a1 and P2d reports, root verifier, actor replay authority, shard/component and executor, WGPU kernel glue, ActionBus, plugin app, and guest host boundary. No source was edited. No Cargo, Nx, native/Wasm, browser, or runtime gate was run.

## Blocking Findings

1. **The ActionBus/plugin-session half of the claimed live route is fail-closed before the session exists.** `VcsArtifactApp::dispatch_typed_command_inner` calls `require_complete_tool_operation_pipeline` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:19019`. That helper unconditionally returns `Err(..."interactive-job.full-operation-pending"...)` at `:19010-19012`. Consequently its later `tool_jobs.dispatch`, `MountedWorkerJobSession::try_new`, and shared-pool `drive_worker_step` code at `:19092-19130` is unreachable on every production command. The static verifier merely checks those strings are present; it does not prove that the route is executable.

2. **There is no production caller for the replay client.** The complete source census of `replay_job_turn(` in the WGPU glue finds only the public client wrapper (`:4074`), private state handler (`:5682`), and the request-loop dispatch (`:6394`). No product/session/ActionBus caller invokes the wrapper. The `KernelRequest::ReplayJob` census has only its declaration, wrapper construction, request accounting, and dispatch. Therefore the claimed ActionBus → plugin session → KernelClient → `Kernel::submit` → pinned shard → restore/start route has no reachable production entry point.

3. **The 1/2/4/default worker evidence is a mock-shard unit fixture, not a mounted client-route law.** `mounted_fixed_replay_uses_the_same_shard_guest_route_at_one_two_four_and_host_default_workers` directly constructs `WorkerPool` and `ShardExecutor` with `MockGuestRuntime` at `.../shard/🏃️executor.rs:726-744`. It manually sends `Payload::JobStep`/`Payload::JobReplay` frames at `:775-798`. It neither invokes `KernelClient::replay_job_turn` nor traverses ActionBus, the application session, P2d, or the live kernel queue. It cannot satisfy the contract's required end-to-end deterministic record/replay proof.

4. **Replay ingress is not backpressure-safe.** `KernelPoolState::run_turn` sets `entry.replay_started = true` while constructing `Payload::JobReplay` at `.../wgpu/📦️glue.rs:5847-5852`, before `self.runtime.submit(envelope)` is checked for `Backpressure::Accept` at `:5880-5885`. A non-Accept result only logs. The next replay request will emit `Payload::JobStep` because the retained state now says replay has started, even though the shard never received the mandatory restore/start `Payload::JobReplay`. This violates exact refusal/retention and one-owner restart semantics under mailbox pressure.

These findings falsify the P2c contract's mounted route, actual production caller census, worker-count determinism, and backpressure requirements. They are source-level blockers independent of deferred compilation/runtime work.

## Confirmed Partial Foundation

- The actor log is fixed-capacity (`256` records with retained fixed pages), records the requested identity fields, performs fuel-before-page-copy, records cancellation classification in its digest, and has incremental close/recovery structures.
- The shard has fixed replay seed/refusal backing and explicit checkpoint, restore, and start phases.
- The WGPU state has a fixed replay slot plus P2d publication/ACK composition. These pieces are not accepted as a mounted end-to-end implementation because the blockers above leave the route unreachable or incorrect under backpressure.
- P2a1 preservation remains green in its focused verifier.

## Executed Static Evidence

```text
bun ./📜️script.ts verify interactivity tool-jobs --p2c-only --self-test
[verify interactivity tool-jobs p2c] live-source clean; hostile-mutations=53.

bun ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test
[verify interactivity tool-jobs p2a1] live-source clean; hostile-mutations=13.

git diff --check HEAD -- <four P2c source files> 📜️script.ts
# clean; no output
```

`rustfmt --edition 2021 --check` was executed on the four P2c source files. It is **RED on the shared current tree**, reporting inherited formatting at actor `component.rs:320,331` and unrelated included WGPU renderer/plugin files. This audit did not edit them. The specific P2c regions were not identified as formatting failures by that command.

The caller census above is the required independent census: no product `KernelClient::replay_job_turn` call exists; the only 1/2/4/default fixture bypasses the client/session route and uses a mock shard.

## Required Remediation

Mount one real command/job entry through the successful ActionBus/plugin session into `KernelClient::replay_job_turn` and the existing kernel queue, with the full identity retained from admission. Replace the unconditional app gate with actual bounded pipeline authority for the mounted fixture/path. On every non-`Accept` replay submit, retain the exact request and keep `replay_started` false until an accepted submission/acknowledged start; provide explicit bounded retry/refusal/close. Add a hostile source mutation for that ordering. Replace the mock-only worker-count law with a mounted KernelClient/P2d client-route law at 1/2/4/default workers, including wrong identity and MAX+1/backpressure/refusal witnesses.

## Deferred Gates

Executable acceptance remains deferred: Cargo/debug/release/strict-warning, native and both Wasm targets, browser-mounted preview, real cancellation/panic/stuck/close stress, measured p99 cancellation, and the final 8 ms matrix were intentionally not run in this source/static audit.
