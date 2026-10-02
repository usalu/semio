# Quiz pets: a roster grounded in the quiz topics

Read-only exploration for the ticket `QUIZ-PETS` (2026-10-02). Nothing in the repo was changed apart from this file.

**Read for this report:**
- the catalog `🎓️teaching/🏛️architecture/❓️quiz/🔣️.json`
- all four topic quizzes `🎓️teaching/🏛️architecture/⚡️energy/{🧲️physics,🔥️heating,❄️cooling,📊️demand}/❓️quiz/🔣️.json`, completely, both languages
- `🎓️teaching/🏛️architecture/README.md` and `❓️quiz/README.md`
- the home layout spec `❓️quiz/🧪️tests/🥞️layered-home/🟦️.ts`
- the palette in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css` and `🖌️ui/🎨️.css`, and the quiz sheet `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🎨️.css`

Notation used throughout, one code per task: `P1 P2 P3` physics, `H1 H2` heating, `C1 C2` cooling, `D1 D2` demand (table in 1.3). An item is written `P2:sun` = task P2, item id `sun`. A dagger † after a justification means it rests on general physics, not on the text of a quiz item.

---

## 0. Summary of the proposal

- Only the four `⚡️energy` quizzes exist: 9 tasks, 109 items. No further topic folder exists or is announced (README, catalog, design.md of the quiz ticket name none).
- The proposed roster has **20 pets**: the 9 seeds plus 11 more (`windy boily roofy insuly shady venty chilly kettly flamy thermy servy`), all "thing + y".
- Every pet has quiz text behind it (section 2.1). The weakest grounding, to confirm with the owner: **cloudy** (no item names a cloud, only clear-sky and weather-averaged values), **radiatory** (named exactly once, in `H1:window-passive-house`, though the heating load that radiators deliver is the subject of all of `H2`), **thermy** (no item, but every task states temperatures) and **servy** (one item). Cloudy and radiatory are owner seeds and fit; thermy and servy are the first to drop if 18 is preferred over 20.
- Five of the proposed pets echo existing task icons: sunny = ☀️ (P2), battery = 🔋 (P3), waly = 🧱 (H1), venty = 🌬️ (C1), chilly = ❄️ (C2); and flamy is the first word of the P2 title "From Tea Light to Sun".
- Cast model: each quiz has a troupe of 7 (3 core, 4 rotation); at most 5 are visible at once (3 on phones); home shows the 9 seeds; the other 11 visit their topic card.
- Social graph: 67 non-neutral pairs of 190, each with a physical one-line reason (section 4). Headline pairs: sunny+solary (+0.9), pumpy+radiatory (+0.9), waly+insuly (+0.9), solary vs cloudy (-0.7), chilly vs radiatory (-0.7), windowy vs waly (-0.5).
- Colours: 20 body colours chosen so each has at least 3:1 contrast against both the light base `#f7f3e3` and the dark base `#001117` (sunny is the one exception); every pet still needs a 2 px outline in `--foreground` because contrast against the glass panels drops to between 2.0:1 and 4.3:1.

---

## 1. Topics and inventory

### 1.1 The tree (three levels deep)

```
🎓️teaching/
├── README.md
├── 🏛️architecture/                       the site quizzes.architektur-und-technologie.de
│   ├── README.md
│   ├── ⚡️energy/                          the only topic domain
│   │   ├── ❄️cooling/❓️quiz/🔣️.json        quiz "cooling"
│   │   ├── 📊️demand/❓️quiz/🔣️.json         quiz "demand"
│   │   ├── 🔥️heating/❓️quiz/🔣️.json        quiz "heating"
│   │   └── 🧲️physics/❓️quiz/🔣️.json        quiz "physics"
│   └── ❓️quiz/                            the site, not a topic
│       ├── 🔣️.json                        the catalog (id architecture)
│       ├── 🟦️.ts  🌐️.html  🎨️.css          entry, document, brand layer
│       ├── 🏗️builder  📦️packages  🚀️deploy  🧱️stack  🎭️e2e  🧪️tests
│       └── README.md
└── 🛂️proctor/                            the API server, no topics
```

No other topic is planned in the repo text I read: the catalog lists exactly the four quizzes, the README tables list four, and neither the README nor `design.md` of `QUIZ-PRODUCT-AND-TEACHING-PROCTOR` mentions a fifth. The domain folder is named `⚡️energy`, so more domains or topics may follow; the roster below should therefore be data (section 7), not code.

### 1.2 Catalog and quizzes

Catalog `architecture`: "Architecture and Technology Quizzes" / "Quizze zu Architektur und Technologie", introduction in 5 paragraphs, 7 badges (`physics-expert` 🧲, `heating-expert` 🔥, `cooling-expert` ❄️, `demand-expert` 📊, `numerical-brain` 🧮, `pattern-seer` 🔍, `completionist` 🏁). Quiz order in the catalog: physics, heating, cooling, demand.

| id | emoji | title en | title de | tasks | items | drawn per run |
|---|---|---|---|---:|---:|---|
| `physics` | 🧲 | Physical Understanding | Physikalisches Verständnis | 3 | 42 (16 + 14 + 12) | 12 + 10 + 9 |
| `heating` | 🔥 | Heating | Heizen | 2 | 28 (17 + 11) | 10 + 8 |
| `cooling` | ❄️ | Cooling | Kühlen | 2 | 24 (15 + 9) | 10 + 7 |
| `demand` | 📊 | Energy Demand | Energiebedarf | 2 | 15 (6 + 9) | 6 (all) + 7 |

Total: 9 tasks, 109 items (counts verified against the `"explanation"` count per file: 42, 28, 24, 15).

### 1.3 Tasks

| code | task id | kind | title en | title de | icon, motion | items (drawn) | quantity or axes |
|---|---|---|---|---|---|---:|---|
| P1 | `power-or-energy` | classification | Power or Energy? | Leistung oder Energie? | ⚡ pulse | 16 (12) | categories `power`, `energy` |
| P2 | `powers` | sorting | From Tea Light to Sun | Vom Teelicht bis zur Sonne | ☀️ spin | 14 (10) | power, W, logarithmic, 25 decades |
| P3 | `energies` | sorting | From Phone Charge to World Energy Use | Von der Handyladung zum Weltenergieverbrauch | 🔋 bounce | 12 (9) | energy, Wh, logarithmic, 16 decades |
| H1 | `u-values` | matching | U-values of Building Components | U-Werte von Bauteilen | 🧱 flip | 17 (10) | U-value, W/(m²·K) |
| H2 | `heating-load-and-demand` | matching, 2 dims | Heating Load and Heating Demand | Heizlast und Heizwärmebedarf | 🔥 float | 11 (8) | W/m² and kWh/(m²·a) |
| C1 | `air-change-rates` | matching | Air Change Rates | Luftwechselraten | 🌬️ sway | 15 (10) | 1/h |
| C2 | `cooling-load-and-demand` | matching, 2 dims | Cooling Load and Cooling Demand | Kühllast und Kühlbedarf | ❄️ spin | 9 (7) | W/m² and kWh/(m²·a) |
| D1 | `standard-profiles` | classification with spider diagram | Energy Standards and Their Profiles | Energiestandards und ihre Profile | 🕸️ sway | 6 (all) | 6 categories `profile-a..f`, 4 axes: heating, cooling, ventilation, net energy costs |
| D2 | `final-energy` | matching | Final Energy Demand | Endenergiebedarf | 🔌 bounce | 9 (7) | kWh/(m²·a) |

### 1.4 Where pets will stand: the home page

Desktop home is a 3 x 3 grid (column weights 1 : 1.5 : 1, row weights 1 : 1.4 : 1) of nine cards, DOM order `learner, physics, intro, heating, board, cooling, badges, demand, prefs`. The quizzes sit on a compass around the leaderboard: physics north, heating west, cooling east, demand south. That is a gift for the cast: radiatory (west) and chilly (east) can bicker across the leaderboard. Below 768 px home is a list of sections. Other pages: introduction, one quiz page, the three task kinds, results, badges, leaderboard, preferences (modules `🏠️home 👋️introduction 📖️quiz-page 🧩️task 🏁️results 🏅️badges 🏆️leaderboard 🎛️preferences`).

### 1.5 Inventory: every concrete thing or concept, with where it occurs

"Pet" names the roster entry that embodies it (section 2). Items in `code:item-id` form.

**Sources, weather, fuels**

| Thing or concept | Pet | Occurs in |
|---|---|---|
| Sun, sunlight, solar constant | sunny | P2:sun, P2:sunlight-on-earth, P2:sunlight-square-metre; P3:world-primary-energy (explanation: the Sun delivers it in about an hour); P1:pv-module-peak (1,000 W/m²); solar gains in C2:passive-house-home, new-home-geg, office-geg-shading, attic-flat, office-1970s; D1:plus-energy-house. Task icon ☀️ of P2 |
| Cloud, overcast | cloudy | named by no item. Indirect: "clear summer day" P2:sunlight-square-metre; weather-averaged 950 kWh/kWp in P1:pv-annual-yield; "hot design day" and "hottest weeks" in C2 (school-new-build); solar gains in C2 |
| Wind, wind turbine | windy | P2:wind-turbine (5 MW), P3:wind-turbine-year (10 GWh, about 2,000 full-load hours), P2:ice-train (explanation: as much as 1.6 turbines); source Deutsche WindGuard |
| Water, steam, hot water | kettly, boily | P3:boil-water (1 l from 20 °C to boiling, 93 Wh); hot water 12.5 kWh/(m²·a) in every D2 item |
| Ground, environment heat | pumpy | H1:floor-geg (ground warmer than winter air); D2:kfw-40-heat-pump (more than two thirds of the heat from the environment) |
| Fuels: heating oil, petrol, gas | boily | P1:heating-oil-litre, P3:heating-oil-litre, P3:petrol-tank, gas in all D2 gas items and D1 |
| Flame, tea light | flamy | P1:tea-light, P2:tea-light |
| Nuclear unit, grid, world energy | (none) | P1:nuclear-unit, P2:nuclear-unit, P2:germany-electricity, P2:world-primary-power, P3:germany-primary-energy, P3:world-primary-energy |

**Building and envelope**

