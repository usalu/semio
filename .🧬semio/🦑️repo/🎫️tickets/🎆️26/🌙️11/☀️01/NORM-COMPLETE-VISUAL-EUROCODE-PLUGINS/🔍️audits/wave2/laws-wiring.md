# Laws wiring, wave 2

Nothing compiled or run. All edits are unverified by the compiler.

## Shared helper
`projection-kit/test-support`: new `assert_projection_laws(scene, diagrams, handles)` = `assert_scene_laws` (validate, unique ids, layers, bilingual names, `german_scene_leaks`) + `assert_diagram_laws` (validate, table twin, `german_diagram_leaks`) + every diagram has at least one axis and every axis a non-empty unit + `assert_handle_laws` + finite handle anchors + every handle writes at least one field. One copy of the composition instead of seven.

## Wired (new test `every_example_and_showcase_projection_obeys_the_scene_diagram_and_handle_laws`, appended to each `scene/tests/unit`)
Scene, diagrams (focus None) and handles (selection = all scene entities). The scene and diagram texts carry both languages, so EN presence and DE leaks are checked in one pass.
- en1993: compliant, overloaded, `frame_portal_showcase`, `annex_showcase`, `special_parts_showcase`, welded variant.
- en1994: default, unpropped, custom plate, bridge girder, fire demanding, continuous, column kinds, failing.
- en1995: `fixtures()` (building, failing, both bridges), burning, `catalogue_demo_document`, `entity_demo_document`.
- en1996: compliant, failing, rich, opening, members, basement, concentrated load, reinforced.
- en1997: `compliant_demo`, `noncompliant_demo`, `showcase_demo`.
- en1998: all 8 `fixtures()`.
- en1999: compliant, failing.
Example DSL assets are not decoded in the scene tests: the bundled examples equal these code-built snapshots (existing asset-vs-code tests in en1998, en1999, en1995 guard that).

## Pre-scan for violations
Static greps over the seven scene trees: no `writes: vec![]` handles, no axis with an empty unit, no `LocalizedCopy::new(x, x)` with real words (identical pairs are symbols or valid German such as "Position x", "Moment M", "Block"). No fix needed; the tests are the guard. Likely first failures to watch: a handle with empty `writes`, English words in a German slot found by `looks_english`.

## project_whole in en1991 and en1992
Not switched, legitimately different:
- en1991 uses `project_parts` (documented in the kit: for kits whose scene entity ids do not follow the order of their document entities, `project_whole`'s `canonical_order` would sort them differently; it assembles exactly as the live projector does). Switching would change entity order and golden output.
- en1992 `build()` runs the modules once with `Part::All` and needs the same `Out` for `handles` (taken from `out` before merge) and `diagrams`; it already uses the same two primitives `project_whole` is built from (`merge` + `canonical_order` + finish). `project_whole` takes a `Fragment` from `K::fragment` and would discard the handles, forcing a second run. Keep; a follow-up could add `project_whole_with(fragment)` to scene-kit.

## Files
Edited: `scene/tests/unit/🦀️.rs` of en1993 to en1999, test-support `🦀️.rs`; appended "- ready for rebuild: laws wiring" to `🗑️generated/build-en1993..en1999/latest.md`.
