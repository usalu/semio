# Storyboard: Pareto Optimality (`🏆️pareto`, `Beat5_ParetoFront`)

Phase 1 (storyboard only, no source edits). Target runtime ≈ 3 min 20 s (35 clauses; 34 when `unsupported` is dropped). Numbers in `{…}` come from `pareto()` / `evaluate()`; sample values in the field table are placeholders from `storyboard-pareto-sketch.py` (rough coefficients, only used to check text lengths and the shape of the front).

## 1. Current scene (captured with `capture.py 🏆️pareto Beat5_ParetoFront en`)

33 clauses, abstract: designs `A–E`, cost in "k€" vs "t CO₂" (invented values 100/50, 120/35, 80/65, 140/60), `E = A` as duplicate, dashed guide through the nondominated set, then a cartoon "design space → objective space" with three pavilions. No axis value is shown by its meaning; the pavilion glyphs are not tied to any point. Pedagogical points kept: dominance definition (no worse everywhere, strictly better once), nondominated set, trade-offs, duplicates in objective space, sampled front is only an approximation, set (design space) vs front (objective space), weighted sums miss nonconvex fronts, the front does not choose (preferences do), 2D hides trade-offs. Frames: `🗑️generated/frames/Beat5_ParetoFront-en-*.png`.

## 2. Concept

One seminar room (8 × 6 × 3 m, glazed south façade, simplified annual teaching model). The plot is **objective space** (façade cost CHF across, annual electricity kWh/a up). The room is **design space** (WWR, insulation t, overhang d). Every scatter point is a façade; one live cursor binds both: the cursor's position is `(cost, electricity) = evaluate(trackers)` and the room, the stacked electricity bar, the price chip and the reading cards read the same trackers. Nothing on screen is normalized or lettered; designs are identified by their thumbnail (mini façade: window area, overhang, insulation line) and their numbers.

Story arc, with the physical reason behind every move:

1. **Room, two goals, 30 designs** (grid WWR 5 × t 3 × d 2). Hover shows the room.
2. **Dominance** with `dominated_example`: loser = large glazing without overhang (summer sun passes, chiller runs, bigger window costs more), winner = moderate glazing: cheaper and less electricity. Draw the dominating quadrant; loser greys.
3. **Rule** as three checked rows filled with the pair's real numbers; **twin** (same numbers, different cladding) shows why "strictly better once" matters.
4. **Front** by a cost sweep with a running-minimum staircase (the algorithm is visible), thumbnails on the front, guide line, "sampled only" via one off-grid design that beats a front member, set vs front.
5. **Walk along the front**: room morphs per step; "+CHF X buys −Y kWh/a", CHF per saved kWh/a (the steepness of the step), simple payback at the tariff versus the service life; bars grow, turn red after the break-even: the **knee**. Changing tariff moves the knee.
6. **Third criterion** (daylight factor %, dot size + colour): a design dominated in 2D is nondominated in 3D.
7. **Preferences** (link to the index clip): weight slider with iso-score line touching one front point; sweeping the weight slides the pick along the front and morphs the room; some front points are never picked; the front never chooses. Closing cards: cheapest, knee, weighted pick.

## 3. Screen layout (16:9, 14.22 × 8 scene units, y up)

Fixed zones: title chip top-left (y 3.4 … 3.95), caption band y ≤ −2.62 (two lines max). Working stage `x ∈ [−6.8, 6.8]`, `y ∈ [−2.55, 3.25]` (13.6 × 5.8, `fit_band(top=3.25)`).

```
y= 3.95 ┌────────────┐
        │ title chip │
y= 3.25 └────────────┘
 ┌───────────── PLOT (objective space) ─────────┐ ┌──────────── ROOM PANEL (design space) ─────────────┐
 │ Electricity [kWh/a]                          │ │  ☀ rays            ┌──────────────┐  ┌─┐ stacked │
 │ 2.85 ┤  ·  ·                  ·  ·  ·        │ │   ╲ ╲  overhang ═══╡ room section  │  │■│ heating │
 │      │     ·  ·   ○ grey cloud ·  ·          │ │    ╲ ╲   window ▌   │ lamps / patch │  │■│ cooling │
 │      │  ●╌╌●╌╌╌●                              │ │         ┌chip 22,704 CHF┐ └──────────────┘  │■│ lighting│
 │      │        ╲╌╌╌●╌╌●   front + thumbnails   │ │         └ 932 kWh/a ┘                      └─┘ total   │
 │      │ [no façade here]    ╲╌╌●               │ │ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │
 │−1.85 └──────────────────────────────────────  │ │ READING PANEL (y −2.5 … 0.15)                      │
 │      13,000     Façade cost [CHF]     29,000 │ │ ┌ +CHF ┐  ┌ payback bars per front step ┐          │
 └──────────────────────────────────────────────┘ │ ├ −kWh/a┤  │  ▇  ▇  ▃ ┆ - - life 30 a - - ┆          │
   x −6.8 … −0.2                                  │ └ CHF/kWh/a ┘ └───────────────────────────┘          │
 ┌──────────────────── caption band (≤ 2 lines) ────────────────────────────────────────────────────────┐
```

