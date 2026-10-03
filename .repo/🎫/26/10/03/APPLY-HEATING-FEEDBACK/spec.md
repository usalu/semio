# Apply Heating Review Feedback — Work Spec

Ticket: `2026/10/03/APPLY-HEATING-FEEDBACK` (folder `.repo/🎫/26/10/03/APPLY-HEATING-FEEDBACK/`).
Repo root: `/Users/niloufarghandehariyoon/Developer/semio`.

The dev's instruction: **do not rebuild from scratch — apply only the supervisor's feedback; everything the feedback does not mention stays unchanged** (same beats, same order, same visuals, same narration unless a feedback item touches it).

## Supervisor feedback (verbatim, German)

Regeln:
- Alles muss visuell sein (erklären, visualisieren, morphen etc meinen alle das gleiche)
- Ideen müssen ineinander morphen

Allgemein:
- Alle Faktoren sollen Animation bekommen von 0-1, viele fehlen noch.
- Alle Teilwerte der Gleichungen müssen Resultat von der physischen Repräsentation sein (Flächen, Faktoren, etc)
- Noch viel unwichtige und unverknüpfte Informationen anstelle von zusammenhängend und spannend
- Text übertrieben und unnötig wuchtig ("Architektur formt Raum", "und sie hört nie auf", etc)
- Architektur und Bauphysik sind nicht getrennt, sondern sollten eine Einheit bilden
- Latex muss richtig dargestellt werden (Formeln, Einheiten, etc - momentan sind subscript, superscript, brüche etc nicht richtig dargestellt)

Heizen:
- Transmission sollte animiert sein
- Erklären, warum man U Wert mit der Dicke multipliziert um "Flächenfaktor" zu erhalten, normalisiert auf 1m²
- Beziehen auf vorheriges Video: Wie viel Watt geht durch eine ungedämmte Ziegelwand durch, eine gedämmte, eine hochgedämmte, etc und im Vergleich zu Toastern, etc
- Wärmebrücken erklären: Grund: Wände werden als Papier abstrahiert, dann muss man sich entscheiden, ob außen, mitte, innen und es entstehen fehlende Flächen (entweder zu wenig oder zu viel) verrechnet. Je nach Berechnungsmethoden wird darauf geschlagen oder weg genommen (Beispiele)
- Nettovolumen sieht fälschlicherweise aus wie Bruttovolumen
- Luftwechsel sollte animiert sein (unterschiedliche Beispiel von kein Luftwechsel nötig zu Operationsälen - Beispiele die auf eine Skala morphen)
- Erklären, dass Kipplüftung schlecht ist, da wenig Luft ausgewechselt wird, aber viel Energie verloren geht.
- Sonneneinstrahlung im Winter füllt zu viel und wirft nicht den richtigen Schatten (links oben im Raum kommen keine Strahlen direkt hin)
- Visualisierung zur Erklärung vom Verschattungsfaktor fehlt (Animation von 1 bis 0)
- Visualisierung Ausnutzungsgrad der Wärmegewinne.
- Übergangsverluste im Haus (Leitungsrohre, etc) erklären

## How the general rules translate into code

1. **Typeset every formula, symbol and unit** with the LaTeX-subset typesetter in `tutorial/manim_visuals.py` — never `Text("R_si")`, `Text("Q̇_k")`, `"[m²·K/W]"`, `"Φ_V"`, `"c_Luft"` etc.
   - `math_text(src, font_size=…, color=…)` / `math_label(src, at=None, size=…, color=…, edge="center")` — supports `x_{i}`, `x^{2}`, `\frac{a}{b}`, `\dot{Q}`, `\mathrm{kWh}`, `\text{mit Leerzeichen}`, `\textcolor{#HEX}{…}`, spaces `\, \; \quad \!`, `{,}` decimal comma, and the symbols `\cdot \approx \times \to \Rightarrow \uparrow \pm \le \ge \Delta \Sigma \eta \varphi \phi \rho \lambda \theta \vartheta \mu \infty \Phi \Psi \psi \chi \ldots`. Greek capitals may also be typed directly (Φ). Variables are italic, `\mathrm{}` is upright — subscripts that are words/abbreviations must be `\mathrm{}` (e.g. `R_{\mathrm{si}}`, `c_{\mathrm{Luft}}`, `\eta_{\mathrm{WRG}}`, `\Phi_{\mathrm{V}}`, `F_{\mathrm{sh}}`), index variables stay italic (`U_{i}`).
   - `math_panel(parts, color=P_TEAL, size=None, edge_buff=FORMULA_PANEL_EDGE_BUFF)` → `(row, box, items)` — replaces `equation_row` + `formula_panel`; `parts` are `(key, latex, colour)`; `highlight_param(items, key, color=…)` still works on its items.
   - `math_readout(fn, at, size=…, color=…, edge="left")` — `always_redraw` label from `fn()` returning a LaTeX string; `at` may be a point or a callable returning a point (follow a moving mobject).
   - `de_num(value, digits)` — German number string for math (`1{,}43`, `10\,812`).
   - `place_math(mob, at, edge)` — sit a typeset box on a baseline point.
   - Units: `1{,}43\,\mathrm{W/(m^{2}\,K)}` or `\frac{\mathrm{W}}{\mathrm{m^{2}\,K}}`; area `\mathrm{m^{2}}`, volume `\mathrm{m^{3}}`, per hour `\mathrm{h^{-1}}` or `1/\mathrm{h}`.
   - `beat_subtitle(...)` is plain text: subtitles must not contain sub/superscripted symbols — reword them in plain German words (e.g. "Personenabwärme Φ_p" → "Personenabwärme").
   - German caption narration (the third tuple element) is plain text read aloud — no underscores; write symbols as the repo already does ("R-si", "Phi-V", "c-Luft", "Eta-WRG", "F-sh", "Q-Punkt").
   - Reference implementation of all of this: `tutorial/energy/demand/1_physical_fundamentals/scene_1.py` (its `_m`, `_readout`, `_panel` are now the shared `math_label`, `math_readout`, `math_panel`).
