# Storyboard: Multiple Criteria and Index (`⚖️index`, `Beat4_NormalizedIndex`)

Phase 1 (design only). Contract: `plan.md` (design study, visual principle), `api.md` (model and toolkit), `agent-rules.md`.
Old pedagogical points kept: incommensurable units, direction, normalization ranges, weights, weighted-sum index, sensitivity of the ranking, range dependence on the compared set, documentation of weights, compensation limit and the Pareto bridge.

## 1. Concept in one paragraph

Five real façade variants of the seminar room are the actors from first to last frame. Each is a card with its façade thumbnail and four measured quantities with units. The viewer first sees why kWh/(m²·a) and CHF cannot be added (and that a naive sum depends on the arbitrary unit). Normalization is then made physical: one criterion at a time a ruler in real units carries the five thumbnails at their true values; two tracker-driven moves (subtract the best, divide by the span) turn it into the 0…1 ruler, a live room panel always showing what the pointed value means. Daylight flips the ruler. The four rulers become one table whose columns are stretched to the weights; every cell is a filled bar, and sliding the filled parts together gives the index bar (coloured weighted segments). The shortest bar wins, its room is shown. A cost-driven client re-weights, the bars and ranks move and another room wins; a bump chart shows where the winner switches. A sixth façade stretches the cost ruler, squeezes all other values and flips the order of the original five. The scene closes with the decision record, the compensation limit and the Pareto teaser.

Visual rule from the owner (applies to every screen): no value without its meaning on the same screen. Variant thumbnails stand for every value on rulers, table and charts; the room panel (live room section, kWh bars, price chip, daylight depth, carbon cloud) follows the focused variant.

## 2. Stage, grid, palette

Free stage (between title chip and caption band): `x ∈ [-6.8, 6.8]`, `y ∈ [-2.6, 2.58]` (13.6 × 5.18 units, `SAFE_*` of the energy toolkit; 1 unit = 90 px at 1280 × 720). Caption band below `y = -2.62`; title chip above the stage (top-left), nothing of the scene may enter either. Fonts: `BODY_FONT_SIZE` for values, `LABEL_FONT_SIZE` for units and labels, never smaller than 0.8 × `LABEL_FONT_SIZE`.

Colours (energy palette, one per criterion, always accompanied by a glyph and the direction arrow so colour is never the only carrier):

| Criterion id | Colour | Glyph | Direction | Unit |
| --- | --- | --- | --- | --- |
| `electricity` | `P_CYAN` | `object_silhouette("bulb")` | ↓ less is better | kWh/(m²·a) |
| `cost` | `P_ORANGE` | coin glyph (new) | ↓ | CHF |
| `daylight` | `P_YELLOW` | `object_silhouette("sun")` | ↑ more is better | % (mean daylight factor) |
| `carbon` | `P_PRIMARY` | `object_silhouette("factory")` | ↓ | kg CO₂e |

Accent `P_GREEN` = best / winner, `P_ORANGE` outline = warning (compensated weakness), `P_WHITE` = ink and axes. Language: every visible string and number through `tr` / `number` / `quantity`.

Variant names: from `variants()[i].title[lang]`, expected short form (model agent: ≤ 18 characters, one line at `LABEL_FONT_SIZE`, e.g. "Punched 30 %", "Ribbon + overhang", "Full glass + overhang", "Punched, 30 cm insulation", "Full glass, no overhang"; the last two need a two-line break allowed, ≤ 2 lines of ≤ 16 characters).

## 3. Screens (layout sketches)

### Screen A: variant cards (clauses `variants` … `best`)

```
y= 2.55 ┌──────┐┌──────┐┌──────┐┌──────┐┌──────┐   five cards 2.5 w × 4.9 h, gap 0.2
        │ name ││ name ││ name ││ name ││ name │   centres x = -5.4 -2.7 0 2.7 5.4, y = 0.1
y= 1.45 │▭thumb││      ││      ││      ││      │   facade thumbnail 2.1 × 0.79
y= 0.85 │spec  ││      │   spec line (window % · overhang m · insulation cm), font 13
y= 0.25 │💡 74.3│   row 1 electricity: value (BODY) / unit (LABEL) + glyph + ↓
y=-0.47 │🪙 18 400│  row 2 cost CHF ↓
y=-1.19 │☀ 3.1 │   row 3 daylight % ↑
y=-1.91 │🏭 2 310│  row 4 carbon kg CO₂e ↓
y=-2.35 └──────┘   best cell of each row gets a green frame in `best`
```

### Screen B: units (clauses `units` … `units_3`)

