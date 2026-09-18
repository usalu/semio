# 🕵️ Audit — `s.wfc.grid3d` (3d-grid artifact) vs developer brief and plan §3/§7

Scope: `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d` (crate `semio-s-artifact-wfc-grid3d`), audited read-only
against `📓️plan.md` §1/§3/§7 and the slice's own `📓️grid3d.md`. Every row below is checked against the
code, not just against the slice report — the slice report is treated as a claim to verify, not a
source of truth.

## Verdict table

| # | item | verdict | evidence |
|---|---|---|---|
| 1.1 | Document model: width/height/depth + non-uniform `cellSizesX/Y/Z` | PASS | `🧬️schema/📸️snapshot/🦀️.rs:250-283` — `width/height/depth: u32`, `cell_sizes_x/y/z: Vec<f64>`; `axis_offset` (`:388-390`) is a genuine cumulative sum (`sizes.iter().take(index).sum()`), `axis_size` (`:394-396`) defaults to `1.0` for a short array. |
| 1.2 | Periodicity per axis | PASS | `📸️snapshot/🦀️.rs:262-267` `periodic_x/y/z: bool`; inference maps to `Boundary::Wrap`/`Open` per axis (`💡️inferences/🦀️.rs:158-164`). |
| 1.3 | Tiles with `TileMedia3d` = inline Mesh \| MeshChild `s.stdio.semio@v1/mesh` | PASS | `📸️snapshot/🦀️.rs:52-55` `Grid3dTileMedia::{Mesh{mesh}, MeshChild{child: store::ArtifactChild<SemioMeshSnapshot>}}`; `🦀️.rs:52-55` (crate root) `mesh_child_handle` mints the `s.stdio.semio@v1/mesh` dialect handle. Unhydrated child renders as unit-box placeholder, not a failure (`🌐️scene-internals/🦀️.rs:97-99`). |
| 1.4 | Six-direction rules | PASS | `Grid3dDirection ∈ {LEFT,RIGHT,FRONT,BACK,BOTTOM,TOP}` (`📸️snapshot/🦀️.rs:104-118`), `Grid3dRule{id,tileAId,tileBId,direction,allowed}` (`:205-211`). Closed allow-list semantics (deny always wins) implemented in `💡️inferences/🦀️.rs:300-316` and documented at `📸️snapshot/🦀️.rs:199-202`. |
| 1.5 | Pinned / masked | PASS | `Grid3dPinnedCell{x,y,z,tileId}`, `Grid3dCell{x,y,z}` (`📸️snapshot/🦀️.rs:217-234`), both `#[dsl(table)]` sorted collections. |
| 1.6 | Deviation: commit row is `{x,y,z,tileId}` record, not `[x,y,z,tileId]` 4-tuple | PASS (justified) | `Grid3dAssignment` struct (`💡️inferences/🦀️.rs:76-83`); the slice's stated reason (`semio_framework_value_derive` has no `ToValue`/`FromValue` for 4-tuples) is a real, narrow wire-representability constraint, not a scope cut — **nice-to-have to note, not blocking**. |
| 1.7 | Deviation: commit carries `satisfiable: bool` | PASS (justified) | `Grid3dInferenceCommit{satisfiable,assignments}` (`:90-95`); the engine publishes `wfc-unsatisfiable` as a job **fault** detail (`WFC_UNSATISFIABLE` const, intercepted at `💡️inferences/🦀️.rs:518-524`), so encoding "no consistent assignment" as an answer rather than an inference failure is the correct fix, matching the framework's own "answer, not exception" idiom used elsewhere (job bug avoided, §3 below). |
| 2.1 | Both editor windows exist as World3d, row 50/50 | PASS | `✏️editor/🎭️modes/✏️edit/🦀️.rs:18-20` `layout()` = `create_default_layout([grid, preview], "row", [50.0,50.0], ["Grid","Preview"])`. Both window kinds declare `surface_kind: SurfaceKind::World3d` (`🧱️grid/🦀️.rs:72`, `👁️preview/🦀️.rs:36`). |
| 2.2 | `wfc-grid3d-grid`: one box instance per cell at CUMULATIVE non-uniform offsets | PASS | `grid_instances_json` (`🌐️scene-internals/🦀️.rs:146-168`) loops `z,y,x` and places each instance at `cell_origin(...)` = `[axis_offset(x),axis_offset(y),axis_offset(z)]`, scaled to `cell_extent`. Offset math verified correct (cumulative sum, not index×constant). |
| 2.3 | Pinned/masked cells visually distinguished | PASS | `grid_meshes_json` builds one cage mesh per pinned tile's tint + one masked cage (`:133-141`); `grid_instances_json` selects mesh id by `(pinned, masked)` match (`:154-158`). |
| 2.4 | Pick → pin/mask via utilities | PASS | `pickCell` handler reads `grid3d_active_utility` (window-instance-first, `active_utility_by_window_id`, then focused window, then flat id — `✏️editor/🦀️.rs:73-83`) and dispatches `pin_cell`/`mask_cell`/no-op (`✏️editor/🦀️.rs:180-190`). |
| 2.5 | `wfc-grid3d-preview`: `meshes_json` one entry per tile, inline/hydrated/placeholder | PASS | `preview_meshes_json` (`🌐️scene-internals/🦀️.rs:171-176`) maps `snapshot.tiles` 1:1 to mesh entries via `tile_mesh`, which handles inline mesh, degenerate inline (falls back to unit box), and `MeshChild` placeholder (`:85-99`). |
| 2.6 | `instances_json` one record per SOLVED cell scaled to cell box | PASS | `preview_instances_json` (`:180-184`) maps `assignments` 1:1 (filtered to in-grid) to instance records at `cell_origin`/`cell_extent`. Test `every_example_renders_one_instance_per_solved_cell` (`👁️preview/🧪️tests/🔬️unit/🦀️.rs:21-31`) asserts `instances.matches("\"meshId\"").count() == cells` for **both** bundled examples. |
| 2.7 | `instances_delta_json` present | PASS | `preview_instances_delta_json` (`:188-202`), gated by `delta_is_worth_publishing` (< half the set changed, residency `revision > 1`) — verified by test at `preview/🧪️tests/🔬️unit/🦀️.rs:34-41`. |
| 2.8 | Viewer covers the same ground | PASS | `👁️viewer/…/👁️preview/🦀️.rs` renders the identical `scene_internals` functions inline (no app transient available to a viewer); imports **nothing** from `✏️editor` (`grep` confirms only `crate::schema::*` and framework imports) — matches the `policyViewerPurityBreaches` rule the slice claims. |
| 3.1 | Tiled model + `Stencil3d::Face6` | PASS | `declare_stencil_relations_3d_tiled(&mut builder, &engine::grid3d::Stencil3d::Face6)` (`💡️inferences/🦀️.rs:285`). |
| 3.2 | `Grid3dTopology` (mask, periodic boundaries) | PASS | `Grid3dTopology::new(w,h,d,&Stencil3d::Face6, relations, boundary(periodic_x/y/z), mask)` (`:344-357`). |
| 3.3 | Resumable `WfcJob<Grid3dTopology>` | PASS | `engine::job::WfcJob::new(...)` / `WfcRestore::new(...)` from a checkpoint (`:378-386`), driven per-step through `InteractiveJob::step`. |
| 3.4 | Three job bugs avoided | PASS | (a) one payload page per step — `encode_one` returns `Yield` after each committed page, comment at `:551-556` explains the `OpportunityExhausted`-empty-fault trap it avoids; (b) both retained payloads retired — `retire(&mut candidate.state)` **and** `retire(&mut candidate.output)` on both the Solve and Restore arms (`:495-511`); (c) close ladder before drop — `close_owned()` (`begin_close` + loop `close_step` to `terminal_is_empty`) called on every child `WfcJob`/`WfcRestore` before it's released (`:452-454`, `:502-503`, `:522`). |
| 3.5 | `satisfiable: bool` on the commit | PASS | Same as 1.7; `Grid3dInferenceCommit.satisfiable` set false only on the intercepted `wfc-unsatisfiable` fault (`:518-524`). |
| 4.1 | ≥1 fixture quintet per mutation | PASS | 14 mutation folders under `🧬️mutations/**`, 14 matching fixture folders under `🧫️fixtures/🧬️mutations/**`, each with `📸️snapshot/⬅️before`, `➡️after`, `🦠️mutation`, `🎯️outcome`, `🔺️diff` (spot-checked `📌️pin-cell/📌️pins-the-far-cell-to-wall`: all 5 files present). |
| 4.2 | Mounted Rust test per mutation | PASS | Each mutation module has a `🧪️tests/<name>/🦀️.rs` wired via `#[cfg(test)] #[path=...]` in the crate root (e.g. `🦀️.rs:225-227` for change-seed). |
| 4.3 | `.feature` row + python oracle branch + oracle manifest vector, per mutation | PASS | `🧪️tests/🧩️mutate-wfc-grid3d-1/{🥒️.feature,🐍️.py,🦀️.rs}` present; oracle manifest `🔮️oracles/🔣️.json` `mutationCatalogs[0].vectors` enumerates all 14 kinds with scenario dirs matching the fixture tree. |
| 4.4 | Render tests both windows, both examples, instance count = solved cell count | PASS | Grid window: `every_example_renders_a_non_empty_grid_scene` (`🧱️grid/🧪️tests/🔬️unit/🦀️.rs:16`). Preview window: `every_example_renders_one_instance_per_solved_cell` (see 2.6), iterates `[blocks, pipes_3d]` explicitly. |
| 4.5 | Deterministic-seed solve + contradiction tests | PASS | `the_same_seed_and_spec_always_produce_the_same_assignment` and `a_grid_with_no_allowed_pair_is_reported_as_a_contradiction_not_a_panic` (`💡️inferences/🧪️tests/🔬️unit/🦀️.rs:56`, `:38`); deny-beats-allow test also present (`:48`). |
| 4.6 | Claimed green runs, quoting logs | PASS | `🗑️generated/grid3d/test-final.log:869`: `test result: ok. 201 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.14s`. `check-final.log` (14:09, newest, postdates the report) and `wasm-1.log`/`clippy-4.log` all end `Finished ... target(s)` with zero errors in this crate (framework-crate warnings only, none touching grid3d). `gen-4.log:831`: `test result: ok. 2 passed; 0 failed`. |
| 5.1 | Approved verbs (`fix`/`clear`/`remove`/`restore`) per plan §7 mapping | PASS | `pin-cell`→`SemanticDescriptor{verb:"fix",record:"Fixed"}`, `unpin-cell`→`"clear"/"Cleared"`, `mask-cell`→`"remove"/"Removed"`, `unmask-cell`→`"restore"/"Restored"` (grepped all four mutation files directly). |
| 5.2 | Direction enum per-variant `#[value(rename = "…")]` + wire round-trip test | PASS | `📸️snapshot/🦀️.rs:105-117` uses `#[value(rename = "LEFT")]` etc. per variant (not `rename_all`, with an explanatory docstring on why). Test `every_direction_carries_its_screaming_wire_token` (`snapshot/🧪️tests/🔬️unit/🦀️.rs:70-84`) round-trips `to_value`→JSON string assert→`from_value`→equality for all six tokens — a genuine wire round trip, not just a label check. |
| 5.3 | Icon ids valid | PASS | `grid-3x3`, `mouse-pointer`, `lock`, `square-dashed`, `preview`, `box`, `eye`, `pencil` all found in `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs`. |
| 5.4 | `UiText` status ≤ 512 bytes | PASS | Test `the_status_line_stays_inside_one_ui_text_admission_unit` asserts `status.len() <= 512` (`preview/🧪️tests/🔬️unit/🦀️.rs:53-57`). |
| 5.5 | Floats `N.0` | PASS | Spot-checked `📏️change-cell-sizes/…/⬅️before/🔣️.json`: `"cellSizesX":[1.0,2.0]` etc., all canonical. |
| 5.6 | No dependency on another app plugin crate | PASS | `📦️packages/🦀️rust/Cargo.toml:21-46` — only framework crates, `semio-s-artifact-stdio-semio/txt`, `semio-s-plugin-wfc-engine`, `pack`, `serde*`. No `semio-s-plugin-{draw,raster,lowpoly,procedural,…}`. |
| 5.7 | Labels en+de | PASS | `artifact_kind()` localization rows "3D Grid"/"3D-Raster" (crate root `🦀️.rs:71-72`); every `LocalizedLabel::native("…", "…")` call site inspected carries both languages (Select/Auswählen, Pin/Anheften, Mask/Ausblenden, Grid/Raster, Preview/Vorschau, Edit/Bearbeiten, View/Ansicht, all action definitions). |
| 5.8 | Emoji docstrings | PASS | Every `//!`/`///` block opens with a unique fitting emoji throughout the files read. |
| 5.9 | No comments inside definitions | **PARTIAL** | Three inline `//` (not `///`) comments sit *inside* function bodies: `💡️inferences/🦀️.rs:505-508` (inside the `step()` match arm, explaining the two-retained-payload trap) and `:551-556` (same function, explaining the one-page-per-step trap); `📸️snapshot/💾️binary/🦀️.rs:90-91` (inside `Drop::drop`, explaining the `ManuallyDrop` safety invariant). A fourth, `🦀️.rs:507` (crate root) `// ---- Shims: flat access…` is a bare non-emoji section-header comment between module declarations. All four are substantively justified (unsafe-code rationale, a genuinely non-obvious framework trap) but are a literal violation of the stated convention. **Nice-to-have to reformat as doc comments on the enclosing block/function**, not a functional defect — none hide a logic error, all match content already stated in the file's module-level `//!` docs. |
| 6 | `io::io()` entries empty, native DSL/pack composer real | PASS | `🦀️.rs` (subset root) `:23-37`: `io() { entries: &[] }`, with the honest reasoning comment "no foreign-format hop of its own" — matches slice claim exactly. |

