# 🧊 wgpu vs React parity audit — Shell / Window System / Chrome

Lane: shell layout regions, window system (docking/tiling), panels, menus, overlays, keyboard
shortcuts, theme tokens, hit-testing. Read-only audit, 2026-09-17. Produced by one coordinating
pass plus four parallel deep-dive passes (shell layout+theme; window system; panels/menus/overlays;
keyboard+hit-testing/coordinate-space), all findings cross-checked against each other and, where
noted, independently re-verified by the coordinating pass. All paths absolute under
`/Users/ueli/Documents/semio` unless otherwise noted.

**Actual file sizes** (ticket brief's line counts were slightly off): React `ShellHost/🟦️.tsx` =
11,059 lines (not 13.8k), `ui/🎯️targets/⚛️react/🟦️.tsx` = 11,529 lines, `Shell/🟦️.tsx` = 1,283
lines, `ChromePanels/🟦️.tsx` = 1,408 lines. wgpu `Shell/🎯️targets/🧊️wgpu/🦀️.rs` = 16,325 lines
(matches), `Dock/🎯️targets/🧊️wgpu/🦀️.rs` = 1,653 lines, `os/…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
= 14,929 lines.

## 0. Architecture notes (read this before the gap table)

1. **This is not a floating-overlapping-window OS.** Both React and wgpu implement a VS-Code-style
   **tiling/docking window manager**: a recursive layout tree of `row`/`column` splits and tab
   `stack`s, with each stack's tabs additionally grouped into one of 4 chrome corners
   (`WindowStackCorner::TopLeft/TopRight/BottomLeft/BottomRight`). There is no window free-drag,
   no per-window resize corners, and no z-order/raise-on-click model on *either* side — that is
   correct parity, not a gap. Core React file:
   `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx` (2,136 lines — `Mode`, `ModeDockTabBar`,
   `ModeDockStack`, drop-zone math). This file was **not** in the ticket's file list and should be
   added to future audits of this lane.
2. **A large amount of the type/behavior surface is shared by construction**, which is why so much
   of this audit comes back "OK": `WindowLayoutNode`/`WindowStackCorner`
   (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📐️layout/🦀️.rs:251-266`), `ContextMenuItem`, and the
   `SHELL_KEYBINDINGS` chord table are consumed by both targets (and the TUI target) from one
   source; theme metrics/colors come from one generated `ui_styling` crate
   (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs`, header: *"Auto-generated from
   framework/ui/styling/🔣️.json — do not edit by hand"*) consumed by both the CSS
   (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`, 7,491 lines) and wgpu's
   `🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs`. A dedicated regression suite already asserts some of this:
   `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs`
   (988 lines — keybinding routing vs. `SHELL_KEYBINDINGS`, example picker, mode switching, overlay
   safe-area, session switching). **Treat anything covered by that test file as lower-priority.**
   The real risk surface is what that file does **not** cover: pixel geometry, DPI/coordinate
   handling, and several overlay/panel subsystems that have no wgpu implementation at all.
