# 📓️ Content Report — First Quizzes of quizze.architektur-und-technologie.de

Work package: the four energy quizzes, the `architecture` catalog and the two teaching READMEs. Inputs: `📓️design.md`
(§1 tree, §2 ids, §4 draw, §6 scoring, §7 badges), the contract `🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json` and
`📓️research-quiz-content-data.md` (every "Näherung"/"didaktisch" value re-checked; see §3).

## 1. Files

| File | Content |
|---|---|
| `C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🧲️physics\❓️quiz\🔣️.json` | quiz `physics`: classification power/energy (16 items, draw 12), sorting powers (14, draw 10, W), sorting energies (12, draw 9, Wh) |
| `C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🔥️heating\❓️quiz\🔣️.json` | quiz `heating`: matching U-values (17, draw 10), matching heating load + heating demand (11, draw 8, two dimensions) |
| `C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\❄️cooling\❓️quiz\🔣️.json` | quiz `cooling`: matching air change rates (15, draw 10), matching cooling load + cooling demand (9, draw 7, two dimensions) |
| `C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\📊️demand\❓️quiz\🔣️.json` | quiz `demand`: classification standards → spider profiles (6 onto 6, no draw, 4 axes), matching final energy (9, draw 7) |
| `C:\git\semio\🎓️teaching\🏛️architecture\❓️quiz\🔣️.json` | catalog `architecture`: 5 intro paragraphs (en/de), 4 quiz paths, 7 badges |
| `C:\git\semio\🎓️teaching\README.md` | the teaching area: proctor, site, topic tree with quiz and clip leaves, how to add a quiz |
| `C:\git\semio\🎓️teaching\🏛️architecture\README.md` | the site: parts, quizzes, badges, sources per quiz, conventions for derived values |
| ticket `validate_quiz_content.py` | jsonschema 4.26 (draft-07 + `referencing`) and the semantic rules of design §2/§4/§6/§7 |
| ticket `check_quiz_core.ts` | cross-check through the owned TS core: `quizIssues`, `catalogIssues`, `sheetOf`, `scoreRun` over 200 seeded runs per quiz |

Every quiz: `schema: "semio.quiz/v1"`, `$schema` = `../../../../../🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json#/$defs/Quiz`
(repo convention with `#/$defs/…`); catalog `$schema` = `../../../🧰️framework/…/🔣️.json#/$defs/Catalog`. All ids are
English kebab-case; every learner-visible text has `en` and `de`; every item has a 1–2 sentence explanation with value,
reasoning and source. Every path segment carries U+FE0F after its emoji (checked by the script).

## 2. Items, values and sources

### 2.1 `physics`

**power-or-energy** (classification; categories `power` "Power"/"Leistung", `energy` "Energy (demand)"/"Energie(bedarf)")

| Item | Category | Value in explanation | Source |
|---|---|---|---|
| tea-light | power | 35 W | paraffin ≈ 12 Wh/g, 12 g in 4 h; Wikipedia „Teelicht“ 30–40 W |
| kettle | power | 2 kW | rating plate; 230 V/16 A ≤ 3.7 kW |
| resting-person | power | 100 W | ISO 7730/8996: 58 W/m² × 1.8 m² |
| pv-module-peak | power | 0.43 kWp | IEC 60904-3 STC |
| heating-load | power | 18 kW | 110 m² × 160 W/m² (heating quiz, IWU EFH_E) |
| nuclear-unit | power | 1.41 GW | Isar 2 net |
| car-engine | power | 100 kW = 136 PS | 1 PS = 735.5 W |
| wallbox | power | 11 kW | 3 × 230 V × 16 A |
| household-electricity | energy | 2,500 kWh/a | Stromspiegel |
| ev-battery | energy | 60 kWh | VW ID.3 58 kWh net |
| heating-oil-litre | energy | 10 kWh | DIN 51603-1: 42.6 MJ/kg × 0.845 kg/l |
| heating-demand-house | energy | 33,000 kWh/a | 110 m² × 303 kWh/(m²·a), IWU EFH_E |
| chocolate-bar | energy | 540 kcal ≈ 0.63 kWh | nutrition label |
| pv-annual-yield | energy | 9,500 kWh/a | Fraunhofer ISE ≈ 950 kWh/kWp |
| phone-charge | energy | 15 Wh | 4,000 mAh × 3.85 V |
| energy-certificate | energy | kWh/(m²·a) | GEG § 85 |

