# Demonstrator overview cards — window silhouette chrome

Date: 2026-09-16.

## Problem

`DemonstratorCard` on the landing overview used a standalone rounded `ui-glass` box with a large centered icon. That diverged from the rest of the OS shell, where surfaces use `WindowChrome` U-cutout silhouettes and cap-row chips (icon + label), with drag handles only on movable chrome.

## Change

- New `♻️mit-bestand/🧺️demonstrator/⚛️demonstrator-card.tsx` renders each pane picker with `WindowChrome` (`level="dialog"`).
- Title chip: `windowChromeTitleChipClass` + pane `Icon` + label — no `DragHandle`, no close/enlarge controls.
- Body: tagline + “Demonstrator öffnen” affordance inside the silhouette body glass.
- `data-demonstrator-pane-card` / `data-pane-id` for acceptance coverage.

## Follow-up (hover / glass / outline)

- Hover was wired to `WindowChrome` `active`, which paints the **primary** silhouette stroke — replaced with `lifted` (transform only) and `active={false}` so hover uses the normal → emphasized CSS stroke.
- Removed `windowBodyFrameClass` (`ui-surface`) and `shadow-lg` from the body plane; glass stays on chip-cap + `window-chrome-body-surface` (`ui-glass`), matching introduction/context menu.
- `🎨️globals.css`: transparent stack rule for `demonstrator-pane-card-stack`.

## Verification

- Playwright: `demonstrator overview: pane cards use window-silhouette chrome without drag handles` in `🧪️tests/🎭️acceptance/🟦️.ts`.
