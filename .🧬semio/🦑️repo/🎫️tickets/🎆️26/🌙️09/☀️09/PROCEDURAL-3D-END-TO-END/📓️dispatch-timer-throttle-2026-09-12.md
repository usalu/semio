# One owned continuation scheduler — and what the ~24 s extension hop actually is

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-12, continuation of a lane whose previous agent
stopped at "now let me run the twin and the vitest suite". Repo MCP unavailable (`invalid initialize
params`), so the ticket folder was managed on disk; no ticket state opened/closed. Concurrent lanes
(`example-switch-runtime`, `role-switch-runtime`, `node-graph-canvas-paint`,
`selection-merge-vocabulary`) were editing the same files throughout; every red below is attributed.

## TL;DR

1. The scheduler asked for in `📓️audit-extension-hop-latency-2026-09-12.md` §4.1 **exists, is wired,
   is tested and is proven correct at runtime**: `🪃️continuation/🟦️.ts`, used by
   `scheduleDispatchAction`, `yieldPluginUiContinuation`, the typed-operation wake and the actor
   teardown marker. Measured on 6018 with a temporary bracket: a `delay_ms: 0` re-arm goes from armed
   to fired in **71 ms**, not the ~1 000 ms a throttled nested `setTimeout` would cost.
2. **The audit's causal hypothesis is therefore disproven by measurement.** The re-arm timer was never
   where the ~24 s went. The same bracket shows the cost sits INSIDE the dispatch it schedules:
   `continuation dispatch settled … tookMs=17657 / 18027 / 11867`. The silent ~11-13 s windows the
   audit localized to `scheduleDispatchAction` are the `flowEvalTick` command's own settle, and they
   bracket the node-graph canvas surface-create/first-draw chain (§3).
