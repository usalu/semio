# WGPU renderer vs React renderer: OS shell chrome parity audit

Auditor: opus55 (read-only). No source files were edited. No git commands were run.

## Scope and method

- React base: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` (ShellHost 24,161 lines TS/TSX; Shell, ChromePanels, ShellFooter, ShellSearch, UtilityTree, UiPreferences, HubFirstRun elements) plus UI framework React chrome at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/`.
- wgpu: `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/` (308 Rust files, 42,879 lines; 305 under `📥️staging/`) and the crate facades under `🧰️…/engine/🎯️targets/🧊️wgpu/`.
- Method: grep-based anchor mapping, file-size and token counts, and reading of crate roots. The React ShellHost and wgpu staging trees were NOT read line by line. Rows marked "unverified" were not confirmed in a running build or by pixel comparison. No screenshots or computed-style comparison were made.
- Paths below are relative to the engine base `E = 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` unless absolute. `S = E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/`. `R = 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/`.

## Architecture finding that frames the rest

The wgpu shell is not organized as the crates suggest. Each crate such as `🍱️shell-view-chrome` (`🦀️.rs`, about 7.5 KB) is a facade. It mounts staged modules with `#[path = "…/📥️staging/…/🦀️.rs"]` and `pub use … ::*` (for example `E/🎯️targets/🧊️wgpu/🍱️shell-view-chrome/🦀️.rs`, lines 56-75). The real code is in `S`.

Within `S`, one concern is split across many folders. Each folder is named after a concern and holds a partial "staging of X" module. Examples:

- `🧱️chrome-paint-and-navbar` appears in 20 folders.
- `🎬️shell-actions` appears in 19.
- `🔖️identity-and-types` appears in 18.
- `🎨️theme-file-import-and-accessibility` appears in 18.
- `🗨️chrome-overlays-and-tour` appears in 16.
- 27 concern names appear in more than one folder in total.

The largest single file is `S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs` at 977 lines. Each crate manifest also sets `maxSourceFiles = 32` and `maxActiveLines = 4000` in `[package.metadata.semio.rust-source-boundary]`. These limits are enforced against the facade, not the real source, which makes them weak guards. This is the main maintenance risk. Parity fixes will have to be made in several matching files.

## 1. Feature inventory

Severity: P0 blocks parity for a user-visible flow. P1 visible behavior or look differs. P2 minor or unverified detail. "unverified" means not confirmed by reading both sides in full or by running the app.

