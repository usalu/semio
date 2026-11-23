# Verification: reviewer feedback in the final German "Heating demand" video

Read-only check. No source file was edited, nothing was rendered, no git command was run.

- Video: `.🧬semio/🎓️teaching/⚡️cache/videos/final/de/speaker/rendered/Full_Heating_Demand_speaker_de_1080p30.mp4`
  (duration 31:09.8, 1920x1080, 30 fps; export manifest dated 2026-11-14 12:44).
- Scene order in the export manifest (job names): Fahrplan, DreiWegeDerWaerme, MakroUndMikro, RWert, UWertUndGradient,
  WattVergleich, Gebaeudehuelle, Waermebruecken, Lueftung im Winter, InterneGewinne (Personen/Geraete/Licht, Summe),
  Verschattung/Sonnenschutz, Gesamtenergiedurchlassgrad, Thermische Masse, Hauptgleichung, Gesamtverlust, Jahresenergie,
  Ausnutzungsgrad, Anlagenverluste.
- Source base: `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/` (abbrev. below: `🧲️fundamentals`, `🧱️conduction`,
  `🌬️convection`, `👥️internal-gains`, `☀️solar-gains`, `🧮️calculation`, `🗺️roadmap`; all `/🐍️.py`).
- Frames: `🗑️generated/fix-heat/verify-heat-scan/t_<sec>.png` (0 to 1860 s), `verify-heat-b/t_<sec>.png`,
  `verify-heat-c/t_<sec>.png`, `verify-heat-d/t_<sec>.png`; contact sheets `sA.png`, `sB.png`, `sC.png`, `tA.png`, `tB.png`,
  `tD.png` in the same folders. Times are given as m:ss.

## Verdict table

| Item | Verdict | One-line reason |
|---|---|---|
| F00 bare "[W]" | APPLIED | No bracketed unit left in heating sources or frames; only the allowed "[Q] = W" form. |
| F06(1) roadmap first | APPLIED | "Der Fahrplan des Videos" is the first content scene at 0:30. |
| F11 "Delta-Theta" | APPLIED | No spelled-out Delta in heating narration or captions; Δθ is shown everywhere. |
| F19 zweiter Hauptsatz | APPLIED | Caption and chip read "der zweite Hauptsatz der Thermodynamik" at 1:50. |
| F20 elektromagnetische Welle und Licht | APPLIED | Spectrum panel "Licht und Wärmestrahlung sind dieselbe Wellenart" at 4:15 to 4:25. |
| F21 R_si/R_se box, U later | APPLIED | Box at 5:40 reads "R_si innen, R_se außen: Konvektion und Strahlung in einem Wert"; U is introduced only from 8:00. |
| F22 units in equations, fraction bars | PARTLY | Symbolic formulas are clean and fraction bars are used, but the 27:00 substitution still has units mid-equation, and the 29:00 formula is wrong. |
| F23 thermometers differ | APPLIED | Thermometers start at 40 °C vs 0 °C (10:21) and converge to 20/20 at 10:33. |
| F24 "— dies ist die Temperaturdifferenz Δθ" | APPLIED | Caption at 13:00 ends "dies ist die Temperaturdifferenz Δθ." |
| F25 thermal bridge tidied, U = 0,24 | APPLIED | 15:30 shows "U = 0,24 W/(m²K)" explicitly; the bridge is split into three clean steps (15:30, 15:45, 16:15). |
| F26 DIN 4108-6 symbols | NOT APPLIED for the η formula, PARTLY overall | η formula "η·Q_g = η·(Q_S+Q_i)" at 29:00 is wrong; "Q_l" reads like "Q_I"; energy/power mixing and a value contradiction at 27:30. |
| F27 solar simpler, g placed | PARTLY | g⊥ is now placed correctly on the glass figure (22:00), but the master equation with five factors still follows (25:00). |
| F28 F_C instead of F_sh | APPLIED | F_C is used at 23:00, 25:00 and 27:20; no F_sh anywhere in heating sources. |
| F29 GTZ with indoor temperature | APPLIED | At 26:30 the graph has a visible orange line "θi = 20 °C Raumtemperatur" and "Heizgrenze 15 °C", and the caption names θi. |

## Item evidence

