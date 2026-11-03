# Visual Grips v2 (VISUAL-GRIPS wave 2 leftovers)

Nothing compiled or run (no cargo); verified by reading. "ready for rebuild: visual grips v2" appended to `build-en1991`, `build-en1998`, `build-en1999` `latest.md` and `requests.md`. en1994 was the only artifact in `freeze.txt`; none of my edits touched it.

## 1. en1998 variable rows
- New module `en1998/…/🎬️scene-structures/🏷️variables/🦀️.rs` (`marks`, `Mark::{entity, glyph, name, grip}`): a downward `LoadGlyph::Point` (kind Variable, magnitude Q_k, shaft follows the magnitude) per row at the owner's level, rows spaced 1.5 m along X around the anchor. Grip `<entity>:qk` = shared `handle_kit::load_arrow` on `…variables[id=V].qkN` (unit N; edit rules and typed kinds `change-{storey,bridge,tower}-variable-qk-n` already exist). Length scale = 25 % of the owner's permanent action per metre (stable while the row is dragged).
- Hooks: storeys at the centre of mass (`🏢️buildings::masses_and_centres`, `🕹️handles::all`), bridges at deck centre top (`🌉️ground-and-bridges::variables_anchor`, used by scene and grips for both bridge code paths), towers at the top (`🏭️vessels-and-towers`). No new file in `en1998-scene`; scene-structures is the small sibling crate.
- Test: `every_variable_action_row_is_a_load_glyph_with_a_magnitude_grip_writing_its_typed_leaf` (3 fixtures, glyph magnitude, path, base, unit, grip stands at qk / per_metre above the level).

## 2. en1999 first-drag placement rows
- The generic mechanism already existed and needed no shared edit: `handle_kit::move_handles` (grip base falls back to the standing pose), `PlacementCommit::of_writes` (insert on first drag, set afterwards, one undo step), app-surface `placement_edits` / `resolve_placed_fields` / `complete_preview` / `overlay_writes` (preview carries the full standing pose so the overlay inserts the same row). Existing shared tests: `a_drag_on_an_entity_without_a_row_inserts_…` and `move_handles_write_placement_leaves_…`.
- The gap was en1999's own gate `if let Some(placed) = d.placements…`. Replaced by `move_handles` for members (yaw grip unless cantilever/upright), cold-formed sheets and shells (no yaw; layout ignores it), with or without a row (`stored_placements` helper).
- Test: `entities_without_a_placement_row_get_move_grips_whose_first_drag_inserts_the_row_and_the_next_continues_it` (bare doc: grips exist, first write is one `Insert`, folded doc then yields a `Set` of 4 leaves, sheets/shells advertise grips).

## 3. en1991 visuals (`🎬️scene-members/🚁hazards`)
- Falling loads: mass block (side = cbrt(m / 2400 kg/m3), clamped 0.2 to 2 m) at the drop height, fall path ribbon, impact zone on the floor, bound dimensions `:drop-height` (dropHeight) and `:stopping` (stoppingDistance, drawn below the floor).
- Execution imperfections: nominal axis, exaggerated sway ribbon and sine bow ribbon (factor so the top reaches 4 % of the height, capped at 100) plus label "drawn xk"; the true-geometry force glyph, `:axis` and tolerance grip are unchanged.
- Derailment: both rails (1.435 m gauge) on the track, derailed vehicle position marker on the track axis, F_dx and F_dy glyphs at 1.8 m on the structure (design forces), bound distance dimension (distance).
- Test: `falling_loads_imperfections_and_derailments_draw_their_design_situation` (entities, block height, sway factor, force magnitudes, dimension values and bindings). Public helpers `hazards::{block_side, exaggeration}`.

## 4. Budget
No code added to `structural-scene`. Added bytes: scene-structures about 3 KB (new file), en1998-scene a few lines in four existing files, en1991 scene-members about 6 KB (237 KB of 262 KB), en1999 scene a few lines.

## Open / unverified
- Everything is uncompiled. Riskiest: `Mark::grip` (load_arrow signature), `variables_anchor` (`super::schema::bridge::supports` path), the `[..].map(|(px, py)| pose.at(..))` arrays in `derailments`, test `top()` closure in en1991.
- Goldens of en1998, en1999, en1991 scenes change; re-bless once (`SEMIO_UPDATE_GOLDEN=1`).
- Falling/imperfection/derailment still have no 2D diagram (no formula to chart); the 3D visuals replace it. No new bilingual field-meta needed (all leaves exist).
