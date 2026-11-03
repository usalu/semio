# 🧍 Round 2 static sweep (agent hz): architecture clips

Read-only except this report and the scripts in `🧰️hz/`. No code edited. Paths relative to `A = 🎓️teaching/🏛️architecture/`, `V = A/⚡️energy/🏗️authoring/🎨️visuals/`. Line numbers are as of this sweep; other agents edit concurrently, and the manifest grew from 143 to 144 casts during the sweep.

## Verdict

- Mostly clean. Nearly every H1–H12 item from `audit-final-static-r2.md` is now a cast call. Remaining open items are the meters and thermometers (borderline), the ceiling grille, and one building store in practice (H10).
- Cast ids: no missing ids, no unused ids. All 144 manifest casts are reached, 124 by literal and 20 by dynamic construction.
- Thermal view: four energy figures carry heat meaning and use a dark-line cast. The optimization room lamps are the clearest case.
- Guard `🧪️tests/🐍️staffage_only.py`: 5 tests pass (unittest exit 0). Its name-based check cannot see primitive helpers. Proposals are in section 4.

## 1. Primitive compositions that depict physical objects

### 1a. Still open (energy clips)

| # | Location | Construct | Verdict and route |
|---|---|---|---|
| P1 | `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py:113-122` `_vent_unit`, called at `:236` | Ceiling grille: Rectangle plus slot Lines | Fixture (grille). Not on the contract's glyph list. Route: new `ceiling-grille` cast, or keep as a section detail if the coordinator accepts it. |
| P2 | `🌬️ventilation/…/📦️volume/🐍️.py:312-325` `_clock`, called at `:776`; `🪟️natural/🐍️.py:179-192` `_clock`, called at `:506, :609` | Clock face and hands from `clock_glyph` | Duplicate of V `clock_glyph` (`V/🐍️.py:1187`). Time glyph, low priority. Route: use `clock_glyph` directly. |
| P3 | `📊️demand/🎬️clip/♨️system-losses/🐍️.py:313-327` `_meter`, called at `:482, :513, :1159-1160` | Utility meter dial with needle, plus bolt | Device. The contract lists "meters" as diagram glyphs, so this is borderline. Decide: keep as glyph, or make a `utility-meter` cast. |
| P4 | `📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:232-237` `_meter_glyph`, called at `:439, :644, :829` | Energy meter dial with needle | Same as P3. |
| P5 | `📊️demand/🎬️clip/🧪️practice/🐍️.py:980-990` `_gas_meter`, called at `:1158` | Gas meter box, dial, needle, counter rectangles | Device (box with dial). Same as P3, but it is a physical box. Route: `gas-meter` cast, or keep as glyph. |
| P6 | `📊️demand/🎬️clip/🧪️practice/🐍️.py:1159` `energy_tank(...)` (V `energy_tank`, `V/🐍️.py:1221-1241`) | Building store drawn as a rect with level, beside the boiler and gas meter | **H10 still open.** The coordinator ruled that a building's hot-water or buffer store uses the `hot-water-cylinder` asset. Route: `hot-water-cylinder`. |
| P7 | `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:547-561` `_thermometer`, calls `:1147, :1937`; practice `🧪️practice/🐍️.py:654-667` `_thermo`, calls `:745, :765` | Thermometer tube, bulb, column primitives | Instrument. Both duplicate V `thermometer_glyph` (`V/🐍️.py:1202-1220`). Consolidate onto `thermometer_glyph`. |
| P8 | `🧲️physics/…/🧲️fundamentals/🐍️.py:677, :1085` `_energy_tank` (V `energy_tank`) | 1 kWh and vacuum energy quantity gauge | Abstract gauge, not a store. Keep, per the coordinator's ruling on abstract bars. |
| P9 | `📊️demand/🎬️clip/⚡️demand-power/🐍️.py:146-149` `_wide_tank` | Abstract kWh gauge | Keep, per ruling. |

Thermometer instances (`thermometer_glyph`, V `thermometer_glyph`): about 25 call sites across energy clips, for example `❄️cooling/…/⚖️heating-comparison/🐍️.py:173`, `🌬️…/❓️need/🐍️.py:491-492, :620`, `🌬️…/🌡️heat-flow/🐍️.py:339, :495`, `📊️…/⚡️demand-power/🐍️.py:658-660, :1117`, `📊️…/🧪️practice/🐍️.py:327, :1076`, `🔥️…/🧱️conduction/🐍️.py:368`, `🔥️…/🧮️calculation/🐍️.py:589`, `📊️…/🔁️recap/🐍️.py:359-360`. Borderline: the contract does not list thermometers as glyphs. Decide once for all of them. Thermometer readings carry heat meaning, so the thermal-view rule may apply.

### 1b. Checked and resolved (no action)

