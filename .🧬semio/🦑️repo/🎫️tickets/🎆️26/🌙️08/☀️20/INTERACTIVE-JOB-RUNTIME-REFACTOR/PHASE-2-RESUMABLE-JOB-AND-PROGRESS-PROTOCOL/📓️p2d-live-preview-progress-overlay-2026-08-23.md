# P2d Live Preview and Progress Overlay — 2026-08-23

## Pre-edit caller and reachability census

This census was written before production edits. It narrows the packet to the live native WGPU
`ShardOutcome::Job` path identified by `📓️p2-current-status-gap-audit-2026-08-23.md`.

### Exact source reachability

| Surface | Pre-edit census | Consequence |
|---|---:|---|
| Production `ShardOutcome::Job` construction | 1 | `plugin/host/shard/component.rs` creates one `JobPublication` after one retained shard job opportunity. |
| Production WGPU `ShardOutcome::Job` match arms | 0 | `kernel_runtime::KernelPoolState::run_turn` matches `Turn` and `Fault`, then discards `Job` in `_ => {}`. |
| Test/codec `ShardOutcome::Job` references | 5 | Shard tests and codec fixtures prove encoding and producer behavior but not live host consumption. |
| Production `JobTurnBridge` callers | 0 | The generic actor bridge remains test-only and is outside this packet. |
| Production `JobReplayLog` callers | 0 | The dynamic replay record remains outside this packet. |
| External `ProgressEvent` publishers | 0 | Progress vocabulary is not mounted before this packet. |
| External actor `SceneStore` apply/commit callers | 0 | The committed actor store is not the WGPU host's current UI route and must remain separate from previews. |

### Live route before the cut

```text
ShardLoop::drive_one
  -> ShardOutcome::Job { actor, JobPublication }
  -> OutcomeSink / ParallelRuntime::wait_for_outcomes
  -> KernelPoolState::run_turn
  -> `_ => {}`
  -> owner dropped without preview publication, progress ACK, or actor freshness validation
```

`KernelPoolState::run_turn` is executed by the retained kernel task on the process WorkerPool, not by
the WGPU/UI callback. It is therefore the narrow worker-owned mounting point. The scale-bench `Env`
has separate outcome loops and is not part of the live application route or this packet.

### Planned owned cut

- Add a schema-first fixed actor authority keyed by actor, job, operation, base revision, generation,
  step sequence, and preview sequence.
- Preallocate fixed slots and retirement slots; reject cap/+1 without taking the publication owner.
- Coalesce only a strictly next preview for the same exact operation identity. Move the displaced
  owner to fixed bounded retirement before adopting the new owner.
- Keep preview/progress records distinct from committed `SceneStore` snapshots.
- Mount one owner-moving `ShardOutcome::Job` arm in live WGPU `run_turn`; retain rejected/stale/fault
  owners for bounded close and surface an explicit ACK/abort status.
- Begin actor/realm retirement on app destruction and release at most one owner per maintenance turn.
- Do not edit `run_to_completion`, `run_on_worker`, `WorkerJobSession`, `JobTurnBridge`,
  `JobReplayLog`, or shard one-poll scheduling.

## Implementation and verification

### Source cut

The packet is source-complete and ready for a non-author source audit. It is not runtime-accepted.

#### Actor-owned authority

`🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` now owns the
`JobProgressOverlayStore` taxonomy:

- `JobProgressIdentity` is the exact actor/job/operation/base-revision/generation/step-sequence/
  preview-sequence key.
- `JobProgressLiveAuthority` is admitted separately from output ownership. A first publication
  without an already-admitted live operation is rejected `Stale`; a publication never certifies
  itself.
- The active registry is `[JobProgressSlot; 64]`; the retirement registry is
  `[JobProgressRetirementSlot; 128]`. Neither registry hashes, reallocates, or grows.
