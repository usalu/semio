# R1 Explore Framework Inferences (BIM Plugin Input)

Ticket: `26/10/08/BIM-PLUGIN` (read-only exploration, no repo files modified).
Repo: `C:\git\semio`. Verbatim code below is spliced from the source files at generation time.

## 0. Verdict

- The framework HAS an inference abstraction. It has three layers:
  1. A typed, pure, whole-snapshot trait `Inference<P>` plus metadata trait `InferenceSpec<P>` (`protocol` crate, `📡️spr/🎮️command`).
  2. A per-field, per-entity engine `InferredField<P>` with a merkle dependency-hash cache, a resumable cursor/fuel driver, cancellation and an optional diff gate (`💡️inference` module, re-exported as `store::infer_field` and `protocol::infer_field`).
  3. An artifact-level registration and execution layer: `ArtifactInferenceDescriptor` (schema facets), `ArtifactInferenceService` (executable, versioned, budgeted, cancellable), the plugin manifest roster `ContributedInferenceMetadata`, and the MCP `inference_*` job quartet (guest or hub execution, optional proposal-then-approve commit).
- Evaluation is LAZY and ON REQUEST. Grep found no call from the store dispatch/apply path to `infer` after a diff. The only diff-driven path is `infer_field_after_diff` (tier-1 read-region gate), and no production caller of it was found (only tests).
- Every derived value must be declared with `#[derived]` in the inference schema, never stored in the snapshot. That is the existing pattern in gismap, equation and gltf.
- The framework has NO geometric constraint solver and NO B-Rep-dependent inference that caches handles. Those gaps matter for BIM (section 5).

## 1. Locations (real paths)

Framework trait and types:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` lines ~27-104: `Inference<P>`, `TouchedPaths`, `DiffRegions`, `InferenceFieldSpec`, `InferenceSpec<P>`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs` (559 lines): engine. Key items: `DepHash`, `InferredField<P>` (line ~131), `InferenceCacheConfig` (default `enabled: false`), `InferenceCache` (LRU, byte budget), `InferenceSession`, `InferenceCursor`, `infer_field_step`, `try_infer_field`, `infer_field`, `infer_field_after_diff`.
- Engine tests: `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🧪️tests/🔬️unit/🦀️.rs`, fixtures `🧫️fixtures/🪜️stepped-driver/🔣️.json`, `🧫️fixtures/🌱️owned-dependencies/🔣️.json`.
- Artifact schema registry: `🧰️framework/🔨️modules/🧬️schema/📇️registry/🦀️.rs` lines ~286-300 (`ArtifactInferenceDescriptor { id, inference: FacetLeaves }`), ~350-394 (`ArtifactInferenceRegistry`), ~503 (Descriptor impl), ~551-598 (register and catalog accessors).
- Plugin service: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` lines ~1495-1700 (`ArtifactInferenceServiceMetadata`, `ArtifactInferencePayloadContract`, `ArtifactInferenceExecution`, `ArtifactInferenceService`), ~1584 (`ArtifactInferenceExecutionRequest`), ~1740-1810 (`ArtifactInferenceServiceRegistry`, `register_artifact_inference_service`), ~2019 (`WireArtifactInferenceCacheMode`).
- Manifest roster: `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` lines ~6515-6620 (`ContributedInferenceMetadata`, `InferencePayloadContract`, `InferenceCommitBinding`, `InferenceArtifactBinding`, `ArtifactContributionDescriptor.inferences`).
- Host, MCP and jobs: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs` (discovery plus submit/events/cancel/approve, hub relay), `.../💡️inference/💼️jobs/🦀️.rs` (one job model: guest or hub, proposal, approval), `🌉️mcp/🏠️workspace/🦀️.rs` ~1677 (`infer_real`, synchronous `inference_run`).
- Print product (small reference): `🧰️framework/🛍️products/📓️print/🦀️.rs` (registration plus `execute_chart_inference_controlled`).
- Store test harness: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` ~30296 (`SubsetRoundtripSpec::infer`), ~30321 (`assert_inference_determinism`), ~30358 (S6 determinism stage). `store::infer_field` is the engine re-exported through `store` (used by wfc tests).
- Plugin-level inference catalogues (each named inference is a `💡️inferences/<emoji><slug>/` child):
  - Equation (best full example, quoted in section 3A): `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/` (`🧭topology/`, `🌱roots/`).
  - GIS map (best derived-scalar example, quoted in section 3B): `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/` (`📦bounds/`), plus the port/service side `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/` (MCP bridge, worker, client, fixtures) and editor command `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💡️inference/🦀️.rs`.
  - glTF geometric indicators (richest derived-geometry set, not quoted, too large): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/` with `🏗️dag-assembly/🦀️.rs` (`compute_gltf_inference`: per-part indicators, per-pair clearance/adjacency/contact), `↕️thickness/`, `🧱️area-volume/`, `↔️clearance/`, `🤝️adjacency/`, `🔨️geometry-core/`; service in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs` ~195-215 (`infer_gltf_leaf_cold`, `payload: None`).
  - Flow B-Rep geometry (operator evaluation as inference): `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/💡️inferences/📐️geometry/` (contextual service, request/result JSON, fixture `🧫️fixtures/🔣️.json`).
  - WFC (`store::InferredField` on Solve, Contradiction, Entropy): `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🔨️modules/🏠️host/💡️inferences/🦀️.rs` (lines ~855-938) and `.../🧪️tests/🔬️unit/🦀️.rs`.
  - Print chart admission (validation-style inference, quoted in section 3C): `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🦀️.rs`.

Related framework modules (survey, section 4): `🧰️framework/🔨️modules/⏪️time-travel/🦀️.rs`, `🧵️job/🦀️.rs`, `⏳️async/🦀️.rs`, `🗿️artifact-reference/`, `🕸️graph/`, `🧊️3d/📐️brep/💡️queries/`, `🛢️db/📽️projection/🦀️.rs` (under `💻️os/🔨️modules`), `🧮️math/🎯️sampling/🦀️.rs`.

## 2. Mechanics

### 2.1 Declaration (inputs and output)

- Typed inference: `Inference<P>` takes a snapshot `P` and returns `Self` (a `#[derived]` struct). Bounds: `Clone + Default + ToValue + FromValue` (not serde, by design). LAWS in the doc comment: pure (reads only the snapshot), deterministic, checked (refusals propagate as `ValueError`). `infer` is the single semantics source; every cache path must equal it.
- Metadata: `InferenceSpec<P>` gives `inference_schema_id()`, `schema_version()` (salts cache keys) and `fields()` (a list of `InferenceFieldSpec { id, reads }`, the declared snapshot read-set).
- Per-entity field: `InferredField<P>` declares `Key`, `Value`, `Dependency: ToValue`, `FIELD_ID`, `SCHEMA_VERSION`, `reads()`, `plan(snapshot) -> Vec<InferenceStep{key, parents}>` (topological, roots first), `dep_input(snapshot, key, parents)` (the honesty contract: must cover everything `compute` reads for that key, including the edge data to each parent), `compute(snapshot, key, parent_values) -> Value` (pure), optional `value_bytes` and resumable `compute_step(..., fuel)`.
- Typed output of an artifact inference is a `#[derived]` struct with one field per named inference (equation: `topology`, `roots`; gismap: `position_count`, `route_count`, `region_count`, `bounds`; gltf: `geometry`). The print chart inference has no `#[derived]` field, it returns `chart`, `diagnostics`, `complete`.

### 2.2 Registration

