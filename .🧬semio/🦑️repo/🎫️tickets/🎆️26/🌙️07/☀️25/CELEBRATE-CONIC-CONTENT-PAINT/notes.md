# Celebrate Conic Content Paint

## Problem
Celebrate (`data-celebrated="true"`) only painted the spinning conic ring on `::after`. Text, icons, and drag handles kept solid emphasized/element colors.

## Fix
- `--celebrate-conic` on `[data-celebrated="true"]` host; spin on host, burst on `::after`.
- `@property --celebrate-border-angle` now `inherits: true`.
- Leaf hosts paint text via `background-clip: text`, stroke icons via `destination-in` on SVG-owning `[data-icon]` nodes only.
- Celebrated drag handles override hover-scope emphasized color.
- Window/dock shells excluded from leaf list.

## Foreground-only follow-up
- Removed `tree-icon` / `drag-handle` from icon blend hosts (they wrapped `[data-icon]` and leaked conic as a fill).
- Celebrated tree rows paint elbow/stem/guide-line strokes with `--celebrate-conic`.
- Ancestor `IndentationLines` on branch content `:has()` a celebrated row spin the same conic on guide lines.

## Icon mask follow-up (2026-07-31)
- Root cause: `mix-blend-mode: destination-in` is invalid in CSS; oversized `::before` conic rectangles showed as background fills on icons and drag grips.
- `Icon` stamps `--icon-mask` (memoized `iconMaskImage()` data URI) on SVG-backed wrappers.
- `CelebrateContent` paints SVG icons via `mask-image: var(--icon-mask, transparent)` and hides inner `svg`; glyph kinds use `background-clip: text`.
- Vitest: celebrate CSS contract + icon markup; Playwright fixture `celebrate-foreground-proof.mjs` logs `[DEBUG]` computed paint and writes `celebrate-foreground-proof.png`.
- Storybook build/dev failed pre-existing (`MainFileEvaluationError` in `.storybook/main.ts`); browser regression added to `.storybook/ui-new-stories.spec.ts` for when static build is healthy.
