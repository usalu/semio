# G1 Sketchpad Report

Ticket: `26/08/09/PUZZLE-DESIGN-PARITY`  
Wave: 6 G1  
Owner file: `compose/client/lib/sketchpad/js/index.ts`

## Changes

1. **`sketchpadConnectionTransformParamsFromDto`**
   - Keeps `gap` / `shift` / `rise` / `rotation` / `turn` / `tilt`.
   - Maps compose connection `u`/`v` → puzzle fastener/edge/attraction `x`/`y`.
   - Prefers explicit `x`/`y` when already present; never emits `u`/`v`.

2. **`sketchpadDesignPuzzle2dFixtureFromDesign` / `sketchpadDesignVolumeFixtureFromDesign`**
   - Connection params now include `x`/`y` via the transform helper.
   - Nodes/objects emit `anchor: "fixed" | "derived"` from compose `connectionKind` (`FIXED`→`fixed`, `CONNECTED`→`derived`), falling back to authored `position` presence.
   - Diagram/volume seeds use authored `position` only for **Fixed** pieces; Derived stay at origin/identity so puzzle-owned flatten computes centers/planes.
   - Never seeds from `flatPosition`.

3. **GraphQL kit read** (`SKETCHPAD_KIT_READ_INNER`)
   - Selects `connectionKind` on pieces so live kits can carry accurate anchors.

4. **Embedded vitest**
   - Extended `sketchpadDesignVolumeFixtureFromDesign` with Fixed-seed / `u`→`x` assertions; existing puzzle-5d flatten case now also asserts edge `x`/`y`.

## Test results

### Ticket-local helper + source contract check
Command: `bun g1-sketchpad-check.mjs` (ticket folder)  
Result: **PASS** (`🧪g1-sketchpad-check-result.json`)

Covered:
- `u`/`v` → `x`/`y` (no `u`/`v` emission)
- Fixed/Connected → `fixed`/`derived`
- Authored pose ignores `flatPosition`
- Fixture builders Fixed-only seed + `anchor` source contracts

### Package vitest (`bun ./📜️script.ts test` / `vitest.config.ts`)
Result: **blocked (pre-existing)** — `@semio-tech/framework-platform-core` resolves to missing path `framework/product/platform/core/index.ts` (vitest/tsconfig aliases stale; `runVitest` also looks for missing `🧪vitest.config.ts`). Not introduced by G1.

## Files changed

- `compose/client/lib/sketchpad/js/index.ts`
- Ticket report + scratch under `PUZZLE-DESIGN-PARITY/` (`🧪g1-sketchpad-report.md`, check harness, logs)
