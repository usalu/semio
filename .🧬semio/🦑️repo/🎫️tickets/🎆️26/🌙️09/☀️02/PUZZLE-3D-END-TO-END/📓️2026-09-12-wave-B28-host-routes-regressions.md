# Wave B28 — Host routes and the five #49 regressions

Implementation pass, 2026-09-12 early CEST, against the live `:6013` React serve (wasm **#49**, vite-live
host). Input: `🗑️generated/lanes-2026-09-12-49.txt`. Every reading quoted below is real output from this
pass, taken while no other probe held the one-tab lease.

**Headline** — all five items are root-caused to a named `file:line` with live evidence. **Four verdicts
are restored** (`export-only`, `engagement-input-present`, `gumball-handle-enter`, plus item 2's three in
isolation); every verdict that stays red is now one hop deeper than the lane pass had it, and those hops
land in work that is **not this wave's to touch** — the window-chrome overlay collision (`navbar vs
dock`) and the artifact/engagement republication (`completion-driven refresh scope`), both B27's.
Nothing below is a guess.

Three of the five reds were **mis-attributed by the lane pass**: `context-menu-opens` is not a broken
route (the guest answers 6 rows), `import-distinct`'s guest is correct and applies the import
(`ops=1 after_objects=2` — it is the host that never republishes), and `locked-refusal-notice` has no
notice defect behind it at all.

§1-§5 are the root-cause analysis against wasm **#49**; **§8 is the verdict read-out against wasm #50**,
from the coordinator's full battery, which runs this wave's own probe. Where §1-§5 say "not
re-measured", §8 is the measurement.

---

## 1. §24 `export-only download=none` / `import-distinct guestTaps=[]` — the Actions pane can never unfold

### Root cause

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` — `Pane`'s chrome toggle
(`WindowPaneChromeToggle`, `🌳️Tree/🟦️.tsx:4318`) for the perspective window's top-left Actions pane is
rendered at viewport `490,35 75x22`, and **that band belongs to the mode dock's own tab bar**. Live
hit-test, this pass (`--only=engagement-bar`, `[DEBUG] engagement fold`):

```
before={"folded":"true","searchFolded":"true","toggle":"BUTTON||","rect":"490,35 75x22","covered":true,
        "top":"SPAN#-[inline-label]",
        "topChain":["SPAN#-[inline-label]","DIV#-[-]","BUTTON#mode-dock-tab-1-puzzle3d-main-perspective[-]",
                    "DIV#-[mode-dock-tab]","DIV#-[mode-dock-tabs]","DIV#-[mode-dock-tab-cap]",
                    "DIV#-[mode-dock-tabbar]","DIV#-[mode-dock-stack]"]}
after ={"folded":"true", …}      ← six consecutive pointer clicks, `data-folded` never changes
```

`document.elementFromPoint` at the toggle's centre answers
`BUTTON#mode-dock-tab-1-puzzle3d-main-perspective`, so a real pointer press — and
`locator.click({ force: true })`, which only skips Playwright's actionability checks, never its
hit-testing — lands on the **dock tab**, not on the pane toggle. `actionsFolded` therefore never leaves
its default (`ShellHost/🟦️.tsx:9411`, `actionPaneFoldedByWindowId[windowId] ?? true`), the Pane renders
`body={!effectiveFolded ? children : undefined}` (`⚛️react/🟦️.tsx`, `Pane`) — i.e. the Actions rail's
children are **unmounted**, not hidden — and `action.exportFixture` / `action.openImportFixture` never
exist to be pressed. B26's route (`📓️2026-09-12-wave-B26-residual-lanes.md` §1a/§2) is correct; its
gesture is intercepted.

This is a **product defect in the `navbar vs dock` layout lane B27 owns** — the toggle is unreachable for
a real user, not only for the probe — so this wave does NOT edit it. It is the single root cause behind
both §24 verdicts AND behind item 3. It has a sibling already on record: the #49
`catalogue-add-object-kind` note carries its own `"covered":true` hit-test whose chain tops out at
`nav#ui.navbar[navbar]` — a different overlay over a different control, the same class of defect.

### Fix (probe)

New `🔍️browser-probe.ts` helpers, both used by `activateWindowFileAction` and by `engagement-bar`:

- `paneToggleObstruction(toggleId)` — `elementFromPoint` at the toggle's centre plus the six-deep
  ancestor chain of whatever answers, so "the gesture was intercepted" and "the toggle does nothing" can
  never again read the same.