```
B1 (units)                                   B2/B3 (units_2, units_3)
y= 0.9   [💡 74.3 kWh/(m²·a)] + [🪙 18 400 CHF] = ?   x -6.2 … 6.2      y= 1.7 … -1.7  five naive bars (pitch 0.7)
y= 0.0   red ✖ over "="  and chip "no unit"                              x -5.8 … -3.6 thumb + name; bars x -3.4 … 4.0
y=-1.2   the same variant's thumbnail left, small                         right x 4.4 … 6.7: unit switch chip, winner tag
```

### Screen C: the ruler (clauses `ruler` … `matrix`)

```
y= 2.35  [glyph Criterion name · unit · ↓/↑ direction]  x -6.6 …                    │ room panel x 2.5 … 6.7
y= 1.35  thumbnails lane 1 (1.15 × 0.43)  ┐ two staggered lanes above the line      │ y 2.35 variant name
y= 0.60  thumbnails lane 2                ┘ leader lines down to the true value      │ y 1.9 … 0.2 room_section (scale 0.6): wall, window ∝ WWR,
y=-0.30  ruler line  x -6.2 … 1.8 (L = 8.0), ticks, tick labels below (y -0.62)      │      overhang, sun rays, patch, dimmed lamps
y=-1.15  value tags (staggered), "best" green flag, "worst" red flag                   │ y -0.2 … -2.4 energy_bars (heat/cool/light) with live
y=-2.0   note chip "compared set: 5 variants" (brace under all markers) in `mapped`   │      kWh/a and kWh/(m²·a) total (criterion-specific view below)
```

Panel content per criterion (same frame, same room, different overlay): electricity = `energy_bars`; cost = façade drawing with price chip and wall/glass/insulation/overhang cost split; daylight = room section with daylight depth gradient on the floor and the mean factor %; carbon = factory glyph with a cloud scaled by kg CO₂e.

### Screen D: penalty table and index board (clauses `matrix` … `sensitivity_3`, and `range_3`)

```
y= 2.45  header chips over columns: [💡 Energy 25 %] [🪙 Cost] [☀ Daylight] [🏭 Carbon]   (percent live from weight trackers)
y= 1.95  split bar 100 % (column widths = weights)                                       │ winner panel x 3.7 … 6.7
y= 1.45  row 1  ① ▭thumb  name │ cell cell cell cell │ score    pitch 0.66              │ room_section (scale 0.42)
y= 0.79  row 2                                                                          │ four raw values (glyph, value, unit)
y= 0.13  row 3        name col x -6.7…-4.2 (rank badge, thumb 0.8 × 0.3, name font 16)   │ "winner" frame green
y=-0.53  row 4        cells x -4.2 … 2.0 (6.2 wide), cell width = weight × 6.2           │
y=-1.19  row 5        score x 2.1 … 3.3                                                  │
y=-1.85  row 6 (only range_3)    axis hint under rows: index 0 = ideal … 1 = worst         │
```

Cell = outline (criterion colour 40 %) + filled part of fraction `normalized` (criterion colour 70 %) + number (font 16). In compact state the empty remainders fade and the filled parts slide together into one multicoloured bar per row, bar length = score × 6.2.

### Screen E: bump chart (clauses `sweep`, `sweep_2`)

```
left x -6.5 … 1.0 : axes, x = cost weight 0 … 100 % (other weights keep their proportions), y = rank 1 (top) … 5, five coloured lines (thumbnail at line end)
                    vertical marker at the current cost weight, dot on each line; shaded band where the top two scores are close
under the axes   : winner strip y = -2.15, one thumbnail per region, switch value labelled in %
right x 1.9 … 6.7 : room panel of the current winner (room_section scale 0.5) + four raw values + the four weights as chips
```

### Screen F: record and bridge (clauses `record`, `limits`, `next`)

```
record: one decision-record card (x -4.5 … 4.5, y 2.3 … -2.3), 2 × 2 compartments: Weights chips | Compared set (5 thumbnails) | Ranges best…worst per criterion with units | Result (winner thumbnail, index)
limits: card shrinks to the left (x -6.5 … -1.5); right: two chips "Constraints: filter" → "Weights: rank" joined by an arrow; the winner's weakest board cell pulses orange
next:   cost × electricity scatter with five thumbnails at their real values; dashed outline around the non-dominated ones; "Pareto" chip
```

## 4. Beat table

Estimated seconds are computed by the toolkit formula (`hold_for`): maximum of German reading time (words / 1.6), German speech (words / 2.2), English speech (words / 2.5), minimum 1.5 s. Animations run inside that budget.

### Screen A

