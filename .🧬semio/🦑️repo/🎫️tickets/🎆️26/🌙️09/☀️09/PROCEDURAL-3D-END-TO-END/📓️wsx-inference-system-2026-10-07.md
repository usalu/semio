# 📓 WSX Inference System Audit (read-only)

Scope: the framework's inference system (derived, read-only values computed from an artifact snapshot), its schema-first `💡️inferences/` folders, the expensive-geometry execution paths, generation3d's real geometry path, and the recipe for adding an inference.

All paths are relative to the repo root `/Users/ueli/Documents/semio/`. Line numbers were read directly. Nothing was built, run or tested (read-only audit). Claims marked **unverified** were not traced to runtime.

Abbreviations used below (full paths are the same, shortened for reading):

- `OSMOD` = `🧰️framework/🛍️products/💻️os/🔨️modules`
- `CADINF` = `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `GENINF` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `PUZINF` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`

---

## 1. Core lifecycle

### 1.1 Files and what they define

| Concern | Location |
|---|---|
| Inference trait (pure, sync, whole snapshot) | `OSMOD/📡️spr/🎮️command/🦀️.rs:32-34` `pub trait Inference<P>: Clone + Default + ToValue + FromValue { fn infer(snapshot: &P) -> Result<Self, ValueError>; }` |
| Read-region vocabulary | same file `:36-73` (`TouchedPaths`, prefix-in-either-direction intersection) |
| Tier-1 diff gate trait | same file `:83-85` `pub trait DiffRegions { fn touches(&self) -> TouchedPaths; }` |
| Field spec + family spec | same file `:90-104` (`InferenceFieldSpec { id, reads }`, `InferenceSpec<P>::{inference_schema_id, schema_version, fields}`) |
| Per-entity DAG cache, DepHash, driver | `OSMOD/💡️inference/🦀️.rs` (322 lines) |
| Mount of the cache module as `os_inference` | `OSMOD/../📦️packages/🦀️rust/🦀️.rs:266-273` (`#[path = "../../🔨️modules/💡️inference/🦀️.rs"] pub mod os_inference`, `pub use crate::os_inference::*`) |
| Executable inference service (plugin boundary) | `OSMOD/🔌️plugin/🦀️.rs:1495-1830` (metadata, payload contract, `ArtifactInferenceService`, registry, `wire_artifact_infer`) |
| Cold job bridge `semio.infer` | `OSMOD/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs` (725 lines) |
| Guest entry from host | `OSMOD/🔌️plugin/🖥️host/🦀️.rs:5967-5968` (`infer` -> `run_job_on_worker("semio.infer", …)`) |
| MCP/agent access | `OSMOD/🌉️mcp/💡️inference/🦀️.rs` (1944 lines), resource `semio://artifact/{artifactId}/inference/{field}` at `OSMOD/🌉️mcp/🧠️context/🦀️.rs:247-266` |
| Shell per-document inference port | `OSMOD/🖥️shell/🦀️.rs:322-328` (`SetDocumentInferencePort`, `inference_port_by_document`); TS opening mailbox `OSMOD/💡️inference/🚪️opening/🟦️.ts` (`InferencePortOpeningMailboxV1`), consumed by `OSMOD/📺️renderer/.../🏛️ShellHost/🟦️.tsx:3164-3169, 3613-3620` |

### 1.2 `InferredField` / DepHash / cache (`OSMOD/💡️inference/🦀️.rs`)

- `DepHash` (`:18-54`): merkle-style link. `DepHash::root` = `blake3(field_id ‖ 0 ‖ schema_version ‖ 0 ‖ input)` (`:24-31`). `DepHash::chain` folds the sorted parent hashes via `semio_framework_hash::merkle_node` (`:36-53`).
- `InferredField<P>` trait (`:84-113`): `FIELD_ID`, `SCHEMA_VERSION`, `reads()` (tier-1 read set), `plan(snapshot) -> Vec<InferenceStep{key,parents}>` (roots first), `dep_input(snapshot,key,parents)` (honesty contract: must cover everything `compute` reads), `compute(snapshot,key,parent_values)`.
- Config (`:117-139`): `InferenceCacheConfig { enabled: false, budget_bytes: 16 MiB, persistence: None, record_stats: false }`. `InferencePersistence::Projection` exists (`:136-139`) but nothing in the tree uses it.
- Cache (`:150-230`): `HashMap<DepHash, CacheEntry>` + `VecDeque` LRU + byte budget. `touch` is O(n) per hit (`:212-217`).
- Session / tier-1 state (`:237-245`): `InferenceSession { roots: HashMap<&'static str, (DepHash, Vec<u8>)> }`.
- Driver `infer_field` (`:254-284`): walks `F::plan`, computes `dep_hash` from `dep_input` and parent hashes, consults the cache, computes on miss. With no cache it is a plain recompute (cache-transparency law).
- Gate `infer_field_after_diff` (`:289-315`): if `!diff.touches().intersects_any(F::reads())` and the session holds a result, return it; otherwise run `infer_field` with the cache and store the result.

Two correctness notes in this file:

