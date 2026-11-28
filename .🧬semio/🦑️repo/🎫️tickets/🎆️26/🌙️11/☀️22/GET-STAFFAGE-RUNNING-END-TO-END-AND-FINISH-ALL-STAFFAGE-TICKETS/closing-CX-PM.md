FINAL (2026-11-27 13:40: portable-math case green; Math.hypot replaced by the existing portable.hypot in both polyhedron files; kernel 32 files 730 passed; staffage digests/exports/bounds 143 passed, no re-pin needed)

# CX-PM: portable math in the polyhedron kernel

## Cause
`🎞️scenes` bans `Math.(sin|cos|tan|atan*|acos|asin|exp|expm1|log*|pow|cbrt|hypot|sinh|cosh|tanh)` outside `🧮️math/🎯️portable` and tests. Two files still used `Math.hypot`:
`M/✅️validation/🪨️polyhedron/🟦️.ts` (axis length, face-normal lengths, cross length) and `M/📐️bounds/🪨️polyhedron/🟦️.ts` (`unit`, Jacobi rotation `Math.hypot(1, delta)`).

## Cure
Both now `import * as portable from "../../🧮️math/🎯️portable/🟦️.ts"` and call `portable.hypot(...)`, the helper already used by `🧮️math/🟦️.ts` (`length`) and `✅️validation/🟦️.ts`. It already has fixtures and tests in `🧫️fixtures/🎯️portable-math` and `🧪️tests/🎯️portable-math`, so no new helper or fixture was needed. `✅️validation/🟦️.ts` (pending change by another agent) was not touched.

## Runs (through `repo capacity run`, pinned bun)
- Kernel `M/🧪️tests` (own config `U/🧰️matrix/CX-PM/vitest.config.mjs`, since the framework config needs `SEMIO_VITEST_POLICY`): 32 files, 730 passed, 1 skipped (skip pre-existing).
- Staffage `🧊️model-digests`, `🧊️model-exports`, `🧊️model-bounds`: 3 files, 143 passed. No digest shifted, so nothing re-pinned.
