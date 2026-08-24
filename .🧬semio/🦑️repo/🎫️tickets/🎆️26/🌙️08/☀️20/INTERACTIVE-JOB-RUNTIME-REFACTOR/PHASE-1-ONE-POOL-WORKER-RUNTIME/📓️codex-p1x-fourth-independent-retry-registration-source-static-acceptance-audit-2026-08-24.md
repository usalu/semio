# P1x Fourth Independent Retry-Registration Source/Static Acceptance Audit

Date: 2026-08-24  
Auditor: Codex independent read-only audit  
Verdict: **GREEN — P1x source/static acceptance only.**

## Scope

Read completely: repository `AGENTS.md`; the Phase-1/master plan and current residuals; P1x
census/contract; the three prior P1x RED reports; the current Sol implementation handoff; live
engine P1x state, laws and helpers; native `WorkerPool`/`TimerWheel`; and the root P1x
verifier/self-mutations. P1w/P1q were rechecked through their isolated preservation gates.

No production source or verifier was edited. This is not Cargo/native/Wasm/browser/runtime-test
evidence and does not close Phase 1.

## Former REDs And Current Production

The prior REDs are closed in the live source:

- Main refusal retains `error.into_job()` before `Queued → Retry`, then registers its own
  `callback_at(..., move || state.retry())` in
  [`component.rs:5863`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5863).
  Its callback consumes the retained job and handles stale generation, cancellation, deadline, and
  exhaustion before publishing `terminal_job` and entering governed callback close at
  [`component.rs:5879`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5879).
- The independently mounted rejection-close authority has the corresponding exact refusal branch
  at [`component.rs:5307`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5307)
  and bounded terminal-job/owner close at
  [`component.rs:5323`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5323).
- Native timer callbacks are actually serviced at the head of `WorkerPool::worker_loop`, before
  work selection, at [`component.rs:1591`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:1591).
  `callback_at` schedules on that wheel and wakes idle workers at
  [`component.rs:1700`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:1700).

## Focused Refusal-Branch Mutations

I independently removed, in memory and separately, the exact main and rejection-close
`callback_at(... state.retry())` expression from only their respective `submit_exact` bodies.
Both mutated bodies fail the same branch-local requirement used by the P1x verifier:

`P1x saturation/retry lacks its exact refusal-branch callback_at registration or bounded stale/cancel/deadline/exhaustion close handoff`

and

`P1x rejection-close saturation retry lacks its exact refusal-branch callback registration or exact job/owner close handoff`.

The independent static mutation check left three unrelated retry-registration sites in the full
P1x region in each case. Thus acceptance is not caused by a generic `callback_at` token elsewhere.
The root verifier itself slices the main and rejection `submit_exact` bodies separately at
[`📜️script.ts:10273`](/Users/ueli/Documents/semio/📜️script.ts:10273) and
[`📜️script.ts:10299`](/Users/ueli/Documents/semio/📜️script.ts:10299), asserts the exact
post-`Queued → Retry` registrations at [`📜️script.ts:10379`](/Users/ueli/Documents/semio/📜️script.ts:10379),
and permanently executes the two separate hostile mutations at
[`📜️script.ts:10516`](/Users/ueli/Documents/semio/📜️script.ts:10516).

## Adversarial Law And Narrowed Liveness Check

The two-worker law creates an interactive two-worker pool, holds the `Maintenance` violator,
temporarily gates the other service worker, and fills alternating `Lane::Io` submissions until
queue saturation at [`component.rs:10537`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:10537).
It mounts actual P1x cancel/deadline/exhaustion authorities and the real rejection-close authority
in `Retry` with retained jobs before releasing only the service gate. The source law proves exact
terminal-job retirement, zero backend polls, no ninth refusal after exhaustion, and retained
admission/registry drain while the Maintenance gate remains false at
[`component.rs:10950`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:10950).
It contains no `timer_wheel().fire_due` use; the P1x verifier rejects manual-fire evidence.

The sole permanently non-returning worker law is correctly limited to discoverability: retained
`Retry` job, exact storage/document, admission and registry stay present; backend polls remain zero;
terminal close reports `Blocked` until real pool service returns
([`component.rs:11053`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:11053)).
The P1x contract contains the matching non-latency guarantee and prohibits invented timer threads,
second pools, and caller-driven completion.

## Verification Performed

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1x` | PASS — live source and hostile mutations clean. |
| In-memory branch-local main registration removal | Rejected; 3 unrelated registrations remained. |
| In-memory branch-local rejection registration removal | Rejected; 3 unrelated registrations remained. |
| `bun ./📜️script.ts verify interactivity p1w` | PASS — preserved. |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS — preserved. |
| Scoped `rustfmt --check` on engine and async runtime | PASS. |
| Scoped `git diff --check` | PASS. |
| P1x source/law `timer_wheel().fire_due` scan | No manual P1x law invocation; remaining hits are async implementation/docs/tests and verifier hostile mutations. |

No concrete remaining P1x source/static counterexample was found. No Cargo, Nx, build, Wasm,
browser, broad gate, or runtime Rust test was run.