- `:303-313`: the session "root" is `merkle_collection` over the **indices** `"0","1",…` of the result keys, not over values. It is stored at `:313` and never compared against anything (the gate at `:295` reads only `(_, bytes)`). The gate therefore relies only on `touches ∩ reads`, which is what the LAW in `spr/🎮️command/🦀️.rs:80-82` requires, but the stored hash is dead.
- `:260, :268, :275`: `filter_map` silently drops parents missing from `hashes`/`values`. A plan-order violation would fold fewer parents and pass fewer parent values to `compute`, which then reads `parents.first()` (see puzzle `flat-position`). This fails silently rather than loudly.

### 1.3 `Inference<P>::infer` and the lifecycle

Lifecycle as implemented:

1. **Snapshot**: an artifact's document snapshot (`CadSnapshot`, `Generation3dSnapshot`, `Puzzle3dSnapshot`, `SemioDrawingSnapshot`, `GltfSnapshot`, …).
2. **Infer**: `impl protocol::Inference<S> for XInference { fn infer(&S) -> Result<Self, ValueError> }` (pure, synchronous, whole-snapshot). Most `XInference` types are assembled from `InferredField` drivers via `store::infer_field(snapshot, None)`, e.g. drawing `SemioDrawingInference::infer` at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/💡️inferences/🦀️.rs:32-38`.
3. **Publish** (three distinct routes, none stores results):
   - **Descriptor only** (schema catalog): `ArtifactDeclaration::builder(...).inferences([XInference descriptor])` (`OSMOD/🔌️plugin/🦀️.rs:3278`). This registers the five facet leaves for schema tooling. It does NOT register an executable service.
   - **Executable service**: `ArtifactInferenceService::new(metadata, fn)` / `new_contextual(...)` registered via `.inference_services([...])` (`OSMOD/🔌️plugin/🦀️.rs:3292`). Only five such services exist in the tree: gis `gismap` (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:174,348`), cad `aec-building` extension (`✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🦀️.rs:139,176`), stdio `gltf` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs:112,121,195,215`), print `chart` (`OSMOD/../📓️print/🦀️.rs:60-68`), flow brep `geometry` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/💡️inferences/📐️geometry/🦀️.rs:35-47`, registered at `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs:2137-2139`).
   - **Query/result**: results are returned as canonical bytes to the caller (`ArtifactInferenceExecution.canonical_payload`, `OSMOD/🔌️plugin/🦀️.rs:1620-1627`). Optional commit: `ArtifactInferencePayloadContract.commit` turns a result into a proposal that is applied through the normal edit path (`OSMOD/🌉️mcp/💡️inference/🦀️.rs:7-11`). No persistence layer stores inference values.
4. **Consume**: the agent/MCP side reads `inference_list`, `inference_get`, `inference_submit`, `inference_events`, `inference_cancel`, `inference_approve` (`OSMOD/🌉️mcp/💡️inference/🦀️.rs:1-16`) and the resource `semio://artifact/{id}/inference/{field}`. The desktop shell holds a per-document inference port (`OSMOD/🖥️shell/🦀️.rs:322-328`, `ShellHost/🟦️.tsx:2583-2584, 3164-3169`). **No renderer or window code reads an `XInference` value**; the grep hits in `PluginRuntime/🟦️.tsx` and `Shell/🟦️.tsx` are English prose, not code. Windows read the document snapshot and the retained preview transient instead.

### 1.4 Cache invalidation via DiffRegions: what actually runs

- `DiffRegions` implementors in the tree: `SpaceHistoryDiff` (`OSMOD/🏪️store/🦀️.rs:28015`), `ChartDiff` (`OSMOD/../📓️print/🧬️schema/🔀️diff/🦀️.rs:136`), plus test types. CAD, puzzle, generation3d, drawing, mesh, brep and gltf do **not** implement it (`spr/🎮️command/🦀️.rs:80-82` says adoption is per-type via a `POLICY_DIFF_REGIONS` allowlist; not checked).
- Callers of `infer_field_after_diff`: none outside the module's own tests.
- Production callers of `infer_field` (searched across `✏️s/🔌️plugins` and `OSMOD`/`framework`, excluding tests): every one passes `None`:
  - drawing `🖊️drawing/🧬️schema/💡️inferences/🦀️.rs:35`
  - stdio mesh `🔺️mesh/🧬️schema/💡️inferences/🦀️.rs:39`
  - stdio brep validation `🧊️brep/🧬️schema/💡️inferences/🦀️.rs:117`
  - math equation `➗️equation/.../🌱roots/🦀️.rs:180` (`protocol::infer_field(..., None)`)
  - The only `Some(cache)` call is the driver itself (`OSMOD/💡️inference/🦀️.rs:302`) and tests.
- Result: **the DepHash cache and the tier-1 gate are not active anywhere in production.** The "incremental" behaviour exists only in unit tests (see 1.6).
- `OSMOD/🛢️db/🔍️query/🦀️.rs:167-169, 677, 1382-1417` documents a `QueryResultField: InferredField` / `pack::infer_field` path with an `InferenceCache`. There is no `QueryResultField` type or impl in that file (grep shows doc-comment hits only). **Doc drift.**

### 1.5 The one real DAG-cache consumers that do run (with cache off)