**powers** (sorting, W, logarithmic, prefixed; 25.0 decades)

| Item | Value [W] | Source |
|---|---|---|
| tea-light | 35 | Wikipedia „Teelicht“ |
| resting-person | 100 | ISO 7730 |
| sunlight-square-metre | 1,000 | IEC 60904-3 |
| kettle | 2,000 | rating plate |
| wallbox | 11,000 | 3 × 230 V × 16 A |
| heating-load-old-house | 18,000 | IWU EFH_E, DIN EN 12831 |
| car-engine | 100,000 | 136 PS |
| wind-turbine | 5 × 10⁶ | Deutsche WindGuard 2025: 635 turbines, 3,251 MW → 5.1 MW |
| ice-train | 8 × 10⁶ | DB class 403 |
| nuclear-unit | 1.41 × 10⁹ | Isar 2 net |
| germany-electricity | 6.0 × 10¹⁰ | AGEB 2025: 527.4 TWh ÷ 8,784 h |
| world-primary-power | 1.87 × 10¹³ | EI Statistical Review 2025: 592 EJ ÷ 31.6 Ms |
| sunlight-on-earth | 1.74 × 10¹⁷ | 1,361 W/m² × π (6,371 km)² |
| sun | 3.828 × 10²⁶ | IAU 2015 B3 |

**energies** (sorting, Wh, logarithmic, prefixed; 16.0 decades)

| Item | Value [Wh] | Source |
|---|---|---|
| phone-charge | 15 | 4,000 mAh × 3.85 V |
| boil-water | 93 | 4.19 kJ/(kg·K) × 80 K |
| chocolate-bar | 628 | 540 kcal × 1.163 Wh/kcal |
| daily-food | 2,700 | DGE ≈ 2,300 kcal |
| heating-oil-litre | 10,000 | DIN 51603-1 |
| ev-battery | 60,000 | VW ID.3 |
| petrol-tank | 440,000 | 50 l × 8.8 kWh/l |
| household-electricity | 2.5 × 10⁶ | Stromspiegel |
| heating-demand-house | 3.3 × 10⁷ | IWU EFH_E |
| wind-turbine-year | 1.0 × 10¹⁰ | 5 MW × 2,000 h |
| germany-primary-energy | 2.925 × 10¹⁵ | AGEB 2025: 10,529 PJ |
| world-primary-energy | 1.644 × 10¹⁷ | EI 2025: 592 EJ |

### 2.2 `heating`

**u-values** (matching, W/(m²·K), logarithmic, not prefixed)

| Item | U | Source |
|---|---|---|
| single-glazing | 5.8 (Ug) | BAnz AT 04.12.2020 B1, Tab. 3 |
| aluminium-window-1970s | 4.3 (Uw) | BAnz Tab. 3 (Alu/steel, insulating glass, ≤ 1983) |
| roller-shutter-box | 3.6 | BAnz Tab. 2 (uninsulated, ≤ 1994) |
| box-type-window | 2.7 (Uw) | BAnz Tab. 3 (wood, two panes, ≤ 1994) |
| solid-roof-1950s | 2.1 | BAnz Tab. 2 (roof solid, ≤ 1957) |
| front-door-geg | 1.8 | GEG 2024 Anlage 1 |
| half-timbered-wall | 1.5 | BAnz Tab. 2 (clay infill ≤ 25 cm, ≤ 1957) |
| window-geg | 1.3 (Uw) | GEG 2024 Anlage 1 |
| hollow-brick-wall-1970s | 1.0 | BAnz Tab. 2 (perforated brick/hollow block, 1969–1978) |
| window-passive-house | 0.8 (Uw) | PHI component criterion |
| masonry-wall-1980s | 0.6 | BAnz Tab. 2 (1984–1994) |
| triple-glazing | 0.5 (Ug) | EN 673, two low-e + gas |
| floor-geg | 0.35 | GEG 2024 Anlage 1 |
| wall-geg | 0.28 | GEG 2024 Anlage 1 (matches repo `geg_anlage2_reference_u::WALL`) |
| roof-geg | 0.20 | GEG 2024 Anlage 1 |
| wall-passive-house | 0.15 | PHI guide value opaque components |
| roof-passive-house | 0.10 | ≈ 35 cm insulation, λ 0.035 |

