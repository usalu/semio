# Celebrate Tab Label Foreground

## Problem
Panel/chrome tab labels did not show the celebrate conic on the text (icons did after the mask fix). Labels stayed solid white/emphasized.

## Root cause
CelebrateContent painted labels via `> span` (direct child only). Window panel chrome tabs nest the label under `modeDockTabLabelClassName` (`div > span.truncate`), so the selector never matched.

## Fix
- Stamp `data-slot="inline-label"` on panel tab, mode-dock tab, pane chrome, button-group, and action-group label spans (toggle-group already had it).
- CelebrateContent paints `[data-slot="inline-label"]` with `background-clip: text` + `-webkit-text-fill-color: transparent` (same as tree-label).
- Include `window-pane-chrome-toggle` in celebrate leaf hosts for icons/labels.

## Proof
`celebrate-label-proof.mjs` logs `[DEBUG]` computed label/icon paint and writes `celebrate-label-proof.png`.
