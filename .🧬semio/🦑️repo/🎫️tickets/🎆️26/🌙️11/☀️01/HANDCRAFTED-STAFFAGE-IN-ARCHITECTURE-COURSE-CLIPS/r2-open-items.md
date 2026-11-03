# R2 open items (agent oi, status pass by agent ol)

Read-only pass over the 26 `r2-*-report.md` files and `summary.md`. No code or asset was changed. Paths are relative to `🎓️teaching/🏛️architecture/` (A) unless shown otherwise. `S` = `A/🧍️staffage/🗂️catalog`, `V` = `A/⚡️energy/🏗️authoring/🎨️visuals`, `E` = `A/⚡️energy`.

Status pass (ol, final): all 62 `r2-*-report.md` files and `audit-final-static-r2.md` were read. The `status (ol)` column is the current state and supersedes the older "later resolution" column. Status codes: **done** (the report named in brackets fixed it), **ruled no-change** (the reason is given), **open** (still open, with owner where a report names one). Spot checks in the source tree are noted per row. New items raised by the reports are in section 7 (OI-110 to OI-135).

Abbreviations for reports: `ha1` `ha2` `ha3` audit appliances and handcrafted; `hl`, `hl2` language; `kp` kettle and humidifier; `hz` static sweep; `g4` `g5` `g6` guard and devices; `rb` BIM; `rc` cooling; `rd1` `rd2` demand; `rh` `rh2` heating; `rp1` `rp2` physics; `rv` ventilation; `rx` `rx2` non-energy; `ro` optimization; `ht` test sweep; `hp` cache and export; `cx` `px` cancellation; `cs` catalog style; `hs` style pass; `hc` hygiene; `tc` test coverage; `fx` `fx2` test fixes; `ex` full export; `ep` export-performance; `vf3` vitest; `xc` cross-clip; `mv` move and sorption; `pa` asset physics; `pv` drawings; `ph` phase change; `cp` `cp2` cooking pot; `ts` thermal span; `tp` thermal physics; `tt` speech; `nv` narration; `bp` breathing; `sm2` humidifier and shower; `sl` system-losses; `dc` docs; `dl` `dp` dependencies; `it` integration.

## 1. Blocking correctness

| id | area | file:line | what is open | raised by | later resolution | status (ol) |
|---|---|---|---|---|---|---|
| OI-01 | BIM model | `🏗️bim/🎬️clip/🧱️model/🐍️.py:2268` | Finish reserve `RES-FIN-ROOF-GRAVEL-SL-R-705` not fully occupied (about 0.00297 m3). `building_geometry` takes more than 2 h, so every BIM clip that calls it (information, consistency, exchange) cannot render. | rb | none. Model owner must fix. | open (BIM model owner, not named). Checked: rb says model unchanged. |
| OI-02 | BIM tests | `🏗️bim/🎬️clip/🧪️tests/🐍️.py:287` (`ModelCase.setUpClass`) | Test class setup is the same geometry build, so `BimInformationContract` and `BimConsistencyContract` stop with 0 tests run and a setUpClass error after about 2 h. | rb | none. | open (same root as OI-01). |
| OI-03 | staffage guard, Rule A | `E/📊️demand/🎬️clip/🧪️practice/🐍️.py` (`_gas_meter`) | Primitive-built gas meter (box, dial, needle). Not allowlisted. Manifest already has `gas-meter` casts, so it should be a cast. Makes the guard red. | cs, rd1, rd2, rh, rb, rv, rx, hz (P5), g6 | Partly resolved. Open. | done (g4). `_gas_meter` (practice py:961) is now only a `fitted_staffage` cast wrapper with no primitives. Guard green in g4 and sm2. |
| OI-04 | practice building store (H10) | `E/📊️demand/🎬️clip/🧪️practice/🐍️.py` (`energy_tank(... height=1.30, width=0.80 ...)`, now about line 1142) | Building hot-water or buffer store drawn as an abstract rectangle. Coordinator ruled it must use the `hot-water-cylinder` asset. | hz (P6) | Open. | open (coordinator). g4 kept `energy_tank` on purpose (it is the "Quelle" gauge of the balance cut) and asked the coordinator to overrule. Still `energy_tank(...)` in the file. |
| OI-05 | staffage guard allowlist | `V/🧪️tests/🔣️staffage_only.json` `rule_a.allowed` (`primary-energy/🐍️.py::_plant_panel`) | Reported as a stale entry. | rv | Unverified. | done (g4 final run passed; sl, tp saw the stale entry before that). Spot check: no `_plant_panel` entry in the JSON now. |
| OI-06 | staffage guard, coverage | `V/🧪️tests/🔣️staffage_only.json` `rule_a.allowed` | g6 says the guard must print OK after g4 lands. The final guard run was never reported. | g6, rd1, rd2, rp1, rp2 | Open (guard not re-run). | done (sm2: staffage_only 20 tests OK after the xc merge; g4 final pass). Rule A is still name-filtered, see OI-40. |
| OI-07 | catalog React test | `S/🔣️.json:6545` (manifest entry `plate-heat-exchanger`) | React catalog-loading test fails with `manifest-mismatch` at `entries[137].analytics`. | ht | Unverified (probable open, not re-run). | done (fx2: `transport` added to the manifest entry; sweep 147 entries, 0 mismatches). vf3 full catalog rc=0; React 107 pass. |
| OI-08 | heading test | `🔤️fonts/🧪️tests/📑️heading`, excel `⚡️energy/🐍️.py` and grasshopper `🫗️casting/🐍️.py` | Two failing assertions for the removed `_title` heading API. | ht | Unverified (not re-run). | done (fx2: `_title` renamed `_heading` / `_view_title`; heading test 3 of 3 OK). |
| OI-09 | integration test | `V/🧪️tests/🐍️integration.py` (`test_physics_scene_constructor_bounds`) | Radiator height 0.3408 vs expected 0.408. | rb, rd1 | Unverified (probably changed, not re-run). | done (it: test corrected to the front cast and the winter radiator AST; integration 9 of 9 OK). Scene judged correct. |
| OI-10 | cast exporter memory | cast `heating-internal-gains-server-rack` (1206 MB peak), `demand-primary-energy-tree` (757 MB) | Peak memory not explained by frame count. Not investigated. | hp | Open. | open (performance owner, not named). ex saw batch peaks up to 767 MB but did not explain the per-cast peak. |
| OI-11 | cast size budget | cast output, 73 casts above 200 KiB total | Unclear whether the 200 KB limit is per frame or per cast. | hp | Judgement, open. | open (judgement, no owner). ex: per frame only the tree and two volume casts exceed 200 KiB. |
| OI-12 | cooling placement | `❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py` (`_build_ahu`) | Two reports describe different fixes (rc shift form, cs vane lift). | rc, cs | Unverified (render). | done (ts: cooler redesigned as a wall-mounted unit, `MOUNT_TOP_FRACTION`; heating-comparison Beat1 to Beat3 rendered, no overlaps). rc and cs placement is superseded. |

