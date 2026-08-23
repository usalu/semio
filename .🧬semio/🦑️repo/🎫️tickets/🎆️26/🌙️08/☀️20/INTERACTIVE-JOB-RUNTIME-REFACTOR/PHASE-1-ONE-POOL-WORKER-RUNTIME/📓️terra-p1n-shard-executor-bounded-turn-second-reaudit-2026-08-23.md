# Terra Second Re-Audit: P1n ShardExecutor Bounded-Turn Handoff — 2026-08-23

## Verdict

**REJECT** for this narrow source-only packet.

The remediation closes the ordinary malformed/permanent-frame path: it retains
the raw owner in a fixed FIFO terminal ring, preserves its consumed ingress
epoch, closes new ingress before a later transport send, and excludes a stored
terminal owner from ordinary work readiness. The wake, bounded timer retry,
one-registration, fixed deferred-authority, and one-mixed-authority structures
also remain present.

However, the same raw-terminal guarantee fails when its new fixed terminal
ring reaches capacity. This is a feasible path because many frames can be
admitted to the native transport before the first malformed one closes ingress.
The 257th malformed or permanently-over-capacity frame is neither terminalized
nor parked behind a host-controlled retry; it remains a `rejected_frame`, keeps
its original epoch unacknowledged, and causes successor scheduling with no
source-level progress. Retrieval of one prior terminal frame does not re-arm
or otherwise make this retained owner observable for a later one-shot retry.

This is not Phase 1 acceptance and does not claim compilation or runtime
behavior.

## Scope

Read `AGENTS.md`, the Phase 1/2 readiness audit, both prior Terra P1n
rejections, the updated `📓️p1n-shard-executor-bounded-turn-handoff-2026-08-23.md`,
the live scoped diff, production source, and the direct and TypeScript
adversarial fixtures. No source, script, manifest, lock, coordinator,
checklist, or existing ticket artifact was changed. This report is the one
permitted new audit artifact. Cargo, Nx, Wasm, browser, network, and root lint
were not run.

The worktree has concurrent staged and unstaged changes outside this packet.
The exact findings below are against the live production source, not an
attribution of those unrelated changes.

## Prior Blockers and New-Path Review

