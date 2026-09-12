# Wave B47 — the seven non-selection residuals of full battery #58

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · wasm #58 on `:6013` (host vite-live) · every browser lane run in the
FOREGROUND after checking `pgrep -f 'bun .*browser-pro[b]e'` · written incrementally while the wave ran.

Handed set (FAIL in `🗑️generated/battery-2026-09-13-58-6013.txt`, PASS=73 FAIL=25 FAULTS=0), everything
outside B46's selection-dependent mutate family:

1. `projection-control-flips` / `projection-repaints-camera`
2. `settings-value-reaches-window-rail`
3. `camera-emits-no-artifact-history`
4. `clipboard-copy-writes-a-fragment` / `clipboard-paste-reaches-the-guest` / `clipboard`
5. `import-distinct`
6. `outliner-hide-applies` (+ `volume-brush-arm waitedMs=20363`)
7. `locked-refusal-notice`

Predecessors read first: `📓️2026-09-13-wave-B45-full-run-bisect-3.md` (the bisect method and the four
faces of the post-fill write stall), `📓️2026-09-13-wave-B44-mutation-latency-large-document.md` (the
per-command turn count residual), `📓️2026-09-12-wave-B41-projection-id-probe-scoring.md` (instance-qualified
measure ids and `data-published-value`), `📓️2026-09-12-wave-B36-full-run-bisect-2.md`,
`📓️2026-09-12-wave-B33-full-run-vs-fresh-lane.md`.

The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the ticket
folder is managed on disk. No git command that modifies state was run; the ticket is not closed; nothing
under `🗑️generated` was deleted.

---

## 0 Infrastructure note that changes how every claim below was read

`rg`'s output through this harness MANGLES the matched substring: the source line
`console.warn("[DEBUG] importFixture ingress", …)` came back as `console.warn("n-picker …")`, and the guest's
`eprintln!("[DEBUG] puzzle3d.import.ingress …")` came back as `"[DEBUG] ningress …"` — with
`--color=never` as well. Two candidate "literal corruption" findings this wave evaporated when the same
lines were read with `python3` instead (`cat` agrees with python). **Every source quotation below is a
python read, never an `rg` match line.** Worth carrying: a wave that greps for a debug-tap name and sees it
mangled can conclude a codemod ate it.

---

## 1 `projection-control-flips` / `projection-repaints-camera` — one probe defect on top of one product obstruction

### 1.1 What battery #58 actually recorded

```
[69.6s]  nudge …projection-orthographic-view select options=[] current= picking=-1
[100.0s] projection nudge … {"before":{…,"pressed":"closed","value":"","published":"","text":""},
                             "after":{…,"pressed":"closed","value":"","published":"","text":""},"waitedMs":30362}
[130.4s] verdict projection-control-flips   FAIL publishedBefore="" publishedAfter="" waitedMs=30371 draftText=""
[160.6s] verdict projection-repaints-camera FAIL waitedMs=30236   (pose bit-identical)
```

