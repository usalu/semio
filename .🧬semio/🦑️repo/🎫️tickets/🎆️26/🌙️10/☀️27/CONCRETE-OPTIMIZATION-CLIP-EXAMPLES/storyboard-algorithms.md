# Storyboard: Optimization Algorithms (`🧬️algorithms`, `Beat6_EvolutionarySearch`)

Phase 1 only (no source edits). Baseline reviewed with `capture.py`: the current scene is three abstract parabolas (gradient / derivative-free / population), 0-1 "genes", designs `P2/P3`, a target point at (0.25, 0.75). All of that is replaced. Pedagogical points kept from `present_algorithms`: three method families (gradient-based, derivative-free/direct, evolutionary), the genetic loop evaluation -> selection -> crossover -> mutation -> survival, budgets and stopping, repeated generations, restarts, no proof of a global optimum for a black-box simulation, confirm with a finer check.

## Concept

One question carried through the whole clip: **which window share and overhang give the seminar room (south façade, 16 cm insulation) the least annual electricity, when every design costs one annual simulation?**

- **Landscape** = annual electricity E [kWh/a] over WWR 10-80 % (x) and overhang 0-1.2 m (y), drawn as heat-map plus contour map straight from `surface()` (41 x 41 = `{runs_map}` simulations).
- **Fog of search**: the map is the truth the solver does not see. After the reveal it dims to a faint "fog"; every simulated design appears as a dot painted in the map colour of its own electricity. At the end of each method the fog lifts again, so the viewer sees where the walk really went.
- **Always visible companion**: the room section (window height = WWR, slab depth = overhang, summer/winter rays) and the stacked electricity bar (heating / cooling / lighting, live kWh/a and total) show the current iterate. Moving the map pointer moves the room; the bar shrinks as the walk improves.
- **Cost is visible**: a run tape (one square per simulation, budget `{budget}`) plus a counter "23 / 50 simulations" fills with every evaluation. Gradient descent visibly pays for its slopes.
- **Three methods, three colours, one pointer** (same start for gradient and pattern search), then one chart: best-so-far kWh/a against simulations for all three.

Colour key (fix after capture, contrast first): gradient = `P_WHITE`, pattern = `P_ORANGE`, genetic = `P_YELLOW`; electricity bar per api.md (heating red, cooling blue, lighting yellow); map ramp = dark teal luminance ramp (`P_DEEP_DARK` -> `P_SECONDARY`, <= 60 % opacity, so orange/yellow/white paths read); crossover provenance: parent 1 = `P_SECONDARY`, parent 2 = `P_ORANGE`; mutation delta = `P_PRIMARY`; map minimum = `P_GREEN`.

No `A/B/C`, no `P1`, no 0-1 axes, no `f(x)`: cards are identified by their thumbnail and values ("45 % / 0.60 m / 8,240 kWh/a"); `id`s from `genetic()` stay internal.

## Screen Layout (16:9, Manim frame 14.22 x 8, stage between title and caption)

Stage y in [-2.60, 2.58] (SAFE_BOTTOM / SAFE_TOP), x in [-6.9, 6.9]. Title top-left, caption band y in [-4.0, -2.62]. Three layouts share one grid.

