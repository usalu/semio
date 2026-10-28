# Narration Audit: Six Design-Optimization Clips

Read-only audit of the `NARRATION` lists in `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/{📉️extrema, 🎲️sampling, 🧪️experiments, ⚖️index, 🏆️pareto, 🧬️algorithms}/🐍️.py`. Each file was read once (snapshot dumped to the scratchpad) and checked for terminology, German quality, English quality, meaning parity, numbers, length (English lines at most 95 characters) and didactics (opening question, closing concrete answer). Only the NARRATION lists and TITLE dicts were in scope; no source file was changed.

Proposed German uses Swiss orthography (ss instead of ß), see rule R4. Proposed lines use the `%` and `kWh/a` forms of rules R2 and R3.

## Counts

| Clip | Rows audited | Rows with a proposal | English lines over 95 characters | Didactic fix needed |
|---|---|---|---|---|
| extrema | 27 | 38 | 2 | opening and closing |
| sampling | 32 | 38 | 0 | opening and closing |
| experiments | 34 | 61 | 0 | opening and closing |
| index | 34 | 38 | 0 | opening and closing (plus title) |
| pareto | 32 | 41 | 0 | opening and closing |
| algorithms | 40 | 50 | 0 | closing |

The "rows with a proposal" column counts table rows (English and German, including inserted lines), not distinct lines.

Over-length lines in the current files: extrema `sweep_4` (96) and `scope_2` (96). Several German lines run to 100 characters (experiments `saving_70` 100, extrema `sweep_4` 101); German has no fixed limit, but these are shortened where the proposal is given.

## R. Rules (apply to all six clips)

- **R1 Numbers.** Spell out one to twelve in English and German (nine / neun, four / vier), unless a unit follows. Use digits from 13 up and for any number with a unit (`5 kWh/a`, `30 %`, `0.6 m`). Current violations are all in `algorithms` (for example `5 runs`, `4 Läufe`, `8 Entwürfe`, `7 Generationen`, `6 runs`).
- **R2 Percent and units.** A space before `%` and before every unit: `30 %`, `0.6 m`, `3.0 CHF`, `75 %`. German decimal comma (`0,6 m`, `3,0 CHF`), English decimal point. Thousands: `1,681` (en) and `1.681` (de). Degrees without a space (`180°`).
- **R3 Annual electricity.** Symbol `kWh/a` in both languages, never `kWh per year`, `Kilowatt hours per year`, `pro Jahr` directly after `kWh/a` (remove the redundant `every year` / `pro Jahr`).
- **R4 Orthography.** The files use ß in 16 places (listed in the section "ß list"). The course is Swiss (CHF, Zurich), so Swiss ss is recommended everywhere. If the course is meant for Germany, keep ß; then every ß listed below stays as it is.
- **R5 Address.** German imperatives use `du` (`Nenne`, `Prüfe`, `Dokumentiere`); `wir` for the team. Keep this; no mixed `Sie`.
- **R6 Quotes.** No quotation marks occur in any narration line. None needed.
- **R7 English spelling.** British throughout (`neighbour`, `centre`, `metre`, `judgement`, `colour`); keep it.
- **R8 Simulation, not measurement.** The model is simulated. Never `measure`, `messen`, `Messung` for a design result, except where the line explicitly contrasts with real measurements (experiments `response_3`, `repeat_2`). `run` (en) and `Lauf` (de) are not used for a simulation; use `simulation` / `Simulation`. `evaluate` / `bewerten` / `Auswertung` are replaced by `simulate` / `simulieren`.
- **R9 Electricity classes never mixed.** Annual electricity `E` = `electricity` / `Strombedarf` (kWh/a). Parts: `heating electricity` / `Heizstrom`, `cooling electricity` / `Kühlstrom`, `lighting electricity` / `Beleuchtungsstrom`. Never `Lichtstrom` (German for luminous flux). Never `Wärmebedarf` for these. Sun energy stays `solar energy` / `Sonnenenergie`, so it is never confused with electricity.
- **R10 Index criterion = intensity.** The index electricity criterion is kWh/(m²·a) (model `electricity` key `intensity`), not kWh/a. Name it `electricity intensity` / `Strombedarf pro m²` in the index clip, so it is not confused with the total E used in the other five clips.

## Glossary (English / German)

| Concept | English | German | Use / avoid |
|---|---|---|---|
| One parameter set | design | Entwurf | Not `variant` / `Variante` / `Design`. `design` in plural `designs` / `Entwürfe`. |
| Glazed façade | glazed façade | verglaste Fassade | Introduce as `verglaste Fassade`, then `Fassade`. `Glasfassade` only in running text if it is already introduced. |
| Design variable | design variable | Entwurfsvariable | In sampling, extrema, pareto, algorithms. `factor` / `Faktor` only in experiments (defined there as design variable). |
| Orientation θ | orientation | Ausrichtung | Not `bearing`, `Richtung`, `Orientierung`. `compass direction` / `Himmelsrichtung` only for the name of a direction. |
| Window-to-wall ratio | window-to-wall ratio (WWR), then window share | Fensterflächenanteil (WWR), then Fensteranteil | Never `window size` / `Fenstergröße` (WWR is relative, not absolute). Never bare `window` for the variable. |
| Overhang depth d | overhang | Vordach; Vordachtiefe (depth) | Not `Dachüberstand`, `Auskragung`. |
| Insulation thickness t | insulation | Dämmung (Dämmstärke t) | Keep `Dämmung` in lists. |
| Objective | objective | Zielgröße | Pareto and extrema. `goal` / `Ziel` not used for objectives. |
| Criterion | criterion | Kriterium | Index only (weights). |
| Annual electricity E | electricity | Strombedarf | kWh/a. Generic `Strom` is allowed only in compounds (`Strombedarf`, `Heizstrom`). |
| Electricity intensity | electricity intensity | Strombedarf pro m² | Index only, kWh/(m²·a). |
| Heating / cooling / lighting electricity | heating / cooling / lighting electricity | Heizstrom / Kühlstrom / Beleuchtungsstrom | Not `Heizen` / `Kühlung` / `Licht` as stand-ins for electricity. Labels `Heizen`, `Kühlen`, `Licht` in the scenes are on-screen words, see cross-clip notes. |
| Simulation | simulation; simulate | Simulation; simulieren | Not run / Lauf / evaluate / Auswertung / bewerten. |
| Sample | sample (point before simulation) | Stichprobe; Punkt | Only in sampling. |
| Budget | budget | Budget | Keep. |
| Minimum vs optimum | minimum = value of E; optimum = design θ* | Minimum = Wert; Optimum = Entwurf | Index, extrema, algorithms: do not call a value `optimum`. |
| Front | Pareto front | Pareto-Front | Keep. |
| Dominated | dominated | dominiert | Keep. |
| Tile | tile | Kachel | Pareto. |
| Map | map | Karte | Algorithms. |
| Teaching model | simplified annual teaching model | vereinfachtes Jahresmodell | Not `Jahres-Lehrmodell` (experiments, sampling). |
| Cost | façade cost | Fassadenkosten | CHF. |
| Tariff | tariff (CHF/kWh) | Tarif (CHF/kWh) | Keep. |
| Daylight | daylight factor | Tageslichtquotient | Criterion in index is `Daylight factor`. Plain `daylight` allowed for the sun / light phenomenon. |
| Embodied carbon | embodied carbon | Graue Emissionen | Model title. Narration currently says `Carbon` / `CO₂`. |
| Lower / higher is better | lower is better / higher is better | kleiner ist besser / mehr ist besser | Not `weniger ist besser`. |
| Units | kWh/a; %; m; CHF; CHF/a | kWh/a; %; m; CHF; CHF/a | Space before unit (R2). |
| Sun seasons | summer sun / winter sun | Sommersonne / Wintersonne | Keep. |

