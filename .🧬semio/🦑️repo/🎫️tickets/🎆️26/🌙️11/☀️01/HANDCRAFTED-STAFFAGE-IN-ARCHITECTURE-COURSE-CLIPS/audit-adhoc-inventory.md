# Ad-hoc Staffage Inventory: Architecture Course Clips

Read-only audit. No repo file was modified. Snapshot taken 2026-11-01 about 01:40 to 01:45 (local). The working tree is being edited by other agents right now (the staffage bridge `🎨️visuals/🧍️staffage/` appeared during the audit), so line numbers are a snapshot. Re-run the scan before acting on a specific line.

Base path for all relative paths below: `C:\git\semio\🎓️teaching\🏛️architecture\`

## Count summary

| Metric | Count |
|---|---|
| Videos in course manifest `⚡️energy/🎬️clip/🔣️.json` | 26 (energy 5, BIM 6, Grasshopper 5, Excel 4, optimization 6) |
| Unique segment scripts referenced by the manifest | 47 (plus 26 video-level driver scripts, all present) |
| Ad-hoc person call sites (`person_glyph`, hand-built `_stand`, `_person`, `mini_person`) | 11 in 3 clips (physics 3, practice 4, cooling internal gains 4) |
| Ad-hoc seated person call sites (`seated_person_glyph`) | 0 in clips now (helper still defined and tested) |
| Ad-hoc device/object call sites (`object_silhouette`, hand-built laptop, monitor, vacuum, boiler, heat pump, fans, lamps, cooler, tealight, shower head, power plant, pump) | about 50 across 15 clip files |
| `watt_anchor` call sites with a mismatched device (person shown as bulb, irradiance shown as vacuum, body heat shown as bulb) | 4 |
| Clip-level calls already routed through the staffage cast layer (`staffage`, `staffage_motion`, `staffage_height`) | about 45 call sites in 22 files |
| Cast IDs used by clip code but absent from the cast manifest `🎨️visuals/🧍️staffage/🔣️.json` (28 entries) | about 50 real (62 raw regex matches, a few are false positives) |
| False positives (not staffage) | `_tree` IFC class tree (BIM objects), Grasshopper data trees and `_figures` plates, Excel `silhouettes` data tiles, `_standin` geometry (casting), `_fan` in Grasshopper trees diagram, `respiration_parts` (defined, no call in manifest scope) |

Symbol icons (sun, moon, cloud, thermometer, clock, microbe, flame, droplets, bulb and lamp glyphs used as data markers) are excluded from the ad-hoc counts. They are not entourage. They are listed in section 4 for completeness.

## 1. Manifest clips and their scripts

| Series | Video id (output) | Chapter scene | Segment script (relative) |
|---|---|---|---|
| Energy | physical_fundamentals (Full_Physical_Fundamentals) | PhysicalFundamentals_FullSection (Beat1 to Beat8) | `⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` |
| Energy | ventilation (Full_Ventilation) | Ventilation_01 to _06 | `⚡️energy/🌬️ventilation/🎬️clip/❓️need`, `📦️volume`, `🌡️heat-flow`, `🪟️natural`, `⚙️mechanical`, `🤝️handoff` (each `🐍️.py`) |
| Energy | heating (Full_Heating_Demand) | Heating_01 to _06 | `⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals`, `🧱️conduction`, `🌬️convection`, `👥️internal-gains`, `☀️solar-gains` (+ physics fundamentals), `🧮️calculation` |
| Energy | cooling (Full_Cooling_Demand) | Cooling_01 to _06 | `⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison`, `👥️internal-gains`, `💧️transmission-humidity` (+ physics fundamentals), `☀️solar-radiation` (+ heating solar gains), `📐️system-sizing`, `🌤️summer-strategy` |
| Energy | energy_balance (Full_EnergyBalance) | EnergyBalance_01 to _07 | `⚡️energy/📊️demand/🎬️clip/🔁️recap`, `⚖️regulations`, `⚡️demand-power`, `♨️system-losses` (+ heating calculation), `🏭️primary-energy`, `🏷️certificate`, `🧪️practice` |
| BIM | bim_information, bim_objects, bim_representations, bim_models, bim_exchange, bim_consistency | BIM_01 to BIM_06 | `🏗️bim/🎬️clip/📊️information`, `🧬️objects`, `👁️representations`, `🧩️models`, `🔌️exchange`, `🔗️consistency` |
| Grasshopper | grasshopper_anatomy, _capsule, _tower, _casting, _trees | Grasshopper_01 to _05 | `🦗️grasshopper/🎬️clip/🦴️anatomy`, `💊️capsule`, `🗼️tower`, `🫗️casting`, `🌳️trees` |
| Excel | excel_anatomy, _references, _program, _energy | Excel_01 to _04 | `📊️excel/🎬️clip/🦴️anatomy`, `📍️references`, `📋️program`, `⚡️energy` |
| Optimization | optimization_extrema, _sampling, _experiments, _index, _pareto, _algorithms | Optimization_* (single chapter each) | `🎯️optimization/🎬️clip/📉️extrema`, `🎲️sampling`, `🧪️experiments`, `⚖️index`, `🏆️pareto`, `🧬️algorithms` |

Shared helper modules imported by manifest clips and not themselves in the manifest:
- `⚡️energy/🏗️authoring/🎨️visuals/🐍️.py`: `person_glyph` (l.1195), `seated_person_glyph` (l.1205), `object_silhouette` (l.1189), `radiator` (l.1134), `watt_anchor` (l.366), `respiration_parts` (l.315), `_SILHOUETTES` (l.28, loads `🫟️silhouettes/🔣️.json`).
- `🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py`: room scene helper used by all optimization clips. Hits in section 3.

## 2. Ad-hoc staffage by series and clip

Path root: `C:\git\semio\🎓️teaching\🏛️architecture\`. Status key: AD-HOC = drawn by primitives or the visuals helpers and not routed through the archive. CAST-MISSING = routed through `staffage*()` with a cast ID that the cast manifest does not define. SYMBOL = a device-like marker that the archive does not cover, which could stay as an icon.

### 2.1 Energy: physical_fundamentals

| File:line | Helper / construct | Depicts | Suggested archive asset (pose or clip) | Status |
|---|---|---|---|---|
| `⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:494` (`_person = person_glyph`), uses l.1049, l.1655, l.3095 | `person_glyph` | Standing adult, scale 1.5 and 2.6 for close-ups and person-to-room scale | `👤️humans/standing-adult`, pose `relaxed` (or `walking-commuter`, pose `standing`) | AD-HOC |
| `...🧲️fundamentals/🐍️.py:554-556` (`_vacuum`), used l.1109, l.2559 | `object_silhouette("vacuum")` | Vacuum cleaner canister, hose and wand | none; new `🔌️appliances/vacuum-cleaner` | AD-HOC (no asset) |
| `...🧲️fundamentals/🐍️.py:559` (`_shower_head`), used l.1157 | hand-built wall arm and rose | Shower head | none; new `🔌️appliances/shower-head` | AD-HOC (no asset) |
| `...🧲️fundamentals/🐍️.py:1459`, `:1560` (`_lamp`) | hand-built lamp | Ceiling or pendant lamp | none; new `🔌️appliances/pendant-lamp` (the heating clip already declares `heating-internal-gains-pendant-lamp`) | AD-HOC (no asset) |
| `...🧲️fundamentals/🐍️.py:1873`, `:2032`, `:2331` (`_radiator` alias of visuals `radiator`) | visuals `radiator()` (hand-built rectangle, grille, fins, feet) | Panel radiator | `🧱️components/panel-radiator`, pose `comfort` (or `rated`) | AD-HOC |
| `...🧲️fundamentals/🐍️.py:501`, `:526` (`_flame`) | path-drawn flame | Flame icon for combustion | keep as symbol (no archive flame) | SYMBOL |

Note: `person_bars()` at l.3108 is a chart, not staffage.

### 2.2 Energy: ventilation

| File:line | Helper / construct | Depicts | Suggested archive asset (pose or clip) | Status |
|---|---|---|---|---|
| `⚡️energy/🌬️ventilation/🎬️clip/❓️need/🐍️.py:142-147` (`_fan`), used l.637 | hand-built ring and three blades | Fan-driven airflow marker | none; keep as airflow symbol, or new `🔌️appliances/ventilation-fan` | SYMBOL |
| `...❓️need/🐍️.py:135-165` (`_stand`, `_room_occupants`) | `staffage_motion` with cast `ventilation-need-occupant` (+ mirrored) | Breathing occupant, office-neutral | `👤️humans/standing-adult`, pose `relaxed`, clip `idle-breathing` | CAST (in manifest) |
| `...📦️volume/🐍️.py:111-113` (`_fan`), used l.427 | hand-built ring and blades | Ventilation fan in a use-room | none; new `🔌️appliances/ventilation-fan` | SYMBOL |
| `...📦️volume/🐍️.py:291-295` (`_stand`), used l.1078 | `staffage_motion`, cast `ventilation-volume-occupant` | Occupant in use-room | `humans/standing-adult` pose `relaxed` | CAST-MISSING (casts `ventilation-volume-office-worker`, `-student`, `-cook`, `-surgical-lamp` absent) |
| `...📦️volume/🐍️.py:395-420` (`_mini_room`, `_use_glyph`) | hand-built box room; kind branches (storage, passive house, office, classroom, kitchen, operating room) | Use-type mini rooms with people, lamp, slabs | office: `🛋️furniture/height-adjustable-desk` seated-height plus `office-swivel-chair` seat-high; classroom: `🛋️furniture/cafe-chair` upright-empty; kitchen: `🔌️appliances/toaster` (kitchen counter is not in catalog); storage: `🪑️objects/shopping-trolley` loaded; operating room: none | AD-HOC (mostly no asset) |
| `...⚙️mechanical/🐍️.py:136-147` (`_fan`), used l.282, 296, 310, 311, 469, 470, 784, 804 | hand-built ring and blades | MVHR and exhaust fans in duct plans | new `🔌️appliances/mvhr-unit` (none in catalog) | SYMBOL (8 sites) |
| `...⚙️mechanical/🐍️.py:164-168` (`_stand`), used l.757 | `staffage_motion` cast `ventilation-mechanical-occupant` | Occupant, heated winter | `humans/standing-adult` relaxed, clip `idle-breathing` | CAST (in manifest) |
| `...🪟️natural/🐍️.py:132-136` (`_stand`), used l.363, 1041, 1479 | `staffage_motion` cast `ventilation-natural-occupant` | Occupant | `humans/standing-adult` relaxed | CAST (in manifest) |
| `...🌡️heat-flow/🐍️.py:322` | `staffage("ventilation-heat-flow-heater")` | Heater coil | `components/panel-radiator` rated | CAST (in manifest) |
| `...🤝️handoff/🐍️.py` | none | No staffage | none | clean |

### 2.3 Energy: heating

| File:line | Helper / construct | Depicts | Suggested archive asset (pose or clip) | Status |
|---|---|---|---|---|
| `⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:122-129` (`_radiator`, `_occupant`) | `staffage("heating-fundamentals-panel-radiator")`, `staffage_motion("heating-fundamentals-occupant")` | Radiator and occupant | `components/panel-radiator` (comfort), `humans/standing-adult` (relaxed, clip `idle-breathing`) | CAST-MISSING (both) |
| `...🧱️conduction/🐍️.py:251-254` (`_person_breathing`) | `staffage_motion("heating-conduction-person")` | Standing adult in thermal view | `humans/standing-adult` | CAST-MISSING |
| `...🧱️conduction/🐍️.py:257-263` (`_toaster`) | `staffage("heating-conduction-toaster")` | Toaster | `🔌️appliances/toaster` (on disk, not in catalog index) | CAST-MISSING |
| `...🧱️conduction/🐍️.py:266-272` (`_tealight`) | hand-built cup, wick, ellipse flame | Tealight candle | none; new `🪑️objects/tealight` | AD-HOC (no asset) |
| `...🧱️conduction/🐍️.py:275-281` (`_devices`) | wraps the above | Device row | as above | AD-HOC wrapper |
| `...🧱️conduction/🐍️.py:1018` | `staffage_motion(heater_cast)` | Heater body | `components/panel-radiator` | CAST-MISSING |
| `...🌬️convection/🐍️.py` | none | No staffage | none | clean |
| `...👥️internal-gains/🐍️.py:109`, `:249`, `:252`, `:254`, `:445`, `:549`, `:620`, `:623`, `:625` | `staffage_motion`/`staffage` with `heating-internal-gains-*` (office-worker, standing-adult, desk, laptop, pendant-lamp) | Office occupant, desk, laptop, pendant lamp | `humans/seated-reader` (seated-upright); `humans/standing-adult` (relaxed); `furniture/height-adjustable-desk` (seated-height); `furniture/office-swivel-chair` (seat-high); new `appliances/laptop`; new `appliances/pendant-lamp` | CAST-MISSING (all 9) |
| `...👥️internal-gains/🐍️.py:390` | `watt_anchor(PHI_P_W, compare="bulb", title="eine sitzende Person")` | Seated person labelled with a bulb | `humans/seated-reader`, pose `seated-upright` (bulb is the wrong icon) | AD-HOC (mismatch) |
| `...☀️solar-gains/🐍️.py:994` | `staffage("heating-solar-gains-blind")` | External blind | `components/external-venetian-blind`, pose `lowered-horizontal` | CAST-MISSING |
| `...🧮️calculation/🐍️.py:66-68`, `:215` | archive build helper, casts `heating-calculation-*` (person, laptop, desk, lamp, radiator, radiator-mirrored) | Occupant, desk, laptop, lamp, radiator | `humans/standing-adult`; `furniture/height-adjustable-desk`; new `appliances/laptop`; new `appliances/pendant-lamp`; `components/panel-radiator` (rated) | CAST-MISSING (all 6). The earlier scan had hand-built person and laptop at l.695, 703, 1096, 1099, 1243, 1290; those are now migrated. |
| `...🧮️calculation/🐍️.py:588`, `:692`, `:921-922`, `:1102` | thermometer, sun, moon | Icons | keep (not staffage) | SYMBOL |

### 2.4 Energy: cooling

| File:line | Helper / construct | Depicts | Suggested archive asset (pose or clip) | Status |
|---|---|---|---|---|
| `⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:125-146` | `staffage`/`staffage_motion`/`staffage_height` with `cooling-heating-comparison-occupant` (fanning), table, toaster, desk, laptop, lamp | Occupant, table, toaster, desk, laptop, lamp | `humans/standing-adult` (relaxed, or `seated-reader` if seated); `furniture/dining-table` (closed); `appliances/toaster`; `furniture/height-adjustable-desk` (seated-height); new `appliances/laptop`; new `appliances/pendant-lamp` | CAST-MISSING (all 6) |
| `...⚖️heating-comparison/🐍️.py:193-195` (`fan_ring` Circle plus blades) | hand-built fan | Desk fan | new `appliances/desk-fan` | AD-HOC (no asset) |
| `...⚖️heating-comparison/🐍️.py:279` | `watt_anchor(60, compare="laptop", title="Gerät")` | Laptop badge | new `appliances/laptop` | AD-HOC (no asset) |
| `...⚖️heating-comparison/🐍️.py:280` | `watt_anchor(100, compare="bulb", title="Person")` | Person badge drawn as a bulb | `humans/standing-adult` relaxed | AD-HOC (mismatch) |
| `⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:107-137` (`_figure`, `_seated`, `_workstation`) | `staffage_motion`/`staffage` with `cooling-internal-gains-*` (seated-worker, office-chair, desk, monitor) | Seated office worker, chair, desk, monitor | `humans/seated-reader` (seated-upright); `furniture/office-swivel-chair` (seat-high); `furniture/height-adjustable-desk` (seated-height); new `appliances/monitor` | CAST-MISSING |
| `...👥️internal-gains/🐍️.py:329` | `watt_anchor(100, compare="bulb", title="Körperwärme")` | Body heat badge drawn as a bulb | `humans/standing-adult` relaxed | AD-HOC (mismatch) |
| `...👥️internal-gains/🐍️.py:448`, `:493`, `:515`, `:825` (`person_glyph` and `mini_person`) | `person_glyph` (scale 1.25, 1.8, 0.5, 1.0) | Crowd of people, single person, icon person | `humans/standing-adult` (relaxed), `walking-commuter` (standing) for the crowd | AD-HOC (4 sites) |
| `...👥️internal-gains/🐍️.py:533` | `watt_anchor(5000, compare="toaster", title="Gesamtwärme")` | Toaster badge | `appliances/toaster` | AD-HOC (already matches an asset; route through cast) |
| `...👥️internal-gains/🐍️.py:607`, `:828`, `:956` | `object_silhouette("laptop")` | Laptop | new `appliances/laptop` | AD-HOC (no asset) |
| `...👥️internal-gains/🐍️.py:681` | `object_silhouette("monitor")` | Monitor | new `appliances/monitor` | AD-HOC (no asset) |
| `...👥️internal-gains/🐍️.py:716`, `:831`, `:962` | `lamp_glyph` | Ceiling and pendant lamps | new `appliances/pendant-lamp` | AD-HOC (no asset) |
| `...👥️internal-gains/🏃️activity/` (SVG `😴️sleep`, `🚶️walk`, `🏃️sprint`) | SVG activity assets (`_ACTIVITY_ROOT` no longer used by the clip) | Sleep, walk, sprint silhouettes | sleep: `furniture/single-bed` pose `asleep` (verify occupant); walk: `humans/walking-commuter` `standing`; sprint: `humans/jogger` `stretch-stand`, `humans/running-child` `mid-stride` | ASSET DRIFT (still referenced by visuals tests `🧪️tests/🐍️silhouettes.py:101` and the activity tests) |
| `...🌤️summer-strategy/🐍️.py:93-106` (`_person`, blind) | `staffage_motion`/`staffage` with summer-strategy cast | Occupant, window blind | `humans/standing-adult` (relaxed); `components/external-venetian-blind` (lowered-closed) | CAST-MISSING |
| `...🌤️summer-strategy/🐍️.py:438` | `object_silhouette("microbe")` | Microbe icon | keep as symbol | SYMBOL |
| `...☀️solar-radiation/🐍️.py:220` | `watt_anchor(800, compare="vacuum", title="Bestrahlungsstärke je m²")` | Solar irradiance badge drawn as a vacuum cleaner | `environments` preset (sun) or keep the sun symbol; do not use a vacuum | AD-HOC (mismatch) |
| `...💧️transmission-humidity/🐍️.py:142` | `sun_glyph` | Sun icon | keep (symbol) | SYMBOL |
| `...📐️system-sizing/🐍️.py` | Circle discs | Data discs | not staffage | false positive |

### 2.5 Energy balance (demand)

| File:line | Helper / construct | Depicts | Suggested archive asset (pose or clip) | Status |
|---|---|---|---|---|
| `⚡️energy/📊️demand/🎬️clip/🔁️recap/🐍️.py:200-218` | `staffage_motion("demand-recap-person")`, `staffage("demand-recap-radiator", "-lamp", "-desk", "-laptop")` | Occupant, radiator, lamp, desk, laptop | `humans/standing-adult`; `components/panel-radiator`; `furniture/height-adjustable-desk`; new `appliances/laptop`; new `appliances/pendant-lamp` | CAST-MISSING (5) |
| `⚖️regulations/🐍️.py:327-336`, `:390-392`, `:424` | `staffage_motion("demand-regulations-person")`, `staffage` lamp and radiator | Occupant, lamp, radiator | as above | CAST-MISSING (3) |
| `⚡️demand-power/🐍️.py:256`, `:285-294`, `:748-749` | `staffage` radiator, `_stand`, `_lamp` (casts `demand-power-*`) | Radiator, occupant, lamp | as above | CAST-MISSING (3) |
| `♨️system-losses/🐍️.py:293-296` (`_stand`), `:510-511` | `staffage_motion("demand-system-losses-person")`, `staffage("demand-system-losses-radiator")` | Occupant, radiator | `humans/standing-adult`; `components/panel-radiator` | CAST-MISSING (2) |
| `♨️system-losses/🐍️.py:299-301` (`_flame`) | `object_silhouette("flame")` | Flame icon | keep (symbol) | SYMBOL |
| `♨️system-losses/🐍️.py:306-311` (`_boiler`), `:513`, `:807` | hand-built casing with flame | Gas boiler | new `appliances/gas-boiler` (none in catalog) | AD-HOC (no asset) |
| `♨️system-losses/🐍️.py:323-328` (`_heat_pump`), `:545`, `:808` | hand-built box with fan | Heat pump | `machinery/heat-pump-outdoor-unit`, pose `heating` (or `defrosting`) | AD-HOC (asset exists, route it) |
| `🏭️primary-energy/🐍️.py:127-144` (`_stand`) and heater | `staffage_motion("demand-primary-energy-person")`, `staffage("demand-primary-energy-radiator")` | Occupant, radiator | `humans/standing-adult`; `components/panel-radiator` | CAST-MISSING |
| `🏭️primary-energy/🐍️.py:200-204` (`_power_plant`), `:458`, `:651` | `object_silhouette("factory")` | Factory or power plant | none; new `machinery/power-plant-stack` or keep symbol | AD-HOC (no asset) |
| `🏭️primary-energy/🐍️.py:207-213` (`_pump`), `:459`; inside `_heat_pump` at l.237-243 | hand-built ring and blades | Pump or compressor | new `appliances/circulation-pump` | AD-HOC (no asset) |
| `🏭️primary-energy/🐍️.py:223-227` (`_tree`), `:655` | `staffage("demand-primary-energy-tree")` (archive cast) | Wood fuel tree | `vegetation/english-oak` calm (only if entourage; for a fuel icon keep the symbol) | CAST-MISSING |
| `🏭️primary-energy/🐍️.py:229-235` (`_boiler`), `:455`, `:843` | hand-built casing with flame | Boiler | new `appliances/gas-boiler` | AD-HOC (no asset) |
| `🏭️primary-energy/🐍️.py:237-243` (`_heat_pump`), `:845` | hand-built box with pump | Heat pump | `machinery/heat-pump-outdoor-unit` | AD-HOC (asset exists) |
| `🏷️certificate/🐍️.py:253-270` (`_stand`, `_lamp`), `:373-374`, `:417-426`, `:846-847` | `staffage` casts `demand-certificate-*` (person, walker, sleeper, lamp, radiator, buyer, tenant) | Occupants, lamp, radiator | `humans/standing-adult`; `humans/walking-commuter` (standing); sleeper: `furniture/single-bed` (asleep); `components/panel-radiator`; new `appliances/pendant-lamp` | CAST-MISSING (all 7) |
| `🧪️practice/🐍️.py:139-142` (`_stand`) used at l.317, l.1154, l.1307, l.1308 | `person_glyph` (ad hoc) | Occupant, landlord, tenant | `humans/standing-adult` (relaxed); tenant `humans/standing-phone-user` (texting) | AD-HOC (4 sites) |
| `🧪️practice/🐍️.py:145-148` (`radiator` in `_stand` neighbour) | visuals `radiator()` | Radiator | `components/panel-radiator` | AD-HOC |
| `🧪️practice/🐍️.py:993-1000` (`_boiler`), l.1153 | hand-built boiler | Boiler | new `appliances/gas-boiler` | AD-HOC (no asset) |
| `🧪️practice/🐍️.py:1081` | `lamp_glyph` | Lamp | new `appliances/pendant-lamp` | AD-HOC (no asset) |
| `🧪️practice/🐍️.py:161`, `:642`, `:656`, `:664` | Ellipse, Dot, Circle | Streak and bulb markers | not staffage | false positive |

### 2.6 BIM (manifest)

| File:line | Helper / construct | Depicts | Suggested archive asset (pose or clip) | Status |
|---|---|---|---|---|
| `🏗️bim/🎬️clip/🔗️consistency/🐍️.py:273-276` (`person`, cast `bim-consistency-reviewer`, and `bim-consistency-decider` in the cast manifest) | `staffage` cast | Reviewer and decider | `humans/standing-adult` relaxed (reviewer) and `arms-crossed` (decider) | CAST (in manifest, OK) |
| `🏗️bim/🎬️clip/🧬️objects/🐍️.py:333`, `:372` (`_tree`) | IFC class tree | Class hierarchy, not a tree | none | false positive |
| `🏗️bim/🎬️clip/📊️information`, `👁️representations`, `🧩️models`, `🔌️exchange` | no hits | | | clean |

### 2.7 Grasshopper (manifest)

| File:line | Helper / construct | Depicts | Suggested archive asset | Status |
|---|---|---|---|---|
| `🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py:206`, `:680` (`_fan`) | diagram fan of data trees | Data trees | none | false positive |
| `🦗️grasshopper/🎬️clip/🌳️trees/...` `aggregate_tree`, `🦴️anatomy/🐍️.py:336`, `:682`; `🧱️model/🐍️.py:900-1464` (`_tree_item`, `_cast_tree`) | Grasshopper `Tree` data structure | Data trees | none | false positive |
| `🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py:40` (`_figures`) | data plates | Anatomy figures (diagram) | none | false positive |
| `🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py:516` (`_standin`) | geometry stand-in for a cast solid | Solid stand-in | none | false positive (verify) |
| `💊️capsule`, `🗼️tower` | no hits | | | clean |

### 2.8 Excel (manifest)

| File:line | Helper / construct | Depicts | Suggested archive asset | Status |
|---|---|---|---|---|
| `📊️excel/🎬️clip/📋️program/🐍️.py:354-357` (`occupant`), used by `plan(people=True)` at l.347 and `small_plans` l.381-391 | `staffage` cast `excel-program-occupant` (in manifest) | One person per occupant in a floor plan | `humans/standing-adult` relaxed | CAST (in manifest, OK) |
| `📊️excel/🎬️clip/📋️program/🐍️.py:191-204` (`silhouettes`) | morph silhouette of data tiles | Data tile shapes | none | false positive |
| `📍️references`, `🦴️anatomy`, `⚡️energy` | no hits | | | clean |

### 2.9 Optimization (manifest)

| File:line | Helper / construct | Depicts | Suggested archive asset | Status |
|---|---|---|---|---|
| `🎯️optimization/🎬️clip/📉️extrema/🐍️.py:314-340`, `:448` (`_lamps`) | hand-built lamp row | Ceiling lamps for lighting | new `appliances/pendant-lamp` | AD-HOC (no asset) |
| `...📉️extrema/🐍️.py:439` | `_art.lamp_glyph` | Lamp with cable and shade | new `appliances/pendant-lamp` | AD-HOC (no asset) |
| `...📉️extrema/🐍️.py:441` | `_art.staffage(cast="optimization-extrema-radiator")` | Radiator | `components/panel-radiator` rated | CAST (in manifest, OK) |
| `...📉️extrema/🐍️.py:441` | `_room._cooler` (room visuals l.1335) | Cooler or AC unit | `machinery/heat-pump-outdoor-unit`, pose `cooling` | AD-HOC (asset exists, route it) |
| `...⚖️index/🐍️.py:133` | `object_silhouette(GLYPHS[criterion])`: bulb, sun, factory | Criteria icons for electricity, daylight, carbon | electricity: `objects/street-lamp` (lit-night) or new pendant lamp; daylight: keep sun symbol; carbon: none | SYMBOL (criteria icons) |
| `...🧪️experiments/🐍️.py:271` | `_room._cooler` | AC unit | `machinery/heat-pump-outdoor-unit` cooling | AD-HOC |
| `...🧪️experiments/🐍️.py:275`, `:906` | `staffage` casts `optimization-experiments-occupant`, `-window` (in manifest) | Occupant, tilt window | `humans/standing-adult` relaxed; `components/tilt-window` tilted-open | CAST (in manifest, OK) |
| `...🧪️experiments/🐍️.py:279` | `_art.lamp_glyph` | Lamp | new `appliances/pendant-lamp` | AD-HOC (no asset) |
| `...🎲️sampling`, `🏆️pareto`, `🧬️algorithms/🐍️.py:86-109` (`_figures`) | data figures | Numbers, not people | none | false positive |

### 2.10 Shared optimization room scene (imported by all optimization clips, not in manifest)

| File:line | Helper / construct | Depicts | Suggested archive asset | Status |
|---|---|---|---|---|
| `🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py:586` | `lamp_glyph` x3 | Ceiling lamps | new `appliances/pendant-lamp` | AD-HOC (no asset) |
| `...🏠️room/🐍️.py:587`, `:588` | `staffage` casts `optimization-room-occupant`, `optimization-room-radiator` (in cast manifest) | Occupant, radiator | `humans/standing-adult` relaxed; `components/panel-radiator` rated | CAST (in manifest, OK) |
| `...🏠️room/🐍️.py:785`, `:1335` (`_cooler`) | hand-built cooler | AC unit | `machinery/heat-pump-outdoor-unit` cooling | AD-HOC |
| `...🏠️room/🐍️.py:585`, `:1298` | `sun_glyph` | Sun icon | keep (symbol) | SYMBOL |

## 3. Cast layer status (shared by all clips)

- `⚡️energy/🏗️authoring/🎨️visuals/🐍️.py:39-41` now imports `staffage`, `staffage_motion`, `staffage_height` from the new bridge `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py`. Before this snapshot, those names were undefined in the visuals module, so every clip that used them would have failed on import.
- The cast manifest `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json` defines 28 casts. They map to archive slugs, for example `standing-adult` (relaxed, arms-crossed), `panel-radiator` (rated, off, thermal), `tilt-window` (tilted-open), `ventilation-need-occupant` (`idle-breathing`, `thermal`, `office-neutral`). Only the ones listed in section 2 as CAST (in manifest) are fully resolved.
- About 50 cast IDs used by clip code are absent from that manifest (for example `heating-internal-gains-*`, `heating-calculation-*`, `demand-*`, `cooling-heating-comparison-*`, `ventilation-volume-*` except `-occupant`, `heating-conduction-*`, `heating-fundamentals-*`, `physics-fundamentals-*`, `bim-*` beyond the two listed). Clips that reference them will fail at cast lookup. Re-check after the owner finishes the cast manifest.
- The archive catalog `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json` lists 108 entries and 9 categories. `🔌️appliances/toaster` exists on disk but is not indexed. There is no laptop, monitor, vacuum, lamp, boiler, pump, fan, factory or tealight asset. Those are the gaps.
- The export work `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/` is untracked and in progress (`🟦️.ts`, `🧬️schema/`).

## 4. Excluded as icons or data (not staffage)

Sun, moon, cloud, thermometer, clock, droplets, microbe and flame glyphs, plus bulb-like markers used as data points, are symbols and were not counted. Examples in `⚡️energy/🏗️authoring/🎨️visuals/🐍️.py`: `sun_glyph` (l.1159), `moon_glyph` (l.1166), `thermometer_glyph` (l.1253), `cloud_glyph` (l.1223). `lamp_glyph` (l.1294) is counted above only where a lamp is a room object.

## 5. Suggested new catalog assets (no archive equivalent)

| Gap | Used by (clip, approx. sites) | Suggested category and slug |
|---|---|---|
| Laptop | cooling internal gains (3), heating-comparison (1), heating calculation, recap, internal gains heating | `appliances/laptop` (open or closed, screen-on pose) |
| Monitor | cooling internal gains (1) | `appliances/monitor` (on, off) |
| Pendant or ceiling lamp | physics (2), heating internal gains, certificate, practice, optimization (5 sites) | `appliances/pendant-lamp` (off, on, dimmed) |
| Vacuum cleaner | physics fundamentals (2), solar radiation badge (1, wrong icon used now) | `appliances/vacuum-cleaner` |
| Shower head | physics fundamentals (1) | `appliances/shower-head` |
| Gas boiler | system losses (2), primary energy (2), practice (1) | `appliances/gas-boiler` (off, firing) |
| Circulation pump | primary energy (2) | `appliances/circulation-pump` |
| MVHR or ventilation fan | ventilation mechanical (8), volume (1), need (1) | `appliances/mvhr-unit` (off, run, boost) |
| Desk fan | heating-comparison (1) | `appliances/desk-fan` |
| Tealight | heating conduction (1) | `objects/tealight` |
| Power plant or factory silhouette | primary energy (2), optimization index (1) | `machinery/power-plant-stack` or keep as symbol |
| Sleeping occupant | certificate (sleeper), activity sleep | `furniture/single-bed` pose `asleep` (verify it carries an occupant) or new `humans/sleeping-adult` |

Existing archive assets that cover the rest: `humans/standing-adult` (relaxed, arms-crossed, pointing, hands-warming), `humans/seated-reader` (seated-upright, reclined-reading, reaching-for-cup), `humans/walking-commuter` (standing, walk-cycle), `humans/jogger` (stretch-stand), `humans/standing-phone-user` (texting), `humans/running-child`, `furniture/height-adjustable-desk` (seated-height, standing-height), `furniture/office-swivel-chair` (seat-high), `furniture/cafe-chair` (upright-empty), `furniture/dining-table`, `furniture/single-bed` (asleep), `components/panel-radiator` (rated, comfort, off), `components/external-venetian-blind` (lowered-horizontal), `components/tilt-window` (tilted-open), `objects/street-lamp` (lit-night), `machinery/heat-pump-outdoor-unit` (heating, cooling, defrosting), `appliances/toaster`, `vegetation/english-oak` (calm).

## 6. Recommended follow-up (for the owner, not done here)

1. Finish the cast manifest so the about 50 missing cast IDs resolve, or drop the casts that are no longer used.
2. Route the hand-built device helpers to the archive: `heat-pump-outdoor-unit` (system losses, primary energy, optimization cooler), `panel-radiator` (visuals `radiator()`), `toaster`, `standing-adult` and `seated-reader` (replace `person_glyph` and `seated_person_glyph`).
3. Fix the three wrong `watt_anchor` icons: person shown as bulb (cooling internal gains l.329, heating-comparison l.280, heating internal gains l.390) and irradiance shown as vacuum (solar radiation l.220).
4. Decide the gap list in section 5 (new archive assets) before writing more hand-built devices.
5. Retire `🏃️activity` SVGs once the activity tests move to the archive walk and sprint casts.

Re-run this audit after the in-progress edits settle, because several files changed between the first and second scan of this session (heating calculation, heating internal gains, certificate, recap and regulations moved from hand-built to cast calls).