| Required property                                                                           | Result                     | Independent evidence                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------------------------------------------------------------------- | -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| No executor `block_on`, receive wait, or epoch-drain loop                                   | **PASS, source structure** | Production `ShardExecutor::run` polls one retained `ShardLoop::drive_one` future and reads at most one outcome with `try_recv_now`; it contains no `block_on`, `.recv().await`, byte-drain loop, or pool-closure epoch loop. `ShardDrive::Blocked` remains an enum arm but is not manufactured by the pending branch.                                                                                        |
| At most one transport frame and one actor-turn/job-step/lifecycle authority per drive       | **PASS, source structure** | `pump_primed` receives only if no deferred owner exists, consumes at most the selected frame, then pops one interactive/background `DeferredAuthority`; every `Register`, `Unregister`, `Event`, `JobStep`, `Cancel`, `Suspend`, and `Resume` arm returns after one unit. Decoding a Grant iterates to pre-admit its envelopes only; it does not execute their lifecycle work inline.                        |
| Stale epoch before mutable shard work                                                       | **PASS, source structure** | `run` checks `admitted_epoch` before taking `state`; a completed drive compare-exchanges only its reported consumed epoch from `epoch - 1`.                                                                                                                                                                                                                                                                  |
| Generation-tagged wake without hot pending resubmit                                         | **PASS, source structure** | `ShardDriveWake` holds `Weak<ShardExecutor>` plus the drive generation. `request_drive_wake` one-shot claims the matching generation, and the pending path parks unless a raced wake has already claimed its one successor.                                                                                                                                                                                  |
| Quiet-ingress retry, finite exhaustion, and terminal successor owner                        | **PASS, source structure** | `try_submit` preserves `rejected.into_job()`. Contended/saturated retries use existing `WorkerPool::callback_at`, coalesce by generation, and terminalize after attempt eight. Shutdown/poison/exhaustion transfer one `PoolJob` through `terminal_handoff` and expose take/resume methods. No new executor thread or runtime was found.                                                                     |
| Fixed registration and deferred lifecycle ownership                                         | **PASS, source structure** | Registration and both pending lanes use `FixedOwnerRing`; `run` pops at most one registration before the one drive poll. Ring capacity is 256 items with checked byte accounting and generation-bearing `OwnerKey`; no new dynamic event/job/cancel deferred queue remains.                                                                                                                                  |
| Late/terminal ingress exact handback before mutation                                        | **PASS, source structure** | `send_frame` holds `ingress_gate`, rejects raw bytes above `SHARD_FRAME_MAX_BYTES` or closed/shutdown ingress before `kernel_side.send_now`, then only increments epoch and lane hint after a successful owned send. `TerminalFrameOwner` exposes both reason and `into_frame`.                                                                                                                              |
| Grant and every frame/envelope variant have raw item/byte preflight without whole re-encode | **PASS, source structure** | `Register`/`Unregister` reserve one background slot and the complete raw frame length. `Grant` and standalone `Envelope` count every envelope by lane and distribute the already-admitted raw frame length through `split_frame_credit`; dispatch enqueues `Event`, `JobStep`, `Cancel`, `Suspend`, and `Resume` with that credit. No production whole-envelope `pack_encode` occurs in this admission path. |
| Ordinary malformed/nested/permanent-cap raw terminalization and retrieval                   | **PASS, bounded only**     | Decode/trailing/validation faults and an empty-lane permanent preflight failure call `retain_terminal_frame`; `drive_one` reports the original consumed epoch and terminal state, `run` acknowledges it and closes ingress. `take_terminal_frame` uses `pop_front`, so a single ordinary owner is retrieved once and does not itself constitute readiness.                                                   |
| Terminal raw frame at fixed terminal-ring capacity                                          | **FAIL**                   | Detailed below. The capacity-full branch returns the exact frame as a transient `Full` rejection rather than a terminal/parked owner, so it re-enters the scheduling path with no capacity-changing work.                                                                                                                                                                                                    |

## Blocking Terminal-Capacity Sequence

`terminal_frames` is a `FixedOwnerRing<Vec<u8>, SHARD_DEFERRED_ITEMS>` with
`SHARD_DEFERRED_ITEMS = 256`. Its byte limit is finite as well. The bounded
container itself is appropriate; its full transition is not.

1. Before the first worker closure runs, callers can admit more than 256
   frames through `send_frame`; the native channel is not given a frame-count
   cap. The first malformed frame later closes new ingress, but does not remove
   already admitted FIFO frames.
2. For the first 256 malformed/nested-malformed/permanent-cap frames,
   `retain_terminal_frame` succeeds and returns `FrameAdmissionError::Fault`.
   `pump_primed` records that frame's original epoch, and `run` acknowledges it.
