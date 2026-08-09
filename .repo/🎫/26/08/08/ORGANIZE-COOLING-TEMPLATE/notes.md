# Organize Cooling Demand Animations — Summary

Ticket: `26/08/08/ORGANIZE-COOLING-TEMPLATE`  
Date: 2026-08-08  
Goal: `tutorial/energy` (Cooling demand pedagogy on the five-rule Manim template)  
Note: Repo MCP unavailable this session — ticket folder created manually.

## What was done (same treatment as Heating today)

Applied `tutorial/.agents/skills/generate-manim-tutorial` five-rule template across **all Cooling topics 1–6**:

1. Consistent styling — `apply_scene_style`, palette from `manim_visuals`, type scale from `manim_fonts`
2. Clear German topic headings — `scene_title` / `beat_subtitle`, persistent `TITLE_DE`
3. Academic formulas with units — `equation_row` → `formula_panel` + `highlight_param` (`[W]`, `[W/(m²·K)]`, `[m²]`, `[K]`, `[m³/s]`, …)
4. Embedded `NARRATION` clauses drive VO + German subtitles + `hold_for` timing
5. German `caption_bar` / `swap_caption` synced to animation; content repositioned clear of formula/caption zones (animations kept, not redesigned)

## Topics

| Topic | Canonical file | Beats |
|-------|----------------|-------|
| 1 heating vs cooling | `scene_1.py` | Beat1–3 |
| 2 internal gains | `scene_2.py` | Beat1–4 |
| 3 transmission humidity | `scene_3.py` | Beat1–4 |
| 4 solar radiation | `scene_4.py` | Beat1–5 |
| 5 systemauslegung | `scene_5.py` | Beat1–5 |
| 6 lüftungssysteme | `scene_6.py` | Beat1–7 |

Every `generate_audio.py` now imports `narration_text(cls.NARRATION)` — no duplicated prose.

Legacy left in place (not deleted): `1_heating_vs_cooling/heating_vs_cooling.py`, `2_internal_gains/merged_scenes.py`.

## Scene 6 critical fix

Removed LaTeX/dvisvgm/`MathTex`; Φ formula is Text `equation_row` / `formula_panel` with `[-]`.

## Smoke checks run

- Scene 3 `Beat1_TransmissionOpaque` (full `-ql` video) ✓ — caption + title clear
- Scene 3 `Beat4_SensibleVsLatent` (still) ✓ — formula with units
- Scene 4 `Beat5_SolarCoolingLoad` (still) ✓ — `Q̇_S,tr … [W]`
- Scene 6 `Beat4_Waermerueckgewinnung` (still) ✓ — Text Φ (no MathTex); message repositioned under exchanger tags
- Topics 1–2 beats smoke-rendered by parallel pass (see `notes_scenes_1_2.md`)

Media under `.repo/🎫/26/08/08/ORGANIZE-COOLING-TEMPLATE/smoke/`.

## Follow-ups for the next pass

- Regenerate TTS (`generate_audio.py`) so spoken audio matches the new clause-split NARRATION
- Re-mux via `build_full_video.py` for HQ + voice sync
- Eye-check dense beats (duct/radius, Scene 6 overview) in Sideview if any caption still feels tight