| Thing or concept | Pet | Occurs in |
|---|---|---|
| House, building, apartment block | housy | all of H2 (11); all of D1 (6) and D2 (9); P1:heating-load, P1:heating-demand-house, P1:energy-certificate; P2:heating-load-old-house; P3:heating-demand-house; C2:passive-house-home, new-home-geg, school-new-build, office-*, hospital, attic-flat; C1:apartment, passive-house-dwelling |
| Window, glazing | windowy | H1:single-glazing, aluminium-window-1970s, box-type-window, window-geg, window-passive-house, triple-glazing; C1:classroom (window ventilation struggles at 4.8 1/h); C2:attic-flat (roof windows), office-1970s (fully glazed); D1:plus-energy-house (larger glazing) |
| Wall | waly | H1:half-timbered-wall, hollow-brick-wall-1970s, masonry-wall-1980s, wall-geg, wall-passive-house; H2:plattenbau (concrete sandwich panels), gruenderzeit (thick brick walls, party walls). Task icon 🧱 of H1 |
| Roof | roofy | H1:solid-roof-1950s, roof-geg, roof-passive-house; C2:attic-flat (roof skin 60 to 70 °C); D1:unrenovated-old-building (uninsulated roof heats up in summer); P1:pv-annual-yield (rooftop system) |
| Insulation | insuly | thickness named in H1:wall-geg (12 cm), roof-geg (17 cm), wall-passive-house (24 cm), roof-passive-house (35 cm); absent in H1:hollow-brick-wall-1970s, solid-roof-1950s; thin in H1:masonry-wall-1980s; H2:gruenderzeit-retrofit, kfw-40, kfw-55; D2:deep-retrofit-heat-pump, old-house-heat-pump ("even without insulation") |
| Floor slab, basement ceiling | (none) | H1:floor-geg |
| Door | (none) | H1:front-door-geg |
| Roller-shutter box | shady | H1:roller-shutter-box (3.6 W/(m²·K)) |
| External shading, blinds | shady | C2:passive-house-home, new-home-geg, office-passive-house, office-geg-shading, attic-flat (unshaded), office-1970s (without); D1:passive-house ("planned shading") |
| Ventilation, heat recovery, fans, air leakage | venty | all 15 items of C1; C2:office-passive-house (night ventilation); D1 ventilation axis, kfw-40, passive-house (at least 75 % recovery), enev-2014 (without recovery); D2:kfw-40-heat-pump, passive-house-direct-electric; H2:gruenderzeit-retrofit, passive-house |
| Cleanroom filters (HEPA) | venty | C1:cleanroom-iso-7, cleanroom-iso-6, cleanroom-iso-5 |

**Building services and energy devices**

| Thing or concept | Pet | Occurs in |
|---|---|---|
| Heat pump | pumpy | D2:kfw-40-heat-pump, deep-retrofit-heat-pump, old-house-heat-pump; D1:kfw-40, passive-house (compact unit), plus-energy-house; H2:geg-2024 |
| Boiler (gas, condensing, low-temperature, oil) | boily | D2:kfw-55-gas-solar, enev-2014-gas, wschvo-1995-gas, gruenderzeit-gas, old-house-gas; D1:unrenovated-old-building, wschvo-1995, enev-2014 |
| Radiator, heating load | radiatory | named once: H1:window-passive-house ("so that no radiator is needed below the window"); subject of H2 (10 to 160 W/m²), P1:heating-load, P2:heating-load-old-house (18 kW) |
| PV module and system | solary | P1:pv-module-peak, P1:pv-annual-yield; P2:sunlight-square-metre (module with 20 % efficiency); D1:plus-energy-house and the "after PV credit" cost axis of D1 |
| Solar thermal collectors | solary (outfit) | D2:kfw-55-gas-solar |
| Battery, accumulator | battery | P1:ev-battery, P1:phone-charge, P1:wallbox (fills the car battery), P3:ev-battery, P3:phone-charge, P3:petrol-tank (seven times the car battery). Task icon 🔋 of P3 |
| Chiller, cooling machine | chilly | C2:passive-house-home ("without a chiller"), school-new-build ("cooled"), hospital, office-1970s, data-centre. Task icon ❄️ of C2 |
| Kettle | kettly | P1:kettle, P2:kettle, P3:boil-water (kettle needs about 3 minutes), P2:heating-load-old-house ("nine kettles") |
| Server, data centre | servy | C2:data-centre (1,000 W/m², 8,000 kWh/(m²·a)) |
| Thermometer (temperatures) | thermy | 20 °C inside, -12 °C outside, 32 K in the H2 prompt and explanations; per kelvin in H1; 26 °C in the C2 prompt and the D1 cooling axis; 25 °C overheating limit in C2:passive-house-home; 60 to 70 °C in C2:attic-flat; 20 °C to boiling in P3:boil-water; 25 °C test condition in P1:pv-module-peak |
| Car, wall box, train | (none) | P1:car-engine, P1:wallbox, P2:wallbox, P2:car-engine, P2:ice-train, P3:petrol-tank |
| Person, food, phone | (none) | P1:resting-person, P2:resting-person, P1:chocolate-bar, P3:chocolate-bar, P3:daily-food; people as heat loads in C1 (single-office, sports-hall, cinema, classroom, restaurant) and C2 (school-new-build, office-geg-shading). The learner is the only human in the cast. |
| Household electricity, energy certificate | (none) | P1:household-electricity, P3:household-electricity, P1:energy-certificate |

Items with no pet reaction at all (system-level or human): P1: resting-person, nuclear-unit, car-engine, household-electricity, chocolate-bar. P2: resting-person, car-engine, nuclear-unit, germany-electricity, world-primary-power. P3: chocolate-bar, daily-food, household-electricity, germany-primary-energy. That is 14 of 109 items; Appendix A lists the reaction pets for all 109.

---

## 2. Proposed roster: 20 pets

Naming: owner's nine kept exactly; the eleven additions follow thing + "y" (`wind` turbine → windy, `boil`er → boily, `roof` → roofy, `insul`ation → insuly, `shad`ing → shady, `vent`ilation → venty, `chill`er → chilly, `kettl`e → kettly, `flam`e → flamy, `therm`ometer → thermy, `serv`er → servy). Tier 1 = owner seeds, tier 2 = strongly grounded (three or more items or a whole task), tier 3 = narrower but memorable.

Accessible names: pattern `<Nickname>, the <thing>` / `<Nickname>, <article> <Ding>`. German nouns follow the German texts of the quizzes ("Akku", "Wärmepumpe", "Windenergieanlage", "Sonnenschutz", "Kältemaschine", "Teelicht", "Wasserkocher", "Rechenzentrum"). Icon column: a suggestion for the monochrome Noto Emoji glyph (the site ships the monochrome Noto Emoji face and sets `font-variant-emoji: text`; colour emoji must not be used).

| # | id | tier | accessible name en | accessible name de | thing | Ding | icon | quizzes (core ● / rotation ○ / spotlight ▫) |
|---:|---|---|---|---|---|---|---|---|
| 1 | `sunny` | 1 | Sunny, the sun | Sunny, die Sonne | sun | die Sonne | ☀️ | physics ●, cooling ○ |
| 2 | `cloudy` | 1 | Cloudy, the cloud | Cloudy, die Wolke | cloud | die Wolke | ☁️ | physics ○, cooling ○ |
| 3 | `housy` | 1 | Housy, the house | Housy, das Haus | house | das Haus | 🏠 | heating ○, demand ○ |
| 4 | `solary` | 1 | Solary, the solar panel | Solary, das Solarmodul | PV panel | das Solarmodul | 🔆 | demand ●, physics ○ |
| 5 | `radiatory` | 1 | Radiatory, the radiator | Radiatory, der Heizkörper | radiator | der Heizkörper | ♨️ | heating ●, demand ○ |
| 6 | `pumpy` | 1 | Pumpy, the heat pump | Pumpy, die Wärmepumpe | heat pump | die Wärmepumpe | 🌀 | demand ●, heating ▫ |
| 7 | `windowy` | 1 | Windowy, the window | Windowy, das Fenster | window | das Fenster | 🪟 | heating ●, cooling ▫ |
| 8 | `waly` | 1 | Waly, the wall | Waly, die Wand | wall | die Wand | 🧱 | heating ● |
| 9 | `battery` | 1 | Battery, the battery | Battery, der Akku | battery | der Akku (die Batterie) | 🔋 | physics ● |
| 10 | `windy` | 2 | Windy, the wind turbine | Windy, die Windenergieanlage | wind turbine | die Windenergieanlage (das Windrad) | 💨 | physics ○ |
| 11 | `boily` | 2 | Boily, the boiler | Boily, der Heizkessel | gas or oil boiler | der Heizkessel (Gaskessel) | 🔥 | demand ●, physics ▫ |
| 12 | `roofy` | 2 | Roofy, the roof | Roofy, das Dach | roof | das Dach | 🛖 | heating ○, cooling ▫ |
| 13 | `insuly` | 2 | Insuly, the insulation | Insuly, die Dämmung | thermal insulation | die Dämmung | 🧶 | heating ○, demand ○ |
| 14 | `shady` | 2 | Shady, the external sun shading | Shady, der Sonnenschutz | external blind or shutter | der Sonnenschutz (die Jalousie) | 😎 | cooling ●, heating ▫ |
| 15 | `venty` | 2 | Venty, the ventilation unit | Venty, das Lüftungsgerät | heat-recovery ventilation unit | das Lüftungsgerät | 🌬️ | cooling ●, demand ○ |
| 16 | `chilly` | 2 | Chilly, the chiller | Chilly, die Kältemaschine | chiller | die Kältemaschine | ❄️ | cooling ● |
| 17 | `kettly` | 3 | Kettly, the kettle | Kettly, der Wasserkocher | electric kettle | der Wasserkocher | 🫖 | physics ●, cooling ▫ |
| 18 | `flamy` | 3 | Flamy, the tea light | Flamy, das Teelicht | tea light with flame | das Teelicht | 🕯️ | physics ○ |
| 19 | `thermy` | 3 | Thermy, the thermometer | Thermy, das Thermometer | thermometer | das Thermometer | 🌡️ | heating ○, cooling ○ |
| 20 | `servy` | 3 | Servy, the server rack | Servy, das Rechenzentrum | server rack in a data centre | das Rechenzentrum (der Serverschrank) | 🖥️ | cooling ○ |

### 2.1 Why each pet belongs: the justifying items

Trigger items: when one of these is drawn, the pet may appear as a guest or react to it, even if it is not in the quiz's visible cast.

