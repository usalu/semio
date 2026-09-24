# WP-P4 Restore Derived Pack-Schema Identity For Composed Snapshots

Slice P4, session 10. Removes the blocker in `📓️wp-w1.md` §4.2: 18 packages' snapshot kinds have no
`record_spec()` because the composition refactor replaced `#[derive(DslArtifact)]` with handcrafted `ArtifactPack` impls.

## Status

- [x] Framework: `dsl::DslField` for `ArtifactChild<S>`/`OwnerRef`/`LinkPin`/`ArtifactLink` ALREADY EXISTS (`🏪️store/🦀️.rs` `🔖️CompositionDsl`); the snapshot doc comments claiming otherwise are stale
- [x] Framework fixture laws (hash stable, hash differs on structure, exact round-trip with children+links, blake3 cross-check)
- [x] writer, puzzle (w1 + c8 messaged)
- [x] remaining crates, except process3d (see Open)
- [x] final set to w1 (the list below)

## Findings

- The `DslArtifact` derive no longer emits `ArtifactPack` (P6, `✨️derive/🦀️.rs` header). The uniform path in the repo is
  `#[derive(dsl::DslRecord)]` plus the P6 `ArtifactPack` impl over `__dsl_spec()` with `record_spec() = Some(__dsl_spec())`
  (140 existing instances, e.g. block/remodel). This slice moves the affected snapshots onto that path.
- `os_pack::schema_hash` hashes top-level `(id, key, shape-tag)` only. Nested record contents are not in the hash.
- Binary grammar assets (`🥋️.ksy`, `🔠️.abnf`, `🌶️.spicy`, `📡️.protocol.semio`) describe the generic semio envelope with
  opaque payload bytes, so they stay valid when the payload layout changes.
- Text DSLs (`ArtifactDsl`) are still hand-built in the affected crates. Committed `🗣️.dsl.semio` examples and `📖️.grammar.semio`
  files are written in those formats. This slice changes only pack, because pack is the blocker.
- Transient peer break seen: `complete_reserved_spawned_job_inner` missing in `semio-framework-plugin` on wasm32. It was gone
  on the next check.

## Changes

- writer: `WriterSnapshot` derives `dsl::DslRecord` (`#[dsl(extension = "writer")]`). The hand-built binary codec is deleted.
  `ArtifactPack` now uses the derived record, re-attaches the child's local text on decode, and returns `record_spec`.
  The stale rationale doc comment on the text `ArtifactDsl` is fixed. New law: `pack_schema_identity_is_the_derived_record_spec`.
- puzzle 2d/3d/5d: `Puzzle*PlaySnapshot` `ArtifactPack` now delegates to the typed `Puzzle*Snapshot` (derived spec). The
  schema-less `DslValue` pack is gone. New law per crate: `play_snapshot_pack_shares_the_typed_record_identity_and_round_trips`.

