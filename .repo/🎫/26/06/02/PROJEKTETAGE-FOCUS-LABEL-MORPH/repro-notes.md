# Repro notes

## Root cause
`.reveal .presentation-morph-source` used `visibility: hidden !important` at rest. reveal.js FLIP measurement treats hidden targets as non-animatable; focus tiles were unmatched and faded instead of morphing into per-tile ghosts on `catalogue-labels`.

## Fix
Rest ghosts with `opacity: 0` only; keep `visibility: visible` so label-position ghosts remain measurable. Pending/running rules unchanged (ghosts visible during morph, fade out; labels fade in).

## Tests
- framework/product/presentation/renderer/react: 64 passed
- framework/product/presentation/core: 40 passed
- Integration: `auto-animates catalogue focus into inline column labels`, `fires reveal auto-animate when advancing projektetage focus to labels`
