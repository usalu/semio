# 🪟️ W12d — Pane chip ids, fold effects and the projection toggle (family E)

Packet input: `📓️w11a-prepared-world-mesh-missing.md` §6 family E — `pane-chip-search-toggle`,
`pane-chip-windowcontrols`, `pane-chip-utilitybar-unfold`, `pane-chip-projection-toggle`, read as
"chip hit targets resolve `absent`/`suffix` on opposite sides; React's projection toggle closes three
surfaces wgpu leaves open".

**The id vocabulary was already one.** All five chips are minted with React's own spelling on both
renderers (`WindowPaneChip::control_id`, fixed by W9c §4) and the shared fixture pins them. What
actually differed in family E is four separate things, none of them an id: a pointer STACKING order
inverted against React, a shell overlay the Search chip raised, a phantom React control, and a
routing namespace the utility rail never had. One of the packet's premises — the projection toggle
closing three surfaces — is a misread of a single flapping React column and is disproven below.

---

## 0. What the runs actually record

`🗑️generated/w11a-parity-run-14/steps.json` is the only run where React's column moves for these
steps, and it is the one run where React was **falling apart mid-journey**: steps 8, 12, 13 and 15-17
report `ready: null`, `surfaces: 0` and a control count of `1` or `68` (a boot skeleton), against 337
for a live shell. The three surfaces "removed" by `pane-chip-projection-toggle` in that run are
`panel:framework.panel.catalogue`, `window:puzzle3d-main-perspective` and `window:puzzle3d-main-top`
— i.e. every surface the shell had, which is a remount, not a fold.

Ten stable runs record the same three steps identically (`parity-run-{1..10,12}`, `w5b-react-8`):

| step | React `resolved` | React Δ controls | React surfaces |
| --- | --- | --- | --- |
| `pane-chip-windowcontrols` | `suffix` | 337 → 337 (none) | none |
| `pane-chip-utilitybar-unfold` | `suffix` | 337 → 355 (**+20**) | none |
| `pane-chip-projection-toggle` | **`absent`** | 355 → 355 (none) | **none** |

So React's projection chip moves nothing at all, and the journey's key names no React control — which
is also what wgpu answers, so that step already MATCHED. See §5.

`🗑️generated/w12c-parity-run-15/` (2026-09-18T17:39Z) is unusable for the same reason in the other
direction: its React column reports `controls: 1` and `absent` for EVERY pane-chip step, i.e. the
reference never booted. The stable oracle for family E stays runs 1-12 plus `w5b-react-8`.

---

## 1. `pane-chip-search-toggle` (and `-engagement-toggle`): the press never reached the chip — on EITHER renderer

### Root cause

Not an id: both renderers resolve `framework.window.puzzle3dMainTop.search.toggle` by suffix, with
almost the same rect (React `[232, 64, 72, 22]`, wgpu `[238.6, 57.6, 60.3, 22.4]`). The press lands
somewhere else, differently on each side:

- **React** journals `addObjectKind` on BOTH the Actions and the Search chip step, in every run, and
  the folds never open (control count unchanged). `🗑️generated/parity-run-12/react/09-…png` shows why:
  the journey addresses `puzzle3dMainTop`, the LEFT pane, whose top-left and top-middle chips lie under
  the open **Catalogue panel**; the click hits the catalogue's `Hexagonal Cut Concrete Forest Right`
  row, which dispatches the guest's `addObjectKind` (the screenshot shows the new red slab in both
  panes). This is deliberate React behaviour: "window pane toggles … stay on their authored anchors
  behind anchored chrome panels — panels paint above `z-window` and occlude overlap without shifting
  pane chrome" (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx:200`), with
  `--z-window: 10 < --z-pane: 20 < --z-panel: 30` (`🖱️ui/🎨️styling/🖌️ui/🎨️.css:834-836`).
- **wgpu** answered the CHIP, because W9c deferred the chips' hit rows past every docked panel
  (`ShellChromeFramePhase::PaneOverlayHits` ran after `Panels`) on the reading that "React's `Pane`
  overlay is a layer above the panel". That reading is inverted. In `parity-run-12` the same press
  leaked further still and dispatched `interactionHover` into the world.

