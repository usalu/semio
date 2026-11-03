# r2-dc report: round 2 docs

## Changed
- `T/contract.md`: appended "Round 2 additions" (schema and solver, cast exporter CLI, Python API, guard, archive, cast ids, open items). Every item was checked in code, not taken from the reports.
- `S/README.md` (`🎓️teaching/🏛️architecture/🧍️staffage/README.md`): new sections "Thermal options" (`phaseChange` on material and region, `thermalSpanK`) and "Course casts" (manifest, `theme`, exporter command, progress lines, cancellation, `--parent`). Inserted before "Run".
- No code changed. AGENTS.md not touched. No git commands run.

## Verified in code (file:line)
- `phaseChange`: schema `S/🧬️schema/🔣️.json:155` (`PhaseChange`), `:227` and `:2471`; twin `S/🧬️schema/🟦️.ts:87-109` and `:507`; solver `S/🔨️modules/🌡️thermal/🧱️balance/🟦️.ts:34,69`; catalog rules `S/🔨️modules/🗂️catalog/🟦️.ts:302` (`phase-change-model`) and `:329` (`evaporation-vapour`). Assets: `electric-kettle` asset JSON line 95, `cooking-pot` line 82.
- `thermalSpanK`: schema `S/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json:58`, twin `…/🧬️schema/🟦️.ts:34,82-83`, exporter `…/🎭️cast/🟦️.ts:127`. All 85 thermal casts in `V/🧍️staffage/🔣️.json` carry 25. The r2-ts report does not exist, so this is verified from code only.
- Exporter `S/🔨️modules/📤️export/🎭️cast/🟦️.ts`: `--parent` parse at `:241-261`; `processAlive` and `watchParent` at `:264-282`; `main` at `:284-307` (signal set `:286`, `[pid]` line `:287`, watchdog `:291`, `[cast i/n]` and progress `:294`, `[cancelled]` and exit 130 `:297-305`). `render` yields per frame (`:134`). `publish` re-checks the signal before rename (`:198`).
- Python `V/🧍️staffage/🐍️.py`: `_SHAPE` with optional `<title>` at `:42`; `_Tree` `:138-218`; `_run` with a daemon reader and `wait(0.1)` polling `:220-261`; `ensure_casts(ids, progress)` `:263-286` (`--parent os.getpid()` in the exporter call at line 140, `--lang en` on the same line).
- Guard `V/🧪️tests/🐍️staffage_only.py`: Rule A `primitive_objects` and `test_rule_a_*` (`:73-83`, `:155-173`); Rule B `resolved_casts`, `non_thermal_violations`, `test_rule_b_*` (`:118-135`, `:175-191`). JSON `🔣️staffage_only.json`: `rule_a.allowed` 14 entries with reasons; `rule_b.allowed` 8 entries, all present in the manifest as `dark-line`; `forbidden_casts` includes the three `shared-watt-*` ids.
- Moved assets: `S/🗂️catalog/🏗️machinery/` holds `plate-heat-exchanger` and `sorption-wheel` (14 assets in the folder). Both asset JSONs have `"category": "machinery"`. Root `🗂️catalog/🔣️.json` `entries`: both `machinery`; machinery count 14. Neither is in `🔌️appliances`.
- Tests the reports cite exist: `V/🧍️staffage/🧪️tests/🐍️.py:500` (`class Cancellation`) and `:603`; `S/🧪️tests/🎭️cast-export/🟦️.ts:257` (cancellation describe), `:328`, `:335`, `:353`.

## Corrections to the reports
- r2-g6 says Rule B has 12 allowlisted ids; the JSON has 8. r2-g6 also says Rule A has 15 entries; the JSON has 14.
- r2-g5 names cast ids that no longer exist (`optimization-room-lamp-on/-off`, `optimization-index-lamp`, `physics-fundamentals-blind`, `heating-solar-gains-blind`). The current ids are `shared-lamp-*`, `shared-blind-lowered` and `cooling-summer-strategy-blind`. Their content (thermal, environment, `theme: dark`) is in place.
- No r2-ts or r2-pf report exists. `thermalSpanK` is present in code and is documented here from the code.

## Open items (code, not changed by me)
1. `S/🔨️modules/📤️export/🎭️cast/🟦️.ts:130` has a `[DEBUG] pf` hook, `process.env.PF_NO_SESSION`, inside `render`. Remove it or make it a real option. Its owner is not in this round.
2. `V/🧍️staffage/🐍️.py:140` passes `--lang en` for casts without `language`. This conflicts with the no-default-language rule. It is latent because the figure-only casts print no text.
3. `🟦️.ts` `main` calls `stopWatching()` only on the error path (`:298`). The timer is unref'd, so this is harmless.
4. Not run here: the vitest and Python suites were not re-run. Test counts come from the reports and were not re-verified. POSIX cancellation remains unverified on Bun (see r2-px).

## Deleted
- Nothing generated. No temporary files written. The background grep I started over `🎓️teaching` was stopped (task `b40y3lxpb`).
