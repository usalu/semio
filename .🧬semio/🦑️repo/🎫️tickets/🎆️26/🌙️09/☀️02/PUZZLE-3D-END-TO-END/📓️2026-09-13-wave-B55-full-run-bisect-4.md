# Wave B55 — full-run bisect #4 (battery #61 reds)

Ticket 26/09/02/PUZZLE-3D-END-TO-END. Baseline: `🗑️generated/battery-2026-09-13-61-6013.txt`
(FAULTS=0, PASS=71, FAIL=25). Serve: `:6013` (wasm #61, host vite-live).

Assignment: 8 red verdicts + make the `mutate` group run on a document the verdicts can reason about.

## 0. Baseline position map (read off battery #61, no new run needed)

`plan` (battery #61, line 5) puts the `mutate` group in this order:

```
context-menu-rows, pick-object, context-menu, tool-category, fill-tab, fill-abort-engagement,
fill-wait-ready, fill-apply-max, fill-history, selection-surfaces, clipboard-copy-paste,
marquee-drag, marquee-click, locked-refusal, gumball-drag, frame-perspective, brush-stroke,
suggestions-open, volume-brush, relocate, engagement-bar, outliner-rows, catalogue-panel,
selection-keybindings
```

Census timeline from the same file (`count=` samples, in step order):

| after step | instances |
| --- | --- |
| reboot for `mutate` | 1 (`treeItems=24`, `instanceCount=1`) |
| `fill-abort-engagement` | 0 |
| **`fill-apply-max`** | **161** |
| `clipboard-copy-paste` … `catalogue-panel` | 161 |

**The polluter is `fill-apply-max`, not `brush-stroke`.** `brush-stroke` sits at position 17 —
*after* `clipboard-copy-paste` (11) and `locked-refusal` (14), both of which already saw 161
objects. The 160 `puzzle3d.brush.*` ids are what the **Fill** tool registers when the count slider
is driven to its maximum with `End` (`fill-apply-max`, probe line 720). So ordering `brush-stroke`
last would change nothing; the bound has to go on the fill slider. → `--paint-limit` (§9).

`context-menu-rows` is position 1 and runs on `instanceCount=1` immediately after the group reboot,
i.e. it is *already* a fresh lane inside the battery. Its two reds are therefore genuine defects,
not pollution.

## 1. `context-menu-object-vocabulary` + `context-menu-zoom-moves-camera` — FIXED (product)

**Both are one defect. The shell's context menu was unreachable by any pointer.**

### Bisect

No prefix bisect was needed: `context-menu-rows` is the *first* step of the `mutate` group and runs on
`instanceCount=1` right after the group reboot. A fresh `--only=context-menu-rows` lane reproduced both
reds identically (`🗑️generated/wave-B55-ctxmenu-fresh.txt`), so neither is pollution.

B45's single `await …hover().catch(() => {})` hid the decisive fact. Replacing it with a named gesture
ladder that reports each outcome (probe, permanent) gave:

```
attempts=["hover=threw:TimeoutError: hover: Timeout 2500ms exceeded … rows=5 ariaExpanded=false geometry=725,472 155x16 active=none top=CANVAS",
          "hover-force=ok rows=5 ariaExpanded=false … top=CANVAS",
          "ordinal-digit=ok rows=7 ariaExpanded=true … top=CANVAS"]
```

- the plain hover **times out on actionability** — the row does not receive pointer events;
- a *forced* hover and a *forced* click both land and change nothing (`aria-expanded=false`);
- the pure-keyboard route (press the row's ordinal `4`) opens the submenu in **54 ms** and both rows
  appear (`rows=7`).

So the vocabulary was never missing and the guest authoring is correct
(`✏️s/…/🧊️3d/…/✏️editor/🦀️.rs:2851-2860` builds `menu.group.hand` with `hide-show` + `lock-unlock`).
The menu simply could not be pointed at. A second permanent dump (`context-menu stacking=…`) named it:

```
hit: "CANVAS pe=auto z=auto"   hitsRowOrDescendant: false
chromeClass: "… z-menu w-auto min-w-[10rem] max-h-layout-command overflow-y-auto"
chromeZ: "auto"   zMenuToken: "unset"
chain: BUTTON#duplicate pe=auto z=auto → DIV[window-chrome-body] z=1 →
       DIV[context-menu-content] pe=none z=auto pos=fixed → DIV[portal-layer] pe=none z=auto → DIV iso=isolate
```

That also explains `context-menu-zoom-moves-camera`: the probe's `[id="zoom"]` click carries
`force: true`, so it dispatched at the row's coordinates — onto the **canvas**. The console tail
(`setCamera … effects:0`, camera byte-identical) is the orbit controller answering a canvas click, not
`focusSelection` ever running.

### Root cause

`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` — the stylesheet that carries
`@import "tailwindcss"` and defines `@utility z-menu { z-index: var(--z-menu) }` (line 6908) plus the
`--z-*` LevelZScale (lines 812-821) — **declared no `@source` for its own module**. Only the file
carrying `@import "tailwindcss"` has its `@source` honoured, and only relative to itself; the two
`@source` lines in `🧰️framework/🔨️modules/🖱️ui/🧵️styles/🎨️.css` reach nothing in `🖱️ui`
(`../../../../🧱️elements` is `<repo>/🧱️elements`, which does not exist — and rewriting it to `../`
changed nothing, measured).

Consequence: Tailwind never saw `z-menu` in any scanned file, never generated `.z-menu`, and therefore
tree-shook `--z-menu` out of `@theme` as well. The served CSS on :6013 carried only
`.z-10 .z-20 .z-30 .z-40 .z-45 .z-50 .z-60 .z-panel .z-tutorial` — and `.z-panel`/`.z-tutorial` only
because those two names also occur inside the *engine's* own scanned tree
(`🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx`, `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`). That is why
the loss was invisible: every ui class that a consumer also happens to use survives.
`.z-dialog`, `.z-pane`, `.z-base`, `.z-navbar` and `max-h-layout-command` were missing too.

### Fix — product

`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css:14` → `@source "../../";` (the whole `🖱️ui`
module), with the measurement recorded in a docstring above it. Served CSS 264 555 → 321 157 bytes;
`.z-menu`, `.z-dialog`, `.z-pane`, `.z-base`, `.z-navbar`, `.z-tutorial` and `max-h-layout-command`
all appear.

### Verdicts (fresh `--only=context-menu-rows`, `🗑️generated/wave-B55-ctxmenu-after.txt`)

```
context-menu stacking={"row":"duplicate at 1045,469","hit":"SPAN pe=auto z=auto","hitsRowOrDescendant":true,
  "chromeClass":"… z-menu …","chromeZ":"50","zMenuToken":"50", …}
attempts=["hover=… rows=7 ariaExpanded=true geometry=963,509 164x16 active=true top=menu.group.hand"]
[78.5s] verdict context-menu-opens PASS
[78.5s] verdict context-menu-object-vocabulary PASS
[78.5s] verdict context-menu-zoom-row-action-is-registered PASS
[85.0s] verdict context-menu-zoom-moves-camera PASS   (was waitedMs=30449 → now settles in ~6.5 s)
```

`hover` still logs a `TimeoutError` while *having worked* (`rows=7`, `ariaExpanded=true`,
`top=menu.group.hand`): opening the submenu widens the row 155 → 164 px, so Playwright's stability
check loses the race it started. The ladder absorbs that, and `openedVia=hover` is now the real
pointer route.

### Probe changes (kept, names unchanged)

- `context-menu-rows`: the submenu opener is a named gesture ladder (`hover`, `hover-force`,
  `ordinal-digit`, `click`, `arrow-down-right`) that logs each attempt's outcome, the row's
  `aria-expanded`, its rect, `data-active` and the element winning the hit test at its centre.
- new permanent `context-menu stacking=…` log: the element `elementFromPoint` returns over the first
  menu row, the top of `elementsFromPoint`, and the pointer-events/z-index/position/isolation chain
  above the row, plus the chrome's class, computed `z-index` and resolved `--z-menu`.

## 2. Fresh lanes for the other six reds — all green fresh

`bun 🔍️browser-probe.ts --only=<step> --port=6013`, one lane each
(`🗑️generated/wave-B55-fresh-<step>.txt`):

| lane | verdicts |
| --- | --- |
| `locked-refusal` | `locked-flag-row PASS`, `locked-refusal-notice PASS` (notice in 0.4 s) |
| `volume-brush` | `volume-brush-arm PASS`, `…-target-volume-attribute PASS`, `…-add-target-volume PASS`, `…-voxel-dims PASS` |
| `outliner-rows` | `outliner-panel-opens PASS`, `outliner-hide-control-present PASS`, `outliner-hide-applies PASS`, `outliner-show-restores PASS` |
| `catalogue-panel` | all five PASS, incl. `catalogue-add-selects-new-object PASS` |

So every one of the six is position-dependent, exactly as B33/B36/B45 found for their reds.

### 2.1 The fill prefix does not reproduce them

One growing-prefix run
(`--only=tool-category,fill-tab,fill-abort-engagement,fill-wait-ready,fill-apply-max,clipboard-copy-paste,locked-refusal,volume-brush,outliner-rows,catalogue-panel`,
`🗑️generated/wave-B55-prefix-fill.txt`) could not arm the Fill tool at all —
`fill wait poll count=0 … sliders:[] toggles:["ui.fullscreen.toggle=false"]` twelve times, then
`fill count after wait: 0` and `no slider found for fill-apply`. The Fill panel needs the steps the
battery runs before it (`context-menu-rows`, `pick-object`, `context-menu`) to reach the tool category,
so a prefix that starts at `tool-category` measures a document of **1-2** objects, not 161. Results:

```
clipboard-precondition PASS   clipboard-copy-writes-a-fragment PASS
clipboard-paste-reaches-the-guest PASS   clipboard PASS
locked-refusal-notice FAIL locked=true flag="locked true" notices=[] waitedMs=30747
volume-brush-arm PASS  outliner-hide-control-present PASS  catalogue-add-selects-new-object PASS
outliner-show-restores FAIL restored=false
```

Two readings, both useful:

- **the clipboard trio is not broken by object count.** It passed on a 12-object document that had
  already run the four fill steps, so "reserved clipboard slots after many commands" and "the Fill tool
  armed" are both ruled out as the polluter; what battery #61 measured must come from further left.
- `locked-refusal-notice` fails **with the lock engaged** (`locked=true`, unlike battery #61's
  `locked=false`), on a selection of `object-1` (a clipboard-paste product picked by pane fraction)
  rather than `seed-left-001`. The gumball *was* grabbed
  (`gumball grabbed tried=["1@469,448=false","0.8@474,442=false","0.6@479,436=true"]`) and 30.7 s of
  polling saw no `locked` notice, where the fresh lane saw `"Selection is lockedClose"` 0.4 s after the
  grab. So this is not the late-publication story B44/B47 recorded: the flag row engages fine here.

## 3. Battery #62 — the z-index fix cleared six of the eight reds

`bun 🔍️browser-probe.ts --battery --reload-between-groups --port=6013`
(`🗑️generated/battery-2026-09-13-62-6013-B55.txt`, the fill suite still mid-group).

```
battery PASS=84 FAIL=13 FAULTS=0 first-hard-fault-at=none guest-death-faults=0      (#61: PASS=71 FAIL=25)
```

Cleared **at their full-run positions**, with the document size now printed in every mutate verdict:

```
battery verdict context-menu-object-vocabulary PASS instances=1@step-start
battery verdict context-menu-zoom-moves-camera PASS instances=1@step-start
battery verdict clipboard-precondition PASS instances=145@step-start
battery verdict clipboard-copy-writes-a-fragment PASS instances=145@step-start
battery verdict clipboard-paste-reaches-the-guest PASS instances=145@step-start
battery verdict clipboard PASS instances=145@step-start
battery verdict catalogue-add-selects-new-object PASS instances=5@step-start
```

The clipboard trio passing on a **145-object** document settles the bisect question the coordinator
posed: it was never reserved slots, an armed Fill tool, or `puzzle3d.brush.*` objects copy refuses.
`clipboard-paste-reaches-the-guest` had diagnosed its own blocker correctly in #61 — "the shell opens
the STAGED paste form instead of executing" — and #61's very next line was
`clipboard staged-paste execute controls=0`: the form's `Execute` control could not be reached, because
the surface carrying it sat at `z-index: auto` under the canvas, exactly like the context menu.

Still red and mine: `locked-refusal-notice`, `volume-brush-arm` (+ its two dependants),
`outliner-hide-control-present`. Red and NOT mine: `gumball-scene-delta`, `brush-preview-place`,
`relocate-pose-delta` (B54's lane), `export-only`/`export-names-the-example`/the three `import-*`
(B53's lane).

### 3.1 B46's arena-capacity theory is dead

Every one of my three remaining reds carries its document size now:

```
locked-refusal-notice       instances=147@step-start
volume-brush-arm            instances=4@step-start
outliner-hide-control-present instances=5@step-start
```

`volume-brush` and `outliner-rows` fail on a **4-5 object** document. "the arena cannot afford inline
toggles on a 161-object document" cannot be the cause of `controls=[]`. (The census also shows the
document *shrinking* 147 → 4 between `locked-refusal` and `gumball-drag`, so the fill's brush objects
are not even durable.)

## 4. `--paint-limit` cannot bound the paint; ordering can

`fill-apply-max` presses `End` on `[role="slider"]`. Measured rail: **`min=0 max=1000 now=0`** — it is a
parameter slider, not an object count, and the `puzzle3d.brush.*` objects are registered by the build
job the armed Fill tool starts. `--paint-limit=12` (battery #62) still left **127** objects:

```
[433.1s] fill rail={"min":0,"max":1000,"now":0} paintLimit=12 target=12
[442.6s] fill count after End: 1
[442.7s] fill instances after End: {"count":128, …}
```

So the flag stays (documented for exactly what it does and does not do, default `0` = `End`), and the
isolation is done by ORDER: a new `STEP_TRAILS_ITS_GROUP` runs the six fill steps
(`tool-category`, `fill-tab`, `fill-abort-engagement`, `fill-wait-ready`, `fill-apply-max`,
`fill-history`) LAST inside `mutate`, mirroring the existing `STEP_LEADS_ITS_GROUP`. It costs the fill
suite nothing — its precondition is the tool category, which `context-menu-rows`/`pick-object` still
establish first.

## 5. `outliner-hide-control-present` — FIXED (probe)

### Bisect / evidence

`outliner-rows` verdicts came off the **Inspection** panel, not the outliner:

```
[1148.1s] ensurePanel framework.panel.artifact already-open active=true body=0 clicked=false
[1148.1s] verdict outliner-panel-opens PASS instances=5@step-start
[1148.1s] outliner rows before=[{"id":"puzzle3d-play-inspector","text":"INSPECTION Schema puzzle.3d.fixture …"},
                               …,{"id":"puzzle3d-fill-count","text":"Count 1"},{"id":"puzzle3d-play-distribution", …}]
[1148.1s] outliner row controls=[]
```

`already-open … body=0` is the tell: `ensurePanel`'s `isOpen` short-circuits on `active === true` and
never presses the tab, so the step measured whatever panel *was* mounted. Every Hide/Lock locator in the
step is scoped to `[id^="panel:puzzle3d-play-document/"]`, of which there were zero.

### Root cause — probe, two independent defects in one reader

`🔍️browser-probe.ts`, `ensurePanel`'s `tabActive`:

1. it resolved the tab with `document.getElementById(id)` and then climbed with
   `closest('[role="tab"], [data-slot="panel-tab-button"]')`, falling back to the element itself — so any
   *other* element sharing that id decided the answer;
2. it substring-tested a **concatenation** of four attributes against `/true|active|selected|open/i`, in
   which a single `data-state="inactive"` matches `/active/` and `"…open…"` matches any word containing it.

Measured attributes of a live `panel-tab-button` (:6013): `data-slot="panel-tab-button"
data-tab-id="<id>" data-tab-kind=leaf data-level=panel aria-pressed="false"` — no `aria-selected`,
`data-state` or `data-active` at all on an inactive tab; the ACTIVE one publishes
`pressed=true state=on active=true`.

### Fix — probe

`tabButtonState()` addresses the tab button as itself
(`[data-slot="panel-tab-button"][data-tab-id="<id>"]`, then `[…][id="<id>"]`, then `getElementById`),
compares each value **whole** against `{true, active, open, on, selected}`, and reports every field plus
`sharedIds` so a future disagreement is readable. `ensurePanel`'s already-open line now carries that
whole record.

### Verdicts (`--only=locked-refusal,outliner-rows`, `🗑️generated/wave-B55-prefix-outliner.txt`)

```
[36.7s] ensurePanel framework.panel.inspection already-open active=true body=12 clicked=false tab={"slot":"panel-tab-button","pressed":"true","selected":"","state":"on","active":"true","ok":true,"sharedIds":1}
[75.4s] ensurePanel framework.panel.artifact pre-click attrs={"matches":1,… "aria-pressed=false" …}
[77.7s] ensurePanel framework.panel.artifact clicked=ok active=true body=7 waitedMs=557
[78.3s] outliner rows before=[{"id":"puzzle3d-play-document","text":"OBJECTS … Hide Unlock REFERENCES TARGET VOLUMES"}, …]
[78.5s] outliner row controls=[{"tag":"button","slot":"action","text":"Hide"},{"tag":"button","slot":"action","text":"Unlock"}, …]
[79.2s] verdict outliner-hide-control-present PASS instances=1@step-start
```

## 6. `locked-refusal-notice` — FIXED (probe)

### Bisect / evidence

Battery #62, in step order:

```
[608.1s] lock flag row="locked true" locked=true waitedMs=3950
[610.8s] gumball hits {"hits":[{"kind":"moveX", …},{"kind":"moveY", …},{"kind":"moveZ", …},{"kind":"origin", …}],"entered":false}
[610.8s] gumball axis projection len=75 stride=83,47
[632.3s] gumball hits {"hits":[ …4 handles… ],"entered":false}
[649.0s] gumball hits {"hits":[],"entered":false}
[664.4s] gumball hits {"hits":[],"entered":false}
[678.0s] gumball hits {"hits":[],"entered":false}
[708.4s] verdict locked-refusal-notice FAIL locked=true flag="locked true" notices=[] waitedMs=30471
```

There is **no `gumball grabbed` line at all**, and the handle list goes empty after the first press. So
this is not B44/B47's late publication and not the 147-object document: the lock engaged in 3.9 s, and
then no `translateSelection` was ever dispatched, so there was no refusal to notice. The fresh lane sees
`notices=["Selection is lockedClose","Agent disconnected"]` 0.4 s after the grab.

### Root cause — probe

`dragGumballMoveX` walks four fractions of the projected origin→tip segment and presses each one. A press
that **misses** the handle is a plain canvas press-drag-release — a pick that clears the selection — and a
cleared selection takes the gumball with it. The ladder therefore destroys its own precondition on the
first miss and the remaining three candidates press empty space. On the small fresh document the first or
second fraction hits, which is why every fresh lane passes.

### Fix — probe

`dragGumballMoveX(box, reselect?)`: after a miss whose follow-up read shows **zero** handles, the caller's
`reselect` runs and the helper waits for the handles to come back before the next fraction. `tried` now
marks the miss that lost the selection (`0.8@…=false selection-lost`) and a `gumball reselect after miss@f
handlesBack=n waitedMs=…` line names the recovery. Both call sites pass one
(`ensureWorldSelection("locked-regrab" | "gumball-regrab", framed.table)`), so `gumball-drag` gets the
same protection.

## 7. `volume-brush-arm` — root cause named, diagnostic added (product defect, not yet fixed)

### Bisect / evidence

Battery #62:

```
[954.6s] utilities already-open clicked=false focusPick=none ids=[… "volumeBrush=Volume Brush" …]
[956.7s] arm-utility volumeBrush click=ok shape={"tag":"button","slot":"toggle-group-item","disabled":false,
           "state":"off","pressed":"false","pointerEvents":"auto","rect":[642,817,105,22],
           "hit":"SPAN#-[tree-label]","mine":false}
[977.0s] arm-utility volumeBrush found=1 before=select active=select settled=false waitedMs=20347
```

`mine: false` with `hit: SPAN[tree-label]`: the forced click landed on a **tree row of the panel dock**,
not on the Volume Brush toggle. So `activeUtility=select` is not a dropped round trip — no press ever
reached the toggle. The two dependent reds follow mechanically: with the utility unarmed, the Alt-click
adds no target volume (`before=0 after=0`) and the voxel measures never appear.

Geometry: the Perspective pane is `947x814`, and its utility ribbon
(`#ui.utilities.puzzle3d-main-perspective`) sits at `[493,820,294,22]` — *below* the pane, in the same
band as the panel dock. Every ancestor of the toggle is `position: static; z-index: auto` up to the
ribbon, so nothing gives the ribbon a stacking level; whichever of the two surfaces comes later in DOM
order paints on top, and a dock tree tall enough to reach y≈828 takes the press. A fresh page with the
Artifact panel open measures `mine: true, top: SPAN[inline-label]`, and a two-step
`--only=outliner-rows,volume-brush` lane also measures `mine: true` and PASSES — the overlap needs the
dock state the full battery reaches.

### Probe change (kept)

`armUtility`'s `shape` dump now carries, whenever `mine` is false, the covering element's own `rect`, its
owning surface (`ribbon`/`panel-body`/`tree`/`panel:*`/`menu`) and its text, plus the ribbon's own rect —
so the next red states the overlap instead of implying it.

### Still open

The product question is which surface owns that 22 px band. The utility ribbon is pane chrome and must be
pressable whenever it is visible; a panel body that can paint over it is a layout defect, not a z-order
preference (giving the ribbon `z-pane` = 20 would still lose to a panel at `z-panel` = 30). Naming the fix
needs the battery state, which §8 re-measures with the `cover` dump in place.

## 8. Battery #63 — volume-brush cleared by the ordering; two probe regressions found and fixed

`🗑️generated/battery-2026-09-13-63-6013-B55.txt` — the first run with the fill suite trailing.

```
battery PASS=83 FAIL=14 FAULTS=0
battery verdict volume-brush-arm PASS instances=3@step-start
battery verdict volume-brush-add-target-volume PASS instances=3@step-start
battery verdict volume-brush-voxel-dims PASS instances=3@step-start
```

`volume-brush-arm` clears purely from the reordering: the surface that covered the ribbon
(`hit=SPAN[tree-label]`) was the panel dock rendering the fill suite's own tree, and with the fill suite
running last the utility ribbon is reachable at `volume-brush`'s position. The geometric overlap the
`cover` dump exists to name is still there as a latent product defect — see §7 — it simply no longer
has a tall tree over it at that point in the run.

Two verdicts that were green in #61/#62 went red in #63, both from the same reader, both now fixed:

```
verdict inspection-object-fields FAIL instances=1@step-start populated=false empty=false id=null
verdict inspection-locked-flag-row FAIL instances=1@step-start lockChrome=false
verdict fill-history-entry FAIL instances=5@step-start entries=0
verdict outliner-hide-control-present FAIL instances=3@step-start byText=0 byLabel=0 controls=[]
```

All four read a panel that was not the one mounted:

```
[436.6s] ensurePanel framework.panel.inspection already-open active=true body=0 clicked=false tab={"pressed":"true","selected":"","state":"","active":"true","ok":true,"sharedIds":1}
[1355.9s] ensurePanel framework.panel.artifact  already-open active=true body=0 clicked=false tab={"pressed":"true","selected":"","state":"","active":"true","ok":true,"sharedIds":1}
[1355.9s] outliner rows before=[{"id":"framework.history.entry.1","text":"Set Active Example↶"}, …]      ← the HISTORY panel
[1590.1s] fill history after apply: {"entries":[],"entryCount":0,"tree":["puzzle3d-play-inspector=INSPECTION …"]}  ← the INSPECTION panel
```

versus a correctly mounted one at 325.3 s: `body=7 … "state":"on"`.

### Root cause (probe) — `data-state="on"` is the only field that tracks the mounted panel

`aria-pressed="true"` and `data-active="true"` stay set on a tab whose panel is **not** the one the dock
renders. `ensurePanel`'s `isOpen` was `state.active === true || (body ? state.body > 0 : false)`, so a
stale tab flag SUBSTITUTED for a body that was not there and the press that would have switched the dock
was skipped.

### Fix (probe)

`isOpen` is now `body ? state.body > 0 : state.active === true` — when a caller names the body, that body
is the only truth, and the tab's flags can neither stand in for it nor veto it. The "published late" worry
the old reading defended against is handled where it belongs: the press is followed by the existing 20 s
body poll, and a press that toggled an open-but-unpublished panel shut is followed by one retry press
(`ensurePanel <tab> retry-click=…`). `tabButtonState` also resolves the id-bearing tab button first
(ids are unique here, `sharedIds:1`), then `[data-tab-id]`.

### Verdicts (`--only=selection-surfaces,outliner-rows`, `🗑️generated/wave-B55-panelbody.txt`)

```
[22.3s] ensurePanel framework.panel.inspection clicked=ok active=true body=4 waitedMs=4
[34.4s] verdict inspection-object-fields PASS instances=1@step-start
[34.4s] verdict inspection-locked-flag-row PASS instances=1@step-start
[41.4s] ensurePanel framework.panel.artifact clicked=ok active=true body=7 waitedMs=29
[41.4s] verdict outliner-panel-opens PASS instances=1@step-start
[41.4s] verdict outliner-hide-control-present PASS instances=1@step-start
[47.0s] verdict outliner-hide-applies PASS instances=1@step-start
[49.4s] verdict outliner-show-restores PASS instances=1@step-start
```

### Not mine, concurrent peer work

`projection-control-flips`, `projection-repaints-camera` and `example-switch` all went red in #63 and all
three fail the same way — a `slot=select-trigger` that never opens (`example options=[] pickerPresent=1
opened=false waitedMs=8038`, `state="closed" expanded="false" mine=true`). `git log --date=iso` shows
peers editing `🖱️ui/🪟️viewport/🧪️tests/📐️projection`, `🧑‍🎨engine/🎯️targets/🧊️wgpu/📽️projection` and
`.storybook/stories/ui/🔝NavbarExampleSelect.stories.tsx` in the window between #62 and #63 (commits 614
and 615). Flagged, not chased.

## 9. `locked-refusal-notice` — the lock machinery is innocent; the residual red is B54's never-settling transform

Reproduced deterministically with
`--only=context-menu-rows,pick-object,context-menu,selection-surfaces,clipboard-copy-paste,marquee-drag,marquee-click,locked-refusal`
(`🗑️generated/wave-B55-prefix-locked{2,3,4}.txt`) after the gumball re-grab fix, so the gesture now
lands:

```
[…] lock flag row="locked true" locked=true waitedMs=3751
[…] gumball grabbed tried=["1@455,442=false","0.8@462,437=false","0.6@469,432=true"] stride=-76,58
[…] locked refusal hops grabbed=true poseChanged=false poseLen=785/785 console=[
      "[DEBUG] gumball pose delta {action: translateSelection, ids: Array(1), mode: mesh}",
      "[DEBUG] performInvocation {\"invocationKind\":\"action\",\"actionId\":\"translateSelection\"}",
      "[DEBUG] command ingress lane {\"actionId\":\"translateSelection\",\"seq\":225,\"lane\":\"Interactive\"}"]
[…] verdict locked-refusal-notice FAIL locked=true flag="locked true" grabbed=true poseChanged=false notices=[] waitedMs=30262
```

The host dispatched `translateSelection` and **there is no `performInvocation settled` for it** in the
following 30 s — no completion, no effects, no notice, no pose change. So this is not a lock defect and
not a notice defect:

- the guest's refusal path is present and correct —
  `✏️s/…/🧊️3d/…/✏️editor/🦀️.rs:7887` routes `translateSelection` through `Puzzle3dScaleWork`, whose
  `Puzzle3dScaleStage::Volumes` completion emits `labels.selection_locked` when every mutation was
  filtered out by `!object.locked` (line 4657, the same refusal `Puzzle3dWorldRelocateStage::Object`
  cites at line 5259);
- the fresh lane proves that path end to end: `notices=["Selection is lockedClose","Agent disconnected"]`
  **0.4 s** after the grab.

`translateSelection` is declared
`.action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)` (line 8731), i.e.
it is pumped as a retained interactive job. A dispatched `Migrated` job that never settles is exactly the
symptom the other two long-run reds in #63 carry — `gumball-scene-delta FAIL sceneDelta=false
waitedMs=30355` and, in #62, `relocate-pose-delta FAIL beforeLen=1129 afterLen=1129 waitedMs=30499` —
which is **B54's mutation-latency lane (guest lanes/reactor)**. Left there deliberately; no product edit
from this wave.

### Probe changes (kept)

- `dragGumballMoveX(box, reselect?)` — see §6.
- `locked-refusal` now records `grabbed`, `poseChanged` and a filtered console tail around the drag, so the
  three worlds a bare `notices=[]` conflated (never dispatched / refused silently / refused and not
  rendered) are separated in the verdict itself.

### Side finding worth its own ticket

Every canvas press in that lane logs

```
error: semio: app "s.puzzle.puzzle3d@1/*#editor" dropped action "worldPick" dispatched from window kind
"puzzle3d-main": no window kind declares it (window kinds: puzzle3d-main).
Declare it with .window_kind_actions()/.window_kind_action_refs() on the window that dispatches it.
```

`worldPick` is dispatched by the world host and dropped by the app on every press. Selection still works
(it travels as `interactionSelect`), so nothing visible depends on it today — but a dispatch that is
always dropped is either a missing declaration or dead host code.

## 10. Battery #64 — final full-run verification

`bun 🔍️browser-probe.ts --battery --reload-between-groups --port=6013`
(`🗑️generated/battery-2026-09-13-64-6013-B55.txt`), everything from §1-§9 in place.

```
battery PASS=89 FAIL=10 FAULTS=0 first-hard-fault-at=none guest-death-faults=0
```

| | #61 (baseline) | #62 | #63 | #64 |
| --- | --- | --- | --- | --- |
| PASS | 71 | 84 | 83 | **89** |
| FAIL | 25 | 13 | 14 | **10** |

### The eight assigned reds, at their full-run positions

| verdict | #61 | #64 | fix |
| --- | --- | --- | --- |
| `context-menu-object-vocabulary` | FAIL | **PASS** `instances=1` | product — `🖌️ui/🎨️.css` `@source` |
| `context-menu-zoom-moves-camera` | FAIL | **PASS** `instances=1` | product — same |
| `clipboard` | FAIL | **PASS** `instances=1` | product — same |
| `clipboard-copy-writes-a-fragment` | FAIL | **PASS** `instances=1` | product — same |
| `clipboard-paste-reaches-the-guest` | FAIL | **PASS** `instances=1` | product — same |
| `catalogue-add-selects-new-object` | FAIL | **PASS** `instances=3` | product — same |
| `volume-brush-arm` (+ `-add-target-volume`, `-voxel-dims`) | FAIL | **PASS** `instances=3` | probe — fill suite trails its group |
| `outliner-hide-control-present` | FAIL | **PASS** `instances=3` | probe — `ensurePanel` body is the authority |
| `locked-refusal-notice` | FAIL | FAIL `instances=3 grabbed=true poseChanged=false` | **handed to B54** — `translateSelection` never settles (§9) |

### The remaining 10 FAILs in #64, by owner

- **B54 (mutation latency, guest lanes/reactor)** — one family, one symptom (a transform command
  dispatched and never settled): `locked-refusal-notice`, `gumball-scene-delta`, `relocate-pose-delta`.
- **B53 (export/download)** — `export-only`, `export-names-the-example`, `import-same-file-idempotent`,
  `import-distinct`, `import-distinct-records-history`.
- **peer-concurrent select churn** — `example-switch` (`select-trigger` never opens; the step's retry
  path still reaches Nakagin, and `example-switch-instances` passes). `projection-control-flips` and
  `projection-repaints-camera`, red in #63 from the same cause, are **green again in #64**.
- **newly reachable, unowned** — `outliner-show-restores`. It could not run in #61/#62 because
  `outliner-hide-control-present` returned early; now Hide applies (`outliner-hide-applies PASS`) and
  Show does not restore (`outliner show controls=1`, then `restored=false`). The row's own label *did*
  come back (`afterShowHead` reads `"… Hide Lock"`, i.e. the unhidden state), so the half that stays
  unrestored is the world one — `outliner-show-restores` requires both the row set to equal `before` and
  the world scale to leave `[0,0,0]`. Worth its own lane; outside this wave's assignment.

## 11. What rides the next battery (#65)

Nothing of mine is pending measurement: every product and probe change in this report is verified in #64
at full-run position. The next battery carries only other waves' work.

## 12. Changes made

### Product (1 file)

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css:5-14` — `@source "../../";` with the measurement in a
  docstring. Emits `.z-menu`, `.z-dialog`, `.z-pane`, `.z-base`, `.z-navbar`, `max-h-layout-command` and
  everything else used only inside the `🖱️ui` module. Served CSS 264 555 → 321 157 bytes. No test compiles
  this stylesheet (the only test naming `z-menu`,
  `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`, asserts the CLASS NAME on the element, which
  did not change), so no vitest lane is implicated; nothing in the host's TypeScript or in any guest crate
  was touched, so no renderer-react lane and no cargo law applies.

### Probe — `🔍️browser-probe.ts` (names preserved; every step and verdict name unchanged)

1. `context-menu-rows`: named gesture ladder for opening a submenu group + permanent
   `context-menu stacking=…` dump (hit test, `elementsFromPoint`, pointer-events/z-index/position/isolation
   chain, chrome class, computed `z-index`, resolved `--z-menu`).
2. `STEP_TRAILS_ITS_GROUP` — the six fill steps run last inside `mutate`.
3. `paintLimit` / `--paint-limit=<n>` — documented for what it is (a slider value, default `0` = `End`) and
   what it is not (an object-count bound); `fill-apply-max` also logs `fill rail=…` and
   `fill instances after End=…`.
4. Per-verdict `instances=<n>@step-start` on every `mutate` verdict, plus `group` and
   `instancesAtStepStart` in the ndjson record.
5. `ensurePanel`: `tabButtonState()` addresses the tab button as itself and compares values whole; the
   panel BODY is the authority when a caller names one; one retry press when the body never arrives.
6. `dragGumballMoveX(box, reselect?)` re-establishes the selection a missed press destroyed; both call
   sites pass one.
7. `armUtility`: the `shape` dump names the covering element's rect, owner and text, and the ribbon's rect.
8. `locked-refusal`: records `grabbed`, `poseChanged` and a filtered console tail around the drag.

`bun x tsc --noEmit … 🔍️browser-probe.ts` reports the two pre-existing errors only
(`import.meta.dir` under an ad-hoc config, and `modifiers` on `page.mouse.click` at line 2062 in the
suggestions lane); no error from any change above.

### Evidence files kept under `🗑️generated/`

`battery-2026-09-13-6{2,3,4}-6013-B55.txt`, `wave-B55-ctxmenu-{fresh,fresh2,fresh3,fresh4,after}.txt`,
`wave-B55-fresh-{locked-refusal,volume-brush,outliner-rows,catalogue-panel}.txt`,
`wave-B55-prefix-{fill,outliner,volume,locked,locked2,locked3,locked4}.txt`, `wave-B55-panelbody.txt`,
`wave-B55-served-css{,-after,-src2}.txt`.