```
 L1 SEARCH (tasks, gradient, pattern)                                   y
 +--------------------------------------------------------------+       2.58
 | Annual electricity [kWh/a]    [run tape ▮▮▮▮▮▮▯▯▯▯] 23/50 sim|       2.45
 | Overhang d [m]                                               |
 | 1.2 +----------------------+     ROOM SECTION (south)  | bar  |       2.1
 |     |   HEAT MAP + CONTOURS|     window ∝ WWR, slab = d  | ▇ 3 |
 |     |   fog, path, pointer |     sun rays, lamps dimmed  | ▇ 2 |
 |  0  +----------------------+                             | ▇ 1 |
 |     10        45        80   WWR [%]                      | tot |      -0.5
 |  colour bar [kWh/a]          chips: WWR 45 % · d 0.60 m          |      -0.9
 |                              method strip (slopes / probes / step)|     -1.5..-2.4
 +--------------------------------------------------------------+       -2.60
   x: map -6.2..-1.2, room -0.5..4.7, bar 5.3 (labels to 6.9)
   map plot 5.0 x 3.7 (y -1.65..2.05), ticks y -1.9, axis labels y -2.3

 L2 GENETIC                                    L3 COMPARE
 map as L1 (cloud of 8 yellow dots)            chart: simulations (0..budget) vs best kWh/a
 +cards 4 x 2 (1.2 x 1.05) x -0.5..4.7,        x -6.5..0.3, y -1.9..2.1, three step-lines,
   rows y 1.65 / 0.5: thumbnail, "45 % · 0.6 m",  dashed green line = map minimum
   "8,240 kWh/a", mini bar                      right x 1.6..6.9: three rows (y 1.7/0.2/-1.3):
 + focus room (scale .6) x -0.5..2.6, y -2.4..-0.2   method name · thumbnail · kWh/a · runs
 + chips (generation, parent pair) x 3.0..4.7        live: thumbnails morph with best-so-far
 bar column x 5.3 as L1 (best card or focus card)

 L4 TOOLS: node diagram full width: two sliders -> simulation box -> kWh/a -> solver loop
```

## Placeholders (every number comes from the model, formatted per language)

`NARRATION` is generated from templates with `{name}` filled by `number()` (en `1,681`, de `1.681`); never typed by hand.

| Placeholder | Source |
| --- | --- |
| `{runs_map}` | len(`surface()["wwr"]`) x len(`surface()["overhang"]`) = 1,681 |
| `{opt_wwr}` `{opt_d}` `{opt_kwh}` | `surface()["minimum"]` (WWR %, overhang m, kWh/a) |
| `{start_wwr}` `{start_d}` `{start_kwh}` | `gradient_path()[0]` (equals `pattern_path()[0]`) |
| `{g_per_step}` | runs per gradient step = 4 probes + new point (= 5) (**new field**) |
| `{s_wwr}` `{s_d}` | slope at the start, kWh/a per WWR-%-point and per metre (**new field**, signed) |
| `{g_final}` `{g_runs}` / `{p_final}` `{p_runs}` | last iterate electricity and cumulative evaluations (**new field**) |
| `{noise}` | study noise level in % (**new**, optional beat) |
| `{pop}` `{gen}` `{n_children}` | population size 8, generation count, children per generation (6 with two elites) |
| `{m_wwr}` `{m_d}` | `mutation` deltas of the featured child (signed, %-points and m) |
| `{ga_final}` `{ga_runs}` | best of last generation, cumulative evaluations |
| `{budget}` `{spread}` | common run budget (~50); max-min of the three bests at the budget |

## Beat Table

Column "State" is the screen after the clause; "Animation" is passed to `self.step(key, ...)` (use `explain` only for rows without animation). Clause time is about 5-6 s each (reading speed plus VO); total about 5:00 with all rows, 4:30 without the optional rows, 3:50 with the cut list (see the end of the table).

### 1. The task (0:00-0:20)

| Key | State | Animation | Linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `task` | L1: empty map axes (WWR %, overhang m), room section at baseline design, bar at baseline total | FadeIn axes, room, bar, header | `evaluate(baseline)` | Seminar room, south façade: which window share and overhang need the least electricity? | Seminarraum, Südfassade: Welche Fenster und welches Vordach brauchen am wenigsten Strom? |
| `task_2` | Baseline dot on the map, tape square 1, counter "1 / `{budget}`" | dot appears, one tape square pops, counter ticks | baseline kWh/a = dot colour = bar total | Each design costs one annual simulation, so we can only afford a few. | Jeder Entwurf kostet eine Jahressimulation, wir können uns nur wenige leisten. |
| `task_3` | Same; "solver" chip beside the pointer | chip fades in | - | A solver decides which design to simulate next. | Ein Solver entscheidet, welchen Entwurf er als Nächstes simuliert. |

### 2. The hidden landscape (0:20-0:50)