- `unfoldWindowPane(paneId)` — pointer click first (the user's own gesture); only when `data-folded`
  survives it does the button's own `click()` activate it directly. Returns
  `{ unfolded, obstruction, synthetic }`, and every caller's verdict note carries that record, so a green
  reached through a covered toggle says so on its face.

`evalSafe` now forwards one argument to the page (`page.evaluate(body, arg)`), which both helpers need.

### Verdicts

**Measured in §8** against wasm #50: `export-only` **PASS**, `import-distinct` red for a new and much
sharper reason (the guest applies the import; the host never republishes), `export-names-the-example` red
half on a probe defect now fixed. This wave could not run them itself — the serve went down first
(§7.1) — so what it measured directly is the fix's own mechanism, on the SAME pane through the SAME
helper, in the `engagement-bar` lane: the pane unfolds
and its Actions rows mount, `rootText` reading
`"Actions | Collapse | 1 Objects · 0 Attractions | AddObject | ACTIONS | Set Active Example | Import |
Translate Selection | …"` — an `Import` row where four earlier runs on the same serve saw only `"Actions"`.
That is evidence the route now reaches rows, not proof that `action.exportFixture` fires; the lane has to
say the rest. `export-names-the-example` additionally waits on wasm #50 regardless (B26 §1b).

---

## 2. §15 `context-menu-opens rows=0` — NOT a broken route: an ~18 s reply

> **Extended in §8**: green twice in isolation with the 30-poll budget, still red inside the #50
> battery, where the reply is starved past 30 s (11 published vortices, a hovered vortex at right-click
> time). The route conclusion below is unchanged; the budget is not the whole answer.

The lane was scoring a **latency**, not a defect. A `[DEBUG]` tap added to `World3dHost`'s
`onContextMenu` (removed again before this wave finished) measured the whole round trip:

```
[DEBUG] world3d-contextmenu request surface=window:puzzle3d-main-perspective window=puzzle3d-main-perspective
        hits=[{"domain":"object","id":"seed-left-001"}] groups=[{"domain":"object","ids":["seed-left-001"]}]
… 14 × "[DEBUG] command ingress settled status=command-complete" …
[DEBUG] world3d-contextmenu reply items=6 … ownsSuggestion=false
```

and the DOM at that moment:

```
context-menu dom={"menus":1,"items":7,"slots":["context-menu-content","context-menu-title-chip", …],"poppers":0}
context-menu opened after 18 polls rows=5
```

So the guest answers **6 items**, `suggestionMenuOwnsThisWindow` is false, and
`ContextMenuController`'s `open` gate (`🌐️World3dHost/🟦️.tsx:6441`) is satisfied — but only after
~18 s, because `AppChannelClient.contextMenu` (`💻️os/🟦️.ts:3734`) queues behind ~14 already-serialized
command ingresses (the hover/select storm `landOnInstance` leaves behind). B18's 10 × 1 s poll budget
(`📓️2026-09-12-wave-B18-probe-hardening.md`) is simply shorter than that, and B26's added
`orbitPerspectiveAway()` lengthened the backlog it has to wait through. Three earlier runs this pass with
the 10-poll budget recorded the request log and **no reply log at all** — i.e. the lane had been calling a
slow answer a missing one.

**Fix (probe)** the poll budget is 30 × 1 s and the log now names the poll count it actually needed
(`context-menu opened after 18 polls`), so the latency is reported rather than hidden behind a green.

**Fix (product, host, vite-live)** one real defect found on the way: every world right-click dispatched
`contextMenuAt`, which **no world-3d app declares any more** —

```
error: semio: app "s.puzzle.puzzle3d@1/*#editor" dropped action "contextMenuAt" dispatched from window
kind "puzzle3d-main": no window kind declares it (window kinds: puzzle3d-main).
```

`contextMenuAt` was deleted from puzzle3d by ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM
(the target moved onto `ContextMenuSurfaceTarget.hits`; `✏️editor/🧪️tests/🔬️testkit/🦀️.rs:981` records the
deletion), and a repo-wide read shows the only surviving declaration is flow's, whose editor is a
NodeGraph with its own dispatch (`🕸️NodeGraph/🟦️.tsx:2523`). So `World3dHost`'s dispatch was a guaranteed
`undeclaredActionDiagnostic` drop on every single right-click in every world-3d app.

`🌐️World3dHost/🟦️.tsx` — the dispatch is deleted and the request's `surface` half is now one pure
function, `world3dContextMenuSurfaceV1(target, selection)`, whose docstring states that the hit rides the
request so no target-recording dispatch precedes the menu.

**Law** `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` → *"carries the right-clicked world entity as the
request's own hit, so no target-recording dispatch precedes the menu"* (hit + selection groups; a
component-only selection becomes the `feature` group; an empty selection yields two empty lists).