- drawing `flattenedScene` (`🖊️drawing/🧬️schema/💡️inferences/🎛️flattened-scene/🦀️.rs`, 179 lines): `impl store::InferredField` at `:122`, `reads = [layers, styles]` at `:129-131`, per-node key `"<layer>:<p0>.<p1>…"`, parent transform composed in `compute`. Runs via `infer_field(None)`, i.e. a pure per-node recompute on each call.
- Puzzle3d `flatPosition` (section 2.5).
- Equation `roots` (`➗️equation/.../🌱roots/🦀️.rs:180`, `protocol::infer_field(None)`).

### 1.6 Tests that exist for the cache laws

- `OSMOD/💡️inference/🧪️tests/🔬️unit/🦀️.rs` (251 lines): `infer_field_computes_expected_values_over_the_dag`, `disabled_cache_matches_pure_recompute`, `cold_and_warm_cache_match_pure_recompute`, `tiny_budget_eviction_storm_still_matches_pure_recompute`, `changing_a_leaf_weight_only_recomputes_that_leaf_and_its_descendants`, `changing_the_root_weight_recomputes_the_entire_subtree`, `identical_snapshot_recompute_is_all_cache_hits`, `schema_version_bump_yields_zero_hits_on_an_otherwise_warm_cache`.
- Puzzle flat-position unit tests: `changing_a_leaf_own_vortex_does_not_recompute_ancestors`, `changing_the_root_position_recomputes_the_whole_chain`, `changing_an_attraction_center_param_never_touches_the_plane_chain`, `disabled_cache_matches_pure_recompute`, `typed_dependency_matches_neutral_serde_oracle` (`PUZINF/📍️flat-position/🧪️tests/🔬️unit/🦀️.rs`).
- Not run (read-only audit; no cargo per instructions).

---

## 2. Schema-first layout of `💡️inferences/` and the codegen question

### 2.1 Layout convention (all families)

```
<artifact>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/
  🦀️.rs            family root: #[derive(ToValue, FromValue, ArtifactSchema)] struct XInference { #[derived] field: … }
                    impl protocol::Inference<Snapshot>, impl protocol::InferenceSpec<Snapshot>,
                    fn x_artifact_inference_descriptor() { FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") } }
                    mod tests (#[path = "🧪️tests/🔬️unit/🦀️.rs"])
  🟦️.ts  🔗️.graphql  🔣️.json  🛰️.proto   facet leaves (5 formats, one descriptor)
  <emoji><slug>/    one named inference per slug dir: own 🦀️.rs (+ 🟦️.ts leaves), mounted only by the family root
  🧪️tests/🔬️unit/🦀️.rs
```

Mounting is by `#[path]` only, from the artifact root: `GENINF` is mounted at `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs:211-220` (`#[path = "."] pub mod inferences { … pub mod topology { #[path = "…/🧭topology/🦀️.rs"] … } }`).

### 2.2 CAD (`CADINF`)

- `🦀️.rs:16-26` `CadInference { #[derived] object_count: usize, vertex_count: usize, bounds: Option<CadBounds> }`, `#[artifact_schema(id = "s.cad.cad.inference")]`.
- `:28-35` `infer` = `object_count(snapshot)`, `vertex_count(snapshot)`, `scene_bounds(snapshot)`.
- `:37-51` `InferenceSpec`: three fields `s.cad.cad.inference.{objectCount,vertexCount,bounds}`, each with `reads: [shapeModel, buildingModel, energyModel, structureClassicModel]`.
- `:57-62` descriptor `include_str!`s the five leaves.
- `📦bounds/🦀️.rs` is a stub: `scene_bounds` returns `None` unconditionally (`:27-29`), `vertex_count` returns `0` (`:37-40`), `object_count` counts the four fixed model-child SLOTS (`:33-35`). The module doc says real bounds need the composed children (`:4-11`).
- **Mixed content**: `CADINF/🦀️.rs` is 1042 lines. Lines 76-421 (`mod derive_transformation`) and 422-571 (`mod construct_query`) and 572-1042 (`mod scene_compute`, incl. `cad_brep_kernel` `:630`, `typology_brep_mesh` `:636`, `pane_world_solids` `:679`, `export_mesh_from_scene` `:983`) are geometry/kernel code that is **not** on the `infer` path. They sit inside the inference family root (see 4.4).
- Leaves: `🟦️.ts` (1520 lines) is the full schema TS (interfaces plus `// #region` blocks for the query DSL), `🔣️.json` declares `x-semio-derived` on the three fields, `🔗️.graphql` uses `@derived`, `🛰️.proto` uses a `// @derived` comment.

### 2.3 Generation3d (`GENINF`), family root `🦀️.rs` (67 lines)