| Zone | Scene coordinates | Content |
| --- | --- | --- |
| Plot | axes box `x −5.7 … −0.5`, `y −1.85 … 2.85`; tick labels below to −2.2, x title y −2.45, y title rotated at x −6.5 | scatter, cursor, quadrant, staircase, front, thumbnails, knee ring, iso-score line |
| Room | section centre (2.7, 1.55), room 3.6 × 2.2 m drawn at 1 m = 0.9 units, sun glyph (0.55, 2.95), window wall on the left (towards the sun) | `room_section` season "summer" unless stated |
| Energy bar | `x 5.6 … 6.5`, `y 0.45 … 3.1`, same kWh/a-to-length scale as the plot's y axis | stacked heating (red) / cooling (blue) / lighting (yellow), live total, tick for previous design |
| Price chip | below the room, `(2.7, 0.62)` | cost with coin glyph (orange) |
| Reading panel | `x 0.2 … 6.8`, `y −2.5 … 0.15` | left: three stacked `stat_card`s (extra cost, saved electricity, CHF per kWh/a), right: payback bar chart with service-life line; replaced by rule rows / twin / closing cards in other beats |
| Intro | whole stage `y −2.5 … 3.2` | beat `room` only: room centred at scale 1.3, callouts left/right |

Colour roles (energy palette): cost `P_ORANGE`, heating `P_RED`, cooling `P_BLUE`, lighting `P_YELLOW`, front and "worth it" `P_GREEN`, dominated dots `PASTEL_GREY` at opacity 0.4, labels/chips `P_WHITE`/chrome. Daylight uses a `color_ramp` dark → `P_YELLOW`. Space labels "DESIGN SPACE" `P_CYAN`, "OBJECTIVE SPACE" `P_GREEN`.

Ranges are derived from the grid (rounded outward to a nice tick step); axes always carry units, never normalized values.

## 4. Fields (all from the model)

| Field | Meaning | Source | Sample (placeholder) |
| --- | --- | --- | --- |
| `n`, `n_wwr`, `n_t`, `n_d` | grid size and level counts | `pareto()["designs"]` (twin excluded from `n`) | 30, 5, 3, 2 |
| `c_L`, `e_L` | dominated design cost / electricity | `dominated_example.loser` | 22,704 CHF, 932 kWh/a |
| `dc`, `de` | cost and electricity advantage of the winner | winner vs loser | 4,644 CHF, 220 kWh/a |
| `c_W`, `e_W` | winner cost / electricity (rule rows only, not in narration) | `dominated_example.winner` | 18,060 CHF, 712 kWh/a |
| `nf` | distinct front points | `pareto()["front"]` | 6 |
| `wwr_r` | WWR of the off-grid refinement design | `pareto()["refinement"]` | 57 |
| `dc1`, `de1`, `r1`, `pay1` | first walk step: extra cost, saved electricity, CHF per kWh/a, payback in years | `pareto()["steps"][0]`, `payback()` | 2,142 CHF, 116 kWh/a, 18.5 CHF, 12 |
| `tariff`, `life` | electricity price CHF/kWh, façade service life a | study JSON | 0.25, 30 |
| `tariff_hi` | second tariff | study JSON | 0.40 |
| `nf3` | front points in 3 criteria | `pareto()["front3"]` | 9 |

Narration strings are built at import from templates with both languages formatted by the pure helper `format_number(value, digits, language)` (en `1,250.5`, de `1.250,5`), so TTS timing manifests and transcripts stay stable and numeric.

## 5. Narration (complete, en / de, ≤ 95 characters after substitution)

Keys follow `key`, `key_2` … (`explain`). `unsupported` is conditional: emitted only when the model reports at least one front point outside the weighted-sum hull.

