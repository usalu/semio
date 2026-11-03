# Asset agent A8 report

Generator scripts: `T/🧰️a8/` (`table.mjs`, `board.mjs`, `hood.mjs`, `crates.mjs`, shared `lib.mjs`/`space.mjs`, `build.mjs`, `casts.mjs`, `cast.sh`, `raster.py`, `glyphs.py`). The scripts only emit JSON; every shape, pose and decision text is handcrafted per scale. Shapes are written in absolute metres and converted to bone frames by `space.mjs`.

## Task 1: assets

All four have manifest rows, en/de text, scales 1:50, 1:100, 1:200 authored individually and 1:500 declared unavailable (reason, limiting feature, alternatives 200/100), elevation view only. Catalog check passes for each (`--asset`), and for the categories furniture and appliances. Category objects shows one failure, `stepladder` (`climb-and-work@1.500: force residual -5.63`), which is not mine and was present independently (file unmodified in git).

| Asset | Category | Size (default pose) | Poses | Clips | Heat |
|---|---|---|---|---|---|
| `operating-table` | 🛋️furniture | 2.0 x 0.6 x 1.96 m (table 2.0 x 0.5, top 0.90 m, mobile light stand 0.62 m beside it), 248 kg | `light-off`, `light-on`, `head-down` (top tilts 12 deg about a revolute pivot) | `switch-on-warm-up`, `tilt-table` | light 120 W (housing 90, lens 30), plume emitter |
| `chalkboard` | 🪑️objects | 4.0 x 1.26 x 0.08 m, 72 kg; 2.0 m sliding front leaf (prismatic) | `leaf-left`, `leaf-middle`, `leaf-right` | `slide-leaf`, `sun-on-board` (sunlit-fraction track) | passive, takes sun |
| `kitchen-extractor-hood` | 🔌️appliances | side elevation 0.5 x 0.915 m (0.9 m true width = `depthM`), 14.5 kg, wall at x = 0 | `off`, `low`, `extracting` | `switch-on-warm-up`, `step-down-and-off` | 150 W (canopy/motor 120, lamps 30); `vent` emitter 0.14 m3/s at full speed through the duct collar |
| `storage-crate-stack` | 🪑️objects | five 600 x 400 x 300 mm crates, 1.5 m high stacked, 9.5 kg | `high` (stacked, 10 kg goods each), `half`, `low` (nested empty, 0.62 m) | `unload-stack`, `load-stack` | passive HDPE |

Design notes:
- Operating light is a mobile stand with spring arm (so the asset has one floor origin); the table height stays 0.90 m (the real 0.8 to 1.0 m range is in the notes). `low`/`high` of the crates are the real nesting and stacking states (chain of prismatic bones, upper crates drawn behind lower ones when nested).
- Wall-mounted items follow `split-ac-indoor-unit`: origin at the lower edge on the wall plane, pin plus roller supports, `ground: authored`.
- Provenance cites IEC 60601-2-46, IEC 60601-2-41, ASHRAE 170, EN 14434, IEC 60335-2-31, IEC 61591, ISO 3394, plus Incropera and ISO 10456; dimensions, masses and heat splits are tagged `authored-estimate`.

Visual check: casts (`theme: dark`) of every asset rendered through the cast CLI at 1:50, 1:100 and 1:200 (dark-line and thermal), rasterised with PyMuPDF and looked at. The table reads as table plus stand plus lamp head (only the head is warm in thermal); the hood reads as a chimney hood (canopy and lamps warm); the crates show the nested steps clearly; the chalkboard in dark-line is mostly outline (dark board on dark paper), the frame and leaf read at 1:50.

## Task 2: ventilation clips

Replaced in `🌬️ventilation/🎬️clip/📦️volume/🐍️.py` (`_use_glyph`), all with new casts in `V/🧍️staffage/🔣️.json` (`theme: dark`):
- storage room: three crate stacks `ventilation-volume-crates-low/-half/-high` (replaces the crossed boxes);
- classroom: `ventilation-volume-blackboard` (unit 0.25, replaces the rectangle; the board is kept at the old 1.0 unit width, so it is smaller relative to the pupils than a real 4 m board);
- kitchen: `ventilation-volume-hood` (mirrored, at the right wall, replaces the polygon; cook moved to x = 0.52 under it, air paths re-aimed to the hood inlet);
- operating room: `ventilation-volume-operating-table` (table plus stand with the light, replaces the two lines and the `ventilation-volume-surgical-lamp` pendant cast, which is removed from the manifest).
`window_glyph` building diagrams are untouched.

## Extra scope from the coordinator: ventilation scene test

`V/🧪️tests/🐍️scenes.py::test_occupants_clear_open_window_swing`: the last loop compared `get_right()` and `get_left()` of neighbouring occupants, but the staffage groups include the breathing plume and alternate mirrored, so bounds overlap although the bodies do not. It now derives the feet positions (group x minus the offset of the cast at x = 0) and requires consecutive feet to be farther apart than the body width (0.2 x occupant height), the same rule `_room_occupants` places by. The window-clearance assertions on the group bounds are unchanged (stricter). Both ventilation scene tests pass (`test_occupants_clear_open_window_swing`, `test_ventilation_moisture_tracks_placed_occupants`).

## Verification

- Catalog check: `--asset` passes for all four (office-laser suitable at 1:50, 1:100, 1:200); `--category furniture` and `appliances` pass; `objects` has only the foreign `stepladder` failure; the full manifest check prints `✓ manifest` (124 entries) before the per-asset run timed out in my shell (`--no-physics`).
- Vitest staffage suites (`repo run @teaching/staffage:test -- catalog schema`) did not run: the repo wrapper waited more than 15 minutes for another agent's dependency build. Not verified.
- Ventilation unit tests: `🧪️architecture` and `📦️volume/🧪️tests` pass (7 tests); scene tests `test_occupants_clear_open_window_swing` and `test_ventilation_moisture_tracks_placed_occupants` pass.
- Render: `Beat2_Luftwechselrate` rendered at l / 15 fps, de, speaker (`T/🧰️a8/render.sh`), frames extracted with PyAV (`T/🧰️a8/frames.py`) and looked at: the five use rooms show crates, blackboard, kitchen hood and operating table with light. The kitchen hood is small (unit 0.31) because the chimney must stay under the mini room's ceiling slab. Exit code 1 of manim came from its rich logger failing to print the emoji path (cp1252), the mp4 was written.
- The generated folder `T/🗑️generated/a8/` was deleted.