**heating-load-and-demand** (matching; `heating-load` W/m², `heating-demand` kWh/(m²·a); both logarithmic)

| Item | Load | Demand | Source / derivation |
|---|---|---|---|
| passive-house | 10 | 15 | PHI criteria |
| gruenderzeit-retrofit | 16 | 20 | IWU MFH_B package 2 (H_T/A 0.39 + MVHR 0.10) × 32 K; demand 20.2 |
| kfw-40 | 20 | 26 | IWU Tab. 26: 21.7 per A_N ≈ 26 per m² living area; H′T ≤ 55 % ref |
| kfw-55 | 25 | 35 | IWU Tab. 26: 29.9 per A_N ≈ 35; H′T ≤ 70 % ref |
| geg-2024 (GEG 2024, minimum envelope, heat pump) | 40 | 50 | § 16 GEG: H′T ≤ 1.0 × reference; IWU Tab. 26: 43.7 per A_N ≈ 50; design practice 30–50 W/m² |
| sfh-2000s | 50 | 89 | IWU EFH_J (H_T/A 1.14, ventilation 0.5 1/h) |
| plattenbau | 60 | 122 | IWU NBL_GMH_F (H_T/A 1.37) |
| sfh-1990s | 73 | 135 | IWU EFH_I (H_T/A 1.78) |
| gruenderzeit | 95 | 197 | IWU MFH_B (H_T/A 2.44) |
| sfh-1970s | 115 | 224 | IWU EFH_F (H_T/A 3.13) |
| sfh-1960s | 160 | 303 | IWU EFH_E (H_T/A 4.50) |

Loads of existing types: (H_T/A + 0.34 × 0.6 × 2.5) × 32 K. Demands of existing types: IWU 2015 appendix C.3, TABULA
standard boundary conditions per m² heated living area (calculated demand, not the consumption-calibrated values).
Load and demand orders agree for all 55 pairs.

### 2.3 `cooling`

**air-change-rates** (matching, 1/h, logarithmic)

| Item | n [1/h] | Derivation / source |
|---|---|---|
| warehouse | 0.13 | EN 16798-1 very low-polluting 0.35 l/(s·m²) over 10 m |
| passive-house-dwelling | 0.3 | PHI 30 m³/h per person |
| apartment | 0.5 | DIN 1946-6: −0.001·A² + 1.15·A + 20 = 125 m³/h for 100 m² |
| sports-hall | 0.8 | DIN 18032-1: 60 m³/h per athlete, 30 in 2,230 m³ |
| single-office | 1.5 | EN 16798-1 cat. II: 7 l/s·P + 0.7 l/(s·m²), 12 m², 3 m |
| residential-car-park | 2.4 | GaVO 6 m³/(h·m²), 2.5 m |
| cinema | 3.5 | EN 16798-1 cat. II, 1 seat/m², 8 m |
| classroom | 4.8 | EN 16798-1 cat. II, 28 pupils, 60 m², 3 m |
| restaurant | 6.4 | EN 16798-1 cat. II, 1.5 m²/guest, 3 m |
| laboratory | 8.3 | DIN 1946-7: 25 m³/(h·m²), 3 m |
| operating-room | 20 | DIN 1946-4 class Ib: 60 m³/(h·m²), 3 m |
| commercial-kitchen | 25 | VDI 2052: 12–30 1/h |
| cleanroom-iso-7 | 60 | IEST-RP-CC012 |
| cleanroom-iso-6 | 150 | IEST-RP-CC012: 90–180 |
| cleanroom-iso-5 | 540 | ISO 14644-4: 0.45 m/s × 3,600 s ÷ 3 m |

**cooling-load-and-demand** (matching; `cooling-load` W/m², `cooling-demand` kWh/(m²·a))