3. A **separate, fatal, silent defect was found and fixed** along the way: both of the install's
   `flowEvalTick` re-arms were being dropped without a word, so on the current build the evaluation
   chain never started at all (`meshes: 0`, nodes stuck `queued`/`blocked` for 110 s). Cause: the
   `dispatchAction` branch read `loadedPlugins` from the render closure, and the DEFERRED effect pass
   runs against an `applyHostEffects` captured one render before the installed program reached that
   state — so `loadedPlugins` was `[]` and `if (pluginEntry)` skipped both re-arms. Now reads
   `loadedPluginsRef.current` (this file's own convention everywhere else) and a miss is loud.
4. After that fix the chain converges end to end: **all 7 `window:procedural-main` node statuses `ok`,
   `window:procedural-preview` `meshes: 3`, `facesDone 8/8`, `unitsDone 44/44`, `ratio 1.0`** — at
   **122 s**, i.e. unchanged from the 112 s baseline, exactly as the disproof predicts.

---

## 1. What was on disk from the previous agent (verified, not assumed)

| file | state | verdict |
|---|---|---|
| `🧰️framework/🔨️modules/⏳️async/🪃️continuation/🟦️.ts` | new, 505 lines: ports, scheduler, virtual host, fixture runners, in-source vitest | complete, kept |
| `…/🪃️continuation/🧫️fixtures/🔣️.json` | new, 8 language-agnostic cases incl. the 10-hop re-arm law | complete, kept |
| `…/🪃️continuation/🧪️tests/🔬️node-twin/🟦️.ts` | new, framework-free twin on the real event loop | complete, kept |
| `…/⏳️async/🟦️.ts` | re-exports `./🪃️continuation/🟦️.ts` | complete |
| `…/⏳️async/📦️packages/🟦️typescript/{📋️project.json,📜️script.ts}` | `test` + `twin` targets registered | complete |
| `…/📦️packages/🟦️typescript/vitest.config.ts` | `includeSource` carries the continuation suite (shares the file with the boxed-fixed-slots lane) | complete |
| `🛠️ShellHelpers/🟦️.tsx` | `scheduleDispatchAction` takes a `ContinuationScheduler`, returns its cancel | complete |
| `🔌️PluginRuntime/🟦️.tsx` | `yieldPluginUiContinuation` → `hostContinuations.yieldContinuation()`; `teardownPluginActor` marker; typed-operation wake keyed `operation-wake#<instance>` | complete — **zero literal `setTimeout(`/`setInterval(` remain in this file** |
| `🧪️tests/🔬️engine-contract/🟦️.ts` | 3 `scheduleDispatchAction` laws incl. the monkeypatched-slow-`setTimeout` 10-hop regression | complete; one TS cast repaired here (§5) |

Nothing was half-written. What was missing was everything after "write it": no run, no typecheck, no
timer audit, no runtime proof, no report.

## 2. The scheduler's contract (as committed)

`delayMs <= 0` is a **continuation** — one `MessageChannel` macrotask, never a timer, never
`requestAnimationFrame`/`requestIdleCallback` (neither fires in a hidden tab; evaluation must keep
converging when nobody is looking, only paint may stop). `delayMs > 0` is a **deadline** — a real
timer, but ONE per scheduler, armed at the earliest deadline and re-armed on drain. Every request is
cancellable; requests sharing a `key` coalesce to one run at the EARLIEST deadline with the NEWEST
callback; ties run in enqueue order. `hostContinuations` is the single process-wide instance, so "one
implementation, not two" is a fact about the object graph, not only the source.

## 3. Runtime measurement on 6018 — the disproof

Temporary `[DEBUG]` bracket around `scheduleDispatchAction` (armed → fired → dispatch settled),
removed again afterwards. Probe `🐍️console-dump-probe.mjs`, headless + never foregrounded — i.e. the
exact renderer class Chrome throttles. Raw console in `🗑️generated/timer-throttle-7/`.

```
50700  [DEBUG] continuation armed  action=flowEvalTick delayMs=0     (×2)
50771  [DEBUG] continuation fired  action=flowEvalTick waitedMs=71   (×2)
68445  [DEBUG] continuation dispatch settled action=flowEvalTick tookMs=17657
68814  [DEBUG] continuation dispatch settled action=flowEvalTick tookMs=18027
83719  [DEBUG] continuation armed  action=flowEvalTick delayMs=0
83783  [DEBUG] continuation fired  action=flowEvalTick waitedMs=72
95657  [DEBUG] continuation dispatch settled action=flowEvalTick tookMs=11867
```

**Armed → fired: 71-72 ms.** A throttled nested `setTimeout(0)` chain in this page would be ~1 000 ms
per hop and the audit attributed 11-13 s per window to it. Neither number appears. The scheduler does
what it says, in the very page the hypothesis was built on.

**Where the time actually is.** Inside the dispatch — 11.9-18.0 s per `flowEvalTick`. Zooming the
clean run (`🗑️generated/timer-throttle-8/console.txt`) into one hop's two silent windows:

```
25064  extension completion submitted req 1
       ← 6 045 ms, zero lines
31109  flow surface created surface=2 483x814 dpr=1 present=2d
31109  node-graph surface ready nodes=7 edges=6
31579  dag draw lod=normal
       ← 6 471 ms, zero lines
38050  command ingress settled  ← the guest's flowEvalTick finally completes
38506  performInvocation flowEvalTick (next hop)
```

Both silent windows sit either side of the node-graph canvas being **created and first-drawn again**,
and the guest's command settle waits behind them. `node-graph host mount` → `session ready` →
`attach called` → `flow surface created` → `dag draw` repeats **once per hop** for a 7-node/6-edge
graph, on a page that logs `No available adapters` (headless, no GPU) and falls back to `present=2d`.
That is ~6 s of main-thread work, twice per hop, ≈ the audit's "two silent ~11-13 s windows per hop"
— same shape, different owner. It belongs to the `node-graph-canvas-paint` lane, not to this one, and
is the next thing to attack for the ticket's wall-clock target.

Corroboration that it is not the guest: every extension answer is still `turns: 1`, and the audit's
own §2 already ruled out per-hop actor re-instantiation.

## 4. The silent drop that killed the chain (fixed here)

On the build served at the time of this lane, the chain did not merely run slowly — it did not run.
Three consecutive 70-110 s probes ended with `window:procedural-main` nodes at
`profile: queued / extrusion-axis: computing / extrude: blocked`, preview `meshes: 0`, and **zero**
`flowEvalTick` invocations. The bracket showed why:

```
[DEBUG] setContributions deferred effects  … 2× dispatchAction{flowEvalTick, delayMs: 0}
[DEBUG] deferred effects entering applyHostEffects count=2 ownerCurrent=true
[DEBUG] dispatchAction branch action=flowEvalTick plugin=procedural entry=false loaded=      ← EMPTY
```

`publishContributions`'s `dispatchDeferredEffects` re-enters `applyHostEffects` on a microtask against
the closure captured when the publish STARTED — one render before the program it just installed
reached `loadedPlugins` state. The branch's `const pluginEntry = loadedPlugins.find(…)` therefore
missed, and `if (pluginEntry) { … }` skipped both re-arms **with no output at all**. Fixed at
`🏛️ShellHost/🟦️.tsx` by reading `loadedPluginsRef.current` — the convention this file already uses at
~20 other sites for exactly this reason — and by making the miss a loud `[os-shell]` error instead of
a silent `continue`. The sibling silent return (`if (!live) return;` in the same microtask) is loud
now too. With that one change the identical probe converges (§6).

## 5. Timer audit — every `setTimeout`/`setInterval` reachable from the evaluation path

Converted (all now on `hostContinuations`):

| site | was | now |
|---|---|---|
| `🛠️ShellHelpers` `scheduleDispatchAction` | `setTimeout(fn, delayMs)`, injectable | `scheduler.schedule`, returns the cancel |
| `🔌️PluginRuntime` `yieldPluginUiContinuation` | its own private `MessageChannel` + waiter array | `hostContinuations.yieldContinuation()` — one primitive, not two |
| `🔌️PluginRuntime` `teardownPluginActor` | `setTimeout(…, 0)` to drop the tearing-down marker | `schedule(…, 0)` — a clamped timer kept a re-activated actor id looking torn-down for a second |
| `🔌️PluginRuntime` typed-operation wake | `setTimeout(drainTypedOperations, nextWake)` | `schedule(…, nextWake, "operation-wake#<instance>")` — coalesced per instance |

Left as real timers, with the reason:

| site | delay | why it stays |
|---|---|---|
| `🏛️ShellHost` `awaitOperationSettle` | `OPERATION_SETTLE_WATCHDOG_MS` | a watchdog; a deadline that fires late is safe, and it must NOT outrun the completion it guards |
| `🏛️ShellHost` presence heartbeat (`setTimeout` 1 s + `setInterval`) | cadence | network liveness, not evaluation; a hidden tab beating slower is correct |
| `🏛️ShellHost` tutorial recording samplers (100 ms, 5 s) | cadence | recording a session nobody is watching has no meaning |
| `🏛️ShellHost` transient notice (4 s), layout-change settle | UI | debounces on rendered chrome |
| `🏛️ShellHost` camera tween | `requestAnimationFrame` | paint; deliberately stops in a hidden tab |
| `🛠️ShellHelpers` `loadPluginModuleResilient` | idle re-arm | a load deadline, not a continuation |
| `🛠️ShellHelpers` `AutoCheckinScheduler` | `AUTO_CHECKIN_IDLE_MS` | an idle debounce by definition |
| `🛠️ShellHelpers` `createInFlightSkippingInterval` (World3d `suggestionsTick`, `fillBuildTick`) | 120 ms | a cadence FLOOR with in-flight skipping, not a zero-delay chain. Converting it to a continuation turns a 120 ms floor into a spin; a hidden tab clamps it to ~1 Hz, which slows the fill utility but cannot stall it (the gate degrades to the guest's real turn time anyway). Worth revisiting only if a hidden-tab fill cadence is ever a requirement. |
| `🛠️ShellHelpers` download anchor cleanup (`window.setTimeout`) | DOM housekeeping | not on any evaluation path |
| `🌐️World3dHost` camera dispatch debounce, marquee commit hold (250 ms), celebration timer | UI | debounces/animation |
| `📮️shard-client` `startWatchdog` | `watchdogIntervalMs` | heartbeat liveness ladder; late detection is safe, early detection is a false positive |
| `🎭️actor` `TurnScheduler` pump | `queueMicrotask` | already unthrottled; nothing to convert |
| `🏛️ShellHost` `dispatchDeferredEffects` | `queueMicrotask` | same |

`🔌️PluginRuntime/🟦️.tsx` now contains **no** literal `setTimeout(`/`setInterval(` call at all — only
the doc comments explaining why.

## 6. Gates run (quoted, nothing claimed unrun)

| gate | command | result |
|---|---|---|
| scheduler vitest | `bun nx run @semio-tech/framework-async:test` | **24 passed / 24** (2 files: 20 continuation + 4 boxed-fixed-slots) |
| node twin | `bun nx run @semio-tech/framework-async:twin` | **8 cases held on the virtual clock AND on the real event loop**; `10 chained zero-delay hops on the real loop: 0 ms` (throttled equivalent ~10 000 ms) |
| engine regression law | `bun ./📜️script.ts test long -t "scheduleDispatchAction"` | **3 passed**, 956 skipped — incl. the monkeypatched-slow-`setTimeout` 10-hop chain asserting 0 patched calls and < 250 ms |
| react engine corpus | `SEMIO_TEST_LEVEL=long bun ./📜️script.ts test long` | **948 passed / 11 failed / 959** |
| react typecheck | `bun ./📜️script.ts typecheck` | **867 errors**, down from 875 at lane start; **0 in `🛠️ShellHelpers`, 0 in `🪃️continuation`**, and the one error this lane owned (an over-narrow `as typeof globalThis.setTimeout` cast on the regression test's monkeypatch) is repaired |
| runtime | `SEMIO_PROBE_SECONDS=160 SEMIO_PROBE_OUT=timer-throttle-8 bun 🐍️console-dump-probe.mjs` | converged (below) |

**The 11 corpus reds are all other lanes'**, none in a file or symbol this lane touches:

- `🧪️tests/🖱️world3d-interaction/🟦️.tsx` ×5 (`instance-pick-additive/-subtractive/-subtractive-on-command/-invertive`, modifier chords) — `selection-merge-vocabulary`.
- `engine-contract > coalescing action dispatcher`, `> framework renderer hosts … vortex marker`,
  `> every self-gating world lane dispatches through the awaitable twin` — same lane's
  `createCoalescingActionDispatcher`/World3d dispatch rework.
- `engine-contract > buildNoteShellCommandAction` — an undo-inverse lane (`inverseArgs`,
  `inverseCommandId` appear in the received descriptor).
- `PluginRuntime > surface render ViewModel` (an extra `window`/`canvas-body` row) and
  `> readAppDocumentPack` (an extra `ops` field) — the document/window projection lanes.

At lane start the same corpus was 4 failed / 952 passed; the delta is entirely the above lanes landing
work during this session, and `🧪️tests/🔀️surface-switch` (red at lane start) is now green again.

**Runtime, final clean run** (`🗑️generated/timer-throttle-8/`):

```
 7 420  setContributions deferred effects  (2× dispatchAction{flowEvalTick, delayMs:0})
 7 490  performInvocation flowEvalTick ×2          ← 70 ms after the deferred pass, not 17 s
25 064  extension completion submitted req 1  ok  199 B
52 871  extension completion submitted req 2  ok  122 B
76 049  extension completion submitted req 3  ok  124 B
102 025 extension completion submitted req 4  ok  393 B
117 129 extension completion submitted req 5  ok  131 B
122 095 extension completion submitted req 6  ok  2 265 B
```

`window:procedural-main`: `height/radius/sides/profile/extrusion-axis/extrude/column-preview` — all
`ok`. `window:procedural-preview`: `meshes: 3`, `phase: idle`, `facesDone 8/8`, `unitsDone 44/44`,
`ratio 1.0`. Convergence **≈122 s**, against the 112 s baseline.

That number is the honest headline: **the brief's "well under 20 s" target was not met, because the
thing it assumed was the bottleneck was not the bottleneck.** The scheduler removes a real hazard
(and would have cost ~1 s/hop the moment anything else got fast), the silent-drop fix restores a chain
that was fully dead, and the remaining ~13 s per half-hop is now measured, localized and attributable
— §3 names the file and the call chain to attack next.

## 7. Files touched by this lane

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🪃️continuation/🟦️.ts` (verified, kept)
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🪃️continuation/🧫️fixtures/🔣️.json` (verified, kept)
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🪃️continuation/🧪️tests/🔬️node-twin/🟦️.ts` (verified, kept)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — `dispatchAction` branch reads `loadedPluginsRef`, both silent drops made loud
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — `as unknown as typeof globalThis.setTimeout` on the throttle monkeypatch
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️dispatch-timer-throttle-2026-09-12.md` — this report

Outputs (deletable with the ticket): `🗑️generated/timer-throttle/` (test + typecheck logs, the
response-error probe) and `🗑️generated/timer-throttle-{2,3,4,5,6,7,8}/` (probe consoles, hosts, shots).

## 8. Next, for whoever picks this up

1. **`node-graph-canvas-paint` lane**: the node-graph canvas is mounted, attached, surface-created and
   first-drawn once per `flowEvalTick` settle, ~6 s each on a `present=2d` fallback, twice per hop.
   That is the ticket's remaining wall clock. Re-use the surface across refreshes instead of
   re-creating it, or move the draw off the settle path.
2. The audit doc `📓️audit-extension-hop-latency-2026-09-12.md` §2/§4.1 should be read alongside this
   one: its localization of the gap was right, its attribution of the gap to `setTimeout` was not, and
   §3 above has the console lines that settle it.
3. `createInFlightSkippingInterval`'s 120 ms floor is the only continuation-adjacent timer left. If a
   hidden-tab fill cadence ever matters, the fix is a settle-driven re-arm with a floor, not a
   conversion to a zero-delay continuation.