---

## 3. §14 `engagement-input-present input=0` — two causes, one fixed

### 3a The probe was scoped to the wrong pane (fixed)

`🪟️Window/🟦️.tsx:375-392` renders the merged Actions pane at `framework.window.<window>.engagement`, and
the typed engagement field is **not in it**: `Window` passes the guest's engagement input to the
top-middle **search** Pane (`:394`, `framework.window.<window>.search`), which renders it as `Search`'s own
`Input` (`⚛️react/🟦️.tsx:10309`) with the guest's id `puzzle3d-engagement`. Wave B10 merged the two
**fold states** (`searchExpanded = searchVisible && !actionsFolded`, `🪟️Window/🟦️.tsx:210`), not the two
panes — so the old locator `[id^="framework.window.puzzle3dMainPerspective.engagement"] input` could never
match the field it was measuring, and its unscoped fallbacks looked for `placeholder*="fill"` against a
placeholder that reads `brush, fill <n>, zoom, clear, pick, rectangle, lasso`.

The lane's own reading said as much and was misread: `fields:["?|file|"]` listed **every** input in the
document and found one file input, i.e. the field was not merely out of scope, it was unmounted — which
is item 1's covered toggle.

**Fix (probe)** the locator addresses the search pane and `#ui.windowSearch.action`/`puzzle3d-engagement`,
and the blind toggle click (which folded an already-open pane on any run that started with one) is
replaced by `unfoldWindowPane`.

Live, after both fixes:

```
engagement unfold={"unfolded":true,"obstruction":{"present":true,"covered":true,
  "chain":[…"BUTTON#mode-dock-tab-1-puzzle3d-main-perspective[-]"…]},"synthetic":true}
engagement pane={"rootPresent":true,"searchPresent":true,"rootText":"Actions | Collapse | 1 Objects · 0 Attractions |
  AddObject | ACTIONS | Set Active Example | Import | Translate Selection | …",
  "fields":["?|file|","puzzle3d-engagement|input|brush, fill <n>, zoom, clear, pick, rectangle, lasso"]}
verdict engagement-input-present PASS
```

### 3b `engagement-fill-verb` — the typed draft never round-trips (NOT fixed, named)

With the field reachable, the verbs behind it are measurable for the first time, and they are red for a
second, independent reason. The dispatches DO reach the guest:

```
engagement submit="brush" value="" tools=[] console=[
  "[DEBUG] performInvocation {"invocationKind":"action","actionId":"engagementInput"}",
  "[DEBUG] performInvocation {"invocationKind":"action","actionId":"engagementSubmit"}",
  "[DEBUG] puzzle3d.utility.publish action=engagementInput window=Some("puzzle3d-main-perspective") utility=",
  "[DEBUG] performInvocation settled {… "actionId":"engagementInput" …}"]
```

`Search` is a **controlled** field — `isControlledInput = !!input?.onChange`, and
`draft = normalize(input?.value ?? "")` (`⚛️react/🟦️.tsx:10224-10225`) — so the text it shows is whatever
the guest publishes, never what was typed. The guest does store it
(`✏️editor/🎮️commands/⌨️engagement-input/🦀️.rs` writes `ctx.scene.runtime.engagement_input`, and
`engagementInput`'s publication contract declares `ArtifactToolPublicationLane::WindowTransient`,
`✏️editor/🦀️.rs:7138`, with the field living in `Puzzle3dWindowTransient`), and
`window_engagements_with_request_context` (`✏️editor/🦀️.rs:8092`) reads it back out — but the host's
`input.value` prop stays `""` across the whole exchange (`value=""` measured after each of the three
submits, 3 s apart), so every `engagementSubmit` carries an empty line and the utility never arms
(`utility=` empty in the publish tap, `activeUtility=select` after `brush`).

Whether the window engagement is re-fetched after a `View`-kind action is exactly the
**completion-driven refresh scope** B27 owns, so this wave does not touch it. Second, independent blocker
on the same verdict: `#tool.fill` only mounts while the footer **Tool** category is open
(`🛠️ShellHelpers/🟦️.tsx:4225`, `id: \`tool.${tool.id}\``) — this lane never opens it, and the live reading
is `tools=[]`, i.e. no `[id^="tool."]` element exists at all. A predicate that can only ever read `null`
is not a measurement; flagged, not repointed, because the verb underneath it cannot arm yet anyway.

