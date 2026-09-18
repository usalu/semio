# W2k — puzzle 5D, RTL flow direction, chrome polish, one positioner, mono from tokens, icon re-raster, dispatch semantics, dead-code decision

Packet W2k of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`: the bundle of small/medium parity items
left by Wave 1 — audit §7's `P-puzzle5d`, `P-rtl-flow-direction`, `P-chrome-polish`,
`P-shell-dead-code-cleanup`, `P-keyvalue-react-gap`, plus W1o §5 gaps 2/3/4/5, W1n gap 6, W1k gap 1,
W1g gap 1 and W1m gaps G1/G3.

**Crate stability for the coordinator: `semio-framework-ui` (`--features wgpu-engine`) and
`semio-framework-os-renderer-wgpu` both compile clean, native and `wasm32-unknown-unknown`, as of the
last edit in this packet. The activation build can be restarted.** The two mid-packet breakages the
coordinator hit (`windows.get(SurfaceId)`, `no field muted on ChromePalette`) were transient states
inside single edits; from that report onwards every item was gated with
`cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` before the next one started
(logs `🗑️generated/w2k-ui-check-{2..8}.txt`, `w2k-os-check-{1..5}.txt`).

---

## 1. One positioner, in the contract (W1n gap 6)

W1n re-ported React's `resolvePopoverPlacement` into the wgpu target but could not touch the second,
already-diverged copy at `🖌️render/🖱️dispatch/🦀️.rs:513-620`, which still carried a three-variant
`BelowAnchorWithFlip`/`AtPointer`/`Centered` model. The two live in different crates
(`semio-framework-ui` and `semio-framework-ui-render`) with no edge between them, so "single
implementation" meant moving the math to the one crate both depend on.

New: **`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪟️overlay/🦀️.rs`** (241 lines), mounted at
`🧬️contract/🦀️.rs`. It owns `OverlaySide` (+ `opposite`, `transform_origin`), `OverlayAlign`,
`AnchoredPlacement` (+ `POPOVER`/`TOOLTIP`/`MENU`), `OverlayPlacement`, `ResolvedOverlayPlacement`,
`OverlayRect`, `OverlayKind` (6 variants), `DismissPolicy`, `TOOLTIP_DWELL_SECONDS`,
`TOOLTIP_HOVER_OUT_SECONDS`, `resolve_anchored_placement`, `resolve_centered_placement`,
`resolve_overlay_placement`, `resolve_select_inline_left`.

| Site | Before | After |
|---|---|---|
| `🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:672-745` | its own copy of all eight types + the math | `pub use ui_contract::{…}` at `:672-676`; `resolve_anchored_placement` at `:806` is a 1-line shim over the contract, `overlay_rect` at `:801` is the ONE `Rect`→`OverlayRect` conversion |
| `🖌️render/🖱️dispatch/🦀️.rs` | `OverlayKind`(5)/`OverlayPlacement`(3)/`DismissPolicy` + `resolve_overlay_placement` (diverged) | `pub use ui_contract::{…}` at `:496-502`; `resolve_overlay_placement`/`_side` at `:563`/`:571` forward to the contract. Only `OverlayAnchor` stays local (it names this module's `ElementId`) |
| os `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:18056, :18077` | 5-arg call | 6-arg, `ui_contract::FlowInline::Ltr` (shell root is React's default flow) |

React ref: `🧱️elements/🗨️Popover/🟦️.tsx:216-259`. The clamp is React's
`Math.min(Math.max(…))` verbatim; the flip-only-if-the-flip-fits rule is preserved.

## 2. RTL flow direction (audit §7 `P-rtl-flow-direction`)

`⚡️events/🦀️.rs:877` used to self-document the gap ("RTL mirroring is not applied — this target
carries no per-window flow direction yet"). It was the only `rtl` mention in the whole wgpu target.

**Contract vocabulary** — `🧬️contract/📐️layout/🦀️.rs`, new region `🧭️Flow`: `FlowInline{Ltr,Rtl}`
(+ `is_rtl`, `inline_sign`), `FlowBlock{Down,Up}` (+ `is_reversed`), `UiFlow{inline,block}`
(+ `DEFAULT`, `merged`, `for_anchor`). Twins of React's
`🔨️modules/🧭️flow-direction-context/🟦️.tsx:14,17,20-23,35,41-44` and — the important one —
`UiFlow::for_anchor` is `flowFromAnchor` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6859-6860`), the SOLE
production origin of a non-default flow on either renderer: **flow is dock-anchor geometry, not
language.** A right anchor flips inline, a bottom anchor flips block, a middle anchor never mirrors.
Both shipped locales are LTR and `🎚️axes/🔣️.json` declares no direction, so this is structural, not a
visible bug today.

**Per-window state** — `EventRouter` (`⚡️events/🦀️.rs`) gained `flow: UiFlow` with `flow()`,
`flow_inline()`, `set_flow()` (answers whether it changed). `Ui::set_window_flow(window_id, flow)` /
`Ui::window_flow(window_id)` (`⚙️engine/🦀️.rs:705-728`) invalidate the layout generation exactly like
`set_viewport`, because a direction change re-solves the inline axis.

