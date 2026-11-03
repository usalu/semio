# G1 report: plant house, pumpjack, circulation pump, radiator valve, energy_tank (H1, H2, H3, H4, H10, H12)

S = `🎓️teaching/🏛️architecture/🧍️staffage`, V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, E = `🎓️teaching/🏛️architecture/⚡️energy`, T = this ticket folder. Generators and scripts: `T/🧰️g1/`.

## New handcrafted assets (slugs for other agents)

| Slug | Category | What | Scales |
|---|---|---|---|
| **`thermostatic-radiator-valve`** (H4, H12) | 🔌️appliances | Brass angle valve with 60 mm thermostat head seen along its axis; the knob turns from star setting (0 deg) to setting 5 (270 deg). Poses `open`, `part-open`, `closed`; clips `close-down`, `open-up` (flow track drives head and knob warmth). Thermal: body stays at supply temperature (3 W), head and knob warm only with flow (0.8 W, 0.4 W). | 1:50, 1:100 authored; 1:200, 1:500 unavailable (pipe stub 17 mm) |
| **`room-thermostat`** (H1) | 🔌️appliances | 90 mm wall thermostat face-on: display window, heating-call lamp, set-point dial turning 20 to 24 degrees. Poses `idle`, `calling`, `set-high`; clips `turn-up`, `heating-cycle`. Thermal: small electronics load, 0.14 W idle, 0.7 W with the relay closed. | 1:50, 1:100 authored; 1:200, 1:500 unavailable |
| **`oil-pumpjack`** (H2) | 🏗️machinery | Conventional beam pumpjack in side elevation (9.2 m x 5.5 m, 18 t): slab and skid, A-frame post, walking beam with horsehead, pitman, crank with counterweight, gearbox, belt guard, motor, wellhead with polished rod and carrier bar. The four-bar linkage is solved for every crank angle (beam, pitman and rod keys), so the nodding is kinematically closed. Poses `stopped`, `mid-stroke`, `bottom-of-stroke`; clips `pump-cycle` (looping, 3 s per revolution) and `start-up`. Thermal: motor 700 W, gearbox 900 W, wellhead 300 W. | all four authored |

Provenance: EN 215 and DIN 18380 (valve), EN 60730-1 and EN 14597 (thermostat), API 11E and Takacs 2015 (pumpjack), Incropera 2018; masses and powers are authored estimates. The two small devices are drawn face-on (the knob turns about the viewing axis), like the fan of A10. Manifest rows were inserted before `room-thermostat`/`tealight` in `S/🗂️catalog/🔣️.json` (minimal Edit). Generators: `T/🧰️g1/{kit,thermostat,trv,pumpjack,build}.mjs` (`lib.mjs`, `space.mjs` copied from A10).

## Call sites

