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

**The hop that fails in the battery is the press, and its two budgets.** `action.exportFixture` is row
**79 of 96** in the perspective window's Actions rail, at `y=1990` inside a pane band of `y 58…865` —
1 125 px below the fold of an `overflow-y:auto` scroller (`scrollHeight=2365 clientHeight=807`), and
`activateWindowFileAction` never scrolls that rail. Scrolled in, the row is visible, enabled, stable and
hit-owning, and Playwright's own call log then shows the press reaching `performing click action` and
**hanging there past its 8 s budget** — the shell's main thread under the Nakagin document does not return
inside it (§7). The press is usually still DELIVERED, and the file then lands 21-35 s later, past the
probe's 20 s `waitForEvent("download")`. So `export-only` reads `download=none` on a lane that works,
for three compounding reasons: no scroll, a click budget below the main thread's stall, and a download
budget below the document's measured latency.

Two products are implicated, and only one is a product defect:

| | verdict |
|---|---|
| guest `Scene` stage → `PuzzleCommandWorkStep::Download` → segmented publication → host drain → sink → browser `download` | **correct, live on #61** (proven above) |
| the probe's press route: no scroll, a 6 s click budget, a 20 s download budget | **probe defect** → recipe for B55 in §5/§7, the probe is not edited here |
| the rail renders every declared action, including the ones declared NOT palette-visible | **product defect, host — FIXED** → §4.1 (96 → 81 rows live, §7) |
| the dev server never invalidates an edited source module on macOS, so host fixes measure as absent | **product defect, host — FIXED** → §4.2 |
| the shell's main thread does not acknowledge a pointer press within 8 s on the Nakagin document | **product finding, B54's lane** → §7 |

---

## 1 The hop table — measured, `🔍️b53-export-hops.ts` on `:6013`

