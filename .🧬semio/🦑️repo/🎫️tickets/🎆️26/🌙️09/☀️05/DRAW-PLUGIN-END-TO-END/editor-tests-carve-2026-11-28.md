# Draw editor tests carve (2026-11-28)

Problem: `semio-s-artifact-draw-drawing-editor-tests` peaked at 4136 MB in its final link (ceiling 4096 MB).

## Carve

New sibling crate `semio-s-artifact-draw-drawing-editor-tests-3` at `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧁️companion-3` (same layout as `🍰️companion-2`: `🦀️.rs` root, `📦️packages/🦀️rust/Cargo.toml` with `[lib] path = "../../🦀️.rs"`). Dev-dependencies copied from the parent crate.

The unit context was one inline module plus nine law files entangled by shared helpers. It is now shared as two modules mounted by `#[path]` in both crates:

- `🔬️unit/🧬️prelude/🦀️.rs`: the former root imports, `context` (drawing fixture) and `retire_owned_fixture_view`.
- `🔬️unit/🧩️harness/🦀️.rs`: helpers formerly spread over laws (`inline_selection_app`, `settled`, `load_drawing_fixture`, `assert_*_publication*`, `admit_drawing_envelope`, `drive_drawing_load`, `canvas_scene*`, `settle_drawing_geometry`, `rendered_drawing_canvas`, `first/last_layer_id`, `selected_strokes`).

Split of tests (none deleted, only re-mounted):

| crate | roots | laws |
| --- | --- | --- |
| `editor-tests` (kept) | `🔬️unit/🦀️.rs`, `🔬️archive-load`, window ownership | gesture, canvas, utility, window; archive-load law (unchanged, `[DEBUG]` print kept); `history_edit_acceptance_law!` (now at `unit_tests::` root, so run once) |
| `editor-tests-3` (new) | `🔬️unit-2/🦀️.rs` | selection, node, command, envelope, pointer, clipboard runtime |

Test path change: the two history-edit acceptance tests moved from `unit_tests::context::` to `unit_tests::`. `verify-draw-native-current26.py` expectations (window_ownership_tests, `unit_tests::canvas_laws::...`) are unaffected, so it needs no edit.

## Registration

- drawing workspace `Cargo.toml` members
- `📦️packages/🦀️rust/📋️native/🔣️.json` (catalog) and `📋️native/🧪️tests/🔣️.json` (required package, `expectedPackages` 40 -> 41)
- `/Users/ueli/Documents/semio/📋️build-owners.json`

## Link evidence

See the final section (filled after the builds).
