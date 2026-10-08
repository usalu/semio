# Lane K — Kernel-owned shape values (AD3)

Date: 2026-10-07/08. Crate `semio-framework-3d`, kernel root `K` = `🧰️framework/🔨️modules/🧊️3d/`. Paths below are relative to `K`.

## API added

Module `📐️brep/⚙️engine/🪅️shape-value/🦀️.rs` (mounted in `📐️brep/⚙️engine/🦀️.rs:17-19`, re-exported as `semio_framework_3d::brep::engine::{ShapeValue, ShapeRoot, ShapeWire, ShapeComponent, ShapeTessellationJob, ImportedShape}`).

| Item | Signature | File:line |
|---|---|---|
| `ShapeWire` | `struct { members: Vec<(EdgeId,bool)>, vertices: Vec<VertexId>, closed: bool, label: PersistentLabel }` (`Clone, Debug, PartialEq, ToValue, FromValue`) | shape-value `🦀️.rs:14` |
| `ShapeRoot` | `enum { Vertex(VertexId), Edge(EdgeId), Wire(ShapeWire), Face(FaceId), Shell(ShellId), Solid(SolidId), Compound{solids,label}, Curve{curve:Curve3,label}, Surface{surface:Surface,label} }` (camelCase) | `:25` |
| `ShapeComponent` | `struct { kind: GeometryKind, label: PersistentLabel }` | `:40` |
| `ShapeValue` | `struct { root: ShapeRoot, body: Body }` — `Clone, Debug, PartialEq, ToValue, FromValue`; `body` is exactly the entity closure of the root, ids dense in index order, labels verbatim, `body.labels.next` = highest carried label + 1 | `:48` |
| `ShapeValue::kind` | `fn kind(&self) -> GeometryKind` | `:71` |
| `ShapeValue::label` | `fn label(&self) -> Option<PersistentLabel>` (root label) | `:87` |
| `ShapeValue::components` | `fn components(&self, kind: GeometryKind) -> Vec<ShapeComponent>` (Vertex/Edge/Face/Shell/Solid from the body in arena order; Wire/Compound/Curve/Surface = the root when it has that kind) | `:101` |
| `ShapeValue::content_hash` | `fn content_hash(&self) -> String` (hash of the canonical JSON encoding; cache-key material) | `:115` |
| `ShapeValue::check` | `fn check(&self) -> Result<(), BrepError>` (dangling ids / root / label high-water; run by every import and tessellation) | `:121` |
| `ShapeValue::tessellate_job` | `fn tessellate_job(&self, deflection: f64) -> Result<ShapeTessellationJob, BrepError>` | `:173` |
| `ShapeValue::tessellate` | `fn tessellate(&self, deflection: f64) -> Result<MeshTransfer, BrepError>` | `:189` |
| `ShapeTessellationJob` | `step(&mut self, budget: usize) -> Result<TessellationStep, BrepError>`, `progress() -> TessellationProgress`, `cancel()`, `is_terminal()`, `into_mesh() -> Option<(MeshTransfer, TessellationReport)>`; owns its `ShapeValue`, no session | `:202-234` |
| `Brep::export_shape` | `fn export_shape(&self, handle: &GeometryHandle) -> Result<ShapeValue, BrepError>` | `:256` |
| `Brep::import_shape` | `fn import_shape(&mut self, shape: &ShapeValue) -> Result<GeometryHandle, BrepError>` | `:285` |
| `Brep::import_shape_mapped` | `fn import_shape_mapped(&mut self, shape: &ShapeValue) -> Result<ImportedShape, BrepError>`; `ImportedShape { handle, label_offset }`, `session_label(label) -> PersistentLabel` | `:291`, `:55-63` |

Supporting kernel changes:
- `Body::merge_selected(&mut self, other, keep: Option<&ReachSet>) -> MergeMap` (`📐️brep/📸️representation/🕸️topology/🦀️.rs:883`); `Body::merge` now delegates to it. This is the extraction primitive (merge of a reachable subset into a fresh body).
- `PartialEq` derived on `Body`, `Store`, `Slot` (`🏟️arena/🦀️.rs`), `LabelSource` (`🕸️topology/🦀️.rs`).
- `TessellationJob::for_solids` (`📐️brep/💡️queries/🧩tessellation/🦀️.rs:248`) and Compound support in `Brep::tessellate_sync` / `tessellate_job_sync` (compounds previously could not be tessellated at all).
- Determinism fix in the boolean engine: `group_shells` (`📐️brep/🛠️operations/🔀️boolean/🦀️.rs:2201`) iterated a `HashMap`/`HashSet`, so fuse and cut results had a different face order on every run (8 builds of the same recipe gave 8 distinct values). Now `BTreeMap<FaceId, BTreeSet<FaceId>>`.
- `HalfedgeMesh` pure-consumer read API (`🥽️mesh/🦀️.rs:395-410`): `positions() -> Vec<[f32;3]>`, `polygons() -> Vec<Vec<u32>>` (wound, flipped faces reversed), `edge_ids() -> Vec<EdgeId>` (one canonical id per undirected edge), `vertex_normal(VertexId) -> MeshResult<Vec3>`. Constructors from indexed polygons already existed (`from_faces`, `from_indexed_triangles`, `from_face_loops`, `from_indexed_triangles_by_face_id`); counts, `vertex_position`, `face_vertex_ids`, `face_normal`, `edge_endpoints` already existed.

## Semantics other lanes must know