| hop | instrument | reading |
|---|---|---|
| 0 example switch | navbar label + `data-instances-json` bytes | `census after switch {"example":"Nakagin Capsule Tower",…,"instanceJsonBytes":[54254,54254]}` → `switch proven=true` (boot read `[266,266]` on Concrete Forest) — B48's blind switch is excluded |
| 1 Actions pane | `data-folded` on `framework.window.puzzle3dMainPerspective.engagement` | folded on arrival; the pane-chrome toggle press times out (B45's obstruction lineage) yet the pane opens ~28 s later: `pane folded=absent exportRows=1` |
| 2 the row exists | `[id="action.exportFixture"]` | `count=1`, `text=Export`, `display=block`, `visibility=visible`, `pointer-events=auto`, `aria-disabled=null` — never hidden, never disabled |
| 3 the row is BELOW the fold | `getBoundingClientRect` + scroll chain | `rect=[493,1990,294,24]`, `inViewport=false`, `ownsHit=false`, `hitChain=[]`; scroller `DIV[window-engagement-body] ov=auto scrollTop=0 scrollHeight=2365 clientHeight=807` |
| 4 the rail CAN scroll | one `scrollTop` write from the page | `after write scrollTop=900 rowY=1090` — the band scrolls, the row moves; nothing is clipped away |
| 5 press without a scroll | `locator.click({force:true})`, battery's route | `TimeoutError: click: Timeout 8000ms exceeded` for both the strict and the forced press, in two independent runs. Delivery is a COIN FLIP: run 1 still logged `[DEBUG] performInvocation {"actionId":"exportFixture"}` after its timed-out presses, run 2 logged nothing at all |
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

## 4 The product defects, and the fix

### 4.1 The rail renders what it declares it excludes — `🛠️ShellHelpers/🟦️.tsx:3914`

`windowActionPaneNode`'s own docstring says it "resolves a window kind's **panel-eligible** actions", and
nothing in it ever was:

```ts
const resolvedActions = resolveWindowActions(app, windowKind);   // HEAD
```

`resolveWindowActions` deliberately preserves every declared action — it is the DISPATCH resolver, and its
own law says so ("preserves every definition owned by the window"). The two other user-facing surfaces
curate it themselves:

- the command palette — `🛠️ShellHelpers/🟦️.tsx:4050` `if (!definition.inPalette) continue;`
- the shell fallback menu — `🏛️ShellHost/🟦️.tsx:10522` `if (!action.inPalette) continue;`, above it the
  comment that names exactly this class: *"most apps declare internal/pointer-tracking view actions
  (worldHover, engagementInput, …) as window actions purely for dispatch plumbing; only palette-worthy
  ones belong in a user-facing menu"*.

The rail — a user-facing surface by the same standard — had no such filter, so it published the framework's
own reserved verbs (`interactionSelect`, `interactionHover`, `setActiveUtility`, `setActiveTool`,
`noteShellCommand`, `setHistoryCommandFilter`, `startIntroduction`, `recordTutorial`, `revertToCommand`,
all declared `in_palette: false` in `🛂️manifest/🦀️.rs`) and the app's own `in_palette(false)` verbs
(`importFixture`, `addObjectKind`, `deleteAttraction`, `deleteTargetVolume`, `setTargetVolumeFlag`).

**Fix** (host, one predicate, `🛠️ShellHelpers/🟦️.tsx:3914`):

```ts
const resolvedActions = resolveWindowActions(app, windowKind).filter((action) => action.inPalette);
```

placed BEFORE the `length === 0` early return, so a window whose every declared action is plumbing renders
no rail chip at all instead of an empty one.

**Law** — `🧪️tests/🔬️engine-contract/🟦️.ts`, in the existing `window action panel — staging and single
dispatch (P1/P2)` describe:

> `renders only palette-eligible rows, and no rail at all when every declared action is dispatch plumbing`

RED on HEAD with the live symptom, GREEN after:

```
× renders only palette-eligible rows, and no rail at all when every declared action is dispatch plumbing
AssertionError: expected [ 'action.category.actions', …(4) ] to deeply equal [ 'action.category.file', …(1) ]
+   "action.category.actions"
+   "action.worldPointerDown"
+   "action.interactionSelect"
    "action.category.file"
    "action.exportFixture"
```

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts --testNamePattern='palette-eligible rows|window action panel'
 Test Files  1 passed | 34 skipped (35)
      Tests  7 passed | 1049 skipped (1056)
```

### 4.2 Why the fix measured as ABSENT live — the dev server never invalidates an edited module on macOS

The first run after the fix still read `action rows count=96` and `Export` at `y=1990`. The served module,
fetched two ways, says why:

```
GET /@fs/…🛠️ShellHelpers/🟦️.tsx            → 650 282 B, occurrences of `action.inPalette`: 0
GET /@fs/…🛠️ShellHelpers/🟦️.tsx?t=99999    → 651 843 B, occurrences of `action.inPalette`: 1
```

The file on disk carries the fix; the server's CACHED transform does not. The chain:

| hop | where | what it does |
|---|---|---|
| 1 | `⚙️vite.config.ts:170` | `watch: null` — Vite runs no chokidar watcher (for a good, measured reason: FSEvents consolidation × 1 316 watched paths wedges the server under a concurrent cargo build) |
| 2 | `🔌️vite-plugins/🟦️.ts:872` `semioSourceWatchVitePlugin` | replays `node:fs` recursive-watch events on Vite's watcher emitter |
| 3 | `:883` (HEAD) | `server.watcher.emit(statSync(path).isDirectory() ? "addDir" : "add", path)` for every `rename` event whose path exists |
| 4 | macOS `fs.watch(root, { recursive: true })` | reports **`rename`** for a write to an existing file — measured, both styles: in-place `writeFileSync` → `rename:🧰️framework/🟦️.ts`; atomic temp+rename → `rename:🧰️framework/🟦️.ts.tmp` only |
| 5 | `vite/dist/node/chunks/config.js:25655` | `watcher.on("change")` → `moduleGraph.onFileChange(file)` — **the only invalidation path**; `watcher.on("add")` → `onFileAddUnlink` never touches the module graph |
| 6 | the running server's argv | `SEMIO_VITE_HMR=0` → `hmr: false` → `:25635` `onHMRUpdate` early-returns, so not even the HMR pass reaches the module |

So on macOS every edit of an already-transformed module reaches Vite as `add`, and the pre-edit transform is
served for the life of the server. `touch`ing the file emits a SECOND event on the same path, which macOS
does report as `change` — which is why the fleet's `touch` recipe works, and why this was invisible until a
fix was measured rather than assumed.

**Fix** (host, `🔌️vite-plugins/🟦️.ts`): an existing FILE is replayed as `add` AND `change` — both facts are
true of an atomic save (a new inode appeared; the module changed), and the `change` is idempotent for a
genuinely new file (nothing imports it, so it resolves to no module). Directories keep `addDir`.

**Laws** — `🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts`, beside the existing watch-policy laws, one per write style, each
proving the watcher armed on a SEPARATE file and then writing the target exactly ONCE (macOS answers only
the first write to a path with `rename`, so a retry loop over the target would have passed on its second
write while a real single save stayed invisible):

> `replays an in-place write over an existing source file as a change, the only event that invalidates Vite's module graph`
> `replays an atomic save over an existing source file as a change, the only event that invalidates Vite's module graph`

RED on HEAD, with the wrong event named:

```
× replays an in-place write … AssertionError: one save of a modified module must reach Vite as a change: expected [ …(4) ] to include 'change:🧰️framework/🟦️.ts'
× replays an atomic save  … AssertionError: … expected [ 'add:🧰️framework/🔎️arm.ts', …(3) ] to include 'change:🧰️framework/🟦️.ts'
```

GREEN after (the whole `dev server watch policy` group plus the rest of the file):

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts --testNamePattern='invalidates Vite|watch policy'
      Tests  23 passed | 52 skipped (75)
```

The one red file in that lane is `📜️script.ts`, and it is a peer's: `Cannot bundle built-in module
"bun:sqlite" imported from "…🦑️repo/🔨️modules/📚️library/⚡️caching/🔒️leases/🟦️.ts"` — the
`NX-COMPLETE-TASK-CACHING` lease store, untouched by this wave.

🧯 The watcher fix itself only takes effect for the NEXT `:6013` start (a plugin is a config dependency, and
with `hmr: false` Vite does not restart itself either). This wave must not restart a shared server, so the
ShellHelpers fix was made live the way the fleet already knows how: `touch`.

## 5 Recipe for B55 — the probe owns the press, and the press owns the scroll

`activateWindowFileAction` (`🔍️browser-probe.ts:1405`) unfolds the pane, then presses
`[id="action.<id>"]` with `click({ force: true, timeout: 6000 })`. `force` waives the actionability
CHECKS; it does not waive Playwright's scroll-and-hold, and a row 1 125 px below a re-rendering rail's fold
cannot be held still — measured twice as `TimeoutError: click: Timeout` with no dispatch, and once as
`strict click ok` the moment the row was scrolled into the band first. The fix is three lines, in the probe,
before the press (B55 owns the file; this wave does not touch it):

```ts
const row = page.locator(`[id="action.${actionId}"]`);
await row.first().scrollIntoViewIfNeeded({ timeout: 6000 }).catch((error) => log(`action-pane ${actionId} scroll failed ${String(error).split("\n")[0].slice(0, 120)}`));
// 📐️ Hit-tested, NOT forced: after the scroll the row owns `elementFromPoint` at its own centre
// (measured `ownsHit:true`, `hitChain:["SPAN[tree-label]", …, "DIV#action.exportFixture[tree-item-row]"]`),
// so a real press is available and a timeout here is a product finding again rather than a scroll miss.
await row.first().click({ timeout: 8000 }).catch((error) => log(`action-pane ${actionId} click failed ${String(error).split("\n")[0].slice(0, 160)}`));
```

Two more readings worth carrying into the probe, both measured here:

- the rail's row dump is sliced at 24 (`rows.slice(0, 24)`), which is why `Export` looked absent for five
  waves — it is row 79. Log `rows.length` beside the slice, or slice at the pane's own row count.
- the row's box is STABLE once scrolled (24 consecutive animation frames at
  `493,449,294,24@scrollTop=1541`), so a post-scroll timeout is never "the rail is animating".

## 6 The requested run, and what it proves

`bun 🔍️browser-probe.ts --only=example-switch,export-import --port=6013` (after the ShellHelpers fix, before
the rail could be curated live — §4.2):

```
[47.2s]  verdict example-switch-instances PASS
[91.7s]  action-pane exportFixture rows=1 unfold={"unfolded":true,"obstruction":{"present":true,"covered":false,…}}
[97.7s]  action-pane exportFixture click failed TimeoutError: click: Timeout 6000ms exceeded.
[97.7s]  export download=none example=Nakagin Capsule Tower expected=nakagin-capsule-tower.json
[97.7s]  verdict export-only FAIL download=none dest=…/probe-2026-09-12T22-08-26-export.json
[97.7s]  verdict export-names-the-example FAIL download=none expected=nakagin-capsule-tower.json
[106.1s] action-pane openImportFixture click failed TimeoutError: click: Timeout 6000ms exceeded.
[116.7s] verdict import-same-file-idempotent FAIL not reachable — export-only produced no file for THIS document
[116.7s] verdict import-distinct FAIL [expect-41] not reachable — export-only produced no file for THIS document
[116.7s] verdict import-distinct-records-history FAIL not reachable — export-only produced no file for THIS document
```

| verdict | this run | why, and who owns it |
|---|---|---|
| `example-switch-instances` | **PASS** | — |
| `export-only` | FAIL `download=none` | the press, not the lane: the SAME gesture with a scroll in front of it produced `nakagin-capsule-tower.json`, 145 714 B (§0). **Probe** (B55, §5) |
| `export-names-the-example` | FAIL `download=none` | ditto — the name itself is proven correct and per-example |
| `import-same-file-idempotent` | FAIL not reachable | blocked by `export-only`; `openImportFixture` is row 80 and fails the same press |
| `import-distinct` | FAIL not reachable | ditto |
| `import-distinct-records-history` | FAIL not reachable | ditto |

**Nothing here waits for #62.** No guest change was needed: the guest halves of the segmented lane are
already in #61 and they work (hops 7-13). The two host fixes are live from source (§4.1 after a `touch`,
§4.2 from the next server start). What remains is the probe's press route.

## 7 Run 5 — the curated rail live, and the press's real budget

Same probe, after the §4.1 fix was made live by `touch` (and it IS live: the served module carries the
predicate, §4.2):

```
[218.0s] switch proven=true
[229.1s] action rows count=81
[229.1s] exportFixture shape {"present":true,"rect":[493,1750,294,24],…}
```

**96 → 81 rows**, exactly the 14 non-palette verbs plus the `TARGETS` header whose three rows were all
non-palette, and `Export` rises 240 px (row 79 → 69, `y=1990 → 1750`). The rail still overflows the band, so
the rail curation alone does not put `Export` in front of a pointer — it removes the rows that had no
business being there, which is its own defect.

The press, with the row scrolled in and provably stable:

```
[233.3s] row stability ["493,552,294,24@scrollTop=1198", …×24 identical]
[241.4s] exportFixture strict click failed TimeoutError: click: Timeout 8000ms exceeded.
  - locator resolved to <div … id="action.exportFixture" …>
  - attempting click action
    - waiting for element to be visible, enabled and stable
    - element is visible, enabled and stable
    - scrolling into view if needed
    - done scrolling
    - performing click action        ← the log ENDS here
[249.5s] exportFixture forced click failed TimeoutError: click: Timeout 8000ms exceeded.
[254.7s] exportFixture synthetic dispatch synthetic on SPAN#-
[276.1s] download=nakagin-capsule-tower.json
[276.1s] download saved …/🗑️generated/b53-export-2026-09-12T22-54-20.json bytes=145714
[295.8s] page taps {"urls":[145714],"anchors":["nakagin-capsule-tower.json|blob:http://"],"saves":0}
```

Every actionability check passes and the press still times out **inside** `performing click action`: the
renderer never acknowledges the input within 8 s. That is a main-thread stall on the Nakagin document, not an
unreachable control — the same lane B54 measures — and it is why `force: true` made no difference in any run
(`force` waives the checks that were already passing).

The lane end-to-end, twice, on the same document:

| run | machine | press → `download` event | bytes | name |
|---|---|---|---|---|
| 3 | idle | **3.7 s** (`strict click ok` 357.6 s → 361.3 s) | 145 714 | `nakagin-capsule-tower.json` |
| 5 | loaded (two peer probes, cargo checks) | **21 s** after the delivered press (254.7 s → 276.1 s), 35 s after the first | 145 714 | `nakagin-capsule-tower.json` |

So the probe's `waitForEvent("download", { timeout: 20000 })` is itself under-budgeted for this document: the
listener arming was never the defect, the budget is. §5's recipe therefore needs all three:

1. `scrollIntoViewIfNeeded` before the press (the row is 1 125 px — now 885 px — below the fold);
2. a click budget above the main thread's stall, or a tolerated click timeout (the press is delivered anyway
   — `[DEBUG] performInvocation {"actionId":"exportFixture"}` appears after a timed-out press);
3. `waitForEvent("download")` at ≥ 60 s for `export-only` on Nakagin (measured 3.7 s idle, 21-35 s loaded).

🧪 The synthetic `MouseEvent` dispatch in run 5 is a DIAGNOSTIC only — it proves the row's handler and the
whole lane, never that a user can reach it. The user-route proof is run 3's hit-tested `strict click ok`.

## 8 Verification — every command in the FOREGROUND, tails quoted

### 8.1 `@semio-tech/framework-renderer-react` (the host half + the new rail law)

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts --testNamePattern='palette-eligible rows|window action panel'
 Test Files  1 passed | 34 skipped (35)
      Tests  7 passed | 1049 skipped (1056)
```

Full lane (`🗑️generated/b53-renderer-react.txt`):

```
 ❯ …🧪️tests/🧩️package-integration/🟦️.ts (20 tests | 6 failed) 3397ms
 ❯ …🧱️elements/🔌️PluginRuntime/🟦️.tsx (108 tests | 1 failed) 8346ms
 Test Files  2 failed | 33 passed (35)
      Tests  7 failed | 1049 passed (1056)
```

`🧪️tests/🔬️engine-contract/🟦️.ts` — the file this wave's law lives in, and the only file that reads the
pane — passes whole (588 tests). **None of the 7 reds is new or attributable here:**

- 6 in `🧩️package-integration` are toolchain laws (`the repository-pinned Bun runtime`, `identical bytes
  twice with matching independent SHA-256`, `byte-identical worker bytes`, `digest-verified devcontainer …
  packageManager pin`, `astral-emoji-bearing browser entry`, `only the current no-follow package through
  both independent TypeScript compilers`) — nothing about actions, panes or downloads;
- 1 is `keeps the leftover vortex id on an armed brush window` in `🧱️elements/🔌️PluginRuntime/🟦️.tsx`, a file
  this wave never touched, asserting `🌐️World3dHost/🟦️.tsx` — which a peer committed at
  `8add1df147 2026-09-12 21:17:59 +0200`, mid-wave (`git log -1 --date=iso`), and which times out at 5 004 ms.

### 8.2 `@semio-tech/framework-os-dev` (the watcher half)

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts --testNamePattern='invalidates Vite|watch policy'
      Tests  23 passed | 52 skipped (75)
```

One red FILE, a peer's and not a test: `FAIL |@semio-tech/framework-os-dev| 📜️script.ts` →
`Cannot bundle built-in module "bun:sqlite" imported from "…🦑️repo/🔨️modules/📚️library/⚡️caching/🔒️leases/🟦️.ts"`
(the `NX-COMPLETE-TASK-CACHING` lease store).

### 8.3 `tsc --noEmit` (scoped to the touched files)

```
bun node_modules/typescript/bin/tsc --noEmit --strict … 🛠️ShellHelpers/🟦️.tsx      → no diagnostic naming inPalette / windowActionPaneNode / the changed lines
bun node_modules/typescript/bin/tsc --noEmit --strict … 🔌️vite-plugins/🟦️.ts 🧹️config/🟦️.ts
  🧹️config/🟦️.ts(14,23): error TS7016: Could not find a declaration file for module 'picomatch'
```

The one diagnostic is the loose invocation's own (the file's vitest config resolves `picomatch`); the changed
lines (`🔌️vite-plugins/🟦️.ts:879-891`) report nothing, and the rest of the noise this invocation surfaces
(`ImportMeta.dir`, `bun:sqlite`, implicit `any` in unrelated `🎭️actor` trees) is repo-wide and pre-existing.

### 8.4 Rust — unchanged by this wave, re-proven

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib export -- --test-threads=1
test editor::puzzle3d::component::unit_tests::export_fixture_downloads_round_trippable_json ... ok
test editor::puzzle3d::component::unit_tests::export_fixture_names_the_download_after_the_active_example ... ok
test editor::puzzle3d::component::unit_tests::export_over_the_inline_budget_streams_one_segmented_download_carrying_the_whole_fixture ... ok
test editor::puzzle3d::component::unit_tests::export_refuses_a_payload_above_the_declared_segmented_budget_with_a_notice ... ok
test editor::puzzle3d::component::unit_tests::exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity ... ok
test editor::puzzle3d::component::unit_tests::import_fixture_reproduces_the_exported_document ... ok
test editor::puzzle3d::component::unit_tests::leftover_export_fixture_downloads_the_boot_example_json ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 746 filtered out; finished in 27.23s
```

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
warning: `semio-s-artifact-puzzle-3d` (lib) generated 95 warnings
    Finished `dev` profile [unoptimized] target(s) in 35.20s          → 0 errors

cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
    Checking semio-s-plugin-puzzle v0.1.0 (…/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 2m 00s          → 0 errors
```

No wasm build was run, and none is needed: the guest half of this lane is already in #61.

## 9 Files

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `windowActionPaneNode` filters `inPalette`, the curation its own docstring promised; docstring states the rule and the measurement |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | the rail-curation law (RED on HEAD with the live symptom) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts` | `semioSourceWatchVitePlugin` replays an existing file as `add` AND `change`, so an edit invalidates Vite's module graph on macOS |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts` | two watcher-invalidation laws, one per write style, single-write with a separate arming file |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/🔍️b53-export-hops.ts` | this wave's hop tap (retained: rail census, scroll/stability/hit diagnostics, sink taps, download proof) |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/📓️2026-09-13-wave-B53-nakagin-export-full-run.md` | this report |

## 10 Open, for the coordinator

1. **The probe's press route** (B55): §5 + §7's three budgets. Until then `export-only`,
   `export-names-the-example` and the three `import-*` verdicts stay red on a lane that demonstrably works.
2. **The main-thread stall** (B54's lane): a pointer press on the Nakagin document is not acknowledged
   within 8 s, and the export takes 21-35 s under load versus 3.7 s idle.
3. **The app's own curation debt** (rides #62, not taken here): `✏️editor/🦀️.rs` declares ~50 raw dispatch
   verbs (`worldPointerDown`, `registerBrushMesh`, `suggestionsTick`, `fillBuildTick`, `transformBegin`,
   `engagementInput`, `hoverSuggestion`, `setChunkSize`, `setPanelPage`, …) as palette-visible, so they still
   fill the rail, the command palette AND the shell fallback menu. Each `.in_palette(false)` there removes a
   row from all three surfaces; ~25 of the 81 remaining rows are user verbs, which would put `Export` inside
   the pane band without any scroll. It is a judgment call per verb and it changes palette/menu verdicts
   other waves score, so it wants its own wave rather than a sweep inside this one.
4. **`SEMIO_VITE_HMR=0`**: with the watcher fixed, invalidation no longer depends on HMR — but the running
   `:6013` predates both fixes, so every host change landed before its next start is served stale unless the
   file is `touch`ed a second time. Worth one restart at the coordinator's convenience.

## 11 The same run AFTER the rail curation went live — unchanged, and predicted

`bun 🔍️browser-probe.ts --only=example-switch,export-import --port=6013`, with the 81-row curated rail
served (§7):

```
[115.1s] action-pane exportFixture rows=1 unfold={"unfolded":true,"obstruction":{"present":true,"covered":false,…}}
[121.5s] export download=none example=Nakagin Capsule Tower expected=nakagin-capsule-tower.json
[164.7s] battery verdict boot PASS
[164.7s] battery verdict example-switch-instances PASS
[164.7s] battery verdict export-only FAIL download=none
[164.7s] battery verdict export-names-the-example FAIL download=none expected=nakagin-capsule-tower.json
[164.7s] battery verdict import-same-file-idempotent FAIL not reachable — export-only produced no file for THIS document
[164.7s] battery verdict import-distinct FAIL [expect-41] not reachable — export-only produced no file for THIS document
[164.7s] battery verdict import-distinct-records-history FAIL not reachable — export-only produced no file for THIS document
[164.7s] battery verdict guest-alive-replace PASS
[164.7s] battery verdict battery-hard-faults PASS
[164.7s] battery verdict battery-faults PASS
```

Predicted and unchanged: curation lifts `Export` to `y=1750`, still 885 px below the band's 865 px edge, so
the press route still has no scroll and still under-budgets both the click (6 s) and the download (20 s).
`FAULTS=0`, `guest-alive-replace PASS` — the actor behind the lane is alive throughout, so these five reds
measure the press, not a corpse.

**Waits for #62: nothing.** Waits for B55's probe recipe: all five.