- `:14-20` `Generation3dInference { #[derived] topology: Generation3dTopology }`.
- `:22-29` `infer` = `compute_generation3d_topology(snapshot)`.
- `:31-41` `InferenceSpec`: `s.procedural.generation3d.inference.topology`, `reads: ["hostSnapshot"]`.
- `:48-55` descriptor; unit tests at `🧪️tests/🔬️unit/🦀️.rs` (determinism; empty document is empty and cycle-free; linear chain).
- Slug `🧭topology/🦀️.rs` (81 lines): Kahn topological sort over `hostSnapshot.widgets` / `synapses` (`:42-80`); outputs `nodeCount, edgeCount, topoOrder, depth, cycleFree` (`:17-23`). **No geometry.** Its doc says it covers `fixture`'s graph while the field reads `hostSnapshot`, a terminology drift.
- Leaves: `🟦️.ts` (79 lines) contains interfaces **and** a systematic runtime parser block (`proceduralGeneration3dInferenceGuard*`, `parseGeneration3dInference`, `parseGeneration3dTopology`), `🔗️.graphql` (`@derived`), `🔣️.json`, `🛰️.proto`. The `🟦️.ts` guard block is the only one of the five checked that carries a parser; it looks machine-emitted (see 2.6).

### 2.4 Flow brep extension (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/💡️inferences/📐️geometry/`)

This is not a family root; it is an **executable service leaf** with a payload contract:

- `🦀️.rs:6-7` `GEOMETRY_INFERENCE_SCHEMA = "s.flow-extension-brep.geometry"`, `GEOMETRY_ARTIFACT_KIND = "s.flow.flow"`.
- `:25-32` `GEOMETRY_INFERENCE_CONTRACT`: `payload_schema_id`, `input_schema: include_str!("📥️request.json")`, `output_schema: include_str!("📤️result.json")`, `progress_unit: "operator-step"`, `artifact_binding: None`, `commit: None`.
- `:35-47` `geometry_inference_service()` = `ArtifactInferenceService::new_contextual(metadata{owner: "flow-extension-brep", artifact_kind: "s.flow.flow", inference_schema: "s.flow-extension-brep.geometry", …}, infer_geometry)`.
- `:60-124` `infer_geometry`: budget guard `:64-66`, requires a `brep.` operator `:69-71`, refuses referenced geometry without dependencies `:85-87`, passes `budget`/`round_units` = `work_units` and `cancellation_id` `:107-109`, cold cache cancels the node `:110`, bypass zeroes node hash `:111`, calls `flow_extension_sdk::evaluate_step_envelope` `:115`, returns `complete` and `validity` `:116-123`.
- Fixture `🧫️fixtures/🔣️.json`: language-agnostic law (`operatorId: brep.prim3d.box`, budget 1, expected `done/complete/phase`).
- Its artifact is `s.flow.flow`, not generation3d.

### 2.5 Puzzle3d `flatPosition` (`PUZINF`)

- Family root `🦀️.rs:19-31`: `Puzzle3dInference { #[derived] flat_positions: BTreeMap<String, FlattenPose> }`. `infer` (`:24-30`) = `flatten_snapshot(snapshot).into_iter().collect()`. **The InferredField chains are not used by `infer`.**
- `InferenceSpec.fields()` (`:40-45`) declares `s.puzzle.puzzle3d.inference.flatPosition.plane` and `…flatPosition.center` (both `reads: [objects, attractions]`). The struct field is `flat_positions` (`flatPositions`). The declared ids and the struct do not line up.
- `📍️flat-position/🦀️.rs` (177 lines): two `store::InferredField<Puzzle3dSnapshot>` impls: `Puzzle3dFlatPlane` (`:30-111`) and `Puzzle3dFlatCenter` (`:117-170`). Separate chains by design (`:113-116`).
  - `plan` = BFS order from `flatten_objects_with_assignment` (`:43-55`).
  - `dep_input` (`:57-86`) and `compute` (`:88-110`) each call `assignment_for(snapshot)` (`:19-21`), which reruns the whole BFS. The file's own comment (`:15-18`) admits this is O(n²) per pass.
- `🗜️flatten/🦀️.rs`: `FlattenPlane` (`:15-19`), `flatten_snapshot` (`:272-274`), `flatten_objects_with_assignment` (`:284`, returns order plus parent assignment).
- `PUZINF/🧪️tests/🔬️unit/🦀️.rs:48` `inference_matches_flatten_snapshot_directly`: confirms `infer` is the direct flatten, not the chains.
- The 5d variant `🖐️5d/…/🎛️flat-position/🦀️.rs` likewise calls `flatten_objects` directly (`:92`) and `flatten_snapshot` (`:107`).

### 2.6 Stdio semio artifact inferences (samples)

- `…/🧿️semio/…/🖊️drawing/…/💡️inferences/🦀️.rs` (family root): `infer` via `infer_field(None)` (see 1.3).
- `…/🧿️semio/…/🔺️mesh/…/💡️inferences/🦀️.rs:38-40`: `aabb` via `infer_field(None)`; `📦aabb/🦀️.rs` is the field.
- `…/🧿️semio/…/🧊️brep/…/💡️inferences/🦀️.rs:117`: `validation_report` = `infer_field(None)`, "Pure computational geometry validation belongs to `semio_framework_3d::brep::queries::validation`" (`:2`).
- `…/🧊️gltf/…/🚪️io/💾️binary/💡️inferences/` is a codec ("leaf envelope"), not an inference field; `🧊️gltf/🦀️.rs:215` is the only production `…Inference::infer` call found outside tests (through the gltf service).

### 2.7 Which files are generated vs hand-written

