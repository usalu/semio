# 🎫️ React remaining reds — lane `react-remaining-reds` (2026-09-15, session 5)

Port **6018** (`📜️serve-generation3d-react.sh`, screen `g3dreact`, `SEMIO_VITE_HMR=0`). This lane takes the
reds `📓️react-current-tree-battery-2026-09-14.md` §6 handed on, plus `📓️window-gaps-followup-2026-09-14.md`
§5's G1 and G2, finds each one's owning layer, fixes it there, and pins it with a law.

- Evidence: `🗑️generated/react-reds/` (recon, per-probe runs, the battery scoreboard).
- Recon added: `🐍️remaining-reds-recon.mjs`, `🐍️german-outline-recon.mjs`.
- **Every product change in this lane is HOST TypeScript.** This lane changed no guest Rust and staged no
  guest, so nothing here needed a restage; the wasm the final battery ran against is the one a peer wave
  staged at 01:26 (§10), not one this lane built.

---

## 0. The six reds, in one line each

| # | red | owning layer | what it turned out to be |
|---|---|---|---|
| 1a | `gaps` · `export-after-generate` click timeout | the PROBE | it aimed at a pixel the window's own pane toggle owns |
| 1b | `gaps` · `doc-panel-select` never published | `🕸️NodeGraph` | the graph surface published NO selection lane at all |
| 2 | `i18n-a11y` · five German graph strings missing | the PROBE | it never opened the panel that paints them; the product was already right |
| 3 | `journey` · `no actor for instance 1` | `🔌️PluginRuntime` | the actor was destroyed before its dispatch queue was retired |
| 4 | `historyJsonPublished: false` | `🏛️ShellHost` / `🛠️ShellHelpers` | nothing published the framework history cursor |
| 5 | G1 · `Escape` leaves a stale outline mark | `🕸️NodeGraph` + `🌐️World3dHost` | TWO host bugs, neither the one the handover named |
| 6 | G2 · the fold control paints over the Inspection tab | `🖼️Panel` / `⚛️react` cap row | the cap row was pinned to the dock body's width |

---

## 1. G2 — the dock's `Collapse` control laid out UNDER its own tab strip

### Measured, before

`🐍️remaining-reds-recon.mjs` → `🗑️generated/react-reds/recon/recon.json`, viewport 1600×1000:

```
cap      [1297, 3, 300, 22]   chips [1361, 3, 144, 22]   chipsScroll 238 / 144
controls [1297, 3,  64, 22]   fold  [1297, 3,  64, 22]
framework.panel.inspection [1267, 2, 92, 22] centre 1313
  → elementsFromPoint: span → button#framework.panel.top-right.fold → div[window-chrome-controls]
  → reachable: FALSE
```

The cap row is `[tab strip | gap | controls]`, laid out right-to-left (`dir="rtl"` on a right-anchored
panel root). The row was `w-full` of a 300 px dock body, minus a 92 px navbar trailing-end reserve and
a 64 px `Collapse` control — leaving the strip 144 px for content that measures 238 px. The strip is
`flex: 0 1 auto` over a container with **visible overflow**, so it kept its own width and simply painted
outside the box flex gave it, across the controls, which carry `z-[2]` and win the hit test. Clicking the
middle of the Inspection tab collapsed the dock.