### F00 Einheiten, "[W]" (global)
- Source: grep over the heating clip for `[W]`, `[kWh]`, `[\mathrm` returns nothing.
- Frames: `verify-heat-scan/t_1500.png` (25:00) shows "[Q̇_S] = W" and "[Q_Speicher] = J", which is the allowed `[Q] = W` form.
  `verify-heat-scan/t_1080.png` (18:00) shows "Q̇_V = V · n · c_Luft · Δθ · (1 − η_WRG)" with no unit in the middle.
- Verdict APPLIED.

### F06(1) Fahrplan first, Q̇ and V
- Source: `🗺️roadmap/🐍️.py:54` `HEADING = ("The roadmap of this video", "Der Fahrplan des Videos")`.
- Frame `verify-heat-scan/t_30.png` (0:30): "Der Fahrplan des Videos" with chapters 1 to 6, "Φ, Δθ", "U, λ, ψ", "← Lüftung: Q̇_V".
- Frame `verify-heat-scan/t_0.png` (0:00) is a blurred course-overview card ("Architektur · Gebäudesysteme"), not a content scene.
  The first content scene is the roadmap, so the item is met.
- Q̇_V is used only for ventilation (18:00, 26:00). Φ is the generic heat flow (9:15 "Φ = U·A·Δθ", 13:00).
- Verdict APPLIED.

### F11 "Delta-Theta" spelled out
- Source: grep for "Delta-", "Delta " (not part of `\Delta`) in the heating clip returns no narration hits.
  Narration uses `$\Delta\theta$`, e.g. `🧱️conduction/🐍️.py:686` "dies ist die Temperaturdifferenz $\Delta\theta$."
- Frames: `t_90.png` ("Δθ = 20K"), `t_780.png` (caption "Temperaturdifferenz Δθ").
- Verdict APPLIED.

### F19 "der zweite Hauptsatz der Thermodynamik"
- Source: `🧲️fundamentals/🐍️.py:471` "Von allein nie umgekehrt: der zweite Hauptsatz der Thermodynamik — darum muss ein Gebäude beheizt werden."
  Chip at line 506: "Der zweite Hauptsatz der Thermodynamik".
- Frames: `verify-heat-b/t_110.png` (1:50) shows the chip under the stage and the caption; `t_114.png` (1:54) still shows the chip.
  The section header "2. Hauptsatz der Thermodynamik" is top-right through section 1.1.
- Verdict APPLIED.

### F20 Bezug zu elektromagnetischer Welle und Licht
- Source: `🧲️fundamentals/🐍️.py:869-871` axis "(elektromagnetische Wellen)", heading "Licht und Wärmestrahlung sind dieselbe Wellenart", "sichtbares Licht", "Infrarot = Wärmestrahlung"; narration line 902 "Wärmestrahlung ist eine elektromagnetische Welle wie das Licht — nur mit größerer Wellenlänge: Infrarot."
- Frames: `verify-heat-b/t_255.png` (4:15) shows the spectrum panel with UV, sichtbares Licht, Infrarot = Wärmestrahlung, λ in µm.
  `t_265.png` (4:25) shows "Sonne (5800 K): kurzwellig" and "Raumflächen (20 °C): langwellig" on the same panel.
- Layout note: in `t_265.png` the panel covers the left part of the room sketch (wall and person). It is an overlay card, not an overlap defect, but it hides the stage.
- Verdict APPLIED.

### F21 R_si/R_se box clear, U introduced later
- Source: `🧲️fundamentals/🐍️.py:1083` box text "R_si innen, R_se außen: Konvektion und Strahlung in einem Wert" (the old "fasst … zusammen" wording is gone).
  U heading `🧲️fundamentals/🐍️.py:1503` "U-Wert — wie leicht Wärme hindurchgeht"; narration line 1481 "Zuerst der Gesamtwiderstand R_T: die Luftschichten R_si und R_se plus alle Schichten d/λ dazwischen."
- Frames: `verify-heat-b/t_340.png` (5:40) shows the box "R_si innen, R_se außen: Konvektion und Strahlung in einem Wert" with no U symbol.
  `verify-heat-scan/t_480.png` (8:00) shows the R_T box and caption; the U heading "U-Wert — wie leicht Wärme hindurchgeht" appears only there.
  `verify-heat-c/tA.png` (8:20 to 8:45) introduces U = 1/R_T with "U: Wärmestrom je m² und je Kelvin Temperaturunterschied".
