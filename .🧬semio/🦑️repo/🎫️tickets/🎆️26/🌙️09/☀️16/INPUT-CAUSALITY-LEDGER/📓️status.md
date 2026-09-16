# 🎯️ Input Causality Ledger — status

Ticket opened 2026-09-16 by session ⚪37cccb68 (Opus 5). Start commit: `🗑️generated/start-commit.txt`.
Origin: fem2d "swallowed click after marquee" report; investigation refuted the busy-drop hypothesis on the
direct-actor path and found the real gates (see `📓️design-input-causality-ledger.md` §0).

## Definition of done
1. Laws L1–L7 of the design hold, with pure law tests for the ledger, CAS registers, sample batching, refusal surfacing, causal dequeue and the browser-actor queue.
2. The 8 s `setActiveUtility` echo-off gate and the `pointerleave`→`pointerUp` forgery are deleted; no `performance.now()` remains in `onAction` semantics.
3. Interaction verbs emitted by a guest fold in-reactor (one round trip per pick/hover/marquee, no host re-dispatch).
4. Acceptance probe (design §6) green on the fem2d React lane and on one hub-mounted lane.
5. Crates green natively + wasm32-wasip2; existing laws green.

## Log
- 2026-09-16 investigation + design written (phases 1–4). No code changed yet.
- 2026-09-16 evening — implementation wave 1 (8 agents, disjoint files) landed, all foreground-verified:
  - contract module `🏛️ShellHost/🎯️input-ledger/🟦️.ts` (ledger, causal key, CAS register, refusal vocabulary + de/en notices, gesture sample lane) + `🧪️tests/🎯️input-ledger` 35 laws, `input-ledger-check` script;
  - `📬️mailbox`/`🧵️turn-scheduler`: optional `order` key (ordered-only reordering rule), 12 laws;
  - runtime plumbing: `PluginDispatchHintV1 { order }` from `handleAction/handleCommand` → `AppChannelClient.command` → `runQueuedTurn` → `serializeCommandIngressForActor(…, order)`; wgpu bridge twin with a bounded ordered pending list; engine-contract + wgpu laws;
  - browser-actor transport: `BrowserActorActionMailboxV1` is a bounded FIFO (cap 32, never throws synchronously, timer starts at send, `browserActorActionRefusalReasonV1`); worker `dispatchAction` queues instead of `action-busy`, awaits UI-patch/view-refresh quiescence instead of `action-owner-mismatch`, accepts the painted revision; backbone-envelope-io laws;
  - reactor: guest-emitted `ReplayShellCommand{interaction verb}` folds inline in the same turn (`apply_interaction_verb`, `dispatch_emit` wrapper + typed-operation ladder unit), 4 Rust laws; `DispatchAction{interaction verb}` fold + guest test flips in flight (wave 2);
  - guests: fem2d/draw/layout/wires/vcs/generation2d `CanvasPointerMove.samples`, `CanvasPointerUp.cancelled`, laws per crate;
  - ShellHost: `onAction` → `Promise<InputOutcomeV1>`, every former silent gate is a typed refusal (plain console line + throttled notice), CAS utility/tool registers with `expectedGeneration` stamped by UtilityTree/ShellHelpers, 8 s echo-off DELETED, follow-ups carry `causedBy`/`order`; Canvas2dHost drives `createGestureSampleLaneV1` (`samples`, `cancelled`), react-renderer maps pointerleave/pointercancel/lostpointercapture to a cancel; UtilityTree picker deactivates only on an explicit press.
- Battery `🐍️input-battery.mjs` on the live `:6086` lane (old wasm, new host): PASS — marquee-click 6/6 cleared with exactly 2 sends settling after mouseup (drain 129–196 ms steady state; before: 1–3 s), utility-toggle deactivates at 1.5 s (0 echo-off lines), utility-switch 2 hops, refusal-visible: 337 issued / 337 applied / 0 refused. Reports: `🗑️generated/battery-host-only-3/`.
- Blocked → unblocked: first wasm activation failed on a peer's mid-refactor fem3d (`MeshedSolid`); recompiled ~1 h later, activation re-run (`🗑️generated/activate-2.log`).
