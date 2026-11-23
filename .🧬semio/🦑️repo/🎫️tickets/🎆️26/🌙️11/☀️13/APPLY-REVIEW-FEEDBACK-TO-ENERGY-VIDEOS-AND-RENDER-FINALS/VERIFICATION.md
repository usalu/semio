# Verification: every feedback item against the final videos

Each item of `FEEDBACK.md` was checked in the rendered German finals (frames) and in the sources by four
independent audits. Their reports, with frame names, timestamps and source lines per item, are in `verify/`:
`PHYSICS.md`, `VENTILATION.md`, `HEATING.md`, `COOLING-BALANCE.md`. This file is the index and records where
I overruled or acted on an audit verdict. Timestamps are positions in the speaker final of the named video.

## Per item

| Item | Feedback (short) | Status | Where it shows |
| --- | --- | --- | --- |
| F00 | "Q in W", "[Q] = W", "Q = 120 W", never "[W]" | applied | all five; no bracket unit left in any source (grep) or frame |
| F01 | JAZ reference (Bohne) | applied | physics: "Bohne (2026), 13. Aufl." on the closing slide, JAZ via "VDI 4650 Blatt 1:2024-02" |
| F02 | Endenergie vs. Wärme | applied | physics: bar chart "Nutzwärme Q_N aus Q_E = 1 kWh Endenergie" |
| F03 | Willems 2024 unclear | applied | physics: the unclear tag is gone; closing slide lists keywords |
| F04 | "Grenzwert nach Pettenkofer" | applied | ventilation 1:02, on screen and in the caption |
| F05 | "bedarf es immer der Lüftung neben Heizung und Kühlung" | applied | ventilation 1:40 |
| F06 | Fahrplan first in every video, Q̇ for both, index V = Lüftung, "Wärmestrom" uses Φ, no Δx | applied | roadmap is the first content scene in all five; ventilation step 3 shows Φ, hand-over shows Q̇_V |
| F07 | drop "die Außenluft verblasst" | applied | ventilation: phrase absent from every source |
| F08 | ρ 1,27 | applied | ventilation 7:02: "ρ ≈ 1,2 kg/m³ (20 °C)", "ρ ≈ 1,27 kg/m³ (5 °C)" |
| F09 | "Gebäudebezogene Einheiten" / "SI-Einheiten" | applied | ventilation 7:52–7:56: both headers and "Einheiten variieren, nicht nur in der DIN" |
| F10 | no units in the middle of symbolic equations | applied | all five; numbers in substitutions carry their unit (agreed rule, `CONVENTIONS.md`) |
| F11 | Δθ notation | applied | all five; no "Delta-Theta"/"Delta-x" anywhere |
| F12 | reference for 8 % | applied | ventilation 14:02: "Maas (1995), Diss. Univ. Kassel · Kap. 8.2, S. 74" |
| F13 | openings / chimney | applied | ventilation 17:02–18:42: low "Zuluftöffnung", top "Abluftöffnung", chimney |
| F14 | Δρ(θ), v(Δp) | applied | ventilation 17:22–17:40: Δρ = ρ(θe) − ρ(θi), Δp = Δρ·g·h, v = √(2Δp/ρ) |
| F15 | sun / shading visual | applied | ventilation 19:26, 20:22 |
| F16 | supply low, extract high | applied | ventilation 22:00: "Zuluft unten", "Abluft oben" |
| F17 | enthalpy symbol, winter latent, "−" wrong | applied | ventilation: "Q̇_V,lat = 770 W", "nicht in der Heizlast", no minus sign |
| F18 | "Kühlung" → "Kühltechnik", show the sum | applied | ventilation: sums 2270 W / 1260 W; roadmap chip fixed in round 4 (0:44) |
| F19 | "der zweite Hauptsatz der Thermodynamik" | applied | heating 1:50 |
| F20 | electromagnetic wave / light | applied | heating 4:15–4:25 |
| F21 | clearer R_si/R_se box, U later | applied | heating 5:40 box, U from 8:00 |
| F22 | units and fraction bars | applied | heating, cooling, energy balance; see note 1 |
| F23 | thermometer levels | applied | heating 10:21 (40 °C / 0 °C) → 10:33 (20 / 20) |
| F24 | "— dies ist die Temperaturdifferenz Δθ" | applied | heating 13:00 |
| F25 | tidy thermal bridge, "U = 0,24 W/m²K" | applied | heating 15:30–16:15; corner overlap fixed in round 3 (16:05) |
| F26 | DIN 4108-6 symbols | applied | heating; see note 2 |
| F27 | simpler solar, g misplaced | applied | heating 22:00 (glass figure) and 24:40 (three-term worked equation, round 5); see note 3 |
| F28 | F_sh → F_C | applied | heating 23:00, 25:00, 27:20; cooling solar scenes |
| F29 | GTZ with indoor temperature | applied | heating 26:30: line "θi = 20 °C Raumtemperatur", "Heizgrenze 15 °C", caption names θi |
| goal | introduction uses real frames, no placeholders | applied | `UI-INTRO-REPORT.md`; overview cards of all five videos |