- Verdict APPLIED. Note: the U heading sits in the same beat as the R_T box at 8:00, although the box itself contains no U.

### F22 Einheiten nicht in der Mitte, Bruchstrich
- Good: the symbolic formulas use fraction bars and no mid-equation units, e.g. `t_1080.png` (18:00), `t_1500.png` (25:00) "[Q̇_S] = W", and `t_1740.png` (29:00) "0,90 · 5000 kWh/a = 4500 kWh/a".
- Still mid-equation: `verify-heat-scan/t_1620.png` (27:00) "= (115 W/K + 75 W/K) · 3500 Kd/a · 24 h/d"; `verify-heat-b/t_555.png` (9:15) "Φ = U · A · Δθ = 143 W/m²K · 72m² · 20K = 2059W" (no space before W).
  Source for the 27:00 chain: `🧮️calculation/🐍️.py:442` (scene "Vom Wärmestrom zur Jahresenergie"). The substitution follows the "every number carries its unit" convention, but the reviewer asked to remove units from the middle, so this is at most partly met.
- Formula error: see F26 (29:00).
- Verdict PARTLY.

### F23 Thermometerstände visuell unterschiedlich
- Source: `🧱️conduction/🐍️.py:363-420` (Beat1_MakroUndMikro) `ValueTracker(40.0)` and `ValueTracker(0.0)`, thermometer glyphs with level, and labels "θ in °C" and "Zwei Ziegel im selben Mauerwerk".
- Frames: `verify-heat-d/tD.png` (sheet of 618, 621, 624, 627, 633, 638):
  - 621 (10:21): θ1 = 40 °C (red, full) vs θ2 = 0 °C (teal, empty).
  - 624 (10:24): 35 °C vs 5 °C.
  - 627 (10:27): 23 °C vs 17 °C.
  - 633 and 638 (10:33, 10:38): "Gleichgewicht: θ = 20 °C", both at 20.
- Verdict APPLIED.

### F24 "— dies ist die Temperaturdifferenz Δθ"
- Source: `🧱️conduction/🐍️.py:686` "Durch die Wand fällt die Temperatur von innen nach außen — dies ist die Temperaturdifferenz $\Delta\theta$."
- Frame `verify-heat-scan/t_780.png` (13:00): the same caption is on screen, with the Δθ bracket on the gradient plot "Δθ 25K".
- Verdict APPLIED.

### F25 Wärmebrücke aufgeräumt, U = 0,24 W/(m²K) explizit
- Source: `🧱️conduction/🐍️.py:1209` "hier $U = 0{,}24\,\mathrm{W/(m^{2}K)}$"; `:1215` "Mit Außenmaßen zählt das Eckquadrat doppelt …"; `:1224` "Balkonplatte mit ψ = 0,5 W/(mK) auf l = 8 m ergibt 4 W/K".
- Frames:
  - `verify-heat-b/t_930.png` (15:30): single L-shaped corner, "d = 0,40 m", "außen / Mitte / innen", "U = 0,24 W/(m²K)" explicit.
  - `verify-heat-b/t_945.png` (15:45): Außenmaß 400 cm, U·l·h, 5,76 vs tatsächlich 5,28 W/K; ψ_e = −0,48 W/K / 6 m = −0,080 W/(mK). Clean; the faded labels are mid-animation.
  - `verify-heat-b/t_975.png` (16:15): balcony plan view, "ψ · l = 0,5 W/mK · 8,0 m = 4,0 W/K".
- Verdict APPLIED. The narration sum in line 1227 ("rund 2,1 W/K") is not on screen; not checked further.

### F26 Symbole auf DIN V 4108-6 abstimmen
- Good: heating symbols follow the DIN notation: H_T, H_V, F_Gt, G_t,20/15, η, Q_g, Q_S, Q_i, F_F, F_W, F_S, F_C (calc and solar-gains chapters).
- Error 1, η formula: `🧮️calculation/🐍️.py:822-823` "η · Q_g = η · (Q_S + Q_i)". Screen `verify-heat-scan/t_1740.png` (29:00) shows the same "η · Q_g = η (Q_S + Q_i)".
  DIN V 4108-6 defines Q_g = Q_S + Q_i and Q_h = Q_l − η · Q_g; the video's own line `🧮️calculation/🐍️.py:1065` uses the correct Q_h = Q_l − η · Q_g. The η formula is wrong.