| Key | State | Animation | Linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `map` | Heat map revealed, colour bar in kWh/a, counter races 1 -> `{runs_map}` | image fades in by sweep from low WWR to high, contours drawn, counter/tape speed-ramp (tape shows a compressed 1,681 sliver), pointer follows the sweep and the room morphs fast | `surface()` grid | The full map needs {runs_map} simulations; colour is annual electricity in kWh/a. | Die ganze Karte braucht {runs_map} Simulationen; Farbe ist der Jahresstrom in kWh/a. |
| `map_2` | Map minimum star `P_GREEN`, pointer parked there, room + bar show the optimum | pointer flies to the minimum, `set_design`, label "{opt_kwh} kWh/a" | `surface()["minimum"]` | Its valley bottom lies at {opt_wwr} % window share and {opt_d} m overhang. | Der Talboden liegt bei {opt_wwr} % Fensteranteil und {opt_d} m Vordach. |
| `map_3` | Map dims to fog (about 25 %), tape and counter reset to 1, pointer back to the start, optimum star hidden | map opacity down, star fades, tracker returns to start | start design | In practice we see only the points we simulate; the rest stays in the fog. | In der Praxis sehen wir nur simulierte Punkte; der Rest bleibt im Nebel. |

### 3. Gradient descent (0:50-1:55)

| Key | State | Animation | Linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `gradient` | Start dot in `P_WHITE` ring, method label "Gradient descent", strip empty | label slides in, ring pulses | `gradient_path()[0]` | Gradient descent starts at {start_wwr} % windows and {start_d} m overhang. | Der Gradientenabstieg startet bei {start_wwr} % Fenster und {start_d} m Vordach. |
| `gradient_2` | Four probe crosses N/S/E/W of the start; five tape squares in white; counter +5 | probes pop one after another, each flies one square into the tape; probe dots painted with their own map colour | probes of iterate 0 | It probes four neighbours for the slope, plus the new point: {g_per_step} runs per step. | Es prüft vier Nachbarn für die Steigung plus den neuen Punkt: {g_per_step} Läufe pro Schritt. |
| `gradient_3` | Strip chips: "Slope windows: -38 kWh/a per %" and "Slope overhang: -210 kWh/a per m" (units, no symbols); probes dim | chips write on; the two probe pairs connect by thin secants | `slope` | Slope: {s_wwr} kWh/a per window percent, {s_d} kWh/a per metre of overhang. | Steigung: {s_wwr} kWh/a je Fensterprozent, {s_d} kWh/a je Meter Vordach. |
| `gradient_4` | Downhill arrow (length = step) from the dot; pointer moved to iterate 1; room window larger, slab deeper; bar total lower with "-x kWh/a" delta chip | arrow grows, pointer follows, `set_design`, bar shrinks, delta chip | iterate 1 | The arrow points downhill; the room follows and the bar shrinks. | Der Pfeil zeigt bergab; der Raum folgt, der Balken schrumpft. |
| `gradient_5` | Trail of iterates 2..n, counter ends at iterate-n runs, tape filled in 5-blocks | montage: for each iterate probe flash -> arrow -> move (0.5 s each), room morphs, bar shrinks, tape + counter; `seconds` about 7 | `gradient_path()` | Repeat: probe, slope, step. Every step adds {g_per_step} runs to the counter. | Wiederholen: prüfen, Steigung, Schritt. Jeder Schritt erhöht den Zähler um {g_per_step}. |
| `overshoot` | Replay of the iterate that overshoots: long arrow crosses the valley floor, pointer lands on the far wall, bar jumps up with red "+x kWh/a" chip, room shows the oversized design | trail segment redrawn highlighted, bar flash `P_PRIMARY` | the overshooting iterate (**model requirement**) | A step that is too long jumps across the valley and the electricity rises again. | Ein zu langer Schritt springt über das Tal, und der Stromverbrauch steigt wieder. |
| `overshoot_2` | Shorter arrow from there, zigzag trail visible, step-length chip "step 0.5 x" | arrow shrinks, pointer moves back | next iterate | A shorter step lands closer; the descent zigzags down the valley. | Ein kürzerer Schritt trifft besser; der Abstieg zickzackt durchs Tal. |
| `gradient_result` | Pointer at the end, fog lifts to 60 % behind the trail, result chip "{g_final} kWh/a / {g_runs} simulations", room and bar at the result | map opacity up, chip FadeIn, "dense map minimum" star shown for comparison | last iterate, `{opt_kwh}` | Result: {g_final} kWh/a after {g_runs} simulations, most of them spent on slopes. | Ergebnis: {g_final} kWh/a nach {g_runs} Simulationen, die meisten für Steigungen. |
| `gradient_result_2` | Tape recoloured: 4/5 probes (grey-white) vs 1/5 steps | tape squares regroup into probe vs step share | probe share | Noise, jumps or a second valley can mislead a slope; it finds the nearest valley. | Rauschen, Sprünge oder ein zweites Tal täuschen die Steigung; sie findet nur das nächste Tal. |