### Fix

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:17196` / `:17397` / `:17399` — the `PaneOverlayHits` phase now runs
**between** `MainWindow` and `Panels`. `InputState::hit_at` resolves in reverse registration order, so
that single move is React's whole three-level stack: a chip still outranks the window body and the
world surface it is painted over, and a docked panel floating over the chip now takes the pointer back.
The field docstring (`:3033`) carries the evidence and the React citation.

**React ref**: `🪟️Window/🟦️.tsx:200`, `🖱️ui/🎨️styling/🖌️ui/🎨️.css:834-836`.
**Law**: `a_pane_chips_hit_row_is_flushed_after_its_window_body_and_before_the_panels_that_occlude_it`
(`🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs:246`) — drives the real chrome walk from that phase,
asserts it advances to `Panels`, that the body row precedes the chip row, that a point on the chip
answers the chip, and that the same point answers a panel registered after it. This supersedes W9c's
`a_pane_chips_hit_row_is_registered_after_the_panels_it_must_outrank`
(`📓️w9c-behaviour-parity-run-2.md` §9 gate table).

### Residual (not fixed here, different lane)

An open panel's own BOX registers no hit row — only its retained content rows and its resize handle do.
So a press in a panel's PADDING over a chip still resolves the chip, where React's panel element takes
it. `ShellState::pointer_is_over_open_panel` already answers `Chrome` for that point, so it never leaks
to the world; making `hit_at` agree means the panel walk registering its own box (panel lane, not the
pane lane). Dropping the chip's row instead would make a partially covered — but visible — chip
unpressable, which is worse than the gap.

---

## 2. `pane-chip-search-toggle`: the chip raised a surface React does not have

### Root cause

`toggle_window_pane_chip`'s `Search` arm also set `search_open`/`overlay_state = OverlayState::Search`
— the SHELL's centred `⌘K` palette, the surface `ui.search.toggle` and the chord own. React's Search
chip only unfolds the window's own top-middle `Pane` (sharing `actionsFolded` with Actions) and focuses
its input (`setEngagementBarFolded`, `🪟️Window/🟦️.tsx:388`). One gesture, two different surfaces.

### Fix

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13849` — `toggle_window_pane_chip` is a fold toggle and nothing else:
`Actions | Search` share one arm, the overlay lines are gone, and the window activation moved to the
TOP of the function so every chip activates its own window, which is what React's
`GhostRegionShell onPointerDownCapture → onActivate` does for any pointer inside a window.

**Journal parity**: React journals NOTHING on a chip press (`actionCounts: {}` on the one chip step
that really lands, `pane-chip-utilitybar-unfold`, in all ten runs) — a fold is local UI state on both
renderers. No `noteShellCommand` is owed.

**Remaining gap (a surface, not a defect of this packet)**: wgpu paints **no Actions/Search pane body
at all**. `action_panel_folded` is written in four places and read only to pick the chip's id and
active flag; `render_engagement_input` no longer exists (only a docstring mentions it), and the chrome
walk's per-window children are `measures → chips → utilities`. So React's "window-scoped search palette
with React's control set" has no wgpu twin to census yet. Its control set also cannot be measured from
any run in this ticket, because the press never reached the chip on React either (§1) — the honest
input for that packet is a run taken AFTER this fix, with the catalogue panel closed.

---

## 3. `pane-chip-utilitybar-unfold`: the rail's rows were unroutable, and its census is React's minus DOM scaffolding

### Census

React adds exactly 20 rows (`parity-run-12`, `w5b-react-8`). Fourteen are `data-slot` scaffolding its
DOM census counts and no canvas renderer publishes (`window-chrome-body-surface` ×2, `utility-bar-body`,
`ribbon-zone`, `ribbon-group`, `ribbon-item`, `ribbon-row`, `toggle-group`, `window-chrome-cap`,
`window-chrome-gap`, `inline-label` ×4) — the same reading `📓️w9c` §3 established for the panels. The
six that NAME something are `framework.window.puzzle3dMainTop.utilityBar.fold` (the chip flipping),
the rail root `ui.utilities.puzzle3d-main-top`, and one row per leaf: `transform`, `brush`,
`volumeBrush`, `worldRelocate`. wgpu publishes the chip flip and one row per leaf, each carrying the
leaf's framework element id as its TAIL, and removes `…utilityBar.unfold` — one for one.

