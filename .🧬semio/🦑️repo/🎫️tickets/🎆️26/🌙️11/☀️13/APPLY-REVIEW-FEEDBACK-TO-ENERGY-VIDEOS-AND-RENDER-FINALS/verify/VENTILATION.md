# Verification: reviewer feedback in the final German Ventilation video

Scope: items F00, F04-F18 from `FEEDBACK.md`, checked against the sources in
`🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/` and against the rendered video
`.🧬semio/🎓️teaching/⚡️cache/videos/final/de/speaker/rendered/Full_Ventilation_speaker_de_1080p30.mp4`
(31:09 long, frames extracted at 1280x720).

Frame files: `TK/🗑️generated/fix-heat/<OUT>/t_<sec>.png`. Short names used below:
`[a]` = `verify-vent-a`, `[b]` = `verify-vent-b`, `[c]` = `verify-vent-c`, `[d]` = `verify-vent-d`,
`[e]` = `verify-vent-e`, `[f]` = `verify-vent-f`, `[g]` = `verify-vent-g`, `[h]` = `verify-vent-h`,
`[i]` = `verify-vent-i`. `verify-vent-scan/sheet_0..3.png` are 30-second contact sheets.
Source paths are relative to the clip folder `🎬️clip/` (chapter folders as in the repo).
Times are video time (m:ss).

## Verdict table

| Item | Verdict | One-line reason |
|------|---------|-----------------|
| F00 | APPLIED | No bare `[W]`/`[kWh]` bracket units in any ventilation script; units appear as `in W`, `= 340 Wh`. |
| F04 | APPLIED | "Grenzwert nach Pettenkofer" on screen at 1:02 and in narration. |
| F05 | APPLIED | "darum bedarf es immer der Lüftung neben Heizung und Kühlung" on screen at 1:40. |
| F06 | APPLIED | Roadmap is the first scene (0:00), uses Φ for the heat flow, Q̇_V for the handover, no Δx; the leftover "Kühlung" chip is counted under F18. |
| F07 | APPLIED | "verblasst" is absent from every ventilation script (source only; no frame captured). |
| F08 | APPLIED | Slide shows ρ ≈ 1,2 kg/m³ (20 °C) and ρ ≈ 1,27 kg/m³ (5 °C) at 7:02, caption "Kältere Luft ist dichter". |
| F09 | APPLIED | Headers "Gebäudebezogene Einheiten" / "SI-Einheiten" and line "Einheiten variieren, nicht nur in der DIN" at 7:52-7:56. |
| F10 | APPLIED | Numeric substitutions carry units on every number (1:40, 7:32, 17:40); symbolic equations carry no mid-line units. |
| F11 | APPLIED | Shown as "Temperaturdifferenz Δθ"; no "Delta-Theta" or "Delta-x" in any source or frame. |
| F12 | APPLIED | Frame at 14:02 shows "Maas (1995), Diss. Univ. Kassel · Kap. 8.2, S. 74" next to the 8 % figure (locator not checked against the book). |
| F13 | APPLIED | Stack-effect scene at 17:02-18:42 shows a low "Zuluftöffnung" and a top "Abluftöffnung" behind a chimney box. |
| F14 | APPLIED | Δρ = ρ(θe) − ρ(θi) with values at 17:22, Δp = Δρ·g·h at 17:40, v = √(2Δp/ρ) = 1,15 m/s around 17:30. |
| F15 | APPLIED | Sun with shading ("Verschattung" label, hatched rays on the facade) at 19:26 and 20:22. |
| F16 | APPLIED | Mechanical Grundtypen at 22:00: "Abluft oben", "Zuluft unten". Residual: Querlüftung at 16:15 still shows straight same-height flow (see Notes). |
| F17 | APPLIED | Winter latent load "Q̇_V,lat = 770 W" shown as a bar, labelled "nicht in der Heizlast"; no minus sign; latent term uses r₀ "Verdampfungsenthalpie". |
| F18 | PARTLY | "Kühltechnik", the sums (2270 W winter, 1260 W summer) and two separate panels are on screen, but the roadmap chip at 0:44 still reads "→ Kühlung: Q̇_V". |

## Per-item evidence

### F00 Einheiten, no "[W]"
- Source: a grep for `[W]`, `[K]`, `[kWh]`, `[m`, `\,[` in all ventilation `🐍️.py` files returns only Python list brackets.
- Frames: `[e] t_1060`: "Δp = Δρ · g · h = 0,032 kg/m³ · 9,81 m/s² · 2,5 m = 0,8 Pa"; `[d] t_472`: "≈ 75 W/K" and "≈ 74 W/K"; `[b] t_1850`: "Q̇_V,sens = 450 W".
- Verdict: APPLIED.