- **H1** `E/🔥️heating/🎬️clip/🧮️calculation/🐍️.py` `_plant_house`: the gas boiler is `heating-calculation-boiler` (`gas-boiler`, clip `heating-cycle`, `BOILER_RATE` 60), the store is `heating-calculation-tank` (`hot-water-cylinder`, clip `charge-up`, the old `level` tracker seeks the clip at 900 s, so 0.9 to 0.75 shows the cooling), the thermostat is `heating-calculation-thermostat` (`room-thermostat`, `turn-up`, frozen at rate 0, 0.17 units high, `Wiggle` still works). The `energy_tank` import and `P_DEEP_GREY` are gone.
- **H2** `E/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py` `_derrick`: now the pumpjack `demand-primary-energy-pumpjack` (scale 100, thermal, `winter-overcast`), fitted into 1.36 x 0.85 and looping at rate 1; the lattice tower is gone.
- **H3** `E/📊️demand/🎬️clip/🧪️practice/🐍️.py`: the pump is `demand-practice-pump` (`circulation-pump`, `run-high`), `fitted_staffage(..., rate=1.0, loop=True)`; the impeller updater and `PUMP_SPIN` are gone.
- **H4** same file `_valve(center, opening)`: `demand-practice-valve` (`thermostatic-radiator-valve`, `close-down`), the `opening` tracker seeks `1 + (1 - opening) * 6` s (1 = open, 0 = closed). The `gate` argument, the handle and `valve_gate` are removed (the group fades with `FadeIn`/`FadeOut`).
- **H12** `🪟️natural`: not touched (A11 removes the valve circle). A11 can reference `thermostatic-radiator-valve` with a cast such as `{"asset": "thermostatic-radiator-valve", "pose": "open", "view": "thermal", "environment": "indoor-heated-winter", "theme": "dark"}`.
- **H10** `energy_tank` kept where the clip draws an abstract quantity: demand-power `_wide_tank` (accumulated kWh), physics fundamentals (the "1 kWh" tank that becomes the P x t rectangle, vacuum cleaner), practice (the primary-energy "Quelle" gauge). Only the calculation clip draws a physical building store, so only there it became the cylinder. Docstrings of `energy_tank` (`V/🐍️.py`) and `_wide_tank` now say "abstract quantity gauge" (physics code untouched).
- Manifest `V/🧍️staffage/🔣️.json`: `demand-practice-pump`, `demand-practice-valve`, `demand-primary-energy-pumpjack`, `heating-calculation-boiler`, `heating-calculation-tank`, `heating-calculation-thermostat`.

## Looked at

- Pumpjack cast sheet (frames of `pump-cycle`, thermal): beam nods, horsehead tangent rope and carrier bar stay on one line, crank and pitman stay connected; thermal view is one flat colour at `winter-overcast` (motor only slightly warmer).
- Thermostat poses (dial turns clockwise with the set point) and valve poses (pointer turns, closed head is cool) as cast sheets.
- Calculation `AnlagenVerluste` (chain and emission beats, from the partial movies): boiler and cylinder sit in the cellar between the labels "Speicher" and "Wärmeerzeuger", pipes meet them; thermostat next to its label.
- Primary energy Beat 1 (Gewinnung beat): the pumpjack nods at the left, centred under "Gewinnung", clear of the plant.
- Practice: the full Beat 2 render was too slow on the loaded machine, so I rendered a ticket-local mini scene (`T/🧰️g1/scene_practice.py`) with the real `_valve` and pump code at the same positions: four valves at the riser tops close to 0.30, 0.45, 0.80, 1.0, pump spins; the valves read small (0.3 x 0.42 units) but legible.

## Verification

- Catalog check: new assets pass (`--asset`), `appliances` 20 of 20 and `machinery` all pass.
- Python: `V/🧪️tests` `staffage_only` (5), `icons` (8), `transforms` (2), `🐍️` (7) OK. See the end of this file for `integration`, `scenes`, `section_consumers`, `sections` and the vitest suites.

## Notes

- Cast exports for the full manifest re-run whenever another agent changes exporter sources; the machine was heavily loaded (about 40 minutes for one beat), so full-scene renders of practice Beat 2 and the rest of `AnlagenVerluste` were stopped after the changed beats.
- Python writes into the emoji paths fail intermittently with `Errno 22`; `T/🧰️g1/edit_*.py` retry (and must raise when all retries fail).

## Final test results

- Staffage vitest (library, `q2-vitest-alias.config.ts`): 51 of 51 files, 994 tests pass, 10 skipped.
- Visuals Python tests: `section_consumers` (9), `sections` (9), `scenes` (11) OK. `scenes` `test_plant_equipment_and_material_use_room_scale` needed an update: the boiler and cylinder are now fitted casts whose box is met within 25 percent (the thermal frames are larger than the first frame the fit uses) and whose bottom sits within 0.06 units of the cellar floor (`V/🧪️tests/🐍️scenes.py`). `integration` was started but ran for more than 30 minutes on the loaded machine and was stopped, not verified.
- No clip-specific unit tests exist for practice, primary energy and calculation beyond the above.