| File | Status | Evidence |
|---|---|---|
| family root `🦀️.rs`, slug `🦀️.rs`, `🧪️tests/…` | Hand-written (source of truth) | structure, `include_str!`, `#[derived]` |
| `🛰️.proto`, `🔗️.graphql`, `🔣️.json` | Hand-authored leaves | `OSMOD/../📇️registry/🦀️.rs:71` ("Five handcrafted leaf bodies"); AGENTS.md "handcraft all assets" |
| `🟦️.ts` (cad, puzzle, flow) | Hand-authored interfaces | no generated marker |
| `🟦️.ts` (generation3d) | **Looks machine-emitted** (parser guard block with a fixed naming template) | no `GENERATED` header; the emitter was not found (see below) |
| `🤖️generated/` (puzzle `🧊️3d`) | Generated: manifest registry and `🔠️types` | `puzzle3d/🤖️generated/📇️registry/🦀️.rs` header `// Generated manifest registry`; the only generator found is `📜️script.ts generate plugin-glue` (`📜️script.ts:574-604`), which writes subset inventories, not inference leaves |

**Codegen tool**: I did not find an emitter for the facet leaves or the TS guard block. Searched: `OSMOD` and `🧰️framework` for `GuardReject`, `GuardObject`, `Refusal extends Error`, `x-semio-derived` emitters, `FacetLeaves {` emitters, and the root `📜️script.ts` `generate` subcommands (`taxonomy`, `plugin-glue`, `scale-fixture` only). **Unverified** whether the guard block is produced by an unlisted tool or by hand.

What validates the leaves instead (drift checks, not codegen):

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/`: `derivation-presence`, `slug-leaf-presence`, `family-root-completeness`, `assembly-coverage`, `emoji-uniqueness`, `source-admission`, `state-separation`, `aggregate`.
- `…/🧬️schema/🔍️field-discovery/` extractors per format (`🦀️rust`, `🟦️typescript`, `🔣️json-schema`, `🔗️graphql`, `🛰️protobuf`) feeding `⚖️comparison` (field-by-field diff of the five leaves against the Rust source).
- `🧰️framework/🔨️modules/🧬️schema/🧪️tests/📤️schema-export-entries/` (Rust catalog dump checked against `🧫️fixtures/📤️schema-export-entries-dump.json`).

Note: `🛰️.proto`'s own comment in the CAD leaf (`// @derived`) and the `x-semio-derived` in JSON are the markers the derivation law checks.

---

## 3. Expensive inferences: async, jobs, progress, cancellation, budgets, wasm

### 3.1 Inference-level budget and validation

- `WireArtifactInferenceBudget { allocation_bytes, work_units, recursion_depth }` (`OSMOD/🔌️plugin/🦀️.rs:2010`), required non-zero (`:2133-2148`).
- `validate_wire_request_resources` (`:2133`): cache-mode vs previous-state rules (`:2150-2161`), dependency owners unique (`:2164-2175`), request bytes ≤ `allocation_bytes`, work = 1 + dependencies ≤ `work_units` (`:2182-2185`).
- `validate_inference_execution` (`:2190-2`): result bytes + diagnostics ≤ `allocation_bytes`; `actual_cache_mode` must equal the requested one.
- Cancellation registry: `begin_artifact_inference` claims the id (`:2117-2122`, in-flight refusal), `inference_cancelled` reads the cancelled set (`:2125-2130`), `wire_artifact_infer_with_service` checks it before executing (`:2247-2265` region).

### 3.2 The `semio.infer` cold job (`…/⚛️reactor/💼️jobs/💡️infer/🦀️.rs`)

- `job_infer` (`:212-222`): if `ActionBus::production()` has a factory for `semio.infer/<inference_schema>`, admit an `InteractiveInferenceJob` (persistent worker session); otherwise a `TwoPhaseBoundedJob` that calls `crate::app::wire_artifact_infer` (`:232-234`).
- Work-unit prices (`OSMOD/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs:112-122`): `WORK_UNITS_EXECUTE = 1_024`, `WORK_UNITS_PUMP = 64`, `WORK_UNITS_VALIDATE = 1`, `WORK_UNITS_RETIRE = 1`.
- Interactive state machine (`:249-257`): `Dispatch -> Pump -> OutcomeClose -> SessionClose -> RejectedClose -> Complete`. Each state is one step-job opportunity. `pump` spends the granted fuel (`:428-450`), and `pump_transition` checks cancellation first (`:452-454`).
- Step gating: `step` refuses a state whose price exceeds the granted fuel (`:619-626`, `job.infer.budget-exhausted`). `cancel()` sets the flag and cancels the token (`:638-641`).
- User-visible lane budget used for the step (`OSMOD/🔨️modules/🧵️job/🦀️.rs:396-397`): `USER_VISIBLE_LANE_WALL_US = 2_000` (2 ms), `USER_VISIBLE_LANE_FUEL = 6_000_000`. The infer job passes `fuel_per_step = budget.work_units.clamp(1, USER_VISIBLE_LANE_FUEL)` and `step_budget_us = USER_VISIBLE_LANE_WALL_US` (`:391-400`).
- Progress channels (`:13-184`): `Scheduled` marker; `Preview` (single slot, latest coalesced, 1 MiB cap, `:122-133`); `Diagnostic` ring (32 items / 64 KiB, `:162-175`); checkpoint is one retained slot (`:288`, `:492-494`). The lossless checkpoint ring (`LOSSLESS_MAX_ITEMS`, `publish_lossless`) is **test-only** (`:15-16, 135-160`). Progress unit is declared per payload contract (`"operator-step"` for brep geometry).
- Result size guard `encode_result` (`:672-`), allocation-bounded.

