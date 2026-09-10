# Wave G — Shell Tool Tab Pressed State & Tool Category Reveal

Ticket: `2026/09/02/PUZZLE-3D-END-TO-END`
Wave: **W-G** (React renderer shell defects)
Date: 2026-09-09/10
Surface: `http://localhost:6013/?plugin=puzzle3d` (framework/os/dev react renderer, HMR)

## 0. Assignment

Two browser-verified defects handed over from the runtime-verification pass:

1. **Fill tab pressed state is wrong on first use** — after boot, pressing the ribbon tab `tool.fill`
   dispatches `setActiveTool { toolId: "" }` (deactivate) although no tool is active; a second press
   activates. Suspected conflation of "selected panel tab" with "tool active".
2. **The Tool category needs two clicks** — the first press on `framework.category.tool` produces one
   `handleAction` and no `tool.fill` element; only the second press renders the Fill tab.

## 1. Ground truth — live browser reproduction (2026-09-09, port 6013)

All observations below were taken from the live renderer, not from reading code.

### 1.1 Boot state (profile with persisted dock UI)

`localStorage["semio.os.config"].dockUi.apps["s.puzzle.puzzle3d@1/*#editor"]`:

```json
{"version":3,"anchors":{
  "top-left":{"visible":true,"path":["framework.panel.artifact"]},
  "bottom-right":{"visible":true,"path":["framework.panel.history"]},
  "bottom-middle":{"visible":true,"path":["framework.category.tool","tool.fill"]}},
 "pathMemory":{"framework.category.tool":"…"}}
```

So the per-anchor **`visible` + `path` are persisted** and restored on boot.

DOM probe right after boot, before any click:

| element | attribute | value |
| --- | --- | --- |
| `button[data-slot=panel-tab-button]#framework.category.tool` | `data-active` | `true` |
| `button[data-slot=panel-tab-button]#tool.fill` | `data-active` | `true` |
| `button[data-slot=toggle-group-item]#tool.fill` | `aria-pressed` | **`false`** |

→ the Fill **leaf tab renders selected while the tool is not active**. This is the desync described in
defect 1. Note also that `tool.fill` is a **duplicate DOM id**: the panel tab button and the in-tree
`Toggle` both carry it.

### 1.2 Press sequence on the Fill tab (persisted profile)

| press | result |
| --- | --- |
| 1st on `#tool.fill` (tab button) | leaf **collapses** (`data-active` → `-`), tree toggle disappears, tool still inactive, history notes `Switch Panel T…` (one `noteShellCommand` → one `handleAction`) |
| 2nd on `#tool.fill` | leaf selected again **and** `toggle-group-item#tool.fill aria-pressed=true` → tool active |

→ "a second click then activates the tool" reproduced exactly.

### 1.3 Press sequence on the Tool category (persisted profile)

| press | result |
| --- | --- |
| 1st on `#framework.category.tool` | panel **folds**: `tool.fill` disappears from the DOM, one `handleAction` (the `shell.panelToggle` note) |
| 2nd on `#framework.category.tool` | panel reopens, `tool.fill` back, tool active |

→ "two clicks before its ribbon items appear" reproduced exactly.

### 1.4 Cleared profile (control)

After `localStorage.clear()` + reload, `bottom-middle` boots folded with an empty path. **One** press on
`#framework.category.tool` opens the panel and reveals the `tool.fill` leaf tab
(`dockUi… bottom-middle = {"visible":true,"path":["framework.category.tool"]}`). The category itself is
therefore *not* two-click-broken on a clean profile; the two-press symptom is produced by the persisted
open-on-the-Tool-category state plus the desync of 1.1.

### 1.5 Every `onAction` dispatched by each press (temporary `[DEBUG] onAction` probe, removed again)

A temporary `console.log("[DEBUG] onAction", …)` at the head of `ShellHost`'s `onAction` funnel recorded
the exact dispatches (the probe has been removed from the tree again):

| press (persisted profile, before the fix) | dispatches |
| --- | --- |
| `framework.category.tool` #1 | `noteShellCommand shell.panelToggle {anchor:"bottom-middle", visible:false}` — **one** action, the fold |
| `framework.category.tool` #2 | `noteShellCommand shell.panelToggle {… visible:true}` — the reopen |
| `tool.fill` #1 | `noteShellCommand shell.panelTab {tabId:"framework.category.tool"}` — the leaf collapse, **no `setActiveTool` at all** |
| `tool.fill` #2 | `setActiveTool {toolId:"fill"}` + `noteShellCommand shell.panelTab {tabId:"tool.fill"}` |