2. **Every factor animates from 0**: every quantity that is shown gets a `ValueTracker` that starts at 0 and counts up to its value while its drawing grows (`math_readout` shows the live number). Dimensionless factors (F_f, g, F_sh, η, …) sweep **0 → 1** (or 1 → 0) with a visible physical effect, then settle on their real value.
3. **Partial values come from the drawing**: an area is read from drawn dimension arrows (width × height), a volume from the drawn section × depth, a factor from counted rays / measured glass fraction / measured areas. Compute in Python from the geometry/trackers and display the result — never a hard-coded finished number that does not match the drawing.
4. **Morph ideas into each other**: when a picture becomes a symbol/number, use `ReplacementTransform` / `TransformFromCopy` / a flying copy into the formula slot, not FadeOut + FadeIn.
5. **Plain, factual text**: no ALL-CAPS labels, no exclamation marks, no dramatised narration ("ein Sieb", "Geschichte", "Reise", "Jetzt drehen wir um", "Sehr Hoch"). Keep only information that connects to the beat's quantity.
6. **Architecture + building physics as one**: every quantity sits on a building element (wall, window, room, roof) rather than an abstract block.

## Shared numbers (keep identical across modules)

One example house runs through Modul 2, 3, 5 and the final calculation. Δθ = 20 K (innen 20 °C, außen 0 °C) unless a beat explicitly varies it.

House: footprint 10 m × 8 m, eaves height 6 m, ridge 8,5 m (gable 2,5 m), two storeys.

Envelope (Modul 2 `Beat5_Gebaeudehuelle`, final `ReviewingHeatLosses`):

| Element | A [m²] | U [W/(m²K)] | U·A [W/K] | Δθ [K] | Φ [W] |
|---|---|---|---|---|---|
| Dach | 89 | 0,20 | 17,8 | 20 | 356 |
| Außenwand (opak) | 209 | 0,24 | 50,2 | 20 | 1 003 |
| Fenster | 30 | 1,10 | 33,0 | 20 | 660 |
| Haustür | 2 | 1,30 | 2,6 | 20 | 52 |
| Boden gegen Erdreich | 80 | 0,30 | 24,0 | 10 (Erdreich ≈ 10 °C) | 240 |
| **Summe** | **410** | | **H_T ≈ 115,6 W/K** | | **Φ_T ≈ 2 311 W** |

Wall build-ups (Modul 1 Beat8, Modul 2 Beat4 — computed from layers, R_si = 0,13, R_se = 0,04 m²K/W):
- Ungedämmte Ziegelwand (1960): Innenputz 1,5 cm λ 0,70 · Vollziegel 36,5 cm λ 0,75 · Außenputz 2 cm λ 0,87 → R_ges ≈ 0,70 → **U ≈ 1,43**
- Gedämmt: same + 12 cm Dämmung λ 0,035 → R_ges ≈ 4,13 → **U ≈ 0,24**
- Hochgedämmt: same + 30 cm Dämmung λ 0,035 → R_ges ≈ 9,27 → **U ≈ 0,11**
- Saniert (Modul 1 Beat8): same + 20 cm Dämmung → R_ges ≈ 6,41 → **U ≈ 0,16**