### F04 "Grenzwert nach Pettenkofer"
- Source: `❓️need/🐍️.py:223` `"Bei rund $1000\,\mathrm{ppm}$ gilt die Luft als verbraucht: der Grenzwert nach Pettenkofer."`; label `❓️need/🐍️.py:284` `"Limit after|Pettenkofer (1858)"`.
- Frame `[a] t_62`: "Grenzwert nach / Pettenkofer (1858) / 1000 ppm" in the CO₂ column; caption identical to line 223.
- Verdict: APPLIED.

### F05 "bedarf es immer der Lüftung neben Heizung und Kühlung"
- Source: `❓️need/🐍️.py:238` `"Diesen Bedarf gibt es im Winter wie im Sommer, darum bedarf es immer der Lüftung neben Heizung und Kühlung."`
- Frame `[i] t_100`: the same caption; formula `V̇_Pers = 4 · 30 m³/h = 120 m³/h` with units on each number.
- Verdict: APPLIED.

### F06 Roadmap first, consistent symbols, no Δx
- Roadmap: `🗺️roadmap/🐍️.py` (Beat1_Fahrplan). It is the first scene of the video.
- Frames: `[a] t_20` and `[h] t_44`: "Der Fahrplan des Videos", six cards: Warum lüften (CO₂), Luftmenge (V, n, V̇), Wärmestrom (Φ), natürlich (A_eff), mechanisch (η_WRG), Übergabe (Q̇_V). No Δx anywhere in the roadmap.
- Source: `❓️need/🐍️.py:419-420` "Dieser Wärmestrom $\Phi$ ist als Last am Raum $\dot{Q}_{\mathrm{V}}$, V für Lüftung."
- Verdict: APPLIED. The leftover "Kühlung" chip is counted under F18.

### F07 "die Außenluft verblasst" removed
- Source: no occurrence of "verblass" in any ventilation `.py` (grep, case-insensitive).
- No frame captured for the old position (about 2:00-3:00); verdict rests on the source.
- Verdict: APPLIED (source-verified only).

### F08 ρ = 1,27 kg/m³ addressed
- Source: `🌡️heat-flow/🐍️.py:273-274`: "Ein Kubikmeter Luft bei $20\,°\mathrm{C}$ hat eine Masse von etwa $\rho = 1{,}2\,\mathrm{kg/m^3}$. Kältere Luft ist dichter: $1{,}27\,\mathrm{kg/m^3}$ bei $5\,°\mathrm{C}$."
- Frame `[a] t_422`: "ρ ≈ 1,2 kg/m³ (20 °C)" and "ρ ≈ 1,27 kg/m³ (5 °C)" in the left column; caption "Kältere Luft ist dichter: 1,27 kg/m³ bei 5 °C."
- Verdict: APPLIED.

### F09 unit slide headers and note
- Source: `🌡️heat-flow/🐍️.py:406` "Gebäudebezogene Einheiten", `:409` "SI-Einheiten"; narration clause `variiert` in the same file: "Der Einheitengebrauch variiert, nicht nur in der DIN."
- Frames `[d] t_472` and `[f] t_476`: headers "Gebäudebezogene Einheiten" (orange) and "SI-Einheiten" (cyan). On-screen line at 7:56: "Dieselbe Physik, zwei Einheitensätze (Rundung: 0,335 ≈ 0,34). Einheiten variieren, nicht nur in der DIN."
- Verdict: APPLIED.

### F10 units in the middle of equations
- Source: `🪟️natural/🐍️.py:708` `Q = c_Luft · V · Δθ = 0,34 Wh/(m³K) · 50 m³ · 20 K = 340 Wh` (units on each number).
- Frames: `[i] t_100` "V̇_Pers = 4 · 30 m³/h = 120 m³/h"; `[a] t_842` "Q = c_Luft · V · Δθ = 0,34 Wh/m³K · 50 m³ · 20 K = 340 Wh"; `[e] t_1060` "Δp = 0,032 kg/m³ · 9,81 m/s² · 2,5 m = 0,8 Pa"; `[b] t_1080` "V̇ = C_d · A · v = 0,6 · 0,1 m² · 1,15 m/s = 0,069 m³/s ≈ 250 m³/h"; `[a] t_452` (c_Luft): "Q = 1,2 kg · 1005 J/kgK · 1,0K".
- Symbolic lines (e.g. `V̇ = V · n = 220 m³/h`, `Q̇_V,sens = V̇ · c_Luft · Δθ`, chain frames `[b] t_1692`) carry no mid-line units; units appear on the result ("in W").
- Cosmetic only: missing thin spaces ("1,2kg") in a few captions.
- Verdict: APPLIED.