## Known-gaps rating (from `📓️grid3d.md` §7)

| gap | rating |
|---|---|
| `wfc-js:test` blocked by a missing `📋️project.json` on the sibling `◻️2d` crate | **not blocking for grid3d** — root cause is outside this artifact (plugin-root/slice-P scope), correctly deferred. |
| `verify taxonomy enforce` blocked by pre-existing unsorted `🧾️manifest.json` | **not blocking for grid3d** — pre-existing, unrelated, zero wfc rows in the stale manifest; correctly deferred, though it does mean taxonomy conformance is untested by tooling (nice-to-have to re-run once slice P's fix lands). |
| Preview solves synchronously on the render path (no retained-tool caching) | **nice-to-have** — measured sub-millisecond for both bundled examples; only a future document far larger than the examples would need the retained-tool fix. Correctly flagged, not swept under the rug. |
| Playground boot / browser probes / storybook gate not exercised by this slice | **not blocking for grid3d** — explicitly belongs to the plugin-root/W2 verification wave per plan §6, not to an artifact slice. |

No gap in the slice's own list is blocking for the brief; all are correctly scoped to a different slice
or genuinely deferred future work.

## Ordered fix list

1. (nice-to-have) Reformat the four inline `//` comments (`💡️inferences/🦀️.rs:505-508`, `:551-556`,
   `📸️snapshot/💾️binary/🦀️.rs:90-91`, crate-root `🦀️.rs:507`) into proper emoji-led `///`/`//!` doc
   comments on the enclosing item, per the "no comments inside definitions" convention. Content is
   already correct and non-redundant; this is pure convention compliance, zero behavioral change.
2. (nice-to-have) Once slice P fixes the `◻️2d` crate's missing `📋️project.json`, re-run
   `bun nx run @semio-tech/wfc-js:test` and `verify taxonomy enforce --scope "…/🧱️grid3d"` for grid3d
   specifically, to close the two tooling gaps this slice correctly left open.
3. (nice-to-have, not urgent) If a future example grows past the bundled `blocks`/`pipes-3d` scale,
   revisit the preview window's per-render synchronous solve (§7 gap 5) with a retained tool run instead
   of `ArtifactEditor::render`-path solving.

No blocking-for-brief defect was found: the document model, both editor windows' geometry/instancing,
the viewer parity, the inference wiring (`Stencil3d::Face6`, `Grid3dTopology`, resumable `WfcJob`, the
three job-bug fixes, `satisfiable`), the mutation/fixture/test/oracle coverage, and the naming/icon/verb
conventions all check out against the plan and the code as written.

**Totals: 33 PASS, 1 PARTIAL (non-blocking convention nit), 0 FAIL.**
