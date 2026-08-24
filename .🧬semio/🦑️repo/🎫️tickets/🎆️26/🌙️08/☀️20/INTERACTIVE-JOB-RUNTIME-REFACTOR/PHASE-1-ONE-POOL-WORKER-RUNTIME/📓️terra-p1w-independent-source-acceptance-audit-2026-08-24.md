# Independent P1w Source Acceptance Audit

Date: 2026-08-24  
Auditor: Terra independent read-only audit  
Verdict: **RED — do not accept P1w.**

## Scope And Method

Read in full:

- `/Users/ueli/Documents/semio/AGENTS.md`;
- the P1w caller census and implementation report named in the audit assignment;
- the live DB engine P1w region, its exact `Database::open_with` call site and P1w laws;
- the root P1w verifier and relevant native `WorkerPool` scheduling implementation.

No production source or verifier was changed. No Cargo, Nx, Wasm, browser, native/release, or broad build was run.

## Blocking Finding: Cancellation Can Admit A Concurrent First Poll

`DatabaseCatalogBootstrapFuture::cancel` treats `polling == false` as permission to schedule a second callback at [component.rs:3292](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3292). The active callback clears `scheduled` *before* it has taken the work or raised the polling gate at [component.rs:2853](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2853), while `polling` is only set after the callback has taken a work owner at [component.rs:2948](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2948). This is a load/store observation, not an atomic claim of the sole poll authority.

Concrete permitted execution on a native pool with at least two workers:

1. The Handoff callback clears `scheduled` at line 2855 and installs `Poll` work at lines 2930–2936.
2. Its scheduled Poll callback begins and clears `scheduled` at line 2855, but has not yet reached line 2950 or raised `polling` at line 2954.
3. A facade thread calls `cancel`; it observes `polling == false` and schedules a second callback at lines 3293–3298.
4. The original Poll callback takes the unique work and raises `polling`; the second callback can concurrently observe `Poll` plus cancellation, set `RetainWork`, and schedule retirement at lines 2870–2880 without owning that work.
5. The first callback can then execute the backend `cas_root` poll at [component.rs:2958](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2958), after cancellation has already transitioned the state. The two callbacks can independently transition, publish, and retire the same operation graph.

This violates the required Handoff-to-first-poll cancellation property and the promised one polling gate. It also invalidates the one terminal result/owner-retirement guarantee: a callback can publish/retire a cancelled state while the other retains (and may poll) the backend work. `scheduled` only prevents duplicate queueing while set; it is deliberately cleared before the exclusive poll authority exists.

The shared `WorkerPool` really can execute queued jobs concurrently: it constructs `worker_count` native threads at [component.rs:1627](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:1627) and accepts independent lane jobs at [component.rs:1663](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:1663). Therefore this is not a single-thread-only hypothetical.

## Verifier False-Green

The reported P1w verifier passes because it checks only for the presence of `polling.load`, `wake_requested.store`, and `schedule` in cancel/Drop at [script.ts:9841](/Users/ueli/Documents/semio/📜️script.ts:9841); it never requires an atomic poll claim, never checks that `scheduled` remains claimed until that claim, and never injects the stated interleaving. Its hostile fixture represents the cancellation contract as bare tokens at [script.ts:9902](/Users/ueli/Documents/semio/📜️script.ts:9902), so the self-test cannot establish runtime mutual exclusion.

The live interruption law is also not a multi-worker race law. Its controlled submit hook stores jobs in one manually popped queue at [component.rs:7360](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:7360); the handoff law executes those jobs serially at [component.rs:7400](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:7400). It cannot expose the two-worker cancellation window above.

## Non-Blocking Evidence

- Exact caller cutover is present: fresh `Database::open_with` submits `open_catalog_bootstrap_retained`, awaits, and returns the retained storage/epoch at [component.rs:5066](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5066).
- The backend CAS is mounted only when `DatabaseCatalogBootstrapWork::poll` first runs on the I/O job at [component.rs:2645](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2645), and mismatch validation preserves an exact backend error at [component.rs:3046](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3046).
- The static implementation includes generation-qualified admission/release at [component.rs:2245](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2245), retained refusal closure at [component.rs:2469](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2469), and result-Drop handback at [component.rs:2401](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2401). Those shapes do not cure the blocking concurrent-owner transition.

## Gates Run

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1w` | PASS — static verifier false-green against the finding above |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS |
| `rustfmt --edition 2021 --check --config skip_children=true` on DB engine | PASS |
| scoped `git diff --check` on DB engine and root script | PASS |
| caller/CAS/poll/forbidden-equivalent source sweep | completed; no additional P1w caller bypass found |

## Required Acceptance Closure

Make acquisition of the sole drive/poll authority atomic across the interval currently opened by lines 2855–2954, and preserve it until the work owner is safely republished. Cancellation/Drop must only record wake/cancel against that claimed authority, never submit a competing driver. Add a deterministic two-worker law that freezes a Poll callback after `scheduled` is cleared but before the poll claim, cancels from another thread, then proves: no backend poll begins after cancellation; exactly one terminal completion exists; the exact storage/pages remain recoverable; and admission/registry retires exactly once. Extend the root hostile mutations to reject removal of that atomic claim and restoration discipline.
