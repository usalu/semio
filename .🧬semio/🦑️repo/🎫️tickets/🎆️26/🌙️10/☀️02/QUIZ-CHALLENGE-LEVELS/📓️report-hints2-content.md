# Report — hint revision round 2: short labels and familiar references (content)

Agent: quiz content, 2026-10-04. Design §8.4a items 4 and 5; schema shapes from `📓️hints-schema-landed.md` "Round 2"
(`short: ShortText` right after `label`; `familiar` after `value`/`values`, before `explanation`).

## Changed files

Updated:
- `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/❓️quiz/🔣️.json` — 34 `short` (items, 1 category), 12 `familiar`
- `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/❓️quiz/🔣️.json` — 25 `short` (items, 2 quantities), 6 `familiar`
- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/❓️quiz/🔣️.json` — 18 `short` (items, 2 quantities), 7 `familiar`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/❓️quiz/🔣️.json` — 14 `short` (items, 3 axes), 2 `familiar`
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🧪️catalog/🟦️.ts` — two content rules per quiz (below), docstring extended

Created (ticket folder, kept): `hints2_content_table.ts` — checks the rules independently of the test and prints the table below.

No label, value, explanation or icon changed.

## Site content rules (catalog test, per quiz file)

1. "gives every label longer than 40 characters a short form, keeps every short form brief, plain and distinct in its task":
   every item, category, axis and quantity (sorting `quantity`, matching `dimensions[].quantity`) whose label exceeds 40 code
   points in either language has `short`; every `short` is ≤ 40 code points in both languages, starts with no digit and
   holds no parenthesis (`/^[^\d()][^()]*$/u`); the items' effective names (`short ?? label`) are distinct per task and language.
2. "marks at least one familiar item in every sorting and matching".

## Runs

- `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/architecture-quiz:test` → 5 files, **243 passed** (schema round 2 landed; ajv validates the new members).
- `bunx vitest run --config 🧪️tests/🎚️config/🟦️.ts --reporter=verbose catalog` (site root) → **34 passed**, the 8 new cases among them.
- `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/architecture-quiz:typecheck` → exit 0.
- `bun hints2_content_table.ts` → "problems: none"; familiar per task: powers 5/14, energies 7/12, u-values 3/17,
  heating-load-and-demand 3/11, air-change-rates 5/15, cooling-load-and-demand 2/9, final-energy 2/9.
- `SEMIO_TEST_BUDGET_MS=900000 NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/quiz:test` → 11 files, **435 passed, 3 failed**:
  `read-views` (expects `under`, gets `verdict: "reversed"`), `shared-vectors` learner-lifecycle (`hints-on-easy/…/energy-carriers`:
  3 entries produced, 4 committed — the cap of 3), `document-validation` (easy-run hints vs contract). All three use core fixtures
  (`plant`, `bulb`, `energy-carriers`), not the energy quizzes: in-flight round-2 core/vector work, not this content.
- `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/architecture-quiz:check` (proctor check, run after `📓️report-hints2-rust-proctor.md`
  appeared) → exit 0: catalog architecture, 4 quizzes, 7 badges; physics 3 tasks, heating/cooling/demand 2 tasks each, all accepted.

## Deviations and decisions

- **Where `short` goes:** every label > 40 in either language, plus shorter labels whose parentheses (`Burning tea light (heat)`,
  `Single glazing (one pane of glass)`, `Data centre (server room floor)`, `Energy (demand)`, …) or leading article/number
  (`One full smartphone charge`, `Ein Liter Heizöl`, `A full 50-litre tank of petrol`) read badly after "1 ×". Where only one language
  is long, the other language's `short` repeats or trims its label (a `ShortText` needs both).
- **Two-dimension quantities and all long axes** get a plain short (`Heating load`/`Heizlast`, `Net energy costs`/`Netto-Energiekosten`);
  the `cooling` axis too (drops "to keep 26 °C"). Single-dimension quantities stay without (labels already short, never named in a hint).
- **Case:** shorts keep the labels' sentence case (capital first letter). The site's own hint test already lower-cases axis names
  (`spoken` in `⛰️challenge-levels`), so the client can lower-case EN mid-sentence; upper-casing would not be safe the other way round.
- **Values hidden:** shorts drop the numbers that leak a value (`136 PS`, `10 kWp`, `50-litre`, `1 m²`, room sizes, years of a range).
  Kept are names that are not the value: `KfW Efficiency House 40/55`, `WSchVO 1995`, `EnEV 2002/2014`, `GEG 2024`, `ISO class 5/6/7`, `ICE 3`.
- **Classification items** (`power-or-energy`) keep their quantity word ("Heat given off…", "Capacity of…", "Energy in…"),
  so the short says exactly what the label asks to classify and reveals no more than the label does.
- **Familiar** = an everyday thing or place a student can picture: tea light, sitting person, kettle, wall box, car engine; phone
  charge, boiling water, chocolate bar, daily food, litre of heating oil, car battery, tank of petrol; single glazing, roller-shutter
  box, front door; Plattenbau, Gründerzeit block, 1960s family house; apartment, sports hall, cinema, classroom, restaurant;
  attic flat, school; old family house with old gas boiler, Gründerzeit block with gas heating. Not on astronomical, national or
  standard-specific items (Sun, Germany, KfW, GEG reference parts, cleanrooms, data centre). `sunlight-square-metre` stays unmarked
  (picturable, but its power is not something a layperson can size).
- **German "beim {{axis}}/{{quantity}}" (§8.4a.6/7):** `Heizlast`, `Kühllast` (feminine) and `Netto-Energiekosten` (plural) do not
  take "beim". No natural masculine/neuter noun keeps the meaning, so the shorts stay exact and the template must not hard-code
  the article (e.g. "bei {{quantity}}" or "— {{quantity}}"). Passed on to the client agent below.
- The water label still starts with a digit (`1 Liter Wasser …`); its short "Aufkochen eines Liters Wasser" fixes the hint. The
  label itself was not renamed (out of scope; display only).

## Notes for the next agent

- Client: use `short ?? label` for items, categories, axes and quantities in hints; mind the German article issue above.
- Vectors/Python reference built from the live quizzes now see `short` and `familiar`; regenerate after the core lands.
- The 3 red `@semio-tech/quiz` tests belong to the core round-2 owners (verdict/cap fixtures), not to this content.

## Every short label and familiar flag

Lengths in code points (U+00A0 shown as a space). "–" = no short.

| Quiz / task | Part | Label EN (len) | Label DE (len) | Short EN (len) | Short DE (len) | Familiar |
|---|---|---|---|---|---|---|
| physics / power-or-energy | item `resting-person` | Heat given off by a person sitting still (40) | Wärmeabgabe eines ruhig sitzenden Menschen (42) | Heat given off by a sitting person (34) | Wärmeabgabe eines sitzenden Menschen (36) |  |
| physics / power-or-energy | item `nuclear-unit` | Electrical output of a nuclear power plant unit (47) | Elektrische Leistung eines Kernkraftwerksblocks (47) | Output of a nuclear power plant unit (36) | Leistung eines Kernkraftwerksblocks (35) |  |
| physics / power-or-energy | item `car-engine` | Rated output of a car engine in horsepower (PS) (47) | Motorleistung eines Autos in PS (31) | Car engine output in horsepower (31) | Motorleistung eines Autos in PS (31) |  |
| physics / power-or-energy | item `household-electricity` | Annual electricity use of a two-person household (48) | Jahresstromverbrauch eines Zwei-Personen-Haushalts (50) | Annual electricity use of two people (36) | Jahresstromverbrauch zweier Personen (36) |  |
| physics / power-or-energy | item `ev-battery` | Usable capacity of an electric car battery (42) | Nutzbare Kapazität eines E-Auto-Akkus (37) | Capacity of an electric car battery (35) | Kapazität eines E-Auto-Akkus (28) |  |
| physics / power-or-energy | item `heating-oil-litre` | Energy content of one litre of heating oil (42) | Energieinhalt eines Liters Heizöl (33) | Energy in a litre of heating oil (32) | Energieinhalt eines Liters Heizöl (33) |  |
| physics / power-or-energy | item `heating-demand-house` | Annual heating demand of an unrenovated single-family house (59) | Jahres-Heizwärmebedarf eines unsanierten Einfamilienhauses (58) | Annual heating demand of an old house (37) | Jahres-Heizwärmebedarf eines Altbaus (36) |  |
| physics / power-or-energy | item `pv-annual-yield` | Annual yield of a 10 kWp rooftop solar system (45) | Jahresertrag einer 10-kWp-Solaranlage auf dem Dach (50) | Annual yield of a rooftop solar system (38) | Jahresertrag einer Dach-Solaranlage (35) |  |
| physics / power-or-energy | item `energy-certificate` | Primary energy demand stated in an energy performance certificate (65) | Primärenergiebedarf im Energieausweis (37) | Primary energy in an energy certificate (39) | Primärenergiebedarf im Energieausweis (37) |  |
| physics / power-or-energy | category `energy` | Energy (demand) (15) | Arbeit (6) | Energy (6) | Arbeit (6) |  |
| physics / powers | item `tea-light` | Burning tea light (heat) (24) | Brennendes Teelicht (Wärme) (27) | Burning tea light (17) | Brennendes Teelicht (19) | yes |
| physics / powers | item `resting-person` | Person sitting still (body heat) (32) | Ruhig sitzender Mensch (Körperwärme) (36) | Person sitting still (20) | Ruhig sitzender Mensch (22) | yes |
| physics / powers | item `sunlight-square-metre` | Sunlight on 1 m² at noon on a clear summer day (46) | Sonnenlicht auf 1 m² an einem klaren Sommermittag (49) | Summer noon sunlight on a square metre (38) | Mittagssonne auf einem Quadratmeter (35) |  |
| physics / powers | item `kettle` | Electric kettle (15) | Wasserkocher (12) | – | – | yes |
| physics / powers | item `wallbox` | Wall box charging an electric car (33) | Wallbox beim Laden eines E-Autos (32) | – | – | yes |
| physics / powers | item `heating-load-old-house` | Design heating load of an unrenovated 1960s single-family house (63) | Heizlast eines unsanierten Einfamilienhauses der 1960er-Jahre (61) | Heating load of an unrenovated house (36) | Heizlast eines unsanierten Hauses (33) |  |
| physics / powers | item `car-engine` | Car engine at full throttle (136 PS) (36) | Automotor unter Volllast (136 PS) (33) | Car engine at full throttle (27) | Automotor unter Volllast (24) | yes |
| physics / powers | item `wind-turbine` | Modern onshore wind turbine at rated wind speed (47) | Moderne Windenergieanlage an Land bei Nennwind (46) | Onshore wind turbine at rated wind (34) | Windenergieanlage an Land bei Nennwind (38) |  |
| physics / powers | item `ice-train` | ICE 3 high-speed train at full power (36) | ICE-3-Hochgeschwindigkeitszug unter Volllast (44) | ICE 3 train at full power (25) | ICE 3 unter Volllast (20) |  |
| physics / powers | item `nuclear-unit` | Nuclear power plant unit (electrical, Isar 2) (45) | Kernkraftwerksblock (elektrisch, Isar 2) (40) | Nuclear power plant unit (24) | Kernkraftwerksblock (19) |  |
| physics / powers | item `germany-electricity` | Germany's average electricity consumption (41) | Deutschlands mittlerer Stromverbrauch (37) | Germany's average electricity use (33) | Deutschlands mittlerer Stromverbrauch (37) |  |
| physics / powers | item `world-primary-power` | Humanity's average primary energy use (37) | Mittlerer Primärenergieverbrauch der Menschheit (47) | Humanity's primary energy use (29) | Primärenergieverbrauch der Menschheit (37) |  |
| physics / powers | item `sun` | Total radiant power of the Sun (30) | Gesamte Strahlungsleistung der Sonne (36) | The Sun (7) | Sonne (5) |  |
| physics / energies | item `phone-charge` | One full smartphone charge (26) | Eine volle Smartphone-Ladung (28) | Full smartphone charge (22) | Volle Smartphone-Ladung (23) | yes |
| physics / energies | item `boil-water` | Heating 1 litre of water from 20 °C to boiling (46) | 1 Liter Wasser von 20 °C zum Kochen bringen (43) | Boiling a litre of water (24) | Aufkochen eines Liters Wasser (29) | yes |
| physics / energies | item `chocolate-bar` | Food energy of a 100 g chocolate bar (36) | Brennwert einer 100-g-Tafel Schokolade (38) | – | – | yes |
| physics / energies | item `daily-food` | Daily food energy of an adult (29) | Täglicher Nahrungsenergiebedarf eines Erwachsenen (49) | Daily food energy of an adult (29) | Tagesnahrung eines Erwachsenen (30) | yes |
| physics / energies | item `heating-oil-litre` | One litre of heating oil (24) | Ein Liter Heizöl (16) | Litre of heating oil (20) | Liter Heizöl (12) | yes |
| physics / energies | item `ev-battery` | Usable capacity of an electric car battery (42) | Nutzbare Kapazität eines E-Auto-Akkus (37) | Full electric car battery (25) | Voller E-Auto-Akku (18) | yes |
| physics / energies | item `petrol-tank` | A full 50-litre tank of petrol (30) | Eine volle 50-Liter-Tankfüllung Benzin (38) | Full tank of petrol (19) | Volle Tankfüllung Benzin (24) | yes |
| physics / energies | item `household-electricity` | Annual electricity use of a two-person household (48) | Jahresstromverbrauch eines Zwei-Personen-Haushalts (50) | Annual electricity use of two people (36) | Jahresstromverbrauch zweier Personen (36) |  |
| physics / energies | item `heating-demand-house` | Annual heating demand of an unrenovated 1960s single-family house (65) | Jahres-Heizwärmebedarf eines unsanierten Einfamilienhauses der 1960er-Jahre (75) | Annual heating demand of an old house (37) | Jahres-Heizwärmebedarf eines Altbaus (36) |  |
| physics / energies | item `wind-turbine-year` | Annual yield of a modern onshore wind turbine (45) | Jahresertrag einer modernen Windenergieanlage an Land (53) | Annual yield of a wind turbine (30) | Jahresertrag einer Windenergieanlage (36) |  |
| physics / energies | item `germany-primary-energy` | Germany's annual primary energy consumption (43) | Jährlicher Primärenergieverbrauch Deutschlands (46) | Germany's annual primary energy use (35) | Jährliche Primärenergie Deutschlands (36) |  |
| physics / energies | item `world-primary-energy` | The world's annual primary energy consumption (45) | Jährlicher Primärenergieverbrauch der Welt (42) | World's annual primary energy use (33) | Jährliche Primärenergie der Welt (32) |  |
| heating / u-values | item `single-glazing` | Single glazing (one pane of glass) (34) | Einfachverglasung (eine Glasscheibe) (36) | Single glazing (14) | Einfachverglasung (17) | yes |
| heating / u-values | item `aluminium-window-1970s` | Aluminium or steel window with double insulating glass, before 1984 (67) | Aluminium- oder Stahlfenster mit Isolierverglasung, vor 1984 (60) | Old aluminium or steel window (29) | Altes Aluminium- oder Stahlfenster (34) |  |
| heating / u-values | item `roller-shutter-box` | Uninsulated roller-shutter box, before 1995 (43) | Ungedämmter Rollladenkasten, vor 1995 (37) | Uninsulated roller-shutter box (30) | Ungedämmter Rollladenkasten (27) | yes |
| heating / u-values | item `box-type-window` | Wooden box-type or coupled window with two panes, before 1995 (61) | Holz-Kasten- oder Verbundfenster mit zwei Scheiben, vor 1995 (60) | Old wooden box-type window (26) | Altes Holz-Kastenfenster (24) |  |
| heating / u-values | item `solid-roof-1950s` | Uninsulated solid (concrete) roof, before 1958 (46) | Ungedämmtes massives Dach (Beton), vor 1958 (43) | Uninsulated concrete roof (25) | Ungedämmtes Betondach (21) |  |
| heating / u-values | item `front-door-geg` | Front door of the GEG reference building (40) | Haustür des GEG-Referenzgebäudes (32) | – | – | yes |
| heating / u-values | item `half-timbered-wall` | Half-timbered wall with clay infill, uninsulated, before 1958 (61) | Fachwerkwand mit Lehmausfachung, ungedämmt, vor 1958 (52) | Uninsulated half-timbered wall (30) | Ungedämmte Fachwerkwand (23) |  |
| heating / u-values | item `hollow-brick-wall-1970s` | Wall of perforated bricks or hollow blocks, 1969–1978 (53) | Wand aus Hochlochziegeln oder Hohlblocksteinen, 1969–1978 (57) | Perforated brick wall from the 1970s (36) | Hochlochziegelwand der 1970er-Jahre (35) |  |
| heating / u-values | item `window-passive-house` | Passive-house window (triple glazing, insulated frame) (54) | Passivhausfenster (Dreifachverglasung, gedämmter Rahmen) (56) | Passive-house window (20) | Passivhausfenster (17) |  |
| heating / u-values | item `masonry-wall-1980s` | Masonry wall, 1984–1994 (Thermal Insulation Ordinance 1982) (59) | Mauerwerkswand, 1984–1994 (Wärmeschutzverordnung 1982) (54) | Masonry wall to WSchVO 1982 (27) | Mauerwerkswand nach WSchVO 1982 (31) |  |
| heating / u-values | item `triple-glazing` | Triple insulating glass (glazing only) (38) | Dreifach-Wärmeschutzverglasung (nur Glas) (41) | Triple insulating glass (23) | Dreifach-Wärmeschutzglas (24) |  |
| heating / u-values | item `floor-geg` | Floor slab or basement ceiling of the GEG reference building (60) | Bodenplatte oder Kellerdecke des GEG-Referenzgebäudes (53) | Floor of the GEG reference building (35) | Bodenplatte des GEG-Referenzgebäudes (36) |  |
| heating / u-values | item `wall-geg` | External wall of the GEG reference building (43) | Außenwand des GEG-Referenzgebäudes (34) | Wall of the GEG reference building (34) | Außenwand des GEG-Referenzgebäudes (34) |  |
| heating / heating-load-and-demand | item `gruenderzeit-retrofit` | Gründerzeit apartment building (1860–1918) retrofitted with passive-house components (84) | Gründerzeit-Mehrfamilienhaus (1860–1918), mit Passivhaus-Komponenten saniert (76) | Retrofitted Gründerzeit building (32) | Sanierter Gründerzeitbau (24) |  |
| heating / heating-load-and-demand | item `kfw-40` | New single-family house, KfW Efficiency House 40 (48) | Neues Einfamilienhaus, KfW-Effizienzhaus 40 (43) | KfW Efficiency House 40 (23) | KfW-Effizienzhaus 40 (20) |  |
| heating / heating-load-and-demand | item `kfw-55` | New single-family house, KfW Efficiency House 55 (48) | Neues Einfamilienhaus, KfW-Effizienzhaus 55 (43) | KfW Efficiency House 55 (23) | KfW-Effizienzhaus 55 (20) |  |
| heating / heating-load-and-demand | item `geg-2024` | New single-family house to GEG 2024 with the minimum envelope and a heat pump (77) | Neues Einfamilienhaus nach GEG 2024 mit Mindest-Gebäudehülle und Wärmepumpe (75) | New house to GEG 2024 (21) | Neubau nach GEG 2024 (20) |  |
| heating / heating-load-and-demand | item `sfh-2000s` | Single-family house 2002–2009 (EnEV 2002) (41) | Einfamilienhaus 2002–2009 (EnEV 2002) (37) | House to EnEV 2002 (18) | Haus nach EnEV 2002 (19) |  |
| heating / heating-load-and-demand | item `plattenbau` | GDR large-panel apartment block 1969–1978, unrenovated (54) | DDR-Plattenbau 1969–1978, unsaniert (35) | Unrenovated GDR panel block (27) | Unsanierter DDR-Plattenbau (26) | yes |
| heating / heating-load-and-demand | item `sfh-1990s` | Single-family house 1995–2001 (WSchVO 1995) (43) | Einfamilienhaus 1995–2001 (WSchVO 1995) (39) | House to WSchVO 1995 (20) | Haus nach WSchVO 1995 (21) |  |
| heating / heating-load-and-demand | item `gruenderzeit` | Gründerzeit apartment building 1860–1918, unrenovated (53) | Gründerzeit-Mehrfamilienhaus 1860–1918, unsaniert (49) | Unrenovated Gründerzeit building (32) | Unsanierter Gründerzeitbau (26) | yes |
| heating / heating-load-and-demand | item `sfh-1970s` | Single-family house 1969–1978, unrenovated (42) | Einfamilienhaus 1969–1978, unsaniert (36) | Unrenovated 1970s house (23) | Unsaniertes Haus der 1970er-Jahre (33) |  |
| heating / heating-load-and-demand | item `sfh-1960s` | Single-family house 1958–1968, unrenovated (42) | Einfamilienhaus 1958–1968, unsaniert (36) | Unrenovated 1960s house (23) | Unsaniertes Haus der 1960er-Jahre (33) | yes |
| heating / heating-load-and-demand | quantity `heating-load` | Specific heating load (21) | Spezifische Heizlast (20) | Heating load (12) | Heizlast (8) |  |
| heating / heating-load-and-demand | quantity `heating-demand` | Annual heating demand (21) | Jahres-Heizwärmebedarf (22) | Heating demand (14) | Heizwärmebedarf (15) |  |
| cooling / air-change-rates | item `warehouse` | High-bay warehouse without workplaces (10 m high) (49) | Hochregallager ohne Arbeitsplätze (10 m hoch) (45) | High-bay warehouse (18) | Hochregallager (14) |  |
| cooling / air-change-rates | item `passive-house-dwelling` | Passive-house apartment with heat-recovery ventilation (54) | Passivhauswohnung mit Wärmerückgewinnungslüftung (48) | Passive-house apartment (23) | Passivhauswohnung (17) |  |
| cooling / air-change-rates | item `apartment` | Apartment, nominal ventilation (30) | Wohnung, Nennlüftung (20) | – | – | yes |
| cooling / air-change-rates | item `sports-hall` | Single sports hall (15 × 27 m, 5.5 m high) with 30 athletes (59) | Einfeldsporthalle (15 × 27 m, 5,5 m hoch) mit 30 Sporttreibenden (64) | Sports hall during training (27) | Sporthalle beim Training (24) | yes |
| cooling / air-change-rates | item `single-office` | Single office (12 m², one person) (33) | Einzelbüro (12 m², eine Person) (31) | Single office (13) | Einzelbüro (10) |  |
| cooling / air-change-rates | item `residential-car-park` | Underground car park of a residential building, mechanically ventilated (71) | Tiefgarage eines Wohngebäudes, maschinell belüftet (50) | Underground car park (20) | Tiefgarage (10) |  |
| cooling / air-change-rates | item `cinema` | Cinema auditorium, sold out (27) | Kinosaal, ausverkauft (21) | – | – | yes |
| cooling / air-change-rates | item `classroom` | Classroom with 28 pupils (60 m², 3 m high) (42) | Klassenzimmer mit 28 Schulkindern (60 m², 3 m hoch) (51) | Classroom during lessons (24) | Klassenzimmer im Unterricht (27) | yes |
| cooling / air-change-rates | item `restaurant` | Restaurant dining room (22) | Gastraum eines Restaurants (26) | – | – | yes |
| cooling / air-change-rates | item `cleanroom-iso-7` | Cleanroom ISO class 7 (e.g. pharmacy compounding) (49) | Reinraum ISO-Klasse 7 (z. B. Apothekenherstellung) (50) | Cleanroom ISO class 7 (21) | Reinraum ISO-Klasse 7 (21) |  |
| cooling / air-change-rates | item `cleanroom-iso-5` | Cleanroom ISO class 5 with unidirectional airflow (e.g. chip fab) (65) | Reinraum ISO-Klasse 5 mit turbulenzarmer Verdrängungsströmung (z. B. Chipfabrik) (80) | Cleanroom ISO class 5 (21) | Reinraum ISO-Klasse 5 (21) |  |
| cooling / cooling-load-and-demand | item `passive-house-home` | Passive-house home with external shading (40) | Passivhaus-Wohngebäude mit außenliegendem Sonnenschutz (54) | Passive house with shading (26) | Passivhaus mit Sonnenschutz (27) |  |
| cooling / cooling-load-and-demand | item `new-home-geg` | New home to GEG 2024 with external blinds (DIN 4108-2) (54) | Neues Wohngebäude nach GEG 2024 mit Außenjalousien (DIN 4108-2) (63) | New home with external blinds (29) | Neues Wohnhaus mit Außenjalousien (33) |  |
| cooling / cooling-load-and-demand | item `school-new-build` | New school building (GEG), cooled (33) | Neues Schulgebäude (GEG), gekühlt (33) | New school building, cooled (27) | Neues Schulgebäude, gekühlt (27) | yes |
| cooling / cooling-load-and-demand | item `office-geg-shading` | New office building (GEG) with external sun shading (51) | Neues Bürogebäude (GEG) mit außenliegendem Sonnenschutz (55) | New office with external shading (32) | Neues Büro mit Sonnenschutz (27) |  |
| cooling / cooling-load-and-demand | item `attic-flat` | Attic flat under an uninsulated roof with unshaded roof windows (63) | Dachgeschosswohnung unter ungedämmtem Dach mit unverschatteten Dachfenstern (75) | Attic flat under an uninsulated roof (36) | Dachwohnung unter ungedämmtem Dach (34) | yes |
| cooling / cooling-load-and-demand | item `hospital` | Hospital with operating rooms and intensive care (48) | Krankenhaus mit OP-Sälen und Intensivstation (44) | Hospital with operating rooms (29) | Krankenhaus mit OP-Sälen (24) |  |
| cooling / cooling-load-and-demand | item `office-1970s` | 1970s office tower, fully glazed, without external shading (58) | Bürohochhaus der 1970er-Jahre, vollverglast, ohne außenliegenden Sonnenschutz (77) | Fully glazed office tower (25) | Vollverglastes Bürohochhaus (27) |  |
| cooling / cooling-load-and-demand | item `data-centre` | Data centre (server room floor) (31) | Rechenzentrum (Serverraumfläche) (32) | Data centre (11) | Rechenzentrum (13) |  |
| cooling / cooling-load-and-demand | quantity `cooling-load` | Specific cooling load (21) | Spezifische Kühllast (20) | Cooling load (12) | Kühllast (8) |  |
| cooling / cooling-load-and-demand | quantity `cooling-demand` | Annual cooling demand (21) | Jahres-Kühlbedarf (17) | Cooling demand (14) | Kühlbedarf (10) |  |
| demand / standard-profiles | item `unrenovated-old-building` | Unrenovated old building (1960s single-family house) (52) | Unsanierter Altbau (Einfamilienhaus der 1960er-Jahre) (53) | Unrenovated old building (24) | Unsanierter Altbau (18) |  |
| demand / standard-profiles | item `wschvo-1995` | House to the Thermal Insulation Ordinance 1995 (WSchVO 1995) (60) | Haus nach Wärmeschutzverordnung 1995 (WSchVO 1995) (50) | House to WSchVO 1995 (20) | Haus nach WSchVO 1995 (21) |  |
| demand / standard-profiles | axis `cooling` | Cooling demand to keep 26 °C (28) | Kühlbedarf für 26 °C (20) | Cooling demand (14) | Kühlbedarf (10) |  |
| demand / standard-profiles | axis `ventilation` | Ventilation heat loss (air leakage plus ventilation after heat recovery) (72) | Lüftungswärmeverlust (Undichtheit plus Lüftung nach Wärmerückgewinnung) (71) | Ventilation heat loss (21) | Lüftungswärmeverlust (20) |  |
| demand / standard-profiles | axis `costs` | Net energy costs for heating, hot water and auxiliary energy (after PV credit) (78) | Netto-Energiekosten für Heizung, Warmwasser und Hilfsenergie (nach PV-Gutschrift) (81) | Net energy costs (16) | Netto-Energiekosten (19) |  |
| demand / final-energy | item `kfw-40-heat-pump` | KfW Efficiency House 40 with heat pump and heat-recovery ventilation (68) | KfW-Effizienzhaus 40 mit Wärmepumpe und Wärmerückgewinnungslüftung (66) | KfW 40 house with heat pump (27) | KfW-40-Haus mit Wärmepumpe (26) |  |
| demand / final-energy | item `deep-retrofit-heat-pump` | 1960s single-family house, deeply retrofitted with passive-house components and a heat pump (91) | Einfamilienhaus der 1960er-Jahre, tiefgreifend mit Passivhaus-Komponenten und Wärmepumpe saniert (96) | Deep-retrofitted house with heat pump (37) | Tief sanierter Altbau mit Wärmepumpe (36) |  |
| demand / final-energy | item `passive-house-direct-electric` | Passive house with direct-electric heating and hot water (56) | Passivhaus mit Direktstromheizung und elektrischer Warmwasserbereitung (70) | Passive house with electric heating (35) | Passivhaus mit Direktstromheizung (33) |  |
| demand / final-energy | item `kfw-55-gas-solar` | KfW Efficiency House 55 with gas condensing boiler and solar hot water (70) | KfW-Effizienzhaus 55 mit Gas-Brennwertkessel und Solarthermie (61) | KfW 55 house with gas and solar (31) | KfW-55-Haus mit Gas und Solarthermie (36) |  |
| demand / final-energy | item `enev-2014-gas` | New house to EnEV 2014 with gas condensing boiler (49) | Neubau nach EnEV 2014 mit Gas-Brennwertkessel (45) | EnEV 2014 house with condensing boiler (38) | EnEV-2014-Neubau mit Gas-Brennwertkessel (40) |  |
| demand / final-energy | item `old-house-heat-pump` | Unrenovated 1960s single-family house with an air-source heat pump (66) | Unsaniertes Einfamilienhaus der 1960er-Jahre mit Luft-Wärmepumpe (64) | Unrenovated house with heat pump (32) | Unsanierter Altbau mit Wärmepumpe (33) |  |
| demand / final-energy | item `wschvo-1995-gas` | Single-family house to WSchVO 1995 with low-temperature gas boiler (66) | Einfamilienhaus nach WSchVO 1995 mit Niedertemperatur-Gaskessel (63) | WSchVO 1995 house with gas boiler (33) | WSchVO-1995-Haus mit Gaskessel (30) |  |
| demand / final-energy | item `gruenderzeit-gas` | Unrenovated Gründerzeit apartment building with gas central heating (67) | Unsaniertes Gründerzeit-Mehrfamilienhaus mit Gas-Zentralheizung (63) | Gründerzeit building with gas heating (37) | Gründerzeitbau mit Gas-Zentralheizung (37) | yes |
| demand / final-energy | item `old-house-gas` | Unrenovated 1960s single-family house with old gas boiler (57) | Unsaniertes Einfamilienhaus der 1960er-Jahre mit altem Gaskessel (64) | Unrenovated house with old gas boiler (37) | Unsanierter Altbau mit altem Gaskessel (38) | yes |
