# P1p Store-Sync Retained Actor-Turn Audit — 2026-08-23

## Verdict

**REJECT — source packet only.** The replacement removes the nested native `block_on` path structurally, but the live native actor has no durable owner when idle. It can be dropped while the document remains open, so later exact-admitted mailbox messages are never driven. This is a P1p blocker; it is not a Phase 1 verdict.

## Blocking Current-Source Evidence

`native_actor::spawn_actor` constructs `Arc<ActorRunner>` and returns `()`; its registered mailbox/readiness callbacks capture only `Weak<ActorRunner>`. `ArtifactHost::open` then stores an `OpenDocument` containing only `cmd_tx`, `events`, and `presence`—not the runner, a runner handle, or a terminal-owner handle.

The normal no-work path is reachable without any timing assumption:

1. `ArtifactActor::drive_one` rotates through its phases and its `Status` phase returns `ArtifactDrive::Idle { deadline: [reconnect_at, fs_deadline].min() }`.
2. With no hub reconnect or folder debounce deadline, that deadline is `None`.
3. `ActorRunner::run_job` retains the actor, clears `scheduled`, and calls `arm_deadline(None)`, which deliberately arms no callback.
4. The running submitted closure returns. Its strong `Arc<ActorRunner>` is dropped; the actor's readiness callback and the mailbox wake callback each retain only the weak reference. No strong owner remains in `ArtifactHost`.
5. A later `ArtifactMailboxSender::send` admits a message and invokes its wake callback, but `weak.upgrade()` fails. The message stays in the fixed mailbox until capacity is exhausted; no actor exists to consume it.

This defeats the required retained, wake-driven actor semantics and makes the removed idle-poll cadence a message-loss/stall path. The direct fixtures cover wake coalescing and runner units while a local `Arc<ActorRunner>` is still held; they do not cover open → quiet idle → later host/session/worker/Shell send.

The same missing durable handle means `ActorRunner::take_terminal_job` and `resume_terminal_job` are private and have no production caller. A rejected pool closure can be retained internally, but `ArtifactHost` cannot retrieve or close/resume that exact terminal owner. The terminal fixture reaches the private method directly and does not prove the host path.

## Structural Checks That Did Pass

- The native production actor no longer contains `runtime.block_on`, `semio_framework_async::block_on`, `tokio::spawn`, a Tokio runtime builder, `UnboundedSender<ArtifactActorMsg>`, `unbounded_channel::<ArtifactActorMsg>`, `self.remote.drain()`, or a status `while` drain.
- `ArtifactMailboxSender` has a 64-slot array and 1 MiB checked ledger. `artifact_actor_message_bytes` walks local mutation fields/dependencies/payloads, nested presence identifiers/collections/views/UI, and preview payloads before mutation; its error owns the original message.
- `drive_one` takes at most one mailbox command or one rotating connect/hub/connect-result/watch/reconnect/folder/backbone/status opportunity, and the native backbone uses one `try_pop_front`.
- The one-poll runner uses a generation-tagged `Wake`, coalesced `scheduled`/`wake_requested` state, a one-millisecond generation-keyed timer-wheel retry with an eight-attempt bound, and one-at-a-time mailbox/turn terminal closure in its local implementation.
- The wasm command receiver has been migrated to `ArtifactMailboxReceiver`. Its separate unbounded `Vec<u8>` WebSocket ingress remains a production wasm queue, but it is not the removed `ArtifactActorMsg` command sender; no runtime behavior was executed in this audit.

Those facts do not close the idle lifetime or terminal reachability defects above.

## Required Repair Boundary

Keep a strong, owned runner/terminal handle for the full `OpenDocument` lifetime. `ArtifactHost::close` must direct that handle into the existing one-owner terminal progression, and the handle must expose exact terminal-job take/close or resumption ownership. The callback may remain weak only after a separate strong host owner guarantees the runner lives until terminal closure. Add an adversarial source/runtime fixture for a quiescent open document followed by a later sender wake, and for host-visible terminal rejected-job retrieval. No unrelated actor, runtime, dependency, or Phase 1 work is required by this repair.

## Gates Run

| Command | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` over store-sync, file watcher, store worker, Shell, store, and native-I/O Rust paths | PASS |
| `bun ./📜️script.ts verify interactivity --self-test` | PASS; deny mode clean |
| `bun ./📜️script.ts verify interactivity` | PASS; deny mode clean |
| Production-path scans for nested block, Tokio actor/runtime, unbounded actor command sender, bulk backbone/status drain, and the legacy idle cadence | PASS for those textual patterns |
| `git diff --check` | PASS |
| `git diff --cached --check` | PASS |
| `git diff HEAD --check` | PASS |

The verifier reports one pre-existing allowlisted blocking-bridge census item outside this packet. Its P1p rules are useful regressions for listed substitutions, but they do not reason across `spawn_actor`, weak captures, `OpenDocument`, and the host's later send; their green result does not contradict this rejection.

## Deliberate Limits and Remaining Phase 1 Evidence

Cargo compilation, Rust fixtures, WorkerPool scheduling/race execution, DB/folder/hub I/O, browser/Wasm, network, Nx, and root lint were not run by instruction. The report therefore makes no compile, timing, I/O, or runtime security claim.

After the source blocker is repaired, the wider Phase 1 gate still needs serialized runtime evidence: process thread census, interactive admission/over-release behavior, quiet-pool retry delivery, cancellation timing, and end-to-end actor ordering under interruption. P1n and P1o's previously accepted source-only packets remain outside this rejection.
