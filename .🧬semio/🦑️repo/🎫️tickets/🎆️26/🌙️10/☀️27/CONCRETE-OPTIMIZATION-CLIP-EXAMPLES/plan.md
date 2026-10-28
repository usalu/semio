# Concrete Optimization Clip Examples

## Request

The six optimization clips (`🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip`) are abstract: a bare quartic `f(x)`, normalized 0–1 points, designs `A–E` with invented numbers, a target point for the genetic search. The owner wants every clip to carry one proper example in which **every plotted value is shown at the same time by its meaning**, rendered with the depth of the energy clips (`🎓️teaching/🏛️architecture/⚡️energy/**/🎬️clip`).

## One Design Study For All Six Clips

**Seminar room, Zurich-like climate.** One room, 8 m wide × 6 m deep × 3 m high (48 m² floor). Only one façade is glazed; the rest is adiabatic/opaque. All numbers are produced by one transparent annual teaching model (stdlib Python, schema-first JSON inputs). No measured data; every clip says "simplified annual teaching model" once.

Design variables (physical units, never normalized on screen):

| Variable | Symbol | Range | Meaning on screen |
| --- | --- | --- | --- |
| Orientation of the glazed façade | θ | 90°–450° (E → S → W → N → E) | room rotates in plan under a compass + sun arc |
| Window-to-wall ratio | WWR | 10–80 % | window opening grows in elevation and section |
| Overhang depth | d | 0–1.2 m | projecting slab, shadow line, blocked summer rays |
| Insulation thickness | t | 10–30 cm | wall layer thickness in section (Pareto, index only) |

Responses (all from `evaluate`):

| Response | Unit | Colour (energy palette) | Meaning on screen |
| --- | --- | --- | --- |
| heating electricity | kWh/a | `P_RED` | heat-pump share of the stacked bar, heat loss arrows |
| cooling electricity | kWh/a | `P_BLUE` | chiller share, summer sun patch |
| lighting electricity | kWh/a | `P_YELLOW` | lamp glyphs dimming with daylight |
| total electricity / intensity | kWh/a, kWh/(m²·a) | `P_WHITE` | bar total, plot y value |
| façade cost | CHF | `P_ORANGE` | coin/price chip |
| embodied carbon | kg CO₂e | `P_PRIMARY` | — |
| mean daylight factor | % | `P_YELLOW` | daylight depth in section |

Physics outline (model agent tunes coefficients and documents them in the JSON):

- Façade 8 × 3 = 24 m²; window area `A_w = WWR·24`, opaque `A_o = 24 − A_w`.
- `U_wall(t) = 1 / (R_si + R_se + R_masonry + t/λ)`, λ = 0.035 W/(m·K); `U_window` ≈ 0.9 W/(m²·K); fixed other losses (roof, floor, ventilation) as one `H_other` W/K.
- Heating-season and cooling-season vertical irradiance as a function of orientation (first/second harmonic): south strongest in winter, east/west strongest in summer.
- Overhang shading factor depends on depth, season (sun altitude) and orientation (effective on south, weak on east/west); saturating, e.g. `d/(d + h₀)`.
- Heating: losses × heating degree hours − utilization × (solar + internal gains), floored at 0; heat pump COP.
- Cooling: summer solar + internal gains − removable heat, floored at 0; chiller EER.
- Lighting: installed power × hours × (1 − daylight autonomy(WWR, d)).
- Cost: wall, window, insulation per cm, overhang per m depth.

## Mandatory Model Behaviour (tests must assert it)

