# Draw host-tests and model-tests, static repair (2026-11-24)

Read-only pass: no cargo, rustc or build was run. Every statement is marked PROVEN (read from current code) or INFERRED (reasoned, to be confirmed by the next compile). Draw root below = `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing`; Subsets = `🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 1. host-tests (`semio-s-artifact-draw-drawing-host-tests`)

Test sources: `🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/**` (companion root `🔨️modules/🏠️host/🧪️tests/🥣️companion/🦀️.rs`).

### Drift found and fixed (PROVEN against production)
| # | Stale test code | Production today | Fix |
|---|---|---|---|
| 1 | `authority.step(&mut DrawingSnapshot, &DrawingMutation, cx)` (kit x3, candidate-laws x4, duplicate-laws x4, edit-laws x4) | `step(DrawingSourceAccess, RetainedCloneRef<DrawingMutation>, cx)`; `DrawingSourceAccess` is `pub(super)` | new `HeldMutation` in the kit (owns the mutation in a `FixtureSource<DrawingMutation>`, plus a sealed `FixtureSource<DrawingSnapshot>` copy for `DuplicateLayer`); production gets one testkit-gated entry point (below) |
| 2 | `use ...host::owned::initialization_turn::*` | module no longer exists | import removed |
| 3 | `PagedList::reserve_one(usize::MAX)` | takes an effect; cold form is `reserve_one_cold(usize)` | kit `reserve_native_layer_slots` |
| 4 | `DrawingMutationDigestAuthority::close_step(DRAWING_OWNED_FIELD_BYTES)` | `close_step(&mut self)` | kit `digest()` |
| 5 | `clone.index == 1` / `== 4` on `DrawingSegmentsCloneAuthority` / `DrawingTextCloneAuthority` | `DrawingNativeCloneAuthority` has `cursor, value, retirement, phase, terminal`, no `index` | edit-laws: the cancellation now lands after N advance turns of the retained clone (N = 2 path, 5 text; earlier when `phase != 0`), asserting the clone is still retained, then the unchanged cancel / close / `source == expected` assertions |
| 6 | `create_drawing_boolean_layer(.., vec![x.into()])`, `vec!["a".into(), "b".into()].into()` | children is `PagedList<PagedUtf8>` | kit `nested_snapshot`, `rich_layer` |
| 7 | `RenameLayer { layer_id: "x".repeat(..) }` (String) | `layer_id: PagedUtf8` | candidate-laws `.into()` |
| 8 | arena-laws: `context` / `exhausted` / `contended` `StepContext`s keep `&mut preview_sequence` to scope end (`StepContext` implements `Drop`) while a second context borrows it | | `drop(context)`, `drop(exhausted)`, `drop(contended)` after their last use |
| 9 | arena-laws `close_arena_bootstrap_step -> store::SnapshotRetirementStep` | `DrawingMutationArenaPoolBootstrap::close_step` returns `RetainedCloneStep` (each released root is `copied_items: 1`) | helper returns `RetainedCloneStep`; the count of released roots is the sum of `copied_items` (at most 1 per step) |

### Production change (testkit-gated, no visibility widened)
`🔨️modules/🏠️host/🧰️owned/🧩️mutation-candidate/🦀️.rs`: `#[cfg(any(test, feature = "testkit"))] DrawingMutationCandidateAuthority::step_with_custody(source, base, mutation_ref, cx)`. It picks `DrawingSourceAccess::Shared(source, base)` exactly while `needs_shared_snapshot(mutation)` holds (duplicate layer, phases `LocateCloneSource` / `PrepareOwnedValue`, PROVEN from `step_prepare_owned_value` which requires `access.base()`), otherwise `Exclusive`. The existing `testkit` feature of `semio-s-artifact-draw-drawing-host` is already enabled by the companion. Nothing in production calls `step` any more (grep over the repo: only host files), so no production caller was touched.

### Not verified (INFERRED)
- That the full host-tests crate now compiles: ticket named 27 errors, I fixed every class listed there plus the `SnapshotRetirementStep` mismatch and the one `RenameLayer` String; unlisted remaining errors cannot be excluded without a compiler.
- That `FixtureSource<DrawingMutation>` / `FixtureSource<DrawingSnapshot>` admission fits the stock fixture wallet for the large fixtures (`DuplicateLayer` of the nested snapshot, 4097-byte rename). Mutations are cloned (`DrawingMutation: Clone`, PROVEN) so the existing `drain_mutation(mutation)` calls stay.
- That the released-root counts asserted in arena-laws (`failure_at`, `(owner + 1) * 20`, `4 * 20`) still equal the `copied_items` sum: production code reads consistently (`DrawingMutationArenaOwnerBuilder::close_step` releases one root per call with `copied_items: 1`), but the old assertions were written against another return type.

## 2. model-tests (`semio-s-artifact-draw-drawing-model-tests`)

Companion root `🧬️schema/🧪️tests/🥣️companion/🦀️.rs`, manifest `🧬️schema/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml`.

### Fixed (PROVEN unless marked)
1. Manifest: added dev-dependencies the mounted tests name but the manifest lacked: `semio-framework-2d`, `semio-framework-artifact-reference`, `semio-framework-pixels`, `semio-framework-value-persistent`, `semio-s-artifact-draw-drawing-io` (all workspace entries exist), and path dependency `semio-framework-value-testing-admission` (receiver driver).
2. Companion root: `extern crate semio_framework_pack as pack;` (the fill-rule test uses `pack::snapshot::ArtifactPack`; the alias used to live in the io crate lib).
3. `test_step` was `#[cfg(test)]` inside the model lib, unreachable from the companion, and its `drive_reply` call had no type arguments (E0282 recorded on 2026-11-23 for the model lib's own test cfg). Removed from `🧬️schema/📸️snapshot/🔎️lookup/🫴️original/🦀️.rs`, defined in the companion root with `drive_reply::<RetainedCloneStep, ValueError>`; the two lookup tests (`original-text`, `equality`) import `crate::test_step`. No other user of `test_step` exists (grep).
4. `CURVE_LINE_SAMPLE_STEPS` was a private const of the geometry crate: new `#[cfg(any(test, feature = "testkit"))] pub mod testkit` in `🧬️schema/🧮️geometry/🦀️.rs` exposes it; unit test uses `semio_s_artifact_draw_drawing_geometry::testkit::CURVE_LINE_SAMPLE_STEPS`.
5. Unit test: `crate::font_family::DrawingFontFamily::Anta` (resolved against the companion crate) now the model path.
6. Vector tests (`🧵️vector/🧪️tests.rs`, `🧪️preparation.rs`): `RetainedCloneSource::admit_owned` does not exist; replaced by `FixtureSource::from_owner` (the fixture drains itself on drop, so the manual `source.close_step` loops, `next_close_demand`, `try_authority` are removed; `try_authority().unwrap()` became `source.borrow().get()`). `tests.rs` additionally imports the trait `EmbeddedChildVectorSourceV1` so `DRAWING_VECTOR_SOURCE.read(..)` / `read_demand()` resolve (the methods exist, the trait was not in scope).
7. Receive test (`🛬️receive/🧪️tests/🦀️.rs`): `NativeReceive::begin_receive` / `receive_birth_demand` and `RetainedCloneSource::admit_owned` are gone. Rewritten over the shared `receiver-driver.rs` (same driver as the FEM node test): for cuts 0, 1, 20, 200, 100000 and the uncut run, `driver::lifecycle` receives or cancels the `DslValue` corpus, denies every demanded axis before each funded turn, and closes receiver and source; the uncut run asserts equality with the serde-decoded `DrawingSnapshot`. Per-turn heap equality is asserted by the driver (INFERRED to match the removed hand-written assertions; the old test compared serde output only at the largest cut, the new one at the uncut receive).

### Removed tests (subject no longer exists; this IS a coverage change)
Three tests in `🧬️schema/🧪️tests/🔬️unit/🦀️.rs` call functions that do not exist in any crate today: `resolve_boolean_layer_segments`, `resolve_trace_layer_segments`, `drawing_image_asset_luma`. The behaviors moved into staged scene jobs (`🎬️scene/🔀️booleans`, `🔍️trace`: luma is computed inside the trace job state machine, `"luma"` phase, formula `(299 r + 587 g + 114 b) * a / 255000`). Removed:
- `resolve_boolean_layer_segments_returns_empty_for_missing_children_and_invalid_operation`
- `resolve_trace_layer_segments_returns_empty_without_assets_or_source_or_valid_decode`
- `drawing_image_asset_luma_uses_intrinsic_alpha_weighted_samples_and_refuses_wrong_extent`
The scene crate keeps its own unit suites (`🎬️scene/🔀️booleans/🧪️tests/🔬️unit/🦀️.rs`, `🎬️scene/🔍️trace/🧪️tests/🔬️unit/🦀️.rs`); whether they cover the missing-child, invalid-operation, missing-asset and luma-weight cases was NOT checked. Follow-up: add those cases through `DocumentVectorJob` (see `scene_plan` in `🎬️scene/📋️prepare/🧰️testkit/🦀️.rs`) once the refusal contract of the job is confirmed by a run.

### Not verified / left
- Compile state of the mounted io tests (binary / text / sqlite snapshot, `🚪️io/🧪️tests/🔬️unit`), affine, fill-rule, inferences, topology, shape coordinates, validation, fill sampling and the unit file beyond the points above: names and types read consistent (checked: layer base and body structs, topology API, mutation field types, `DrawingVectorSource`, validation test already on `FixtureSource`), but only a compile shows the rest.
- `nested_snapshot`-style `.into()` inference of the 2026-11-23 note ("extra args on several calls") was not reproducible by reading; if the compile still reports extra-argument errors they are in call sites I judged consistent.

## 3. editor-tests-2 / editor-interaction-tests / examples

No certain cause found by reading; nothing changed there.
- `export_work_tests::original_export_read_returns_under_exact_physical_grants` (`✏️editor/🎮️commands/📤️export-document/🧵️export/🧪️tests/🔬️unit/🦀️.rs`): the test pumps `returned` reads with `pump_returns` and expects the store disposer to reach terminal-empty; the 11-23d hypothesis (`SnapshotReadLeaseRegistryOwner::close_custody_step` waits for `strong_count() == 1`, so some alias outlives the returned reads) is consistent with the code but I could not identify the holder: it lives in the framework store/snapshot-read layer plus the export work testkit, both outside what I may touch or compile.
- `panels::properties::...inspector_controls_bind_the_events_the_host_dispatches`: the test is the only one in the file that opens a tree window, and builds the `ViewModel` with struct-update syntax. Both `TreeWindows` and `ViewModel` are plain `String` / `Vec` owners (PROVEN), so the leaked "UI arena descendants" are not the view; the remaining candidates are the rendered open-window rows or their projection. Not decidable by reading; needs the per-node arena accounting run noted on 11-23d.
- Clipboard cases: another agent.

## Files changed (absolute)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧩️mutation-candidate/🦀️.rs` (production, testkit-gated `step_with_custody`)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🦀️.rs`
- `.../🔬️retained-mutation-authority/🧰️kit/🦀️.rs`, `🧩️candidate-laws/🦀️.rs`, `🪞️duplicate-laws/🦀️.rs`, `✏️edit-laws/🦀️.rs`, `🏊️arena-laws/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/🦀️.rs` (production, testkit-gated `testkit` module)
- `.../🧬️schema/📸️snapshot/🔎️lookup/🫴️original/🦀️.rs` (production, removed unused `#[cfg(test)] test_step`)
- `.../🧬️schema/📸️snapshot/🔎️lookup/📝️original-text/🧪️tests/🦀️.rs`, `.../⚖️equality/🧪️tests/🦀️.rs`
- `.../🧬️schema/🧪️tests/🥣️companion/🦀️.rs`, `.../📦️packages/🦀️rust/Cargo.toml`
- `.../🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `.../🧬️schema/🧵️vector/🧪️tests.rs`, `🧪️preparation.rs`
- `.../🧬️schema/🛬️receive/🧪️tests/🦀️.rs`
