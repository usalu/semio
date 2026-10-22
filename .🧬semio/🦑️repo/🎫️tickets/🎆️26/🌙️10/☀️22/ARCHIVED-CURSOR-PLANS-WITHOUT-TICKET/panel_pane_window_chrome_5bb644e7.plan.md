---
name: Panel Pane Window Chrome
overview: "Bring `Panel` and `Pane` to full visual and behavioral parity with window chrome: focus-driven active border instead of a permanently red one, correct z-level, unclipped silhouette, window-style multi-tab pills with an aligned U-cutout, and a fold control cap."
todos:
  - id: ticket
    content: Read repo://goals and open a ticket for panel/pane window-chrome parity
    status: completed
  - id: surface-active
    content: Add useSurfaceActive hook (pointerdown + focusin, JS only) and wire Panel/Pane active props to it
    status: completed
  - id: level-shape
    content: Replace magic zIndex defaults with getLevelZClass tokens and change Panel root to overflow-visible
    status: completed
  - id: window-tabs
    content: Restyle panel-variant tabs to modeDockInactiveTabClass/BeforeGap/ActiveTab, drop w-full and the divider stroke so the U-gap opens after the last tab
    status: completed
  - id: dedupe-active-class
    content: Collapse panelTabActiveClass into modeDockActiveTabFillClass
    status: completed
  - id: fold-control
    content: Add fold-only controls cap to Panel and Pane using ui.common.collapse
    status: completed
  - id: css-cleanup
    content: Narrow the chrome-frame hover selector in ui.css to the mobile panel
    status: completed
  - id: tests
    content: Extend existing vitest blocks for active-on-focus, z level, multi-tab pills, fold control, and CSS assertions
    status: completed
  - id: verify
    content: Run ui/js/react typecheck and tests via nx and confirm a resting panel shows the gray border
    status: completed
isProject: false
---

# Make Panels and Panes Match Window Chrome

All component work is in [ui/js/react/index.tsx](ui/js/react/index.tsx); styling in [ui/styling/js/ui.css](ui/styling/js/ui.css).

## Verified root causes

- **Border always red.** `Panel` hardcodes `active` (line 19727) and `Pane` uses `active={!effectiveFolded}` (line 20007), so `WindowChromeSilhouetteBorder` always resolves `data-kind="active"` and strokes `var(--active-base)`. A window only does this when `stackGloballyActive = Boolean(activeId && activeWindowId === activeId)` (line 26501). As a side effect the hover-emphasis rule in `ui.css` (6805-6810), guarded by `:not([data-active="true"])`, is dead for panels and panes.
- **Wrong level.** `Panel` defaults `zIndex = 20` (line 19651) which is `--z-pane`, not `--z-panel: 30`. `getLevelZClass()` (7163-7177) already maps `panel -> z-panel` but nothing uses it.
- **Flat tab strip.** `WindowChrome` renders `titleChips` into one glass `window-chrome-chip-cap` (8938-8940). `Panel` puts the whole `PanelTabBar` there, so tabs are `panelAnchorTabButtonClass` buttons separated by `border-e` (7292-7299). Window tab chrome lives only in `ModeDockTabBar` (26604-26644): per-tab cells, `modeDockInactiveTabClass` pills vs. borderless `modeDockActiveTabClass`.
- **Clipped silhouette.** `Panel`'s root is `overflow-hidden` (line 19719); `ModeDockStack` (26727) and the `WindowChrome` stack (8932) are `overflow-visible`.
- **No controls cap.** `WindowChrome` supports `enlarge`/`close` (8942-8969); `Panel` and `Pane` pass neither.
- **Dead second border system.** `[data-slot="chrome-frame"]` hover CSS (6801-6803) targets panel and pane, but only `MobilePanel` still emits that slot.

## 1. Focus-driven active state

Add a `useSurfaceActive(ref)` hook in a new subregion beside the `WindowChrome` region. It installs document capture listeners for `pointerdown` and `focusin` and reports whether the last such event target is inside the surface root. Because only one node can be the last-interacted node, this is exclusive by construction and mirrors the "last-clicked window wins" semantics of `dock.activateWindow` (26889).

- `Panel`: `active={surfaceActive}` instead of `active`.
- `Pane`: `active={!effectiveFolded && surfaceActive}`.

This must stay JS-only. The `ShellParentHover` region comment (`ui.css` 6199-6206) forbids `:focus-within` for border emphasis, and tests at lines 28399-28414 and 35935-35939 assert it is absent. Do not add any `:focus-within` CSS rule.