| key | en | de |
| --- | --- | --- |
| `room` | Our seminar room has one glazed façade: we choose window size, insulation and overhang. | Unser Seminarraum hat eine verglaste Fassade: Wir wählen Fenster, Dämmung und Dachüberstand. |
| `room_2` | A simplified annual teaching model gives every façade its cost and its electricity. | Ein vereinfachtes Jahresmodell liefert für jede Fassade Kosten und Stromverbrauch. |
| `objectives` | We want both small: cost to the left, annual electricity at the bottom. | Beides soll klein sein: Kosten links, Jahresstrom unten. |
| `objectives_2` | Every façade becomes one point: its cost across, its electricity up. | Jede Fassade wird ein Punkt: Kosten nach rechts, Stromverbrauch nach oben. |
| `grid` | We evaluate {n} designs: {n_wwr} window sizes, {n_t} insulations, {n_d} overhangs. | Wir bewerten {n} Entwürfe: {n_wwr} Fensteranteile, {n_t} Dämmstärken, {n_d} Dachüberstände. |
| `grid_2` | Point at any dot: the room shows that façade with its price and its electricity. | Zeige auf einen Punkt: Der Raum zeigt diese Fassade mit Preis und Stromverbrauch. |
| `extremes` | The cheapest façade and the most frugal one lie far apart: none is both. | Die günstigste und die sparsamste Fassade liegen weit auseinander: Keine ist beides. |
| `dominated` | This façade costs {c_L} and uses {e_L} per year. | Diese Fassade kostet {c_L} und verbraucht {e_L} im Jahr. |
| `dominated_2` | That one is {dc} cheaper and uses {de} less. | Jene ist {dc} günstiger und braucht {de} weniger Strom. |
| `dominated_3` | So the first is dominated: with these two goals nobody would pick it. | Die erste ist also dominiert: Mit diesen zwei Zielen wählt sie niemand. |
| `rule` | One façade dominates another if it is no worse in cost and no worse in electricity. | Eine Fassade dominiert eine andere, wenn sie bei Kosten und Strom nicht schlechter ist. |
| `rule_2` | And it must be strictly better in at least one of the two. | Und in mindestens einem der beiden muss sie echt besser sein. |
| `twin` | Two façades can share one point: same cost, same electricity, different cladding. | Zwei Fassaden können einen Punkt teilen: gleiche Kosten, gleicher Strom, andere Verkleidung. |
| `twin_2` | Neither is strictly better, so neither dominates the other: both stay. | Keine ist echt besser, also dominiert keine die andere: Beide bleiben. |
| `front` | Sweep from cheap to expensive and keep a façade only if it beats all cheaper ones. | Von günstig zu teuer: Nur wer weniger Strom braucht als alle günstigeren, bleibt. |
| `front_2` | What remains is the Pareto front: {nf} façades, and every gain now costs something. | Übrig bleibt die Pareto-Front: {nf} Fassaden, und jeder Gewinn kostet nun etwas. |
| `approximate` | Between the points, an unevaluated design may still beat a front member. | Zwischen den Punkten kann ein nicht ausgewerteter Entwurf ein Frontmitglied schlagen. |
| `sets` | The room is design space, the plot is objective space; evaluation maps one onto the other. | Der Raum ist der Entwurfsraum, das Diagramm der Zielraum; die Auswertung bildet ab. |
| `walk` | Walk along the front, starting at the cheapest façade. | Wir gehen die Front entlang, beginnend bei der günstigsten Fassade. |
| `walk_2` | Step one: {dc1} more buys {de1} less electricity every year. | Schritt eins: {dc1} Mehrkosten sparen {de1} Strom pro Jahr. |
| `walk_3` | Divide the two: {r1} for every kWh/a saved. | Teilt man beides: {r1} für jede eingesparte kWh/a. |
| `payback` | At {tariff} CHF per kWh the savings repay this step in {pay1} years. | Bei {tariff} CHF je kWh amortisiert sich dieser Schritt in {pay1} Jahren. |
| `payback_2` | A façade lasts about {life} years: only steps that repay sooner are worth buying. | Eine Fassade hält etwa {life} Jahre: Nur schneller amortisierte Schritte lohnen sich. |
| `knee` | Walk on: each step saves less per franc, until payback passes the façade's lifetime. | Weiter: Jeder Schritt bringt weniger je Franken, bis die Amortisation die Lebensdauer sprengt. |
| `knee_2` | Here is the knee: up to this façade money buys a lot, beyond it almost nothing. | Hier liegt das Knie: Bis zu dieser Fassade bringt Geld viel, danach kaum noch. |
| `knee_3` | A higher tariff moves the knee: the front stays, the best choice changes. | Ein höherer Tarif verschiebt das Knie: Die Front bleibt, die beste Wahl ändert sich. |
| `third` | Add daylight as a third goal: dot size and colour show the daylight factor in %. | Tageslicht als drittes Ziel: Punktgröße und Farbe zeigen den Tageslichtquotienten in %. |
| `third_2` | This design lost in two dimensions: a dimmer room was cheaper and used less. | Dieser Entwurf verlor in zwei Dimensionen: Ein dunklerer Raum war billiger und sparsamer. |
| `third_3` | Counting daylight, nothing beats it on all three goals, so it joins the front. | Mit Tageslicht schlägt ihn nichts in allen drei Zielen: Er gehört zur Front. |
| `third_4` | A two-dimensional picture can hide a trade-off: check every criterion. | Ein zweidimensionales Bild kann Zielkonflikte verbergen: Prüfe jedes Kriterium. |
| `weights` | To pick one façade, weigh the goals, as in the index video. | Um eine Fassade zu wählen, gewichten wir die Ziele wie im Indexvideo. |
| `weights_2` | Whatever the weights, the winner sits on the front; changing them slides along it. | Welche Gewichte auch gelten: Der Sieger liegt auf der Front und gleitet mit ihnen entlang. |
| `unsupported` | Weights cannot reach every front point: a façade in a dent never wins. | Gewichte erreichen nicht jeden Frontpunkt: Eine Fassade in einer Mulde gewinnt nie. |
| `decision` | The front removes bad designs; it does not choose. | Die Front entfernt schlechte Entwürfe; sie wählt nicht. |
| `decision_2` | Tariff, lifetime, weights and design judgement choose: the front shows what is possible. | Tarif, Lebensdauer, Gewichte und Entwurfsurteil entscheiden: Die Front zeigt das Mögliche. |