## Per-clip proposals

Each table lists only lines with a problem. `key` is the scene key. Proposed text is complete. For insertions the key says `(new)`.

### 1. Extrema (📉️extrema)

Opening: statement (fix). Closing: abstract `scope_2` followed by a generic `scope` line; the answer is only implicit and `scope` is a jargon imperative (fix: concrete answer last).

| key | lang | problem | proposed replacement text |
|---|---|---|---|
| formulate | en | Opens with a statement, not a concrete question about the room. | This seminar room has one glazed façade. Which compass direction should it face? |
| formulate | de | Same as en; de opens with a statement. | Dieser Seminarraum hat eine verglaste Fassade. In welche Himmelsrichtung soll sie zeigen? |
| formulate_2 | en | `bearing` is jargon; use the glossary term `orientation`. | The façade orientation θ is our design variable: east 90°, south 180°, west 270°, north 360°. |
| formulate_2 | de | `Fassadenrichtung` replaced by glossary term `Ausrichtung`. | Die Ausrichtung der Fassade θ ist unsere Entwurfsvariable: Ost 90°, Süd 180°, West 270°, Nord 360°. |
| response | en | `kWh per year` breaks R3; `in kWh/a` is the unit. | The objective is the room's electricity for heating, cooling and lighting, in kWh/a. |
| response | de | `Ziel` should be `Zielgröße`; `Heizen, Kühlen, Beleuchten` are infinitives, use nouns; `pro Jahr` breaks R3. | Die Zielgröße ist der Strombedarf für Heizung, Kühlung und Beleuchtung, in kWh/a. |
| domain | en | `bearings` is not in the glossary; `record` hides that this is a simulation. | We turn the façade from east, 90°, through all orientations and simulate electricity in kWh/a. |
| domain | de | `messen` implies a measurement, but the value is simulated (R8; experiments says "keine Messung"). `Richtungen` replaced by `Ausrichtungen`; `ab Osten` is not idiomatic. | Wir drehen die Fassade von Osten, 90°, über alle Ausrichtungen und simulieren den Strombedarf in kWh/a. |
| sweep_2 | en | `cooling` alone is ambiguous (R9); line is 104 characters if extended. | The high summer sun hits south glass steeply: cooling electricity drops to the lowest so far. |
| sweep_2 | de | `Kühlung` is the process, not the electricity; `Südglas` is unidiomatic. | Die hohe Sommersonne trifft das Südglas steil: Der Kühlstrom sinkt auf den bisher kleinsten Wert. |
| sweep_3 | en | `the curve peak` is unclear (E is the curve); cooling needs its electricity name. | Facing west, the low evening sun floods the room: cooling electricity and the total peak. |
| sweep_3 | de | `Kühlung und Kurve steigen an` mixes process and curve; `Kurve` is not the physical quantity. | Nach Westen flutet die tiefe Abendsonne den Raum: Kühlstrom und Strombedarf erreichen ihr Maximum. |
| sweep_4 | en | 96 characters (over 95); `cooling is least` lacks the electricity term. | Facing north, no sun hits the glass by day: cooling electricity is least, heating rises. |
| sweep_4 | de | `Kühlung minimal` and `Heizstrom steigt` mixed; `Glas` fine. | Nach Norden trifft tagsüber keine direkte Sonne das Glas: Kühlstrom minimal, Heizstrom steigt wieder. |
| local | en | `north-west` should be one word (glossary spelling). | Start with the façade facing northwest, 315°, and always turn toward less electricity. |
| local | de | `Wir starten mit Nordwest` is not idiomatic. | Wir starten mit der Fassade nach Nordwest, 315°, und drehen stets dorthin, wo der Strombedarf sinkt. |
| local_2 | de | `Nachbarrichtung` should be `Nachbarausrichtung` (glossary). | Jede Nachbarausrichtung ist schlechter, also halten wir an: lokales Minimum bei 360°, Nord. |
| global_2 | en | `priced here in kWh and CHF` treats kWh as a price; the sentence mixes units and money. | Stopping in the north valley wastes electricity every year, and that costs CHF as well. |
| global_2 | de | Same error as en (kWh is not a price). | Der Halt im Nordtal verschwendet jedes Jahr Strom, und das kostet auch CHF. |
| argument | en | `kWh per year` breaks R3. | The optimal design is θ* = 180°, south. Its value E(θ*) is the electricity in kWh/a. |
| argument | de | `pro Jahr` breaks R3. | Der optimale Entwurf ist θ* = 180°, Süd. Sein Wert E(θ*) ist der Strombedarf in kWh/a. |
| argument_2 | en | `kWh per year` breaks R3. | Keep them apart: the design θ* is a direction; the value E(θ*) is electricity in kWh/a. |
| argument_2 | de | Grammar: singular subject `Wert` with plural verb `sind`. | Nicht verwechseln: Der Entwurf θ* ist eine Richtung, der Wert E(θ*) ist der Strombedarf in kWh/a. |
| compare_2 | de | Clause missing its verb (`wie unser Start bei 315°` without `tat`). | Bei vielen Variablen kann eine Suche im falschen Tal enden, wie es unser Start bei 315° tat. |
| maximize | en | `how many kWh and CHF per year` breaks R3 and uses `per year` for a unit. | Now the opposite: how many kWh/a and CHF/a do we save against an east façade? |
| maximize | de | `Wie viele kWh` is ungrammatical (mass unit); `pro Jahr` breaks R3. | Nun umgekehrt: Wie viel sparen wir gegenüber einer Ostfassade, in kWh/a und CHF/a? |
| negate | en | `S` (saving) is not defined before use; line is 102 characters. | Minimizing E equals maximizing the saving S: valleys become peaks, the curve flips. |
| negate | de | `S` not defined; `heißt` (ß, R4). | E minimieren heisst die Einsparung S maximieren: Täler werden Gipfel, die Kurve klappt um. |
| constraint | de | ß (R4): `Straße`. | Die Strasse im Süden ist laut: Die Glasfassade muss zum ruhigen Hof zeigen, Nord ± 90°. |
| constraint_2 | de | `ergänzt kein Ziel` should be `Zielgröße`; `ändert keine Werte` is vague (en: no value of E). | Eine Nebenbedingung begrenzt die zulässigen Entwürfe. Sie fügt keine Zielgröße hinzu und ändert keinen Wert von E. |
| feasible | en | `global optimum` used for a value; the value is the global minimum. | Among the allowed orientations, north, 360°, is now the global minimum. |
| feasible | de | `Optimum` for a value; use `Minimum` (glossary). | Unter den zulässigen Ausrichtungen ist Nord, 360°, nun das globale Minimum. |
| feasible_2 | en | `extra kWh and CHF per year` breaks R3. | The quiet courtyard has a price: extra kWh/a and CHF/a compared with the free south optimum. |
| feasible_2 | de | `Südoptimum` compound; `pro Jahr` breaks R3. | Der ruhige Hof hat seinen Preis: Mehr kWh/a und CHF/a als beim freien Optimum im Süden. |
| scope_2 (move before scope) | en | Abstract closing; `window size` should be `window share`; move it before the concrete answer `scope`. | Change window share or climate, and the optimum moves: its value, perhaps its direction. |
| scope_2 (move before scope) | de | `Fenstergröße` (R-glossary); `verschiebt` clearer. | Ändern sich Fensteranteil oder Klima, verschiebt sich das Optimum: sein Wert, vielleicht seine Richtung. |
| scope | en | Closing is jargon imperative (`State direction, domain and scope`) and not an answer about this room. The constraint from `constraint` makes the answer north 360°, not south. Make it the final, concrete line. | For this room, with the courtyard rule θ in 270°–450°, the answer is north, 360°. |
| scope | de | Jargon imperative (`Nenne Richtung, Bereich, Geltung`); closing must be concrete. | Für diesen Raum mit der Hofregel θ zwischen 270° und 450° lautet die Antwort: Nord, 360°. |