| key | visual state | animation | linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `variants` | Screen A without rows: five cards with heading and façade thumbnail | cards fade in left to right (LaggedStart 0.15), thumbnails build window then overhang slab | `variants()[i].title`, `.design` | Five façade variants for the same seminar room. | Fünf Fassadenvarianten für denselben Seminarraum. |
| `variants_2` | spec line appears under each thumbnail: window share · overhang · insulation | Indicate window and overhang of each thumbnail in turn, spec lines count in | `design.wwr`, `.overhang`, `.insulation` | Only window share, overhang and insulation differ. | Nur Fensteranteil, Vordach und Dämmung unterscheiden sich. |
| `criteria` | rows 1 and 2 on every card: electricity (bulb, ↓), cost (coin, ↓) | row-wise sweep over the five cards, values count up from 0 (1.2 s), arrow draws | `raw.electricity`, `raw.cost` of `variants()` | Electricity and cost: lower is better. | Strom und Kosten: weniger ist besser. |
| `criteria_2` | rows 3 and 4: daylight (sun, ↑), carbon (factory, ↓) | same sweep, the ↑ arrow in yellow pulses once | `raw.daylight`, `raw.carbon` | Daylight: higher is better. Carbon: lower is better. | Tageslicht: mehr ist besser. CO₂: weniger ist besser. |
| `best` | every row: best cell framed green with "best" tick | frames Create row by row (0.6 s each); thumbnails of the favourites pulse | best per row by `criteria()[c].direction` over raw | No variant is best everywhere. | Keine Variante ist überall die beste. |

### Screen B

| key | visual state | animation | linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `units` | `clear_stage`; the electricity and cost cells of the first variant detach and sit as two chips with "+", "=", "?"; red cross over "="; chip "no unit" | cells fly from card to centre (ReplacementTransform), cards fade, `cross_mark` draws | first variant `raw.electricity`, `raw.cost` | Kilowatt-hours per square metre plus francs? That sum has no unit. | Kilowattstunden pro Quadratmeter plus Franken? Die Summe hat keine Einheit. |
| `units_2` | five naive bars (thumbnail, name, one bar of four coloured segments: lengths = raw numbers as shown); cost segment fills nearly the whole bar | chips dissolve into row 1, other rows grow in; sum number at bar end | `raw` of all criteria summed, scaled to the longest bar | Added anyway, the biggest numbers decide, not the importance. | Trotzdem addiert, entscheiden die größten Zahlen, nicht die Wichtigkeit. |
| `units_3` | switch chip "Cost: CHF" becomes "Cost: thousand CHF"; cost segments shrink, carbon segments dominate; bars re-sort, winner tag jumps | chip flips (Transform), segment widths tween 1.5 s, rows swap, Indicate winner tag | naive sum with cost ÷ 1000; ranking differs from CHF case | Count cost in thousand francs instead and the ranking changes. | Rechne die Kosten in Tausend Franken, und die Rangfolge ändert sich. |

### Screen C