- Fresh session: `import_shape` restores every label, and handles are `hash(kind,label)`, so the imported handle equals the exported handle and `handle_for_label(label)` resolves every sub-element.
- Populated session (e.g. a boolean with two operands imported into one session): labels shift by `label_offset`; use `import_shape_mapped` and `ImportedShape::session_label(label)` to translate a label held against the value. Labels of an operation's result are session-minted; persist selections as labels of the value they were taken from.
- `handle_for_label` does not resolve Wire/Compound/Curve/Surface labels (they are not arena entities); keep their handle from the import.
- A value never aliases: importing it twice into the same session yields two distinct handles.
- Value-level tessellation supports Vertex, Face, Shell, Solid, Compound, Wire (same set as the session path plus Compound); Edge/Curve/Surface return `InvalidInput`, as in the session.
- Equality is value equality (`==`); canonical bytes are `semio_framework_pack_json::to_json_string(&shape)`; `content_hash()` is the cheap key.

## Tests (all run, results seen)

Rust, target dir `.🧬semio/🦑️repo/⚡️cache/cargo/target-g3d-lane-k`:

- `cargo test -p semio-framework-3d --lib -- shape_value bulk_read` → 9 passed (8 shape-value + 1 mesh). Tests live in `📐️brep/⚙️engine/🪅️shape-value/🧪️tests/🔬️unit/🦀️.rs` and `🥽️mesh/🧪️tests/🔬️unit/🦀️.rs`:
  - `shape_value_round_trips_every_kind_with_fixture_counts_and_measures` — all 9 kinds, JSON codec round trip, fresh-session handle equality, label equality, component counts, volume/area/length vs analytic fixture values.
  - `shape_value_booleans_on_imported_shapes_equal_session_native_results` — fuse/cut/intersect on imported values vs session-native vs parry3d cuboid algebra.
  - `shape_value_labels_resolve_after_import_in_fresh_and_populated_sessions` — every vertex/edge/face/shell/solid label of a cut-through-hole solid resolves in a fresh session (same handle as the source) and shifted in a populated one; pre-existing handles untouched.
  - `shape_value_encoding_and_handles_are_deterministic` — export-import-export byte identical, two fresh sessions identical handles and bytes, same-session re-import does not alias.
  - `shape_value_recipes_build_identical_values_in_every_fresh_session` — the boolean nondeterminism regression (red before the `group_shells` fix: 8 distinct hashes, green after: 1).
  - `shape_value_tessellation_equals_session_tessellation_and_is_resumable` — one-shot and budget-1 stepping equal `tessellate_sync`; monotone progress; cancel yields `Cancelled` and no mesh.
  - `shape_value_fixture_tessellations_match_fresh_value_tessellation_and_oracle_measures`.
  - `shape_value_import_refuses_dangling_references_without_mutation`.
  - `bulk_read_accessors_expose_a_pure_consumers_view_and_rebuild_the_mesh`.
- Regression: `-- brep::engine brep::representation brep::operations::boolean mesh::tests` → 366 passed, 0 failed (this includes the `Body::merge` / `group_shells` changes). Remaining suite (`-- --skip` the four filters above): 269 passed, 4 failed, then did not finish within ~30 minutes under fleet load (slow `mesh::modeling` tests reported "running for over 60 seconds"; the run was abandoned without a final result line). The 4 failures are all in the peer lane's in-progress `brep::queries::analysis::tests` (`a_compound_combines_its_solids_with_the_parallel_axis_theorem`, `solid_distances_match_the_fixture`, `face_and_edge_tables_describe_every_primitive`, `primitives_match_their_closed_form_mass_properties_and_bounds`), not in code this lane touched.

TypeScript: `NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/s-3d-js:test --skip-nx-cache` → red first (1 failed, 9 passed: fixture had no `tessellations`), then green (10 passed). The new test `measures shape-value fixture tessellations with independent Three volume, area and manifold checks` is in `🧪️tests/🧪️semio-tech-geometry-brep-js/🟦️.ts`; it recomputes signed volume and `three.Triangle` area from the fixture meshes, compares to the fixture's analytic volume/area (relative tolerance 1e-6, 2% for the faceted cylinder), checks every welded edge is used exactly twice and the face-label count.

Fixture: `📐️brep/⚙️engine/🧫️fixtures/🪬️shape-values/🔣️.json` — 14 recipe cases (box, cylinder, fuse/cut/intersect of cubes, vertex, edge, face, shell, planar face, rectangle wire, compound, line curve, plane surface) with kind, labelled component counts and analytic measures, plus 5 pinned tessellations (positions, indices, face labels). Consumed by Rust (`include_str!`) and TypeScript.

## Open issues

1. TDD order: the `ShapeValue` module and its tests were written together; only the determinism test, the TS test and the fixture tessellations were observed red before green. The first compile of the module was green.
2. Booleans may have further nondeterminism sources (`HashMap`/`HashSet` iteration in `🛠️operations/🔀️boolean/🦀️.rs`); `group_shells` was the one that changed output in the fixture cases (fuse, cut, intersect, 6 repeated builds each now give 1 hash). Other operations (sweeps, blends, offsets) were not audited. Any DepHash cache in the geometry inference relies on this.
3. Labels of a boolean result are not stable across edits of the upstream chain beyond what the kernel history provides; AD3's "stable for an unchanged upstream chain" holds (deterministic builds), not across upstream edits.
4. `ShapeValue` encodes the whole closure; it is not structurally shared between widget outputs (a `ShapeValue` per widget output). Large shapes will want `Arc<ShapeValue>` at the inference layer; the API takes `&ShapeValue`.
5. `HalfedgeMesh` from a `MeshTransfer` of a shape: not added (the existing `from_indexed_triangles_by_face_id` takes the contract mesh's `position`/`index` plus per-triangle face ids derived from `face_groups`).
6. Cargo builds were repeatedly blocked for minutes by peers' half-landed files (`🥽️mesh/🔎️quality` tests, `💡️queries/🔎️analysis`); waited and retried, no forward fixes needed.
