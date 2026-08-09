# Notes — Scenes 4 / 5 / 6 Template Migration

Ticket: `26/08/08/ORGANIZE-COOLING-TEMPLATE`  
Scope: Cooling topics 4 (solar radiation), 5 (systemauslegung), 6 (lüftungssysteme).

## Shared changes (all three)

- Palette imported from `manim_visuals` (no local hex block).
- `apply_scene_style(self)` first line of every `construct()`.
- Font sizes only from `manim_fonts` type scale.
- Persistent `TITLE_DE` via `scene_title` / `play_scene_title` on Beat1, `self.add(title)` later.
- `beat_subtitle()` under the title; German captions via `caption_bar` / `swap_caption`.
- Formulas via `equation_row` → `formula_panel` + `highlight_param` (units tagged in-row or as small labels).
- Clause-sized `NARRATION` triples drive TTS + subtitles + `hold_for` timing.
- `generate_audio.py` imports `NARRATION` through `narration_text()`.
- Content shifted up clear of formula panel (`edge_buff=1.7`) and caption bar (`edge_buff=0.35`).

## A) `4_solar_radiation`

**Classes:** `Beat1_SolarIrradiance`, `Beat2_FrameFactor`, `Beat3_ShadingFactor`, `Beat4_GlassTransmittance`, `Beat5_SolarCoolingLoad`

- Removed local `_equation_row` / `_boxed`.
- Irradiance / frame / shading / g_tot / Q̇_S,tr formulas in the fixed panel with units (`[W/m²]`, `[m²]`, `[-]`, `[W]`).
- Chart, section, and room visuals shifted up; fly-in master equation still animates mid-frame then snaps into `formula_panel`.

## B) `5_systemauslegung`

**Classes:** `Beat1_MechanicalVentilation`, `Beat2_VolumeFlowEquation`, `Beat3_IsolateAirflow`, `Beat4_DuctCrossSection`, `Beat5_CalculateRadius`

- Removed local `_equation_row`.
- Bottom `_step_label` callouts moved mid-lower (no longer compete with caption bar); pacing now mostly caption clauses.
- Isolated airflow written as a flat Text equation row (no fraction TeX): `q_v,R = Q̇_S,tr / (ρ_a · c_p,a · Δθ)` with `[m³/s]`.
- Continuity / area / radius formulas use `formula_panel` with `[m²]` / `[m]` tags.

## C) `6_lueftungssysteme` (critical)

**Classes:** `Beat1_Systemuebersicht` … `Beat7_Zusammenfassung` (7 beats)

- Removed LaTeX bootstrap (`os` / `subprocess` / `_ensure_latex_paths` / dvisvgm wiring).
- Removed `MathTex` `_equation()`; Φ temperature-ratio formula is Text `equation_row` + `formula_panel` + `highlight_param("val")`.
- Removed unshared `P_MUTED` (mapped to `P_TEAL`).
- Stack Δp and air-change `n` equations also use `equation_row` / `formula_panel` with `[Pa]` / `[1/h]`.
- Former `_pad(budget, used)` timing replaced by `hold_for` on NARRATION keys.

## Issues / follow-ups

- Visual smoke-check (`manim -ql`) not run in this pass — layout clearance should be verified on Beat4 Φ, Beat5 duct/radius, and Scene 6 Beat2/4/5 where dense graphics meet the formula+caption stack.
- Scene 6 Beat3/6 had slightly more former `_pad` phases than NARRATION keys; trailing holds reuse the last key (still valid, slightly longer linger on the final clause).
- Original Scene 6 backup kept under `.repo/🎫/26/08/08/ORGANIZE-COOLING-TEMPLATE/scene_6.py.bak`.