| key | visual state | animation | linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `ruler` | header chip with bulb; ruler 0 … D kWh/(m²·a) with ticks; five thumbnails on their true values (two lanes, leaders, tags); room panel frame appears | `clear_stage`; ruler draws left to right, thumbnails drop in sorted by value (LaggedStart 0.2), tags count in | `raw.electricity` per variant, ruler domain `[0, nice(max)]` | Give each criterion its own ruler, starting with electricity. | Gib jedem Kriterium ein eigenes Lineal, zuerst dem Strom. |
| `ruler_2` | panel shows the room of the pointed variant: window, overhang, sun rays, lamps, `energy_bars` with live kWh/a; flags "best" (green) and "worst" (red) | pointer travels best → worst, stops 0.5 s at each variant: `set_design` morphs room and bars together with the pointer | `evaluate(design)` of the focused variant: heating, cooling, lighting, intensity | Each variant sits at its real value; the room shows what it means. | Jede Variante liegt auf ihrem echten Wert; der Raum zeigt die Bedeutung. |
| `shift` | brace "− best" above the axis; the axis origin slides under the best thumbnail; tick labels now read "above best" | offset tracker `a: 0 → best` (2.0 s); thumbnails and ticks pan left, part left of the origin fades; tick numbers count continuously | `min(raw.electricity)` | Step one: subtract the best value of the set. | Schritt eins: Ziehe den besten Wert der Menge ab. |
| `shift_2` | brace "÷ span" from best to worst; ticks become 0, 0.25, 0.5, 0.75, 1; header: 0 = best · 1 = worst | scale tracker `k: k0 → L / span` (2.0 s), thumbnails spread, worst lands at x = 1.8 | `max − min` of `raw.electricity`; resulting `normalized.electricity` shown as tags | Step two: divide by the span from best to worst. | Schritt zwei: Teile durch die Spanne vom besten zum schlechtesten Wert. |
| `mapped` | brace under all five thumbnails labelled "compared set: 5 variants"; flags retarget to "0" and "1" | brace draws; pointer visits best (0) then worst (1), room panel follows | `index(w)[i].normalized.electricity` | Zero is the best of these five, one the worst of these five. | Null ist die beste dieser fünf, eins die schlechteste dieser fünf. |
| `cost` | header coin, orange; ruler in CHF; thumbnails glide to cost positions; panel: façade with price chip and cost split | thumbnails glide (1.2 s), then both moves replay fast (1.8 s); panel chip morphs per variant | `raw.cost`, `normalized.cost` | Cost gets the same ruler: the cheapest façade becomes zero. | Die Kosten erhalten dasselbe Lineal: Die günstigste Fassade wird null. |
| `daylight` | header sun, yellow, ↑ more is better; ruler in %, best = highest on the right; the mapping is mirrored (negative `k`) and the arrow flips to "0 = best"; panel: room with daylight depth gradient | thumbnails swing on arcs over the line to their mirrored places (2.5 s), arrow rotates 180° | `raw.daylight`, `normalized.daylight` | Daylight: more is better, so the ruler is flipped. | Tageslicht: mehr ist besser, also wird das Lineal umgedreht. |
| `matrix` | carbon ruler flashes through (factory, 1.0 s), then the four rulers fold into the table: thumbnail and name rows × four columns, each cell a filled bar with its number; header says 0 = best, 1 = worst | ruler thumbnails fly into the row labels, ticks become cell outlines (ReplacementTransform), cells fill left to right (LaggedStart) | `index(w)[i].normalized` for all four | Carbon follows the same rule: four comparable values between zero and one. | CO₂ folgt derselben Regel: vier vergleichbare Werte zwischen null und eins. |

### Screen D (weights, index, winner, sensitivity)

| key | visual state | animation | linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `weights` | header chips over the columns, all at 25 %, split bar of 100 % above; equal columns | chips and split bar fade in, percentages are `math_readout` of the weight trackers | `weight_sets.balanced` keys (equal start 1/4) | Weights say how much each criterion matters; together they make 100 %. | Gewichte sagen, wie wichtig jedes Kriterium ist; zusammen ergeben sie 100 %. |
| `weights_2` | columns re-proportioned to the balanced set; chips read 40 / 30 / 20 / 10 % | `set_weights` tween (2.5 s): column edges slide, fills stretch with their cell, split bar segments follow | `weight_sets.balanced` | Here: energy 40 %, cost 30 %, daylight 20 %, carbon 10 %. | Hier: Energie 40 %, Kosten 30 %, Tageslicht 20 %, CO₂ 10 %. |
| `weights_3` | widest column (energy) outlined; arrow note "wider = stronger pull" | Indicate the widest and narrowest column in turn | weights | A wider column means a stronger pull on the result. | Eine breitere Spalte zieht das Ergebnis stärker an. |
| `index` | compact state: empty cell remainders gone, filled parts form one coloured bar per row; score numbers at bar ends; thin axis hint index 0 … 1 | remainders fade, fills slide left and close gaps (1.8 s, LaggedStart per row), scores count up | `index(balanced)[i].score` = Σ weight × normalized | Slide the filled parts together: their length is the index. | Schiebe die gefüllten Teile zusammen: Ihre Länge ist der Index. |
| `index_2` | rows sorted by score, rank badges 1 … 5, winner row framed green | rows swap to rank order through rank trackers (2.0 s), badges pop in, frame draws | `index(balanced)[i].rank` | The shortest bar has the lowest index and ranks first. | Der kürzeste Balken hat den kleinsten Index und steht vorn. |
| `winner` | winner panel right: its room (window, overhang, rays, lamps) and its four raw values with glyphs and units | panel slides in, `set_design` to the winner's design, values count up | `variants()[winner]` evaluate and raw | Its room: not best everywhere, but best overall. | Sein Raum: nicht überall der beste, aber insgesamt der beste. |
| `winner_2` | the winner's weakest cell (largest `normalized`) outlined orange and linked to the strongest cell | outline draws, dashed arrow from weak to strong cell | argmax / argmin of winner `normalized` | Strengths elsewhere outweigh its weakest criterion. | Stärken anderswo gleichen sein schwächstes Kriterium aus. |
| `sensitivity` | header gets a persona chip "cost-driven client" (person glyph); weights slide to cost 60 %, others in the old proportions | `set_weights` tween (3.0 s): cost column widens, bars recompute live, rank numbers unchanged until the next clause | `reweigh(balanced, "cost", 0.6)` | A cost-driven client puts more weight on cost. | Ein kostenorientierter Bauherr gewichtet die Kosten stärker. |
| `sensitivity_2` | rows reorder, badges renumber | rank trackers tween (2.0 s), swapping pairs slide 0.25 sideways to avoid overlap | `index(cost_driven)[i].rank` | The bars change and the ranking reorders. | Die Balken ändern sich, und die Rangfolge ordnet sich neu. |
| `sensitivity_3` | new winner framed green, old winner outlined grey; winner panel swaps room (window and overhang morph) | `set_design` morph (1.5 s), Indicate new frame | new winner ≠ balanced winner | Another façade wins, though no variant changed. | Eine andere Fassade gewinnt, obwohl sich keine Variante geändert hat. |