## 2. Physics (thermal and energy balance)

| id | area | file:line | what is open | raised by | later resolution | status (ol) |
|---|---|---|---|---|---|---|
| OI-20 | cooking pot energy budget | `S/🪑️objects/cooking-pot/🔣️.json` | Water capacity about 125 times too small. Steam not modelled. Needs one coordinated redesign. | ha3 (item 2) | Open. | done (cp2: 1 L and 1 kg pot, `heatCapacityJM2K` 208 150; ph phase change gives steam; solver boils at 319 s; clip retimed; scene `m = 1 kg` matches). |
| OI-21 | hot-water cylinder | `S/🔌️appliances/hot-water-cylinder/🔣️.json` | Title "200 L" and mass 275 kg, but 1.6 m high and 0.54 m jacket. Product choice. | ha1 | Judgement, open (kp ruled it untouched). | done (pa: redrawn, jacket 0.60 m, height 1.48 m, width 0.66 m, dry mass 80 kg, 200 kg water as filling). Residual: 1.48 m is still above the 1.2 to 1.3 m typical for 200 L; product title to be confirmed by the owner. |
| OI-22 | gas boiler | `S/🔌️appliances/gas-boiler/🔣️.json` | Flue gas emitter carries about 330 W of 20 kW input. Confirm intended share. | ha1 | Judgement, open. | done (pa: flue 430 W, 2.1 % of input, documented as the intended band). |
| OI-23 | electric kettle mass | `S/🔌️appliances/electric-kettle/🔣️.json` | Mass convention: dry mass vs water mass. `massKg` 2.22 includes the 1 kg water bone. | ha1, kp | Judgement, open. | open (owner ph or coordinator). pa says to set `massKg` 1.22 for the dry-mass convention. Not applied. |
| OI-24 | evaporative humidifier drip | `S/🔌️appliances/evaporative-humidifier/🔣️.json` (clip `drip`) | Clip is non-looping. Cast callers depend on the loop flag. | ha1, kp | Judgement, open. | done (sm2: `loop` true, keys at 0, 1.1 and 1.2 s; hard reset at the wrap, noted as a limit). |
| OI-25 | sorption wheel | `S/🏗️machinery/sorption-wheel/🔣️.json` | Temperatures set through power. Adsorption heat and 70 C regen air do not go through air emitters. | ha2 (item 3) | Judgement, open. | done (mv: physical model with `process-air` and `regen-air` vent emitters, coil heated to 70 C, `rotation` and `regeneration` keys). Residual narration: OI-112. |
| OI-26 | toaster surface | `S/🔌️appliances/toaster/🔣️.json` | Top plate 100 W on 0.045 m2, too hot for a plate. | ha2 (item 11) | Open. | done (pa: top plate 100 W to 25 W, about 75 C; slot air takes 75 W). Casing residual: OI-113. |
| OI-27 | shower spray | `S/🔌️appliances/shower-head/🔣️.json` | Spray 300 W from 12 L/min at 38 C is a fudge-like source. | ha2 (item 11), summary | Judgement, open. | open (judgement, no owner). No report changed the spray. sm2 notes the spray is not visible at card scale. |
| OI-28 | pendant lamp | `S/🔌️appliances/pendant-lamp/🔣️.json` | Model books 100 % of the 60 W as heat. Lampholder area 0.012 m2 vs about 0.005 m2. | ha2 (items 6, 7) | Judgement, open. | done (pa: lampholder 0.005 m2; kept as 60 W incandescent with 95 % to surfaces and 3 W light absorbed). |
| OI-29 | thermostatic valve | `S/🔌️appliances/thermostatic-radiator-valve/🔣️.json` | Body 3 W plus the 55 C pipe contact. Possible double count. | ha2 (item 8) | Open. | done (pa: removed the 55 C pipe contact from the three poses; 3 W standing heat is the sole source). |
| OI-30 | MVHR casing | `S/🔌️appliances/mvhr-unit/🔣️.json` | Casing 45 W behind R 0.5 may run warm. No loop clip for steady spin. | ha2 (items 10, 11) | Open. | done (pa: casing 45 W to 12 W; new looping clip `normal-spin`). |
| OI-31 | plate heat exchanger and split AC partial states | `S/🏗️machinery/plate-heat-exchanger/🔣️.json`, `S/🔌️appliances/split-ac-indoor-unit/🔣️.json` | Partial bypass scales temperature linearly (true half bypass about -12 K supply, not -1.6 K). | ha2 (item 5) | Accepted approximation. | ruled no-change (ha2 accepted the linear approximation). |
| OI-32 | split AC latent | `S/🔌️appliances/split-ac-indoor-unit/🔣️.json` | 2.5 kW rating is about 2.0 kW sensible plus about 0.5 kW latent. Latent not modelled. | ha2 (item 9) | Accepted. | ruled no-change (ha2 accepted). |
| OI-33 | pumpjack, expansion valve | `S/🏗️machinery/oil-pumpjack/🔣️.json`; `S/🏗️machinery/expansion-valve/🔣️.json` | Pumpjack height and motor heat low. Valve mass high. | ha3 (item 9) | Open. | done (pa: motor 1800 W, gears 720 W, `heightM` 5.57 m at mid-stroke with 6.27 m stopped pose documented; valve mass 0.45 kg). pv's 6.3 m failure was an intermediate value and is gone. Residual: OI-114. |
| OI-34 | panel radiator mass | `S/🧱️components/panel-radiator/🔣️.json` | `massKg` 255.68 includes the 229.5 kg wall bone. Owner must choose: keep the wall in the asset or move it out. | ha3 (item 3) | Decision, open. | done as a workaround (pa: wall bone mass set to 0, `massKg` 26.18, documented). Geometry still includes the wall; the owner may still move it out. |
| OI-35 | sleeping metabolic rate | `S/👤️humans/sleeping-person/🔣️.json` | `activityMet` 0.8 vs ISO 8996 and ASHRAE 0.7. Breathing amplitude 0.5 deg / 0.003 m, below the rule example. | ha3 (items 4, 5) | Judgement, open. | done for MET (pa: 0.7 in poses, base and turn-over track). Breathing amplitude ruled no-change (ha3: catalog-wide norm). |
| OI-36 | colour range | cast exporter `🎨️depiction/🟦️.ts:66` (`range: shared`) | Shared range -20 to 60 C makes a 35 C laptop in a 22 C office pale. `spanK` proposed, not implemented. | rc, cs | Judgement, open. | done (ts: `thermalSpanK` 25 K on all 85 thermal casts of the manifest; schema, twin and exporter; laptop reads 0.56 of the ramp, was 0.36). |
| OI-37 | thermal rule, blinds | `E/🧲️physics/…/🧲️fundamentals/🐍️.py` (`BLIND_CAST`), `E/🔥️heating/🎬️clip/☀️solar-gains/🐍️.py` | `physics-fundamentals-blind` and `heating-solar-gains-blind` are dark-line, but shade radiant heat gain. | hz (§3) | Judgement, open. | done (g5: both thermal, environment `summer-hot-sun`; geometry verified equal to the dark-line cast). |
| OI-38 | thermal rule, optimization | `V/…/🎯️optimization/…` (room lamps, index lamp, extrema, experiments occupant and window) | Lamps are heat sources but dark-line. Hz says switch to thermal. | hz (§3) | Open. | done (g5: room lamps, index lamp, experiments occupant and window thermal; extrema cooler and radiator already thermal; experiments occupant enlarged to 0.85 m). |
| OI-39 | guard Rule B and C | `V/🧪️tests/🐍️staffage_only.py` | Rule B (energy clips naming dark-line casts) and Rule C not implemented. | hz (§4) | Open (proposal). | done (g6: Rule B implemented with allowlist of 8 furniture ids, verified by dc; Rule C is the manifest-view check inside Rule B). |
| OI-40 | guard Rule A design | `V/🧪️tests/🐍️staffage_only.py` | Name-based rule cannot see primitive helpers with ordinary names. | hz (§4), tc | Open. | done (g6: AST pass over every function in a clip script, primitive call plus no staffage call). Residual: still filtered by an object-noun name regex. |
| OI-41 | thermometers | about 25 call sites, e.g. `❄️cooling/…/⚖️heating-comparison/🐍️.py` | `thermometer_glyph` used as a diagram instrument. Borderline for the thermal rule. | hz (P7), cs | Judgement, open. | open (judgement, decide once). De facto glyph: g4 consolidated the physics and practice thermometers onto V `thermometer_glyph` and rd1 and rp1 allowlisted them as diagram instruments. No ruling recorded. |
| OI-42 | abstract energy gauges | `E/🧲️physics/…` (`_energy_tank`), `E/📊️demand/…/⚡️demand-power/🐍️.py` (`_wide_tank`) | Kept as abstract gauges per coordinator ruling. | hz (P8, P9) | Accepted. | ruled no-change (coordinator ruling, hz). |
| OI-43 | kettle clip length | `S/🔌️appliances/electric-kettle/🔣️.json` (clip `boil`) | Clip is 200 s, not the 180 s in the original notes. | kp | Accepted, note only. | ruled no-change (note only). ph later retimed the boil to the solver result (184 s). |
| OI-44 | laptop cast pale | `E/🏗️…/internal-gains` Beat3 and Beat4 desk (rc) | Open laptop pose reads pale, low contrast. Physically right. | rc | Judgement, open. | done (ts: 25 K span makes the 35 C laptop read clearly warmer). |
| OI-45 | cooler condensate pipe | cooling `heating-comparison` cooler | Condensate pipe dips about 0.1 below slab line. | rc | Open. | done (ts: cooler mounted on a wall; open vane ends above the floor). |
| OI-46 | Nakagin podium furnishing | `🏢️nakagin/🐍️.py:1388-1445, 2440-2590`, `🎨️visuals/🐍️.py:1675` | Furnishing built as 3D boxes inside the building model. Kept for occlusion and tests. | rx (N1) | Decision, open. | open (owner decision: keep or drop from tower views). |
| OI-47 | cast geometry, plan occupants | `📊️excel/🎬️clip/📋️program/🐍️.py` | Plan occupants use one relaxed pose; bed and seated plan postures not shown. | rx | Judgement, open. | open (judgement, rx). Overlap fixed by E1 (`occupant_spots`); pose still one. |
| OI-48 | evaporative humidifier category | `S/🔌️appliances/evaporative-humidifier/` | ha2 asked for a move to `🏗️machinery` with plate HX and sorption wheel. | ha2 (item 1) | Partly resolved. Open for humidifier. | open (no owner named). mv moved plate HX and sorption wheel only. Spot check: humidifier still in `🔌️appliances`. |