### Fix — `chromeHostedPanelCapRowStyle`

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` (new, exported) and
`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx` (uses it as the cap row style for EVERY open
chrome-hosted anchor, not just `top-right`). An open chrome-hosted cap sizes to its CONTENT
(`width: max-content`) and never below the body it caps (`minWidth: 100%`), so flex places the fold
control BESIDE the strip. A chrome-hosted cap lives in the shell chrome band with free space beside it —
growing along the inline axis is what that band is for.

### Measured, after

```
cap      [1203, 3, 394, 22]   chips [1267, 3, 238, 22]   chipsScroll 238 / 238
controls [1203, 3,  64, 22]   fold  [1203, 3, 64, 22]
framework.panel.inspection [1267, 3, 92, 22] centre 1313 → reachable: TRUE
all 12 panel tabs reachable (`gaps` · `panel-tab-reachable`)
```

`🐍️react-gap-probe.mjs` gained a `panel-tab-reachable` row: it hovers the Inspection tab to reveal the
controls and asserts every `[data-slot="panel-tab-button"]`'s own centre answers itself under
`elementsFromPoint`. That is the live half of the law — jsdom performs no layout, so the vitest twin
below pins the geometry rule instead.

---

## 2. G1 — `Escape` left a stale outline mark, for two reasons and neither was the one on record

The handover's narrowing ("the clearing turn carries no presence") is **wrong**, and this lane measured
both real causes.

### 2a — `Escape` on a graph surface dispatched a DELETED verb

`🗑️generated/react-reds/outline-selection/console.txt`, between the last arrow and the re-entry:

```
error semio: app "s.procedural.generation3d@1/*#editor" dropped action "setMediaNodeSelection"
  dispatched from window kind "procedural-main": no window kind declares it
```

`handleGraphKeyboard` (`🧱️elements/🕸️NodeGraph/🟦️.tsx`) dispatched `setMediaNodeSelection` on `Escape` —
one of the four action builders ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM **deleted**
when selection became framework-owned (`space_interaction_select`'s own doc names it). No window kind
declares it any more, so the shell dropped every press and the graph's `Escape` cleared nothing at all.
It now dispatches `nodeGraphActions.clearSelection` — the reserved framework verb, added to the graph
scene's own action table in `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` beside `select`/`hover`.

### 2b — a CLEARED leftover overlay still carried the ids it had just retired

With 2a fixed the row STILL stayed marked, and two rows were marked after the next arrow. Instrumented
on the live serve (temporary `[DEBUG]` logs, since removed):

```
turn presence  [["1:framework.panel.artifact","extrusion-axis",false,false]]     ← the guest retires it
leftover apply {publishedIds:["extrusion-axis"], cleared:true, overlayIds:["extrusion-axis"]}
```

So the **guest is exact** — the clearing turn's `turn-result.presence` carries the cleared mark on the
same turn. The residue is a HOST lane: `treeItemToTreeData` ORs `leftoverTreeItemSelectedV1(key, leftoverIds)`
into every outline row's `isSelected`, and `leftoverOverlayCarryingSelectionV1`'s cleared branch returned
`{...utility, selectionCleared: true}` — keeping whatever ids the publication carried. Every OTHER reader
of that overlay already projects a clear as an absence (`applyLeftoverInteractionView` dispatches
`selectedIds: []`; `mergeWorldSelectionWithLeftoverV1` returns `ids: []`), so the object itself was the
one place that held the contradiction.

- `🧱️elements/🌐️World3dHost/🟦️.tsx` — a cleared overlay carries `ids: []`, `gumballActive: false`,
  `gumballAnchorId: null`. It still `applies` (`leftoverWorldOverlayAppliesV1` counts `selectionCleared`),
  so the one publication that exists to REMOVE a selection is still acted on.
- `🧱️elements/🏛️ShellHost/🟦️.tsx` — `clearSelection` retires that overlay on the SAME turn it is
  dispatched, before the round trip, so the row goes idle on the keystroke rather than on the guest's
  answer.

### Rows

`🐍️outline-selection-probe.mjs` on 6018 (the probe lane `window-gaps-followup` wrote and never registered):

| | before | after 2a only | after 2a + 2b |
|---|---|---|---|
| exactly one row is marked at a time | ✓ | ✗ `[1,1,1,1,1,2]` | ✓ |
| `Escape` retires the mark | ✗ `["Vector Evaluated"]` | ✗ `["Vector Evaluated"]` | ✓ `[]` |
| a traversal after `Escape` re-marks exactly one row | ✓ | ✗ two rows | ✓ |
| **total** | **9/10** | **7/10** | **10/10** |

The middle column is worth keeping: fixing only the dispatched verb made the row WORSE, which is how 2b
was found.

---

## 3. `doc-panel-select` — the surface that holds the selection never said so

Clicking `panel:procedural-play-graph/height` in the Artifact panel invoked `interactionSelect`, marked
its row, and fed the Inspection panel — but the step's verdict reads the published selection back, and
the only `[data-selection-json]` in the whole document was the 3D preview's, whose ids are MESH ids
(`extrude@solid`), never node ids. So "selecting through the document panel does not select" could not be
told from "the surface that holds the selection publishes nothing".

`🧱️elements/🕸️NodeGraph/🟦️.tsx` now publishes `nodeGraphSurfaceSelectionDomV1(scene)` on the graph host —
the graph twin of `🌐️World3dHost`'s `data-selection-json`, built from the same `NodeGraphScene` the canvas
paints (`selection`, `highlighted`, `hover`). Measured after:

```
window:procedural-main  {"selectedIds":["height"],"highlightedIds":["height","height@number"],
                         "hoverTarget":{"nodeId":"height","portId":null},"editable":true}