`options=[]` is the whole story: the listbox was never in the document when the step counted it, so the step
pressed **Escape** and then spent 60 s settling two verdicts on a control nobody had asked to change.
`draftText=""` (B41's PASS had `"Plan"` after 35 ms) confirms not even the optimistic draft moved.

### 1.2 Probe half — a fixed 500 ms sample on a portalled listbox

`🔍️browser-probe.ts` `nudgeMeasure`, combobox branch (pre-B47): `click({force:true})` → `waitForTimeout(500)`
→ `options.count()` **once**. Now: the popover is POLLED (8 s), the trigger is pressed a second time if it is
still shut, and the trigger's own `data-state` plus the options are logged. Measured, fresh lane
(`🗑️generated/wave-B47-lane-b.txt`):

```
[17.8s] nudge …projection-orthographic-view select opened=true waitedMs=2 triggerState=open
        options=["Plan","Top","Bottom","Front","Back","Left","Right"] current= picking=0
[18.3s] projection nudge … {"after":{…,"text":"Plan"},"waitedMs":333}
[18.8s] verdict projection-control-flips   PASS
[18.8s] verdict projection-repaints-camera PASS
```

### 1.3 Product half — the Inspection panel covers the trigger's centre

The same two verdicts still fail **from the full-run position**, and the new hit-test dump says why
(`wave-B47-lane-i.txt`, lane `--only=camera-gestures,projection-options`):

```
projection chrome geometry={"rail":[1130,58,300,544],"railStack":"DIV#-:relative/auto",
 "panels":["panel-body-stack=[1137,26,300,198]@DIV#framework.panelTab.framework.panel.inspection:absolute/30",
           "puzzle3d-play-inspector=[1138,27,298,196]@DIV#framework.panelTab.framework.panel.inspection:absolute/30",
           "panel-body-stack=[1137,840,300,34]@DIV#framework.panelTab.framework.panel.history:absolute/30", …]}

nudge …projection-orthographic-view select shape={"rect":[1130,107,104,16],"disabled":false,
 "expanded":"false","pointerEvents":"auto","mine":false,"owner":"puzzle3d-play-inspector",
 "hitRect":[1177,11,99,196],"hitZ":"static/auto",
 "chain":["SPAN#-[tree-label]","DIV#-[-]","DIV#-[-]","DIV#-[tree-row-content]","DIV#-[tree-row-layout]",
          "DIV#puzzle3d-play-inspector[tree-property-item]","DIV#-[tree-section-wrapper]","DIV#-[tree]",
          "DIV#-[scroll-area-viewport]"]}
```

- The perspective window's **measures rail** is `x 1130‑1430, y 58‑602`, and the stacking context it paints
  in is `position: relative; z-index: **auto**`.
- The **Inspection panel body** is `x 1137‑1437, y 26‑224`, inside
  `DIV#framework.panelTab.framework.panel.inspection` at `position: absolute; z-index: **30**`.

They are the same 300 px right-hand column. The panel wins by z-index, so everything in the rail's top
~170 px band — which is exactly the Projection group, `orthographic-view` at `y 107‑123` — takes its presses
on an Inspection tree row. `force: true` cannot help: it skips actionability, never hit-testing. **A human
cannot use the Projection select while Inspection is open.** Same family as B45 §2.2 ("a grab strip never
covers a button"), one layer out: a floating panel body covering window chrome.

The polluting predecessor is any step that opens a right-hand panel — `camera-gestures` does it through
`readHistoryEntryIds` → `ensurePanel framework.panel.history`, which makes the dock render the Inspection
ghost body as well (B45 §2.1's `activeTabs:["framework.panel.inspection","framework.panel.history"]`).

**Owner / handover.** The rail is `Pane anchor="top-right"` portalled through `overlaySlot="window-measures-overlay"`
in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx:321-360`; the ghost panel column is `🖼️Panel`'s
`framework.panelTab.*` at `z-30`. Fixing it is a layering decision, not a one-liner — raising the rail above
`z-30` only moves the obstruction onto the panel rows underneath it, so the dock column has to be excluded
from the window's own overlay region. Left to the shell-layout owner **with the geometry above**; it is not
puzzle3d's and not the probe's.

**Probe half of the handover is done:** the red now names the obstruction in 0 ms instead of burning 60 s.

```
verdict projection-control-flips FAIL … OBSTRUCTED puzzle3d-play-inspector covers the trigger centre —
  trigger rect=[1130,107,104,16] covering rect=[1177,11,99,196] chain=["SPAN#-[tree-label]",…]
```

Verdicts: **PASS** in any lane where the dock is not over the rail (`wave-B47-lane-b.txt`); **FAIL, obstruction
named**, from the full-run position (`wave-B47-lane-i.txt`, `-j`). Class: **probe** (sample) **+ product**
(layering).

---

## 2 `settings-value-reaches-window-rail` — a framework defect: a scalar gesture payload evicted the action's arguments

### 2.1 Two probe defects had to be cleared before the product one was visible

| # | what the verdict compared | why it could not decide |
|---|---|---|
| a | the rail read ONCE, 1.3 s after the bump (`await page.waitForTimeout(2500)`) | a settings → guest → window-config → rail round trip is three hops; no `waitedMs` in the red at all |
| b | the **perspective** window's rail, always | the panel writes to the window it NAMES, and only the perspective's rail was in the document |

Both fixed in `🔍️browser-probe.ts`: the rail read is a 30 s `settleFor` scored on `data-published-value`, the
census covers EVERY `…/puzzle3d-play-grid-spacing` element, the panel's own owner is read and logged, and the
owning window's measures rail is unfolded first. That turned one ambiguous red into a measurement:

```
[9.4s]  settings owner={"title":"Settings — puzzle3d-main-top","activeWindow":null,
                        "modeTabs":["mode-dock-tab-0-puzzle3d-main-top=true","mode-dock-tab-1-puzzle3d-main-perspective=true"]}
[16.8s] settings owner rail window=puzzle3d-main-top stillFolded=0
[47.0s] settings rails=[{"id":"puzzle3d-main-top/puzzle3d-play-grid-spacing","published":"10","value":"10"},
                       {"id":"puzzle3d-main-perspective/puzzle3d-play-grid-spacing","published":"10","value":"10"}]
        owner=puzzle3d-main-top target=10.5 settled=false waitedMs=30174
```

**Both** windows' rails stayed at 10 for 30 s on an IDLE one-object document while the stepper rendered 10.5.
So it was never latency and never the wrong pane: the write did not land. (`settings-grid-spacing-bumps` had
been going green on that 10.5 — a `NumberStepper` publishes no `data-published-value` at all, so its box is
the host's optimistic draft and proves nothing. It now also reports the dispatch count.)

### 2.2 Root cause — `🧰️framework/…/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:1963-1968` (pre-fix)

The dispatch reached the guest and settled empty (`wave-B47-lane-e.txt`):

```
[DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"setGridSpacing"}
[DEBUG] command ingress lane {"actionId":"setGridSpacing","seq":24,"lane":"Interactive"}
[DEBUG] puzzle3d.utility.publish action=setGridSpacing window=Some("puzzle3d-main-top") utility= map_hit=false
[DEBUG] performInvocation settled {"actionId":"setGridSpacing","frames":2,"historyUpserts":0,"effects":0}
```

`uiIntentPayload` was:

```ts
if (intent.input === null) return intent.args ?? undefined;
if (intent.args === null) return intent.input;
if (typeof intent.args === "object" && … && typeof intent.input === "object" && …) return { ...intent.args, ...intent.input };
return intent.input;            // ← a SCALAR payload replaced the authored args wholesale
```

A `NumberStepper`'s `onChange` reports the **number** (`NumberStepperView`,
`🗣️Interpreter/🟦️.tsx:1178` → `emitIntent(…, toUiValue(value))`), so `intent.input === 10.5`, a scalar. None of
the first three branches matched, the last one returned the bare `10.5` — and the authored
`{ windowId: "puzzle3d-main-top" }` went with it. Every guest command reads NAMED arguments
(`set_spacing` → `puzzle3d_absolute_or_delta` → `args.get("value")` / `args.get("delta")`,
`engagement_input` → `args.get("value")`, `engagement_control_select` → `args.get("id") | args.get("value")`),
so a bare scalar is read as nothing at all: command complete, zero mutations, zero effects, silence. **The
whole app Settings panel — all four steppers — was mute, for every app that authors a scalar control through
a `BuiltNode`.** The map-payload path had a law; the scalar path had none.

### 2.3 Fix — product, one function

A scalar payload is now NAMED by its trigger (`delta` → `delta`, otherwise `value`) and MERGED over the
authored args. Map payloads merge exactly as before.

### 2.4 Law

`🧰️framework/…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`, beside the existing map-payload law:

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/…/vitest.config.ts --reporter=verbose \
    -t "names a scalar intent payload after its trigger"
 ✓ …/🔬️engine-contract/🟦️.ts > s workflow flow routing > names a scalar intent payload after its trigger and keeps the authored arguments 1ms
$ … -t "bridges semantic intent scope"
 ✓ …  bridges semantic intent scope, version, args, and input into the plugin action channel 1ms
      Tests  1 passed | 1000 skipped (1001)
```

(The one red in that project, `🧪️tests/🧩️package-integration/🟦️.ts` — `Cannot find module
'../../🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script'` — is a peer's in-progress move, present before this
wave's edits and unrelated to them.)

### 2.5 Verdicts, browser-measured after the fix

`touch` on the edited file and a `curl` of its `@fs` URL first (B45 §8's recipe): `grep -c uiInputField` → `2`.

```
lane --only=settings-panel,window-options                   (wave-B47-lane-f.txt)
[32.9s] settings rails=[{"id":"puzzle3d-main-perspective/puzzle3d-play-grid-spacing","published":"13","value":"13"}]
        owner=puzzle3d-main-perspective target=13 settled=true waitedMs=1
[32.9s] verdict settings-value-reaches-window-rail PASS

lane --only=window-content,camera-gestures,projection-options,window-options,settings-panel   (wave-B47-lane-g.txt)
[173.3s] verdict settings-value-reaches-window-rail PASS   ← the full-run position
```

**`waitedMs=1`**: the value was already on the rail by the time the poll took its first sample. Class:
**product** (framework), with two probe defects cleared on the way.

---

## 3 `camera-emits-no-artifact-history` — the measurement was writing the rows it counted

### 3.1 The question the brief asked, answered

Battery #58: `newEntries=["framework.history.entry.3","framework.history.entry.4","framework.history.entry.6"]
before=2 after=5`. The verdict read entry **ids** only, so nothing in the red said which verbs those were.
With labels read alongside the ids, the answer is: **not one camera row.**

```
lane --only=camera-gestures,projection-options,settings-panel        (wave-B47-lane-a.txt)
[9.9s]  history quiesce camera-before rows=4 settled=true waitedMs=1386
[20.4s] history quiesce camera-after  rows=5 settled=true waitedMs=610
[20.4s] verdict camera-emits-no-artifact-history PASS

lane --only=window-content,camera-gestures,projection-options,window-options,settings-panel   (wave-B47-lane-g.txt)
[24.3s] camera history rows new=["framework.history.entry.6=Activate Window↶"]
        chrome=["framework.history.entry.6=Activate Window↶"] artifact=[]
[24.3s] verdict camera-emits-no-artifact-history PASS
```

The single new row is `Activate Window` — a shell `WindowConfig` command the PROBE issues. B10's and B36's
claim stands unchanged: `setCamera` emits no artifact mutation.

### 3.2 Two probe defects, both of them the reader moving what it measures

1. **The `before` read was taken before the rows quiesced.** `readHistoryEntryIds` opens the History panel
   through `ensurePanel`, and that open is itself a `Toggle Panel` command whose history row lands one round
   trip AFTER the panel body renders. B38 had already fixed "the read closes the panel"; this is the next
   layer — the read's own row arriving after the read. `settledHistoryEntryRows` now polls until two
   consecutive reads agree and logs the wait (`waitedMs=1386` / `610` above).
2. **`dismissChrome` pressed "Collapse".** It is called once per drag inside `camera-gestures`, and that
   button folds a window pane — a `WindowConfig` write with its own history row. Three drags, three rows, and
   a red that named the camera. Inside the one step whose verdict counts history rows the dismissal is now
   Escape-only.

And the predicate is **attributed, not counted**: a new row is evidence against the camera only once its
LABEL is not shell chrome (`CHROME_HISTORY_ROW`: `Toggle Panel`, `Switch Panel Tab`, `Activate Window`,
`Resize Window`, `Set Active Tool`/`Utility`, `Collapse`/`Expand`, `Toggle Pane`). Both sets are printed on
every run — PASS included — so the attribution is auditable rather than hidden inside a boolean.

### 3.3 The same defect, one verdict over

`window-options-emit-no-history` reads history the same way and went **FAIL** in a two-step lane
(`newEntries=["framework.history.entry.5","framework.history.entry.6"] before=3 after=5`) purely because the
History panel opened fresh there, while battery #58 happened to find it already open. Given the same quiesce
and the same attribution it is green from the full-run position too:

```
wave-B47-lane-g.txt
[153.0s] history quiesce window-options-before rows=5 settled=true waitedMs=867
[162.9s] window-options history rows new=[] chrome=[] artifact=[]
[162.9s] verdict window-options-emit-no-history PASS
```

Class: **probe**. No product change. Verdicts: `camera-emits-no-artifact-history` **PASS** fresh and in the
full-run position; `window-options-emit-no-history` **PASS** (and no longer position-dependent).

---

## 4 The clipboard family — the route is intact; the guest's own selection is what is missing

### 4.1 Fresh lane, gate-clear, five steps deep (`wave-B47-lane-q.txt`)

```
$ bun 🔍️browser-probe.ts --only=pick-object,context-menu,locked-refusal,outliner-rows,clipboard-copy-paste --port=6013
[29.4s] verdict clipboard-precondition PASS
[32.2s] clipboard copy hops={"copyTurns":2,"copyEffects":["1","1"],"pasteTurns":0,…} waitedMs=502
[32.2s] verdict clipboard-copy-writes-a-fragment PASS
[32.9s] clipboard paste hops={"pasteTurns":1,"pasteEffects":["0"],…} waitedMs=0
[32.9s] verdict clipboard-paste-reaches-the-guest PASS
[34.2s] verdict clipboard PASS
```

`copyEffects=["1","1"]` — `mod+c` reaches the guest and comes back with exactly the one
`Effect::ClipboardWrite` the host retains, twice (Meta and Control), in **502 ms**. So nothing in the
hotkey→shell→guest→effect route is broken, and the paste and census verdicts follow. B38's fresh-lane PASS
reproduces.

### 4.2 What #58's `copyEffects=["0","0"]` therefore means

`Puzzle3dClipboardJob::emit`'s `copy` arm
(`…/✏️editor/🦀️.rs:7684`) answers `Emit::default()` — zero effects — exactly when
`puzzle3d_copy_fragment_from` returns `ClipboardError::EmptySelection`, i.e. when
`puzzle3d_selected_objects_from(&marks, &fixture)` resolves NOTHING out of the interaction snapshot the job
was handed. So in #58 the guest saw an empty selection, on a document whose **two world surfaces both
published one** — the step's own precondition log proves the host side:

```
#58 [446.5s] clipboard precondition attempt=0 via=pane-fraction
             state=["puzzle3d-main-top=puzzle3d.brush.f34f86bcf2562d95",
                    "puzzle3d-main-perspective=puzzle3d.brush.f34f86bcf2562d95"] waitedMs=4
     [451.6s] clipboard inspection-wait populated=true id=Id seed-left-001 lock=true
```

Note the disagreement in the same run: the world surfaces name `…f34f86bcf2562d95`, the Inspection panel is
still rendering `seed-left-001`. Two published views of one selection, out of step — the same staleness §7
measures on the flag row. `copyEffects` is the third view, and it is the guest's own.

**Class: product, and it is B46's selection lane, not the clipboard's.** Zero effects on a settled `copy` is
a REPORT that the guest's interaction snapshot was empty; the clipboard arm is behaving exactly as
specified. Nothing to fix here, and the `clipboard*` verdicts will follow the selection lane.

### 4.3 What this wave added so the next battery does not need this reasoning

A kept tap in the same family as `puzzle3d.import.*`, in the `copy` arm itself:

```
[DEBUG] puzzle3d.clipboard.copy marks=<snapshot.selected.len> resolved=<objects> objects=<fixture.objects>
```

`marks=0` says the guest was handed nothing; `marks>0 resolved=0` says the ids it was handed name no object in
the fixture it was handed. Those are different defects and `effects:0` cannot tell them apart. **Rides wasm
#59** (`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` → 0 errors, §8).

---

## 5 `import-distinct` — a hard-coded fossil fixture, and a full-screen tutorial veil that ate every click

### 5.1 The veil, which is the wave's widest finding

While chasing why the example picker would not open, the hit-test dump named the element under it
(`wave-B47-lane-n.txt`):

```
example picker shape={"rect":[629,3,192,22],"slot":"select-trigger","state":"closed","expanded":"false",
 "disabled":false,"mine":false,
 "chain":["DIV#-[-]{ui-veil z-tutorial inset-0 absolute pointer-events-auto}@0,0,1440,900:absolute/10000/pe=auto",
          "DIV#-[-]{pointer-events-none absolute inset-0}@0,0,1440,900:absolute/auto/pe=none",
          "DIV#-[-]{semio-scope}@0,0,1440,900:relative/auto/pe=auto","DIV#root[-]",…]}
example options=[] pickerPresent=1 opened=false waitedMs=8062
```

A **1440×900 `div.ui-veil.z-tutorial` at `z-index: 10000` with `pointer-events: auto`** — the introduction
overlay's scrim (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:5815`, `veilBlocksPointer ?
"pointer-events-auto" : "pointer-events-none"`) — covering the entire application, **16 s after
`waitForBoot` had logged `dismissed welcome tour`**. While it is up nothing in the app is clickable:
`elementFromPoint` returns the veil for every control, and `force: true` does not help (force skips
actionability, never hit-testing). Every press is swallowed in silence.

`waitForBoot` pressed Skip exactly ONCE, so an introduction that re-arms afterwards left the rest of the run
clicking into a scrim. This is the likeliest common cause of the battery's scattered, unexplained click
timeouts — `activate-perspective: FAILED TimeoutError: click: Timeout 5000ms exceeded` at `[15.1s]` of #58
among them.

**Fix — probe.** `clearIntroductionVeil` polls for a wide `.ui-veil` with `pointer-events: auto`, presses
Escape and then Skip up to four times, and LOGS a veil that will not clear (rect, z, the candidate controls
it can see). Every step's `dismissChrome` calls it, and `example-switch` now calls `dismissChrome` first.
Whether the re-arming itself is a product defect is left named, not chased: the probe's job is to not measure
through a scrim.

Immediately measured (`wave-B47-lane-o.txt`):

```
[10.3s] veil dismissChrome cleared=true state=null
[10.4s] example options=["…No example","…Concrete Forest","…Nakagin Capsule Tower"] pickerPresent=1 opened=true waitedMs=2
[33.5s] example after switch: Nakagin Capsule Tower
```

Before this, **`example-switch` had been silently not switching**: `options=[] pickerPresent=1`, no
`setActiveExample` dispatch, the document left on the one-object Concrete Forest — and
`example-switch-instances` green the whole time, because it only asserts `count > 0`. Every later step in
such a lane measures the SMALL document while the report says Nakagin. (The step's own click is now
`force: true` + an 8 s poll, B44 §1.0's two reasons.)

### 5.2 The import defect itself — `🔍️browser-probe.ts` fed a fossil

With the example actually on Nakagin, the guest taps came back and named the real chain
(`wave-B47-lane-o.txt`):

```
[84.9s]  verdict export-only FAIL download=none          ← B45 §5's residual: the segmented export arm is not in the served wasm
[138.3s] distinct ingress
  hostIngress=[… "importFixture ingress" … "payloadLen":9408 …]
  guestTaps=["[DEBUG] puzzle3d.import.ingress args=true payload_len=7542",
             "[DEBUG] puzzle3d.import.parsed objects=1 before=180",
             "[DEBUG] puzzle3d.import.apply ops=215 after_objects=1",
             "[DEBUG] puzzle3d.import.ingress args=true payload_len=9428", …]
[140.8s] verdict import-distinct FAIL before=180 after=180
```

`objects=1 before=180` → `ops=215 after_objects=1`. The step **replaced the 180-object Nakagin document with
a one-object Concrete Forest**, because of this, in the step body:

```ts
const fallback = join(OUT, "probe-2026-09-10T13-52-33-export.json");   // ← a 2026-09-10 fossil, 7 542 bytes
const feed = existsSync(dest) ? dest : fallback;
```

When `export-only` fails — which on Nakagin it does, every time, for reasons B36 §5/B45 §5 already own — there
is no export of THIS document, and the step imported a three-day-old one-object file instead. Consequences,
all of them false readings:

- `import-same-file-idempotent` **PASS** on a document that had just been wholesale replaced (its two
  censuses were both the pre-import 180, read before the world lane republished).
- `import-distinct` built its "distinct" payload from that fossil (2 objects) and compared a stale 180-instance
  census with itself: `before=180 after=180`. #58's `guestTaps=[]` is the same step one layer earlier.

**Fix — probe.** No fallback file, ever. An import step with no export of its own document is **not
reachable** and says so, naming the missing file:

```
wave-B47-lane-p.txt
[89.2s] verdict export-only FAIL download=none dest=…/probe-2026-09-12T16-10-07-export.json
[91.8s] import feed=none not reachable — export-only produced no file for THIS document (dest=…);
        importing another run's export would replace the document under test
[91.8s] verdict import-same-file-idempotent FAIL not reachable — …
[91.8s] verdict import-distinct FAIL not reachable — …
[91.8s] verdict import-distinct-records-history FAIL not reachable — …
```

`import-same-file-idempotent` additionally settles on the guest's own `puzzle3d.import.apply` tap instead of a
fixed 2 s, so its two censuses can no longer straddle the import.

### 5.3 Where the import verdicts stand

On a payload the export can produce they are **PASS**, with the whole chain visible
(`wave-B47-lane-l.txt`, Concrete Forest, 9 408 bytes):

```
guestTaps=["puzzle3d.import.ingress args=true payload_len=9428",
           "puzzle3d.import.parsed objects=2 before=1",
           "puzzle3d.import.apply ops=1 after_objects=2"]
distinct instances before={"count":1,…} after={"count":2,"ids":["seed-left-001","probe-distinct-…"]}
verdict import-distinct PASS   ·   verdict import-distinct-records-history PASS
```

On Nakagin they are **honestly unreachable until `export-only` works**, which needs the wasm carrying the
segmented export arm (B38/B43 on disk, B45 §5 measured absent). Class: **probe** (the fossil fallback and the
veil), blocked behind a **product** residual that is already owned.

---

## 6 `outliner-hide-applies` — NOT latency. The discriminator is what else is selected.

### 6.1 The premise correction

B45 §4 attributed this to the post-fill write stall and handed it to B44. Re-measured on #58 (which carries
B44's 32× `fixture_geometry_fingerprint` fix), with no fill anywhere in the plan:

| # | lane (`--only=`) | verdict | `waitedMs` | selection at the click | run |
|---|---|---|---|---|---|
| O0 | `outliner-rows` | **PASS** `worldHiddenAfter=["seed-left-001"]` | **2 345** | `selectionBefore=[]` | `wave-B47-lane-r.txt` |
| O1 | `pick-object,context-menu,locked-refusal,outliner-rows,clipboard-copy-paste` | **FAIL** `worldHidden=[]` | **30 237** | `object-1` (a SIBLING, locked) | `wave-B47-lane-q.txt` |

Three objects, no fill, an idle app — and the same row's Hide, with the same authored args, lands in 2.3 s
with nothing selected and never lands in 30 s with a sibling selected. **The fill is not necessary and
latency is not the mechanism.** (`volume-brush-arm waitedMs=20363` in #58 is B45 §3.4's separate, genuinely
post-fill window-config starvation and is untouched here.)

### 6.2 What the round trip looks like in the failing lane

```
[117.8s] outliner hide clicked={"tag":"button","slot":"action","text":"Hide","row":"panel:puzzle3d-play-document/seed-left-001"}
         rowObject=seed-left-001 waitedMs=30237
         worldHiddenBefore=[…"hidden":[]…]  worldHiddenAfter=[…"hidden":[]…]
[117.8s] outliner hide console tail=[
  "[DEBUG] performInvocation {\"actionId\":\"setSelectionFlag\"}",
  "[DEBUG] command ingress lane {\"actionId\":\"setSelectionFlag\",\"seq\":115,\"lane\":\"Interactive\"}",
  "[DEBUG] puzzle3d.utility.publish action=setSelectionFlag window=Some(\"puzzle3d-main-perspective\") …",
  "[DEBUG] command ingress settled status=command-complete observed=command-complete",
  "[DEBUG] performInvocation settled {\"actionId\":\"setSelectionFlag\",\"frames\":2,\"historyUpserts\":0,\"historyCanUndo\":null,\"effects\":0}"]
[117.8s] verdict outliner-hide-applies FAIL … beforeHead == afterHead   (byte-identical)
```

`historyUpserts: 0` **and** `effects: 0`. Not a refusal either: `Puzzle3dActionCtx::refuse_when_locked` and
`refuse_without_selection` both push `Effect::Notify`, which would read `effects: 1`. So the command ran and
wrote nothing — the shape B45 §4.2 named, here with no fill in sight.

### 6.3 The guest is not the defect — proven natively

The row authors explicit arguments
(`…/✏️editor/📌️panels/🗿️artifact/🦀️.rs:131-150`: `flag_args` → `{entity, flag, ids:[id], value: !hidden}` on
`Trigger::Activate`), and `set_selection_flag`
(`…/🎮️commands/🔖️set-selection-flag/🦀️.rs`) takes the explicit branch whenever `entity` AND `ids` decode.
A new law pins that this branch ignores the live selection and the entity's own lock — and it passes:

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib \
    -- --test-threads=1 --nocapture an_explicit_outliner_flag_write
test editor::puzzle3d::component::unit_tests::an_explicit_outliner_flag_write_ignores_whatever_is_selected ...
[DEBUG] puzzle3d.flag.explicit entity=object flag=locked value=true ids=["seed-left-001"]
[DEBUG] puzzle3d.flag.explicit entity=object flag=hidden value=true ids=["seed-left-001"]
[DEBUG] explicit flag write hidden=Some(true) locked=Some(true) selection=vortex
ok
test result: ok. 1 passed; 0 failed; 0 ignored; 745 filtered out; finished in 0.15s
```

(The law selects a VORTEX, locks the object through its row, then hides the object through its row: both
explicit writes land. `object_flag` reads the projection, so this is the document, not the render.)

So the guest, given the args, does the right thing whatever is selected. **Therefore in the failing browser
lane the explicit args did not arrive** — the command fell through to the selection branch, where all three
id lists were empty and `apply_puzzle3d_selection_flag` returns at `ids.is_empty()` three times: no mutation,
no notice, `command-complete`. That is the only reading consistent with all four observations (args authored,
guest correct, zero effects, zero upserts).

Two candidates remain for WHERE they are lost, and this wave does not choose between them by guesswork:
the host's row-action dispatch dropping a `RowAction` binding's args, or the guest's process-global
`UiValue` argument arena refusing the row's `{entity, ids}` map under load and publishing the row action
without them — the arena is one page for every panel of every plugin at once, and
`with_hide_lock_actions`' own docstring (`🗿️artifact/🦀️.rs:151-160`, B44 §6.2) records it running out on the
flagship document.

### 6.4 What was added so #59 decides it

A kept tap, `🎮️commands/🔖️set-selection-flag/🦀️.rs`:

```
[DEBUG] puzzle3d.flag.explicit  entity=<e> flag=<f> value=<v> ids=[…]
[DEBUG] puzzle3d.flag.selection entity=<Option> flag=<f> value=<v> objects=<n> vortices=<n> volumes=<n>
```

One line per dispatch, naming the branch. `flag.selection entity=None … objects=0` ⇒ the args were lost on
the way (host or arena); `flag.explicit … ids=["seed-left-001"]` ⇒ the write happened and the gap is the
republication. **Rides wasm #59.** Class: **product**, mechanism localised to the argument hop, and the next
battery names it without another bisect.

---

## 7 `locked-refusal-notice` — the flag row is not broken; it is the same stale publication

Fresh lane, five steps, gate-clear (`wave-B47-lane-q.txt`):

```
[48.8s] verdict locked-flag-row PASS
[53.4s] lock flag row="locked true" locked=true waitedMs=4451
[77.6s] verdict locked-refusal-notice PASS
```

The Inspection `locked` row goes **true in 4 451 ms** and the guest's refusal notice follows, so the chain
row write → `setSelectionFlag` → Inspection re-render is intact end to end. #58's
`locked=false flag="locked false" waitedMs=30400` is therefore not a broken route but the same publication
staleness B45 §4.3 measured at 45 222 ms on a filled document — and §4.2 of this report shows the same
disagreement in #58's own clipboard step, where the world surfaces name `puzzle3d.brush.f34f86bcf28…` while
Inspection still renders `seed-left-001`.

The Inspection panel's memo key is not the cause: `Puzzle3dDocumentTreeKey::of` hashes
`main::fixture_geometry_fingerprint`, and `hash_object`
(`…/🪟️windows/🧊️main/🦀️.rs:548-569`) hashes `hidden` and `locked` per object, so a flag flip does move the
key. B44 closed that.

What remains is the publication arriving late on a large document — B44's named residual, the per-command
turn count at the host's one-item publication grant
(`store::ArtifactStoreOneItemGrant { maximum_items: 1, … }` in `publish_mounted_typed_operation_unit`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`). Not this wave's file and a Codex peer is live in it.

Class: **product, already owned (B44 §6)**. Verdict: **PASS** fresh (`waitedMs=4451`); on the filled document
it remains a latency red, and the `puzzle3d.flag.*` tap of §6.4 will distinguish "the write never happened"
from "the panel had not caught up" the first time it rides a battery.

---

## 8 Verification — every command run in the FOREGROUND, tails quoted

### 8.1 Smoke

```
$ bun 🔍️browser-probe.ts --only=boot --port=6013
[9.4s] battery PASS=3 FAIL=0 FAULTS=0 first-hard-fault-at=none guest-death-faults=0
[9.4s] done booted=true faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=6
```

### 8.2 Browser lanes (all under `🗑️generated/`, all gate-checked)

| lane | `--only=` | outcome |
|---|---|---|
| `wave-B47-lane-a.txt` | `camera-gestures,projection-options,settings-panel` | `PASS=15 FAIL=3` — item 3 flipped to PASS; items 1/2 measured honestly for the first time |
| `wave-B47-lane-b.txt` | `projection-options,settings-panel` | `PASS=10 FAIL=1` — projection both **PASS** with the polled listbox |
| `wave-B47-lane-c/-d/-e.txt` | `activate-perspective,settings-panel` / `settings-panel` | the settings owner window, both rails, and the `setGridSpacing` console hop |
| `wave-B47-lane-f.txt` | `settings-panel,window-options` | `PASS=17 FAIL=1` → after the fix, `settings-value-reaches-window-rail` **PASS** `waitedMs=1` |
| `wave-B47-lane-g.txt` | `window-content,camera-gestures,projection-options,window-options,settings-panel` | `PASS=30 FAIL=2` — the full-run position: items 2 and 3 **PASS**, item 1 the named obstruction |
| `wave-B47-lane-h/-i/-j.txt` | `camera-gestures,projection-options` | the obstruction, its DOM chain, and the two boxes with their stacking contexts |
| `wave-B47-lane-k/-l.txt` | `example-switch,export-import` | the import chain green on a 9 KB payload; the silent example-switch miss |
| `wave-B47-lane-m/-n.txt` | `example-switch` | the `ui-veil` |
| `wave-B47-lane-o.txt` | `example-switch,export-import` | Nakagin reached; the fossil import caught in the act (`objects=1 before=180`) |
| `wave-B47-lane-p.txt` | `example-switch,export-import` | the four import verdicts as **not reachable** instead of fiction |
| `wave-B47-lane-q.txt` | `pick-object,context-menu,locked-refusal,outliner-rows,clipboard-copy-paste` | `PASS=12 FAIL=2` — clipboard ×3 and locked-refusal **PASS**; item 6's discriminator |
| `wave-B47-lane-r.txt` | `outliner-rows` | `PASS=8 FAIL=0` — `outliner-hide-applies` **PASS** `waitedMs=2345` (no regression from the host fix) |

### 8.3 Host TypeScript

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/📦️packages/🟦️typescript/vitest.config.ts
 Test Files  1 failed | 31 passed (32)
      Tests  6 failed | 1025 passed (1031)
```

All six reds are in `🧪️tests/🧩️package-integration/🟦️.ts` (`ReferenceError: Bun is not defined` in the wgpu
generated-worker group, plus the unresolved `🧊️wgpu/…/📜️script` import) — a peer's in-progress wgpu package
move, present before this wave's edits and in no file it touched. **No new failure.** Full tail in
`🗑️generated/wave-B47-renderer-react.txt`.

`bun x tsc --noEmit -p …/⚛️react/📦️packages/🟦️typescript/tsconfig.json`: 1 016 diagnostics, none from
`🛠️ShellHelpers/🟦️.tsx` and none from the new law (lines 7375-7397). The nearest engine-contract error,
`🟦️.ts(7331,23) Cannot find name 'ActionDescriptor'`, is a pre-existing law two describes above it; the bulk
belong to `🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`, a peer's file this project pulls in.

### 8.4 Guest (Rust) — what rides wasm #59

```
$ RUST_MIN_STACK=134217728 cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
warning: `semio-s-artifact-puzzle-3d` (lib) generated 92 warnings (run `cargo fix …`)
    Finished `dev` profile [unoptimized] target(s) in 52.85s
```

**0 errors**, 92 pre-existing warnings.

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib \
    -- --test-threads=1 --nocapture an_explicit_outliner_flag_write
[DEBUG] puzzle3d.flag.explicit entity=object flag=locked value=true ids=["seed-left-001"]
[DEBUG] puzzle3d.flag.explicit entity=object flag=hidden value=true ids=["seed-left-001"]
[DEBUG] explicit flag write hidden=Some(true) locked=Some(true) selection=vortex
ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 745 filtered out; finished in 0.15s
```

⚠️ A follow-up run of the wider `outliner` filter could NOT be completed: a peer's in-progress refactor of
`semio-framework-os-kernel` broke the workspace mid-wave (`error[E0004]` non-exhaustive
`operation::Phase` match, then 11 errors including unsatisfied `Send`/`OpBinary`/`OpText` bounds), and
puzzle3d depends on it. Two retries, both blocked; the file is not this wave's. The new law's own green run
above is from before that breakage, on the tree that also passed `cargo check` cleanly. **Re-run
`-- --test-threads=1 outliner` once the kernel compiles again.**

Riding #59: the two kept taps (`puzzle3d.flag.explicit` / `puzzle3d.flag.selection` in
`🎮️commands/🔖️set-selection-flag/🦀️.rs`, and `puzzle3d.clipboard.copy` in the `copy` arm of
`Puzzle3dClipboardJob::emit`) plus the new law. No behavioural guest change — both taps are observables in
the kept `puzzle3d.import.*` family, deliberately NOT removed, because §4 and §6 both end in a question only
the guest's own view of its selection can answer. Say the word and they go.

---

## 9 Handover — what this wave named and does not own

1. **A floating panel body covers window chrome** (§1.3). Inspection at `absolute/z-30`, `x 1137‑1437`,
   `y 26‑224`; the perspective window's measures rail at `relative/z-auto`, `x 1130‑1430`, `y 58‑602`. The
   overlap swallows every press in the rail's top band, the Projection selects included. Shell layout, not
   z-order: raising the rail only moves the obstruction onto the panel rows beneath it.
2. **The introduction veil re-arms after it is dismissed** (§5.1). `div.ui-veil.z-tutorial.inset-0` at
   `z-index: 10000`, `pointer-events: auto`, 1440×900, observed 16 s after the welcome tour was skipped.
   While it is up the application is inert. The probe now clears it on every `dismissChrome`; whether the
   overlay should re-arm at all belongs to `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:5815`'s
   owner. Worth checking against the battery's other unexplained click timeouts.
3. **The outliner row's explicit `{entity, ids}` never reach the guest once a sibling is selected** (§6.3).
   The guest is proven correct natively; the loss is in the row-action argument hop — host dispatch or the
   process-global `UiValue` arena. `puzzle3d.flag.*` decides it on #59.
4. **`exportFixture` still emits one inline effect on a 145 KB payload** (§5.3) — B45 §5's residual,
   re-measured unchanged on #58. Until it lands, the four `import*`/`export*` verdicts on Nakagin are
   honestly unreachable rather than red-for-the-wrong-reason.
5. **The guest's interaction snapshot reads empty on the flagship document** (§4.2, §7) — one mechanism
   behind `copyEffects=["0","0"]`, the stale Inspection flag row, and B46's mutate family.

## 10 Infrastructure, for the next wave

- **`rg` through this harness mangles the matched substring** (§0). Read source with `python3`.
- **A host-TS fix is not live until the served transform says so** (B45 §8, used twice here): `touch` the
  file, then `curl` its `@fs` URL and grep for the change. `grep -c uiInputField` → `2` before any browser
  claim in §2.5.
- **A `pgrep` gate that prints a PID after the shell has already composed the command still means STOP.**
  Two lanes (`-p`, `-r`) started with one lingering probe PID visible. Both measure deterministic probe logic
  and `-r`'s reading is corroborated by B45 §4.1's L0, but neither is offered as a contended-load number.
- **A verdict that only asserts "the control's own box moved" measures the host's optimistic draft.**
  `settings-grid-spacing-bumps` passed for weeks on a `NumberStepper` value that the program had never seen
  (a `NumberStepper` publishes no `data-published-value` at all). Score on what the PROGRAM published, or on
  the console hop — B41's rule, and it generalises past select triggers.
- **Three verdicts in this set were decided by the probe's own gestures**: `ensurePanel`'s panel-toggle row,
  `dismissChrome`'s "Collapse" press, and a hard-coded fixture from a run three days earlier. A reader must
  never move what it measures, and a step must never feed itself data it did not just produce.

### 8.5 One contaminated lane, recorded and discarded as proof

`wave-B47-lane-s.txt` (`--only=boot,camera-gestures,projection-options,window-options,settings-panel,outliner-rows`)
was started with **two other probes already live** (`pgrep` printed `16807` and `24406`) and against a
`:6013` whose document had meanwhile gained a `REFERENCES` section (`panel:puzzle3d-play-document/ref-masterarbeit`
— a peer's fixture change mid-wave). Its numbers are contended and its `settings-value-reaches-window-rail`
and `outliner-hide-applies` readings are **discarded**; §2.5 and §6.1's proofs are the gate-clear lanes
`-f`/`-g` and `-q`/`-r`.

It is kept for two things it shows anyway:

```
[115.9s] camera history rows new=["…entry.3=Toggle Panel↶","…entry.4=Switch Panel Tab↶","…entry.6=Activate Window↶"]
         chrome=[… all three …] artifact=[]
[115.9s] verdict camera-emits-no-artifact-history PASS
[159.9s] verdict projection-control-flips FAIL publishedBefore="" publishedAfter="" waitedMs=30517 draftText="Plan"
```

1. §3's attribution doing exactly its job under load: **three** new rows, all three the probe's own shell
   chrome, `artifact=[]`, and the verdict green — this is the `before=2 after=5` of battery #58, named.
2. A THIRD failure mode for `projection-control-flips`, distinct from §1.2's sample and §1.3's obstruction:
   here the listbox opened and the draft reached `"Plan"` while `data-published-value` stayed `""` for
   30 517 ms. That is §7's publication latency, on the control B41 built the published-value reading for —
   and the red now says which of the three it was, every time.