## 3. Visual or layout

| id | area | file:line | what is open | raised by | later resolution | status (ol) |
|---|---|---|---|---|---|---|
| OI-50 | appliances previews | `S/🔌️appliances/electric-fan-heater/🔣️.json`, `electric-kettle`, `evaporative-humidifier`, `gas-hob` | Four touched assets were never rendered after pen changes. | ha1 | Open. | done (pv: 26 assets rendered dark-line and thermal at 1:50 and 1:100, including these four). |
| OI-51 | pens at 1:50 | `S/🔌️appliances/toaster/🔣️.json`, `vacuum-cleaner/🔣️.json` | Body pen 0.35 mm at 1:50, double the 0.18 mm standard. | ha2 (item 4) | Open. | done (pv: body 0.18 at 1:50 and 0.18 at 1:100, family values). |
| OI-52 | shower hose | `S/🔌️appliances/shower-head/🔣️.json` | Hose thicker than real width at small scales. | ha2 (item 4) | Open. | done (pv: 1:100 a 0.13 mm line; 1:200 kept as 0.1 mm because dropping the hose broke the extent check; stated in `thinElements`). |
| OI-53 | pen family outliers | `S/🔌️appliances/decentralised-hrv-fan`, `desktop-monitor`, `kitchen-extractor-hood`, `circulation-pump`, `gas-boiler`, `hot-water-cylinder` | Within range, not changed. | ha1 | Accepted, note. | ruled no-change for the rest (ha1). pv snapped the 1:200 ground pens of circulation-pump, gas-boiler and hot-water-cylinder to 0.25. |
| OI-54 | pens off ISO steps | `S/👤️humans/…`, `S/🏗️machinery/oil-pumpjack/🔣️.json` | Pens 0.10, 0.15, 0.20, 0.22 are off ISO 128-20 steps. | ha3 (item 7) | Judgement, open. | ruled no-change (pv surveyed 144 assets; the archive references use these values; only the pumpjack pen snapped to 0.25). Pen-series policy is an owner decision. |
| OI-55 | vehicle interiors | `S/🚜️vehicles/vintage-car-1929/🔣️.json`, `city-car/🔣️.json` | Interior passive region needed. | ha3 (item 1) | Open. | done (cp: new passive `interior` region; 0 unbound shapes in `cp/unbound.py`). |
| OI-56 | 1:500 part counts | `S/🏗️machinery/oil-pumpjack`, `S/👤️humans/seated-office-worker`, `person-cooking`, `standing-adult`, `sleeping-person` | 12 to 18 shapes at 1:500 vs rule of 1 to 4 closed paths. | ha3 (item 6) | Open. | ruled no-change (pv: 12 to 18 is the norm of the references; shapes are bone-bound; each reads as one silhouette at 1:500). Owner decision only if a rig change is wanted. |
| OI-57 | dimension text | `S/🛋️furniture/operating-table/🔣️.json`, `S/🧱️components/tilt-window/🔣️.json`, `S/🪑️objects/tealight/🔣️.json` | Summary text vs numbers. | ha3 (item 8) | Open. | done (pv: operating-table `depthM` 0.5, tilt-window and tealight summaries state what the numbers include). |
| OI-58 | widthM naming | `S/🔌️appliances/laptop/🔣️.json`, `split-ac-indoor-unit/🔣️.json` | `widthM` is side-elevation extent, `depthM` lateral width. | ha2 (item 2) | Decision, open. | ruled no-change (pv: keep the values, the checker defines `widthM` as the default-elevation extent; the definition is now written into the schema). |
| OI-59 | toaster and hob plan views | `S/🔌️appliances/gas-hob/🔣️.json` | Hob read from top. Plan at 1:50 and 1:100 next step. | ha1 | Judgement, open. | done (pv: plan view at 1:50 and 1:100, all five regions drawn in plan). |
| OI-60 | MVHR, pump, boiler loops | `S/🔌️appliances/circulation-pump`, `gas-boiler`, `duct-fan`, `hob` | Loop clips exist only for the pump. Hob `flicker` and humidifier `drip` are candidates. | ha1 | Judgement, open. | done (pv: gas-hob `flicker` loop, gas-boiler `steady-burn` loop; pa: mvhr `normal-spin`; sm2: humidifier `drip`). |
| OI-61 | thermal sheet check | `S/👤️humans/…` and `S/🔌️appliances/…` (cs sheets) | Render sheets of 1:100 spot checked only. | cs | Accepted. | ruled no-change (cs accepted the spot check). |
| OI-62 | rd2 system losses | `E/📊️demand/🎬️clip/♨️system-losses/🐍️.py` Beat3 | Boiler and heat pump icons have no floor line. | rd2 | Judgement, open. | done (sl: floor line under boiler and pump, faded in with the stage tag). |
| OI-63 | rd2 readouts | demand `system-losses` Beat3, `🪟️…/🔁️recap` | Transient readout glitches (`Wärme20kWh`, thin space in `22Monate`). | rd2 | Open, out of staffage scope. | done (sl: `_readout` uses `become`, so stale copies no longer pile up; `22Monate` uses `\;`). |
| OI-64 | rh thermostat label | `E/🔥️heating/🎬️clip/🧮️calculation/🐍️.py` AnlagenVerluste | Thermostat label tight against radiator plate. Transient crossing text in ReviewingHeatGains. | rh | Open. | done (rh2: label offset verified; frozen Q copies now pulse in place and fade). |
| OI-65 | rd1 small devices | practice Beat3 and primary-energy Beat3 `_boiler` and heat pump | Half a person height, near readable lower limit at 480p. | rd1 | Judgement, open. | ruled no-change (rd1: small by design, readable at 480p). |
| OI-66 | rp1 shower icon | `E/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:710` | Shower icon narrow (tall asset). | rp1 | Judgement, open. | done (sm2: `window` support in `staffage_motion` and `fitted_staffage`; icon enlarged). Spray not visible at card scale (open note in sm2). |
| OI-67 | rv volume Beat1 | `E/🌬️ventilation/🎬️clip/📦️volume/🐍️.py` Beat1 "10m" dimension | Dimension under formula box. Not staffage. | rv | Open, out of staffage scope. | open (out of scope, no owner). |
| OI-68 | rh conduction Beat4 | `E/🔥️heating/🎬️clip/🧱️conduction/🐍️.py` Beat4 | Toaster (0.74) and person (1.3) icon scales are deliberate comparison icons. | rh | Accepted. | done as superseded (rh2: one shared metre, toaster 0.24 m, tealight 0.046 m on one ground line, with magnifier insets). |
| OI-69 | hs dead imports | about 25 clip headers (ruff F401), about 25 clips with unused locals (F841) | Not in rule list. | hs | Open (infra). | done (hc: F401 and F841 fixed in the 50 touched files, ruff clean). Residual: clips outside the ticket list were not scanned. |
| OI-70 | hs `_blind_hit` and `_first_slat_hit` | `E/⚡️…/☀️solar-gains/🐍️.py`, physics fundamentals | Share ray and lamella maths but differ in semantics. Not merged. | hs | Judgement, open. | ruled no-change (hs: semantics differ, positive-along requirement and fallback; not merged). |
| OI-71 | hs CRLF line endings | `solar-radiation`, `fundamentals (heating)` and others | Working copy CRLF, HEAD LF. | hs | Open (infra). | done (hc: six files rewritten to HEAD style; `script.ts` kept CRLF as HEAD; other 93 already matched). |
| OI-72 | German wording | `S/🚜️vehicles/city-bus/🔣️.json` poses[2].title "Kneeling mit offenen Türen" | "Kneeling" accepted German transit term. | hl2 | Judgement, open. | ruled no-change (hl2 accepted). A native reviewer may prefer "Absenkung" (see OI-73). |
| OI-73 | German review method | all `de` strings (catalog) | All German review done by reading, no native check or grammar tool. | hl, hl2 | Open (method). | open (method; needs a native German reviewer). hl2 scanned 17564 pairs with a heuristic. |
| OI-74 | Gas meter icons and glyph scope | summary "Diagram icons stay glyphs: sun, flame, arrows, meters, clock, thermometer" | Meters and thermometers classed as glyphs, but gas meter is a box. Contradiction. | summary, hz | Open (decision). | open (coordinator wording). g4 made gas and electricity meters device casts (`gas-meter`, `electricity-meter`), so the glyph wording in the summary and contract is out of date. |