### Screen E (sweep)

| key | visual state | animation | linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `sweep` | bump chart (rank vs cost weight %), marker at 60 %; winner strip; room panel of the current winner; weights as chips | `clear_stage`; lines draw; marker slides 60 → 0 → 100 % (4.0 s), room panel and strip follow live, at each switch the room morphs | `index(reweigh(balanced, "cost", w))` for w = 0, 5, …, 100 % | Sweep the cost weight to see where the winner switches. | Verschiebe das Kostengewicht und sieh, wo der Sieger wechselt. |
| `sweep_2` | band where the top two scores differ by less than 0.03 shaded orange with label "close call" | band fades in, marker rests inside it, both rooms flash alternately | `score` gap of rank 1 and 2 | Close calls are fragile: test the ranking before you decide. | Knappe Entscheide sind fragil: Prüfe die Rangfolge, bevor du entscheidest. |

### Screen C again, then D (range dependence)

| key | visual state | animation | linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `range` | cost ruler in normalized state (best 0, worst 1) with five thumbnails; a sixth thumbnail (`variants(extra=True)[5]`, noticeably dearer) drops in far beyond the old worst; its room appears in the panel | `clear_stage`; ruler rebuilt; thumbnail falls in from the top (1.2 s) with price tag | `variants(extra=True)[5].raw.cost` | The rulers depend on the variants compared: add a sixth façade. | Die Lineale hängen von der Vergleichsmenge ab: Füge eine sechste Fassade hinzu. |
| `range_2` | old worst is no longer 1; the five old thumbnails squeeze towards 0; ghost ticks mark their former places; tags show old → new value | scale tracker `k` re-tweens to the new span (2.5 s), tags `math_readout` | `index(w, ids)` normalized cost for 5 vs 6 variants | Best and worst move, so all other normalized values shrink. | Bester und schlechtester Wert verschieben sich, alle anderen Werte schrumpfen. |
| `range_3` | board (screen D) with six rows; first five carry rank before → after; the pair whose order flipped is linked by a ⇄ arrow in orange | `clear_stage`, board builds in compact state, rows slide to the new order (2.0 s), arrow draws | `index(w)` over 5 vs `index(w, ids)` over 6: rank change among the original five | The order of the original five can flip, though they did not change. | Die Reihenfolge der ersten fünf kann kippen, obwohl sie gleich blieben. |

### Screen F (record, limits, bridge)

| key | visual state | animation | linked values | EN | DE |
| --- | --- | --- | --- | --- | --- |
| `record` | decision record card in four compartments: weights chips, five thumbnails (compared set), ranges best … worst per criterion with units, result (winner, index) | `clear_stage`; compartments fill by flying in existing chips and thumbnails (LaggedStart 0.4, 3.5 s) | `weight_sets.balanced`, min/max of `raw` per criterion, winner | Document weights, compared variants and ranges with every decision. | Dokumentiere Gewichte, Varianten und Wertebereiche bei jeder Entscheidung. |
| `limits` | card shrinks left; right: chips "Constraints: filter" → "Weights: rank" with arrow; winner's weak cell orange | card scales to the left (1.2 s), chips and arrow draw, weak cell pulses | none (symbolic), weak cell of winner | Strengths can offset weaknesses; mandatory limits stay constraints. | Stärken gleichen Schwächen aus; zwingende Grenzen bleiben Nebenbedingungen. |
| `next` | scatter: cost (CHF) × electricity (kWh/(m²·a)), five thumbnails at their real values; outline around those no other variant beats on both | `clear_stage`; axes and thumbnails fade in, dashed hull draws | `raw.cost`, `raw.electricity`; non-dominated set computed in the scene | To see trade-offs before weighting, look at Pareto fronts. | Um Zielkonflikte vor dem Gewichten zu sehen, betrachte Pareto-Fronten. |