3. **The single highest-confidence finding, independently reached by two separate deep-dive passes**
   (window-system pass and keyboard/hit-testing pass) from different code paths, is a **DPI /
   scale-factor unit mismatch**: the wgpu content root (`screen_w`/`screen_h`) is sized in
   **physical** pixels (`css_width * dpr`), raw pointer coordinates from winit are **physical**
   pixels (consistent with the root — hit-testing itself is not desynced), but **every chrome
   layout constant** (`chrome_px()`, `navbar_height`, `footer_height`, tab height, resize-handle
   hit width, padding, gaps — i.e. everything in `theme.rs` and used throughout the 16.3k-line
   Shell file) is a **logical/CSS-pixel** value taken unmodified from the same generated tokens
   React's DOM/CSS consumes, with **zero `dpr` multiplication anywhere** in the chrome layout code.
   At `scale_factor == 1.0` this is invisible; on any HiDPI display (the default on this
   environment's own macOS/Retina host) every fixed-size chrome element renders and hit-tests at
   roughly `1/dpr` of its intended size/position relative to the full-size canvas — this is the
   most likely single explanation for "ui elements are placed totally different, the window system
   doesn't work." See §7 for exact citations and the two candidate fixes.

---

## 1. Shell layout regions

| Feature | React (file:line) | wgpu (file:line or absent) | Status | Geometry / constants |
|---|---|---|---|---|
| Root shell layout (navbar → subnavbar → canvas+panels → footer, flex column) | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🟦️.tsx:106-141` — `flex flex-col h-screen w-screen`; navbar/subnavbar/footer are `flex-shrink-0`; middle band is `flex flex-1 min-h-0 relative`; panels float **inside** that middle band (`ANCHORS.map(anchor => <Panel anchor={anchor}/>)`), comment: *"panels open below the navbar / above the footer instead of floating over them"* | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13247-13249` `body_rect()`: `top = navbar_height + tutorial_bar_reserve; Rect(0, top, screen_w, screen_h - top - footer_height)`; `render_navbar_step` (13583), `render_tutorial_bar_step` (13863), `render_footer_step` (14015) | **OK (structural)**, but see §0.3 — the *rect math* matches, the *unit system* underneath it doesn't | navbar/footer height = `chrome_px(9.0)` on both sides via shared tokens (React `--size-large` = 9×`--ui-spacing`; wgpu `NAVBAR_HEIGHT_UI_SPACING`/`FOOTER_HEIGHT_UI_SPACING` = 9.0, `🎨️styling/🔤️tokens/🦀️.rs:270,275`) |
| Navbar content | `ShellHost/🟦️.tsx:10951-10957` `<Navbar items={navbarItems} showFullscreenToggle={!mobile}/>`; items built 10052-10089 — logo/title, `NavbarExampleSelect`, role/mode `ButtonGroup` (9159-9218) | `render_navbar_cluster_step` (`🐚️Shell/…/🦀️.rs:13825-13857`) — flat row of `ShellNavbarControl` icon/label items | **P2 divergent** | Content simplified vs. React's example-selector/mode-switcher cluster; not item-by-item diffed |
| Subnavbar (tutorial bar) | `ShellHost/🟦️.tsx:10958-10979` conditional `<TutorialBar>` | `render_tutorial_bar_step` (13863), `tutorial_bar_height() = theme.navbar_height` (12084-12085) | **OK** | height reservation matches |
| Footer / utility bar | `footerItems` (`ShellHost/🟦️.tsx:10608-10630`) — `PanelChromeTabBar` at `bottom-left`/`bottom-middle`/`bottom-right` anchors, `PresenceBar`, funding credits | `render_footer_step` (14015-14090) walks `active_utilities` tree (`render_footer_utility_node(s)`, 9908-10012); native-only sync/checkin indicator (`#[cfg(not(target_arch="wasm32"))]`, ~14083) | **P1 divergent** | wgpu footer has no bottom-anchor `PanelChromeTabBar` docking (consistent with the panel-anchor gap, §3); wasm/browser wgpu additionally lacks the sync/checkin indicator that native wgpu has |
| Left / right "rails" | Not literal sidebars — 8 independently-floating anchor slots, see §3 | 2 hardcoded slots (`left_panel_open`/`right_panel_open`) | **P0** | See §3 — this is the ticket's "rails" question and the anchor-count gap is the answer |
| Status bar | None on either side — grep for `statusbar`/`status-bar`/"status bar" = 0 hits in both `ShellHost/🟦️.tsx` and the 16.3k-line wgpu Shell file | — | **OK (N/A)** | Both fold status into the footer |
| Command palette (persistent panel) | React surfaces it as a **persistent `bottom-middle` dock anchor** (`buildCommandCategoryTabs`) — self-documented by wgpu's own comments below | `build_command_panel_ui` exists but is **`#[cfg(test)]`-gated** (`🐚️Shell/…/🦀️.rs:~11447-11470`) — not compiled into production. Doc comment at lines 103-108: *"React surfaces its command palette as a persistent `bottom-middle` dock anchor... which this renderer has no equivalent of"*; comment at ~11446-11451: *"building a real middle anchor would mean touching `dock`/restructuring `ShellTypes`'s hardcoded 2-column model, both out of scope."* | **P0 missing in production** | Self-documented, deliberate, out-of-scope-flagged gap |
| Search / Find modal overlay (⌘K-style) | `UISearch`/`UIFind` mounted at `ShellHost/🟦️.tsx:10993-10994` | `OverlayState::Search`/`::Find` → `render_overlay_step` (`🐚️Shell/…/🦀️.rs:9144,9213,14099-14105`): centered box `Rect(w*0.5-200, navbar_height+8, 400, h*0.55)`; item list via `SearchPaletteItem`/`build_search_items` (128, 8880-8931) | **OK (structural)**, size is hardcoded px not token-driven | Different feature from the command palette above — this one exists and is roughly positioned right; wgpu's 400×(0.55·h) box is literal, not derived from any layout token — worth a follow-up pixel diff against React's popover sizing tokens |

## 2. Theme tokens

| Token family | React source of truth | wgpu | Status |
|---|---|---|---|
| Spacing / size scale | `🎨️styling/🖌️ui/🎨️.css:89` `--spacing-compact:0.2rem` → `--ui-spacing`/`--size-*` (`:731-747`) | `UI_SPACING_COMPACT_PX = 3.2` (`🎨️styling/🔤️tokens/🦀️.rs:269`), `chrome_px(mult) = 3.2*mult` | **OK** — same generated source |
| Border radius | All `--radius-*` = `0rem` (`ui.css:772-781`) — deliberately flat | `radii::CHROME = 0.0` (`🔤️tokens/🦀️.rs:76`) | **OK** |
| Z-scale | `--z-base:0,--z-window:10,--z-pane:20,--z-panel:30,--z-dialog:40,--z-menu:50,--z-navbar:100,--z-modal:1000,--z-tutorial:10000` (`ui.css:833-842`) | `Level` enum (`🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs:47-70`) `Base..Menu`, ordinal-derived glass/surface formula | **OK (structural)**; not independently confirmed that every numeric z ordinal lines up 1:1 across all 9 CSS z-levels vs. wgpu's 6-level enum — flagged for a follow-up |
| Base/window/panel/foreground/accent/muted colors (light + dark) | `ui.css:88-113` (light), `:574-588` (dark) — e.g. `--base:#f7f3e3`, `--accent:#ff344f` | `CHROME_LIGHT`/`CHROME_DARK` `ChromePalette` (`🔤️tokens/🦀️.rs:537-591`), linear-float RGBA decoding back to the same hex | **OK** — byte-derived from one source |
| Dark-mode trigger | `.dark { … }` class on scope root (**not** `prefers-color-scheme`) | consumes `CHROME_DARK` via explicit theme selection, not an OS media query either | **OK** (same mechanism family) |
| `error` outcome color | `--color-danger: #a60009` (dark maroon, `palette.css:176`) | `Rgba::new(0.95,0.35,0.35,1.0)` — light salmon/pink, **hardcoded literal**, not derived from `CHROME_*` (`🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs:~251-259`) | **P1 divergent** — large perceptual mismatch, and it's the color used for fault/error states |
| `success` outcome color | `--color-success: #7eb77f` (`palette.css:179`) | `Rgba::from_srgb8(36,158,91,255)` (`#249E5B`) — hardcoded, different green | **P1 divergent** |
| `progress` outcome color | no CSS token found for this concept | `Rgba::from_srgb8(67,132,245,255)` (`#4384F5`, blue) — no React counterpart to diff against | **P2** (verify whether React has an equivalent under a different name) |
| `warning` outcome color | `--color-warning: #fccf05` (`palette.css:177`) | `Rgba::from_srgb8(252,207,5,255)` (`#FCCF05`) | **OK** — exact match |
| Layout engine (flexbox parity) | Real CSS flexbox everywhere (wrap, align-items, justify-content, gap, per-child grow/shrink) | Two implementations: `🎯️targets/🧊️wgpu/📐️flex/🦀️.rs` (390 lines, Taffy-backed, explicitly aimed at "pixel-parity") is **entirely `#[cfg(test)]`-gated** (confirmed 39 `#[cfg(test)]` occurrences covering every item in the file) — **not compiled into production**. What ships is `🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs` (191 lines): single-axis stack only, no wrap, no per-child grow/shrink/basis (`extra_per_child = available / child_heights.len()` — equal distribution only), no align-items (children always stretched to full cross-axis), no justify-content variants. Own doc comment (flex.rs:5-7): *"the old immediate-mode `layout` region stays in place — `widgets`/`chrome` still call its `layout_vertical`/`layout_horizontal` directly, so it isn't deleted this milestone."* | **P0** | Well-documented, currently-unresolved architectural gap between the two renderers' layout primitives |

## 3. Panel anchor model — the "rails placed totally different" finding

| Feature | React | wgpu | Status |
|---|---|---|---|
| Anchor enumeration | `ANCHORS = ["top-left","top-middle","top-right","right-middle","bottom-right","bottom-middle","bottom-left","left-middle"]` — 8 compass points (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6825-6828`); every one is a real, independently-floating `<Panel>` slot per `📐️Layout/🟦️.tsx:126-139` | `PanelAnchor` enum mirrors all 8 names exactly, with `vertical()`/`horizontal()` helpers matching React's `anchorVertical`/`anchorHorizontal` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2899-2946`) — **the type exists**, but... | type-level **OK** |
| Anchor usage in production | All 8 usable; ~5 concrete panel tabs (Workbench/Display/Details/Settings/Chat) can be routed to any anchor via `PanelGroup` | Real state is a **hardcoded 2-slot model**: `left_panel_open`/`right_panel_open` + `LeftPanelKind{Workbench,Display}` / `RightPanelKind{Details,Settings,Chat}` (`🐚️Shell/…/🦀️.rs:113-125,2552-2553`). `PanelAnchor::from_group()` only maps `top-right`/`bottom-left`/`bottom-right`, defaulting everything else to `TopLeft` (2946-2953). Own doc comment (2888-2898): *"This shell only ever surfaces the four corners today... The four edge-middle anchors... are real anchors with nothing assigned to them yet — matches upstream, where `PanelGroup` never maps to a middle anchor either."* `group_side()` (10121-10126) further folds even the 4 corners down to left/right only. | **P0** |

This is very likely a second, independent contributor to "elements placed totally different": even once
the DPI bug (§7) is fixed, wgpu structurally cannot place a panel anywhere except a left or right
slot, while React can float a panel at any of 8 compass positions. Any React layout that uses a
top/bottom/middle anchor (the command palette's `bottom-middle` being the flagship example, §1) has
literally no wgpu destination to render into.

## 4. Window system (docking/tiling)

| Feature | React (file:line) | wgpu (file:line) | Status | Geometry |
|---|---|---|---|---|
| Open window | `insertWindowAsTab`/`insertWindowAsTabAtCorner` (`🎨️Canvas/🟦️.tsx:344,356`), drop-zone insert (`:692`) — tiled, `size` as % of parent split, no cascade/center offset | `DockState::insert_tab`/`insert_tabs_at_corner_with_kinds` (`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:398-431`), `split_stack_with_window`/`split_root_with_window` (451-527) | **OK** | Same tiling model, same absence of cascade (correct) |
| Close window | `removeWindowFromLayout` (`🎨️Canvas/🟦️.tsx:307-318`) — always allows close, collapses empty stack/axis | `close_window_in_stack` (`🛰️Dock/…/🦀️.rs:200-226`) **refuses to close the last tab in a stack** (`if windows.len() <= 1 { return false; }`, 211-213); a separate `remove_window` (462-470) does allow full removal but is **not** the function the UI close button calls (`🐚️Shell/…/🦀️.rs:7975-7982` calls `close_window_in_stack`) | **P2 divergent** | Closing an app's only window in a stack silently no-ops in wgpu; React would collapse the layout |
| Focus / active window | `activeWindowId`, SVG "silhouette" outline (`WindowChromeSilhouetteBorder`, `🎯️targets/⚛️react/🟦️.tsx:7787-7850`), tab z-classes (`z-20` active fill / `z-30` inactive, 7680-7703) | `active_window_id`/`active_stack`, `set_stack_active`/`sync_active_window` (`🛰️Dock/…/🦀️.rs:170-183`); visual = solid `theme.accent` border + `active_foreground` tint (`render_stack`, 1263-1291); click routing confirmed via `dock.tab.<path>.<id>` hit (`🐚️Shell/…/🦀️.rs:7965-7982`) | **OK** (functionally); simpler flat-color border vs. React's SVG silhouette shape is a cosmetic-only P2, not deep-audited | — |
| Z-order | None (tiling model — panes never overlap) — only fixed CSS chrome-tier layering (`z-window`/`--z-panel`/`z-tutorial+1`) | None found (`z_order`/`z_index`/`stacking`/`raise`/`bring_to_front` = 0 hits) — layering is implicit paint order | **OK** — correct absence on both sides |
| Drag/move | Drag subjects are tabs/stacks, not windows (`DragHandle`, `🎨️Canvas/🟦️.tsx:1094`); drop-zone math `computeModeDropZone`/`resolveModeSplitSideInBody` (797, 778) — dominant-axis-from-center | `DockDragState`/`DockDragPayload` (`🛰️Dock/…/🦀️.rs:143-157`), drag begins on `dock.tab.*`/`dock.stack.*` pointer-down (`🐚️Shell/…/🦀️.rs:7296-7318`), 25px² move threshold; `compute_dock_drop_zone`/`resolve_split_side` (`🛰️Dock/…/🦀️.rs:773-795`) is an **exact algorithmic port** of the React functions | **OK** | Faithful port |
| Resize handles (splitters) | `applyAxisResizeDelta` clamps `8%–92%` (`🎨️Canvas/🟦️.tsx:593-611`); exact hit-px of `↔️Resizable/🟦️.tsx` not independently re-read this pass | `apply_split_drag_on_node` clamps `.clamp(0.08,0.92)` (`🛰️Dock/…/🦀️.rs:247`) — **exact match**. `SPLIT_VIS_PX=6.0`, `SPLIT_HIT_MIN_PX=20.0` (1128-1129); 10px corner-join hit squares (`register_join_corner_hits`, 1205-1219) mirroring React's `MODE_JOIN_CORNER_TOUCH_EPS` (`🎨️Canvas/🟦️.tsx:515-580`) | **OK** on the clamp; handle px-width parity vs React **not independently confirmed** (follow-up: read `↔️Resizable/🟦️.tsx`) | 8%/92% split clamp confirmed identical |
| Maximize/minimize/restore | No "minimize" — only maximize-one-stack-to-fill-Mode ("Focus"/"Unfocus"). `showMaximize = !mobile && modeCollectWindowIds(layout).length > 1` (`🎨️Canvas/🟦️.tsx:1158,1800`) — **button hidden** for single-window layouts and on mobile | `toggle_maximize`/`maximized_stack` (`🛰️Dock/…/🦀️.rs:190-196`), rect special-case for maximized stack (262-291) — but `render_stack` (1279-1315) **paints the focus/maximize button unconditionally**, no `canMaximize`/mobile gating found anywhere | **P2 divergent** | wgpu always shows a maximize icon, even for single-window layouts where React hides it |
| Tabs / corner grouping | 4 corners per stack (`WindowStackCorner`), `modeStackTabsByCorner` (`🎨️Canvas/🟦️.tsx:320-333`) | `tabs_by_corner` (`🛰️Dock/…/🦀️.rs:69-82`), `collect_corner_tab_bars_for_stack` (886-908) | **OK** | Structurally matched |
| Snapping / drop-zone geometry | `resolveModeSplitSideInBody` — dominant-axis-from-center, always previews exact 50% split (`🎨️Canvas/🟦️.tsx:778-796`) | `resolve_split_side` (`🛰️Dock/…/🦀️.rs:754-770`) — exact algorithmic copy, `split_drop_preview_in_body` (817-828) same 50% halves | **OK** | Exact port including corner-tab-bar-checked-first ordering |
| **Window chrome buttons** | Tab structure L→R: `[icon][title]` → **Focus/Unfocus** (only if `showMaximize`) → **Close (X)** (always) → drag-handle grip. **At most 2 action buttons** (`ModeDockTabBar`, `🎨️Canvas/🟦️.tsx:1033-1102`) | `render_stack` (`🛰️Dock/…/🦀️.rs:1298`): `let actions = [("focus", focus_icon), ("new", "app-window"), ("close", "x")];` — **3 buttons on every tab, unconditionally**; width reserved for exactly 3 (`dock_tab_chip_width(...,3)` L875, `select_w = tab.rect.w - action_w*3.0` L1310). The `.new` action (icon `app-window`) has **no React tab-bar counterpart at all**, and is **never dispatched**: `🐚️Shell/…/🦀️.rs:7965` handles `.focus`, `:7975` handles `.close`; every other reference to `.new` (7173, 7297) only **strips** the suffix while resolving a drag/window id — confirmed **zero** `ends_with(".new")` dispatch arm anywhere in the file | **P0** | A concrete, always-visible, dead 3rd button on every window tab in every app, that also widens every tab vs. React's narrower 2-button tabs — independently verified by re-reading `render_stack` L1298-1315 and the Shell dispatch match arms at L7965/7975 |
| Title bar / labels | Tabs double as titlebar, `truncate` CSS ellipsis, `max-w-[12rem]` (`🔨️modules/🎛️chrome-control-presentation/🟦️.ts:35`); tab height `min-h-medium` | Tab height = `theme.control_height` = `chrome_px(7.0)` = 22.4 logical px (`🎨️theme/🦀️.rs:228`, `🔤️tokens/🦀️.rs:272`); label painted via `atlas.measure_text` with **no explicit truncation/ellipsis logic** found in `render_stack` L1291-1295 — tab width is measured from the label, implying tabs **grow to fit text** rather than truncating | **P2 divergent** (not fully confirmed) | Likely no ellipsis truncation at ~12rem; flagged for follow-up read of the tab-width computation path |
| Per-window options chip (measures rail) | Foldable rail, `🪟️Window/🟦️.tsx:43-46` (`measuresFolded`); default/min/max width from `domSizePx("layoutPanelRailUiSpacing"/"...MinUiSpacing"/"...MaxUiSpacing")` (`🎯️targets/⚛️react/🟦️.tsx:8130-8136`), default documented as matching the 300px floating-panel default | Present: `measures_folded`/`measures_expanded`/`measures_width`/`measures_resize_*` fields (`🐚️Shell/…/🦀️.rs:2621-2654`), painted by `paint_window_measures_step` (3315) | **OK, present** | Exact width-constant parity not independently re-verified |
| **Hit-testing / pointer routing** | DOM-native (browser hit-tests) | Internally self-consistent within the dock module: `plan_dock_windows` (13317-13340) computes the same `rect` used for paint AND stores it as `dock_canvas_bounds` (13326), later reused for `paint_chrome`/`register_resize_hits` (13462-13463) — **no desync inside the dock module**. See §7 for the actual (upstream) bug. | **P0 — see §7** | — |

## 5. Panels (ChromePanels / floating-anchor panels)

| Feature | React | wgpu | Status |
|---|---|---|---|
| Open/close | `PanelState{visible,size,path}` per anchor, reducer `SET_PANEL_VISIBLE`/`SET_PANEL_SIZE`/`SET_PANEL_PATH` (`🐚️Shell/🟦️.tsx:495-503,707-709,887-891`); render toggle swaps full `WindowChrome` vs. chip-only (`🖼️Panel/🟦️.tsx:396-568`), no open/close transition | Governed entirely by the 2-slot model (§3) | **P1 missing** (6 of 8 anchors have no real panel surface — same root cause as §3) | — |
| Collapse | "Collapsed" = `visible=false` → chip-only `WindowChrome` (still occupies a chrome band, just the tab row) | No panel-level "collapsed" geometry distinct from not-drawn; the 26 "collapse" hits in the Shell file are all **tree-section** accordion fold state (`collapsed_sections: HashMap<String,bool>`, 2556, 8032-8040), a different concept | **P2 divergent** | wgpu has no folded-chip-row visual state for a closed panel |
| Resize | `PanelResizeHandle` (`🖼️Panel/🟦️.tsx:339-388`), `minSize=200,maxSize=600` defaults, corner panels get 1 handle / middle panels get 2 (×2 delta factor) | `panel_resize_origin_width` (2570), clamp `.clamp(theme.panel_min_width, floating_panel_max_width(...))` (7434-7438), `PANEL_RESIZE_HIT_PX=20.0` (13457/13478), `DockAxis::Horizontal` drag. `DEFAULT_PANEL_WIDTH_PX=300.0` matches React's `🛠️ShellHelpers/🟦️.tsx:221` `DEFAULT_PANEL_WIDTH_PX=300` exactly | **OK** (for the 2 panels that exist) | Matching default-width constant; scope-limited by §3, not a resize-mechanics gap |
| Pin | No "pin" concept found on either side (only unrelated `std::pin`/"pinned to the body width" hits) | Same | **OK (N/A)** | Not a real feature on either side |
| Content projection | Declarative `PanelTreeUnit`/`TreePanelConfig` (schema-driven `Tree`) **plus** an arbitrary-content escape hatch: `TreeDataSection.emptyState?: ReactNode` (`🌳️Tree/🟦️.tsx:1042,1075,1538`), used for e.g. the chat panel (`createFrameworkChatPanelTab`, `📌️ChromePanels/🟦️.tsx:1376-1391`) — a full arbitrary React subtree hosted inside a panel | Content is exclusively `UiNode`/`UiTree` values laid out by the fixed-credit `MountedLayout` engine (`📌️mounted_layout/🦀️.rs:11-25`: `LAYOUT_NODE_CREDITS=4096`, hard-faults past capacity). `RightPanelKind::Chat` is just another declarative node-tree kind, **not** a live embedded widget | **P1 missing** | No portal/arbitrary-widget hosting mechanism exists |
| Paging / virtualization | Real windowed tree rendering (ticket 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING) — DOM "window mirror", lazy-loaded windows (`🌳️Tree/🟦️.tsx:212,406,715-893,3312-3427`) | No virtualization found; instead a **fixed node-credit budget** that **hard-faults** (`MountedLayoutFault::NodeCredits`) past ~4,096 nodes rather than paging | **P1 missing** | React streams arbitrarily long lists; wgpu refuses past a hard cap |

## 6. Context menus, tooltips, overlays

| Feature | React | wgpu | Status |
|---|---|---|---|
| Context menu | Portal to `document.body` (`🖱️ContextMenu/🟦️.tsx:128-137,623`), viewport clamp (786-805), submenu flip-on-overflow (445-460), separate text-selection menu host | `ContextMenuState`/`ContextMenuItem` (`🐚️Shell/…/🦀️.rs:744-770`), full keyboard nav (`context_menu_move_active` etc., 777-898, own test dir `🐚️Shell/🧪️tests/🔬️wgpu-context-menu-keyboard/`), viewport clamp (14295-14297), submenu flip (14896-14959), dismiss-on-outside-click (8631-8639) | **OK — best-matched subsystem audited** | Independently implemented but behaviorally equivalent |
| Tooltips | `ChromeControlHint` (`💡️ChromeControlHint/🟦️.tsx`), delay **`CHROME_CONTROL_TOOLTIP_DELAY_MS = 400`** (line 20), immediate hide on leave (114-116) | `ChromeTooltipHover`, delay **`CHROME_TOOLTIP_DELAY_MS = 500.0`** (`🐚️Shell/…/🦀️.rs:11680`), same immediate-hide behavior (explicitly documented as matching, 11668-11672), placement via `resolve_overlay_placement` (14615) | **P2 divergent** | Same architecture, numeric mismatch: **400ms vs 500ms** — trivial fix |
| Presence bar (human collaborators) | `PresenceBar` (`👥️PresenceBar/🟦️.tsx`), `PRESENCE_BAR_DEFAULT_MAX=5` (line 22), deterministic HSL palette (`presenceColor`, 70-79). Note: cursor/viewport sharing was **removed from the wire** already on the React side (`ShellHost/🟦️.tsx:7451-7455`) — avatar roster only, not live cursors | Ported 1:1: `👥️PresenceBar/🎯️targets/🧊️wgpu/🦀️.rs` (152 lines vs React's 151), `PRESENCE_BAR_DEFAULT_MAX=5` (28), byte-identical color algorithm (75), wired via `presence_peer_rows_for_surface` (`🐚️Shell/…/🦀️.rs:513-526`) | **OK** | Genuine parity, including the constant |
| **Agent approval overlay** | `🤖️AgentApprovals/🟦️.tsx` (160 lines) — Radix `Dialog`, auto-opens when `approvals.length>0` (127-160), capability/diff/risk/requestedBy display, deny/once/session decisions (105-116); mounted unconditionally at `ShellHost/🟦️.tsx:10997`; backed by a real wire protocol `GatewayToShell::ApprovalRequested`/`ApprovalResolved` (`🌉️mcp/🧵️bridge/🦀️.rs:1293,1306,1337,1382,1416,1955`) | **Zero hits** for `AgentApproval`/`ApprovalRequested`/`AgentBridge`/`GatewayToShell` anywhere in the 16.3k-line Shell file. `🤖️AgentApprovals/` has **no `🎯️targets/🧊️wgpu` subdirectory at all** (confirmed via `find`) | **P0 — missing entirely** | The wire protocol is shared and already flowing; there is simply no wgpu UI consuming it. An approval request delivered to a wgpu-rendered shell has no way to reach the user today |
| Agent presence indicator | `🚦️AgentPresence/🟦️.tsx` (48 lines) — a small chrome status indicator for AI-agent activity (distinct from the human-collaborator `PresenceBar` above) | **No `🎯️targets/🧊️wgpu` subdirectory** under `🧱️elements/🚦️AgentPresence/` at all (confirmed via `find`) | **P1 missing** | Small (48-line) feature, low implementation cost |
| **Transient notice / non-fatal status banner (React's de-facto "toast")** | `showTransientNotice` (`ShellHost/🟦️.tsx:8065-8117`) — single active notice (new one replaces the old, not stacked), auto-dismiss **`setTimeout(…, 4000)`** (8071), severity-tinted via `TRANSIENT_NOTICE_TONE_CLASS` (802). Rendered top-center: `className="pointer-events-auto absolute top-workbench left-1/2 z-50 -translate-x-1/2 …"` (10932-10942), with an explicit close button. This is the **only** surfacing mechanism for non-fatal faults across the whole shell: viewer-read-only, mutation-rejected, render-error, keybinding-unowned, remote-merge-outcome all route through it (six call sites: 5350, 5506, 6344, 6747/6827/9332/9364, 6947/9347, 8093, 8115, 8626). Several sibling top-center overlays share the same `top-workbench left-1/2 z-50` slot for other one-off statuses: `BootstrapStatusNotice`, `ExecutionTargetStatusNotice`, `ArtifactCreationProgressNotice`, `DirectoryBootstrapStatusNotice` (10879-10905) | **Zero hits** for `notice`/`TransientNotice`/`transient_notice` anywhere in the 16.3k-line wgpu Shell file (independently grepped by the coordinating pass; a parallel deep-dive pass searching specifically for `Toast`/`Notification` also found 0 hits, but under those literal names — the actual React mechanism is named `TransientNotice`, not `Toast`, which is why a name-literal search alone under-reports this gap) | **P1 missing** (upgrade from the panels/overlays pass's "OK (N/A)" verdict — see note) | This is a real, load-bearing gap, not an absent-on-both-sides feature: every non-fatal fault in React surfaces through this exact mechanism, and wgpu has no equivalent path for the user to ever see e.g. "your change was rejected" or "you are in read-only mode" |

## 7. Keyboard shortcuts (shell-level) and coordinate-space root cause

### 7a. Shortcut inventory

Two React pipelines, both gated through `useShellKeydown`:

**Static table** `SHELL_KEYBINDINGS` (`🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:163-190`):

| Chord (React) | Action | wgpu | Status |
|---|---|---|---|
| `mod+p` | `ui.search.toggle` | `🐚️Shell/…/🦀️.rs:9095-9104` | OK |
| `mod+f` | `ui.find.toggle` | `:9105-9111` | OK |
| `mod+[` / `mod+]` | nav back/forward | `:9112-9125` | OK |
| `mod+up` | nav up | `:9126-9135` | OK |
| `mod+shift+f` | `os.toggleFullscreen` | wgpu checks `F11` **or** `ctrl+meta+f` (`:9327`) — not `mod+shift+f` | **P2 divergent** |
| `ctrl/meta+b`, `ctrl/meta+shift+b` | panelAnchor topLeft/topRight | `:9136-9147` (maps to left/right panel only) | OK (2-panel model) |
| `ctrl/meta+m`, `+shift+m`, `+alt+…` (6 chords) | panelAnchor topMiddle/rightMiddle/bottomRight/bottomMiddle/bottomLeft/leftMiddle | **0 hits** for `panelAnchor` anywhere in the file | **P1 missing** — matches the §3 anchor gap exactly |
| `mod+shift+w` | `ui.window.close` | **0 hits** anywhere under the OS renderer or ui wgpu target | **P1 missing** |
| `mod+shift+enter` | `ui.window.focus` | absent (only an unrelated test-fixture chord reuses the string) | **P1 missing** |
| `mod+shift+n` | `ui.window.newWindow` | absent | **P1 missing** |
| `mod+alt+→/←` | mode next/previous | `shell_mode_step_chord` (10441-10450) | OK |
| `mod+alt+e/v` | role editor/viewer | `shell_role_chord` (10426-10440) | OK |
| `mod+z`, `mod+shift+z` | undo/redo | `match_app_keybinding` (9409-9412) | OK |
| `mod+y` (Windows-style redo alias) | redo | **0 hits** for `"y"` redo handling | **P2 divergent** (minor) |

Per-app `session.app.keybindings` loop (`handleAppKeydown`, `ShellHost/🟦️.tsx:8566-8668`) and command-palette chords (`handleCommandKeydown`, 9378-9407) both have working wgpu counterparts (`match_app_keybinding`, `resolved_commands` chord matching).

No `TODO`/`FIXME`/`unimplemented!`/`todo!`/`panic!` markers found in any traced input/keyboard code path — the gaps above are silent omissions, not marked stubs.

### 7b. Coordinate-space / DPI root cause (headline P0)

Independently reached by two separate deep-dive passes tracing different entry points:

- Raw pointer capture: `WindowEvent::CursorMoved{position}` → `app.last_pointer_pos = (position.x as f32, position.y as f32)` → `DispatchEvent::PointerMove{x,y}` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:694-698`). winit's `CursorMoved.position` is always **physical** pixels; **no** `to_logical`/`scale_factor` division happens anywhere in `normalize()` (685-731).
- Content-root size: `🧊️renderer/🦀️.rs:14002-14004` (resize) and `:14366-14367` (boot) set `shell.screen_w/h = css_width/height * dpr` — also **physical** pixels. `🌐️browser-worker/🦀️.rs:569-570` and `🧊️gpu/🦀️.rs:325-334` (surface/paint) are consistent with this — physical throughout.
- **So pointer coordinates and the content root agree with each other** (both physical) — hit-testing itself is not desynced, and this rules out a naive "click doesn't land on what's drawn" bug.
- **But** every chrome layout constant — `chrome_px()`, `theme.navbar_height`/`footer_height`/`control_height`, `SPLIT_HIT_MIN_PX=20.0`, `PANEL_RESIZE_HIT_PX=20.0`, tab action width `14.0+padding_standard`, all paddings/gaps — is computed from `UI_SPACING_COMPACT_PX=3.2` (`🎨️styling/🔤️tokens/🦀️.rs:269`), the **same generated token** React's DOM/CSS consumes as **logical/CSS pixels** (the browser auto-scales these for DPI; React never touches `devicePixelRatio` — 0 hits in `ShellHost/🟦️.tsx`). **No `dpr` multiplication exists anywhere** in `🎨️theme/🦀️.rs`, `🧮️layout/🦀️.rs`, or `📌️mounted_layout/🦀️.rs`.
- **Net effect**: at `scale_factor==1.0` numerically invisible. On any HiDPI display (this environment's own host is `Darwin`/macOS, i.e. Retina by default) the physical canvas is `dpr`× larger than the design assumes, while every fixed-size chrome element (navbar height, tab height, resize-handle grab zones, button sizes, paddings) stays pinned to its 1× logical-pixel size — chrome renders compressed into a fraction of the true canvas, resize handles become tiny/hard-to-hit, and the whole shell reads as "placed totally different" from the DPI-correct React/DOM reference. This is systemic (touches essentially every fixed-size chrome element in the 16.3k-line file), not a per-widget bug.
- **Two candidate fixes** (not applied — read-only audit): (a) set `screen_w/h` to **logical** pixels at the three resize/boot call sites above (there is already a `WindowMetrics::logical_width()`/`logical_height()` helper doing exactly `physical/scale_factor`, at `🧰️framework/🔨️modules/🖱️ui/🖥️host/🪟️window/🦀️.rs:34-46`, that appears **unused** at these specific call sites — grep shows it's called elsewhere, e.g. `🧊️renderer/🦀️.rs:12823`, but not at the 14002-14004/14366-14367 sites), and divide incoming pointer x/y by `scale_factor` before writing to `input.pointer_x/y`, letting only the final GPU present stage re-apply `dpr`; or (b) keep `screen_w/h` physical and multiply every `chrome_px`/theme constant by `dpr`. (a) is smaller and more consistent with the token pipeline being logical-pixel by construction.

---

## Recommended work packets

Sized for one engineer each; file seams chosen to minimize overlap. Ordered roughly by leverage
(fix the two P0 root causes first — everything downstream becomes far easier to verify visually
once they land).

1. **DPI/scale-factor unit fix (the headline root cause).**
   Files: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
   (lines ~14002-14004, ~14366-14367), `…/🌐️browser-worker/🦀️.rs` (~569-570),
   `…/🪟️winit-app/🦀️.rs` (~694-698, pointer capture). React reference: none needed — this is a
   unit-consistency bug, not a feature gap; the target behavior is "wgpu chrome at 1x DPI looks
   identical to itself at 2x DPI, just sharper." Use the existing
   `🧰️framework/🔨️modules/🖱️ui/🖥️host/🪟️window/🦀️.rs:34-46` `logical_width()`/`logical_height()`
   helpers instead of raw `physical.width`. Acceptance: resize a wgpu window between a 1x and 2x
   (Retina) display or force `scale_factor=2.0` in a test harness and confirm navbar height, tab
   height, and resize-handle hit rectangles stay pixel-identical relative to the window's logical
   size; add a regression test asserting `chrome_px` output is independent of `scale_factor`.

2. **Panel anchor model: wire the remaining 6 of 8 anchors.**
   Files: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`PanelAnchor::from_group`, ~2946-2953;
   `group_side()`, ~10121-10126; panel state fields ~2550-2554). React reference:
   `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🟦️.tsx:126-139` (the `ANCHORS.map` floating-panel
   loop) and `🎯️targets/⚛️react/🟦️.tsx:6825-6875` (`ANCHORS`, `anchorPositionStyle`). Acceptance: a
   panel can be opened at each of the 8 named anchors independently (not just left/right), with
   position/size matching `anchorPositionStyle`'s insets; existing `left_panel_open`/
   `right_panel_open` behavior is preserved as the `top-left`/`top-right` (or equivalent) special
   case.

3. **Ship the production command-palette dock panel.**
   Files: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — remove `#[cfg(test)]` from `build_command_panel_ui`
   (~11447) and wire it to the `bottom-middle` anchor from packet 2. React reference:
   `buildCommandCategoryTabs`/`buildCommandCategoryTree` in
   `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`. Depends on packet 2 landing first (needs a
   real `bottom-middle` slot to dock into). Acceptance: the commands panel is visible and populated
   by default (not just reachable via the separate ⌘K search overlay), matching React's
   always-present bottom-middle placement.

4. **Remove the dead "new window" tab button; gate maximize like React does.**
   Files: `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` (`render_stack`, ~1298-1315: drop the `("new",
   "app-window")` entry from `actions`, change action-width math from `×3` to `×2`; add
   `can_maximize`/mobile gating around the focus/maximize action). React reference:
   `ModeDockTabBar` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1033-1102`, `showMaximize`
   logic at 1158, 1800). Acceptance: every window tab shows exactly 2 action buttons (focus/maximize
   only when `>1` window in the stack and not mobile; close always); tab width shrinks accordingly;
   no `control_id` ending in `.new` is ever registered.

5. **Agent approval overlay for wgpu.**
   New file(s) under `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` or a new
   `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs`.
   React reference: `🤖️AgentApprovals/🟦️.tsx` (full file, 160 lines) and the wire protocol at
   `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs:1293-1416,1955`. Acceptance: a
   simulated `ApprovalRequested` frame produces a visible modal listing capability/diff/risk, with
   deny/once/session actions that emit the corresponding `ApprovalResolved` frame; auto-opens purely
   from `pendingApprovals.length > 0` state, matching React.

6. **Transient notice (non-fatal status banner) for wgpu.**
   Files: new overlay state in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (a `TransientNotice{message,
   severity, code}` field alongside existing `OverlayState`, rendered top-center at `navbar_height +
   8px`, z-level `Menu`, single active notice replacing on new arrival, 4000ms auto-dismiss timer).
   React reference: `showTransientNotice`/`TransientNotice` (`ShellHost/🟦️.tsx:8065-8117`,
   `TRANSIENT_NOTICE_TONE_CLASS` at 802, JSX at 10932-10942). Acceptance: a rejected mutation or a
   viewer-read-only fault produces a dismissible top-center banner tinted by severity that
   auto-clears after 4s, exactly one at a time.

7. **Panel content-projection escape hatch (arbitrary widget hosting).**
   Files: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs` (extend
   `MountedLayoutJob`/the node-tree model with a "host-provided content" leaf kind that a caller can
   fill outside the declarative `UiNode` schema) — coordinate with whoever owns the wgpu chat-panel
   plan, since `RightPanelKind::Chat` is the concrete forcing case. React reference: `emptyState`
   escape hatch (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:1042,1075,1538`) and its use at
   `📌️ChromePanels/🟦️.tsx:1376-1391`. Acceptance: at least one panel (recommend the chat panel) is
   driven by host-provided content rather than the fixed `UiNode` schema, without breaking the
   node-credit budget for every other panel.

8. **Panel list paging instead of hard node-credit fault.**
   Files: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs` (`LAYOUT_NODE_CREDITS`
   admission path, ~11-25, 433/472/600/664 fault sites) and
   `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs`. React reference: windowed tree
   rendering from ticket 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING
   (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:212,406,715-893,3312-3427`) — coordinate with
   that ticket's owner since this is the same underlying feature. Acceptance: a tree with more nodes
   than `LAYOUT_NODE_CREDITS` renders a scrollable/paged window instead of raising
   `MountedLayoutFault::NodeCredits`.

9. **Keyboard shortcut parity sweep: window-scope and remaining panel-anchor chords.**
   Files: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (keyboard match arms near 9095-9153; add `mod+shift+w`
   close, `mod+shift+enter` focus, `mod+shift+n` new-window, the 6 missing `panelAnchor.*` chords
   once packet 2 lands, fix the `os.toggleFullscreen` chord to `mod+shift+f`, add `mod+y` redo
   alias). React reference: `SHELL_KEYBINDINGS`
   (`🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:163-190`). Acceptance:
   every row of the existing `wgpu-shell-chrome-parity` test's keybinding-routing test
   (`every_shared_keybinding_row_routes_to_its_shell_verb_and_outranks_app_keybindings`,
   `🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs:313-348`) still passes, plus new assertions for
   the 3 window-scope chords and 6 panel-anchor chords.

10. **Theme outcome-color correction (`error`/`success`, verify `progress`).**
    Files: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs` (~251-259) — replace the
    hardcoded `error`/`success` `Rgba::new`/`from_srgb8` literals with values derived from
    `CHROME_LIGHT`/`CHROME_DARK` (or from `--color-danger`/`--color-success` if those aren't yet
    projected into the generated token crate — coordinate with whoever owns
    `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs` codegen). React reference:
    `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css:176-179` (`--color-danger`,
    `--color-success`, `--color-warning`). Acceptance: `theme.error`/`theme.success` decode to the
    same hex as `--color-danger`/`--color-success` in both light and dark mode, same way
    `theme.warning` already matches exactly.

11. **Production flexbox layout engine (retire the test-only `flex.rs` / production `layout.rs`
    split).** Files: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs` (390 lines, currently
    `#[cfg(test)]`-gated throughout) and `…/🧮️layout/🦀️.rs` (191 lines, what actually ships). This
    is a larger, riskier packet than the others — recommend scoping it as "make `flex.rs` compile
    and be used for one concrete region (e.g. the footer utility row) in production, behind a
    feature flag, without removing `layout.rs`'s callers elsewhere yet" rather than a full swap.
    React reference: any CSS flexbox usage is the reference (e.g.
    `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🟦️.tsx`'s Tailwind flex classes). Acceptance: at
    least one production chrome region gains real wrap/align-items/justify-content/per-child-grow
    behavior verified against a React screenshot at 2–3 different viewport widths.

12. **Window close / last-tab semantics and tab-label truncation.**
    Files: `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` (`close_window_in_stack` vs `remove_window`, 200-226 /
    462-470; `render_stack` label-width computation, ~1291-1295 — clip to a `max-w`-equivalent and
    add ellipsis instead of letting the tab grow). React reference: `removeWindowFromLayout`
    (`🎨️Canvas/🟦️.tsx:307-318`, `collapseLayout:225`) and
    `🔨️modules/🎛️chrome-control-presentation/🟦️.ts:35` (`max-w-[12rem]` + `truncate`). Acceptance:
    closing an app's only remaining window in a stack collapses the layout the same way React does
    (not a silent no-op); a long window title truncates with an ellipsis at the same effective width
    React uses instead of growing the tab indefinitely.