| Feature | React file:line | wgpu file:line or MISSING | Gap | Severity |
|---|---|---|---|---|
| Shell root (title bar, navbar, body, footer) | `E/🧱️elements/🏛️ShellHost/🟦️.tsx` (`FrameworkOsShell`, 484 lines); `E/🧱️elements/🐚️Shell/🟦️.tsx` (64 lines) | `S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs` (977 lines, largest) | Structure exists on both sides. Exact layout and gaps unverified. | P2 |
| Navbar title (session/app breadcrumb) | `E/🧱️elements/🏛️ShellHost/🔎️command-bar/` and shell title logic (not pinned to a line) | `S/🚶️session/🧱️chrome-paint-and-navbar/🦀️.rs:66` (`shell_navbar_title`); `S/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs:123` | Exists. Text parity and i18n unverified. | P2 |
| App / space switcher | `E/🧱️elements/🏛️ShellHost/🛂️directory-route/`, `🏠️space-administration/` (path only) | `S/🔀️actions/📨️command-dispatch/📨️dispatch-app-switch/🦀️.rs` (dispatch only); no dedicated switcher view found | Only the dispatch side was found. The visible switcher UI on the wgpu side is unverified. | P1 (unverified) |
| Footer / status bar | `E/🧱️elements/🏛️ShellHost/🖼️canvas-footer/🟦️.tsx:34` (`useCanvasAndFooter`); `E/🧱️elements/🏛️ShellFooter` (folder exists) | `S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs` (footer and fullscreen icon at line 250); `S/🎛️prefs/🖼️chrome-frame-and-main-window/🦀️.rs:52` | Footer and status items are present. Item list and status-surface (`R/🚦️status-surface/🟦️.tsx`: Badge:47, StatusChip:76, ErrorView:151) status tones are not mapped. | P1 (unverified) |
| Utility bar / utility tree | `E/🧱️elements/🎛️UtilityTree/🟦️.tsx` (`UTILITY_CATEGORIES` 70, `sortUtilityNodes` 61) | `S/🖐️pointer/🧱️chrome-paint-and-navbar/🦀️.rs:60-67` (`WINDOW_UTILITY_RAIL_PARENT`); tree row chevron `tree_row_has_chevron` | Utility rail exists. Sort order and categories not verified. | P2 |
| Panel dock: anchors (8-point), split, merge, reorder | `R/🧲️panel-dock/🟦️.tsx:254` (`PanelDockProvider`); header comments lines 12-13 (eight anchors, tree units dock between leaf tabs); `E/🧱️elements/🏛️ShellHost/🏗️dock-assembly/` | `S/🚶️session/⚓️dock-model-and-measures/🦀️.rs:35,44` (`DockTabMoveTarget`, `move_tab_in_dock`); `S/🚶️session/⚙️settings-and-dock-persistence/🦀️.rs:321` (`move_dock_tab`); anchor enum in `🛰️dock` crate (12 lines, thin) | Move and persistence exist. Eight-anchor drop geometry and tree-leaf drop rules not verified against React. | P1 (unverified) |
| Tab drag and drop indicator | `R/🧲️panel-dock/🟦️.tsx:13` (native HTML5 drag-and-drop); `R/📑️panel-chrome-tab-bar/🟦️.tsx:32` (`PanelChromeTabBar`) | `S/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs:352-383` (insert preview); `S/🖍️view-paint/🔖️identity-and-types/🦀️.rs:7` (`TAB_INSERT_PREVIEW_WIDTH_PX = 2.0`) | Insert preview exists. The drop indicator for anchor zones is not matched by any "drop indicator" symbol (zero hits for `drop_indicator`, `DropIndicator`, `dock_drop`, `DropZone`). Visual match unverified. | P1 |
| Maximize / restore panel | React: no `maximize` symbol found under `R/🧲️panel-dock`, `R/📑️panel-chrome-tab-bar`, or `E/🧱️elements/🏛️ShellHost/🏗️dock-assembly`. Location of React behavior unknown. | `S/🚶️session/🧩️session-and-topology/🦀️.rs:214-233` (`maximized_stack`); `S/🖐️pointer/🖱️pointer-input/🎯️shell-hit/🦀️.rs:236` (`toggle_maximize`) | wgpu has maximize. React counterpart not located. Parity cannot be confirmed. | P1 (unverified) |
| Panel close / reopen | React reopen logic not located by name. | `S/📄️docs/🧰️window-actions-and-commands/🦀️.rs` (reopen-related comments); no reopen panel action found | Reopen appears only in comments. Treat as MISSING until confirmed. | P1 (unverified) |
| Panel default width | `E/🧱️elements/🏛️ShellHost/🏗️dock-assembly/🪟️panel-reveal-and-tool-tabs/🟦️.ts:226` (`panelFitV1(window.innerWidth, 300)`) | `S/⚓️dock/⚓️dock-model-and-measures/🦀️.rs:121` (`DEFAULT_PANEL_WIDTH_PX = 300.0`) | Same value (300). Collapse rule parity unverified. | P2 |
| Window chrome (titles, tabs, buttons, focus) | `R/🧱️window-chrome/🟦️.tsx:25` (`ChromeAwareWindowScrollSurface`); `R/🪟️chrome/` (`ResponsiveLabel`, `🏷️responsive-label/🟦️.tsx:10`) | `S/🎚️chrome-state/🖼️chrome-frame-and-main-window/🦀️.rs` (not line-checked); window measures in `S/🔲️windows/` | Titles and tabs exist. Focus styles and responsive label behavior unverified. | P1 (unverified) |
| Desktop titlebar and fullscreen | `R/🖥️fullscreen/🟦️.tsx:94` (fullscreen toggle with Maximize2/Minimize2 icons), `:100` (`NavbarTrailingChromeSlot`), `:151` (`DesktopTitlebar`) | `S/🖐️pointer/🖱️pointer-input/🎯️shell-hit/🦀️.rs:212` (`ui.fullscreen.toggle`); `S/🐙️state/🧩️session-and-topology/🦀️.rs:61-62` (state flags); icon at `S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:250` | Toggle exists with the same icon pair. Titlebar layout (`DesktopTitlebar`) not matched. | P1 (unverified) |
| Command palette / search | `E/🧱️elements/🏛️ShellHost/🔎️command-bar/🟦️.tsx:8` (`useCommandBar`); `E/🧱️elements/🏛️ShellHost/🔎️command-bar/🔸️command/`; `R/🔎️window-search/` (37 lines) | `S/🕹️input/🔖️identity-and-types/🦀️.rs:207` (`ShellPaletteKind`); `S/🎹️commands/⌨️keyboard-and-palette/🦀️.rs:44` (`palette_query`); palette width `S/🚶️session/🔖️identity-and-types/🦀️.rs:95` (`SHELL_PALETTE_DESKTOP_MAX_WIDTH_PX = 512`); accessibility `S/🪀️view-panels/🎨️…/🦀️.rs:23` | Palette and search exist. Width 512 px not matched to a React value (React uses tokens, unverified). Keyboard navigation parity unverified. | P1 (unverified) |
| Search scope / window search | `R/🔎️window-search/🟦️.ts` | `S/🕹️input/⌨️keyboard-and-palette/🦀️.rs` (palette query) | Only partly mapped. | P2 |
| Preferences page (theme, density, language, metrics) | `E/🧱️elements/🎚️UiPreferences/🟦️.ts` (323 lines; `UI_PREFERENCES_CONFIG_SCHEMA` at 9; `readUiPreferences` 94); `E/🧱️elements/🏛️ShellHost/🎨️theme-driver-settings/🟦️.tsx`; `🎛️platform-prefs/🟦️.ts` | `S/🎛️prefs/🎨️theme-file-import-and-accessibility/🦀️.rs:251` (theme color mixing); `S/🎛️prefs/🔑️storage-prefs-and-i18n/🦀️.rs`; `S/🎛️prefs/📎️local-folders/` | Theme and language prefs exist. Density is MISSING on both sides as far as grep shows (React: no `density` symbol in UiPreferences or platform-prefs; wgpu: 2 hits, both unrelated). Metrics customization not located on either side. | P1 |
| Theme application | `R/🎨️chrome-classes/🟦️.ts` (52 lines, no literal hex); design tokens at `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/` | Theme tokens used about 948 times across `S` | Strong token usage on the wgpu side. Visual match per token unverified. | P2 |
| Locale EN / DE | `E/🧱️elements/🏛️ShellHost/🌐️locale-picker/🟦️.tsx` (`ShellLocalePicker`); inline `{ en, de }` strings (example `E/🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts:424`) | About 106 `de`/`Locale` references in `S` | Both sides have DE. Coverage of every string not checked. | P2 (unverified) |
| Tutorial / tour bar | `R/🎥️tutorial-ui/🟦️.tsx:73` (`TutorialBar`), `:169` (`TutorialCaptions`), `:191` (`TutorialVideoOverlay`), `:227` (`TutorialGhostPointer`); engine `R/🎞️tutorial-engine/` | `S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:475` (`render_tutorial_bar_step`); `S/🫧️surfaces/🎓️tutorial-runtime/🦀️.rs:74` (`tutorial_bar_height`); gesture overlay `S/🪝️window-actions/🎓️tutorial-runtime/🦀️.rs:134` | Bar and gesture overlay exist. React video overlay and captions have no matching wgpu symbol (grep for `video` / `caption` not run in full, so unverified). Ghost pointer mapped only to gesture overlay. | P1 (unverified) |
| Intro / tour overlay boxes | `R/🎓️introduction/🟦️.tsx:376` (`max-w-sm` = 384 px at 16 px root); `:105` (`INTRODUCTION_DOCK_REACH_PX = 40`), `:108` (`INTRODUCTION_CLEAR_RANGE_PX = 120`); `R/📐️introduction-geometry/🟦️.ts:206` (`INTRODUCTION_INFO_BOX_GAP_PX = 0`) | `S/🗨️view-overlays/🗨️chrome-overlays-and-tour/🦀️.rs:97` (`INTRODUCTION_INFO_BOX_GAP = 16.0`); `:142` (`INTRODUCTION_BOX_MAX_WIDTH = 384.0`); `:144` (`MIN = 240.0`) | Max width 384 matches `max-w-sm`. The info-box gap disagrees: React 0 px (`INTRODUCTION_INFO_BOX_GAP_PX = 0`) vs wgpu 16.0. Dock reach 40 and clear range 120 not found on wgpu side (unverified). | P1 (gap value to confirm) |
| Tooltips | React tooltip component not located by name. Chrome tooltips in `E/🧱️elements/🏛️ShellHost/` (not pinned). | `S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:709` (`render_chrome_tooltip_step`); `S/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs:525` (`render_chrome_tooltip`); registration `S/🎚️chrome-state/🗨️chrome-overlays-and-tour/🦀️.rs:633` (`register_tooltip`) | Tooltips exist. Delay timing (`chrome_tooltip_ready`) and styling unverified against React. | P2 (unverified) |
| Toasts / transient notices | `E/🧱️elements/🏛️ShellHost/🍞️transient-notice/🟦️.tsx:9` (`TransientNotice`); `E/🧱️elements/🏛️ShellHost/🧱️notice-bands/🟦️.tsx` (`NoticeStack`, `NoticeCell`, `FloatingPane`) | `S/🍱️view-chrome/🗨️chrome-overlays-and-tour/🦀️.rs:15` (`transient_notice_rect`); `S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:831` (`render_transient_notice_step`); `S/🖍️view-paint/🗨️chrome-overlays-and-tour/🦀️.rs:15,19,28` (`TRANSIENT_NOTICE_MAX_WIDTH = 520`, tone mapping) | Notices exist. `NoticeStack` stacking (React) has no found wgpu counterpart. Width 520 not matched to a React value. | P1 (unverified) |
| Context menus | React context menu not located by name. | `S/📄️docs/🧰️window-actions-and-commands/🦀️.rs:104` (`KeyAction::ContextMenu`); `S/🪀️view-panels/🎨️…/🦀️.rs:17` (`HitKind::ContextMenu` to `menuitem`); right-click state (`RightClickState` in `S/🕹️input/…`) | Context menu exists in wgpu. React menu look and keyboard flow unverified. | P2 (unverified) |
| Hotkeys / keyboard shortcuts | `R/🔥️hotkeys/🟦️.ts:9` (`useActionHotkey`); `:31` (`requiredPanelToggleHotkey`); `:34` (`PANEL_TOGGLE_HOTKEYS`) | `S/🪀️view-panels/🎨️…/🦀️.rs:90,242` (`shell_control_hotkey_badge`); `S/🕹️input/⌨️keyboard-and-palette/🦀️.rs`; `S/🖐️pointer/⌨️keyboard-and-palette/⌨️keyboard/🦀️.rs` (650 lines) | Shortcut table exists. Panel-toggle chord table and wgpu table not diffed. | P1 (unverified) |
| Selection marquee (chrome) | `R/🔲️selection-marquee/🟦️.tsx:26` (`SelectionMarquee`); `:45` (`SELECTION_DRAG_DIRECTION_THRESHOLD_PX = 2`); re-exported at `R/🟦️.tsx:323` | MISSING for chrome. Marquee symbols exist only for canvas gizmo in UI wgpu crate (`🖍️draw/🏷️types/🎯️marquee-gizmo-and-icon-atlas/🦀️.rs`) and world touch (`E/🎯️targets/🧊️wgpu/🧊️renderer/…/🖐️world-touch/🦀️.rs`). | React shell usage of `SelectionMarquee` not confirmed (only TiledMapHost has its own marquee). If the shell uses it, it is MISSING. | P1 (unverified) |
| First-run / onboarding | `E/🧱️elements/🎓️HubFirstRun/🟦️.tsx:41` (`HubFirstRun`), `:50` (`hubFirstRunStageAtIndex`) | MISSING. Zero hits for `first_run`, `FirstRun`, `HubFirstRun`, or `onboard` in `S`. | Entire first-run flow absent. | P0 |
| Sign-in / hub connection | `E/🧱️elements/🔗️HubConnection/🏛️workspace/🟦️.tsx`; `E/🧱️elements/🏛️ShellHost/🔐️identity-binding/🟦️.ts` | `S/📒️directory/🏛️space-admin-and-settle/🦀️.rs:478` (`HUB_SIGN_IN_DEADLINE_MS = 30_000`), `:482` (`ShellHubSignInAnswer`); `S/📒️directory/📨️command-dispatch/🦀️.rs:207` (`persist_hub_connection_book`) | Sign-in state logic exists. Sign-in UI and first-run stage UI are not confirmed. | P0 (UI part) |
| Presence (peer cursors, editing peers) | `E/🧱️elements/🏛️ShellHost/👥️presence-scope/🟦️.ts` | `S/📒️directory/🖼️chrome-frame-and-main-window/🦀️.rs:32` (`board_peer_overlays`); `S/📒️directory/⏪️time-travel/🦀️.rs:46,70` (`time_travel_peer_presence`) | Presence is painted on the board and in time-travel. Peer chrome (navbar avatars, if any) unverified. | P2 (unverified) |
| Close guard / unsaved marker | `E/🧱️elements/🏛️ShellHost/🚪️close-guard/🟦️.tsx` (`DocumentCloseDialog`, `UnsavedMarker`) | `S/📂️doc-open/🗨️chrome-overlays-and-tour/` and dialog staging (not line-checked) | Logic appears present. Dialog look unverified. | P2 (unverified) |
| Animations and transitions | React CSS transitions (not enumerated) | About 25 hits across 16 files in `S` (`Fade`/`anim`/`transition`), mostly non-chrome | Likely absent or minimal on wgpu chrome. Unverified. | P1 (unverified) |
| Focus styles | `R/🪟️chrome/`, Tailwind focus classes (not enumerated) | `focus_ring`/`FocusRing`: 8 hits in 3 files in `S` | Focus ring is thin. Unverified. | P2 (unverified) |
| Accessibility (a11y tree) | React ARIA attributes (not enumerated) | `S/🪀️view-panels/🎨️…/🦀️.rs:17,23,350` (roles and palette/notice nodes); `E/🎯️targets/🧊️wgpu/♿️native-accessibility/` | Accessibility nodes exist for palette and notices. Full coverage unverified. | P2 (unverified) |
| Selection-based utility panels, panel resize grip | `R/🧲️panel-dock/` (drag handles) | `S/⚓️dock/🔖️identity-and-types/🦀️.rs:9` (`PANEL_CHROME_GRIP_SIZE = ICON_TREE_ROW`) | Grip size is taken from shared layout. Drag behavior unverified. | P2 |
| Agent overlays | Not inventoried | `S/` agent overlay tests (`🧪️wgpu-agent-overlays` in ShellHost tests folder) | Not audited. | P2 (unverified) |