## 5. Model requirements this storyboard relies on

All numbers on screen are model outputs; the following assertions must hold (tests in `🧪️cases/🔣️.json`, add to the model brief) or the beat named in brackets is replaced by its fallback:

1. `criteria()` order `electricity`, `cost`, `daylight`, `carbon`, directions min, min, max, min; units exactly as in section 2.
2. `variants()` returns five variants with short titles (en, de) and `design`; `variants(extra=True)` returns six (the sixth marked `"extra": true`, not in the default list). The sixth is dearer than the old maximum (clearly stretches the cost range) and is itself never the winner.
3. No variant is best in all four criteria; at least three distinct variants are best in some criterion (`best`).
4. `study["weights"]`: `balanced` = electricity 0.40, cost 0.30, daylight 0.20, carbon 0.10. Helper `reweigh(weights, criterion, share)`: sets one criterion's share, scales the others in their old proportions (cost 0.60 gives 23 / 11 / 6 % in display rounding with sum 100).
5. `index(weights, ids=None)`: min–max normalization over the compared ids; best = 0, worst = 1, daylight reversed; score = Σ weight × normalized; rank 1 = lowest score.
6. Winner under `balanced` differs from the winner under cost 0.60 (`sensitivity_3`); the bump chart has at least one switch in cost weight 0 … 100 %; at 60 % no tie.
7. The order of the original five differs between `index(balanced)` over five and over six variants; the sixth is not rank 1 (`range_3`).
8. Naive sums (all raw numbers added) rank differently with cost in CHF and cost in thousand CHF (`units_3`). If not, the scene keeps only `units` and `units_2` and replaces `units_3` by an Indicate of the dominating segment.
9. Normalized, rank and score values are exposed unrounded; the scene rounds for display only. Scores are not all equal and consecutive scores differ by at least 0.02 under `balanced` (bars distinguishable).
10. The raw ranges of the five variants leave thumbnail room on the ruler: no more than two thumbnails closer than 4 % of the ruler (the dodge handles two lanes).

## 6. Toolkit widgets needed beyond `api.md`

`api.md` provides `thumbnail`, `room_section`, `energy_bars`, `set_design`, `study_trackers`, `facade`, `plan`, `live_marker`, `number`, `quantity`. This scene additionally needs (toolkit agent, all in `🎨️visuals/🐍️.py` or a small `⚖️decisions` helper; all read numbers from the model, no typed values):

| Widget | Purpose | Contract sketch |
| --- | --- | --- |
| `criterion_style(id)` | colour, glyph, direction arrow and unit for the four criteria | returns dict `{color, glyph(point, scale), arrow(direction), unit}`; glyphs from `object_silhouette` plus new coin |
| `coin_glyph(point, scale, label="CHF")` | cost glyph | circle with ring and label |
| `variant_cards(variants, rows)` | screen A | VGroup of `window_card`s with heading, `thumbnail`, spec line and per-criterion row handles `cards[i].row(id)`; per-cell frame method for the green best frame |
| `value_ruler(domain, unit, markers, trackers, lanes=2)` | screen C and range | line, ticks and labels as `always_redraw` of two trackers `offset`, `scale` (screen x = x0 + (value − offset) × scale); markers = (thumbnail, value) with leader lines, tag lanes above and below (dodging, no overlap); tick label mode `absolute | above_best | normalized`; negative `scale` mirrors; animation helpers `shift_ruler(ruler, to, run_time)` and `stretch_ruler(ruler, to, run_time)`; flags `best`/`worst`; `compared_set_brace` |
| `ruler_panel(trackers, kind)` | right panel on screen C and E (and winner panel on D) | room section plus criterion overlay: `energy_bars` for electricity, `price_chip` + cost split for cost, daylight gradient layer for `room_section` (`layer="daylight"`) for daylight, `carbon_cloud(value)` for carbon; `focus(variant, run_time)` animates `set_design` |
| `daylight_layer(trackers, point, scale)` | floor/ceiling gradient and mean-factor label for `room_section` | needs `evaluate().daylight` and `autonomy` |
| `carbon_cloud(value, point)` / `price_chip(value, point)` | meaning of carbon and cost | cloud scales with kg CO₂e, chip with CHF from the model |
| `index_board(variants, criteria, weight_trackers, ids)` | screens D and range | header chips with live percent, split bar, rows (rank badge, thumbnail, name, four cells, score); a cell = outline + filled fraction + number; column widths and fills as `always_redraw` of the weight trackers; one cached `index(weights)` call per frame (hidden updater), not one per row; `board.compact(run_time)` (fade remainders, slide fills together, score count-up); rank trackers per row with `board.rank_to(run_time)` (swapping rows slide apart 0.25 sideways); `board.add_row(variant)`; `board.mark_flip(a, b)` |
| `set_weights(trackers, weights, run_time)` | tween the four weight trackers together | analogous to `set_design` |
| `segment_row(segments, point, unit_length)` | naive sum bars on screen B (and the compact bar) | horizontal counterpart of `stacked_bar` returning bars, anchors, end point |
| `unit_switch(left, right)` | the CHF ⇄ thousand CHF toggle chip | two chips, Transform on flip |
| `bump_chart(series, x_label, marker_tracker, band)` | screen E | rank axes (ordinal), lines per variant with thumbnail at the end, marker bound to a tracker, shaded band, winner strip with thumbnails |
| `decision_record(weights, variants, ranges, result)` | screen F | `window_card` with four compartments built from existing chips and thumbnails |
| `scatter_with_thumbnails(points, x, y)` | the Pareto teaser on `next` | axes with units, thumbnails at real values, dashed outline of non-dominated points |
| `rank_badge(rank)` | numbered round badge, green for 1 | small VGroup |
| `layout_conflicts` support | none extra; but board and ruler labels need stable names so conflicts can be reported | — |