## 4. Tests not run

| id | area | file:line | what is open | raised by | later resolution | status (ol) |
|---|---|---|---|---|---|---|
| OI-80 | BIM info and consistency suites | `🏗️bim/🎬️clip/🧪️tests/🐍️.py` | Never ran (see OI-01, OI-02). | rb | Open. | open (blocked by OI-01). |
| OI-81 | BIM clip contract | `🏗️bim/🎬️clip/🧪️tests/🐍️.py` `BimClipContract` (cheap tests) | Timed out at 560 s in the bilingual caption fit test. | rb | Open. | open (rb: not finished; no later report). |
| OI-82 | optimization full suite | `🎯️optimization/🎬️clip/🧪️tests/🐍️.py:1184` | Only `SchemaFirstStudy` re-run after the $schema fix. Full 75-test suite not re-run. | ro | Fix verified (see spot check S13). Full re-run open. | open (full re-run not reported). |
| OI-83 | POSIX cancellation | cast export `🟦️.ts` (SIGTERM case) | Bun half on Linux and macOS not run. Python half verified in WSL (7 of 7). | cx, px | Open. | open (infra; no POSIX host with bun). |
| OI-84 | devcontainer | repo-wide | Devcontainer not run. Docker daemon stopped. | px | Open (infra). | open (infra; Docker stopped). |
| OI-85 | excel lifecycle | `📊️excel/🎬️clip/🧪️tests/🎭️presentation` (`test_native_lifecycle`) | Ran 25 min at 3.4 GB and was killed. | rx | Open. | open (rx: unverified). |
| OI-86 | kettle and humidifier clips | `S/🔌️appliances/electric-kettle`, `evaporative-humidifier` | Python unittest for clip modules not run after kp, ph, sm2 and cp2 edits. | kp | Open (partly resolved). | open (no clip unittest run reported for the kettle or humidifier after these edits). |
| OI-87 | asset previews | `S/` touched assets in ha1, ha2, ha3 | Rendered previews not reviewed. | ha1, ha2, ha3 | Open. | open (partly done). pv rendered 26 assets. Not reported rendered: ha1 decentralised-hrv-fan, desktop-monitor, duct-fan, kitchen-extractor-hood; ha2 server-rack; ha3 expansion-valve, panel-radiator, sleeping-person, person-cooking, seated-office-worker, chalkboard, storage-crate-stack. |
| OI-88 | 1:100 cast sheets | `S/` cs sheets | 1:100 sheets generated, spot checked only. | cs | Accepted. | ruled no-change (accepted). |
| OI-89 | Tests for slow modules | `ht` sweep | Integration, browser, interaction, chapters ran before later edits. | ht | Open (rerun when fixes land). | open (no owner; slow modules not rerun in one pass). |
| OI-90 | sweep module list | `🧰️ht/modules.txt` (modules 27, 33, 34, 35) | Four listed modules have no test cases (exit 5). | ht | Open (coordinator). | open (coordinator: drop them from the list). |
| OI-91 | staffage vitest counts | staffage library and React vitest | Staffage 1018 pass (ht), 1029 (ht) after q2 config. React 1 failure (OI-07). | ht | Open for React. | partly done. React 107 pass (fx2, rx2, vf3). Library: vf3 saw 2 failing before the ts fix; ts reports thermal-span and cast-export pass. No single final library run. |
| OI-92 | 600-frame pose test | `🧪️tests/🐍️` (V staffage tests) | Solves 600 thermal frames, takes seconds to a minute. | tc | Accepted. | ruled no-change (accepted). |
| OI-93 | Python `.feature` coverage | staffage bridge | Python side has no `.feature`, cases are JSON. | tc | Judgement, open. | open (judgement, no owner). |

