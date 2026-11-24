# C2: unit ceiling (policy, pure verdict, runtime alarm, law)

Date 2026-11-24. ADM = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/`, CP = `…/🏗️native-build/🧩️component-pass/`.

## Files changed
- `ADM/🧬️schema/🔣️.json`: `Policy.const.unitCeiling` (4096 MB, host 16384 MB, closed `carving` list: cad 6550 and puzzle 6148, action `thin-lto-closure`, EN description inside the object); new definitions `UnitCeilingVerdictV1` and `Messages` (EN and DE `unitCeiling.exceeded` / `unitCeiling.resolved`).
- `ADM/🟦️.ts`: type `UnitCeilingVerdictV1`; `unitKeyPartsV1(key)`; `unitCeilingVerdictV1(policy, observations)`; `formatUnitCeilingV1(language, verdict)`; `unitCeilingAlarmV1(policy, row)` (one `EN / DE` line, `null` when within the ceiling or listed).
- `CP/🟦️.ts`: import of `unitCeilingAlarmV1`; in `recordCompletedUnit` a per-build `ceilingAlarms` set and one `log("[native-build] <en> / <de>")` per unit key (the driver's own `log`, same bilingual `" / "` pattern as `describeLocalComponentJobsV1`).
- New `ADM/🧪️tests/🧢️unit-ceiling/`: `🔣️.json` (14 cases: measured telemetry min/median/max per final + 13 edge cases), `🧬️schema/🔣️.json` (closed corpus schema), `🐍️.py` (independent oracle, stdlib only, reads policy from the schema file), `🟦️.ts` (6 tests).

## API
- `unitCeilingVerdictV1`: per unit key the largest `peakRssBytes` ceil to MiB; `exceeded` (peak > ceiling, `listed` by (crate, target)), `resolved` (listed entries observed with largest peak <= ceiling), `ok` = no unlisted exceeder and nothing resolved; malformed keys ignored; order peak desc then key.
- Real-ledger law reads `<repo>/.cache/b/r/m/components/build-telemetry/units.jsonl` (the driver's cache root layout); skips when absent (it is absent on main right now: the Rust backing image `fixed.dmg` does not mount).

## Tests
- `repo test-file …/🧢️unit-ceiling/🟦️.ts`: 6 pass, 0 fail (48 expects). Measured case: exceeded cad 6550 + puzzle 6148 (both listed), dag 3291 within, ok true; oracle equality over all 14 cases.
- `repo test-file …/🎟️admission/🧪️tests/🟦️.ts`: 14 pass, 3 fail (17 tests). The failures are the real-Cargo tests that go through `🧪️tests/🦀️cargo/🟦️.ts:75` (`repo cargo`), which die with `diskutil image attach … fixed.dmg exited with status 1 / Failed to mount the volume (reference d87380fb)`: the known unmounted main Rust backing, unrelated to this change. Not worked around.

## Not done
- No Rust wrapper edit, no cargo build, no `🌡️host-capacity/` change; no per-host-class ceiling (single `megabytes`/`hostMegabytes`); no wiring of the verdict into a gate other than the log alarm and the test law; the type-check (tsc) was not run separately, only bun test.