- H1 gas boiler, hot-water store, thermostat, radiator: now `heating-calculation-*` casts (`🔥️heating/🧮️calculation/🐍️.py:1260-1271`). The chimney rectangles (`:1273-1276`) are structure.
- H2 `_derrick`: now `fitted_staffage("demand-primary-energy-pumpjack")` (`📊️…/🏭️primary-energy/🐍️.py:191-194`).
- H3 circulation pump: `demand-practice-pump` (`🧪️practice/🐍️.py:760`).
- H4 `_valve`: `demand-practice-valve` (`🧪️practice/🐍️.py:647-651`).
- H5 MVHR housing: `_mvhr` uses `ventilation-mechanical-mvhr` (`🌬️…/⚙️mechanical/🐍️.py:146-148`).
- H6 hob, pot, kettle: `physics-fundamentals-hob|pot|kettle` (`🧲️physics/…/🧲️fundamentals/🐍️.py:744, :2839-2844`).
- H7 `_blind`: `BLIND_CAST` (`🧲️physics/…:426, :430-455`).
- H8 heat-pump exchanger, compressor, expansion valve: `physics-fundamentals-evaporator|condenser|compressor|valve` (`:1814-1817`).
- H9 DEC equipment: `cooling-summer-strategy-wheel|exchanger|humidifier|heater` (`❄️cooling/🌤️summer-strategy/🐍️.py:305-322`).
- H10 in `_plant` (`📊️…/♨️system-losses/🐍️.py:329`) and `🔥️…/🧮️calculation` store: now `_tank` and `heating-calculation-tank`. Only practice `:1159` remains (P6).
- H12 radiator valve circle: the valve is now a `VectorizedPoint` anchor (`🌬️…/🪟️natural/🐍️.py:248`), with a dial mark (`:261-267`).
- Kettle relabel: now a kettle cast (`🧲️physics:744`).
- Physics `_clock` duplicate: now `clock_glyph` (`🧲️physics:532`).
- `_filled_outline` docstring: now "diagram icon outline" (`V/🐍️.py:1147-1148`).
- Dead watt entries bulb, monitor, vacuum: gone from `V/🐍️.py:322-345`.
- Watt laptop: `shared-watt-laptop` is now `view: thermal` in the manifest, so the dark-line finding no longer applies.
- Optimization room radiator, cooler, occupant, and extrema radiator and cooler: thermal in the manifest.

### 1c. Outside energy (checked, no action)

- `🎯️optimization/🎬️clip/🎲️sampling/🐍️.py:304` `_meters`, `🏆️pareto/🐍️.py:911` `_payback_table`, `🧪️experiments/🐍️.py:374` `_table`: charts and tables.
- `🦗️grasshopper/…/🌳️trees/🐍️.py:420` `present_trees`, `💊️capsule/🐍️.py:578` `_table`, `🦴️anatomy/🐍️.py:354, :361`: data diagrams.
- `🧲️physics/…/🧲️fundamentals/🐍️.py:3051` `person_bars`: bar chart.

## 2. Cast ids

- Missing from manifest: none. The literal scan found no cast-like id outside the manifest.
- Unused in manifest: none. The 20 ids not named as a literal are reached dynamically:
  - `cooling-internal-gains-*` (12 ids): `_CAST + name` (`❄️cooling/👥️internal-gains/🐍️.py:93, :115, :159, :167`).
  - `ventilation-volume-crates-{low,half,high}`: f-string (`🌬️…/📦️volume/🐍️.py:424`).
  - `shared-watt-*` (5 ids): `_WATT_CASTS` through `watt_anchor(compare=…)` (`V/🐍️.py:322-345`).
- `V/🧍️staffage/🔣️.json` now has 144 casts (143 during the first scan). The count rose during the sweep. Recheck after the coordinator's merge.

## 3. Heat-carrying figures that are not in a thermal view

The contract (`contract.md:14`) says energy clips use `thermal` wherever a figure carries heat meaning. Dark-line casts in energy clips, with the file:line that uses them:

| Cast (view dark-line) | Use | Heat meaning? | Verdict |
|---|---|---|---|
| `physics-fundamentals-blind` | `🧲️physics/…/🧲️fundamentals/🐍️.py:426` `BLIND_CAST`, `:430-455` | Solar shading of heat gain | **Candidate.** The clip is about radiation heat. Decide: thermal, or keep as furniture-like shading. |
| `heating-solar-gains-blind` | `🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:857, :866, :1020-1028` | Solar gain through the blind | **Candidate**, same as above. |
| `cooling-internal-gains-desk`, `-office-chair` | `❄️cooling/👥️internal-gains/🐍️.py:144-145` `_workstation`, via `_CAST` | Furniture beside thermal laptop and occupant | Keep (no heat). |
| `heating-internal-gains-desk` | `🔥️heating/👥️internal-gains/🐍️.py:255` | Furniture beside thermal occupant | Keep. |
| `heating-calculation-desk` | `🔥️heating/🧮️calculation/🐍️.py:720` | Furniture beside thermal laptop | Keep. |
| `demand-recap-desk` | `📊️…/🔁️recap/🐍️.py:217` | Furniture beside thermal laptop | Keep. |
| `cooling-heating-comparison-desk`, `-table` | `❄️…/⚖️heating-comparison/🐍️.py:137, :143` | Furniture; toaster and laptop are thermal | Keep. |
| `demand-primary-energy-tree` | `📊️…/🏭️primary-energy/🐍️.py:218` | Tree (no heat) | Keep. |
| `ventilation-volume-blackboard`, `-crates-{low,half,high}` | `🌬️…/📦️volume/🐍️.py:424, :435` | Storage and board (no heat) | Keep. |