## 5. Infra or out of scope

| id | area | file:line | what is open | raised by | later resolution | status (ol) |
|---|---|---|---|---|---|---|
| OI-100 | repo dependencies | `repo dependencies ensure` | Refuses with "immutable input capture work refused". | summary, q2, fleet brief | Open (infra, do not fix in ticket). | open (infra). dl used `repo dependencies lock` successfully to refresh `uv.lock`. |
| OI-101 | ffmpeg | repo tooling | repo ffmpeg broken. PyAV used for frames, PyMuPDF for SVG. | fleet brief, rv, rh | Workaround in place. | open (infra; workaround in place). |
| OI-102 | cache | `.🧬semio/🎓️teaching/⚡️cache/staffage/` | 3 incomplete key folders, 10 manifest casts without folder. | hp | Open (self-healing). | open (self-healing; `ensure_casts` recreates). hp cleaned 367 stale key folders. |
| OI-103 | cancel on Windows | `🧍️staffage/🐍️.py` `_Tree` | Graceful CTRL_BREAK reaches only processes sharing console. | cx | Accepted. | ruled no-change (accepted; job object kills the rest). |
| OI-104 | POSIX orphan | `🧍️staffage/🐍️.py` `_Tree` | If Python is hard-killed on POSIX the exporter survives until the watchdog. | px | Closed by watchdog (verified on Linux by px for the Python half). | done (px: `--parent` watchdog, 500 ms; Python half verified on Linux 7 of 7; Bun half is OI-83). |
| OI-105 | tc psutil | staffage tests | psutil not a dev dependency. Tests use tasklist and ps. | tc | Accepted. | ruled no-change (accepted). |
| OI-106 | deleted test file | `E/🏃️activity/…/🧪️tests/🐍️.py` (internal-gains activity) | Listed by git as modified but deleted on disk. | hs | Open (infra, git state). | open (infra, git state; not changed here). hc also reports `🏗️authoring/🎨️visuals/🧪️tests/🐍️silhouettes.py` missing on disk. |
| OI-107 | ticket-local runners | `🧰️…` scripts | Scripts are ticket-local. | fleet brief, q2 | Accepted for this ticket. | ruled no-change (accepted for this ticket). |
| OI-108 | cast-export scripted writes | `E/🔥️heating/…/☀️solar-gains/🐍️.py` | EINVAL and PermissionError incident truncated a file. | hs | Resolved (fixed). | done (hs: Edit fixes; scripted writes use temp file and `os.replace`). |
| OI-109 | summary "all staffage" claim | `summary.md` Outcome section | Says every device and machine is now a cast. | summary | Open (wording). | open (coordinator wording). OI-03 and OI-20 are done; OI-04 (practice store) is still abstract, so the "all" claim stays wrong until it is decided. |