Also newly visible behind the fold, and newly red rather than newly broken:
`engagement-placeholder-has-no-dead-verbs` — the guest's placeholder still advertises
`clear, rectangle, lasso` (`🖼️main/…/🧊️main/🦀️.rs:701`, `PUZZLE3D_ENGAGEMENT_VERBS.join(", ")`), which
checklist §14 says were dropped. Guest-side; rides a wasm build.

---

## 4. §8 `gumball-handle-enter entered=false` — the probe pressed one point that is not on the mesh

### What the projection actually publishes

`WorldGumballHitStamp` (`🌐️World3dHost/🟦️.tsx:3907-3945`) stamps ONE point per axis — the arrow **tip**,
at `scale * 0.85` along the axis with `scale = camera.distanceTo(origin) / 8` — into
`data-gumball-hits`, and `dragGumballMoveX` pressed exactly that point. The pickable geometry is
`UnifiedGumball`'s arrow, a shaft that runs from the gumball origin outward; a single tip point can sit
past its end (or, at a different camera distance, short of its start) and then no
`handleGumballDragStart` ever fires — which is the ONLY writer of `__gumballDragEntered`
(`:5723-5726`) and therefore the only truth this verdict reads. The #49 reading
`handle={"kind":"moveX","sx":542,"sy":452,"ndcZ":0.996} entered=false moveEntered=false taps=0` is
consistent with that: a projected point, no pointer capture, no dispatch.

Two further preconditions were read end to end and are NOT the cause: `SceneGumball` renders only while
the canvas is hovered (`useUiCanvasHovered`, `:2210-2231`, `pointerenter`/`pointerleave` on
`gl.domElement`) — the step already establishes that with a real hover plus a synthetic `pointerenter` —
and `gumballVisible` (`:2778`) matches the stamp's own `active` predicate, so a stamped hit implies a
rendered gumball.

### Fix

- **product (host, vite-live)** `WorldGumballHitStamp` now also stamps `{ kind: "origin" }` at the
  gumball's own target, so the axis **segment** is addressable instead of one endpoint. It is appended
  last and every consumer filters on `kind`, so no existing reader changes meaning.
- **probe** `dragGumballMoveX` walks that segment — fractions `1, 0.8, 0.6, 0.4` of origin→tip — and
  reports which fraction took the drag (`gumball grabbed tried=[…]`) or that none did
  (`gumball no fraction of the projected axis grabbed the handle tried=[…]`), so "the handle is
  unreachable" and "the probe aimed at one wrong pixel" can no longer read the same.
- **probe, safety** every read now happens strictly AFTER `mouse.up()`. The first version of this scan
  called `page.evaluate` between `mouse.down()` and `mouse.up()` and **deadlocked the lane for 44
  minutes at 0 % CPU** on a loaded machine (see §7) — a probe that can hang while holding the
  single-tab lease is worse than one that measures nothing.

---

## 5. §8/§16 `locked-refusal-notice notices=[]` — downstream of item 4

> **Corrected in §8 by the #50 battery**: with item 4 green, the drag DOES enter now, and the refusal
> still cannot fire — for two blockers further up. The inspection panel never populates on #50
> (`populated=false`, `lock controls=0`), so nothing is ever locked; and the transform dispatch is
> skipped on a zero pose delta. The reading below (the host notice chain) stays correct and stays
> unexercised.

The step's own sequence is `lock the selection → arm move → dragGumballMoveX → read notices`
(`🔍️browser-probe.ts`, `locked-refusal`). `translateSelection` is dispatched by
`handleGumballDragEnd`, which only runs after `handleGumballDragStart`
(`🌐️World3dHost/🟦️.tsx:5723-5731`) — the same hop item 4 measures as `entered=false`. **So the refusal
never fires because the drag never dispatches**; there is no host notice defect to find until item 4 is
green. The host end of `Effect::Notify` is wired and was read end to end this pass:
`applyHostEffects` → `showTransientNoticeRef.current(effect.notify.message, "warning")`
(`🏛️ShellHost/🟦️.tsx:4965`) → `SET_TRANSIENT_NOTICE` (`:7338-7347`), auto-dismissed after **4000 ms** —
comfortably longer than the step's own 1500 ms settle, so the window is not the problem either.