- A single publication page is capped at 16 KiB. Simultaneously owned payload authority is capped at
  512 items and 4 MiB with checked addition before the publication crosses into the overlay.
- `preflight` checks StepContext cancellation/fuel, realm closure, operation, base revision,
  generation, contiguous step sequence, and deterministic preview sequence before reserving the
  active and retirement slots.
- `publish_reserved` moves the exact publication owner. Preview replaces only the same exact live
  operation, with the displaced preview already reserved into bounded retirement. Checkpoints,
  commit candidates, faults, cancellation, and yield remain distinct from committed scene state.
- `JobProgressRejected` returns the exact rejected publication. `take` returns a checked-out
  preview, and `JobProgressCheckout::Drop` restores that same owner and epoch to the original slot.
- ACK clears only the matching epoch/identity. Abort moves the exact staged preview into its
  pre-reserved retirement slot and restores the prior preview/identity from the inverse handback
  authority. Actor/all close releases at most one nested owner or one scalar per StepContext grant
  and exposes `terminal_is_empty`.

The owned schema metadata and generated TypeScript projection add `JobProgressIdentity`,
`JobProgressKind`, and `JobProgressReceipt`.

#### Live WGPU consumer

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
now gives `KernelPoolState` the fixed overlay and a fixed 64-slot rejected-owner handback registry.

- An accepted `Payload::JobStep` establishes independent operation/base/generation authority before
  `ParallelRuntime::submit`.
- The live `for outcome in outcomes` match has an owned `ShardOutcome::Job` arm. It creates one
  bounded StepContext, preflights, publishes, then ACKs or aborts. It no longer falls through `_`.
- Stale, malformed, saturated, or missing-authority publications move into bounded retirement;
  their payload is not recreated from sender input.
- `Payload::Cancel`, app destruction, and each extension removed by cascade destruction begin actor
  overlay close. The existing retained kernel task pumps one cleanup opportunity before accepting
  another request.
- The shard producer and its one-poll-per-pump implementation were not edited.

There is deliberately no trust-on-first-publication fallback. If no product path first submits a
`JobStep`, a `Job` outcome is retained as stale rather than being allowed to define its own expected
revision/generation. Exposing an additional product job-submit API is outside this narrowly scoped
consumer packet.

### Permanent source fixtures and mutations

Five actor fixtures cover:

1. exact preview pointer identity plus checked-out Drop handback;
2. 64/+1 admission and slot-generation reuse;
3. exact 16 KiB/+1, base mismatch, out-of-order sequence, and cancellation owner preservation;
4. commit freshness plus two-grant nested-owner/shell retirement; and
5. deterministic coalescing replay for identical publication streams.

`📜️script.ts` adds `toolJobActorProgressOverlayExact` to the production tool-job gate and ten
discriminating mutations. They reject resizable active storage, missing generation validation,
discarded rejected owners, missing checkout handback, abort that loses last-valid, missing pre-submit
live authority, wildcard-ignored Job outcomes, missing ACK, missing realm close, and missing cancel
close.

### Files changed

- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`
- `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🟦️actor.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- `📜️script.ts`
- this report and the two deterministic JSON ledgers beside it

No `JobTurnBridge`, `JobReplayLog`, batch adapter, plugin shard source, Cargo manifest, or P3 source
outside the required WGPU glue consumer was edited by this packet.

### Gates run

| Gate | Result |
|---|---|
| `rustfmt --edition 2021 --check` on actor and WGPU glue | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS, 309 |
| `bun ./📜️script.ts verify interactivity --plain --deny` | final rerun RED solely on concurrent P1 database capability-open owner-republication predicate; an earlier stable-tree run was DENY clean and no finding names this P2 packet |
| two JSON tool-job verifier runs | expected RED, byte-identical |
| deterministic ledger census | 50 hosts, 50 invocations, 775 rows, 0 admitted, 884 residual, 8 framework reserved, 35 importers, 35 globals, 18 failure classes |
| scoped working diff check / whole `git diff HEAD --check` | PASS / PASS |
| staged-only diff check | RED only because a concurrent stage captured the pre-repair EOF blank lines in this report and `p2-current-status-gap-audit`; working/HEAD content is clean and this packet did not modify the index |