## Where I overruled an audit verdict
1. Heating F22 "partly" (27:00 units in the equation): this is a substitution with numbers, and the agreed
   rule puts the unit on every number. The symbolic line above it carries no units.
2. Heating F26 "not applied" (η formula, Q_i = 180 kWh/a, η_WRG = 0,43): the sampled frames caught counters
   while they were still running. The settled frames (18:08 and 27:42) show η_WRG = 0,8 and
   Q_i = 2000 kWh/a. "η·Q_g = η·(Q_S + Q_i)" is the DIN 4108-6 utilisation term and is correct.
3. Heating F27 "partly" (the main equation at 25:00 keeps five factors): accepted and fixed in round 5.
   The g-value panel is one glass figure with the percentages and g⊥ on the pane. The boxed DIN 4108-6
   formula stays (F26), but the worked example below it now has three terms,
   "Q̇_S = 768 W · 0,60 · 0,50 ≈ 232 W"; a small line beneath shows where they come from
   ("400 W/m² · 1,92 m² = 768 W" and "F_F · F_W · F_S · F_C ≈ 0,50"). The row of unit dashes is gone.
4. Physics "etwa 45 g/h": 30 W latent ÷ about 2430 kJ/kg ≈ 44 g/h. The value is right; nothing changed.

## Frames behind these rulings (re-checked in the current finals, kept in `verify/frames/`)
| Item | Frame | What it shows |
| --- | --- | --- |
| F22 | `heating-speaker-1620.png` (27:00) | symbolic line "Q_l = (H_T + H_V) · G_t,20/15 · 24 h/d" without loose units; the substitution below carries each unit on a fraction bar |
| F26 | `heating-speaker-1088.png` (18:08) | settled value "Q̇_V ≈ 300 W" with (1 − η_WRG) in the formula |
| F26 | `heating-speaker-1662.png` (27:42) | "Q_S ≈ 3000 kWh/a", "Q_i ≈ 2000 kWh/a", both labelled DIN V 4108-6:2003-06 |
| F26 | `heating-speaker-1740.png` (29:00) | "η·Q_g = η·(Q_S + Q_i)", "0,90 · 5000 kWh/a = 4500 kWh/a", η named after DIN V 4108-6 |
| F26 | `heating-speaker-1755.png` (29:15) | balance with "η·Q_g 4500 kWh/a" and "Q_l 16000 kWh/a" |
| F27 | `heating-speaker-1320.png` (22:00) | one glass figure: 100 %, reflected 20 %, in the glass 30 %, transmitted 50 %; "g⊥ = 0,60" sits on the pane and in its place in the formula |
| F28 | `heating-speaker-1395.png`, `-1500.png` | shading scale with F_S; boxed formula "Q̇_S = I_s·A·g⊥·F_F·F_W·F_S·F_C" with F_C, no F_sh |
| F27 | `heating-speaker-1480.png`, `heating-spectator-1480.png`, `heating-speaker-1500.png` (24:40–25:00, round 5) | worked line "Q̇_S = 768 W · 0,60 · 0,50 ≈ 232 W"; detail line with the opening product and the four factors summed to 0,50; no overlap in either audience version |
| F29 | `heating-speaker-1590.png`, `heating-spectator-1590.png` (26:30) | orange line "θi = 20 °C Raumtemperatur", dashed "Heizgrenze 15 °C" over the outdoor temperature curve; caption names θi and G_t,20/15 |
| F09 | `ventilation-spectator-476.png` (7:56) | headers "Gebäudebezogene Einheiten" and "SI-Einheiten", line "Einheiten variieren, nicht nur in der DIN." |
| F18 | `ventilation-spectator-44.png`, `-1300.png` | roadmap chip and chapter chip read "Kühltechnik"; "ungenutzt" clear of the frame edge |
| clock | `physics-spectator-808.png` (13:28) | "22:00 Uhr" with a space, audience version |

## Where I acted on an audit finding (round 4)
- Ventilation roadmap chip "→ Kühlung" → "→ Kühltechnik" (0:44), with the roadmap test fixture.
- Ventilation hand-over line reads "Lüftungsterm Q̇_V der Heizlast" (30:40).
- Ventilation "ungenutzt" label moved off the frame edge (21:40).
- Physics clock reads "22:00 Uhr" with a space (13:28).
All four are confirmed by eye in the round 4 speaker finals.

## Left as designed
- Cooling roadmap step 6 shows the annual energy Q_c without a dot; loads carry the dot.
- "Sorptionsgestützte Kühlung (DEC)" keeps its technology name.
- The energy balance closing overview shows the still empty "Parametrik" card of the course.
- φ stays relative humidity; Φ is the heat flow.
- Cross-flow ventilation at 16:15 keeps a level flow path: both windows are at the same height.