### 2. Sampling (🎲️sampling)

Opening: statement (fix). Closing: abstract `limits_3`, which should end on a concrete answer for this south façade (fix; keep the validation remark as an inserted line before it).

| key | lang | problem | proposed replacement text |
|---|---|---|---|
| parameters | en | Opens with a statement, not a concrete question about the room. | Which window share and overhang depth suit this south façade? |
| parameters | de | Opening statement; also the list lacks `und`. | Welcher Fensteranteil und welche Vordachtiefe passen zu dieser Südfassade? |
| run | en | `heat` and `light` are ambiguous (Wärme/Licht); `plays` is informal. | One simulation runs through a whole year of sun, heat and daylight. |
| run | de | `Licht` should be `Tageslicht`; `spielt ... durch` is informal for a simulation. | Eine Simulation durchläuft ein ganzes Jahr mit Sonne, Wärme und Tageslicht. |
| run_2 | en | `saves heat` treats thermal heat as the saving; the objective is electricity (R9). Line length kept at 85. | Winter sun cuts heating electricity, summer sun adds cooling, daylight cuts lighting. |
| run_2 | de | `spart Heizen` and `spart Licht` (Licht = light, not electricity; Heizen = verb). | Wintersonne senkt den Heizstrom, Sommersonne erhöht den Kühlstrom, Tageslicht senkt den Beleuchtungsstrom. |
| run_3 | en | `electricity per year` breaks R3; `One number` needs the object. | One number per design: electricity in kWh/a, carried by each point. |
| run_3 | de | `Strom pro Jahr` breaks R3; Strom should be Strombedarf (E). | Eine Zahl je Entwurf: Strombedarf in kWh/a, getragen von jedem Punkt. |
| grid | en | `takes three levels` is unidiomatic for a design choice. | A full grid uses three levels per axis and combines them all. |
| grid | de | `nimmt` unidiomatic. | Ein volles Raster nutzt drei Stufen je Achse und kombiniert alle. |
| grid_2 | en | `Run them` is not the glossary term. | Simulate them one by one: room, sun rays and electricity bar for each. |
| grid_2 | de | `rechnen` is not the glossary term (R8). | Wir simulieren sie nacheinander: Raum, Sonnenstrahlen und Strombalken für jeden Entwurf. |
| grid_3 | en | `run` (R8). | The ninth simulation empties the budget. |
| grid_3 | de | `Lauf` (R8). | Die neunte Simulation leert das Budget. |
| growth | en | `factors` is the experiments term; sampling uses design variable (glossary). | More design variables: orientation with four levels, insulation with three. |
| growth | de | `Faktoren` and `Orientierung` replaced by `Entwurfsvariablen`, `Ausrichtung`. | Mehr Entwurfsvariablen: Ausrichtung mit vier, Dämmung mit drei Stufen. |
| growth_3 | en | `runs` (R8). | Every extra level multiplies the simulations: here twelve times our budget. |
| growth_3 | de | `Läufe` (R8). | Jede weitere Stufe vervielfacht die Simulationen: hier zwölfmal unser Budget. |
| random | en | `pairs` should be designs (each pair is one design). | Random sampling draws nine independent designs, recorded so everyone sees the same points. |
| random | de | `Paare` should be `Entwürfe`; sentence needs a verb after the appositive. | Zufälliges Sampling zieht neun unabhängige Entwürfe, gespeichert, damit alle dieselben Punkte sehen. |
| gaps | en | `a wide window with a deep overhang` is not WWR terminology. | This box stays empty: no draw combines a large window share with a deep overhang. |
| gaps | de | `großes Fenster und tiefes Vordach` in a sentence with `keine Ziehung hat`: unidiomatic; ß (R4). | Leere Box: Keine Ziehung kombiniert grossen Fensteranteil mit tiefem Vordach. |
| lhs | en | `one sample in each` is vague. | Latin hypercube sampling: nine strata per axis, exactly one sample in each stratum. |
| lhs | de | `je eine Stichprobe` is vague. | Latin-Hypercube-Sampling teilt jede Achse in neun Schichten, je Schicht genau eine Stichprobe. |
| marginals | en | `one window range` is not glossary term. | This vertical strip is one window-share band and holds exactly one design. |
| marginals | de | `Fensterbereich` could mean window area; use glossary. | Dieser senkrechte Streifen ist ein Band des Fensteranteils und enthält genau einen Entwurf. |
| marginals_2 | en | `holds exactly one, too` is clumsy. | The horizontal strip also holds exactly one: each axis is covered evenly. |
| marginals_2 | de | Missing verb: `jede Achse gleichmässig abgedeckt` (ß, R4). | Auch der waagerechte Streifen enthält genau einen: Jede Achse ist gleichmässig abgedeckt. |
| reveal | en | `sets` is ambiguous; name them point sets. | To judge the point sets, reveal the surface with a dense reference grid. |
| reveal | de | `Menge`, `freilegen` ok; name as point sets. | Zur Beurteilung der Punktmengen legen wir die Fläche mit einem dichten Referenzraster frei. |
| reveal_2 | de | `Strom` should be `Strombedarf` (E); `heißt` (R4); `im Inneren` lacks its referent. | Dunkler heisst weniger Strombedarf. Der tiefste Punkt liegt im Inneren des Bereichs. |
| compare | en | `each set's` and `dense grid` vs `reference grid` (reveal) are inconsistent. | Compare each point set's best design with the optimum of the dense reference grid. |
| compare | de | `Menge`, `Rasters`: same inconsistency. | Vergleiche den besten Entwurf jeder Punktmenge mit dem Optimum des dichten Referenzrasters. |
| compare_2 | de | `Jeder Beste` unclear. | Jeder beste Entwurf ist der beste von neun, doch keiner erreicht das Optimum. |
| limits_3 (insert before) | en | Keeps the validation remark (currently the closing) as an inserted line before the concrete closing. | The model itself still needs validation. |
| limits_3 (insert before) | de | Same. | Und das Modell selbst braucht noch eine Prüfung. |
| limits_3 | en | Closing is abstract (validation); must close on a concrete answer about this room. | For this south façade, nine simulations found a good design, not a proven optimum. |
| limits_3 | de | Same as en. | Für diese Südfassade fanden neun Simulationen einen guten Entwurf, aber kein bewiesenes Optimum. |

