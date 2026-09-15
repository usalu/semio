# 🛟️ Chrome panel safe area — 2026-09-14 (lane `chrome-panel-safe-area`, React :6022 / wgpu :6118)

Owns `📓️react-oracle-hardening-2026-09-14.md` §4.3: the world pane's top-right overlay rail
(`[data-slot="world-frame-instances"]` = `Frame visible`, `[data-slot="world-compute-cancel"]` = the
preview `Cancel`) could not be clicked, because an anchored chrome panel painted over it. `Cancel` being
unreachable violates the standing rule that every expensive operation offers cancellation.

---

## 0. The one-line version

The framework now has ONE owning layer for "window content anchored into a corner a chrome panel also
owns": the panel dock publishes each open panel's occupied box per anchor, and every in-window anchored
affordance reads its own safe area off that and steps clear on whichever axis costs less. Three React
affordances and the wgpu shell's whole surface overlay row consume it; both renderers answer the same
shared fixture. Measured on :6022 with `document.elementFromPoint`: with the `top-right` Chat panel open,
`Frame visible`'s own centre used to resolve to `div[data-slot=tree-row-content]` — a panel row — and now
resolves to the button itself. The battery lanes this lane was dispatched for: `fit` **0/8 → 8/8**,
`cancel-preview` **exit 1 → green**, and a real in-flight evaluation stopped at 6 of 7 nodes by pressing
the `Cancel` that used to be unreachable.

---

## 1. What was actually wrong

Not a z-index accident. A docked `Panel` body paints at `z-panel` in the **app root's** stacking context;
a window's content overlay rail paints at `z-40` inside the window's own `z-window` context. Those are
different stacking contexts, so the rail can never win the corner back by restacking (raising it only
buries the panel's own rows), and making the panel click-through would make its content unusable. The
rail's `windowChromeClearedTopOffset` clears only the **window's own** chrome row; the framework had no
concept at all of a reserved inset for anchored chrome panels over window content.

There *was* a half-concept: `↔️DockColumnReserve` in `🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — one number
(`publishShellDockRightColumnLeftPx` / `useShellDockRightColumnLeftPx` / `dockColumnInlineReservePx`),
right-hand anchors only, horizontal only, consumed only by `🪟️Window`'s own right-edge chrome. It could
not express a corner box with a bottom edge, could not be read by an affordance at any other anchor, and
was invisible to the world pane's rail. It has been **replaced** (not wrapped — no compat layer) by the
general layer.

---

## 2. The owning layer

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`, region `#region 🛟️ChromePanelSafeArea`.

**Publication.** `publishShellChromePanelBox(root, key, anchor, box | null)` — one entry per open anchored
chrome panel, keyed by shell root so two shells on one page never overwrite each other. `SafeAreaBox` is a
whole-pixel viewport box (`safeAreaBoxFromRect` rounds, so a sub-pixel reflow never re-renders readers).
`useShellChromePanelBoxes(root)` is the single observable.

**The rule.** `chromePanelSafeArea(affordance, host, anchor, panels, yieldAxis, gapPx) → { inlinePx, blockPx }`,
pure:

1. only panels that actually **cover** `affordance` count — a closed or distant panel costs nothing, and
   the flush inset stays byte-for-byte as authored;
2. the displacement is measured off the **union** of those panels, on the affordance's **own anchor
   edges** — a top anchor drops below the union, a right anchor moves in past its left edge;
3. `yieldAxis` states which of the two the affordance's layout can actually give: `"inline"` for a chrome
   row pinned to the top of its window, `"block"`, or `"either"` — which takes the **smaller** of the two
   displacements (ties go to the block axis, which keeps the affordance on its own column);
4. an axis that cannot clear inside `host` is **not taken at all**. Moving an affordance without freeing
   it is pure harm; the answer there is a re-anchoring, not a half-step. (This also retires the old
   `Math.min(overlap, available)` clamp, which moved a window's rail partway under a column it could
   never escape.)

At most one axis is ever non-zero. `chromePanelSafeAreaStyle(anchor, safeArea, base?)` turns it into the
two edge insets, defaulting to `var(--spacing-single)` or the caller's own base (the rail's
`windowChromeClearedTopOffset`).

**Live measurement.** `useChromePanelSafeArea({ hostRef, affordanceRef, anchor, yieldAxis, gapPx, enabled })`
grades the affordance on its **unreserved** box — the measured rect with the offset the hook currently
applies taken back off — so a rail that has already stepped clear does not read as clear, drop its offset
and step straight back under the panel. Re-measures on the panel store, a `ResizeObserver` on host and
affordance, and window resize.