Fallback if `gradient_path()` has no overshoot: replace `overshoot`/`overshoot_2` by one beat `step` "A tiny step is safe but needs many runs; a large one may jump over the valley." / "Ein kleiner Schritt ist sicher, braucht aber viele Läufe; ein großer kann das Tal überspringen." with the arrow drawn twice (tiny = 6 runs, large = ghost landing across the valley). Prefer the model delivering a real overshoot (see model requirements).

### 4. Pattern (compass) search (1:55-2:45)

| Key | State | Animation | Linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `pattern` | Fog map wiped of the gradient trail (kept as faint grey ghost), pointer back at the same start, method label "Pattern search", tape reset in `P_ORANGE`, counter 1 | trail fades to ghost, pointer returns, label | `pattern_path()[0]` | Pattern search needs no slope: it only compares simulated designs. | Die Mustersuche braucht keine Steigung: Sie vergleicht nur simulierte Entwürfe. |
| `pattern_2` | Compass cross with arm length = step (chip "step 12 % / 0.15 m"), four probe dots painted in their map colour with delta chips ("-120", "+310" kWh/a vs centre, red = worse) | four arms extend, probes evaluated one by one, four tape squares, chips pop | probes of iterate 0 | From the same start it tries window share and overhang, each up and down: 4 runs. | Vom selben Start probiert sie Fenster und Vordach, je größer und kleiner: 4 Läufe. |
| `pattern_3` | Best probe ringed `P_GREEN`, pointer moves there, room morphs, bar shrinks, compass recentres | ring, move, `set_design`, arms re-draw | iterate 1 | The best neighbour wins; if it beats the centre, the room moves there. | Der beste Nachbar gewinnt; ist er besser als die Mitte, zieht der Raum um. |
| `pattern_4` | At the first stalled iterate: all four deltas positive (red); arms contract to half length, step chip "6 % / 0.075 m" | arms shrink with a halving tick, chip counts down | `pattern_path()` entry with `moved = false` | If no neighbour wins, halve the step and probe again. | Gewinnt kein Nachbar, halbiere den Schritt und probiere erneut. |
| `pattern_5` | Trail to the end, counter at the final count | montage like `gradient_5` (7 s): probe flash, move or halve, tape fills in 4-blocks | `pattern_path()` | Repeat; the step shrinks as the valley floor is approached. | Wiederholen; der Schritt schrumpft, je näher der Talboden rückt. |
| `noise` (optional) | Beside the compass: the gradient arrow from the first step wobbling over three recorded noise draws, compass pick unchanged; chip "noise ±{noise} %" | arrow jitters through the draws, compass ring stays | recorded noisy probes (**optional model**) | Simulation noise of ±{noise} % shakes a slope, but the ranking of four values holds. | Simulationsrauschen von ±{noise} % verwackelt Steigungen, die Rangfolge von vier Werten hält. |
| `pattern_result` | Fog lifts 60 %, both trails on the map (white ghost, orange solid), chip "{p_final} kWh/a / {p_runs} simulations", room and bar at result | like `gradient_result` | last iterate | Result: {p_final} kWh/a after {p_runs} simulations, with no derivatives. | Ergebnis: {p_final} kWh/a nach {p_runs} Simulationen, ganz ohne Ableitungen. |