Ventilation (Modul 3, final): V_e = 580 m³ (section 72,5 m² × 8 m), net **V ≈ 440 m³** (≈ 0,76 · V_e, DIN V 18599-1 / GEG pauschal for ≤ 3 Vollgeschosse), n = 0,5 h⁻¹, c_Luft = ρ·c = 1,2 kg/m³ · 1 005 J/(kg K) = 1 206 J/(m³K) ≈ 0,34 Wh/(m³K) → **Φ_V = 440 · 0,5 · 0,34 · 20 ≈ 1 496 W**, H_V ≈ 74,8 W/K. With WRG η = 0,8 → 299 W.

Solar (Modul 5): window 1,2 m × 1,6 m → A = 1,92 m²; G = 400 W/m² (klarer Wintertag, Südfassade); F_f = A_Glas/A = 0,70; g = 0,60 (10 rays: 2 reflected, 3 absorbed of which 1 re-emitted inward, 5 transmitted → (5+1)/10); F_sh winter ≈ 0,80 (overhang, from geometry) → Φ_solar ≈ 400 · 1,92 · 0,70 · 0,60 · 0,80 ≈ 258 W per window.

Annual (final): G_t ≈ 3 500 K·d/a, Q_Verlust = (H_T + H_V) · G_t · 24 h/d ≈ 190,4 · 3 500 · 24 / 1000 ≈ 16 000 kWh/a; Q_sol ≈ 3 000 kWh/a, Q_int ≈ 2 000 kWh/a → Q_Gewinn = 5 000 kWh/a; η_h ≈ 0,90 → Q_h ≈ 16 000 − 4 500 = 11 500 kWh/a. System losses (DIN V 18599-5): Übergabe 5 % ≈ 575, Verteilung 30 m · 10 W/m · 2 900 h = 870, Speicherung 1,5 kWh/d · 200 d = 300, Erzeugung η_g = 0,95 → ≈ 697 → Endenergie Q_E ≈ 13 940 kWh/a.

Watt references from the previous video (Physikalische Grundlagen power scale): Teelicht ≈ 30 W, Mensch in Ruhe ≈ 100 W, Staubsauger ≈ 1 kW, Einfamilienhaus am kältesten Tag ≈ 8 kW. `watt_anchor(..., compare="toaster")` uses Toaster = 1 000 W.

## Repo rules (CLAUDE.md, mandatory)

- Edit the existing files only. Do **not** create files outside the ticket folder; temporary scripts/logs go to `.repo/🎫/26/10/03/APPLY-HEATING-FEEDBACK/checks/` with your module in the filename. Do not delete them afterwards.
- Do **not** edit `tutorial/manim_visuals.py`, `tutorial/manim_fonts.py`, `full_heating_video.py`, docs or `launch.json` — the orchestrator owns those. If you need a helper, add it to your own scene file inside a `#region`.
- Do **not** run any modifying git command (commit, stash, checkout, reset, …). Others are editing the repo at the same time.
- Structure new code with `#region Name` / `#endregion`; every new docstring starts with a unique fitting emoji; concise code; **no comments inside function bodies** in new code; temporary prints prefixed `[DEBUG] ` and removed when done.
- Keep the existing beat pattern: `NARRATION = [(key, english, german), …]`, `caption_bar` / `swap_caption` / `hold_for(self, self.NARRATION, key, used=…)`, `_din_ref` corner citation, `beat_subtitle`, persistent `TITLE_DE`.
- German captions must fit `caption_bar` in ≤ 2 lines (check with a small script that builds `caption_bar(de)` and counts lines, like `.repo/🎫/26/10/03/APPLY-PHYSICAL-FUNDAMENTALS-FEEDBACK/checks/caption_check.py`).

## Verification (mandatory before reporting done)

Render every beat you touched from the repo root:

```bash
LAYOUT_CHECK=1 .venv/bin/manim -ql --disable_caching --media_dir /tmp/heat_<module> <scene file> <BeatClass> 2>&1 | grep -E "\[LAYOUT\]|Error|Traceback|File \"|line [0-9]+|Exception" | grep -v SyntaxWarning
ffmpeg -v error -y -i /tmp/heat_<module>/videos/<file stem>/480p15/<BeatClass>.mp4 -vf "fps=1/2,scale=427:240,tile=4x4" /tmp/heat_<module>/sheets/<BeatClass>_%02d.png
```

Look at every contact sheet (Read the PNG) and fix overlaps, clipped labels, text in the formula/caption bands, wrong geometry. Finish with **zero `[LAYOUT]` lines** for every beat and no exceptions. Report: beats changed/added (class names, in order), what each feedback item became, the computed numbers, render results, and any remaining issue.
