# Notes — Center Cooling Layout (Scenes 3–6 + Scene 1)

Ticket: `26/08/08/CENTER-COOLING-LAYOUT`  
Goal: `tutorial/energy` (Repo MCP unavailable — ticket folder reused)  
Date: 2026-08-08

## Layout target

| Band | Content |
|------|---------|
| Top | `scene_title` + `beat_subtitle` |
| Middle (~y −0.2…+1.6, anchor ≈ `ORIGIN` / `UP*0.25`) | houses, rooms, diagrams, graphs |
| Bottom | `formula_panel` (`edge_buff=1.7`) then `caption_bar` (`edge_buff=0.35`) — **not moved** |

No narration / timing / animation-sequence rewrites — position/scale only.

## Position changes per file

### `1_heating_vs_cooling/scene_1.py`

| What | Before | After |
|------|--------|-------|
| `_build_cross_section_house` default `center` | `LEFT * 0.8 + DOWN * 0.55` | `UP * 0.35` (horizontal center, slight up) |

Beats call the default, so winter/summer/cooling houses sit mid-screen.

### `3_transmission_humidity/scene_3.py`

| What | Before | After |
|------|--------|-------|
| Shared `CONTENT_CENTER` | — | `UP * 0.25` |
| Beat1–3 house center `hc` | `LEFT * 0.5 + UP * 0.15` | `LEFT * 0.35 + CONTENT_CENTER` |
| Beat2 `clock_center` | `LEFT * 3.2 + UP * 2.35` | `LEFT * 3.2 + UP * 1.85` (clear of subtitle) |
| Beat4 divider | `UP*2.0` → `DOWN*0.6` | `UP*1.7` → `DOWN*0.45` |
| Beat4 column titles | `UP * 1.55` | `UP * 1.35` |

### `4_solar_radiation/scene_4.py`

| What | Before | After |
|------|--------|-------|
| Shared `CONTENT_CENTER` | — | `UP * 0.25` |
| Beat1 facade `fac_c` | `LEFT*4.3 + UP*0.55` | `LEFT*4.3 + CONTENT_CENTER + UP*0.1` |
| Beat1 axes | `RIGHT*3.7 + UP*0.35` | `RIGHT*3.7 + CONTENT_CENTER` |
| Beat2 window | `LEFT*3.4 + UP*0.35` | `LEFT*3.4 + CONTENT_CENTER` |
| Beat3 section (wall/floor/ceiling/slats/labels/scale) | floor −1.75, labels −2.15/−1.85 (formula zone) | wall `1.65…−0.95`, floor −1.45, labels −1.65/−1.55, scale `sy=0.45` |
| Beat4 pane / labels / waves | pane `1.9…−1.35`, labels high/low | pane `2.15…−0.95`; Außen/Innen `UP*1.6`; waves + merge nudged mid |
| Beat5 room + fly-in eq | `floor_y=-1.55`, `ceil_y=1.15`, eq `UP*2.05`, fly y `1.35` | `floor_y=-1.1`, `ceil_y=1.6`, eq `UP*1.55`, fly y `1.0` |

### `5_systemauslegung/scene_5.py`

| What | Before | After |
|------|--------|-------|
| Shared `CONTENT_CENTER` | — | `UP * 0.1` |
| `_build_room` default | `DOWN * 0.35` | `CONTENT_CENTER` |
| `_step_label` | `DOWN * 2.35` | `DOWN * 1.5` (above formula) |
| Beat1 room | `DOWN*0.05`, h=`3.2` | `DOWN*0.1`, h=`2.9`; RLT unit `UP*0.42` (was `0.58`) |
| Beat5 chain | `DOWN * 0.55` | `CONTENT_CENTER + DOWN * 0.15` |

### `6_lueftungssysteme/scene_6.py`

| What | Before | After |
|------|--------|-------|
| Shared `CONTENT_CENTER` | — | `ORIGIN` |
| `_step_label` | `DOWN * 1.85` | `DOWN * 1.5` |
| Beat1 taxonomy | root `UP*2.0`, branches `UP*0.8` | root `UP*1.55`, branches `UP*0.55` |
| Beat2 room `center` | `DOWN * 0.15` | `CONTENT_CENTER` |
| Beat3 rooms | `DOWN * 0.45`, h=`2.15` | `UP * 0.4`, h=`2.05`; fans/ducts retuned |
| Beat3 headers y | `2.42` | `2.2` |
| Beat3 legend | `DOWN * 2.42` | `DOWN * 1.5` |
| Beat3 verdict | `to_edge(DOWN, buff=0.26)` (caption zone) | `move_to(DOWN * 1.45)` |
| Beat4 exchanger channels | `y_sup/y_exh = 1.15 / −0.35` | `1.35 / −0.15` |
| Beat5 rooms | `UP * 0.15` | `CONTENT_CENTER` |
| Beat6 blocks / classes | `UP*1.3` / `DOWN*0.75` | `UP*1.15` / `DOWN*0.55` |
| Beat6 `eq_note` | `next_to(eq_box, DOWN)` (into caption) | `move_to(DOWN * 1.5)` |
| Beat6 room vol morph | `DOWN * 0.05` | `CONTENT_CENTER` |
| Beat7 columns `top_y` | `2.18` | `2.05` |
| Beat7 footnote | `DOWN * 2.55` | `DOWN * 1.5` |

## Smoke

`.venv/bin/manim -ql -s --media_dir .repo/🎫/26/08/08/CENTER-COOLING-LAYOUT/smoke/sN …`

| File | Beat | Still |
|------|------|-------|
| scene_1 | `Beat1_WinterGains` | `smoke/s1/images/…` |
| scene_3 | `Beat1_TransmissionOpaque` | `smoke/s3/images/…` |
| scene_4 | `Beat3_ShadingFactor` | `smoke/s4/images/…` |
| scene_5 | `Beat1_MechanicalVentilation` | `smoke/s5/images/…` |
| scene_6 | `Beat3_MechanischeGrundtypen` | `smoke/s6/images/…` |

Visual check: main diagrams sit in the middle band; formula/caption slots untouched; Scene 6 legend/verdict/footnote clear of the caption strip.

## Files touched

- `tutorial/energy/demand/Cooling/1_heating_vs_cooling/scene_1.py`
- `tutorial/energy/demand/Cooling/3_transmission_humidity/scene_3.py`
- `tutorial/energy/demand/Cooling/4_solar_radiation/scene_4.py`
- `tutorial/energy/demand/Cooling/5_systemauslegung/scene_5.py`
- `tutorial/energy/demand/Cooling/6_lueftungssysteme/scene_6.py`
- `.repo/🎫/26/08/08/CENTER-COOLING-LAYOUT/notes_center_3_6.md` (this file)
- `.repo/🎫/26/08/08/CENTER-COOLING-LAYOUT/smoke/**` (stills)
