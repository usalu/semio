# 🎯️ Input Causality Ledger — design

Author: session ⚪37cccb68, 2026-09-16. Evidence: live fem2d React lane `:6086`, in-page console hook on
`[DEBUG] performInvocation` / `command ingress lane` / `action failed` (recipe in §8).

## 0. What was measured (why the old hypothesis is wrong, and what is actually broken)

| Claim | Result |
|---|---|
| "a UI action arriving while the instance is busy is dropped" | **Refuted on the direct-actor path.** `onAction` → `performInvocation` → `runQueuedTurn` → `serializeCommandIngressForActor` is a per-actor FIFO (`🔌️PluginRuntime/🟦️.tsx:1196-1210`, cap 256, all Interactive lane → `rejected` is loud, `dropped` cannot happen). 146-round-trip drag + click at 0/60/120/300/500/800 ms after mouseup: the click reached the actor and cleared every time. |
| single-flight "busy" exists somewhere | **Yes, only on the hub-mounted browser-actor path**: `BrowserActorActionMailboxV1.dispatchRequest` throws `another action pending` (`🎯️action-handoff/📮️requests/🟦️.ts:36`), and the worker answers `action-busy` / `action-owner-mismatch` while a UI patch is unacked (`🏪️store/👷️worker/🟦️.ts:1903-1925`). Both surface as `console.error` + a "render error" toast — wrong, but not silent. |
| clicks vanish silently | **Yes, one confirmed gate**: `ShellHost/🟦️.tsx:6391` ignores any utility *deactivation* within 8 s of an arm (`lastUtilityArmAtRef`, commit `46c3cb9de0`, puzzle3d "echo-off"). The echo it targets is the UtilityTree picker's `onValueChange("")` resync (`🎛️UtilityTree/🟦️.tsx:318-335`, `utilityId: ""`); a user toggle-off carries `utilityId: <active id>` — same gate, different intent. |
| 1–2 s lag after mouseup | Every DOM `pointermove` is one guest round trip (`♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx:138` → `Canvas2dHost` `pointerMove` → `canvasPointerMove` command, no coalescing). fem2d answers each with a partial window refresh and, when not tracking, an `interactionHover` re-dispatch (`🕹️interaction/🖱️canvas-gesture/🦀️.rs:435`). ~30 ms per round trip ⇒ a 1 s drag drains for 1–3 s. |
| ordering hazard | The marquee commit is a **guest follow-up** (`Effect::ReplayShellCommand{interactionSelect}` → host `applyHostEffects` → `onActionRef.current(...)`, `ShellHost/🟦️.tsx:5672-5680`) enqueued only after `canvasPointerUp` settles, i.e. **behind any pointer input the user issued meanwhile**. Held in every sweep because the click's own follow-up is enqueued later still — but it is causally wrong and one extra round trip per pick/hover. |
| first-activation layout shift | Activating a utility from idle inserts the options row and scrolls the Utilities bar (~100 px shift); a fast second click lands on the neighbour. UX, not dispatch. |

Root causes, in order of architectural weight: (1) no provenance / causal order on inputs, so guest follow-ups race later inputs and heuristics get bolted on; (2) wall-clock gates standing in for versioned state; (3) per-event commands instead of gesture batching; (4) three transports with three backpressure vocabularies.

## 1. Laws (what the architecture must guarantee, transport-independent)