## 6. Beat table

State vocabulary: **cursor** = live 2D marker on `evaluate(focus trackers)` with drop lines and a readout; **room** = `room_section` + `energy_bars` + price chip bound to the same trackers; "morph" = `set_design(trackers, design, run_time)`; dot states: `plain` (P_WHITE small), `grey` (PASTEL_GREY, 0.4), `front` (P_GREEN). `seconds` is the animation time inside the clause; the rest is the VO hold.

### Act A – Room and two goals (≈ 35 s)

| key | visual state | animation | linked values |
| --- | --- | --- | --- |
| `room` | Stage empty. Room section centred (scale 1.3), baseline design, summer sun + rays through the window, callouts with leader lines: "Window-to-wall ratio {wwr} %" at the window, "Insulation {t} cm" at the wall layer, "Overhang {d} m" at the slab. | `Create` room shell, `FadeIn` sun, `shine` rays, `LaggedStart` the three callouts (`Indicate` each in turn) | baseline `wwr`, `insulation`, `overhang` from `study["baseline"]` |
| `room_2` | Price chip (orange coin, CHF) under the room, stacked energy bar right of the room with heating/cooling/lighting labels and total. | `FadeIn` chip and bar, bar `GrowFromEdge` segment by segment, `Indicate` total | `evaluate(baseline)`: `cost`, `heating`, `cooling`, `lighting`, `electricity` |
| `objectives` | Room scales/shifts into the room panel (x 2.7), callouts fade. Plot axes appear: "Façade cost [CHF]" and "Electricity [kWh/a]" with ticks from the grid range; green "better" arrow in the lower-left diagonal. | `Transform` room group to its panel place, `Create` axes, `FadeIn` labels and arrow | axis ranges from `pareto()` grid min/max, units fixed |
| `objectives_2` | Baseline marker = cursor dot on the plot, dashed drop lines to both axes with values; chips of the room (CHF, kWh/a) pulse together with the drop-line readouts. | `FadeIn` cursor, `Create` drop lines, `Indicate` chip + readout simultaneously | `cost`, `electricity` of baseline = chip values = axis readouts |
| `grid` | Scatter fills with {n} plain dots. Cursor hops through ≈ 8 representative grid designs in grid order while the room morphs each hop; last hop rests on the full-glass design. | `LaggedStart(FadeIn(dot))` (lag 0.08), parallel hop sequence `set_design` 0.45 s each, `seconds=4.5` | all `pareto()["designs"]`; trackers take each hop's design |
| `grid_2` | Cursor arrow ("hover" glyph) visits three dots: a moderate façade, the large-glass one without overhang (summer rays pass, cooling bar grows), a deep-overhang one (rays blocked on the slab). Readout chip beside the cursor: "CHF · kWh/a". | cursor path along evaluate, `set_design` 1.3 s each, `Indicate` growing bar segment | `evaluate(design)` for the three picks (selected by the scene: max cooling, max shading) |
| `extremes` | Cursor jumps to the cheapest design (top-left) then to the lowest-electricity design (bottom-right); the room morphs between a small window without overhang and a larger glazing with overhang. A dashed rectangle in the empty lower-left corner with the tag "No façade here" / "Keine Fassade". | cursor path, `set_design` 1.4 s each, `Create` dashed rectangle + `FadeIn` tag | argmin cost, argmin electricity from the grid; corner = `[min cost, min electricity]` rectangle |

