# Quiz content — wording and number/unit typography

Scope: the four quiz content files under `🎓️teaching/🏛️architecture/⚡️energy/<topic>/❓️quiz/🔣️.json` (`🧲️physics`, `🔥️heating`, `❄️cooling`, `📊️demand`). Texts only. Source of findings: `📓️audit-final-i18n-a11y.md` A4, A5, I17, I19, I20, I22, N12, N14.

Legend: `⍽` stands for U+00A0 (NO-BREAK SPACE). In the JSON files it is written as the JSON escape (backslash + `u00a0`), never as the invisible character. JSON paths use indexes; the ids of the task and item follow in parentheses.

## Result in numbers

| | physics | heating | cooling | demand | total |
|---|---|---|---|---|---|
| strings changed / strings | 98 / 196 | 59 / 130 | 59 / 114 | 36 / 94 | 252 / 534 |
| wording changes (strings) | 26 | 7 | 6 | 8 | 47 |
| — I17 German wording | 9 | 1 | 1 | 4 | 15 |
| — I19 "asks" → "requires" | 0 | 0 | 4 | 0 | 4 |
| — I20 (c) card value first | 14 | 0 | 0 | 0 | 14 |
| — I22 abbreviation expanded (EN) | 3 | 6 | 1 | 4 | 14 |
| no-break spaces written | 454 | 153 | 232 | 141 | 980 |
| thousands separators (DE) | 26 | 0 | 8 | 0 | 34 |
| percent EN (space removed) / DE (U+00A0) | 1 / 1 | 4 / 3 | 1 / 1 | 3 / 3 | 9 / 8 |
| em dash → en dash (EN) | 2 | 0 | 1 | 0 | 3 |
| minus sign changed | 0 | 0 | 0 | 0 | 0 |

I17 is the audit's 15 findings, one string each: the heating description, nine "-Jahre", three "Rate", "das Füllen", "macht … zu Bewegung". Every wording string is listed below.

## Wording changes

### 🧲️physics