- **L1 Admit or refuse, never drop.** Every input becomes a ledger entry; its terminal outcome is `applied | refused(reason) | superseded(by)`. A refusal is typed, surfaced, and awaitable. No code path returns `undefined` on an input.
- **L2 Causal closure.** For one client, inputs to one window apply in issue order, and every guest follow-up of input *N* (interaction verb, `dispatchAction` re-arm, extension completion) applies before input *N+1*.
- **L3 Versioned registers.** Host-owned interaction registers (active utility per window, active tool, leftover selection overlay, selection mode) are `{ value, generation }` with compare-and-set writes. Stale writers are refused with `stale-generation`. No `performance.now()` gate anywhere in semantics; timers remain only as liveness watchdogs.
- **L4 Batched samples.** A gesture's samples are appended to one in-flight command per gesture; nothing is lost, latency is bounded by one turn, backlog is O(gestures) not O(events).
- **L5 Transport parity.** Direct actor (`PluginRuntime`), hub browser-actor (`worker` reservation + `BrowserActorActionMailboxV1`), wgpu bridge: all are bounded FIFOs with coalescing for samples and the same `Backpressure` vocabulary (`accept | coalesced | rejected`; `dropped` is forbidden for inputs).
- **L6 Collaboration/versioning.** Interaction state is per-client session state (reactor `interaction_selection_snapshot`), never in the replicated document. Samples are `Ephemeral` frames (never logged); gesture-end is the only command-log row (`ActionKind::Interaction`, already out of the history panel). A document-mutating gesture commits one inverse group at gesture-end — the undo unit is the gesture.
- **L7 Connection shortage.** Everything is local-first on the actor. Hidden tab / worker loss / hub offline never lose a ledger entry: un-acked entries are retained and re-issued after recovery (idempotent by generation). No semantic depends on wall-clock progress.

## 2. Components

### A. `InputLedger` (host, one per shell, keyed per window instance)
`🏛️ShellHost/🎯️input-ledger/🟦️.ts` (new, pure, law-tested), used by `onAction` as its first and only funnel.

```ts
type InputProvenance = Readonly<{
  windowId: string | null;      // window instance, or null for mode-level
  inputSeq: number;             // monotonic per shell
  causedBy: number | null;      // inputSeq of the input whose guest answer produced this entry
  origin: "user" | "guest" | "tutorial" | "replay" | "tick";
}>;
type InputOutcome = { kind: "applied"; inputSeq: number }
  | { kind: "superseded"; inputSeq: number; by: number }
  | { kind: "refused"; inputSeq: number; reason: InputRefusalReason; retryable: boolean };
type InputRefusalReason = "stale-generation" | "instance-sealed" | "instance-retired" | "queue-full"
  | "undeclared-action" | "viewer-read-only" | "owner-mismatch" | "mutation-rejected";
```
`ActionDescriptor` gains an optional `provenance` (additive; every existing caller keeps working — the ledger stamps `origin: "user"` when absent). `onAction` returns `Promise<InputOutcome>` (today `void | Promise<void>`), and every early `return;` inside it becomes `return refuse(entry, reason)` — the ledger is where refusals are made visible (§G).

### B. Causal order in the actor queue
`serializeCommandIngressForActor` (`🔌️PluginRuntime/🟦️.tsx:1206`) gains an `order` key: `{ causal: causedBy ?? inputSeq, arrival }`. The thunk scheduler dequeues the smallest `causal`, FIFO within it. A guest follow-up with `causedBy = N` is inserted **before** any queued input with `inputSeq > N`, satisfying L2 without the host having to know what the follow-up does. Implementation: `BoundedMailbox` gets an optional insertion key (a stable sort key per lane; the existing `coalesce` stays); `TurnScheduler.enqueue` passes it through. Bounded, no new allocation class.

Most follow-ups on the hot path disappear entirely with C, so B mainly covers `dispatchAction` re-arms, extension completions and `replayShellCommand` verbs that genuinely need shell authority (`os.*`).