### 3. Experiments (🧪️experiments)

Opening: first line is a method statement (fix: make it name this room; line 2 is the first concrete question). Closing: `purpose_4` is abstract (fix). Jargon `bilinear` / `product term` appears without a room example (fix in `model`). Terminology: `window size` (must be window share), `runs` (must be simulations), `kilowatt hours per year` (R3).

| key | lang | problem | proposed replacement text |
|---|---|---|---|
| question | en | Opens with method statement; `evaluate` (R8). | Sampling asks where to simulate. Experiments start with a question about this room. |
| question | de | `auswerten` (R8); no room reference in the first line. | Sampling fragt, wo wir simulieren. Versuchsplanung beginnt mit einer Frage zu diesem Raum. |
| question_2 | en | `window size` (WWR is relative); `summer cooling` lacks the electricity term (R9). | How do window share and overhang change the summer cooling electricity of our seminar room? |
| question_2 | de | `Fenstergröße` (R4, R-glossary) and `Kühlung` instead of `Kühlstrom`. | Wie verändern Fensteranteil und Vordach den sommerlichen Kühlstrom unseres Seminarraums? |
| question_3 | en | `Only window` is not the variable name. | Sun, room and weather stay fixed. Only window share and overhang change. |
| question_3 | de | `Nur Fenster` likewise. | Sonne, Raum und Wetter bleiben fest. Nur Fensteranteil und Vordach ändern sich. |
| factors | en | `percent` (R2: `30 %`); WWR not defined at first use. | Factor A is the window-to-wall ratio (WWR): 30 % or 70 %. |
| factors | de | `Prozent` (R2); WWR not defined at first use. | Faktor A ist der Fensterflächenanteil (WWR): 30 % oder 70 %. |
| factors_2 | en | `none, or one metre`: `metre` (R2 `m`); `none` ambiguous. | Factor B is the overhang depth: 0 m (no overhang) or 1 m. |
| factors_2 | de | `keines oder ein Meter` ambiguous and not unit-spaced. | Faktor B ist die Vordachtiefe: 0 m (kein Vordach) oder 1 m. |
| factors_3 | en | `setting` is not the level term used in the clip (`levels`). | Code every low level as minus one and every high level as plus one. |
| factors_3 | de | `Einstellung` replaced by `Stufe`. | Kodiere jede niedrige Stufe als minus eins und jede hohe als plus eins. |
| plan | en | `runs all four combinations` (R8). | A full two-by-two plan simulates all four combinations of A and B. |
| plan | de | `führt ... aus` (R8). | Ein vollständiger Zwei-mal-zwei-Plan simuliert alle vier Kombinationen von A und B. |
| plan_2 | en | `the window grows` should be `window share grows`. | Each corner is one summer room: along A the window share grows, along B the overhang. |
| plan_2 | de | `das Fenster` should be `der Fensteranteil`. | Jede Ecke ist ein Sommerraum: Entlang A wächst der Fensteranteil, entlang B das Vordach. |
| response | de | `Energie` alone can be read as electricity; specify solar energy (R9). | Jeder Strahl trägt gleich viel Sonnenenergie. |
| response | en | `energy` alone (R9). | Every ray carries the same amount of solar energy. |
| response_2 | en | `kilowatt hours per year` (R3). | The model returns each room's cooling electricity in kWh/a. Lower is better. |
| response_2 | de | `Kilowattstunden pro Jahr` (R3). | Das Modell liefert den Kühlstrom jedes Raums in kWh/a. Kleiner ist besser. |
| response_3 | de | `Jahres-Lehrmodell` vs `Jahresmodell` (glossary). | Ein vereinfachtes Jahresmodell, keine Messung. |
| plot | en | `window size` (WWR); `Plot` as imperative; `cooling` unnamed. | We plot cooling electricity against window share, with one line per overhang depth. |
| plot | de | `Kühlung`, `Fenstergröße` (R4, glossary). | Wir tragen den Kühlstrom über dem Fensteranteil auf, eine Linie je Vordachtiefe. |
| plot_2 | en | `Each point is the room beside it` is unclear; `one metre` (R2). | Each point is one room: orange without overhang, green with 1 m. |
| plot_2 | de | `zugehörige Raum` unclear. | Jeder Punkt ist ein Raum: orange ohne Vordach, grün mit 1 m. |
| saving_30 | en | `percent` (R2); `blocks little sun and saves little` is vague (which electricity). | At 30 % the window catches few rays, so the overhang saves little cooling electricity. |
| saving_30 | de | `Prozent` (R2); `spart wenig` unnamed. | Bei 30 % fängt das Fenster wenige Strahlen, daher spart das Vordach wenig Kühlstrom. |
| saving_70 | en | `percent` (R2); `saves more` unnamed; line 94 ok. | At 70 % the window catches many more rays, so the same overhang saves much more. |
| saving_70 | de | `Prozent`; 100 characters; `spart mehr` unnamed. | Bei 70 % fängt das Fenster viel mehr Strahlen, daher spart dasselbe Vordach deutlich mehr. |
| interaction_2 | en | `size of the window` (WWR, R-glossary). | The effect of the overhang depends on the window share. |
| interaction_2 | de | `Fenstergröße` (R4). | Die Wirkung des Vordachs hängt vom Fensteranteil ab. |
| main | en | `mean` (use `mean` consistently); `70` and `30` lack `%` / `percent` (R2). | Main effect of A: the mean at 70 % minus the mean at 30 % window share. |
| main | de | `Mittel` is not the term; `Prozent` (R2). | Haupteffekt A: Mittelwert bei 70 % minus Mittelwert bei 30 % Fensteranteil. |
| main_2 | de | `Mittel` as in main. | Haupteffekt B: Mittelwert mit Vordach minus Mittelwert ohne Vordach. |
| difference_2 | en | `bigger windows` (WWR) and `pays more` vague; `Half the difference` repeats the previous line. | Half that difference is the interaction: a bigger window share makes the overhang save more. |
| difference_2 | de | `Fenster`, `wertvoller` vague; `größer` (R4). | Halbe Differenz ist die Wechselwirkung: Je grösser der Fensteranteil, desto mehr spart das Vordach. |
| model | en | `runs` (R8); `bilinear`, `product term` introduced without the room example (didactic). | Four simulations fit a bilinear model: a constant, two slopes and a product term, A times B. |
| model | de | `Läufe` (R8); `Produktterm` without example. | Vier Simulationen ergeben ein bilineares Modell: eine Konstante, zwei Steigungen und einen Produktterm A·B. |
| model_2 | en | `runs` (R8). | Setting A and B to zero predicts the plan's middle: the mean of all simulations. |
| model_2 | de | `Läufe` (R8). | A und B gleich null sagen die Planmitte voraus: den Mittelwert aller Simulationen. |
| model_3 | en | `kilowatt hours per year per coded step` (R3). | Each coefficient is half an effect, in kWh/a per coded step. |
| model_3 | de | `Kilowattstunden pro Jahr je kodiertem Schritt` (R3). | Jeder Koeffizient ist ein halber Effekt, in kWh/a je kodiertem Schritt. |
| curvature | en | `response` unexplained; name the quantity (R9). | Two levels only draw straight lines. The true cooling electricity may bend between them. |
| curvature | de | `Antwort` is not the term; name the quantity. | Zwei Stufen zeichnen nur Geraden. Der wahre Kühlstrom kann dazwischen gekrümmt verlaufen. |
| confirm | en | `run`, `50 percent glazing`, `0.5 metre` (R2, R8). `centre` is British and fine. | So we add a confirmation simulation at the middle: 50 % window share, 0.5 m overhang. |
| confirm | de | `Bestätigungslauf`, `Prozent`, `Meter` (R2, R8). | Darum eine Bestätigungssimulation in der Mitte: 50 % Fensteranteil, 0,5 m Vordach. |
| confirm_2 | en | `than the room actually needs` treats the model as if it were the room; the comparison is with the simulation. | The model predicts more cooling electricity than the simulation returns. |
| confirm_2 | de | Same error as en. | Das Modell sagt mehr Kühlstrom voraus, als die Simulation liefert. |
| confirm_3 | en | `half metre` (R2); `the response bends` (jargon). | The first 0.5 m of overhang already delivers most of the saving: the curve bends. |
| confirm_3 | de | `halben Meter` (R2); `Antwort` (jargon); ß (R4). | Schon die ersten 0,5 m Vordach bringen den Grossteil der Einsparung: Die Kurve krümmt sich. |
| repeat | de | `Rauschschätzung` is a compound that is hard to say; `wieder` is weak. | Dieselbe Simulation liefert erneut genau denselben Wert: keine Schätzung des Rauschens. |
| repeat_2 | en | `repeat runs` (R8). | Measured rooms scatter: physical experiments repeat measurements to estimate variability. |
| repeat_2 | de | `Läufe` (R8). | Gemessene Räume streuen: Reale Versuche wiederholen Messungen, um die Streuung zu schätzen. |
| purpose | en | `runs` (R8); `to answer` vague. | Plan simulations to answer the question, estimate effects and confirm. |
| purpose | de | `Läufe` (R8); `antworten` vague. | Plane Simulationen, um die Frage zu beantworten, Effekte zu schätzen und zu bestätigen. |
| purpose_2 | en | `runs` (R8); `fraction` is jargon for a fractional plan. | Add insulation as factor C: a full plan needs eight simulations, a fractional plan only four. |
| purpose_2 | de | `Läufe` (R8); `Dämmung als Faktor C` (glossary: Dämmstärke t). | Dämmstärke als Faktor C: Der volle Plan braucht acht Simulationen, ein Teilplan nur vier. |
| purpose_3 | en | `A B` is not an operator; `the two are confounded` lacks the effect names. | Here C moves exactly with the interaction A×B: the two effects are confounded. |
| purpose_3 | de | `A B` as in en. | Hier ändert sich C genau wie die Wechselwirkung A·B: beide Effekte sind vermengt. |
| purpose_4 | en | Closing is abstract (`fractional designs ... document the confounding`); `One number for both together` unclear. Close on a concrete room answer. | For our room, insulation as factor C saves four simulations, but its effect hides in A×B. |
| purpose_4 | de | Same as en; `dokumentiere` is fine as du-imperative but the closing must be concrete. | Für unseren Raum spart Dämmung als Faktor C vier Simulationen, aber ihr Effekt steckt in A·B. |