```

The probe's own wait predicate was also wrong: it accepted the TREE ROW's mark as settlement, which the
host paints a beat before the surface re-renders, so it sampled the surface too early. It now waits for
the publication its verdict asserts.

---

## 4. `export-after-generate` — the probe aimed at an occupied pixel

The step failed with `click: Timeout 30000ms` on `[data-surface-id="window:procedural-main"]` at
position `(20, 20)`. `elementsFromPoint` there answers

```
button#framework.window.proceduralMain.engagement.toggle[window-pane-chrome-toggle]
  → div[window-chrome-chip-cap] → div[window-engagement-zone] → div#…engagement[window-engagement-overlay]
```

The window's engagement zone hangs its `Actions`/`Utilities` toggle over the body's leading corner **by
design**; the click was never going to land. `🐍️react-gap-probe.mjs` now focuses the flow window through
`focusMainSurface()` — the low centre band, the same empty aim point its own `clearSelection` helper
already uses. With that, the whole step runs end to end:

```
addGeneration → generate preview meshes 3, phase idle
→ back to edit → Actions pane → action.exportDocument → #format → "STL Mesh"
→ download generation3d.stl, 3810 bytes, head "solid generation3d-previ"
```

**Not a product defect.** Nothing in the export path was changed by this lane.

---

## 5. `i18n-a11y` — the product was already right; the probe never opened the panel

`🐍️german-outline-recon.mjs` (new) switches the locale and OPENS the Artifact panel, checking
`data-active-tab-id` rather than trusting one toggle press. On the same build that scored the row red:

| | English | German |
|---|---|---|
| `graph_nodes` | `NODES` | `KNOTEN` |
| `graph_wires` | `WIRES` | `LEITUNGEN` |
| `graph_input_port` | `… Input` | `… Eingang` |
| `graph_output_port` | `… Output` | `… Ausgang` |
| `status_ok` | `… Evaluated` | `… Ausgewertet` |

All five come from `🗣️terminology/🦀️.rs` through `graph_outline`/`node_status_label`/`node_ports`, and the
Flow window draws its nodes on a GPU canvas with no DOM text at all — so the outline in the Artifact panel
is the ONLY surface that paints them, and the probe read `document.body.innerText` with that panel closed.
`🐍️i18n-a11y-customization-probe.mjs` now opens it (`openArtifactOutline`) for the English baseline, the
German pass and the after-reload pass.

**Still English inside the outline, and NOT claimed as terminology:** the node labels themselves
(`Column Height`, `Polygon`, `Vector`, `ExtrudeCurve`, `Preview`) and the port abbreviations (`Num`, `RA`,
`SI`, `W`, `VE`). Those are DOCUMENT content and operator-catalogue metadata, not UI chrome — a different
owner and a different question.

---

## 6. The teardown race — `no actor for instance 1`

### What it actually was

`🗑️generated/react-verify/journey/console.txt`, the single fault that turned the whole 23/23 walk red:

```
42963  node-graph host unmount surface=window:procedural-main
43330  hot-swap procedural {addedApps: [], removedApps: []}
111279 performInvocation {"instanceId":1,"actionId":"interactionSelect"}
111280 typed-operation completion subscription failed … no channel for instance 1
111351 local interaction observation failed … no channel for instance 1
111358 action failed interactionSelect {targets: [], merge: replace, method: pick}
         Error: … no actor for instance 1
         at requireActorId → Object.enqueue → AppChannelClient.sendCommand