- Error 2, WRG value and loss: `verify-heat-scan/t_1080.png` (18:00) shows "η_WRG = 0,43", "−640 W", "Q̇_V ≈ 860 W". Caption `🌬️convection/🐍️.py:160` says "η_WRG = 0,8 … bleiben nur 300 W". Screen and caption contradict each other.
- Error 3, power vs energy and a value mismatch: `verify-heat-b/t_1650.png` (27:30) shows "Q_i ≈ 180 kWh/a" while the caption `🧮️calculation/🐍️.py:659` says "rund 2000 kWh/a". Also `t_1640.png` (27:20) shows "Q_S ≈ 3000 kWh/a", matching its caption (`:656`, "rund 3000 kWh/a").
- Glyph: `verify-heat-scan/t_1620.png` (27:00) renders "Q_l" in a way that reads as "Q_I"; the source uses `Q_{\mathrm{l}}` (calc line 431). Readability issue.
- Verdict NOT APPLIED for the η formula and the value mismatch; PARTLY overall.

### F27 Solar detail reduced, g⊥ placed
- Source: `☀️solar-gains/🐍️.py:635` "Der Gesamtenergiedurchlassgrad g⊥ ist dimensionslos und kommt als weiterer Faktor in die Gleichung."; `:878` "Nur ein Restanteil kommt durch — der Sonnenschutzfaktor F_C sinkt auf etwa 0,15."
- Frames:
  - `verify-heat-scan/t_1320.png` (22:00): g⊥ = 0,60 is shown on the glass with 100 %, reflektiert 20 %, im Glas 30 %, durchgelassen 50 % and "50 % + 10 % = 60 % erreichen den Raum". g⊥ sits in the boxed formula. This placement is correct.
  - `verify-heat-scan/t_1380.png` (23:00): F_C scale, shown as "F_C = 0,14" while the caption says "etwa 0,15" (minor mismatch).
  - `verify-heat-scan/t_1500.png` (25:00): the full master equation with five factors (F_F, F_W, F_S, F_C plus g⊥) and a worked example "768 W · 0,60 · 0,70 · 0,90 · 0,80 · 1,00 ≈ 232 W". This is still the detailed version the reviewer asked to simplify.
- Verdict PARTLY.

### F28 F_C statt F_sh
- Grep for `F_sh`, `Fsh`, `F_{\mathrm{sh}}` in the heating clip returns nothing.
- Frames: F_C at 23:00 (`t_1380.png`), 25:00 (`t_1500.png`), 27:20 (`verify-heat-b/t_1640.png`). Source: `☀️solar-gains/🐍️.py:878`, `:1094`.
- Verdict APPLIED.

### F29 GTZ mit Innenraumtemperatur
- Source: `🧮️calculation/🐍️.py:425-426` "Die Gradtagzahl braucht eine Innentemperatur: Raumtemperatur θi = 20 °C und Heizgrenze 15 °C — zusammen G_t,20/15."
- Frame `verify-heat-b/t_1590.png` (26:30): orange horizontal line labelled "θi = 20 °C Raumtemperatur" and dashed "Heizgrenze 15 °C", with the caption on screen.
- Verdict APPLIED.

## Other observations (not among the requested items)
- Frame `verify-heat-b/t_1200.png` (20:00): duplicated "Q̇_i,l" labels and a faded formula are mid-animation; ignored.
- Frame `verify-heat-b/t_630.png`: mid-animation of the thermometers; ignored.
- References on screen (not verified against the editions): "DIN V 4108-6:2003-06 (zurückgezogen)" (27:00), "DIN 4108-2:2026-05" (10:18), "DIN/TS 18599-2:2025-10" (15:00), "VDI 3807 Blatt 1:2013-06" (27:00).
- Video is 31:09 (not 35 min); the reviewer's timestamps are not used here, the frame times are the video times.
- Frames at other times were sampled only every 30 to 60 s outside the scenes listed above; no claim is made for scenes not inspected.