## 6. Spot checks of claimed fixes (grep and reads)

Checked 17 claims by grep and reads in round 1 (oi). Result: 16 confirmed in code. 1 claim (S17, guard allowlist entries) is not present in the code (discrepancy). OI-12 is a claim superseded by a later edit and is not in this table.

| # | claim (report) | where checked | verdict |
|---|---|---|---|
| S1 | walking-commuter `title.de` and `description.en` swapped back (hl) | `S/👤️humans/walking-commuter/🔣️.json:3966-3969` | Confirmed. |
| S2 | optimization schema test renamed to draft 2020-12 (ro) | `🎯️optimization/🎬️clip/🧪️tests/🐍️.py:1187-1189` | Confirmed. |
| S3 | plate HX plates and ports passive; supply and exhaust emitters; collars (ha2) | `S/🏗️machinery/plate-heat-exchanger/🔣️.json` regions `plates` and `ports` `model: passive` (:1192, :1225), anchors `supply-collar` (:200) and `exhaust-collar` (:211) | Confirmed. Note: plate HX is now in `🏗️machinery` (ha2 said it was not moved). |
| S4 | sorption-wheel `start-and-run` end angle 720 to 408 (ha2) | `S/🏗️machinery/sorption-wheel/🔣️.json:1128` | Confirmed. Superseded by mv's physical rewrite. |
| S5 | electric kettle water region heats directly, `heatW` 1930 (kp) | `S/🔌️appliances/electric-kettle/🔣️.json:1442` | Confirmed. |
| S6 | electric kettle `window` region renamed `water` (ha1) | same file | Confirmed. |
| S7 | humidifier `vapourKgPerKg` 0.004 to 0.0106 (kp) | `S/🔌️appliances/evaporative-humidifier/🔣️.json:1273` | Confirmed. |
| S8 | pendant-lamp shade `areaM2` 0.24 to 0.11 (ha2) | `S/🔌️appliances/pendant-lamp/🔣️.json:1257` | Confirmed. |
| S9 | tilt-window sash mass 33.0 kg (ha3) | `S/🧱️components/tilt-window/🔣️.json:90` | Confirmed. |
| S10 | cast exporter SIGBREAK, `started` callback, `[cancelled]` with exit 130, `--parent` watchdog (cx, px) | `S/🔨️modules/📤️export/🎭️cast/🟦️.ts` | Confirmed. POSIX branch not run (OI-83). |
| S11 | staffage `_Tree` job object, CTRL_BREAK, killpg, `ensure_casts` passes `--parent` (cx, px) | `V/🧍️staffage/🐍️.py` | Confirmed. |
| S12 | `bim-exchange-imposed-person` cast added to manifest (rb) | `V/🧍️staffage/🔣️.json` | Confirmed. |
| S13 | `APPLIANCE_Y` used for boiler and tank on the floor (rd2) | `E/📊️demand/🎬️clip/♨️system-losses/🐍️.py` | Confirmed. |
| S14 | practice radiator viewpoint front added (rd1) | `V/🧍️staffage/🔣️.json` | Confirmed. |
| S15 | dead helpers removed (`_shade_factor`, `_bell`, `_droplets`), shared `lamella_faces` and `tracked_clock` (hs) | `E/` grep and `V/🐍️.py` | Confirmed. |
| S16 | physics `shower_icon` added to Beat1 (rp1) | `E/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | Confirmed. |
| S17 | guard allowlist for `_meter`, `_meter_glyph`, `_gas_meter`, `_thermo`, `_thermometer` (rd1, rd2, rp1) | `V/🧪️tests/🔣️staffage_only.json` `rule_a.allowed` | Not present. Removed by the coordinator (g6). Discrepancy recorded. Current state: `_gas_meter` is a cast wrapper (OI-03). |

Not checked here (round 1): the guard run, the React catalog test and the integration test. Round 2 status for those is in OI-06, OI-07 and OI-09.

## 7. New open items raised by the R2 reports (added by ol)

| id | area | file:line | what is open | raised by | status (ol) |
|---|---|---|---|---|---|
| OI-110 | speech for changed clauses | cooling `internal-gains` Beat1 "interior" (clause_02); heating `internal-gains` Beat2 "power" (clause_01); both en and de | Not re-synthesized. No TTS key set (`TTS_API_KEY`, `OPENROUTER_API_KEY`, no `teaching/.env`). No narration audio exists for these two beats. | tt, nv | open (owner: user, provide TTS key; then OI-111). |
| OI-111 | speech tooling | `🔊️speech/🐍️.py` (`subtitle_clauses`, `synthesize_clause_audio`); per-clip speech modules | German only. Per-clip modules write without the `<language>` segment that the runtime reads. `cmd_synth --beat` regenerates every clause. Heating speech module is legacy (one whole-beat file). | tt | open (speech tooling owner). |
| OI-112 | sorption narration and view | cooling `summer-strategy` Beat2 narration; sorption wheel asset | Narration says "Solarwärme regeneriert das Rad" with a sun glyph, but the heating coil is the heat source. The face-on view cannot show process and regeneration plumes. | mv | open (scene author). |
| OI-113 | toaster casing | `S/🔌️appliances/toaster/🔣️.json` | Casing 180 W on 0.24 m2 (about +75 K), not touched. | pa | open (judgement). |
| OI-114 | pumpjack skid mass | `S/🏗️machinery/oil-pumpjack/🔣️.json` | Motor bone 1100 kg for a 30 kW motor (includes skid). | pa | open (judgement). |
| OI-115 | renders after changes | natural, mechanical and volume scenes with `shared-person-pathlines`; mechanical occupant, volume glyphs, vacuum, natural radiators; other thermal scenes | Not re-rendered after bp, xc and ts edits (ambient change only tints). | bp, xc, ts | open (render check, no owner). |
| OI-116 | laptop vent jet | cooling or heating laptop cast | Vent jet reaches about 0.5 m along the desk (physical 1.1 L/s, +10 K). | bp | open (judgement). |
| OI-117 | physics clerk scale | `E/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` near line 984 | Clerk at 0.72 of house scale, reads as distance by design. | xc | open (judgement, left as is). |
| OI-118 | BIM information car | `🏗️bim/🎬️clip/📊️information/🐍️.py` | Car placement inside the full scene never viewed (bay scale only). | rb | open (BIM owner). |
| OI-119 | phase-change vitest | `S/🧪️tests/🫕️phase-change/🟦️.ts` | mv reported kettle plateau balance 1.7e-5 vs 1.9e-6. No later report confirms a pass. | mv | open (verify). |
| OI-120 | phase change scope | `S/🔨️modules/🌡️thermal/…`, transport sources | Transport thermal switch keyed on the evaporation vent. No depletion or freezing direction. | ph | open (judgement). |
| OI-121 | invisible motion | `E/🌬️ventilation/🎬️clip/❓️need/🐍️.py` `_room_occupants` | Builds a prototype `_stand` motion only to read height. Not shown. | hp | open (minor; use `staffage_height`). |
| OI-122 | hob heat source | `S/🧍️…/person-cooking` hob pose | Pot is `operating` with key `hob`, so the pot is the source. Plausible. | ha3 | open (judgement). |
| OI-123 | visual weak spots | open laptop 1:50 (V-shaped), chalkboard plain frame, gas-hob thin slab | Left as authored. | cs | open (judgement). |
| OI-124 | undeclared imports | `anastruct`, `pandas`, `pyet`, `psychrolib`, `ezdxf`, `reportlab`, `pythermalcomfort`, `pathops`, `PIL` in test and oracle files | Not declared in the teaching `test` group. Declare or drop. | dp, dl | open (teaching python owner). |
| OI-125 | oracle docstrings | `🫕️phase-change` and other oracles | Some still say `uv run --with ...`; should read `--project <python pkg> --group test`. | dl | open (minor). |
| OI-126 | ticket data | phase-change tests and fixtures, `🧍️staffage/🧪️tests`, guard vectors | Untracked in git. Not committed (correct per rules). | dp, dl | open (commit only when the owner asks). |
| OI-127 | report drift | `r2-g5-report.md` (removed cast ids), `r2-g6-report.md` (Rule A 15 vs 14, Rule B 12 vs 8) | Reports cite old ids and counts. The code is right. | dc | open (coordinator; reports only). |
| OI-128 | span in viewer | React viewer range control | Does not offer the `span` mode. | ts | open (minor). |
| OI-129 | search facets | React search | No material or function facet. | rx2 | open (judgement; feature). |
| OI-130 | React check | React overview en and de; `tsc --noEmit` | Visual check and dev preview not done. 12 tsc errors in `🧰️framework` (pre-existing, not staffage). | rx2 | open (not staffage scope, except the visual check). |
| OI-131 | heading renders | excel `Beat4_Energy`; grasshopper `Beat4_Casting` | Not rendered, verified by test only. | fx2 | open (render when host is idle). |
| OI-132 | heating Beat4 intro | `E/🔥️heating/🎬️clip/🧱️conduction/🐍️.py` Beat4 intro | Lower half empty; a height dimension was suggested. | rh2 | open (judgement, minor). |
| OI-133 | hob plan labels | hob plan viewpoint | Dimension labels show the height (0.09 m), an annotation quirk. | pv | open (engine quirk). |
| OI-134 | HRV reference regime | `S/🔌️appliances/decentralised-hrv-fan` emitters | Supply -5 K, exhaust -21 K; reference air (room or outdoor) not stated. | ha1 | open (judgement). |
| OI-135 | process kill | shell of rh2 | `taskkill //IM python.exe` may have stopped a concurrent agent's python process. No loss reported. | rh2 | open (check that no render was lost). |

