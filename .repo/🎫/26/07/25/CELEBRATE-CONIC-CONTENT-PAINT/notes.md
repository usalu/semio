# Celebrate Conic Content Paint

## Problem
Celebrate (`data-celebrated="true"`) only painted the spinning conic ring on `::after`. Text, icons, and drag handles kept solid emphasized/element colors.

## Fix
- `--celebrate-conic` on `[data-celebrated="true"]` host; spin on host, burst on `::after`.
- `@property --celebrate-border-angle` now `inherits: true`.
- Leaf hosts (buttons, toggles, tabs, tree rows, introduction labels, pane chrome) paint text via `background-clip: text`, stroke icons/grips via `destination-in` blend.
- Celebrated drag handles override hover-scope emphasized color.
- Window/dock shells excluded from leaf list.
