# Storyboard: `📉️extrema` · `Beat1_LocalGlobalExtrema` · "Minimization and Maximization"

Phase 1 (storyboard only). Target running time about 3:30 (29 clauses, one per 6-8 s, animations run inside their clause). Narration keys follow the `explain` contract: `key` starts the beat, `key_2 … key_n` follow on the same diagram state.

## 1. What is wrong today

Captured with `T/capture.py 📉️extrema Beat1_LocalGlobalExtrema en`: a three-window pavilion glyph that never changes after the first beat, then a bare quartic `f(x) = x⁴/4 − x³/3 − x²` on axes with no units, the points `L`/`G` at `x = −1`/`2` with a value of "−2.67" that means nothing, and a constraint `x ≤ 0.5` that has no reason. The picture of the building and the plotted value are never connected. Pedagogical points kept: objective direction, domain, local vs. global minimum (circle vs. square), design `θ*` vs. value `E(θ*)`, `min f ⇔ max −f`, constraint limits feasibility and can move the optimum, scope of any optimality claim.

## 2. Concept

One seminar room (8 m × 6 m × 3 m, only the 8 m façade is glazed), Zurich-like climate, simplified annual teaching model. Fixed design while turning: window-to-wall ratio, overhang, insulation from `orientation_study()["design"]` (planned: 60 %, 0 m, 16 cm), shown once as a design chip row. **Design variable** = bearing θ of the glazed façade, compass convention E 90°, S 180°, W 270°, N 360°, E again 450° (θ is the compass bearing of the façade normal). **Objective** = annual electricity E(θ) = heating + cooling + lighting in kWh/a, minimized.

One `ValueTracker` θ drives everything at the same time: the plan rotates, the section changes sun entry, the stacked bar changes, the plot marker moves. Nothing on screen is typed by hand; the strings below only name model fields.

Physical story per direction (the model is tuned to this; Phase 2 of the model agent must confirm it, see section 7):

| θ | Façade | Heating | Cooling | Lighting | Result |
| --- | --- | --- | --- | --- | --- |
| 90° E | morning sun | medium | low summer sun enters deep: high | medium | high, local max |
| 180° S | midday sun | low winter sun is free heat: lowest | high summer sun strikes steeply: modest | lowest | **global minimum** |
| 270° W | evening sun | medium | low summer evening sun: peak | medium | highest, local max |
| 360° N | diffuse | highest | lowest | highest of the four | **local minimum**, at least 15 % worse than S |
| 450° E | = 90° | | | | curve closes |

## 3. Screen layout (16:9, frame 14.22 × 8 units)

Stage between title tab and caption band: y from −2.55 to +3.05 (the title tab sits top-left at y ≈ 3.3-3.85, the two-line caption band starts at y ≈ −2.6; nothing but the caption lives below).

### 3a. Dashboard (beats from `domain` to `compass`, reused in the maximize and constraint phases)

```
 title tab (shared)
+----------------------+--------------------------------------------------+------------+
| A  PLAN  x -6.9..-2.9| B  PLOT  x -2.6..4.7                             | C  BAR     |
|  y 3.05 .. -0.65     |  readout row y 2.75:                             | x 5.0..6.9 |
|                      |   θ = 180° · South      E = 12,340 kWh/a         |            |
|    N                 |  +----------------------------------------+     |  total     |
|  ( compass r 1.2 )   |  | E in kWh/a                              |     |  12,340    |
|  W ( room 8x6 m )  E |  |   curve E(θ), live marker + drop lines  |     |  [light.]  |
|  ( rotates with θ )  |  |   ○ local min N   ■ global min S        |     |  [cool. ]  |
|    S                 |  |   shaded bands, dashed limits           |     |  [heat. ]  |
|  sun arcs r 1.4/1.65 |  +----------------------------------------+     |  kWh/a     |
|  winter yellow       |   E     S     W     N     E   (compass tick row) |            |
|  summer orange       |   90°  180°  270°  360°  450°                  |  ref ghost |
+----------------------+   Orientation of the glazed façade θ (°)       |  (later)   |
| A2 SECTION           |                                                |            |
|  y -0.85 .. -2.50    |  fixed-design chips under axis title (y -2.45)  |            |
|  window left, sun    |                                                |            |
|  rays winter/summer, |                                                |            |
|  floor patch, lamps  |                                                |            |
+----------------------+--------------------------------------------------+------------+
 caption band (shared, 2 lines max)
```