## 8. Checked as resolved in this pass (ol spot checks)

- `--lang` (dc, bp): not present in the staffage Python caller `V/🧍️staffage/🐍️.py`. Resolved.
- `PF_NO_SESSION` debug hook (dc): not found under `🧍️staffage`. Resolved.
- `_Path` NameError in `practice` (g4, mv): the file now does `from pathlib import Path`. Resolved.
- `_plant_panel` stale allowlist entry (OI-05): not in `staffage_only.json`. Resolved.
- `silhouettes.py` and the internal-gains activity test: missing on disk (OI-106). Left open for git state.
- `energy_tank` in practice (OI-04): still present, deliberately kept by g4. Open.
- Humidifier category (OI-48): still in `🔌️appliances`. Open.

## Summary of counts

- Section 1 to 5, tracked items OI-01 to OI-109 (90 items): done 42, ruled no-change 17, open 31.
- Section 7, new items OI-110 to OI-135 (26 items): all open.
- Total open: 57.

## Still open

- Model owner: OI-01, OI-02, OI-80, OI-81 (BIM reserve and setup time), OI-118.
- Coordinator: OI-04 (practice store), OI-74 and OI-127 (wording), OI-90 (module list), OI-109 (summary wording), OI-82 and OI-89 (full re-runs).
- Owner decision: OI-46 (Nakagin furnishing), OI-34 (wall in panel radiator, workaround only), OI-21 (cylinder product height), OI-54 (pen-series policy, owner only if wanted).
- ph or coordinator: OI-23 (kettle dry mass).
- Judgement, no owner: OI-11, OI-27, OI-41, OI-47, OI-48, OI-67, OI-93, OI-113 to OI-117, OI-120, OI-122, OI-123, OI-132, OI-134.
- Verification gaps: OI-83, OI-84, OI-85, OI-86, OI-87, OI-91, OI-119, OI-131.
- Speech and narration: OI-110, OI-111, OI-112.
- Infra and git: OI-100, OI-101, OI-102, OI-106, OI-124 to OI-126, OI-130, OI-135.
- Performance: OI-10 (cast peak memory).
- Language review: OI-73 (native German reviewer).
- Minor or cosmetic: OI-121, OI-125, OI-128, OI-129, OI-133.
