# Wave B53 — Nakagin `export-only`: the segmented lane is GREEN live; the unreachable hop is the Actions rail

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · 2026-09-12/13 · wasm **#61** on `:6013` (host vite-live).
Written incrementally. Every command ran in the FOREGROUND with its tail quoted.

Inputs read first: `🗑️generated/battery-2026-09-13-61-6013.txt` (the failing battery),
`📓️2026-09-12-wave-B38-probe-recipes-export-segments.md` §2, `📓️2026-09-12-wave-B43-segmented-download-fault.md`,
`🔍️browser-probe.ts` (`activateWindowFileAction`, the `export-import` step — read only, B55 owns it),
`🛠️ShellHelpers/🟦️.tsx` (`buildActionCategoryTree`, `windowActionPaneNode`), `🏛️ShellHost/🟦️.tsx:5384`,
`🔌️PluginRuntime/🟦️.tsx:772`, `✏️editor/🦀️.rs:3920-3936 / 8497`, `📤️export-fixture/🦀️.rs`.

---

## 0 Headline

**The export lane is not broken.** Driven by a REAL hit-tested click on the Actions-pane `Export` row, Nakagin
on wasm #61 streams its whole 145 714 B fixture through the segmented lane and the browser fires a `download`
event named `nakagin-capsule-tower.json`, saved and re-parsed as 180 objects:

```
[357.6s] exportFixture strict click ok
[361.2s]   · warning: [DEBUG] performInvocation settled {"invocationKind":"action","instanceId":1,"actionId":"exportFixture","frames":2,"frameKinds":["Invocation","Ephemeral"],"historyCursor":null,"historyUpserts":0,"historyCanUndo":null,"effects":1}
[361.3s] download=nakagin-capsule-tower.json
[361.3s] download saved …/🗑️generated/b53-export-2026-09-12T21-30-35.json bytes=145714
[364.2s] page taps {"urls":[145714],"anchors":["nakagin-capsule-tower.json|blob:http://"],"saves":0}
```

B43's bridge fix IS live and IS sufficient: 3.7 s from press to file, one assembled blob of exactly
145 714 B, one anchor carrying the per-example filename. No refusal code, no worker fault, no drain stall.

**The hop that fails in the battery is the press itself.** `action.exportFixture` is row **79 of 96** in the
perspective window's Actions rail, at `y=1990` inside a pane band of `y 58…865` — 1 125 px below the fold of
a `overflow-y:auto` scroller (`scrollHeight=2365 clientHeight=807`). `activateWindowFileAction` never scrolls
that rail, so its `click({ force: true, timeout: 6000 })` times out on an element Playwright cannot hold
still outside the scroll viewport, and `export-only` reads `download=none` on a lane that works.

Two products are implicated, and only one is a product defect:

| | verdict |
|---|---|
| guest `Scene` stage → `PuzzleCommandWorkStep::Download` → segmented publication → host drain → sink → browser `download` | **correct, live on #61** (proven above) |
| the probe's press route (no scroll of the rail) | **probe defect** → recipe for B55 in §5, the probe is not edited here |
| the rail renders every declared action, including the ones declared NOT palette-visible | **product defect, host** → §4 |

---

## 1 The hop table — measured, `🔍️b53-export-hops.ts` on `:6013`