Host crossing: `OSMOD/🔌️plugin/🖥️host/🦀️.rs:5967-5968` (`semio.infer` on a worker, child cancel token). The MCP route runs it on a headless inference guest: `OSMOD/🌉️mcp/🏠️workspace/🦀️.rs:896` (`headless_inference_budget`), `:1702` (`route.router.infer`).

### 3.3 Geometry DAG evaluation (the closest existing pattern)

- The **flow graph** (`FlowHostSnapshot` widgets and synapses) is evaluated by the neural engine's `Evaluator::evaluate_channels_budgeted` (`OSMOD/🧠️neural/⚙️engine/🦀️.rs:2346`) under `EvalStepBudget { dispatches, deadline }` (`:2211-2229`; `PROBE`, `UNBOUNDED`, `dispatches(n)`, `until(n, now, deadline)`).
- Host budget for one flow tick: `OSMOD/🌊️flow/🖥️host/🦀️.rs:3012-3016` `flow_eval_tick_budget` = `dispatches(FLOW_EVAL_TICK_STEP_BUDGET)` or `until(…, started + FLOW_EVAL_TICK_ELAPSED…)`.
- Operators `brep.*` are not compiled into the guest. They are contributed at runtime (`preview-eval/🦀️.rs:15-18`) and executed by the geometry extension actor.
- The inference-shaped wrapper of that DAG is the brep geometry service (section 2.4). It evaluates **one operator per call**, reuses resumable state through `nodeHash`/`resume`, and does not walk the graph itself. The walk is driven by the preview tick. So it is the closest analogue, not a DAG-inference.

### 3.4 CAD brep meshes from its document

- CAD builds meshes synchronously in-process: `cad_brep_kernel()` (`CADINF/🦀️.rs:630`), `typology_brep_mesh` calls `kernel.tessellate(&handle, 0.1)` (`:647`), `pane_world_solids` (`:679`), `export_mesh_from_scene` (`:983`), `run_derive_from_geometry` (`:300`, boolean fuse at `:273`).
- Callers: CAD editor `✏️s/🔌️plugins/📐️cad/…/✏️editor/🦀️.rs` and its tests; `…/🧬️schema/📐️geometry/🦀️.rs`. No `ArtifactInferenceService`, no job, no budget, no progress, no cancellation on this path.
- Also `CADINF/🦀️.rs:124-125` `face_mesh_analytics` tessellates each face at tolerance 0.1 for centroid/normal (`:159-167`).

### 3.5 Wasm guest execution