### Root cause (the real defect this step hid)

The rail's rows are painted INSIDE a world pane's rect and registered directly (not deferred), and
`ShellState::pointer_hit_owner` claims a press only by `HitKind` or by a chrome NAMESPACE. A utility
row is `HitKind::Toggle`/`Button` under `framework.utility.…`, which no clause matched — so every
press on `Transform`/`Brush` in a pane's rail was handed to `enqueue_world3d_event` and the utility
never armed. This is the same gap the pane chips had before W9c; it arrived when the rail moved off
the footer onto the pane.

### Fix

- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13649` — `WINDOW_UTILITY_RAIL_PARENT` names the rail's routing
  namespace once, with the reason it exists (React needs none: its rail is DOM above the canvas).
- `:9733` — `pointer_hit_owner` claims it, beside `framework.window.` and
  `framework.worldOrbit.projection.`.
- The three registration sites mint through the constant instead of three literals.

**Why the ids keep a namespace**: React names a leaf by the bare utility id (`id={entry.id}`,
`🎛️UtilityTree/🟦️.tsx:237`) because an introduction step addresses a utility that way
(`introductionUtilityId`, `🏛️ShellHost/🟦️.tsx:9593`), and this renderer already publishes that exact
element id through `ShellChromeBuildState::register_element_rect`. A hit's `control_id` is this
renderer's ROUTING key, not its element id, and a bare `transform` is unclaimable by any prefix rule —
so the rail keeps `framework.utility.<kind>.<elementId>`, whose tail resolves the same leaf by suffix
on both sides. (React's bare ids are additionally duplicated across window instances — two unfolded
panes render `transform` twice, the invalid-HTML defect `🌐️World3dHost/🟦️.tsx:5214` documents for the
projection pane. Unifying that means instance-scoping React's leaves plus an element alias for the
introduction anchors; it is a framework-contract change, not a W12d fix.)

**Laws**: `unfolding_the_utilities_chip_publishes_reacts_own_rail_census`
(`🪟️wgpu-window-pane-chrome/🦀️.rs:498`) and
`a_pane_utility_row_is_claimed_by_the_shell_and_not_by_the_world_under_it` (`:562`).

---

## 4. `pane-chip-windowcontrols`: React published an empty control group

### Root cause

React's `framework.window.<segment>.windowControls` measured `[524, 65, 2, 16]` — two pixels wide, no
items, no action, no surface change, in every run. `Window` minted the `ActionGroup` whenever
`showControls` was true, and `ModeWindowDescriptor` OMITS `onClose`/`onMaximize`/`onMinimize`/
`onOpenInNewWindow` (`🎨️Canvas/🟦️.tsx:75`) while `🏛️ShellHost/🟦️.tsx` passed `showControls: true` for
every window — focus and close live on the dock TAB (`mode-dock-tab-focus` / `mode-dock-tab-close`).
So the shell published a control id that names nothing; the OS dev catalogue smoke test already
filtered it out by name (`💻️os/🧑‍💻dev/🧪️tests/🔬️catalog-smoke/🟦️.ts:163`). There was no "missing wgpu
publication": there was a phantom React one.

### Fix

`🖱️ui/🧱️elements/🪟️Window/🟦️.tsx:277` — the group is minted only when it carries a control
(`hasWindowControls`), and the now-meaningless `showControls` prop is deleted from `WindowConfig`, from
the component, from the two stories and from the three `ShellHost` descriptors. Hosts that DO hand in a
callback (the dev catalogue) are unaffected — every item was already conditional on its own callback.

**Laws**: `Window mints no window-control group when it is handed no window control` and
`Window mints the group again as soon as one control is handed to it`
(`🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`, beside the existing mobile/label pins), plus
`no_window_publishes_an_empty_window_control_group` (`🪟️wgpu-window-pane-chrome/🦀️.rs:582`) which pins
the wgpu side to the same fixture rule. Both renderers now resolve this key `absent` — one vocabulary,
one answer.

---

## 5. `pane-chip-projection-toggle`: already matched; the "three surfaces" were a remount

React spells this chip `framework.worldOrbit.projection.<segment>.pane.fold` — `Pane`'s default
`childElementId(id, "pane", "fold")` with no explicit `toggleId` (`🌐️World3dHost/🟦️.tsx:5218` +
`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`'s `chromeToggleId`), and wgpu mints exactly that string. The journey's
key is `projection.toggle`, which is no control on EITHER renderer, so both answer `absent` and the
step matches. The same is true of `windowOptions.toggle` (React spells it `measures.{unfold,fold}`).

**No renderer change is owed.** What IS owed is one line in the journey, in
`🐍️parity-interact-probe.mjs`'s `PANE_CHIPS`: `projection.toggle` → `pane.fold` and
`windowOptions.toggle` → `measures.unfold`, so those two steps measure a control instead of measuring
their own key. The probe is W12c's file and is in flight, so it is NOT edited here —
**W12c: please make that change; the step names derive from the key, so keep the display names if the
run tables should stay comparable.**

### The fold contract, pinned

React gives every `Pane` its own `useState(true)` (`🪟️Window/🟦️.tsx:169`/`:176`/`:183`,
`🌐️World3dHost/🟦️.tsx:5225`) and couples exactly two of them (Actions ↔ Search). So:

- folds are independent; no chip closes another pane's rail, and none closes a sibling WINDOW's,
- no chip opens or closes a surface,
- no chip raises a shell overlay.

`one_pane_chip_press_flips_one_fold_and_moves_no_surface` (`🪟️wgpu-window-pane-chrome/🦀️.rs:451`)
asserts all three per chip against `chrome_surface_census`, and the fixture states them in
`paneFolds` so the oracle is readable without the Rust.

---

## 6. Files

| file | change |
| --- | --- |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | `:3033` field doc · `:9733` predicate claims the rail · `:13649` `WINDOW_UTILITY_RAIL_PARENT` · `:13849` `toggle_window_pane_chip` · `:17196`/`:17397`/`:17399` phase order |
| `🐚️Shell/🧫️fixtures/🪟️window-pane-chrome/🔣️.json` | `paneFolds`, `utilityRailCensus`, `windowControls` blocks |
| `🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs` | 4 new laws, 2 rewritten |
| `🖱️ui/🧱️elements/🪟️Window/🟦️.tsx` | empty control group removed, `showControls` deleted |
| `🖱️ui/🧱️elements/🪟️Window/📖️stories/🧪️.story.tsx`, `🏛️ShellHost/🟦️.tsx` | `showControls` call sites |
| `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` | 2 new React laws |

## 7. Verification

| command | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | ✅ clean | — |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 window_pane_chrome -- --test-threads=1` | ✅ 15/15 | — |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 -- --test-threads=1` | 897 passed, **2 failed (both pre-existing, neither mine)** | `🗑️generated/w12d-renderer-tests.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | ✅ 0 errors | `🗑️generated/w12d-wasm-check.txt` |
| `bun ./📜️script.ts test -t "window-control group" --run` (ui-react) | ✅ new law passes | — |
| `bun ./📜️script.ts test -t "Window" --run` (ui-react) | 66 passed, **2 failed (pre-existing)** | — |

Pre-existing failures, unchanged by this packet:

1. `async_boundary_tests::presenter_ack_retirement_source_mutations_are_denied` — the stale
   source-text contract `📓️w11a` §7 already reported.
2. `scenes::admitted_surface_map_tests::admitted_surface_slot_tables_are_heap_first…` — a peer's
   in-flight `World3dState` growth (`element_bytes` 22752 → 22920); nothing here touches that struct.
3. React `Window merges the ad-hoc actionPane…` — **proved** pre-existing: restoring the exact
   pre-change control-group behaviour behind a temporary `true ||` still fails it (the second toggle
   click does not re-fold), and the failure is in the fold lane, not the controls lane.
4. React `WindowChrome stamps data-dim on the body glass surface` — renders `WindowChrome` directly,
   never `Window`.

Live confirmation of §1-§4 is W12c's next parity run. Expect: `pane-chip-{engagement,search}-toggle`
to journal what React journals for a press that lands on the Catalogue panel (and NOT to leak
`interactionHover` into the world), `pane-chip-windowcontrols` to read `absent`/`absent`, and
`pane-chip-{projection,windowOptions}-toggle` to keep matching — measuring a real control only once
the two journey keys are respelled (§5).
