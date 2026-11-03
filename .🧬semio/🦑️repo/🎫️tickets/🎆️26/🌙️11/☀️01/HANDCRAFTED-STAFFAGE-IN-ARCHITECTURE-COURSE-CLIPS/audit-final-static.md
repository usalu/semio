# Final Static Staffage Audit: Architecture Course Clips

## Summary verdict

**Not complete.** The cast migration is mostly done: every cast id that clip code uses resolves to the manifest, and all 114 manifest casts pass the structural checks. Most inventory rows that were CAST-MISSING are now migrated. About 20 hand-built devices and helpers remain, four `watt_anchor` badges still show the wrong icon, four people in energy clips are drawn in a non-thermal view, and several legacy helpers are dead code.

Headline counts:

| Check | Result |
|---|---|
| Manifest casts (`⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json`) | 114 (inventory said 28, stale) |
| Manifest casts failing structural checks (theme, view, pose xor clip, asset, pose/clip slug, environment) | 0 of 114 |
| Literal cast ids used in clip code but missing from manifest | 0 |
| Dynamic or prefix-built cast ids checked | all resolve (see section 3) |
| Manifest orphans (no code path) | 4: `standing`, `walking`, `radiator`, `seated-reader` (plus `shared-radiator`, only reachable through dead `radiator()`) |
| Inventory rows still ad hoc (devices, helpers) | about 20 (section 2, list A) |
| Wrong watt badge icon (person or irradiance drawn as a bulb or vacuum) | 4 (list B) |
| Energy people or heat-emitting badges not thermal | 4 people, 6 badge uses (list C) |
| Removed legacy items confirmed gone | `person_glyph`, `seated_person_glyph`, `object_silhouette`, `_SILHOUETTES`, `🫟️silhouettes/`, `🏃️activity/` |