## 7. Implementation notes for phase 2

- Scene file stays one class `Beat4_NormalizedIndex` with `present_index`; a small number of private helpers per screen (`_cards`, `_naive`, `_ruler_stage`, `_board`, `_sweep`, `_record`). Constants for coordinates at the top of the class.
- Shared trackers: one `study_trackers` per room panel, one `weights` dict of four `ValueTracker`s, one `ruler` pair (`offset`, `scale`). Nothing is moved by hand: moving a tracker moves the room, the bars, the ranks and the numbers.
- `capture.py` replaces animation time by 0.01 s: all end states must come from trackers or `Transform`, never from time-based updaters (rank swapping and compact step use trackers for this reason).
- Every clause after the first of a key is a plain `step` with its own small animation where the table says so; the first clause carries the main animation.
- Layout checks to run: caption band clear, no overlap between value tags on rulers, header chips and column widths at 10 % weight (cell width 0.62 must still hold a 16 pt number), German strings (longer) on screens A and D.
- Hand over: model gaps from section 5 and widgets from section 6 first; then scene.

## 8. Timing

34 clauses (all ≤ 79 characters English and ≤ 79 German, limit 95). Hold budget by the toolkit formula 198.8 s (3.3 min); with `clear_stage` fades, caption fades and the title run the clip lasts about 3.4 min, inside the 2.5 to 4 minute target. Longest animations (weights tween 2.5 s, sensitivity tween 3.0 s, sweep 4.0 s, record 3.5 s) are all shorter than their clause budget, so no clause is stretched.

| Screen | Clauses | Seconds |
| --- | --- | --- |
| A cards | `variants` … `best` (5) | 20 |
| B units | `units` … `units_3` (3) | 19 |
| C rulers and table | `ruler` … `matrix` (8) | 51 |
| D weights, index, winner, sensitivity | `weights` … `sensitivity_3` (10) | 60 |
| E bump chart | `sweep`, `sweep_2` (2) | 12 |
| Range dependence | `range` … `range_3` (3) | 21 |
| F record, limits, bridge | `record`, `limits`, `next` (3) | 16 |

If the owner wants it shorter: drop `variants_2`, `weights_3`, `winner_2` and `sweep_2` (about 19 s) without breaking the line of thought.

## 9. Ready-to-paste `NARRATION` (generated from the tables above)