### Act B – Dominance (≈ 35 s)

| key | visual state | animation | linked values |
| --- | --- | --- | --- |
| `dominated` | Cursor on the loser; room = loser (large glazing, no overhang, season chosen by the larger of `|Δheating|`, `|Δcooling|`, normally summer: rays pass, cooling bar large). Dot gets an orange ring, readout "22,704 CHF · 932 kWh/a". | cursor jump, `set_design` 1 s, `Create` ring | `dominated_example.loser` |
| `dominated_2` | Green translucent quadrant from the loser dot to the axes corner (everything cheaper **and** frugal), dashed border. Cursor travels to the winner inside it; room morphs (window smaller, shading/overhang as the model says), faint ghost outline of the loser's window and overhang stays. Plot: ΔCHF bracket (horizontal, orange) and ΔkWh/a bracket (vertical, white) between the two dots with values; reading panel shows "Cheaper ✓ −4,644 CHF" and "Less electricity ✓ −220 kWh/a"; energy bar keeps a tick at the loser's total. | `FadeIn` quadrant, cursor move + `set_design` 1.8 s, `GrowFromCenter` brackets, `FadeIn` check rows | `dominated_example.winner`, `dc`, `de`, `cost`, `electricity` |
| `dominated_3` | Loser dot → grey with a red `cross_mark`, its thumbnail dims; winner stays orange-ringed then turns green later. Quadrant fades to 10 %. | `dot.animate.set_color(grey)`, `FadeIn(cross_mark)`, quadrant `FadeOut` to low opacity | same pair |
| `rule` | Reading panel: three rows, each with the winner thumbnail, a "≤"/"<" sign and the loser thumbnail. Row 1: cost `{c_W} ≤ {c_L}` ✓. Row 2: electricity `{e_W} ≤ {e_L}` ✓. Thumbnails morph out of the two small façade glyphs shown on the dots (`ReplacementTransform`, symbol-token pattern). | `ReplacementTransform` thumbnails → row tokens, `Write` rows 1–2 with ✓ | cost and electricity of the pair |
| `rule_2` | Row 3 appears: "strictly better in at least one" with `<` circled in both rows (here both are strict). | `FadeIn` row 3, `Circle` around `<` | same pair; strictness computed from the values |

### Act C – Twin (≈ 12 s)

| key | visual state | animation | linked values |
| --- | --- | --- | --- |
| `twin` | Room panel replaced by two façade thumbnails at scale 1.8 with different cladding finish (brick red vs timber warm grey) and identical windows/overhang; both have a thin line to **one** doubled-ring dot on the plot; single readout "CHF · kWh/a" bracket joins them. | `FadeOut` room + bars, `FadeIn` thumbnails, `Create` ring + two leader lines | `twin_example` (a, b), objective vectors equal; `finish` ignored by `evaluate` |
| `twin_2` | Reading rows reappear for this pair: cost `=` ✓, electricity `=` ✓, strictly better ✕ (red). Both thumbnails get a green ring, the dot turns green on its own at the next beat. | `FadeIn` rows, `cross_mark` on row 3, `Create` rings | same |

### Act D – Front (≈ 55 s)

| key | visual state | animation | linked values |
| --- | --- | --- | --- |
| `front` | Room returns (kept design under the sweep). Vertical sweep line moves from cheap to expensive; a green staircase (running minimum of electricity) is drawn under it. A dot that does not lower the staircase turns grey as the sweep passes; one that does turns green and the cursor hops onto it (room morphs 0.35 s). | `UpdateFromAlphaFunc` sweep line over 6 s, staircase `Create` in segments, dot colour animations at crossing times | sorted `designs` by cost; `front` ids; grey = all others |
| `front_2` | Staircase fades, the green dashed guide polyline connects the front points; thumbnails of all front façades pop up on the empty lower-left side of the line with short leaders (alternating offsets, no overlaps); counter chip "{nf} façades". | `FadeOut` staircase, `Create` `DashedVMobject` guide, `LaggedStart` thumbnails (0.12 lag) | `front` ids, `designs[i].design` for thumbnails |
| `approximate` | Cursor moves onto the refinement design: a dashed ghost dot appears **below** the guide, room morphs to it. One front member greys, the guide redraws through the ghost (tag "not in the grid" / "nicht im Raster"). | ghost dot `FadeIn`, cursor + `set_design` 1.6 s, member `set_color(grey)`, guide `become` new polyline, then `FadeOut` ghost and restore the member at the start of `sets` | `pareto()["refinement"]` (design, cost, electricity, dominated front id) |
| `sets` | Headers "DESIGN SPACE: window · insulation · overhang" above the room panel (cyan) and "OBJECTIVE SPACE: cost · electricity" above the plot (green). Arrow room → plot labelled "evaluate" / "Auswertung". Brackets: "Pareto set" around the front thumbnails, "Pareto front" along the guide line. | `FadeIn` headers, `GrowArrow`, `Create` brackets | `front` ids |