Coordinates (units): plan centre (−4.9, 1.1), compass ring r 1.2, winter arc r 1.4, summer arc r 1.65, room scale 0.15 u/m; section centre (−4.9, −1.7) 3.6 wide × 1.5 high with `room_section`; plot axes x −1.8…4.5 (6.3), y −1.55…2.1 (3.65); compass tick row y −1.85, axis title y −2.2, design chip row y −2.5; stacked bar centre x 5.4, width 0.8, y −2.0…2.0 with total readout at y 2.4, segment labels via `side_labels` x 5.95…6.9. Curve colour `P_CYAN`, live marker `P_WHITE`, local minimum circle `P_YELLOW`, global minimum square `P_GREEN`, constraint band `P_ORANGE` (opacity 0.18) with dashed limit, bar segments `P_RED` heating / `P_BLUE` cooling / `P_YELLOW` lighting.

### 3b. Intro layout (beats `formulate` … `response_2`)

Plan large and alone, left of centre: compass ring r 2.2 at (−3.0, 0.2), room scale 0.28 u/m (2.24 × 1.68), summer/winter arcs outside the ring. Right half (x 1.2…6.8) holds the θ card (`formulate_2`), then the stacked bar column and the section (`response`), then the "simplified annual teaching model" chip (`response_2`). The dashboard of 3a is built from this layout by shrinking the plan to column A, which also moves the compass ring.

### 3c. Savings layout (beats `maximize` … `negate_2`)

Same A and B. In B the left axis (E) gets a twin: right axis "Savings S in kWh/a" with a second tick column "CHF/a" at the tariff. Column C shows two bars: ghost east reference (outlined, label `E(90°)`) and the live bar; the savings bracket between the two tops reads `S = … kWh/a · … CHF/a` on a price chip. Plan shows a small ghost east room outline (dashed) next to the live room.

### 3d. Constraint layout (beats `constraint` … `feasible_2`)

Same A, B, C. Extra in A: street glyph (grey road band, car, noise ripples) outside the south edge of the plan, courtyard glyph (trees, green patch) outside the north; compass south half-ring 90°-270° shaded `P_ORANGE` with "street noise" label. Extra in B: infeasible band 90°-270° shaded and hatched, dashed limit at 270°, label "infeasible: street noise".

### 3e. Scope layout (beats `scope`, `scope_2`)

Plan, section and bar fade to 30 %; centred card (x −3.2…3.2, y −1.9…2.5) with three rows: Direction, Domain, Scope; the plot shrinks behind it to a thumbnail top right.

## 4. Beat table

Placeholders in `{braces}` are resolved from the model at build time (`os` = `orientation_study()`): `{θ_G}`,`{E_G}` = `os.global_minimum` and its electricity; `{θ_L}`,`{E_L}` = the north entry of `os.local_minima`; `{start}` = `os.start`; `{E_ref}` = electricity at `os.reference`; `{lo}`,`{hi}` = `os.constraint`; `{θ_F}`,`{E_F}` = `os.feasible_minimum` and its electricity; `{tariff}` = `os.tariff`; derived in the scene from `evaluate`: `{waste_kwh}` = E_L − E_G, `{waste_pct}` = 100·waste_kwh/E_G, `{waste_chf}` = waste_kwh·tariff, `{S_kwh}` = E_ref − E_G, `{S_chf}` = S_kwh·tariff, `{price_kwh}`/`{price_chf}` the same for the constrained optimum. Number formatting by `number()`/`quantity()` (en `12,340`, de `12.340`). Animation seconds are the run times passed to `step`; the clause hold comes from `hold_for`.

Caption length budget: ≤ 95 characters with placeholders counted as 6 characters (checked by `T/check-extrema-narration.py`).