| JSON path | language | old | new |
|---|---|---|---|
| `tasks[0].prompt` (power-or-energy) | de | eine Leistung ist (eine Rate: Energie pro Zeit, gemessen in Watt) | eine Leistung ist (Energie pro Zeit, gemessen in Watt) |
| `tasks[0].categories[0].description` (power-or-energy / power) | de | Eine Rate: wie schnell Energie umgesetzt wird | Energie pro Zeit: wie schnell Energie umgesetzt wird |
| `tasks[0].items[4].explanation` (power-or-energy / heating-load) | de | Einfamilienhaus der 1960er (110 m² … | Einfamilienhaus der 1960er-Jahre (110⍽m² … |
| `tasks[0].items[7].explanation` (power-or-energy / wallbox) | de | die Rate, mit der der Autoakku gefüllt wird. | die Geschwindigkeit, mit der der Autoakku geladen wird. |
| `tasks[0].items[9].explanation` (power-or-energy / ev-battery) | de | dauert das Füllen 60 ÷ 11 ≈ 5,5 Stunden | dauert das Laden 60⍽÷⍽11⍽≈⍽5,5⍽Stunden |
| `tasks[0].items[11].explanation` (power-or-energy / heating-demand-house) | de | Rund 33 000 kWh pro Jahr für ein Haus der 1960er (110 m² … | Rund 33.000⍽kWh pro Jahr für ein Haus der 1960er-Jahre (110⍽m² … |
| `tasks[0].items[11].explanation` (power-or-energy / heating-demand-house) | en | …; IWU German residential building typology 2015, type EFH_E) | …; IWU, the Institute for Housing and Environment, German residential building typology 2015, type EFH_E) |
| `tasks[0].items[15].explanation` (power-or-energy / energy-certificate) | en | (GEG § 85) | (German Buildings Energy Act, GEG, § 85) |
| `tasks[1].items[5].label` (powers / heating-load-old-house) | de | Heizlast eines unsanierten Einfamilienhauses der 1960er | Heizlast eines unsanierten Einfamilienhauses der 1960er-Jahre |
| `tasks[1].items[2].explanation` (powers / sunlight-square-metre) | en | About 1,000 W: the standard irradiance … | About 1⍽kW (1,000⍽W): the standard irradiance … |
| `tasks[1].items[2].explanation` (powers / sunlight-square-metre) | de | Rund 1000 W: die Standard-Bestrahlungsstärke … | Rund 1⍽kW (1.000⍽W): die Standard-Bestrahlungsstärke … |
| `tasks[1].items[3].explanation` (powers / kettle) | en | About 2,000 W according to the rating plate | About 2⍽kW (2,000⍽W) according to the rating plate |
| `tasks[1].items[3].explanation` (powers / kettle) | de | Rund 2000 W laut Typenschild | Rund 2⍽kW (2.000⍽W) laut Typenschild |
| `tasks[1].items[9].explanation` (powers / nuclear-unit) | en | 1,410 MW net electrical output | 1.41⍽GW (1,410⍽MW) net electrical output |
| `tasks[1].items[9].explanation` (powers / nuclear-unit) | de | 1410 MW elektrische Nettoleistung | 1,41⍽GW (1.410⍽MW) elektrische Nettoleistung |
| `tasks[1].items[13].explanation` (powers / sun) | en | 3.828 × 10²⁶ W (IAU 2015 nominal solar luminosity) | 382.8⍽YW (3.828⍽×⍽10²⁶⍽W, IAU 2015 nominal solar luminosity) |
| `tasks[1].items[13].explanation` (powers / sun) | de | 3,828 × 10²⁶ W (nominale Sonnenleuchtkraft der IAU 2015) | 382,8⍽YW (3,828⍽×⍽10²⁶⍽W, nominale Sonnenleuchtkraft der IAU 2015) |
| `tasks[2].items[3].explanation` (energies / daily-food) | en | (DGE reference values for moderate activity, … | (reference values of the German Nutrition Society, DGE, for moderate activity, … |
| `tasks[2].items[6].explanation` (energies / petrol-tank) | de | doch ein Motor macht nur etwa ein Viertel davon zu Bewegung. | doch ein Motor setzt nur etwa ein Viertel davon in Bewegung um. |
| `tasks[2].items[7].explanation` (energies / household-electricity) | en | About 2,500 kWh = 2.5 MWh per year | About 2.5⍽MWh (2,500⍽kWh) per year |
| `tasks[2].items[7].explanation` (energies / household-electricity) | de | Rund 2500 kWh = 2,5 MWh pro Jahr | Rund 2,5⍽MWh (2.500⍽kWh) pro Jahr |
| `tasks[2].items[8].label` (energies / heating-demand-house) | de | … eines unsanierten Einfamilienhauses der 1960er | … eines unsanierten Einfamilienhauses der 1960er-Jahre |
| `tasks[2].items[10].explanation` (energies / germany-primary-energy) | en | About 2,925 TWh: 10,529 PJ in 2024 | About 2.925⍽PWh (2,925⍽TWh): 10,529⍽PJ in 2024 |
| `tasks[2].items[10].explanation` (energies / germany-primary-energy) | de | Rund 2925 TWh: 10 529 PJ im Jahr 2024 | Rund 2,925⍽PWh (2.925⍽TWh): 10.529⍽PJ im Jahr 2024 |
| `tasks[2].items[11].explanation` (energies / world-primary-energy) | en | About 164,000 TWh: 592 EJ in 2024 | About 164.4⍽PWh (164,400⍽TWh): 592⍽EJ in 2024 |
| `tasks[2].items[11].explanation` (energies / world-primary-energy) | de | Rund 164 000 TWh: 592 EJ im Jahr 2024 | Rund 164,4⍽PWh (164.400⍽TWh): 592⍽EJ im Jahr 2024 |

Card values of I20 (c), computed from `value` and the task's `quantity` (unit Wh, four significant digits, SI prefix with the mantissa in [1, 1000)) and confirmed with `Intl.NumberFormat` in bun: 2 500 000 Wh → 2.5 MWh / 2,5 MWh; 2 925 000 000 000 000 Wh → 2.925 PWh / 2,925 PWh; 164 400 000 000 000 000 Wh → 164.4 PWh / 164,4 PWh. The other nine items of the task already named the card's value first. Task `powers` (unit W, follow-up request): 1 000 W → 1 kW; 2 000 W → 2 kW; 1 410 000 000 W → 1.41 GW / 1,41 GW; 3.828e26 W → 382.8 YW / 382,8 YW.

### 🔥️heating

| JSON path | language | old | new |
|---|---|---|---|
| `description` | de | Heizwärmebedarfe von Gebäuden von den 1960ern bis zum Passivhaus. | Heizwärmebedarfe von Gebäuden aus den 1960er-Jahren bis zum Passivhaus. |
| `tasks[0].items[0].explanation` (u-values / single-glazing) | en | Source: BAnz AT 04.12.2020 B1, Table 3 | Source: Federal Gazette (BAnz AT 04.12.2020 B1), Table 3 |
| `tasks[0].items[5].explanation` (u-values / front-door-geg) | en | Source: Gebäudeenergiegesetz (GEG) 2024, Annex 1. | Source: German Buildings Energy Act (Gebäudeenergiegesetz, GEG) 2024, Annex 1. |
| `tasks[0].items[10].explanation` (u-values / masonry-wall-1980s) | en | to meet the WSchVO 1982. | to meet the Thermal Insulation Ordinance (WSchVO) 1982. |
| `tasks[1].items[1].explanation` (heating-load-and-demand / gruenderzeit-retrofit) | en | IWU modernisation package 2 for type MFH_B | IWU (Institute for Housing and Environment) modernisation package 2 for type MFH_B |
| `tasks[1].items[2].explanation` (heating-load-and-demand / kfw-40) | en | About 20 W/m² and 26 kWh/(m²·a): envelope loss … | About 20⍽W/m² and 26⍽kWh/(m²·a) for this funding standard of KfW (German state development bank): envelope loss … |
| `tasks[1].items[5].explanation` (heating-load-and-demand / sfh-2000s) | en | About 50 W/m² and 89 kWh/(m²·a): IWU type EFH_J … | About 50⍽W/m² and 89⍽kWh/(m²·a) under the Energy Saving Ordinance (EnEV): IWU type EFH_J … |

### ❄️cooling

| JSON path | language | old | new |
|---|---|---|---|
| `tasks[0].items[2].explanation` (air-change-rates / apartment) | en | DIN 1946-6 asks −0.001·A² + … | DIN 1946-6 requires −0.001·A² + … |
| `tasks[0].items[4].explanation` (air-change-rates / single-office) | en | category II of EN 16798-1 asks 7 l/s per person | category II of EN 16798-1 requires 7⍽l/s per person |
| `tasks[0].items[5].explanation` (air-change-rates / residential-car-park) | en | the garage ordinances of the German states ask 6 m³/h | the garage ordinances of the German states require 6⍽m³/h |
| `tasks[0].items[10].explanation` (air-change-rates / operating-room) | en | DIN 1946-4 asks 60 m³/(h·m²) | DIN 1946-4 requires 60⍽m³/(h·m²) |
| `tasks[1].items[1].explanation` (cooling-load-and-demand / new-home-geg) | en | the summer heat protection of DIN 4108-2 limits solar gains | the summer heat protection of DIN 4108-2, which the GEG (German Buildings Energy Act) requires, limits solar gains |
| `tasks[1].items[7].label` (cooling-load-and-demand / office-1970s) | de | Bürohochhaus der 1970er, vollverglast | Bürohochhaus der 1970er-Jahre, vollverglast |

### 📊️demand

| JSON path | language | old | new |
|---|---|---|---|
| `tasks[0].items[0].label` (standard-profiles / unrenovated-old-building) | de | Unsanierter Altbau (Einfamilienhaus der 1960er) | Unsanierter Altbau (Einfamilienhaus der 1960er-Jahre) |
| `tasks[0].items[0].explanation` (standard-profiles / unrenovated-old-building) | en | heating (IWU type EFH_E), a leaky envelope | heating (type EFH_E of the IWU, the Institute for Housing and Environment), a leaky envelope |
| `tasks[0].items[2].explanation` (standard-profiles / enev-2014) | en | minimum envelope, which EnEV 2016 tightened by about 20 % | minimum envelope, which the Energy Saving Ordinance (EnEV) 2016 tightened by about 20% |
| `tasks[0].items[3].explanation` (standard-profiles / kfw-40) | en | Profile A: about 26 kWh/(m²·a) heating | Profile A, a funding standard of KfW (German state development bank): about 26⍽kWh/(m²·a) heating |
| `tasks[1].items[1].label` (final-energy / deep-retrofit-heat-pump) | de | Einfamilienhaus der 1960er, tiefgreifend … | Einfamilienhaus der 1960er-Jahre, tiefgreifend … |
| `tasks[1].items[5].label` (final-energy / old-house-heat-pump) | de | Unsaniertes Einfamilienhaus der 1960er mit Luft-Wärmepumpe | Unsaniertes Einfamilienhaus der 1960er-Jahre mit Luft-Wärmepumpe |
| `tasks[1].items[8].label` (final-energy / old-house-gas) | de | Unsaniertes Einfamilienhaus der 1960er mit altem Gaskessel | Unsaniertes Einfamilienhaus der 1960er-Jahre mit altem Gaskessel |
| `tasks[1].items[8].explanation` (final-energy / old-house-gas) | en | in the energy certificate (GEG Annex 10). | in the energy certificate (German Buildings Energy Act, GEG, Annex 10). |

## Mechanical typography

Counts per file and rule; the examples show the new text.

| rule | physics | heating | cooling | demand | examples |
|---|---|---|---|---|---|
| U+00A0 between number and unit, EN | 153 | 63 | 90 | 35 | `About 35⍽W: roughly 12⍽g of paraffin` · `5.8⍽W/(m²·K) (Ug)` · `About 0.13⍽1/h` |
| U+00A0 between number and unit, DE | 142 | 63 | 89 | 35 | `Rund 35⍽W: etwa 12⍽g Paraffin` · `in rund 4⍽Stunden` · `Kühlbedarf für 26⍽°C` |
| U+00A0 next to × ÷ ≈ = ≤, EN | 79 | 12 | 26 | 34 | `110⍽m²⍽×⍽160⍽W/m²` · `(λ⍽=⍽0.035⍽W/(m·K))` · `n50⍽≤⍽0.6⍽1/h` |
| U+00A0 next to × ÷ ≈ = ≤, DE | 79 | 12 | 26 | 34 | `60⍽÷⍽11⍽≈⍽5,5⍽Stunden` · `R⍽≈⍽10⍽m²·K/W` · `35 Heizwärme⍽÷⍽0,9` |
| percent EN: space removed | 1 | 4 | 1 | 3 | `a module with 20% efficiency` · `at most 55% of the GEG reference building` · `at least 75% heat recovery` |
| percent DE: U+00A0 before % | 1 | 3 | 1 | 3 | `ein Modul mit 20⍽% Wirkungsgrad` · `höchstens 55⍽% des GEG-Referenzgebäudes` · `auf 10⍽% der Stunden` |
| thousands separator DE (dot) | 26 | 0 | 8 | 0 | `33 000 kWh` → `33.000⍽kWh` · `8760 h` → `8.760⍽h` · `1800 m³/h in 2230 m³` → `1.800⍽m³/h in 2.230⍽m³` |
| thousands separator EN (comma) | 0 | 0 | 0 | 0 | already a comma everywhere (26 numbers in physics, 8 in cooling); `164,400` comes from I20 (c) |
| em dash → en dash, EN | 2 | 0 | 1 | 0 | `orders of magnitude – from a tea light` · `DIN EN 12831) – the power of nine kettles` · `in 2,230⍽m³ – a large volume` |
| minus sign | 0 | 0 | 0 | 0 | the 8 minus signs (`−12⍽°C` ×4, `−0.001·A²` / `−0,001·A²`, `−2⍽€/(m²·a)` ×2) were already U+2212; no hyphen stands for a minus |

All 34 German thousands separators, each judged in its sentence:

- physics (26): `2.300 kcal` ×2, `1.000 W/m²`, `1.410 MW` ×2, `2.500 kWh` ×2, `8.760 h`, `33.000 kWh` (was `33 000`), `9.500 kWh`, `4.000 mAh` ×2, `1.000 W`, `2.000 W`, `3.251 MW`, `8.784 Stunden`, `1.361 W/m²`, `6.371 km`, `9.000-mal`, `1.900–2.500 kcal` (2), `2.000 Volllaststunden`, `4.000 Zwei-Personen-Haushalten`, `2.925 TWh`, `10.529 PJ` (was `10 529`), `164.400 TWh` (was `164 000`, new value from I20 (c)).
- cooling (8): `1.800 m³/h`, `2.230 m³`, `1.200 m³/h`, `3.600 s`, `1.300 Volllaststunden`, `1.000 W/m²`, `8.000 kWh/(m²·a)`, `8.000 Volllaststunden`.

Not grouped, on purpose: years (1860–1918, 1957, 1958–1968, 1969–1978, 1977, 1978/1979, 1982, 1984–1994, 1995–2001, 2002–2009, 2015, 2016, 2023, 2024, 2025), standards (DIN 1946-4/-6/-7, DIN 4108-2, DIN V 4108-6, DIN 18032-1, DIN 51603-1, DIN EN 12831, EN 16798-1, IEC 60904-3, ISO 7730, ISO 8996, ISO 14644-4, VDI 2052, VDI 2078, IEST-RP-CC012), `EnEV 2002/2014/2016`, `GEG 2024`, `WSchVO 1982/1995`, `BAnz AT 04.12.2020 B1`, type codes.

## Decisions

1. **No-break space as JSON escape.** The files had no no-break space before. A real U+00A0 cannot be told from a space in a diff or an editor, and editing tools silently turn it into a space; the escape stays visible and greppable (same reasoning as `quiz_react_nbsp_escapes.mjs` for the React sources). All other characters stay real characters as before. The typography check fails on a real U+00A0 in the source.
2. **Unit words** that take the no-break space: hours / Stunden, minutes / Minuten, litre / Liter, full-load hours / (Kühl-)Volllaststunden, air changes / Luftwechsel (it is the unit 1/h in words: "8.3⍽air changes"). Counted nouns keep the normal space: students, athletes, pupils, turbines, nuclear units, households, phases, "orders of magnitude", "times".
3. **Operators.** U+00A0 on both sides of × ÷ ≈ = as instructed, and of ≤ (one formula, `n50 ≤ 0.6 1/h`, same class). `+` keeps normal spaces so that a long sum can still break at the plus. Consequence to check in the narrow results column: the longest unbreakable runs are now 38 characters in physics (`4.19⍽kJ/(kg·K)⍽×⍽1⍽kg⍽×⍽80⍽K⍽=⍽335⍽kJ;`), 32 in cooling (`60⍽m²⍽×⍽2.5⍽m³/(h·m²)⍽≈⍽860⍽m³/h`) and 31 in demand (`Warmwasser)⍽÷⍽Jahresarbeitszahl`); before, the longest were German compounds of 26–30 characters. If the explanation column is narrower than that on a phone, it needs `overflow-wrap: anywhere` (client side, not touched).
4. **I22 reading order**: description, then per task prompt → labels of all items → explanations of all items. Where the first use is a short label, the label is unchanged and the expansion is in that item's explanation: heating `front-door-geg` (GEG), `kfw-40` (KfW), `sfh-2000s` (EnEV); cooling `new-home-geg` (GEG); demand `enev-2014` (EnEV), `kfw-40` (KfW).
5. **I22 form inside parentheses.** To avoid a bracket inside a bracket, the expansion is an apposition there: "(German Buildings Energy Act, GEG, § 85)", "(type EFH_E of the IWU, the Institute for Housing and Environment)", "IWU, the Institute for Housing and Environment, German residential building typology 2015". Outside parentheses the form of the task is used ("Thermal Insulation Ordinance (WSchVO) 1982", "Federal Gazette (BAnz AT 04.12.2020 B1)", "IWU (Institute for Housing and Environment)", "KfW (German state development bank)"). In demand `enev-2014` the expansion sits inside the existing parenthesis ("which the Energy Saving Ordinance (EnEV) 2016 tightened").
6. **I22 and the numbers check.** The expansions add no number to the English text (heating `sfh-2000s` says "under the Energy Saving Ordinance (EnEV)" and keeps "since EnEV 2002"), so the check "English and German state the same numbers" holds without exceptions for them.
7. **GEG in heating.** The first explanation had the German long form "Gebäudeenergiegesetz (GEG) 2024", which explains nothing to an English reader; it is now "German Buildings Energy Act (Gebäudeenergiegesetz, GEG) 2024".
8. **DGE** (physics, `energies / daily-food`) is a German abbreviation of the same kind as the listed ones and was expanded too ("German Nutrition Society, DGE"), although the audit did not list it.
9. **I20 (c) scope**: in `energies` the two audit items plus `household-electricity`, whose explanation named 2,500 kWh before the card's 2.5 MWh; "164 000 TWh" became the exact card value 164,400 / 164.400 TWh. In `powers` (follow-up request) the four items whose explanation used another prefix than the card. For `sun` the familiar form and the source share one parenthesis ("382.8⍽YW (3.828⍽×⍽10²⁶⍽W, IAU 2015 nominal solar luminosity)") instead of two brackets in a row.
10. **Method.** Wording and thousands separators were edited one by one; the space, percent and dash rules were applied by `ui_content_typography_check.py --fix` (same rules as the check, idempotent), then all four files were read in full. That reading found one slip of mine (a lost space, "mitLuft-Wärmepumpe" / "mitaltem Gaskessel" in two demand labels), fixed before the validation below.

## Left unchanged, and why

- Physics `powers / world-primary-power`: the card shows `18.7 TW`, the explanation says "About 19 TW" / "Rund 19 TW". This is rounding in the same prefix, not another prefix, so rule (c) does not apply; the typography check prints it as a note.
- Later uses of an abbreviation stay short, including heating `window-geg`, which still reads "Gebäudeenergiegesetz (GEG) 2024" in English.
- German texts: abbreviations are not expanded (I22 is about the English audience).
- English "a rate" (physics prompt, category, wall box) stays; only the German "Rate" was a calque.
- Standards and names are not expanded: DIN, DIN EN, DIN V, EN, ISO, IEC, VDI, IEST, "DB class 403", "Fraunhofer ISE", "AG Energiebilanzen", "Stromspiegel für Deutschland", "Deutsche WindGuard", "Gründerzeit", "Passive House Institute".
- Not a number–unit pair, normal space kept: "§ 85", "§ 16 GEG", "§ 15", "Table 3" / "Tabelle 3", "ISO class 7", "category II", "Isar 2", "ICE 3", "z. B.", "e.g.".
- German compounds with hyphens are already unbreakable where it matters: "230-V/16-A-Steckdose", "2-m²-Modul", "55-%-Primärenergiegrenze", "9.000-mal".
- U+202F as thousands separator (the audit's proposal) is not used; comma / dot as instructed.
- N14 (Brennwert): note only. N11 (reference area of the heating task) was not part of this task.
- Nothing outside the four files was edited (catalog, framework, React client, READMEs, tests).

## Validation

Run from `C:\git\semio` after the last edit (again after the `powers` follow-up, same results).

The follow-up exposed a gap in the typography check: its unit set did not contain SI-prefixed forms such as `YW`, so "382.8 YW" with a plain space passed. The set now holds every SI prefix of the units of `prefixed` quantities (W, Wh); the check then failed on the two strings, `--fix` corrected them, and the check passes.

| command | result |
|---|---|
| `bun nx run @teaching/architecture-quiz:test --skip-nx-cache` | exit 0 — "Test Files 3 passed (3)", "Tests 28 passed (28)", "Successfully ran target test for project @teaching/architecture-quiz" |
| `python <ticket>/ui_content_structure_check.py` | exit 0 — "OK"; cooling 193 JSON paths / 57 texts, demand 183 / 47, heating 219 / 65, physics 301 / 98, each "0 structure findings" |
| `python <ticket>/ui_content_typography_check.py` | exit 0 — "OK", 0 violations in all four files (the same script reported 1063 violations on the files as committed) |
| `python <ticket>/ui_content_typography_check.py --fix` (second run) | every rule count 0 for every file: the fix is idempotent |
| `git diff --stat -- 🎓️teaching/🏛️architecture/⚡️energy` | 4 files changed, 243 insertions(+), 243 deletions(-) — only lines that hold a text |

What the two scripts prove:

- `ui_content_structure_check.py` compares each working-tree file with `git show HEAD:<path>` (read only): everything outside the `en` / `de` string literals is identical character for character (ids, `value`, `values`, `profile`, `min`, `max`, `draw`, `unit`, layout, newlines stay LF); the parsed documents have the same JSON paths in the same order and equal non-text values of the same type; every text has a non-empty `en` and `de`. `--changes` prints the change list this report is built from.
- `ui_content_typography_check.py` checks every text: no digit followed by a plain space and a unit symbol (set: the `unit` fields, with every SI prefix for W and Wh, plus %, K, °C, h, s, g, kg, l, m, cm, mm, km, m², m³, W, kW, MW, GW, TW, PW, Wh, kWh, MWh, GWh, TWh, PWh, J, kJ, MJ, PJ, EJ, kcal, kWp, mAh, met, PS, V, A, €, ct, Pa) or a unit word; no plain space next to × ÷ ≈ = ≤; no space of any kind as thousands separator; no ungrouped quantity of four or more digits (years and standards excepted by rule, not by list); English without "digit, space, %", German without "digit%"; no em dash; no hyphen as minus; no narrow or figure space; no U+00A0 outside the rules and none as a real character; no "Sie / Ihr / Ihnen"; English and German state the same numbers in every text pair, read with the separators of each language (two intended differences: "before 1979" / "bis 1978", "24-hour operation" / "rund um die Uhr"); in the `energies` task the first value of each explanation equals the card's value.
- German formal address: 0 hits. All four files parse as JSON.

Environment notes: the `repo` MCP server was not connected in this session; no ticket was opened or closed. A running process keeps the four quiz files memory-mapped, so a truncating write fails on Windows ("Invalid argument"); `--fix` therefore overwrites in place.

## Files

- Edited: `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/❓️quiz/🔣️.json`, `…/🔥️heating/❓️quiz/🔣️.json`, `…/❄️cooling/❓️quiz/🔣️.json`, `…/📊️demand/❓️quiz/🔣️.json`.
- Kept in the ticket folder: `ui_content_typography_check.py`, `ui_content_structure_check.py`, this report.
- Removed: `🗑️generated/ui-content/` (logs of the runs above).
