# Runtime verification — procedural 3d on the React target (port 6018)

Coordinator-run in the Claude browser pane (viewport emulated 1440×900, tab hidden), against the 19:35 build served by
`serve-generation3d-react-dev` (`📓️wave1-boot-report-2026-09-09.md`). URL `http://127.0.0.1:6018/?plugin=generation3d`.

## 19:50 boot #1

| t | observation |
|---|---|
| +10 s | title `semio · os`, root present, body 103 chars, 0 canvases |
| +40 s | title `semio · procedural · 3d`; chrome renders: panel tabs Artifact/Catalogue/Inspection, Fullscreen, mode tabs Edit/Generate, example picker = **Hexagonal Mushroom Column**; **no window bodies, 0 canvases** — in their place one fault card |
| fault card | `{"code":"plugin.internal","message":"surface context exceeds its wire bound","origin":"plugin","retryable":false}` |
| console | Rust panic in the plugin worker: `ordered-map root must be explicitly retired before drop` at `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81:57`, stack: `OrderedMap<WidgetLayout>::drop` ← `FlowFixture` drop glue ← `FlowHost` drop glue ← `Generation3dPlayApp::pending_effects` closure ← `with_scratch_session` → `unreachable` trap; actor `procedural#1` trapped |
| console | after the trap every turn fails: `runtime instance authority is busy` (`plugin.internal`, retryable=false); `action failed setActiveExample {exampleId: hexagonal-mushroom-column}`; `local interaction observation failed`; `[DEBUG] reactor more-work streak=… command_ingress=true` |

Verdict: shell boots, plugin actor dies on its first `pending_effects`. Two defects to fix before any window can be judged:
1. **Retirement panic** — a scratch `FlowHost`/`FlowFixture` built by `with_scratch_session` is dropped without retiring its cold-tracked `OrderedMap` roots (`layout`).
2. **Surface wire bound** — a window surface context exceeds the framework's wire bound; which surface and which bound to be established (mesh payload of the preview vs flow graph scene).

## 00:20 (09-10) boot #2 — build 23:5x (`📓️rebuild-2026-09-09.md`), serve restarted from the main session (`$S/serve-react.sh`)