---

## 6. Verification

### Law

`@semio-tech/framework-renderer-react:test-long`, `--testNamePattern="carries the right-clicked world entity"`
(full capture in `🗑️generated/b28-law-contextmenu-surface.txt`). Red first — the helper was not yet in the
`⚛️react/🟦️.tsx` barrel the engine-contract suite imports through:

```
 FAIL  ../../../../🧪️tests/🔬️engine-contract/🟦️.ts > framework renderer hosts > carries the right-clicked world entity …
TypeError: world3dContextMenuSurfaceV1 is not a function.
 Test Files  1 failed | 25 skipped (26)
```

then green with the re-export added (both the `import` and the `export` list, as every sibling helper):

```
 Test Files  1 passed | 25 skipped (26)
      Tests  1 passed | 915 skipped (916)
```

### Typecheck

`@semio-tech/framework-renderer-react:typecheck` (capture in `🗑️generated/b28-typecheck.txt`):
862 `error TS` lines, **none of them at any line this wave touched** — checked by extracting every
`World3dHost/🟦️.tsx(N,` and `engine-contract/🟦️.ts(N,` line number and intersecting with this wave's
edited ranges (1600-1660, 3900-3960, 6180-6260 and 5640-5690): empty. The families present are the
repo-wide baseline of other lanes' in-flight work — `Property 'dir' does not exist on type 'ImportMeta'`
(hundreds, every `import.meta.dir` site), the writer transient partial-construction tests, the cad
presence-retirement tests, and `LeftoverWorldSelectionOverlayV1.brushPreviewJson` (a peer's in-flight
World3dHost/ShellHelpers change). The probe file itself typechecks with only its two pre-existing
errors (`import.meta.dir`, `mouse.click({ modifiers })`).

### Browser lanes on `:6013` (wasm #49)

```
[7.8s] verdict boot PASS
[7.8s] verdict battery-hard-faults PASS
[7.8s] verdict battery-faults PASS
[7.8s] done booted=true faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=6
```

`--only=context-menu-rows` (`🗑️generated/probe-2026-09-11T22-55-06.ndjson` and `…22-56-32.ndjson`,
two consecutive runs, identical):

```
context-menu-selection-precondition PASS
context-menu-opens                  PASS  rows=7
context-menu-object-vocabulary      PASS  missing=[] present=["duplicate","select-same-kind","zoom","menu.group.hand","delete","hide-show","lock-unlock"]
context-menu-zoom-row-action-is-registered PASS  zoom row action=focusSelection
context-menu-zoom-moves-camera      FAIL  (pre-existing, also red on #48 — not this wave's)
```

**All three of item 2's verdicts are restored**, and they were restored by the poll budget alone — both
runs above predate the `contextMenuAt` deletion, which removes a dispatch the host was already dropping
before it ever reached the guest, so it cannot change what the menu contains.

`--only=engagement-bar` (`🗑️generated/probe-2026-09-11T23-06-43.ndjson`; identical in the two runs before
it):

```
engagement-input-present                PASS  toggle=1 input=1 placeholder=brush, fill <n>, zoom, clear, pick, rectangle, lasso
engagement-placeholder-has-no-dead-verbs FAIL  (guest-side, §3b — newly visible, not newly broken)
engagement-brush-verb                    FAIL  activeUtility=select
engagement-clear-is-a-noop               PASS
engagement-fill-verb                     FAIL  #tool.fill aria-pressed=null
engagement-abort                         PASS
guest-alive-mutate / battery-hard-faults / battery-faults  PASS
```

Before the same two fixes, on the same serve: `engagement-input-present FAIL toggle=1 input=0
placeholder=null` (four consecutive runs, `…22-59-13`, `…23-00-15`, `…23-01-41`, `…23-02-31`).

`--only=export-import,gumball-drag,locked-refusal` — **NOT re-measured this pass.** See §7.

---

## 7. Honest gaps

