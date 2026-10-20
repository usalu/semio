# Brief — one chapter of the new Ventilation video (video 2 of 5)

Repo: /Users/niloufarghandehariyoon/Developer/semio (bun/nx monorepo; this work is pure Manim/Python under `tutorial/`).
Ticket folder (all temp files, logs, frames go here, never delete them): `.repo/🎫/26/10/20/ADD-VENTILATION-VIDEO-AS-SECOND-TUTORIAL/`
Backup of the pre-change sources: `<ticket>/backup/` (read-only reference — never edit it).

## Why this video exists
Professor feedback: *ventilation affects both heating and cooling — air change rates matter in summer AND winter — so extract all ventilation content from Heating and Cooling into one video that comes before them.* The curriculum becomes 5 videos:

1. Physical Fundamentals (unchanged) → **2. Lüftung (this one, new)** → 3. Heizung → 4. Kühlung → 5. Energiebilanz.

So this video explains ventilation ONCE, season-neutral, and everything it explains must be something Heating or Cooling will later *use* (they will link back and only contain a short bridge beat). Therefore every chapter must visibly tie its content to "what Heizung needs" (winter) and "what Kühlung needs" (summer): same drawing/formula, then both seasons' numbers side by side, and the last beat of each chapter ends with a small hand-over chip (see Style).

Audience: students of building energy. Tone: academic, calm, precise, easy to follow. German on-screen subtitles (the `NARRATION` German strings); keep on-screen labels short and sober.

## You own exactly ONE chapter file
`tutorial/energy/demand/Ventilation/<N_folder>/scene_<N>.py` (+ its `generate_audio.py` and `build_full_video.py`, copied from the EnergyBalance chapter pattern and adapted — see Pipeline files). Other agents write the other chapters concurrently — do NOT touch their files. You MAY NOT edit `tutorial/manim_visuals.py`, `tutorial/manim_fonts.py`, `tutorial/intro/intro_scene.py`, `.vscode/launch.json`, README/docs, or any file under `Heating/`, `Cooling/`, `EnergyBalance/`, `1_physical_fundamentals/` (the lead owns those; the old Heating/Cooling beats are only SOURCES you copy from). If you need a new shared helper, build it locally in your scene file under `#region Shared visual motifs` and tell the lead in your final report so it can be promoted.

## Chapter plan (22 beats). Beat class names, keys and german texts are yours to write, but keep these class names exactly (the composer imports them)

| Folder / file | Chapter title (TITLE_DE) | Beats (class names) |
|---|---|---|
| `Ventilation/1_warum_lueften/scene_1.py` | Warum lüften? | `Beat1_Lueftungsbedarf` · `Beat2_Lueftungsweg` · `Beat3_Fahrplan` |
| `Ventilation/2_luftmenge/scene_2.py` | Wie viel Luft? | `Beat1_Innenvolumen` · `Beat2_Luftwechselrate` · `Beat3_Volumenstrom` |
| `Ventilation/3_waermestrom_luft/scene_3.py` | Was die Luft trägt | `Beat1_Luftwaerme` · `Beat2_SensibleWaerme` · `Beat3_LatenteLast` · `Beat4_WinterUndSommer` |
| `Ventilation/4_natuerliche_lueftung/scene_4.py` | Natürliche Lüftung | `Beat1_Fensterregeln` · `Beat2_StossOderKipp` · `Beat3_Querlueftung` · `Beat4_Auftrieb` · `Beat5_Nachtlueftung` · `Beat6_GrenzenDerFreienLueftung` |
| `Ventilation/5_mechanische_lueftung/scene_5.py` | Mechanische Lüftung und Wärmerückgewinnung | `Beat1_Grundtypen` · `Beat2_Waermerueckgewinnung` · `Beat3_Anlagenvergleich` · `Beat4_Kanalquerschnitt` |
| `Ventilation/6_uebergabe/scene_6.py` | Übergabe an Heizung und Kühlung | `Beat1_Zusammenfassung` · `Beat2_UebergabeHeizungKuehlung` |

