# P7f Independent WFC Cancellation Final Audit

Date: 2026-08-22  
Scope: Current Phase 7 WFC/Assembly ActionBus path, `semio.infer` bridge, cold guest relay, and hot-shard cancellation.  
Method: Fresh read-only source/static audit. Read root and applicable Puzzle instructions, `p7a-wfc-job.md`, all of `p7d-independent-wfc-relay-final-audit.md`, and all of `p7e-independent-wfc-panic-postrepair-audit.md`, including their repair dispositions. No Cargo, Bun, build, test, runtime, Wasm, cache/target, ticket-status, or production-source mutation was performed.

## Verdict: REJECT

P7e's direct P0 is repaired: no current relay or hot-shard production cancellation site discards the fallible `GuestRuntime::cancel_job` result. A failed or panicking *admitted cancel call* quarantines the lease before that call's permit release, and success alone reports `Cancelled` and clears the cleanup obligation.

However, the relay still exposes an uncertain instance between a start/step failure (including retained-future panic) and its separately scheduled cleanup. The request drops its lease as `Available` and releases the semaphore before the caller consumes the fault and schedules `cancel-job`. A second concurrent mounted route may win the semaphore in that interval and enter the guest before the required cleanup/cancellation resolves. That violates the required no-guest-reuse cleanup contract for multi-user concurrent callers. The current authored tests wait for cleanup/quarantine before issuing their next route and therefore do not exercise this interleaving.

## P0 — Start/Step Fault Cleanup Is Not Lease-Atomic

`GuestInstanceLease` restores an unquarantined guest as `Available` on drop ([host component:2754-2767](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2754)). `run_guest_relay_request` converts start/step errors to `GuestRelayCompletion::Started/Stepped(Err(_))`, then unconditionally drops that lease and its semaphore permit before delivering the completion ([host component:2843-2865](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2843)).

Cleanup is only requested later, when a separate `WorkerJobSession` turn reads that completion: the error arm reaches `terminal_with_cleanup` ([host component:2999-3014](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2999)), which calls `schedule_cleanup` and posts a separate retained cleanup future ([host component:2959-2981](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2959)). The same gap follows a retained start/step future panic: `GuestRelayPoolFuture` drops the future (hence restores the lease and releases its permit) before `recover_guest_relay_panic` posts cleanup ([host component:2648-2655](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2648)); the non-cancel panic branch then calls `schedule_guest_relay_cancel` ([host component:2897-2916](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2897)).

Thus, after permit release and before the cleanup future reacquires it, an independently executing `PluginInstanceHandle::infer` / `run_job_on_worker` can create another `GuestColdRelayJob` on the same `Available` slot ([host component:3099-3119](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3099)). `GuestInstanceLease::acquire` accepts `Available` normally ([host component:2725-2742](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2725)), so it can enter `start-job` rather than reject or wait for cleanup. This is a real source ordering issue, not an unverified timing assertion.

The authored start/step failure tests prove only the post-cleanup state: each awaits a cancellation admission and quarantine before attempting the next route ([host component:3604-3641](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3604)); the panic tests likewise wait for cancellation/availability before reuse ([host component:3429-3470](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3429)). No source test holds cleanup admission, races a second mounted route, and proves that route cannot invoke `start-job`/`step-job`.

Required repair: make failed/panicked start and step requests retain exclusive slot ownership through cancellation resolution, or mark the slot non-acquirable/quarantined before releasing the original permit. A successful background cancel may then restore it to `Available`; a cancellation error or panic must retain the typed quarantine. Add one-worker and multi-caller source/runtime regressions that deliberately hold cleanup, race another mounted route, and assert no guest admission, one cancel admission, terminal-once behavior, released permit, and eventual Available-or-Quarantined disposition.

## Source Checks That Pass