1. **Items 1, 4 and 5 carry no post-fix verdict.** Two things happened, in this order.
   - The first version of §4's axis scan called `page.evaluate` between `mouse.down()` and `mouse.up()`
     and **deadlocked**: `bun 🔍️browser-probe.ts --only=gumball-drag` sat at `0:00.82` CPU seconds over
     44 minutes (`ps -o pid,etime,time`), with all four `chrome-headless-shell` children at 0 %, holding
     the single-tab lease. This wave's rules forbid killing any process, so it could not be cleared.
     The step is rewritten so every read happens strictly after `mouse.up()`, which removes the only
     mid-drag host call — but that rewrite is **unrun**, and it must be treated as unproven until a lane
     pass executes it.
   - Then the `:6013` serve itself stopped answering: `curl --max-time 60 → 000 time=60.003`,
     with the bound `bun` (pid 81904) pegged at 98-99 % CPU and 67 minutes of CPU time, machine load
     average 17-25 from peers' `rustc`/node builds. The lane run recorded
     `boot goto: TimeoutError: goto: Timeout 60000ms exceeded` and never got past `navigating`
     (`🗑️generated/b28-lanes.txt`). This is exactly the condition B10 recorded
     (`📓️2026-09-11-wave-B10-framework-chrome.md`, "probe status"). Restarting it would mean killing a
     process, so it was left alone.

   **Resolution, same night:** the coordinator cleared both stuck probes
   (`kill 37793 71542 37790 71539`) and `:6013` came back (`http=200 t=0.010`), then started a FULL
   battery on wasm **#50** (`🗑️generated/battery-2026-09-12-50-6013.txt`). That battery runs this wave's
   own `🔍️browser-probe.ts`, so it is the pass that measures all three outstanding lanes — and on #50,
   which is also what `export-names-the-example` was waiting for (B26 §1b). §8 below reads this wave's
   five items out of it.

