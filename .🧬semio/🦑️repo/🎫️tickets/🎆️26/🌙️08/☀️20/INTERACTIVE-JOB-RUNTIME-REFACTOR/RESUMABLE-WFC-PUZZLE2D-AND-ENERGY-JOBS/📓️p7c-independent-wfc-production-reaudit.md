# P7c Independent WFC Production Re-audit

## 2026-08-22 P0 implementation response

Status: **source/static re-audit-ready; the prior REJECT below remains the historical independent
verdict until re-audit, and executable gates remain open**.

The cited production defect is removed. `GuestColdRelayJob::step` contains no synchronous future
driver. It owns one pending completion receiver and, per caller turn, either submits one guest
start/step future, performs one `try_recv`, or yields. The future owns the guest instance after a
shared semaphore admission, is polled once per finite process-`WorkerPool` closure, retains its
waker across genuine suspension, and restores the instance before publishing its exactly-once
completion. No mutex guard crosses an await, no worker waits for completion, no private
thread/pool/executor exists, and no pending relay turn admits another guest step.

All interactive cold host routes converge on this relay. The 2-ms/user-visible fuel budget,
guest checkpoint/publication bridge, terminal bridge, and host freshness validation remain intact.
Cancellation races the in-flight guest future; pending-call and nonterminal-Drop cleanup use one
atomic cancel admission. The persistent worker session now suppresses both post-terminal job entry
and duplicate terminal delivery.

The adjacent production timer blocker found during reconciliation is also removed:
`PeriodicPoolTimer` uses `WorkerPool::submit_at` so its deadline wait owns no worker.

Authored source regressions cover a deliberately stuck guest on a one-worker pool while another
user-visible job progresses, repeated pending polls with exactly one guest step admission,
completion with no duplicate outcome, cancel-vs-complete with exactly one guest cancel, and
nonterminal Drop while the token is initially live.

Static evidence on the repaired tree:

- Rustfmt check on both repaired leaves: exit 0.
- Interactivity verifier: exit 0, 775/775 bounded production rows, zero
  batch-only/forbidden/deleted rows, one factory/registration/dispatch.
- Repaired production relay/timer region: zero `block_on`, private pool/thread, batch driver, or
  async self-requeue hits.
- Qualified host-component `block_on` occurrences are confined to six pre-existing
  `#[cfg(test)] mock_guest_runtime_tests` calls.
- Repaired-region whitespace/debug scan: zero trailing whitespace or temporary debug output.

No Cargo, Bun test/build, native runtime, actual-pool replay, debug/release executable test, strict
warning build, or Wasm build was run for this response. None is claimed passing. The original P1
required exit gates below therefore remain required for executable acceptance.

Date: 2026-08-22
Scope: Current Phase 7a Assembly/WFC production route only; source/static review.
Verdict: **REJECT**

## Decision

The two P7b production-route defects are resolved in the current source: the public
`semio.infer` route selects the exact `semio.infer` / `s.assembly.solve` ActionBus factory, and
the host router owns live revision/generation authority and validates it immediately before
returning a terminal result. The bridge and factory source also now express the requested
publication and scheduling shape.

This is nevertheless a **REJECT**. The production host relay still calls `block_on` from inside
an `InteractiveJob::step` running on the shared WorkerPool; that conflicts with the plan's
interactive no-blocking boundary. In addition, the required current-tree native/debug/release,
actual-pool, and Wasm gates have not run. No historic result is accepted as proof for this tree.

## Blocking Findings

### P0 — Production relay blocks a shared WorkerPool turn

`GuestColdRelayJob::step` acquires the guest instance and calls
`semio_framework_async::block_on(self.runtime.step_job(...))` directly
([host component:2518](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2518),
[host component:2523](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2523),
[host component:2529](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2529)).
`PluginInstanceHandle::run_job_on_worker` executes that job by `WorkerJobSession::step` on the
process-wide user-visible pool ([host component:2560](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2560),
[host component:2583](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2583),
[host component:2586](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2586)).

This is not a process/test entry point. A guest `step_job` that genuinely suspends or contends on
its runtime can park a user-visible worker, so the source cannot establish the plan's no-blocking
interactive scheduler contract. The per-turn `2 ms` guest grant and `WorkerJobSession` prevent the
old 50M/200-ms/run-to-completion shape, but do not make this nested synchronous drive nonblocking.

### P1 — Required current-tree executable gates remain unrun

The implementation record explicitly says Cargo/test/build was not run and defers focused
debug/release, native dev/strict/release, actual WorkerPool replay, and both Wasm targets
([P7a record:44](📓️p7a-wfc-job.md:44), [P7a record:53](📓️p7a-wfc-job.md:53),
[P7a record:55](📓️p7a-wfc-job.md:55)). The plan requires boundedness, deterministic worker-count
replay, freshness at publication, and Wasm coverage ([attached plan:114](/Users/ueli/.codex/attachments/2225dd4d-c3b6-4564-b4b1-f552928e8ff3/pasted-text.txt:114),
[attached plan:116](/Users/ueli/.codex/attachments/2225dd4d-c3b6-4564-b4b1-f552928e8ff3/pasted-text.txt:116),
[attached plan:120](/Users/ueli/.codex/attachments/2225dd4d-c3b6-4564-b4b1-f552928e8ff3/pasted-text.txt:120)).