### 4. Index (⚖️index)

Opening: statement (fix). Closing: `next` is a pointer to another clip, not a concrete answer (fix). TITLE de: adjective `Normierter` must be lower case. Terminology: `variant` (must be design), `Strom` / `Electricity` (must be Strombedarf pro m² / electricity intensity, R10), `Carbon` / `CO₂` (must be embodied carbon / Graue Emissionen, model title), `Daylight` (must be daylight factor), `weniger` (must be kleiner), `Ihre` (capital formal form in a non-formal sentence).

| key | lang | problem | proposed replacement text |
|---|---|---|---|
| TITLE | de | Adjective `Normierter` capitalized; should be lower case as in the English adjective. | Mehrere Kriterien und ein normierter Index |
| variants | en | Opens with a statement; `variants` banned (R-glossary: design). | Which of five façade designs suits this seminar room best? |
| variants | de | Opens with a statement; `Variante` banned. | Welcher von fünf Fassadenentwürfen passt am besten zu diesem Seminarraum? |
| criteria | en | `Electricity` is the intensity kWh/(m²·a) (R10), not the total. | Electricity intensity and cost: lower is better. |
| criteria | de | `Strom` is ambiguous (R10); `weniger` should be `kleiner`. | Strombedarf pro m² und Fassadenkosten: kleiner ist besser. |
| criteria_2 | en | `Daylight` and `Carbon` are not the model criterion names (`Daylight factor`, `Embodied carbon`). | Daylight factor: higher is better. Embodied carbon: lower is better. |
| criteria_2 | de | `Tageslicht`, `CO₂`, `weniger` vs model names (`Tageslichtquotient`, `Graue Emissionen`). | Tageslichtquotient: mehr ist besser. Graue Emissionen: kleiner ist besser. |
| best | en | `No variant` (glossary). | No design is best everywhere. |
| best | de | `Variante` (glossary). | Kein Entwurf ist überall der beste. |
| units | en | Electricity intensity is per m² and per year; the line should say so (R10). | Kilowatt-hours per square metre per year, plus francs? That sum has no unit. |
| units | de | Same as en. | Kilowattstunden pro Quadratmeter und Jahr plus Franken? Die Summe hat keine Einheit. |
| units_3 | en | `Count in thousands (kCHF, tonnes)` unclear: it counts cost and carbon in different units. | Count cost in kCHF and carbon in tonnes instead, and the ranking changes. |
| units_3 | de | Same as en; `in Tausendern` unclear. | Zähle stattdessen Kosten in kCHF und Emissionen in Tonnen, und die Rangfolge ändert sich. |
| ruler | de | `Strom` (R10). | Gib jedem Kriterium ein eigenes Lineal, zuerst dem Strombedarf pro m². |
| ruler_2 | en | `variant` (glossary). | Each design sits at its real value; the room shows what the value means. |
| ruler_2 | de | `Variante`, `Bedeutung` vague. | Jeder Entwurf liegt auf seinem echten Wert; der Raum zeigt, was er bedeutet. |
| shift | en | `the best value of the set` is vague (set = the five designs). | Step one: subtract the best of these five values. |
| shift | de | `der Menge` vague. | Schritt eins: Ziehe den besten dieser fünf Werte ab. |
| daylight | en | `Daylight` (criterion name); `flipped` is fine but `mirrored` is the term. | Daylight factor: more is better, so the ruler is mirrored. |
| daylight | de | `Tageslicht`; `umgedreht` vs `gespiegelt`. | Tageslichtquotient: mehr ist besser, also wird das Lineal gespiegelt. |
| matrix | en | `four comparable values` but the clip has five designs; `Carbon` (glossary). | Embodied carbon follows the same rule: its five values now lie between zero and one. |
| matrix | de | `vier` (five designs); `CO₂`. | Graue Emissionen folgen derselben Regel: Ihre fünf Werte liegen nun zwischen null und eins. |
| weights_2 | en | Criterion names (R10, glossary). | Here: electricity intensity 40 %, cost 30 %, daylight factor 20 %, embodied carbon 10 %. |
| weights_2 | de | `Strom`, `CO₂`, `Tageslicht` (R10, glossary). Ihre in the matrix line is capitalised (see index). | Hier: Strombedarf pro m² 40 %, Kosten 30 %, Tageslichtquotient 20 %, Graue Emissionen 10 %. |
| index | de | `Ihre` (formal address) capitalised in a non-formal sentence; the pronoun refers to `Teile`. | Schiebe die gefüllten Teile zusammen: ihre Länge ist der Index. |
| sweep_2 | de | `Entscheide` (archaic plural); use `Entscheidungen`. | Knappe Entscheidungen sind fragil: Prüfe die Rangfolge, bevor du entscheidest. |
| range | en | `variants` (glossary). | The rulers depend on the designs compared: add a sixth façade. |
| range_3 | de | `gleich blieben` is less clear than `unverändert blieben`. | Die Reihenfolge der ersten fünf kann kippen, obwohl sie unverändert blieben. |
| record | en | `variants` (glossary); `ranges` vague. | Document weights, compared designs and value ranges with every decision. |
| record | de | `Varianten` (glossary). | Dokumentiere Gewichte, verglichene Entwürfe und Wertebereiche bei jeder Entscheidung. |
| sensitivity_3 | en | `variant` (glossary). | Another façade wins, though no design changed. |
| sensitivity_3 | de | `Variante` (glossary). | Eine andere Fassade gewinnt, obwohl sich kein Entwurf geändert hat. |
| cost | en | `Cost` is the model criterion `Façade cost`. | Façade cost gets the same ruler: the cheapest façade becomes zero. |
| cost | de | `Die Kosten` vs model `Fassadenkosten`. | Die Fassadenkosten erhalten dasselbe Lineal: Die günstigste Fassade wird null. |
| next (new, insert before) | en | Keeps the Pareto pointer as a separate line before the concrete closing. | To see trade-offs before weighting, look at Pareto fronts. |
| next (new, insert before) | de | Same. | Um Zielkonflikte vor dem Gewichten zu sehen, betrachte Pareto-Fronten. |
| next | en | Closing is a pointer to another clip; must close on a concrete answer for this room. | For this room at these weights, the winner has the lowest index. |
| next | de | Same as en. | Für diesen Raum gewinnt bei diesen Gewichten der kleinste Index. |