### Phase A: formulate (layout 3b)

| Key | English | Deutsch | Visual state | Animation | Linked values |
| --- | --- | --- | --- | --- | --- |
| `formulate` | A seminar room has one glazed façade. We choose which compass direction it faces. | Ein Seminarraum hat eine verglaste Fassade. Wir wählen, in welche Himmelsrichtung sie zeigt. | Plan alone: 8 m × 6 m room with cyan glazed 8 m wall, dimension labels "8 m", "6 m", compass N E S W, summer and winter sun arcs; the room faces south. | FadeIn plan (1.2 s); the room turns once E → S → W → N and back to S (4 s) while the bearing readout `θ = …°` follows. | `θ` tracker = 180° at rest |
| `formulate_2` | The façade bearing θ is our design variable: east 90°, south 180°, west 270°, north 360°. | Die Fassadenrichtung θ ist unsere Entwurfsvariable: Ost 90°, Süd 180°, West 270°, Nord 360°. | θ card on the right: "Design variable θ · bearing of the glazed façade". Below it the compass ring is "unrolled" into a strip E 90° · S 180° · W 270° · N 360° · E 450° with a marker. | Strip transforms from the compass ring ticks (1.2 s); marker follows the rotating room once more (S, 180°). | ring ticks → strip ticks, `os.range` = 90°-450° |
| `response` | The objective is the room's electricity for heating, cooling and lighting, in kWh per year. | Das Ziel ist der Strombedarf des Raums für Heizen, Kühlen und Beleuchten in kWh pro Jahr. | Stacked bar column right (heating red, cooling blue, lighting yellow) with live kWh/a per segment and total; below the plan: section with winter and summer sun, floor patch, lamps. | FadeIn bar + section (1.2 s); segments grow from zero with count-up (1.5 s). | `evaluate(θ=180°)`: `heating`, `cooling`, `lighting`, `electricity`, `autonomy` (lamp dimming) |
| `response_2` | We minimize it: lower is better. The values come from a simplified annual teaching model. | Wir minimieren ihn: Kleiner ist besser. Die Werte stammen aus einem vereinfachten Jahresmodell. | Arrow "better ↓" beside the bar; chip row at the stage bottom: "Zurich-like climate · window {wwr} % · overhang {d} m · insulation {t} cm"; no x, no f(x). | Fade arrow + chips (1 s). | `os.design` fields |

### Phase B: domain and sweep (layout 3a)

