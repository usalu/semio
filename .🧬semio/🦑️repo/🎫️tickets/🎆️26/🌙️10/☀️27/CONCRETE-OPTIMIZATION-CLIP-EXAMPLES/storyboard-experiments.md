# Storyboard: Design of Experiments (`🧪️experiments`, `Beat3_FactorialExperiments`)

Phase 1 (storyboard only). Scene file, class name, video id and wrapper stay unchanged. All numbers below are **placeholders** (marked `[p]`); at implementation time every number, including the ones inside narration clauses, comes from `factorial()` / `evaluate()`. Target length: 34 clauses, about 2,800 characters of narration, about 3 min 10 s speech plus transitions, so **about 3 min 30 s**.

## 1. What is wrong with the current scene

Captured with `T/capture.py` (`T/🗑️generated/frames/Beat3_FactorialExperiments-en-*.png`). The four "runs" are identical pavilion glyphs with an invented energy index (9, 5, 7, 19); the plot axis says `Response y` and `-1 → +1`; the model `ŷ = 10 + 3A + 2B + 4AB` and the curvature picture are abstract curves; there is no room, no sun, no unit. The pedagogical order is good and is kept: question first, coded levels, 2² plan, plot, main effects, interaction and formula, bilinear model, curvature, confirmation run, deterministic repeat vs physical replication, fractional designs confound.

## 2. Concept