- Preview/evaluation: the flow session tick runs in the generation3d wasm guest (`preview-eval/🦀️.rs:15-18`: "an in-guest tick can only ever fault" when used synchronously). Operators run on the host extension actor. Tessellation is a `tessellate` `ExtensionInvocation` to the geometry extension (`:839-880`).
- Brep geometry inference: `ExecutionMode::Linked` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs` owned bundle, `bundle.mode(ExecutionMode::Linked)`), so the operator runs native in the host, not in wasm.

---

## 4. generation3d: what the inferences contain and what bypasses them

### 4.1 Current `💡️inferences` content

Only one field: `topology` (`GENINF`). Contents: `nodeCount, edgeCount, topoOrder, depth, cycleFree` (Kahn sort over `hostSnapshot` widgets/synapses). **No geometry, no tessellation, no mesh, no budget.**

### 4.2 Is `topology` executed at all?

- `compute_generation3d_topology` is called only from `Generation3dInference::infer` (`GENINF/🦀️.rs:25`), and that is called only from unit tests.
- Artifact declaration `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs:113-117, 150`: the inference is a capability with a descriptor (`.inferences([descriptor])`) and **no** `.inference_services([...])`. Result: it is a schema-catalog entry, not an executable service. A submit would hit `artifact-inference.not-registered` (`OSMOD/🔌️plugin/🦀️.rs:1770-1771`).
- Consequence: the same is true for CAD, puzzle3d, drawing and flow topology. Their `Inference::infer` impls are executed only by tests or by a service that wraps them (gltf). **Unverified** at runtime (no app run, per instructions).

### 4.3 Bypass list: generation3d geometry evaluation does NOT go through the inference system

| # | Path | Where | What it bypasses |
|---|---|---|---|
| B1 | `previewEval` tool-run job (`⏯️tool-run`) runs a chain of hops, each `flowEvalTick` addressed at a preview window | `preview-eval/🦀️.rs:1-25, 57, 143-155` | `Inference<P>`, `ArtifactInferenceService`, `semio.infer` job, inference cancellation registry |
| B2 | Operator evaluation: `ExtensionInvocation(address, "evaluate", …, "flowEvalResolve")` for each pending operator | `preview-eval/🦀️.rs:946-996` (`evaluate_tick`), budget `EVALUATE_STEP_BUDGET = 8` dispatches, `EVALUATE_STEP_WALL_MICROS = 2_000_000` (`:134, :141`) | inference budgets (`WireArtifactInferenceBudget`); uses extension request fields instead |
| B3 | Mesh production: `ExtensionInvocation(geometry_address, "tessellate", …, "flowTessellateResolve")` with `budget = PREVIEW_TESSELLATE_STEP_BUDGET = 24`, `wallMicros = mesh::TESSELLATE_STEP_WALL_MICROS` | `preview-eval/🦀️.rs:305, 314, 839-880` | inference payload contract; the chunk/resume is a session-local `nodeHash` ledger |
| B4 | Abort/restart release: `release_invocations_for` sends `evaluateCancel` and `tessellateCancel` to the extension address | `preview-eval/🦀️.rs:1119-1131`; resolve at `:1136-1147` | `inference_cancel` / `ArtifactInferenceCancellationGuard` |
| B5 | Editor flow tick (`flow-eval-tick` command) and generate-mode patched fixture (`generation_host_snapshot_for`) | `…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:57-92` | same as B1; generate mode evaluates a patched copy of the host snapshot |
| B6 | Inline continuation: an answer's fold runs the next walk INSIDE the same guest turn | `…/⏱️flow-eval-tick/🦀️.rs:106-120` (`continue_inline`), gated by `inline_continuation_admitted` | job scheduling entirely |
| B7 | Viewer preview uses the same chain (shared module) | `…/👁️viewer/` (`🧪️tests/🔬️unit`, `preview-eval` shared, per module doc `preview-eval/🦀️.rs:20-25`) | same as B1 |
| B8 | Brep geometry inference service exists but is bound to `s.flow.flow` and is not invoked by generation3d | `…/💡️inferences/📐️geometry/🦀️.rs:35-47` | generation3d calls the extension through B2/B3 instead of the service |

Not inspected (**unverified**): generation3d `🚪️io/📤️export` and `🚪️io/📥️import` paths, and the flow-tessellate-resolve/eval-resolve command bodies beyond the 38-line wrappers.

### 4.4 Other notable findings in generation3d and CAD

- The CAD inference folder holds kernel code (section 3.4). The generation3d inference folder holds no geometry, so the "inferences" directory does not describe geometry in either plugin.

---

## 5. Recipe: adding a new inference to an artifact

Use the pure-derived pattern for cheap whole-snapshot values, and the service pattern for anything that needs budgets, cancellation or resume.

1. **Schema family root** `<artifact>/…/🧬️schema/💡️inferences/🦀️.rs`
   - Add a field to the `XInference` struct with `#[derived]` (`GENINF/🦀️.rs:17-20` pattern).
   - Extend `impl protocol::Inference<S>::infer` to populate it.
   - Extend `impl protocol::InferenceSpec<S>::fields()` with `InferenceFieldSpec { id: "s.<…>.inference.<field>", reads: &[…] }`. The `reads` list is the tier-1 read set (`spr/🎮️command/🦀️.rs:87-93`). Keep it complete: an uncovered read silently serves stale values if a cache is ever enabled.
   - If the field is per-entity with dependencies, implement `store::InferredField<S>` in a slug dir (`PUZINF/📍️flat-position/🦀️.rs` pattern) and call `store::infer_field::<S, F>(snapshot, None)` from `infer` (pass `Some(&mut cache)` only when the cache is actually wired, which it is not today).
2. **Slug dir** `<emoji><slug>/🦀️.rs` (+ `🧪️tests/🔬️unit/🦀️.rs`), mounted only by the family root with `#[path]` (`GENINF/🦀️.rs:64-67`, `…/🦀️.rs` artifact root at `:211-220`).
3. **Facet leaves** `🟦️.ts`, `🔗️.graphql`, `🔣️.json`, `🛰️.proto` in the family dir: hand-written, same field set, `@derived` / `x-semio-derived` / `// @derived` markers (see the laws listed in 2.7).
4. **Descriptor**: `<x>_artifact_inference_descriptor()` with five `include_str!` (`CADINF/🦀️.rs:57-62`), then `ArtifactDeclaration::builder(...).inferences([…])` in the artifact root (`GENINF/../🦀️.rs:150`).
5. **Executable service** (only if it must be invocable through `semio.infer` / MCP):
   - Write `pub const fn my_inference_service() -> ArtifactInferenceService { ArtifactInferenceService::new(ArtifactInferenceServiceMetadata { owner, artifact_kind, artifact_schema, artifact_schema_version, inference_schema, inference_schema_version, algorithm_version, policy_version, payload: Some(ArtifactInferencePayloadContract { payload_schema_id, input_schema: include_str!("📥️request.json"), output_schema: include_str!("📤️result.json"), progress_unit: "…", artifact_binding: None, commit: None }) }, infer_fn) }` (pattern: `…/💡️inferences/📐️geometry/🦀️.rs:25-47`, and `OSMOD/🖨️…/📓️print/🦀️.rs:60-68`).
   - Add `.inference_services([my_inference_service()])` to the artifact contribution (`gismap/🦀️.rs:348`).
   - Expensive operations: take `request.budgets.work_units` as the step budget, check `inference_cancelled(request.cancellation_id)` between steps, return `complete: false` with a resumable `previous_state` for the next call (`ArtifactInferenceExecution`, `OSMOD/🔌️plugin/🦀️.rs:1620-1627`).
   - Declare `progress_unit` and `Cold`/`Incremental`/`Bypass` support (`validate_inference_execution` refuses a mismatched `actual_cache_mode`).
