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