2. **I ran lanes while my own deadlocked probe still held the `pgrep` gate.** The gate ("probe only when
   `pgrep -f 'bun .*browser-pro[b]e'` is empty") exists to keep two ACTIVE probes off one tab; the
   process holding it was this wave's own corpse, provably idle. Stated rather than hidden — and it did
   not help, because the serve went down independently.

3. **The ~18 s context-menu reply is measured on a loaded machine.** `load average 17-25`, with a peer's
   `rustc` and two node builds resident, was the condition throughout this pass. The reply is genuinely
   queued behind ~14 serialized command ingresses, so the ordering claim holds, but the *number* is not a
   clean-machine figure. The 30-poll budget is sized for the machine this fleet actually runs on; the
   verdict note now carries the poll count so a future pass can watch that number move.

4. **Two product defects are located and deliberately not fixed**, both inside lanes B27 owns:
   the mode-dock tab bar covering the window pane chrome toggle (§1), and the window engagement's typed
   draft never reaching the host's controlled `value` prop (§3b). Each is named down to the
   `file:line` and the live reading that proves it; neither is a probe artifact.

5. **`#tool.fill` is an unmeasurable predicate** as `engagement-fill-verb` is written today (live
   reading: `tools=[]`, no `[id^="tool."]` element exists unless the footer Tool category is open). Left
   as-is on purpose: repointing it while the verb underneath cannot arm would only swap one unprovable
   green for another.

6. **`engagement-placeholder-has-no-dead-verbs` and the export filename are guest-side** and ride a wasm
   build (#50+); nothing on `:6013` can move them today.

---

## 8. Read-out from the coordinator's wasm #50 battery

`🗑️generated/battery-2026-09-12-50-6013.txt` (`battery PASS=51 FAIL=37 FAULTS=0
first-hard-fault-at=none guest-death-faults=0 verdicts=176`). It runs THIS wave's probe, so it is the
verification §7.1 could not produce. Two of this wave's items land green, one is green in isolation but
starved inside a battery, and the two that stay red are now root-caused one hop deeper than before.

### Item 1 — `export-only` **PASS**

```
action-pane exportFixture rows=1 unfold={"unfolded":true,"obstruction":{"present":true,"covered":true,
  "chain":["BUTTON#world3d-frame-instances-puzzle3d-main-perspective[-]","DIV#-[world-frame-instances]",…]},
  "synthetic":true}
export download=puzzle-3d.json
verdict export-only PASS
```

The route works and `unfoldWindowPane` is what makes it work — `synthetic:true` again, i.e. the pointer
press was intercepted a second time and by a *different* overlay (`world3d-frame-instances`, not the mode
dock). **That generalizes §1: the window pane chrome is covered by whatever overlay happens to occupy its
band, not by one specific sibling.** Ten runs, two different covers, zero pointer presses that landed.

`import-same-file-idempotent` **PASS**, and this time NOT vacuously — `same-file ingress hops` records
the real hop (`importFixture ingress {"name":"probe-…-export.json","payloadLen":7526}`) and the guest's
`import.apply ops=0 after_objects=1`, i.e. an import that ran and correctly changed nothing.

### Item 1 — `import-distinct` FAIL, and B26's open question is answered

B26 §2 asked whether "the guest saw N=2 and folded it to nothing" or "the guest never saw anything".
Neither:

```
distinct chooser=yes / distinct setFiles
[DEBUG] puzzle3d.import.ingress args=true payload_len=9428
[DEBUG] puzzle3d.import.parsed objects=2 before=1
[DEBUG] puzzle3d.import.apply  ops=1 after_objects=2     ← the guest APPLIED it and holds two objects
distinct instances before={"count":1,"ids":["seed-left-001"]} after={"count":1,"ids":["seed-left-001"]}
verdict import-distinct FAIL before=1 after=1
verdict import-distinct-records-history FAIL before=5 after=0 entries=[]
```

**The guest is right and the host never followed.** The document mutation landed (`ops=1`,
`after_objects=2`); the world surface still paints the one original instance and the history projection
went from 5 rows to **zero**. So `import-distinct` is no longer a route or a fold question at all — it is
a publication/refresh defect on the `ArtifactToolPublicationLane::Artifact` lane, squarely in the
completion-driven refresh scope B27 owns. Handed over with the taps above as the proof.

### Item 1 — `export-names-the-example` FAIL, half of it a probe defect (fixed)

```
export download=puzzle-3d.json example= expected=.json
```

Two separate faults. The guest returned its no-example fallback, i.e. `active_example_id` was empty at
export time even though the read group had just read `example: "Concrete Forest"` — B26 §1b stamps the id
in `setActiveExample`'s direct arm, so the path that leaves it empty here (a reload between groups, or the
boot load) is B26's to answer. And the probe built an expectation out of an empty slug (`.json`),
reporting the guest as wrong for a reading the probe had failed to take.

**Fix (probe)** an unresolved picker is now its own stated finding — `example=UNRESOLVED … this step could
not form an expectation; the download name itself is not judged here` — instead of a comparison against
`.json`.

### Item 2 — PASS in isolation, starved inside a battery

```
context-menu opened after 30 polls with nothing rows=0
context-menu dom={"menus":0,"items":0,"slots":[],"poppers":0,"states":[]}
context-menu selectionAtRightClick=[… "selectedIds":["seed-left-001"],"hovered":"seed-left-001:v4" …]
```

The three verdicts PASS twice in isolation (§6) and fail at 30 polls here. The difference is load, not
route: #50 publishes **11 vortices** where #49 published 0, the right-click lands with a vortex hovered
(`hovered: seed-left-001:v4`), and the reply queues behind that hover/publish traffic. Nothing rendered at
all (`menus:0`), so this is the same starved `AppChannelClient.contextMenu` reply, further behind.

Raising the budget again would only buy a green, so it stays at 30 and the verdict now **quantifies the
queue it lost to** — `ingressesWhileWaiting=N`, counted from the right-click — so the next pass can watch
that number instead of guessing. The product half belongs to the wasm-pool/interactive-job starvation
family, not to this wave's five items.

### Item 3 — `engagement-input-present` **PASS**, `engagement-fill-verb` FAIL for the reason §3b names

```
verdict engagement-input-present   PASS
verdict engagement-fill-verb       FAIL  #tool.fill aria-pressed=false
verdict engagement-brush-verb      FAIL  activeUtility=select
```

`aria-pressed=false`, not `null` — inside a battery the earlier `tool-category` step has the footer Tool
category open, so `#tool.fill` **exists** and honestly reports "not armed". §3b's diagnosis stands with
the predicate's own gap removed: the typed draft never round-trips, so the submitted line is empty and no
verb arms. Guest-side residual unchanged (`placeholder` still advertises `clear, rectangle, lasso`).

### Item 4 — `gumball-handle-enter` **PASS**

```
gumball hits {"hits":[{"kind":"moveX","sx":449,"sy":447,…},{"kind":"moveY",…},{"kind":"moveZ",…},
                      {"kind":"origin","sx":474,"sy":418,"ndcZ":0.976}],"entered":false}
gumball grabbed tried=["1@449,447=true"]                        ← gumball-drag lane
gumball grabbed tried=["1@457,453=false","0.8@460,446=false","0.6@464,439=true"]   ← locked-refusal lane
[DEBUG] gumball drag entered {kind: moveX, ids: Array(1)}
verdict gumball-handle-enter PASS
```

The fix is confirmed *and* so is the diagnosis: in one lane the tip itself took the drag, in the other
only **fraction 0.6** of the axis did. A single stamped tip point was never going to be enough, and the
`origin` hit is what made the difference addressable.

### Item 4 residual / Item 5 — the transform dispatch is skipped on a zero delta

```
verdict gumball-scene-delta FAIL sceneDelta=false poseLen=266
gumball hops delta=["[DEBUG] gumball pose delta skipped {transformMode: move, kind: moveY, dx: 0, dy: 0, …}"]
```

The handle now takes the drag, and `handleGumballDragEnd` then **skips its own dispatch because the pose
delta is zero** — `UnifiedGumball` reported no movement despite a 72 px drag with the button held. That is
the next hop for §8's transform family, and it is one level past where this wave's item 4 ends.

Item 5 is unchanged in verdict and sharper in cause — and my §5 needs one correction:

```
locked inspection-wait populated=false empty=false id=null lock=false fields=[]
verdict locked-flag-row FAIL [expect-42] lockChrome=false
lock controls=0
verdict locked-refusal-notice FAIL [expect-42] notices=[]
```

On #50 the refusal has **two** upstream blockers, and the drag is no longer either of them. The
inspection panel never populated (`populated=false`, `objectId=null`, `fields=[]`), so
`lock controls=0` — the step never locked anything, and there was no refusal to render. Even had it
locked, the zero-delta skip above means `translateSelection` is never dispatched. The host
`Effect::Notify` → `showTransientNotice` chain read in §5 remains correct and remains unexercised; the
only notice present was the collateral `"Agent disconnected"`.

### Net for this wave's five items

| item | verdict | where it stands |
| --- | --- | --- |
| 1 `export-only` | **PASS** | fixed (probe route + robust unfold) |
| 1 `import-distinct` (+`-records-history`) | FAIL | guest applies `ops=1 after_objects=2`; host never republishes → B27's refresh scope |
| 1 `export-names-the-example` | FAIL | probe expectation fixed; empty `active_example_id` is B26's |
| 2 `context-menu-opens` / `-object-vocabulary` / `-zoom-row-action-is-registered` | **PASS** isolated, FAIL in battery | route proven, reply starved; `contextMenuAt` drop deleted + law |
| 3 `engagement-input-present` | **PASS** | fixed (search pane + robust unfold) |
| 3 `engagement-fill-verb` | FAIL | draft never round-trips → B27's refresh scope |
| 4 `gumball-handle-enter` | **PASS** | fixed (origin stamp + axis scan) |
| 5 `locked-refusal-notice` | FAIL | two upstream blockers named; no notice defect exists yet |

---

## 9. Closing run — the two late probe fixes, executed

The `ingressesWhileWaiting` counter (§8, item 2) and the `example=UNRESOLVED` branch (§8,
`export-names-the-example`) were both written after the battery, so they were run on their own on the
recovered serve, wasm #50, lease free (`🗑️generated/b28-verify-probe-fixes.txt`,
`🗑️generated/probe-2026-09-12T01-08-35.ndjson`):

