# Python lane handoff: manim clip chrome on the frame model

## Result
One rectangle per card, chips inside on top of the content as translucent glass, a chip draws only its inner edges (abutting chips share one line), square corners, no shadow. The glass values come from the styling token projection (`levels.chipTintPercent` and the level glass), not from a hard-coded constant.

## Token projection (coordinator request)
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts` `emitPython`: now emits `STYLING_LEVELS` (the whole `levels` block: names, steps, glass alpha/blur/saturate, veil, `chipTintPercent`, `zStep`), the same object the TypeScript (`STYLING_LEVELS`) and Rust (`levels::*`) outputs carry.
- `🔤️tokens/🐍️.py` regenerated from the generator itself (`python-tokens-emit.mts` in this folder calls `renderStylingArtifacts()` and writes only the Python artifact; run `node python-tokens-emit.mts` from the repo root). `STYLING_LEVELS["chipTintPercent"] == 10`.
- Test `🎨️surface/🧪️tests/🐍️.py` asserts the projection equals the `levels` block of `🔣️.json`.

## Shared owner (energy `🏗️authoring/🎨️visuals/🐍️.py`, surface `🎬️clip/🪟️presentation/🎨️surface/🐍️.py`)
- `surface.chip_paint(content_level, parent, appearance)`: glass of the level above plus the foreground veil at `chipTintPercent`, folded into one tint and alpha exactly like Rust `Theme::chip_glass`. Schema `🎨️surface/🧬️chip.schema.json`. `surface.LEVELS` is the projection.
- visuals `chip_glass(level, appearance)` (cached), `CHROME_CHIP_FILL/OPACITY` now derived from it (the hard-coded `dark-6-7` / `0.76` are gone).
- `frame_chips(width, height, top_depth, top_chips, bottom_depth, bottom_chips, color, stroke_width, level, appearance, center)`: the glass overlay and the inner edges only (edges deduplicated, abutting chips share one divider). Add it above content that must show through a chip.
- `window_frame(...)`: `title_width/title_depth` replaced by `top_depth/top_chips/bottom_depth/bottom_chips` (scene units, spans from the left edge); new `stroke_width`, `fill_color`, `glass` (a free chip surface), `level`, `appearance`. Callers updated everywhere (fundamentals physics, cooling internal-gains x2, watt_anchor, window_card).
- Consumers moved to the owner: BIM `frame_box` and `class_box` (header is one glass chip, the rule is its inner edge), class tree chips, BIM `badge` (square; glass when not filled), objects `_chip` and `_storey_chip`, representations info table (header chip), capsule cards and table (`_light_card`, light appearance), nakagin `_chip`, roadmap number badge (square glass chip anchored top-centre, was a round opaque disc) and hand-over chips (`window_chip`).
- Squared (radius to 0): bim exchange/consistency plates and rings, casting plates and cards, trees plate, heating fundamentals panel and note ring, primary-energy inset, capsule tag, excel energy panel and tiles, excel references card, excel anatomy plate.

## Audit rows checked against the code (audit2-python.md)
- Wrong or stale: `window_geometry` is already the frame mirror (frame rect, `chip_regions`, `safe_clearances`; its `body` is the text clearance content rect, correct per decision c); `surface_paint` now has a production consumer; `roadmap shades` is a dim veil, not a chip; `window_chip`, `cloud_glyph`, `formula_panel`, `caption_bar`, `reference_chip` need no change (frame without chips, or a plain label).
- Energy/optimization `RoundedRectangle(corner_radius=0)` sites (~40): already one square rectangle with content inside and no chips, left as they are. Bars, rings, drawings and graph rectangles are content.
- Left (decision a, content): Grasshopper canvas/ribbon/viewport/param viewer/groups/widgets, Excel workbook (title bar, pill, buttons), device drawings (thermometer, battery, radiator glow). Guarded by test `test_only_depictions_of_other_programs_and_devices_keep_rounded_corners` with an allowlist in `🎨️semio-style/🔣️.cases.json` (`rounded_depictions`).
- Text content keeps the chip-row clearance (stat card, window card, watt anchor, class box, capsule cards); picture content full bleed with chips on top is available through `frame_chips` (used by the gallery); no existing clip had a picture inside a titled frame that needed it, so none was changed.

## Tests (all run with `/private/tmp/claude-501/pyrun.sh` = `.venv`-independent uv env `uv sync --offline --frozen --group test` of the teaching package, `lockf` lock, language en, profile preview)
- `🎨️visuals/🧪️tests/🎨️semio-style/🐍️.py`: 14 tests OK (was 10). New: chip glass from projection and visibly different from body (composite distance >= 8), chip inner edges vs shapely boundary difference (6 language-neutral cases `chip_edges` in `🔣️.cases.json`), one shared divider overlay, rounded-corner guard. Third-party oracles kept: skia-pathops union, Pillow colour, shapely.
- `🎨️surface/🧪️tests/🐍️.py` (new, with `🔣️.cases.json`, 7 chip cases light/dark): 5 tests OK. Oracle: Pillow `alpha_composite` of glass layer then veil layer, tolerance 1 per channel; schema validation with jsonschema.
- roadmap tests 11 (3 skipped) OK, visuals icons 8 OK, fonts heading 3 OK, bim representations paper 1 OK; BIM `🧬️objects/🧪️tests/🚪️retirement` ran > 15 min without finishing (renders BIM drawings) and was cancelled; BIM `🧪️tests/🧩️chapter-source` 21 of 23 OK, 2 errors that do not touch chrome (`Pages._reveal_then` missing in the source slice the test executes from `🧩️models`, and `COLOCATED_TEST_MODULES` undefined in the exec'd standard hook), neither file is in this lane.
- `🎨️visuals/🧪️tests/🐍️scenes.py`: 14 of 15 OK; `test_occupants_clear_open_window_swing` fails (clearance 0.0863 < 0.1). It concerns staffage occupants and window opening of the `❓️need` clip, not chrome; a HEAD-tree run could not confirm it (the HEAD staffage exporter rejects `--light` in this checkout), so treat it as pre-existing and unrelated until someone proves otherwise.

## Frames
`evidence/python/before-chrome-gallery.png` and `after-chrome-gallery.png` (one scene, `python-chrome-gallery.py` in this folder, rendered at 720p from a HEAD copy and from the working tree): chips were the colour of the body (before), are visibly lighter glass now; the BIM panel went from rounded to square with a header chip; picture frame shows top and bottom chips over the picture with one shared divider; light card header chip. `after-heating-roadmap.png`: real clip still (Beat1_Fahrplan) with the number chips and hand-over chips. No label overlaps in either.

## Open
- Real-clip before/after stills for BIM (`Beat3_SameObject` ran > 8 min without a frame, cancelled), Grasshopper capsule/casting/trees and excel were not rendered; they compile and share the owner that the gallery shows. The HEAD copy cannot render staffage scenes (exporter flag mismatch), so roadmap has an after still only.
- The Rust and TypeScript chip-glass tests can read `🎨️surface/🧪️tests/🔣️.cases.json` (`composite` columns are Pillow values for the Python formula; Rust `chip_glass` folds the same way, compare alpha and composite within 1).
- Poster images under `🖼️posters` were not regenerated (instructed).
- Ticket not closed here.