### 5. Pareto (🏆️pareto)

Opening: statement (fix). Closing: `decision_2` is abstract (fix). Terminology: `Dachüberstand` (must be Vordach), `Stromverbrauch` / `Jahresstrom` (must be Strombedarf), `Ziel` / `goal` (must be Zielgröße / objective), `evaluation` (must be simulation), `window` (must be window share), `kWh/a every year` redundancy (R3), `Dachüberstand` in walk_2.

| key | lang | problem | proposed replacement text |
|---|---|---|---|
| room | en | Opens with a statement, not a concrete question; `window`, `overhang` not glossary terms. | Which window share, insulation and overhang suit this seminar room's glazed façade? |
| room | de | Opens with a statement; `Dachüberstand` (glossary: Vordach); `Glasfassade` replaced by `verglaste Fassade`. | Welcher Fensteranteil, welche Dämmung und welches Vordach passen zur verglasten Fassade dieses Seminarraums? |
| room_2 | de | `Stromverbrauch` vs `Strombedarf` (R-glossary). | Ein vereinfachtes Jahresmodell liefert für jede Fassade Kosten und Strombedarf. |
| objectives | de | `Jahresstrom` vs `Strombedarf`. | Beides soll klein sein: Kosten links, Strombedarf unten. |
| objectives_2 | en | `cost across` vague (horizontal axis). | Every façade becomes one point: cost along the horizontal, electricity up. |
| objectives_2 | de | `Kosten rechts` says where the value lies, not the axis; `Stromverbrauch` (glossary). | Jede Fassade wird ein Punkt: Kosten waagerecht, Strombedarf senkrecht. |
| grid | en | `evaluate`, `window size`, `overhang` (R8, glossary). | We simulate every combination of window share, insulation and overhang. |
| grid | de | `bewerten` (R8), `Fensteranteil ... Dachüberstand` (Vordach). | Wir simulieren jede Kombination aus Fensteranteil, Dämmung und Vordach. |
| grid_2 | en | `price` vs `cost` (model criterion is cost); `electricity` unqualified. | Point at any tile: room, cost and electricity follow. |
| grid_2 | de | `Preis` vs `Kosten`; `Strom` vs `Strombedarf`. | Zeige auf eine Kachel: Raum, Kosten und Strombedarf folgen. |
| dominated | en | `more glass` should be `a larger window share`. | Compare two façades: this one has a larger window share and more insulation. |
| dominated | de | `mehr Glas` vs `größeren Fensteranteil` (grösseren, R4). | Vergleiche zwei Fassaden: Diese hat einen grösseren Fensteranteil und mehr Dämmung. |
| rule | de | `Strom` vs `Strombedarf`. | Dominanz: bei Kosten und Strombedarf nicht schlechter. |
| rule_2 | de | `einem` unclear; `Ziel` (objective). | Und bei mindestens einer der beiden Zielgrößen echt besser. |
| front | en | `beat all cheaper ones` unclear: beat in electricity. | Sweep from cheap to expensive: keep only façades whose electricity beats all cheaper ones. |
| front | de | Grammar: `Es bleiben, die ...` lacks the subject `Fassaden` and the object of `schlagen`. | Von günstig zu teuer: Es bleiben nur Fassaden, deren Strombedarf alle günstigeren unterbietet. |
| approximate | en | `unevaluated` (R8). | Between the grid points, an unsimulated design may still beat a front member. |
| approximate | de | `ungeprüfter` ok; `Punkten` should name the grid. | Zwischen den Rasterpunkten kann ein ungeprüfter Entwurf ein Frontmitglied schlagen. |
| sets | en | `evaluation` (R8). | The room is design space, the plot is objective space; simulation maps one onto the other. |
| sets | de | `Auswertung` (R8); sentence fragment. | Der Raum ist der Entwurfsraum, das Diagramm der Zielraum; die Simulation bildet das eine auf das andere ab. |
| walk_2 | en | `every year` duplicates `/a` (R3); `overhang` costs as step one (check: the chart lists changed variables; confirm the step changes only the overhang). | Step one: a 0.6 m overhang costs 192 CHF more and saves 65 kWh/a. |
| walk_2 | de | `Dachüberstand` (glossary); `pro Jahr` duplicates `/a` (R3); grammar `0,6 m Dachüberstand kosten` (plural verb). | Schritt eins: Ein 0,6 m tiefes Vordach kostet 192 CHF mehr und spart 65 kWh/a. |
| payback_2 | en | `only a faster payback is worth buying` is vague. | A façade lasts about 30 years: only steps that pay back within that lifetime are worth it. |
| payback_2 | de | Same as en. | Eine Fassade hält etwa 30 Jahre: Nur Schritte, die sich in dieser Zeit amortisieren, lohnen sich. |
| knee | de | `bis die Lebensdauer überschritten ist`: the payback exceeds the lifetime, not the lifetime is exceeded; `schwächer` ok. | Weiter: Jeder Schritt spart weniger, bis die Amortisation die Lebensdauer überschreitet. |
| knee_2 | en | `money buys a lot ... almost nothing` is informal; keep the knee metaphor concrete. | Here is the knee: before it, money buys much saving; after it, almost none. |
| knee_2 | de | `Hier das Knie` ungrammatical (`Hier ist das Knie`). | Hier ist das Knie: Davor bringt Geld viel Einsparung, danach kaum noch. |
| third | en | `goal` (glossary: objective). | Add daylight as a third objective: tile colour shows the daylight factor in %. |
| third | de | `Drittes Ziel` (glossary: Zielgröße). | Als dritte Zielgröße kommt das Tageslicht hinzu: Die Kachelfarbe zeigt den Tageslichtquotienten in %. |
| third_2 | en | `used less` vague (electricity). | In 2D it loses: a dimmer room is cheaper and needs less electricity. |
| third_2 | de | `sparsamer` vague; `Strom`. | In 2D verliert er: Ein dunklerer Raum ist billiger und braucht weniger Strom. |
| third_3 | en | `goals` (glossary). | With daylight counted, nobody beats it on all three objectives. |
| third_3 | de | `Zielen` (glossary). | Mit Tageslicht schlägt ihn keiner in allen drei Zielgrößen. |
| third_4 | en | `criterion` should be `objective` in pareto. | A two-dimensional picture can hide a trade-off: check every objective. |
| third_4 | de | `Kriterium` (glossary: Zielgröße). | Ein zweidimensionales Bild kann Zielkonflikte verbergen: Prüfe jede Zielgröße. |
| weights | en | `goals` (glossary). | To pick one façade, weigh the objectives, as in the index video. |
| weights | de | `Ziele` (glossary); `Indexvideo` should be `Index-Video`. | Um eine Fassade zu wählen, gewichten wir die Zielgrößen, wie im Index-Video. |
| decision | en | `bad designs` is vague; the term is dominated. | The front removes dominated designs; it does not choose. |
| decision | de | `schlechte Entwürfe` vague. | Die Front entfernt dominierte Entwürfe; sie wählt nicht. |
| decision_2 | en | Closing is abstract; close on the three cards shown (cheapest, knee, balanced). | Cheapest, knee or balanced: for this room, the front shows the choices; you decide. |
| decision_2 | de | Same as en. | Günstigste, Knie oder ausgewogen: Für diesen Raum zeigt die Front die Wahl; du entscheidest. |