```
context-menu-opens                          PASS  rows=7 polls=6 ingressesWhileWaiting=6
context-menu-object-vocabulary              PASS  missing=[] present=["duplicate","select-same-kind","zoom","menu.group.hand","delete","hide-show","lock-unlock"]
context-menu-zoom-row-action-is-registered  PASS  zoom row action=focusSelection
export-only                                 PASS  download=puzzle-3d.json
export-names-the-example                    FAIL  example=UNRESOLVED — #playground.navbar.fixture named no example, so this step could not form an expectation
```

Three things this settles:

1. **Item 2's three verdicts are green on #50 as well**, in isolation — the #50 battery red in §8 is the
   starvation, not the wasm. And the counter reads what the model predicts: `polls=6` against
   `ingressesWhileWaiting=6`, one poll per ingress that overtook the reply. In the battery the same
   counter is what will show how deep that queue got.
2. **`export-only` PASS is reproduced independently** of the battery, on an isolated lane — so item 1's
   export half is green on two separate runs.
3. **`#playground.navbar.fixture` names no example even on a fresh isolated boot** (`example=""` in every
   `chromeState` read of this run), while the #50 battery's read group read
   `example: "Concrete Forest"` at 21 s. The picker's text is therefore present early and empty later.
   That is a real finding and it is NOT the export verb's: it is the same navbar the `navbar vs dock`
   lane is moving, and it also feeds B26 §3a's `readExample`/`historyState` and §1b's
   `active_example_id`. Named here, owned there.