1. **Extrema** (`WWR 60 %, d 0 m, t 16 cm`, θ over 90°–450°): global minimum near south (≈180°), a distinct local minimum near north (≈360°), maxima near east/west. The north value is ≥ 15 % worse than south. Constraint "façade must face the quiet courtyard, within ±90° of north" → feasible θ ∈ [270°, 450°]; feasible optimum is the north local minimum. Maximize view: annual savings `S(θ) = E(90°) − E(θ)` in kWh/a and CHF/a at a stated tariff; same arg-optimum.
2. **Sampling / algorithms surface** (south, `t 16 cm`, WWR × d): one interior minimum (not on a corner or edge), a curved valley. A 3 × 3 full grid at WWR {10, 45, 80} % × d {0, 0.6, 1.2} m; 9 recorded uniform draws with a visible gap and a cluster; a 9-point Latin hypercube from a recorded permutation. Each sample's evaluated electricity is known; LHS best beats random best; the dense-grid optimum is reported for comparison.
3. **Experiments**: 2² full factorial, A = WWR {30, 70} %, B = d {0, 1.0} m, response = cooling electricity (or cooling demand) kWh/a. The overhang saves much more at 70 % than at 30 % (clear interaction). Main effects, interaction effect and bilinear coefficients are exact contrasts of the four runs. Centre run (50 %, 0.5 m) differs from the bilinear prediction by ≥ 8 % (curvature).
4. **Index**: five named façade variants evaluated by the model (e.g. punched 30 %, ribbon 50 % + 0.6 m, full glass 80 % + 1.2 m, punched 30 % + 30 cm, full glass 80 % no overhang). Criteria: electricity kWh/(m²·a) min, cost CHF min, daylight factor % max, carbon kg CO₂e min. Min–max normalization with units shown, two weight sets produce different winners.
5. **Pareto**: a design grid (e.g. WWR 5 levels × t 3 levels × d 2 levels) evaluated for cost vs electricity; 5–9 front members; one concrete dominated pair; one front step with "+CHF X → −Y kWh/a".
6. **Algorithms** on the surface of 2.: numerical-gradient descent from a stated start converging in ≈ 8–12 iterates; compass/pattern search from the same start; a genetic algorithm with population 8 over ≥ 5 generations, recorded/portable pseudo-random draws, elitism, visible crossover (child takes WWR from one parent and d from the other) and mutation; best-so-far monotone.

All functions deterministic and language-neutral (recorded draws or a documented integer LCG, no host RNG). Every numeric expectation goes to `🧪️cases/🔣️.json`; Python tests check them and re-derive them with NumPy/SciPy (vectorized independent re-implementation, `minimize_scalar`/`brentq`, `scipy.stats.qmc`, `lstsq`, dominance matrix, `minimize(method="Nelder-Mead")`); AJV validates the schema and rejects invalid mutations.

## Visual Principle

Every screen pairs **the plot** with **the room**: when a marker sits at a value, the room drawing shows that exact design and its consequence (window size, overhang, rotated plan, sun rays passing or blocked, lamps dimmed, stacked electricity bar with live kWh, price chip). Moving a marker moves the room. No value appears without units. Every sample/variant point carries a façade thumbnail or is linked to one. Styling, fonts, colours, captions, chrome come from the shared energy authoring toolkit.

## File Ownership

| Owner | Files |
| --- | --- |
| model agent | `🧮️model/🐍️.py`, `🧮️model/🔣️.json`, `🧮️model/🧬️schema/🔣️.schema.json`, `🧪️cases/🔣️.json`, `🧪️tests/🐍️.py`, `🧪️tests/🟨️.cjs` |
| toolkit agent | `🎨️visuals/🐍️.py` (ClipScene base + room widgets; remove topic `present_*` bodies), `🎨️visuals/⚖️decisions/🐍️.py` (delete or reduce to shared plot helpers) |
| scene agents (one each) | `📉️extrema/🐍️.py`, `🎲️sampling/🐍️.py`, `🧪️experiments/🐍️.py`, `⚖️index/🐍️.py`, `🏆️pareto/🐍️.py`, `🧬️algorithms/🐍️.py` — each owns its full `present_<topic>` implementation inside its scene class and its bilingual `NARRATION` |
| coordinator | README, transcripts, rendering, delivery, ticket |

Scene class names, video ids, wrappers and curriculum registrations stay unchanged.