### Chapter content (what each beat explains and where the existing material is)
Existing scenes are the SOURCE: copy the beat's drawing code, helpers, constants and NARRATION into your chapter, then adapt (season-neutral wording, shared numbers below, glyph vocabulary). Sources: `Heating/3_convection/scene_3.py` (H3), `Cooling/3_transmission_humidity/scene_3.py` (C3), `Cooling/5_systemauslegung/scene_5.py` (C5), `Cooling/6_lueftungssysteme/scene_6.py` (C6), `1_physical_fundamentals/scene_1.py` Beat8_Lueftung (PF8; the Physical Fundamentals teaser — do not repeat it, build on it: *V̇ → Q̇_V = ρ·c·V̇·ΔT, 100 m³/h at 20 K = 680 W*).

**Ch 1 – Warum lüften?**
- `Beat1_Lueftungsbedarf` (NEW): a room with people (person_glyph at true scale). What ventilation removes, season-independent: CO₂ (outdoor ≈ 420 ppm, hygiene reference ≈ 1000 ppm "Pettenkofer value", animated 0→1 gauge via `meter`), moisture (droplets; a resting person ≈ 30 W latent, see PF7), odours/pollutants. Fresh air replaces used air → more people need more air (≈ 30 m³/h per person as a planning order of magnitude, DIN EN 16798-1 / DIN 1946-6 cited via the chip). Closing clause: this need exists in winter as much as in summer — hence this video comes before Heating and Cooling.
- `Beat2_Lueftungsweg` (from H3 Beat1_GebaeudeKonvektion): house_section, air streams through openings; winter: warm air leaves (orange), cold air enters (blue); then flip the season (sun/moon glyph) — same openings, the stream reverses: hot air enters, cooled air leaves. Heat moves with the air in both directions.
- `Beat3_Fahrplan` (NEW, short): the roadmap of the video as a visual chain of 5 cards — Luftmenge (V, n, V̇) → Wärmestrom (Δθ, c_Luft, Δx) → natürlich → mechanisch (η_WRG) → Übergabe. Cards light up in order; each card carries its symbol.

**Ch 2 – Wie viel Luft?**
- `Beat1_Innenvolumen` (H3 Beat2_Innenvolumen: V_e = 580 m³ → V ≈ 0.76·V_e ≈ 440 m³).
- `Beat2_Luftwechselrate` (H3 Beat3_Luftwechselrate: n, V̇ = V·n, scale storeroom 0 · Passivhaus ≈ 0.3 · dwelling 0.5 · office ≈ 2 · classroom ≈ 4 · kitchen ≈ 15 · operating room ≈ 20; DIN EN 12831, DIN 1946-4).
- `Beat3_Volumenstrom` (NEW short bridge of notations): *one airflow, two spellings* — V̇ = V·n = 220 m³/h = 0.061 m³/s = q_v,R (Heating writes V·n, Cooling writes q_v,R); animated unit conversion ÷ 3600; also hygiene per person vs. n-approach (4 persons × 30 m³/h = 120 m³/h < 220 m³/h → the n = 0.5 minimum covers it).

**Ch 3 – Was die Luft trägt**
- `Beat1_Luftwaerme` (H3 Beat4_SpezWaermekapazitaet: 1 m³ air, 1.2 kg, 1206 J per K → c_Luft ≈ 0.34 Wh/(m³·K); ρ·c_p/3600 = c_Luft makes the Heating and the Cooling spelling the same thing).
- `Beat2_SensibleWaerme` (H3 Beat5_Lueftungsverlust generalised): Q̇_V,sens = V̇·c_Luft·Δθ with signed Δθ. Winter: 440·0.5·0.34·20 K ≈ 1500 W lost. Summer: Δθ = 6 K (32 °C outside, 26 °C inside) ≈ 450 W gained. Each factor animates 0→1 (meters), result is read from the drawn picture.
- `Beat3_LatenteLast` (C3 Beat3_VentilationHeat + Beat4_SensibleVsLatent adapted): Q̇_V,lat = ρ·r·Δx·q_v,R, Δx = 4.5 g/kg (32 °C/50 % vs 26 °C/50 %), r ≈ 2450 kJ/kg → ≈ 810 W at q_v,R = 0.061 m³/s. Moisture accumulates in a gauge; phase change costs energy.
- `Beat4_WinterUndSommer` (NEW): one formula, two seasons side by side — stacked_bar: winter sensible 1500 W (Heizung uses this, latent not part of the heating load); summer sensible 450 W + latent 810 W (Kühlung uses both). Closing hand-over chip.