## 2. Ad-hoc code findings (wgpu shell)

Counts are from grep over `S` and the crate roots.

1. **Staging layout (structural).** 305 of 308 Rust files sit under `📥️staging/`. Crate facades mount them with `#[path]`. See the architecture section. This name is itself a "staging leftover". The React tree also has a `📝️staging/🟦️.tsx` (`StagedActionDialog`) under `E/🧱️elements/🏛️ShellHost/`.
2. **Staging name leftover.** `S/🔀️actions/🎬️shell-actions/🎯️dispatch-action/⌨️staging/🦀️.rs` is a folder literally named `staging` inside `staging`.
3. **Duplicated concerns.** 27 concern names repeat across folders (see above). Matching `🦀️.rs` names in several folders (for example `🗨️chrome-overlays-and-tour` in 16 folders) mean one behavior can have several partial copies. Risk: fixes applied to one copy and not others.
4. **Hardcoded pixel constants.** 57 bare numeric `const` items in `S`. Examples not derived from the shared theme or metrics:
   - `S/🖍️view-paint/🗨️chrome-overlays-and-tour/🦀️.rs:15` `TRANSIENT_NOTICE_TOP_GAP: f32 = 8.0`
   - `S/🖍️view-paint/🗨️chrome-overlays-and-tour/🦀️.rs:19` `TRANSIENT_NOTICE_MAX_WIDTH = 520.0`
   - `S/🖍️view-paint/🗨️chrome-overlays-and-tour/🦀️.rs:57` `PLUGIN_INSTALL_TOP_GAP = 8.0`
   - `S/🚶️session/🔖️identity-and-types/🦀️.rs:95` `SHELL_PALETTE_DESKTOP_MAX_WIDTH_PX = 512.0`
   - `S/🎛️prefs/⏪️time-travel/🦀️.rs:11` `TIME_TRAVEL_BAND_MAX_WIDTH = 880.0`
   - `S/🎛️prefs/📎️local-folders/🦀️.rs:26` `FOLDER_BAND_MAX_WIDTH = 720.0`
   - `S/🗨️view-overlays/🗨️chrome-overlays-and-tour/🦀️.rs:142,144` intro box `384.0` and `240.0`
   - `S/🪝️window-actions/🧰️window-actions-and-commands/🦀️.rs:12` `WINDOW_PANE_BODY_WIDTH_PX = 300.0`
   - `S/🖍️view-paint/🔖️identity-and-types/🦀️.rs:7` `TAB_INSERT_PREVIEW_WIDTH_PX = 2.0`
   - Several of these are per-band widths with no shared token. Each should be a theme metric or a named metrics entry.