| Item | Load | Demand | Full-load hours / source |
|---|---|---|---|
| passive-house-home | 6 | 2 | ≈ 350; PHI overheating ≤ 10 % > 25 °C |
| new-home-geg | 15 | 5 | ≈ 350; DIN 4108-2 |
| school-new-build | 30 | 8 | ≈ 250 (summer holidays) |
| office-passive-house | 20 | 10 | ≈ 500; PHI non-residential criteria |
| office-geg-shading | 40 | 20 | ≈ 500; VDI 2078 practice |
| attic-flat | 50 | 25 | ≈ 500; VDI 2078 sol-air temperature |
| hospital | 70 | 90 | ≈ 1,300; 24 h, DIN 1946-4 |
| office-1970s | 100 | 60 | ≈ 600 |
| data-centre | 1,000 | 8,000 | ≈ 8,000 |

No normative typical-value table exists for cooling; values are consistent load × full-load-hour pairs within the ranges of
VDI 2078 practice and the research file. The two load/demand inversions (school vs passive office, hospital vs 1970s
office) are intentional and explained (holidays, 24-hour operation).

### 2.4 `demand`

**standard-profiles** (classification, axes; category labels "Profile A…F" / "Profil A…F", letters shuffled so that the
letter order does not reveal the standard)

| Axis | Unit | min | max |
|---|---|---|---|
| heating — heating demand | kWh/(m²·a) | 0 | 350 |
| cooling — cooling demand to keep 26 °C | kWh/(m²·a) | 0 | 10 |
| ventilation — ventilation heat loss (air leakage plus ventilation after heat recovery) | kWh/(m²·a) | 0 | 70 |
| costs — net energy costs for heating, hot water and auxiliary energy (after PV credit) | €/(m²·a) | −5 | 55 |

| Item (standard) | Profile | heating | cooling | ventilation | costs |
|---|---|---|---|---|---|
| unrenovated-old-building | B | 303 | 8 | 61 (n50 8) | 50 |
| wschvo-1995 | D | 135 | 6 | 43 (n50 4) | 23 |
| enev-2014 (EnEV 2014) | F | 55 | 5 | 32 (n50 1.5, no HR) | 10 |
| kfw-40 | A | 26 | 4 | 9 (n50 0.8, HR 80 %) | 4.2 |
| passive-house | E | 15 | 2 | 6.5 (n50 0.6, HR 85 %) | 3.7 |
| plus-energy-house | C | 20 | 7 | 8 (n50 0.6, HR 80 %) | −2 |

Ventilation = 0.34 × (0.07 × n50 + 0.4 × (1 − η_HR)) × 2.5 m × 75 kKh. Costs = final energy (see matching) × gas 12 ct,
heat-pump electricity 30 ct, auxiliary 35 ct; plus-energy: 12.3 kWh bought, 60 kWh PV of which 56 exported at 8 ct.
All profiles lie within [min, max]; d_max = 1.496, closest pair A/E (KfW 40 vs passive house) 0.206 → 0.86 partial credit.

**final-energy** (matching, kWh/(m²·a), logarithmic; all values > 0)

| Item | Final energy | Derivation |
|---|---|---|
| kfw-40-heat-pump | 14 | (26 + 12.5) ÷ SPF 3.5 + 3 |
| deep-retrofit-heat-pump | 23 | (54 + 12.5) ÷ 3.3 + 3 (IWU EFH_E package 2) |
| passive-house-direct-electric | 30 | 15 + 12.5 + 3 |
| kfw-55-gas-solar | 48 | 35 ÷ 0.9 + 0.4 × 12.5 ÷ 0.8 + 3 |
| enev-2014-gas | 80 | 55 ÷ 0.9 + 12.5 ÷ 0.8 + 3 |
| old-house-heat-pump | 122 | (303 + 12.5) ÷ 2.7 + 5 (Fraunhofer ISE field tests ≈ 3) |
| wschvo-1995-gas | 182 | 135 ÷ 0.85 + 12.5 ÷ 0.65 + 4 |
| gruenderzeit-gas | 274 | 197 ÷ 0.8 + 12.5 ÷ 0.5 + 3 |
| old-house-gas | 409 | 303 ÷ 0.8 + 12.5 ÷ 0.5 + 5 (> 250 = class H, GEG Anlage 10) |