- Schema facets: `ArtifactInferenceDescriptor { id, inference: FacetLeaves { rust, typescript, json_schema, graphql, proto } }` registered via `register_artifact_inference_descriptor` (catalog family `artifact-inference`). Each inference ships `🦀️.rs`, `🟦️.ts`, `🔣️.json`, `🔗️.graphql`, `🛰️.proto` (schema-first). Example: `gismap_artifact_inference_descriptor()` and `equation_artifact_inference_descriptor()`.
- Executable service: `ArtifactInferenceService::new(ArtifactInferenceServiceMetadata { owner, artifact_kind, artifact_schema(+version), inference_schema(+version), algorithm_version, policy_version, payload: Option<ArtifactInferencePayloadContract> }, fn)`, or `new_contextual(...)` when it needs the owning instance's resources (flow B-Rep). Registered with `register_artifact_inference_service` into a process-wide registry keyed by artifact kind plus inference schema. `payload: None` means the gateway refuses the call by name.
- Payload contract: `payload_schema_id`, `input_schema` and `output_schema` (JSON Schema text), `progress_unit`, `artifact_binding` (host fills the artifact's canonical `pack`/`spr` pair into a declared field, so clients never type the document), `commit` (`action` name; result becomes a proposal, approval runs the action through the normal edit path).
- Plugin manifest: `ArtifactContributionDescriptor { artifact_kind, mutations, inferences: Vec<ContributedInferenceMetadata> }`; per the manifest doc, accepted only when the artifact kind's owner is a direct plugin dependency. Plugin wiring: `print_plugin()` uses `ArtifactDeclaration::builder(definition).schema(...).inferences([descriptor]).inference_services([service])`.

### 2.3 Evaluation

- Typed: a direct call `XInference::infer(&snapshot)` (tests, native services such as `infer_gis_map_controlled`, `GltfInference` in `infer_gltf_leaf_cold`). The snapshot is the artifact's canonical decoded value.
- Engine driver: `infer_field_step(snapshot, cache, cursor, values, fuel)` resumes over `plan`. For each entity it folds the dependency hash `DepHash` (blake3; `root` for entities without parents, `chain` with parent hashes folded order-independently via `merkle_node`), consults the cache (hit is free), otherwise `compute_step` and stores. `try_infer_field` and `infer_field` drive to completion (`infer_field` panics on error). Cancellation: `InferenceCursor::cancel()` cancels the in-flight compute and refuses later steps with `InferenceError::Cancelled`; finished values stay valid.
- Caching is opt-in: `InferenceCacheConfig::default()` has `enabled: false`, so every path degenerates to pure recompute (cache-transparency law, tested). Cache is in-memory, typed (`Arc<dyn Any>`), LRU with a byte budget. `InferencePersistence::{None, Projection}` is declared; a production persistence adapter was NOT located.
- Incremental (diff-gated): `infer_field_after_diff(snapshot, diff, session, cache)` returns the stored result without walking the plan if `diff.touches()` (a `DiffRegions` impl, prefixes `/`-joined) does not intersect `F::reads()`. Otherwise it runs with the cache and refreshes the session root (merkle over per-entity hashes). Caveat: no production caller found; only engine tests call it.
- Artifact service: `infer(request)` or `infer_with_context(request, context)` with `ArtifactInferenceExecutionRequest { policy, budgets (work_units, recursion_depth, allocation_bytes), cancellation_id, previous_state, requested_cache_mode, canonical_payload, dependencies }`. Returns `ArtifactInferenceExecution { canonical_payload, diagnostics, validity, quality, complete, actual_cache_mode }`. Services check cancellation at checkpoints and enforce budgets. Print chart rejects `Incremental`. The flow B-Rep geometry service maps `Cold` to cancelling the operator's cached evaluation and `Bypass` to `node_hash = 0`.
- Plugin-side MCP: `inference_run` runs synchronously (`infer_real`, `🌉️mcp/🏠️workspace`). `inference_submit`, `inference_events`, `inference_cancel`, `inference_approve` run the same declared service as a job (`💼️jobs`), either in the plugin guest or on the hub (`features.inferenceServices`).

### 2.4 Consumption

- MCP clients: `inference_list`/`inference_get` (discovery from the declared roster), `inference_run`/`inference_submit` (execution), `inference_approve` (commit of proposals via `ActionAdapter`, normal undo and ledger).
- In-app: gismap editor command `propose_bounds_region` (quoted in 3B) emits `Effect::RequestServiceOperation { owner: "gis", service_id: "s.gis.gismap.inference", action: "propose" }`. The host opens an ephemeral port, and the proposal reaches the artifact only through the hub's approval command.
- Viewers and TS: `.../gismap/💡️inference/🪟️presentation/🟦️.tsx` and `🔌️client/🟦️.ts` exist. I did not trace their consumption in depth.

### 2.5 Serialization

- Values: `ToValue`/`FromValue` (`semio_framework_value`) to `DslValue`. Wire bytes: `pack_rt::encode_wire_value` (canonical binary).
- Schemas: `🔣️.json` carries `x-semio-derived: true` on derived fields (gismap). Text form for glTF leaves: LF-only envelope with schema id, version, decimal length, CRC-32 and RFC 8785 canonical JSON (`🚪️io/📝️text/💡️inferences/🦀️.rs`).
- Inference results are NOT part of the artifact snapshot; they are recomputed on demand (or cached in memory by the engine).

### 2.6 Testing

- Law tests per inference: determinism (`inference_determinism_law`) and default (`inference_default_law`), see both quoted examples.
- Language-agnostic fixtures: `🧫️fixtures/🔑️dependency/🔣.json` (equation), `🪜️stepped-driver/🔣️.json` (engine), `🧫️fixtures/🧩️map-create-region-group/🔣️.json` (gismap, referenced from the unit test).
- Independent oracle: `typed_dependency_matches_neutral_serde_oracle` (equation) compares the typed dependency against both the fixture and an independent `serde_json::json!` literal. Engine: `inference_owned_dependency_values_match_neutral_and_serde_oracle`.
- Engine laws: `disabled_cache_matches_pure_recompute`, `cold_and_warm_cache_match_pure_recompute`, `tiny_budget_eviction_storm_still_matches_pure_recompute`, `changing_a_leaf_weight_only_recomputes_that_leaf_and_its_descendants`, `stepped_driver_equals_the_unbounded_driver_for_every_fuel`, `stepped_driver_call_counts_match_the_language_agnostic_fixture`, `cancelling_mid_run_keeps_finished_values_...`, `a_diff_outside_the_reads_serves_the_stored_result_without_walking_the_plan`.
- Subset harness S0-S10 (`assert_subset_roundtrip`): calls `S::infer` twice and asserts equality (S6).
- MCP process and oracle tests: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🌉️mcp/🧪️tests/` (`🔬️oracle`, `🔬️process`, fixtures `💼️service-law`).

## 3. Two most complete examples (verbatim)

### 3A. Equation inference: aggregate `Inference<P>` plus `InferredField` DAG (`roots`)

Why: one artifact inference aggregates two fields. `topology` is whole-snapshot. `roots` is a real per-entity `InferredField` with `plan`, `dep_input`, `compute`, a fixture-backed oracle, and a unit-law suite. It is the framework's "roots can be inferred" example and maps directly to per-wall or per-storey fields.

Aggregate root (`🦀️.rs`):


```rs
//! 💡️ Equation inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (`🧭topology/`, and `🌱roots/` — wave M3a,
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS).

use crate::EquationSnapshot;
use framework_schema::ArtifactSchema;
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use super::roots::{compute_equation_roots, EquationRoot};
use super::topology::compute_equation_topology;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
//#region 🔖️Inference
/// 💡️ Everything inferable from a equation snapshot. One field per named inference under
/// `💡️inferences/` (`topology`, backed by `🧭topology/`; `roots`, backed by `🌱roots/`).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.equation.inference")]
pub struct EquationInference {
    #[derived]
    pub topology: EquationTopology,
    #[derived]
    pub roots: Vec<EquationRoot>,
}

impl Default for EquationInference {
    fn default() -> Self {
        let snapshot = &EquationSnapshot::default();

        Self { topology: compute_equation_topology(&snapshot.graph), roots: compute_equation_roots(snapshot) }
    }
}

impl protocol::Inference<EquationSnapshot> for EquationInference {
    fn infer(snapshot: &EquationSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { topology: compute_equation_topology(&snapshot.graph), roots: compute_equation_roots(snapshot) }
    
        })
    }
}

impl protocol::InferenceSpec<EquationSnapshot> for EquationInference {
    fn inference_schema_id() -> &'static str {
        "s.mathematical.equation.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.mathematical.equation.inference.topology", reads: &["notation", "results", "computed"] }, protocol::InferenceFieldSpec { id: "s.mathematical.equation.inference.roots", reads: &["equation"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.mathematical.equation.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `equation_artifact_schema_descriptor`'s
/// registration.
pub fn equation_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.mathematical.equation.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::topology::EquationTopology;
//#endregion 🔁️Re-exports

```

Aggregate schema (`🔣️.json`):


```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://json.schemas.assets.semio-tech.com/s/equation/equation/inference.json",
  "title": "EquationInference",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "topology"
  ],
  "properties": {
    "topology": {
      "$ref": "#/$defs/EquationTopology",
      "x-semio-derived": true
    }
  },
  "$defs": {
    "EquationTopology": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "topoOrder",
        "depth",
        "cycleFree",
        "nodeCount"
      ],
      "properties": {
        "topoOrder": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "depth": {
          "type": "object",
          "additionalProperties": {
            "type": "integer"
          }
        },
        "cycleFree": {
          "type": "boolean"
        },
        "nodeCount": {
          "type": "integer"
        }
      }
    }
  }
}

```

Aggregate tests (`🧪️tests/🔬️unit/🦀️.rs`):


```rs
use super::*;
use protocol::Inference;

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = EquationSnapshot::default();
    assert_eq!(EquationInference::infer(&snapshot).expect("valid materialized inference fixture"), EquationInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(EquationInference::infer(&EquationSnapshot::default()).expect("valid materialized inference fixture"), EquationInference::default());
}

#[semio_framework_async_macros::async_test]
async fn default_graph_diamond_is_cycle_free_with_two_roots() {
    // 🔷 The default graph is a diamond: a->b, a->c, b->d, c->d — acyclic, `a` is the only root.
    let inferred = EquationInference::infer(&EquationSnapshot::default()).expect("valid materialized inference fixture");
    assert!(inferred.topology.cycle_free);
    assert_eq!(inferred.topology.node_count, 4);
    assert_eq!(inferred.topology.depth["a"], 0);
    assert_eq!(inferred.topology.depth["d"], 2);
    let a_index = inferred.topology.topo_order.iter().position(|id| id == "a").unwrap();
    let d_index = inferred.topology.topo_order.iter().position(|id| id == "d").unwrap();
    assert!(a_index < d_index);
}

#[semio_framework_async_macros::async_test]
async fn default_equation_is_the_zero_polynomial_with_no_roots() {
    // 🔎️ `EquationExprSnapshot::default()` is the integer literal `0` — the zero polynomial has no
    // isolated real roots, an empty `Vec`, never a panic (see `🌱roots`'s own scope tests).
    let inferred = EquationInference::infer(&EquationSnapshot::default()).expect("valid materialized inference fixture");
    assert!(inferred.roots.is_empty());
}
//#endregion 🧪️InferenceLaws