Evidence:

- `p2d-tool-jobs-a.json`
- `p2d-tool-jobs-b.json`

### Honest verdict

- **This P2d source packet: AUDIT-READY, not accepted.**
- **P2b overall: RED.** The fixed live consumer exists, but no Cargo/native/Wasm/browser/runtime
  validation was authorized, and broader product progress presentation remains outside this cut.
- **P2a/P2c: unchanged RED.** This packet did not modify the generic terminal-owner gaps or dynamic
  replay infrastructure identified by the read-only audit.
- **Phase 2 and Phase 8: RED.** The command roster remains intentionally fail closed at 0/884.

## Four-blocker mounted-route repair checkpoint

This section supersedes the earlier source-cut description where it discusses `Payload::JobStep`,
immediate ACK, fault close, or realm close. The packet remains source-audit-ready rather than
accepted because builds and runtime gates were prohibited.

### Independent authority on the autonomous shard lifecycle

The actual `Effect::SpawnJob` path now mints and stores a `JobTurn` before calling
`GuestRuntime::start_job`. The retained authority contains the shard-assigned operation, the
pre-existing base revision, the actor generation, and initial step/preview sequences. Every
autonomous job opportunity looks up that retained value and emits it by value in
`ShardOutcome::Job { actor, authority, publication }`; completion, failure, cancellation, runtime
error, and unregister remove it. WGPU `run_turn` receives the by-value authority and checks every
identity field against the publication before admitting it into `JobProgressOverlayStore`. The
arriving publication can no longer define its own expected operation/base/generation.

### Prepared overlay and presenter ACK

WGPU owns a fixed 64-slot generation-tagged presentation bridge. `publish_job_progress` reserves a
bridge slot before publishing, and preserves both the exact overlay receipt and bridge token in a
fixed `PendingJobProgressPresentation` slot. It does not ACK the overlay.

Frame construction checks out at most one exact presentation lease, writes a bounded solid progress
overlay into the worker-built `RenderSnapshot`, and transports the same lease through
`AppFrameBuild`, preparation, and presentation. After `PreparedRenderGate` acknowledges actual
presenter adoption, `AppPresentPhase::ProgressAcknowledge` submits the matching generation-tagged
token back to the kernel. Queue backpressure keeps the exact lease checked out for retry. Only the
kernel token handler changes the bridge to Presented, ACKs the exact overlay receipt, and releases
the fixed bridge slot. Missing, duplicate, or stale tokens fail closed. Drop before adoption returns
the exact checkout to Ready rather than ACKing it.

### Fault, app, and realm close

- `ShardOutcome::Fault` records a fixed fault-close actor and drives the same retained store close
  instead of leaving an active preview behind.
- app/extension destruction owns a fixed retained close record, cancels one matching pending
  presentation, aborts its exact receipt, begins one actor close, unregisters one actor, and waits
  for the actor terminal witness on separate turns;
- `KernelClient::begin_close_realm` installs a retained `CloseRealm` request. `OsHostRetirement`
  polls that handle before releasing runtime/presenter ownership;
- realm close cancels one pending presentation per turn, calls `begin_close_all` once, pumps one
  overlay retirement opportunity per turn, and requires the overlay store, rejected-owner slots,
  and every fixed presentation-bridge slot to be terminal-empty before completing.

The bridge terminal witness was made explicit; an orphan Reserved, Ready, CheckedOut, or Presented
slot prevents the realm witness rather than being implicitly released.

### Mounted boundary fixtures and verifier mutations

The actor fixture now reaches the simultaneous fixed boundaries, not just the per-page boundary:

- 64 active and 128 retirement slots;
- 512 owned/reserved aggregate items and item +1 rejection;
- 4 MiB owned/reserved aggregate bytes and byte +1 rejection;
- exact rejected identity/payload pointer preservation; and
- bounded terminal close with a worst-case grant ceiling that includes active-to-retirement moves,
  nested page/shell retirement, active-slot release, and the close-all scalar.

The WGPU presentation fixture fills all 64 bridge slots, proves +1 rejection, FIFO checkout, exact
generation/token matching, duplicate release rejection, and the final terminal-empty bridge
witness. The shard fixture exercises consecutive autonomous publications and asserts the retained
authority stays identical while publication sequences advance.

`toolJobActorProgressOverlayExact` now requires the mounted shard authority insertion before
`start_job`, by-value WGPU outcome handoff, absence of ACK in `publish_job_progress`, the prepared
presentation lease and post-adoption ACK phase, fault close, realm `begin_close_all`, and the bridge
terminal witness. Its mutations remove the autonomous authority, aggregate-cap fixture,
post-adoption phase, realm close, and presentation terminal witness independently.

### Repair gates run

| Gate | Result |
|---|---|
| Rust-2021 `rustfmt --check` on actor, shard, WGPU glue, frame job, and OS host | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS, 309 |
| `bun ./📜️script.ts verify interactivity --plain --deny` | PASS; DENY clean with one recorded allowlisted blocking bridge |
| plain tool-job coverage | expected global RED: 50 hosts, 50 invocations, 775 rows, 0 admitted, 884 residual, 309 self-tests |
| scoped and whole working-tree `git diff --check` | PASS |
| production source scans for autonomous authority, exact consumer lease, post-adoption ACK, fault close, and realm witness | PASS |

No Cargo, Nx, Wasm, browser, runtime, network, or root-lint gate was run. The source packet is ready
for independent audit, not accepted. Phase 2 remains RED until serialized build and mounted runtime
evidence are authorized and pass.

## Cancelled-head presentation liveness repair

The coordinator's final source audit found that the original fixed bridge inspected only one
`take_cursor` slot. Cancelling the oldest Ready token made that slot Vacant and could strand every
later Ready owner forever.

The bridge now assigns each reservation a monotonic, non-wrapping `admission_sequence`. `take`
performs one fixed 64-slot, allocation-free scan and checks out the Ready slot with the smallest
sequence. Vacant cancellation holes are skipped, while a returned checkout retains its original
sequence and therefore regains its exact admitted position. Reservation likewise performs a fixed
scan from `reserve_cursor`, so a vacant hole is reusable rather than producing false saturation.
Slot epochs now use checked increment instead of wrapping; an exhausted slot cannot reintroduce an
ABA token.

`cancelled_head_does_not_strand_later_ready_presentations` admits three ordered tokens, cancels the
Ready head, proves the middle token is delivered first, returns that checkout and proves it retains
priority, then delivers the tail and reaches the exact terminal-empty witness. The existing full
64/+1 FIFO, generation, duplicate-release, retry, and realm-terminal semantics remain intact.

The permanent predicate now requires the fixed admission sequence, hole-safe
`oldest_ready_index`, and cancelled-head fixture. A mutation that restores the single-cursor take
necessarily fails the predicate.

### Liveness repair gates

| Gate | Result |
|---|---|
| Rust-2021 `rustfmt --check` on WGPU glue | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS, 309 |
| `bun ./📜️script.ts verify interactivity --plain --deny` | PASS; DENY clean with one recorded allowlisted bridge |
| exact source scan for removed `take_cursor`, sequence selector, fixture, and mutation | PASS |
| scoped and whole `git diff --check` | PASS |

No Cargo, Nx, Wasm, browser, runtime, network, or root-lint gate was run. This focused repair is
source-audit-ready, not independently accepted; Phase 2 remains RED pending runtime evidence.