Outside energy (coordinator rule: thermal comfort follows the thermal rule):

- `optimization-room-lamp-on`, `optimization-room-lamp-off` (`🎯️…/🎨️visuals/🏠️room/🐍️.py:1336-1339`). Dark-line pendant lamp, and a lamp is a heat source. **Open: switch to thermal** (needs `environment`).
- `optimization-index-lamp` (`🎯️…/⚖️index/🐍️.py:134`; `📉️extrema/🐍️.py:439`). Same. **Open.**
- `optimization-experiments-occupant`, `optimization-experiments-window` (`🎯️…/🧪️experiments/🐍️.py:905-906`). Occupant and window in a comfort experiment. **Candidate.**

Cosmetic: `watt_anchor` in `⚖️heating-comparison/🐍️.py:278` now uses the thermal laptop. Resolved.

## 4. Guard `🧪️tests/🐍️staffage_only.py` and `🔣️staffage_only.json`

Current state: 5 tests (scan coverage, forbidden helper names, visuals no longer defines them, no orphan casts, ad hoc paths gone). Passing.

Why the current name list is weak:

- `forbidden_names` is a word-boundary match on the names `person_glyph`, `seated_person_glyph`, `object_silhouette`, `lamp_glyph`, `respiration_parts`. It cannot see a primitive helper with an ordinary name.
- A naive ban on `_derrick`, `_hob`, `_valve`, `_blind`, `_radiator` would produce false positives. These names now exist as legitimate wrappers around staffage: `_derrick` (`📊️…/🏭️primary-energy:191`), `_valve` (`🧪️practice:647`), `_blind` (`🧲️physics:430`), `_radiator` (`🔥️…/🧲️fundamentals:122`), `_person` (`🧲️physics:522`). So do not ban the names. Ban the construction instead.

Proposed additions, in priority order:

1. **Rule A (AST, primitive-built object helper).** For every `FunctionDef` in a `🎬️clip` script, if its name matches an object-noun regex and its body calls a primitive (`Circle`, `Rectangle`, `RoundedRectangle`, `Square`, `Polygon`, `Ellipse`, `Dot`, `Line`, `Arc`, `SurroundingRectangle`, `RegularPolygon`, `Triangle`, `Annulus`) with no staffage call (`staffage`, `fitted_staffage`, `staffage_motion`, `_cast`, `build`, `pendant_lamp`, `watt_anchor`, `_use_cast`, `_figure`), then fail. Put an explicit allowlist with a reason in the JSON for legitimate diagram glyphs (meters, thermometers, clocks, sections). Run `🧰️hz/hz_guard_probe.py` for the current list. Its energy hits at the time of this sweep were: `_vent_unit`, `_clock` ×2, `_valve_dial`, `_meter`, `_meter_glyph`, `_gas_meter`, `_thermo`, `_thermometer`, `_year_clock`, `_plant_panel`, `_lattice`, `_window_wall`. The last three are structure or diagram and need the allowlist. Non-energy hits in grasshopper, excel and optimization are tables and charts.
2. **Rule B (energy thermal view).** In a `⚡️energy` clip, every literal cast id whose manifest `view` is `dark-line` must be in a furniture allowlist (`desk`, `table`, `chair`, `blackboard`, `crates`, `tree`), or fail. Current result: 0 violations, because the blind casts are exempt by the word `blind`. Remove `blind` from the exempt list once the solar blind decision is made, and then the two blind casts fail. This also needs the dynamic `_CAST + name` ids resolved, so add a second pass that reads the cast-name prefix constant.
3. **Rule C (manifest view).** Every `dark-line` manifest cast used in energy clips must be furniture or the rule fails. Currently `optimization-*` lamps and blinds would fail once they are used in energy clips.
4. **`forbidden_casts`.** Add the removed ids `shared-watt-bulb`, `shared-watt-monitor`, `shared-watt-vacuum` to lock the earlier removal. No other ids are safe to add.
5. **`forbidden_paths`.** No change needed.

## 5. Files

- Report: `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS\r2-hz-report.md`
- Scripts (kept): `…\HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS\🧰️hz\hz_casts.py`, `hz_bodies.py`, `hz_refs.py`, `hz_guard_probe.py`
- Guard test (unchanged): `A\⚡️energy\🏗️authoring\🎨️visuals\🧪️tests\🐍️staffage_only.py` and `🔣️staffage_only.json`
- Test run: `m9_tests.sh` with `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only`. Exit 0.
