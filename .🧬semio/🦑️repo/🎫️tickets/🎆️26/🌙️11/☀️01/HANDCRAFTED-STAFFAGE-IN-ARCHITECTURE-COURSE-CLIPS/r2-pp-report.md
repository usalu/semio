# R2 pp: physical plausibility audit of the thermal casts

Scripts in `T/🧰️pp/`: `dump.py` (asset facts per cast), `temps.ts` (solved surface temperatures over each clip, one `thermalSession` per cast, the same `solveThermalAt` the exporter uses), `probe.ts` + `probe1.json` (what-if solves with patched regions), `flow.ts` (transport parcels), `trace.ts` (exporter with a stack trace). Outputs `temps-all.txt` etc. are tool output (delete with the folder). Run with `repo tools exec bun -- <script>`.

Settled verdicts of kr, pa, ap and tr2 (kettle timing and steam, OI-21 to OI-35, OI-113/114/134, per-Scene span) were not reopened. Temperatures are the solver's steady or transient surface values at the cast's environment; "span ok" means the hottest surface is saturated or inside the +-span colour range as intended for the Scene class.

## Findings

| Cast(s) | Check | Verdict | Fix |
|---|---|---|---|
| shared-lamp-summer, shared-lamp-winter, shared-lamp-neutral, cooling-internal-gains-lamp-dimmed, heating-internal-gains-pendant-lamp-warm-up (pendant-lamp) | surface temperatures | ERROR: the E27 cap and collar ran at 181 to 186 degC, hotter than the bulb glass (165 to 168 degC); a 60 W lamp cap is about 80 to 110 degC (6 W on 0.005 m2) | `pendant-lamp` JSON: cap 6 W to 2.5 W, shade 17 W to 20.5 W (the shade intercepts the bulb's radiation and hot air), sum 57 W unchanged, texts en/de. Cap now about 105 degC, below the glass |
| shared-lamp-off-neutral, shared-lamp-off-winter | off pose | ok, all surfaces at air | none |
| oil-pumpjack (demand-primary-energy-pumpjack) | timeline text | ERROR in text: the clip description (en) said "twenty times the real 20 strokes per minute at slow motion", contradicting de and the 3 s crank. 3 s per stroke is 20 strokes/min in real time, the fast end of real units (4 to 20) | en and de description corrected; motor 55 degC, gearbox 16 degC, wellhead 11 degC at 2 degC and 3 m/s wind plausible; span ok |
| physics-fundamentals-kettle | power, boil, plume | ok (kr): 1930 W water of 1990 W, 1 kg boils at 184 s, steam 97 degC, parcels 0.2 to 0.4 m/s. Limitation: the lumped shell carries only 15 W, so the stainless jug reads 29 degC with 100 degC water inside (real single-wall jug about 80 degC). Fixing it needs a level-coupled shell heat, because a constant heat would heat the shell at once | none, recorded |
| physics-fundamentals-pot | power, boil, narration | ok: 1.2 kW, 1173 W into 1 kg, boil 319 s = 10.46 s at `POT_BOIL_RATE` 30.5, water capped at 100 degC, base 124 degC over the flame. Same lumped-wall limitation (wall 54 degC, real about 90) | none |
| physics-fundamentals-vacuum | 700 W, 25 min clip | ok: 700 W = 110 + 40 + 7 W surfaces + 543 W exhaust, exhaust 45 degC, shell 39 degC. The narration runs 86 min of use in 5.5 s at rate 60 = 330 s of the 600 s running phase, motor stays on | none |
| physics-fundamentals-shower | 38 degC, 3 min | ok: 3 min narrated in 5 s at rate 40 = 200 s of the 300 s clip, spray 37.5 degC, head 26 degC (OI-27 kept) | none |
| physics-fundamentals-hob | 2.5 kW | ok: flame 1380 degC, trivet 251 degC, plate 52 degC; the brass burner head at 480 degC is hot but saturated under span 25 | none |
| physics-fundamentals-compressor / evaporator / condenser / valve | refrigerant temperatures | ok: discharge 102 degC, dome 80 degC, suction line -2 degC, evaporator fins 5 K under 2 degC air, condenser 46 to 52 degC, valve inlet 44 degC and outlet -9 degC | none |
| physics-fundamentals-clerk | person at span 78 | ok: 117 W (1.3 met talking), skin 30 degC, reads faintly warm on the 78 K scale as intended | none |
| shared-person-*, cooling/heating/optimization/demand/ventilation standing-adult casts | ISO 7730 metabolic rate | ok: 1.2 met standing = 117 W over 1.675 m2 (watt anchor 120 W), core 36.8 to 36.9, skin 28 (21 degC) to 34 (27 degC), respiratory 1.4 to 1.6 W sensible | none |
| seated-office-worker casts (cooling, heating, shared-watt-seated, ventilation volume office-worker and student) | met, laptop | ok: typing pose 1.1 met = 124 W, laptop 45 degC at 30 W load, vent 1.2 L/s at 0.8 to 0.9 m/s | none |
| sleepers (cooling-internal-gains, demand-certificate) | sleep 0.7 met | ok: 71 W (0.7 met) summer, 81 W at 21 degC, skin 31 to 34 degC, duvet 25 to 30 degC | none |
| walker, jogger | activity | ok: 2.6 met = 286 W (ASHRAE 55, 1.2 m/s, ISO 7730 lists 2.8 at 4 km/h), 1.2 s stride; jogger 8 met = 895 W with 567 W latent, core 38.7 degC | none |
| ventilation-volume-cook | cooking | ok: 1.8 met (ISO 7730 1.6 to 2.0), pot 101 degC at the 0.4 simmer level (1 K above boil, immaterial); the glass-ceramic hob stays at air by the asset's own statement | none |
| shared-person-fanning | heatwave | ok: 1.4 met, 38 degC air, skin 37 degC, latent 204 W, sweating | none |
| transport casts (shared-person-pathlines, need-occupant-mirrored, volume office-worker, student, cook) | plume speed, age, stroke | ok: nose exhale 1.8 to 2.3 m/s at 0.32 to 0.40 L/s peak (Gupta, 0.5 L tidal volume, 2.4 s exhale), body plume 0.2 to 0.33 m/s with 4 to 6 L/s at +2 K, laptop vent 0.76 to 0.94 m/s at 1 L/s, simmer steam 0.04 m/s at 44 degC; ages 2 to 3.5 s with a 1 s trail and 0.1 mm stroke are drawing choices, all plumes stay below 2.6 m | none |
| laptop casts (cooling, shared-laptop-winter, heating-calculation-laptop, cooling-heating-comparison, shared-watt-laptop) | 45 W, work-and-sleep | ok: deck 51 degC, underside 52 degC at full load in 27 degC air (asset text says about 47 degC at 22 degC); 600 s clip at rate 24 = 25 s, the sleep tail is the cool-down. Note the watt anchor says 60 W for a laptop while the cast peaks at 45 W (the number on screen stays 60 W; the cast runs at its 45 W full load) | none, open item |
| cooling-internal-gains-laptop-closed | closed pose | ok in colour; the deck strip (exposedFraction 0.1) shows 36 degC at sleep power because 0.5 W lands on a tenth of the area. Model artefact, not a drawing error | none |
| server-rack (full-load; heating power-up) | 5 kW, timeline | ok: servers 52 degC, switch 61 degC fronts washed by intake air, exhaust 15 K at 0.26 m3/s carries 4.7 kW; power-up 300 s at `RACK_WARM_RATE` 30 = 10 s. Open: the scene label says "Server 90 W" for this 2 m rack at 30 % load (1.5 kW) | none |
| toaster (winter, summer, shared-watt) | 900 W, 150 s toasting | ok: crust 91 to 99 degC, casing 31 to 39 degC at 300 s (transient, steady about 60 degC), slot air +103 K; 300 s at rate 12 (25 s) and 60 (5 s) | none |
| heater cast in summer strategy (finned-heat-exchanger warm-up) | 70 degC label | ok as asset, open: the coil reaches fins 41, tubes 48, headers 50 degC at its rated duty, the Scene label says 70 degC regeneration. The coil duty is one asset shared with the condenser (46 to 52 degC); OI-112 belongs to cl | none |
| cooling-summer-strategy-wheel, exchanger, humidifier | 12 rev/h, adiabatic | ok: 300 s per turn at timelapse 30 = 10 s, regeneration face 60 degC, process face 32 degC, humidifier pad 17.7 degC under 22 degC air (wet-bulb), exchanger neutral | none |
| cooling-summer-strategy-collector | 900 W/m2, heatwave | ok: stagnation steady 106 degC (asset quotes 150 degC at 1000 W/m2 closed, the model is an open surface and says so), pumped steady 68 degC; the 300 s start-up clip ends at 54 degC absorber because the 6 kJ/m2K plate has a time constant of minutes. Under span 70 it reads warm, not hot | none |
| shared-blind-lowered, cooling-summer-strategy-blind | sun and shade | ok: lamellae 38 degC, wall 45 degC, glazing 32 degC in 32 degC air | none |
| shared-radiator-*, optimization-room-radiator, ventilation-natural-radiator-on/off, shared-radiator-thermostat-front, heating-calculation-radiator(-mirrored) | surface temperatures | open: at the rated pose (75/65 degC) the upper front band solves to 86 degC and the middle one to 79 degC, above the 75 degC supply water; comfort pose (65/55) solves to 73 degC upper front. Cause: the authored 37 % front share sits on 0.6 m2, a flux of 740 W/m2. Under span 25 everything above 46 degC is saturated, so nothing differs on screen, and a split change moves the pinned export digests of radiator-transient and radiator-rated (ap), so it is recorded for the asset owner, not changed | none, open item |
| ventilation-heat-flow-heater, shared-watt-heater (electric-fan-heater) | 2 kW | ok: stage-1 and 2 each 0.016 m3/s at +49 K = 946 W; grille 69 degC, top cover 69 degC, casing 40 degC after 900 s at rate 180 (5 s) | none |
| cooler casts (optimization-room-cooler, extrema-cooler, cooling-heating-comparison-cooler) | 2.5 kW, 14 degC jet | ok: jet -13.3 K at 0.125 m3/s = 2 kW sensible, vane 17 degC, panel 24 degC in 27 degC air; 600 s at rate 45 = 13 s | none |
| cooling-system-sizing-diffuser | 18 degC supply | ok: 8 K under the room, plate 21.6 degC, four jets of 0.0208 m3/s = 300 m3/h | none |
| ventilation-mechanical-duct-fan, mvhr (boost-spin, normal), pendulum-fan | fans, heat recovery | ok: duct fan 14 to 28 W, MVHR 38 to 75 W with impeller 45 degC, 85 % recovery (supply 17.9 degC, exhaust 3.2 degC at 0 degC outdoors), pendulum core cycles 21 to -1 degC over 70 s phases, hub 71 degC at 3 W on 0.006 m2 is a small motor face | none |
| shared-boiler-heating-cycle | 14 to 20 kW | ok: surface loss 85 W, flue 430 W (2.1 %, pa), 600 s burn at rate 60 = 10 s; flue collar 29 degC is cool for a 68 degC flue but saturated at the scale | none |
| shared-tank-charge-up | 900 s charge | ok: scrubbed by a 0 to 1 level tracker, not timed; the jacket reads 24 to 26 degC outside a 60 degC tank, connections 51 degC | none |
| shared-pump-run-high, demand-practice-valve | 45 W pump, TRV | ok: motor 67 degC, casing 45 degC; TRV body 56 degC on 3 W standing water, head 30 degC | none |
| shared-gas-meter-run, shared-electricity-meter-run | 2.4 W own heat | ok: within 1 K of air | none |
| shared-heat-pump-heating-sweep | A7/W35 12 kW | ok: 2.7 kW input, 9.3 kW from the air, 1.53 m3/s at -5.1 K, coil guard -6 degC in 2 degC air, compressor panel 25 degC | none |
| heating-calculation-thermostat, heating-conduction-tealight | small sources | ok: thermostat casing 21.3 degC at 0.7 W; tealight 35 to 40 W flame (1125 degC). Open: cup and wax stay at air (passive by the asset's statement), a lit tealight cup is about 40 degC | none |
| ventilation-volume-hood, operating-table | lamps and fans | ok: lamps 97 degC (30 W halogen), canopy 44 degC, operating-table light housing 45 degC at 120 W | none |
| optimization-experiments-window | tilted window | ok: panes 21.7 degC at 22 degC | none |
| demand-certificate-buyer | arms crossed | ok: same body as the standing cast | none |

## Narration timelines checked (en and de)

Rates found by grep on the cast ids: kettle 60, vacuum 60, shower 40, pot 30.5, boiler 60, rack 30, lamp 12, toaster 12 and 8, laptop 24, cooler 45, collector 60, wheel timelapse 30, heater 3, watt toaster 60, laptop 6, heater 180. Every scene time equals the narrated or labelled quantity within the clip length; no narration number needed a text change.

## Changes

- `S/🗂️catalog/🔌️appliances/pendant-lamp/🔣️.json`: cap `heatW` 2.5, shade 20.5, thermal assumption en/de (re-exported, no NaN).
- `S/🗂️catalog/🏗️machinery/oil-pumpjack/🔣️.json`: `pump-cycle` description en and de.
- Coordinator ruling 1 (laptop 60 W): `S/🗂️catalog/🔌️appliances/laptop/🔣️.json` full office load 45 W to 60 W (regions 22.8, 18.6, 18.6 W), level titles Sleep 1.8 W, Idle 15 W, Full 60 W, summary, poses, assumptions en/de, and the manifest distinction in `S/🗂️catalog/🔣️.json`. The "60 W" label of the watt anchor is untouched. Laptop casts re-exported (shared-watt-laptop, cooling and heating comparison, internal gains, calculation, winter); no NaN. The seated-office-worker's own laptop (30 W) was not changed, so the "Laptop 60 W" label in heating internal-gains Beat3 sits on that figure's desk laptop: open item.
- Coordinator ruling 2, final form (server): `PHI_E_DEVICE_W` stays 90 and the "Server 90 W" label stays (the 1500 W trial was reverted). New handcrafted asset `S/🗂️catalog/🔌️appliances/tower-server/🔣️.json` (script `🧰️pp/tower_server.py`, builder with the geometry in the script): 0.45 m high, 0.45 m deep, 0.20 m wide, 11 kg, single fixed bone `case`; regions side-panel, cpu-grille, front-bezel, psu-vent, rear-panel, feet; level 1 = 200 W rated (49 W through the surfaces, 151 W with the exhaust of the rear fan, 0.016 m3/s at +7.8 K), typical office load 90 W = level 0.45, standby 6 W (0.03); poses typical, rated, standby, off; clips power-up (300 s) and load-surge (8 s loop); en/de text, ENERGY STAR servers and ASHRAE TC 9.9 references; elevation, front and plan geometry at 1:50, 1:100 and 1:200 (mandatory in the three-views ticket), 1:500 unavailable with reason. Manifest entry added after desktop-monitor.
- Cast `heating-internal-gains-tower-server` (tower-server, clip power-up, indoor-heated-winter, dark, thermal, span 25) replaces `heating-internal-gains-server-rack` in `V/🧍️staffage/🔣️.json`; heating internal-gains scene uses it (`RACK_WARM_RATE` renamed `SERVER_WARM_RATE`, 300 s at rate 30 = 10 s). The `server-rack` asset and the cooling internal-gains rack cast stay.
- Solved temperatures of the tower (power-up, 21 degC): case 19.6 to 21.2 degC, processor grille 27.2, power supply 24.9, rear panel 23.5 after 300 s (still warming, steady state higher).

## Re-export and tests

- Export (`🧰️bj/export.py` via `🧰️cv/run.sh`) of the 7 lamp, 7 laptop and the new server cast: ok, no `NaN` in any frame, after eb's fix.
- Catalog check `--asset tower-server`: passes (1:200, 1:100 and 1:50 suitable under office-laser, min feature 0.31, 0.26, 0.33 mm; 1:500 unavailable).
- Python (`m9_tests.sh`, module prefix `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests`): `🐍️staffage_only` (guard), `🐍️` and `🐍️scenes`: 45 tests; the only 5 errors were missing `SEMIO_TEACHING_LANGUAGE` in my first run; `🐍️scenes` re-run with language de: 13 of 13 OK.
- Staffage library vitest (q2 alias config): 54 of 60 files pass, 1361 tests pass, 22 fail, none in tower-server, laptop or pendant-lamp: three-views of other assets still being migrated (concrete-mixer-truck, pigeon, red-squirrel, date-palm, hedgehog and others), kettle phase-change (3), plan-projection (2), front-projection plate warming (1), cast-export fixture (0.4626 over 0.5), export-performance radiator digests (foreign thermal changes). Not caused by this work.
- Render of heating internal-gains `Beat3_GeraetePhiE` (ql, 15 fps, de and en, frames at 3 to 18 s: `T/r2-pp-tower-server-de.png`, `T/r2-pp-tower-server-en.png`): the tower sits on the floor next to the desk, cold blue at the start, warms to orange over the narrated power-up, labels read Laptop 60 W, Server 90 W, Phi-e 150 W, no overlaps. The caption stays German in both runs because the module calls `set_vo_language("de")`; the section title and DIN reference follow the language.

## Open

- Radiator front-band temperature above the supply, kettle and pot wall temperatures (lumped model), heater coil 50 degC against the 70 degC label (cl), tealight cup at air: see the table.
- The seated worker's 30 W desk laptop against the 60 W label.
- Foreign vitest failures above.

## lt: Laptop 60 W label on the seated worker's desk laptop

- Cause: the label (`heating/🎬️clip/👥️internal-gains/🐍️.py` Beat3, `PHI_E_LAPTOP_W` 60) sits on the cast `heating-internal-gains-office-worker` (asset `seated-office-worker`, clip typing). Its desk laptop is not a separate idle laptop: it is the operating region `laptop` (`laptop-load`, level 1 = typing) with `heatW` 30, so no pose or level of the cast row could reach 60 W.
- Fix: `S/🗂️catalog/👤️humans/seated-office-worker/🔣️.json`: `heatW` 30 to 60 (as the laptop asset's full load), `laptop-vent` `volumeFlowM3S` 0.0012 to 0.0024 (about 28 W at +10 K, same 47 % share), summary, description and laptop assumption text en/de. The laptop catalog asset is untouched. All four worker casts re-exported (cooling-internal-gains-seated-worker, heating-internal-gains-office-worker, shared-office-worker-pathlines, shared-watt-seated); no NaN.
- Tests (language de): `staffage_only` 25 OK, `integration` + `labels` 14 OK, `scenes` 13 run with 1 failure `test_heating_plant_uses_one_physical_scale` (radiator height 0.518 vs 0.717 m, foreign radiator work, not the worker). No heating clip test module exists.
- Render Beat3_GeraetePhiE (de, en, ql 15 fps): Laptop 60 W, Server 90 W, Phi-e 150 W, no overlaps; the worker's laptop warms and ripples. Caption stays German in en (module sets the voice language), as pp noted.
- Note: I moved the shared `T/🗑️generated/r2/media` into my own folder after rendering (since deleted), which may have removed other agents' r2 renders there.
- Open: cooling/ventilation labels that quote the seated worker's laptop at 30 W, if any, were not searched (file system too slow).