1. **sunny** (☀️, tier 1). Subject of the top of P2: `sun` (3.8×10²⁶ W), `sunlight-on-earth`, `sunlight-square-metre`; `P3:world-primary-energy`; the 1,000 W/m² behind `P1:pv-module-peak`; every solar gain in C2. Topics: physics, cooling.
2. **cloudy** (☁️, tier 1). Indirect only (see 1.5). `P2:sunlight-square-metre` says "clear summer day", which implies its opposite; `P1:pv-annual-yield` (950 kWh/kWp) is a yield averaged over weather; C2 speaks of hot design days and the hottest weeks. Honest rating: weakest textual grounding of the 20, kept because the owner wants it and it is the natural foil for sunny, solary and shady.
3. **housy** (🏠, tier 1). The building itself: all of H2, D1 and D2 are buildings; `P1:heating-load`, `P1:heating-demand-house`, `P1:energy-certificate`, `P2:heating-load-old-house`, `P3:heating-demand-house`.
4. **solary** (🔆, tier 1). `P1:pv-module-peak`, `P1:pv-annual-yield`, `P2:sunlight-square-metre` (20 % efficiency), `D1:plus-energy-house` (PV roof, negative net costs). Outfit variant: orange collector tubes for `D2:kfw-55-gas-solar` (solar hot water).
5. **radiatory** (♨️, tier 1). `H1:window-passive-house` is the only item that names a radiator ("no radiator is needed below the window"). Conceptually it is the receiver of every heating load: H2 (10 to 160 W/m²), `P1:heating-load`, `P2:heating-load-old-house`.
6. **pumpy** (🌀, tier 1). Directly named in `D2:kfw-40-heat-pump`, `deep-retrofit-heat-pump`, `old-house-heat-pump`; `D1:kfw-40`, `passive-house`, `plus-energy-house`; `H2:geg-2024`. Signature number: seasonal performance factor 2.7 to 3.5.
7. **windowy** (🪟, tier 1). Six of the 17 H1 items are windows or glazing; `C1:classroom` (window ventilation); `C2:attic-flat`, `C2:office-1970s`; `D1:plus-energy-house` (larger glazing raises cooling).
8. **waly** (🧱, tier 1). Five H1 walls spanning U 1.5 down to 0.15 W/(m²·K), `H2:plattenbau`, `H2:gruenderzeit`. Its icon 🧱 is already the H1 task icon.
9. **battery** (🔋, tier 1). `P1:ev-battery`, `P3:ev-battery`, `P1:phone-charge`, `P3:phone-charge`, `P1:wallbox`, `P3:petrol-tank` (seven times the car battery). Its icon 🔋 is already the P3 task icon.
10. **windy** (💨, tier 2). `P2:wind-turbine`, `P3:wind-turbine-year`, `P2:ice-train` (1.6 turbines). Two full items plus a source in the README. Could be dropped without hurting topic fit, but the weather trio sunny, cloudy, windy is the strongest comic ensemble.
11. **boily** (🔥, tier 2). Five D2 items and three D1 items are about boilers (generation efficiencies 0.8 to 0.9, hot water as low as 0.5); `P1/P3:heating-oil-litre`.
12. **roofy** (🛖, tier 2). `H1:solid-roof-1950s`, `roof-geg`, `roof-passive-house`; `C2:attic-flat`; `D1:unrenovated-old-building`; carrier of the PV in `D1:plus-energy-house` and `P1:pv-annual-yield`.
13. **insuly** (🧶, tier 2). The cause of nearly every low value in H1 and the whole difference between the rows of H2 and D2 (303 → 54 kWh/(m²·a) in `D2:deep-retrofit-heat-pump`).
14. **shady** (😎, tier 2). Six C2 items turn on external shading, `H1:roller-shutter-box`, `D1:passive-house` ("planned shading").
15. **venty** (🌬️, tier 2). The whole of C1 (15 items), the ventilation axis of D1, heat recovery in H2 and D2.
16. **chilly** (❄️, tier 2). "Without a chiller" (`C2:passive-house-home`), "cooled" (`C2:school-new-build`), `C2:hospital`, `C2:data-centre`, `C2:office-1970s`; its icon ❄️ is the quiz emoji of cooling.
17. **kettly** (🫖, tier 3). `P1:kettle` (power), `P2:kettle`, `P3:boil-water`, `P2:heating-load-old-house` ("nine kettles"). Embodies the central physics idea: a rate, not an amount.
18. **flamy** (🕯️, tier 3). `P1:tea-light` and `P2:tea-light`: the smallest power in the sorting task, and the title of P2 is "From Tea Light to Sun" (flamy and sunny are the two ends of the task).
19. **thermy** (🌡️, tier 3). No item is a thermometer, but every task states temperatures: 20 °C and -12 °C (H2), 26 °C and 25 °C (C2, D1), 60 to 70 °C roof skin, 20 °C to boiling. Abstract but unifying.
20. **servy** (🖥️, tier 3). One item, `C2:data-centre`, but the most extreme one (1,000 W/m², 8,000 kWh/(m²·a)); a run draws 7 of the 9 items, so it appears in about 78 % of runs.

### 2.2 Considered and left out

| Candidate | Why not |
|---|---|
| `doory` (front door) | one item, `H1:front-door-geg`; waly and windowy cover openings |
| `floory` (floor slab) | one item, `H1:floor-geg`; can be a prop of housy |
| `cary` (electric car, wall box) | four items (P1/P2/P3) but off-topic for architecture; battery and kettly carry the energy ideas |
| `tappy` (hot-water tap) | hot water is a number (12.5 kWh/(m²·a)) in D2, never a drawn thing; candidate for a later topic |
| `droppy` (water drop) | cloudy's raindrop covers it |
| `icy` (ice storage), `pipy` (pipe), `fanny` (fan) | not in any quiz; `fanny` also has an unfortunate English reading; the fan is venty's and pumpy's part |
| `personny` | the learner is the person; body heat (100 W) is a prop for housy and thermy |
| `lampy`, `meter` | lighting and meters are mentioned once, in passing |

Name notes for the owner: `windy` and `windowy` differ by two letters and sit on the same screens; the silhouettes (tall rotor tower against a window frame) are what keep them apart, and the accessible names (turbine, window) do the rest. `shady` has an English second meaning ("dubious"); it is a deliberate pun for a pet in sunglasses, and `blindy` is the plain alternative. `chilly`, `sunny`, `windy`, `shady` are also adjectives in English, which is a feature.

---

## 3. Character sheets

### 3.1 Shared rig

Every pet shares one face and one set of behaviour channels, so animation logic is written once.

- Common bones: `root` (ground contact point and shadow), `body`, `face` (child of body), `eyeL` `eyeR` (sclera `#f7f3e3`, pupils follow the cursor when the pet stands still), `lidL` `lidR` (blink every 3 to 6 seconds with jitter), `mouth` (shapes: flat, smile, grin, o, frown), optional `cheekL` `cheekR`.
- Footing: pets walk on top of UI elements, so each has a flat or point `foot line` at y = 0; hovering pets (sunny, cloudy, venty) keep a shadow ellipse that shrinks with height.
- Sizes are in rig units where a typical pet fits a 64 x 64 box: S ≈ 40 tall, M ≈ 52, L ≈ 64, XL ≈ 90 (only windy exceeds the box clearly). Widths are given in the silhouettes.
- Rig classes, to be built in this order for reuse: **Biped** (stubby arms, two legs: housy, solary, radiatory, windowy, waly, battery, boily, roofy, kettly, thermy), **Hover** (no legs: sunny, cloudy, venty), **Slider/Roller** (shady, chilly, servy, insuly), **Quadruped trotter** (pumpy), **Hopper on a base** (flamy), **Tiptoer** (windy).
- All fidgets run only when the learner neither prefers reduced motion nor switched the animation off (`data-icon-motion="off"` is the existing switch for task icons; reuse the same preference); forced-colours mode draws outline only.

### 3.2 The sheets