### Act E – Walk along the front (≈ 70 s)

Reading panel and payback chart are introduced here. Front points are indexed by the scene only for animation (`F1…Fn`, never lettered on screen). Duplicate objective vectors (twin) are skipped.

| key | visual state | animation | linked values |
| --- | --- | --- | --- |
| `walk` | Headers/brackets fade; guide line stays. Room = cheapest front façade; thumbnails fade to 35 % except the current one. Reading panel: three empty cards (Extra cost / Saved electricity / CHF per kWh/a) and an empty payback chart with axis "Payback [a]" and "step" ticks. | `FadeOut` headers, `FadeIn` panel, cursor to F1 `set_design` 1 s | `steps`, tariff, life |
| `walk_2` | Cursor travels F1 → F2 (true evaluate path); segment highlighted green, ΔCHF and ΔkWh/a brackets on the plot; room morphs (e.g. window grows, overhang appears; energy bar segments change colour mix, tick marks the previous total). Cards count up: "+2,142 CHF" (orange), "−116 kWh/a" (white). | `set_design` 2 s, `Create` brackets, `ChangeDecimalToValue` card numbers | `steps[0]` (`cost`, `electricity`), step designs |
| `walk_3` | Third card "18.5 CHF per kWh/a" with a slope triangle on the segment: the price is the inverse steepness of the step; tag "marginal cost" / "Grenzkosten". | `FadeIn` card, `Create` triangle, `Indicate` segment | `steps[0].ratio` |
| `payback` | Payback chart gets bar 1 (height = payback years, green) with value label; cost card shows yearly saving "−116 kWh/a × 0.25 CHF/kWh = 29 CHF/a". | `GrowFromEdge` bar, `Write` saving line | `payback(step, tariff)`, `tariff` |
| `payback_2` | Dashed horizontal line at the service life "Service life {life} a" across the chart; bar 1 sits clearly below it (✓ green). | `Create` dashed line, `Indicate` bar 1 | `service_life` |
| `knee` | Cursor walks the remaining steps (≈ 1.1 s per step); at each: room morph, step cards update, next bar grows; bars pass the life line and turn red (`P_RED`), the cost card turns red once payback > life. The room shows how little each extra step changes (huge cost, tiny saving). | repeated `set_design` + bar `GrowFromEdge`, bars recolour at crossing, `seconds ≈ 7` | all `steps`, `payback` per step |
| `knee_2` | Cursor returns to the knee façade (last step with payback ≤ life); soft green ellipse around the knee region on the plot (before: steep, after: flat); room = knee; knee card "Knee: {cost}, {electricity}, payback {pay} a". | cursor + `set_design` 1.4 s, `Create` ellipse, `FadeIn` card | `knee(tariff, life)` |
| `knee_3` | Tariff chip "0.25 CHF/kWh" slides to "0.40 CHF/kWh"; all payback bars shorten, the red/green split and the knee ring hop one point to the right; room morphs to the new knee; reverse back to 0.25 CHF/kWh in the last 1.2 s. | tracker animation of tariff, bars `always_redraw`, ring move, `set_design` 1.4 s | `knee(tariff_hi, life)` ≠ `knee(tariff, life)`, `payback` recomputed |

### Act F – Third criterion (≈ 40 s)