5. **Mixed sourcing of metrics.** Some constants pull from shared code (`semio_ui_wgpu_layout::chrome::ICON_TINY`, `ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX` in `S/🕹️input/🎨️…/🦀️.rs:35`). Others are literals. The mix makes parity drift likely.
6. **Hardcoded colors.** Only two `Rgba::new` literals found in `S`: `S/🖍️view-paint/🧱️chrome-paint-and-navbar/🦀️.rs:40` (`LOGO_UNTINTED = Rgba::new(1,1,1,1)`, intentional) and `S/🎛️prefs/🎨️…/🦀️.rs:251` (computed mix, fine). Theme usage is about 948 references, so color handling is mostly tokenized.
7. **Dead code.** Zero `#[allow(dead_code)]` and zero `#[deprecated]` in `S`. Not proven dead. Unused-item detection requires `cargo check` (unverified).
8. **TODO / FIXME / XXX / HACK.** Zero hits across the wgpu shell tree (both `S` and the `🧊️wgpu` crate directory).
9. **Legacy / "migration" wording.** `🍱️shell-view-chrome/🦀️.rs` header says the staging modules are mounted "exactly as the shell parent mounted them". Crate doc comments carry plan labels such as `RENDERER-WP4` and `⏱️ P1e`. These reference ticket steps and should be cleaned up once staging is retired.
10. **Crate boundary guard weak.** `maxSourceFiles = 32` and `maxActiveLines = 4000` are declared in each facade `Cargo.toml`. The real source sits outside the crate directory, so the limit does not cover it.
11. **Unscoped cross-crate imports.** `🍱️shell-view-chrome/🦀️.rs` imports from about 20 sibling shell crates and carries `#[cfg(any(test, feature = "testkit"))]` reexports. The wide surface is a coupling risk.