| Key | English | Deutsch | Visual state | Animation | Linked values |
| --- | --- | --- | --- | --- | --- |
| `domain` | Now we turn the façade and record the electricity, starting east at 90 degrees. | Nun drehen wir die Fassade und erfassen den Strombedarf, beginnend im Osten bei 90 Grad. | Layout shifts to the dashboard: plan to column A, section below it, bar to column C, empty axes appear in B with the compass tick row E S W N E / 90° 180° 270° 360° 450°; the room faces east, marker at 90°, bar and section show the east values. | Plan shrinks and moves (1.2 s), axes Create (1.2 s), θ → 90° (1 s). | `evaluate(θ=90°)`, marker readout `θ = 90° · East · E = … kWh/a` |
| `domain_2` | Axes: orientation θ from 90° to 450°, and annual electricity in kWh/a. Lower is better. | Achsen: Ausrichtung θ von 90° bis 450° und jährlicher Strombedarf in kWh/a. Kleiner ist besser. | Axis titles "Orientation of the glazed façade θ (°)" and "Electricity E (kWh/a)", y ticks from the model range; "↓ better" tag at the y axis. | Labels FadeIn (1 s). | y range from min/max of `os.samples` |
| `sweep` | Turning toward south, the low winter sun heats the room for free: heating electricity falls. | Beim Drehen nach Süden heizt die tiefe Wintersonne den Raum gratis: Der Heizstrom sinkt. | Marker travels 90° → 180°, curve is drawn behind it; the plan rotates; the section's winter ray goes deep, floor patch grows; red heating segment shrinks live. | θ 90° → 180° (4 s) with `Create` along the curve; winter ray emphasised (`Indicate`). | `heating`, `solar_winter`, winter arc highlighted |
| `sweep_2` | The high summer sun strikes south glass steeply, so cooling stays modest: this total is lowest. | Die hohe Sommersonne trifft Südglas steil, die Kühlung bleibt mäßig: Summe am kleinsten. | Marker rests at 180°; summer ray is steep and mostly shallow on the floor; cooling segment thin; green square-shaped ring pulses at the minimum (not yet labelled). | Hold at 180° with `Indicate` on the summer ray (1.5 s). | `cooling`, `solar_summer`, `shading_summer` |
| `sweep_3` | Facing west, the low summer evening sun floods the room: cooling and the curve peak. | Nach Westen flutet die tiefe Abendsonne im Sommer den Raum: Kühlung und Kurve steigen an. | Marker 180° → 270°; third (low summer) ray enters horizontally, floor patch deep, blue cooling segment dominates; curve peaks (local maximum). | θ 180° → 270° (3.5 s). | `cooling`, `solar_summer`, local maximum from `os.local_maxima` |
| `sweep_4` | Facing north, little direct sun enters: cooling is least, but heating electricity rises again. | Nach Norden fällt kaum direkte Sonne ein: Kühlung minimal, aber der Heizstrom steigt wieder. | Marker 270° → 360°; rays fade out (sun behind the façade), cooling thin, red heating segment large; curve falls into a second valley. | θ 270° → 360° (3 s). | `heating`, `cooling`, second valley from `os.local_minima` |
| `sweep_5` | At 450° the façade faces east again, so the curve ends where it began. | Bei 450° zeigt die Fassade wieder nach Osten, die Kurve endet also, wo sie begann. | Marker to 450°; plan matches the 90° state; both ends of the curve highlighted with the same height. | θ 360° → 450° (2 s), pulse both ends. | `E(450°) = E(90°)` |

### Phase C: local and global minimum (layout 3a)

| Key | English | Deutsch | Visual state | Animation | Linked values |
| --- | --- | --- | --- | --- | --- |
| `local` | Start with the façade facing north-west, {start}°, and always turn toward less electricity. | Wir starten mit Nordwest, {start}°, und drehen stets dorthin, wo der Strombedarf kleiner wird. | White ring "start" on the curve at 315°; plan shows the room facing north-west; bar and section show its values; small step arrows (± 5°) around the marker. | Marker jumps to start (0.8 s); descent steps 315° → {θ_L}° with the room rotating step by step (3 s, `MoveAlongPath` / `descent` list). | `os.start`, `os.descent` list |
| `local_2` | Every neighbouring orientation is worse, so we stop: a local minimum at {θ_L}°, north. | Jede Nachbarrichtung ist schlechter, also halten wir an: lokales Minimum bei {θ_L}°, Nord. | Circle marker `P_YELLOW` labelled "local minimum" at north; two ghost probes at ±5° with gap chips `+… kWh/a`; plan faces north. | Probes slide out and back (1.5 s); circle FadeIn. | `E(θ_L ± 5°) − E(θ_L)` |
| `global` | But south, {θ_G}°, is lower still: it is the global minimum over the whole range. | Doch Süd, {θ_G}°, liegt noch tiefer: Es ist das globale Minimum im gesamten Bereich. | Green square `P_GREEN` "global minimum" at south; two ghost bars side by side (north faded, south live); vertical gap bracket on the plot labelled `−{waste_kwh} kWh/a`. | Marker travels north → south (3 s) over the hump; square FadeIn; bracket Create. | `os.global_minimum`, `waste_kwh`, `waste_pct` |
| `global_2` | Stopping at north would waste {waste_kwh} kWh/a, {waste_pct} % more, about {waste_chf} CHF every year. | Der Halt im Norden verschenkt {waste_kwh} kWh/a, {waste_pct} % mehr, etwa {waste_chf} CHF pro Jahr. | Price chip (coin glyph) on the gap bracket: "+… kWh/a · +…% · … CHF/a" in `P_ORANGE`. | Chip FadeIn + `Indicate` (1.2 s). | `waste_kwh`, `waste_pct`, `waste_chf` (tariff) |
| `argument` | The optimal design is θ* = {θ_G}°, south. Its value is E(θ*) = {E_G} kWh/a. | Der optimale Entwurf ist θ* = {θ_G}°, Süd. Sein Wert ist E(θ*) = {E_G} kWh/a. | Two linked chips centred above the plot: left "Design θ* = 180° · South" with a mini plan glyph, right "Value E(θ*) = … kWh/a" with a mini bar; arrow "evaluate" between them. | Chips Write/FadeIn with a connecting arrow (1.5 s). | `os.global_minimum`, `E_G` |
| `argument_2` | Keep them apart: the design θ* is a direction, the value E(θ*) is kWh per year. | Nicht verwechseln: Der Entwurf θ* ist eine Richtung, der Wert E(θ*) sind kWh pro Jahr. | The "°" unit on the left chip and "kWh/a" on the right chip highlighted; the left chip points at the compass ring, the right at the bar total. | `Indicate` left then right (2 s). | units from model (°, kWh/a) |
| `compare` | With one variable we can scan every degree and compare all valleys, not just the nearest. | Mit einer Variablen prüfen wir jeden Grad und vergleichen alle Täler, nicht nur das nächste. | Thin scan line sweeps the whole curve, plan flickers through bearings; two result cards: "N 360° · … kWh/a" and "S 180° · … kWh/a", winner S ticked green. | Scan line 90° → 450° (3 s) with room rotation at low fps; cards FadeIn. | `os.samples`, `os.local_minima` |
| `compare_2` | With many variables, a search can end in the wrong valley, just as our start at {start}° did. | Bei vielen Variablen kann eine Suche im falschen Tal enden, wie unser Start bei {start}°. | Descent path from the start re-appears dimmed, ending at the north circle; footnote chip "later videos: sampling and search". | Path redraw (1.5 s). | `os.descent` |