| key | visual state | animation | linked values |
| --- | --- | --- | --- |
| `third` | Walk panel fades. All dots (including grey) resize/recolour by daylight factor (dot radius ∝ %, ramp dark → yellow); legend in the empty lower-left corner "Daylight factor [%]" with gradient 3 … 6; the room section now shows daylight depth on the floor and dimmed lamps; the cursor visits a bright design (lamps dim) and a dark one (lamps lit). | `Transform` dot radii, `color_ramp` fills, `FadeIn` legend, cursor + `set_design` 1.2 s ×2 | `evaluate(design)["daylight"]` per design |
| `third_2` | Cursor on the 2D-dominated design X (grey, big dot); arrow from its 2D dominator Y (cheaper, smaller dot). Room morphs Y → X: the window is larger, the daylight patch reaches deeper, lamps dim. Mini rows: cost ✕ (X costs more), electricity ✕, daylight ✓ (+ pp %). | cursor, `set_design` 1.8 s, `GrowArrow`, row `FadeIn` | `third_example` (X, Y), daylight values |
| `third_3` | X turns green with a ring; every design that is nondominated in 3D gets a green ring, newly joined ones pulse; counter "Front: {nf} → {nf3}". | `set_color`, `Create` rings, `LaggedStart` pulses, counter `ReplacementTransform` | `front3` ids |
| `third_4` | Small "2D" and "3D" front icons side by side on the right: same dots, different rings; emphasis on X. | `FadeIn` icons, `Indicate` X | `front`, `front3` |

### Act G – Preferences (≈ 45 s)

| key | visual state | animation | linked values |
| --- | --- | --- | --- |
| `weights` | Dots return to uniform size/colour, grey vs front; third-criterion legend fades. Weight slider (Cost ⟷ Electricity, shares in %, default 50/50) in the reading panel; iso-score line (straight, normalized by the same min–max as the index video) touches the front at one point; cursor ring there; room = pick; chips show its numbers. | `FadeIn` slider + line, cursor jump + `set_design` 1.3 s | `weighted_pick(w)`, normalization over front |
| `weights_2` | Slider sweeps electricity weight 20 % → 80 %: the iso-line rotates, the pick slides along the front through its members, the room morphs each time (windows grow, overhang appears), grey dots are never touched. | `ValueTracker` on `w`, `always_redraw` iso-line, discrete pick changes with `set_design` 0.9 s, `seconds ≈ 6` | `weighted_pick(w)` for sampled w |
| `unsupported` (conditional) | One front point inside a dent gets a hollow dashed ring "never picked"; the iso-line passes over it during a full sweep. | `Create` ring, sweep animation, `FadeIn` tag | `supported` ids, the unsupported id |
| `decision` | Slider/line fade; plot keeps front + guide. Reading panel becomes three cards, each with thumbnail and numbers, connected to their plot points: "Cheapest", "Knee at {tariff} CHF/kWh", "Weighted pick {w} %". | `FadeOut` slider, `LaggedStart` cards, `Create` leader lines | cheapest, `knee`, `weighted_pick` |
| `decision_2` | Cards pulse in sequence while the room shows each design in turn; end on the knee. Hold. | `Indicate` per card + `set_design` 1.1 s | the three designs |

Estimated runtime: 35 clauses, 2,588 English characters (≈ 15 characters/s ≈ 172 s of speech) plus 0.3 s gaps ≈ 3 min 05 s. The longest animation windows (`grid` 4.5 s, `front` 6 s, `knee` 7 s, `weights_2` 6 s) exceed their clause's speech time by up to ≈ 2 s each, so the total lands at ≈ 3 min 20 s – 3 min 30 s (inside the 2.5 – 4 min target). Act budget: A 35 s, B 33 s, C 12 s, D 45 s, E 60 s, F 32 s, G 38 s.

## 7. Scene state objects (for the implementer)

| Object | Purpose |
| --- | --- |
| `focus = study_trackers(baseline)` | drives room, bar, chip, cursor, readouts |
| `dots: dict[id, Dot]`, `state: dict[id, "plain"|"grey"|"front"|"ring"]` | scatter; `set_state` returns animations |
| `thumbs: dict[id, thumbnail]` | created lazily for front, pair, X, cards |
| `steps = pareto()["steps"]` | walk, cards, chart |
| `tariff = ValueTracker(study tariff)` | payback bars and knee ring |
| `weight = ValueTracker(0.5)` | slider, iso-line, pick |

Rules: no hand-typed number reaches a `Text`; every value passes `quantity()`/`number()`; the dominated pair is checked in the scene by an assert on the model dominance relation (not by the storyboard). Dominated dots are never removed, only dimmed.

## 8. Toolkit widgets needed beyond `api.md`

Required:

