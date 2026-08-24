# P1x Post-RED Independent Source/Static Acceptance Re-Audit

Date: 2026-08-24  
Auditor: Codex independent read-only re-audit  
Verdict: **RED — P1x must not be accepted yet.**

## Scope And Method

Read completely:

- repository and applicable product/OS `AGENTS.md` instructions;
- the prior independent P1x RED audit;
- the implementation/remediation report;
- the P1x caller census contract;
- the live P1x `CreateDocumentCatalogCas` region, public catalog facade, hostile Rust-law bodies, and root P1x verifier/self-mutations;
- the shared `WorkerPool`/`TimerWheel` execution path needed to validate the new callback-close authority.

No production source or verifier was changed. Cargo, Nx, Wasm, browser, runtime Rust tests, and broad builds were not run.

## Resolved Source Shapes

The three previously reported local code shapes are materially repaired:

- Main retry now retains the refused job, evaluates currentness/cancellation/deadline/exhaustion, transfers the exact job to `terminal_job`, and uses a callback-close path ([engine:5845](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5845), [engine:5861](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5861)). The rejection-close authority has the corresponding bounded handoff ([engine:5299](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5299), [engine:5315](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5315)).
- The fixed admission envelope is supplemented with a checked observed-backing ledger. It observes initial document/base vector capacities, each existing and cloned string capacity, candidate vector capacity, Arc control, and page bytes; allocation owners are stored before resulting overage faults ([engine:5016](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5016), [engine:6043](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:6043), [engine:6121](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:6121), [engine:6190](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:6190), [engine:6240](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:6240)).
- Claim, revalidate, and pending-token retirement use `try_lock` plus one deferred callback; the public `Database::catalog` captures an immutable `Arc` under its mutex and deep-clones only after releasing it ([engine:6333](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:6333), [engine:6454](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:6454), [engine:6509](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:6509), [engine:7295](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:7295)).

## Blocking Finding: Callback Close Cannot Progress Under The Required Permanent Held-Pool Saturation

The new authority removes the *second lane admission*, but it does not provide an execution source independent of the shared pool. `callback_at` merely stores the callback in that pool's timer wheel ([async component:1697](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:1697)). On native targets, the timer wheel is serviced only at the head of `WorkerPool::worker_loop` ([async component:1591](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:1591)); its own contract explicitly says an idle/awake worker calls `fire_due` ([async component:700](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:700)). `fire_due` executes the callback synchronously on that caller ([async component:778](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:778)).

### Hostile Interleaving

1. Create a single-worker shared pool, submit the blocking `Lane::Io` job, and fill the I/O queue until it rejects. This is exactly the live held-saturation fixture ([engine:10440](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:10440)). Do not release its condition variable.
2. Submit P1x. `submit_exact` retains the exact refused driver job in `retry_job`, changes `Queued → Retry`, and registers `state.retry()` with `callback_at` ([engine:5845](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5845)).
3. Cancel it, expire its deadline, or wait through the retry limit. All of those terminal checks live inside the queued timer callback, not in `cancel` or `submit_exact` ([engine:5861](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5861)).
4. The sole worker remains inside the held I/O closure, never returns to the top of `worker_loop`, and therefore never calls `fire_due_batch`. The retry callback never executes. The state stays at `Retry`, retaining the refused job, admission, storage, document, cursor/base, and registry entry indefinitely. The same mechanism strands `DatabaseCreateCatalogRejectedClose` at `Retry` ([engine:5315](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5315)).

This is not cured by `callback_at` avoiding a new I/O-queue admission. Callback *execution* still requires a shared worker opportunity. Thus the source still violates the required permanent held-`Lane::Io` saturation behavior: bounded exhaustion/cancel/deadline cannot reach exact incremental close in the live runtime.

## Hostile-Law And Verifier False-Green

The held-saturation law manually invokes `pool.timer_wheel().fire_due(u64::MAX)` from the test task ([engine:10745](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:10745), [engine:10755](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:10755)). That external call is precisely the progress source absent from the production hostile trace, and it synchronously runs the callbacks on the test task. It therefore proves only the close logic *if another driver services the wheel*, not the asserted held shared-pool behavior.

The permanent P1x verifier recognizes timer firing only as a law-body token and does not bind the law to the native worker-loop servicing condition or reject manual external `fire_due` as the only progress source ([script.ts:10343](/Users/ueli/Documents/semio/📜️script.ts:10343), [script.ts:10508](/Users/ueli/Documents/semio/📜️script.ts:10508)). Its 62 mutations all pass while this live interleaving remains possible. The P1x static gate is consequently false-green for this requirement.

## Other Re-Audit Results

- No further concrete source/static counterexample was found in the observed capacity ledger, retained overage retirement, nonblocking claim/revalidate/retire mutex paths, or post-unlock `Database::catalog` clone.
- The P1w and P1q regions were not modified by this finding; their isolated preservation gates pass.
- The Rust hostile laws are source evidence only; they were not executed.

## Isolated Checks Run

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1x` | PASS — false-green for the held-pool timer-execution counterexample above. |
| `bun ./📜️script.ts verify interactivity p1w` | PASS. |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS. |
| `rustfmt --edition 2021 --check --config skip_children=true` on DB engine | PASS. |
| Scoped `git diff --check` on DB engine/root verifier | PASS. |

No Cargo, Nx, Wasm, browser, runtime Rust test, or broad build was run.

## Required Remediation Direction

Either narrow the contract so that a permanently non-returning shared-pool worker is explicitly outside P1x's liveness guarantee, or provide a bounded close/timer progress authority that can run while every I/O worker is held. The current law must reproduce that authority in the same way as production; manually calling `TimerWheel::fire_due` from the test thread cannot demonstrate it.