```

Not a role switch. A **hot-swap** destroyed the session-owning instance, and the remounting node-graph
host republished its selection reset onto it 68 s later. `sealedInstancesRef` — the ledger
`dropForSealedInstance` reads — is written by the session-switch gate ONLY, so a hot-swap left every late
arrival to fall through to a stack.

And the stack came from the wrong layer: `adaptPluginHandle.destroyApp` awaited `handle.destroyApp` FIRST
and disposed the channel afterwards. The close ladder runs on the actor's own lifecycle lease and never
speaks through the channel, so for the whole ladder the queue stayed open, admitted the command, and only
`requireActorId` — one layer down, actor already revoked — refused it.

### Fix — retire the queue before the actor, and answer typed

- `🧰️framework/🛍️products/💻️os/🟦️.ts` — `AppChannelClient.retire()`: closes the command queue and settles
  every gesture already in it, WITHOUT ending the outcome subscription. `dispose()` now calls it and then
  ends the subscription, so a close the native side REFUSES leaves the iterator exactly where it was and
  the retry can still retire it (the two standing channel-close laws stay green unchanged).
- `🧱️elements/🔌️PluginRuntime/🟦️.tsx` — `destroyApp` marks the instance retired, `retire()`s the channel,
  THEN destroys the actor, then disposes. `markPluginInstanceRetiredV1`/`isPluginInstanceRetiredV1` (new,
  exported) carry the retirement as a property on the error, so the two gates keep their message text
  verbatim — several probes count `no actor for instance` / `no channel for instance` as their own
  evidence. A channel missing because the instance was RETIRED is typed; a channel missing because
  `createApp` was never called stays a loud failure.
- `🧱️elements/🏛️ShellHost/🟦️.tsx` — `dropForRetiredInstance` reads that answer at the three sites the
  journey's faults came from (`action failed`, `typed-operation completion subscription failed`,
  `local interaction observation failed`): one typed line per late arrival, no stack.

---

## 7. `data-history-json` — the shell now publishes the cursor undo and redo act on

`shellHistoryCursorDomV1` (`🧱️elements/🛠️ShellHelpers/🟦️.tsx`, new) projects the shell's own
`historyProjection` — the reduction of every `HistoryPatch` the guest sent, and the thing that gates the
`framework.history.undo`/`.redo` controls — as `{cursor, canUndo, canRedo, entries, currentCheckpointId,
labels, actionIds, undoLabel, redoLabel}`. `FrameworkOsShellInner`'s root div carries it beside
`data-semio-os-ready`. The label tail is bounded (`SHELL_HISTORY_DOM_LABELS = 8`) because this is chrome
state on a DOM attribute, not the History panel.

Measured live right after the change:

```
data-history-json {"cursor":2,"canUndo":false,"canRedo":false,"entries":2,"currentCheckpointId":null,
  "labels":["snapshot config {…}","Select"],"actionIds":["configApply","interactionSelect"],
  "undoLabel":null,"redoLabel":null}
