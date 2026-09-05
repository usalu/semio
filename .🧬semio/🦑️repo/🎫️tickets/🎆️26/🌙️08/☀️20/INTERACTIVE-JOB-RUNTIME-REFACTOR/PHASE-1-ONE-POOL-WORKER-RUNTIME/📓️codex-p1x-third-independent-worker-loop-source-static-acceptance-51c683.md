# P1x Third Independent Worker-Loop Source/Static Acceptance Audit

Date: 2026-08-24  
Auditor: Codex independent read-only audit  
Verdict: **RED — do not accept P1x yet.**

## Scope And Method

Read completely:

- repository `AGENTS.md`, the interactive-runtime master plan and current residual checkpoint;
- the P1x caller census, both prior Codex RED audits, and the current P1x implementation handoff;
- live DB-engine P1x production/laws, native `WorkerPool`/`TimerWheel`, and root P1x verifier/self-mutations;
- the retained P1w/P1q verification surfaces.

No production source or verifier was edited. No Cargo, Nx, build, Wasm, browser, runtime Rust test, or broad gate was run.

## Accepted Repairs

The former caller/test-task `TimerWheel::fire_due` evidence is gone from the P1x laws. Native
`WorkerPool::worker_loop` services bounded due callbacks before work selection at
[`async component.rs:1591`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:1591)-[`1614`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:1614), and `callback_at` wakes idle workers at
[`1697`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:1697)-[`1703`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:1703).

The finite one-worker law uses a real held worker which later returns to that loop, then asserts
P1x cancel/deadline/exhaustion/rejection close, no backend poll, exact terminal-job retirement and
admission/registry drain at
[`engine component.rs:10813`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:10813)-[`10901`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:10901).
The census precisely narrows permanent sole-worker behavior to discoverability/no backend poll/no
latency claim. The sole-worker law preserves that state without manually servicing timers at
[`10935`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:10935)-[`10965`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:10965).

## Blocking Finding: Two-Worker Proof Is Detached From P1x, And The Verifier Accepts A Missing P1x Retry Timer

The claimed two-worker reserved-capacity proof at
[`engine component.rs:10903`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:10903)-[`10933`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:10933)
does prove that a generic `WorkerPool::callback_at` closure runs on a second native worker while a
`Lane::Maintenance` closure is held. It creates no `DatabaseCreateCatalogFuture`, no P1x rejection,
no retained `retry_job`/`terminal_job`, no cancellation/deadline/exhaustion, no P1x close, no
zero-poll witness, and no registry/admission drain. Thus it cannot establish the requested P1x
reserved-capacity timer progress.

More critically, the permanent verifier cannot detect removal of the P1x retry timer registration.
The only retained-state registration after an I/O refusal is
[`engine component.rs:5863`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5863)-[`5874`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5863):

1. the refused exact driver is retained and authority becomes `Retry`;
2. `callback_at(... state.retry())` is meant to give that P1x state its next real-loop opportunity.

The verifier's retry predicate only requires an undifferentiated
`retained.includes("self.pool.callback_at")`, not a callback registration inside `submit_exact` or
the `Retry` transition ([`script.ts:10374`](/Users/ueli/Documents/semio/📜️script.ts:10374)). Other
unrelated P1x paths (`defer_catalog_contention` and callback close) keep that token present at
[`engine component.rs:5829`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5829)-[`5842`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5842)
and [`5913`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5913)-[`5935`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5913).

### Hostile Static Interleaving

Apply this focused hostile source mutation solely to the rejection branch in `submit_exact`:

```rust
// replace the line registering `move || state.retry()` with:
drop(state);
```

All existing verifier predicates remain satisfied: the exact job is retained, `Queued → Retry`
remains, the terminal checks remain in `retry`, the worker loop still fires timers, and other P1x
callbacks preserve the unscoped `self.pool.callback_at` token. The generic two-worker law and its
token-only verifier evidence remain unchanged; no existing self-mutation removes this particular
registration.

Then take a native interactive pool with two workers, hold one bounded `Lane::Maintenance`
violator, saturate the P1x I/O submission, and submit/cancel a P1x create request. The remaining
worker has real reserved capacity and correctly services the wheel, but there is no P1x callback on
that wheel. The P1x state remains `Retry` with its exact refused job, storage, document, admission
and registry retained; cancellation/deadline/exhaustion cannot enter callback close, and no
admission/registry drain occurs. This is precisely the regression the requested two-worker
P1x liveness proof must reject, yet the P1x verifier would be green.

This is a verifier/law adequacy failure, not a claim that the current live registration is absent.
Under the acceptance rule, a permanent verifier that accepts this concrete source mutation is a
source/static counterexample.

## Required Repair

Replace the generic two-worker callback law with a real P1x saturated request while one native
low-priority violator is held. It must prove the second worker executes the P1x retry/close
callback and assert cancel, deadline, exhaustion and rejection-close outcomes including no ninth
refusal/backend poll, exact job/owner retention, one terminal retirement, and registry/admission
drain. Bind the verifier to the `submit_exact` refusal branch: retained job before `Queued → Retry`,
then its exact `callback_at(... state.retry())` registration. Add a faithful self-mutation deleting
or bypassing that registration; it must be rejected independently of other callback sites.

## Isolated Checks Run

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1x` | PASS — false-green for the focused retry-timer mutation above. |
| `bun ./📜️script.ts verify interactivity p1w` | PASS — preserved. |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS — preserved. |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` on P1x engine and async runtime | PASS. |
| Scoped `git diff --check` on engine, async runtime, verifier and P1x contract | PASS. |

No Cargo, Nx, Wasm, browser, runtime Rust test, or broad build was run.