One question, answered by **four summer room sections that are the data**. "How do window size (A: WWR 30 % vs 70 %) and overhang (B: 0 m vs 1.0 m) change the summer cooling electricity of the seminar room?" Every run is a room (summer sun at the model's summer altitude, rays blocked by the overhang or passing through the glass, sun patch on the floor, heat ripples where it lands, cooling bar in kWh/a). Every plotted point is linked to its room; every effect is shown as a pair of rooms (columns = A, rows = B, diagonals = A·B) and as a number in kWh/a with its meaning in plain words.

Three ideas make the physics visible and carry the interaction:

1. **One ray = one fixed amount of energy.** Rays are spaced uniformly along the window height. A 70 % window therefore receives many more rays than a 30 % window, and the same overhang blocks more rays (more kWh/a) on the big window. That is why the overhang saves 150 kWh/a at 30 % but 650 kWh/a at 70 %: the interaction is geometry, not statistics.
2. **The savings are drawn three times at once:** blocked rays in the sections, a green gap segment between the ghost bar and the real bar in the card, a vertical arrow between the two points in the plot. Same number, three views.
3. **Contrast geometry.** Effects are read off the 2×2 square: A = right column minus left column, B = lower row minus upper row, A·B = main diagonal minus anti-diagonal. The same highlights mark the rooms, the plot means and the formula.

Factor letters A/B/C are the coded statistics names and appear **only next to their physical name and unit** ("A · window size, WWR, %"). No `x`, `y`, `f(x)`, no normalized axis, no invented index.

### Placeholder numbers used below `[p]`

| Run | A | B | WWR | overhang | cooling kWh/a |
| --- | --- | --- | --- | --- | --- |
| r1 | −1 | −1 | 30 % | 0 m | 620 |
| r2 | +1 | −1 | 70 % | 0 m | 1,480 |
| r3 | −1 | +1 | 30 % | 1.0 m | 470 |
| r4 | +1 | +1 | 70 % | 1.0 m | 830 |

Derived by the scene from `factorial()` (arithmetic on its runs, never typed): savings `s30 = r1 − r3 = 150`, `s70 = r2 − r4 = 650`; means A: 545 → 1,155; means B: 1,050 → 650; effects `A = +610`, `B = −400`, `AB = −250` (= ½[(r4 − r2) − (r3 − r1)]); coefficients `b0 = 850`, `b1 = +305`, `b2 = −200`, `b12 = −125`; centre run predicted `850`, actual `760`, gap `−90` (−11 %), share of the 1.0 m saving already reached at 0.5 m `73 %`. Half fraction (C = insulation 10 / 30 cm, generator C = AB): alias contrast `−290` = `AB (−250) + C (−40)`.

## 3. Stage and layouts

Frame 14.22 × 8 (16:9). Title chip top-left (existing). **Stage** = the free band between title and caption: x ∈ [−6.8, 6.8], y ∈ [−2.45, 2.60] (13.6 × 5.05). No fixed formula panel is used (formulas are content strips inside the stage), so nothing reserves the bottom band except the caption. Captions are one or two lines (≤ 95 characters).

### L1 big room (clauses 1–6: question, factors)

```
y=2.6  ┌───────────────────────────────────┬──────────────────────────────────────────┐
       │ ☀ (-6.3, 2.35)                     │ RIGHT PANEL  x -0.2 … 6.8               │
       │   ╲ ╲ ╲ ╲   rays at summer altitude│  clauses 1-3: question card (3.3, 1.45)   │
       │ ══╗▒ overhang d  (window wall x=-6.5)│   "How do window size and overhang…"  │
       │   ║ window  ░░ room 6 m × 3 m       │   chips: A · window size · 30 … 70 %   │
       │   ║ (height ∝ WWR)                  │          B · overhang · 0 … 1.0 m     │
       │ ▬▬ sun patch + heat ripples         │          response · cooling kWh/a     │
       │ ▓▓▓▓▓▓▓▓▓░░░░░░░  850 kWh/a (bar)   │  clauses 4-6: two level sliders        │
y=-2.45└───────────────────────────────────┴──────────────────────────────────────────┘
```

Big room `BR`: scale 1.0 unit/m, centre (−3.45, −0.2) (x −6.5…−0.5, y −1.7…1.3), cooling bar under it at y −2.1 (width 6.0 = 0…1,600 kWh/a, live number at its end). Trackers `T_big = study_trackers(...)` shared by room, bar, sliders.

### L2 square of four rooms + design matrix (clauses 7–11)

```
y=2.6  ┌────────────────────────────────────────┬─────────────────────────────────────┐
       │ run 3 (A−1,B+1)      run 4 (A+1,B+1)   │ RIGHT PANEL x -0.2 … 6.8            │
       │ [thumb] 30 % · 1.0 m [thumb] 70 % · 1.0│  design matrix (x 0.2…6.4)          │
       │ ┌section 2.6×1.3┐ ↑B ┌section 2.6×1.3┐ │   Run │ A │ B │ WWR │ Overhang │ kWh/a│
       │ └───────────────┘    └───────────────┘ │    1  │−1 │−1 │ 30 %│  0 m    │ 620 │
       │ ▓▓░░ 470 kWh/a        ▓▓▓▓▓▓░░ 830     │    2  │+1 │−1 │ 70 %│  0 m    │1,480│
       │ run 1 (A−1,B−1)  →A  run 2 (A+1,B−1)   │    3  │−1 │+1 │ 30 %│ 1.0 m   │ 470 │
       │ … same cards …                          │    4  │+1 │+1 │ 70 %│ 1.0 m   │ 830 │
y=-2.45└────────────────────────────────────────┴─────────────────────────────────────┘
```

Card (`RC`): header 0.30 (thumbnail 0.55 × 0.28 + `A−1 · B+1` + `30 % · 1.0 m`, size ≈ 18), section 2.6 × 1.3 (0.433 unit/m, depth 6 m, height 3 m), horizontal cooling bar 2.6 × 0.22 (0…1,600 kWh/a) with live number. Card centres: columns x = −5.3 (A−1) and −2.3 (A+1); rows y = +1.2 (B+1) and −1.2 (B−1). Square edges: short arrows between cards labelled `A →` (between columns, y 0) and `B ↑` (between rows). Each card owns its own tracker set (fixed values), so nothing moves unless the scene animates it. This block **stays on screen from clause 7 to clause 24 unchanged** (no scaling transitions).

### L3 block + interaction plot (clauses 12–24)

Block as in L2 on the left. Right panel (x −0.2 … 6.8): formula/meaning strip y 1.85…2.55, plot axes box x 1.0…6.3 × y −1.75…1.50 (tick labels y −1.95, axis titles y −2.3 / rotated at x 0.2). Axes: x = `Window-to-wall ratio (%)`, range 20–80, ticks 30, 50, 70; y = `Cooling electricity (kWh/a)`, range 0–1,600, ticks every 400. d = 0 m: orange circles; d = 1.0 m: green squares (shape and colour redundant). Leader lines from each card bar end to its plot point live only during the fly-in.

### L4 centre room (clauses 25–30)

Block and plot are replaced on the left by one big room (same `BR` geometry as L1, new trackers at 50 % / 0.5 m) with cooling bar plus **ghost bar** (dashed outline at the bilinear prediction). Right panel: plot as in L3 but without the cards' leader lines, the dashed mid line for d = 0.5 m and the centre markers.

### L5 half fraction (clauses 31–34)

Block of four cards again (same slots), now with C (insulation) in the header and the wall layer drawn at 10 / 30 cm. Right panel: sign table with columns A, B, C, A·B and cooling; the C and A·B columns are identical and highlighted.

### Colour roles (energy palette)

Sun / rays `P_YELLOW`; blocked rays dimmed `P_YELLOW` at 35 % with a short `cross_mark` in `P_RED` at the overhang; heat ripples `P_ORANGE`; cooling bars and cooling numbers `P_BLUE`; overhang d = 0 m line `P_ORANGE`, d = 1.0 m line `P_GREEN`; savings arrows, savings segments and "saves" chips `P_GREEN`; effect numbers `P_YELLOW`; interaction, curvature gap and warnings `P_ORANGE`; model/prediction ghosts `P_PRIMARY` dashed; confounded columns `P_RED` frame.

## 4. Beat tables

`key_n` follow-ups are separate `step`s of the same `explain`. Time is a rough estimate of narration plus animation. Narration placeholders `{…}` are formatted per language (en `1,480`, de `1.480`; decimals `0.5` / `0,5`); `{±x}` renders as `plus 305` / `minus 200` (`plus`/`minus`, `plus`/`minus` in German) so the text speaks cleanly.

### Scene 1: the question (L1, about 24 s)

Start: stage empty except title.

| Key | Visual state after the clause | Animation | Linked values | English | German |
| --- | --- | --- | --- | --- | --- |
| `question` | Big room `BR` at the baseline (WWR 50 %, d 0.5 m) built from floor/walls; sun glyph top-left; question card slot empty. | `Create` room shell, `FadeIn` sun; `shine` rays (LaggedStart) through the glass, the blocked ones stop at the overhang; floor patch fades in with `ripples`; cooling bar grows with the live number. | `T_big` (WWR 50 %, d 0.5 m); bar = `evaluate(T_big).cooling` | Sampling asks where to evaluate. Experimental design starts with a question. | Sampling fragt, wo wir auswerten. Versuchsplanung beginnt mit einer Frage. |
| `question_2` | Question card on the right with three chips: A window size (WWR), B overhang depth, response cooling electricity in kWh/a. | Card `FadeIn`; the room demonstrates the question: WWR tracker 50 → 70 → 30 → 50 % (window grows, more rays, bar rises, then falls), then d 0.5 → 1.0 → 0 → 0.5 m (overhang grows, rays blocked, bar falls, then rises). | `T_big.wwr`, `T_big.overhang` swept; bar live | How do window size and overhang change the summer cooling of our seminar room? | Wie verändern Fenstergröße und Vordach die sommerliche Kühlung unseres Seminarraums? |
| `question_3` | Same; a thin "fixed" strip under the card: sun · room · weather · tariff (small icons with labels). | `FadeIn` strip; `Indicate` on the room outline once. | none | Sun, room and weather stay fixed. Only window and overhang change. | Sonne, Raum und Wetter bleiben fest. Nur Fenster und Vordach ändern sich. |

### Scene 2: factors and coded levels (L1, about 20 s)

| Key | Visual state | Animation | Linked values | English | German |
| --- | --- | --- | --- | --- | --- |
| `factors` | Right panel: level slider A (track 30 … 70 %, ticks at 30 and 70 %, faint centre tick at 50 %); marker bound to `T_big.wwr`. | Question card shrinks into the slider header; marker goes to 30 % then 70 % (room window shrinks/grows, bar follows), rests at 30 %. | `T_big.wwr` 30 → 70 → 30 %; bar live | Factor A is the window-to-wall ratio: 30 or 70 percent. | Faktor A ist der Fensteranteil: 30 oder 70 Prozent. |
| `factors_2` | Slider B (0 … 1.0 m, ticks at 0 and 1.0 m, centre tick 0.5 m); marker bound to `T_big.overhang`. | Marker goes to 0 m then 1.0 m (overhang extends, rays blocked, bar falls), rests at 0 m. | `T_big.overhang` 0 → 1.0 → 0 m | Factor B is the overhang depth: none, or one metre. | Faktor B ist die Vordachtiefe: keines oder ein Meter. |
| `factors_3` | Both sliders show coded labels `−1` under the low tick and `+1` under the high tick, `0` under the centre tick; coding line `A = (WWR − 50 %) / 20 %`, `B = (d − 0.5 m) / 0.5 m`. | Labels `FadeIn` with `LaggedStart`; coding line writes in; room cycles once through the four corner settings (30/0 → 70/0 → 70/1.0 → 30/1.0) with the sliders. | `T_big` set by `set_design` to the four corners | Code every low setting as minus one and every high setting as plus one. | Kodiere jede niedrige Einstellung als minus eins und jede hohe als plus eins. |

### Scene 3: the 2² plan (L2, about 20 s)

| Key | Visual state | Animation | Linked values | English | German |
| --- | --- | --- | --- | --- | --- |
| `plan` | Big room, question card and sliders fade out; four cards appear on the square: static rooms (no sun yet), headers with thumbnail, coded label and physical setting; design matrix on the right with columns Run, A, B, WWR, Overhang (cooling column still empty). | `clear_stage`; cards `FadeIn` LaggedStart in run order 1, 2, 3, 4 with `Indicate` on the matching matrix row; square edge arrows `A →`, `B ↑` `Create`. | card trackers = the four `factorial()["runs"][i]["design"]`; matrix cells = coded values and `design` fields | A full two-by-two plan runs all four combinations of A and B. | Ein vollständiger Zwei-mal-zwei-Plan führt alle vier Kombinationen von A und B aus. |
| `plan_2` | Label "2² = 4 runs" under the matrix; the thumbnails are emphasised. | Each thumbnail pulses once while its section `Indicate`s (only that card changes brightness); chip `2² = 4 runs` `FadeIn`. | none | Each corner is one summer room. The miniature facade shows what changed. | Jede Ecke ist ein Sommerraum. Die kleine Fassade zeigt, was sich geändert hat. |

### Scene 4: the responses (L2, about 24 s)

| Key | Visual state | Animation | Linked values | English | German |
| --- | --- | --- | --- | --- | --- |
| `response` | Summer sun in all four rooms: rays (equal spacing), blocked rays stopped at overhangs, floor patches, heat ripples; cooling bars filled with live numbers; matrix cooling column filled. | `shine` in all four rooms (LaggedStart); then bars grow from 0 with count-up numbers while the matching matrix cell types in (run by run, `Indicate` on the card). | `evaluate(run_i).cooling` for i = 1…4; ray count ∝ `solar_summer`, blocked share from `shading_summer` | The sun shines into all four rooms; the model returns the cooling electricity of each run. | Die Sonne scheint in alle vier Räume; das Modell liefert den Kühlstrom jedes Laufs. |
| `response_2` | Lowest bar (run 3) framed green, highest (run 2) framed orange; chip "lower is better". | Frames `Create`; chip `FadeIn`. | `argmin`, `argmax` of the four responses | {r1}, {r2}, {r3} and {r4} kilowatt hours per year. Lower is better. | {r1}, {r2}, {r3} und {r4} Kilowattstunden pro Jahr. Kleiner ist besser. |
| `response_3` | Footnote chip under the matrix: "Simplified annual teaching model, not measurements." (the one required disclaimer). | `FadeIn` chip; frames dim. | none | A simplified annual teaching model, not a measurement. | Ein vereinfachtes Jahres-Lehrmodell, keine Messung. |

### Scene 5: the plot (L3, about 18 s)

| Key | Visual state | Animation | Linked values | English | German |
| --- | --- | --- | --- | --- | --- |
| `plot` | Matrix fades; axes (WWR %, cooling kWh/a) with tick labels and light grid; four points (orange circles d = 0 m, green squares d = 1.0 m) with value tags; two connecting lines; legend `● overhang 0 m   ■ overhang 1.0 m`. | `Create` axes; each point flies from its card bar end to its position (short arc, trail fades, number tag follows) in run order; lines `Create`. | point (wwr %, cooling) = (`design.wwr·100`, `cooling`) of each run | Plot cooling against window size, with one line per overhang depth. | Wir tragen die Kühlung über der Fenstergröße auf, eine Linie je Vordachtiefe. |
| `plot_2` | Plot unchanged; color links card frame ⇄ point. | LaggedStart pulses: card frame and its plot point `Indicate` together (1, 2, 3, 4). | none | Every point is the room next to it: orange without overhang, green with one metre. | Jeder Punkt ist der zugehörige Raum: orange ohne Vordach, grün mit einem Meter. |

### Scene 6: the savings and the interaction (L3, about 40 s)

| Key | Visual state | Animation | Linked values | English | German |
| --- | --- | --- | --- | --- | --- |
| `saving_30` | Left column cards (runs 1 and 3) emphasised (others at 45 % opacity). In run 3 the ghost of run 1's bar sits behind the real bar; the gap segment is green with `−{s30} kWh/a`. In the section the blocked rays are dimmed with crosses at the overhang, the passing rays stay bright. Plot: vertical green arrow from the orange to the green point at 30 % with label `−{s30} kWh/a`. | Pulse the blocked rays of run 3; ghost bar `Create`; gap segment grows; `dim_arrow` + chip `GrowArrow`. | `s30 = r1 − r3`; blocked rays = `rays(d=1.0)` minus passing rays at 30 % | At 30 percent the overhang blocks few rays and saves {s30} kilowatt hours per year. | Bei 30 Prozent blockiert das Vordach wenig Strahlen und spart {s30} Kilowattstunden pro Jahr. |
| `saving_70` | Right column emphasised; run 4 with ghost of run 2 and green gap `−{s70} kWh/a`; many more blocked rays than in run 3 (the window is 2.3× taller); plot: second, longer arrow at 70 % `−{s70} kWh/a`. | Same sequence, slower (the bigger saving gets the longer animation); both arrows stay. | `s70 = r2 − r4` | At 70 percent the same overhang blocks many rays and saves {s70}. | Bei 70 Prozent blockiert dasselbe Vordach viele Strahlen und spart {s70}. |
| `interaction` | Both arrows labelled; a dashed grey "parallel" copy of the orange line is drawn starting at the green 30 % point, ending at the grey ghost end `parallel` at 70 %; the gap between the ghost end and the real green point is bracketed `500 kWh/a more saving`. | Ghost line `Create` (dashed); gap bracket `GrowFromCenter`; chip "non-parallel lines". | ghost y at 70 % = `r3 + (r2 − r1)` ; gap = `s70 − s30` | The savings differ, so the lines are not parallel: that is an interaction. | Die Einsparungen unterscheiden sich, die Linien sind nicht parallel: eine Wechselwirkung. |
| `interaction_2` | Chip on the plot: "Overhang effect depends on window size"; the two blocked-ray pictures at 30 % and 70 % pulse together. | `Indicate` on the blocked rays of run 3 and run 4 with the arrows. | none | The effect of the overhang depends on the size of the window. | Die Wirkung des Vordachs hängt von der Fenstergröße ab. |

### Scene 7: main effects and the interaction effect (L3, about 42 s)

Cards return to full brightness. The strip above the plot (y 1.85…2.55) carries the formula of each beat as a `math_label` row; a meaning chip sits under the strip.

| Key | Visual state | Animation | Linked values | English | German |
| --- | --- | --- | --- | --- | --- |
| `main` | Cards: left column (A−1) framed cyan, right column (A+1) framed yellow, with a bracket `mean 545 → mean 1,155 kWh/a`. Plot: hollow diamonds at 30 % (545) and 70 % (1,155) midway between the two lines, a dashed grey connector, yellow arrow `+610 kWh/a`. Strip: `A = (r2 + r4)/2 − (r1 + r3)/2 = 1,155 − 545 = +610 kWh/a`. Meaning chip: "Going from 30 to 70 % window adds 610 kWh/a, averaged over both overhangs." | Column frames `Create`; diamonds `FadeIn`; connector `Create`; arrow `GrowArrow`; formula `Write`. | `effects.a`, means from `runs` | Main effect of A: the mean at 70 minus the mean at 30 percent, +{eA} kilowatt hours per year. | Haupteffekt A: Mittel bei 70 minus Mittel bei 30 Prozent, +{eA} Kilowattstunden pro Jahr. |
| `main_2` | Rows framed (upper row B+1 green, lower row B−1 orange) with a bracket `mean 1,050 → mean 650 kWh/a`. Plot: two horizontal dashed mean lines at 650 and 1,050 kWh/a across the plot, green arrow between them `−400 kWh/a`. Strip: `B = (r3 + r4)/2 − (r1 + r2)/2 = 650 − 1,050 = −400 kWh/a`. Chip: "A 1 m overhang saves 400 kWh/a, averaged over both window sizes." | Replace the A overlay by the B overlay (`ReplacementTransform`); mean lines `Create`; arrow `GrowArrow`. | `effects.b` | Main effect of B: mean with overhang minus mean without, {eB} kilowatt hours per year. | Haupteffekt B: Mittel mit Vordach minus Mittel ohne Vordach, {eB} Kilowattstunden pro Jahr. |
| `difference` | Main diagonal (runs 1 and 4) framed purple, anti-diagonal (runs 2 and 3) framed cyan. Plot: the two savings arrows reappear (−150 at 30 %, −650 at 70 %); strip: `AB = ½ [ (r4 − r2) − (r3 − r1) ]`. | Diagonal frames `Create`; arrows `FadeIn`; formula `Write`. | `effects.ab` structure | The interaction effect is half the difference of the two savings. | Der Wechselwirkungseffekt ist die halbe Differenz der beiden Einsparungen. |
| `difference_2` | Strip evaluates with numbers: `AB = ½ [ (−650) − (−150) ] = −250 kWh/a`; chip: "The overhang pays off more the bigger the window." | Substitute numbers by `TransformMatchingShapes`; result `Indicate`. | `effects.ab` | Half of {s70} minus {s30} gives the interaction effect: {eAB} kilowatt hours per year. | Die Hälfte von {s70} minus {s30} ergibt den Wechselwirkungseffekt: {eAB} Kilowattstunden pro Jahr. |

### Scene 8: the bilinear model (L3, about 30 s)

| Key | Visual state | Animation | Linked values | English | German |
| --- | --- | --- | --- | --- | --- |
| `model` | Overlays and diagonal frames fade. Strip: `kWh/a = 850 + 305·A − 200·B − 125·A·B`. The two plot lines are recolored as "model" (they are the model: one straight line per overhang setting). Four coefficient chips under the plot: `850 kWh/a  mean of the four runs`, `+305  per +20 points of glazing`, `−200  per +0.5 m of overhang`, `−125  A·B`. | Strip `Write`; lines flash once; chips `LaggedStart`. | `coefficients = [b0, b1, b2, b12]` | The four runs fit a bilinear model: {b0} {±b1} A {±b2} B {±b12} A B. | Die vier Läufe ergeben ein bilineares Modell: {b0} {±b1} A {±b2} B {±b12} A B. |
| `model_2` | A faint dashed mid line at d = 0.5 m (average of the two lines) appears; a hollow star at WWR 50 % marks the model's value `850 kWh/a`; it is also shown as a dashed ghost bar in the future centre room slot (a small label "0 / 0 = middle of the plan"). | Mid line `Create`; star `FadeIn` with a drop line to both axes (`live_marker` drop lines). | `centre.predicted` (= b0) | Setting A and B to zero predicts the middle of the plan: {b0}, the mean of all four runs. | A und B gleich null sagen die Planmitte voraus: {b0}, der Mittelwert aller vier Läufe. |
| `model_3` | Coefficient chips get their meaning in kWh/a highlighted; a note "coefficient = half an effect" connects `+305` to `+610` and `−125` to `−250`. | Small arrows from effect to coefficient `Create`, labelled `÷ 2`. | `coefficients`, `effects` | Each coefficient is half an effect: kilowatt hours per year per coded step. | Jeder Koeffizient ist ein halber Effekt: Kilowattstunden pro Jahr je kodiertem Schritt. |

### Scene 9: curvature and the confirmation run (L4, about 38 s)

| Key | Visual state | Animation | Linked values | English | German |
| --- | --- | --- | --- | --- | --- |
| `curvature` | Block fades; the plot keeps the four points and straight lines and moves to the full right panel width (x −0.2…6.8 unchanged). Between 30 % and 70 % on the d = 0.5 m mid line, two semi-transparent alternative curves (one sagging, one bulging) through the same end points with a "?" at 50 %. Left: big room slot empty, label `middle of the plan: 50 % · 0.5 m`. | Alternatives `Create` (dashed, 40 % opacity); "?" pulses. | mid line end points = average of the two lines at 30 % and 70 % | Two levels only draw straight lines. The true response may bend between them. | Zwei Stufen zeichnen nur Geraden. Die wahre Antwort kann dazwischen gekrümmt sein. |
| `confirm` | Big room `BR` appears at 50 % window, 0.5 m overhang with rays, blocked rays, patch; cooling bar with the **ghost bar at 850 kWh/a** (dashed `P_PRIMARY` outline); the real bar grows to 760. | `FadeIn` room; ghost bar `Create`, then the real bar grows; marker "actual" (yellow filled) appears on the plot below the hollow star; vertical gap bracket. | `centre.design`, `centre.predicted`, `centre.actual` | So we add a confirmation run in the middle: 50 percent glazing, 0.5 metre overhang. | Darum ergänzen wir einen Bestätigungslauf in der Mitte: 50 Prozent Fenster, 0,5 Meter Vordach. |
| `confirm_2` | Gap segment in the bar and on the plot in orange with `−{gap} kWh/a ({gap%} %)`; text chip "model 850 · room 760". | Gap `GrowFromEdge`; chip `FadeIn`; `Indicate` on the star and the marker. | `predicted − actual`, relative gap | The bilinear model predicts {p}; the room delivers {c} kilowatt hours per year. | Das bilineare Modell sagt {p} voraus; der Raum liefert {c} Kilowattstunden pro Jahr. |
| `confirm_3` | The room shows both overhangs: the real 0.5 m (solid) and a dashed 1.0 m outline; chip "0.5 m already delivers 73 % of the 1.0 m saving". Plot: the full true curve at d = 0.5 m traced by a live marker while the big room's window grows 30 → 70 %; the curve bends below the straight dashed mid line. | `set_design` sweeps `T_big.wwr` 30 → 70 % with `live_marker` drawing the trail; curve stays; chip `FadeIn`. | `evaluate` at d = 0.5 m for WWR 30…70 %; ratio `saving(0.5 m) / saving(1.0 m)` at 50 % | The first half metre of overhang already delivers {share} percent of the saving. | Der erste halbe Meter Vordach liefert schon {share} Prozent der Einsparung. |

### Scene 10: repeat versus replicate (L4, about 22 s)

| Key | Visual state | Animation | Linked values | English | German |
| --- | --- | --- | --- | --- | --- |
| `repeat` | Right panel card "Run it again": two bars side by side from the same centre room, both exactly 760 kWh/a, with `=` between them; label "no scatter, no noise estimate". | The big room re-runs (rays shine again, bar regrows to the same value); a second identical bar `Create`; `=` `FadeIn`. | `centre.actual` twice | Running the same simulation again gives {c} again: no independent estimate of noise. | Dieselbe Simulation liefert wieder {c}: keine unabhängige Rauschschätzung. |
| `repeat_2` | Card "Measure it again": three semi-transparent bars of the same room at slightly different heights, a dashed band spanning them and a `?` instead of numbers; icons for cloud/occupants/open window; label "illustration, no simulated values". | Bars `GrowFromEdge` with a visible wobble; band `Create`; icons `FadeIn`. | none (purely qualitative, no numbers shown) | Measured rooms scatter, so physical experiments repeat runs to estimate variability. | Gemessene Räume streuen, daher wiederholen reale Versuche Läufe, um die Streuung zu schätzen. |

### Scene 11: purpose and the fractional design (L5, about 36 s)

| Key | Visual state | Animation | Linked values | English | German |
| --- | --- | --- | --- | --- | --- |
| `purpose` | Right panel checklist with three ticked lines: "question: window and overhang → cooling", "effects: four runs → ±kWh/a", "confirmation: middle run → 760 vs 850". The big room fades to the thumbnail of the centre room beside the checklist. | `clear_stage`; checklist `LaggedStart` with tick marks `Create`; values from the earlier beats. | `effects`, `centre` | Plan runs to answer the question, estimate the effects and confirm the result. | Plane Läufe, um die Frage zu beantworten, Effekte zu schätzen und das Ergebnis zu bestätigen. |
| `purpose_2` | Four cards again, now with the wall layer drawn (10 cm vs 30 cm insulation) and headers `A · B · C`; a small cube sketch (2³ = 8 corners, four filled) at the matrix corner: "full plan 8 runs, fraction 4 runs". Sign table appears: A, B, C, cooling. | `FadeIn` cards with LaggedStart; cube corners fill; table rows `Write`. | `fractional()["runs"]` (generator C = AB) and the full 2³ count | Add insulation as factor C: a full plan needs eight runs, a fraction only four. | Mit Dämmung als Faktor C braucht der volle Plan acht Läufe, ein Teilplan nur vier. |
| `purpose_3` | Table gains an `A·B` column; the `C` and `A·B` columns are identical (+ − − +); both framed `P_RED` with an `=` sign; in the cards the C+ rooms and the AB+ rooms coincide (runs 1 and 4 marked). | Add the `A·B` column; frames `Create`; `=` `GrowFromCenter`; `Indicate` runs 1 and 4. | alias structure from `fractional()` | In these four runs C moves exactly with the A B interaction: the two are confounded. | In diesen vier Läufen ändert sich C genau wie die Wechselwirkung A B: beide sind vermengt. |
| `purpose_4` | Result strip: `contrast = AB + C = −290 kWh/a`; below it a revealed box (greyed, "from all eight runs"): `AB = −250 · C = −40`. | Strip `Write`; the reveal box `FadeIn` after a short pause; final chip "Fractional designs save runs: document the confounding". | `fractional()["alias_contrast"]`, `["true"]` | The data show {m} for both together. Fractional designs save runs; document the confounding. | Die Daten zeigen {m} für beide zusammen. Teilpläne sparen Läufe; dokumentiere die Vermengung. |

Clause total: 3 + 3 + 2 + 3 + 2 + 4 + 4 + 3 + 4 + 2 + 4 = 34.

## 5. On-screen strings (en / de, both required)

| Role | English | German |
| --- | --- | --- |
| Question card | How do window size and overhang change summer cooling? | Wie verändern Fenstergröße und Vordach die sommerliche Kühlung? |
| Chip A | A · window size (WWR) | A · Fenstergröße (Fensteranteil) |
| Chip B | B · overhang depth | B · Vordachtiefe |
| Chip response | Cooling electricity, kWh/a | Kühlstrom, kWh/a |
| Fixed strip | Sun · room · weather fixed | Sonne · Raum · Wetter fest |
| Card header | A −1 · B +1   30 % · 1.0 m | A −1 · B +1   30 % · 1,0 m |
| Run | Run | Lauf |
| Overhang (matrix) | Overhang | Vordach |
| Axis x | Window-to-wall ratio (%) | Fensteranteil (%) |
| Axis y | Cooling electricity (kWh/a) | Kühlstrom (kWh/a) |
| Legend | overhang 0 m / overhang 1.0 m | Vordach 0 m / Vordach 1,0 m |
| Lower better | Lower is better | Kleiner ist besser |
| Disclaimer | Simplified annual teaching model, no measurements | Vereinfachtes Jahres-Lehrmodell, keine Messungen |
| Savings | saves 150 kWh/a | spart 150 kWh/a |
| Non-parallel | Non-parallel lines: interaction | Nicht parallele Linien: Wechselwirkung |
| Effect A | More glass costs 610 kWh/a (mean over both overhangs) | Mehr Glas kostet 610 kWh/a (Mittel über beide Vordächer) |
| Effect B | A 1 m overhang saves 400 kWh/a (mean over both window sizes) | Ein Meter Vordach spart 400 kWh/a (Mittel über beide Fenstergrößen) |
| Effect AB | The overhang pays off more the bigger the window | Das Vordach lohnt sich umso mehr, je größer das Fenster ist |
| Coefficients | mean of four runs · per +20 points of glazing · per +0.5 m overhang | Mittel der vier Läufe · je +20 Prozentpunkte Fenster · je +0,5 m Vordach |
| Centre | middle of the plan: 50 % · 0.5 m | Planmitte: 50 % · 0,5 m |
| Gap | model 850 · room 760 kWh/a | Modell 850 · Raum 760 kWh/a |
| Replicates | Illustration, no simulated values | Illustration, keine simulierten Werte |
| Alias | contrast = A·B + C | Kontrast = A·B + C |
| Insulation | C · insulation 10 / 30 cm | C · Dämmung 10 / 30 cm |

## 6. Toolkit widgets needed beyond `api.md`

Requested for `🎨️visuals/🐍️.py` (shared by other scenes unless marked scene-local):

1. **`room_section` extensions.** (a) uniform ray spacing along the window height so ray count ∝ admitted energy; (b) per ray state: `blocked` (stops at the overhang: dimmed plus a short `cross_mark`) or `passing` (continues to the floor); the blocked share equals the model's `shading_summer`; (c) `ghost_overhang=d_ref` draws a dashed outline of a reference overhang (`confirm_3`); (d) `ghost`/`compare` mode that returns the blocked-ray group so a scene can pulse it; (e) wall layer thickness from the insulation tracker (`purpose_2`); (f) a `thumbnail`-compatible `facade` glyph with a corner badge for C. Also accept `point` and `scale` as constants only (no callable needed because the block never scales).
2. **`value_bar(point, width, value_fn, vmax, unit, color, ghost=None, orientation="h")`.** Horizontal single-value bar with live number (`quantity`), optional ghost outline at a second value and a coloured difference segment with label; `api.md`'s `energy_bars` is a stacked vertical heating/cooling/lighting bar and does not do this. (`energy_bars(parts=("cooling",))` plus the ghost option would also do.)
3. **`unit_axes(origin, x_len, y_len, x_range, y_range, x_ticks, y_ticks, x_title, y_title)`.** Plain-text tick labels with language-aware numbers (no TeX), light grid, titles carrying the unit, `c2p`. The current `ClipScene.axes` has no tick labels.
4. **`level_slider(point, length, tracker, low, high, centre, coded=True, unit_fmt)`.** Track, ticks at the levels with coded `−1 / 0 / +1` labels, marker bound to a `ValueTracker`; animates with `set_design`.
5. **`data_table(columns, rows, col_widths, highlight)`.** Text grid without TeX; per-cell `fill` animation, column/row highlight frames, optional equal-column marking (used for `purpose_3`). Also usable by the pareto/index scenes.
6. **`mean_marker` and `parallel_ghost` helpers** (plot overlays): hollow diamond at a computed mean, dashed connector, dashed parallel line from a point with a given slope, bracket between two y values. Scene-local unless another scene wants them.
7. **`contrast_frames(cards, mode)`.** Highlight frames over the 2×2 card block for `A` (columns), `B` (rows) and `AB` (diagonals). Scene-local.
8. **`sign_table` / cube sketch** for the 2³ fraction (`purpose_2`). Scene-local.
9. **`fmt` helpers at module level.** `number(value, digits, language)` and `signed_words(value, language)` callable at import time, because `NARRATION` rows are class attributes that embed model numbers in both languages (`{r1}`…`{m}`). The instance method `ClipScene.number` is not usable there.
10. **Reused unchanged from the energy toolkit:** `sun_glyph`, `beam`/`radiation_ray`, `ripples`, `cross_mark`, `dim_arrow`, `chip`, `math_label`, `math_readout`, `side_labels`, `caption_bar`, `hold_for`, `section_material`/`section_assembly`, `fit_band`.

## 7. Model requests (to the model agent)

* `factorial()` must keep `runs` in the order r1…r4 above (`(−,−)`, `(+,−)`, `(−,+)`, `(+,+)`), and expose `effects` as full effects (`a`, `b`, `ab` with `ab = ½[(r4 − r2) − (r3 − r1)]`, coefficients = effects/2) and `centre.{design,predicted,actual}`.
* Add the summer sun altitude used by `room_section` (`study["summer_altitude"]`), plus per run `solar_summer` and `shading_summer` (convention: documented, rays derive from it).
* Add `fractional()` for the half fraction: `{"generator": "C=AB", "factors", "runs": [{"coded":[a,b,c], "design", "response"}], "alias_contrast": v, "true": {"ab": v, "c": v}}` where `true` comes from the full 2³ evaluated with `evaluate` (insulation 10 / 30 cm). Placeholders assume `AB −250`, `C −40`.
* Test expectations from `plan.md` must hold: centre differs from the bilinear prediction by at least 8 % (placeholder −11 %), and the 0.5 m overhang delivers more than half of the 1.0 m saving at 50 % (placeholder 73 %); the overhang saves more at 70 % than at 30 % (placeholder 650 vs 150).

## 8. Risks and layout notes for implementation

* Plot-point fly-in from the left column crosses the right column; use arcs and `FadeOut` trails, no persistent leaders, to avoid layout conflicts.
* The 150 kWh/a arrow at 30 % is short (about 0.3 units); put the label to the left of the axis side and keep the arrow at least 0.25 long.
* Cards are small (section 2.6 × 1.3); overhang 1.0 m is 0.43 units, so the blocked ray crosses must be drawn at 3 px minimum. If the checker flags clipping, cut the room section to the first 4 m depth and widen the scale to 0.65 unit/m.
* Keep every formula strip a single line ≤ 6.8 units wide (y 1.85…2.55); `purpose_3` and `purpose_4` use the same strip for the alias contrast.
* All narration numbers and all on-screen numbers read the same `factorial()` values, so the two cannot diverge.