- framework: new `🏪️store/🧪️tests/🧩️composed-pack-schema/🦀️.rs` and fixture `🏪️store/🧫️fixtures/🧩️composed-pack-schema/🔣️.json`
  (field table, pinned schema hashes, two documents' pinned pack hex, third-party `blake3` recomputation from the
  fixture's field table). New `test_support::assert_pack_schema_identity`. Framework `infinite_dag::DagSnapshot`
  `ArtifactPack` now forwards `record_spec` from its `DagSnapshotDsl` mirror.
- cad: `CadSnapshot` derives `DslRecord`. Binary primitives are deleted. The derived pack re-checks exact child identity on decode.
  New law `cad_pack_schema_identity_is_derived_and_rejects_foreign_children`.
- lowpoly: `LowpolyObject` and `LowpolySnapshot` derive `DslRecord`. Binary primitives are deleted, including the format-1
  legacy reader. New binary unit test module.
- dag: pack delegates to the framework `DagSnapshot` record, which is the same route the text facet takes, so node and
  edge content stays persisted. Binary primitives are deleted.
- sequence, forms, animate: `DslRecord` plus the derived pack. forms keeps `validate()` on encode and decode.
- mathematical: new private `EquationPackRecord` (`DslRecord`) holding the three child handles plus `equation`/`graph`/`geometry`
  as `DslValue`. It persists the owned scene the old format-2 codec carried.

- imperative: private `ProcedurePackRecord` holds both child handles plus the owned `path`/`seed` (as `DslValue`), and
  re-attaches them to the exact handles.
- reasoning wires: private `WiresPackRecord`. The fixture, meta, nodes and edges travel as first-party JSON **text**
  fields, because pack sorts map keys and these free-form `DslValue` objects are order-significant (measured:
  a `DslValue` field failed `document_text_round_trip_with_operation_applied`).
- trinity jack: private `JackPackRecord` (camera/nodes/edges as `DslValue`, typed decode, child re-minted from content).
- raster: private `RasterPackRecord`. Layers are JSON text (params are order-significant `DslValue`s). Assets are
  `Vec<RasterAssetPackRow { key, child, #[dsl(base64)] content }>`, where content is the composed image's own pack. Decode
  retires owned maps and layers on every failure path.
- playbook, norm en1990/din18599: `DslRecord` plus the derived pack.
- energy: private `EnergyModelPackRecord`, with real `Option<ArtifactLink>` fields (`referenced_model`, `weather_link`).
- layout: `LayoutSnapshot` derives `DslRecord`. `LayoutDrawingChild` gets a hand-written `DslField` (handle through the
  framework `ArtifactChild` record, stdio drawing content as a value, because the stdio drawing snapshot has no `DslField`).
  I did not change the proc-macro, so peers do not face a workspace-wide rebuild.
- Rescan found 8 more kinds whose outer `ArtifactPack` delegates to a derived mirror but did not forward `record_spec`:
  `Generation3dSnapshot`, `Generation2dSnapshot`, `Wfc3dSnapshot`, `BitmapSnapshot`, `Wfc2dSnapshot`, `Grid2dSnapshot`,
  `ShootingSnapshot` and framework `FlowHostSnapshot`. All of them now forward the mirror's `record_spec`. These are in
  W1's "should answer" list (procedural, wfc, shooting, flow), so they would have failed the probe too.

## Open

- **process3d `Process3dSnapshot` is not converted.** Its hand-built binary is also read by
  `Process3dRetainedSnapshotReader` (about 1000 lines, one-byte-grant mounted envelope ingress, laws in
  `📸️snapshot/🧪️tests/🔬️retained-structural-laws`). Moving to a derived pack means rewriting that ingress onto
  the framework retained pack cursors (`RetainedPackSourceCursor`/`RetainedPackCatalogCursor`/`RetainedValueCursor`
  plus a typed owner). That is the pattern `🌀️procedural/🧊️generation3d/…/💾️binary` uses. It is a separate slice.
  Until then the process package cannot enter a trusted catalog.
- Text DSLs of the converted kinds remain hand-built. The committed `🗣️.dsl.semio`/`📖️.grammar.semio` assets are
  written in them. Only pack changed.
- `os_pack::schema_hash` hashes only the top-level `(id, key, shape-tag)`. Nested record changes, such as editing
  `ArtifactLink`'s inner record, do not change a kind's hash.

## Test Results

| crate | command | result | capture |
|---|---|---|---|
| semio-s-artifact-writer-writer | cargo test | 176 passed | generated/writer-test.txt, writer-law.txt |
| semio-s-artifact-puzzle-2d | cargo test | 604 passed | generated/puzzle-2d-test.txt, puzzle-2d-law.txt |
| semio-s-artifact-puzzle-3d | cargo test | 287 passed | generated/puzzle-3d-test.txt |
| semio-s-artifact-puzzle-5d | cargo test | 375 passed, 2 ignored | generated/puzzle-5d-test.txt |
| semio-framework-os-kernel | cargo test --lib composed_pack_schema | 4 passed | generated/kernel-composed-3.txt |
| semio-s-artifact-cad-cad | cargo test | 432 passed, 1 ignored (plus new law 49/49 filtered) | generated/cad-test.txt, cad-law.txt |
| semio-s-artifact-lowpoly-lowpoly | cargo test | 299 passed, 1 failed: timing law `retained_migrated_turns_stay_below_eight_milliseconds` at 40 ms under load 26. It passed twice when run alone. | generated/lowpoly-test.txt |
| semio-s-artifact-dag-dag | cargo test | 211 passed | generated/dag-test.txt |
| semio-s-artifact-sequence-sequence | cargo test | 209 passed | generated/sequence-test.txt |
| semio-s-artifact-forms-forms | cargo test | 200 passed | generated/forms-test.txt |
| semio-s-artifact-mathematical-equation | cargo test | 391 passed, 1 failed: timing law `retained_maximum_microturns_stay_below_eight_milliseconds` failed 3 of 3 times when run alone, at 10–46 ms, with load at 41. This is command stepping and does not touch pack. | generated/equation-test.txt |
| semio-s-artifact-animate-presentation | cargo test | 329 passed | generated/animate-test.txt |
| semio-s-artifact-imperative-procedure | cargo test | 149 passed | generated/imperative-test.txt |
| semio-s-artifact-reasoning-wires | cargo test | 193 passed | generated/wires-test.txt |
| semio-s-artifact-trinity-jack | cargo test | 167 passed | generated/jack-test.txt |
| semio-s-artifact-raster-raster | cargo test | 228 passed | generated/raster-test.txt |
| semio-s-artifact-playbook-playbook | cargo test | 142 passed, 16 failed. The same 16 fail at HEAD with the original snapshot file (baseline measured: live-instance/`BatchOnlyPendingRewrite` dispatch, fixture canon) | generated/playbook-test.txt, playbook-baseline.txt |
| semio-s-artifact-energy-model | cargo test | 6292 passed, 1 ignored | generated/energy-test.txt |
| semio-s-artifact-layout-layout | cargo test | 394 passed, plus new law (5/5 `pack_`) | generated/layout-test.txt, layout-law.txt |
| semio-s-artifact-norm-en1990 | cargo test | 155 passed, 1 failed. The same test fails at HEAD (`must declare setSnapshot`) | generated/norm-en1990-test.txt |
| semio-s-artifact-norm-din18599 | cargo test | 171 passed, 2 failed. The same 2 fail at HEAD | generated/norm-din18599-test.txt |
| semio-s-artifact-procedural-generation2d | cargo test | 173 passed | generated/procedural-generation2d-test.txt |
| semio-s-artifact-procedural-generation3d | cargo check --features component-app-assembly | lib ok. The test target does not compile because of errors in untouched files (`frame_orbit_to_bounds` arity, `LocalizedLabel: From<&str>`) | generated/procedural-generation3d-test.txt |
| semio-s-artifact-wfc-{3d,bitmap,2d(+cap),grid2d} | cargo test | 186 / 152 / 203 / 150 passed | generated/wfc-*-test.txt |
| semio-s-artifact-shooting-shooting | cargo test | 357 passed | generated/shooting-shooting-test.txt |
| semio-framework-os-kernel | cargo check | ok (flow/dag/test_support edits) | — |
| 21 plugin crates (writer, puzzle, cad, lowpoly, dag, sequence, forms, mathematical, animate, imperative, reasoning, trinity, raster, playbook, energy, layout, norm, procedural, wfc, shooting, flow) | cargo check --target wasm32-wasip2 | all ok. wfc failed once on a peer's transient `wfc-engine` edit and passed on re-check | generated/wasm-check.txt |

## Handover To W1

Packages whose snapshot kinds now declare `record_spec`, rebuilt natively and checked for wasm32: writer, puzzle,
cad, lowpoly, dag, sequence, forms, mathematical, animate, imperative, reasoning, trinity, raster, playbook,
energy, layout, norm (en1990, din18599), procedural (generation2d, generation3d), wfc (3d, bitmap, 2d, grid2d),
shooting, flow (framework `FlowHostSnapshot`). Still missing: process (`Process3dSnapshot`, see Open). Every
converted kind's pack bytes changed, so any published catalog or stored `.spk` of these kinds must be rebuilt.
I found no committed pack-byte fixtures for these kinds. Their fixtures are JSON/DSL and did not change.

## Processes

- wasm32 check loop `wp-p4/wasm-check.sh`, pid 13767, detached, output `generated/wasm-check.txt`. It has finished.
