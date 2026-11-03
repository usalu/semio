# Round 2, agent g4: devices from hz items P1 to P7

S = `🎓️teaching/🏛️architecture/🧍️staffage`, V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, E = `🎓️teaching/🏛️architecture/⚡️energy`. Generators: `T/🧰️g4/` (`gas.mjs`, `electricity.mjs`, `diffuser.mjs`, `common.mjs`, `build.mjs`, `cast.sh`, `render.sh`, `chain.sh`, `sheet.py`); they reuse `T/🧰️a10/lib.mjs` and `space.mjs`.

## New handcrafted assets (S/🗂️catalog/🔌️appliances, manifest rows inserted before `storage-crate-stack`)

| Asset | Content | Check |
|---|---|---|
| `gas-meter` | G4 bellows meter, front view, 0.24 x 0.27 m: two shells with seam, two union-nut stubs, eight-digit counter, test dial with pointer bone. Poses `off`, `low-flow`, `rated-flow`; clips `run` (pointer turns 2 turns in 4 s, usable as a seek target) and `day-profile`. en/de. | office-laser suitable at 1:50, 1:100, 1:200; 1:500 unavailable |
| `electricity-meter` | Ferraris meter, 0.17 x 0.285 m: six-digit counter, window with rotor disc and mark (rotor bone), rating plate (heat from the measuring system), terminal cover. Poses `idle`, `half-load`, `full-load`; clips `run`, `daily-load`. | same |
| `ceiling-diffuser` | Four-way square diffuser, 0.595 m, side view: flange flush at the ceiling plane, three stepped cones, round collar; thermal with two `vent` emitters (jets left and right along the ceiling, 300 m3/h of 18 degC air in total, -8 K against 26 degC), transport analytics. Poses `off`, `half`, `design`; clips `switch-on`, `modulate`. Support anchors pin plus vertical roller (a horizontal roller left a force residual). | same |

Catalog check: all three print a check mark; the full catalog run is in `🗑️generated/g4/catalog.log` (see result below).

## Call sites

- P1 `E/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py`: `_vent_unit` removed, new `_diffuser(pos, width)` calls `staffage("cooling-system-sizing-diffuser", ...)` with the flange on the ceiling soffit. Looked at: plate cools to blue, collar orange inside the slab. The cast carries no `transport` (the scene already draws the supply paths, and transport plumes inflate the bounds, see `polish-f2-report.md`).
- P3 `E/📊️demand/🎬️clip/♨️system-losses/🐍️.py`: `_meter` removed. Pipeline meters (gas on the gas line, electricity on the power line) and the two comparison meters in Beat4 are `fitted_staffage("shared-gas-meter-run" | "shared-electricity-meter-run", ..., rate=1.0, loop=True)`. Decision: physical devices on a pipe or wire. Looked at: Beat3, Beat4.
- P4 `E/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py`: `_meter_glyph` removed. The house supply meter (Beat1) and the heat-pump or boiler meter (Beat3) are device casts (gas or electricity by `kind`). Beat2 carrier rows: `METER_CASTS` maps `strom` and `gas` to a cast; the `holz` and `umwelt` rows no longer get a meter because wood and ambient heat are not metered at the building boundary (the rail ends at the counting point as before). Looked at: Beat1, Beat2, Beat3.
- P5 `E/📊️demand/🎬️clip/🧪️practice/🐍️.py`: `_gas_meter` stays as a function name but is now only a cast wrapper (no primitives): `fitted_staffage` of the gas meter, with a seek driven by the `rate` tracker (`GAS_METER_TURN_S`), like `_valve`. Looked at: Beat3 (meter on the line).
- P6 practice `energy_tank` at `TANK_X`: **kept, deliberately not replaced by `hot-water-cylinder`.** The object is the "Quelle" (source) gauge of the balance cut `Raum | Zähler | Quelle`: its level drains from 1.0 to 0.8 as the energy source supplies the gas, and the narration says "weiter hinaus bis zur Quelle". It is not a heat store in the building, so a hot-water cylinder would claim a store in the gas line. It is the abstract quantity gauge covered by the coordinator's P8/P9 ruling. Coordinator may overrule; the replacement is one line (`fitted_staffage("demand-system-losses-tank"...)` in place of `energy_tank`).
- P7 `thermometer_glyph` (V) gained `tracker`, `ramp`, `gate`, `tick_color` options (defaults keep the old behaviour; adds the `tick` entry). Physics `_thermometer` is now `_manim_module.thermometer_glyph`; practice `_thermo` is deleted and its two call sites use `thermometer_glyph(..., tracker=, ramp=THERMO_RAMP, gate=, tick_color=)` (`th["frame"]` is now `th["group"]`). Looked at: physics Beat2 (thermometer 38 degC), practice Beat2 (room thermometers with colour ramp and ticks).
- P2 ventilation `📦️volume` and `🪟️natural` `_clock`: already routed to V (`tracked_clock`) by rv; nothing left.

## Helpers gone (for g6, allowlist cleanup)

Deleted: `_meter` (system-losses), `_meter_glyph` (primary-energy), `_thermo` (practice), `_vent_unit` (system-sizing), physics `_thermometer` body (now an alias of V). `_gas_meter` (practice) remains but builds only a staffage cast, so it needs no allowlist entry. Physics `_clock` was already an alias.

## Manifest

Casts added by me were merged by xc into `shared-gas-meter-run` and `shared-electricity-meter-run`; `cooling-system-sizing-diffuser` stays. Call sites were already updated by xc.

## Verification

- Renders at ql, 15 fps, one scene at a time, looked at: system-sizing Beat2; system-losses Beat3, Beat4; primary-energy Beat1, Beat2, Beat3; practice Beat2, Beat3; physics Beat2 and Beat5 (rendered, exit 0).
- Midway the exporter was broken by someone else's edit (`thermalSession is not defined`, `width is not defined` once); retried after it recovered.
- Staffage_only guard before the final merge: 17 of 18 pass, one failure on a stale allowlist entry `primary-energy::_plant_panel` that is not mine. Final test and catalog results are appended below.

## Final results

- Full catalog check: 146 of 147 assets pass, including `gas-meter`, `electricity-meter`, `ceiling-diffuser`. Foreign failures: `oil-pumpjack` and three manifest mismatches (`mvhr-unit` clips, `oil-pumpjack` and `hot-water-cylinder` heightM), not touched by me.
- Visuals tests (scenes, integration, staffage_only, tests, icons, section_consumers): 64 run, 5 errors, none from my edits: (a) `integration.test_demand_emitters_and_occupants` four subtests: `Unknown cast 'demand-certificate-radiator'` (removed by the xc merge, certificate clip call site or the test still names it); (b) `section_consumers.test_practice_room_row`: `NameError: _Path` at practice `🐍️.py:84` (the `from pathlib import Path as _Path` line is gone from practice, a foreign edit; renders worked earlier). staffage_only passed in this run (the stale `_plant_panel` entry was fixed by someone).
- Generated output deleted; scripts kept in `T/🧰️g4/`.