### F11 Δθ instead of "Delta-Theta"
- Source: no "Delta-Theta" or "Delta-x" in any ventilation `.py`. `🌡️heat-flow/🐍️.py:522` `_name(r"\Delta\theta = …", tr("Temperature difference", "Temperaturdifferenz"))`; `🤝️handoff/🐍️.py:243` "Die Temperaturdifferenz $\Delta\theta$ treibt den fühlbaren Strom".
- Frames: `[a] t_1122` "Im Winter: Δθ = 20K statt 8K"; `[b] t_1850` "Δθ = 6K".
- Verdict: APPLIED.

### F12 reference for the 8 %
- Source: `🪟️natural/🐍️.py:593` `_din_ref(_references.cite(("maas-1995", "kap8")) + tr(": n = 0.56 … 1.38 1/h", ": n = 0,56 … 1,38 1/h"))`. The 8 % claim is at `🪟️natural/🐍️.py:555-556` "$\frac{5\,\mathrm{min}}{60\,\mathrm{min}} \approx 8\,\%$".
- Frame `[a] t_842`: header "DIN 1946-6:2019-12 / Maas (1995), Diss. Univ. Kassel · Kap. 8.2, S. 74 – Kippfenster: n = 0,56 … 1,38 1/h"; left bar labelled "8 %".
- Not verified: that Kap. 8.2, S. 74 exists in the book (no access). The reference is findable only by that locator.
- Verdict: APPLIED (reference shown; locator not checked against the book).

### F13 openings instead of window low + exhaust high
- Source: `🪟️natural/🐍️.py:1047` `"Zuluftöffnung"`, `:1068` `"Abluftöffnung"`, `:983` "Treppenhaus oder Schacht (Kamin) führt die warme Luft zur hohen Abluftöffnung".
- Frames: `[a] t_1022`, `[a] t_1042`, `[a] t_1122`: "Zuluftöffnung" low-left, "Abluftöffnung" at the top behind a dashed chimney box.
- Verdict: APPLIED.

### F14 Δρ(θ) and v(Δp) introduced
- Source: `🪟️natural/🐍️.py:988` "Outside (θe = 18 °C) air has ρ = 1.212 kg/m³ … Δρ = 0.032 kg/m³"; `:991` "Δp = Δρ · g · h = 0.8 Pa"; `:994` "v = √(2Δp/ρ) gives 1.15 m/s".
- Frames: `[a] t_1022` "Warum steigt sie? Warme Luft ist leichter: die Dichte ρ(θ) sinkt mit steigender Temperatur θ."; `[a] t_1042` "Δρ = ρ(θe) − ρ(θi) = 1,212 − 1,180 = 0,032 kg/m³"; `[e] t_1060` "Δp = Δρ · g · h = 0,032 · 9,81 · 2,5 m = 0,8 Pa". The v = √(2Δp/ρ) line is seen only in the 17:30 contact-sheet thumbnail (`verify-vent-scan/sheet_2.png`), not at full size.
- Verdict: APPLIED.

### F15 sun and shading visually present at night ventilation
- Source: `🪟️natural/🐍️.py:1293` "Tagsüber trifft die Sonne die Fassade. Außenliegende Verschattung hält die Strahlung vor dem Glas ab …"; label `:1413` `"Verschattung"`.
- Frames: `[b] t_1166`: sun, hatched shading lines on the facade and the label "Verschattung" with 27 °C in the mass; `[a] t_1222`: sun rays on the facade in the winter "Nacht geschlossen / Tag kurz stoßlüften" frame.
- Night frames `[a] t_1142` and `[a] t_1197` show a sun disc beside the chart without shading; these are night frames, not a defect.
- Verdict: APPLIED.

### F16 supply low, extract high, no short-circuit look
- Source: `⚙️mechanical/🐍️.py:257` "Die Abluftanlage saugt oben mechanisch ab — die Zuluft strömt ungefiltert unten gegenüber nach …"; `:366-367` `_zone_tag(tr("extract high", "Abluft oben"), …)`, `_zone_tag(tr("supply low", "Zuluft unten"), …)`.
- Frames: `[b] t_1320` (Drei mechanische Grundtypen): Abluftanlage with "Abluft oben" at the high fan and "Außenluft ungefiltert" at the low inlet; the other two panels are still fading in.
- Residual, not the mapped item: `[c] t_975` "Querlüftung: beide Öffnungen zählen" (16:15) still shows straight, same-height flow from A1 to A2. If the reviewer's "Kurzschluss" meant this picture, F16 is only PARTLY.
- Verdict: APPLIED for the mapped scene (mechanical Grundtypen).