**Ch 4 – Natürliche Lüftung** (every beat shows or states both seasons; where the source narration says "Sommer/Kühlung" only, add the winter side in a clause or label)
- `Beat1_Fensterregeln` (C6 Beat2_Fensterregeln) · `Beat2_StossOderKipp` (H3 Beat7_KippOderStoss: 340 Wh vs ≈ 780 Wh) · `Beat3_Querlueftung` (C6 Beat3) · `Beat4_Auftrieb` (C6 Beat4; in winter the Δθ is bigger → stronger stack effect, drawn) · `Beat5_Nachtlueftung` (C6 Beat5) · `Beat6_GrenzenDerFreienLueftung` (C6 Beat6 + one clause for winter: gap ventilation has η = 0 → full loss, which hands over to the mechanical chapter).

**Ch 5 – Mechanische Lüftung und Wärmerückgewinnung**
- `Beat1_Grundtypen` (C6 Beat7_MechanischeGrundtypen; the supply/exhaust picture from C5 Beat1 may be reused inside it).
- `Beat2_Waermerueckgewinnung`: MERGE H3 Beat6_Waermerueckgewinnung and C6 Beat8_Waermerueckgewinnung into ONE beat with ONE definition η = (θ_ZUL − θ_AUL)/(θ_ABL − θ_AUL), plate exchanger animation (flows never mix). Winter: Q̇_V·(1 − η_WRG), 1500 W → 300 W at η = 0.8 (bars). Summer: 32 °C outside, 26 °C exhaust → 5 K of 6 K gained = cold recovery (η ≈ 0.8) → sensible 450 W → ≈ 90 W; plate exchanger recovers sensible heat only, the latent 810 W is untouched (that is why Cooling later needs dehumidification).
- `Beat3_Anlagenvergleich` (H3 Beat8_Lueftungssysteme: window η = 0 · central ≈ 90 % saving · decentral 70–90 %, DIN 1946-6) + the Passivhaus remark from C6 Beat8 (an open window recovers nothing).
- `Beat4_Kanalquerschnitt` (C5 Beat4_DuctCrossSection + Beat5_CalculateRadius condensed into one beat): size the duct from the HYGIENE airflow of this video: q_v,R = 0.061 m³/s, v_m = 2.5 m/s → A = q_v,R / v_m ≈ 0.0244 m² → r = √(A/π) ≈ 0.088 m (Ø ≈ 18 cm). Last clause: Kühlung will need several times more air for its load — same method, bigger duct.