| Requirement | Evidence | Static result |
| --- | --- | --- |
| Fallible result is preserved; success alone is cancellation | `cancel_guest_job_once` propagates `cancel_job(...).await?`; the explicit false result is not represented as success ([host component:2786-2811](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2786)). Only `GuestRelayCompletion::Cancelled` clears `cleanup_required` ([host component:3025-3029](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3025)). | PASS |
| Failed/panicking admitted cancel quarantines before its permit release | Foreground conversion calls `guest.quarantine` on ordinary error or consumed admission ([host component:2798-2811](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2798)); the lease transitions `Leased` to `Quarantined` on drop ([host component:2754-2767](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2754)), and the request drops lease before permit ([host component:2863-2865](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2863)). Cancel panic quarantines after the retained future has dropped ([host component:2908-2916](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2908)). | PASS |
| Typed owned quarantine and prompt post-quarantine rejection | The slot stores `detail: Vec<u8>` ([host component:2701-2705](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2701)); acquisition clones and restores the quarantine without guest entry ([host component:2725-2742](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2725)). Ordinary foreground, Drop, start-failure, and step-failure cancellation-error tests assert the stored detail/no re-entry ([host component:3528-3641](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3528)). | PASS after quarantine is installed; P0 rejects the pre-cleanup window |
| No retry/double cancel and background cancel error handling | Scheduling and admission use independent compare-exchange gates ([host component:2868-2894](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2868)); cleanup errors quarantine the lease ([host component:2884-2893](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2884)). | PASS |
| Relay-owned caller token is not preempted by generic driver token | `GuestColdRelayJob` observes its own `cancel` in `step` ([host component:2984-2998](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2984)), while the session passes its separate `params.cancel` only to generic `drive_step` ([job framework:766-803](../../../../../../../../../../../../🧰️framework/🔨️modules/🧵️job/🦀️component.rs:766)). The dedicated test supplies distinct relay/session tokens and cancels only the relay token ([host component:3474-3495](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3474)). | PASS (static) |
| Drop/session abandonment requests cleanup and terminal delivery is once | `Drop` requests cleanup while the obligation remains ([host component:3045-3050](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3045)); later relay turns yield after terminal delivery ([host component:2984-2988](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2984)). The Drop failure test covers exactly-one cancel and quarantined later rejection ([host component:3573-3601](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3573)). Direct document lifecycle wiring remains runtime-unverified. | PASS (static) |
| Closed receiver has cleanup fallback | A closed completion receiver becomes a typed fault through `terminal_with_cleanup` ([host component:2999-3006](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2999)). | PASS (static; runtime unrun) |
| Retained-waker and one-worker progress shape | Finite single polling, wake coalescing, and panic recovery are in [host component:2596-2668](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2596); authored source coverage checks pending guest / competitor progress / terminal-once ([host component:3352-3410](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3352)). | PASS (static; race runtime unrun) |
| Poisoned slot recovery | Slot lock operations recover poisoned mutexes with `PoisonError::into_inner` ([host component:2725-2767](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2725)); authored test covers retained mounted use ([host component:3644-3657](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3644)). | PASS (static) |
| Hot-shard `Effect::CancelJob` bookkeeping, fault, and retirement | Records are removed only on `Ok(())`; error retires the actor then sends `ShardOutcome::Fault` ([shard component:595-627](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs:595)). Failure source test asserts one admission, retirement, no step/retry, and exact fault ([shard component:1245-1280](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs:1245)). | PASS (static) |
| Hot-shard `Payload::Cancel` stable order, first fault, retirement | `running_jobs` is a `BTreeSet` ([shard component:262-285](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs:262)); actor cancellation walks its filtered order, retains the first fault, unregisters regardless, and reports `Fault` rather than `Cancelled` ([shard component:729-758](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs:729)). | PASS (static) |
| Production ActionBus, freshness, and fixed lossless bridge | Procedural registration invokes the Assembly factory on `ActionBus::production` ([procedural component:139-147](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🦀️component.rs:139)); bridge dispatches the exact factory-owned key/schema ([infer bridge:171-223](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:171)); terminal freshness is checked immediately before exposure ([host component:4626-4648](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:4626)). Lossless storage is a fixed two-slot array with checked bytes ([infer bridge:69-127](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:69)). | PASS (static) |
| Exact WFC checkpoint/commit materialization bounds | Checkpoint and commit capacity arithmetic/`try_reserve_exact` are checked, including the assignment vector ([WFC job:320-374](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️component.rs:320)); per-append guards and exact-max/+1 source tests remain present ([WFC job:964-1081](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️component.rs:964)). | PASS (static) |
| No prohibited cold-relay execution substrate | The production relay uses the process-wide host pool and retained finite polls ([host component:239-242](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:239), [host component:2596-2668](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2596)); no production relay `block_on`, private pool/thread, `mem::forget`, or `ManuallyDrop` call was found in the reviewed relay region. | PASS (static) |

## Explicitly Unrun Native, Release, Wasm, And Runtime Gates

No executable pass is claimed. The following remain unrun on the current tree:

