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
