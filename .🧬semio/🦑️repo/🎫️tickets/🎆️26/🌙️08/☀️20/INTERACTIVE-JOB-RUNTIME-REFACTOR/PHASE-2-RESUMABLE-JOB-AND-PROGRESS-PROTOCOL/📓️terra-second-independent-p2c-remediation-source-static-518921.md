# Terra Second Independent P2c Remediation Source-Static Audit — 2026-08-25

## Verdict

**RED — not accepted.** The accepted-submit replay-state repair is real, but the claimed production mount is not an owned ActionBus/plugin-session replay path. Its only native caller is a detached `mountedJobReplay` JSON argument that the ProgramBridge removes before the normal command exchange. The purported 1/2/4/default-worker proof is likewise a direct `MockGuestRuntime`/`ShardExecutor` test, while the production kernel accepts only the one process-global pool count.

This report is additive. It preserves, rather than replaces, [`📓️terra-independent-p2c-live-fixed-replay-source-static-audit-2026-08-25.md`](./📓️terra-independent-p2c-live-fixed-replay-source-static-audit-2026-08-25.md).

## Scope and Method

- Read the P2 master/repair contract, prior Terra RED, updated Sol remediation report, and the accepted P2a1/P2d material.
- Independently read the current ProgramBridge, plugin application, actor, shard, executor, WGPU kernel/client glue, and `📜️script.ts` verifier.
- No source was edited. No Cargo, Nx, Wasm, browser, or build gate was run.
- Executable/runtime acceptance remains deliberately deferred; this is a source-static audit only.

## Gate Record

| Check | Result | Meaning |
| --- | --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --p2c-only --self-test` | PASS | Baseline plus all 67 hostile static mutations passed. |
| `bun ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test` | PASS | All 13 preservation mutations passed. |
| `git diff --check HEAD --` across P2c actor/shard/executor/WGPU/plugin/ProgramBridge/verifier paths | PASS | No whitespace error in the inspected diff. |
| `rustfmt --edition 2021 --check ProgramBridge/component.rs` | PASS | The new bridge file itself is formatted. |
| Full six-file `rustfmt --check` | FAIL (pre-existing/tree-wide formatting drift) | It reports 735 diffs, including actor/WGPU and transitive module files. No formatting was changed in this audit. |

The static verifier is not an acceptance proof for the two blockers below: it recognizes the ProgramBridge parser/call/order as text, but does not require a product producer of the replay authority or test delivery through the actual singleton kernel pool.

## Verified Remediation That Is Structurally Sound

1. The plugin no longer unconditionally rejects the typed-operation path. `PluginApp::require_complete_tool_operation_pipeline` verifies that the command's instance is live and its admitted contract is bounded/resumable before dispatch. This is an improvement over the prior hard reject.
2. `KernelPoolState::run_turn` retains `replay_submit_sequence` and emits `Payload::JobReplay` until `runtime.submit` returns `Backpressure::Accept`. Only that branch calls `accept_replay_submission`; a non-accepted replay start returns `CommandIngressStatus::Idle` before ticking. The retained-packet/unit law also verifies that a retry reuses its sequence, actor, authority, request, worker count, and slot.
3. `request_job_replay` refuses an unequal worker count and a repeat begin whose retained worker count/slot does not exactly match. The lower actor/shard/WGPU retained replay, recovery, and bounded close machinery remains present for cancellation, deadline, stale, fault, panic, `Drop`, and close paths.

Those observations support the local state transition only. They do not establish that a live normal ActionBus session has created the authority that reaches it.

## Blocking Evidence

### 1. Production ProgramBridge replay is a detached, unowned JSON detour

`ProgramBridge/🧊️component.rs:42-94` defines `MOUNTED_JOB_REPLAY_ARGUMENT`, deserializes it from arbitrary invocation arguments, removes it from those arguments, and independently calls `KernelClient::replay_job_turn(instance_id, job, worker_count, begin)`. Both native handlers do perform `exchange` and decode its `InvocationResult` first (`:219-241`), but no value from that exchange/session is used to mint or verify the replay request.

A live-source census of `mountedJobReplay` outside documentation, scripts, and build products finds only this ProgramBridge source. There is no ActionBus/result/session schema, factory, authority, or mounted control producer. The bridge control has only `{ job, workerCount?, begin }`; it carries none of the required operation identity, generation, seed, mounted route, worker slot, restore/start ordinal, or request digest. The tests at `:361-382` only parse this map and resolve a supplied host-default number; they do not instantiate a ProgramBridge, plugin application/session, `KernelClient`, or P2d overlay.

Therefore the native code is ordered as *normal exchange, then optional unrelated magic control*. It is not the required causal route:

`ActionBus → admitted plugin session → KernelClient → Kernel::submit → pinned shard → GuestRuntime restore/start`.

It also prevents this audit from accepting exact owner/refusal behavior for wrong route, seed, generation, or slot: those values cannot enter the native bridge control at all, and the normal invocation does not return an owned replay authority to bind them.

### 2. The 1/2/4/default-worker matrix cannot reach the production mounted pool

Production `KernelClient::replay_worker_count` reads the singleton `renderer_worker_pool().worker_count()` (`wgpu/📦️glue.rs:4076-4078`). `KernelPoolState::request_job_replay` then rejects every count other than `self.worker_count` (`:5027-5035`) with “replay worker count does not identify the mounted shared process pool.” Thus on one real process only its actual pool size can enter; `1`, `2`, and `4` cannot all be verified through the mounted route merely because the parser accepts them.

The claimed matrix at `shard/🏃️executor.rs:726+` directly constructs a `MockGuestRuntime`, a headless `WorkerPool::new(..., worker_count)`, and a `ShardExecutor`, then sends frames. It bypasses ProgramBridge, `KernelClient`, the plugin ActionBus/session, the mounted renderer pool, and P2d. It is useful lower-level coverage, but expressly a test-only/private-pool proof and cannot discharge the production-matrix contract.

## Required Closure

1. Replace the removable invocation argument with a fixed, owned replay authority minted only from the successful normal ActionBus/plugin-session outcome. It must bind the full replay identity: operation, generation, seed, mounted route, actual pool count/slot, restore/start ordinal, request version/digest, and terminal replay record. `handle_action`/`handle_command` must consume that authority rather than arbitrary JSON.
2. Add an end-to-end native fixture that drives the production ProgramBridge and `KernelClient` through an actual plugin session and configured mounted kernel pool for `1`, `2`, `4`, and default worker configurations. It must not construct a private `ShardExecutor`, a mock guest runtime, or a detached replay control.
3. Exercise exact refusals for wrong count, route, seed, generation, slot, ordinal, and digest at the public route; assert the named owner/error and no mutation of replay start/submit sequence.
4. Extend the P2c mutation verifier so parser-only/magic-control and mock-only worker-matrix implementations fail the gate.
5. Run the deferred executable gates after the source route exists: production ActionBus-to-GuestRuntime replay; deterministic record/replay at all four worker configurations; reject/Idle/backpressure retry; cancel/deadline/stale/fault/panic/Drop/close retirement; and P2a1 preservation.

## Acceptance Status

The submitted change improves the retained start/ACK law and clears the earlier unconditional plugin rejection structurally. It does **not** yet meet the P2c live fixed replay-driver acceptance criteria. Keep P2c **RED** until both the owned production authority and real mounted-pool matrix evidence exist, then re-run this audit plus the executable gates.