6. **Tests**
   - Unit laws (family-level): determinism (`GENINF/🧪️tests/🔬️unit/🦀️.rs:28-31`), empty-identity (`:42-49`), a fixed linear-chain expectation (`:51-60`).
   - Cache laws if `InferredField` is used: copy the `OSMOD/💡️inference/🧪️tests/🔬️unit/🦀️.rs` set (disabled == pure, cold == warm, eviction storm, schema-version salt).
   - A language-agnostic fixture for the service, as in `…/📐️brep/💡️inferences/📐️geometry/🧫️fixtures/🔣️.json`. The repo rule (AGENTS.md) also asks for the same output checked against at least one third-party library; none of the existing inference tests does this.
   - Run the `🦑️repo` inference laws (2.7) to confirm the five leaves match the Rust source.
7. **How a viewer window reads it**
   - Windows do not read `XInference` values (section 1.3). A window reads the document snapshot through `ArtifactView` and its retained preview transient.
   - To display an inference: either (a) the agent/MCP path (`inference_submit` -> `inference_events` -> `inference_get`, or the `semio://artifact/{id}/inference/{field}` resource), or (b) the shell's per-document inference port (`ShellHost` `inferencePortOwnerRef`, `:3166-3169`), which opens an installed-service owner through `InferencePortOpeningMailboxV1` (`💡️inference/🚪️opening/🟦️.ts`).
   - **Unverified**: the full data flow from port to a viewer pane was not traced.

---

## 6. Summary of defects and drift found (read-only)

| # | Where | Finding |
|---|---|---|
| D1 | `OSMOD/💡️inference/🦀️.rs:303-313` | Session root hash is built over indices, not values; stored and never compared. Dead. |
| D2 | `OSMOD/💡️inference/🦀️.rs:260, 268, 275` | Missing parents are silently dropped (`filter_map`); a plan-order violation misaligns `compute`'s `parents`. |
| D3 | `OSMOD/💡️inference/🦀️.rs:212-217` | LRU `touch` is O(n) per cache hit. |
| D4 | `OSMOD/💡️inference/🦀️.rs:206` | `insert` on an existing key does not reconcile `used_bytes`. Unreachable from `infer_field` today (inserts only follow misses). |
| D5 | `OSMOD/💡️inference/🦀️.rs:136-139` | `InferencePersistence::Projection` is defined and unused. |
| D6 | All production `infer_field` call sites | Pass `None`: the DepHash cache and the tier-1 gate are inactive in production. `infer_field_after_diff` has no callers. |
| D7 | `PUZINF/🦀️.rs:19-31, 40-45` | `Inference::infer` bypasses the `flatPosition` chains and calls `flatten_snapshot` directly; spec ids (`flatPosition.plane/center`) do not match the struct (`flatPositions`). |
| D8 | `PUZINF/📍️flat-position/🦀️.rs:19-21, 57-58, 88-89` | `assignment_for` reruns the full BFS in every `dep_input` and `compute`; O(n²) per pass (self-declared). |
| D9 | `CADINF/📦bounds/🦀️.rs:27-40` | `bounds` is always `None`, `vertexCount` always `0`; `objectCount` counts four slots. Stub. |
| D10 | `CADINF/🦀️.rs:76-1042` | Kernel and mesh code (not inference) lives inside the inference family root. Layering smell. |
| D11 | `GENINF/🦀️.rs:150` and `…/🧭topology/` | Descriptor-only: listed in the schema catalog, no executable service, not invocable; `topology` doc says `fixture`, field reads `hostSnapshot`. |
| D12 | `OSMOD/🛢️db/🔍️query/🦀️.rs:167-169, 1382-1417` | Documents an `InferredField`/`InferenceCache` path that does not exist in code. Doc drift. |
| D13 | `OSMOD/💡️inference/🔌️service/🟦️.ts` | Lives under `💡️inference` but is the installed-service protocol (not inference). Naming drift. |
| D14 | `OSMOD/💡️inference/🦀️.rs:249` and `…/🧪️tests` | The cache module imports its codecs from `os_io::text::inferences`; tests need that path too. Not a defect, but a cross-module coupling to note. |
| D15 | `OSMOD/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs:15-16, 135-160` | Lossless checkpoint channel is `#[cfg(test)]`; production keeps only one checkpoint slot. |

---

## 7. Open items (not verified in this audit)

- No runtime check of any claim (no build, no test, no app run; per read-only instructions).
- The emitter for the generation3d `🟦️.ts` parser block was not found.
- Whether any viewer pane consumes an inference value at runtime was not traced end to end.
- `preview-eval` flow-tessellate-resolve / flow-eval-resolve bodies and the `TESSELLATE_STEP_WALL_MICROS` value were not read.
- The `POLICY_DIFF_REGIONS` allowlist (mentioned in `spr/🎮️command/🦀️.rs:75-79`) was not located.
