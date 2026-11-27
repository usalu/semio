# Capacity status test expectations, 2026-11-26

## Verified behaviour (intentional)

- `repo capacity status` prints `Memory guard: no warden is running on this machine` when no warden runs. Source: `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🟦️.ts` (`formatHostWardenStatusV1`), schema key `statusAbsent`, and its own test in `.../🛡️warden/🧪️tests/🟦️.ts:648`. The command printer in `🛍️products/🦑️repo/.../host-capacity/⌨️command/🟦️.ts` always prints it.
- A critical sample with a feasible minimum now answers `pressure-critical`, not `starved`. Source: `.../📏️decision/🟦️.ts` (the `hard` branch refuses `starved` only when `infeasible`). Introduced in commit `9c8a98ec234`, which matches the `hostStarvedMillisecondsV1` docstring: a feasible request is never refused, it waits.

## Test changes

- `🧪️tests/🪟️partial/🟦️.ts`: partial sample now expects 3 lines (status, partial marker, warden line). Complete sample expects 2 lines, the second being the warden line.
- `🧪️tests/🪜️phases/🟦️.ts`: the grant line moved from index 1 to index 2. Index 1 is the warden line, asserted in English and German (`Speicherwächter: auf diesem Rechner läuft kein Wächter`, from the schema).
- `🧪️tests/🛟️unsatisfiable/🟦️.ts`: critical sample at `starvedMilliseconds` expects `pressure-critical`.

No JSON fixtures were read by these three tests, so none changed. No assertions were weakened.

## Results

- `🪟️partial`: 2 pass, 0 fail
- `🪜️phases`: 6 pass, 0 fail
- `🛟️unsatisfiable`: 13 pass, 0 fail