```

`🐍️editor-verbs-keyboard-probe.mjs`'s `undo`/`redo` rows are no longer decided by "an action id crossed
the console" — a chord bound to nothing at all would also produce that. `historyChordStep` reads the
published cursor before the press, records an `<verb>-armed` row naming whether the history had anything
to act on, and then requires the cursor to MOVE in the right direction (or, on an unarmed history, to stay
put). A red now says whether the chord missed or the history was empty.

---

## 8. Laws

| law | where | result |
|---|---|---|
| retires an instance's dispatch queue before its actor, so a gesture arriving during teardown never reaches the actor | `🧪️tests/🔌️plugin-runtime/🟦️.tsx` | pass |
| keeps a never-created instance a loud failure rather than a retirement | same | pass |
| keeps the exact channel subscribed through refused close and releases only that channel after retry (pre-existing, must still hold) | same | pass |
| settles an old channel close without removing a replacement using the same numeric instance (pre-existing) | same | pass |
| `Escape` on a graph surface clears the FRAMEWORK selection, not a deleted media-graph verb | `🧪️tests/🔬️engine-contract/🟦️.ts` | pass |
| a graph surface publishes the selection it paints, in the same shape the 3D pane does | same | pass |
| the shell publishes the framework history cursor undo and redo are decided by | same | pass |
| a cleared leftover overlay carries no ids, so an outline row it named goes idle on the clearing turn | same | pass |
| an open chrome-hosted dock lays its fold control BESIDE the tab strip, never under it | same | pass |

Commands:

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript
SEMIO_TEST_LEVEL=long bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts \
  --testNamePattern="the shell says what it holds"
SEMIO_TEST_LEVEL=long bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts \
  --testNamePattern="dispatch queue before its actor|never-created instance a loud failure|refused close and releases only that channel|replacement using the same numeric instance"
```

The `elementFromPoint` half of the cap-row law lives in the probe, not in vitest: jsdom performs no
layout, so `elementsFromPoint` there answers nothing. The vitest twin pins the layout RULE (content
sizing, the body floor, and a fails-before overlap computed from the measured 300/238/64/92 geometry);
`gaps` · `panel-tab-reachable` pins the pixel on a real browser.

---

## 9. Files changed

Product (all host TypeScript):
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` — `nodeGraphActions.clearSelection`.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — `chromeHostedPanelCapRowStyle`.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx` — the open chrome-hosted cap row uses it.
- `🧰️framework/🛍️products/💻️os/🟦️.ts` — `AppChannelClient.retire()`; `dispose()` built on it.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` — `Escape` → `clearSelection`; `nodeGraphSurfaceSelectionDomV1` + `data-selection-json`.
- `…/🧱️elements/🌐️World3dHost/🟦️.tsx` — a cleared leftover overlay carries no ids.
- `…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — `shellHistoryCursorDomV1`; the staged-argument form section of the Actions pane and of the command palette is labelled.
- `…/🧱️elements/🏛️ShellHost/🟦️.tsx` — `data-history-json`; `dropForRetiredInstance` at three sites; `clearSelection` retires the leftover overlay on the same turn.
- `…/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — the retirement ledger and typed marks; `destroyApp` ordering.

Laws:
- `…/🧪️tests/🔌️plugin-runtime/🟦️.tsx`, `…/🧪️tests/🔬️engine-contract/🟦️.ts`.

Ticket:
- `🐍️remaining-reds-recon.mjs`, `🐍️german-outline-recon.mjs` (new).
- `🐍️react-gap-probe.mjs` (`panel-tab-reachable`, `focusMainSurface`, `doc-panel-select` wait predicate, `Format` identical-by-design).
- `🐍️i18n-a11y-customization-probe.mjs` (`openArtifactOutline`).
- `🐍️editor-verbs-keyboard-probe.mjs` (`historyChordStep`).
- `🐍️react-battery.mjs` (registers `outline-selection`, which lane `window-gaps-followup` wrote and never registered anywhere).
- `🐍️selection-prune-probe.mjs` — one selector, keyed on `[data-guest-selection-json]`. That probe belongs
  to lane `selection-prune-interact`; its row and its verdicts are untouched. A bare
  `querySelector("[data-selection-json]")` used to be unambiguous because only the 3D pane published that
  attribute, and §3 above makes the Flow window publish it too — FIRST in document order. Repairing the
  selector I made ambiguous is this lane's debt, not theirs.
- `📜️restage-react-reds.sh` (new; see §10).
- this report.

---

## 10. The tree was fine; ONE served module was not

Between 01:1x and 01:41 every boot on 6018 died on `Error: actor-ui-patch.pairing` — the shell mounted
nothing, and `render failed`, `local interaction observation failed`, `readConflicts failed`,
`history snapshot failed` and a page error all carried that same code. A peer wave was landing the
ui-patch BATCH change across `🎭️actor/🚪️lifetime/🩹️patch/{🦀️.rs,🟦️.ts}`, `📮️shard-client/🟦️.ts`,
`⚛️reactor/📨️pending/🦀️.rs` and `🔌️plugin/🖥️host/📥️ui-patch/🦀️.rs`, and the obvious reading was "their
guest half needs a restage". It was not: **their restage ran and the fault stayed.**

`curl` of the SERVED module settled it (`📓️…` memory rule: verify the served module, never the file date):

```
served  if (patchCount !== 0 && patchCount !== 1 || (patchCount === 1) !== (receipt != null)) throw …
disk    if (!Number.isSafeInteger(patchCount) || patchCount < 0 || (patchCount > 0) !== (receipt != null)) throw …
```

The vite serve was holding the PRE-batch transform of `🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts` — the old
one-patch cap — against a guest that had correctly started batching. `touch` on that one file, a re-fetch
through `/@fs/`, and the boot was clean in the same minute (`ROOT_HTML_LEN 179995`, 0 page errors).
A long-lived serve does eventually miss an edit, and it misses it per-module: eight modules this lane had
touched were current while a ninth, nobody here had touched, was hours stale.

One earlier incident in the same window WAS a real half-landed rename and is worth recording:
`submitPluginLifecycleTurn`'s capture switch still named `"issued-ui-ack"` while the `PluginLifecycleWork`
union, `runPluginLifecycleTurn` and the call site had all moved to `"issued-ui-acks"`, so every boot threw
`actor-lifecycle.work-kind` at its first UI-patch acknowledgement. This lane completed that one line
rather than waiting — finishing a peer's own rename, never reverting it.

`📜️restage-react-reds.sh` is left in the ticket unused: two peer restages were already in flight when this
lane went to start one, and a third would only have starved them (`📓️` memory: a full fleet kills long
Rust builds).

---

## 11. Battery

```
SEMIO_BATTERY_URL=http://127.0.0.1:6018/?plugin=generation3d SEMIO_BATTERY_ROOT=react-reds \
  bun 🐍️react-battery.mjs --only=gaps,i18n-a11y,journey,graph-keyboard,outline-selection