**Mirrored rules**

| Behaviour | React | wgpu now |
|---|---|---|
| Anchored `align: Start`/`End` | `align === (rtl ? "end" : "start")`, `🗨️Popover/🟦️.tsx:228` | `resolve_anchored_placement`'s `inline_start` branch; `side` is NOT mirrored, exactly as React leaves it |
| `Select` popup inline edge | its OWN positioner's `inlineStart`/`inlineEnd`, `🔽️Select/🟦️.tsx:259-261` | `ui_contract::resolve_select_inline_left` |
| Horizontal arrow pair | `dir === "rtl"` inversion in `🎚️Slider/🟦️.tsx:383-384`, `📑️Tabs/🟦️.tsx:164-165`, `🎛️ToggleGroup/🟦️.tsx:118-119` | `EventRouter::mirrored_inline_key`, applied ONCE in the `KeyDown` arm so every value control inherits it instead of each re-deriving it |
| Horizontal scroll sign | DOM-native | `route_scroll` takes `delta_x * flow.inline.inline_sign()` |
| Caret motion | byte offsets | deliberately NOT mirrored — `route_edit_key` walks char boundaries, already logical |

## 3. Chrome polish

### 3a `celebrate-border-burst` / `-spin`
Was: `theme.accent` hairline ring, aliased to the introduce pulse, at three paint sites
(`🖌️paint/🦀️.rs:403, :1784, :2380`), with an in-source admission that `Theme` had no colour triad.

