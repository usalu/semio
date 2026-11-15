# Binding conventions for all five energy videos

These decisions resolve the reviewer feedback (`FEEDBACK.md`) once, so that
every agent edits consistently. They apply to everything SHOWN (formulas,
labels, tables, axis labels, captions/subtitles, quiz JSON) in physics,
ventilation, heating, cooling and demand (energy balance), English and German.

## 1. Units (F00, F10, F22)

- Never a bare bracketed unit: `[W]`, `[kWh]`, `\,[\mathrm{K}]` are forbidden.
- Allowed forms only: `Q in W` (German "Q in W", English "Q in W"),
  `[Q] = W`, or a value with unit `Q = 120 W`.
  - Axis labels and table headers: `θ in °C`, `Q in kWh`, `t in h`.
  - Symbolic formulas carry no unit; put `Φ in W` next to or under them when
    the unit matters.
- Numeric substitution: EVERY number carries its unit, not only the result:
  `75 m³/h · 0,34 Wh/(m³K) · 20 K = 510 W`. Never `75 · 0,34 · 20 = 510 W`.
  The same holds for numbers spoken/shown in captions.
- Compound units are set with a fraction bar in display math:
  `\frac{\mathrm{W}}{\mathrm{m^2\,K}}`, `\frac{\mathrm{Wh}}{\mathrm{m^3\,K}}`.
  In running text/captions the inline form `W/(m²K)` is fine. Units are
  upright (`\mathrm`), separated from the number by a thin space, German
  decimal comma in German text (`0{,}34`), point in English.
- Check that units cancel correctly on screen; fix any dimensionally wrong
  line you meet.

## 2. Symbols written as symbols (F11, F24)

- Everything shown uses the real symbol: `Δθ`, `θ`, `Φ`, `Q̇`, `V̇`, `ρ`, `λ`,
  `η`, `c_p`, `U`, `R_si`, `R_se`. Never "Delta-Theta", "Q-Punkt", "Rho",
  "c-Luft", "Theta-i" in a caption.
- Narration clauses `(key, en, de)` write every symbol, formula fragment and
  value-with-compound-unit as INLINE TeX between dollar signs, e.g.
  `"Treiber ist die Temperaturdifferenz $\Delta\theta$, innen minus außen."`,
  `"… $\dot{Q}_{\mathrm{V}}$ …"`, `"… $0{,}34\,\mathrm{Wh/(m^3K)}$ …"`.
  Use raw strings where backslashes occur. The shared caption layer renders
  `$…$` as math inside the caption bar and the shared speech layer speaks it
  through the central lexicon (`🏗️authoring/🔊️speech`, built by the speech
  agent; documented in `SPEECH-LEXICON.md` in this ticket). Use only TeX
  that the lexicon can speak: Greek letters, `\Delta`, `\dot{}`, sub/
  superscripts, `\mathrm{}`, `\frac{}{}`, `\cdot`, `=`, `+`, `-`, numbers,
  units. Do not spell symbols out to help the TTS, and do not put Unicode
  math letters (Δ, θ, Φ) outside `$…$` in narration.
- Phrase pattern wanted by the reviewer: "… die Temperaturdifferenz Δθ".

## 3. Heat flow, loads, energies (F06)

- `Φ` (upright capital Phi, `\Phi`) = Wärmestrom in W as the generic physical
  quantity (definition, conduction/convection/radiation laws,
  `Φ = U · A · Δθ`). Used WITHOUT building index.
- `Q̇` (`\dot{Q}`) in W = load/power at the room or building, with index, the
  same in heating AND cooling: `Q̇_T` transmission, `Q̇_V` ventilation,
  `Q̇_S` solar, `Q̇_I` internal, `Q̇_H` heating load, `Q̇_C` cooling load.
  The first time `Q̇_T`/`Q̇_V` appear in a video, tie them to Φ in one line
  ("der Wärmestrom Φ, als Last am Raum Q̇").
- Index `V` is reserved for ventilation (Lüftung). Never use `V` for
  "Verlust"; never use `L` for Lüftung (old `Q̇_L` → `Q̇_V`). Latent part:
  `Q̇_{V,lat}`, sensible `Q̇_{V,sens}`.
- `Q` in kWh = energy over a period, DIN V 4108-6 notation (section 5).
- `V̇` volume flow in m³/h, `n` air change rate in 1/h (h⁻¹), `V` volume in m³.
- `Δx` (humidity ratio difference) appears only where latent heat is actually
  derived, never on a roadmap/overview.