### 5. Genetic algorithm (2:45-4:00)

| Key | State | Animation | Linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `population` | Layout L2: room section replaced by 8 face-down cards, map back to fog, trails cleared; eight hollow `P_YELLOW` dots on the map; two gene pills ("window share", "overhang") under a legend | room slides down to the small focus slot, cards deal in, tape recoloured yellow | `genetic()["generations"][0]["population"]` | A genetic algorithm keeps {pop} designs at once; window share and overhang are its genes. | Ein genetischer Algorithmus hält {pop} Entwürfe; Fensteranteil und Vordach sind seine Gene. |
| `population_2` | Cards flipped: thumbnail, "45 % / 0.60 m", "8,240 kWh/a", mini bar; map dots filled with the same colour as the card bar; counter +8 | LaggedStart flip, each flips with a tape square, dot paints in; best card ringed and drives focus room + bar | initial population | Each card shows one design and its simulated electricity: {pop} runs. | Jede Karte zeigt einen Entwurf und seinen simulierten Strom: {pop} Läufe. |
| `population_3` | Dots on the map pulse in sequence, cloud covers several regions | Indicate cloud, hull dashed | initial dots | Spread over the map, they explore several regions at once. | Über die Karte verteilt erkunden sie mehrere Bereiche zugleich. |
| `tournament` | Two random cards lifted out of the grid, "VS" between them, lower kWh/a wins and gets a `P_SECONDARY` frame (parent 1); their map dots ring | cards slide out, compare, loser sinks back, ring on map | `tournaments[0]` (**new field**) | Selection: pick two cards at random; the one with less electricity becomes a parent. | Auswahl: Zwei Karten zufällig ziehen; die mit weniger Strom wird Elternteil. |
| `tournament_2` | Second duel, winner framed `P_ORANGE` (parent 2); chip "random draws are recorded" (portable seed) | duel replays faster | `tournaments[1]` | Good designs are chosen more often, yet weak ones keep a chance. | Gute Entwürfe werden öfter gewählt, schwache behalten eine Chance. |
| `crossover` | Grid dims; two big façade drawings (parents, `facade()` elevation) left, child façade right; gene pills: WWR pill of parent 1 flies into the child, overhang pill of parent 2 flies in; child window height = parent 1, overhang = parent 2; kWh/a of the child shown after one tape square | pills detach and fly (curved path), child façade assembles, `set_design` on the focus room | featured pair of `children[0]["genes"]` | Crossover: the child takes the window share from one parent, the overhang from the other. | Kreuzung: Das Kind erbt den Fensteranteil vom einen, das Vordach vom anderen Elternteil. |
| `crossover_2` | Map: dashed rectangle spanned by the two parent dots, child dot at a corner, projection lines to both axes in the parents' colours | rectangle draws, drop lines, child dot appears | parent and child designs | On the map the child sits at a corner of the box spanned by its parents. | Auf der Karte liegt das Kind an einer Ecke des von den Eltern gespannten Rechtecks. |
| `mutation` | Gene pills of the child show small red deltas ("+3 %", "-0.05 m"), child façade and map dot shift slightly; kWh/a before/after chip, ring red if worse / green if better | delta arrows on pills, dot jitters to the mutated spot, façade morphs, chip | `children[0]["mutation"]` | Mutation nudges the child a little: {m_wwr} % windows, {m_d} m overhang. | Mutation verschiebt das Kind leicht: {m_wwr} % Fenster, {m_d} m Vordach. |
| `mutation_2` | Second mutated child that gets worse (red ring) next to the first (green), both brief | quick replay of a second child | `children[1]` | A small random change keeps variety; it may help or hurt. | Eine kleine Zufallsänderung erhält die Vielfalt; sie kann nützen oder schaden. |
| `children` | Grid returns: six child cards fly into free slots (in the grid above the eight old cards), tape +6, counter | children LaggedStart (fast, 0.25 s each), each with a tape square and a map dot in `P_YELLOW` fill | all `children` of generation 1 | Every child needs its own simulation: {n_children} new runs per generation. | Jedes Kind braucht eine eigene Simulation: {n_children} neue Läufe pro Generation. |
| `survival` | All cards sorted by kWh/a; best (elite) gets a star, worst six fade with a `cross_mark`; map dots of the dropped designs fade | cards slide into rank order, elite badge pops, losers fade | `survival` result of generation 1 | Elitism: the best design always survives, so the record can never get worse. | Elitismus: Der beste Entwurf überlebt immer, der Rekord wird also nie schlechter. |
| `survival_2` | Next generation grid with eight cards (elites + best children), generation chip "2" | grid re-forms | generation 2 | Weak designs make room for the best children: the next generation. | Schwache Entwürfe weichen den besten Kindern: die nächste Generation. |
| `generations` | Generation chip counts 2 -> `{gen}`; map: cloud of eight dots contracts around the valley, best-card thumbnail and focus room morph per generation, bar shrinks, tape fills with 6-blocks; trail of "best" dot in yellow | montage (8 s): per generation quick card swap, cloud morph (dots Transform), best dot moves | all generations | Repeat for {gen} generations: the cloud contracts around the valley. | Wiederhole {gen} Generationen lang: Die Wolke zieht sich um das Tal zusammen. |
| `generations_2` | Mini history strip under the bar: best kWh/a per generation as steps, with flat stretches | strip draws | `best` per generation | The best card improves in jumps, not smoothly; the room and bar follow. | Die beste Karte verbessert sich sprunghaft; Raum und Balken folgen. |
| `genetic_result` | Fog lifts to 60 %, chip "{ga_final} kWh/a / {ga_runs} simulations", focus room at best design | like the other results | last generation | Result: {ga_final} kWh/a after {ga_runs} simulations. | Ergebnis: {ga_final} kWh/a nach {ga_runs} Simulationen. |
| `genetic_result_2` | Tape in yellow longer than the other two (cost comparison as three coloured bars, run counts) | three tape totals line up | `{g_runs}` `{p_runs}` `{ga_runs}` | It spends more runs, but it copes with jumps, noise and several valleys. | Es braucht mehr Läufe, kommt aber mit Sprüngen, Rauschen und mehreren Tälern zurecht. |