Hot water 12.5 kWh/(m²·a) is the DIN V 4108-6 standard value; efficiencies follow DIN V 4701-10 practice.

### 2.5 Catalog

Introduction (title "How the quizzes work" / "So funktionieren die Quizze"), five paragraphs: what the quizzes train;
randomized runs, whole-run submission, repeatable, best run counts; partial points, magnitude-weighted pair penalties on
a logarithmic scale, partial credit for similar profiles; anonymous/pseudonym/name without password and that the same
handle continues the same record (design §8 `learner-recalled`); badges and leaderboard (sum of best scores).

Badges in evaluation order: 🧲 `physics-expert` (Physik-Profi), 🔥 `heating-expert` (Heiz-Profi), ❄️ `cooling-expert`
(Kühl-Profi), 📊 `demand-expert` (Energiebedarfs-Profi) — all `perfect-quiz`; 🧮 `numerical-brain` (Zahlenhirn,
`perfect-tasks` sorting: 2 tasks); 🔍 `pattern-seer` (Pattern Seer / Musterblick, `perfect-tasks` classification: 2 tasks);
🏁 `completionist` (Completionist / Rundum dabei, `completed-quizzes`). German labels are gender-neutral.

## 3. Corrections to the research values

| Research value | Now | Why |
|---|---|---|
| World primary energy 1.72 × 10¹⁷ **kWh**; solar energy per year 1.52 × 10²¹ **kWh** | 1.644 × 10¹⁷ **Wh**; solar-year item dropped | the research values were Wh mislabelled as kWh (factor 1,000 too large); the list now ends at world consumption as specified |
| World 619.6 EJ (IEA/Enerdata 2023), 19.6 TW | 592 EJ (2024), 18.7 TW | Energy Institute Statistical Review 2025 (total energy supply, physical content method) |
| Germany 2,997.5 TWh (2023) | 2,925 TWh | AGEB annual report 2024: 10,529 PJ |
| Germany mean load 56 GW (491 TWh) | 60 GW | AGEB 2024 gross electricity consumption 527.4 TWh ÷ 8,784 h |
| Onshore wind turbine 4 MW | 5 MW | WindGuard: 2024 average 5.1 MW |
| Powers list starting at LED standby, with hair dryer 1,800 W and kettle 2,200 W (factor 1.2), coal unit | starts at the tea light (as specified), hair dryer and coal unit dropped, "beyond": Earth's sunlight and solar luminosity | spec and distinguishability |
| U-values "DIN 4108-4/TABULA" (brick 1.4, pitched roof 1.5, timber-beam ceiling 1.2, old door 3.2, glass blocks 2.8, triple Ug 0.7, PH wall 0.13, PH roof 0.12) | BAnz AT 04.12.2020 B1 table values (see §2.2); glass blocks dropped (no primary source); triple Ug 0.5; PH wall 0.15, roof 0.10 | the Bundesanzeiger tables (Tab. 2/3) are the normative pauschal values for existing components and were read directly |
| Heating load/demand "IWU/TABULA, Näherung" (e.g. 1950s SFH 160/220, WSchVO 77 100/150, WSchVO 95 70/110, EnEV 2002 55/85, Plattenbau 130/170, Gründerzeit 120/180) | IWU 2015 appendix C.3 per type (EFH_E 303, EFH_F 224, EFH_I 135, EFH_J 89, NBL_GMH_F 122, MFH_B 197) with loads from the typology's H_T | the IWU PDF was read directly; the Plattenbau values were far too high for such a compact block; labels now name the real type and age class |
| KfW 55 30/40, KfW 40 20/28, GEG 40/55 | 25/35, 20/26, 40/50 | IWU Tab. 26 (per A_N ×≈ 1.2 to living area); keeps factor ≥ 1.25 to the EnerPHit retrofit (16/20) |
| Renovated Plattenbau, 1970s office (heating) | dropped; EnerPHit Gründerzeit added | no primary source; the IWU package-2 type is sourced |
| Air change: open-plan office 3.0, single office 1.0, classroom 4.0, lecture hall 5, meeting room 6, restaurant 8, lab 10, sports hall 4, pool 3, car park 6, ISO 7 40 | open-plan and meeting room dropped, single office 1.5, classroom 4.8, cinema 3.5, restaurant 6.4, lab 8.3, sports hall 0.8, pool dropped, car park 2.4, ISO 7 60; added warehouse 0.13, PH dwelling 0.3, ISO 6 150, ISO 5 540 | recomputed from EN 16798-1 cat. II, DIN 1946-7, DIN 18032-1, GaVO (6 m³/(h·m²), confirmed in the MAICO norm table), IEST-RP-CC012; the pool value had no verifiable basis |
| Cooling pairs (e.g. passive home 5/5, hospital 70/50, school 30/12, data centre 1,200/8,000; supermarket, hotel) | 6/2, 70/90, 30/8, 1,000/8,000; supermarket and hotel dropped; GEG home, passive office, attic flat added | made every pair consistent as load × full-load hours (offices 500–1,000 h per nPro/Viessmann); supermarket cooling is dominated by refrigerated cabinets, not space cooling |
| Demand profile axis "Lüften" = n50 in 1/h | ventilation heat loss in kWh/(m²·a) derived from n50 and heat recovery | commensurable with the heating axis and physically meaningful (spec: energy/airtightness-derived axis) |
| Profile costs 22/11/6/3/1.8/−2 €/(m²·a) (heat × 10 ct) | 50/23/10/4.2/3.7/−2 | costs now from final energy incl. boiler efficiency, hot water and auxiliary energy at 2025 prices |
| Profile cooling 5/6/10/10/6/10 | 8/6/5/4/2/7 | ordered by summer heat protection (DIN 4108-2 since the 2000s, PHI overheating check, large plus-energy glazing) and consistent with the cooling quiz (PH home 2, GEG home 5) |
| Final energy task with primary energy rows (DIN V 18599-12 Tab. 5: 66/100/85; f_P products) and plus-energy 0 | only final energy, all > 0, nine building/system pairs | the task asks final energy; a logarithmic scale needs positive values |
| Old SFH final energy 250 | 409 | 303 ÷ 0.8 + hot water + auxiliary; 250 was inconsistent with its own 220 heating demand plus losses |