- compile and all authored relay/shard/Assembly/WFC/bridge tests in debug and release, including the new fallible-cancel, panic/poison, channel-close, retained-waker lost/double-wake, terminal-race, session-abandonment, and one-worker progress cases;
- the missing P0 concurrent-route-before-cleanup regression, plus allocation-pressure and p99/max watchdog measurements;
- real public-factory replay with 1/2/4/default worker counts and mounted `semio.infer` freshness/document-close integration;
- procedural native development, strict `-D warnings`, and release gates;
- `wasm32-unknown-unknown` and `wasm32-wasip2` build/runtime gates.

The report is intentionally source/static only. Historical successful commands recorded in earlier notes were neither run nor treated as verification of this current audit.

## 2026-08-22 repair disposition

Status: **the cited start/step cleanup ownership P0 is repaired; ready for independent
source/static re-audit. The historical REJECT above remains the prior verdict until that re-audit.**

The slot state machine now makes cleanup non-reusability explicit. `GuestInstanceSlot` owns
`CleanupPending { instance: Option<GuestInstance>, detail: Vec<u8> }` in addition to
`Available`, `Leased`, and `Quarantined`. Every mounted start/step lease begins with an unwind
disposition of cleanup-pending. Ordinary start/step errors replace that detail with their exact
typed fault context. Lease Drop installs the guest into `CleanupPending` under the slot mutex
before the semaphore permit is released, including retained-future unwind.

Cleanup uses a separate cleanup-only lease. It can remove the instance only from
`CleanupPending(Some(_))` and leaves `CleanupPending(None)` publicly visible while the sole
fallible `cancel-job` admission is in flight. Public mounted routes preflight
`CleanupPending`/`Quarantined` and fail promptly; the post-permit acquisition check closes the
preflight race. A raced rejection is a distinct non-cleaning completion, so it cannot schedule
cleanup for a job that never entered the guest. Only successful cleanup transitions to
`Available`. Ordinary cancel error, cancel panic, or consumed admission transitions to
`Quarantined` with owned detail. External Drop cleanup marks an available or leased slot pending;
ordinary request success cannot erase that marker, while producer-owned successful foreground
cancellation resolves it.

Ordinary start/step producers now schedule their already-marked cleanup before publishing their
fault completion. Retained panic recovery observes the cleanup-pending state installed by lease
Drop and schedules the same cleanup-only path. Separate compare-exchange gates still provide
exactly one schedule and exactly one guest cancellation admission. The terminal receiver still
publishes at most once and yields thereafter. The retained-waker pool, one-finite-poll shape, and
one-worker progress contract are unchanged; no executor, thread, pool, or retry was added.

Three deterministic source regressions add an explicit post-release/pre-receiver barrier:

- ordinary start error races a second mounted route, proves no re-entry, then cleanup success
  transitions to `Available` and permits a later clean route;
- ordinary step error races a second mounted route, proves unchanged start/step counts, then cancel
  error transitions to stable typed `Quarantined`;
- retained start panic races a second mounted route before recovery scheduling, proves no re-entry,
  then successful cleanup transitions to `Available`.

At each held barrier the source assertions require `CleanupPending`, zero premature cancel
admission, prompt second-route rejection, and unchanged guest admission counts. After release they
require the original producer fault, exactly one cancel admission, and deterministic final state.
These regressions are authored only and were not executed.

Allowed current-tree evidence:

- `rustfmt --edition 2021 --check` passed for the host relay, hot shard, inference bridge, and WFC
  leaves.
- `bun ./📜️script.ts verify interactivity tool-jobs --format json` exited 0 with 775/775 bounded
  production rows, zero batch-only/forbidden/deleted rows, one production
  factory/registration/dispatch, and zero failures.
- Scoped `git diff --check` passed for the host and shard leaves. Relay production scans found
  zero `block_on`, private worker/thread/pool, batch driver, `mem::forget`, `ManuallyDrop`, or
  temporary debug-output hits. Relay/shard scans found zero discarded fallible cancellation
  results.
- The hot-shard stable `BTreeSet` order, error retirement/unregistration, and typed fault paths
  remain present. WFC checked exact reservations and every per-append guard remain present. The
  inference bridge remains a fixed two-item lossless array with checked aggregate byte addition.
- Cargo, build, tests, runtime, cache deletion, and Wasm commands were not run. No executable,
  timing, race, watchdog, native, release, strict-warning, mounted-integration, worker-count, or
  Wasm pass is claimed.
