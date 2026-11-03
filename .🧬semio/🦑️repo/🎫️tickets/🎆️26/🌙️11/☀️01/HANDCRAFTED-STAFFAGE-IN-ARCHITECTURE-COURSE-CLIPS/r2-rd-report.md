# R2 rd: radiator supply cap and level-coupled kettle/pot walls

S = `🎓️teaching/🏛️architecture/🧍️staffage`, T = this ticket. Scripts in `T/🧰️rd/` (probe.ts, edit_*.py, pin.ts, fx.ts, render_av.sh).

## 1. Root cause of radiator bands above the supply (79 to 86 degC against 75)
The plates were `operating` regions: an authored heat (`heatW`, front share 37 percent on 0.6 m2, 740 W/m2) was pushed into the surface, so the solved temperature was set by the flux and not by the water. No supply-temperature boundary existed, not a wrong environment or emitter reference. Fix: a new schema model `hydronic` (circuit water at the region's `flowPosition`, supply falling to return from the table `rows` at the operating level, below the first row linear to the room air, plus a water-side `filmM2KW`), so a surface can never exceed the supply.
- panel-radiator regions panel-rear, 3 front bands (flow positions 1/6, 1/2, 5/6), convector, valve are `hydronic` with rows 55/45, 65/55, 75/65 at levels 0.515, 0.748, 1; plate film 0.003, convector fin film 0.05 (calibrated), valve 0.01 m2K/W.
- Result at the rated pose (21 degC room): upper 71.5, mid 68.3, lower 65.1, rear 68.3, convector 52.5, valve 70.5 degC (max under 75). Total output 1175 W rated (EN 442: 1169 W at dT 49), 903 W comfort (EN 442 about 870 to 900), 646 W night. The EN 442 exponent now emerges from the correlations.

## 2. Level-coupled walls (kettle shell, pot wall)
New model `coupled`: `{region, wetFraction, filmM2KW}`; the wall conducts from the source region's water over the wetted share (kettle 0.6, pot 0.5, film 0.0025) and the heat it takes is removed from the source balance (`RegionFrame.draw(T)`, a monotone function of the source temperature; steady solve nests the walls, transient solve sub-steps to the wall time constant and advances the wall along the ramp of the water). Plateau: kettle shell 97.3 degC (3 K under the water), pot wall 96.3 degC (3.7 K), against 29 and 54 degC before.
- Kettle retune (ruling: the narration number is the lesson, boil at 184 s; physics tests unchanged, clip untouched): the first coupled solve boiled at 198 s. Physically justified levers: the water under the closed lid starts at room temperature (water `exposedFraction` 0.05 in the three poses, no evaporative chilling, 21.9 degC instead of 17.7), shell heat capacity 2765 J/(m2K) (0.7 mm steel as its layer says, was 3500), water heat 1955 W (plate 40 + lid 5 + 1955 = 2 kW; the old 15 W shell share is now drawn from the water). Solved boil at about 183 s, plateau evaporation about 0.8 g/s. `physics-fundamentals` clip tests (boil_start 184, element power within 5 s of 356 kJ / 1955 W) pass.
- Pot unchanged in power (1173 W): boils within the 319 s key; texts en/de updated.

## 3. Code and schema
- `S/🧬️schema/🟦️.ts` (+ `🔣️.json`): `THERMAL_MODELS` hydronic and coupled, `HydronicCircuit`, `CircuitRow`, `WaterCoupling`, `operatingKeyOf`.
- `S/🔨️modules/🌡️thermal/🧱️balance/🟦️.ts`: `circuitWaterC`, `inflowOf` for the two models, `RegionFrame.sourceC/draw`, `NO_DRAW`.
- `S/🔨️modules/🌡️thermal/🟦️.ts`: `withCoupling`, `steadyNodesOf` (steady cap shortcut at a phase point, bit-identical), `stepRamped`, `stepRegions`; header doc.
- `animation/🟦️.ts`, `catalog/🟦️.ts` (operating keys, validation of rows, flow position, coupled source), React detail `🧬️model/🟦️.tsx` plus i18n en/de (`detail.model.hydronic|coupled`, `detail.param.circuit|supply|source`).
- Assets: `panel-radiator`, `electric-kettle`, `cooking-pot` JSON (models, texts en/de).

## 4. Tests
- New language-agnostic case set: `S/🧪️tests/🚰️hydronic-coupling/` (`🥒️.feature`, `🟦️.ts`, scipy/CoolProp oracle `🐍️.py`, fixture `S/🧫️fixtures/🚰️hydronic-coupling/🔣️.json`): 5 circuit plates (transient and steady within 0.05 K, max surface at or below water and supply), coupled wall against a 2-node `solve_ivp` (water 0.6 K, wall 0.6 K, plateau conversion power 3 percent, wall within 6 K of the water and never above), catalog radiator (all poses at or below 75, band order, 1.05 to 1.3 kW), kettle and pot plateau gap.
- Library vitest (q2 alias config) after the change: hydronic-coupling, phase-change, thermal-scenarios, transport-assets, front-projection, equilibrium, thermal-transient, catalog-physics all green. Full run earlier: remaining failures are other tickets (three-views rollout, plan-projection, walker-line digest).
- `front-projection` fixture `minSpreadK` 8 to 5 (upper 71.5 to lower 65.1 is 6.4 K, the 10 K circuit spread x 4/6; the old 12.7 K was the authored flux artefact).
- Python (`m9_tests.sh`, language en): physics fundamentals + `scenes` + `staffage_only`: 51 tests OK; earlier `staffage_only`, `scenes`, `section_consumers` 47 OK (cast re-export of radiator, kettle, pot casts happens inside these runs).

## 5. Pinned digests re-pinned (`S/🧫️fixtures/🚀️export-performance/🔣️.json`)
- `radiator-transient`: all 10 files (0000 to 0008 svg and the index json).
- `radiator-rated`: 1 of 2 (0000.svg; the json index is unchanged).
- Not touched: `walker-line` (red from concurrent drawing work, owner re-pins after rollout), `walker-thermal`. Kettle and pot are not pinned casts.

## 6. Radiator height (coordinator question, scenes test)
- Single truth: `panel-radiator` `dimensions.heightM` = 0.71 m; `staffage_height("shared-radiator-thermostat")` = 0.71. The scene scale in `AnlagenVerluste` is 1.6 / 2.5 = 0.64, so the cast radiator is 0.71 x 0.64 = 0.454 units high.
- I did not change any radiator geometry (my edits are thermal only; the geometry hunks in the radiator JSON are from the front/plan view rollout). The 0.518 measured by `radiator.height` is the cast group bounds (0.81 m equivalent: the figure plus the motion overlay), 14 percent over the figure, inside the test's 15 percent.
- `scenes.json` `radiator_height_m` (0.81, a measured render value) is deleted; `test_heating_plant_uses_one_physical_scale` now derives `radiator_m = scale * staffage_height("shared-radiator-thermostat")` (0.71 m from the asset). Test passes. The overview-emitter test already used the same derivation.
- Render of `AnlagenVerluste` (de, ql, 15 fps): see the end of this file.

## 7. Open
- The shared `cooking-pot` JSON was overwritten by another agent once (stale copy) and my edit re-applied; check `coupled` is still present in that file at merge time.
- Kettle plateau evaporation is 0.8 g/s (was 0.85 for the bare water).

## 8. AnlagenVerluste render outcome
Rendered once (de, ql, 15 fps, one scene, `🧰️rd/render_av.sh`): exit 0, the mp4 was produced. Radiators are built at `0.71 x 0.64 = 0.454` figure height (asset `heightM` 0.71 on the 0.64 scene scale), not the 1.41 times too large size of the former `metre` call. Generated output deleted.
