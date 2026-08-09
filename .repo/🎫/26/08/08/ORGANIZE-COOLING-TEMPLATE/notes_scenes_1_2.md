# Organize Cooling Template — Scenes 1 & 2 Migration Notes

Ticket: `26/08/08/ORGANIZE-COOLING-TEMPLATE`  
Date: 2026-08-08  
Scope: Cooling topics **1** and **2** only (3–6 untouched).

## Goal association

Work supports `tutorial/energy` (Cooling demand pedagogy animations on the five-rule Manim template). Repo MCP was unavailable this session; ticket folder used manually.

## Template rules applied

1. `apply_scene_style(self)` first line of every `construct()`
2. Palette imported from `manim_visuals` (no hex copy-paste)
3. All `font_size=` from `manim_fonts` constants
4. Persistent `TITLE_DE` via `scene_title` / `play_scene_title` / later `self.add(title)`
5. Formulas via `equation_row` → `formula_panel` + `highlight_param` (no MathTex, no char slicing)
6. Units on academic formulas (`[W]`)
7. Every `Beat*` has `NARRATION: list[tuple[str,str,str]]`
8. German subtitles via `caption_bar` / `swap_caption`; timing via `hold_for`
9. `Write()` only for title; `FadeIn` for other `Text`
10. Reposition for formula/caption clearance; animations kept, not redesigned

## A) `Cooling/1_heating_vs_cooling/`

### Beat classes (`scene_1.py`)

| Class | Section | Notes |
|-------|---------|-------|
| `Beat1_WinterGains` | Winter helpful gains | Title animate; solar + internal gains + watt anchors |
| `Beat2_SummerOverheat` | Summer greenhouse trap | `self.add(title)`; red sun, heat fill, thermometer → 35 °C |
| `Beat3_CoolingSystem` | Mechanical cooling | Exhaust convection streams; cool-down → 21 °C |

`TITLE_DE = "Heizlast vs. Kühllast"`

### Files touched / created

- **Created** `scene_1.py` — canonical Beat* scenes (from `heating_vs_cooling.py`)
- **Created** `generate_audio.py` — imports `NARRATION` via `narration_text`
- **Created** `build_full_video.py` — Beat1–3 scene list
- **Created** `manim.cfg` — Sideview low-quality stub
- **Left in place** `heating_vs_cooling.py` — legacy monolithic `HeatingVsCooling` (not deleted)

### Smoke

- `manim -ql` `Beat1_WinterGains` ✓
- `manim -ql` `Beat2_SummerOverheat` ✓

## B) `Cooling/2_internal_gains/`

### Beat classes (`scene_2.py`)

| Class | Source content | Formulas |
|-------|----------------|----------|
| `Beat1_OfficeRoom` | `Scene1` facade → insulated office | none |
| `Beat2_HumanFactor` | `Scene2` modes + `Scene3` hall scale | `Q̇_Pers = n · q̇_p  [W]` |
| `Beat3_DevicesLighting` | `EquipmentAndPlugLoads` + `ArtificialLightingScene` | `Q̇_Geräte = Σ P_el · f_N  [W]`; `Q̇_Licht = Σ P_Licht · f_g  [W]` |
| `Beat4_CumulativeLoad` | `InternalGainEquation` cards + sum | `Q̇_i = Q̇_Pers + Q̇_Geräte + Q̇_Licht  [W]` |

`TITLE_DE = "Interne Wärmegewinne"`

Extra legacy scenes still only in `merged_scenes.py` (not in pipeline):  
`SensibleVsLatentHeat`, `Scene8`, `Scene9`, `Scene10`, `FullHiddenHeatVideo`.

### Files touched / created

- **Created** `scene_2.py` — canonical Beat* file
- **Updated** `generate_audio.py` — imports Beat* `NARRATION` (no duplicated prose)
- **Unchanged list** `build_full_video.py` — already listed Beat1–4 class names / `scene_2.py`
- **Left in place** `merged_scenes.py` — legacy Scene* monolith
- **Unchanged** `manim.cfg`

### Smoke

- `manim -ql` `Beat3_DevicesLighting` ✓
- `manim -ql` `Beat4_CumulativeLoad` ✓

Render media for verification lives under  
`.repo/🎫/26/08/08/ORGANIZE-COOLING-TEMPLATE/media_s{1,2}/`.

## Not done (out of assignment)

- Topics 3, 4, 5, 6
- TTS generation / full HQ compose (pipeline stubs ready)
- Deleting or rewriting `heating_vs_cooling.py` / `merged_scenes.py`