1. `live_point(axes, trackers, x_fn, y_fn, readout=True)` – 2D cursor (dot, drop lines, readout chip) following two functions of `evaluate(trackers)`; `live_marker` in `api.md` handles only one x tracker.
2. `scatter(axes, designs, key_fns)` returning `dots: dict[id, Dot]` and `set_state(id, state)` → animation (plain / grey / front / ring), size/colour mapping (`size_fn`, `color_ramp`).
3. `dominance_quadrant(axes, point, corner)` – shaded dominating rectangle towards the ideal corner, dashed border, fade levels.
4. `delta_brackets(axes, p_from, p_to, x_label, y_label)` – horizontal and vertical brackets with values (orange / white).
5. `sweep_staircase(axes, designs)` – sweep line plus running-minimum staircase; returns the animation and the per-design crossing time.
6. `front_guide(axes, points)` – dashed polyline with label "guide".
7. `step_cards(...)` – three stacked `stat_card`s with counting numbers (extra cost orange, saved electricity white, CHF per kWh/a) and a saving line "kWh/a × CHF/kWh = CHF/a".
8. `payback_chart(steps, tariff_tracker, life)` – bars (green below life, red above), dashed service-life line, axis "Payback [a]", `always_redraw` on the tariff tracker.
9. `knee_marker(axes, point)` – soft ellipse plus label.
10. `thumbnail(design, point, scale, finish=None)` – extend with optional cladding colour; `thumb_token` so thumbnails can `ReplacementTransform` into formula rows (symbol-token pattern).
11. `check_rows(rows)` – ✓/✕ comparison rows with thumbnails left and right (`rule`, `twin_2`, `third_2`).
12. `weight_slider(tracker, left_label, right_label)` and `iso_score_line(axes, weight, normalization, anchor)` – tangent line to the front with the pick; same min–max normalization as the index video.
13. `daylight_legend(ramp, low, high)` – dot-size plus colour legend with units.
14. `format_number(value, digits, language)` pure module function so `NARRATION` can be built at import with numbers in both languages.
15. `room_section(..., ghost=design)` optional faint outline of a reference design (window/overhang) for before/after comparison, and `energy_bars(..., previous_tick=True)` optional tick at the previous total.

Nice to have: `space_headers(plot, room)` (the two coloured headers plus arrow), `hull_pick(front, weight)` as scene-side helper if the model does not expose one.

## 9. Model requests (to the model agent)

1. `pareto()["designs"]` rows carry `cost`, `electricity`, `daylight`, `heating`, `cooling`, `lighting` (or are resolvable through `evaluate`); `evaluate` ignores unknown design keys (cosmetic `finish`).
2. `dominated_example`: loser = high WWR (≥ 65 %) without overhang, winner within the grid, both well separated on screen (≥ 15 % of each axis range), cooling difference larger than the heating difference.
3. `front` sorted by cost with 6–9 distinct objective points; the front must involve all three variables (WWR, insulation, overhang) so the walk morphs visibly; the front must be convex-ish with at least one point outside the weighted-sum hull (`supported` list) if physically natural, otherwise omit `unsupported`.
4. `twin_example` = `{a, b}` with `b` a copy of a front design plus `finish`; identical objectives; both in `front`.
5. `steps`: front steps with `cost` (CHF added), `electricity` (kWh/a saved), `ratio` (CHF per kWh/a, strictly increasing along the front); `payback(step, tariff)` in years; `knee(tariff, life)` id; study JSON with `tariff` (0.25 CHF/kWh), `tariff_hi` (0.40) and `service_life` (30 a) chosen so that payback increases monotonically, the first step is ≤ 0.5 × life, the last ≥ 2 × life, it crosses `life` exactly once at an interior step, and `knee(tariff_hi)` ≠ `knee(tariff)`.
6. `refinement` = `{design, cost, electricity, dominates: [front ids]}` off-grid (e.g. WWR between two levels) dominating at least one front member.
7. `third_example` = `{dominated_2d, dominator}` and `front3`: a design dominated in (cost, electricity) but nondominated in (cost, electricity, −daylight); the dominator has less daylight.
8. `weighted_pick(weight_electricity)` over the front using the min–max normalization of the index clip, returning a front id; monotone along the front as the weight grows.
9. Tests: every claim above is asserted (dominance relation, front uniqueness, payback monotonicity, knee shift, third-criterion revival, twin equality).

## 10. Risks

- Narration with numbers needs the import-time template path (widget 14); otherwise strip numbers from the clauses and keep them only on screen.
- If the model produces a front where step paybacks never cross the service life, `knee` degenerates: request 5 is mandatory.
- Thumbnails on 30 dots would overlap (neighbours in cost are ≈ 0.25 units apart): thumbnails only on front points, the pair, X, cards; every other dot is reachable through the cursor.
- Marker path between designs follows `evaluate` of interpolated variables and can leave the guide line: this is intentional, truthful and supports `approximate`.