### 6. Algorithms (🧬️algorithms)

Opening: already a concrete question (`task`), good. Closing: `close` is abstract (fix; insert the room answer). Terminology: `runs` (R8), `Läufe` (R8), `Fenster` for WWR, `Strom` for Strombedarf, `Design` in German (glossary: Entwurf), `Rekord`, `Elternteil` article. Numbers: digits 1 to 12 (R1).

| key | lang | problem | proposed replacement text |
|---|---|---|---|
| map | en | `colour is annual electricity` (`colour shows`); `kWh/a` ok. | The full map needs 1,681 simulations; colour shows annual electricity in kWh/a. |
| map | de | `Jahresstrom` (glossary: Strombedarf). | Die ganze Karte braucht 1.681 Simulationen; die Farbe zeigt den Strombedarf in kWh/a. |
| map_2 | en | `less window` (WWR). | The valley floor is the best design: less window share, deeper overhang. |
| map_2 | de | `weniger Fenster` (WWR); `Fenster` alone. | Der Talboden ist der beste Entwurf: weniger Fensteranteil, tieferes Vordach. |
| gradient | en | `75 % windows` (WWR). | Gradient descent starts at a 75 % window share and 0.1 m overhang. |
| gradient | de | `75 % Fenster` (WWR). | Der Gradientenabstieg startet bei 75 % Fensteranteil und 0,1 m Vordach. |
| gradient_2 | en | `5 runs per step` (R1, R8). | Four close neighbours reveal the slope: five simulations per step. |
| gradient_2 | de | `5 Läufe` (R1, R8). | Vier nahe Nachbarn verraten die Steigung: Fünf Simulationen pro Schritt. |
| gradient_3 | en | `More window` (WWR); `electricity` for Strombedarf is fine. | More window share raises the electricity here, more overhang lowers it. |
| gradient_3 | de | `Fenster` (WWR); `Stromverbrauch` (glossary: Strombedarf). | Mehr Fensteranteil erhöht hier den Strombedarf, mehr Vordach senkt ihn. |
| overshoot | en | `to too little window` is unidiomatic. | The first step overshoots the valley floor, to too small a window share. |
| overshoot | de | `zu wenig Fenster` unidiomatic; `schießt` (R4: schiesst). | Der erste Schritt schiesst über den Talboden hinaus, auf einen zu kleinen Fensteranteil. |
| overshoot_2 | de | `Strom` (glossary: Strombedarf). | Wiederholt zickzackt der Pfad; manche Schritte erhöhen sogar den Strombedarf. |
| gradient_result | en | `its best design` has no clear subject; `map minimum` vs `minimum`. | After 50 simulations, gradient descent's best design lies within 1 % of the map minimum. |
| gradient_result | de | `sein bestes Design` (glossary: Entwurf, Deutsch); unclear subject. | Nach 50 Simulationen liegt der beste Entwurf des Gradientenabstiegs innerhalb von 1 % des Kartenminimums. |
| gradient_result_2 | en | `runs` (R8); `Four of five` (R1 for simulations). | Four of every five simulations probe slopes; noise or a second valley can mislead them. |
| gradient_result_2 | de | `Läufen` (R8). | Vier von fünf Simulationen tasten Steigungen ab; Rauschen oder ein zweites Tal können sie täuschen. |
| pattern_2 | en | `windows` (WWR); `4 runs` (R1, R8). | From the same start it tries window share and overhang, up and down: four simulations. |
| pattern_2 | de | `Fenster` (WWR); `4 Läufe` (R1, R8); `größer` (R4). | Vom selben Start probiert sie Fensteranteil und Vordach, grösser und kleiner: Vier Simulationen. |
| pattern_result | en | `After 49 simulations it is within 1 %` contradicts `genetic_result_2` (16 simulations to reach 1 %); state the first reach, then the end. | Pattern search first reaches within 1 % of the minimum after 16 simulations, with no slope. |
| pattern_result | de | Same contradiction; `Minimums`. | Die Mustersuche erreicht innerhalb von 1 % des Minimums nach 16 Simulationen, ganz ohne Steigung. |
| population | en | `8 designs` (R1). | A genetic algorithm keeps eight designs; window share and overhang are its genes. |
| population | de | `8 Entwürfe` (R1); `Fenster` (WWR). | Ein genetischer Algorithmus hält acht Entwürfe; Fensteranteil und Vordach sind seine Gene. |
| population_2 | en | `8 runs` (R1, R8). | Each card shows one design and its simulated electricity: eight simulations. |
| population_2 | de | `8 Läufe` (R1, R8); `Strom` (glossary). | Jede Karte zeigt einen Entwurf und seinen simulierten Strombedarf: acht Simulationen. |
| tournament | de | `wird Elternteil` needs an article; `Strom` (glossary). | Auswahl: Zwei Karten zufällig ziehen; die mit weniger Strombedarf wird ein Elternteil. |
| crossover | en | `takes windows` (WWR). | Crossover: the child takes window share from one parent, overhang from the other. |
| crossover | de | `Die Fenster` (WWR). | Kreuzung: Das Kind erbt den Fensteranteil vom einen, das Vordach vom anderen Elternteil. |
| children | en | `Six children ... 6 runs` mixes spelled and digit numbers (R1, R8). | Six children, one simulation each: six simulations per generation. |
| children | de | `Sechs Kinder ... 6 Läufe` (R1, R8). | Sechs Kinder, je eine Simulation: sechs Simulationen pro Generation. |
| survival | en | `the record never worsens` (`record` is informal; glossary: best value). | Elitism: the two best parents survive, so the best value never gets worse. |
| survival | de | `Rekord` informal; `Die zwei besten` (R-style: beiden). | Elitismus: Die beiden besten Eltern überleben, so verschlechtert sich der Bestwert nie. |
| generations | en | `7 generations` (R1). | Repeat for seven generations: the cloud contracts and the best card improves in jumps. |
| generations | de | `7 Generationen` (R1). | Wiederhole sieben Generationen: Die Wolke schrumpft, die beste Karte verbessert sich sprunghaft. |
| genetic_result | en | `the minimum`: name the map minimum (as in gradient_result). | After 50 simulations the best card is within 1 % of the map minimum. |
| genetic_result | de | `des Minimums` as en. | Nach 50 Simulationen liegt die beste Karte innerhalb von 1 % des Kartenminimums. |
| genetic_result_2 | en | `runs` (R8); `the others over twice as many` ambiguous. | Within 1 %: pattern search needs 16 simulations, the others more than twice as many. |
| genetic_result_2 | de | `Läufe` (R8); `Die anderen` ok. | Bis 1 %: Die Mustersuche braucht 16 Simulationen, die anderen mehr als doppelt so viele. |
| compare | de | `über den Simulationen` unclear (plotted against); `bester Strom` (glossary). | Gleiches Modell, gleiches Budget: der bisher beste Strombedarf, aufgetragen gegen die Simulationen. |
| compare | en | `against simulations used` ok; `electricity` ok. | Same model, same budget: the best electricity so far, plotted against simulations used. |
| compare_2 | en | `runs` (R8); `within 2 kWh/a` lacks the reference (of the map minimum, verify). | Pattern search gets there first; after 50 simulations all three lie within 2 kWh/a. |
| compare_2 | de | `ist zuerst da` informal; `Läufen` (R8). | Die Mustersuche ist zuerst am Ziel; nach 50 Simulationen liegen alle drei innerhalb von 2 kWh/a. |
| proof | en | `1,681-run map` (R8). | None of them proves the global optimum; only the full map of 1,681 simulations could show it. |
| proof | de | `Karte mit 1.681 Läufen` (R8). | Keines beweist das globale Optimum; nur die volle Karte mit 1.681 Simulationen könnte es zeigen. |
| tools_2 | en | `runs` (R8). | Choose by smoothness, noise and simulation budget; a population copes with several valleys. |
| tools_2 | de | `Läufen` (R8); `Glattheit` (use Glätte). | Wähle nach Glätte, Rauschen und Simulationsbudget; eine Population meistert mehrere Täler. |
| close (new, insert before) | en | Keeps the abstract statement as an inserted line before the concrete closing. | Algorithms choose what to simulate next; model and architect decide what counts. |
| close (new, insert before) | de | Same. | Algorithmen wählen die nächste Simulation; Modell und Architekt entscheiden, was zählt. |
| close | en | Closing is abstract; close on a concrete answer for this room (from compare_2). | For this room, all three methods land within 2 kWh/a of the map minimum after 50 simulations. |
| close | de | Same. | Für diesen Raum landen alle drei Methoden nach 50 Simulationen innerhalb von 2 kWh/a des Kartenminimums. |