### 6. Compare, limits, tools (4:00-4:30)

| Key | State | Animation | Linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `compare` | L3: chart axes (simulations 0-`{budget}`, best electricity kWh/a), dashed green line "map minimum {opt_kwh} kWh/a"; right: three rows (method, thumbnail, kWh/a, runs); map and cards leave | stage swap; three step-lines draw simultaneously along a moving run cursor; thumbnails morph with the best-so-far design of each method | `convergence()` (**new**) | Same model, same budget: best electricity found so far, against simulations used. | Gleiches Modell, gleiches Budget: bester bisher gefundener Strom über den Simulationen. |
| `compare_2` | Curves complete; slope annotations (white steep, orange regular, yellow staircase) | Indicate each curve with its thumbnail | curves | Gradient descent drops fast, pattern search steadily, the genetic algorithm in jumps. | Der Gradientenabstieg fällt schnell, die Mustersuche stetig, der genetische Ansatz sprunghaft. |
| `compare_3` | Final points ringed, bracket between best and worst "spread {spread} kWh/a", each row shows gap to map minimum in kWh/a and %, three final room mini-sections | brackets, chips | final bests at `{budget}` | After {budget} runs the three designs differ by at most {spread} kWh/a. | Nach {budget} Läufen unterscheiden sich die drei Entwürfe um höchstens {spread} kWh/a. |
| `stopping` | Vertical dashed line at the budget, flat-tail band "improvement below tolerance", three rule chips (budget / tolerance / stagnation), one lit | lines draw, chips, the lit rule pulses | budget, tolerance (`{tol}` kWh/a over `{k}` runs, **new**) | Stop at a run budget, a tolerance or a stagnation rule, and report which one. | Stoppe bei Laufbudget, Toleranz oder Stagnation und berichte, welche Regel gegriffen hat. |
| `proof` | Tape zoom-out: the `{budget}` squares shrink to a sliver in a bar of `{runs_map}` ("3 % of the map"); map back at 60 % in a corner with the three end dots near the star | tape zoom, thumbnail of map slides in | `{budget}/{runs_map}` | None of them proves the global optimum; only the {runs_map}-run map could show it. | Keines beweist das globale Optimum; nur die Karte mit {runs_map} Läufen könnte es zeigen. |
| `restarts` (optional) | Three coloured start dots from other map corners with short trails to the same valley, seed chips | trails draw | extra starts (**optional model**) | So log seeds and settings, restart from other designs, and compare the results. | Protokolliere also Startwerte, starte von anderen Entwürfen neu und vergleiche. |
| `tools` | L4: node diagram (WWR slider, overhang slider, "annual simulation" box, kWh/a readout, "solver" box with return arrow); slider values equal the best design | wires pulse, sliders move on solver beats, readout counts | best design, `evaluate` | Parametric tools wire this up: sliders, a simulation and a solver for the next design. | Parametrische Werkzeuge verdrahten das: Regler, Simulation, Solver für den nächsten Entwurf. |
| `tools_2` (optional) | Three chips under the diagram: "smooth model: gradient", "noisy: pattern", "jagged / many valleys: genetic" | chips write on | - | Choose the solver by model smoothness, noise and the number of runs you can afford. | Wähle den Solver nach Glattheit, Rauschen und den bezahlbaren Läufen des Modells. |
| `confirm` | Winner room section large (L1 right side), bar, stamp "model result: check with a finer simulation" | `set_design` to winner, stamp | winner design | Check the winner with a finer simulation, then judge it as architecture. | Prüfe den Sieger mit einer feineren Simulation und beurteile ihn dann architektonisch. |
| `close` | Winner room + map with the three trails and the star, caption only | hold, slow fade | - | Algorithms choose what to simulate next; model and architect decide what counts. | Algorithmen wählen die nächste Simulation; Modell und Architekt entscheiden, was zählt. |