## 4. Air properties (F08)

- Checked: dry air at 1013 hPa has ρ = 1,204 kg/m³ at 20 °C, 1,27 kg/m³ at
  5 °C, 1,29 kg/m³ at 0 °C. The norm value `ρ · c_p = 0,34 Wh/(m³K)`
  (DIN V 4108-6, DIN EN 12831) corresponds to ρ ≈ 1,2 kg/m³ and
  c_p ≈ 1,0 kJ/(kgK) (1,2 · 1006 / 3600 = 0,335). Keep `ρ = 1,2 kg/m³` but
  ALWAYS show its condition: `ρ ≈ 1,2 kg/m³ (20 °C)`, and say once that colder
  air is denser (1,27 kg/m³ at 5 °C) while the norm calculates with the fixed
  0,34 Wh/(m³K).

## 5. DIN V 4108-6 notation for the balance (F26, F28)

- `Q_h = Q_l − η · Q_g`, `Q_l = Q_T + Q_V`, `Q_g = Q_S + Q_i`
  (heating demand, losses, gains, solar, internal; utilisation factor `η`).
- `H_T`, `H_V` in W/K; `Q_T = F_Gt · H_T` in the heating-period method,
  `F_Gt` in kKh/a.
- Solar gains: `Q_S = Σ I_s · F_S · F_C · F_F · F_W · g_⊥ · A` with
  `F_S` shading by surroundings/overhangs, `F_C` sun-protection device
  (NOT `F_sh`, which is ISO 52016-1 notation), `F_F` frame reduction factor
  (glazed fraction), `F_W` non-perpendicular incidence (0,9), `g_⊥` total
  solar energy transmittance at normal incidence, `A` window area (rough
  opening). The same `F_C` is used in cooling/summer (DIN 4108-2).
- `θ_i` indoor, `θ_e` outdoor temperature; `Δθ = θ_i − θ_e`.
- `R_si`, `R_se` surface resistances, `U` in W/(m²K), `λ` in W/(mK),
  `d` thickness in m, `ψ` in W/(mK) linear thermal bridge.

## 6. References (F01, F03, F12, F04)

- Every on-screen reference must be findable: author, year/EDITION actually
  current, and a locator that exists in the source (chapter title or section
  number or table number) plus the keyword under which the claim is found
  there. No bare "Bohne 2022" / "Willems et al. 2024".
- If you cannot verify the locator (no access to the book), cite a source
  you can pin precisely instead: the DIN/EN/VDI clause or table, the GEG
  paragraph, an official agency page (UBA, BAFA, BWP, dena) – with clause
  number. Do not invent page numbers. Use WebSearch to verify editions and
  clause numbers; record every reference you changed, with the evidence, in
  your report.
- Heat-pump seasonal performance: the term in the literature is
  "Jahresarbeitszahl (JAZ)" per VDI 4650 Blatt 1 and "SCOP" per DIN EN 14825;
  introduce the term in words where JAZ is first used.
- "Pettenkofer": wording is "Grenzwert nach Pettenkofer" (1000 ppm CO₂),
  source Pettenkofer 1858 / DIN EN 16798-1 categories.

## 7. Roadmap (F06)

- Every video starts with its roadmap (Fahrplan) as the very first scene;
  a dedicated agent adds these – content agents do not add roadmap scenes,
  but must keep the chapter/step names they change in sync with it by
  listing renamed chapters in their report.

## 8. Engineering rules

- AGENTS.md applies. Shared live worktree: NO mutating git commands; re-read
  a file immediately before each edit; other agents edit neighbours.
- English and German strings are both maintained (`tr(en, de)`, narration
  tuples `(key, en, de)`); fix both languages.
- Keep narration timing helpers (`hold_for`, `subtitle_text`) consistent when
  clauses are added/removed. No comments inside definitions.
- Staffage/devices only via handcrafted casts (`fitted_staffage`,
  `staffage_motion`); simple diagram glyphs (openings, arrows, sun,
  thermometer) may be drawn. The guard test
  `🏗️authoring/🎨️visuals/🧪️tests/🐍️staffage_only` must stay green.
- Verify visually: render the changed scene(s) in the PREVIEW profile into a
  PRIVATE output root `<ticket>/🗑️generated/<your-name>-out` (never the shared
  cache), extract frames with ffmpeg and LOOK at them (Read tool) until
  nothing overlaps, clips or leaves the frame.