```

Field `roots` (`🌱roots/🦀️.rs`):


```rs
//! 🌱️ `roots` — the user's own named example ("things such as roots can be inferred"). First real
//! `impl InferredField<P>` in this codebase (grepped repo-wide: every other named inference —
//! `🧭topology`, `📦bounds`, `flat-position` — documents why it uses the plain whole-snapshot
//! `compute_X(snapshot) -> X` pattern instead; `roots` is the genuine fit the trait was designed
//! for, since real roots form a small indexed COLLECTION with no cross-root dependency, matching
//! `InferredField::Key`'s intent). `Key = usize` (index into the isolated-root list, ascending —
//! `polynomial::roots::isolate_real_roots`'s own order), no parents (roots don't depend on each
//! other). `compute()` delegates into `📈️polynomial-internals`' real Sturm-sequence isolation +
//! bisection refinement — none of that math is reimplemented here.
//!
//! Scope (honest limitation, matches `EquationNode`'s own): only equations that reduce to a
//! single-variable, INTEGER-coefficient polynomial (`Add`/`Mul`/`Pow(Symbol, IntegerLiteral)`/
//! `Integer`/`Rational` with `denom == 1`) produce roots; anything else (rational coefficients,
//! multiple variables, `Fn`/`Piecewise`/etc.) plans ZERO steps — an empty root list, not a panic
//! or a wrong answer. Extending this to the full `cas::rootof`/`cas::solve`/
//! `polynomial::algebraic` machinery (irrational/complex roots, symbolic closed forms) is future
//! work once the mutation/inference table grows past this vertical slice.

use crate::standards::v1::subsets::any::schema::snapshot::{EquationExprSnapshot, EquationNode, EquationNodeKind};
use crate::EquationSnapshot;
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use std::collections::BTreeMap;

//#region 🔖️IntegerPolynomialExtraction
/// 🌉 Structural walk of `EquationNode` → `(variable name, degree -> integer coefficient)` — `None`
/// the instant the tree leaves this wave's scope (rational coefficient, second variable, non-integer
/// exponent, or any `Fn`/`Piecewise`/etc node — `EquationNode` can't even represent those yet).
fn extract_integer_polynomial(node: &EquationNode) -> Option<(String, BTreeMap<u32, number::Integer>)> {
    fn walk(node: &EquationNode, var: &mut Option<String>) -> Option<BTreeMap<u32, number::Integer>> {
        match &node.kind {
            EquationNodeKind::Integer { lexeme } => Some(BTreeMap::from([(0u32, lexeme.parse().ok()?)])),
            EquationNodeKind::Rational { numer, denom } => {
                if denom == "1" {
                    Some(BTreeMap::from([(0u32, numer.parse().ok()?)]))
                } else {
                    None
                }
            }
            EquationNodeKind::Symbol { name } => {
                match var {
                    None => *var = Some(name.clone()),
                    Some(existing) if existing != name => return None,
                    Some(_) => {}
                }
                Some(BTreeMap::from([(1u32, number::Integer::one())]))
            }
            EquationNodeKind::Add { terms } => {
                let mut acc: BTreeMap<u32, number::Integer> = BTreeMap::new();
                for term in terms {
                    for (degree, coeff) in walk(term, var)? {
                        let entry = acc.entry(degree).or_insert_with(number::Integer::zero);
                        *entry = entry.add(&coeff);
                    }
                }
                Some(acc)
            }
            EquationNodeKind::Mul { factors } => {
                let mut acc: BTreeMap<u32, number::Integer> = BTreeMap::from([(0u32, number::Integer::one())]);
                for factor in factors {
                    let rhs = walk(factor, var)?;
                    let mut next: BTreeMap<u32, number::Integer> = BTreeMap::new();
                    for (deg_a, coeff_a) in &acc {
                        for (deg_b, coeff_b) in &rhs {
                            let entry = next.entry(deg_a + deg_b).or_insert_with(number::Integer::zero);
                            *entry = entry.add(&coeff_a.mul(coeff_b));
                        }
                    }
                    acc = next;
                }
                Some(acc)
            }
            EquationNodeKind::Pow { base, exponent } => {
                let EquationNodeKind::Integer { lexeme } = &exponent.kind else { return None };
                let exp: u32 = lexeme.parse().ok()?;
                let EquationNodeKind::Symbol { name } = &base.kind else { return None };
                match var {
                    None => *var = Some(name.clone()),
                    Some(existing) if existing != name => return None,
                    Some(_) => {}
                }
                Some(BTreeMap::from([(exp, number::Integer::one())]))
            }
        }
    }
    let mut var = None;
    let coeffs = walk(node, &mut var)?;
    Some((var.unwrap_or_else(|| "x".to_string()), coeffs))
}

fn to_poly_u(coeffs: &BTreeMap<u32, number::Integer>) -> crate::polynomial::univariate::PolyU<number::Integer> {
    let mut poly = crate::polynomial::univariate::PolyU::zero();
    for (degree, coeff) in coeffs {
        poly = poly.add(&crate::polynomial::univariate::PolyU::monomial(coeff.clone(), *degree as usize));
    }
    poly
}

/// 🌉 `None` when `equation` is outside this wave's scope (see module doc) — the ONE place
/// `plan`/`dep_input`/`compute` all funnel through, so all three always agree on scope.
fn equation_integer_polynomial(equation: &EquationExprSnapshot) -> Option<crate::polynomial::univariate::PolyU<number::Integer>> {
    let (_, coeffs) = extract_integer_polynomial(&equation.expr)?;
    if coeffs.is_empty() {
        return None;
    }
    Some(to_poly_u(&coeffs))
}
//#endregion 🔖️IntegerPolynomialExtraction

//#region 🔖️RootValue
/// 🌱️ One isolated-then-refined real root, as a decimal `f64` approximation — the refined rational
/// interval's midpoint, narrowed to `REFINE_WIDTH`.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase")]
pub struct EquationRoot {
    pub approx: f64,
}
//#endregion 🔖️RootValue

//#region 🔖️InferredField
/// 🌱️ The bisection target width every `compute()` call refines to — `1 / 10^9`, matching the
/// precision the migrated `polynomial::algebraic` tests already assert against (`1e-6`/`1e-9`
/// tolerances), so `roots`' output is at least as precise as what those tests already trust.
fn refine_width() -> number::Rational {
    number::Rational::new(number::Integer::one(), number::Integer::from_i64(1_000_000_000)).expect("1/10^9 is a valid rational")
}

pub struct EquationRootsField;

impl protocol::InferredField<EquationSnapshot> for EquationRootsField {
    type Dependency = semio_framework_value::DslValue;
    type Key = usize;
    type Value = EquationRoot;

    const FIELD_ID: &'static str = "s.mathematical.equation.inference.roots";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["equation"]
    }

    /// 🧭️ Isolates once to learn how many real roots exist (Sturm-sequence sign-change counting —
    /// `polynomial::roots::isolate_real_roots`) and plans one step per index, no parents: roots of
    /// the same polynomial don't depend on each other's values.
    fn plan(snapshot: &EquationSnapshot) -> Vec<protocol::InferenceStep<Self::Key>> {
        let Some(poly) = equation_integer_polynomial(&snapshot.equation) else { return Vec::new() };
        (0..crate::polynomial::roots::isolate_real_roots(&poly).len()).map(|index| protocol::InferenceStep { key: index, parents: vec![] }).collect()
    }

    /// 🔑️ The polynomial's own coefficients (so ANY edit to ANY coefficient invalidates every
    /// root's cache entry, not just the one nearest that coefficient — a real polynomial's roots
    /// are a global function of ALL coefficients, unlike `flat-position`'s local per-edge deps) plus
    /// this key's isolating interval (so a coefficient edit that shifts WHICH interval index `key`
    /// lands on also invalidates, even if the isolation count happens to stay the same).
    fn dep_input(snapshot: &EquationSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Self::Dependency {
        let Some(poly) = equation_integer_polynomial(&snapshot.equation) else { return semio_framework_value::DslValue::Null };
        let coefficients = poly.coeffs().iter().map(|coefficient| semio_framework_value::DslValue::String(coefficient.to_string())).collect();
        let interval = crate::polynomial::roots::isolate_real_roots(&poly).get(*key).map(|(lo, hi)| semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::String(lo.to_string()), semio_framework_value::DslValue::String(hi.to_string())])).unwrap_or(semio_framework_value::DslValue::Null);
        semio_framework_value::DslValue::object([("coefficients".into(), semio_framework_value::DslValue::Array(coefficients)), ("interval".into(), interval)])
    }

    /// 🧮️ Re-isolates (cheap relative to refinement — Sturm sequences over small integer
    /// polynomials) and bisects the `key`-th interval down to `refine_width()`, returning the
    /// refined interval's midpoint as `f64`.
    fn compute(snapshot: &EquationSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let Some(poly) = equation_integer_polynomial(&snapshot.equation) else { return EquationRoot::default() };
        let intervals = crate::polynomial::roots::isolate_real_roots(&poly);
        let Some((lo, hi)) = intervals.get(*key) else { return EquationRoot::default() };
        let (lo, hi) = crate::polynomial::roots::refine_root(&poly, lo, hi, &refine_width());
        let midpoint = lo.add(&hi).div(&number::Rational::from_i64(2, 1).expect("2/1 is valid")).unwrap_or_else(number::Rational::zero);
        EquationRoot { approx: midpoint.to_f64() }
    }
}

