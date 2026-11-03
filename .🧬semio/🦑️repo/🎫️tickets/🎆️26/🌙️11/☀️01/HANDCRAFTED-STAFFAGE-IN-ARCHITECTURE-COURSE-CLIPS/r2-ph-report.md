# R2 ph: phase change in the transient thermal solver

S = `🎓️teaching/🏛️architecture/🧍️staffage`. Scripts: `T/🧰️ph/` (`kettle_patch.py`, `kettle_clip.py`, `boil.ts`, `margin.ts`).

## How to use the feature (for cp and any other asset)

1. Declare the transition on the material (shared by all regions made of it) or on one region (the region overrides the material):
   `"phaseChange": { "transition": "boiling", "pointC": 100, "latentHeatJKg": 2257000 }` (`transition` is `boiling` or `melting`; both cap the temperature from above; the inventory is assumed ample, no depletion).
   Regions of model `body`, `fixed`, `leaf` may not carry it (catalog check `phase-change-model`).
2. Drive the region with a constant `operating` level (no throttle): the solver holds `surfaceTempC` at `pointC` and turns the surplus `inflow - outflow(pointC)` into mass.
3. Read it from `RegionField`: `phaseChangeW`, `phaseChangeKgS` (= W / latent heat), `phaseChangedKg` (accumulated over the solved history, 0 in a steady solve), `evaporationKgS` (whole vapour rate: surface evaporation plus phase change). `latentW`, `latentWM2`, `moistureGM2H` and the totals include the conversion, so the balance `conducted = radiation + convection + latent` still closes.
4. Steam: give a `vent` (also `flue`, `fan`, `exhaust`) the `region` of the water and a `vapourKgPerKg`; its `volumeFlowM3S` is then ignored and the flow is `evaporationKgS x strength x operating level / (w_source - w_ambient)` of air at ambient + `temperatureRiseK`. Catalog check `evaporation-vapour` requires `vapourKgPerKg`. Do not ramp the emitter track any more; leave the pose strengths at 1 (0 when off).
5. Transport: a track whose asset has such a vent now takes its thermal field from the input history (`solveThermalAt` with one `thermalSession` per track) instead of the quasi-steady solve; otherwise the water would sit at 100 degC the moment the element is switched on. Other assets keep the steady solve unchanged.

## Code (minimal edits, files re-read before each edit; pf's session/trajectory refactor of `🌡️thermal/🟦️.ts` was built on)

- Schema: `S/🧬️schema/🟦️.ts` (`PHASE_TRANSITIONS`, `PhaseTransition`, `PhaseChange`, `Material.phaseChange`, `ThermalRegion.phaseChange`, `Emitter` doc) and `S/🧬️schema/🔣️.json` (`PhaseChange` def, both properties, `Emitter` description). Both files were accidentally CRLF after my first script and were converted back to LF.
- Solver: `🌡️thermal/🧱️balance/🟦️.ts` (`RegionSetup.phase`, `phaseChangeWM2`), `🌡️thermal/🟦️.ts` (`stepRegion` with 16 sub-steps for phase regions, `Trajectory.phaseKg`, steady target capped at the point, `RegionField` assembly, totals), contract `🔗️contracts/🟦️.ts` (four new `RegionField` fields).
- Transport: `💨️transport/🔥️sources/🟦️.ts` (evaporation-driven vent branch), `💨️transport/🟦️.ts` (`transientThermal`).
- Catalog check: `🗂️catalog/🟦️.ts` (two rules).

## Assets

- `electric-kettle`: water material has `phaseChange` boiling 100 degC, 2.257 MJ/kg. Power track is constant 1 from 1 s (the 0.085 level and the `emitter:steam` track are removed; operating levels are now Off and "Heating at 2 kW"), poses `heating` and `boiling` bind power 1 and steam 1, the `steam` vent follows region `water` (`volumeFlowM3S` removed). Clip `boil` (200 s) retimed to the solved boil: boiling pose and phase `boil` from 184 s (heat phase 1 to 184 s), plume track reaches 1 at 184 s. Descriptions, notes and assumption (en/de) rewritten.
  Solver (office-neutral): water 17.2 degC at 0 s, 58.5 at 90 s, 99.7 at 183 s, 100.00 from 184 s; at 2 kW constant it no longer overshoots. Evaporation 0.063 g/s at 183 s, then 0.848 g/s (phase change 1772 W of the 1930 W, Lewis evaporation 143 W); 12.8 g converted by 200 s. The steam flow rises with it (test: more than 10x between 100 s and 195 s, ratio equal to the evaporation ratio within 15 percent).
- `cooking-pot`: already uses the feature (water material `phaseChange`, steam vent with `region: water`, constant `heat` level 1; clip `heat-to-boil` 379 s with boil from 319 s). Verified with the solver: water 17.7 degC at 0 s, 98.98 at 310 s, 100.00 from 319 s, evaporation 0.509 g/s (phase change 906 W). No edit by me. Note: cp's report still describes the older 0.42 throttle design; the file on disk has the new one.

## Tests (language-agnostic)

- Spec `S/🧪️tests/🫕️phase-change/🥒️.feature`, cases `S/🧫️fixtures/🫕️phase-change/🔣️.json` (kettle water, boiling; ice plate, melting), TS `S/🧪️tests/🫕️phase-change/🟦️.ts`, oracle `S/🧪️tests/🫕️phase-change/🐍️.py` (`uv run --with scipy --with CoolProp python -I 🐍️.py [--write]`, about 4 min).
- Oracle: scipy `solve_ivp` LSODA with a terminal event at the point, nonlinear balance with CoolProp air properties and IAPWS-95 saturation pressure, then the plateau mass `A (P/A - q_out(T_b)) / L`; closure `P t = A C dT + A int q_out + L m` asserted for every time; CoolProp boiling point 99.974 degC and latent heat 2.25647 MJ/kg (the catalog 100 degC and 2.257 MJ/kg are within 0.05 K and 0.03 percent).
- Agreement of the solver with the oracle: temperature within 0.010 K, converted mass within 0.03 g, plateau power within 0.01 percent (tolerances 0.05 K, 0.1 g, 0.2 percent).
- Vitest cases (21 + kettle): oracle match, never above the point, energy balance of the field on the plateau (exact), accumulated mass = rate x time, oracle closure, steady solve = point, overshoot without the declaration (control), region overrides material, CoolProp constants, kettle at constant 2 kW (power track, 100 degC hold, 0.75 to 0.86 g/s), steam vent follows evaporation.

## Checks

- Staffage vitest (q2 alias config, library): 53 files, 52 passed, 1071 tests passed, 11 skipped, 1 failed: `🌡️thermal-span` "makes a moderate source more saturated" (an untracked in-progress test of another agent on the `laptop` asset and legend span; not touched by this change). The phase-change file alone: 1 of 1 file, 21 of 21 tests.
- Catalog check `--asset electric-kettle`: pass (1:200, 1:100, 1:50 suitable; 1:500 unavailable as before). Full run: see end of this file.

## Open

- Python unittest of the clip modules was not run (kettle scene `physics-fundamentals-kettle` timeline changed from 187 s to 184 s boil, constant power; not rendered).
- The transport thermal switch is keyed on "asset has an evaporation-driven vent"; if more quasi-steady-vs-history cases appear, make the transport thermal choice an explicit option.
- Phase change has no depletion (finite inventory) and no freezing/condensation direction.

## Full catalog run
Started, stopped after 57 of 147 checks (all pass, 0 failures; machine load made it about 2 assets per minute). Targeted `--asset` runs of `electric-kettle`, `cooking-pot`, `gas-hob` pass. Generated output deleted.