Number of clauses: 48, about 5:00 at 5-6 s each including animation holds. Four are optional (`noise`, `restarts`, `tools_2`, `mutation_2`): without them 44 clauses, about 4:30. Cut list for about 3:50 (drop in this order): the four optional ones, then `tournament_2`, `gradient_result_2`, `genetic_result_2`. Mentioned only in the `tools` beat: parametric solvers in architecture software, no brand names.

## Linked-Value Wiring (one set of trackers)

- `study_trackers(design)` with orientation fixed at 180 (south), insulation 16 cm; `wwr` and `overhang` trackers are the **only** state. Map pointer = `map_point(map, trackers)` (always_redraw), room section, façade/bar and the readout chips read the same trackers.
- Search animations never move a dot without moving the trackers: `set_design(trackers, iterate_design, run_time)` is the single move primitive; the dot, room and bar interpolate together (bar scale fixed to the map maximum, so shrinking is honest; dashed reference lines at the start total and at the map minimum).
- Cards (GA) are static `facade_card` objects built from `evaluate(design)`; the card in focus is copied into the trackers, never drawn twice with different numbers.
- Every probe, child, population member increments one `runs` ValueTracker; the tape and counter are `always_redraw` from it, so the displayed cost always equals the number of dots painted on the map.

## Toolkit Widgets Needed Beyond api.md