/// 🌱️ Assembles the whole `roots` field via `protocol::infer_field` — the real dependency-hash-
/// chained plan/compute orchestration `InferredField` exists for, not a hand-rolled loop over
/// `plan()`/`compute()`. Returns a plain ascending `Vec` (index order) for `EquationInference`.
pub fn compute_equation_roots(snapshot: &EquationSnapshot) -> Vec<EquationRoot> {
    let values = protocol::infer_field::<EquationSnapshot, EquationRootsField>(snapshot, None);
    values.into_values().collect()
}
//#endregion 🔖️InferredField

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

Field tests (`🌱roots/🧪️tests/🔬️unit/🦀️.rs`):


```rs
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::EquationNodeLabel;

/// 🧪️ `x^2 - 3x + 2 = (x-1)(x-2)`, roots `{1, 2}` — built directly as a labeled tree (`Add` of
/// `x^2`, `-3x`, `2`), the same shape `expr_to_equation_node` would produce from
/// `cas::polybridge`'s own canonical `Add` term order.
fn quadratic_with_roots_one_and_two() -> EquationExprSnapshot {
    let x = EquationNode { label: EquationNodeLabel(1), kind: EquationNodeKind::Symbol { name: "x".into() } };
    let two_exp = EquationNode { label: EquationNodeLabel(2), kind: EquationNodeKind::Integer { lexeme: "2".into() } };
    let x_squared = EquationNode { label: EquationNodeLabel(3), kind: EquationNodeKind::Pow { base: Box::new(x.clone()), exponent: Box::new(two_exp) } };
    let neg_three = EquationNode { label: EquationNodeLabel(4), kind: EquationNodeKind::Integer { lexeme: "-3".into() } };
    let neg_three_x = EquationNode { label: EquationNodeLabel(5), kind: EquationNodeKind::Mul { factors: vec![neg_three, x] } };
    let two = EquationNode { label: EquationNodeLabel(6), kind: EquationNodeKind::Integer { lexeme: "2".into() } };
    let expr = EquationNode { label: EquationNodeLabel(7), kind: EquationNodeKind::Add { terms: vec![x_squared, neg_three_x, two] } };
    EquationExprSnapshot { expr, next_label: 8 }
}

#[semio_framework_async_macros::async_test]
async fn extracts_integer_polynomial_coefficients_by_degree() {
    let (var, coeffs) = extract_integer_polynomial(&quadratic_with_roots_one_and_two().expr).expect("polynomial in scope");
    assert_eq!(var, "x");
    assert_eq!(coeffs.get(&0).map(|c| c.to_string()), Some("2".to_string()));
    assert_eq!(coeffs.get(&1).map(|c| c.to_string()), Some("-3".to_string()));
    assert_eq!(coeffs.get(&2).map(|c| c.to_string()), Some("1".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn plan_has_one_step_per_isolated_root_with_no_parents() {
    let mut snapshot = EquationSnapshot::default();
    snapshot.equation = quadratic_with_roots_one_and_two();
    let steps = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::plan(&snapshot);
    assert_eq!(steps.len(), 2, "x^2-3x+2 has exactly two real roots");
    assert!(steps.iter().all(|step| step.parents.is_empty()), "roots never depend on each other");
}

#[semio_framework_async_macros::async_test]
async fn compute_equation_roots_finds_one_and_two() {
    let mut snapshot = EquationSnapshot::default();
    snapshot.equation = quadratic_with_roots_one_and_two();
    let mut roots: Vec<f64> = compute_equation_roots(&snapshot).into_iter().map(|r| r.approx).collect();
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(roots.len(), 2);
    assert!((roots[0] - 1.0).abs() < 1e-6, "expected root near 1.0, got {}", roots[0]);
    assert!((roots[1] - 2.0).abs() < 1e-6, "expected root near 2.0, got {}", roots[1]);
}

#[semio_framework_async_macros::async_test]
async fn out_of_scope_equation_plans_zero_steps_not_a_panic() {
    // 🔎️ Rational, non-integer coefficient — documented scope boundary, not a crash.
    let snapshot = EquationSnapshot::default(); // default equation is the integer literal 0
    let steps = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::plan(&snapshot);
    assert!(steps.is_empty(), "the zero polynomial has no isolated real roots to plan");
}

/// 🧪️ Determinism law (mirrors `🧭topology`'s own `inference_determinism_law`): same snapshot,
/// same `DepHash` chain, same values — required for `protocol::infer_field`'s cache to ever be
/// safe to enable for this field.
#[semio_framework_async_macros::async_test]
async fn dep_hash_is_deterministic_across_repeated_calls() {
    let mut snapshot = EquationSnapshot::default();
    snapshot.equation = quadratic_with_roots_one_and_two();
    let first = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::dep_input(&snapshot, &0, &[]);
    let second = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::dep_input(&snapshot, &0, &[]);
    assert_eq!(first, second);
}

/// 🧪️ The whole point of a real `DepHash` chain: an edit that changes a coefficient must
/// change `dep_input`'s values for every root, proving the chain is actually wired to
/// `equation`, not a constant.
#[semio_framework_async_macros::async_test]
async fn dep_input_changes_when_a_coefficient_changes() {
    let mut before = EquationSnapshot::default();
    before.equation = quadratic_with_roots_one_and_two();
    let mut after = before.clone();
    after.equation.replace(EquationNodeLabel(6), &EquationNodeKind::Integer { lexeme: "99".into() });
    let before_bytes = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::dep_input(&before, &0, &[]);
    let after_bytes = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::dep_input(&after, &0, &[]);
    assert_ne!(before_bytes, after_bytes, "changing a coefficient must change the DepHash input");
}

/// 🧪️ Native serde independently checks the neutral logical dependency projection.
#[semio_framework_async_macros::async_test]
async fn typed_dependency_matches_neutral_serde_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔑️dependency/🔣.json")).unwrap();
    let mut snapshot = EquationSnapshot::default();
    snapshot.equation = quadratic_with_roots_one_and_two();
    let dependency = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::dep_input(&snapshot, &usize::MAX, &[]);
    let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::from_dsl_value(&dependency).to_string()).unwrap();
    let independent = serde_json::json!({"coefficients":["2","-3","1"],"interval":null});
    assert_eq!(actual, fixture);
    assert_eq!(actual, independent);
}

```

Field fixture (`🌱roots/🧫️fixtures/🔑️dependency/🔣.json`):


```json
{
  "coefficients": [
    "2",
    "-3",
    "1"
  ],
  "interval": null
}

```

Field TypeScript facet (`🌱roots/🟦️.ts`):


```ts
/** 🌱 `roots` — the equation's real roots, isolated (Sturm sequences) and bisection-refined to
 * 1/10^9. Scope: single-variable, integer-coefficient equations only (see the Rust component's
 * doc header) — anything else infers an empty list, never a wrong or missing value. */

export interface EquationRoot {
  approx: number;
}

```

### 3B. GIS map inference: derived scalars, `Inference<P>`, proposal path, and editor command

Why: a typed `Inference<P>` with `#[derived]` fields, a `bounds` sub-inference, unit laws, and the clearest in-repo example of an inference output turned into an edit proposal (`bounds_proposal`, `create_region_group_work`). It also shows the editor command that requests the port. It does NOT use `InferredField` (its own comment says the whole-snapshot scan is enough).

Inference root (`🦀️.rs`):