## ß list (rule R4, Swiss ss)

Current occurrences of ß in narration (16 lines), and the proposed Swiss form:

| clip | key | current | proposed |
|---|---|---|---|
| extrema | negate | heißt | heisst |
| extrema | negate_2 | größte | grösste |
| extrema | constraint | Straße | Strasse |
| extrema | scope_2 | Fenstergröße (removed by proposal) | none (Fensteranteil) |
| sampling | gaps | großes | grosses |
| sampling | gaps_3 | gleichmäßige | gleichmässige |
| sampling | marginals_2 | gleichmäßig | gleichmässig |
| sampling | reveal_2 | heißt | heisst |
| experiments | question_2 | Fenstergröße (removed by proposal) | none (Fensteranteil) |
| experiments | plot | Fenstergröße (removed by proposal) | none (Fensteranteil) |
| experiments | interaction_2 | Fenstergröße (removed by proposal) | none (Fensteranteil) |
| experiments | difference_2 | größer | grösser |
| experiments | confirm_3 | Großteil | Grossteil |
| index | units_2 | größten | grössten |
| algorithms | overshoot | schießt | schiesst |
| algorithms | pattern_2 | größer | grösser |

Pareto contains no ß. Keep the ß spelling only if the course is meant for German readers; in that case every proposal above that uses `grösser`, `heisst`, `Strasse`, `Grossteil` or `schiesst` must be reverted to ß.

## Didactic check

| Clip | Opens with a concrete question about the room? | Closes with a concrete answer? | Jargon without room example |
|---|---|---|---|
| extrema | No (statement); fixed by `formulate` | No (abstract `scope_2`, implicit `scope`); fixed | `bearing`, `feasible`, `constraint` (defined in same line, ok) |
| sampling | No (statement); fixed by `parameters` | No (validation remark); fixed by `limits_3` | `strata` (introduced with nine slices per axis in the lhs row; ok) |
| experiments | No (method statement; first room question on line 2); fixed by `question` | No (`purpose_4` abstract); fixed | `bilinear`, `product term` (fixed by `model`); `fraction` (fixed by `purpose_2`) |
| index | No (statement); fixed by `variants` | No (`next` pointer); fixed by `next` | `normalized index`, `ruler` (explained by the room in `ruler_2` and `units`) |
| pareto | No (statement); fixed by `room` | No (`decision_2` abstract); fixed | `dominance`, `front`, `knee` (all tied to room examples; ok) |
| algorithms | Yes (`task` is a concrete question about the room) | No (`close` abstract); fixed by `close` | `elitism`, `crossover`, `mutation` (defined in their lines; ok) |

## Cross-clip notes (outside NARRATION, same terminology issue)

- TITLE dicts: index de title (above). Other TITLEs are consistent and in order.
- On-screen labels that carry the old terms and should be aligned when the narration changes: `Licht` in extrema `_cards` and sampling `_bars`/`_key` (should be Beleuchtung); `Kühlung`/`Heizen` as electricity labels; index `Electricity` criterion label (should show intensity, R10); `Carbon` / `CO₂` label in index `SHORT` (should be Graue Emissionen).
- Sampling growth labels `Orientierung` and `Fenster`/`Fensteranteil` mix; index and extrema use `Fensteranteil` / `window share`.
- Sampling footnote `Vereinfachtes Jahres-Lehrmodell` vs extrema and pareto `vereinfachtes Jahresmodell`; use `vereinfachtes Jahresmodell` everywhere (glossary).
- Sampling uses `Sampling` (de) as a loanword in German titles and labels; keep it, it is the course term.
- Index `Strom` label in `SHORT` uses `Strom`; use Strombedarf (R10).
- Verify before recording: (a) index electricity criterion unit `kWh/(m²·a)` is correct for the narration in `units`; (b) `walk_2` step one changes only the overhang; (c) `compare_2` "within 2 kWh/a" refers to the map minimum.