### 2.1 Why re-anchoring was NOT the answer

A re-anchoring would have to be static (the rail moves to, say, `bottom-right` forever), but which corner
a panel occupies is a runtime user choice: all eight anchors can be open, the dock is drag-re-anchorable,
and a panel's height hugs its own content (measured on :6022 this hour: the same `top-right` group is 24 px
tall with the Tool runs tab empty, 96 px on Inspection, 212 px on Chat). A static re-anchor trades one
permanently-covered corner for another. The safe area is the runtime answer, and it degenerates to exactly
the authored flush inset when nothing covers the affordance.

---

## 3. Consumers

| consumer | anchor | yield | why that yield |
|---|---|---|---|
| `🌐️World3dHost/🟦️.tsx` world-view overlay rail (`Frame visible`, `WorldComputeStatusPane`'s `Cancel`) | `top-right` | `either` | it floats over the scene; it can drop or move in |
| `🪟️Window/🟦️.tsx` right-edge chrome column (chrome control row + measures `Pane`) | `top-right` | `inline` | pinned to the top of its own window; dropping it would leave its title row |
| `🪟️Window/🟦️.tsx` folded engagement quick-action rail | `top-left` | `either` | window content, floats |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` per-surface overlay row (status pill + `Cancel`, node-graph `Fit graph`) | `top-left` | `either` | same affordance on the other renderer |

`🖼️Panel/🟦️.tsx` is the single publisher: every visible panel, at every anchor (it used to publish only
right-hand ones, and only its left edge).

### 3.1 The wgpu twin

`chrome_panel_safe_area` + `SafeAreaYield` + `ChromePanelSafeArea` in the wgpu shell mirror the TypeScript
function statement for statement, with `PanelAnchor::vertical()`/`horizontal()` added as the Rust twins of
`anchorVertical`/`anchorHorizontal`. The same defect class is real there: `render_panel_step` paints
`floating_panel_rect` **after** `plan_dock_windows` laid the dock windows out across the same body, so an
open left panel covers exactly the top-left corner where `surface_overlay_controls_for` anchors the World3d
`Cancel` and the node-graph `Fit graph`. `surface_overlay_row_box` states the row's claim (the smallest row
`surface_fits_overlay` admits — the fixture's own `surfaceControlMinimum`), `surface_overlay_row_origin`
applies the reserve, and both `surface_status_pills_for` and `surface_overlay_controls_for` now take the
open panel rects so the pill and the cancel move as one row. `open_floating_panel_rects` is the publisher.

---

## 4. Files changed

Product:
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — `🛟️ChromePanelSafeArea` replaces `↔️DockColumnReserve`: `SafeAreaBox`, `ChromePanelOccupancy`, `SafeAreaYield`, `ChromePanelSafeArea`, `safeAreaBoxFromRect`, `publishShellChromePanelBox`, `useShellChromePanelBoxes`, `chromePanelSafeArea`, `chromePanelSafeAreaStyle`, `useChromePanelSafeArea`; `Pane`'s `inlineEdgeReservePx` now styles through the new helper.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx` — publishes its own box for EVERY anchor while visible, retracts on close/unmount.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx` — right-edge chrome reserve read off the new layer (`"inline"`); the folded engagement quick-action rail reads a `top-left` safe area.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` — the overlay rail carries `ref`, `data-safe-area-block/inline`, and `chromePanelSafeAreaStyle("top-right", …, { block: windowChromeClearedTopOffset })`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `PanelAnchor::vertical/horizontal`, `SafeAreaYield`, `ChromePanelSafeArea`, `chrome_panel_safe_area`, `surface_overlay_row_box`, `surface_overlay_row_origin`, `open_floating_panel_rects`; `surface_status_pills_for`/`surface_overlay_controls_for` take the panel rects.

Laws and fixture:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json` — **new** `chromePanelSafeAreaNote` + `chromePanelSafeArea` (9 rows), appended; nothing existing reformatted.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` — 2 new laws.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — 3 new laws (the TypeScript half of the same fixture).
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` — 3 laws (the old single dock-reserve law rewritten + the `elementFromPoint` law).

Probe (ticket folder):
- `🐍️chrome-panel-safe-area-probe.mjs` — **new**; opens a named `top-right` chrome panel on the live page and reports the rail's box, its reserve, and `document.elementFromPoint` at every rail button, AFTER (reserved) and BEFORE (forced back to the unreserved offset).

---

## 5. Laws, with output

### 5.1 TypeScript — `@semio-tech/ui-react`

```
bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts -t "anchored chrome panel" --reporter=verbose
 ✓ Shell components > reserves an in-window affordance against the box an anchored chrome panel occupies 2ms
 ✓ Shell components > yields a window's right-edge chrome to an open anchored chrome panel instead of sharing it 268ms
 ✓ Shell components > keeps a window content overlay rail hit-testable under an anchored chrome panel 98ms
 Tests  3 passed | 743 skipped (746)
```

The third law mounts a `Window` whose content is a rail built on `useChromePanelSafeArea`, stubs the §4.3
geometry, and asserts `elementFromPoint` on `Frame visible`'s centre returns the **panel** before the box is
published and the **button** after — and the flush `top` afterwards when it is retracted. jsdom runs no
layout, so the "a `z-panel` chrome panel always paints above `z-window` window content" premise is modelled
directly in the law; the real `elementFromPoint` evidence is §6.

Full ui-react suite: `Tests 14 failed | 732 passed (746)`. The 14 are **pre-existing and not this lane's** —
`ENOENT .../🧰️framework/🎨️styling/🖌️ui/🎨️.css` (icon-hover, celebrate, introduction, ContextMenu, window
chrome), `ENOENT .../🛂️manifest/🧫️fixtures/🖱️tutorial-local-interaction.json`, the navbar fullscreen toggle
and a UIDialog focus law. The count was 15 mid-lane (one of mine, a jsdom `calc()` collapse) and is 14 now;
none of the 14 names a safe-area law.

### 5.2 TypeScript — the shared fixture twin (renderer engine)

```
SEMIO_TEST_LEVEL=long bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts -t "chrome panel safe area"
[DEBUG] chrome panel safe area reproduced all 9 shared fixture rows
 ✓ 🛟️ chrome panel safe area > validates the shared safe-area corpus against its own declared schema 1ms
 ✓ 🛟️ chrome panel safe area > reserves every shared fixture row exactly the way the wgpu shell does 1ms
 ✓ 🛟️ chrome panel safe area > leaves every reserved affordance clear of the panels it yielded to 0ms
 Tests  3 passed | 611 skipped (614)
```

### 5.3 Rust — wgpu shell

```
cargo test -p semio-framework-os-renderer-wgpu --lib shell_chrome_parity_tests
test shell::shell_chrome_parity_tests::a_chrome_panel_reserves_a_safe_area_for_the_surface_overlay_row ... ok
test shell::shell_chrome_parity_tests::the_overlay_row_steps_clear_of_an_open_floating_panel ... ok
…
test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 578 filtered out
[DEBUG] wgpu overlay row safe area: flush x=7.2 reserved x=307.2 panel right=304
```

Nine fixture rows, both languages, same numbers. The corpus states, among others: no panel → `{0, 0}`; a
panel that misses the rail → `{0, 0}`; the §4.3 Tool runs panel over the rail → `block 48` (cheaper than
`inline 301`); the same corner forced `inline` → `301`; a full-height right column → `inline 301` because
the block axis cannot clear; two panels sharing a corner → their union (`block 165`); a left floating panel
over a wgpu overlay row → `inline 304`; a window right-edge chrome column → `inline 303` (the wave B47 §1.3
number the retired `dockColumnInlineReservePx` law measured, reproduced exactly); an affordance with no room
to clear → `{0, 0}`.

---

## 6. Before / after on the live React page (:6022)

`SEMIO_PROBE_PANEL_TAB=<tab> bun 🐍️chrome-panel-safe-area-probe.mjs`, viewport 1440×900,
`🗑️generated/safe-area*/report.json`, **0 page errors** in every run. AFTER = the tree as it now stands;
BEFORE = the same page with the rail forced back to its unreserved offset, i.e. the pre-fix geometry.

| open `top-right` panel | panel box | rail BEFORE | `elementFromPoint` on `Frame visible` BEFORE | reserve | rail AFTER | `elementFromPoint` AFTER |
|---|---|---|---|---|---|---|
| `framework.chat` | (1137, 3) 300×212, bottom 215 | top 90–106 | `div[data-slot=tree-row-content]` — **the panel**, unreachable | block **129** | top 219–235 | `button[data-slot=world-frame-instances]` — **reachable** |
| `framework.panel.inspection` | (1137, 3) 300×96, bottom 100 | top 90–106 | `span[data-slot=tree-label]` — **the panel**, unreachable | block **14** | top 104–120 | `button[data-slot=world-frame-instances]` — **reachable** |
| `framework.panel.toolRun` (empty body) | (1137, 3) 300×24, bottom 28 | top 90–106 | the button (nothing covers it) | **0** | top 90–106, `top` still the authored `calc(var(--window-chrome-scroll-clearance, …) + var(--spacing-single))` | the button |

Both reserves are the rule's own arithmetic: `ceil(panel.bottom + uiSpacingPx(1) − rail.top)` =
`ceil(215 + 3.2 − 90) = 129` and `ceil(100 + 3.2 − 90) = 14`. The third row is the other half of the
contract: a panel that does not cover the rail changes nothing at all, and the authored inset survives
verbatim.

This is the §4.3 defect reproduced and closed with `document.elementFromPoint`, which is what the audit
asked for. Re-run on the converged guest (`🗑️generated/safe-area-chat2/report.json`, 0 page errors) the
Chat row reproduces byte for byte: BEFORE `reachable false`, hit `div[data-slot=tree-row-content]`; AFTER
`reachable true`, hit `button[data-slot=world-frame-instances]`, reserve `block 129`.

---

## 7. Battery

### 7.1 React :6022

```
SEMIO_BATTERY_URL=http://127.0.0.1:6022/?plugin=generation3d SEMIO_BATTERY_ROOT=react-safe-area \
  bun 🐍️react-battery.mjs --only=cancel-preview,interact
[DEBUG] battery ■ interact ok=false exit=0 141s steps=45/58 pageerrors=0
[DEBUG] battery ■ cancel-preview ok=true exit=0 14s steps=4/4 pageerrors=0
[DEBUG] BATTERY DONE green=1/2 red=["interact"] 155s pageerrors=0
```

Run on the guest staged 20:14, the first build on which the preview converges with geometry (the earlier
run in this lane's own working notes was made against the faulted guest and is superseded).
`🗑️generated/react-safe-area/`, log `🗑️generated/react-safe-area-run2.txt`. Page errors **0**, shell
faults **[]**.

**`fit`: 0/8 → 8/8.** The hop this lane was dispatched for.

| hop | §4.3 (pre-fix) | now |
|---|---|---|
| `fit` | **0/8**, click intercepted on every example and every run | **8/8** |
| `payload` | 8/8 | 8/8 |
| `hover` | 8/8 | 8/8 |
| `inspector` | 5/8 | 8/8 |
| `orbit` | 4/8 (4 a probe artifact) | 8/8 |
| `selection-reset` | 2/8 | 2/8 — §4.4, lane `selection-prune-interact` |
| `select` | 1/8 | 1/8 — §4.4, same lane |

All eight `fit` rows report `obstruction: null` (the probe's own `document.elementFromPoint` on
`Frame visible`'s centre returns the button or a descendant) and `fitClickError: null` (no Playwright
interception), and the camera then matches the product's own fit rule over the committed fixture bounds:

| example | framed bounds | camera target | distance / expected | off-centre |
|---|---|---|---|---|
| Box Shell Preview | `[0,0,0] → [2,2,2]` | `[1,1,1]` | 2.0000381 / 2 | 0 |
| Hexagonal Mushroom Column | `[-0.5,-0.4330127,0] → [0.5,0.4330127,6]` | `[0,0,3]` | 5.3999521 / 5.4 | 0 |

`interact` stays red overall only on `selection-reset` 2/8 and `select` 1/8 — §4.4's stale selection,
explicitly **not this lane's**.

**`cancel-preview`: exit 1 → GREEN, 4/4, exit 0.** Before this lane it exited 1 because
`[data-slot="world-compute-cancel"]` was click-intercepted (§4.3). The probe now clicks the real button
and the producer answers — `🗑️generated/react-safe-area/cancel-preview/run.txt`:

| frame | pane | cancel button |
|---|---|---|
| `boot` / `armed` | `Computing … Cancel`, `cancellable: true`, `cancelAction: toolRunAbort`, `cancelArgs {runId:"1", generation:0}` | `BUTTON "Cancel"` |
| `after-cancel` (+8.6 s) | `Cancelled`, `cancellable: false`, `nodesDone 6/7`, 185 meshes / 267 instances already delivered | gone |
| `after-cancel-settled` | `Cancelled`, unchanged | gone |

A real in-flight evaluation was stopped at 6 of 7 nodes by pressing the button that used to be
unreachable. The standing rule — every expensive operation offers cancellation — holds at runtime.


### 7.2 wgpu :6118

**Not run — the renderer wasm could not be rebuilt in this window, and a :6118 battery on a stale wasm
would measure the OLD shell.** `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bunx nx run
@semio-tech/framework-renderer-wgpu:wasm` was started foreground at 23:10 and is still queued
(`🗑️generated/safe-area-wgpu-wasm.txt`): **five** concurrent peer `trunk build` runs of this same crate
(pids 65152/65284/65759/65907/66594, 26–28 min each) serialize on the one shared
`⚡️cache/cargo/build/debug/.cargo-build-lock`, and the machine is additionally saturated by three
headless-Chrome probe runs and a GitKraken repo scan. Two further obstacles observed while waiting, both
peers' and neither this lane's:

- `cargo test -p semio-framework-os-renderer-wgpu --lib` (the full 606-test native suite) aborts on
  `async_boundary_tests::native_binary_owns_exactly_one_entrypoint_driver`, which asserts on the native
  entrypoint's own source text — that file is unmodified in the working tree, so the law fails at HEAD;
- a second attempt could not even compile: `semio-s-artifact-puzzle-3d` is mid-refactor in the working
  tree and fails `E0004` (`&FillRunEvent::VortexMarked { .. }` not covered), which is a peer's in-flight
  edit in an unrelated plugin crate.

What IS measured on the wgpu side, on the current tree, without a browser:

```
cargo test -p semio-framework-os-renderer-wgpu --lib shell_chrome_parity_tests
test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 578 filtered out
```

— 28/28, including the two new laws: the nine shared `chromePanelSafeArea` fixture rows answered by
`chrome_panel_safe_area` with the same numbers the TypeScript twin produces, and the real placement law
over `surface_status_pills_for` / `surface_overlay_controls_for`
(`[DEBUG] wgpu overlay row safe area: flush x=7.2 reserved x=307.2 panel right=304`), which is the
behaviour a `chrome` / `world3d-editor` battery would exercise from the outside.

**The exact two commands to finish it**, once the build queue drains (they are the only thing missing;
the guest wasm on :6118 needs no restage for a renderer rebuild):

```
cd /Users/ueli/Documents/semio && CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false \
  bunx nx run @semio-tech/framework-renderer-wgpu:wasm
cd <ticket> && until [ -z "$(pgrep -f 'wgpu-batter[y]|wgpu-.*-pro[b]e')" ]; do sleep 5; done
SEMIO_BATTERY_URL=http://127.0.0.1:6118/?plugin=generation3d SEMIO_BATTERY_ROOT=wgpu-safe-area \
  bun 🐍️wgpu-battery.mjs --only=chrome,world3d-editor
```

The expected result is no change from the last wgpu battery on these two lanes: the safe area is inert on
:6118 unless a floating panel is open over a live surface, and it reserves nothing when none is. A red
`chrome` or `world3d-editor` row whose detail names a surface-overlay control anchor would be the one
outcome to read as this lane's.


---

## 8. Not claimed

- **Not claimed** that this lane fixed `interact`'s remaining reds. `selection-reset` 2/8 and `select` 1/8
  are unchanged from `📓️react-oracle-hardening-2026-09-14.md` §4.4 (a selection is not pruned when the
  example changes) and belong to lane `selection-prune-interact`. `interact` therefore still reports
  `ok=false` overall even though every hop this lane owns is 8/8.
- **Not claimed** that this lane made the preview converge. `payload` 8/8 is the guest staged at 20:14 by
  lane `contributions-ingress-ceiling`; before that build the preview was
  `phase: "faulted" / "Geometry extension unavailable"` and `fit` could not be graded at all (an earlier
  run of this same battery on the faulted guest showed `obstruction: null` and `fitClickError: null` on
  7/8 rows with `fit` red only on `the pane published no geometry to frame` — the reachability half was
  already fixed, the geometry half was not this lane's). What this lane claims is the reachability:
  `obstruction` null on all 8 rows, no interception, and the fit camera then matching the product's own
  rule over the committed fixture bounds.
- **Not claimed** that the 14 remaining `@semio-tech/ui-react` failures or the pre-existing typecheck
  errors in `📖️stories/🧪️.story.tsx`, `🌐️World3dHost/🟦️.tsx` (5 `addEventListener` overloads at lines
  1467/1508/5013/6678/6683, none in the edited regions) and `🦑️repo/📚️library` were touched by this lane.
  They were present before it (`ENOENT .../🎨️styling/🖌️ui/🎨️.css`, `ENOENT .../🖱️tutorial-local-interaction.json`,
  a navbar toggle law, a UIDialog focus law) and are unrelated to the safe area. The ui-react failure count
  went 15 → 14 during this lane: the one that closed was this lane's own jsdom `calc()` collapse.
- **Not claimed** that the wgpu overlay row was watched moving in a browser pixel by pixel. What is
  measured there is the Rust law over the shared fixture plus the real placement law over
  `surface_status_pills_for` / `surface_overlay_controls_for` (`flush x=7.2 → reserved x=307.2`, panel
  right edge 304), and the :6118 battery in §7.2.