```rs
//! 💡️ GIS map inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use super::bounds::{all_lon_lat_pairs, lon_lat_bounds};
use crate::{GisMapDrawingChild, GisMapSnapshot, GisMapValueChild};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::mutations::{create_node, inverse_semio_drawing_mutation, SemioDrawingMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode;
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::mutations::{inverse_semio_value_mutation, SemioValueMutation};
//#region 🔖️Inference
/// 💡️ Everything inferable from a gismap snapshot. Today: per-collection feature counts and the
/// geographic bounding box across every `positions`/`routes`/`regions` feature (see
/// `📦bounds/🦀️.rs`). A simple whole-snapshot scalar — no `InferredField` caching, the
/// feature collections here are small.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gismap.inference")]
pub struct GisMapInference {
    #[derived]
    pub position_count: usize,
    #[derived]
    pub route_count: usize,
    #[derived]
    pub region_count: usize,
    #[derived]
    pub bounds: Option<GisMapBounds>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GisMapProposalError {
    Identity,
    Stale,
    Bounds,
    Composition,
    InverseRefused(semio_framework_value::ValueError),
}

/// 🧩️ One bounded typed parent+drawing+value work owner prepared from an immutable Map base.
#[derive(Debug, PartialEq)]
pub struct GisMapCreateRegionGroupWorkV1 {
    pub parent: crate::mutations::GisMapMutation,
    pub parent_inverse: Vec<crate::mutations::GisMapMutation>,
    pub drawing_child: GisMapDrawingChild,
    pub drawing: SemioDrawingMutation,
    pub drawing_inverse: Vec<SemioDrawingMutation>,
    pub value_child: GisMapValueChild,
    pub value: SemioValueMutation,
    pub value_inverse: Vec<SemioValueMutation>,
}

impl GisMapInference {
    /// 🌐️ Produces one typed region mutation, without applying it or granting approval authority.
    pub fn bounds_proposal(&self, snapshot: &GisMapSnapshot, job_id: &str) -> Result<crate::mutations::GisMapMutation, GisMapProposalError> {
        use crate::{
            mutations::{create_region::CreateRegion, GisMapMutation},
            MapFeature,
        };
        if job_id.len() != 32 || !job_id.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
            return Err(GisMapProposalError::Identity);
        }
        let id = format!("inference-{job_id}");
        if self.position_count != snapshot.positions.len() || self.route_count != snapshot.routes.len() || self.region_count != snapshot.regions.len() || snapshot.regions.iter().any(|region| region.id == id) {
            return Err(GisMapProposalError::Stale);
        }
        if self.region_count >= 65_536 {
            return Err(GisMapProposalError::Bounds);
        }
        let bounds = self.bounds.as_ref().ok_or(GisMapProposalError::Bounds)?;
        if ![bounds.lon_min, bounds.lon_max, bounds.lat_min, bounds.lat_max].iter().all(|value| value.is_finite())
            || bounds.lon_min < -180.0
            || bounds.lon_max > 180.0
            || bounds.lat_min < -90.0
            || bounds.lat_max > 90.0
            || bounds.lon_min > bounds.lon_max
            || bounds.lat_min > bounds.lat_max
        {
            return Err(GisMapProposalError::Bounds);
        }
        let ring = [[bounds.lon_min, bounds.lat_min], [bounds.lon_max, bounds.lat_min], [bounds.lon_max, bounds.lat_max], [bounds.lon_min, bounds.lat_max], [bounds.lon_min, bounds.lat_min]]
            .map(|point| semio_framework_value::DslValue::Array(point.map(semio_framework_value::DslValue::float).into()));
        let data = semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String(id.clone())), ("kind".into(), semio_framework_value::DslValue::String("inference-bounds".into())), ("ring".into(), semio_framework_value::DslValue::Array(ring.into()))]);
        Ok(GisMapMutation::CreateRegion(CreateRegion { index: snapshot.regions.len(), item: MapFeature { id, data } }))
    }

    /// 🧬️ Builds exactly one stable-member parent+drawing+value CreateRegion work group.
    pub fn create_region_group_work(&self, snapshot: &GisMapSnapshot, job_id: &str) -> Result<GisMapCreateRegionGroupWorkV1, GisMapProposalError> {
        use crate::mutations::{apply_gis_map_mutation, inverse_gis_map_mutation, GisMapMutation};
        use crate::schema::gis_map_snapshot_to_drawing;

        use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
        use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::mutations::apply_semio_drawing_mutation;
        use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::mutations::apply_semio_value_mutation;

        let parent = self.bounds_proposal(snapshot, job_id)?;
        let GisMapMutation::CreateRegion(created) = &parent else { return Err(GisMapProposalError::Composition) };
        if snapshot.image.is_some() || snapshot.drawing.child_id != "gismap-drawing" || snapshot.value.child_id != "gismap-value" {
            return Err(GisMapProposalError::Composition);
        }
        let parent_inverse = inverse_gis_map_mutation(snapshot, &parent).map_err(GisMapProposalError::InverseRefused)?;
        let mut after = snapshot.clone();
        apply_gis_map_mutation(&mut after, &parent).map_err(|_| GisMapProposalError::Composition)?;
        if after.drawing != snapshot.drawing || after.value != snapshot.value || after.image != snapshot.image {
            return Err(GisMapProposalError::Composition);
        }

        let before_drawing = gis_map_snapshot_to_drawing(snapshot);
        let after_drawing = gis_map_snapshot_to_drawing(&after);
        if before_drawing.schema != after_drawing.schema || before_drawing.canvas != after_drawing.canvas || before_drawing.styles != after_drawing.styles || before_drawing.layers.len() != 1 || after_drawing.layers.len() != 1 {
            return Err(GisMapProposalError::Composition);
        }
        let (before_children, after_children) = match (&before_drawing.layers[0].root, &after_drawing.layers[0].root) {
            (DrawNode::Group { transform: before_transform, children: before_children }, DrawNode::Group { transform: after_transform, children: after_children })
                if before_transform == after_transform
                    && before_drawing.layers[0].id == after_drawing.layers[0].id
                    && before_drawing.layers[0].name == after_drawing.layers[0].name
                    && before_drawing.layers[0].visible == after_drawing.layers[0].visible =>
            {
                (before_children, after_children)
            }
            _ => return Err(GisMapProposalError::Composition),
        };
        if after_children.len() != before_children.len() + 1 || !after_children.starts_with(before_children) {
            return Err(GisMapProposalError::Composition);
        }
        let drawing = SemioDrawingMutation::CreateNode(create_node::CreateNode { parent: NodePath { layer: 0, path: Vec::new() }, index: before_children.len(), node: after_children[before_children.len()].clone() });
        let drawing_inverse = inverse_semio_drawing_mutation(&drawing, &before_drawing).map_err(GisMapProposalError::InverseRefused)?;
        let mut projected_drawing = before_drawing.clone();
        apply_semio_drawing_mutation(&mut projected_drawing, &drawing);
        if projected_drawing != after_drawing {
            return Err(GisMapProposalError::Composition);
        }

        let before_value = crate::gis_map_value_from_descriptor(&crate::schema::gis_map_descriptor_value(snapshot));
        let after_value = crate::gis_map_value_from_descriptor(&crate::schema::gis_map_descriptor_value(&after));
        let value_payload = crate::semio_value_from_intrinsic(&created.item.data);
        let value = SemioValueMutation::from_value(semio_framework_value::DslValue::object([
            ("mutation".into(), semio_framework_value::DslValue::String("insertListItem".into())),
            ("path".into(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::object([("kind".into(), semio_framework_value::DslValue::String("key".into())), ("key".into(), semio_framework_value::DslValue::String("regions".into()))])])),
            ("index".into(), created.index.to_value()),
            ("value".into(), value_payload.to_value()),
        ]))
        .map_err(|_| GisMapProposalError::Composition)?;
        let value_inverse = inverse_semio_value_mutation(&value, &before_value).map_err(GisMapProposalError::InverseRefused)?;
        let mut projected_value = before_value;
        apply_semio_value_mutation(&mut projected_value, &value);
        if projected_value != after_value {
            return Err(GisMapProposalError::Composition);
        }
        let bytes = [&parent.to_value(),&parent_inverse.to_value(),&drawing.to_value(),&drawing_inverse.to_value(),&value.to_value(),&value_inverse.to_value()].into_iter().fold(0usize,|sum,value|sum.saturating_add(intrinsic_extent(value)));
        if bytes > 65_536 {
            return Err(GisMapProposalError::Bounds);
        }
        Ok(GisMapCreateRegionGroupWorkV1 { parent, parent_inverse, drawing_child: snapshot.drawing.clone(), drawing, drawing_inverse, value_child: snapshot.value.clone(), value, value_inverse })
    }
}

impl protocol::Inference<GisMapSnapshot> for GisMapInference {
    fn infer(snapshot: &GisMapSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { position_count: snapshot.positions.len(), route_count: snapshot.routes.len(), region_count: snapshot.regions.len(), bounds: lon_lat_bounds(&all_lon_lat_pairs(snapshot)) }
    
        })
    }
}

impl protocol::InferenceSpec<GisMapSnapshot> for GisMapInference {
    fn inference_schema_id() -> &'static str {
        "s.gis.gismap.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.positionCount", reads: &["positions"] },
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.routeCount", reads: &["routes"] },
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.regionCount", reads: &["regions"] },
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.bounds", reads: &["positions", "routes", "regions"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.gis.gismap.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `gismap_artifact_schema_descriptor`'s registration.
pub fn gismap_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.gis.gismap.inference",
        inference: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::bounds::GisMapBounds;
//#endregion 🔁️Re-exports

/// 📏️ Complete owned intrinsic extent, independent of wire escaping and serializer choices.
fn intrinsic_extent(root:&semio_framework_value::DslValue)->usize{use semio_framework_value::DslValue;let mut bytes=0usize;let mut pending=vec![root];while let Some(value)=pending.pop(){bytes=bytes.saturating_add(std::mem::size_of::<DslValue>());match value{DslValue::String(text)=>bytes=bytes.saturating_add(text.len()),DslValue::Bytes(data)=>bytes=bytes.saturating_add(data.len()),DslValue::Array(items)=>pending.extend(items),DslValue::Object(members)=>for(name,value)in members{bytes=bytes.saturating_add(name.len());pending.push(value)},_=>()}}bytes}

```

Schema (`🔣️.json`):


```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://json.schemas.assets.semio-tech.com/s/gis/gismap/inference.json",
  "title": "GisMapInference",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "positionCount",
    "routeCount",
    "regionCount",
    "bounds"
  ],
  "properties": {
    "positionCount": {
      "type": "integer",
      "minimum": 0,
      "x-semio-derived": true
    },
    "routeCount": {
      "type": "integer",
      "minimum": 0,
      "x-semio-derived": true
    },
    "regionCount": {
      "type": "integer",
      "minimum": 0,
      "x-semio-derived": true
    },
    "bounds": {
      "oneOf": [
        {
          "$ref": "#/$defs/GisMapBounds"
        },
        {
          "type": "null"
        }
      ],
      "x-semio-derived": true
    }
  },
  "$defs": {
    "GisMapBounds": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "lonMin",
        "lonMax",
        "latMin",
        "latMax"
      ],
      "properties": {
        "lonMin": {
          "type": "number"
        },
        "lonMax": {
          "type": "number"
        },
        "latMin": {
          "type": "number"
        },
        "latMax": {
          "type": "number"
        }
      }
    }
  }
}

```

