# Plugin ACK, Runnable, and Maintenance Audit

Status: **source partial PASS; production/lifecycle RED**. The original false-terminal maintenance branch is repaired in the current bytes. The fixed 64-slot stage-0 fairness and callback-execution proof remain RED. This is a current-source audit only; no build or runtime command was run.

## Scope

Audited the current retained typed-operation state machine, runtime continuation selector, live-maintenance worker, shell ACK route, bounded close route, and the newly staged Child runtime law.

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs`

The older `📓️terra-typed-result-ack-watchdog-audit.md` describes the removed poll-count retransmission behavior. This report supersedes it for the current bytes.

## What the current change gets right

`queue_page` installs exactly one retained page, marks it unpresented, and enters `AwaitingAck`. `take_result_page` now returns a clone once, marks it presented, and returns `None` until an explicit successful ACK clears the page. `acknowledge_result_page` verifies the complete token and the awaiting stage before it clears the page, increments the sequence, and selects Publishing or Retiring. That eliminates the prior poll-count mutation of `attempt` and does not redeliver a presented page.

- State transition and one-page invariant: `plugin/🦀️.rs:16332-16339`.
- Single-shot delivery: `plugin/🦀️.rs:16366-16372`.
- Full-token ACK and Child/Store ownership checks: `plugin/🦀️.rs:16381-16403`.

The new predicate correctly separates retained ownership from continuation work: Worker, Publishing, and Retiring are runnable; an AwaitingAck operation is runnable only until its page has been presented. The runtime instance selector uses this predicate rather than the broader pending predicate.

- Predicate: `plugin/🦀️.rs:16374-16379`.
- App-level aggregation: `plugin/🦀️.rs:24598-24609`.
- Bounded cross-instance selection: `plugin/🦀️.rs:31313-31345`.

This is sufficient source evidence for no immediate output-spin from a *presented* page. It is not evidence that a lost ACK is recoverable: there is no result-page deadline, retry request, or explicit transport-loss cancellation in the current path. A lost ACK retains one of the 64 operation slots until exact ACK, app close, or another existing cancellation path. That may be an acceptable nonclaim for this packet, but it is not end-to-end delivery liveness.

## Repair re-audit: false terminalization is removed

The prior decisive defect is **source-repaired**. Stage 0 now returns `Pending { 0, 0 }` for `Publishing` and the typed `AwaitingInput` outcome for `AwaitingAck`; only a genuine `Retiring` `Complete` reaches the exact terminal-empty/remove branch.

```rust
Retiring    => operation.retirement_step(...),
Worker      => operation.drive_worker_step(&pool)?,
Publishing  => Pending { released_items: 0, released_bytes: 0 },
AwaitingAck => AwaitingInput { ... },
```

at `plugin/🦀️.rs:24199-24214`. `RuntimeLiveCleanupJob` yields for `AwaitingInput` (`:29472-29475`) and the live watchdog resets its structural credit only for that typed category (`:29508-29526`). App close separately clears a retained result page and moves it to `Retiring` (`:23666-23690`); the runtime close path treats an unexpected `AwaitingInput` as a fault (`:29714-29716`). The original normal delayed-ACK terminal-removal path is therefore absent in current source.

This is not a native pass. The result page remains single-shot and the accepted ACK still verifies the full token before it clears the owner (`:16368-16405`).

## Fairness, close ownership, and watchdog classification

### Fairness: still RED at stage 0

The continuation selector is bounded and round-robin, and `has_runnable_work` correctly excludes a *presented* ACK page (`plugin/🦀️.rs:16376-16380`). Stage 0 itself is not yet a scan: it calls `next_id_from(cursor)` once, advances the cursor, and immediately returns the selected operation's outcome (`:24186-24214`). `next_id_from` is a first-occupied-slot lookup, not an eligible-stage lookup (`:15633-15640`).

At the fixed capacity of 64, 63 presented `AwaitingAck` owners before one `Worker`/`Retiring` owner make stage 0 return `AwaitingInput` 63 times before it reaches the executable owner. Since stage 0 is visited once per 21 maintenance stages (`:24183-24185`), the executable owner is delayed for up to 1,324 `maintenance_step` invocations (bounded by 64 × 21 = 1,344). This is eventual round-robin, but it is not the required work-first maintenance fairness.

It also weakens the intended permanent-Blocked watchdog. Every encountered ACK waiter resets the app-global `maintenance_stalled_steps` (`:29512-29514`), so an otherwise permanently blocked Worker can be repeatedly exempted merely because unrelated result pages await renderer input. The existing 256-credit unit test proves only an isolated `Blocked` value, not this mixed registry state.

### Close ownership: source-qualified PASS

Normal app close does not require renderer ACK. For AwaitingAck it first checks the fixed page byte grant, clears the app-owned page, enters Retiring, and then retires pending store/child-publication owners with terminal-empty checks:

- AwaitingAck close handoff: `plugin/🦀️.rs:23666-23690`.
- Child/store retained-owner retirement: `plugin/🦀️.rs:16422-16449`.
- Terminal witness includes page, publications, captured child root, input, completion, and cancellation lease: `plugin/🦀️.rs:16555-16569`.

The shell has a cloned delivery buffer, so clearing the app's retained page on instance close is not an ownership leak. A late ACK then fails harmlessly because the operation is gone. This is source evidence only; no delayed-ACK close law has executed.

### Permanent Blocked watchdog: category is now correct, selection still is not

The watchdog correctly charges a real zero-release `Pending` or `Blocked`, faults on the 256th consecutive structural stall, and resets on actual progress (`plugin/🦀️.rs:29508-29526`). Its existing law pins a permanently blocked live cleanup owner to that behavior (`plugin/🦀️.rs:29975-29982`). Keep that law and semantic category unchanged.

An ACK wait is neither retirement progress nor a permanently blocked owner. The new typed `AwaitingInput` category is the correct distinction; it must only be emitted once the bounded stage-0 scan has established that no `Worker` or `Retiring` owner can be driven. Emitting it for the first occupied slot is what currently masks a live blocked owner.

## Smallest remaining production correction

Keep the current `PluginCloseStep::AwaitingInput`; no new queue, status, or compatibility path is required. Change only stage 0 at `plugin/🦀️.rs:24186-24214`:

1. Iterate exactly `0..ARTIFACT_LIVE_OUTPUT_SLOTS` from `maintenance_tool_cursor` using the registry's slot/id lookup. Inspect each occupied operation without taking it.
2. Select the first `Worker` or `Retiring` operation, then advance the cursor to that slot plus one and drive exactly that owner. This preserves round-robin among maintenance-executable owners.
3. Track whether the scan saw `Publishing` or an unpresented `AwaitingAck` (`has_runnable_work()`); if no worker/retirer exists, return zero-release `Pending`, not `AwaitingInput`, because local continuation can still act.
4. Return `AwaitingInput` only if every retained typed operation is a *presented* `AwaitingAck`. With no operation return ordinary zero-release `Pending`.

The scan is fixed 64 slots, allocation-free, and uses existing registry authority. Crucially, it does not reset the watchdog while a Worker/Retiring owner is present: an actual `Blocked` result then remains charged by the existing 256-credit mechanism. `Publishing` remains an internal no-release condition, not an external-input wait.

## Missing proof rows

Extend the existing neutral `tool-latest-wins` ACK fixture rather than treating a private state test as transport proof. The current fixture pins 512 pre-ACK polls, one delivery, and attempt 1 (`plugin/🥇️tool-latest-wins.json:114-118`), but its Rust state test only repeats `take_result_page` (`plugin/🦀️.rs:17610-17628`) and does not execute runtime maintenance.

Required native laws:

1. **Delayed shell ACK ordering.** Route the first Child page through `renderer_exchange_bytes`, run a real next reactor poll carrying its `Event::Message` ACK, and assert cleanup neither faults nor removes it before event processing. Then observe Child ACK, Terminal delivery, Terminal ACK, and terminal-empty close.
2. **Actual callback, not scheduling-only.** The current 512 loop (`plugin/🦀️.rs:34174-34180`) calls `plugin_step_live_cleanup`, whose `true` means a job was submitted or was already queued/running (`:30174-30213`), and then calls a self-waking local future (`:15873-15885`). `maintenance_generation` increments before `try_submit` (`:30181-30196`), so neither proves the worker callback ran. Add a `#[cfg(test)] AtomicU64` counter to `RuntimeAppCell`, incremented only immediately before `RuntimeLiveCleanupJob::step` calls `instance.app.maintenance_step` (`:29456-29457`). Rename the neutral fixture field to `preAckMaintenanceSteps` (512), and make the native law drive the real scheduler until that counter advances by exactly 512 under a fixed outer deadline. At each observed step assert: status is not FAULT, the same Child token/page remains retained and presented, there is no duplicate delivery, and the structural-stall counter remains zero. This is an execution witness; do not use `maintenance_generation` as one.
3. **Mixed fairness.** Fill 63 slots with presented ACK waiters and put one real Worker or Retiring owner in the remaining slot. Drive enough real maintenance steps for one bounded scan (64 stage-0 selections) and assert B is driven before `64 × 21` app maintenance stages, while each A page/token/publication is unchanged. A companion all-ACK fixture must emit `AwaitingInput` and retain zero stall credit.
4. **Permanent block remains fatal.** In the same 63-waiter/one-Worker topology, make the selected Worker return `Blocked`. Assert the stage-0 scan selects it rather than returning `AwaitingInput`; then preserve the existing 256-credit fatal law. This is the missing proof that unrelated renderer waits cannot exempt structural deadlock.
5. **Close and hostile ACK.** Close with a presented Child page under a short page grant then exact grant; assert no direct Drop and terminal-empty. Add duplicate, cross-instance, generation/sequence-mismatched, and post-close ACK rows; none may wake, clear, or revive an owner.
6. **Explicit lost-ACK policy.** Either add a bounded host retry/deadline/cancel law, or state and test the limited contract: no retry, retained slot until app close/cancellation, and no false liveness claim.

## Acceptance boundary

Single-shot delivery, the `AwaitingInput` split, and the close distinction are source-correct in isolation. The original false terminal/removal diagnosis is withdrawn for the repaired current bytes. However, stage-0's first-occupied selection can mask a real blocked owner for up to a full 64-slot rotation, and the 512-poll law proves scheduling only, not callback execution. No runtime/lifecycle acceptance is justified until the bounded eligible scan and callback-witness native laws run green. This audit does not claim browser rendering, socket delivery, child-publication atomicity, or recovery after a lost shell ACK.
