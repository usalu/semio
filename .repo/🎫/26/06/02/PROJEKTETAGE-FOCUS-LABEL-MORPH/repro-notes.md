# Repro notes

## Root cause
`.reveal .presentation-morph-source` used `visibility: hidden !important` at rest. reveal.js FLIP measurement treats hidden targets as non-animatable; focus tiles were unmatched and faded instead of morphing into per-tile ghosts on `catalogue-labels`.

## Fix
1. Rest ghosts with `opacity: 0` only; keep `visibility: visible` so label-position ghosts remain measurable.
2. Morph-source wrappers pin to `morphFrom` label frames (never ephemeral focus drag transforms).
3. `clearRevealAutoAnimateInlineLayout` strips only `transform`/`transition` on `[data-auto-animate-target]` (never `left`/`top`/`width`/`height` on morph-into labels).
4. Force layout on label-slide `.presentation-morph-source` nodes before focus→labels morph.

Expanded data already had label positions; DOM tests confirm `catalogue-labels--Stütze` uses inline label slot %, not focus column %.

## Tests
- framework/product/presentation/renderer/react: 64 passed
- framework/product/presentation/core: 40 passed
- Integration: `auto-animates catalogue focus into inline column labels`, `fires reveal auto-animate when advancing projektetage focus to labels`