### F17 latent/enthalpy symbol, winter latent loss, no wrong minus
- Source: `🌡️heat-flow/🐍️.py:766` "Auch Feuchte geht mit der Abluft verloren, rund 770 W latent, doch die Heizlast zählt per Konvention nur den sensiblen Anteil."; `:778` "die Heizung übernimmt den Winterwert als Lüftungsverlust Q̇_V"; `🤝️handoff/🐍️.py:254-255` "Q̇_V,lat = V̇·ρ·r₀·Δx: 770 W in winter, 810 W in summer".
- Frames: `[a] t_618`: "Q̇_V,lat = ρ · r · Δx · q_v,R in W" with "1,2 kg/m³ · 4,5 g/kg = 5,4 g/m³"; `[a] t_660`: "Dieselbe Formel, zwei Jahreszeiten" with the winter latent bar; `[b] t_1830`: Heizung column "Q̇_V,lat = 770 W", then "nicht in der Heizlast"; `[b] t_1850`: Kühltechnik "Q̇_V,lat = 810 W", sum "Q̇_V = 1260 W".
- Sign: no minus in any latent term in the frames above.
- Symbol: the chain now ends with r₀ "Verdampfungsenthalpie" (`🤝️handoff/🐍️.py:216`). The reviewer's screenshot is lost, so the check is by content, not by pixel.
- Caveat: the winter latent is labelled "nicht in der Heizlast" (a convention) rather than as a loss; the loss is narrated at 11:00 ("geht mit der Abluft verloren").
- Verdict: APPLIED.

### F18 "Kühltechnik" instead of "Kühlung", sum shown and referred to
- Source: `🤝️handoff/🐍️.py:184` `(tr("Cooling", "Kühltechnik"), COL_COOL)`; `:441` "Die Kühltechnik übernimmt beide Anteile, ihre Summe Q̇_V = 1260 W, als Lüftungslast"; `:291` `head_s` "Sommer · Kühltechnik".
- Frames: `[b] t_1655` "Winter · Heizung" / "Sommer · Kühltechnik" columns; `[b] t_1692` chain with "Winter 2270 W" and "Sommer 1260 W" sum line; `[b] t_1850`: "→ Kühltechnik: Q̇_V", "Q̇_V = 1260 W", "Nachtlüftung → Speicher mit WRG: Q̇_V,sens = 90 W".
- Residual: `🗺️roadmap/🐍️.py:66` `\to\;\text{Kühlung:}\;\dot{Q}_{\mathrm{V}}`; frame `[h] t_44` shows the chip "→ Kühlung: Q̇_V" in the roadmap, while its narration says "Kühltechnik" (`🗺️roadmap/🐍️.py:51`). The feedback asked for the replacement everywhere.
- Column layout: heating and cooling are two separate panels in `[b] t_1830`, so the "…spalte" concern is addressed.
- Verdict: PARTLY (one "Kühlung" remains in the roadmap chip at 0:44).

## Notes and layout defects (held frames only)

- `[b] t_1830` and `[b] t_1850`: "Lüftungsterm Q̇_V der Heizlast": "Lüftungsterm" runs into the Q̇_V symbol with no space (overlap-like). Visible in both frames.
- `[g] t_1290` ("Grenzen der freien Lüftung"): the label "ungenutzt" touches the left frame edge (text starts at about x = 3 px of 1280); the line "Außenluft zu heiß, schwül oder kalt" runs close to the right edge. Nothing is cut off.
- `[c] t_975` Querlüftung (16:15): straight same-height flow between A1 and A2; possible remaining "Kurzschluss" look (see F16).
- Not in the feedback, observed only: `[a] t_842` shows the tilted-window bar as "69 Wh" (about 20 % of the 340 Wh purge) next to "8 %" air exchange; the two figures are not explained on screen.
- Frames during fades and animations (e.g. blank card frames in the Grundtypen and chain scenes) were not judged.

## Files

- Ticket: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS/`
- Frames: `TK/🗑️generated/fix-heat/verify-vent-{scan,a,b,c,d,e,f,g,h,i}/`
- This report: `TK/verify/VENTILATION.md`
- No source file was edited, no git command was run, nothing was rendered.
