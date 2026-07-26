---
name: Intro Bottom Cutout
overview: Extend the window silhouette and WindowChrome with a mirrored bottom U-cutout, then move introduction Back/Next into bottom chips so the info box matches the top title/close chrome.
todos:
  - id: silhouette-bottom
    content: Extend WindowSilhouetteMetrics + windowSilhouettePath + measureWindowSilhouetteMetrics for bottom cutout
    status: completed
  - id: window-chrome-footer
    content: Add WindowChrome footerLeftChips/footerRightChips row with transparent gap
    status: completed
  - id: intro-footer-chips
    content: Move introduction Back/Next into bottom chrome chips; keep Skip on top close
    status: completed
  - id: css-tests-bottom
    content: CSS transparent footer-gap + vitest for path and introduction footer
    status: completed
isProject: false
---

# Introduction Mirrored Bottom Cutout

## Goal

Introduction steps keep the top U-chrome (title+drag | gap | Skip) and gain a **mirrored bottom cutout** for navigation:

```
[ title + drag ]========[ Skip ]
|                              |
|            body              |
|                              |
[ Back? ]========[ Next/Done? ]
```

Back sits left; Next/Done sits right; the gap between them punches through like the top gap. Only chips that exist render (first step: no Back; interaction-gated steps: no Next).

## Silhouette path + metrics

Today [`windowSilhouettePath`](ui/js/react/index.tsx) only notches the top:

```ts
return `M${x0},${y0} H${tabs} V${cap} H${gapEnd} V${y0} H${x1} V${y1} H${x0} Z`;
```

Extend [`WindowSilhouetteMetrics`](ui/js/react/index.tsx) with bottom fields (default `0` so ModeDock/panel/pane/context-menu stay top-only):

- `bottomLeftWidth`, `bottomRightWidth`, `bottomCapHeight`

Mirrored path (clockwise; bottom metrics `0` collapses to today’s path):

```
M x0,y0
H tabs V cap H (w-controls) V y0 H x1
V y1
H (w-bottomRight) V (h-bottomCap) H bottomLeft V y1 H x0
Z
```

Update [`measureWindowSilhouetteMetrics`](ui/js/react/index.tsx) to also read:

- `[data-slot="window-chrome-footer"]` (cap height)
- `[data-slot="window-chrome-footer-gap"]` (left edge → bottomLeftWidth)
- `[data-slot="window-chrome-footer-right"]` (right edge → bottomRightWidth)

Keep existing top selectors (`window-chrome-gap` / `mode-dock-*`) unchanged.

## WindowChrome footer API

In [`WindowChrome`](ui/js/react/index.tsx) add optional footer props mirroring the top cap:

- `footerLeftChips?: React.ReactNode`
- `footerRightChips?: React.ReactNode` (or a single right cluster)

When either is set, render a footer row under the body:

```
[footer-left chips] | footer-gap (transparent) | [footer-right chips]
```

Same glass rules as the top: glass only on chip cells, gap `bg-transparent` / no backdrop-filter (reuse the existing `[data-slot="window-chrome-gap"]` CSS guard for the footer gap slot).

Body stays between top cap and footer; no absolute inset fill.

## Introduction wiring

In `UIIntroduction` ([~6440](ui/js/react/index.tsx)):

- Remove the in-body Back/Next button row
- Pass footer chips into `WindowChrome`:
  - Left: Back chip (`stepIndex > 0`) styled like a window title chip / control button
  - Right: Next or Done chip (`advanceByButton`)
- Keep Skip as top `close`
- Keep step counter in the body (or move to a non-chip body header — leave in body)

Use the same control affordance grammar as top close (icon + tiny label), not full `Button` ghosts — so the chips read as chrome caps.

## Tests + CSS

- Extend existing silhouette path vitest for a bottom-cutout metrics case (and `0` bottom metrics ≡ old path)
- Extend introduction appearance tests: footer slots present, Back/Next not inside body footer row, gap transparent
- Add `[data-slot="window-chrome-footer-gap"]` to the transparent-gap CSS rule next to `window-chrome-gap` in [`ui/styling/js/ui.css`](ui/styling/js/ui.css)

## Out of scope

- Context menus, panels, panes, ModeDock (bottom metrics stay 0)
- Dialogs
- Changing Skip / hotkey behavior