## 4. Validation

Commands (repo root):

```
.venv/Scripts/python.exe "<ticket>/validate_quiz_content.py"   # jsonschema 4.26 + semantic rules
bun "<ticket>/check_quiz_core.ts"                             # owned TS core: quizIssues, catalogIssues, sheetOf, scoreRun
```

### 4.1 jsonschema + semantic rules (exit 0)

```
jsonschema 4.26.0 — schema 🧰️framework\🛍️products\❓️quiz\🧬️schema\🔣️.json is a valid draft-07 schema
Catalog: schema (Catalog) valid; introduction 5 paragraphs
physics: schema (Quiz) valid
  power-or-energy (classification): 16 items, draw 12; power ×8, energy ×8
  powers (sorting): 14 items, draw 10; ascending, 25.0 decades; smallest neighbour factor 1.60
  energies (sorting): 12 items, draw 9; ascending, 16.0 decades; smallest neighbour factor 3.70
heating: schema (Quiz) valid
  u-values (matching): 17 items, draw 10; smallest neighbour factor 1.15
  heating-load-and-demand (matching): 11 items, draw 8; load 1.20, demand 1.11; 0 of 55 cross-dimension inversions
cooling: schema (Quiz) valid
  air-change-rates (matching): 15 items, draw 10; smallest neighbour factor 1.25
  cooling-load-and-demand (matching): 9 items, draw 7; load 1.25, demand 1.25; 2 of 36 inversions (intended)
demand: schema (Quiz) valid
  standard-profiles (classification): 6 items, no draw; 6 profiles complete and within [min, max]; d_max 1.496
  final-energy (matching): 9 items, draw 7; smallest neighbour factor 1.30
Badges: physics/heating/cooling/demand-expert select 3/2/2/2 tasks; numerical-brain 2 sorting tasks;
        pattern-seer 2 classification tasks; completionist 4 quizzes
errors: 0
warnings: 10 (neighbour factor < 1.25, no duplicates)
  u-values: 0.5→0.6 (1.20), 1.3→1.5 (1.15), 1.5→1.8 (1.20), 1.8→2.1 (1.17), 3.6→4.3 (1.19)
  heating-load: 50→60 (1.20), 60→73 (1.22), 95→115 (1.21)
  heating-demand: 122→135 (1.11), 197→224 (1.14)
```