Sub-inference `bounds` (`📦bounds/🦀️.rs`):


```rs
//! 📦 `bounds` — one named inference: geographic bounding box across every `positions`/`routes`/
//! `regions` feature. `MapFeature::data` is deliberately untyped (`semio_framework_value::DslValue`, the engine's
//! `Shape::Value` escape hatch — see `crate`'s own docs), so the box is derived
//! by a generic coordinate-pair scan over each feature's raw value rather than by assuming a fixed
//! shape: any `{lon, lat}` object or `[number, number]` pair anywhere inside `data` counts as one
//! point. This uniformly covers `positions` (`{lon,lat}`), `routes` (`points: [[lon,lat], …]`) and
//! `regions` (`ring: [[lon,lat], …]`) without hard-coding any of those field names. Simple
//! whole-snapshot scalar: no `InferredField` caching, feature counts here are small.

use crate::GisMapSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📦Bounds
/// 📦 Geographic bounding box across every scanned `(lon, lat)` pair.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct GisMapBounds {
    pub lon_min: f64,
    pub lon_max: f64,
    pub lat_min: f64,
    pub lat_max: f64,
}

/// 🗺️ Recursively collects every `{lon, lat}` object and `[number, number]` pair inside `value`.
pub(crate) fn scan_lon_lat_pairs(value: &semio_framework_value::DslValue, out: &mut Vec<(f64, f64)>) {
    visit_lon_lat_pairs(value, 1, &mut |lon, lat| out.push((lon, lat)), &mut |_| Ok::<(), std::convert::Infallible>(())).expect("infallible coordinate collection");
}

fn visit_lon_lat_pairs<E>(value: &semio_framework_value::DslValue, depth: u32, point: &mut impl FnMut(f64, f64), checkpoint: &mut impl FnMut(u32) -> Result<(), E>) -> Result<(), E> {
    checkpoint(depth)?;
    match value {
        semio_framework_value::DslValue::Object(entries) => {
            let lon = entries.iter().find(|(key, _)| key == "lon").and_then(|(_, value)| value.as_f64());
            let lat = entries.iter().find(|(key, _)| key == "lat").and_then(|(_, value)| value.as_f64());
            if let (Some(lon), Some(lat)) = (lon, lat) {
                point(lon, lat);
            }
            for (_, value) in entries {
                visit_lon_lat_pairs(value, depth.saturating_add(1), point, checkpoint)?;
            }
        }
        semio_framework_value::DslValue::Array(items) => {
            if let [a, b] = items.as_slice() {
                if let (Some(a), Some(b)) = (a.as_f64(), b.as_f64()) {
                    point(a, b);
                    return Ok(());
                }
            }
            for item in items {
                visit_lon_lat_pairs(item, depth.saturating_add(1), point, checkpoint)?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// ⏱️ Folds coordinates without retaining a point vector and checks every visited value.
pub(crate) fn controlled_lon_lat_bounds<E>(snapshot: &GisMapSnapshot, checkpoint: &mut impl FnMut(u32) -> Result<(), E>) -> Result<Option<GisMapBounds>, E> {
    let mut bounds = None;
    for feature in snapshot.positions.iter().chain(snapshot.routes.iter()).chain(snapshot.regions.iter()) {
        visit_lon_lat_pairs(&feature.data, 1, &mut |lon, lat| include_point(&mut bounds, lon, lat), checkpoint)?;
    }
    Ok(bounds)
}

fn include_point(bounds: &mut Option<GisMapBounds>, lon: f64, lat: f64) {
    *bounds = Some(match bounds.as_ref() {
        Some(current) => GisMapBounds { lon_min: current.lon_min.min(lon), lon_max: current.lon_max.max(lon), lat_min: current.lat_min.min(lat), lat_max: current.lat_max.max(lat) },
        None => GisMapBounds { lon_min: lon, lon_max: lon, lat_min: lat, lat_max: lat },
    });
}

/// 📦 Bounding box across every scanned `(lon, lat)` pair, or `None` when nothing scanned.
pub(crate) fn lon_lat_bounds(pairs: &[(f64, f64)]) -> Option<GisMapBounds> {
    pairs.iter().fold(None, |acc, &(lon, lat)| {
        let mut bounds = acc;
        include_point(&mut bounds, lon, lat);
        bounds
    })
}

/// 🗺️ Scans every feature across `positions`/`routes`/`regions` for coordinate pairs.
pub(crate) fn all_lon_lat_pairs(snapshot: &GisMapSnapshot) -> Vec<(f64, f64)> {
    let mut pairs = Vec::new();
    for feature in snapshot.positions.iter().chain(snapshot.routes.iter()).chain(snapshot.regions.iter()) {
        scan_lon_lat_pairs(&feature.data, &mut pairs);
    }
    pairs
}
//#endregion 📦Bounds

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

Inference laws (`🧪️tests/🔬️unit/🦀️.rs`):


```rs
use super::*;
use crate::{gis_map_snapshot_with_derived_children, MapFeature};
use protocol::Inference;

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = GisMapSnapshot { positions: vec![MapFeature { id: "p1".into(), data: semio_framework_value::DslValue::from(serde_json::json!({ "lon": 1.0, "lat": 2.0 })) }], routes: Vec::new(), regions: Vec::new(), ..Default::default() };
    assert_eq!(GisMapInference::infer(&snapshot).expect("valid materialized inference fixture"), GisMapInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(GisMapInference::infer(&GisMapSnapshot::default()).expect("valid materialized inference fixture"), GisMapInference::default());
}

#[semio_framework_async_macros::async_test]
async fn map_create_region_group_work_stabilizes_parent_drawing_value_without_image() {
    use crate::mutations::apply_gis_map_mutation;
    use crate::schema::gis_map_snapshot_to_drawing;
use crate::standards::v1::subsets::any::io::text::snapshot::gis_map_descriptor_json;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::mutations::apply_semio_drawing_mutation;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::mutations::apply_semio_value_mutation;

    let feature = |id: &str, data: serde_json::Value| MapFeature { id: id.into(), data: semio_framework_value::DslValue::from(data) };
    let snapshot = gis_map_snapshot_with_derived_children(GisMapSnapshot {
        positions: vec![feature("point-a", serde_json::json!({ "id": "point-a", "lon": 7, "lat": 47 }))],
        routes: vec![feature("route-a", serde_json::json!({ "id": "route-a", "points": [[8, 46], [9, 48]] }))],
        ..Default::default()
    });
    let inferred = GisMapInference::infer(&snapshot).expect("valid materialized inference fixture");
    let work = inferred.create_region_group_work(&snapshot, "11111111111111111111111111111111").expect("typed group work");
    assert_eq!(work.drawing_child.child_id, "gismap-drawing");
    assert_eq!(work.value_child.child_id, "gismap-value");
    assert!(snapshot.image.is_none());

    let mut parent_after = snapshot.clone();
    apply_gis_map_mutation(&mut parent_after, &work.parent).expect("parent applies");
    assert_eq!(parent_after.drawing, snapshot.drawing);
    assert_eq!(parent_after.value, snapshot.value);
    let before_drawing = gis_map_snapshot_to_drawing(&snapshot);
    let after_drawing = gis_map_snapshot_to_drawing(&parent_after);
    let mut projected_drawing = before_drawing.clone();
    apply_semio_drawing_mutation(&mut projected_drawing, &work.drawing);
    assert_eq!(projected_drawing, after_drawing);
    for inverse in &work.drawing_inverse {
        apply_semio_drawing_mutation(&mut projected_drawing, inverse);
    }
    assert_eq!(projected_drawing, before_drawing);

    let before_value = crate::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(&snapshot));
    let after_value = crate::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(&parent_after));
    let mut projected_value = before_value.clone();
    apply_semio_value_mutation(&mut projected_value, &work.value);
    assert_eq!(projected_value, after_value);
    for inverse in &work.value_inverse {
        apply_semio_value_mutation(&mut projected_value, inverse);
    }
    assert_eq!(projected_value, before_value);

    for inverse in &work.parent_inverse {
        apply_gis_map_mutation(&mut parent_after, inverse).expect("parent inverse applies");
    }
    assert_eq!(parent_after, snapshot);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🧫️fixtures/🧩️map-create-region-group/🔣️.json")).expect("neutral Map membership corpus");
    for row in fixture["membershipCases"].as_array().expect("membership cases") {
        let mut candidate = snapshot.clone();
        candidate.drawing.child_id = row["drawingChildId"].as_str().unwrap().into();
        candidate.value.child_id = row["valueChildId"].as_str().unwrap().into();
        candidate.image = row["imageChildId"].as_str().map(|id| {
            store::ArtifactChild::new(id.to_owned(), semio_framework_artifact_reference::ArtifactRef { artifact_id: id.to_owned(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } })
        });
        let supplied_image = candidate.image.clone();
        if row["deriveChildren"].as_bool().unwrap() {
            candidate = gis_map_snapshot_with_derived_children(candidate);
        }
        assert_eq!(candidate.image, supplied_image, "deriving children must preserve a supplied image");
        let unchanged = candidate.clone();
        let result = GisMapInference::infer(&candidate).expect("valid materialized inference fixture").create_region_group_work(&candidate, fixture["jobId"].as_str().unwrap());
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{}", row["name"]);
        if !row["accepted"].as_bool().unwrap() {
            assert_eq!(result.unwrap_err(), GisMapProposalError::Composition);
        }
        assert_eq!(candidate, unchanged, "planning must not change the supplied snapshot");
    }
}
//#endregion 🧪️InferenceLaws

