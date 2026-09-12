# Wave B49 — the tutorial veil, the Inspection/rail overlap, and a row action's "lost" arguments

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · written incrementally while the wave ran · every verification
command run in the FOREGROUND with its tail quoted · every browser lane run after checking
`pgrep -f 'bun .*browser-pro[b]e'`.

Handed set (from `📓️2026-09-13-wave-B47-full-run-residuals.md` §1, §5.1, §6):

1. the welcome-tour veil that leaves the whole app inert after Skip,
2. the Inspection panel body covering the perspective window's measures rail,
3. a row action's explicit `{entity, flag, ids, value}` lost when a selection exists.

All three are PRODUCT defects. Two are fixed and measured in the browser. The third's premise turned out to
be false: the host carries the arguments intact — measured, quoted in §3 — and what remains is one guest
exit, localized to three lines.

The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`, `semio
(CONNECTION_CLOSED)`), so the ticket folder is managed on disk. No state-modifying git command was run, the
ticket is neither closed nor reopened, nothing under `🗑️generated` was deleted, and the one temporary
`[DEBUG]` trace this wave needed was removed again (§3.2 names the exact line to re-arm it).

---

## 1 The welcome-tour veil — TWO product defects, one on top of the other

### 1.1 What the handed evidence actually said

`🗑️generated/wave-B47-lane-n.txt` (one page load, no reload):

```
[9.1s]  boot dismissed welcome tour
[25.7s] example picker shape={… "chain":["DIV#-[-]{ui-veil z-tutorial inset-0 absolute pointer-events-auto}
        @0,0,1440,900:absolute/10000/pe=auto", …]}
[25.7s] example options=[] pickerPresent=1 opened=false waitedMs=8062
```

and every post-B47 lane (`-o`, `-p`, `-q`, `-r`, `-s`): a blocking veil found by the FIRST `dismissChrome`
1–10 s after `waitForBoot` logged `dismissed welcome tour`, cleared once, and never seen again in the rest
of a 100 s+ lane. `clearIntroductionVeil` only logs when it FINDS a blocking veil, so the shape is "the
first Skip press did not end the tour", not "the tour re-arms after every dismissal": `waitForBoot` presses
Skip with a plain, actionability-checked `click({ timeout: 3000 })`, while `clearIntroductionVeil` presses
`Escape` (the `ui.introduction.skip` keybinding) and then `click({ force: true })`, which skips
hit-testing. That difference is the whole of it.

Two reducer facts rule out a re-arm inside one page load: `AUTO_START_INTRODUCTION` dedupes per key
(`🐚️Shell/🟦️.tsx:929-936`, pinned by an existing engine-contract law), and `session.app.id` is a boot
constant (`ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor`, read out of `localStorage` in §1.4).

### 1.2 Root cause A — the info box is placed OUTSIDE the viewport, with its only exits on it

Measured in a browser pane at a real 1440×900 viewport, on a first-run boot (seen flag cleared), before any
fix:

```
veil  [0, 0, 1425, 885]   z-index 10000  pointer-events auto   (elementFromPoint at its centre = the veil)
box   [-80, -78, 422, 108]                                     data-slot="introduction-info-box"
skip  [287, -78, 55, 31]  id="ui.introduction.skip"            elementFromPoint(centre) = null
title "Welcome to Puzzle 3D"
```

The box's top-left is 80 px past the left edge and 78 px ABOVE the top edge, so its Skip control's centre
(y ≈ −62) is outside the viewport: `document.elementFromPoint` returns `null` there. Nothing can press it —
not Playwright's actionability-checked click, **and not a user** — while the step's veil (puzzle3d's first
step `welcome` has no `introduce`, so `veilBlocksPointer` is true) owns every pointer in the application.
Escape works, and `force: true` works, which is exactly the asymmetry the lanes recorded.

`resolveIntroductionPlacement` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:4778-4809`, pre-fix) clamped
every ANCHORED branch and returned the centered one raw:

```ts
const centered = { top: (viewport.height - boxSize.height) / 2, left: (viewport.width - boxSize.width) / 2 };
if (!anchorRect || placement === "center") return centered;
```

A box measured larger than its (portal-host-measured) viewport therefore centers to negative coordinates.

**Fix** — same file, every branch clamps now, the centered one included:

```ts
const gap = INTRODUCTION_INFO_BOX_GAP_PX;
const clamp = (position: IntroductionInfoBoxPosition) => clampIntroductionInfoBoxPosition(position, boxSize, viewport, gap);
const centered = clamp({ top: (viewport.height - boxSize.height) / 2, left: (viewport.width - boxSize.width) / 2 });
```

### 1.3 Root cause B — the chrome body plane eats the whole cap row, because `z-[n]` is not emitted

With the box on-screen the Skip control was still not reachable:

```
skip [876, 401, 45, 22]   elementFromPoint(898,412) = DIV[window-chrome-body]   ← not the button
stack at that point: [window-chrome-body, SPAN, BUTTON#ui.introduction.skip, window-chrome-controls, window-chrome-cap]
```

and the reason, read off the live DOM:

```
window-chrome-cap      class "relative z-[2] …"   computed z-index: auto
window-chrome-body     class "z-[1] … window-silhouette-content-plane relative …"   computed z-index: auto   margin-top: -22.39px
window-chrome-footer   class "relative z-[2] …"   computed z-index: auto
(the veil's own `z-tutorial` computes 10000, and panel bodies compute 30 — NAMED z utilities do apply)
```

`WindowChrome`'s internal stacking rode Tailwind ARBITRARY z utilities (`z-[1]`/`z-[2]`) that this design
system's stylesheet does not emit — all three computed `z-index: auto`, so DOM order decided instead. The
body plane negative-margins UP over the cap row by design (so the silhouette outline can wrap the chips),
and it is a LATER sibling, so it painted over the entire cap row: every cap-row control became unreachable
by any hit-tested press — the tour's Skip among them. The silhouette clip does not save it, and by design
cannot: the plane's own clip polygon keeps the plane at full height exactly ACROSS the chip x-ranges
(`polygon(0 0, 156.7 0, 156.7 22.4, 371.8 22.4, 371.8 0, 416.9 0, …)` — cut away only in the gap BETWEEN
the chips, because the chips are meant to paint on top of it).

**Fix** — `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`, two exported constants next to
`windowControlsCapClass` and four call sites inside `WindowChrome` (cap row, body plane, both footer
variants):

```ts
export const WINDOW_CHROME_CHIP_ROW_STYLE: React.CSSProperties = { zIndex: 2 };
export const WINDOW_CHROME_BODY_PLANE_STYLE: React.CSSProperties = { zIndex: 1 };
```

Stated inline, the authored order cannot silently vanish with a utility that was never generated. This is
the B45 §2.2 family ("a grab strip never covers a button"), one layer in: a chrome plane over its own
control row. It fixes every `WindowChrome` consumer's cap row, not just the tour's.

### 1.4 Hardening — a dismissal is durable for the session, whatever re-publishes the app

The app definition the introduction hangs off is republished (new object, same content) on every full
refresh, hot-swap and re-established session, so the auto-start effect re-runs many times per boot; its only
guards were the per-key reducer dedupe and a device-local `localStorage` flag that an ephemeral brand's
in-memory `StoragePort` never receives. The decision is now one pure predicate plus a session-durable
record of the answer:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:316-341` — new
  `shouldAutoStartIntroduction({appId, hasIntroduction, tutorialActive, suppressed, replayOnLoad, seenOnDevice, dismissedAppIds})`;
  refuses an empty app id (an id that has not arrived yet would arm once under `""`, persist nothing, and
  arm again the moment the real id lands) and refuses any app id already answered in this session.
- `🏛️ShellHost/🟦️.tsx:4036-4071` — the auto-start effect now calls it; `:4186-4199` — `dismissIntroduction`
  records `session.app.id` in `dismissedIntroductionAppIdsRef` (lives for the shell mount, exactly "this
  session", so a `replayIntroductionOnLoad` brand still replays on the next LOAD).
- Re-exported through `🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx` next to `shouldPersistIntroductionSeen`.

### 1.5 Escape hatch — a blocking veil is never a dead end

`⚛️react/🟦️.tsx:5813-5830`: the veil is now `data-slot="introduction-veil"`, `aria-hidden`, and while it
blocks (`veilBlocksPointer`) a press on it ends the tour (`cursor-pointer`, `onPointerDown={skip}`).
Keyboard parity was already there (`ui.introduction.skip` = `escape`). A scrim that owns every pointer in
the app must itself be the way out.

### 1.6 Laws

`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`:

- `resolveIntroductionPlacement > keeps a centered box inside the viewport, so its Skip and Next controls stay clickable`
  (the measured 1585×1041-in-1425×885 case and a 0×0 viewport).
- `UIIntroduction veil and elevation > ends the tour when its blocking veil is pressed, so a screen step can never leave the app inert`
  (veil is `pointer-events-auto` + `aria-hidden`, `pointerDown` ⇒ `onDismiss(false)`).
- `UIIntroduction veil and elevation > never captures pointer events with no step to show` (a step index past
  the last step renders no veil at all).
- `control chrome > panel chip-cap and controls paint glass above one transparent clipped payload` — its dead
  `expect(body.className).toContain("z-[1]")` assertion replaced with the real thing:
  `expect(body.style.zIndex).toBe("1")` and `expect(cap.style.zIndex).toBe("2")`.

`🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`:

- `shouldAutoStartIntroduction offers an app's tour once per session and never re-arms a veil the user dismissed`
  (dismissed app id refused even with `replayOnLoad`; a different app id still armed; empty app id refused).

```
$ bun x vitest run --config 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts -t "resolveIntroductionPlacement" --reporter=verbose
 ✓ …> resolveIntroductionPlacement > centers when there is no anchor rect, or placement is center 8ms
 ✓ …> resolveIntroductionPlacement > keeps a centered box inside the viewport, so its Skip and Next controls stay clickable 0ms
 ✓ …> resolveIntroductionPlacement > auto picks the side with the most free space 0ms
 ✓ …> resolveIntroductionPlacement > explicit placements position relative to the anchor and stay within the viewport 0ms
      Tests  4 passed | 727 skipped (731)

$ bun x vitest run … -t "ends the tour when its blocking veil is pressed" --reporter=verbose
      Tests  1 passed | 728 skipped (729)

$ bun x vitest run … -t "panel chip-cap and controls paint glass" --reporter=verbose
 ✓ …> control chrome > panel chip-cap and controls paint glass above one transparent clipped payload 355ms
      Tests  1 passed | 730 skipped (731)

$ SEMIO_TEST_LEVEL=long bun x vitest run --config …/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts -t "shouldAutoStartIntroduction" --reporter=verbose
 ✓ …> shell option locks (SEMIO_LOCKED_*) > shouldAutoStartIntroduction offers an app's tour once per session and never re-arms a veil the user dismissed 1ms
      Tests  1 passed | 1038 skipped (1039)
```

### 1.7 Browser verdicts

After the placement clamp alone (`🗑️generated/wave-B49-lane-a.txt`), the tour still survived the boot Skip —
which is what named root cause B:

```
[10.2s] boot dismissed welcome tour
[16.0s] veil dismissChrome cleared=true state=null      ← a blocking veil was still there to clear
```

After the chrome stacking fix, measured in the pane at 1440×900 on a first-run boot:

```
skip  [876, 401, 45, 22]   reachable: true    (elementFromPoint at its centre is a SPAN INSIDE the button)
caps  [{z:"2"}]   bodies [{z:"1"}]
press on the veil at (100,800) ⇒ tourStillUp false, veilStillUp false,
                                 localStorage ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor=true
then watched a full boot to 2 windows / 2 canvases: veils 0, introBox false      ← no re-arm
```

and in the probe's own terms (`wave-B49-lane-c.txt`, 117 s, every step calling `dismissChrome`):

```
[8.5s] boot dismissed welcome tour
(… no `veil …` line anywhere in the lane: the single plain Skip click landed, so there was never a
   blocking veil left to clear …)
```

Class: **product ×2** (viewport-clamping + chrome stacking), **fixed**; plus session-durability hardening
and an escape hatch. The B47 probe-side `clearIntroductionVeil` stays valuable as a belt.

---

## 2 The Inspection panel body over the window's measures rail

### 2.1 Root cause — nothing reserved the column, and both sides ask for the same inset

The dock's right column and a window's measures rail compute the SAME inset from the same helper against
DIFFERENT containing blocks, and neither knows about the other:

- `🖼️Panel/🟦️.tsx:442-446` → `chromeHostedOpenPanelPositionStyle(anchor)` /
  `anchorPositionStyle(anchor)`, at `getLevelZClass("panel")` = `z-index: 30`, positioned against Layout's
  middle region.
- `🪟️Window/🟦️.tsx:321-361` → the measures rail as `Pane anchor="top-right"`
  `overlaySlot="window-measures-overlay"`, positioned against `window-body` at `z-pane` inside the window's
  own stacking context.
- `⚛️react/🟦️.tsx` `anchorPositionStyle` — the only horizontal term is
  `style[horizontal] = "var(--spacing-single)"`; `chromeHostedOpenPanelPositionStyle`'s whole delta is
  vertical. No side-aware reserve exists anywhere; the one precedent,
  `shellNavbarTrailingEndReserveStyle`, reserves a panel CAP row against the navbar, not a window rail.
- `📐️Layout/🟦️.tsx:127-142` — the eight anchored panels are absolute siblings OVER the canvas, so the
  canvas (and every window in it) is laid out as if the dock column were not there.

B47 §1.3 measured the consequence: panel body `x 1137‑1437` over rail `x 1130‑1430`, the panel winning by
z-index, so every press in the rail's top band landed on an Inspection tree row —
`OBSTRUCTED puzzle3d-play-inspector covers the trigger centre`. Restacking cannot fix it (raising the rail
only buries the panel's own rows), so the window has to yield the overlap.

### 2.2 Fix — the dock publishes its column, the window yields exactly the overlap

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`, new `#region ↔️DockColumnReserve` next to the navbar
reserve it mirrors:

- `publishShellDockRightColumnLeftPx(root, key, leftPx | null)` / `useShellDockRightColumnLeftPx(root)` — the
  viewport x of the leftmost OPEN right-hand dock panel, per shell root, one entry per right anchor
  (`top-right`/`right`/`bottom-right` can be open at once), `null` when the column is closed.
- `dockColumnInlineReservePx(chromeRightEdgeX, dockColumnLeftEdgeX, availableWidthPx)` — the overlap and
  nothing more, clamped to what the window can actually spare (its body width minus the rail's own minimum),
  `0` when the column is closed or the window stops short of it.
- `anchorInlineEdgeReserveStyle(anchor, reservePx)` — `anchorPositionStyle`'s edge inset moved inward on this
  anchor's own horizontal edge; `undefined` for a middle anchor or a zero reserve, so a dock-less window's
  rail keeps its flush `var(--spacing-single)` byte-for-byte (the two existing `Window` laws still pass).
- `Pane` gained `inlineEdgeReservePx` (default `0`), applied to its position style and its `maxWidth`.

`🖼️Panel/🟦️.tsx:429-457` publishes its own left edge in a layout effect (ResizeObserver + `resize`),
retracting on close and on unmount. `🪟️Window/🟦️.tsx:203-225` reads the column, measures its own body, and
passes the reserve to the measures `Pane` (`:349`) and to the window-controls cluster (`:317-321`), which
lives at the same corner and was equally buried.

### 2.3 Law

`🧪️owned-locale-detector-retirement/🟦️.tsx`:
`Shell components > yields a window's right-edge chrome to an open right-hand dock column instead of sharing it`
— the arithmetic (closed column ⇒ 0; a window that stops short ⇒ 0; the measured 1440/1137 overlap ⇒ 303;
clamped by available width; negative available ⇒ 0), the anchor styles (right/left/middle, zero reserve),
and then the real `Window` + store + `Pane`: flush at rest, `calc(var(--spacing-single) + 303px)` while a
dock column is published at x 1137 under a body whose right edge is 1440, flush again once it retracts.

```
$ bun x vitest run --config 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts -t "yields a window's right-edge chrome" --reporter=verbose
 ✓ |@semio-tech/ui-react| ../../🟦️.tsx > Shell components > yields a window's right-edge chrome to an open right-hand dock column instead of sharing it 187ms
      Tests  1 passed | 729 skipped (730)
```

### 2.4 Browser verdict — the hit test the brief asked for

`🗑️generated/wave-B49-lane-b.txt`, `--only=camera-gestures,projection-options` (the lane composition that
leaves Inspection + History open over the rail — the one B47 measured as OBSTRUCTED):

```
projection chrome geometry={"rail":[833,58,300,544], …,
  "panels":["panel-body-stack=[1137,26,300,67]@DIV#framework.panelTab.framework.panel.inspection:absolute/30", …]}
nudge …projection-orthographic-view select shape={"rect":[833,123,104,16],"pointerEvents":"auto","mine":true,
  "owner":"puzzle3d-main-perspective/puzzle3d-measure-projection-orthographic-view","hitRect":[833,123,104,16],
  "chain":["BUTTON#puzzle3d-main-perspective/puzzle3d-measure-projection-orthographic-view[select-trigger]", …]}
nudge … select opened=true waitedMs=1 triggerState=open options=["Plan","Top","Bottom","Front","Back","Left","Right"]
projection nudge … {"after":{…,"text":"Plan"},"waitedMs":3,"obstruction":null}
```

The rail moved out of the dock column (x 1130 → 833) while Inspection stayed open at x 1137;
`elementFromPoint` at the Projection trigger's centre IS the trigger (`mine: true`, `obstruction: null`), the
listbox opens in 1 ms and the draft flips to `"Plan"`. **A human can use the Projection select with
Inspection open.**

Still FAIL, and NOT layout: `projection-control-flips` / `projection-repaints-camera` remain red with
`publishedBefore="" publishedAfter="" draftText="Plan"` — the optimistic draft moves and the guest never
publishes the value. That is B45 §2 / B44's publication residual on the same control, untouched here.

Class: **product, fixed** (the obstruction). Desktop first, and mobile is explicitly excluded
(`🪟️Window/🟦️.tsx:207-211`): the mobile chrome docks ONE full-width panel sheet rather than a side column,
so there is no column to yield to and yielding to a sheet would push the rail across its own window. The
store stays subscribed unconditionally and only the resolved value is zeroed, so hook order never changes
when the layout flips.

---

## 3 `outliner-hide-applies` — the host carries the arguments; the guest's dispatch drops the command

### 3.1 The premise was false, and this is the line that settles it

A temporary trace at the single hop every row action passes through
(`🛠️ShellHelpers/🟦️.tsx:1991` `uiIntentToActionDescriptor`) was added, measured in
`🗑️generated/wave-B49-lane-a.txt`, and removed again:

```
[117.2s] outliner hide console tail=[
  "[DEBUG] uiIntent {\"action\":\"setSelectionFlag\",\"node\":\"seed-left-001\",\"trigger\":\"activate\",
                     \"args\":{\"entity\":\"object\",\"flag\":\"hidden\",\"ids\":[\"seed-left-001\"],\"value\":true},
                     \"payload\":{\"entity\":\"object\",\"flag\":\"hidden\",\"ids\":[\"seed-left-001\"],\"value\":true}}",
  "[DEBUG] performInvocation {\"invocationKind\":\"action\",\"instanceId\":1,\"actionId\":\"setSelectionFlag\"}",
  "[DEBUG] command ingress lane {\"instanceId\":1,\"actionId\":\"setSelectionFlag\",\"seq\":35,\"lane\":\"Interactive\"}",
  "[DEBUG] puzzle3d.utility.publish action=setSelectionFlag window=Some(\"puzzle3d-main-perspective\") utility= map_hit=false",
  "[DEBUG] command ingress settled status=command-complete observed=command-complete",
  "[DEBUG] performInvocation settled {…,\"actionId\":\"setSelectionFlag\",\"frames\":2,\"frameKinds\":[\"Invocation\",\"Ephemeral\"],
                                      \"historyCursor\":null,\"historyUpserts\":0,\"historyCanUndo\":null,\"effects\":0}"]
```

**The host sends `{entity, flag, ids: ["seed-left-001"], value: true}` verbatim.** B47 §6's two candidates
(the host's row-action dispatch dropping a binding's args, and the guest's `UiValue` arena publishing the
row action without them) are both excluded: the arena refusal path
(`📌️panels/🗿️artifact/🦀️.rs:161-173` `with_hide_lock_actions`) drops the whole toggle rather than its args,
so a Hide button that EXISTS carries them, and the host then hands them through untouched.

Battery #59's own selection state (`battery-2026-09-13-59-6013.txt:614-618`) is worth correcting too: the
selected object was `seed-left-001` ITSELF, not a sibling — `selectionBefore=["puzzle3d-main-top=seed-left-001",
"puzzle3d-main-perspective=seed-left-001", …]`. So neither branch of `set_selection_flag`
(`🎮️commands/🔖️set-selection-flag/🦀️.rs:21-41`) could write nothing: explicit ids name the row's object, and
the live selection holds the same object. Both branches would have mutated.

And B47 §6.4's discriminator did NOT ride #59: the served component
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🧩️puzzle/semio_s_plugin_puzzle_component.core.wasm`,
built 09-12 15:01) contains `puzzle3d.utility.publish` once and `puzzle3d.flag.explicit` / `flag.selection`
ZERO times. Their absence from #59's console is therefore not evidence about the branch — nobody should wait
on those taps until a wasm built after B47 is served.

### 3.2 Where the remaining half is

`historyUpserts: 0, effects: 0, command-complete` with the args present means the dispatch produced no
document delta, and `✏️editor/🦀️.rs`'s staged prologue has exactly three exits with that signature, all of
them BEFORE or AROUND the command arm:

- `:3442` `if self.refreshed(config, view_state, window_id).is_none() { return (Emit::default(), EphemeralEmit::default()); }`
- `:3446` `let Some(mut scene) = self.scene.take() else { return (Emit::default(), EphemeralEmit::default()); };`
- `:3459-3467` the `aborted` arm (returns effects only — and a refusal notice would have shown `effects: 1`).

Everything downstream of them is sound and already pinned: the arm takes the explicit branch whenever
`entity` and `ids` decode (B47's native law `an_explicit_outliner_flag_write_ignores_whatever_is_selected`),
and the delta bridge does emit a flag mutation (`🧬️schema/🧬️mutations/🦀️.rs:185-190`
`change_object_hidden` / `change_object_locked`, reached from `:3469-3474`
`puzzle3d_operations_from_fixture_change`).

This wave did not edit the guest: it is B48's live lane (the guest selection lane), one tap inside
`dispatch_step` decides which of the three exits fires, and that needs a wasm rebuild B48 owns anyway. To
re-arm the host-side half of the evidence in one line, put back at `🛠️ShellHelpers/🟦️.tsx:1992`:

```ts
console.warn("[DEBUG] uiIntent", JSON.stringify({ action: intent.action.name, node: intent.nodeKey, trigger: intent.trigger, args: intent.args, payload }));
```

### 3.3 Law — the host half stays true

`🔬️engine-contract/🟦️.ts`:
`carries an outliner row action's explicit ids to the action channel whatever else is selected` — the real
`uiNodeToTreePanelConfig` + `📃️UiDocumentStore` + `🗣️Interpreter` path, an authored `rowActions` binding with
`{entity, flag, ids, value}`, clicked once with no presence overlay and once under a
`UiPresenceOverlayContext` marking the row AND a sibling selected; both dispatch
`{controllerId: "puzzle3d-play", action: "setSelectionFlag", args: {entity, flag, ids, value}}`.

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config …/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts -t "carries an outliner row action" --reporter=verbose
 ✓ |@semio-tech/framework-renderer-react| …/🔬️engine-contract/🟦️.ts > s workflow flow routing > carries an outliner row action's explicit ids to the action channel whatever else is selected 80ms
      Tests  1 passed | 1038 skipped (1039)
```

Verdict: `outliner-hide-applies` still **FAIL** (`waitedMs=30229`, `worldHidden=[]`,
`wave-B49-lane-c.txt`), with the host exonerated by measurement and the guest exit localized to three lines.
Class: **product, guest-side, B48's lane.**

---

## 4 Verification — every command in the FOREGROUND, tails quoted

### 4.1 Host TypeScript

```
$ bun x tsc --noEmit -p 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json
exit=2 · 648 errors, all pre-existing in other packages (repo library/test modules, a peer's live area)
per-file count vs the B45 baseline (`🗑️generated/wave-B45-ui-react-typecheck.txt`):
  🖱️ui/🎯️targets/⚛️react/🟦️.tsx        now=0    baseline=0
  🪟️Window/🟦️.tsx                     now=0    baseline=0
  🖼️Panel/🟦️.tsx                      now=0    baseline=0
  🏛️ShellHost/🟦️.tsx                  now=0    baseline=0
  🐚️Shell/🟦️.tsx                      now=0    baseline=0
  🔬️engine-contract/🟦️.ts             now=0    baseline=0
  🧪️owned-locale-detector-retirement  now=153  baseline=153   (the suite's endemic implicit-any set)
```

Zero new type errors in every file this wave touched.

### 4.2 ui-react vitest lane (full)

```
$ bun x vitest run --config 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts
 Test Files  3 failed | 19 passed (22)
      Tests  14 failed | 717 passed (731)
```

No NEW failure. The 14:

- 12 × `Error: ENOENT: no such file or directory, open '…/🧰️framework/🎨️styling/🖌️ui/🎨️.css'` (and one
  `…/🛂️manifest/🧫️fixtures/🖱️tutorial-local-interaction.json`) — a peer is relocating the taxonomy live; the
  stylesheet now lives under `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`. Read-only fallout, untouched here.
- 1 × `UIDialog accessibility > keeps a nested owned kind picker focusable…` (an arg-payload assertion in an
  area this wave never touches).
- 1 × `owned Diagram force > cursorizes live controlled 20,000-node…` — `lastValidPublicationGeneration()`
  undefined; the suite renders no `Panel`/`Window`/`WindowChrome`/`Pane`/introduction (grep: zero matches),
  and both Diagram files were last modified 09-12 15:22, before this session.

The one failure this wave DID cause was its own dead assertion — `expect(body.className).toContain("z-[1]")`
in `control chrome > panel chip-cap…` — repaired in §1.6 to assert the stacking that now actually applies.

### 4.3 renderer-react vitest lane (`SEMIO_TEST_LEVEL=long`, includes `🔬️engine-contract`)

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config …/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts
 Test Files  1 failed | 32 passed (33)
      Tests  6 failed | 1033 passed (1039)
```

All 6 are `🧪️tests/🧩️package-integration > framework renderer wgpu generated worker` (the pinned-Bun /
byte-identical-worker family) — the same six listed in the ticket's own `🗑️generated/b34-vitest-full.txt`
baseline. Every `🔬️engine-contract` test passes, including this wave's two new laws.

### 4.4 Guest (Rust)

Not touched, so nothing rebuilt and no guest law added: §3 proves the host half by measurement, and the
remaining guest exit belongs to B48's live lane. B47's `puzzle3d.flag.*` taps exist in source and are absent
from the served wasm (§3.1) — the next plugin build carries them.

### 4.5 Browser lanes (all gate-checked, all under `🗑️generated/`)

| lane | `--only=` | what it settles |
|---|---|---|
| `wave-B49-lane-a.txt` | `window-content,outliner-rows,projection-options` | the `uiIntent` args trace (§3.1); veil still up after boot Skip with only the clamp in place |
| `wave-B49-lane-b.txt` | `camera-gestures,projection-options` | the Inspection/rail hit test, `mine: true`, `obstruction: null` (§2.4) |
| `wave-B49-lane-c.txt` | `window-content,outliner-rows,projection-options` | guard after the chrome-stacking change: `PASS=12 FAIL=4`, verdict-for-verdict identical to lane a, and NO `veil …` line anywhere |

```
$ bun .../🔍️browser-probe.ts --only=window-content,outliner-rows,projection-options --port=6013
[116.9s] battery PASS=12 FAIL=4 FAULTS=0 first-hard-fault-at=none guest-death-faults=0
```

The four reds in that composition are `projection-control-flips`, `projection-repaints-camera` (§2.4, guest
publication), `outliner-hide-applies` and `outliner-show-restores` (§3, guest dispatch).

---

## 5 Handover

- **Rides the next battery, no wasm needed:** the tour now dismisses on one plain Skip press (so lanes stop
  measuring through a scrim), and the Projection select is reachable with Inspection open.
- **Needs a wasm built after B47:** `puzzle3d.flag.explicit` / `flag.selection` (absent from the served
  component, §3.1). With them, one battery says whether `set_selection_flag`'s arm ran at all.
- **B48 (guest selection lane):** §3.2's three exits in `✏️editor/🦀️.rs:3442`, `:3446`, `:3459-3467` are the
  whole remaining surface for `outliner-hide-applies`; the host is measured clean and lawed.
- **Named, not chased:** the introduction box's `boxSize` state measured ~1585×1041 for a box that renders
  417×83 (that is WHY the centering went negative). The clamp makes it harmless, but the measurement itself
  (`boxRef` `offsetWidth/offsetHeight` under the portal host's first layout) is still wrong and worth a
  later look.
- **Also named:** `z-[n]` arbitrary Tailwind utilities do not reach the served stylesheet at all. This wave
  removed the four load-bearing ones inside `WindowChrome`; `⚛️react/🟦️.tsx` still carries others
  (`floatingPaneAsideClass`, `windowCapFrameClass`, `windowControlsCapClass`, `:10467`), and every one of
  them is currently a no-op. Worth one sweep by whoever owns the styling build.