**1. sunny** (hover, M)
- Silhouette: circle of radius 24 with 12 rounded triangular rays in two rings (6 long, 6 short, alternating), round face with two big eyes and wide smile.
- Bones: `root` (hover point), `body`, `rayRingA` and `rayRingB` (each 6 child ray bones with their own scale), `face`, `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `cheekL`, `cheekR`, `glow` (soft halo).
- Locomotion: hovers 10 units above the foot line with a slow sine bob and glides sideways; no steps; shadow shrinks when high.
- Fidgets: (1) rays pulse in a travelling wave; (2) the ray rings counter-rotate slowly; (3) squints and pops tiny shades when a bright item (`P2:sun`) is hovered, and sulks (rays droop, glow dims) when cloudy is in front.
- Temperament: warm, confident, a little vain, playful; sulks quickly, forgives quickly.

**2. cloudy** (hover, M wide)
- Silhouette: cumulus with a flat belly and three overlapping lobes (radii 14, 18, 13), about 64 wide, a small wisp at one end.
- Bones: `root`, `body`, `lobeL`, `lobeM`, `lobeR` (squash and stretch, springy), `face` on `lobeM`, `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `cheekL`, `cheekR` (puff up to blow), `dripPoint` (raindrop emitter), `wisp`.
- Locomotion: drifts at 14 units height, slow horizontal glide with elastic squash at start and stop, wobbling when windy spins.
- Fidgets: (1) drips one raindrop that falls, splashes and turns teal; (2) lobes breathe and rearrange; (3) puffs both cheeks and blows a gust (windy's blades speed up, flamy flinches).
- Temperament: cheeky, teasing, daydreamy; likes to drift in front of sunny and pretends it was an accident.

**3. housy** (biped, L)
- Silhouette: wall block 40 wide, 34 high, pitched roof overhanging by 6, chimney on the right, a door in the lower middle that doubles as the mouth; two short legs, two stubby arms at the wall corners.
- Bones: `root`, `body` (walls), `roof` (hinged at the eaves, can tip like a hat), `chimney` (smoke emitter), `door` (opens as mouth), `eyeL`, `eyeR` (set under the eaves, not windows, so windowy stays unique), `lidL`, `lidR`, `armL`, `armR`, `legL`, `legR`, `coat` (optional insulation layer shown when insuly has wrapped it).
- Locomotion: waddles on two short legs; roof bounces a little on each step; never runs.
- Fidgets: (1) chimney blows a smoke ring; (2) roof tips like a hat to greet a visitor pet; (3) shivers and clutches its walls when "old house" items are drawn, sighs cosily when wrapped by insuly.
- Temperament: kind, sheltering host, a worrier about draughts; calm.

**4. solary** (biped, M)
- Silhouette: rounded rectangle panel 36 wide, 48 high with a 2 column by 3 row cell grid and a light aluminium frame, on a hinge above two short legs; eyes sit in the two top cells.
- Bones: `root`, `hips`, `panel` (tilt hinge, ±35°), `cells[6]` (each can light up), `frame`, `glint` (sweeps over the glass), `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `armL`, `armR`, `legL`, `legR`.
- Locomotion: walks on two short legs; the panel keeps tilting toward sunny while moving, like a sunflower; hops over gaps.
- Fidgets: (1) tilts toward sunny's position and angle; (2) cells light up one by one (charging sparkle); (3) shivers and dims all cells when cloudy covers sunny, perks up when the cloud passes.
- Temperament: eager, devoted, dramatic; shy in the dark.

**5. radiatory** (biped, S wide)
- Silhouette: wide panel radiator 52 wide, 38 high with 6 vertical ribbed fins, a thermostat knob top right and two pipe stubs at the lower corners that serve as feet; face on the middle fins.
- Bones: `root`, `body`, `fins[6]` (each with a phase for a ripple), `knob` (rotates), `pipeL`, `pipeR` (feet), `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `armL`, `armR` (thin pipes).
- Locomotion: shuffles on the pipe stubs in short stiff steps, shoulders rocking.
- Fidgets: (1) fins ripple with a heat shimmer (colour passes from sand to coral); (2) turns its own knob up and down; (3) gurgles, a bubble pops from the top (bleeding air), then offers a warm hug with both arms.
- Temperament: warm, cuddly, loyal, sleepy; huffy when it has to work against open windows or chilly.

**6. pumpy** (quadruped trotter, M)
- Silhouette: rounded square outdoor unit 40 x 40 with a big circular fan grille (radius 14) on the front, side louvres, four stubby feet, two refrigerant pipes at the back as a tail; eyes above the grille.
- Bones: `root`, `body`, `fan` (spin), `grille`, `louvreL`, `louvreR`, `pipeTail` (2 segments), `feetFL`, `feetFR`, `feetBL`, `feetBR`, `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`.
- Locomotion: trots on four feet with a gentle rocking; the fan speed follows its activity.
- Fidgets: (1) fan spins up and down; (2) breathes in cool air on the left and out warm air on the right with coloured streaks; (3) hums and vibrates, then does a small proud hop (3 kWh of heat per kWh bought).
- Temperament: calm, hard-working, efficient, an introvert who lights up around radiatory.

**7. windowy** (biped, L)
- Silhouette: window frame 34 wide, 46 high with two casement sashes, a cross of muntins, a diagonal glint on the glass, a sill ledge as base with two small feet; the lids are tiny curtains.
- Bones: `root`, `frame` (body), `sashL`, `sashR` (hinged, act as arms), `panes`, `glint`, `curtainL`, `curtainR` (lids), `eyeL`, `eyeR`, `mouth` (on the lower pane), `sill`, `footL`, `footR`.
- Locomotion: hops on the two feet, sashes flapping slightly; sidesteps when something big passes.
- Fidgets: (1) opens a sash, the curtain flutters, closes it again with a shiver; (2) fogs its pane with breath and draws a small face or a digit in the condensation; (3) glint sweeps across the glass when someone is looking at it.
- Temperament: curious, chatty, sensitive to draughts, a bit of a show-off (it is transparent and cannot hide its feelings).

**8. waly** (biped, M wide)
- Silhouette: stout slab 44 wide, 40 high with four staggered brick courses, a flat cap on top, short sturdy legs, small arms.
- Bones: `root`, `body`, `courses[4]` (each can jiggle sideways), `cap`, `coat` (plaster or insulation layer), `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `armL`, `armR`, `legL`, `legR`.
- Locomotion: marches with heavy steps and a small ground-tap squash; never hops; slow to start, slow to stop.
- Fidgets: (1) a brick wiggles loose and is pushed back in; (2) knocks on itself and listens; (3) crosses its arms stubbornly when windowy talks, puffs a little dust.
- Temperament: stoic, stubborn, steady, patient; proud of its mass.

**9. battery** (biped, M)
- Silhouette: AA-style capsule 28 wide, 52 high with a terminal nub on top (cap) and a window of 4 charge bars on the lower half; eyes above the bars.
- Bones: `root`, `body`, `cap` (tilts like a hat), `bars[4]` (scale and colour), `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `armL`, `armR`, `legL`, `legR`, `spark` (emitter on the cap).
- Locomotion: bounces upright with a spring; slumps and plods when bars are low.
- Fidgets: (1) charge bars fill one by one and drain again; (2) a tiny spark jumps from the cap; (3) yawns and sags when drained, springs back when solary or windy is near.
- Temperament: energetic, restless, generous, playful; sleepy when empty.

**10. windy** (tiptoer, XL)
- Silhouette: slender tapered tower (10 wide at the base, 6 at the top, 64 high) carrying a round hub with the face and three blades of length 28; two small feet.
- Bones: `root`, `tower` (sway), `nacelle` (yaw), `hub` (face), `rotor` (spin), `blade[3]`, `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `footL`, `footR`.
- Locomotion: tiptoes with small steps, swaying; leans into a gust and skids when cloudy blows.
- Fidgets: (1) rotor spins, speed proportional to a wind variable (driven by cloudy's gusts); (2) stops the rotor to scratch its head or yawns in a calm; (3) bends the tower and yaws the nacelle toward a gust.
- Temperament: lively, airy, daydreaming; restless in calm, euphoric in a storm.

**11. boily** (biped, M)
- Silhouette: round-shouldered standing boiler 40 wide, 50 high with a round flame window on the lower front, a pressure gauge dial upper right, a flue pipe on top, two pipes at the base as feet.
- Bones: `root`, `body`, `flue` (smoke), `gauge` and `needle`, `flameWindow` (flame sprite with flicker), `pipeL`, `pipeR` (feet), `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `armL`, `armR`.
- Locomotion: plods on the pipe feet with a low rumble; each step rattles the flue.
- Fidgets: (1) the flame flickers orange or blue; (2) the gauge needle wobbles and the flue puffs a smoke ring; (3) burps a tiny flame ("whoomp") and mutters.
- Temperament: grumpy veteran, set in his ways, easily overheated, secretly soft; nostalgic about old standards (0.8 efficiency).

**12. roofy** (biped, S wide)
- Silhouette: gable roof like a tent or a hat, 56 wide, 34 high, eaves overhang, ridge cap, a chimney stub and a dormer window as a third "eye" (decorative); two thin legs under the eaves, eave tips as hands.
- Bones: `root`, `roof` (planes L and R), `ridge`, `tiles` (pattern), `chimney`, `dormer`, `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `legL`, `legR`, `eaveL`, `eaveR` (arms).
- Locomotion: scampers on two thin legs like a hat with legs; can tip itself over a pet to shelter it.
- Fidgets: (1) tiles ripple like fish scales; (2) tips itself like a hat; (3) sweats drops in sun and shivers with a tiny snow cap when cold.
- Temperament: protective, a little shy, proud; sensitive to sunny's heat.

**13. insuly** (roller, S)
- Silhouette: pink fluffy mineral-wool batt, a soft rounded rectangle 44 x 36 with a scalloped, fibrous outline and a thin foil-faced strip on one side; tiny tuft arms and feet.
- Bones: `root`, `body` (soft body, 8 perimeter puff bones on springs), `facing` (strip), `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `tuftArmL`, `tuftArmR`, `tuftFootL`, `tuftFootR`, `blanket` (unrolls to wrap a neighbour).
- Locomotion: squishy hop, or rolls up like a bedroll and rolls forward; slow.
- Fidgets: (1) puffs expand and contract (breathing); (2) unrolls over waly, roofy or housy and pats the layer flat; (3) fibres bristle when wet (cloudy near), then it fluffs itself dry.
- Temperament: gentle, motherly, sleepy; dislikes getting wet.

**14. shady** (slider, M)
- Silhouette: external blind 40 x 40: a box-shaped headrail on top (the roller-shutter box) and six horizontal slats below, a pull cord with a tassel, sunglasses laid over two slat gaps.
- Bones: `root`, `headrail`, `slats[6]` (each tiltable), `cord` (pendulum), `shades` (eyes replaced by lenses, lids are slat tilts), `mouth`, `legL`, `legR` (little feet below the lowest slat).
- Locomotion: shuffles with clattering slats, or rolls down and up in place to reach a higher perch.
- Fidgets: (1) slats tilt in a wave; (2) pulls itself down or up; (3) pushes the sunglasses up the nose, the cord swings.
- Temperament: cool, laid-back, sarcastic, unflappable; naps in the sun it is blocking.

**15. venty** (hover, S)
- Silhouette: compact rounded box 44 wide, 34 high with a diamond-shaped heat-exchanger core visible in the middle, a round window onto a fan wheel, and four duct stubs (two per side) that act as arms.
- Bones: `root`, `body`, `fanWheel` (spin), `core` (diamond pattern), `filter` (slides out), `ductL1`, `ductL2`, `ductR1`, `ductR2` (flexible two-segment tubes), `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`.
- Locomotion: hovers 6 units above the foot line on a cushion of its own airflow, small air jets underneath; slides.
- Fidgets: (1) fan wheel spins up and down; (2) inhales cold on one side and exhales warm on the other, the streaks swapping colour as they pass the core; (3) pulls the filter out, taps the dust off and slides it back.
- Temperament: refined, fastidious, a worrier about air quality; polite.

**16. chilly** (slider, M)
- Silhouette: boxy cabinet 44 wide, 36 high with rounded corners, a frosted front panel with a snowflake emblem, ice crystals on top, two coolant pipes (blue and red) at the sides.
- Bones: `root`, `body`, `frost` (grows and melts), `crystals[3]`, `emblem` (rotates), `pipeL`, `pipeR`, `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `armL`, `armR`, `bladeL`, `bladeR` (skate feet).
- Locomotion: glides on ice-skate blades, overshoots corners, a long slide before it stops.
- Fidgets: (1) frost spreads over the front and melts; (2) the emblem spins and breath fogs; (3) sneezes an ice cube.
- Temperament: aloof, dry-witted, grumpy-cool; picks fights with radiatory; secretly lonely.

**17. kettly** (biped, S)
- Silhouette: bulbous electric kettle 36 wide, 40 high, conical top with lid knob, spout left, handle right (an arm), a base plate with a power lamp; two short legs below the plate.
- Bones: `root`, `body`, `lid` (hinge), `spout`, `handle` (arm), `steam` (emitter), `powerLamp` (glow), `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `legL`, `legR`.
- Locomotion: hops, rocking on its base; gets faster as it "heats".
- Fidgets: (1) heats up: blushes red, rattles, the lid hops, steam whistles; (2) cools down with a sigh; (3) pours a tiny stream and blinks the power lamp.
- Temperament: excitable, hot-headed, impatient, a drama queen; high power, low stamina (a rate, not an amount).

**18. flamy** (hopper on a base, S)
- Silhouette: a tea light: aluminium cup (24 wide, 12 high), wax top, and a teardrop flame (18 wide, 28 high) that is the body with the face on it.
- Bones: `root`, `cup`, `wax`, `flame` (chain of 3: base, mid, tip), `core` (inner flame), `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `flickerL`, `flickerR` (arms).
- Locomotion: scoots on the cup in tiny hops; the flame lags behind movement.
- Fidgets: (1) flame flickers and the tip curls; (2) shrinks to an ember and sputters back to life; (3) leans away from draughts and gazes up at sunny.
- Temperament: small, shy and timid but brave; afraid of cloudy's rain and gusts; dreamy.

**19. thermy** (biped, L tall thin)
- Silhouette: slim thermometer 12 wide, 64 high: glass tube with tick marks, a round bulb at the bottom carrying the face; a red or blue liquid column; thin stick arms and legs.
- Bones: `root`, `bulb` (body, face), `tube`, `mercury` (scale-Y bone = temperature mood), `ticks`, `eyeL`, `eyeR`, `lidL`, `lidR`, `mouth`, `armL`, `armR`, `legL`, `legR`.
- Locomotion: long thin strides, or pogo hops on the bulb; leans to read.
- Fidgets: (1) the mercury rises near sunny, radiatory or servy and falls near chilly; (2) shivers or sweats at extremes; (3) taps the tube to check the scale, holds itself up to a neighbour to "measure" it.
- Temperament: nervous, precise, fussy, honest to a fault; blushes red from the top.

**20. servy** (roller, L)
- Silhouette: tall server rack 36 wide, 60 high with five horizontal drive bays, blinking LEDs on the right, vents at the sides, two caster wheels, a cable tail.
- Bones: `root`, `body`, `bays[5]` (LED states), `vents` (heat shimmer), `cableTail` (2 segments), `casterL`, `casterR`, `eyeL`, `eyeR` (in the top bay), `lidL`, `lidR`, `mouth`, `tendrilL`, `tendrilR` (cable arms).
- Locomotion: rolls slowly on casters with a whirr; heavy.
- Fidgets: (1) LEDs blink in binary patterns; (2) vents shimmer red and drop sweat when hot, a tiny ice pack on its head; (3) "reboots": a yawn, all LEDs off for a second, then on.
- Temperament: nerdy, busy, always overheated, perfectionist; never sleeps.

---

## 4. Social relations

Scale: affinity from -1 (hostile) to +1 (devoted); 0 is neutral and not listed. 67 pairs are non-neutral of the 190 possible. Values are for the unordered pair. All relations are meant as comedy of physics, with the squabbles mild (the lowest value is -0.7). † marks a justification based on general physics rather than on a quiz item.

### 4.1 Pair list, strongest first

| pair | affinity | physical justification |
|---|---:|---|
| pumpy – radiatory | +0.9 | Best friends: the heat pump serves low-temperature radiators (D2:kfw-40-heat-pump, H2:geg-2024); radiatory delivers pumpy's warmth into the room. |
| sunny – solary | +0.9 | PV turns sunlight (1,000 W/m² at test, P2:sunlight-square-metre) into electricity (P1:pv-module-peak); solary tilts toward sunny and glows when lit. |
| waly – insuly | +0.9 | Insuly wraps waly: 1.0, 0.28, 0.15 W/(m²·K) (H1:hollow-brick-wall-1970s, wall-geg, wall-passive-house). |
| roofy – insuly | +0.8 | 35 cm of insuly gives roofy U 0.10 (H1:roof-passive-house) instead of 2.1 (H1:solid-roof-1950s). |
| pumpy – insuly | +0.7 | Deep retrofit cuts the heating demand from 303 to 54 kWh/(m²·a), so pumpy buys 23 instead of 122 kWh/(m²·a) (D2:deep-retrofit-heat-pump vs old-house-heat-pump). |
| roofy – solary | +0.7 | Rooftop PV (P1:pv-annual-yield) and the PV roof of the plus-energy house (D1:plus-energy-house): roofy carries solary. |
| shady – chilly | +0.7 | External shading is what spares the chiller: 6 W/m² and 2 kWh/(m²·a) (C2:passive-house-home) against 100 W/m² unshaded (C2:office-1970s). |
| battery – solary | +0.6 | Battery stores solary's surplus: the amount (kWh) behind the rate (kWp), cf. P1:ev-battery and P1:pv-module-peak. |
| chilly – pumpy | +0.6 | Siblings: a heat pump is a chiller run backwards (refrigeration cycle †); chilly cools where pumpy heats. |
| cloudy – windy | +0.6 | Wind drives clouds; cloudy's puffed-cheek gust spins windy's blades (P3:wind-turbine-year, about 2,000 full-load hours). |
| insuly – housy | +0.6 | Insuly is housy's coat: retrofit and new-build standards (H2:gruenderzeit-retrofit, kfw-40, passive-house). |
| pumpy – solary | +0.6 | Plus-energy house: the PV roof feeds the heat pump (D1:plus-energy-house, net cost -2 EUR/(m²·a)); 30 ct purchase vs 8 ct feed-in. |
| pumpy – venty | +0.6 | Passive-house compact unit: heat recovery of at least 75 % plus a small heat pump (D1:passive-house). |
| radiatory – boily | +0.6 | Old couple: radiators at boiler flow temperatures (D2:old-house-gas, D2:gruenderzeit-gas). |
| servy – chilly | +0.6 | Chilly works around the clock for servy: 1,000 W/m² and about 8,000 full-load hours (C2:data-centre). |
| shady – windowy | +0.6 | Shady sits on windowy's head (roller-shutter box, H1:roller-shutter-box) and keeps the glass from overheating (C2:attic-flat unshaded). |
| waly – roofy | +0.6 | Teammates of the opaque envelope; GEG reference values 0.28 and 0.20 W/(m²·K) (H1:wall-geg, roof-geg). |
| battery – windy | +0.5 | Battery buffers windy's gusty output (P3:wind-turbine-year, 10 GWh per year) †. |
| chilly – venty | +0.5 | Teammates in air conditioning: filtered, cooled supply air (C1:operating-room, C2:hospital). |
| flamy – boily | +0.5 | Same family of burners: tea light (P1:tea-light), heating oil 10 kWh/l (P3:heating-oil-litre), gas flame (D2). |
| flamy – sunny | +0.5 | Smallest and largest of the powers: 35 W against 3.8×10²⁶ W (P2:tea-light, P2:sun); flamy looks up to sunny. |
| roofy – housy | +0.5 | The roof is the hat of the house (H1 roof items, H2 houses). |
| solary – windy | +0.5 | Complementary renewables (PV by day and summer, wind by night and winter †); P1:pv-annual-yield next to P3:wind-turbine-year. |
| waly – housy | +0.5 | Walls make the house (H1 wall items, H2 houses). |
| windowy – housy | +0.5 | Windows are the eyes of the house (H1 window items, H2 houses). |
| chilly – thermy | +0.4 | Chilly takes orders from thermy's 26 °C line (C2 prompt, D1 cooling axis). |
| kettly – boily | +0.4 | Both boil water: 93 Wh per litre (P3:boil-water); hot water 12.5 kWh/(m²·a) (D2). |
| pumpy – housy | +0.4 | Housy is the home pumpy heats (H2:geg-2024, D2). |
| radiatory – housy | +0.4 | Radiatory delivers the heating load of housy: 10 to 160 W/m² (H2). |
| radiatory – thermy | +0.4 | Thermy keeps radiatory at the 20 °C design line (H2 prompt). |
| servy – venty | +0.4 | Server halls live on moving air; venty brings filtered air like in cleanrooms (C1:cleanroom-iso-7/-6/-5) †. |
| shady – cloudy | +0.4 | Cloudy supplies shade for free, shady gets a day off †. |
| sunny – windy | +0.4 | Winds are driven by solar heating of the atmosphere †; windy thanks sunny (P2:wind-turbine, P2:sun). |
| venty – housy | +0.4 | Housy breathes through venty (C1:apartment, D1). |
| venty – insuly | +0.4 | Passive-house trio: insulation, airtightness, heat recovery (D1:passive-house). |
| battery – cloudy | +0.3 | Battery bridges the gaps cloudy leaves in the solar supply †. |
| boily – housy | +0.3 | Boily is the old heart of the house (D2 gas items). |
| chilly – cloudy | +0.3 | Overcast days lower the cooling load (C2 hot design day) †. |
| kettly – cloudy | +0.3 | Kettly's steam looks like a baby cloud (P3:boil-water) †. |
| pumpy – thermy | +0.3 | Seasonal performance depends on the temperature lift; thermy reads it out †. |
| radiatory – waly | +0.3 | Radiatory hangs on waly; waly stores the warmth (H1 wall items). |
| servy – kettly | +0.3 | Fellow heaters: every watt of IT power ends as heat (C2:data-centre), as does the kettle's 2 kW (P1:kettle). |
| shady – housy | +0.3 | Shady guards the house against summer sun (C2:new-home-geg). |
| solary – housy | +0.3 | Solary lives on the roof of the house (D1:plus-energy-house). |
| sunny – housy | +0.3 | Winter solar gains warm the house (H2); summer ones overheat it (C2). |
| venty – waly | +0.3 | An airtight envelope (n50 of at most 0.6 1/h, D1:passive-house) lets venty do all the breathing. |
| windowy – sunny | +0.2 | Windowy loves winter sun (solar gains) and flinches in summer (see seasonal modifier in 4.4). |
| boily – insuly | -0.2 | Insuly shrinks boily's job (303 to 15 kWh/(m²·a), H2); boily feels redundant. |
| roofy – windowy | -0.2 | Roof windows leak heat and gain sun (C2:attic-flat); roofy grumbles about the skylight. |
| battery – kettly | -0.3 | Amount against rate: kettly's 2 kW (P1:kettle) would empty a 15 Wh phone charge (P3:phone-charge) in under a minute; they bicker "power or energy?". |
| boily – solary | -0.3 | Fossil against sun: about 50 EUR/(m²·a) for the old gas house against -2 for the plus-energy house (D1). |
| chilly – windowy | -0.3 | Unshaded glazing is chilly's biggest load (C2:office-1970s, 100 W/m²). |
| kettly – flamy | -0.3 | 2 kW against 35 W (P2:kettle, P2:tea-light): kettly boasts 57 times flamy's power. |
| radiatory – windowy | -0.3 | Single glazing (5.8, H1:single-glazing) makes radiatory work overtime; a passive-house window (0.8, H1:window-passive-house) makes radiatory redundant. |
| servy – thermy | -0.3 | Servy runs hot (1,000 W/m², C2:data-centre); thermy's mercury shoots up and he fusses. |
| solary – shady | -0.3 | A shadow on a module drops its output †; solary flinches at shady's shade. |
| thermy – sunny | -0.3 | Sun drives the mercury up (roof skin 60 to 70 °C, C2:attic-flat). |
| chilly – sunny | -0.4 | Solar gains are chilly's workload (C2:attic-flat 50 W/m², C2:office-1970s 100 W/m²). |
| roofy – sunny | -0.4 | The sun heats the roof skin to 60 to 70 °C (C2:attic-flat); roofy sweats. |
| sunny – cloudy | -0.4 | Playful tease: cloudy drifts across sunny and takes away the 1 kW/m² of a clear summer noon (P2:sunlight-square-metre). |
| windowy – venty | -0.4 | Window ventilation loses heat and struggles at 4.8 1/h (C1:classroom); venty recovers at least 75 % (D1:passive-house). |
| flamy – cloudy | -0.5 | Rain and gusts snuff out a flame †. |
| sunny – shady | -0.5 | Shady blocks sunny's solar gains (C2:new-home-geg, C2:office-geg-shading); sunny pouts at the sunglasses. |
| windowy – waly | -0.5 | Heat-loss bickering: window 1.3 against wall 0.28 W/(m²·K), 4.6 times (H1:window-geg vs wall-geg); "you leak" against "I let in light". |
| pumpy – boily | -0.6 | SPF 2.7 to 3.5 against boiler efficiency 0.8 to 0.9: 122 against 409 kWh/(m²·a) for the same old house (D2:old-house-heat-pump vs old-house-gas). |
| chilly – radiatory | -0.7 | Opposite jobs, one removes heat and one adds it; heating and cooling at once wastes energy (20 °C design inside in H2, 26 °C cooling line in C2). |
| solary – cloudy | -0.7 | Module output scales with irradiance: solary sulks when cloudy stands in front of sunny (P1:pv-annual-yield already averages clouds in). |

### 4.2 Matrix

Numbers are the affinity, an empty cell is neutral, `–` is the diagonal. Column headers are the row numbers: 1 sunny, 2 cloudy, 3 housy, 4 solary, 5 radiatory, 6 pumpy, 7 windowy, 8 waly, 9 battery, 10 windy, 11 boily, 12 roofy, 13 insuly, 14 shady, 15 venty, 16 chilly, 17 kettly, 18 flamy, 19 thermy, 20 servy.

| |1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|17|18|19|20|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 sunny |–|-0.4|+0.3|+0.9|||+0.2|||+0.4||-0.4||-0.5||-0.4||+0.5|-0.3||
| 2 cloudy |-0.4|–||-0.7|||||+0.3|+0.6||||+0.4||+0.3|+0.3|-0.5|||
| 3 housy |+0.3||–|+0.3|+0.4|+0.4|+0.5|+0.5|||+0.3|+0.5|+0.6|+0.3|+0.4||||||
| 4 solary |+0.9|-0.7|+0.3|–||+0.6|||+0.6|+0.5|-0.3|+0.7||-0.3|||||||
| 5 radiatory |||+0.4||–|+0.9|-0.3|+0.3|||+0.6|||||-0.7|||+0.4||
| 6 pumpy |||+0.4|+0.6|+0.9|–|||||-0.6||+0.7||+0.6|+0.6|||+0.3||
| 7 windowy |+0.2||+0.5||-0.3||–|-0.5||||-0.2||+0.6|-0.4|-0.3|||||
| 8 waly |||+0.5||+0.3||-0.5|–||||+0.6|+0.9||+0.3||||||
| 9 battery ||+0.3||+0.6|||||–|+0.5|||||||-0.3||||
| 10 windy |+0.4|+0.6||+0.5|||||+0.5|–|||||||||||
| 11 boily |||+0.3|-0.3|+0.6|-0.6|||||–||-0.2||||+0.4|+0.5|||
| 12 roofy |-0.4||+0.5|+0.7|||-0.2|+0.6||||–|+0.8||||||||
| 13 insuly |||+0.6|||+0.7||+0.9|||-0.2|+0.8|–||+0.4||||||
| 14 shady |-0.5|+0.4|+0.3|-0.3|||+0.6|||||||–||+0.7|||||
| 15 venty |||+0.4|||+0.6|-0.4|+0.3|||||+0.4||–|+0.5||||+0.4|
| 16 chilly |-0.4|+0.3|||-0.7|+0.6|-0.3|||||||+0.7|+0.5|–|||+0.4|+0.6|
| 17 kettly ||+0.3|||||||-0.3||+0.4||||||–|-0.3||+0.3|
| 18 flamy |+0.5|-0.5|||||||||+0.5||||||-0.3|–|||
| 19 thermy |-0.3||||+0.4|+0.3||||||||||+0.4|||–|-0.3|
| 20 servy |||||||||||||||+0.4|+0.6|+0.3||-0.3|–|

The pair list and the matrix were generated from one data table and checked for duplicates, unknown ids and range, so they agree.

### 4.3 Who gets along: summary per pet

| pet | friends (> 0) | rivals (< 0) | strongest friend | strongest rival |
|---|---:|---:|---|---|
| sunny | 5 | 5 | solary +0.9 | shady -0.5 |
| cloudy | 5 | 3 | windy +0.6 | solary -0.7 |
| housy | 11 | 0 | insuly +0.6 | – |
| solary | 6 | 3 | sunny +0.9 | cloudy -0.7 |
| radiatory | 5 | 2 | pumpy +0.9 | chilly -0.7 |
| pumpy | 7 | 1 | radiatory +0.9 | boily -0.6 |
| windowy | 3 | 5 | shady +0.6 | waly -0.5 |
| waly | 5 | 1 | insuly +0.9 | windowy -0.5 |
| battery | 3 | 1 | solary +0.6 | kettly -0.3 |
| windy | 4 | 0 | cloudy +0.6 | – |
| boily | 4 | 3 | radiatory +0.6 | pumpy -0.6 |
| roofy | 4 | 2 | insuly +0.8 | sunny -0.4 |
| insuly | 5 | 1 | waly +0.9 | boily -0.2 |
| shady | 4 | 2 | chilly +0.7 | sunny -0.5 |
| venty | 6 | 1 | pumpy +0.6 | windowy -0.4 |
| chilly | 6 | 3 | shady +0.7 | radiatory -0.7 |
| kettly | 3 | 2 | boily +0.4 | battery -0.3 |
| flamy | 2 | 2 | sunny +0.5 | cloudy -0.5 |
| thermy | 3 | 2 | radiatory +0.4 | servy -0.3 |
| servy | 3 | 1 | chilly +0.6 | thermy -0.3 |

Factions that fall out of the matrix (useful for grouping and for who stands next to whom by default): **weather** sunny, cloudy, windy; **envelope** housy, waly, roofy, windowy, insuly, shady, venty; **heat** radiatory, pumpy, boily, flamy, kettly; **cold** chilly, servy, thermy (thermy is the go-between); **electricity** solary, battery (and windy). Housy has no rivals: it is the host.

### 4.4 Modulation rules (so relations are alive, not static)

- **Solary → sunny** affection is multiplied by sunny's brightness and is replaced by the -0.7 sulk while cloudy overlaps sunny's disc.
- **Windowy → sunny** flips with a "season" variable of the page or with the drawn items: +0.2 in the heating quiz (solar gains help), -0.3 in the cooling quiz (solar gains hurt).
- **Radiatory – chilly** only squabbles while both are "on" (both on screen and active, as on the home page); a single one alone has nobody to argue with. Thermy (+0.4 to each) steps between them as mediator.
- **Pumpy – radiatory** gets +0.1 when radiatory is "large and cool" (low-temperature heating), -0.1 when radiatory shows the boily flow colour.
- **Learner events** (answers) switch pairs into scenes: a right answer on an item that justifies a pair makes them cheer; a wrong one makes them squabble or look away (windowy – waly on a wrong H1 U-value, pumpy – boily on a wrong D2 final energy). Trigger items per pet are listed in 2.1 and Appendix A.

### 4.5 Signature scenes (ready-made vignettes)

1. **Eclipse.** Cloudy drifts in front of sunny; solary droops, battery pats solary, sunny pouts; windy spins faster as cloudy puffs.
2. **Heat-loss squabble.** Windowy and waly trade U-values (1.3 against 0.28); insuly arrives and wraps waly; windowy sulks, then shady puts sunglasses on windowy as a peace offering.
3. **The temperature dispute.** On the home page radiatory (west) and chilly (east) shout across the leaderboard; thermy runs between them reading "20 °C" and "26 °C".
4. **Retrofit.** Insuly wraps housy, boily is retired and trundles off muttering "0.8", pumpy plugs into radiatory with a proud 3.5.
5. **Power or energy.** Kettly boasts "2 kW", battery answers "60 kWh", both end up looking at flamy "35 W".
6. **From tea light to Sun.** Flamy gazes up at sunny; cloudy blows flamy out; kettly sheepishly re-lights it with a lamp flash.

---

## 5. Casts

### 5.1 Casting rules

- Each quiz has a **troupe of 7**: **3 core** (always visible on that quiz's pages) and **4 rotation** (at most 2 visible at a time, swapped every 60 to 120 seconds or on a task change).
- Visible at once: up to **5** on desktop, **4** on tablet, **3** (the core) on phones. The home page shows the seeds (5.4).
- A task page has a **spotlight** of 2 to 3 pets (5.3) drawn from the quiz's troupe plus item-triggered guests (2.1, Appendix A).
- Every pet is in at least one troupe, so none is orphaned.
- Pair rule: the visible cast should contain at least one friend pair and, where the graph has one, one rival pair.

### 5.2 Per quiz

| quiz | core (3) | rotation (4) | why |
|---|---|---|---|
| 🧲 physics | kettly, battery, sunny | flamy, windy, solary, cloudy | kettly (power) and battery (energy) embody the two categories of P1; flamy and sunny are the ends of P2; windy and battery carry P3; solary and cloudy for the PV and clear-sky items |
| 🔥 heating | radiatory, waly, windowy | housy, insuly, roofy, thermy | H1 is walls, windows and roofs; H2 is houses and their loads; insuly is the reason for every low U-value; thermy for 20 °C and -12 °C |
| ❄️ cooling | chilly, shady, venty | sunny, cloudy, servy, thermy | C1 is ventilation; C2 is shading and chillers; sunny and cloudy as source and foil; servy for the data centre; thermy for the 26 °C line |
| 📊 demand | pumpy, boily, solary | housy, insuly, venty, radiatory | D2 is supply systems (heat pump, boiler, PV, solar hot water); D1 profiles are houses with insulation and ventilation; radiatory is the heat emitter of every D2 house |

Visible cast examples: physics desktop = kettly, battery, sunny + flamy, windy (3 + 2); cooling phone = chilly, shady, venty.

### 5.3 Per task (spotlight)

| task | spotlight | notes |
|---|---|---|
| P1 power-or-energy | kettly (Power), battery (Energy), sunny | the two category cards each get a referee; sunny for `pv-module-peak`, `heating-load` goes to radiatory as guest |
| P2 powers | flamy (low end), sunny (high end), windy (5 MW in the middle) | walk to the ends of the sorting list; ice-train and wind-turbine trigger windy |
| P3 energies | battery, windy, kettly | phone-charge and ev-battery, wind-turbine-year, boil-water |
| H1 u-values | waly, windowy, roofy, insuly (3 at once) | one of each family of item; `roller-shutter-box` calls shady, `front-door-geg` and `floor-geg` call housy |
| H2 heating-load-and-demand | housy, radiatory, insuly | thermy joins for the 20 °C line; `geg-2024` calls pumpy |
| C1 air-change-rates | venty, windowy, chilly | `classroom` calls windowy, `operating-room` chilly, `commercial-kitchen` kettly |
| C2 cooling-load-and-demand | shady, chilly, sunny | `attic-flat` calls roofy and windowy, `data-centre` calls servy and thermy |
| D1 standard-profiles | housy, venty, pumpy | spider axes: heating (insuly), cooling (shady), ventilation (venty), costs (solary for the PV credit) |
| D2 final-energy | pumpy, boily, solary | solary wears the collector outfit for `kfw-55-gas-solar`; housy joins on `old-house-gas` (class H) |

Existing task icons are already pets in the making: ☀️ sunny, 🔋 battery, 🧱 waly, 🌬️ venty, ❄️ chilly, 🔥 boily (heating row) or flamy.

### 5.4 Home page (catalog level)

Cards in DOM order and grid cells: learner (NW), physics (N), intro (NE), heating (W), board (centre), cooling (E), badges (SW), demand (S), prefs (SE).

| surface | default pets (the nine seeds) | visitors (tier 2 and 3, one at a time every 1 to 2 minutes, stay 30 seconds) |
|---|---|---|
| learner (NW) | housy (welcomes the learner) | thermy |
| physics (N) | sunny, solary (sunny shines on solary on top of the card) | kettly, flamy, windy |
| intro (NE) | – (text-heavy; pets pass over it, nobody stays) | windy |
| heating (W) | radiatory, waly, windowy | insuly, roofy, boily |
| board (centre) | nobody stands on it permanently (readability); pets cross it | any |
| cooling (E) | cloudy (drifts along the east side and across the top to tease sunny; overcast means little cooling, so it likes chilly) | chilly, shady, venty, servy |
| badges (SW) | battery (the charge bars are the badges' progress) | – |
| demand (S) | pumpy | boily, venty |
| prefs (SE) | – | thermy (adjusts the dials), chilly |

Rules: width ≥ 1024 px shows the nine seeds plus at most one visitor; 768 to 1023 px shows five (sunny, cloudy, housy, radiatory, pumpy); below 768 px shows three (sunny, cloudy, housy). The home spans all four topics, so no topic-specific core applies there.

### 5.5 Coverage check

● core, ○ rotation, ▫ spotlight or visitor only, – none. Every row has at least one mark.

| pet | home | physics | heating | cooling | demand |
|---|:-:|:-:|:-:|:-:|:-:|
| sunny | ● | ● | – | ○ | – |
| cloudy | ● | ○ | – | ○ | – |
| housy | ● | – | ○ | – | ○ |
| solary | ● | ○ | – | – | ● |
| radiatory | ● | – | ● | – | ○ |
| pumpy | ● | – | ▫ | – | ● |
| windowy | ● | – | ● | ▫ | – |
| waly | ● | – | ● | – | – |
| battery | ● | ● | – | – | – |
| windy | ▫ | ○ | – | – | – |
| boily | ▫ | ▫ | – | – | ● |
| roofy | – | – | ○ | ▫ | – |
| insuly | ▫ | – | ○ | – | ○ |
| shady | ▫ | – | ▫ | ● | – |
| venty | ▫ | – | – | ● | ○ |
| chilly | ▫ | – | – | ● | – |
| kettly | ▫ | ● | – | ▫ | – |
| flamy | ▫ | ○ | – | – | – |
| thermy | ▫ | – | ○ | ○ | – |
| servy | ▫ | – | – | ○ | – |

---

## 6. Colours

### 6.1 The site palette (quoted from the repo)

Source: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css` (generated from `framework/ui/styling/🔣️.json`), aliases in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` lines 88 to 124. The site sheet `🎓️teaching/🏛️architecture/❓️quiz/🎨️.css` only imports this chain; the quiz sheet `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🎨️.css` consumes `--base`, `--foreground`, `--active-base`, `--border-normal-color`, `--color-warning`, `--color-secondary` and the per-learner `--quiz-peer` colours.

Brand and signal colours (`@theme`):

| token | value | note |
|---|---|---|
| `--color-primary` | `#ff344f` | coral red; `--active-base` and `--accent` in both themes |
| `--color-secondary` | `#34d1bf` | teal; used for notes |
| `--color-tertiary` | `#fa9500` | orange |
| `--color-warning` | `#fccf05` | yellow; alert bars |
| `--color-danger` | `#a60009` | deep red |
| `--color-info` | `#dbbea1` | sand |
| `--color-success` | `#7eb77f` | sage green |
| `--color-diff-added` | `#00d492` | mint green |
| `--color-indirect-handle` | `#c4e4d5` | pale mint |
| `--color-dark` | `#001117` | near-black teal, dark base |
| `--color-light` | `#f7f3e3` | cream, light base |
| grays (warm, green-tinted) | `--color-gray` `#7b827d`; `--color-gray-100` `#06171c` … `--color-gray-300` `#334041`, `--color-gray-400` `#555f5d`, `--color-gray-600` `#a2a59d`, `--color-gray-700` `#c4c4b9`, `--color-gray-800` `#dfddd0`, `--color-gray-900` `#f1edde`; `--color-d-g-g-g` `#4c5756`; `--color-light-gray` `#d3d2c5` | |

Semantic aliases per theme:

| alias | light (`:root`) | dark (`.dark`) |
|---|---|---|
| `--base` | `#f7f3e3` | `#001117` |
| `--panel` | `#c9c8bd` | `#1d2b2f` |
| `--foreground` | `#001117` | `#f7f3e3` |
| `--muted` | `#f0ecdd` | `#07181d` |
| `--muted-foreground` | `#3e494a` | `#91968f` |
| `--accent`, `--active-base` | `#ff344f` | `#ff344f` |
| `--border-normal-color` | `#7b827d` | `#7b827d` |

Presence colours (learners' cursors): twelve hues 0, 210, 120, 30, 270, 180, 330, 60, 240, 150, 300, 90 at `hsl(h 68% 32%)` in light and `hsl(h 72% 62%)` in dark. Cursors are small arrows with a name label; pets are large outlined figures with faces, so they do not compete, but pet bodies should avoid pure presence swatches by being outlined.

Fonts: Anta (sans), Kelly Slab (serif), Share Tech Mono, monochrome Noto Emoji. Motion hooks that exist: `prefers-reduced-motion`, `data-icon-motion="off"`, `forced-colors: active`.

### 6.2 Rules for the pet colours

1. Every pet is drawn with a **2 px outline in `--foreground`** (flips with the theme; 17.3:1 against `--base`). The fills against the glass panels reach only 2.0 to 3.7:1 on the light panel `#c9c8bd` and 2.3 to 4.3:1 on the dark panel `#1d2b2f` (the outline gives 11.4:1 and 13.1:1), so the outline is not optional.
2. Each **body** colour was tuned in lightness only, to be at least **3:1 against both** `#f7f3e3` and `#001117` (WCAG 1.4.11 for graphical objects). The band that satisfies both is relative luminance 0.115 to 0.267. Exception: sunny keeps the brand yellow because a dull gold would not read as a sun; it relies on the outline in the light theme (1.34:1 on cream, 12.9:1 on dark).
3. **Accent** and **detail** colours come from the site palette and sit inside the body, so they need no contrast against the page, only against the body.
4. Eyes: sclera `#f7f3e3`, pupil and lid line `#001117` (17:1).
5. Hue plan, so the roster stays legible at a glance: weather = yellow, blue-grey, lavender (sunny, cloudy, windy); envelope = tan, brick, slate, pink, violet (housy, waly, roofy, insuly, shady); heat = coral, teal, cast-iron brown, orange, steel (radiatory, pumpy, boily, flamy, kettly); cold and air = ice blue, lime, thermy's blue-to-red gradient (chilly, venty, thermy); electricity = indigo, green, dark slate-teal (solary, battery, servy). Silhouettes do the rest.
6. In forced-colours mode the browser replaces custom colours (the existing sheet handles this with system colours for other states); the pets' silhouettes and outlines must stand without colour, and the outline should switch to `CanvasText`.

### 6.3 Per-pet colours

Contrast ratios are WCAG relative-luminance ratios (computed, not estimated). L = against `#f7f3e3`, D = against `#001117`.

| pet | body | L : D | accent | detail | where the colours sit |
|---|---|---|---|---|---|
| sunny | `#fccf05` (`--color-warning`) | 1.34 : 12.87 (outline in light) | `#fa9500` (`--color-tertiary`) | `#ff344f` (`--color-primary`) | body disc and halo; rays; cheeks. Dual-safe fallback for body: `#a68802` (3.07 : 5.62) |
| cloudy | `#63939a` | 3.06 : 5.65 | `#34d1bf` (`--color-secondary`) | `#c4e4d5` (`--color-indirect-handle`) | body; raindrop; belly highlight |
| housy | `#b87f46` | 3.06 : 5.64 | `#a60009` (`--color-danger`) | `#dbbea1` (`--color-info`) | walls; roof; door and trim |
| solary | `#2c5fb0` | 5.60 : 3.09 | `#c4e4d5` | `#c4c4b9` (`--color-gray-700`) | cells; cell highlights; aluminium frame |
| radiatory | `#ff344f` (`--color-primary`) | 3.22 : 5.36 | `#fa9500` | `#f7f3e3` | fins; heat shimmer; knob |
| pumpy | `#1e9b8d` | 3.08 : 5.61 | `#34d1bf` | `#fa9500` | housing; fan blades and cool intake; warm outlet |
| windowy | `#7b827d` (`--color-gray`, frame) | 3.54 : 4.88 | `#bfe6ea` (glass) | `#f7f3e3` | frame; panes; glint |
| waly | `#a3472f` | 5.40 : 3.20 | `#dbbea1` | `#4c5756` | bricks; mortar; shadow side |
| battery | `#3e9c64` | 3.07 : 5.62 | `#00d492` (`--color-diff-added`) | `#c4c4b9` | casing; charge bars; terminal cap |
| windy | `#8282dd` | 3.05 : 5.66 | `#f7f3e3` | `#4c5756` | hub and nacelle; blades; tower shade |
| boily | `#705c54` | 5.64 : 3.06 | `#fa9500` | `#fccf05` | cast-iron casing; flame; flame core |
| roofy | `#5d7185` | 4.53 : 3.81 | `#a3472f` | `#334041` (`--color-gray-300`) | slate tiles; chimney brick; tile lines |
| insuly | `#e0617f` | 3.05 : 5.66 | `#f7f3e3` | `#a64a63` | wool; fluff highlights; fold shadows |
| shady | `#8a5fb8` | 4.28 : 4.04 | `#c4c4b9` | `#001117` | slats; slat highlight; sunglasses |
| venty | `#68952c` | 3.19 : 5.42 | `#f7f3e3` | `#c4e4d5` | casing; fan blades; air streaks |
| chilly | `#1f95c8` | 3.05 : 5.67 | `#f7f3e3` | `#bfe6ea` | cabinet; frost and snowflake; ice |
| kettly | `#808d98` | 3.06 : 5.66 | `#ff344f` | `#f7f3e3` | steel body; handle and knob; steam |
| flamy | `#de6a00` | 3.06 : 5.65 | `#fccf05` | `#c4c4b9` | outer flame; inner flame; tin cup |
| thermy | gradient `#2f7fae` (3.96 : 4.36) at the bulb to `#ff344f` (3.22 : 5.36) at the top | both ends >= 3.2 | `#e8e2c8` (glass tube) | `#7b827d` | liquid column; glass; scale ticks |
| servy | `#406670` | 5.63 : 3.07 | `#00d492` | `#ff344f` | cabinet; status LEDs; alert LED |

Palette tokens reused unchanged: `--color-warning`, `--color-tertiary`, `--color-primary`, `--color-secondary`, `--color-indirect-handle`, `--color-danger`, `--color-info`, `--color-gray`, `--color-diff-added`, `--color-gray-300`, `--color-gray-700`, `--color-d-g-g-g`, `--color-light`, `--color-dark`. Colours added (not in the palette, tuned in lightness from palette hues to hit 3:1 on both themes): `#63939a`, `#b87f46`, `#2c5fb0`, `#1e9b8d`, `#a3472f`, `#3e9c64`, `#8282dd`, `#705c54`, `#5d7185`, `#e0617f`, `#8a5fb8`, `#68952c`, `#1f95c8`, `#808d98`, `#de6a00`, `#2f7fae`, `#e8e2c8`, `#406670`, `#bfe6ea`, `#a64a63`, `#a68802`. If the owner wants no colour outside the palette, the alternatives are: housy body `#dbbea1`, cloudy `#c4e4d5`, venty `#c4e4d5`, kettly `#c4c4b9` (all with outline, about 1.2 to 1.6 : 1 on cream).

---

## 7. Hand-off notes and open points

- **Data, not code.** Schema-first suggestion: one roster file at catalog level with, per pet, `id`, accessible names (en, de), `justifiedBy: [{quiz, task, item}]` and `colours`; relations as a list of `{a, b, affinity}`; each quiz file gets `cast: {core: [...], rotation: [...]}`. A check in `🧪️tests/🧪️catalog` can then fail the build when a cast pet has no justifying item in that quiz or names an unknown item. That enforces "pets always fit the topics", also for later topics. (Not implemented here; this task was read-only.)
- **Weakest grounding to confirm with the owner:** cloudy (no item), servy (one item), thermy (concept, not item), windy (two items). All four keep value as characters; drop servy or thermy first if 18 is preferred over 20.
- **Naming checks:** `windy` against `windowy`; `shady` double meaning; `battery` is the one name that is already the noun.
- **Task text mentions in German** use "Akku" (not "Batterie") for the car and phone battery; the accessible German name uses "der Akku".
- **Not covered by any pet** (14 items, Appendix A): human body and food (P1/P2/P3), cars and trains, power plants and national or world totals, household electricity. They are context, and the learner is the human of the cast.
- **Risk:** the site is bilingual with no default language; pet names are nicknames and stay identical in both languages, only accessible descriptions differ.

---

## Appendix A: which pets can react to which item (all 109)

`–` = no pet reaction. Pets listed are candidates for a cameo or reaction when the item is drawn.

**P1 power-or-energy (16):** tea-light: flamy, boily. kettle: kettly. resting-person: –. pv-module-peak: solary, sunny. heating-load: radiatory, housy. nuclear-unit: –. car-engine: –. wallbox: battery. household-electricity: –. ev-battery: battery. heating-oil-litre: boily, flamy. heating-demand-house: housy, insuly, radiatory. chocolate-bar: –. pv-annual-yield: solary, roofy, cloudy. phone-charge: battery. energy-certificate: housy.

**P2 powers (14):** tea-light: flamy. resting-person: –. sunlight-square-metre: sunny, solary, cloudy. kettle: kettly. wallbox: battery. heating-load-old-house: radiatory, housy, kettly. car-engine: –. wind-turbine: windy. ice-train: windy. nuclear-unit: –. germany-electricity: –. world-primary-power: –. sunlight-on-earth: sunny. sun: sunny.

**P3 energies (12):** phone-charge: battery. boil-water: kettly, boily. chocolate-bar: –. daily-food: –. heating-oil-litre: boily, flamy. ev-battery: battery. petrol-tank: battery. household-electricity: –. heating-demand-house: housy, insuly. wind-turbine-year: windy. germany-primary-energy: –. world-primary-energy: sunny.

**H1 u-values (17):** single-glazing: windowy, radiatory. aluminium-window-1970s: windowy. roller-shutter-box: shady, windowy. box-type-window: windowy. solid-roof-1950s: roofy, insuly. front-door-geg: housy. half-timbered-wall: waly. window-geg: windowy. hollow-brick-wall-1970s: waly, insuly. window-passive-house: windowy, radiatory. masonry-wall-1980s: waly, insuly. triple-glazing: windowy. floor-geg: housy, insuly. wall-geg: waly, insuly. roof-geg: roofy, insuly. wall-passive-house: waly, insuly. roof-passive-house: roofy, insuly.

**H2 heating-load-and-demand (11):** passive-house: housy, venty, insuly. gruenderzeit-retrofit: housy, insuly, venty. kfw-40: housy, insuly, venty. kfw-55: housy, insuly. geg-2024: housy, pumpy. sfh-2000s: housy. plattenbau: housy, waly. sfh-1990s: housy. gruenderzeit: housy, waly. sfh-1970s: housy. sfh-1960s: housy, radiatory. (radiatory and thermy may accompany every item of this task.)

**C1 air-change-rates (15):** warehouse: venty. passive-house-dwelling: venty, housy. apartment: venty, housy. sports-hall: venty. single-office: venty. residential-car-park: venty. cinema: venty. classroom: venty, windowy. restaurant: venty. laboratory: venty. operating-room: venty, chilly. commercial-kitchen: venty, kettly. cleanroom-iso-7: venty. cleanroom-iso-6: venty. cleanroom-iso-5: venty, servy.

**C2 cooling-load-and-demand (9):** passive-house-home: shady, chilly, housy. new-home-geg: shady, housy. school-new-build: chilly, sunny. office-passive-house: shady, venty. office-geg-shading: shady, sunny, chilly. attic-flat: roofy, windowy, sunny, chilly. hospital: chilly, venty. office-1970s: windowy, sunny, chilly. data-centre: servy, chilly, thermy.

**D1 standard-profiles (6):** unrenovated-old-building: housy, boily, roofy. wschvo-1995: housy, boily. enev-2014: housy, boily, venty. kfw-40: housy, pumpy, venty. passive-house: housy, venty, shady, pumpy, insuly. plus-energy-house: solary, roofy, pumpy, windowy.

**D2 final-energy (9):** kfw-40-heat-pump: pumpy, venty. deep-retrofit-heat-pump: pumpy, insuly. passive-house-direct-electric: housy, venty. kfw-55-gas-solar: boily, solary. enev-2014-gas: boily. old-house-heat-pump: pumpy. wschvo-1995-gas: boily. gruenderzeit-gas: boily. old-house-gas: boily, housy.

Items with no pet: P1: resting-person, nuclear-unit, car-engine, household-electricity, chocolate-bar. P2: resting-person, car-engine, nuclear-unit, germany-electricity, world-primary-power. P3: chocolate-bar, daily-food, household-electricity, germany-primary-energy. Total 14 of 109.