| t | observation |
|---|---|
| +40 s | title `semio · procedural · 3d`; chrome as before; **no trap, no wire-bound fault** (both boot #1 defects gone) |
| window area | one status line instead of window bodies: `plugin-ui.intake-budget-exhausted:1:procedural-main:74736` — the flow window surface (74 736 B) exhausts the retained-UI intake budget; this is the catalogue-embedding defect (`📓️unit-suite-2026-09-09.md` §3.1), catalogue lane in flight |
| console | `[DEBUG] thunk failed command-ingress:procedural#1 typed-operation failed: validation failed: batched item candidate failed its exact fixed fold contract`; same message on `action failed setActiveExample {exampleId: hexagonal-mushroom-column}` and `local interaction observation failed` |
| console | `[DEBUG] typed-operation publication turn=1024 operations=["27:Retiring:true:true"] latest_wins_empty=true …` — the retained typed operation for the example switch retires without publishing |
| DOM | 0 canvases, no `framework.window.*` ids rendered |

Verdict: actor survives now; two blockers remain — (1) flow window surface over the intake budget (catalogue lane), (2) every typed operation fails the "batched item candidate … exact fixed fold contract" validation, so no action (example switch, interaction observation) publishes.

## 02:55 (09-10) boot #3 — restage with catalogue fix (`$S/activate-1.log`), serve with `SEMIO_VITE_HMR=0`

| t | observation |
|---|---|
| +20 s | title `semio · procedural · 3d`; **both edit-mode windows render their chrome and bodies**: `#framework.window.proceduralMain` (966×836, 2 canvases) and `#framework.window.proceduralPreview` (451×836, 1 canvas); no fault card, no intake-budget line, no fold-contract error in the retained console window |
| flow scene (fiber) | `scene.nodes=7, edges=6` for hexagonal-mushroom-column, viewport zoom 1.78, statusJson: height/radius/sides `ok`, `profile` **queued**, `extrusion-axis` **computing**, `extrude` **queued**, column-preview ok → evaluation stalls at the first extension node |
| flow canvases | 300×150 (never resized to the 966×836 container) → blank window; the preview canvas is correctly 451×836 |
| world scene (DOM) | `data-meshes-json="[]"`, `data-instances-json="[]"` → no geometry yet (consistent with the stalled eval) |
| unhandled rejections ×2 | `SemioFaultError: window kind procedural-main does not own action ""` from `PluginRuntime/🟦️.tsx:1539 invocationFromFrames` ← `ShellHelpers/🟦️.tsx:318` — an empty action id invoked on the flow window |
| console | React warning `Cannot update a component (UiNodeView) while rendering a different component (FrameworkOsShellInner)` |
| console | no `[DEBUG]` lines mentioning flowEval/tessellate/intake in the retained window (2 879 older lines dropped) |

Verdict: shell + both windows alive; blockers now (1) flow evaluation never completes the first extension round trip (`extrusion-axis` stuck `computing`), so no mesh; (2) node-graph canvas sizing (300×150) leaves the flow window blank; (3) empty-action invocation fault.

## 03:40 (09-10) boot #4 — same restage, page reload after the React renderer fixes (`📓️renderer-fixes-2026-09-10.md`; serve restarted, HMR off)

| observation |
|---|
| flow window canvases now 966×836 (sizing fix live) but both are 2D contexts with 0 painted pixels — the node-graph wasm engine never attached (no WebGL context on the GPU canvas); preview canvas 451×836 WebGL, nothing drawn |
| no unhandled rejections (empty-action fault gone) |
| eval status unchanged: `extrusion-axis: computing`, preview status `phase: idle, evalLen 0` |
| in-page filtered console hook (the tab buffer drops thousands of `[DEBUG] reactor` lines): `[DEBUG] action failed setActiveExample {"exampleId":"hexagonal-mushroom-column"} [DEBUG] coerceWireBytes: unsupported payload {"tag":"none"}` and the same for `interactionSelect {"surfaceId":"1","domainId":"graph","targets":"[]","merge":"replace","method":"pick"}`; nine `hot-swap flow-extension-*` lines; no attach/engine lines |

Verdict: every action now fails in the TS wire decoder (`coerceWireBytes` rejects a `{tag:"none"}` payload — an option/none variant the friendly decoder does not accept), so the example load and the eval tick chain never start; the node-graph engine attach also does not happen.

## 04:20 (09-10) boot #5 — 04:02 restage (fold contract + extension registry fix), plus TS fixes picked up by reload (node-graph attach without WebGPU device; option-none unwrap in `wireEffectToFriendly`'s `paramPack`/`packField`; option-none fault envelope in `commandIngressFaultDisplay`)

| observation |
|---|
| `[DEBUG] node-graph host mount … → session ready → attach called 966x836 → flow surface created device=webgpu → surface ready nodes=7 edges=6 → dag draw lod=normal/detail` — the flow graph engine attaches and draws (the hidden pane still shows a blank composite) |
| no `coerceWireBytes` / `does not own action` failures any more |
| every typed operation now faults in Preflight: `typed-operation failed: retained command exceeds semantic work capacity` (`🔌️plugin/🧵️retained-command/🦀️.rs` Preflight: `extent` vs `maximum_work_items`) — seen on the framework `interactionSelect` observation; the example load/eval chain does not start, eval still `extrusion-axis: computing`, preview meshes `[]` |
| console buffer is flooded by token-split worker `[DEBUG] maintenance stage=… / cooperative maintenance callback overran the interactive ceiling (12200 us)` lines; the tab drops thousands of messages per boot, so in-page hooks + pattern reads are required |

Verdict: one blocker left in the action path (work capacity vs the new two-row footprint); work-capacity lane dispatched with a restage.

## 05:30 (09-10) boot #6 — 05:22 restage (work-capacity + envelope-load + 3d-suite fixes)

| observation |
|---|
| after 80 s: title stays `semio · os`, body `No plugins loaded`, 0 canvases |
| console: `PluginRuntime: actor procedural#1 trapped: shard 0 worker fault [handler/first-step] … {"code":"plugin.reactor-turn-deadline","message":"guest lifecycle turn exceeded strict time authority; receipt retained","origin":"framework","retryable":true}` → the guest's FIRST lifecycle turn overran the strict turn deadline and the runtime treats the (retryable) fault as a trap; every later action: `program procedural: no actor for instance 1 (createApp not called, or already destroyed)` |
| an older module version in the same buffer shows `plugin.reactor-close-authority: native close terminal unavailable` on a `[handler/turn]` (previous load's close path) |

Verdict: regression on this restage — the plugin's first step exceeds the strict time authority (new bootstrap work, or the box under load), and a retryable deadline fault is fatal on first-step.

## 06:35 (09-10) boot #7 — 06:19 restage (tick addressing, work capacity, envelope load, FlowHost, hot path, first-step deadline) + TS fixes (JSON-512 reader)

| observation |
|---|
| boots reliably; both edit windows attach and paint; no faults, no `typed-operation failed`, no `action failed`, no JSON error card |
| the retained tick chain now runs continuously (`command-ingress` thunks, `completion apply operation=1428…1442`) |
| console (46× per boot): `flowTessellate skipped: no plugin contributes flow extension 'brep'` — producers still emit the manifest id `brep` while the registry now addresses extensions by owning plugin id (`flow-extension-brep`) |
| flow status still `extrusion-axis: computing`, `profile/extrude: queued`; preview meshes `[]`, status `idle, evalLen 0` |
| remaining console chatter is TS: `[DEBUG] thunk start/done`, `completion apply`, `applyHostEffects refresh` per action |

Verdict: action path healthy end to end; the last blocker before geometry is extension-id translation on the tessellate/evaluate producers (lane dispatched, restages).

## 07:25 (09-10) boot #8 — 07:14 restage (extension addressing)

| observation |
|---|
| boots; both windows attach; no faults/errors; tick chain runs |
| preview status is now a typed, user-visible fault: `phase: faulted`, label `Geometry extension unavailable / Geometrie-Erweiterung nicht verfügbar`, `fault.code: flow.extension-not-contributed`, `extensionId: brep` — accurate: the served plugin has no flow-extension contributions installed (`📓️extension-addressing-2026-09-10.md` §6) |
| flow status unchanged (`extrusion-axis: computing`); meshes `[]` |
| TS per-action chatter (`thunk start/done`, `command ingress settled`) still present — gating did not cover these emitters |

Verdict: only the contribution delivery (`setContributions` paged channel, lane in flight) stands between the app and geometry.

## 07:55 (09-10) wgpu wasm boot #1 — `$S/wgpu-dev.sh` (private target, trunk serve on 6118), URL `http://localhost:6118/?plugin=generation3d`

| observation |
|---|
| page `Semio Wgpu`, one 1440×900 canvas, `navigator.gpu` present; shell-boot stalls at 86 % with `wgpu renderer fault: worker-boot-failed: shell-boot: create_app promise failed: wgpu-ui.native-owner-required — No UI-thread frame fallback was attempted.` |
| console: the **flow** plugin (in the generation3d closure) panics on `[handler/first-step]`: `interactive-job.catalog-authority: tool proof catalog must exactly join migrated generated declarations to live concrete factories … tool 'addGeneration' … generated_migrated=false` — its `generated_ids` (34, incl. addGeneration/removeGeneration/selectGeneration/renameGeneration/updateGenerationValues, duplicateWidget, connectMediaPorts, reorganize, renameFlowWidget, nodeGraphEdit, spotlightCommit, runExtensionAction, focusSelection) exceed its `migrated` proof set; actor `flow#1` trapped |

Verdict: two wgpu-only blockers — the flow artifact's proof catalog is out of sync with its generated declarations (traps on any host that instantiates it), and the wgpu shell's worker boot requires a native UI owner it never gets.

## 08:50–09:00 (09-10) boot #9 / #9b — 08:35 restage (contributions delivery)

| observation |
|---|
| #9: contributions push failed at once: `setContributions command failed procedural missing field 'locale'` — the shell passed the raw session view state (no `locale`) to `handleCommand` for `setContributions`/`setAppRegistrations`; fixed in `🏛️ShellHost/🟦️.tsx` by passing `resolvedTargetViewState(session)` (coordinator patch) |
| #9b (reload): no contribution errors; ~60 s after boot the procedural actor dies with `memory allocation of 189328 bytes failed` → `rust_oom` → `unreachable` in `wit_bindgen::rt::async_support::start_task` (`__export_poll_cabi`) → `actor procedural#1 trapped [handler/turn]`; before the trap: flow status unchanged, preview `faulted / Geometry extension unavailable`, meshes `[]` |

Verdict: the paged contributions now reach the guest, and the guest's wasm linear memory is exhausted during/after the install (guest-memory lane dispatched with restage).

## 09:05 (09-10) wgpu wasm boot #2 — after `📓️wgpu-shell-boot-2026-09-10.md` (bundle rebuilt, trunk serve restarted)

| observation |
|---|
| `shell-construct 85% — wgpu renderer fault: ui-turn-overrun: progress-hook UI turn took 5.000 ms — No UI-thread frame fallback was attempted.` — a 5 ms wall-clock UI-turn budget on the construct progress hook is fatal (hidden pane, box load ~20) |

Verdict: same wall-clock-verdict family as the guest first-step deadline; wgpu UI-turn lane dispatched (executing-time pricing, graceful degradation, bundle rebuild).

## 09:35 (09-10) wgpu wasm boot #3 — after `📓️wgpu-ui-turn-2026-09-10.md` + closure rebuilt with the flow catalog fix

| observation |
|---|
| `plugin-graph 25% — worker-boot-failed: worker-boot-step-overrun: plugin-graph took 8.300 ms against a 8 ms budget`; the new fallback panel reports `Surface: faulted; UI-thread frames: unavailable (OffscreenCanvas transferred); Worker terminated: yes; UI turns over budget 0/0` — the frame worker's boot steps still carry a fatal 8 ms wall-clock budget |

Verdict: same verdict family one layer down (worker boot steps); wgpu worker-boot lane dispatched.

## 09:55 (09-10) boot #10 — 09:41 restage (guest-memory + flow catalog + close ladder)

| observation |
|---|
| after the nine `hot-swap flow-extension-*` lines: `PluginRuntime: shard 0 lost, restoring actors: procedural#1` → `turn failed for actor procedural#1: shard 0 terminated` → `Framework OS boot failed: plugin-ui.native-owner-required` (`🔌️PluginRuntime/🟦️.tsx:1395`); page `No plugins loaded`, no windows |
| no panic text, no wasm stack — the shard worker died silently on/after instantiation of the 09:41 component |

Verdict: regression on the 09:41 build (worker terminates before the first turn completes) and the React runtime's boot cleanup masks the real cause with `plugin-ui.native-owner-required` (the same guard the wgpu lane removed on its side).

## 10:20 (09-10) boot #11 — 09:41 restage + shard-client first-turn ladder (TS, `📓️shard-termination-2026-09-10.md`)

| observation |
|---|
| boots (≈150 s to window bodies: the guest's first `poll` runs ~3 s synchronously, then the 73 contribution pages); both windows attach; no traps, no action failures |
| after 2.5 min: preview still `faulted — Geometry extension unavailable (brep)`, flow `extrusion-axis: computing`, meshes `[]` — contributions either not pushed for this session or installed without re-arming the faulted evaluation |

Verdict: last step to geometry is the contribution→evaluation hand-off (re-arm lane dispatched).

### 10:35 boot #11 — interactions (checklist steps 10-13, 19, addGeneration)

| action | how | result |
|---|---|---|
| Example switch → Box Shell Preview | picker (`ref` click on the option) | picker label changes; flow graph and preview unchanged (still hexagonal nodes); no console failure — publication/refresh gap (example-switch lane) |
| Mode → Generate | `#playground.navbar.modes.generate` | **works**: three windows render — `generation3dGenerations` ("GENERATIONS (no generations) · ACTIONS · Add Generation"), `generation3dGenerateForm` ("Add a generation to edit input values."), `generation3dGeneratePreview` (1 canvas) with Window Options / Actions / Utilities chrome |
| Add Generation | click on the tree action row | no visible change after 20 s, no console failure — same publication gap class, or the tree row needs activation rather than a click |

## 11:05 (09-10) wgpu wasm boot #4 — after `📓️wgpu-worker-boot-2026-09-10.md` (served live)

| observation |
|---|
| plugin-graph and shell-construct pass (budget breaches recorded, not fatal: UI 1/0 worst 11 ms @ progress-hook; Worker 1/0 worst 8.1 ms @ plugin-graph:order); fails at `shell-boot 86% — renderDocument promise failed: wgpu-ui.intake-budget-exhausted` (fixed retained-UI intake budget on wgpu; React's proportional budget landed 09-09) |

Verdict: wgpu intake-budget lane dispatched.

### Root cause of the dead UI actions (React, boot #11)
`📓️tree-action-dispatch-audit-2026-09-10.md`: every generation3d `WindowKindDefinition.actions` is empty, so ShellHost's `declaredAction` gate rejects `addGeneration`/`setActiveExample`/… before `handleAction` (ungated `[DEBUG] skipping undeclared action` warn). Window-kind actions lane dispatched.

## 11:35 (09-10) boot #12 — 10:54 restage (poll-leak future shrink; contributions + registry addressing already in)

| observation |
|---|
| boots in ≈3 min (first poll ~3 s synchronous + 73 contribution pages); both windows attach |
| **contributions install and evaluation starts**: the shell dispatches `invokeExtension {extensionId: flow-extension-math, capability: evaluate, req: 15n}` — the first time the runtime resolves an extension |
| every delivery of the extension result traps the guest: `unreachable at wasm.abort ← abort_internal ← cabi_realloc ← poll` → `actor procedural#1 trapped [handler/turn]` → actor restored → next delivery (`req: 16n`) traps identically (endless loop) |
| preview `faulted`, meshes `[]` |

Verdict: the last hop (extension result → guest) aborts in the guest allocator while lowering the poll input; realloc lane dispatched.

## 11:45 (09-10) wgpu wasm boot #5 — after `📓️wgpu-intake-budget-2026-09-10.md` (served)

| observation |
|---|
| `renderer-runtime 65% — worker-boot-timeout: Worker reported no boot progress for 60000 ms` — the 76 MB renderer wasm compile/instantiate under load 55-70 in a hidden tab exceeds the wall-clock watchdog; Worker terminated |

Verdict: last wall-clock verdict in the wgpu boot; watchdog lane dispatched (phase-declared heartbeats + module cache).

## 12:20–12:35 (09-10) wgpu wasm boot #6 — after `📓️wgpu-boot-watchdog-2026-09-10.md` (served 12:14), box load 90-118

| observation |
|---|
| no watchdog kill any more: `wasm-compile 100 %` → `shell-boot 86 %` reached after ≈170 s; then no progress for ≥5 min, no fault card, no console line, canvas 1×1 (pane reopened mid-boot) |

Verdict: the wgpu shell-boot phase (plugin actors' first steps, contribution pages) is silent and either extremely slow under load or stalled on the same guest abort React shows on extension-result delivery; wgpu shell-boot silence lane dispatched.
