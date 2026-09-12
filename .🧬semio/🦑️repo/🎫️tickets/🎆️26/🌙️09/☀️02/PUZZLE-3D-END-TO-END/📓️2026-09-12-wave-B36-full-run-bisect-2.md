# Wave B36 — full-run bisect, round 2: the panel-tab toggle, the panel that renders a default, and the add that does not select

Ticket 26/09/02/PUZZLE-3D-END-TO-END · wasm #53 on `:6013` (host vite-live, incl. B33) · every probe run
foreground with the `pgrep` gate checked first · method: B33's growing `--only=` prefixes in one browser, plus
one purpose-built isolator, `🔍️b36-pollution.ts` (input, kept), that reads the DOM directly so a claim is a
measurement and not an inference.

---

## 0 Headline

The 13 verdicts I was handed do **not** have 13 causes. They have four, and only two of them are product:

| # | cause | verdicts it owns | class |
|---|---|---|---|
| A | **A panel tab is a TOGGLE, and the probe uses a tab click as "ensure open"** — so any predecessor (or the shell's own auto-reveal on selection) that already left the panel open makes the probe COLLAPSE it | `inspection-object-fields`, `inspection-locked-flag-row`, `locked-flag-row`, `locked-refusal-notice`, `camera-emits-no-artifact-history`, `outliner-hide-control-present` | probe (recipes §7) |
| B | **The Settings panel renders a per-window option from `Puzzle3dWindowConfig::default()` while writing to the focused pane** | `settings-value-reaches-window-rail` | **product — fixed, §3** |
| C | **`addObjectKind` never selected what it added** (and the probe's `selectionState()` reads a field that never holds a selection) | `catalogue-add-selects-new-object` (+ `duplicate-reselects-clone`, not mine) | **product — fixed, §4** + probe (§7.4) |
| D | probe locators that can never hit their target on this shell: the language control lives in a panel the probe never opens; `projection-options` nudges a *pane*, not a control | `locale-control-present`, `projection-repaints-camera` | probe (§7.2, §7.3) |

And three of the 13 are **not full-run-only at all** — they fail in a fresh single-step lane on #53 too, so the
premise "passes in a fresh lane" is stale for them (§2.2): `locale-control-present`, `clipboard`, and (in the
lane that matters) `delete-selection`'s sibling `duplicate-reselects-clone`.

---

## 1 The bisect tables

One probe invocation per row, one browser each, `bun 🔍️browser-probe.ts --only=… --port=6013`.

### 1.1 The inspection family (`inspection-object-fields`, `inspection-locked-flag-row`)

| # | prefix (`--only=`) | verdict | run |
|---|---|---|---|
| L0 | `selection-surfaces` | **PASS** `populated=true id=Id seed-left-001 lock=true` | `probe-2026-09-12T05-01-11` |
| L1 | `pick-object,context-menu,tool-category,fill-tab,fill-abort-engagement,fill-wait-ready,fill-apply-max,fill-history,selection-surfaces` | **FAIL** `populated=false id=null` | `05-01-44` |
| L2 | `pick-object,context-menu,selection-surfaces` | **FAIL** | `05-04-27` |
| L3 | `pick-object,selection-surfaces` | **FAIL** | `05-05-12` |

**The polluting predecessor is `pick-object` — one canvas click.** The whole fill suite is innocent (L2 = L3).

What L3's own console says (`probe-2026-09-12T05-05-12.md`, console tail lines 115-121, 171):

```
leftover InteractionView {"selectedIds":["seed-left-001"],…,"gumball":true}     ← the pick landed
leftover Inspection tab {"anchor":"top-right","path":["framework.panel.inspection"]}
leftover Inspection refresh {"epoch":1,"selectedIds":["seed-left-001"],…}       ← the SHELL revealed the panel
…
history patch applied {…,"labels":["Toggle Panel"],"canUndo":true}              ← the probe's tab click CLOSED it
```

The selection is live and correct the whole time (`selectedIds:["seed-left-001"]`, `gumball:true`); it is the
panel that is gone. Measured head-on with the isolator:

```
bun 🔍️b36-pollution.ts --case=inspection-toggle
VERDICT after-first-pick            selected=[{"selectedIds":["seed-left-001"],"activeUtility":"select"}, …] inspectorRows=12
VERDICT after-inspection-tab-click  inspectorRows=0  selected=[… "seed-left-001" …]  text=null
VERDICT after-second-tab-click      inspectorRows=12
```

So: **the shell auto-reveals Inspection on a selection change; the probe then clicks that tab, which toggles the
panel shut.** Fresh `--only=` lanes pass because the click wins the race against the reveal; under a full
battery's load the reveal lands first and the click inverts it. Same mechanism, same fix, for
`locked-refusal` (`🔍️browser-probe.ts:1405` clicks `#framework.panel.inspection` right after its own pick) —
`locked-flag-row lockChrome=false`, then `lock controls=0`, then `locked-refusal-notice locked=false`.

### 1.2 The history family (`camera-emits-no-artifact-history`)

`readHistoryEntryIds` (`🔍️browser-probe.ts:1863`) clicks the History tab and reads. The `camera-gestures` step
calls it twice, and `history-open` has already left that tab active:

```
bun 🔍️b36-pollution.ts --case=history-toggle
VERDICT after-first-history-click  entries=4
VERDICT after-second-history-click entries=0      ← toggled shut
VERDICT after-third-history-click  entries=6
```

That is exactly #53's `before=0 after=7`: the "before" read closed the panel, the "after" read reopened it, and
`newHistoryEntries` then reported seven PRE-EXISTING rows (`entry.1 = Set Active Example`, `entry.2 = Resize
Window`, `entry.4 = Activate Window`) as new. **`setCamera` emits no artifact history; the product is right and
the measurement was wrong.** The fresh lane passes because with History not yet open the first click OPENS it.

### 1.3 The outliner (`outliner-hide-control-present controls=[]`)

```
bun 🔍️b36-pollution.ts --case=outliner-toggle
VERDICT after-first-artifact-click  rows=7
VERDICT after-second-artifact-click rows=0
```

`openPanel(/document|artifact|outliner/i)` clicks `framework.panel.artifact`; `context-menu-rows` runs
immediately before `outliner-rows` and selects through that very panel, so the tab is already active and the
click collapses it. `outliner-panel-opens` still PASSES (the tab exists and was clicked) while every
`panel:puzzle3d-play-document/` control is gone — which is precisely the `controls=[]` in the note.

### 1.4 `delete-selection` (`before=6 after=7`)

| # | lane | result |
|---|---|---|
| M0 | `🔍️b36-pollution.ts --case=dup-delete` (fresh page, one `Meta+d`, one `Delete`, per-poll census) | `duplicate before=1 after=2 waitedMs=9739` · `delete before=2 after=1 waitedMs=9847` · `delete-late-arrival changed=false` |
| M1 | `--only=selection-keybindings` | `duplicate-selection PASS` · `delete-selection PASS` |
| M2 | `--only=suggestions-open,selection-keybindings` | PASS / PASS — the third precondition resolved to a clone, `ids=["object-1"]` |
| M3 | `--only=catalogue-panel,selection-keybindings` | `duplicate-selection FAIL selected=[] before=3 after=3` · `delete-selection FAIL selected=[] before=3 after=3`, and **all three `select()` attempts read `EMPTY`** |

`catalogue-panel` is the polluting predecessor, and what it breaks is the step's **precondition, not the
command**: with two more objects in the document the re-framing puts nothing under the step's hardcoded pane
fraction (`frameForestTable`'s `0.78, 0.42`), so `Meta+d`/`Delete` are pressed on an empty selection and the
guest refuses correctly (`refuse_without_selection`). On a fresh page one `Meta+d` produces exactly one clone
and no late arrival (M0), so there is **no duplicate-admission defect to hop to B21/B24**: nothing in any lane I
ran admitted a command twice.

#53's own shape (`before=6 after=7`, `selected=["object-2"]`) is the same precondition drift one step further
along — `object-N` is `next_object_id()`'s own spelling, i.e. the pick had landed on a CLONE — plus one more
object arriving inside the 30 s delete window. I could not reproduce the arrival in isolation and the run's
console ring is long gone, so I am **not** claiming a cause for that last +1; §7.5 is the recipe that would
make it attributable next time (per-poll census WITH ids, and re-select by id through the outliner instead of by
pane fraction).

### 1.5 `volume-brush-arm activeUtility=select`

Candidate (c) of the brief — "a stale armed tool blocks `setActiveUtility volumeBrush`" — is **disproved**:

```
bun 🔍️b36-pollution.ts --case=volume-arm
VERDICT after-arm-brush   utility=["brush","brush"]
VERDICT after-escape      utility=["brush","brush"]     ← Escape did NOT disarm (keyboard focus was not the pane)
VERDICT after-arm-volume  utility=["volumeBrush","volumeBrush"]
```

Arming `volumeBrush` **from a brush-armed pane works**, so an armed predecessor cannot be what leaves
`activeUtility=select`. B33's diagnosis (the arm's own round trip starves behind the queue) remains the standing
explanation; on #53 the two probe lanes I needed for the follow-up bisect both died in the stale-vite boot
failure of §7 and I re-ran only the ones above. Not closed by this wave.

---

## 2 Premise corrections

### 2.1 The battery's step order in #53 was not the current one

`context-menu-rows` was still in its registration slot in #53's plan (`plan:` line at 7.0 s), so
`context-menu-object-vocabulary`/`context-menu-zoom-row-action-is-registered` in that run are B33 §7.2's
ordering red, already fixed by the `STEP_LEADS_ITS_GROUP` hoist now in the probe. Not re-bisected.

### 2.2 Three of the thirteen fail in a FRESH lane on #53

| lane | verdict |
|---|---|
| `--only=locale-switch` | `locale-control-present FAIL switched=false` |
| `--only=clipboard-copy-paste` | `clipboard FAIL delta=0 historyHasCopyPaste=false` (`copyBtn=0 copyById=0`) |
| `--only=selection-keybindings` | `duplicate-reselects-clone FAIL selected=[]` (`delete-selection` PASS) |

They are therefore not state-accumulation reds. `locale-control-present` is §7.2. `clipboard` is a §21 feature
question (no Copy control is reachable at all, and `Meta+C`/`Meta+V` move nothing) — it carries the probe's own
`[expect-42]` tag and is not this wave's. `duplicate-reselects-clone` is §7.4's reader defect.

---

## 3 Product fix 1 — the Settings panel rendered a default it had invented

### 3.1 Root cause, `file:line`

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` `ViewModel::for_panel` — "projects app-level panels without binding
  their controls to a window": `window_id: None`, `focused_window_id` KEPT.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs:511` `capture` — returned `Ok(None)`
  the moment `window_id` was `None`, i.e. for **every panel render**.
- `…/✏️editor/🪟️window/🦀️.rs:282` `config_from_view` → `.unwrap_or_default()`, so the panel's runtime carried
  `Puzzle3dWindowConfig::default()` (`grid_spacing = 10.0`).
- `…/✏️editor/🦀️.rs:8216` resolves `wid` from `focused_window_id` and `…/📌️panels/⚙️settings/🦀️.rs:63` bakes that
  `wid` into every stepper's `windowId` arg.

So the panel **read the default and wrote to the focused pane**. #53:
`settings-value-reaches-window-rail settings=10.5 windowRail=12.5` — `window-options` had moved the rail to 12.5,
the panel showed the invented 10.0, and its `+` bump published 10.5.

### 3.2 The fix

`capture` resolves the addressed instance, and for a panel projection the shell's focused pane:

```rust
let Some(window_id) = view_state.window_id.as_deref().or(view_state.focused_window_id.as_deref()) else { return Ok(None) };
```

One rule, stated in the function's own docstring: **a surface reads exactly the state it writes.** It needed no
schema move (the four Settings fields stay per-window, as B12's two addressing laws demand) and it fixes the same
class for every plugin panel, not just this one.

### 3.3 Law

`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `the_settings_panel_renders_the_focused_panes_own_value_not_a_default`:
nudge the PERSPECTIVE pane's spacing to 12.5, assert the rail carries it, then render the panel through the
testkit's exact panel projection (`render_panel_body`, `ViewModel::for_panel` + `focused_window_id`) and assert
the stepper's own `component.value`.

Before the fix (the defect, reproduced):

```
assertion `left == right` failed: the Settings panel renders the focused pane's own spacing, never a default it invented:
{"bindings":[{"action":{"name":"setGridSpacing",…},"args":{"windowId":"puzzle3d-main-perspective"},…}],
 "component":{"step":0.5,"type":"numberStepper","uniform":true,"value":10.0},"key":"puzzle3d-play-settings.grid-spacing.control"}
  left: Some(10.0)
 right: Some(12.5)
```

After:

```
test editor::puzzle3d::component::tests::the_settings_panel_renders_the_focused_panes_own_value_not_a_default ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 728 filtered out; finished in 0.12s
```

---

## 4 Product fix 2 — `addObjectKind` now selects what it adds

### 4.1 Root cause, `file:line`

- `…/✏️editor/🦀️.rs:4388` (`Puzzle3dAddObjectKindWork`'s `Publish` stage, the cursorized path the interactive job
  actually runs) completed with `Emit { artifact_mutations, ui_scope, ..Default::default() }` — **no
  `interaction_writes`**, where `duplicate_selection` and `add_brush_object` both end with
  `ctx.replace_selection(PUZZLE3D_GRANULARITY_OBJECT, …)`.
- `…/✏️editor/🎮️commands/🌱️add-object-kind/🦀️.rs` (the direct reducer arm) had the same omission.

Every way a user adds an object — the catalogue row, the catalogue drag-drop and the §23 Add Object dialog —
therefore left the new object unselected: nothing in the inspector, no gumball, no outliner highlight, and
`Delete` aimed at whatever was selected before.

### 4.2 The fix

Both arms now publish the sanctioned write, and the tool's publication contract declares the lane it uses
(`…/✏️editor/🦀️.rs:7143`, `ArtifactToolPublicationLane::Interaction` added — without it the retained operation
faults with *"typed-operation emitted a store lane absent from its exact factory publication contract"*, which is
how the framework proves a lane was undeclared rather than silently dropping it).

### 4.3 Laws

New: `add_object_kind_selects_the_object_it_adds` — the first add selects exactly its own object at OBJECT
granularity, the second **replaces** rather than widens, and every selected id is asserted against the live
document so a write naming a phantom fails.

Updated (a real consequence, not a weakened assertion):
`selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice` seeded its empty precondition with
`addObjectKind`; it now clears explicitly through `CLEAR_SELECTION_ACTION_ID` before asserting the refusals.

```
cargo test … --lib add_object_kind -- --test-threads=1
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 725 filtered out; finished in 0.34s

cargo test … --lib selection_scoped_commands_with_no_selection -- --test-threads=1
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 729 filtered out; finished in 0.29s
```

**This half rides #54** — it is guest Rust, so `catalogue-add-selects-new-object` cannot flip on the #53 wasm
currently served, and it will only flip once §7.4's reader defect is fixed too.

---

## 5 `export-only` — bisected to the example switch, and the hop is named with a byte count

Two lanes, one browser each, nothing else in the plan:

| lane | example | verdict |
|---|---|---|
| `--only=export-import` | Concrete Forest, 1 object | `export download=concrete-forest.json` · **`export-only PASS`** · `export-names-the-example PASS` · run `probe-2026-09-12T06-09-40` |
| `--only=example-switch,export-import` | Nakagin, 180 objects | `export download=none example=Nakagin Capsule Tower expected=nakagin-capsule-tower.json` · **`export-only FAIL`** · `export-names-the-example FAIL` · run `06-10-23` |

**The polluting predecessor is `example-switch`, and the discriminator is the fixture's SIZE, not any leftover
state.** `action-pane exportFixture rows=1` in both lanes — the row exists and was clicked.

Where it dies, with evidence:

- The GUEST emits the effect in both lanes. Its own reactor log is identical:
  `performInvocation settled {"actionId":"exportFixture","frames":2,…,"effects":1}` — Nakagin and Concrete
  Forest, byte for byte the same shape.
- In Rust, in-process, `export_fixture_names_the_download_after_the_active_example` exports Nakagin and gets
  `DownloadMediaExport { filename: "nakagin-capsule-tower.json" }` — **green**.
- The payloads, measured (a temporary `[DEBUG]` print inside that law, removed again before this report):

```
export payload filename=concrete-forest.json        bytes=7542    encoding=Some("utf-8")   → downloads
export payload filename=puzzle-3d.json             bytes=151     encoding=Some("utf-8")   → downloads
export payload filename=nakagin-capsule-tower.json bytes=145714  encoding=Some("utf-8")   → download=none
```

So the drop is **between the guest's effect list and the host's applier, on payload size alone** — 7 542 bytes
crosses, 145 714 bytes does not. That is the 64 KiB contiguous-request ceiling this codebase already knows about,
and the framework already carries the mechanism that exists for exactly this case and that `export_fixture` does
not use:

- `…/📺️renderer/🧑‍🎨engine/🧱️elements/📤️SegmentedDownload/🟦️.ts:5,7` — `SEGMENTED_DOWNLOAD_MARKER_PREFIX =
  "semio-segmented-handle-v1:"`, `MAX_SEGMENTED_DOWNLOAD_BYTES = 32 << 20`.
- `…/🏛️ShellHost/🟦️.tsx:5250` — the host already routes to `drainSegmentedMediaExport` when `encoding` starts
  with that marker, and to the inline `downloadMediaExport` otherwise.
- guest side: `🔌️plugin/🦀️.rs:27090` `take_segmented_download_chunk` over the `segmented_downloads` registry,
  exposed through `plugin_take_segmented_download_chunk` (`:32470`).
- the app side that must change: `…/✏️editor/🎮️commands/📤️export-fixture/🦀️.rs` pushes the whole JSON inline as
  `Effect::DownloadMediaExport { data, encoding: Some("utf-8") }`, unconditionally.

**Fix shape for the owning wave** (not landed here — it moves `exportFixture` off the plain-effect lane and onto
the segmented registry, which is a redesign of that command's job, not an edit): publish a segmented handle above
the inline budget and keep the inline path below it. Until then, every example bigger than Concrete Forest
exports as silence — no download, no notice, and a green `effects:1` in the console.


---

## 7 For B35 (probe owner) — exact recipes, no probe edits made

### 7.1 A panel tab click is a TOGGLE. Never use one as "ensure open".

`openPanel` (`:1874`), `readHistoryEntryIds` (`:1863`) and the two inline Inspection clicks
(`selection-surfaces` `:1297`, `locked-refusal` `:1405`) all click a tab and assume the panel is then open. It is
a toggle (`history patch applied labels=["Toggle Panel"]`), and the shell ALSO auto-reveals Inspection on every
selection change, so these clicks close panels at random depending on load.

Recipe — read the body first, click only if it is absent, then poll for the body:

```ts
const ensurePanel = async (tabId: string, bodySelector: string) => {
  const body = () => page.evaluate((sel) => document.querySelectorAll(sel).length, bodySelector);
  if (await body()) return { opened: true, clicked: false };      // already open — a click would CLOSE it
  await page.locator(`[data-slot="panel-tab-button"][id="${tabId}"]`).first().click({ force: true, timeout: 4000 }).catch(() => {});
  const settled = await settleFor(body, (count) => count > 0, 8000);
  return { opened: settled.ok, clicked: true };
};
```

with `bodySelector` per panel: `[id^="panel:puzzle3d-play-inspector/"]` (Inspection),
`[id^="framework.history.entry."], [id="framework.history.commands"]` (History),
`[id^="panel:puzzle3d-play-document/"]` (outliner), `[id*="puzzle3d-play-settings."]` (puzzle3d Settings).
`readHistoryEntryIds` must use it too, or its `before` reading keeps closing the panel it is about to measure.

### 7.2 `locale-control-present` — the language control is in a panel the probe never opens

Measured (`--case=settings-language`): `framework.settings.language` is **absent at boot and absent while
`puzzle3d.panel.settings` is open**, and appears (`count=2`) only after clicking the tab `framework.settings`.
`openPanel(/settings/i)` always matches `puzzle3d.panel.settings` first (it precedes `framework.settings` in the
tab roster and both render the text "Settings"). Recipe: `setLanguage` must address the FRAMEWORK settings panel
by exact id — `ensurePanel("framework.settings", '[id="framework.settings.language"]')` — and only then open the
select. (The same trap awaits any future `/settings/i` match; the id is the only stable handle.)

### 7.3 `projection-repaints-camera` — `ids[0]` is a pane, not a control

#53, `projection-options`: `ids=["framework.worldOrbit.projection","framework.worldOrbit.projection",
"puzzle3d-measure-projection-orthographic-view",…]` and `projection nudge framework.worldOrbit.projection:
{"before":{"tag":"div","slot":"pane",…,"text":"Projection"},"after":{…,"text":"Projection Collapse Parallel
Orthographic…"}}`. The step nudged a **collapsed pane** (two of them exist, one per window), "flipped" it by
expanding it, and then asked the camera to have changed. Recipe: filter the id list to real controls
(`data-slot` in `select-trigger|slider|tree-action-checkbox`) AND to the perspective window's own subtree, then
nudge `puzzle3d-measure-projection-orthographic-view`; a `slot:"pane"` hit is a locator miss and must be
reported as one.

### 7.4 `selectionState()` cannot see a world selection at all

`selectionState()` folds in `data-interaction-json`'s `view.selection?.ids/.instances` — and that record has no
such field. The host's own contract says so twice: `worldSurfaceSelectionDomV1`
(`🌐️World3dHost/🟦️.tsx:1362-1390`, wave B20 defect 1) publishes the painted selection as
**`data-selection-json`** `{selectedIds, activeObjectId, hoverTarget, …}`, because *"`WorldInteractionRecord`
carries neither `selectedIds` nor a hover target (it is the utility/brush/fill record)"*. So the world half of
`selectionState()` is dead code and the verdicts that depend on it —
`catalogue-add-selects-new-object`, `duplicate-reselects-clone` — can only ever see `aria-selected` DOM chips
(#53 reported `selected=["puzzle3d-play-distribution=Distribution"]`, a Display-panel chip). Recipe: read
`data-selection-json`'s `selectedIds` exactly as `worldInteraction()` already does, and keep the aria sweep as a
second, separately-named observable. `duplicate_selection_reselects_the_created_clones` is GREEN in Rust, so
that verdict is measuring the reader, not the guest.

### 7.5 `selection-keybindings` — make the precondition and the census attributable

The step's `select()` is a click at a hardcoded pane fraction; once the document carries more objects (after
`catalogue-panel`) it selects nothing at all (§1.4 M3) and the lane reports `before=n after=n`, which reads like
a refusing guest. Two changes: (a) re-select by ID through the outliner row for a known id, falling back to the
pane fraction, and (b) log the census WITH ids on every poll of the duplicate and delete windows, so a late
arrival names itself (`+["object-3"]`) instead of showing up as `after=7`.

### 7.6 Budget note

`focus-selection`'s `orbitPerspectiveAway` burned its full 30 s in #53 (`moved=false waitedMs=30138`) and then
the focus itself settled in 665 ms. An orbit that does not move is a precondition failure worth its own verdict
rather than a silent 30 s.

---

## 8 Infrastructure — the stale-vite boot failure, again

Two of my lanes died with `booted=false windows=0 canvases=0 faults=1` and a single collateral fault:

```
pageerror: SyntaxError: The requested module
'/@fs/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts'
does not provide an export named 'shardWorkerUrl'
```

`shardWorkerUrl` is on disk at line 52. B33 §8's recipe works verbatim: `touch` the file named in the
SyntaxError and re-run — the very next lane booted and produced verdicts. Console tail for this failure is
`[vite] connecting… / connected.` and nothing else, so **a boot failure with a two-line console tail is this,
not the app.**

---



---

## 9 Gates

All foreground, tails quoted.

**New and changed laws** (`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`):

```
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib the_settings_panel_renders_the_focused_panes_own_value -- --test-threads=1
test editor::puzzle3d::component::tests::the_settings_panel_renders_the_focused_panes_own_value_not_a_default ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 728 filtered out; finished in 0.12s

cargo test … --lib add_object_kind -- --test-threads=1
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 725 filtered out; finished in 0.34s

cargo test … --lib selection_scoped_commands_with_no_selection -- --test-threads=1
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 729 filtered out; finished in 0.29s

cargo test … --lib export_fixture -- --test-threads=1          (re-green after the temporary [DEBUG] came out)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 727 filtered out; finished in 1.04s
```

**Compile gates** — both clean, warnings only (all pre-existing, none in an edited region):

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
warning: `semio-s-artifact-puzzle-3d` (lib) generated 88 warnings …
    Finished `dev` profile [unoptimized] target(s) in 19.54s

cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
    Checking semio-s-plugin-puzzle v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 38.59s

cargo check -p semio-framework-plugin
warning: `semio-framework-plugin` (lib) generated 5 warnings …
    Finished `dev` profile [unoptimized] target(s) in 33.42s
```

**`semio-framework-plugin` lib suite — my framework line changes NOTHING.** Measured twice, with the one-line
`capture` fallback reverted and restored, and the failing-test NAME SETS diffed (not just the counts):

```
capture reverted:  test result: FAILED. 530 passed; 145 failed; 0 ignored; 0 measured; 0 filtered out
capture restored:  test result: FAILED. 529/530 passed; 145 failed …
diff baseline-names wave-names → (empty)
```

145 pre-existing reds in that crate (the global-registry order-dependence family of this ticket's own
`📓️2026-09-10-order-dependent-tests-audit.md`, plus live peer churn). A 146/145 flap between two runs of the
same code is that suite's own flakiness; the name sets are identical.

**`semio-s-artifact-puzzle-3d` lib suite — 716 passed, 14 failed**, one fewer than before this wave (my knock-on
is fixed). Attribution, each measured:

| test | in isolation | whose |
|---|---|---|
| `selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice` | **ok** | **mine — fixed** (§4.3) |
| `selected_object_inspector_renders_that_object_field_group`, `settings_panel_steppers_carry_their_value_and_the_trigger_they_dispatch_on`, `the_settings_panel_renders_the_focused_panes_own_value_not_a_default`, 4× `panels::catalogue::tests::*` | **ok** (7 tests) | order-dependent: `app.render(…).expect("render")` at `🔬️testkit/🦀️.rs:593/608` returns `Err` late in the suite — the 64-slot `SurfaceReconcileOutputs` pool of the A2 audit's family 1/2 |
| `world_pick_null_clears_without_reselecting_first_object`, `world_vortices_reveal_in_selected_mode_only_for_the_selected_object`, `gumball_active_only_for_transform_utilities_with_object_selection` | FAILED | **peer** — all three die on the SAME step, `dispatch(CLEAR_SELECTION_ACTION_ID)` no longer clears (`"clicking empty background must clear"`, `"clearing the selection hides the markers again"`, `"an unattached gumball must never render" left: Some(true)`). Proven not mine: they fail identically with my `capture` line reverted. **Worth someone's wave — clearing the selection is broken in three laws at once.** |
| `the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind` | FAILED | **peer** — the unfocused fallback now resolves `puzzle3d-main-top` where the law expects `puzzle3d-main`; fails with my line reverted too |
| `every_context_menu_row_dispatches_a_declared_action` | FAILED | **peer** — `5` Zoom-to-Selection rows where the law expects `3` |
| `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin` | FAILED | **peer/load** — `worst turn 4.608292ms over 26 turns exceeds … 2ms` |
| `two_instances_converge_disjoint_object_edits_via_backbone` | FAILED | **peer** — `"remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized"` |

**Second plugin crate, same diff method** — `capture` is framework code that every plugin panel render goes
through, so it was measured on another artifact crate too:

```
cargo test -p semio-s-artifact-puzzle-5d --lib -- --test-threads=1
capture restored:  test result: FAILED. 160 passed; 71 failed; 0 ignored; 0 measured; 0 filtered out
capture reverted:  71 failed
diff base-names wave-names → IDENTICAL FAILURE SETS
```

**Renderer-react vitest and `tsc --noEmit`: skipped, with reason.** This wave changed no host TypeScript at all
(three `.rs` files plus the ticket's own `🔍️b36-pollution.ts`), so neither gate can measure anything but peers'
live churn. The §7 recipes are prose for B35, not edits: `🔍️browser-probe.ts` is untouched.

---

## 10 What rides #54

Both product halves are Rust, so **neither can flip a verdict on the #53 wasm currently served on `:6013`**:

- `addObjectKind`'s re-selection is guest code (`✏️editor/🦀️.rs` + `🎮️commands/🌱️add-object-kind/🦀️.rs` + the
  tool's publication contract) → `catalogue-add-selects-new-object`, `add-object-*` and the drag-drop's selection
  need #54. And even on #54 that verdict stays red until §7.4's reader defect is fixed, because
  `selectionState()` cannot see a painted selection at all.
- the Settings panel's `capture` fallback is in `semio-framework-plugin`, which compiles INTO the plugin wasm →
  `settings-value-reaches-window-rail` needs #54 as well.

Browser-visible on #53 without any new wasm: nothing from this wave — every remaining verdict in my set is
either a §7 probe recipe (B35's file) or the §5 export redesign (unowned).

---

## 11 Files

Product:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs` — `WindowConfigOwnerRegistry::capture`
  falls back to `focused_window_id`, so a PANEL body reads the window config it writes to (+ the docstring that
  states the rule).
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` —
  `Puzzle3dAddObjectKindWork`'s `Publish` stage emits the `InteractionWrite` that selects what it created;
  `addObjectKind`'s publication contract declares `ArtifactToolPublicationLane::Interaction`.
- `…/✏️editor/🎮️commands/🌱️add-object-kind/🦀️.rs` — the direct reducer arm re-selects too (+ docstring).

Laws:
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `the_settings_panel_renders_the_focused_panes_own_value_not_a_default` (new),
  `add_object_kind_selects_the_object_it_adds` (new),
  `selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice` (its empty precondition is now
  established explicitly, because the seed gesture selects).

Ticket:
- `🔍️b36-pollution.ts` (input, kept) — the isolator, seven cases, unbounded console log.
- `🗑️generated/b36-L0-selection.txt`, `b36-L1-fillprefix.txt`, `b36-L2-pickmenu.txt`, `b36-L3-pick.txt`,
  `b36-L4-sugg-keys.txt`, `b36-{inspection-toggle,history-toggle,outliner-toggle,settings-language,volume-arm,dup-delete,grid-spacing}-*.txt`
  (+ `.console.txt`), `probe-2026-09-12T05-01-11 … 06-10-23.*`.

Untouched on purpose: `🔍️browser-probe.ts` (B35's), `🌐️World3dHost/🟦️.tsx` and the plugin-host interaction scope
(B34's), every other plugin `🦀️.rs`.
