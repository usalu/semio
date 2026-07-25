---
name: Window Chrome Surfaces
overview: Extract ModeDock’s U-cutout window chrome into a shared primitive, then restyle Panel, Pane, introduction steps, and context menus to use it — with optional left chips and right controls so enlarge/close only appear when they earn a place.
todos:
  - id: extract-window-chrome
    content: Extract WindowChrome + generalized silhouette measure/border; refactor ModeDock to use it
    status: completed
  - id: pane-chrome
    content: Restyle Pane with WindowChrome (chip + U-cutout body; no enlarge/close)
    status: completed
  - id: panel-chrome
    content: Restyle open Panel with WindowChrome (tabs as chips; silhouette replaces chrome-frame)
    status: completed
  - id: intro-chrome
    content: Restyle introduction info box with WindowChrome (title+drag chip; Skip as close)
    status: completed
  - id: context-menu-chrome
    content: Restyle context menus with WindowChrome (title chip; no enlarge/close)
    status: completed
  - id: css-tests
    content: Update ui.css hover/stroke rules; extend existing vitest + fix stories if needed
    status: completed
isProject: false
---

# Window-Styled Chrome for Panels, Panes, Intro, Menus

## Goal

Make Panel, Pane, introduction info boxes, and context menus share the same visual grammar already used by Mode dock windows:

```
[ name chip(s) + drag ]====U cutout====[ enlarge? ][ close? ]
|                                                          |
|                      continuous body border              |
|__________________________________________________________|
```

Controls are **optional per surface**. Context menus get no enlarge. Surfaces without a close action omit close. Folded chips (pane/panel folded state) stay chip-only with no body silhouette.

## Shared primitive (extract from ModeDock)

Today the U-cutout lives only in Mode dock:

- Path: [`windowSilhouettePath`](ui/js/react/index.tsx) (~7686)
- Measure: [`measureWindowSilhouetteMetrics`](ui/js/react/index.tsx) (hardcoded `mode-dock-*` slots)
- SVG border: `ModeDockStackSilhouetteBorder` (~25476)
- Cap row: `ModeDockTabBar` (~25305)

**Extract a reusable `WindowChrome` region** in [`ui/js/react/index.tsx`](ui/js/react/index.tsx) (near existing window silhouette helpers ~7627):

- `WindowChrome` — stack root + silhouette SVG + cap row + body
- Props: `titleChips` (React nodes), optional `enlarge` / `close` actions, `body`, `active`, `className`, `dataSlot` root override
- Cap layout: left chip cluster | flex gap (`data-slot="window-chrome-gap"`) | optional controls cap
- Generalized measure slots (keep ModeDock slots as aliases or dual-query so existing dock tests keep working):
  - `window-chrome-stack` / `mode-dock-stack`
  - `window-chrome-gap` / `mode-dock-tab-gap`
  - `window-chrome-controls` / `mode-dock-controls-cap`
  - `window-chrome-cap` / `mode-dock-tabbar`
- Reuse `windowSilhouettePath`, border kinds (`normal` / `active` / `introduced` / …), and celebrated mask logic
- Drop rectangular `chrome-frame` layers for surfaces that adopt this; silhouette owns the outer stroke

ModeDockStack / ModeDockTabBar become thin wrappers over `WindowChrome` (same behavior, shared paint path).

## Per-surface mapping

| Surface | Left chip(s) | Enlarge | Close | Body |
|---------|--------------|---------|-------|------|
| **Mode dock** (unchanged behavior) | window tabs + drag | Focus/Unfocus when `canMaximize` | Close window | window canvas |
| **Panel** (open) | existing tab chips (+ drag where dock DnD already exists) | omit (no maximize target) | omit on panel itself — fold stays via active-tab toggle | tree/content; silhouette replaces `shellChromeFrameLayerClass` |
| **Pane** (unfolded) | icon + label + `DragHandle` (current toggle content) | omit (fold is not “enlarge”) | omit — fold stays via chip click / existing `onFoldToggle` | pane children; silhouette replaces rectangular frame |
| **Introduction step** | step title + `DragHandle` | omit | Skip/dismiss as close control | body text, logos, interactions, Back/Next footer |
| **Context menu** | short label chip (`Menu` / provided title) | **never** | omit (dismiss is outside-click / Escape) | menu items; U-cutout with `controlsWidth=0` |

Folded panel/pane: keep chip-only affordance (no body, no silhouette) — same as today’s folded chip, just styled as a window title chip.

Bottom-anchored panels (`flex-col-reverse`): keep content-above-chrome flow; silhouette path stays top-cap geometry relative to the chrome stack (cap still visually on the content-facing side via existing reverse flex).

## Implementation steps

1. **Extract `WindowChrome` + generalized silhouette measure/border** in `ui/js/react/index.tsx`; refactor ModeDock to consume it; keep existing ModeDock vitest assertions green (slot aliases OK).
2. **Pane** — replace fill/`chrome-frame` + `windowMeasuresChrome` header with `WindowChrome`; chip click still folds; drag stays on handle.
3. **Panel** — when `visible`, wrap tab strip + body in `WindowChrome` (tabs = left chips, gap, no right controls); remove rectangular `chrome-frame`; folded / chrome-hosted root row behavior unchanged in spirit (tabs remain fold toggles).
4. **Introduction** — replace `GLASS_OVERLAY_BOX_CLASS` box with `WindowChrome`; move Skip onto close control when present; keep Back/Next in body footer. Do **not** change `UIDialog` in this pass (it shares the old glass class today — leave dialogs rectangular until a follow-up).
5. **Context menu** — wrap `ContextMenu` / `ContextMenuController` content in `WindowChrome` with a title chip and no controls; drop `rounded-md` menu rectangle in favor of silhouette + body fill (`ui-glass-menu` stays on body/cap glass).
6. **CSS** in [`ui/styling/js/ui.css`](ui/styling/js/ui.css) — extend hover/emphasized stroke rules from `[data-slot="chrome-frame"]` to window-chrome silhouette borders for panel/pane/intro/menu roots; keep ModeDock hover rules working.
7. **Tests** — extend existing blocks in `ui/js/react/index.tsx` (Panel/Pane chrome, ContextMenu, UIIntroduction appearance, ModeDock silhouette) for: silhouette present, optional controls absent where specified, context menu has no enlarge, intro uses window chrome. Update Storybook stories only if snapshots/selectors break (`Panel`, `ContextMenu`, `UIIntroduction`, `Mode`).

## Out of scope

- Dialogs (`UIDialog`) — same glass box today; migrate in a later ticket
- Changing panel/pane fold semantics or Mode dock maximize/close behavior
- New files — stay in existing `index.tsx` / `ui.css` / existing tests & stories

## Ticket

On execute: list goals via MCP, open/reopen ticket under `R26-03`, bind this plan id, put temp logs under the ticket folder.