```

Scoreboard `🗑️generated/react-reds/scoreboard.json`; stdout `🗑️generated/react-reds/battery-run.txt`,
`battery-gaps-2.txt` (the `gaps` re-run, §11.1) and `battery-keyboard.txt` (`keyboard-verbs`, added to the
same root because §7 changed two of its rows).

**green = 6/6, red = [], 467 s, pageerrors = 0.**

| probe | before | after | seconds | pageerrors |
|---|---|---|---|---|
| `journey` | RED 24/25 (`no actor for instance 1`) | **green 25/25** — 23 converged rows, all 8 examples × both roles | 110 | 0 |
| `i18n-a11y` | RED 5/7 (`2-edit-de`, `6-after-reload`) | **green 7/7** | 61 | 0 |
| `gaps` | RED 11/13 (`export-after-generate`, `doc-panel-select`) | **green 14/14** (12 rows + the two gate steps) | 195 | 0 |
| `graph-keyboard` | 13/13 on :6023, never run here | **green 13/13**, `escape-clear` now mints `clearSelection` | 41 | 0 |
| `outline-selection` | 8/9 on :6023, registered NOWHERE | **green 11/11**, registered in the battery | 35 | 0 |
| `keyboard-verbs` | green 12/12 but `historyJsonPublished: false` | **green 14/14**, `historyJsonPublished: true` | 25 | 0 |

`gaps` gained two rows: `panel-tab-reachable` (G2's live `elementsFromPoint` verdict, 12/12 tabs reach
their own centre) and, from §7, `keyboard-verbs` gained `undo-armed`/`redo-armed`.

### 11.1 What the first `gaps` run found, and why it was the probe again

The gate's first pass scored `gaps` 11/14 with 3 page errors: `wire-undo` failed to cut at all, and a
`FlowMessageRejected` landed 1.2 s into that step. `wire-undo` had been GREEN yesterday — and the reason it
had been green is that `export-after-generate` used to ABORT before opening the window's Actions pane.
Once the export really runs, it leaves that pane EXPANDED over the flow canvas, and the next step drags on
a port handle the graph paints underneath it. This probe's own header states the rule ("any step that opens
a side panel folds it again afterwards"); the export step was the one that did not. It now folds the pane
and records `paneFolded` in its own detail. Re-run: `gaps` 14/14, 0 page errors, `wire-undo` green,
`FlowMessageRejected` gone with it.

The same first pass also carried a guest panic at teardown —
`ordered-map root must be explicitly retired before drop` at
`📡️replication/🌱️value/🗂️ordered/🦀️.rs:81` inside `flow-extension-brep`, followed by
`shard 0 worker fault [handler/turn] … unreachable`. It did not reappear in the re-run or in any of the
other five probes. **Not diagnosed, not claimed** — it is a guest Rust panic on a wasm restaged by a peer
wave an hour earlier, and this lane changed no Rust at all.

---

## 12. Not claimed

- **Two of the six reds were PROBE defects, and are reported as such.** `export-after-generate` (§4) and
  `i18n-a11y` (§5) needed no product change: the export path and the German terminology tables were
  already correct, and the probes could not see them. Nothing in the export or terminology code was
  touched to make either row green.
- **The `undo`/`redo` chords were never exercised against an ARMED history.** §7's rows now read the real
  cursor, and on this app's quiet boot it reads `canUndo: false, canRedo: false` with 6 entries (config
  snapshots, which are not user-undoable), so both rows took the unarmed branch: the chord invokes its
  verb and the cursor stays put. The `-armed` rows make that visible instead of hiding it. That the ARMED
  branch moves the cursor is pinned by the vitest law only, not by a browser run.
- **The `elementFromPoint` half of the cap-row law is in the probe, not in vitest.** jsdom performs no
  layout. The vitest twin pins the layout rule and a fails-before overlap computed from the measured
  geometry; `gaps` · `panel-tab-reachable` pins the pixel on a real browser.
- **G2 is fixed for the `top-right` dock as measured; the other seven anchors are covered by the same code
  path but were not measured.** `chromeHostedPanelCapRowStyle` applies to every open chrome-hosted anchor
  (the old reserve applied to `top-right` only); only the right dock was observed overlapping.
- **The guest panic in `📡️replication/🌱️value/🗂️ordered/🦀️.rs`** (§11.1) is neither diagnosed nor fixed.
- **`every pane of one document names its OWN window surface` fails in `🔬️engine-contract` on this tree,**
  and it is not this lane's: `worldSurfaceSelectionDomV1` gained `selectionMode`/`showEdges` and the
  fixture `🪪️world-surface-identity.json` was not updated with them. Measured red both before and after
  every change in this lane.
- **`data-selection-json` is now published by TWO surface hosts.** That is the point of §3 — a surface
  publishes the selection it paints, and `data-surface-id` disambiguates — but it makes a bare
  `querySelector("[data-selection-json]")` answer the Flow window rather than the 3D pane. Exactly one
  probe in the repo did that (`🐍️selection-prune-probe.mjs`, lane `selection-prune-interact`); its selector
  is repaired, its row and verdicts untouched. No other consumer in `🧰️framework` or in any ticket does.
- **`flow-wire`, `menus`, `status-states`, `panel-i18n`, `inspection-i18n`, `viewer-actions`,
  `toolRunPace`, `role-switch` and `interact` were not run and not touched.** One incidental
  cross-observation for the `menus` lane, from §4's own evidence rather than from their row: on this tree
  `action.exportDocument` DOES open its format list and DOES produce a real file — `STL Mesh` →
  `generation3d.stl`, 3810 bytes, `solid generation3d-preview`.
- **The `journey` fix is proven by absence, not by reproduction.** The fault it removes came from a
  hot-swap tearing down the session instance mid-run; this lane did not force a hot-swap. What is proven
  directly is the LAW (§6/§8) and that 25/25 journey steps, including the shell-fault gate, are green.
- **wgpu is untouched.** This lane owned port 6018 only.