```

Bounds tests (`📦bounds/🧪️tests/🔬️unit/🦀️.rs`):


```rs
use super::*;
use crate::MapFeature;

fn dsl_of(value: serde_json::Value) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::from(value)
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_has_no_bounds() {
    assert!(lon_lat_bounds(&all_lon_lat_pairs(&GisMapSnapshot::default())).is_none());
}

#[semio_framework_async_macros::async_test]
async fn positions_routes_and_regions_all_contribute_points() {
    let snapshot = GisMapSnapshot {
        positions: vec![MapFeature { id: "p1".into(), data: dsl_of(serde_json::json!({ "id": "p1", "lon": -0.1427, "lat": 51.5142 })) }],
        routes: vec![MapFeature { id: "r1".into(), data: dsl_of(serde_json::json!({ "id": "r1", "points": [[1.0, 2.0], [3.0, 4.0]] })) }],
        regions: vec![MapFeature { id: "g1".into(), data: dsl_of(serde_json::json!({ "id": "g1", "ring": [[0.0, 0.0], [1.0, 1.0], [1.0, 0.0]] })) }],
        ..Default::default()
    };
    let bounds = lon_lat_bounds(&all_lon_lat_pairs(&snapshot)).expect("features bound");
    assert_eq!(bounds, GisMapBounds { lon_min: -0.1427, lon_max: 3.0, lat_min: 0.0, lat_max: 51.5142 });
}

```

Editor command requesting the inference port (`✏️editor/🎮️commands/💡️inference/🦀️.rs`):


```rs
//! 💡️ GIS 2D play app command — the Shell-kind effect that asks the host to open its own ephemeral
//! inference port and offer one reviewable bounds-region proposal.
//!
//! This is deliberately NOT a document command: it writes no `GisMapMutation`, holds no job state,
//! names no model/provider/transport, and carries no document, space, request or credential
//! identity. Everything the lifecycle needs — the scope, the idempotency key, the execution-target
//! lease precondition, the progress cursor, the proposal hash, and the Cancel/Approve controls —
//! is host-owned. The proposal itself only ever reaches this artifact through the hub's
//! server-stamped approval command, never through this effect.

use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use crate::GisMapSnapshot;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 💡️ProposeBoundsRegion
pub mod propose_bounds_region {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "propose-bounds-region")]
    pub struct ProposeBoundsRegion {}

    pub fn handle(_payload: &ProposeBoundsRegion, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestServiceOperation { owner: "gis".into(), service_id: "s.gis.gismap.inference".into(), action: "propose".into(), payload: semio_framework::DslValue::Object(Vec::new()) }))
    }
}
//#endregion 💡️ProposeBoundsRegion

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

### 3C. Short reference: print chart inference (admission and diagnostics pattern)

Shows the "inference output = validity plus diagnostics" pattern (also used by gltf `validity`/`diagnostics`) and `Inference`/`InferenceSpec` wiring with `inference_schema_id`. It is validation, not derivation.


```rs
//! 💡️ Chart inference admits authored values and owns its semantic result.
use crate::ChartSnapshot;
use semio_framework_value::{DslValue,ToValue as _};
use semio_framework_value_derive::{FromValue,ToValue};
use protocol::{Inference, InferenceSpec, InferenceFieldSpec};
#[path="🎨theme/🦀️.rs"]
pub mod paint;

pub fn validate_chart(snapshot: &ChartSnapshot) -> Result<(), String> {
    static VALIDATOR: std::sync::OnceLock<Result<semio_framework_schema_validator::OwnedJsonSchemaValidator, String>> = std::sync::OnceLock::new();
    let validator = VALIDATOR.get_or_init(|| semio_framework_schema_validator::OwnedJsonSchemaValidator::compile_intrinsic_with_documents(&semio_framework_value_derive::owned_json_file!("../../🧬️schema/📸️snapshot/🔣️.json"), &[semio_framework_value_derive::owned_json_file!("../../🧬️schema/🔣️.json")]).map_err(|error| error.to_string()));
    validator.as_ref().map_err(Clone::clone)?.validate_intrinsic(&snapshot.to_value()).map(|_| ()).map_err(|error| error.to_string())
}

pub fn catalog() -> Result<&'static DslValue, String> {
    static CATALOG: std::sync::OnceLock<DslValue> = std::sync::OnceLock::new();
    Ok(CATALOG.get_or_init(|| semio_framework_value_derive::owned_json_file!("../../🖼️assets/🔣️viz-catalog.json")))
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub struct ChartDiagnostic { pub code:String,pub path:String,pub message:String }
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub struct ChartInference {
    #[value(default,skip_serializing_if="Option::is_none")]
    pub chart:Option<ChartSnapshot>,
    pub diagnostics:Vec<ChartDiagnostic>,
    pub complete:bool,
}
impl Default for ChartInference {fn default()->Self{Self::infer(&ChartSnapshot::default()).expect("owned chart inference describes admission")}}
impl Inference<ChartSnapshot> for ChartInference {
    fn infer(snapshot:&ChartSnapshot)->Result<Self,semio_framework_value::ValueError>{
        let result=validate_chart(snapshot).and_then(|()|match snapshot.chart.get("language").and_then(DslValue::as_str){Some("en"|"de")=>Ok(()),_=>Err("chart language must be explicitly en or de".into())});
        Ok(match result{Ok(())=>Self{chart:Some(snapshot.clone()),diagnostics:Vec::new(),complete:true},Err(message)=>Self{chart:None,diagnostics:vec![ChartDiagnostic{code:"print.chart.inference".into(),path:"chart".into(),message}],complete:false}})
    }
}
impl InferenceSpec<ChartSnapshot> for ChartInference {
    fn inference_schema_id()->&'static str{"framework.print.chart.inference"}
    fn schema_version()->u32{1}
    fn fields()->&'static [InferenceFieldSpec]{&[InferenceFieldSpec{id:"framework.print.chart.inference.chart",reads:&["chart"]},InferenceFieldSpec{id:"framework.print.chart.inference.diagnostics",reads:&["chart"]}]}
}

```

### 3D. Engine reference (trait definitions, not full)

Framework `Inference<P>` and `InferenceSpec<P>` (`📡️spr/🎮️command/🦀️.rs`, lines 27-35 and 88-104):


```rs
/// 🌱️ Bound on [`protocol::value::ToValue`]/[`protocol::value::FromValue`], not `serde::Serialize`/
/// `serde::de::DeserializeOwned` — the same move [`CompositeMutationKind`] below and
/// `protocol::Mutation` itself already made. Every `#[derive(ToValue, FromValue)]` inference type in
/// the plugin tree implements these and no longer implements serde's, so the serde bound left every
/// one of them failing to satisfy this trait.
pub trait Inference<P>: Clone + Default + protocol::value::ToValue + protocol::value::FromValue {
    fn infer(snapshot: &P) -> Result<Self, semio_framework_value::ValueError>;
}

```


```rs
/// 🧬️ Registrable metadata for an artifact's inference family — twin of `ProjectionClass`'s
/// `id`/`schema_version`/`reads` trio (`crate::os_db::projection`), but static: one impl per `XInference`
/// type, declared next to its `Inference` impl.
pub trait InferenceSpec<P>: Inference<P> {
    fn inference_schema_id() -> &'static str;
    /// 🔢️ Salts every cache key derived from this spec's fields — bump when the derivation
    /// algorithm changes so a warm cache never serves a value computed under the old algorithm.
    fn schema_version() -> u32;
    fn fields() -> &'static [InferenceFieldSpec];
}

