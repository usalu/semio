# Independent P1z Sync-Hello Source/Static Re-Audit

Date: 2026-08-24  
Auditor: Codex, independent read-only source/static audit  
Verdict: **RED — P1z must not be accepted.**

## Scope And Method

Read completely: repository `AGENTS.md`; the governing Phase-1 attachment (both ticket `📌️important.md` files are empty); the P1z caller census, prior RED audit, and remediation handoff; the available P1q/P1w/P1x/P1y boundary audits; live sync, WAL, engine, hub, shared async-pool, and root verifier source.

No production source or verifier was edited. No Cargo, Nx, build, Wasm, browser, or runtime Rust test was run. This report is the sole file written.

The selected cutover itself is present: `Database::hello` mounts `DatabaseSyncHelloFuture` and awaits its terminal witness ([engine](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:7390)); the hub takes `Welcome` once and awaits one `next_frame` for each send ([hub](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:775)); all P1z job submissions select the shared `Lane::Io` ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1337)). No selected `block_on`, nested pool, or eager production follow-up vector was found in that path. These positive shapes do not close Z1–Z4.

## Blocking Counterexamples

### Z1 — Snapshot Output Credit Is Returned While Its Actual Backing Is Still Externally Live

The snapshot cursor pre-debits its new chunk before allocation ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:983)) and transfers that `Vec<u8>` into `ServerFrame::SnapshotChunk`, recording only its capacity in `outstanding_bytes` ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1003)). On the very next frame request it unconditionally returns the item+byte debit before it can know the earlier returned frame has been destroyed ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:971)).

`DatabaseSyncHelloSession::next_frame` takes `&self`, so a caller can retain the first `ServerFrame` and request another ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1734)). The hub happens to send then drop its local frame before its next request, but that is not an ownership acknowledgement enforced by the public session API ([hub](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:787)). Therefore another output allocation can begin after the old debit has been returned while the old allocation remains live: the ledger does not represent actual cumulative output capacity.

The pre-debit helper repairs the previous decoder allocation gap, but it cannot establish the required exact debit return for this escaped output owner.

### Z2 — A Cancelled Session Can Produce Another Tail Or Snapshot Frame

`DatabaseSyncHelloSession::cancel` only sets an atomic and requests deferred close ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1738)). A `next_frame` poll independently sets demand and schedules the driver ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1788)). The driver consumes that demand and invokes `prepared.follow_up.drive_one(ledger)` with no cancellation/deadline control check ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1419)). `drive_one` can then allocate/copy a snapshot fragment or transfer a tail frame ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:962)).

Faithful interleaving: stream one frame; call `session.cancel()`; before the timer callback marks `close_requested`, poll a new `next_frame`; its I/O job reaches the demand branch and creates/publishes another frame. This is cancellation after a cooperative driver opportunity and before the next output/backend-facing opportunity, so it violates the required cancel/deadline recheck.

The helper itself checks on both sides of its own yield ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:649)), but four live cleanup-yield paths discard the result instead of rechecking: decode failure ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:848)), page close ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:870)), replay failure ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:937)), and skipped-tail retirement ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1124)). The direct use of `let _ = database_sync_hello_control(...)` makes the post-yield control outcome ineffective.

### Z3 — Snapshot Close May Suppress A Page-Close Error And Release The Registry Anyway

The snapshot close cursor converts `pages.close_step()` errors to `None` through `.ok().flatten()` ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1053)). In the parent close path, a false return merely reads `prepared.follow_up.terminal_is_empty()` and discards it ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1491)). It may then clear `execution` wholesale ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1523)) and release registry/admission ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1529)).

Thus a real `close_step` error is not retained behind a discoverable close owner. The `DbIoPages` owner is instead dropped through the enclosing execution after release, without proving its terminal cursor or exact page debit. This independently violates the Z3 order requirement.

The named quarantine cursor is better than the prior implicit `None`, but it still destroys the opaque boxed future directly in its one operation and zeroes its byte count rather than returning a ledger debit tied to the backing ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1255)). The snapshot-close counterexample is sufficient even if that opaque-future disposal were accepted as one governed backing.

### Z4 — Rejection Exhaustion Is Neither Discoverable Nor Guaranteed To Close

The rejection close has an attempt/deadline/cancel predicate, but at terminal retry it re-inserts the exact job into a private mutex, stores a numeric terminal value, and returns ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1854)). There is no registry, terminal-witness object, public inspection method, or terminal close cursor retaining that job/input owner. `close_and_take_error` returns only `DbError` ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1917)); after the callback returns the last `Arc<DatabaseSyncHelloRejectedClose>` may disappear, dropping its private `retry_job` and `owners` wholesale.

It is also not bounded in the required liveness sense under the explicit held-pool condition. Rejection installs its retry through `callback_at` ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1828)), but the timer only runs when a shared worker enters `worker_loop` ([async pool](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:1591)); callbacks execute synchronously from that poll ([async pool](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs:796)). If all I/O workers are permanently held, cancellation, deadline, and retry exhaustion are never evaluated. The rejected job and exact owners remain private indefinitely. This reproduces the previously documented P1x timer-progress boundary and is still reachable by P1z.

## Verifier Mutation Reproduction

The root P1z verifier and all requested preserved P1 gates pass, including their current in-memory hostile mutation lists. I independently bound and applied, in memory only, the verifier's exact branch-local replacements for:

- envelope `ledger.observe` bypass;
- post-yield control removal;
- quarantine close removal;
- quarantine byte-zero removal; and
- refusal retry-limit removal.

Each source anchor occurred exactly once and each corresponding self-test literal bound. The full P1z gate then confirmed those listed mutations are rejected.

I also applied a distinct faithful insertion immediately before the existing origin move:

```rust
let uncharged_origin_clone = owners.origin.0.clone();
drop(uncharged_origin_clone);
let origin = protocol::ActorId(std::mem::take(&mut owners.origin.0));
```

The required move remains, and the verifier's clone predicate only tests `owners.origin.clone()`, not `owners.origin.0.clone()` or an arbitrary inserted clone. Its listed origin mutation replaces the move, so it detects the missing move rather than this additive uncharged allocation. The current predicate is therefore false-green for the Z1 inserted-clone mutation. It also has no mutation for escaped snapshot-frame credit, cancel-before-stream-drive, page-close-error suppression, private terminal-witness loss, or held-pool timer non-progress.

## Executed Static Gates

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1z` | PASS — listed mutations clean; insufficient for the findings above |
| `bun ./📜️script.ts verify interactivity p1y` | PASS — preserved gate |
| `bun ./📜️script.ts verify interactivity p1x` | PASS — preserved gate |
| `bun ./📜️script.ts verify interactivity p1w` | PASS — preserved gate |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS — preserved gate |
| scoped `rustfmt --edition 2021 --check --config skip_children=true` on sync/WAL/engine/hub | PASS |
| scoped `git diff --check` | PASS |
| source census | selected engine/hub cutover and one shared `Lane::Io` present; no selected blocking/eager escape found |

No Cargo, Nx, build, Wasm, browser, or runtime Rust test was run.

## Required Closure

Keep escaped snapshot-frame credit until an explicit acknowledgement/retained owner close, and deny stream production immediately on cancellation/deadline. Propagate every close error into a retained close authority and require `terminal_is_empty()` before clearing execution or releasing registry/admission. Give rejection exhaustion a typed, registry-discoverable terminal owner/job/input witness with a close path that does not depend on a permanently held shared worker. Extend the verifier with the additive field-clone, escaped-output-credit, cancel-before-stream-drive, failed-page-close, terminal-discovery, and held-pool timer-progress mutations.