### Phase D: maximization (layout 3c)

| Key | English | Deutsch | Visual state | Animation | Linked values |
| --- | --- | --- | --- | --- | --- |
| `maximize` | The facility manager asks the opposite: how much electricity do we save against an east façade? | Die Betriebsleitung fragt umgekehrt: Wie viel Strom sparen wir gegenüber einer Ostfassade? | Ghost east room (dashed outline) beside the live room; dashed horizontal reference line at `E(90°)` across the plot labelled "east reference"; ghost bar beside the live bar. | Plan ghost + line + ghost bar FadeIn (1.5 s). | `os.reference`, `E_ref` |
| `maximize_2` | Now the goal is to maximize the savings S, in kWh per year and in Swiss francs per year. | Jetzt wollen wir die Einsparung S maximieren, in kWh pro Jahr und in Franken pro Jahr. | Right axis "Savings S (kWh/a)" with a second tick column "CHF/a" at the tariff; formula label `S(θ) = E(90°) − E(θ)`; the price chip shows the live saving at the marker. | Axis + formula FadeIn (1.5 s). | `os.tariff`, `S(θ)` per sample, `savings_chf` |
| `negate` | Minimizing E equals maximizing S: the curve flips over the reference, valleys become peaks. | E minimieren heißt S maximieren: Die Kurve klappt um die Bezugslinie, Täler werden Gipfel. | Curve E(θ) (cyan) morphs into S(θ) (green) by reflection about the reference line; formula `min E ⇔ max S` appears; both extremum markers keep their θ. | `ReplacementTransform` reflected curve (3 s), markers follow. | `S = E_ref − E`, same θ for both markers |
| `negate_2` | The highest saving sits at the same orientation as the lowest electricity: θ* = {θ_G}°. | Die größte Einsparung liegt bei derselben Ausrichtung wie der kleinste Strom: θ* = {θ_G}°. | Marker at 180° on the peak; price chip "S = … kWh/a = … CHF/a"; plan: live south room next to the ghost east room, bars side by side with the bracket. | Marker glides to the peak (1.5 s), chip `Indicate`. | `S_kwh`, `S_chf`, `θ_G` |