## 2. Level and shape

- Drop the magic `zIndex = 20` default on `Panel` (19651) and `Pane` (19947). Apply `getLevelZClass("panel")` / `getLevelZClass("pane")` to the root `className`, and only write an inline `zIndex` when a caller passes one explicitly. The `--z-*` tokens become the single source.
- Change `Panel`'s root `overflow-hidden` to `overflow-visible` (19696 and 19719) so the silhouette stroke and per-tab pills are not clipped.

## 3. Window-style tabs with an aligned U-cutout

Restyle the `"panel"` variant of `PanelTabButton` / `PanelTabRow` (7778-7922) to the mode-dock vocabulary, keeping the existing `PanelTabBar` / `Ribbon` structure:

- inactive tab -> `modeDockInactiveTabClass` (8692)
- last inactive tab before the gap -> `modeDockInactiveTabBeforeGapClass` (8695), mirroring `inactiveTabChromeClass` (26523-26526)
- active tab, when `showActiveColor` -> `modeDockActiveTabClass` (8716)

Two supporting changes:

- `panelTabBarBaseClass` (7280) carries `w-full`, which makes the chip cap span the full width and starves the U-gap. Split it: the `"panel"` variant drops `w-full` so the cap hugs its tabs, matching `mode-dock-tabs` (`flex min-w-0 items-stretch justify-start`, line 26650). The `"chrome"` and `"mobile"` variants keep `w-full`.
- `panelAnchorTabBarClass` (7302-7304) adds a top/bottom divider stroke. Remove it for the `"panel"` variant: with per-tab pills the silhouette and pill borders own every stroke, the same way `ui.css` 7066-7075 zeroes borders inside the dock stack.

No silhouette-path work is needed. `measureWindowSilhouetteMetrics` derives `tabsWidth` from the gap element's left edge, so once the cap hugs its tabs the cutout automatically opens right after the last tab, exactly as it does for a window.

Also collapse `panelTabActiveClass` (8713) into `modeDockActiveTabFillClass` (8710) - both are `interactiveActiveFillClass` under two names, which is what let the two systems drift.

## 4. Fold control cap

Give both surfaces the window cap shape `[chip cap][gap][controls cap]`, with fold only, no enlarge:

- `Panel` (visible branch): `close={{ id, slot: "panel-fold", icon: <CloseIcon className="size-small" />, label, onClick: () => onVisibleChange?.(false) }}`.
- `Pane` (unfolded): the same with `slot: "pane-fold"` and `onClick: onFoldToggle`.

Label comes from the existing `useLabel("ui.common.collapse")` key (already used at line 25545), so no schema change. Keep re-clicking the active root tab (`usePanelTabSelection`, 7621-7654) and clicking the pane's `WindowPaneChromeToggle` as shortcuts.

## 5. Cleanup

Narrow the `chrome-frame` hover selector at `ui.css` 6801-6803 to the mobile panel, since `[data-slot="pane"]` no longer emits that slot. `MobilePanel` (20075-20094) stays full-bleed with its rectangular frame - it has no floating silhouette to be consistent with.

## 6. Tests and verification

Extend the existing vitest blocks in [ui/js/react/index.tsx](ui/js/react/index.tsx) (no new test files):

- Panel silhouette is `data-kind="normal"` at rest and flips to `active` after a `focusin` inside it.
- Panel root resolves to the panel z token, not 20.
- A panel with two or more tabs gives inactive tabs the pill class and the active tab the fill class, and leaves a non-zero `window-chrome-gap`.
- Panel and pane render `window-chrome-controls` with the fold button, and clicking it folds.
- CSS assertions: no `:focus-within` rule was added for panel/pane borders (keeps 35939 green), and the `chrome-frame` hover selector no longer mentions `[data-slot="pane"]`.

Existing tests that assert the old flat-strip classes or the `pane-chrome` slot (around line 33231) will need updating rather than duplicating.

Verify by running the `ui/js/react` typecheck and test targets through nx, and confirm the rendered result against the reported symptom (a panel at rest must show a gray border, not the red active one).

## Ticket

Read `repo://goals` first, then open a ticket for this work (the related [.repo/🎫️/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/ticket.json](.repo/🎫️/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/ticket.json) covers the surface-level formula, not window chrome parity). Keep all scratch output inside the ticket folder.