Checked rules: schema validity of catalog and quizzes; `$schema` resolves to the contract; texts non-empty in en and de;
unique task/item/category/axis/dimension/quiz/badge ids; categories referenced exist; axes max > min; profiles complete
on every axis and within [min, max]; sorting values strictly distinct and positive on logarithmic scales; matching items
carry exactly one value per dimension, positive on logarithmic scales; 2 ≤ draw ≤ items; catalog paths exist and are
unique; badge rules reference existing quizzes and select at least one task; an explanation on every item; U+FE0F after
every emoji path segment. The ten warnings are physically inherent to the sourced tables (BAnz, IWU) and were accepted
instead of bending sourced values; the spec allows this ("where physically sensible").

### 4.2 TS core cross-check (exit 0)

```
catalogIssues: none
physics: quizIssues none — 200 seeded runs: perfect = 1, reversed sortings = 0, random mean 0.488 (0.159–0.814)
heating: quizIssues none — 200 seeded runs: perfect = 1, reversed sortings = 0, random mean 0.499 (0.230–0.818)
cooling: quizIssues none — 200 seeded runs: perfect = 1, reversed sortings = 0, random mean 0.515 (0.180–0.863)
demand:  quizIssues none — 200 seeded runs: perfect = 1, reversed sortings = 0, random mean 0.542 (0.254–0.797)
sheets honour every draw (physics 12/10/9, heating 10/8, cooling 10/7, demand 6/7) for all 800 seeds
failures: 0
```

### 4.3 Site targets on the final content (exit 0 each)

```
bun nx run @teaching/architecture-quiz:test --skip-nx-cache    # vitest: TS core + ajv draft-07 → Test Files 1 passed, Tests 11 passed
bun nx run @teaching/architecture-quiz:check --skip-nx-cache   # proctor check (Rust core):
catalog architecture (…\🎓️teaching\🏛️architecture\❓️quiz\🔣️.json): 4 quizzes, 7 badges, fingerprint c4e280bb…
  quiz physics (3 tasks) revision bda31905…   quiz heating (2 tasks) revision b2a491e4…
  quiz cooling (2 tasks) revision 36008a3f…   quiz demand  (2 tasks) revision d815f8df…
```

So the content passes four independent validators: python jsonschema, the owned TS core, ajv and the owned Rust core.

## 5. Open points for other work packages

