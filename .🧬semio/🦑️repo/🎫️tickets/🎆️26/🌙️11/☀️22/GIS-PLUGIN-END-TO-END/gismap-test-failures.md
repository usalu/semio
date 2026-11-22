# gismap test failures (static diagnosis, no cargo run)

## 1 + 2. colored/figure-ground label tests (with == without)

Cause: label glyphs need the runtime assets (label face) installed in the process; without them `usvg_options_map_labels()` has no face and labels shape no glyphs.

State: already fixed. `labelled_path_counts` in `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🌍️gis-policy/🧪️tests/🔬️unit/🦀️.rs` calls `semio_framework_canvas_assets::testkit::install_repository_assets()`; the gismap package manifest has the dev-dependency with feature `testkit`; the testkit's repository root (`../../../../../..`) and the label font `🔤️MapLabelSans.ttf` resolve. Test file was last written 14:01, so the failing run predates the fix. No change made; only a rerun is needed.

## 3. sqlite_snapshot_gismap_real_declaration_public_typed_io

Failure site: `io_run(&import, file, ...)` (sqlite file to native payload). The import codec runs each stage under `receiving_scope`, which requires every allocation to be paid through the original native port. The stage control has no SQL allocation port, so `control.admit_allocation_bytes` refuses with UnsupportedOwner.

Root cause: `GisMapSnapshot::encode_sqlite_snapshot_native` (and `preflight_sqlite_snapshot_encoding`) ran a borrowed row-admission prepass via `visit_rows`, which allocates a frontier `Vec` through `RowWriter::allocate_frontier`/`push_frontier`, i.e. through the SQL control. Not a missing fixture: csv/tsv `🫴️receiving` folders only hold test fixtures; the other artifacts' encode preflights do not allocate through the control (en1990 counts without a frontier, gisterrain uses a recursive depth-limited census).

Fix (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`):
- split `visit_rows` into `head` (document, children, feature rows) and `node` (one Value row set, delegating children to a callback); `visit_rows` keeps the paid breadth-first frontier for owned projection (row ids unchanged).
- new `census`/`census_rows`: allocation-free depth-first walk (depth cap 64, the intrinsic value depth) through the same `node`, so row/column/byte accounting is identical to the borrowed writer.
- `encode_sqlite_snapshot_native` and `preflight_sqlite_snapshot_encoding` use `census_rows`.

Not compiled or run (host constraint). Rerun: `hoisted_snapshot_1::sqlite_snapshot_gismap_real_declaration_public_typed_io`, plus the other gismap sqlite snapshot tests (`sqlite_snapshot_map_explicit_native_schema_and_row_admission_precedes_work`, `..._whole_native_control_...`, `..._all_owned_variants_...`, `..._deep_retirement_...`) and `hoisted_tests_2::colored_vector_labels_increase_scene_when_enabled`, `hoisted_tests_2::figure_ground_labels_increase_scene_when_enabled`.