```

Engine `InferredField<P>` (`💡️inference/🦀️.rs`, lines 128-173):


```rs
/// 🕸️ One inferred field family, computed entity-by-entity over a dependency DAG. This is the
/// trait real derivation math (e.g. a flatten engine) implements; an artifact's top-level
/// `Inference::infer` assembles its `XInference` struct from one or more `InferredField`s.
pub trait InferredField<P>: Send + Sync + 'static {
    type Key: Clone + Eq + std::hash::Hash + Ord + Send + Sync + ToValue + FromValue + 'static;
    type Value: Clone + Send + Sync + 'static;
    type Dependency: ToValue;

    const FIELD_ID: &'static str;
    const SCHEMA_VERSION: u32;

    /// 🗺️ Coarse tier-1 read-set — checked against a diff's [`crate::os_spr::command::DiffRegions::touches`]
    /// before this field's plan is even walked.
    fn reads() -> &'static [&'static str];

    /// 🧭 Deterministic topological plan over `snapshot`'s entities (roots first — entries with no
    /// parents come before anything that depends on them).
    fn plan(snapshot: &P) -> Vec<InferenceStep<Self::Key>>;

    /// 🔑 Owned dependency values for `key` — EXACTLY the snapshot fields `compute` may
    /// read for this key (excluding parents' OWN upstream values, which are folded in separately
    /// via their already-computed [`DepHash`]es — but INCLUDING the specific edge/connector data
    /// tying `key` to each of `parents`, e.g. a compose-style attraction's params, since that lives
    /// on `key`'s own incoming edge, not on the parent's upstream chain). `parents` is `plan`'s
    /// `InferenceStep.parents` for this key, passed through so implementations don't need to
    /// re-derive "which edge connects to which parent" a second time. Honesty contract: this must
    /// cover everything `compute` reads, or a changed-but-uncovered input silently serves a stale
    /// cached value.
    fn dep_input(snapshot: &P, key: &Self::Key, parents: &[Self::Key]) -> Self::Dependency;

    /// 🧮 Pure per-entity compute, given parents' already-computed values in `plan`'s parent order.
    fn compute(snapshot: &P, key: &Self::Key, parents: &[Self::Value]) -> Self::Value;

    /// ⚖️ The bytes a cached `value` is accounted with against the cache budget; the default is the value's inline size.
    fn value_bytes(value: &Self::Value) -> usize {
        let _ = value;
        std::mem::size_of::<Self::Value>()
    }

    /// 🪜 Resumable, fallible variant of [`compute`](Self::compute): consumes at most `fuel` units and either finishes or parks its progress in `pending`
    /// for the next call. The default finishes in one slice at the cost of one unit, so `compute` stays the single semantic source of every field that does not override this.
    fn compute_step(snapshot: &P, key: &Self::Key, parents: &[Self::Value], pending: &mut Option<Box<dyn InferencePending>>, fuel: usize) -> Result<ComputeStep<Self::Value>, InferenceFault> {
        let _ = (pending, fuel);
        Ok(ComputeStep::Done { value: Self::compute(snapshot, key, parents), fuel_used: 1 })
    }
}

```

Diff-gated driver (`💡️inference/🦀️.rs`, lines 526-547):


```rs
/// ⏩ Diff-gated variant: if `diff.touches()` doesn't intersect `F::reads()`, returns the session's
/// previous full result for this field unchanged (tier-1 gate) instead of walking the plan at all.
/// Falls through to [`infer_field`] (and refreshes the session when the result's root moved) otherwise.
pub async fn infer_field_after_diff<P, F, D>(snapshot: &P, diff: &D, session: &mut InferenceSession, cache: &mut InferenceCache) -> BTreeMap<F::Key, F::Value>
where
    F: InferredField<P>,
    D: crate::os_spr::command::DiffRegions,
{
    if !diff.touches().intersects_any(F::reads()) {
        if let Some(stored) = session.roots.get(F::FIELD_ID).and_then(|entry| entry.result.downcast_ref::<BTreeMap<F::Key, F::Value>>()) {
            return stored.clone();
        }
    }
    let mut cursor = InferenceCursor::new();
    let mut values = BTreeMap::new();
    run_to_end::<P, F>(snapshot, Some(cache), &mut cursor, &mut values).unwrap_or_else(|error| panic!("{error}"));
    let root = result_root::<F::Key, F::Value>(&values, &cursor);
    if session.root(F::FIELD_ID) != Some(root) {
        session.roots.insert(F::FIELD_ID, SessionEntry { root, result: Box::new(values.clone()) });
    }
    values
}

```

## 4. Related framework concepts a BIM model needs

- Queries and projections: `🛢️db/📽️projection/🦀️.rs` (`ProjectionClass` at line ~162: `id`, `schema_version`, `dependencies`, `reads`, `affected_by`, `initial`, `apply`; `ProjectionGraph` validates DAG; incremental apply as commands commit; checkpoints with `db_index::ProjectionIndex`; historical "state as of frontier"; ephemeral preview overlay). This is the event-sourced read model. It is separate from inference (inferences are pure per-snapshot, projections are incremental per-commit). `🛢️db/🔍️query/` holds query evaluation and cites `infer_field` for its cache.
- Brep queries (derived geometry, pure): `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/` with `📏mass-properties/` (volume, mass, inertia), `🌳bounding-volume/`, `🔎️analysis/` (`📦️bounds/`, `🛞️mass/`, `📏distance/`, `🩺validity/`, `🌀curvature/`, `🧮️topology/`), `🏷️classification/`, `✅validation/`. Entry: `analyze_shape(body, scope, tolerance)` (`🔎️analysis/🦀️.rs` line ~139). Suitable as the body of a wall-solid or area/volume `compute`. B-Rep kernel: `🧰️framework/🔨️modules/🧊️3d/📐️brep/🦀️.rs` (module doc only).
- B-Rep as an operator evaluation: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/💡️inferences/📐️geometry/🦀️.rs` (contextual service). Its outputs are "transient instance resources" (handles), so they cannot be cached or serialized as values. This is the main obstacle to caching B-Rep solids.
- Long-running work and progress/cancel: `🧰️framework/🔨️modules/🧵️job/🦀️.rs`: `InteractiveJob::step(&mut StepContext) -> StepOutcome` (not async, bounded, 8 ms ceiling, returns `Yield`, `PreviewReady`, `CheckpointReady`, `Complete(CommitCandidate)`, `Cancelled`, `Fault`); `begin_close`/`close_step`. Async runtime and cancel primitives: `🧰️framework/🔨️modules/⏳️async/🦀️.rs` (`OperationContext`, `CancelToken` at line ~255, `WorkerPool`). Inference jobs use `semio_framework_job::MountedWorkerJobSession` (plugin job `💡️infer`).
- Time travel: `🧰️framework/🔨️modules/⏪️time-travel/🦀️.rs` (1069 lines): pure reducer for ephemeral, local-only history editing; `TimeTravelStage` = `Inactive`, `Editing`, `Replaying`, `Reviewing`, `Choosing`, `Finalizing`; replay report; base-moved events. It is history-editing, not dependency tracking. Inferences run against whatever snapshot is replayed, which is consistent with their purity.
- Artifact references: `🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🦀️.rs` (`ArtifactRef { artifact_id, dialect }`, `ArtifactDialect`, `StandardId`, `SubsetId`); used by `ArtifactDeclaration` (`dialect` in the print declaration) and by inference child dependencies (`inference_child_dependency(slot, child_id)` = `child:{slot}/{child_id}` in the request `dependencies`). Cross-artifact derived values must travel through `dependencies` (bytes), not through direct reads.
- Graphs: `🧰️framework/🔨️modules/🕸️graph/` holds graph algorithms, layout runs and drawing (`🧮️algorithms`, `⏯️layout-run`, `🖊️drawing`). It is NOT an inference dependency graph. The dependency DAG for inference is `InferenceStep.parents` plus `DepHash` chains. Equation topology (`🧭topology/`, `compute_equation_topology(&snapshot.graph)`) is a whole-snapshot inference, not an `InferredField`.
- Constraints and solvers: no geometric constraint solver exists in the framework. `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️.rs` has `Constraint`/`ConstraintSpec` (token-level LLM decoding: regex, trie, JSON schema) and `Solver` (diffusion ODE steps). The only constraint-satisfaction solver in the repo is the WFC engine `✏️s/🔌️plugins/🀄️wfc/⚙️engine/`, used by the wfc plugin. For BIM constraints, the existing pattern is a `#[derived]` validity/diagnostics field (print, gltf).

## 5. Gaps for a BIM plugin (what the framework does not yet give)

1. `Inference<P>` is whole-snapshot and pure, so a whole-model recompute on every query is the default. Per-entity caching needs `InferredField` plus an explicit cache, and the cache is opt-in and in-memory only. A persistence adapter was not located.
2. Incremental evaluation after a diff requires `DiffRegions` to be implemented by the artifact's diff type and the caller to hold an `InferenceSession`. Neither was found wired in any production artifact. A BIM plugin would be the first real consumer, or it must wire the session itself.
3. Values must be `Clone + Send + Sync` (no `ToValue` required for values; `Key` requires it). B-Rep handles are transient resources, so wall solids cannot be cached as values. They must be recomputed from the B-Rep operator each time, or cached as the owned encoded result.
4. `InferredField::dep_input` is the honesty contract. For storey-to-wall height derivation, the wall's `dep_input` must include the edge data to its storey, and the storey's own value flows through `parents`. This is the designed mechanism. However, no plugin `InferredField` with non-empty parents was found: equation roots, wfc3d Solve/Contradiction/Entropy all plan with empty parents. Parent folding (`DepHash::chain`, `parent_values`) is exercised only by the engine's own test fixtures (`DagSnapshot`, `SlowSnapshot`). A BIM field with cross-entity parents would be the first real use.
5. No geometric constraint solver. Constraints (for example a wall axis inside its storey) are best modelled as derived validity plus diagnostics, the print chart and gltf pattern.
6. Derived values must never be stored in the snapshot. Use `#[derived]` in the inference schema, as gismap and equation do.
7. Cancellation and progress: the typed `Inference::infer` has no progress or cancel. Progress and cancel come from the service layer (`ArtifactInferenceExecutionRequest.cancellation_id`, `checkpoint` closures in print and gismap, `progress_unit`) or from `InferenceCursor` (`fraction()`, `cancel()`).

## 6. Unverified or limits

- I did not trace the viewer and presentation code (`🪟️presentation`, `🟦️.tsx`) beyond file existence.
- The `InferencePersistence::Projection` option is declared but I did not find an adapter.
- Plugin roster JSON (the committed `🔣️.json` that lists `contributions.inference_services`) was not located under the plugin folders I searched. The roster types were read from `🛂️manifest/🦀️.rs`.
- `AGENTS.md` forbids the search tool. I used `rg` through Bash as instructed.
