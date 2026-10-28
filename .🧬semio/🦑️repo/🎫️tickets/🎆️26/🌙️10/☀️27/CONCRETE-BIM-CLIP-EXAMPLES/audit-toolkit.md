# Audit: Energy Toolkit Primitives For The BIM Rebuild

Read-only Haiku audit (agent afb3b140ae07e5f0a). Line keys: ENERGY-VIS = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py` (2,590 lines), FONTS = `…/🔤️fonts/🐍️.py`, THEME = `…/🎨️theme/🐍️.py`.

## Bottom Line

- The current BIM drawing layer (`C/🎨️visuals/🐍️.py`, 305 lines) draws one oblique cuboid; plan/section/elevation are plain rectangles; FEM is a 3 × 3 dot grid; class diagram is stacked chips. The rebuild is a rewrite.
- ENERGY-VIS is the reusable core (captions, timing, chips, cards, section material, meters, math labels, layout guard). It has no plan, elevation, axonometric, column, ribbon window, curved glazing, ramp, monospace panel, UML box, mesh, support or deflection primitive.
- Only axonometric code: private `_cube_faces` in `⚡️energy/🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:136` (30° isometric: +x → (0.866, 0.5), +y → (−0.866, 0.5), +z → (0, 1)).
- Monospace: `FONT_MONO = "Share Tech Mono"` (THEME:15) is bundled and in the font catalog but never drawn; `body_text` always uses Anta. Glyph coverage for `# ; ' * $` unverified; missing glyphs fall back silently (FONTS:150–161) and would break alignment.

## Palette (tokens, `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`)

| Token | Hex | Constants |
| --- | --- | --- |
| primary | `#ff344f` | P_PRIMARY, P_RED |
| secondary | `#34d1bf` | P_SECONDARY, P_CYAN, P_TEAL, P_BLUE (no true blue) |
| tertiary | `#fa9500` | P_ORANGE |
| warning | `#fccf05` | P_YELLOW |
| success | `#7eb77f` | P_GREEN |
| info | `#dbbea1` | PASTEL_PURPLE (beige) |
| dark | `#001117` | P_DEEP_DARK (background) |
| light | `#f7f3e3` | P_WHITE |
| gray | `#7b827d` | PASTEL_GREY |
| gray-400 / gray-600 / light-gray | `#555f5d` / `#a2a59d` / `#d3d2c5` | slab / frame / reflection greys in solar-gains |

Current BIM `MUTED = P_PRIMARY` makes "muted" red — fix.

## Typography Ladder

BODY 21.6, LABEL 19.2, CAPTION 24.0, FORMULA 32.4, SUBTITLE 21.6, citation LABEL − 3 = 16.2. Current BIM uses off-ladder 19–27 and 68.

## Stage Geometry (frame 14.22 × 8, y ±4)

| Band | Range |
| --- | --- |
| content band | y 2.58 … −2.60, x ±6.80 (`SAFE_TOP`, `SAFE_BOTTOM`) |
| note band | centre y 2.30, width ≤ 11.0; content ceiling then 1.98 |
| formula panel | y −2.80 … −1.84, width ≤ 11.6 |
| caption | bottom ≈ −3.6, max 2 lines (top ≈ −2.66), width ≤ 11.5; zone −4.00 … −2.62 |
| guard limits | x ±7.05, y ±3.95 |

## Layout Guard

`layout_conflicts(scene)` (ENERGY-VIS:2460): > 2 caption lines; untagged text in caption band; text in formula band when a panel is shown; text–text overlaps (unless same label root); graphics touching caption/formula frames; opaque graphic under text; strokes through text. Zone tags `_layout_zone` = `formula`, `formula_box`, `caption`, `caption_box`. Tagged text skips band checks — current BIM abuses `"formula"` (BIM-VIS:174, 275). Only `Text` instances are checked.

## Reusable Primitives (ENERGY-VIS)

- Captions/timing: `caption_bar` (2034, raises ValueError above 2 lines), `swap_caption` (2084), `hold_for` (2203, budget = max(TTS, word estimate 2.5 wps en / 2.2 de, reading floor 1.6 wps min 1.5 s)), `set_vo_language` (2126).
- Chrome: `window_chip` (735), `chip` (744), `window_frame` (145), `window_card` (1363), `stat_card` (1382), `card_grid` (1403), `cloud_trail` (1229), `side_labels` (1433, dodged callouts with leaders), `note_line` (1340), `dim_arrow` (767), `cross_mark` (757), `fit_band` (179).
- Math: `math_label` (1976), `math_row` (1935), `math_panel` (2010), `math_readout` (1999, `always_redraw`), `highlight_param` (1626), `de_num` (1963).
- Architecture: `section_material` (934), `section_assembly` (968), `window_glyph` (987), `room_section` (1100), `house_section` (1048, gable only), `radiator` (1134), `person_glyph` (1195).
- Gauges: `meter`, `bind_meter`, `recolor_meter` (use instead of animating the fill colour), `stacked_bar`, `energy_tank`, `thermometer_glyph`.

## Idioms

- ValueTracker + `always_redraw` for rays/patches/curves; continuous sweeps inside `hold_for(..., during=lambda rt: [...])` (solar-gains:794–836).
- Live numbers via `math_readout`; optimization room module uses cached-digit `LiveText` to avoid re-rendering text each frame.
- Energy clips: 10–13 narration rows per dense beat.

## Optimization Room Patterns (copy, do not import)

`room_section` (452), `facade` (608), `plan` (652), `_hatch` (926, box only), `_dimension` (376), `dashed` (359), `strokes` (881), `column` (208, dodged labels), `LiveText`/`LiveGroup` (59/92), `reveal`/`dismiss`/`pulse` (781–791). It imports Pillow.

## Must Add

Axonometric prism projector with painter's sort; pilotis, ribbon window, curved glass wall, ramp, roof garden/screens, flat slab; polygon-clipped hatch; monospace code panel (verify glyphs); UML class box (multi-compartment `window_card`); FEM shell mesh, support glyphs; status badges; one shared ClipScene base.
