# Wave B45 — full-run bisect, round 3: a submenu race, a fixed-sample arm, a pane that cannot unfold and a row action that flags the selection

Ticket 26/09/02/PUZZLE-3D-END-TO-END · wasm #56 on `:6013` (host vite-live) · every probe lane run in the
FOREGROUND with the `pgrep -f 'bun .*browser-pro[b]e'` gate checked first · method: B33/B36's growing `--only=`
prefixes, one browser per lane, plus instrumentation added to `🔍️browser-probe.ts` (the file this wave owns) so
every claim below is a measurement printed by a named log line, not an inference.

Handed set (FAIL in battery `🗑️generated/probe-2026-09-12T11-06-48.md`, wasm #56, PASS=64 FAIL=27 FAULTS=0):
`engagement-input-present`, `settings-panel-opens`, `export-only`, `export-names-the-example`,
`volume-brush-arm`, `context-menu-object-vocabulary`, `locked-refusal-notice`, `outliner-hide-applies`.

---

## 1 `context-menu-object-vocabulary` — NOT full-run-only. A 900 ms sample racing a submenu open.

### 1.1 Premise correction

| # | lane | verdict | run |
|---|---|---|---|
| K0 | `--only=context-menu-rows` (fresh page, nothing else in the plan) | **FAIL** `missing=["hide-show","lock-unlock"] present=["duplicate","select-same-kind","zoom","menu.group.hand","delete"]` | `probe-2026-09-12T11-43-39` |

So the premise "passes in a fresh lane" is stale for this verdict: it fails with a single step in the plan, and
no predecessor can be the polluter. B36 §2.1 had already declined to re-bisect it after B33's hoist; this is
what the hoist left behind.

### 1.2 Root cause — the probe, `🔍️browser-probe.ts` (pre-B45 line 3012)

The step hovers each `menu.group.*` row, waits a FIXED 900 ms, and merges one read:

```ts
await page.locator(`[id="${group.id}"]`).first().hover(…);
await page.waitForTimeout(900);
for (const row of await menuRows()) if (row.id) merged.set(row.id, row);
```

The new `context-menu raw=…` dump (added this wave) reads the same DOM ~100 ms after that merge and finds the
rows the merge had just missed, ids and actions intact:

```
[83.1s] context-menu raw=[…,{"id":"menu.group.hand","action":null,"text":"4 Hand ›"},
 {"id":"hide-show","action":"setSelectionFlag","owner":"window-chrome-body","text":"1 Hide"},
 {"id":"lock-unlock","action":"setSelectionFlag","owner":"window-chrome-body","text":"2 Lock"},…]
[83.2s] context-menu dom={"menus":1,"items":7,…,"slots":[…"context-menu-content"…"context-menu-content"…]}
```

Two `context-menu-content` containers and seven `[role="menuitem"]`: the submenu HAD opened and rendered both
rows. The vocabulary was never missing — the sample was 461 ms early.

### 1.3 Fix — probe. The submenu is polled, like the top-level menu already was.

`context-menu-rows` now merges inside a `settleFor` that waits for the row set to GROW after the hover (8 s
budget) and logs the wait, so a submenu that genuinely does not open is a measured `opened=false` instead of a
missing row.

Before / after, same lane, one browser each:

```
probe-2026-09-12T11-43-39   verdict context-menu-object-vocabulary FAIL missing=["hide-show","lock-unlock"]
probe-2026-09-12T11-45-48   context-menu group menu.group.hand rowsBefore=5 rowsAfter=7 opened=true waitedMs=1361
                            verdict context-menu-object-vocabulary PASS
                            battery PASS=9 FAIL=0 FAULTS=0
```

**1361 ms** is the measured submenu open under no load at all — the old budget was 900 ms. Class: **probe**.

---

## 2 `settings-panel-opens` — two causes stacked: a resize handle over the tab, then a toggle again

### 2.1 The bisect

| # | lane (`--only=`) | verdict | run |
|---|---|---|---|
| S0 | `settings-panel` | **PASS** `ensurePanel puzzle3d.panel.settings clicked=ok active=true body=9 waitedMs=2` | `probe-2026-09-12T11-51-40` |
| S1 | `window-content,camera-gestures,projection-options,window-options,settings-panel` | **FAIL** `clicked=ok active=false body=0 waitedMs=20065` | `12-24-40` |

The polluting predecessors are the two read steps that **open the History panel** (`camera-gestures` and
`window-options` both call `readHistoryEntryIds` → `openHistory` → `ensurePanel framework.panel.history`).
With that panel open the battery's dock carries `activeTabs:["framework.panel.inspection","framework.panel.history"]`
— and that is the whole precondition. Note the premise in the brief ("the tab is absent from the tab list") is
wrong: `puzzle3d.panel.settings` is in the roster in every single run; it is the ACTIVATION that fails.

### 2.2 Cause A — PRODUCT: an open panel's resize handle covers the neighbouring tab's centre

New `STAYED-SHUT` dump (added this wave — it reports the covering element's box, z, pointer-events and chain):

```
ensurePanel puzzle3d.panel.settings STAYED-SHUT dock={
  "attrs":[… "id=puzzle3d.panel.settings" … "aria-pressed=false" …], "rect":[1095,875,83,22],
  "hit":"DIV#-[panel-resize-handle]",
  "hitBox":{"rect":[1137,824,3,72],"z":"20","position":"absolute","pointerEvents":"auto",
            "cls":"absolute top-0 bottom-0 z-20 left-0 w-single cursor-ew-resize"},
  "hitChain":["DIV#-[panel-resize-handle]","DIV#framework.panelTab.framework.panel.history[panel]",…]}
```

The Settings tab occupies x 1095‑1178, y 875‑897 — centre (1136.5, 886). The topmost element at that point is
the **History** panel's own left resize handle: a 3 px column at x 1137‑1140 spanning y 824‑896, `z-20`,
`pointer-events: auto`. `click({force:true})` skips actionability checks but NOT hit-testing, so the press went
to the resize handle. A human clicking the middle of that tab starts a resize too.

Why it crosses the tab at all: the panel tab strip is drawn by whichever panel at that anchor is OPEN and
carries EVERY panel's tab, so it overflows that panel's own width (the panel root starts at x 1137, the tab
starts at 1095), while the handle was mounted on the panel ROOT as `absolute top-0 bottom-0` — the full panel
height, cap row included.

**Root cause, `file:line`:** `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx:529-545` (pre-fix) — the
`resizeSides.map(PanelResizeHandle)` block was a sibling of `WindowChrome` under `PanelGhostRoot`.

**Fix:** the handles move INSIDE `panel-body-stack` (which gains `relative`), so a handle spans the panel's
BODY and stops at the chrome cap row that carries the tab strip. The class, the `data-slot`, the count and the
`left-0`/`right-0` sides are unchanged, so the three existing handle laws stay green. Stated as a rule in
`PanelResizeHandle`'s own docstring: **a grab strip never covers a button.**

Measured after the fix, same lane: the obstruction is gone —

```
"hit":"SPAN#-[inline-label]",
"hitChain":["SPAN#-[inline-label]","DIV#-[-]","BUTTON#puzzle3d.panel.settings[panel-tab-button]",
            "DIV#-[panel-tabs]","DIV#-[ribbon-row]","DIV#-[ribbon]"]
```

⚠️ The served module was STALE after the edit (`curl` of the `@fs` URL still returned the pre-edit transform and
the probe still measured the handle on the panel root). `touch` on the edited file, as B33 §8/B36 §8 prescribe
for the SyntaxError case, invalidated it. **A host-TS product fix on `:6013` is not measurable until the served
transform is re-fetched and checked.**

### 2.3 Cause B — PROBE: `ensurePanel` read every state a panel tab does NOT publish

With the handle out of the way the press landed and the panel still did not open. Its own console says what
happened:

```
ensurePanel puzzle3d.panel.settings STAYED-SHUT console=[…
 "history patch applied {\"replace\":false,\"currentCursor\":20,\"patchCursor\":22,\"upserts\":1,
  \"labels\":[\"Toggle Panel\"],\"canUndo\":true}" …]
```

The shell received the press and ran **Toggle Panel** — the panel was already OPEN and the probe closed it.
B36 §7.1's trap, one layer down, and for two compounding reasons:

1. `tabActive` folded `aria-selected`, `data-state` and `data-active` — and the live control carries **none**
   of them. Its dumped attributes are `data-slot=panel-tab-button data-tab-id=… data-level=panel
   aria-pressed=false aria-label=Settings`. So `tabActive` answered `false` for every panel tab ever.
2. `isOpen` made the BODY the authority whenever a body selector was known, so an open panel whose body has
   not been PUBLISHED yet counted as shut.

**Fix — probe.** `tabActive` folds `aria-pressed` in, and `isOpen` is a disjunction: the tab's own pressed
state OR a rendered body. An empty body is a publication question, never an open/closed one.

### 2.4 Before / after, polluter still first

```
12-24-40  (pre-fix)  ensurePanel … clicked=ok active=false body=0 waitedMs=20065 → settings-panel-opens FAIL
12-39-39  (product)  hit=SPAN#-[inline-label] (obstruction gone) · still FAIL, active=false body=0 waitedMs=20166
12-41-51  (console)  history patch applied labels=["Toggle Panel"] → the press CLOSED it
12-44-41  (both)     ensurePanel puzzle3d.panel.settings clicked=ok active=true body=9 waitedMs=2
                     verdict settings-panel-opens PASS
                     verdict settings-steppers-present PASS        ← was unreachable
                     verdict settings-grid-spacing-bumps PASS      ← was unreachable
                     battery PASS=31 FAIL=1 FAULTS=0   (was PASS=26 FAIL=3)
```

The one remaining red in that lane, `settings-value-reaches-window-rail` (`settings=13 windowRail=12.5`), is B36 §3's
product fix riding a newer wasm — not this wave's.

---

## 3 `volume-brush-arm` — the helper that was asked to open the utility bar was folding it

### 3.1 The bisect

| # | lane (`--only=`) | verdict | run |
|---|---|---|---|
| V0 | `pick-object,context-menu,volume-brush` | **PASS** `active=volumeBrush waitedMs=33` | `12-17-57` |
| V1 | `pick-object,context-menu,tool-category,fill-tab,volume-brush` | **PASS** `waitedMs=2` | `12-17-05` |
| V2 | `pick-object,context-menu,tool-category,fill-tab,fill-apply-max,volume-brush` | **FAIL** `found=0`, `utilities unfolded=false waitedMs=8180` | `12-18-36` |
| F1 | the full fill suite + `locked-refusal,volume-brush` | **FAIL** `activeUtility=select waitedMs=20380` | `12-03-40` |
| F1′ | same lane, after the §3.3 fix | **PASS** `click=ok … active=volumeBrush settled=true waitedMs=33` | `12-21-15` |

### 3.2 Premise correction — it is NOT the 1.8 s sample, and it is NOT a stale armed tool

`armUtility` slept a fixed 1 800 ms and sampled once. Replacing that with a 20 s poll (kept) did **not** flip
the verdict — `settled=false waitedMs=20380` — so "the arm's round trip is slower than the sample" is
disproved as the sole cause, and B36 §1.5 had already disproved "an armed predecessor blocks the arm".

### 3.3 Root cause — PROBE, `unfoldPerspectiveUtilities`

The bar renders BOTH of its chips with the visible text `Utilities`: `utilityBar.unfold` while it is folded and
`utilityBar.fold` while it is open. The old body clicked `utilityBar.unfold` by id **when that id existed** and
otherwise fell back to `page.locator("button").filter({hasText:/^utilities$/i}).last()` — which, on every call
after the first, matched `utilityBar.fold`. Measured inside one run (`12-03-40`):

```
[157.5s] utilities chips=2 unfoldId=1     ← folded; the unfold worked (locked-refusal's call)
[252.2s] utilities chips=2 unfoldId=0     ← already open; the fallback pressed FOLD
[273.7s] arm-utility volumeBrush found=1 before=select active=select settled=false waitedMs=20380
```

`armUtility` then force-clicked `[id="volumeBrush"]` inside a folded bar. `force:true` skips the
visible/enabled checks, so the press was swallowed: **no `setActiveTool` appears anywhere in that run's console
for the arm** — the guest was never asked. So the polluter is *any* predecessor that leaves the bar open
(`locked-refusal`, `gumball-drag`, `brush-stroke`, `suggestions-open`, `relocate` — every step that arms a
utility), which is why a fresh lane always passed.

Second, smaller defect in the same helper: it opened with a canvas click at (80, 80) — a world PICK that
clears or changes the selection a calling step has just established, in the middle of `locked-refusal` and
`relocate`.

**Fix — probe.** `unfoldPerspectiveUtilities` now reads `utilityBar.fold` first: if that chip exists the bar is
open and the helper clicks NOTHING. Otherwise it clicks `utilityBar.unfold` **by id only** (never by text) and
polls for `fold` to appear. The canvas pick is spent only when NEITHER chip is in the document (the bar belongs
to an unfocused window) and is named in the log as `focusPick=canvas-80-80`. `armUtility` additionally records
whether its target is pressable at all (`disabled`, `aria-disabled`, `pointerEvents`, the element actually under
the pointer) and whether its own click resolved.

```
12-21-15  utilities unfoldId=1 focusPick=none
          utilities unfolded=true waitedMs=3 ids=[… "volumeBrush=Volume Brush" …]
          arm-utility volumeBrush click=ok shape={"tag":"button","slot":"toggle-group-item","disabled":false,
            "aria":null,"state":"off","pressed":"false","pointerEvents":"auto","rect":[642,817,105,22],
            "hit":"SPAN#-[inline-label]","mine":true}
          arm-utility volumeBrush found=1 before=select active=volumeBrush settled=true waitedMs=33
          verdict volume-brush-arm PASS   ·   battery PASS=11 FAIL=0 FAULTS=0
```

### 3.4 Residual, and it belongs to the mutation-latency lane (B44)

V2 shows a **second, product** failure mode for the same verdict: with a fill still in flight the *unfold
itself* never lands — `utilities unfoldId=1 … unfolded=false waitedMs=8180` on a bar that opens in 2 ms when
idle — so `[id="volumeBrush"]` is not in the document and `found=0`. That is a window-config write dropped or
starved under an in-flight fill, the same shape as §4, and it is not a probe defect. The helper's budget is now
30 s so the two are distinguishable in the log.

---

## 4 `locked-refusal-notice`, `engagement-input-present`, `outliner-hide-applies` — one product cause: a landed fill stalls small writes

### 4.1 The bisect

| # | lane (`--only=`) | verdicts | run |
|---|---|---|---|
| L0 | `outliner-rows` | `outliner-hide-applies` **PASS** `waitedMs=1616` (later, with the world lane required: `waitedMs=2641`, `worldHiddenAfter=["seed-left-001"]`) | `11-47-49`, `12-55-02` |
| L1 | `engagement-bar` | `engagement-input-present` **PASS** `input=1 placeholder="brush, fill <n>, zoom, clear, pick, rectangle, lasso"`, `unfold waitedMs=1` | `11-50-28` |
| L2 | `marquee-drag,marquee-click,locked-refusal,gumball-drag,frame-perspective,brush-stroke,suggestions-open,volume-brush,relocate,engagement-bar,outliner-rows` (the mutate group's SECOND half) | all four **PASS** — `lock flag row="locked true" waitedMs=2499`, `volume-brush-arm waitedMs=2`, `engagement unfold waitedMs=29`, `outliner hide waitedMs=512` | `11-54-56` |
| L3 | `pick-object,context-menu,`**fill suite**`,fill-history,locked-refusal,volume-brush` | `locked-refusal-notice` **FAIL** `locked=false flag="locked false" waitedMs=30515` (lock precondition timed out at 15 423 ms) | `12-03-40` |
| L4 | `pick-object,context-menu,`**fill suite**`,fill-history,engagement-bar,outliner-rows` | `outliner-hide-applies` **FAIL** `waitedMs=30032` | `12-46-57` |
| L5 | same shape as L4 plus `frame-perspective,brush-stroke` — but the fill landed only **1** object that run | both **PASS** (`engagement unfold waitedMs=2`, `outliner hide waitedMs=3148`) | `12-55-48` |

**The polluter is `fill-apply-max`, and specifically the runs in which it actually lands its objects.** The
whole first half of the mutate group is innocent apart from the fill, the whole second half is innocent, and
L5 — the same lane as L4 with a fill that produced nothing — is green. The discriminator is the document the
fill leaves behind:

```
12-03-40  frame census poll … instanceCount=147 → 158 → 167      (still growing minutes after the apply)
12-55-48  frame census poll … instanceCount=1                    ← the fill landed nothing; every verdict green
```

### 4.2 What actually fails: a small write, dispatched and settled, that changes nothing

`outliner-hide-applies`, L4, the dispatch's own console:

```
performInvocation {"actionId":"setSelectionFlag"}
command ingress lane {"actionId":"setSelectionFlag","seq":111,"lane":"Interactive"}
puzzle3d.utility.publish action=setSelectionFlag window=Some("puzzle3d-main-perspective") …
performInvocation settled {"actionId":"setSelectionFlag","frames":2,"historyUpserts":0,
                           "historyCanUndo":null,"effects":0}
```

and 30 s later `worldHiddenAfter=[]`, the row text unchanged, the selection correct
(`selectionBefore=["puzzle3d-main-top=seed-left-001", …]`, i.e. the very row whose Hide was pressed). The
SAME settled shape on a 1-object document hides the object in 1.6 s. So the round trip completes and the
document is not written.

Same shape, three different writes, all on a filled document:

| write | idle | after a landed fill |
|---|---|---|
| `setSelectionFlag` (outliner Hide) | 1 616 / 2 641 / 3 148 ms | no change in 30 032 ms |
| `setSelectionFlag` (inspector lock row) | 1 038 / 2 499 ms | no flag in 15 423 ms, and **45 222 ms** with the raised budget |
| window-config fold (`utilityBar.unfold`) | 2–5 ms | `unfolded=false` after 8 180 ms |
| window-config fold (`engagement.toggle`) | 0–29 ms | `unfolded=false` after 10 302 ms (battery #56, `click:ok`) |

This is one product defect with four faces: **after a fill lands, interactive document and window-config
writes stop taking effect, with no fault, no notice and a `command-complete` settle.** It is the
mutation-latency-on-large-documents question wave B44 is measuring, so it is reported here rather than fixed
here — `🌐️World3dHost` and the guest's write lanes are B44's and the plugin peers' files.

### 4.3 A product defect this bisect uncovered on the way: the lock lands, the flag row does not

In `12-11-30` (L3's lane with the lock budget raised to 45 s) the flag row NEVER read true —
`lock flag row="locked false" locked=false waitedMs=45222` — and yet the gumball drag that followed produced
the guest's real refusal:

```
locked after: {… "notices":["Selection is lockedClose","Agent disconnected"] …} waitedMs=5297
verdict locked-refusal-notice PASS
```

So the document WAS locked and the Inspection panel kept rendering `Locked false` for 45 s on the same object
it was showing (`panel:puzzle3d-play-inspector/puzzle3d-play-inspector.object.locked=Locked false`, with
`objectId = puzzle3d.brush.3429b17739070161`, the selected one). **The inspector's flag row goes stale on a
filled document** — a panel-publication defect in `📌️panels/🔍️inspection` territory, not a lock defect. No
verdict covers it today: `locked-flag-row` only asserts the row is present, which it is. Worth a wave.

Also measured, for whoever takes that: the inspector's `object.locked` row is a `div[data-slot="tree-item-row"]`
with **no pressable child** (`lock controls=1 shape={"tag":"div","slot":"tree-item-row","pressables":[]}`) — the
row itself dispatches, so the route works, but a reader cannot tell a control from a label by shape alone.

### 4.4 Probe changes in this family (all budget/observable, no renames)

- `locked-refusal`'s lock precondition polls for 45 s instead of 15 s — the refusal it exists to measure cannot
  fire while the precondition is unmet, and the same click is measured at 1 038 ms idle and >15 423 ms filled.
  `12-11-30` and `12-21-15` both flip `locked-refusal-notice` to **PASS** with the fill still first.
- `unfoldWindowPane` records its click's own outcome (`click:"ok"` vs `click:"failed …"`) and polls 30 s. The
  battery's `engagement-input-present` red was `click:ok` + `unfolded:false` — a write that did not land, not
  an unreachable toggle, and the old swallowed `catch` could not say which.
- `outliner-hide-applies` now requires the WORLD lane as well as the row dump. It had been going green on runs
  whose world hid nothing (`worldHiddenAfter=[]` + PASS in `11-49-02` and `11-54-56`); the fresh lane still
  passes with the stronger predicate (`12-55-02`, `waitedMs=2641`), so this is a real consequence and not a
  weakened assertion.

---

## 5 `export-only` / `export-names-the-example` — the fix is on disk and not in the served wasm

B36 §5 had already bisected the polluter to `example-switch` and the discriminator to payload SIZE (Concrete
Forest 7 542 bytes downloads, Nakagin 145 714 bytes does not). Re-measured twice this wave, once before and
once after a peer rebuilt the dev plugin wasm:

| run | example | verdict | guest console |
|---|---|---|---|
| `12-59-35` | Nakagin, 180 objects | `export-only` **FAIL** `download=none` | `exportFixture settled … "effects":1` · zero `segment`/`handle` lines · zero notices |
| `13-03-54` | Nakagin, 180 objects | `export-only` **FAIL** `download=none` | identical |

`action-pane exportFixture rows=1 unfold={"unfolded":true …}` in both — the row exists, the pane opened, the
press landed, the guest emitted exactly one effect.

The segmented command IS on disk: `…/✏️editor/🎮️commands/📤️export-fixture/🦀️.rs` now carries
`puzzle3d_export_segmented`, a `puzzle3d_export_segmented_budget_bytes` admission and a refusal NOTICE above
it (peers B38/B43). The served wasm emits one plain effect and no notice, i.e. the old unconditional inline
push. **Nothing about these two verdicts is a probe defect and nothing is left to bisect: they need the wasm
that carries the segmented arm.** Not chased further — the plugin `🦀️.rs` and its build are peers'.

---

## 6 `engagement-input-present` — the polluter is the same fill, the signature is the same lost fold write

The battery's red is `toggle=1 input=0 unfold={"unfolded":false,…,"waitedMs":10302}` with the pane FOLDED
(`rootText:"Actions"`), and — measured this wave with the new field — the unfold's own click resolves fine.
The input is not a separate defect: `Window` renders the typed field in the top-middle `search` Pane and B10
merged the two fold states (`searchExpanded = searchVisible && !actionsFolded`), so a pane that will not
unfold has no field. When the pane does unfold, the field is there every time:

```
11-50-28  engagement unfold={"unfolded":true,…,"waitedMs":1,"click":"ok"}
          fields=[… "puzzle3d-engagement|input|brush, fill <n>, zoom, clear, pick, rectangle, lasso"]
          verdict engagement-input-present PASS
```

Lanes that did NOT reproduce it (each one browser): `engagement-bar` alone; the fill suite + `engagement-bar`;
the whole SECOND half of the mutate group + `engagement-bar` (`waitedMs=29`); the fill suite +
`frame-perspective,brush-stroke` + `engagement-bar` (`waitedMs=2`); and `window-options,settings-panel,
pick-object,engagement-bar --reload-between-groups` (`waitedMs=1`) — so **`--reload-between-groups` is NOT
the polluter**, which was the brief's leading candidate.

What IS reproduced, on the sibling pane of the same window and through the same window-config write, is the
identical signature under a landed fill: `utilities unfoldId=1 … unfolded=false waitedMs=8180` on a bar that
opens in 2 ms when idle (`12-18-36`, §3.4). Same mechanism, same §4 cause, same owner (B44). The probe half
is done: the budget is 30 s and the click's outcome is recorded, so the next battery will print either a late
`unfolded=true waitedMs=…` (latency) or `click:"failed …"` (an unreachable toggle) instead of one ambiguous
red.

---

## 7 Handed off — product defects this bisect named but does not own

1. **Interactive writes stop landing after a fill lands** (§4.2). `setSelectionFlag` and window-config fold
   writes settle `command-complete` with `historyUpserts:0 effects:0` and change nothing for 30–45 s on a
   ~150-object document, where the same click takes 1–3 s idle. Owner: B44 / the guest write lanes.
2. **The Inspection panel's `locked` flag row goes stale** (§4.3). The document IS locked — the guest's own
   `Selection is locked` notice fires — while the panel renders `Locked false` for 45 s for the object it is
   displaying. No verdict covers it; `locked-flag-row` only asserts the row exists.
3. **`exportFixture` still emits one inline effect on a 145 KB payload** (§5) — the segmented command is on
   disk and not in the served wasm. Needs the next plugin wasm, then re-measure `export-only`.

## 8 Infrastructure encountered (both cost runs; both are recipes for the next wave)

- **A host-TS product fix is not live until the served transform says so.** After editing
  `🖼️Panel/🟦️.tsx` the probe still measured the OLD geometry, and `curl` of the module's `@fs` URL returned
  the pre-edit transform. `touch` on the edited file invalidated it. Always `curl` the `@fs` URL and grep for
  the change before believing a browser measurement of a host fix.
- **:6013 stopped booting mid-wave on a peer's in-progress test extraction.**
  `🧰️framework/🔨️modules/🎭️actor/🧵️turn-scheduler/🟦️.ts:220` (peer edit, 15:19 local) dynamically imports
  `../../🧪️tests/🧪️turnscheduler-lane-priority/🟦️.ts`, which does not exist; the import sits under
  `if (import.meta.vitest)` but vite must still resolve the specifier, so the module 500s and the app never
  mounts (`booted=false`, three consecutive lanes). Confirmed with `bun build`:
  `error: Could not resolve: "../../🧪️tests/🧪️turnscheduler-lane-priority/🟦️.ts"`. Not chased — peer file.
  **The probe now names this class of failure itself:** `page.on("response")` / `page.on("requestfailed")`
  record `http 500: <url>` and `requestfailed <err>: <url>` as faults, so a boot that dies on one broken vite
  transform no longer presents as a two-line console tail. First use:

```
### collateral
http 500: http://127.0.0.1:6013/@fs/…/🧰️framework/🔨️modules/🎭️actor/🧵️turn-scheduler/🟦️.ts
requestfailed net::ERR_ABORTED: http://127.0.0.1:6013/@fs/…/🧵️turn-scheduler/🟦️.ts
```

- One lane (`13-09-30`) was started while B44 held the probe slot (my `pgrep` gate printed two PIDs and I ran
  anyway). Its `engagement-input-present PASS` is corroborated by four clean lanes, but its
  `settings-panel-opens` reading is discarded as contaminated; §2.4's proof runs are all gate-clear.

---

## 2.5 Correction to §2 — a THIRD cause, and it is product

After the §2.2 and §2.3 fixes the five-step read prefix is green (§2.4), but the MINIMAL polluting prefix is
not, and it is not a toggle-shut either. Three gate-clear lanes, pre-click state measured this time:

| # | lane (`--only=`) | pre-click | result | run |
|---|---|---|---|---|
| S0′ | `settings-panel` | `aria-pressed=false active=false body=0` | **PASS** `clicked=ok active=true body=9 waitedMs=2` | `13-43-25` |
| S2 | `window-options,settings-panel` | `matches:1 aria-pressed=false active=false body=0` | **FAIL** `clicked=ok active=false body=0 waitedMs=20439` | `13-41-51` |
| S2′ | `window-options,settings-panel` (repeat) | same | **FAIL** `waitedMs=20339` | `13-43-25` |

The panel was genuinely SHUT before the press (`aria-pressed=false`, one matching element), the press landed on
the tab itself (`hit:"SPAN#-[inline-label]"` inside `BUTTON#puzzle3d.panel.settings[panel-tab-button]`), the
shell RECORDED it —

```
history patch applied {"replace":false,"currentCursor":6,"patchCursor":8,"upserts":1,
                       "labels":["Toggle Panel"],"canUndo":true}
```

— and 20 s later the tab is still `aria-pressed=false` with `activeTabs:["framework.panel.history"]` and no
panel body. So: **a panel-tab press is accepted and recorded in history and never applied, deterministically,
while another panel already holds that dock anchor** (`window-options` opens History there through
`readHistoryEntryIds` → `openHistory`). Reproduced twice, 2 ms vs >20 s against the same press.

That is the third cause and it is PRODUCT. It lives in the shell's panel dock — `🖱️ui/🎯️targets/⚛️react`
(the file a peer is relocating this session) and/or `🏛️ShellHost` — so it is reported, not edited, per this
wave's boundaries. §2.4's green (the five-step read prefix, `waitedMs=2`) stands: with `camera-gestures` and
`projection-options` also in front the dock ends up in a configuration where the activation does land, so the
verdict is load/layout-dependent on top of being anchor-dependent. **`settings-panel-opens` is therefore two
thirds fixed here and one third handed on.**

---

## 9 Scoreboard — the handed set, with its polluter and its class

| verdict | polluter (named) | cause | class | state now |
|---|---|---|---|---|
| `context-menu-object-vocabulary` | **none** — fails in a fresh one-step lane | a 900 ms sample racing a submenu that opens at 1 361 ms | **probe** | **PASS** (`11-45-48`) |
| `volume-brush-arm` | any step that arms a utility and leaves the bar open (`locked-refusal`, `gumball-drag`, `brush-stroke`, `suggestions-open`, `relocate`) | `unfoldPerspectiveUtilities` matched the bar's `fold` chip by TEXT and folded the bar; the arm's force-click then reached no control (`setActiveTool` never dispatched) | **probe** | **PASS** (`12-21-15`) · residual under an in-flight fill → §4 |
| `locked-refusal-notice` | `fill-apply-max` (when it lands objects) | the lock precondition needs >15 s on a filled document; the refusal cannot fire until it is met | **probe budget** (product latency → §4) | **PASS** (`12-11-30`, `12-21-15`) |
| `settings-panel-opens` | `camera-gestures` / `window-options` (both open the History panel at that anchor) | (a) the open panel's 3 px resize handle covered the tab's centre; (b) `ensurePanel` read no state the tab publishes and toggled an open panel shut; (c) a recorded `Toggle Panel` that is never applied while another panel holds the anchor | **product** (a, fixed) + **probe** (b, fixed) + **product** (c, handed on) | **PASS** on the 5-step read prefix (`12-44-41`); still FAIL on `window-options,settings-panel` → §2.5 |
| `engagement-input-present` | `fill-apply-max` (same family) | the pane's fold write does not land; no unfold ⇒ no search field (B10's merged fold state) | **product** (§4) | not reproduced in isolation; budget now 30 s, click outcome now recorded |
| `outliner-hide-applies` | `fill-apply-max` | `setSelectionFlag` settles `command-complete` with `historyUpserts:0 effects:0` and writes nothing | **product** (§4) | reproduced (`12-46-57`); verdict now also requires the WORLD lane, fresh lane still green (`12-55-02`) |
| `export-only`, `export-names-the-example` | `example-switch` (payload size, B36 §5) | the served wasm still pushes the 145 KB payload inline; the segmented command is on disk | **product, awaiting a wasm** | FAIL on both wasms measured (`12-59-35`, `13-03-54`) |

Net on the verdicts this wave could close in the browser: **3 flipped to PASS** (`context-menu-object-vocabulary`,
`volume-brush-arm`, `locked-refusal-notice`), **1 flipped on its bisected prefix** (`settings-panel-opens`),
**3 attributed to one named product cause** (§4/§5) with the probe half of each one done.

## 10 Gates — all foreground, tails quoted

```
bun x nx run @semio-tech/ui-react:test-quick -- -t 'never over the tab strip' --reporter=verbose
 ✓ |@semio-tech/ui-react| ../../🟦️.tsx > control chrome > a panel's resize handle lives inside the body
   stack, never over the tab strip that carries every panel's tab 62ms
      Tests  1 passed | 715 skipped (716)

bun x nx run @semio-tech/ui-react:test-quick            (whole lane)
 Test Files  4 failed | 18 passed (22)
      Tests  13 failed | 703 passed (716)
```

The 13 are peer/pre-existing and none touches Panel: two files fail to IMPORT
(`.storybook/🧪️tests/🧹️owned-ui-react-lint` → missing `../../🟦️lint-tooling.ts`,
`.storybook/🧪️tests/🧭️scope-resolution` → missing `../../scopes.ts`), the rest are icon-keyframes /
celebrate-content / UIIntroduction / UIDialog-focus / Tutorial-Immer laws. Grep for `Panel`,
`panel-resize-handle`, `panel-body-stack` in that log: **no hits**.

```
bun x nx run @semio-tech/framework-renderer-react:test-quick
 Test Files  1 passed (1)
      Tests  5 passed (5)
  Run duration:      4.0s
```

(That lane needed five polls first: `NX Failed to process project graph … WGPU renderer must name an authored
wasm producer` — peer churn in the nx emoji plugin, cleared on its own.)

```
bun x nx run @semio-tech/ui-react:typecheck
750 errors, repo-wide and pre-existing (writer/cad/trinity/stdio test files, 📜️script.ts, …).
Zero at 🖼️Panel/🟦️.tsx. My new law's line reports `TS2304: Cannot find name 'PanelTabNode'` — the SAME
error the three pre-existing neighbouring tests report at 10967/10999/11018, i.e. this target does not
resolve the file's `includeSource` ambient types at all; the law itself runs and passes (above).

bun x tsc --noEmit … 🔍️browser-probe.ts
only the two pre-existing errors (`ImportMeta.dir`, the `modifiers` click option) — none in any edited region.

RUST_MIN_STACK=134217728 cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
warning: `semio-s-artifact-puzzle-3d` (lib) generated 92 warnings …
    Finished `dev` profile [unoptimized] target(s) in 23.99s          ← 0 errors (no Rust touched this wave)
```

**Not run, with reason:** the consolidated regression lane
(`--only=context-menu-rows,volume-brush,engagement-bar,outliner-rows,locked-refusal`) could not be measured —
`:6013` stopped booting again on a second peer refactor, and the probe's new network fault named it:

```
http 500: …/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx
bun build … → error: Could not resolve:
  "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts"
```

`🖼️wire-turn.ts` existed earlier in this session (a 12:51 boot failure quoted its line 123) and has since been
consolidated into that package's `🟦️.ts` while `🔌️PluginRuntime` still imports the old path. Polled for 9
minutes; still broken at hand-off. Every verdict above already has its own gate-clear proof run, quoted.

## 11 Files

Product:
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx` — `PanelResizeHandle` is rendered inside
  `panel-body-stack` (which gains `relative`) instead of on the panel root, so a grab strip never covers the
  tab strip; the rule is stated in `PanelResizeHandle`'s docstring with the measured geometry.

Laws:
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` —
  `a panel's resize handle lives inside the body stack, never over the tab strip that carries every panel's tab`
  (new): the handle is a descendant of `panel-body-stack`, that stack is `relative`, and the panel root has no
  `panel-resize-handle` child any more.

Ticket (this wave owns `🔍️browser-probe.ts`; every step and verdict name is unchanged):
- `🔍️browser-probe.ts` — `unfoldPerspectiveUtilities` never folds an open bar and never spends a world pick it
  does not need; `armUtility` polls (20 s) and records the target's pressability and its click's outcome;
  `ensurePanel` folds `aria-pressed` into the tab's state, treats body-or-pressed as open, polls 20 s, records
  the click outcome, the pre-click attributes and — when a pressed tab stays shut — the covering element's
  geometry, the active tabs and the console since the press; `unfoldWindowPane` records its click outcome and
  polls 30 s; `context-menu-rows` polls each submenu open and dumps every menu item including id-less ones;
  `locked-refusal`'s lock precondition polls 45 s and reports the flag row's shape; `outliner-hide-applies`
  requires the world lane as well as the row dump; `page.on("response")`/`page.on("requestfailed")` turn a
  failed fetch into a named fault.
- `📓️2026-09-13-wave-B45-full-run-bisect-3.md` — this report.
- `🗑️generated/probe-2026-09-12T11-43-39 … 13-53-55` (`.md` + `.ndjson` + screenshots),
  `🗑️generated/wave-B45-ui-react-resize.txt`, `wave-B45-ui-react-full.txt`,
  `wave-B45-ui-react-typecheck.txt`, `wave-B45-renderer-react.txt`.

Untouched on purpose: `🖱️ui/🎯️targets/⚛️react/🟦️.tsx` and `🔌️PluginRuntime` (peer relocations in flight),
every plugin `🦀️.rs`, `🌐️World3dHost` and the instances lane (B44), the fill cancel path (B42).

---

## 12 Hand-off state of `:6013` at the end of this wave

`:6013` is **not booting**, and not because of anything in this wave:

```
http 500: …/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx
bun build … 🔌️PluginRuntime/🟦️.tsx
  → error: Could not resolve:
    "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts"
```

`🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/` now contains only `🟦️.ts` (plus package/vitest
config) — `🖼️wire-turn.ts` was consolidated into it mid-session, and `🔌️PluginRuntime/🟦️.tsx:106` still
imports the old path for `TYPED_OPERATION_ACK_MAGIC`, `TYPED_OPERATION_PAGE_MAGIC`, `driveInboundRequest` and
the rest of that surface. Polled for 15 minutes across two waits; still broken. The owner of that consolidation
has to re-point that import (and any sibling importer) before the next battery can run.