### C. In-reactor interaction folding (removes the race and one round trip)
The six interaction verbs are already framework-owned and intercepted in the reactor (`🔌️plugin/🦀️.rs:25803 → dispatch_interaction_action`). Today a guest still has to *ask the host* to dispatch them (`interaction_select_effect` → `Effect::ReplayShellCommand`). New rule in `dispatch_emit` (`🦀️.rs:23995`): an `Effect::ReplayShellCommand { action_id ∈ INTERACTION_ACTION_IDS }` in a guest `Emit` is applied **inline, in the same turn, on the same instance** (`apply_interaction_verb_inline`: parse args → `next_selection`/`next_hover` → `revalidate_and_persist_interaction_state` → merge the app-declared interaction refresh scope into the turn's `UiDirtyScope`). The effect never reaches the host. The reserved-job/permit machinery stays for host-originated interaction verbs (keyboard, tree panel, tutorial).

Consequences: pick/marquee/hover cost one round trip instead of two; causal order is structural (the pick is part of the pointer-up turn); all eight apps that emit `interaction_select_effect`/`interaction_hover_effect` (fem2d, draw, layout, vcs, reasoning, generation2d, …) benefit without change; hub-mounted documents benefit identically because the browser-actor child runs the same reactor.

### D. Gesture sample batching
- Wire (additive): `canvasPointerMove` args gain `samples: [[x,y], …]` (canvas px, oldest first); `x`,`y` remain the last sample so an app that ignores `samples` keeps today's semantics with fewer calls. Same for `worldPointerMove`/map moves.
- Host: `Canvas2dHost`'s session (`📐️Canvas2dHost/🟦️.tsx:563-632`) hands moves to the ledger; the ledger keeps **one in-flight sample command per (window, gestureId)**: if that command is still queued (not yet dequeued by the actor queue), the new sample is appended to its args in place (the runtime already mutates queued payloads latest-wins for coalesced turns — `pendingCoalescedTurns`, `🟦️.tsx:1450`); if it has been dequeued, a new command is opened. Gesture id = pointerdown's `inputSeq`; `pointerup` closes the gesture and is never coalesced.
- Guest: `fem2d::canvas_gesture::pointer_move` iterates `samples` (the lasso needs the polyline; the rectangle needs the last point); the hover hit-test uses the last sample only. The marquee overlay stays app-drawn (partial window refresh once per batch instead of once per event).
- `pointerleave` → `pointerUp` mapping (`react-renderer/🟦️.tsx:192`) is replaced by a ledger `gesture-cancel` entry (the guest sees `canvasPointerUp { cancelled: true }`), so leaving the canvas no longer forges a release.

### E. Versioned interaction registers (replaces the echo-off)
- `activeUtilityByWindowId: Record<windowId, { id: string | null; generation: number }>`; `activeToolId` likewise. `SET_ACTIVE_UTILITY_ACTION_ID` args gain `expectedGeneration` (stamped by the rendering widget from the register it rendered) and `origin`. The interceptor at `ShellHost/🟦️.tsx:6382` becomes a CAS: mismatch ⇒ `refused(stale-generation)`; match ⇒ bump generation, forward to the guest with the new generation in the view state. Delete `lastUtilityArmAtRef` and the 8 s window.
- `Effect::SetActiveUtility` from a guest (puzzle3d `✏️editor/🦀️.rs:3649`) carries the generation it observed in its `ViewModel`; a stale programmatic switch is refused the same way — the puzzle3d echo becomes a *visible* `stale-generation` line instead of a timer.
- `🎛️UtilityTree` picker (`🟦️.tsx:318-335`): a deactivation is emitted only from an explicit item press (`onPressedChange` of the pressed item / pointer event), never from `onValueChange("")` resync. `flushToggles` (`:239-262`) diff-dispatches only entries whose pressed state the user changed.

### F. Transport parity
- `BrowserActorActionMailboxV1`: replace the single pending slot with a bounded FIFO (cap 32) + per-gesture coalescing; `dispatchRequest` never throws synchronously (returns the ledger's refusal on `queue-full`); receipts still match exactly by `actionSequence`.
- Worker `DocumentBrowserActorReservation.dispatchAction` (`👷️worker/🟦️.ts:1903`): `action-busy` → queue on `enqueueTurn` (already serial); the `pendingUiPatch !== null || viewRefresh !== null` precondition → **wait** for `settleUiPatch` (bounded by the existing 15 s patch deadline) instead of `action-owner-mismatch`; `surfaceRevision` check → accept when the request's revision is the latest *acknowledged* revision, so a click issued one patch behind is applied against the state the user saw, not refused.
- wgpu bridge (`🧊️wgpu/🐚️plugin-bridge/🟦️.ts:231-252`): `submitActorWork`/`serializeWgpuActorCall` adopt the same `order` key and sample coalescing; the ledger is shared code.

### G. Refusal surfacing (one vocabulary, always visible)
`InputOutcome.refused` → `showTransientNotice` (throttled per `reason` per 2 s so a burst is one toast), an `aria-live` polite region for a11y, a plain `console.warn` line (never `[DEBUG]`-gated), and a `globalThis.__semioInputLedger` census for probes. `stale-generation` from `origin: "guest"` is logged but not toasted (a programmatic echo is not the user's mistake).

### H. Settle by input
`awaitOperationSettle` (`ShellHost/🟦️.tsx:2237`) is keyed by `inputSeq` in addition to the typed-operation id, so a tick loop, a tutorial director or a probe can `await ledger.settled(inputSeq)`; the 30 s watchdog stays as liveness only.

## 3. Collaboration and versioning (L6)
- Interaction state stays in the reactor's per-instance interaction snapshot; nothing here touches the backbone, `MutationEnvelope`s or the hub. Multiple clients on one document each hold their own ledger; the browser-actor child in the worker runs the same reactor, so C/E/H apply identically there.
- A remote mutation landing mid-gesture: `revalidate_and_persist_interaction_state` already drops ids that no longer exist; the gesture continues, the gesture-end selects what exists at that moment.
- Command log / history: samples are `Ephemeral` (never recorded); `interactionSelect` rows stay `ActionKind::Interaction` (excluded from the history panel); document-mutating gestures (gumball, transform move) commit **one** inverse group at gesture-end, previews stay ephemeral — undo/redo units are gestures.
- Tutorial recorder/director (`ShellHost/🟦️.tsx:6326-6340`): records ledger entries with provenance; replay re-issues them with `origin: "tutorial"` and awaits `settled(inputSeq)` — deviation detection compares ledger entries, not raw actions.

## 4. Connection shortage (L7)
- No semantic timer: the 8 s echo-off is gone; the 30 s settle watchdog and the 15 s patch deadline remain liveness-only and always end in a typed refusal, never a silent resume.
- Hidden tab: `hostContinuations` is already an unthrottled macrotask; with D the backlog on a hidden tab is one open sample command per gesture.
- Worker/shard loss: `handlePluginShardLost` / `rememberInstanceForRecovery` (`🔌️PluginRuntime`) re-issue the ledger's un-acked entries after actor recovery; interaction verbs are set-semantics and E makes registers idempotent, so a re-issue cannot double-apply.
- Hub offline: the browser-actor route is absent → the direct-actor route serves the same ledger (already the fallback in `onAction`); when the hub returns, nothing needs replaying because interaction state is not replicated.

## 5. Multi-app, multi-extension
- Ledger keyed by window instance; spawned apps resolve through the existing `targetSession` remap (`ShellHost/🟦️.tsx:6560-6580`); provenance carries `(pluginId, instanceId)` so a sealed/retired instance yields `refused(instance-sealed|instance-retired)` (the existing `dropForSealedInstance`/`dropForRetiredInstance` become refusal constructors).
- `invokeExtension` inherits `causedBy`; its completion turn is inserted causally (B) so an extension answer never lands behind a later user input to the same instance. The session-work ledger (`🔀️surface-switch/🟦️.ts:201`) keys by the same tuple; a switch's quiesce sees ledger depth, not just outstanding promises.
- Mode-level tools vs window utilities keep their mutual exclusion; both are CAS registers, so a tool switch and a utility click racing across two windows resolve deterministically by generation.

## 6. Phased implementation (each phase ships green on its own)

| Phase | Scope | Files | Laws/tests |
|---|---|---|---|
| 1 Host | A (ledger, provenance, `InputOutcome`), E (CAS registers, picker fix, delete echo-off), G (refusal surfacing), D host-side batching (samples additive; guests unchanged) | `🏛️ShellHost/🎯️input-ledger/🟦️.ts` (new), `ShellHost/🟦️.tsx` (`onAction`, `SET_ACTIVE_UTILITY` interceptor, `awaitOperationSettle`), `🎛️UtilityTree/🟦️.tsx`, `📐️Canvas2dHost/🟦️.tsx`, `🎨️react-renderer/🟦️.tsx`, `🛠️ShellHelpers/🟦️.tsx` (`resolveUtilityActivation` → CAS) | ledger law (vitest, pure): issue/refuse/supersede; CAS law; batching law (N moves ⇒ ≤ ⌈N/dequeues⌉ commands, samples preserved in order); refusal-surfacing law (every early return maps to a reason) |
| 2 Reactor | C (inline interaction folding), D guest `samples` in fem2d/draw/layout/vcs/reasoning/generation2d, gesture-cancel | `🔌️plugin/🦀️.rs` (`dispatch_emit`, new `apply_interaction_verb_inline`), each app's `canvas-gesture` module | Rust unit: an `Emit` carrying `ReplayShellCommand{interactionSelect}` leaves `requested_effects` empty and the selection snapshot updated in one turn; fem2d lasso law over `samples` |
| 3 Transports | F: browser-actor queue, worker wait-for-ack, wgpu parity | `🎯️action-handoff/📮️requests/🟦️.ts`, `🏪️store/👷️worker/🟦️.ts:1903-1925`, `🧊️wgpu/🐚️plugin-bridge/🟦️.ts` | extend `🧪️backbone-envelope-io` mailbox tests: 3 rapid dispatches ⇒ 3 receipts in order, none rejected; worker: action issued while a patch is pending applies after the ack |
| 4 Causal queue | B general `order` key, H settle by `inputSeq` | `🎭️actor/📬️mailbox/🟦️.ts`, `🧵️turn-scheduler/🟦️.ts`, `🔌️PluginRuntime/🟦️.tsx` (`serializeCommandIngressForActor`, `runQueuedTurn`) | engine-contract law: follow-up `causedBy=N` dequeues before input `N+1` |

Acceptance probe (ticket `🐍️input-battery.mjs`, from the in-page helpers in §8): 120-sample marquee + click at 0/60/120/300/500/800 ms ⇒ `Summary` (cleared) every time **and** ≤ 8 guest round trips for the drag (today 146); Marquee→re-click at 1.5 s ⇒ deactivated (today ignored); Marquee→Lasso at 140 ms from idle ⇒ Lasso active; hub-mounted lane: 5 clicks at 50 ms ⇒ 5 `guest-applied` receipts, no toast.

## 7. Risks and non-goals
- Apps that draw a preview per `pointermove` (gumball drag, draw brush) get fewer refreshes under D; the batch still carries every sample, so geometry is unchanged — only intermediate frames. If an app needs a frame per sample it can request `samplesPerCommand: 1` in its window-kind declaration (escape hatch, declared, not default).
- C changes the observable effect list for interaction verbs (they no longer appear in `requested_effects`); `🧪️tests` asserting `ReplayShellCommand{interactionSelect}` presence must assert the selection snapshot instead.
- The puzzle3d `Effect::SetActiveUtility` echo becomes visible as `stale-generation`; its root cause (why the editor re-emits a switch) is puzzle3d work, not blocked by this ticket.
- Non-goal: moving marquee geometry into the framework. Apps keep owning hit-tests and overlays; the framework owns order, batching, registers and refusal.

## 8. Probe recipe used for the evidence
In-page hook (survives nothing — install after each reload): wrap `console.warn/error/log`, keep lines matching
`performInvocation|action failed|command ingress lane|setActiveUtility|hop ignored|browser actor|another action`
with `performance.now()` stamps; helpers `__drag(x0,y0,x1,y1,steps,stepMs)` and `__click(x,y)` dispatch
`PointerEvent`s on `document.querySelectorAll('canvas')[0]`; `__inspect()` reads the Inspection panel text
(`N Selected` vs `Summary`). Sweep: `for delay of [0,60,120,300,500,800]`. Console signature of the confirmed
silent gate: `[DEBUG] setActiveUtility hop ignored echo-off window=<id> requested=<id>`.