So the handover's "dispatches `setActiveTool` with `toolId: ""`" is not produced by the ribbon tab: the tab
press produces no `setActiveTool` at all on the first press. The `toolId: ""` dispatch comes from the
**second** `tool.fill` element — the `Toggle` inside the tool tree (`ShellHelpers` line 3804), which the
duplicated element id makes indistinguishable from the tab. Once the tab finally armed the tool, that
toggle rendered *pressed*, and pressing it dispatched `{toolId: ""}`. Both halves of the reported symptom
therefore come from the same conflation.

The "8 sequential `handleAction` calls of increasing latency" are not part of a press at all: with the Fill
tool armed, `World3dHost`'s ticker dispatches `fillBuildTick {surfaceId:"1"}` about twice a second (probe:
`94332, 94333, 95332, 95333, 96332, 96333 ms` — note the **duplicated pairs**), and their latency grows
while the main thread is busy. That over-firing is a pre-existing, separately-tracked defect (already
measured by a peer wave: "252 `fillBuildTick`s in 35 s, 38 rejected with queue is full",
`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:782` and `…/🌐️World3dHost/🟦️.tsx:4517`) and is out of W-G's scope.

## 2. Root causes

**RC-1 — tool activation hung off the press callback, so it was skipped by every other route into the path.**
`ShellHost`'s `buildPanelSelectionProps.onActiveTabPathChange` was the ONLY place that turned a selected
`tool.<id>` leaf into `setActiveTool`. A path that arrives any other way — the `DockUiStateStore`
arrangement restored on boot, the introduction's `SET_PANEL_PATH`, a dock reset, a program `setActiveTool`
effect — left the two representations of one state disagreeing: leaf selected, tool not armed. The user's
first press on that leaf was then read by `progressPanelTabSelection` as a *re-press of the active
segment*, which collapses it (correct ribbon behaviour) — so the press that should have armed the tool
disarmed the tab instead, and only the following press armed it.

**RC-2 — the tool tree rendered a second activation control carrying the leaf tab's own element id.**
`buildToolTree`'s non-active branch rendered `<Toggle id={`tool.${tool.id}`} pressed={isActive} …>`, i.e.
a second element with the id `tool.fill` (the panel tab button already owns it — the tutorial/introduction
targets that id, and `document.getElementById` resolved to whichever came first). Its `onPressedChange`
dispatched `{toolId: ""}` whenever it was pressed while pressed, which is the reported dispatch.