```python
    NARRATION = [
        ["variants", "Five façade variants for the same seminar room.", "Fünf Fassadenvarianten für denselben Seminarraum."],
        ["variants_2", "Only window share, overhang and insulation differ.", "Nur Fensteranteil, Vordach und Dämmung unterscheiden sich."],
        ["criteria", "Electricity and cost: lower is better.", "Strom und Kosten: weniger ist besser."],
        ["criteria_2", "Daylight: higher is better. Carbon: lower is better.", "Tageslicht: mehr ist besser. CO₂: weniger ist besser."],
        ["best", "No variant is best everywhere.", "Keine Variante ist überall die beste."],
        ["units", "Kilowatt-hours per square metre plus francs? That sum has no unit.", "Kilowattstunden pro Quadratmeter plus Franken? Die Summe hat keine Einheit."],
        ["units_2", "Added anyway, the biggest numbers decide, not the importance.", "Trotzdem addiert, entscheiden die größten Zahlen, nicht die Wichtigkeit."],
        ["units_3", "Count cost in thousand francs instead and the ranking changes.", "Rechne die Kosten in Tausend Franken, und die Rangfolge ändert sich."],
        ["ruler", "Give each criterion its own ruler, starting with electricity.", "Gib jedem Kriterium ein eigenes Lineal, zuerst dem Strom."],
        ["ruler_2", "Each variant sits at its real value; the room shows what it means.", "Jede Variante liegt auf ihrem echten Wert; der Raum zeigt die Bedeutung."],
        ["shift", "Step one: subtract the best value of the set.", "Schritt eins: Ziehe den besten Wert der Menge ab."],
        ["shift_2", "Step two: divide by the span from best to worst.", "Schritt zwei: Teile durch die Spanne vom besten zum schlechtesten Wert."],
        ["mapped", "Zero is the best of these five, one the worst of these five.", "Null ist die beste dieser fünf, eins die schlechteste dieser fünf."],
        ["cost", "Cost gets the same ruler: the cheapest façade becomes zero.", "Die Kosten erhalten dasselbe Lineal: Die günstigste Fassade wird null."],
        ["daylight", "Daylight: more is better, so the ruler is flipped.", "Tageslicht: mehr ist besser, also wird das Lineal umgedreht."],
        ["matrix", "Carbon follows the same rule: four comparable values between zero and one.", "CO₂ folgt derselben Regel: vier vergleichbare Werte zwischen null und eins."],
        ["weights", "Weights say how much each criterion matters; together they make 100 %.", "Gewichte sagen, wie wichtig jedes Kriterium ist; zusammen ergeben sie 100 %."],
        ["weights_2", "Here: energy 40 %, cost 30 %, daylight 20 %, carbon 10 %.", "Hier: Energie 40 %, Kosten 30 %, Tageslicht 20 %, CO₂ 10 %."],
        ["weights_3", "A wider column means a stronger pull on the result.", "Eine breitere Spalte zieht das Ergebnis stärker an."],
        ["index", "Slide the filled parts together: their length is the index.", "Schiebe die gefüllten Teile zusammen: Ihre Länge ist der Index."],
        ["index_2", "The shortest bar has the lowest index and ranks first.", "Der kürzeste Balken hat den kleinsten Index und steht vorn."],
        ["winner", "Its room: not best everywhere, but best overall.", "Sein Raum: nicht überall der beste, aber insgesamt der beste."],
        ["winner_2", "Strengths elsewhere outweigh its weakest criterion.", "Stärken anderswo gleichen sein schwächstes Kriterium aus."],
        ["sensitivity", "A cost-driven client puts more weight on cost.", "Ein kostenorientierter Bauherr gewichtet die Kosten stärker."],
        ["sensitivity_2", "The bars change and the ranking reorders.", "Die Balken ändern sich, und die Rangfolge ordnet sich neu."],
        ["sensitivity_3", "Another façade wins, though no variant changed.", "Eine andere Fassade gewinnt, obwohl sich keine Variante geändert hat."],
        ["sweep", "Sweep the cost weight to see where the winner switches.", "Verschiebe das Kostengewicht und sieh, wo der Sieger wechselt."],
        ["sweep_2", "Close calls are fragile: test the ranking before you decide.", "Knappe Entscheide sind fragil: Prüfe die Rangfolge, bevor du entscheidest."],
        ["range", "The rulers depend on the variants compared: add a sixth façade.", "Die Lineale hängen von der Vergleichsmenge ab: Füge eine sechste Fassade hinzu."],
        ["range_2", "Best and worst move, so all other normalized values shrink.", "Bester und schlechtester Wert verschieben sich, alle anderen Werte schrumpfen."],
        ["range_3", "The order of the original five can flip, though they did not change.", "Die Reihenfolge der ersten fünf kann kippen, obwohl sie gleich blieben."],
        ["record", "Document weights, compared variants and ranges with every decision.", "Dokumentiere Gewichte, Varianten und Wertebereiche bei jeder Entscheidung."],
        ["limits", "Strengths can offset weaknesses; mandatory limits stay constraints.", "Stärken gleichen Schwächen aus; zwingende Grenzen bleiben Nebenbedingungen."],
        ["next", "To see trade-offs before weighting, look at Pareto fronts.", "Um Zielkonflikte vor dem Gewichten zu sehen, betrachte Pareto-Fronten."]
    ]
```