## 3. Prioritized fix work items

Priority order: P0 first. Each item names the wgpu target and the React reference.

1. **P0. First-run flow (HubFirstRun).** Build the wgpu equivalent of `E/🧱️elements/🎓️HubFirstRun/🟦️.tsx` (stage model at `:50`, `hubFirstRunStageAtIndex`). Add it in `S/🪝️window-actions/` or `S/📒️directory/` and mount it from the chrome view in `S/🍱️view-chrome/🗨️chrome-overlays-and-tour/`. Reuse the existing `ShellHubSignInAnswer` (`S/📒️directory/🏛️space-admin-and-settle/🦀️.rs:482`) for sign-in.
2. **P0. Sign-in UI.** Pair the existing sign-in state (`HUB_SIGN_IN_DEADLINE_MS`, `S/📒️directory/🏛️space-admin-and-settle/🦀️.rs:478`) with a visible dialog that matches `E/🧱️elements/🔗️HubConnection/🏛️workspace/🟦️.tsx`.
3. **P1. Introduction info-box gap.** Resolve the disagreement: React `R/📐️introduction-geometry/🟦️.ts:206` (`INTRODUCTION_INFO_BOX_GAP_PX = 0`) vs wgpu `S/🗨️view-overlays/🗨️chrome-overlays-and-tour/🦀️.rs:97` (`16.0`). Confirm the intended value in the browser and fix the wgpu constant. Also locate or add the React `INTRODUCTION_DOCK_REACH_PX` (40) and `INTRODUCTION_CLEAR_RANGE_PX` (120) (`R/🎓️introduction/🟦️.tsx:105,108`) on the wgpu side.
4. **P1. Density preference.** Confirm whether React has a density control (none found in `E/🧱️elements/🎚️UiPreferences/🟦️.ts` or `E/🧱️elements/🏛️ShellHost/🎛️platform-prefs/🟦️.ts`). If it exists in React, add it to `S/🎛️prefs/🔑️storage-prefs-and-i18n/🦀️.rs`. If it does not, drop it from the parity list.
5. **P1. Shell selection marquee.** Confirm whether `R/🔲️selection-marquee/🟦️.tsx` is used by ShellHost. If yes, add a chrome marquee to the wgpu `S/🖐️pointer/🖱️pointer-input/` path, reusing the 2 px threshold (`:45`).
6. **P1. Panel drop indicator.** Compare the eight-anchor drop zones in `R/🧲️panel-dock/🟦️.tsx` (lines 12-13 and the drop surfaces near line 170) with `S/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs:352-383`. Add anchor-zone indicators to match.
7. **P1. Maximize and reopen parity.** Locate the React maximize and reopen behavior (none found under `R/🧲️panel-dock` or `E/🧱️elements/🏛️ShellHost/🏗️dock-assembly`). Confirm and align with `S/🚶️session/🧩️session-and-topology/🦀️.rs:214-233`.
8. **P1. Toast stacking.** Map React `NoticeStack` and `NoticeCell` (`E/🧱️elements/🏛️ShellHost/🧱️notice-bands/🟦️.tsx`) to a wgpu stacking routine in `S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:831`. Confirm the 520 px max width against React.
9. **P1. Tutorial video, captions, ghost pointer.** React `R/🎥️tutorial-ui/🟦️.tsx:169,191,227` has no confirmed wgpu equivalent beyond the gesture overlay. Confirm and add.
10. **P1. Titlebar and fullscreen layout.** Compare `R/🖥️fullscreen/🟦️.tsx:151` (`DesktopTitlebar`) with the wgpu navbar title and fullscreen icon (`S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:250`).
11. **P1. Shortcut table diff.** Diff `R/🔥️hotkeys/🟦️.ts:34` (`PANEL_TOGGLE_HOTKEYS`) and the `SHELL_KEYBINDINGS` source against the wgpu shortcut table (`S/🪀️view-panels/🎨️…/🦀️.rs:90,242`).
12. **P1. Animations.** Enumerate React transitions in the chrome classes and add a timing module on wgpu, or record the gap as accepted.
13. **P2. Consolidate staging.** Merge the 27 duplicated concern names into one folder per concern, then remove `📥️staging` and the `⌨️staging` folder name. Keep crate `Cargo.toml` files as the mount points.
14. **P2. Replace hardcoded widths.** Move the bare width and gap constants listed in section 2 (item 4) into a metrics table next to `semio_ui_wgpu_layout::chrome`. Start with `TRANSIENT_NOTICE_MAX_WIDTH`, `SHELL_PALETTE_DESKTOP_MAX_WIDTH_PX`, `TIME_TRAVEL_BAND_MAX_WIDTH`, `FOLDER_BAND_MAX_WIDTH`.
15. **P2. Fix the crate size guard.** Make `maxSourceFiles` and `maxActiveLines` count the real staged source, or drop the guard after consolidation.
16. **P2. Verify metric parity in a browser.** Capture computed styles of the React chrome (tab bar, footer, notices, intro box) and compare to wgpu layout values. Everything marked "unverified" in the table depends on this.

## Unverified items (summary)

- Visual metrics, colors, fonts, and spacing for every row (no pixel or computed-style comparison done).
- Animation and transition parity.
- Whether React ShellHost uses `SelectionMarquee`.
- Whether React has a density or metrics-customization control.
- React tooltip, context menu, and maximize component locations.
- Full i18n coverage (EN and DE) on both sides.
- Dead-code status of wgpu items (needs `cargo check`).