3. On the next such frame, `terminal_frames.try_push` fails. At
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs:925-930`,
   `retain_terminal_frame` converts that exact terminal raw frame to
   `FrameAdmissionError::Full`. At `:636-644`, `pump_primed` stores it as
   `rejected_frame` with its original epoch and does **not** set
   `last_drive_consumed_epoch`.
4. `has_pending_work` explicitly includes `rejected_frame` (`:591-593`). With
   no ordinary deferred owner left to free capacity, `drive_one` returns
   `MoreWork` without a consumed epoch. The executor sees
   `consumed_epoch < epoch` and schedules again (`executor.rs:631-634`), which
   repeats the same failed terminal push. This is the hot-resubmit condition
   the packet was required to eliminate.
5. `ShardExecutor::take_terminal_frame` only pops a prior terminal frame
   (`executor.rs:376-382`). It neither recognizes this retained raw frame nor
   schedules/re-arms one retry after space becomes available. The new direct
   fixture covers one terminal owner only; it does not fill the terminal ring
   and retrieve through this state.

The raw `Vec<u8>` is preserved during that spin, but it is not terminalized
exactly once, and its original ingress epoch is not consumed. Thus the stated
guarantee fails precisely at the fixed-cap boundary rather than being made
safe by that boundary.

## Fixture Assessment

The TypeScript adversarial self-test is meaningful as a lexical regression
tripwire: each fixture removes or corrupts a required concrete source shape
(wake claim, timer trigger, fault acknowledgement, ingress order, frame-byte
credit, lifecycle deferral, fixed bounds, generation, or single-pop behavior),
and the executed self-test rejects all of them. It cannot prove the above
cross-function capacity/liveness sequence because it never models a full
`terminal_frames` ring.

The direct Rust fixture source meaningfully asserts a single malformed raw
frame is consumed once and becomes non-ready after retrieval; it also checks
raw-credit sum/+1, Suspend/Resume item and byte handback, ring slot reuse, and
mixed FIFO pop order. Those tests were not executed under this audit boundary.
None builds the 257-frame terminal-capacity state or proves a host retrieval
causes exactly one safe retry, so they do not close the blocker.

## Required Repair and Focused Re-Audit Evidence

Before another source re-audit, the terminal-ring-full transition must retain
the exact 257th raw owner in a non-spinning, host-observable terminal handoff
that preserves its original epoch. Draining or closing a terminal owner must
make a single deliberate retry/resume possible only when capacity has changed;
it must not rely on unconstrained successor scheduling or ingress. The repair
must preserve already-admitted FIFO frames without overwriting any prior
terminal owner.

Add direct source/runtime coverage that admits 256 terminal raw frames, then a
257th malformed and a 257th permanently-over-capacity frame. It must prove:

- no repeated pool submission or epoch acknowledgement while terminal capacity
  is unavailable;
- exact original raw bytes and epoch remain owned once;
- taking/closing one terminal owner clears the relevant readiness and causes at
  most one intentionally claimed retry; and
- the re-admitted frame terminalizes once without affecting older owners.

## Executed Static Gates

All commands below exited 0:

```sh
rustfmt --edition 2021 --check '🧰️framework/🔨️modules/⏳️async/🦀️component.rs' '🧰️framework/🔨️modules/🎭️actor/🦀️component.rs' '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs' '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs'
bun ./📜️script.ts verify interactivity
bun ./📜️script.ts verify interactivity --self-test
git diff --check -- '🧰️framework/🔨️modules/⏳️async/🦀️component.rs' '🧰️framework/🔨️modules/🎭️actor/🦀️component.rs' '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs' '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs' '📜️script.ts'
git diff --check
git diff --cached --check
git diff HEAD --check
```

The interactivity verifier reported its existing expected test-only allowlist
record and no unlisted blocking bridge; it is a pass for the verifier, not
runtime evidence. Targeted production scans also found no executor
`block_on`, blocking receive, byte-drain loop, or inline multi-lifecycle loop.

## Limits and Remaining Phase 1 Blockers

No Cargo compilation, Rust test execution, native host launch, timing test,
worker/thread census, or permit-saturation/runtime recovery test was run.
Therefore the structural passes above do not establish compilability, actual
wake/timer firing, retry timing, ownership under concurrent transport races,
or one-pool runtime behavior.

Separately from this rejected P1n packet, the preceding readiness audit remains
RED for the live store-sync nested `runtime.block_on(actor.run_turn())` path in
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs` and the
MCP HTTP transport's separate `tokio::runtime::Runtime::new()` path in
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️component.rs`.
Fresh serialized native evidence for worker/thread census, permits, wake and
saturation timing, and the supported plugin-host synthetic path also remains
required. None of those separate blockers was changed or tested here.