| Widget | Purpose |
| --- | --- |
| `heat_map(axes_spec, grid, ramp, levels, opacity)` | `surface()` as an `ImageMobject` (numpy array, nearest-neighbour) with stdlib marching-squares contours every few hundred kWh/a, labels on 3 contours, colour bar in kWh/a; separate opacity tracker for fog |
| `value_axes(x_range, y_range, x_unit, y_unit, ticks)` | physical axes with unit tick labels and axis titles, no LaTeX (same look as the energy `labeled_axes`) |
| `map_pointer(map, trackers, color, trail=True)` | live pointer with ring and optional trail polyline in method colour; `map_dot(map, design, color)` painted by the map ramp |
| `probe_cross(map, design, step, values, color)` | four probes (finite-difference or compass) with delta chips; `highlight_probe(i)`; `halve(cross)` animation |
| `gradient_arrow(map, design, slope, length, color)` | downhill arrow with a length-scaled label; slope chips with units |
| `run_tape(budget, colors)` and `runs` tracker | one square per simulation, budget end-stop, counter chip "n / budget simulations", `push(n, color)`, `regroup(shares)` |
| `facade_card(design, point, state)` | thumbnail (existing `thumbnail`) + "45 % / 0.60 m" + "8,240 kWh/a" + mini bar; states `normal`, `focused`, `parent1`, `parent2`, `elite`, `dropped`; `card_grid(cards, rows, cols)` |
| `gene_pills(design, colors)` | two pills with icons (window / overhang) and values; `fly_gene(pill, target)` for crossover; `delta_tick(pill, delta)` for mutation |
| `duel(card_a, card_b)` | "VS" tournament selection animation, lower kWh/a wins |
| `crossover_box(map, parent_a, parent_b, child)` | dashed rectangle with corner child and axis drop lines |
| `best_strip(values)` | small best-per-generation step history; `convergence_chart(series, budget, map_minimum)` with moving run cursor, three coloured step-lines and live thumbnail rows |
| `tape_compare(runs_by_method)` | three coloured totals (cost comparison) and the zoom-out sliver for `proof` |
| `solver_diagram(trackers)` | slider, simulation box, readout, solver loop for the tools beat; no brand names |
| `counter_chip(label, tracker, fmt)` | generic live counter ("Generation 3 / 7", "Step 4") |
| `noise_whisker` (optional) | wobbling arrow over recorded noise draws |

`ClipScene.build_narration(templates, values)` (or a module-level helper) fills `{placeholders}` per language with `number()`; `NARRATION` stays `[key, en, de]` rows.

## Model Requirements (additions to api.md, for the model agent)

1. `gradient_path()` items: `design`, `electricity`, `gradient` (kWh/a per %-point and per m, signed), `step` (design units), `probes` (4 designs with electricity), `runs` (this iterate), `evaluations` (cumulative). Start and step size chosen so one iterate overshoots (electricity rises or the pointer crosses the valley floor), then recovers; about 8-12 iterates; deterministic.
2. `pattern_path()` items: `design`, `electricity`, `step` ({wwr, overhang}), `probes` ([{direction, design, electricity}]), `moved` (bool), `evaluations`; at least one halving.
3. `genetic()`: per generation `evaluations` (cumulative), `tournaments` ([{candidates:[id,id], winner}]), `elites` (ids), `survivors` (ids), recorded draws/seed; population 8, 6 children per generation with 2 elites (8 + 6 per generation); at least one mutated child that gets worse and one that gets better; `best` monotone.
4. `convergence()`: `{"budget", "map_minimum", "runs_map", "series": {"gradient": [[runs, best], ...], "pattern": ..., "genetic": ...}}` all on the same run budget (about 50) with best-so-far monotone; the three bests at the budget within a few percent of the map minimum and differing visibly in shape (gradient fast, genetic stair-step).
5. `study["noise"]` plus recorded noise draws (optional; only for `noise`) and extra gradient starts (optional; only for `restarts`).
6. Every placeholder in the table above resolves from the model; the scene tests assert that each `NARRATION` row contains no unresolved `{...}` and that clause lengths stay below 95 characters after substitution (en and de).

## Capture / Verification Plan (Phase 2)

- `capture.py 🧬️algorithms Beat6_EvolutionarySearch en` and `de`, check every key; zero layout conflicts; no text under the caption band (stage ends at y = -2.60); cards, chips and the tape stay inside x in [-6.9, 6.9].
- Compare the room section and the bar against `evaluate(design)` for the pointer position in the frames (pointer and bar must agree in every beat).
- Rendering time: heat map as image (not 1,681 polygons); the montage beats use `seconds` up to 8; one step per iterate, no per-frame model calls other than the tracker-driven `evaluate`.
