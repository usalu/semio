# Compile review H (wave 2): first artifact builds en1994, en1990, en1991

No cargo/rustc/builds. Facts: `:build` compiles lib targets only; for en1990/en1991/en1994 only the model crate ever reached rustc, so every other artifact crate is uncompiled. The shared contract built green at integrate/4 (start 00:32), but v5/v6/v7 shared code (and entity-editing, 02:03-02:12) was edited after and is uncompiled.

## Fixed (certain)
| file | class | fix |
|---|---|---|
| shared `app-surface/🫧️live-lanes/🎛️editor-wiring/🦀️.rs` | E0255: imported `projection_kit::Placement` and defined its own `pub struct Placement` (kills every artifact build) | import `Placement as PlacedRow`, used by `standing_of` |
| shared `app-surface/🧾️panels` + `🎛️live-editor/🦀️.rs` | source gate: panels 4081 active lines (> 4000) refuses admission before rustc | `🧰️entity-editing` mount moved to live-editor (panels 3610/23 files, live-editor 2538/16) |
| en1994 `📍️insert-placement/↩️inverse`, `📍️remove-placement/↩️inverse` | E0560/E0063/E0425 (stale `index` vs id-keyed `RemovePlacement`) | key by `id` |
| en1990 + en1991 `🚪️io/🦀️.rs` | E0432 dangling `pub use mutation_bridge::*;` | re-export from the artifact mutations crate |
| en1990 + en1991 io text/binary diff files, diff crate root + Cargo.toml, io-text/io-binary roots | E0117 orphan `diff_text!`/`diff_binary!` | mounted in the diff crate (`text`, `binary`), `crate::En19xxDiff`, io re-exports as `diff`, diff crate gains `semio-framework-diagnostic` |
| lane 94A (details in `compile-review-h-94A.md`) | E0063 `CompositeColumn::default_example`, E0432 io root, E0117 diff_text/binary, sqlite admission slots, edit-rules arm, test imports | fixed in en1994 |

## Static gates added (rerun before each build, about 4 min)
`🧰️scripts/compile-gate-h/gate.sh [en1994|en1990|en1991|shared|all]`: cargo features and workspace keys vs target manifests (`featck`), import-vs-local-definition clashes (`collide`, the E0255 class), `row_patch!` sets vs model struct fields/types (`patchck`), `Enum::Variant` existence (`enumck`), methods/fields that exist nowhere (`methck`). Also run this session (clean for the three artifacts and shared): trait impl completeness (263/299/439 impls), orphan impls, UNITS symbols vs `dsl(unit)`, ambiguous float literal methods, non-const calls in const/static, `Hatch`/`TrackKind`/`ShortcutAction`/`LiveCommand` exhaustiveness (`exh.py`), cargo features. Source gate: `🧰️scripts/source-boundary-scan.py`.

## Open
- `semio-s-artifact-norm-en1994-checks` is 4089 lines (> 4000): admission refuses it; routed to crate-budget (requests.md).
- 94A open items (other artifacts' same io/diff-test fixes, `AnnexChoice` serde under `test-support`, en1994 testkit placement cases).
- Shared v7 (entity-editing, `shortcut_route`, command macros, editors) landed 02:03-02:12; I read entity-editing, editor-wiring, view-steps, check-parameters, shell-hooks, published-nodes, screen-reader, node-contract, lane-runtime fully: no further certain errors. Unread: visual-windows viewport3d (mesh/hatch ribbons), properties, checks, canvas-lowering, compliance/scene deltas.
- Lanes launched late (90I) or not launchable (agent limit reached): en1994 inferences/scene/editor/viewer, en1990 model/scene/editor, en1991 all, mutation leaves were NOT read by a human-level pass; only the gates above and rsck apply.

## Limits
Shared and artifact trees were edited continuously during the review; files saved after about 02:25 were not seen. No expression-level type check beyond the files named above.