### Phase E: constraint (layout 3d)

| Key | English | Deutsch | Visual state | Animation | Linked values |
| --- | --- | --- | --- | --- | --- |
| `constraint` | The street to the south is loud: the glazed façade must face the quiet courtyard, north ± 90°. | Die Straße im Süden ist laut: Die Glasfassade muss zum ruhigen Hof zeigen, Nord ± 90°. | Curve returns to E(θ) (green → cyan, reverse of `negate`); plan gets street glyph (south) and courtyard glyph (north); south half ring 90°-270° shaded; plot band 90°-270° shaded with dashed limit at 270°, label "infeasible: street noise"; constraint label `{lo}° ≤ θ ≤ {hi}°`. | Curve reverse (1.5 s), street and courtyard FadeIn (1 s), bands Create (1.5 s). | `os.constraint` |
| `constraint_2` | A constraint limits which designs are allowed. It adds no objective and changes no value of E. | Eine Nebenbedingung begrenzt zulässige Entwürfe. Sie ergänzt kein Ziel, ändert keine Werte. | The green square is crossed out in grey; the marker is dragged toward 180°, stops at the limit (270°, west), the room refuses to turn (red flash), and snaps back. Values on the curve inside the band stay visible, only dimmed. | Marker attempt (2.5 s), `cross_mark`, flash. | `os.constraint`, `E(θ)` unchanged |
| `feasible` | Among the allowed orientations, north is now the global optimum: {θ_F}°, E = {E_F} kWh/a. | Unter den zulässigen Ausrichtungen ist Nord nun global optimal: {θ_F}°, E = {E_F} kWh/a. | The north circle morphs into a green square labelled "feasible optimum"; plan faces the courtyard; bar total shown. | Marker glides to north (2 s), circle → square `Transform`. | `os.feasible_minimum`, `E_F` |
| `feasible_2` | The quiet courtyard costs {price_kwh} kWh/a, about {price_chf} CHF/a, versus the free south optimum. | Der ruhige Hof kostet {price_kwh} kWh/a, etwa {price_chf} CHF/a, gegenüber dem unbeschränkten Südoptimum. | Bracket between the greyed south square and the north square with the price chip; text "constraint moved the optimum". | Bracket Create + chip FadeIn (1.5 s). | `E_F − E_G`, `· tariff` |

### Phase F: scope (layout 3e)

| Key | English | Deutsch | Visual state | Animation | Linked values |
| --- | --- | --- | --- | --- | --- |
| `scope` | State direction, domain and scope: minimize E, θ in {lo}°–{hi}°, for this design. | Nenne Richtung, Bereich, Geltung: E minimieren, θ in {lo}°–{hi}°, für diesen Entwurf. | Summary card, three rows: Direction "minimize E (kWh/a)"; Domain "{lo}° ≤ θ ≤ {hi}° (courtyard side)"; Scope "window {wwr} %, overhang {d} m, insulation {t} cm, Zurich-like climate, simplified model". | Card FadeIn row by row (3 s), rest of the screen dims to 30 %. | `os.constraint`, `os.design` |
| `scope_2` | Change the window size or the climate, and the best orientation may change. | Ändern sich Fenstergröße oder Klima, kann sich auch die beste Ausrichtung ändern. | Scope row highlighted; mini-glyphs of a smaller and a larger window beside it with a "?" on θ*. | `Indicate` scope row (2 s). | none |

Tally: 29 clauses; about 24 s of pure animation time on top of the reading holds; estimated total 3:30 (range 3:15-3:50).

## 5. Linking rule (every value shown by its meaning)

| On the plot | Shown at the same time as |
| --- | --- |
| marker θ | plan rotated to that bearing, section sun entry, readout `θ = …° · direction` |
| marker E(θ) | stacked bar heating/cooling/lighting with live kWh/a, total equals marker height |
| local/global markers | ghost probes or ghost bars with the room facing north/south |
| gap bracket | difference of the two bar tops, price chip in CHF/a |
| infeasible band | shaded south half ring of the compass, street glyph |
| S(θ) | live bar vs. ghost east reference bar and price chip |