- Taxonomy: `🔣️taxonomy.json` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` already lists 🎓 🏛 ⚡ 🧲 🔥 ❄ 📊 ❓ 🎬 🛂
  entries (site-infra); no change by this package.
- Cooling values have no single normative table; they are consistent derived values and flagged as representative in
  the site README.

Generated outputs of this package (`🗑️generated/content/*` downloads, the two validation outputs and the two site-target
logs) were deleted after embedding the results above; the other files in `🗑️generated` belong to the other work packages
of this ticket.

## 6. Follow-up fixes (after `📓️audit-content.md`)

### 6.1 Finding 1/1b — "GEG 2024 minimum" vs "EnEV 2014 / GEG 2020" (HIGH)

Decision: keep the heating-quiz values, make the label say exactly what they model, and remove the double naming in the
demand quiz. Justification, verified against the law text (geg-info.de, GEG 2024 § 15 and § 16):

- § 15 (1): a new residential building's annual primary energy demand must not exceed **0.55 ×** the reference building
  (since the 2023 amendment; 0.75 × under GEG 2020).
- § 16: its envelope loss H′T must not exceed **1.0 ×** the reference building's H′T — unchanged since EnEV 2016.

Heating load and heating demand are envelope quantities; the 55 % primary-energy limit is met through the supply system
(a heat pump turns 1 kWh of heat into about 1.8 ÷ 3.5 ≈ 0.5 kWh of primary energy, the reference gas boiler into
≈ 1.1 ÷ 0.95 ≈ 1.2). So a legal GEG 2024 new build may still have the reference envelope with ≈ 40 W/m² and
≈ 50 kWh/(m²·a); the audit's premise that such a building "would not be permitted today" holds for primary energy only,
not for the envelope. Modelling "GEG 2024" at the KfW 55 envelope (as the audit suggested) would teach the wrong law.

| File / pointer | Before | After |
|---|---|---|
| heating `/tasks/1/items/4` (`geg-2024`) label | "New single-family house built to the GEG 2024 minimum" / „…nach GEG-2024-Mindeststandard“ | "New single-family house to GEG 2024 with the minimum envelope and a heat pump" / „Neues Einfamilienhaus nach GEG 2024 mit Mindest-Gebäudehülle und Wärmepumpe“ |
| heating `/tasks/1/items/4` explanation | reference U-values + IWU 43.7 per A_N | adds § 16 (H′T ≤ 1.0 × reference) vs § 15 (55 % primary energy, met by the heat pump) and the new-build design practice 30–50 W/m² for the load |
| demand `/tasks/0/items/2` (`enev-2014`) label | "New build to EnEV 2014 / GEG 2020" | "New build to EnEV 2014" / „Neubau nach EnEV 2014“ |
| demand `/tasks/0/items/2` explanation | 55 kWh/(m²·a) … | adds that 55 lies slightly above today's minimum envelope, which EnEV 2016 tightened by about 20 % |
| cooling `/tasks/1/items/1` (`new-home-geg`) label | "New home to GEG …" | "New home to GEG 2024 …" |

Now each standard has one value everywhere: EnEV 2014 = 55 kWh/(m²·a) (demand profile F, final-energy item
`enev-2014-gas` 80), GEG 2024 minimum envelope = 50 kWh/(m²·a) / 40 W/m² (heating), GEG 2024 home cooling 5 / 15 W/m²
(cooling). The two are distinct standards (EnEV 2016 tightened the envelope; GEG 2020 and GEG 2024 kept it), so the
50/55 gap is now intentional and explained, not a contradiction. The site README "Conventions" and heating sources table
now state § 15/§ 16.

### 6.2 Finding 2 — `sfh-2000s` load 50 vs formula 52.8 (low)

H_T/A = 1.14 W/(m²·K) is the exact IWU appendix C.3 cell for EFH_J (re-read from the PDF). The value 50 came from
ventilation 0.5 1/h (0.425 W/(m²·K)) for buildings since the EnEV 2002, which made the airtight envelope mandatory:
(1.14 + 0.425) × 32 = 50.1. The convention was undocumented; the item explanation and the README "Conventions" now state
n = 0.6 1/h before 2002 and 0.5 1/h from 2002 on. Value unchanged.

### 6.3 Finding 3 — chocolate bar 630 vs 628 Wh (very low)

Agreed: `energies` item `chocolate-bar` is now 628 Wh with "≈ 628 Wh" in both languages (540 × 1.163 = 628.02);
the classification item keeps "≈ 0.63 kWh" (correct rounding).

### 6.4 Finding 4 — asymmetric sorting prompts (very low)

Agreed: the `energies` prompt now ends "…the values span about 16 orders of magnitude." / „…die Werte umfassen rund
16 Größenordnungen.“, matching the `powers` prompt.

### 6.5 Idiom nit

Agreed: catalog `/introduction/paragraphs/2/en` now reads "partially correct answers earn partial points".

### 6.6 Re-validation (all exit 0)

```
.venv/Scripts/python.exe validate_quiz_content.py      → catalog and 4 quizzes schema-valid; errors 0; warnings 10 (unchanged, accepted §4.1)
                                                          energies still strictly ascending; heating 0 of 55 cross-dimension inversions
bun check_quiz_core.ts                                  → catalogIssues none; quizIssues none ×4; 200 seeded runs per quiz:
                                                          perfect = 1, reversed sortings = 0, random mean 0.488 / 0.499 / 0.515 / 0.542; failures 0
bun nx run @teaching/architecture-quiz:test             → vitest 11/11 passed (TS core + ajv)
bun nx run @teaching/architecture-quiz:check            → proctor check: 4 quizzes, 7 badges; revisions physics f54d27b7…,
                                                          heating ce40165e…, cooling 351fcfea…, demand 6265e8a2…
```

The generated logs of this follow-up were deleted after embedding.