| hop | instrument | reading |
|---|---|---|
| 0 example switch | navbar label + `data-instances-json` bytes | `census after switch {"example":"Nakagin Capsule Tower",…,"instanceJsonBytes":[54254,54254]}` → `switch proven=true` (boot read `[266,266]` on Concrete Forest) — B48's blind switch is excluded |
| 1 Actions pane | `data-folded` on `framework.window.puzzle3dMainPerspective.engagement` | folded on arrival; the pane-chrome toggle press times out (B45's obstruction lineage) yet the pane opens ~28 s later: `pane folded=absent exportRows=1` |
| 2 the row exists | `[id="action.exportFixture"]` | `count=1`, `text=Export`, `display=block`, `visibility=visible`, `pointer-events=auto`, `aria-disabled=null` — never hidden, never disabled |
| 3 the row is BELOW the fold | `getBoundingClientRect` + scroll chain | `rect=[493,1990,294,24]`, `inViewport=false`, `ownsHit=false`, `hitChain=[]`; scroller `DIV[window-engagement-body] ov=auto scrollTop=0 scrollHeight=2365 clientHeight=807` |
| 4 the rail CAN scroll | one `scrollTop` write from the page | `after write scrollTop=900 rowY=1090` — the band scrolls, the row moves; nothing is clipped away |
| 5 press without a scroll | `locator.click({force:true})`, battery's route | `TimeoutError: click: Timeout 8000ms exceeded` for both the strict and the forced press (two independent runs), and **no** `performInvocation exportFixture` in the console — the dispatch never happens |
| 6 press after `scrollIntoViewIfNeeded` | rect sampled over 24 animation frames | `493,449,294,24@scrollTop=1541` × 24 — stable — then `exportFixture strict click ok` |
| 7 guest dispatch | `[DEBUG] command ingress lane` | `{"instanceId":1,"actionId":"exportFixture","seq":28,"lane":"Interactive"}` |
| 8 guest `Scene` stage → publication | `[DEBUG] command ingress settled` | `status=command-complete observed=command-complete`, 3.4 s after the press |
| 9 effect count | `[DEBUG] performInvocation settled` | `effects:1` — one `download-media-export`, the segmented marker arm (145 714 B is 2.2× the 65 536 B inline budget, so the inline arm is impossible) |
| 10 `takeSegmentedDownloadChunk` + host drain | refusal codes in the console | **none** — no `segmented-download-chunk-type/-empty/-over-cap/-outstanding-over-cap/-total-over-cap/-authority-invalid`, no worker fault |
| 11 sink | page taps on `URL.createObjectURL` / `HTMLAnchorElement.click` | `urls:[145714]`, `anchors:["nakagin-capsule-tower.json|blob:http://"]` — one blob of the EXACT payload, one anchor, the per-example name (B26/B30) |
| 12 browser `download` event | `page.waitForEvent("download")` armed BEFORE the press | `download=nakagin-capsule-tower.json`, `bytes=145714` |
| 13 the bytes | `json.load` of the saved file | `keys ['schema','domain','meta','objects','attractions','targetVolumes','references']`, `objects 180` — matches the inspector census `Objects 180` |

Hop 5 is the fault, and it is upstream of every guest and host hop this wave was sent to inspect. The probe's
listener timing is NOT the fault (hop 12 was armed before the press in both the battery and this wave).

## 2 Why the battery's own log already said so

`battery-2026-09-13-61-6013.txt:714-719` (full line, unsliced):

```
[1542.1s] action-pane exportFixture rows=1 unfold={"unfolded":true,…}
[1548.2s] action-pane exportFixture click failed TimeoutError: click: Timeout 6000ms exceeded.
  - waiting for locator('[id="action.exportFixture"]').first()
[1549.9s] export download=none example=Nakagin Capsule Tower expected=nakagin-capsule-tower.json
```

`rows=1` — the row was THERE; the press timed out. The pane dump beside it (`rows: rows.slice(0, 24)`) stops
at `action.createAttraction`, the 11th of 96, which is why five waves read the row list as if `Export` were
absent from the rail. It is row 79.

## 3 The rail, measured in full (96 rows, `y` in viewport px, band `58…865`)

| # | row | y | | # | row | y |
|---|---|---|---|---|---|---|
| 1 | `action.category.actions` | 118 | | 71 | `action.category.create` | 1798 |
| 2 | `action.setActiveExample` | 142 | | 75 | `action.category.selection` | 1894 |
| 3 | `action.importFixture` | 166 | | 78 | `action.category.file` | 1966 |
| … | 66 more `ACTIONS` rows | … | | **79** | **`action.exportFixture`** | **1990** |
| 70 | `action.setInteractionGranularity` | 1774 | | 80 | `action.openImportFixture` | 2014 |

The `ACTIONS` category alone carries **69** rows, and they are overwhelmingly raw dispatch plumbing —
`worldPointerDown`, `registerBrushMesh`, `suggestionsTick`, `fillBuildTick`, `transformBegin`/`transformEnd`,
`engagementInput`, `hoverSuggestion`, `setChunkSize`, `setPanelPage`, `setProximityRadius`, … — plus nine
framework-reserved verbs (`setActiveUtility`, `setActiveTool`, `startIntroduction`, `recordTutorial`,
`setHistoryCommandFilter`, `noteShellCommand`, `interactionSelect`, `interactionHover`, `revertToCommand`)
that the framework itself declares `in_palette: false` (`🛂️manifest/🦀️.rs:1002,1022,1040,1170,1176,1199,1211,1228,2642,2657`).
