# Audit guard O2 and P1 (re-run after single 3D placement path)

## Result
PASS. No check, consistency or print-profile problems for the 7 O2 objects and the 11 P1 appliances. Nothing needed a fix or a follow-up.

## How it was run
Each driver was run as one `bun` process, cwd `C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage`.

- O2: `bun T/🧰️O2/all.ts G-O2P1 none <7 ids>`. Covers `checkAssetDetailed` (physics off, 5 samples), `sweep` plus `consistency` for every authored scale. Previews skipped (`none`). Exit 0, about 52 s.
- P1: `bun T/🧰️QA2/all.ts` with `QA2_ASSETS` = the 11 P1 ids, `QA2_RENDER=0`. Covers `checkAssetDetailed` (3 samples), `sweep` plus `consistency`. Exit 0.

Deviations from the brief:
- `T/🧰️P1` has no consistency driver (`run.py` applies specs, `render.ts` renders only). The P1 check and consistency pass therefore used `T/🧰️QA2/all.ts`, the driver that the QA2 report used for the same 11 assets.
- The O2 driver hard-codes its output root to `T/🗑️generated/O2/<round>`. The output was moved into `G-O2P1/O2` afterwards, and the empty parent was removed.
- No renders or pose/clip hashes were produced (`none` and `QA2_RENDER=0`). Visual drift of the front and plan geometry under the new placement path is therefore NOT re-verified here. The three-views vitest was not run.

## O2 (retractable-bollard, shopping-trolley, stepladder, storage-crate-stack, street-lamp, tealight, waste-bin)
- Check: 7 of 7 OK, 0 problems.
- Consistency and sweep: 26 of 26 authored asset/scale combinations `ok`, every pose and clip instant.
- Print profile (office-laser): every authored scale is suitable. Smallest min feature: tealight 1:200 at 0.210 mm (passes); waste-bin 1:500 at 0.220 mm.
- Expected unavailable, not problems: `storage-crate-stack` 1:500 (nesting step 0.16 mm), `tealight` 1:500 (flame 0.02 mm).
- Note: the QA report said 27 combinations. The run found 26 authored and 2 unavailable out of 28 asset/scale pairs, so the count in the QA report is off by one. The counts themselves pass.

## P1 (ceiling-diffuser, circulation-pump, decentralised-hrv-fan, desktop-monitor, duct-fan, electric-fan-heater, electric-kettle, electricity-meter, gas-boiler, gas-hob, gas-meter)
- Check: 11 of 11 `check: ok`, no problems listed.
- Sweep and consistency: no finding lines were printed for any asset or scale, so all pass.

## Output
Driver logs and results were written to `T/🗑️generated/G-O2P1` and deleted after this audit.