## 6. Toolkit widgets beyond `api.md`

Needed (must):

1. `plan(trackers, point, scale, ...)` with extras: `ring_radius`, `arcs=True` (winter and summer sun arcs from the model's sun geometry), `ghost` (dashed outline room at a fixed bearing, for the east reference), `dimensions=True` (8 m / 6 m labels), `sector(lo, hi, color)` for a shaded compass sector (infeasible south half ring).
2. `site_context(plan, street_bearing, quiet_bearing)`: street glyph (road band, car, noise ripples) and courtyard glyph (trees, green patch) outside the plan ring.
3. `bearing_axes(point, x_range, y_range, width, height, x_title, y_title)`: axes with numeric y ticks and a compass tick row "E 90° · S 180° · W 270° · N 360° · E 450°" (no LaTeX), plus a right twin axis for CHF/a (`twin_axis(unit, factor)`).
4. `band_x(axes, lo, hi, color, label)`: shaded vertical band with dashed limit and a label, also hatched for infeasible.
5. `bracket(axes_or_scene, p_top, p_bottom, text)`: vertical difference bracket with a readout chip (also used between bar tops).
6. `marker(kind)` with circle (local) and square (global) glyphs and a `Transform` between them; `probes(axes, trackers, step)` for the ±5° neighbour ghosts.
7. `energy_bars(...)` option `reference=design` for a ghost bar with a savings bracket, and `compact=True` (total only) for the summary and the mini chips.
8. `price_chip(value, unit)`: coin glyph + CHF/a or kWh/a chip with `P_ORANGE` border; used for the waste, saving and constraint price.
9. `room_section(..., season="both")` showing three rays (winter noon, summer noon, low summer evening) with opacity/depth from `solar_winter`, `solar_summer` and the façade-vs-sun azimuth test; rays fade out when the façade turns away from the sun.
10. `unrolled_compass(ring, strip)`: transform from the compass ring to the horizontal bearing strip (`formulate_2`).
11. `decision_card(rows)`: three-row summary card (direction, domain, scope) built on `window_card`/`stat_card` from the energy toolkit.
12. `reflect_curve(axes, curve, level)`: reflection of a curve about a horizontal level as one `ReplacementTransform` (for `negate`).

Nice to have: `scan_line(axes)` for the full sweep in `compare`; `design_chips(design)` row for the fixed design variables; `cross_mark` marker for excluded points (exists in the energy toolkit, re-export only).

## 7. Model requests (for the model agent; placeholders above depend on them)

- `orientation_study()` additionally: `start` (315), `neighbour_step` (5), `descent` (list of `{theta, electricity}` for the greedy ±5° descent from `start` that must end at the north local minimum), `sun` geometry used by the drawings (`winter_altitude`, `summer_altitude`, `low_summer_altitude`, azimuth ranges `winter` and `summer`), and `local_minima` as a list that contains both the north and the south minimum (south = `global_minimum`), all on the same 1° grid as `samples`.
- 315° must lie in the basin of the north minimum (the west peak must be at or below ~290°), the north minimum ≥ 15 % above the south one, `E(450°) = E(90°)`.
- `evaluate` must return the per-season admitted gains and an azimuth-aware winter/summer sun fraction so the section rays use the same numbers.
- Numbers expected to be cardinal: θ_G = 180°, θ_L = 360°, `reference` 90°, `constraint` [270, 450]. If the tuned minima drift a few degrees, the narration uses the `{θ_G}`/`{θ_L}` placeholders unchanged.

## 8. Implementation notes for the later phase

- Build `NARRATION` rows from templates in the class body with a small formatter (`{name}` → `quantity`/`number`), so captions and the plot read the same model fields; keep every English and German clause as one table row (`["key", en, de]`).
- All motion through the one `θ` tracker (`set_design(..., run_time)`); descent and sweep use `ValueTracker.animate`.
- Run `T/capture.py 📉️extrema Beat1_LocalGlobalExtrema en` and `de` for every key; check bar labels with `side_labels`, readouts vs. the title tab, and the infeasible-band label vs. the plot title.