The authored tests are useful source evidence, but this audit did not run Cargo or Bun and cannot
claim that the current code compiles, stays within the watchdog under allocation pressure, or
works on either Wasm target.

## Verified Source Reconciliation

| Requirement | Current source evidence | Static result |
| --- | --- | --- |
| Exact discoverable factory, no synchronous registry bypass for Assembly | The plugin registers the factory and advertises metadata-only routed inference ([procedural plugin:140](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🦀️component.rs:140), [procedural plugin:146](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🦀️component.rs:146)); the factory fixes key/schema at [Assembly inference:17](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:17) and is registered through `register_once` at [Assembly inference:495](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:495). The cold handler chooses ActionBus first and only takes the synchronous registry fallback when that exact key is absent ([infer bridge:164](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:164), [infer bridge:167](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:167), [infer bridge:188](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:188)). | PASS (static) |
| Factory-owned decode and restart checkpoint | The bridge gets the owned payload schema then dispatches canonical payload plus `restored` bytes to the exact factory ([infer bridge:189](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:189), [infer bridge:191](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:191)); factory insertion replaces the request checkpoint only from the separately supplied restart state ([Assembly inference:486](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:486)). | PASS (static) |
| One step per guest continuation; no 50M/200ms or self-requeue | `WorkerJobSession::step` submits one closure and awaits one outcome, without scheduling a successor ([job framework:751](../../../../../../../../../../../../🧰️framework/🔨️modules/🧵️job/🦀️component.rs:751), [job framework:765](../../../../../../../../../../../../🧰️framework/🔨️modules/🧵️job/🦀️component.rs:765)); guest budget is capped to the user-visible fuel and 2-ms wall bound ([infer bridge:193](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:193), [infer bridge:200](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:200), [infer bridge:216](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:216)). Host relay uses the same 2-ms constants ([host component:2508](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2508), [host component:2575](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2575)). | PASS for turn shape; **REJECT** for blocking P0 |
| Explicit preview/checkpoint/commit/diagnostic policies and saturation | Bridge limits preview to 1 MiB/latest slot ([infer bridge:14](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:14), [infer bridge:90](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:90)); checkpoint/commit is FIFO, two items/2 MiB and reports oversize/saturation ([infer bridge:103](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:103)); diagnostics are an overwrite-oldest 32-item/64-KiB ring ([infer bridge:122](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:122)). Every interactive outcome enters this bridge before its checkpoint/terminal boundary ([infer bridge:219](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:219), [infer bridge:225](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:225), [infer bridge:231](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:231)). | PASS (static) |
| Checkpoint restart after state loss | The parent chooses incremental `WfcRestore` when it receives a checkpoint ([Assembly inference:300](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:300)); the mounted route source test exercises start/checkpoint/cancel/restore ([Assembly inference:1053](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:1053)). | PASS (static; unrun) |
| Authoritative live revision+generation validation before terminal exposure | `ArtifactInferenceRouter` inserts one active identity, removes its live authority only after the guest returns, then calls `validate_commit` before `Ok(result)` ([host component:3646](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3646), [host component:3654](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3654), [host component:3662](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3662)). The model actor has an explicit update handoff ([host component:3671](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3671)). | PASS (static; test is unrun) |
| Registration idempotence/collision and real 1/2/4/default pool shape | Production registration/collision source coverage is at [Assembly inference:1085](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:1085). The pool replay helper constructs `WorkerPool` and `WorkerJobSession` ([Assembly inference:1106](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:1106)) and the test requests 1, 2, 4, and `available_parallelism` ([Assembly inference:1134](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:1134)). | PASS (authored shape; unrun) |
| No broad warning suppression in the touched procedural/WFC path | The only procedural `unused_*` allowance is attached to the dyn-enum macro invocation ([procedural plugin:9](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🦀️component.rs:9)); the P7a record states WFC module/crate suppressions were removed ([P7a record:92](📓️p7a-wfc-job.md:92)). | PASS (static) |

## Required Exit Gates

1. Remove the `block_on` call from the user-visible `GuestColdRelayJob::step` path, then prove the
   guest step is nonblocking while preserving exactly one admission per caller turn.
2. Run the focused Assembly/WFC/bridge tests in debug and release, including maximum-admission
   allocation-pressure and preview/watchdog timing evidence.
3. Run the authored actual WorkerPool replay through the exact public factory at 1, 2, 4, and
   host-default counts; retain byte-identical result evidence.
4. Run procedural native development, strict `-D warnings`, and release gates on the immutable
   repaired tree.
5. Build and record both `wasm32-unknown-unknown` and `wasm32-wasip2`; do not infer their status
   from native compilation.
6. Run a mounted host-router → guest `semio.infer` integration test that updates the live
   revision/generation after the final worker step and proves no terminal result is exposed.

## Audit Method and Limits

Read-only source review of the attached interactivity plan, `📓️p7a-wfc-job.md`, the preserved
P7b rejection and implementation response, ActionBus/job framework, cold inference bridge,
production host relay/router, procedural plugin/descriptor path, and Assembly/WFC sources/tests.
No production source, manifest, status, JSON, git state, target directory, or cache was changed.
No Cargo, Bun test/build, or runtime/Wasm command was run; all executable claims above are therefore
explicitly unverified.