**RC-3 (defect 2) — the "two clicks" is RC-1 seen through the persisted-open panel.** The restored
arrangement has `bottom-middle` already open on the Tool category, so the first press on
`framework.category.tool` is an active-root re-press and folds the panel (one `handleAction`), and the
second reopens it. That fold is the deliberate ribbon rule (`progressPanelTabSelection`: "a root re-press
always folds the panel"); what made it read as "the category needs two clicks" is that the category, while
open, looked inert — the Fill tab was selected but the tool was not armed and no measures rendered (RC-1).
On a cleared profile a single press has always opened the category (§1.4).

## 3. Fix

One state, one owner: **the `tool.<id>` leaf tab selected under the Tool category IS the mode's active
tool**, and a single reconciliation pass keeps the two representations equal whatever moved either of them.

### 3.1 `…/🧱️elements/🛠️ShellHelpers/🟦️.tsx`

* `buildToolTree(tool, measures, onAction)` — the activation-`Toggle` branch is **deleted** (with it the
  duplicate `tool.<id>` element id and the `{toolId: ""}` dispatch). A tool tree is now always exactly its
  own `tool.<id>.options` section; a tool whose program published no measures yet resolves to an empty
  section instead of a contradictory second control. `controllerId`/`isActive` are no longer parameters.
* `buildToolTabs(tools, toolMeasuresByToolIdRef, onAction)` — `controllerId` and `activeToolIdRef` dropped
  (the tree no longer depends on which tool is active), and `SET_ACTIVE_TOOL_ACTION_ID` is no longer
  imported by this module.
* **new** `reconcileToolTabSelection(previous, activeToolId, selectedToolId)` → `{ next, effect }`, the
  pure last-change-wins rule, plus the exported `ToolTabSelection` / `ToolTabSelectionEffect` types:
  * the active tool moved → `{kind:"select", toolId}` (the panel path must follow it; `null` collapses to
    the Tool branch),
  * else the selection moved → `{kind:"activate", toolId}` (`null` ⇒ `setActiveTool {toolId:""}`),
  * else `{kind:"idle"}`.
  Both outcomes record the value they are about to establish, so the follow-up pass is idle and the two
  halves cannot bounce; a refused activation self-heals on the next pass by deselecting the leaf.

### 3.2 `…/🧱️elements/🏛️ShellHost/🟦️.tsx`

* The ad-hoc activation inside `buildPanelSelectionProps.onActiveTabPathChange` is **removed** — a press is
  no longer a privileged route.
* **new** effect in `🧭️DockAssembly` (`toolTabSelectionRef`): resolves the anchor that hosts
  `FRAMEWORK_CATEGORY_TOOL_ID` (or the mobile panel), and while that branch is the active root feeds
  `activeToolId` + `toolIdFromPanelTabId(path.at(-1))` through `reconcileToolTabSelection`, dispatching
  either `setActiveTool` (through the normal `onActionStable` funnel, so plugin forwarding, utility mutual
  exclusion and introduction completion all still run) or `SET_PANEL_PATH`/`SET_MOBILE_PANEL_PATH`.
  While the Tool category is *not* the active root the pass returns without touching the ref, so an armed
  tool survives browsing the Command palette and is reconciled again on re-entry.
* `toolTabs` memo now depends on a plain `hasToolSession` boolean instead of `session.app.controllerId`,
  keeping its (and `defaultDock`'s) identity across session churn.

### 3.3 `…/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`

`reconcileToolTabSelection`, `ToolTabSelection`, `ToolTabSelectionEffect` added to both the internal import
list and the public re-export.

## 4. Laws (`…/🧪️tests/🔬️engine-contract/🟦️.ts`, suite `resolveModeTools / buildToolTabs`)

| law | fails before | asserts |
| --- | --- | --- |
| `no tool tree carries a control that duplicates its own leaf tab id` | **yes** — verified by temporarily restoring the old toggle: `AssertionError: expected 'tool.fill' not to be 'tool.fill'` | RC-2 can never come back |
| `reconcileToolTabSelection arms the tool a restored dock arrangement already selects` | yes (rule did not exist) | RC-1/RC-3: a hydrated `["framework.category.tool","tool.fill"]` selection yields `{kind:"activate", toolId:"fill"}`, then idle |
| `… disarms the tool when its leaf collapses, and re-arms on the next press` | yes | collapse ⇒ `setActiveTool ""`, next press ⇒ armed in ONE press |
| `… selects the leaf of a tool armed by the program, and collapses when a utility clears it` | yes | the other direction (program/utility ⇒ tab) |
| `… self-heals a refused activation instead of looping` | yes | no bounce between the two halves |
| `one press on the Tool category reveals its remembered tool leaf and arms that tool` | yes (activation half) | `progressPanelTabSelection` + reconciliation: memoried drill-down ⇒ `tool.fill` ⇒ armed, one press |
| `buildToolTabs builds one leaf per resolved tool, whose lazily-resolved tree carries that tool's own measures` | rewritten for the new signature | a measure-less tool resolves to an **empty** `tool.<id>.options` section |

## 5. Verification

Commands (cwd `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react`):

| command | result |
| --- | --- |
| `bun ./📜️script.ts typecheck` | **821 errors, 0 of them in any file W-G touched** (ShellHost, ShellHelpers, engine-contract, renderer react target index all clean). All 821 are pre-existing peer breakage — top offenders `🦑️repo …/📜️script.ts` (190), `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement` (152), `🧪️tests/🧪️docklayoutstore` (84). |
| `bun ./📜️script.ts test` (fundamental) | **1 passed, 4 skipped (5)** |
| `bun ./📜️script.ts test long --run 'engine-contract'` | **467 passed (467)**, 1 file |
| `bun ./📜️script.ts test long` (all 19 engine suites) | **751 passed (751)**, 19 files |
| fails-before proof | old toggle shape temporarily restored → `1 failed | 466 skipped`, then restored → `467 passed` |

### 5.1 Live browser re-verification (`http://localhost:6013/?plugin=puzzle3d`, HMR)

Seeded profile `bottom-middle = {visible:true, path:["framework.category.tool","tool.fill"]}`:

| step | before W-G | after W-G |
| --- | --- | --- |
| boot | Fill tab selected, tool **not** armed, body shows an unpressed `Fill` toggle | Fill tab selected **and armed**, body shows the real measures: `Count 0 · Hexagonal Cut · Concrete Forest · Left 100% · Distribution` — **zero presses** |
| `document.querySelectorAll('[id="tool.fill"]')` | 2 elements (tab button + toggle) | **1** element (`BUTTON/panel-tab-button`) |
| press Fill (armed) | collapse only, no `setActiveTool` | collapse **and** `setActiveTool {toolId:""}` |
| press Fill again | armed | armed — one press, measures back |
| press `framework.category.tool` | folds (1 action), category looked inert on reopen | folds (1 action); reopen restores Fill **armed with measures**, tool never disarmed by folding |
| cleared profile: press `framework.category.tool` | opens, leaf tabs revealed, body empty | unchanged — opens in ONE press (`panelToggle` + `panelTab`), no spurious `setActiveTool`, no reconciliation loop |

The Fill measures had never rendered before this wave *because the tool was never armed* — arming it is what
makes `toolMeasuresByToolId["fill"]` arrive, which is why deleting the in-tree toggle leaves no hole.

## 6. Findings handed on (out of W-G scope)

1. **`fillBuildTick` fires in duplicated pairs** (`…94332, 94333, 95332, 95333…`) once Fill is armed — the
   same over-firing a peer already measured (`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:782`,
   `…/🌐️World3dHost/🟦️.tsx:4517`: 252 ticks in 35 s, 38 rejected as "queue is full"). This, not the tab
   press, is the source of the "8 sequential `handleAction` calls of increasing latency".
2. **`[DEBUG] setActiveTool failed plugin-ui.intake-budget-exhausted:1:puzzle3d-main-top:69632`** is logged
   while arming Fill; the arming itself still lands (measures render). UI intake budget, another wave.
3. **`plugin-ui.section-root-mismatch:#0`** errors keep flowing from `ShellHost:3676` during refresh in this
   session — unrelated to the tool tabs, but visible in every console capture above.
4. On a **first-ever** profile the Tool category still needs a second press to pick a specific tool, because
   `progressPanelTabSelection` deliberately does not auto-descend into a branch with no drill-down memory
   ("no first-sibling substitution, no auto-descend"). Auto-descending was rejected here: it would arm the
   first tool the moment the category opens and would defeat the introduction's tool-pick step
   (`ShellHost`: "never select the leaf path (selecting auto-activates and would celebrate before they
   act)"). After the first pick the memory makes it one press forever, which is the case the law covers.

## 7. Files changed

* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
* this report + a `W-G` section in `📋️master-plan-2026-09-08.md`

No `[DEBUG]` probe remains in the tree (the `onAction` funnel log used for §1.5 was removed and the
typecheck/test runs above were made after its removal).

## 8. W-G3 fix (wasm #31 empty tool body + remaining one-press defects)

Wave: **W-G3** (2026-09-10). Fresh probe `🗑️generated/probe-2026-09-10T04-32-25.md` + screenshot
`🗑️generated/probe-2026-09-10T04-32-25-fill-tab.png`: Tool panel open, Fill tab selected, **body empty**
(no slider, no measures). Guest `tool_measures` always publishes `fill` (native bijection + fill family
59/0). Packed reserved-section unpack (`sectionValueFromBuiltNode` + `packedTextLeaf`) already lands
the tools JSON (`framework.section.tools`, 2581 B). The drop was **after** the cache write.

### 8.1 Defect 1 — empty tool body

**Cause.** `buildToolTabs` keeps tab object identity and lazy-reads `toolMeasuresByToolIdRef`. The
desktop floating Tool pane is `PanelTreeUnitsPane` (`reactHostPort.memo`). First resolve (boot / tools
surface not yet retained / first Fill select) sees `{}` and renders `tool.fill.options` with no items.
W-G wired `treeContentRevision` only on the **mobile** panel. Desktop `buildPanelProps` never passed
it, so when `SET_TOOL_MEASURES_BY_TOOL_ID` later merged the guest map the memoized pane never
re-called `resolveTree`. Probe sliders/measures stayed `[]`.

Not a packed-leaf drop: tools go through the same reserved-section walk W-X already packed/unpacked.
Not a window/panel id keying bug: the leaf id is still `tool.fill` / measures key `fill`.

**Fix.**

| File | Lines | What |
| --- | --- | --- |
| `…/ShellHelpers` | :3943 | `toolPanelTreeContentRevision` — shared identity for desktop + mobile |
| `…/ShellHost` | :8507, :9156 | mobile + **desktop** `buildPanelProps` pass that revision (`toolMeasuresByToolId` in deps) |

**Law (fails before / passes after).** `late-arriving fill measures change the tool panel tree revision so the memoized pane re-resolves` — empty ref resolves to `[]` items; after the fill slider lands, the same tab's `resolveTree` has one item **and** the revision identity changes (the memo bust `buildPanelProps` was missing).

### 8.2 Defect 2 — first press on `#tool.fill` must activate when inactive

**Cause (still true after W-G).** `progressPanelTabSelection` treats a re-press of the already-selected
leaf as a collapse (`path` walks up to `framework.category.tool`). W-G's `reconcileToolTabSelection`
then sees `selected=null` and dispatches `setActiveTool {toolId:""}`. A follow-up pass before
`activeToolId` actually updates also treated "pending activate fill" as "active moved to null" and
bounced (`select null`).

**Fix.**

| File | Lines | What |
| --- | --- | --- |
| `…/ShellHelpers` | :3968 | `toolLeafInactiveRepress` — collapse of an **inactive** tool leaf keeps the previous path and names the tool to arm |
| `…/ShellHost` | :8550 | `onActiveTabPathChange` applies that helper and dispatches `setActiveTool` |
| `…/ShellHost` | :8454–:8477 | `pendingToolActivateRef` — do not re-enter reconcile until `activeToolId` matches the in-flight activate (or the user moves the path) |

**Law.** `one press on an inactive selected Fill leaf arms the tool instead of collapsing it` — the
ribbon collapse path is still `[framework.category.tool]`; the helper returns `{path: previous, toolId:"fill"}`
when `activeToolId` is null, and `null` when Fill is already armed (so an armed re-press still folds).

Existing W-G reconcile laws unchanged (self-heal / restore / program-select).

### 8.3 Defect 3 — one press on `#framework.category.tool` opens with the Fill tab

**Cause.** `usePanelTabSelection` on a **closed** host swallows a re-select of the current tail
(`raw.last === resolved[raw.length-1]`) so the panel opens without running `progressPanelTabSelection`.
A first-ever / category-only path (`["framework.category.tool"]`) therefore never memory-drilled or
first-child-descended. Second press (now visible) was an active-root re-press and **folded**. That is
the two-click. (A persisted-open category still folds on re-press — that is the ribbon rule; with
defect 1 fixed the open panel is no longer an empty corpse.)

**Fix.** Keep the swallow for a closed host whose path already has a leaf (existing UI laws). When the
closed path is category-only **and** `drillOnOpen` is set (bottom-middle only):

| File | Lines | What |
| --- | --- | --- |
| `…/PanelTabBar` | :178, :217–:219 | optional `drillOnOpen`; applied only on closed + `resolvedPath.length <= 1` |
| `…/ShellHelpers` | :3956 | `toolCategoryOpenPath` — remembered child, else first tool leaf (`tool.fill`) |
| `…/ShellHost` | :8577 | bottom-middle `drillOnOpen` → `toolCategoryOpenPath` |

**Laws.** Existing `one press on the Tool category reveals its remembered tool leaf and arms that tool`
(memory drill via `progressPanelTabSelection([], …)`). New `one press that opens the Tool category
selects Fill when no leaf is remembered` — `progress` stays on the category; `toolCategoryOpenPath`
appends `tool.fill`.

UI regression check (must stay green): `press on a closed host opens it without changing the active path`,
`unfolding restores the stored path`, `picking a tab opens a folded panel`, `PanelTreeUnitsPane re-resolves lazy trees` — **4 passed**.

### 8.4 Verification (helpers — still green; did not close the browser loop)

The §8.1–8.3 helpers remain required, but they modeled a path the **release browser did not take**.
Measures already arrived and `resolveTree` already built Fill items. The pane never mounted because
the selected path stayed on the Tool **branch**.

### 8.6 Live :6014 loop — actual empty-body cause (browser-proven)

Serve: `http://127.0.0.1:6014` (fresh vite, current renderer TS, wasm #31). Probe:

`bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts" --interact --fill --settle=10 --port=6014`

Coordinator empty-body artifacts: `probe-2026-09-10T04-32-25*`, `probe-2026-09-10T04-56-02*`.

DEBUG probe `🗑️generated/probe-2026-09-10T05-00-42.md` proved:

1. Guest measures **do** arrive (`tools section admitted`, `fill: 2`).
2. `resolveToolTree` builds `measureCount: 2, itemCount: 2`.
3. `PanelTreeUnitsPane` resolves `{tabId: tool.fill}` only while the Fill **leaf** is active.
4. Snapshot: `data-active-tab-id=framework.category.tool`, **h=47**, `treeItems=0` — chrome row only (`Tool | Command | Collapse | Fill`).

`Panel` / `LayoutMobilePanel` mount trees only when `activeNode.kind === "leaf"`. A branch active node
⇒ empty hugging strip. The empty strip is the Tool **panel body**, not a Command bar.

`usePanelTabSelection` on a **closed** host with an **empty** current path (`resolvedPath=[]`):

- swallow is `raw.last === resolvedPath[raw.length-1]` → `"framework.category.tool" === undefined` → **false**
- `drillOnOpen` ran only inside that swallow branch
- fallthrough `progressPanelTabSelection` does not auto-descend a branch with no memory → path stays `["framework.category.tool"]`
- DEBUG: `handlePathChange closed {raw: Array(1), resolvedPath: Array(0), swallow: false, hasDrill: true}`

`buildPanelProps` already spread `drillOnOpen`, but `Panel` dropped it and created its own hook without the callback.

### 8.6.1 Fix

| File | What |
| --- | --- |
| `…/PanelTabBar` | `drillOnOpen` on **every** closed-host open (swallow **and** empty-path fallthrough). Existing closed-host UI law unchanged (that test does not pass `drillOnOpen`). `resolvePanelBranchBodyLeaf` — a branch body is the remembered (else first) descendant leaf. |
| `…/Panel` | Forwards `drillOnOpen`. Renders `bodyLeaf` trees when the active node is a branch. |
| `…/Layout` | Same branch-body resolve for the mobile panel. |
| `…/ui-react` barrel | Re-exports `resolvePanelBranchBodyLeaf`. |

**Law.** `a Tool category branch still exposes the remembered Fill leaf trees as the panel body`.

DEBUG confirmation `🗑️generated/probe-2026-09-10T05-08-05.md` + `probe-2026-09-10T05-08-05-tool-category.png` / `-fill-tab.png`: after **one** `#framework.category.tool` press, `tab=tool.fill`, `h=119`, `treeItems=3`, body text `Count | 0 | Hexagonal Cut Concrete Forest Left | 100% | Distribution`. Fill press kept the leaf (inactive-repress). `[DEBUG]` logs then stripped.

### 8.7 Final clean probe + suite tails

| command | result |
| --- | --- |
| renderer-react vitest tool-tab laws (incl. new branch-body law) | **12 passed** / 583 skipped. Foreign: `package-integration` `self is not defined` |
| UI vitest closed-host / revision / fold | **4 passed** / 697 skipped. Foreign: storybook `bun:sqlite` |
| `bun x tsc --noEmit` ui-react | **336 errors, 0 in touched files** (PanelTabBar / Panel / Layout) |
| `bun x tsc --noEmit` renderer-react | **820-class errors, 0 new in touched files** after the `resolvePanelBranchBodyLeaf` annotation rewrite |
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -j 4 -- --test-threads=1` | **639 passed; 1 failed** — `two_instances_converge_disjoint_object_edits_via_backbone` (known foreign) |
| Final clean probe | `🗑️generated/probe-2026-09-10T05-19-26.md` + `probe-2026-09-10T05-19-26-tool-category.png` / `-fill-tab.png` / `-fill-wait-ready.png` |

Final probe (no `[DEBUG]`): one press `#framework.category.tool` → `tab=tool.fill`, `h=119`, `treeItems=3`, body `Count | 0 | Hexagonal Cut Concrete Forest Left | 100% | Distribution`. Same after `#tool.fill` and after 30s. `faults=0`.

Open: probe `sliders`/`ranges` stay `[]`/`0` because Count is not `input[type=range]` (custom tree control). In-tree `toggle-group-item#tool.fill` stays absent (W-G removed the duplicate id). No wasm rebuild. `:6013` untouched.

