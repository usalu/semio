# Celebrate all UI elements when introduction finishes

## Behaviour
- Per-step completion still stamps only the step's introduce / interaction celebrate target via `celebrateElements`.
- When the introduction **completes** (Done on the last informational step, or finishing the last interaction-gated step), `dismissIntroduction(true)` calls `celebrateAllElements()`.
- Skip / Escape calls `dismissIntroduction(false)` and does **not** celebrate.

## Helper
`celebrateAllElements` in `ui/js/react/index.tsx` stamps every mounted element whose `id` (or any `data-element-alias` token) matches the UI element-id grammar, excluding `ui.introduction.*` chrome that unmounts with the tour.