Path conventions: `A` = `C:\git\semio\🎓️teaching\🏛️architecture\` (all relative paths below start from it). `V` = `⚡️energy/🏗️authoring/🎨️visuals/`.

---

## Finding list (actionable)

### List A: still hand-built (ad hoc) in energy clips

| # | File:line | Construct | Used at | Asset in catalog? | Action |
|---|---|---|---|---|---|
| A1 | `⚡️energy/📊️demand/🎬️clip/♨️system-losses/🐍️.py:306` `_boiler` | RoundedRectangle casing, plate, flame | 513, 807 | No (`appliances/gas-boiler` missing) | Add asset, route through cast. |
| A2 | `…/♨️system-losses/🐍️.py:314` `_fan` | Circle ring plus blades | 328 (inside `_heat_pump`) | No | Add fan asset or keep as explicit airflow symbol and document it. |
| A3 | `…/♨️system-losses/🐍️.py:323` `_heat_pump` | Box, fan, compressor ring, fins | 545, 808 | **Yes**: `machinery/heat-pump-outdoor-unit` | Route to `staffage("…", …)` with heating or cooling pose. |
| A4 | `…/♨️system-losses/🐍️.py:352` `_tank` | RoundedRectangle tank, cap, level fill | 512 | No | Add `objects/storage-tank` or keep as a diagram of stored heat and document it. |
| A5 | `⚡️energy/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:207` `_pump` | Ring with three blades | 242, 459 (also inside `_heat_pump`) | No (`appliances/circulation-pump`) | Add asset. |
| A6 | `…/🏭️primary-energy/🐍️.py:229` `_boiler` | Rectangle casing with flame | 455, 843 | No | Add asset (same as A1). |
| A7 | `…/🏭️primary-energy/🐍️.py:237` `_heat_pump` | Rectangle casing, louvres, `_pump` fan | 845 | **Yes**: `heat-pump-outdoor-unit` | Route to the cast (same as A3). |
| A8 | `⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py:998` `_boiler` | Rectangle casing, two Polygon flames | 1157 | No | Add asset, route through cast. |
| A9 | `⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:136` `_fan` | Circle ring plus blades | 282, 296, 310, 311, 469, 470, 784, 804 | No | Decide: fan asset (`appliances/mvhr-unit`) or documented airflow symbol. |
| A10 | `⚡️energy/🌬️ventilation/🎬️clip/❓️need/🐍️.py:142` `_fan` | Circle ring plus blades | 638 | No | As A9. |
| A11 | `⚡️energy/🌬️ventilation/🎬️clip/📦️volume/🐍️.py:111` `_fan` | Circle ring plus blades | 436 | No | As A9. |
| A12 | `⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py:259` `_tealight`, and `:268` `_devices` (wrapper) | Hand-built cup, wick, flame | 274 (inside `_devices`), 850, 954 | No (`objects/tealight` missing) | Add asset, route through cast. |
| A13 | `⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:190-206` | Hand-built split unit: RoundedRectangle, fan ring, blades, coil VMobject | 198, 229 | **Yes**: `appliances/split-ac-indoor-unit` (already used by `optimization-room-cooler`) | Route to the cast. |
| A14 | `⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:570` `_shower_head` | Lines, Circle rose, Dots | 1173 | No (`appliances/shower-head`) | Add asset. |

Inventory gap list (section 5) that is still open: `gas-boiler`, `circulation-pump`, `ventilation fan / mvhr-unit`, `desk-fan`, `tealight`, `shower-head`, `power-plant-stack`, `storage-tank`. Closed since the inventory: `laptop`, `desktop-monitor`, `pendant-lamp`, `vacuum-cleaner`, `toaster`, `sleeping-person`, `split-ac-indoor-unit`, `heat-pump-outdoor-unit`.

### List B: wrong watt badge icon (inventory "mismatch" rows, still present)

| # | File:line | Current code | Problem | Fix |
|---|---|---|---|---|
| B1 | `⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py:220` | `watt_anchor(800, compare="vacuum", title="Bestrahlungsstärke je m²")` | Irradiance badge shows a vacuum cleaner. | Use the sun symbol (`sun_glyph`) or an environment preset. Not a vacuum. |
| B2 | `⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:284` | `watt_anchor(100, compare="bulb", title="Person")` | Person shown as a bulb. | Build the badge from `cooling-heating-comparison-occupant` or `standing-adult` (thermal), not `_WATT_CASTS["bulb"]`. |
| B3 | `⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:328` | `watt_anchor(100, compare="bulb", title="Körperwärme")` | Body heat shown as a bulb. | As B2, with `cooling-internal-gains-standing`. |
| B4 | `⚡️energy/🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:390` | `watt_anchor(PHI_P_W, compare="bulb", title="eine sitzende Person")` | Seated person shown as a bulb. | As B2, with `heating-internal-gains-office-worker`. |

Root cause: `V/🐍️.py:364-378` (`_WATT_DEVICES`, `_WATT_CASTS`) has no person entry, so person badges fall back to the bulb. Add a `person` or `standing` mapping and a seated variant, then change the four call sites.

### List C: non-thermal views where a thermal view is expected (energy clips)

Contract: people and heat-emitting devices in `⚡️energy/**` use `thermal`.

| # | File:line | Cast (manifest) | View now | Expected |
|---|---|---|---|---|
| C1 | `⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1065` | `physics-fundamentals-clerk` (standing-adult, pointing) | dark-line | thermal (person) |
| C2 | `⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py:148` | `demand-practice-talker` (standing-adult, relaxed) | dark-line | thermal (person) |
| C3 | `⚡️energy/📊️demand/🎬️clip/🏷️certificate/🐍️.py:846-847` | `demand-certificate-buyer` (standing-adult, arms-crossed) | dark-line | thermal (person) |
| C4 | `⚡️energy/📊️demand/🎬️clip/🏷️certificate/🐍️.py:848` | `demand-certificate-tenant` (standing-adult, relaxed) | dark-line | thermal (person) |
| C5 | `…/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:530` | `shared-watt-toaster` via `watt_anchor(5000, compare="toaster")` | dark-line | thermal (heat-emitting device) |
| C6 | `…/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1706` | `shared-watt-heater` via `watt_anchor(2000, compare="heater")` | dark-line | thermal (heat-emitting device) |

Also in this class but judgment-only: `shared-watt-laptop` (`heating-comparison:283`), `shared-watt-bulb` (B2 to B4). These are badges, so a thermal icon is acceptable only if the badge itself is meant to show heat. Decide per badge.

Non-thermal furniture in energy clips (`desk`, `office-swivel-chair`, `dining-table`, `blackboard`, `crates`, `blind`, `english-oak`) is not heat-emitting and can stay dark-line. Not listed.

### List D: dead or orphan code and casts

| # | Item | Evidence | Action |
|---|---|---|---|
| D1 | `V/🐍️.py:1160` `radiator()` | No caller in clips. Only `V/🧪️tests/🐍️icons.py:66` calls it. | Delete, or move the test to the cast path. This also makes `shared-radiator` (`V/🧍️staffage/🔣️.json`) orphaned. |
| D2 | `V/🐍️.py:322` `respiration_parts()` | No caller anywhere. | Delete. Breath plumes already go through transport (`ventilation-*`, `transport`). |
| D3 | Manifest `"standing"` (asset `walking-commuter`, pose `standing`) | No code path. Clip code uses `cooling-internal-gains-standing`, which is a different key. | Delete. |
| D4 | Manifest `"walking"` (`walking-commuter`, `walk-cycle`) | No code path. | Delete. |
| D5 | Manifest `"radiator"` (`panel-radiator`, `rated`, dark-solid) | No code path. | Delete. |
| D6 | Manifest `"seated-reader"` (`seated-reader`, `seated-upright`) | No code path (no quoted id in any clip file). | Delete, or use it for the seated-person badge in B4. |
| D7 | `V/🐍️.py:1190` `_filled_outline` | Used only by `icon_glyph` for sun, moon, flame, microbe, factory. Not a figure. | Keep. Confirm no person outline remains in `🔣️icons/🔣️.json` (checked: only icons). |
| D8 | Contract drift: `pendant_lamp` | Contract says `🧍️staffage/🐍️.py`; it is at `V/🐍️.py:1286`. Callers import it from `V`. | Update the contract or move the function. Note: `fitted_staffage` is also in `V/🐍️.py:382`. |

### List E: contract and cleanup checks passed

- `person_glyph`, `seated_person_glyph`, `object_silhouette`, `_SILHOUETTES`, `V/🫟️silhouettes/` and `⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🏃️activity/` are gone. `🧪️tests/🔣️staffage_only.json:7` now forbids `🫟️silhouettes` and `🏃️activity`, and the cooling test `…/👥️internal-gains/🧪️tests/🐍️.py:28` asserts `🏃️activity` is absent.
- `icon_glyph` (`V/🐍️.py:1206`) draws only sun, moon, flame, microbe, factory, which is the contract's icon list (factory is the exception, see F1).
- No stick figure remains in clip code (no limb or head Line/Circle set). The only head-like geometry is cast-based heat spots (`cooling-internal-gains/🐍️.py:100`).
- No SVGMobject of figures in clips. The only SVGMobject use is the presentation renderer.

### List F: judgment calls

- F1: `icon_glyph("factory")` for the power-plant and carbon criterion (`⚡️energy/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:202`, `🎯️optimization/🎬️clip/⚖️index/🐍️.py:127-136`). The contract's glyph list does not include factory. Either accept it as a symbol (the carbon criterion is a pictogram, not staffage) or add a `machinery/power-plant-stack` asset.
- F2: `_fan` (A2, A9, A10, A11) is a symbol in the inventory ("airflow marker") but the contract says devices go to staffage. Decide which rule applies before adding assets.

---

## 1. Inventory re-check (every row of `audit-adhoc-inventory.md`)

Status key: **MIGRATED** = routed through the cast API, file:line of the new call. **AD HOC** = still hand-built. **WRONG ICON** = routed but with a mismatched icon. **SYMBOL** = icon kept, correct per contract. **FP** = false positive. **DEAD** = no caller.

### 2.1 physical_fundamentals

| Inventory row | Status now | Where |
|---|---|---|
| person_glyph (`_person`), l.1049/1655/3095 | MIGRATED | `🧲️fundamentals/🐍️.py:501` `_person` to `staffage_motion`; calls at 1065, 1671, 3110 |
| `_vacuum` (object_silhouette) | MIGRATED | `:564` `_vacuum` to `physics-fundamentals-vacuum`; calls 1125, 2574 |
| `_shower_head` | AD HOC | `:570` (list A14) |
| `_lamp` (1459, 1560) | MIGRATED | `:607` `_lamp` to `physics-fundamentals-lamp`; calls 1475, 1576 |
| `_radiator` (visuals radiator) | MIGRATED | `:484` `_radiator` to cast; calls 1889, 2048, 2346 |
| `_flame` | SYMBOL | `:511`, `:536` |

### 2.2 ventilation

| Inventory row | Status now | Where |
|---|---|---|
| need `_fan` (l.142) | AD HOC | `❓️need/🐍️.py:142` (A10) |
| need `_stand` and `_room_occupants` | MIGRATED | `:135` to `ventilation-need-occupant`, `-mirrored` at `:165` |
| volume `_fan` | AD HOC | `📦️volume/🐍️.py:111` (A11) |
| volume `_stand` | MIGRATED | `:291` to cast; uses `ventilation-volume-occupant` at 1083 (and 436) |
| volume `_mini_room`, `_use_glyph` | MIGRATED (kinds) / architecture box kept | `:395`, `:418`; kinds use `_use_cast` with casts at 431-451 (crates, blackboard, office-worker, student, cook, hood, operating-table) |
| mechanical `_fan` (8 sites) | AD HOC | `⚙️mechanical/🐍️.py:136` (A9) |
| mechanical `_stand` | MIGRATED | `:164` to `ventilation-mechanical-occupant`; use at `:757` |
| natural `_stand` | MIGRATED | `🪟️natural/🐍️.py:132` to `ventilation-natural-occupant`; uses 363, 1041, 1479 |
| heat-flow heater | MIGRATED | `🌡️heat-flow/🐍️.py:322` to `ventilation-heat-flow-heater` |
| handoff | clean | no staffage |

### 2.3 heating

| Inventory row | Status now | Where |
|---|---|---|
| fundamentals `_radiator`, `_occupant` | MIGRATED | `🧲️fundamentals/🐍️.py:122-129` |
| conduction `_person_breathing` | MIGRATED | `🧱️conduction/🐍️.py:246` (`heating-conduction-person`) |
| conduction `_toaster` | MIGRATED | `:249-251` (`heating-conduction-toaster`) |
| conduction `_tealight` | AD HOC | `:259` (A12) |
| conduction `_devices` wrapper | AD HOC (wrapper for A12) | `:268` |
| conduction heater body | MIGRATED | `:1010-1011` (`heater_cast`) |
| convection | clean | none |
| internal-gains desks, laptop, lamps, occupant (9 rows) | MIGRATED | `👥️internal-gains/🐍️.py:109, 249-254, 445, 545, 616-621` |
| internal-gains watt badge (seated person as bulb) | WRONG ICON | `:390` (B4) |
| solar-gains blind | MIGRATED | `☀️solar-gains/🐍️.py:857` `BLIND_CAST`, staffage at `:1028` |
| calculation (6 casts) | MIGRATED | `🧮️calculation/🐍️.py:214-215` (`build`), casts at 223, 713-719, 1110-1113, 1256-1257, 1304 |
| calculation thermometer, sun, moon icons | SYMBOL | `:588, :692, :921-922, :1102` (not re-verified line by line) |

### 2.4 cooling

| Inventory row | Status now | Where |
|---|---|---|
| heating-comparison occupant, table, toaster, desk, laptop, lamp | MIGRATED | `⚖️heating-comparison/🐍️.py:129-150` |
| heating-comparison `fan_ring` desk fan / split unit | AD HOC | `:190-206` (A13) |
| heating-comparison watt laptop | MIGRATED (device badge) | `:283` (`shared-watt-laptop`) |
| heating-comparison watt person as bulb | WRONG ICON | `:284` (B2) |
| internal-gains `_figure`, `_seated`, `_workstation` (seated worker, chair, desk, monitor) | MIGRATED | `👥️internal-gains/🐍️.py:112-160`, casts 610-690 |
| internal-gains watt body heat as bulb | WRONG ICON | `:328` (B3) |
| internal-gains crowd and mini person | MIGRATED | `:446, :491, :513, :812` (`cooling-internal-gains-standing`) |
| internal-gains toaster badge | MIGRATED | `:530` (`shared-watt-toaster`, dark-line, C5) |
| internal-gains laptop, monitor, lamp | MIGRATED | `:607` (server-rack), `:669`/`:684` (monitor, monitor-off), `:707`/`:940` (lamp-dimmed), `:818` (lamp) |
| internal-gains activity SVGs `🏃️activity` | REMOVED | directory gone; test at `…/🧪️tests/🐍️.py:28` |
| summer-strategy `_person` and blind | MIGRATED | `🌤️summer-strategy/🐍️.py:93-106` (`cooling-summer-strategy-occupant` at 95, `-blind` at 102) |
| summer-strategy microbe | SYMBOL | `:438` |
| solar-radiation watt irradiance as vacuum | WRONG ICON | `☀️solar-radiation/🐍️.py:220` (B1) |
| transmission-humidity sun | SYMBOL | `💧️transmission-humidity/🐍️.py:142` |
| system-sizing discs | FP | `📐️system-sizing/🐍️.py` |

### 2.5 energy balance (demand)

| Inventory row | Status now | Where |
|---|---|---|
| recap person, radiator, lamp, desk, laptop | MIGRATED | `🔁️recap/🐍️.py:200-218` |
| regulations person, lamp, radiator | MIGRATED | `⚖️regulations/🐍️.py:327-336, 390-392, 424` |
| demand-power radiator, person, lamp | MIGRATED | `⚡️demand-power/🐍️.py:256, 285-294, 748-749` |
| system-losses person, radiator | MIGRATED | `♨️system-losses/🐍️.py:293-296, 510-511` |
| system-losses flame icon | SYMBOL | `:299-301` |
| system-losses gas boiler | AD HOC | `:306` (A1) |
| system-losses fan | AD HOC | `:314` (A2) |
| system-losses heat pump | AD HOC (asset exists) | `:323` (A3) |
| system-losses tank | AD HOC | `:352` (A4) |
| primary-energy occupant, radiator | MIGRATED | `🏭️primary-energy/🐍️.py:128-145` |
| primary-energy factory (power plant) | ICON (judgment F1) | `:202` |
| primary-energy pump | AD HOC | `:207` (A5) |
| primary-energy tree (fuel) | MIGRATED | `:223-226` (`demand-primary-energy-tree`) |
| primary-energy boiler | AD HOC | `:229` (A6) |
| primary-energy heat pump | AD HOC (asset exists) | `:237` (A7) |
| certificate person, walker, sleeper, lamp, radiator, buyer, tenant | MIGRATED | `🏷️certificate/🐍️.py:253-270, 373-374, 417-426, 846-848` (buyer and tenant dark-line: C3, C4) |
| practice occupant, talker, radiator | MIGRATED | `🧪️practice/🐍️.py:139-154` |
| practice boiler | AD HOC | `:998` (A8) |
| practice lamp | MIGRATED | `:1086` (`demand-practice-lamp-on/off`) |
| practice Ellipse, Dot, Circle | FP | `:161, :642, :656, :664` |

### 2.6 BIM

| Inventory row | Status now | Where |
|---|---|---|
| consistency reviewer and decider | MIGRATED | `🔗️consistency/🐍️.py:273-276` (default cast), `:1109` (decider) |
| objects `_tree` (IFC class tree) | FP | `🧬️objects/🐍️.py:333` |
| information car | MIGRATED | `📊️information/🐍️.py:210-212` (`bim-information-car`, dark-line plan view; a vehicle, not heat-emitting) |
| representations, models, exchange | clean | none |

### 2.7 Grasshopper

All rows are FP or clean (data trees, data plates, geometry stand-in). No change.

### 2.8 Excel

| Inventory row | Status now | Where |
|---|---|---|
| program occupant | MIGRATED | `📋️program/🐍️.py:354-357` (`excel-program-occupant`, dark-line plan) |
| silhouettes (data tiles) | FP | `:191` |

### 2.9 Optimization (manifest)

| Inventory row | Status now | Where |
|---|---|---|
| extrema lamp row `_lamps` | MIGRATED (row gone, `_lamps` is now a table) | `📉️extrema/🐍️.py:314` is a text table; lamp at `:439` (`optimization-index-lamp`) |
| extrema lamp glyph | MIGRATED | `:439` |
| extrema radiator | MIGRATED | `:439-440` (`optimization-extrema-radiator`) |
| extrema cooler | MIGRATED | `:440` (`_room._cooler`, cast in `🏠️room/🐍️.py:1347-1349`) |
| index criteria icons | MIGRATED (lamp) / SYMBOL (sun, factory) | `⚖️index/🐍️.py:127-136` (lamp at 133-134, factory is F1) |
| experiments cooler | MIGRATED | `🧪️experiments/🐍️.py:271` |
| experiments occupant, tilt window | MIGRATED | `:275, :905-906` |
| experiments lamp | MIGRATED | `:279` (`pendant`) |
| algorithms, pareto, sampling `_figures` | FP | `🧬️algorithms/🐍️.py:86-109` |

### 2.10 Shared optimization room (`🏠️room/🐍️.py`)

| Inventory row | Status now | Where |
|---|---|---|
| lamp_glyph x3 | MIGRATED | `:586` (`pendant` at `:1334-1339`) |
| occupant, radiator | MIGRATED | `:587-588` |
| cooler | MIGRATED | `:784`, `:1347-1349` (`optimization-room-cooler`) |
| sun icon | SYMBOL | `:585`, `:1298` |

### Section 3 (inventory) cast layer

The inventory said 28 manifest casts and about 50 missing. Both numbers are stale. The manifest now has 114 casts and no literal cast id in clip code is missing from it.

---

## 2. Staffage-like constructs in clip code (check 2)

Scan: every `🐍️.py` under `🎓️teaching/🏛️architecture/` excluding `.venv`, `🧍️staffage`, tests. Searched for `Circle`/`Line`/`Polygon`/`VMobject`/`SVGMobject` used as figures, body-part words, and defs named person, human, figure, tree, car, chair, desk, bed, sofa, lamp, radiator, table, laptop, monitor, vacuum, boiler, pump, fan, toaster, heater, tealight, shower, cooler, stand, heater.

- Figures: no stick-figure construction found. No SVGMobject of figures. The remaining `_stand`, `_person`, `_figure`, `_occupant`, `_heater`, `_lamp`, `_radiator`, `_vacuum`, `_tree`, `_toaster`, `_person_breathing`, `_car` helpers all call the cast API, except the items in list A.
- Devices still hand-built: list A (boilers, pumps, fans, heat pumps, tank, tealight, split unit, shower head).
- Dead or unused helpers: list D.
- Not staffage (false positives): `_figures` (grasshopper and algorithms data), `_tree` in BIM objects (IFC class tree), `_standin` (casting geometry), `silhouettes` (excel program, morph shapes), `_mini_room` (architecture box), `person_bars` (chart), `_car` in BIM model (IFC geometry).

---

## 3. Cast ids in clip code and manifest checks (check 3)

Method: literal first-argument calls and `cast=`/`on_cast=`/`off_cast=` keywords, quoted-string search for every manifest key across clip code, and manual trace of prefix-built and variable-built ids (`_CAST + name`, `BLIND_CAST`, `heater_cast`, `lamp_cast`, `_ACTIVITY_CAST`, `f"ventilation-volume-crates-{cast}"`, `_WATT_CASTS`). Script output kept in the ticket scratchpad, not in the repo.

Result:

- Literal cast ids used in code and missing from manifest: **0**.
- Prefix-built ids (`cooling-internal-gains-` + name, 13 keys) all exist: sleeper, seated-worker, walker, jogger, standing, laptop, desk, office-chair, server-rack, lamp, lamp-dimmed, monitor, monitor-off. Source: `👥️internal-gains/🐍️.py:93, 112-115, 159-165`.
- Crates (`ventilation-volume-crates-low/half/high`) via f-string at `📦️volume/🐍️.py:431`. All exist.
- Variable ids: `BLIND_CAST` (`☀️solar-gains/🐍️.py:857`), `heater_cast` (`🧱️conduction/🐍️.py:1010`), `lamp_cast` (`📉️extrema/🐍️.py:439`), `optimization-index-lamp`, all exist.
- Manifest structure, all 114 entries: `theme` present and `dark`; `view` valid (`dark-line`, `dark-solid`, `thermal`); `viewpoint` valid where set; exactly one of `pose` or `clip`; asset folder exists under `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/**`; pose or clip slug exists in that asset's `asset.poses` or `asset.clips`; `environment` slug exists in `🌤️environments/🔣️.json` (thermal entries only). **0 failures.**

Manifest orphans (no code path; they are not referenced by clip code):

| Cast | Asset | Status |
|---|---|---|
| `standing` | walking-commuter / standing | orphan (D3) |
| `walking` | walking-commuter / walk-cycle | orphan (D4) |
| `radiator` | panel-radiator / rated, dark-solid | orphan (D5) |
| `seated-reader` | seated-reader / seated-upright | orphan (D6) |
| `shared-radiator` | panel-radiator / rated | effectively orphan: only via dead `radiator()` (D1) |

Everything else in the manifest is used. The cooling and heating internal-gains casts are used through the `_CAST` prefix, so they are not orphans.

---

## 4. Energy clip views (check 4)

All casts with a non-thermal view that clip code in `⚡️energy/**` actually uses:

| Cast | View | Kind | Verdict |
|---|---|---|---|
| `physics-fundamentals-clerk` | dark-line | person | **should be thermal** (C1) |
| `demand-practice-talker` | dark-line | person | **should be thermal** (C2) |
| `demand-certificate-buyer` | dark-line | person | **should be thermal** (C3) |
| `demand-certificate-tenant` | dark-line | person | **should be thermal** (C4) |
| `shared-watt-toaster` (badge) | dark-line | heat-emitting device | **should be thermal** (C5) |
| `shared-watt-heater` (badge) | dark-line | heat-emitting device | **should be thermal** (C6) |
| `shared-watt-laptop`, `shared-watt-bulb` (badge) | dark-line | heat-emitting device | judgment, decide per badge |
| `shared-watt-vacuum` (badge) | dark-line | wrong use: irradiance (B1) | wrong icon |
| `shared-radiator` | dark-line | heat-emitting device | only via dead `radiator()` (D1) |
| `cooling-internal-gains-desk`, `-office-chair`, `heating-internal-gains-desk`, `demand-recap-desk`, `heating-calculation-desk`, `cooling-heating-comparison-desk`, `cooling-heating-comparison-table`, `heating-solar-gains-blind`, `demand-primary-energy-tree`, `ventilation-volume-blackboard`, `ventilation-volume-crates-*` | dark-line | furniture, objects, vegetation, blind | acceptable (not heat-emitting) |

Energy people in thermal (correct): occupants in cooling, heating, ventilation, demand (system-losses, primary-energy, demand-power, regulations, recap) and the thermal staffage pathways.

---

## Recommendations (ordered)

1. Fix the four watt badges (B1 to B4) and add a person mapping to `_WATT_CASTS` in `V/🐍️.py:364-378`. This is the cheapest visible fix.
2. Switch the four energy people (C1 to C4) to `thermal` with an environment, or add thermal casts for them.
3. Route the two heat pumps (A3, A7) and the split unit (A13) to existing assets now, with no new asset needed.
4. Create the missing assets (boiler, pump, fan, tank, tealight, shower head) or explicitly accept them as documented symbols. Decide the fan rule first (F2).
5. Delete dead code and orphans: `radiator()`, `respiration_parts()`, manifest keys `standing`, `walking`, `radiator`, `seated-reader` (or wire `seated-reader` into the seated-person badges). Update the visuals icon test (`V/🧪️tests/🐍️icons.py:66`) with it.
6. Update the contract to say that `pendant_lamp` and `fitted_staffage` live in `V/🐍️.py`, or move them.
7. Refresh the inventory counts (28 casts and about 50 missing are stale; the manifest now has 114 casts and zero missing).