**Ch 6 – Übergabe an Heizung und Kühlung** (the summary the professor's restructure needs: Heating and Cooling will refer to this)
- `Beat1_Zusammenfassung`: one recap board — the symbol cards V, n, V̇ (= q_v,R), c_Luft, Δθ, Δx, η_WRG each with its number from the running example and the formula chain V → V̇ → Q̇_V,sens (+ Q̇_V,lat) → ·(1−η). Cards appear as the narration names them; the formula chain forms from the cards (morph, not pop).
- `Beat2_UebergabeHeizungKuehlung`: two columns on one house_section: **Heizung (Winter)** takes Q̇_V,sens = 1500 W (→ 300 W with WRG) as the ventilation term of the Heizwärmebedarf (Φ_V); **Kühlung (Sommer)** takes sensible + latent 450 W + 810 W, the airflow for sizing, night ventilation as storage discharge, and cold recovery. Below: the series chain as chips (Physikalische Grundlagen → Lüftung → Heizung → Kühlung → Energiebilanz) with Lüftung highlighted.

## The running example (identical numbers in every chapter — a number is changed in ONE place, the file that owns it)
| Value | Number | Owner |
|---|---|---|
| gross volume V_e → net volume V | 580 m³ → ≈ 440 m³ (V ≈ 0.76·V_e) | Ch 2 |
| air change rate n (dwelling) | 0.5 h⁻¹ → V̇ = 220 m³/h = 0.061 m³/s (= q_v,R) | Ch 2 |
| c_Luft | ρ·c_p/3600 = 1.2·1005/3600 ≈ 0.34 Wh/(m³·K) | Ch 3 |
| winter Δθ | 20 K (20 °C inside, 0 °C outside) → Q̇_V,sens ≈ 1500 W | Ch 3 |
| summer Δθ | 6 K (26 °C inside, 32 °C outside) → Q̇_V,sens ≈ 450 W | Ch 3 |
| summer Δx / latent | 4.5 g/kg, r ≈ 2450 kJ/kg → Q̇_V,lat ≈ 810 W | Ch 3 |
| η_WRG | 0.8 → winter 300 W, summer sensible ≈ 90 W (latent unchanged) | Ch 5 |
| duct | v_m = 2.5 m/s, A ≈ 0.0244 m², r ≈ 0.088 m | Ch 5 |
Mirror an owned value in other chapters as a module constant with a `# source of truth: Ch N` note OUTSIDE definitions only if truly necessary (CLAUDE.md forbids comments inside definitions — put the pointer in the region docstring instead).

## Style (same as every other tutorial video — study these files first)
- Read fully before writing: your source beats; `1_physical_fundamentals/scene_1.py` (Beat8_Lueftung and the file header/helpers); `Cooling/6_lueftungssysteme/scene_6.py` (reference style and structure of a restyled chapter, `_stand()` for people on the floor); `EnergyBalance/README.md` (layout guard, primitives); `tutorial/manim_visuals.py` (API list below).
- **Reuse, don't redraw**: use the shared vocabulary from `tutorial/manim_visuals.py` — house_section, room_section, window_glyph/open_window, radiator, sun_glyph, moon_glyph, sun_rays/shine/beam_pulses (straight rays, never wavy), person_glyph/seated_person_glyph (true scale: ≈70 % of inner room height, standing ON the slab top edge), lamp_glyph, thermometer_glyph, clock_glyph, cloud_glyph, droplets, ripples, flow_animation/paced_flow/animate_flow(s)/flow_guides/smooth_path (particle air streams), meter/bind_meter/set_meter (0→1 factor gauges), stacked_bar, labeled_axes, stat_card/card_grid/note_line/side_labels, chip, cross_mark, dim_arrow, color_ramp, energy_tank. Use the drawing code of the source beat as-is where it already uses them. Introduce new geometry only when the explanation needs a shape that does not exist (e.g. plate heat exchanger, duct section) and build it locally.
- **Typesetting**: every formula/unit/subscript/fraction through `math_label`, `math_panel`, `math_readout`, `math_text`, `de_num` — never `Text("Q_h")`-style fake subscripts, never plain-string `equation_row`. Real fractions/roots, upright units via `\mathrm{}`.
- **Visual rules from the dev (apply all)**: everything is visual (explain = visualise = morph); ideas morph into each other (Transform/ReplacementTransform with meaning — a flow arrow becomes a bar segment becomes a formula term); every factor animates 0→1 (meter / ValueTracker / fill); every number in an equation is the visible result of the drawn picture (area, bar length, gauge via `math_readout`), never appears from nowhere; no decorative or unrelated visuals; short sober labels (no dramatic taglines); architecture and physics live in the same picture; nothing overlaps (labels, flows, people, subtitle bar, formula panel); flow must not run through people's bodies or labels; exterior shading slats tilt down toward the outside.
- `hold_for(self, NARRATION, key, used=..., during=fn)` keeps ripples/particles/rays moving while the subtitle is read; `swap_caption`/`caption_bar`/`subtitle_text`/`beat_subtitle` stay as in the sources. NARRATION entries are `(key, english, german)` tuples exactly like the sources; every beat also has the `_din_ref()` standards chip like the sources.
- Season colour language: Lüftung/air = the existing blue (`VENT_BLUE`-style `#60A5FA`), warm air/heat = orange/red, cold = blue/cyan; Heizung hand-over chip = `P_ORANGE`, Kühlung hand-over chip = `P_CYAN`. **Hand-over chip**: the last beat of each chapter ends with a small chip such as `chip("→ Heizung · Modul 3: Φ_V", P_ORANGE)` and/or `chip("→ Kühlung · Teil 3: Q̇_L", P_CYAN)` naming the quantity the later video uses (Ch 6 shows the full hand-over). Keep it short, never overlapping.

## CLAUDE.md rules that apply to you (verbatim intent)
- Edit/create ONLY your own files; never use modifying git commands (no commit, stash, checkout, reset …); never kill by port.
- Greenfield: no legacy support, no compatibility layers, no migration scripts — hand-craft everything.
- Code: concise; NO comments inside definitions; docstrings start with a unique fitting emoji and may link related resources; structure with `#region` / `#endregion` (subregions allowed) like the sources; do not create extra files for tests/examples; temp files, logs, frames only inside the ticket folder; temporary logs carry the prefix `[DEBUG] `.
- Do not depend directly on new external libraries (only what the sources already import, behind the existing interfaces).
- Validate, don't assume: never say a beat works unless you rendered it and checked it.

## Pipeline files (copy-and-adapt, nothing else)
Per chapter folder also create `generate_audio.py` and `build_full_video.py` by copying `EnergyBalance/<chapter>/generate_audio.py` and `build_full_video.py` (e.g. from `EnergyBalance/3_bedarf_vs_leistung/`) and adapting scene file name, beat list, titles and output names to your chapter. Do NOT run the TTS synthesis (`generate_audio.py` without flags) or the audio mux; just make sure `python generate_audio.py --help`-level import works (`.venv/bin/python -c "import ast,sys; ast.parse(open('generate_audio.py').read())"` is enough) and the beat names match. If a chapter folder also needs a `manim.cfg`, copy the one from `Cooling/6_lueftungssysteme/manim.cfg`.

## Method (non-negotiable)
1. Read the sources and references above, then write the chapter file. Keep the file under a few hundred lines per beat; share helpers inside the file through `#region Shared visual motifs`.
2. Render EVERY beat at low quality from the repo root with the layout guard on, one at a time, and fix until clean:
   `LAYOUT_CHECK=1 .venv/bin/manim -ql --media_dir <ticket>/checks/chN/media tutorial/energy/demand/Ventilation/<folder>/scene_N.py <BeatClass> 2>&1 | tee <ticket>/checks/chN/<BeatClass>.log`
   A clean render prints no `[LAYOUT]` lines. (Rendering a beat takes ≈ 30–60 s.)
3. Screenshot review is required: extract dense frames from each rendered mp4 (e.g. `ffmpeg -i X.mp4 -vf fps=1.5 <ticket>/checks/chN/frames/<Beat>_%03d.png`), LOOK at them (Read tool on the PNGs, images are shown visually), and fix anything overlapping, floating, cut off, wrongly scaled or unreadable. Repeat until clean. Check especially: people standing on the slab, labels clear of flows, nothing under the subtitle bar/formula panel, formulas typeset correctly (subscripts, fractions, units), every factor animated 0→1.
4. Verify the numbers in your narration and formulas against the table above and against physics (e.g. 440·0.5·0.34·20 ≈ 1496 W); verify NARRATION german reads naturally and academically.
5. Final report to the lead (short): files created, beats rendered and clean (list), any residual issues, helpers worth promoting to `manim_visuals.py`, numbers you had to deviate on and why.