Now: `Theme` gained `celebrate: [Rgba; 3]` (from `colors::{PRIMARY,SECONDARY,TERTIARY}` — React's
`--celebrate-conic` stops, `🖌️ui/🎨️.css:1239-1247`), `stroke_focus` (`strokes::CHROME_BORDER_FOCUS`,
React's `--stroke-focus`) and `celebrate_duration_seconds` from a NEW token
`metrics.chrome.celebrateBorderDurationSeconds = 1.2` (the CSS had `1.2s` as a bare literal at
`🖌️ui/🎨️.css:117` with no JSON source). `🖌️paint/🦀️.rs` region `🎉️Celebrate` adds
`celebrate_turns(seconds, duration)`, `celebrate_stroke(theme, turns)` (React's ease-in-out burst:
hairline at 0 %/100 %, `--stroke-focus` at 50 %) and `celebrate_color(theme, turns)` (the three conic
stops, interpolated, wrapping back to primary). The frame clock reaches paint through
`DrawList::set_clock_seconds`/`clock_seconds` (`🖍️draw/🏷️types/🦀️.rs:569-579`), stamped once per frame
by `Ui::advance_clock` — the same monotonic seconds every tooltip deadline already runs on.

### 3b `shell-floor-presentation` same-level double-paint suppression
React: `shellFloorPaints(parent) = !(parent.level === "base" && parent.fill !== "none")`
(`🔨️modules/🏠️shell-floor-presentation/🟦️.ts:14`). wgpu had no such predicate and painted
`theme.background` twice per frame — once full-screen in `ShellChromeFramePhase::FrameSetup` phase 2
(`🐚️Shell/…/🦀️.rs:15748`) and again over the main window in `render_main_window_step` phase 0
(`:16264`), the same colour over the same pixels.

Now: `🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs` region `🏠️ShellFloor` adds `SurfaceFill{None,Surface,Glass,Veil}`,
`SurfaceScope{level,fill}` and `shell_floor_paints(Option<SurfaceScope>)`. `ShellChromeBuildState`
gained `chrome_floor_scope`, set by `FrameSetup` when it fills the base floor; the main-window step is
gated on `shell_floor_paints(self.chrome_floor_scope)`. A real predicate on real state, not a
constant-false.

### 3c Bold face + `focus-visible` (W1o §5 gaps 1 and 2)
**`focus-visible`** is closed. `NodeFlags::FOCUS_VISIBLE` (bit 13, `🌳️tree/🦀️.rs:194`) is stamped
beside `FOCUSED` only when a KEY moved focus: `EventRouter` carries a `focus_visible` latch set on
`KeyDown` and cleared on `PointerDown`, and `FocusState::set_focus` now takes the modality. All 11
focus-ring paint sites read one new predicate `focus_ring_visible(flags)` (`🖌️paint/🦀️.rs`), so a
CLICKED control takes focus without the accent ring React would only show for the keyboard — while
caret/edit state and hit routing stay modality-independent on `FOCUSED` alone.

**Bold** is partially closed, and the blocker is an asset, not code: `🖼️assets/🔤️fonts/{🚀️anta,
🧱️kelly-slab,⌨️share-tech-mono}` each ship `📖️regular` only (both are single-weight upstream); the only
bold/semibold outlines in the repo are Noto **Emoji**'s. `📝️text/🦀️.rs` region `🅰️Weight` adds
`TextWeight{Regular,Semibold}` (+ `of(emphasize)`, `strikes()`) and `faux_bold_offset(size)` (4 % of
the em, floored at 0.34 px), and `🪀️widgets/🦀️.rs` adds `draw_text_weighted`, which strikes a semibold
run twice at that offset. The advances — and so every line box and wrap point — stay the regular
face's, which is what makes it a weight swap at React's own SIZE rather than the size swap the `Text`
node still makes. Remaining: wiring it into the retained `Text` arm needs
`retained_text_node_step`'s per-glyph reservation priced for `TextWeight::strikes()` — see Gaps.

### 3d `bg-muted` (W1o §5 gap 3)
`--muted` had no source at any layer: no key in `🎨️styling/🔣️.json`, no `ChromePalette` field, no
`Theme` field; the progress TRACK stood in with `theme.separator`. Added `chrome.muted` to both
appearances (`light-8-9`/`dark-8-9`, matching `🖌️ui/🎨️.css:96`/`:582`), regenerated →
`ChromePalette::muted` + `Theme::muted`, and `paint_progress` now uses it (React's `ProgressView` is
`bg-muted` + `bg-accent`, `🗣️Interpreter/🟦️.tsx:2114-2116`).

### 3e Select max-height, scroll buttons, and a viewport for the kit (W1o §5 gap 4)
`select_menu_top` ported React's flip but never the other half of `resolveSelectPlacement` — the
`availableHeight` clamp — so a 40-item Select drew past the surface edge, and the immediate-mode kit
hardcoded `viewport_h = 0.0` because `WidgetContext` carried no measured viewport.

`🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs` region `🔖️Geometry` gained `select_available_height`
(React's `:255-258`), `select_menu_painted_height` (the clamp), `select_visible_rows`,
`select_scroll_button_height` (`py-single` ×2 + a `size-tiny` chevron, `🟦️.tsx:790-795`),
`select_scroll_step` (React's `max(24, floor(clientHeight × 0.8))`, `:769-775`), `SELECT_SCROLL_MIN_STEP`
and `select_clamped_scroll`. `render_select_menu` clamps its height, culls to the visible band and
paints both scroll chevrons when it scrolls (React always MOUNTS both, `:675-679`).
`WidgetContext::viewport_height` (`🪀️widgets/🦀️.rs:133`) and a `viewport_height` parameter on
`framework_widget_context` (`🗣️Interpreter/…/🦀️.rs:1891`) thread the measured surface through; every
production call site feeds it `DrawList::screen_height()` (new accessor), which the Shell already
stamps per frame.

### 3f `UiCommand::FocusChanged` deferral (W1o §5 gap 5)
No longer deferred. The Interpreter arm stays a no-op BY DESIGN (one consumer, no double-tracking) and
its doc block at `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:471-477` now says so instead of pointing at
`w3-shell-input-cutover`. The consumer got real: `ShellChromeBuildState::content_focus` is
`HashMap<String, Option<NodeId>>` instead of `HashMap<String, bool>`, so the command's node id is
retained — React's own `focusin` listener stores the active ROOT the same way
(`🧱️elements/🌈️Surface/🟦️.tsx:186-197`). `content_focus_node(window_id)` is the new accessor;
`content_has_focus` routes through it. The modality half React gets free from `:focus-visible` is
handled at the source (§3c), not here.

## 4. `mono` derives from tokens (W1k gap 1)

The os wgpu Shell hand-ported **20 `Rgba::from_srgb8` literals** in `fn mono_theme` because mono's
`chrome` group declared **7 keys the default theme lacked** (`canvas`, `window`, `panel`,
`hoverWindow`, `hoverPanel`, `temporary`, `overlayBg` — W1k said six, it is seven) and so could not
share the generated `ChromePalette`. The reverse also held: mono was MISSING 4 `board` keys
(`nodeStroke{Computing,Stale,Error,Blocked}`) and 4 `diagram` keys (`shapeOutline`,
`field{Residual,Reaction,Displacement}`), which `parseUiTheme` never caught because it validates
GROUPS, never their keys.

1. `🎨️styling/🌓️theme/🔣️.json` — the 7 extras dropped (those surfaces now come from the same
   `levels` formula semio's do), the 8 missing keys added off mono's own palette, `muted` added. Both
   appearances now declare exactly the default theme's paint keys, in the same order.
2. `🎨️styling/📽️projection/🟦️.ts` — new `🌓️Premade` region: `loadPremadeThemes()` +
   `emitRustPremadePalettes()`, appended to `emitRust`. Emits `<GROUP>_<THEME>_<APPEARANCE>` over the
   SAME `<Group>Palette` struct → `CHROME_MONO_LIGHT/DARK`, `OUTCOME_MONO_*`, `DIAGRAM_MONO_*`,
   `BOARD_MONO_*`, `MAP_MONO_*`, `CANVAS_MONO_*` (12 constants). No new output path, so the adapter
   manifest is unchanged.
3. `assertPremadeAppearanceKeyParity` runs inside `validatePremadeThemes`, so this class of drift
   cannot recur — `generate` and `check-generated` both fail closed on it.
4. `🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs` — `Theme::mono(dark)` beside `light()`/`dark()`, through the same
   `from_chrome`, so the `level*`/`element*` ramp, metrics, fonts, checker, outcome and diagram hues
   are all token-derived.
5. `🐚️Shell/…/🦀️.rs` — `fn mono_theme` (63 lines, 21 literals) deleted; `resolve_theme_for_ids`
   dispatches to `Theme::mono`. The theme-literal law's allowlist entry for it is **gone**, and
   `no_wgpu_target_paints_a_hand_written_colour_literal` passes without it.

Side effect worth naming: the hand-port read the WRONG source for two fields (`text_muted` and
`text_element` both took `hover_interactive_fill`); `from_chrome` reads `muted_foreground` and
`border_element`. Mono's rendering therefore changes, correctly.

## 5. Icons re-rasterise on a density change (W1g gap 1)

`build_icon_atlas_scaled` was called exactly once, at boot, on both hosts — dragging a window onto a
higher-density display kept boot-density icons forever, while the glyph atlas re-rasterised.

`🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs` region `🔁️IconRaster` adds `IconAtlasRebuild`
(`new`, `cell_size`, `scale_factor`, `progress`, `step(budget)`), `IconAtlasRebuildStep` and
`ICON_RASTER_STEP_BUDGET = 16`. `build_icon_atlas_scaled` now drives that job to completion, so there
is exactly ONE rasterisation path, and `pack_icon_atlas` is the shared packer.

Runtime: `AppRuntime` gained `icon_rebuild` + `icon_raster_scale`; `AppRuntime::resize` calls
`begin_icon_raster`, which arms a rebuild only when the new factor lands on a different atlas CELL
size (`icon_cell_size` rounds and clamps at 3×, so a fractional resize that quantises the same way
must not discard 250 good rasterisations). A new `FrameFinishPhase::IconRaster` — before
`GlyphUpload` — spends one bounded slice per frame and, on the step that completes,
installs the atlas and pushes one `PreparedRenderUpload::IconAtlasPages`.

## 6. UiIntent dispatch is exactly React's runtime semantics (W1m G1 + G3)

**G3 (controller identity) — fixed.** React's `uiIntentToActionDescriptor`
(`🛠️ShellHelpers/🟦️.tsx:2051-2058`) reads `intent.action.scope` as `controllerId`. wgpu's
`UiIntentCommand` carried a second `controller_id` field holding the SESSION controller and
`descriptor()` answered that one, so a document whose binding named another scope dispatched to the
wrong controller. The redundant field is **deleted**; `controller_id()` is
`self.action.scope.as_str()`. Safe because the SDK's `ActionFactory` mints the scope FROM the
controller id (`💻️os/🔨️modules/🔌️plugin/🦀️.rs:6815`), so the two agreed for every SDK-authored
binding and diverged only where React would too. `name@version` for non-v1 verbs is unchanged.

**G1 (kernel seam) — retired, not ported.** `🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs` (207 lines) was
`#[cfg(test)]`-mounted, its own docstring admitted "Test fixture … Production rendering dispatches
through RuntimeMailbox; this fixture has no production router", its `KernelSeam`/`AppKernelSeam`/
`HostWaker` were named ONLY by its own test, and its `KernelOutcome` shadowed the name of the REAL
production kernel exchange in the same file (`🧊️renderer/🦀️.rs:5094`). Under AGENTS.md's
no-scaffolding rule the clean long-term shape is one router, so the fixture and its test are deleted
and `📮️RuntimeMailbox` is documented as the production seam — which is already the exact counterpart
of React's own dispatch (`onActionStable` taking a descriptor built by `uiIntentToActionDescriptor`).
Nothing about wgpu's runtime semantics needed the fixture; what DID differ was the identity that
descriptor carried, and that is fixed at the source.

## 7. puzzle 5D visualisation twin (audit §7 `P-puzzle5d`)

The React "visualisation target" `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/🟦️.tsx` (638 lines)
contains **no JSX, no canvas, no colours** — it is a pure geometry module: `compose5d` (merge two
untyped fixtures) plus a flatten/pose solver and a 2-D topology diagram layout. There was no Rust
counterpart anywhere (`Puzzle5dFastener`'s own docstring at `🖐️5d/🦀️.rs:331` names
`compose geom::flatten`, which did not exist), so the wgpu renderer's existing `Board2d` and
`MeshWindow` projections could only paint AUTHORED poses while React painted solved ones.

New: **`🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📐️geometry/🦀️.rs`** (≈370 lines), mounted ungated at
`standards::v1::subsets::any::geometry` so the renderer reaches it without `component-app-assembly`.
It is the `🔖️Flatten` region ported rule for rule, typed over `Puzzle5dSnapshot` instead of JSON
records (every Rust caller already holds the schema; only the sketchpad bridge needs the untyped
compose half):

- `FlattenPlane` (+ `IDENTITY`), `FlattenPose`, `Attraction::of(&Puzzle5dFastener)`.
- The matrix library transcribed at React's own column-major layout: `plane_to_matrix`,
  `matrix_to_plane`, `mul_mat`, `translation`, `rotation_axis` (Rodrigues),
  `quaternion_from_unit_vectors` (antiparallel branch included), `quaternion_to_matrix`,
  `plane_to_orientation` (all four trace branches), `orientation_to_plane`.
- `compute_child_plane` — the pose solver, same order:
  `parent · T(point) · T(rise) · T(shift) · T(gap) · orientation · T(-childPoint)`, both degenerate
  direction branches at React's `TOLERANCE = 0.01`.
- `diagram_center` — `DIAGRAM_RADIUS = 2.697`, `DIAGRAM_VERTICAL_V_EXTRA = 1.0`,
  `DIAGRAM_HORIZONTAL_SCALE = 3.0633`, rounded to 1e-6.
- `flatten_poses` — the undirected BFS, rooted at every `Fixed` part FIRST then every remaining one,
  degrading an unaddressable endpoint or a missing grip to the identity plane at diagram origin.
- `prepare_topology_poses` — `PUZZLE_5D_TOPOLOGY_ICON_WIDTH = 48`.

Proven against React's own oracle
(`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧪️compose5d-preparetopologymodel/🟦️.tsx:9`): a `fixed` root at
`[1,2,3]` with a `derived` child through one z-facing grip pair, fastener `x: 0.5`, `y: 0.25` → child
world x ≈ 1 and diagram x = `1.5 × 48`. Both hold.

## 8. Shell dead-code decision (audit §7 `P-shell-dead-code-cleanup`)

Every candidate was resolved by an exhaustive consumer hunt (exported symbols grepped repo-wide,
excluding each element's own directory, tests and stories). Targets are declared by DIRECTORY
PRESENCE — `🔣️taxonomy.json:28884-28917` registers only `🧊️wgpu`/`⌨️tui`/`⚛️react`/`⚛️5d-react`, and
`<element>/🟦️.tsx` IS the React target — so absence of `🎯️targets/🧊️wgpu/` is the evidence.

| Element | Targets | Consumers | Verdict |
|---|---|---|---|
| **📑️Tabs** | ⚛️react 246, ⌨️tui 39 | **product**: `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🛡️AdminApp/🟦️.tsx:9,64,66,68,76-91` renders a live `Tabs`/`TabsList`/`TabsTrigger`/`TabsContent` tree | **wgpu twin BUILT** (below) |
| 🛂️SpaceAdministration | ⚛️react 459 | product: `🏛️ShellHost/🟦️.tsx:384,10919` renders `SpaceAdministrationPane` | needs a twin, but it is packet **W2e**'s ("Space Administration pane") — not duplicated here |
| 📤️SegmentedDownload | 🟦️.ts 170 only | product: `🔌️PluginRuntime/🟦️.tsx:111,786`, `🛠️ShellHelpers/🟦️.tsx:199,671` | **no twin**: host I/O helper, not a widget. Not even in `members-of-elements` |
| 🧵️TaskManager | ⚛️react 397 | **zero** outside its own test; self-documented "registrar-only, unmounted" | **no twin**: and when mounted it renders through the GENERIC `Table` scene (`columnsJson`/`rowsJson`), so it rides `SurfaceKind::Table` |
| 📐️Layout | ⚛️react 153 | barrel re-export + 2 tests | **no twin needed**: one already exists inside `🐚️Shell/🎯️targets/🧊️wgpu` — that file self-documents as the twin of `<Panel anchor=…/>` from `📐️Layout`'s `ANCHORS.map` loop (`:16566`, `:16682`) |
| 🧾️Form | ⚛️react 23 | barrel only; zero `<Form` in JSX repo-wide | **no twin** |
| 🔚️Footer | ⚛️react 45, ⌨️tui 26 | barrel + tui dispatcher + 2 tests | **no twin** |
| ➖️Divider, 📃️List, 🧙️Wizard, 🪙️Chip, 🪵️Log | ⌨️tui ONLY (no `🟦️.tsx` at all) | only the ui crate's own tui widget dispatcher and its unit tests. The two external crates on the tui target (`🎛️dashboard`, `⌨️cli`) consume `ui_tui::tui::pty` ONLY | **out of scope**: no product ever constructs `WidgetState::{List,Wizard,Chip,Divider,Log}` |

**Built: `🧱️elements/📑️Tabs/🎯️targets/🧊️wgpu/🦀️.rs`** (167 lines), mounted at
`🎯️targets/🧊️wgpu/🦀️.rs:226` behind the light `wgpu` feature, shaped like the `👥️PresenceBar` twin:
`TabsOrientation`, `TabsActivationMode`, `TabRow`, `TabsKey{Ignored,Focus,Activate}`,
`tabs_step_keys`, `tabs_key` and `build_tabs`. `tabs_key` is React's `moveTabFocus`
(`📑️Tabs/🟦️.tsx:163-181`) rule for rule: it indexes the ENABLED subsequence (React's
`'[role="tab"]:not(:disabled)'`), wraps at both ends, answers `Home`/`End` with the first/last enabled
trigger, mirrors the horizontal pair under RTL, and under `Manual` activation moves focus without
selecting while `Enter`/`Space` selects. `build_tabs` emits one `Button` per row carrying its own
`value` arg (one verb serves every trigger), `presence.selected` for React's `data-state="active"`,
`UiState::Disabled` for its `disabled`, and exactly ONE panel — React's `TabsContent` returns `null`
for an inactive value, so an inactive subtree must not exist here either.

### 🔑️KeyValue — the inverse gap, for the React lane
Confirmed and sharpened. `🔑️KeyValue` is wgpu-ONLY: `🎯️targets/🧊️wgpu/🦀️.rs` (25 lines,
`render_key_value`) with real production consumers (`🪀️widgets/🦀️.rs:361,437,477`;
`🔀️reconcile/🦀️.rs:612,705,836`), and the contract variant `Component::KeyValueList` +
`KeyValueListProps{entries: UiFixedList<KeyValueEntry>}` (`🧬️contract/🧩️component/🦀️.rs:614, :330-336,
:136-142`) is fully wired including the TS projection, typed visitor, compare table and a11y role.
**There is no `🧱️elements/🔑️KeyValue/🟦️.tsx` and no exported React `KeyValue` component anywhere.**
React's rendering is a private 15-line `KeyValueListView` inlined in the OS interpreter
(`🗣️Interpreter/🟦️.tsx:1297-1311`, dispatched at `:2175`). The React-lane action is to EXTRACT that
function into `🧱️elements/🔑️KeyValue/🟦️.tsx` so the element directory stops being wgpu-only — not a
wave-2 wgpu item.

---

## 9. Tests

**New: 61.** All run, all green.

| Suite | Count | Covers |
|---|---|---|
| `🧬️contract/🧪️tests/🔬️overlay-unit/🦀️.rs` (new) | 15 | side/offset, both flip directions, flip-that-also-overflows, MENU inline mirroring, End-align mirroring, Center never mirroring, vertical cross axis direction-invariant, both clamps, collisions-off, centered, transform-origin/opposite tables, `resolve_select_inline_left` all six cases, flow defaults/merge/`for_anchor` all nine anchors/`inline_sign`/wire tags |
| `🧪️tests/🔬️targets-wgpu-events-unit` | +5 | RTL menu mirroring through the shim, router flow default + change-once, the mirrored key table (and the 8 invariant keys), RTL slider stepping the other way, the pointer-vs-Tab focus-visible latch and both node flags |
| `🧪️tests/🔬️targets-wgpu-paint-unit` | +4, 1 extended | celebrate cycle wrap + zero/NaN duration, burst 0/50/100 % + easing, the three conic stops + wrap + interpolation, a celebrating element painting differently as the clock advances while an introducing one does not; `assert_focus_swaps_border_color` gained the POINTER-focus counter-case at every control kind |
| `🧱️elements/🔽️Select/🧪️tests/🔬️wgpu-select-keyboard` | +6 | available height on both sides + unmeasured, the clamp, visible rows (incl. partial-row rule), React's scroll step incl. the 24 px floor and the floor-not-round rule, scroll extent clamp, scroll-button height, RTL inline edge |
| `🧪️tests/🔬️targets-wgpu-theme-token-parity` | +4 | `Theme::mono` off its own generated palettes, every derived surface rule shared with the default (incl. the two fields the hand-port read from the wrong source), premade twins for the 8 keys mono used to omit, `shell_floor_paints` full truth table |
| `🧪️tests/🔬️targets-wgpu-text-unit` | +3 | weight from `emphasize` + strike pricing, the faux-bold offset ramp/floor/monotonicity, a semibold run striking twice at the regular advances and at exactly the offset |
| `🧱️elements/📑️Tabs/🧪️tests/🔬️wgpu-unit` (new) | 7 | step pair per orientation + RTL, disabled-skipping + wrap + Home/End, RTL stepping, manual activation, empty/all-disabled/unclaimed-key/parked-on-disabled, the built tree's triggers + presence + per-trigger value arg, vertical stacking |
| `🧪️tests/🔬️targets-wgpu-action-unit` | +2 | controller IS the authored scope (incl. a foreign scope), `name@version` |
| `🖼️IconRenderHost/🧪️tests/🔬️wgpu-unit` (new) | 5 | cell quantisation incl. the 3× cap and NaN, progress bookkeeping, budget never overspent + multi-step + terminating, zero budget still advances, **stepped result byte-identical to the one-shot build** |
| `🖐️5d/…/📐️geometry/🧪️tests/🔬️unit` (new) | 9 | React's own oracle (world x ≈ 1, diagram x = 1.5 × 48), fixed-vs-derived root, `Fixed` roots first regardless of declaration order, both degraded-hop cases, undirected walk, empty document, the diagram seed circle, both degenerate alignment branches NaN-free |

Peer-stale test code fixed on the way (additive, nothing reverted): 10 `set_focus` call sites in
`🔬️targets-wgpu-events-unit`, 2 `WidgetContext` literals in `🔬️targets-wgpu-engine-unit`, 22
`framework_widget_context` call sites across the os renderer's test suites, the `UiIntentCommand`
fixture in `🔬️wgpu-ui-command-wiring`, the `content_focus` fixture in `🔬️wgpu-chrome-overlays-tour`,
and the mono assertion in `🔬️wgpu-ui-prefs-themes-i18n`.

One committed budget legitimately moved: `⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`
`elementSizeBytes` 160616 → **160768** (+152 B: `Theme.muted`/`stroke_focus`/`celebrate`/
`celebrate_duration_seconds`, `DrawList.clock_seconds`, `EventRouter.flow`/`focus_visible`). The
guard test passes again.

## 10. Verification (all foreground, one cargo at a time, `-j 4`, plain env)

| Gate | Result | Log |
|---|---|---|
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | ✅ 0 errors | `w2k-ui-check-8.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | ✅ 0 errors | `w2k-os-check-5.txt` |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --target wasm32-unknown-unknown` | ✅ 0 errors | `w2k-wasm-check.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | ✅ 0 errors, 59.7 s | `w2k-os-wasm-check.txt` |
| `cargo check -p semio-framework-ui-contract --lib` / `-p semio-framework-ui-render --lib` | ✅ 0 errors | — |
| `cargo check -p semio-s-artifact-puzzle-5d --lib` | ✅ 0 errors | `w2k-puzzle5d-check.txt` |
| `bun ./📜️script.ts check-generated` (styling) | ✅ "generated artifacts are fresh" | — |
| `cargo test -p semio-framework-ui-render --lib` | ✅ **131 / 0** | — |
| `cargo test -p semio-s-artifact-puzzle-5d --lib` | ✅ **374 / 0** (2 ignored) | `w2k-suite-…puzzle-5d.txt` |
| `cargo test -p semio-framework-ui-contract --lib -- overlay flow anchor` | ✅ **17 / 0** | `w2k-contract-overlay-tests.txt` |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib` | **546 / 5** | `w2k-ui-suite-2.txt` |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- <targeted filters>` | see below | `w2k-os-targeted-tests.txt` |

**ui suite, the 5 failures — none in this packet's surface**, and the same count and family the
integrator's gate 5 recorded (506/5 then, 546/5 now = +40 of this packet's tests, 0 new failures):
`prepared::{preparation_yields_at_the_configured_item_budget, receiver_survives_worker_ownership_of_the_job,
retained_codec_source_moves_once_and_retires_one_page_per_governed_step}`,
`engine::retained_document_hostile_fixtures::max_plus_one_…`,
`reconcile::document_tree_reconcile_tests::a_surfaces_second_document_…`. That is 📓️w1-integration.md's
"resident-reservation pricing family"; the exact set shifts between runs, which is itself the
flakiness. Nothing in this packet touches `prepared`, the hostile fixture, or reconcile's document
tree. The one failure that WAS mine — `engine::tests::ui_surface_slot_table_is_heap_first_…` — is
fixed by the recommitted budget above.

**`semio-framework-ui-contract` whole-lib run aborts** (`SIGABRT`, "panic in a destructor during
cleanup" from `UiValueRetirement::drop` inside
`document_component_compare_tests::retained_document_component_compare_cancel_and_contention_…`),
which takes the whole binary down and reports every other test as FAILED. That is exactly the
"drop-witness→SIGABRT hiding renderer tests" item 📓️w1-integration.md already recommends its own
ticket for. Filtered runs are clean.

**os-renderer targeted run — 3 failures, none this packet's:**
`shell::tool_run_panel_tests::{…paints_and_its_buttons_dispatch_the_run, …buttons_are_keyboard_reachable}`
(the panel's `toolRunPause` target is not registered at all, and Enter dispatches `toolRunAbort`) and
`shell::chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments`. All
three live inside the concurrent Shell churn — `git diff --stat` over `🐚️Shell` is **+5453/−1363
across 11 files**, against ~40 lines of mine (the floor-scope guard, `content_focus`'s type, four
`viewport_height` reads, the `mono_theme` deletion). Panel content and window silhouettes are W2c/W2d
territory. Also failing: `ui_prefs_themes_i18n_tests::{env_lock_ignores_unset_and_empty,
shell_pref_locks_reads_the_four_lockable_envs}` — these set and read PROCESS env vars and clobber each
other even at `--test-threads=1` (`left: None, right: Some("en")` and the mirror image); pre-existing,
W1d's `env_lock` area. **Passing in this packet's surface: 57 incl. `icon_atlas` ×5, `shell_input`,
`resolve_theme_for_ids_semio_and_mono_differ`, `wgpu-ui-command-wiring`.**

---

## 11. Remaining gaps

1. **No bold Latin font asset.** `TextWeight`/`faux_bold_offset`/`draw_text_weighted` exist and are
   tested, but the retained `Text` arm still swaps SIZE: `retained_text_node_step` prices output per
   glyph, so carrying `TextWeight::strikes() == 2` needs that reservation doubled. That is a
   paint-lane edit inside the resumable cursor W1l/W2a own. The primitive is ready for it; a real
   face would still be the better fix (`🚀️anta` and `⌨️share-tech-mono` are single-weight upstream, so
   it means sourcing or synthesising an asset).
2. **The celebrate ring is one colour per frame, not a conic gradient across the ring.** The spin and
   the burst are exact; sampling the conic PER EDGE needs either four coloured strips (which changes
   the Celebrating site's instance count, and so its retained reservation) or a conic term in the
   border shader. The latter is the right fix.
3. **`FlowBlock::Up` has no consumer yet.** The vocabulary, the anchor derivation and the predicate
   (`is_reversed`) are in place, but React's block-axis reversal lives in `Panel`/`Pane`
   (`flex-col-reverse`, `capDock: "bottom"`, `justify-end`, reversed `PanelTabBar`, reversed
   `WindowMeasureTreeGroup`) — all inside the os Shell's panel/window code that W2c and W2d are
   actively rewriting. Wiring it there is theirs; the primitive is ready.
4. **Nothing calls `Ui::set_window_flow` in production yet.** React derives it from the dock anchor
   in `Panel`/`Pane`; the wgpu equivalent is the Shell's panel build, same W2c/W2d lease. Until then
   every window is `UiFlow::DEFAULT`, which is what React's own default is — so this is latent, not
   wrong.
5. **RTL is a React-side gap in two places too**, worth raising with that lane: `🖱️ContextMenu`'s
   `contextMenuNavigationFromKey` (`:249-275`) maps `ArrowLeft`/`ArrowRight` to submenu pop/open
   unconditionally, and `📑️Tabs`/`🎛️ToggleGroup`/`🎚️Slider` read a local `dir` prop defaulting to
   `"ltr"` instead of `useFlow()`, so they never actually see a mirrored flow. The wgpu twins adopt
   the intended semantics.
6. **Select scroll buttons paint but do not yet scroll.** `select_scroll_step` and
   `select_clamped_scroll` are the tested arithmetic; routing a press on a scroll chevron into a
   per-popup scroll offset needs a slot on `WidgetContext`'s `scroll_offsets` keyed by the popup id,
   and the retained painter's row band to read it. The clamp and the culling already keep the popup
   on-surface, which was the correctness half.
7. **Premades are projected for Rust only.** `emitTypeScriptTokens`/`emitPython` still see only the
   default theme. Nothing needs it today (React resolves premades at runtime through `parseUiTheme`),
   but it is an asymmetry in the emitter.
8. **React cannot reach `mono` at all.** `builtinUiThemes()` globs `"../theme/*.theme.json"`
   (`🎨️styling/🌓️theme/🟦️.ts:811`), which resolves to a directory that does not exist — the real file
   is `🌓️theme/🔣️.json`. So `builtinUiThemes()` returns `[semio]` and the theme picker never offers
   mono; the `🧪️theme-resolve` suite that asserts otherwise is not wired into any active vitest
   project. **The wgpu Shell may currently be the only surface that renders mono at all** — which
   also means React is not a usable oracle for §4 and the parity check ran against the token source
   instead. A React-lane one-liner, worth doing before anyone compares the two shells.
9. **Mono's non-appearance blocks are still stale** relative to `🔣️.json`: it carries retired
   `opacities.glass*` and `metrics.chrome.glass*` keys and lacks `controlHeightSmallUiSpacing`,
   `iconInlineUiSpacing`, `sizeTinyUiSpacing`, `chromeFocusRingAlpha` and the `toolRun` metric group.
   The new parity law covers appearance PAINTS only, which is what the Rust projection consumes;
   extending it to metrics/opacities/strokes would catch these.
10. **The icon rebuild's upload is one frame, not paged.** The bounded job splits the
    RASTERISATION (the expensive half, ~250 `usvg`/`resvg` calls); the completing step then pushes the
    whole atlas as pages in one go. That is the same shape the glyph atlas's own `take_dirty()`
    re-upload already has, and the memcpy is cheap next to the rasterisation — but a display swap on
    a 3× surface moves ~27 MB in one frame.
11. **`Theme::mono`'s presence palette is appearance-derived, not mono-specific.** `from_chrome`
    takes `PresenceAppearance::{Light,Dark}`; mono authors its own `presence` block, which is not
    projected. Minor, and invisible until a hub session runs under mono.
12. **The shell's own tooltip and dialog pass `FlowInline::Ltr` explicitly**
    (`🐚️Shell/…/🦀️.rs:18056, :18077`). Correct today (the shell root is React's default flow), but it
    should read the shell's flow once the Shell carries one.
