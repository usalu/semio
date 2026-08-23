# P1p Store-Sync Retained Actor Turn

Date: 2026-08-23

## Source Outcome

The native store-sync actor no longer enters a nested executor from a `WorkerPool` closure. The former `runtime.block_on(actor.run_turn())` path and Tokio-owned actor/connect task are removed. `ActorRunner` retains either the actor or its in-progress turn future, polls exactly once under a generation-tagged real waker, and yields the pool slot after every poll.

This is a source-only implementation packet awaiting independent audit. It is not Phase 1 acceptance.

## Rejection Remediation: Durable Quiet Runner

The 2026-08-23 Terra audit correctly found that the original retained-turn packet allowed `Idle { deadline: None }` to release the last strong `ActorRunner` owner. `spawn_actor` now returns an `ArtifactActorRunnerHandle`; the active document registry stores that durable strong handle and transfers it into the generation-keyed closing registry before requesting close. The runner also holds one private retirement owner that is released only after its terminal cursor proves the scheduled job, retry, mailbox, actor/future, and external ticket authorities empty. A quiet actor can therefore park indefinitely and a later mailbox edge can still upgrade its weak readiness registration exactly once.

`ArtifactChannels` no longer exposes another strong runner clone. It returns a shallow `ArtifactActorRunnerTicket` containing a weak, generation-tagged runner reference, an exact returned bit, and host-state lifetime ownership. The runner counts outstanding tickets; terminal-empty and the registry callback cannot succeed until each ticket is returned or dropped. Host clones are counted independently, so an external ticket can keep the closing registry reachable without suppressing last-host close. The internal self-retained owner prevents the final `Arc` from deep-dropping an actor outside the one-owner terminal cursor.

Host close now closes mailbox ingress and requests retained actor close. A retained actor drains one queued mailbox owner or one backbone owner per `drive_one` grant before terminalizing. Explicit cancellation remains available on the strong host handle. `close_step`, `terminal_is_empty`, and `take_terminal_job` expose host-controlled one-owner close and exact rejected-job resume/close; an unresolved public terminal-job owner returns itself to the runner slot on drop.

## Ownership and Scheduling

- `ArtifactMailboxSender` replaces the public Tokio unbounded actor command sender. Its fixed 64-slot FIFO and 1 MiB ledger preflight every message and nested identifier/payload byte before mutation, returning the exact rejected message as `Full`, `Bytes`, `Closed`, or `Stale` ownership.
- Mailbox readiness is edge-triggered and coalesced until the FIFO becomes empty. Registration rechecks readiness under the mailbox lock, avoiding the send/register lost-wake race.
- `ArtifactActor::drive_one` advances one command or one rotating connect, hub, connect-result, watch, reconnect, folder, backbone, or status opportunity. Store-to-actor backbone consumption uses one FIFO `try_pop_front`; no bulk `drain` remains.
- Connect and WebSocket futures are polled once with the runner waker. Reconnect and folder debounce deadlines use one generation-keyed process-pool timer-wheel callback. The previous 4 ms idle re-submission cadence is removed.
- Rejected successor closures retain their exact `Job`; transient saturation/contention has a bounded eight-attempt timer-wheel retry, while shutdown, poison, exhaustion, cancellation, and turn panic transfer ownership to the terminal authority. Terminal mailbox and actor/future owners close one per grant, and the rejected terminal job has explicit take/resume ownership paths.
- The owned file watcher now delivers one completed snapshot per poll, uses nonblocking fixed-channel handoff, and wakes the actor on completion/deadline instead of requiring an actor idle poll.

## Coordinated Call-Site Cutover

- Store worker command fields use `ArtifactMailboxSender`.
- The paused renderer Shell's `cmd_tx` field/import received only the authorized mechanical sender-type migration; its send call shapes are unchanged.
- P8's store-owned `ChannelBackboneRemote::try_pop_front` is the one-owner backbone API consumed here; the legacy bulk `drain` API is absent.

## Direct Source Fixtures

Fixtures cover mailbox item cap/+1 with FIFO handback, byte cap/+1 before mutation, nested presence identifiers/collections, wake storms, stale late ingress, one-owner interrupted close, one backbone owner per opportunity, stale wake generation, turn panic/cancel terminalization, quiet pool saturation with retained timer-wheel retry, idle-then-late-send wakeup, quiet self-retention, external ticket held across close, ticket return before and after close, strong-handle drop prevention, pending detach, terminal-job take/resume/close, host-close registry lifetime, and generation ABA isolation.

The interactivity verifier now rejects production store-sync `block_on`, Tokio spawn/unbounded actor mailbox ownership, command/backbone drains, missing byte admission, non-coalesced wakes, stranded saturation successors, missing terminal retrieval, multi-owner terminal close, the former 4 ms idle cadence, a missing strong host handle, missing self-retirement, a strong external channel handle, ticket-insensitive terminal completion, and a missing host terminal callback. Its adversarial mutations run on every interactivity verification.

## Permitted Verification

| Check | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on store-sync | PASS |
| `bun ./📜️script.ts verify interactivity --self-test` | PASS; deny mode clean, one existing allowlisted process-entry finding |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS; 142 adversarial self-tests clean |
| `bun ./📜️script.ts verify interactivity tool-jobs` | EXPECTED BASELINE FAIL; 884 repository-wide command registrations and the existing P3/P8 ownership gates remain open, with no store-sync-specific failure in the emitted list |
| Production source scans for store-sync `block_on`, Tokio actor spawn/unbounded sender, backbone drain, and event drain | PASS; only two `tokio::spawn` matches are inside the existing native test-only mock hub |
| Scoped `git diff --check` | PASS |
| Whole-worktree `git diff --check` | PASS |

Cargo, Nx, Wasm, browser, network, and root lint commands were intentionally not run under this packet's constraints. Runtime behavior is therefore not claimed.

## Remaining Phase 1 Evidence Blockers

P1n ShardExecutor and P1o MCP transport are independently accepted source-only and were kept stable. This P1p packet still requires independent source audit. Even if accepted, the Phase 1 runtime matrix remains open: there is no current permitted debug/release capture proving process thread census, pool saturation and permit balance/over-release behavior, cancellation latency below the gate, or end-to-end actor ordering under interruption. The separately identified database `run_blocking_op` I/O boundary also still needs explicit latency and ownership validation before a broader nonblocking runtime claim.
