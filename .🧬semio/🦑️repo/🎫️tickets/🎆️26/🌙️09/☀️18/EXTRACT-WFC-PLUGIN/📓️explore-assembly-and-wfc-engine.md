# Assembly artifact + wfc-engine: anatomy and dependency audit

Scope: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly` (crate `semio-s-artifact-procedural-assembly`) and its
`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/` module tree, read directly off disk
2026-09-18. All paths below are relative to the repo root `/Users/ueli/Documents/semio` unless given absolute.

---

## 1. Engine API — `🧩️wfc-engine/`

### 1.0 THE SINGLE MOST IMPORTANT FINDING: most of the engine is `#[cfg(test)]`-only

The root module file is
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🦀️.rs`
(121 lines). Every `pub mod` declaration except **seven** carries `#[cfg(test)]`:

```
#[cfg(test)] pub(crate) mod beam;
             pub mod bitset;          // production
#[cfg(test)] pub(crate) mod chunk;
#[cfg(test)] pub mod constraint;
#[cfg(test)] pub mod constraints_card;
#[cfg(test)] pub mod constraints_conn;
#[cfg(test)] pub mod diag;
#[cfg(test)] pub mod domain;
             pub mod error;           // production
#[cfg(test)] pub mod evolve;
#[cfg(test)] pub mod extract;
#[cfg(test)] pub mod flow;
#[cfg(test)] pub mod grid2d;
#[cfg(test)] pub mod grid3d;
#[cfg(test)] pub mod heuristics;
#[cfg(test)] pub(crate) mod hierarchy;
             pub mod ids;             // production
             pub mod job;             // production
             pub mod model;           // production
#[cfg(test)] pub(crate) mod model_vectors;
#[cfg(test)] pub(crate) mod motif;
#[cfg(test)] pub(crate) mod nogood;
#[cfg(test)] pub mod oracle;
#[cfg(test)] pub mod outcome;
#[cfg(test)] pub(crate) mod parallel;
#[cfg(test)] pub(crate) mod prop_ac3;
#[cfg(test)] pub(crate) mod prop_ac4;
#[cfg(test)] pub(crate) mod propagate;
#[cfg(test)] pub(crate) mod repair;
#[cfg(test)] pub mod sample;
#[cfg(test)] pub mod search;
#[cfg(test)] pub mod serial;
#[cfg(test)] pub mod soft;
#[cfg(test)] pub mod solver_graph;
#[cfg(test)] pub mod solver_grid2d;
#[cfg(test)] pub mod solver_grid3d;
#[cfg(test)] pub mod sparse3d;
#[cfg(test)] pub mod symmetry;
#[cfg(test)] pub mod tiled;
             pub(crate) mod topology; // production
#[cfg(test)] pub(crate) mod trail;
             pub mod weights;         // production
```

**A `cargo build` (non-test) of `semio-s-artifact-procedural-assembly` therefore ships only seven
modules: `bitset`, `error`, `ids`, `job`, `model`, `topology`, `weights`.** Everything else — every
grid solver, the arbitrary-graph reference solver (`solver_graph`), symmetry, overlapping-pattern
extraction, tiled-model construction, nogood learning, beam search, repair, hierarchy, parallel
multi-start, evolutionary seed search, oracles, soft scoring, serialization schemas, diagnostics,
motif/color-refinement, flow constraints, connectivity/cardinality constraints, chunk streaming — is
**dead code in every shipped build**, compiled only when the crate's own `cargo test` runs. I
verified there is no Cargo feature gate reproducing this split (`Cargo.toml` at
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust/Cargo.toml` has no such
feature) and confirmed with a repo-wide grep that no file outside `wfc-engine/` itself references
`wfc_engine::search`, `::solver_graph`, `::solver_grid2d`, `::solver_grid3d`, `::grid2d`, `::grid3d`,
`::symmetry`, `::extract`, `::tiled`, `::hierarchy`, `::nogood`, `::repair`, `::parallel`,
`::constraint`, `::domain`, `::oracle`, `::sparse3d`, `::evolve`, `::motif`, `::beam`, `::soft`, or
`::serial` — zero hits.

**What actually solves in production** is a hand-written, fully self-contained incremental solver
state machine inside `job.rs` (see §1.2) that reimplements observe → sample → propagate →
backtrack *itself*, on top of only `model.rs`'s `CompiledModel` and `topology.rs`'s
(production-only) `GraphTopology`. It does **not** call `search.rs`, `propagate.rs`, `prop_ac3.rs`,
`prop_ac4.rs`, `domain.rs`, or `trail.rs` — those are a separate, test-only "reference" solving path
used only to cross-check the interactive job's behavior in unit tests (see the doc comments quoted
in §1.1's `search`/`prop_ac3`/`prop_ac4` entries — each explicitly says it is checked against, or is
itself, a reference implementation).

**Extraction implication**: an execution agent must not assume "the engine" is a coherent shipped
library. It is two things wearing one name: (a) a small production core (7 modules) driving one
particular incremental algorithm, and (b) a much larger, well-documented, well-tested but currently
*inert* reference/experimental library (32 modules) that would need its `#[cfg(test)]` gates removed
and real call sites wired up to ever run in a shipped plugin. Both are real code worth keeping, but
the recommendation section (§4) treats them as two different migration tracks.

### 1.1 Per-module API and semantics (module-level `//!` doc comments quoted verbatim, one-line summaries added)

For each module: path is
`…/💡️inferences/🧩️wfc-engine/<dir>/🦀️.rs`, line counts from `wc -l`.

| module (file) | lines | test-only? | one-line semantics (from the module's own `//!` doc) |
|---|---|---|---|
| `⚖️weights` | 131 | no (prod) | Pattern weight storage: `w`, `ln w`, `w·ln w` per pattern for O(1) incremental Shannon-entropy, plus an optional exact-integer parallel table. Key type `WeightTable`. |
| `⚠️error` | 169 | no (prod) | Flat, non-nested error enums (`ModelError`, `TopologyError`, `ConstraintError`, `SolveError`) — a failed *solve* (`Unsatisfiable`) is never an error, only a `SolveOutcome`. |
| `⛏️extract` | 174 | **yes** | **Overlapping-pattern extraction from 2D tile samples only** (`extract_2d`, `Sample2d`, `Extract2dConfig`, `PatternDecoder2d`, `ExtractedModel2d`): N×N windows → patterns, frequency → weight, 4-offset overlap agreement → compatibility. Reuses `grid2d::declare_stencil_relations` so extracted models line up with `Grid2dTopology`. **No `extract_3d`/3D overlapping extraction exists** (grepped; only `extract_2d` is defined). |
| `⛓️constraint` | 170 | **yes** | `Constraint` trait + `Constraints` closed-enum dispatcher + `PatternSelector`/`AdjacencyView`. Global constraints beyond binary arc compatibility; deliberately stateless (init-once + validate-on-complete-assignment only — no incremental mid-search propagation, "deferred alongside AC-4's rollback integration"). |
| `🀄️tiled` | 86 | **yes** | `TiledModelBuilder`: one authored tile ↔ one pattern (until `symmetry` expands orbits — not yet wired together), `TileId`-facing wrapper over `model::ModelBuilder`. |
| `🆔️ids` | 83 | no (prod) | Typed `u32` newtype macro generating `PatternId`/`TileId`/`NodeId`/`RelationId`/etc so indices can't be silently swapped. |
| `🌊️flow` | 162 | **yes** | `FlowConstraint`: requires ≥N edge-disjoint paths (uniform capacity 1) from a source set to a sink set through nodes matching a selector; hand-rolled Edmonds-Karp max-flow, checked only at completion. |
| `🌐️domain` | 258 | **yes** | `Domain` (live per-node pattern bitset + cached weight sums, O(1) entropy) and `DomainStore`. `RestrictResult` return type lets propagation/search react to wipeouts/singletons without a second query. |
| `🍰️chunk` | 52 | **yes** | `chunk_seed`/`solve_chunk`: deterministic per-chunk seeding + one-chunk solve given fixed seam values from neighbors. Explicitly does **not** manage a chunk registry, world coordinate system, halo-width auto-detection, or boundary-signature cache — "orchestration concerns... deferred: no concrete consumer needs that bookkeeping yet." |
| `🎛️bitset` | 230 | no (prod, used only by test-gated modules today since only `job.rs`'s own inline bit ops touch domains in the shipped path — but the type itself has no `#[cfg(test)]`) | `PatternSet`: hand-rolled word-packed dynamic bitset over `PatternId`, fused restrict-and-collect op. |
| `🎲️sample` | 53 | **yes** | `ValueSampler` enum + `sample_pattern`: which pattern a decision assigns from a domain. |
| `🎼️motif` | 84 | **yes** | Graph motif extraction via iterative 1-WL color refinement (`refine_colors`, `canonicalize`) — explicitly scoped as "the canonicalization primitive, not the full motifs-as-higher-order-patterns pipeline"; turning a signature into a `PatternId` is deferred. |
| `🏁️outcome` | 68 | **yes** | `SolveOutcome`, `RunReport`, `Solution`, `UnsatReport`, `ContradictionReport`, `PartialState` — what a solve attempt concludes with. |
| `🏗️model` | 599 | no (prod) | `ModelBuilder`/`CompiledModel`: pattern universe + directed relation universe + `allowed[relation][source] → PatternSet` compatibility table (+ transpose `supporters`). **`AssemblyModelBuild`/`AssemblyModelPhase`** (line 194-397) is an *incremental, steppable* builder used by the assembly inference job (see §2.4) — this is the one piece of "assembly-specific" logic living inside the otherwise-generic engine. |
| `🐾️trail` | 119 | **yes** | `Trail`/`DecisionFrame`/`Checkpoint`: append-only removal log grouped into decision frames for one-call backtrack undo. |
| `💼️job` | 1907 | no (prod) | **THE production solver.** See §1.2. |
| `💾️serial` | 184 | **yes** | Versioned serde schemas (`SourceModelDoc`, `CheckpointDoc`) that always re-validate through `ModelBuilder`/live model — never trust external bytes directly. |
| `📣️propagate` | 57 | **yes** | `PropQueue`: the propagation worklist shared by `prop_ac3`/`prop_ac4`. |
| `🔁️prop-ac3` | 75 | **yes** | Reference AC-3 bitset arc-revision propagation; "the reference engine every optimized engine is checked against." |
| `🔄️prop-ac4` | 158 | **yes** | AC-4-style support-count propagation, worklist-driven. Explicitly not yet wired into `search`'s backtracking (rollback of the `counts` auxiliary state is unsolved). |
| `🔍️search` | 724 | **yes** | **The "real" generic search driver** (`solve`, `solve_cancellable`, `solve_all`, `solve_with_constraints`, `SearchConfig`, `SearchMode`, `RestartSchedule`, `Budget`, `CancelToken`): observe → sample → propagate → backtrack loop, restarts, budgets, cancellation. This is the module `job.rs` conceptually duplicates for the interruptible/checkpointable use case rather than calling. |
| `🔗️constraints-conn` | 145 | **yes** | `ConnectivityConstraint`/`ReachabilityConstraint` via hand-rolled union-find, checked at completion. |
| `🔢️constraints-card` | 118 | **yes** | `CardinalityConstraint`/`Scope`: bounds how many nodes in a scope match a selector. |
| `🔦️beam` | 148 | **yes** | `beam_search`/`BeamConfig`: incomplete alternative to backtracking search — width-bounded partial-state beam, never returns `Unsatisfiable`, only `Solved`/`Contradiction`. |
| `🔧️repair` | 75 | **yes** | `repair_region`/`halo`: re-solve a bounded neighborhood, pinning everything outside the halo — thin wrapper over `search::solve`. |
| `🔮️oracles` | 135 | **yes** | Brute-force DFS reference enumerator (`enumerate`, `check_assignment`) sharing no code with `propagate`/`search`, used as differential-test ground truth. |
| `🔲️grid-2d` | 337 | **yes** | `Stencil2d`, `Boundary`, `Grid2dTopology`, `declare_stencil_relations[_tiled]`: dense 2D grid, arithmetic neighbor lookup, zero adjacency storage. |
| `🔳️solver-grid-2d` | 154 | **yes** | `Grid2dSolver`/`Grid2dSolverBuilder`: folds masked cells and `Boundary::FixedOutside` into domain overrides/fixed pins, then calls the generic `search::solve`. Has `solve_chunk`, `get`, `decode_tiles`. |
| `🕳️sparse-3d` | 92 | **yes** | `SparseVolume`/`VoxelCoord`: occupied-coordinate set → `GraphTopology` (arc only between two present voxels) — building block for a future variable-resolution octree, explicitly not that octree itself. |
| `🕸️solver-graph` | 165 | **yes** | **`GraphSolver`/`GraphSolverBuilder` — the arbitrary-topology reference solver.** Thin wiring over `search`/`prop_ac3`; has `solve_all`, `resume` (from `Checkpoint`), `repair`, `solve_beam`, `solve_multi_start`. |
| `🗺️topology` | 382 | **partially prod** — `pub(crate) mod topology;` has NO `#[cfg(test)]`, so it ships. `Topology` trait is `pub(crate)`. `GraphTopology`/`GraphTopologyBuilder`/`from_graph_view` are the production arbitrary-graph type. `AssemblyTopologyBuild`/`AssemblyTopologyPhase` (line 135-282) is the incremental steppable builder the assembly job uses. |
| `🚫️nogood` | 304 | **yes** | Two-watched-literal nogood learning on top of decisions (not ordinary propagation reductions) — explicitly scoped as "purely optional, redundant pruning." |
| `🧊️grid-3d` | 328 | **yes** | `Stencil3d`, `Grid3dTopology`: the 2D grid design extended to a third axis, `NodeId(z*w*h + y*w + x)`. |
| `🧬️evolve` | 102 | **yes** | `evolve`/`EvolveConfig`: population-of-seeds evolutionary outer loop over a `SoftConstraint` scorer. Explicitly "evolves only the seed dimension" — weight-field/tileset-parameter evolution was sketched but deferred. |
| `🧭️heuristics` | 66 | **yes** | `ObserveHeuristic`/`select_unresolved`: which unresolved variable to collapse next (linear scan by design). |
| `🧱️solver-grid-3d` | 141 | **yes** | `Grid3dSolver`/`Grid3dSolverBuilder`: exactly `solver_grid2d`'s design extended to 3 axes. |
| `🧵️parallel` | 53 | **yes** | `multi_start`: deterministic sequential-by-index multi-start (explicitly *not* using private threads — "the process-wide worker pool is the only production CPU-thread owner"). |
| `🩺️diag` | 137 | **yes** | `DiagLevel`, `Event`, `EventSink`, `TraceReplay`, `Metrics` — level-gated, zero-cost-when-off diagnostics/replay. |
| `🪜️hierarchy` | 93 | **yes** | `solve_hierarchy`: macro model solved once, then one independent micro/child model per macro node, seeded off `(macro_seed, node)`. No backtrack-to-macro or boundary-contract mechanism yet. |
| `🪞️symmetry` | 267 | **yes** | **2D dihedral group D4** (`Transform2d`, `SymmetryGroup2d`) *and* **3D** (`Transform3d` via 3×3 matrices, `cube_rotations_24`, `cube_symmetries_48`, `SymmetryGroup3d`) — both 2D and 3D symmetry groups exist and are implemented, contrary to what a quick skim might suggest. |
| `🪶️soft` | 107 | **yes** | `SoftConstraint` trait, `ScoreFn`, `best_of_n`/`BestOfNKeep`/`Attempt`, `WeightField` — purely additive post-hoc scoring layer, never affects validity. |
| root `🦀️.rs` | 121 | n/a | Module manifest only (quoted above). |

### 1.2 The production solver: `job.rs` (1907 lines, entirely shipped)

`job.rs`'s own doc: *"🧵️ Persistent, fuel-bounded WFC execution for interactive and batch workers."*
It implements `semio_framework_job::InteractiveJob` for two types:

- **`WfcJob<T: Topology + Clone>`** (struct at line 433) — a from-scratch solve. Its methods are a
  complete, self-contained reimplementation of the WFC kernel as discrete, resumable *steps*, driven
  by `WfcStage` (line 54): `InitializeDomains → FindMinimumEntropySlot → ChooseCandidate →
  PropagateCompatibilityEdge → DetectContradiction → BacktrackTrailEntry → CommitSlot →
  MaterializeCheckpoint → MaterializeCommit → Complete`. Internals: `initialize_one` (717),
  `find_slot` (798, uses a `BinaryHeap<EntropyEntry>` min-entropy heuristic), `choose_one` (809,
  `WfcSampler::WeightedRoulette` production sampler via `JobRng`, an xoshiro-style 4-word PRNG seeded
  deterministically from `snapshot.seed`), `propagate_one` (912, AC-3-style single-arc revision
  inlined, not calling `prop_ac3.rs`), `detect` (990), `backtrack_one` (1002, own inline trail — not
  `trail.rs`'s `Trail` type). Checkpointing is a hand-rolled binary format (`CHECKPOINT_MAGIC =
  b"SWFCJ002"`, versioned header, ≤`MAX_CHECKPOINT_BYTES` = 1 MiB) so a long solve can be paused and
  resumed across turns.
- **`WfcRestore<T: Topology + Clone>`** (struct at line 1244) — decodes a `WfcJob` back out of
  checkpoint bytes (`RestoreStage`, `decode_header`/`decode_one`/`rebuild_one`/`finish`) and hands
  back a live `WfcJob` via `take_job`.
- Public protocol types: `WfcStage`, `WfcPreview` (per-step UI preview: active slot, candidates,
  tested tile, propagation wave, changed domains, contradiction, backtrack path,
  `incomplete_grid: Vec<Option<u32>>`, counters), `WfcCommit` (`assignment: Vec<u32>` +
  observation/edge/backtrack counters), `WfcSampler` (`WeightedRoulette` in prod; `Uniform` only
  under `#[cfg(test)]`), `WfcJobConfig`.

**Solver class shipped**: this is the **arbitrary-graph solver** (generic over `T: Topology`, and the
only production `Topology` impl is `GraphTopology`). There is **no dense grid-2d/grid-3d solver in
production** — `Grid2dTopology`/`Grid3dTopology`/`Grid2dSolver`/`Grid3dSolver` all live only in the
test-gated `grid2d`/`grid3d`/`solver_grid2d`/`solver_grid3d` modules.

### 1.3 What the original `mathematical_wfc` spec asked for vs. what exists

Ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️26/FEATURE-COMPLETE-MATHEMATICAL-WFC-CRATE/🎫️ticket.json`
("closed", real commit date 2026-08-17 for the ticket file itself — the `🎆️26🌙️07☀️26` in its path is
nominal ticket-open dating, not the real commit date; see the "Auto-Commit Message Date Is Fake"
convention) describes a standalone crate `mathematical_wfc` (bundle `mathematical/wfc/`) with:
`GraphSolver`, `Grid2dSolver`, `Grid3dSolver`; AC-3/AC-4/watched-support; MRV/entropy/degree
heuristics; weighted/uniform/temperature/Gumbel sampling; backtracking + backjumping + nogood
learning (2-watched-literals) + beam search; cardinality/connectivity/reachability/path/cycle/
ordering/symmetry/tuple-table/flow constraints; D4 + cube-rotation symmetry; **2D/3D overlapping
pattern extraction**; chunk streaming; hierarchical macro/micro; parallel multi-start; serde
model/checkpoint schemas; strict-integer deterministic mode with golden replay.

**Cross-check against disk**:
- I ran `git log --all --oneline -S "mathematical_wfc"` across the whole repo history: **zero
  commits** ever added or removed that literal string. `find . -iname "*mathematical_wfc*"` and
  `find . -iname "*mathematical/wfc*"`: **no match anywhere**, including inside
  `.🧬semio/🦑️repo/⚡️cache` build artifacts. The only "mathematical" plugin that exists on disk is
  `✏️s/🔌️plugins/➗️mathematical` (equation/algebra/graph/entropy/sampling/CAS crates — a *different*,
  unrelated math-library plugin family; it has no `wfc` subcrate).
- `wfc-engine/`'s own real (not-nominal) git history starts **2026-08-13 15:56:12** (commit
  `1cf6018596`, `git log --date=iso -- <wfc-engine path>`) — **four days before** the
  `FEATURE-COMPLETE-MATHEMATICAL-WFC-CRATE` ticket file's own real commit date
  (2026-08-17 15:59:36, commit `101a6b4ea8`). wfc-engine has had 23 commits total, the latest real
  date 2026-09-12 21:17:59 (commit `8add1df147`).
- **Conclusion (moderate confidence, not fully provable from git alone)**: the standalone
  `mathematical_wfc` crate described by that ticket never existed as files in this repository's
  history. `wfc-engine/`'s module names match the spec almost 1:1 (`grid2d`, `grid3d`, `symmetry`,
  `nogood`, `beam`, `repair`, `chunk`, `hierarchy`, `parallel`, `flow`, `sparse3d`, `motif`, `evolve`,
  `tiled`, `oracle`, `soft`, `diag`, `heuristics`, `search`, `prop_ac3`/`prop_ac4`) and is almost
  certainly the actual realization of that spec, **authored directly inside the assembly artifact's
  inference tree from the start**, never as an independent crate. The ticket's "closed"/"270 tests
  passing" summary describes real work; it just landed at a different path than its own title
  implies, and — per §1.0 — the bulk of it was placed behind `#[cfg(test)]` rather than wired into
  a shipped module (so it exists as code and as passing tests, but not as shipped functionality).
- **Genuine gap vs. the spec, not just a test/prod-gating issue**: no 3D overlapping pattern
  extraction (`extract_3d`) exists anywhere — `⛏️extract/🦀️.rs` defines only `extract_2d`. No
  path/cycle/ordering/tuple-table constraints exist (only connectivity, reachability, cardinality,
  flow). No temperature/Gumbel sampling (`ValueSampler` in `🎲️sample` — check that file directly if a
  sampling-strategy migration needs the exact variant list). No conflict-directed backjumping
  distinct from nogood learning was found as a separately named mechanism. No strict-integer
  deterministic golden-replay mode was located as a distinct feature (only `WeightTable`'s optional
  exact-integer parallel columns, which is a narrower thing).

---

## 2. Assembly artifact anatomy

Root: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🦀️.rs` (445 lines). Crate:
`semio-s-artifact-procedural-assembly` (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust/Cargo.toml`).
Feature `component-app-assembly` (optional deps: `semio-framework-os-flow`,
`semio-framework-os-infinite`, `semio-framework-ui[-contract|-styling]`) gates `editor`/`viewer`
modules; without it only the schema/mutations/inferences/io surface builds.

Root doc comment: *"Assembly artifact — a WaveFunctionCollapse-style rule/slot composition engine.
Authored fresh (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, packet W2-P5): unlike
generation2d/generation3d, this artifact never had an apps tree to migrate — the schema/mutations/
inferences tree under `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/` predates this ticket and is reused
as-is; only this file plus editor/viewer are new."*

### 2.1 Identity (root file, lines 20-105)

- `ASSEMBLY_DIALECT: Dialect { artifact_kind: ASSEMBLY_DOCUMENT_SCHEMA /* "s.assembly" */, standard:
  "1", subset: SubsetId::ANY }` (line 31).
- `artifact_kind()` → `ArtifactKindSpec { id: "data.assembly", component_kind: "assembly", dimension:
  "data", media_capability: MeshOnly, ... }` (line 39) — headless, no flow-graph fixture.
- `definition()` → `ArtifactDefinition::new(ArtifactIdentity::parse("s.procedural.assembly")?)` with
  capabilities `schema.artifact` (descriptor `b"s.assembly"`), `inference.artifact` (descriptor
  `b"s.assembly.solve"`), `composer.native` (descriptor `b"s.assembly@1/*"`), `codec.document`
  (descriptor `b"s.assembly:assembly"`), `localization.en`/`localization.de` ("Assembly"/"Montage").
  **Two distinct identity strings coexist by design**: the plugin-owner-qualified
  `"s.procedural.assembly"` (used for `ArtifactIdentity`/capability ids and in
  `schema-catalog.json`'s `"s.procedural.assembly.1.any…"` keys) vs. the artifact's own dialect
  string `"s.assembly"` (used in `ASSEMBLY_DIALECT`, the GraphQL/example-picker/mcp-inference-test
  fixtures, and `.vscode/launch.json`'s `SEMIO_APP=s.assembly@1/*#editor`). An extraction must decide
  the new plugin's owner prefix (presumably `s.wfc.*` per artifact) and update **both** strings
  consistently everywhere they appear (§3).
- `declaration()` (feature-gated, line 109) assembles the typed `ArtifactDeclaration`: `.schema(...)`,
  `.inferences([assembly_artifact_inference_descriptor()])`, `.composers(io_registry::entries())`,
  `.document_codec_bare::<AssemblySnapshot, AssemblyMutation>(...)`.
- Lines 118-131: a documented, deliberate gap — no GraphQL/JSON-Schema/Protobuf *artifact-facet*
  descriptor file exists (`schema/🦀️component.rs`-equivalent), unlike `energy.model`'s precedent;
  out of scope for the packet that authored this file. Relevant if the new wfc plugin's artifacts
  need that facet from day one.
- `module_child_handle(module_id)` (line 62): mints a `store::ArtifactChild<...SemioKitSnapshot>`
  pointing at `s.stdio.semio@v1/kit` — **modules are never embedded inline; `AssemblySnapshot.modules`
  holds handles into the stdio-kit artifact family.** This is the actual "tile content" reference
  mechanism (a tile/module is a whole kit sub-document, not a bitmap/mesh field on the snapshot
  itself).

### 2.2 Document model — `AssemblySnapshot` (schema/GraphQL: `🧬️schema/🔗️.graphql`; Rust:
`🧬️schema/📸️snapshot/🦀️.rs`)

```graphql
type AssemblySnapshot {
  schema: String!
  seed: Float!
  slots: [AssemblySlot!]!          # id, x, y, z, pinnedModuleId
  edges: [AssemblySlotEdge!]!      # id, fromSlotId, toSlotId
  modules: [ArtifactChildHandle!]! # childId -> ArtifactRef{artifactId, dialect} into s.stdio.semio kit
  weights: [AssemblyModuleWeight!]!# moduleId, weight
  rules: [AssemblyRule!]!          # id, moduleAId, moduleBId, allowed: Boolean!, params: SemioValue
}
```

Rust side (`📸️snapshot/🦀️.rs`) adds `slot_index`/`edge_index`/`rule_index`/`weight_index` lookup
helpers and a `Default` impl. **This is an arbitrary-neighbor-graph document model** — slots carry
free `(x, y, z)` positions and are connected by an explicit `edges` list (not a fixed grid stencil),
and modules are mesh/kit references, not bitmaps. `AssemblyRule` is **module-pair-scoped and
direction-agnostic** (`moduleAId`/`moduleBId`/`allowed`, no relation-type or edge-direction field) —
see §2.4 for how this compiles to the engine's single `RelationId(0)`.

### 2.3 Mutations (9 total, each a `create/delete/change/connect/disconnect` leaf directory under
`🧬️schema/🧬️mutations/` with `component`+`diff`+`inverse` submodules — verified from the root file's
`#[path]` tree, lines 138-284):

| mutation | folder | fields (from `AssemblyEditorCommand`, editor root file lines ~29-45) |
|---|---|---|
| `create_slot` | `🧩️create-slot` | `index, id, x, y, z, pinned_module_id: Option<String>` |
| `delete_slot` | `🕳️delete-slot` | `id` |
| `create_rule` | `🚦️create-rule` | `index, id, module_a_id, module_b_id, allowed` |
| `delete_rule` | `❌delete-rule` | `id` |
| `change_weight` | `🔢️change-weight` | `module_id, weight` |
| `remove_weight` | `🪶️remove-weight` | `module_id` |
| `connect_slots` | `🔗️connect-slots` | `index, id, from_slot_id, to_slot_id` |
| `disconnect_slots` | `✂️disconnect-slots` | `id` |
| `change_seed` | `🎲️change-seed` | `seed: u64` |

`AssemblyMutation` enum + `apply_assembly_mutation`/`inverse_assembly_mutation` live in
`🧬️schema/🧬️mutations/🦀️.rs` (component file). `AssemblyDiff` (`🔺️diff/🦀️.rs`, 15-160) implements
`protocol::MutationDiff<AssemblySnapshot>` with generic `merge_upserts`/`apply_collection`/
`apply_unordered_collection` helpers shared across the slot/edge/rule/weight collections.

### 2.4 Inference wiring — `🧬️schema/💡️inferences/🦀️.rs` (826 lines)

Doc comment (lines 1-9): *"THE SOLVE ITSELF IS AN INFERENCE... `AssemblySnapshot` only ever
persists the PROBLEM; the SOLUTION, the contradiction/unsat verdict, and the pre-propagation entropy
map are all derived here via `store::InferredField`, never mutation-authored state. The 10,930 LOC
WFC implementation in the sibling `../🧩️wfc-engine/` compute tree becomes the internals of these
`compute()` bodies."*

**Two parallel inference surfaces coexist:**

1. **Three synchronous `store::InferredField<AssemblySnapshot>` impls** (lines 710-826), each with a
   `FIELD_ID`, `SCHEMA_VERSION`, `reads()`, `plan()`, `dep_input()` (feeds `DepHash` caching),
   `compute()`:
   - `AssemblySolve` (`FIELD_ID = "s.assembly.inference.solve"`) → `AssemblySolveResult::{Unsolved,
     Solved{assignments: BTreeMap<String,String>}}` (slot id → module id), via `solve_with_job`.
   - `AssemblyContradiction` (`"s.assembly.inference.contradiction"`) → satisfiability verdict.
   - `AssemblyEntropy` (`"s.assembly.inference.entropy"`) → pre-propagation Shannon entropy over
     modules, via `shannon_entropy_over_modules` (line 812).
   - `solve_with_job` (line 636) builds an `AssemblyInferenceJob`, drives it headless through a
     `BatchJobSession` with a bounded step budget (`fuel_per_step: 1, step_budget_us: 2000`),
     collects the terminal `StepOutcome::Complete` payload as JSON. **Determinism claim in the doc
     comment**: reads only `snapshot` fields (seed included), same resumable `WfcJob` interactive
     callers use, every step watchdog-wrapped and explicitly bounded — "No ambient randomness enters
     the inference, so `DepHash` caching... is sound."

2. **One routed (async, cold-job) inference**: `AssemblyInferenceJob` implements
   `semio_framework_job::InteractiveJob` directly (lines 91-579, its own `AssemblyInferenceStage`
   pipeline distinct from `WfcStage`: `Weights → Modules → Rules → Model → Slots → Edges → Topology →
   Fixed → {Solve|Restore} → EncodeCommit → …`). Registered as a `ToolJobFactory`
   (`AssemblyInferenceJobFactory`, line 576) via `register_assembly_inference_factory(bus:
   &ActionBus)` (line 631, `bus.register_once(...)`), and declared to the plugin builder via
   `.routed_inference(assembly_inference_metadata())` (metadata built at line 38,
   `ArtifactInferenceServiceMetadata`).

**What `advance_compile` (lines 226-344) does — the exact snapshot→engine wiring an extraction must
reproduce**:
   - `Weights` stage: `AssemblySnapshot.weights[]` → `weight_by_id: HashMap<String,f64>`.
   - `Modules` stage: `AssemblySnapshot.modules[]` → one `PatternId` per module (index-order),
     `raw_weights[i] = weight_by_id.get(module_id).unwrap_or(1.0)`.
   - `Rules` stage: for each `allowed == true` rule, inserts **both** `(a,b)` and `(b,a)` into a
     `BTreeSet<(u32,u32)>` — i.e. rules are compiled as a **symmetric, undirected** allowed-pair set,
     then fed to `wfc_engine::model::AssemblyModelBuild::new(weights, pairs)` (the incremental
     model-compile state machine at `model.rs:203-397`), stepped to completion in the `Model` stage.
   - `Slots` stage: `AssemblySnapshot.slots[]` → one `NodeId` per slot (index-order), building
     `wfc_engine::topology::AssemblyTopologyBuild::new(slot_count)`.
   - `Edges` stage: for each edge, `add_arc(from, to, RelationId(0))` **and**
     `add_arc(to, from, RelationId(0))` — again symmetric/undirected, and always the *single* relation
     id 0 (the engine's multi-relation capability is present in `model.rs`/`topology.rs` but assembly
     never uses more than one relation).
   - `Topology` stage: steps `AssemblyTopologyBuild` to a `GraphTopology`.
   - `Fixed` stage: `slot.pinned_module_id` → `(NodeId, PatternId)` fixed pins.
   - Then either `wfc_engine::job::WfcRestore::new(...)` (if resuming a checkpoint) or
     `wfc_engine::job::WfcJob::new(operation, model, topology, WfcJobConfig::default(), None,
     fixed)` — i.e. **assembly drives the production `GraphTopology` + `WfcJob` path directly**, never
     the test-only grid/tiled/symmetry machinery.

`assembly_inference_metadata()` (line 38) and the framework's "routed inference" contract (see §2.5)
are the two things a new wfc plugin's own artifacts must re-declare per artifact kind.

### 2.5 The "routed inference" framework contract

Defined in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:280` (`Plugin::builder(...)
.routed_inference(metadata: ArtifactInferenceServiceMetadata) -> Self`), doc comment: *"🧭️ Advertises
one inference implemented by the exact `semio.infer/<schema>` cold-job route without manufacturing a
second synchronous executable."* `ArtifactInferenceServiceMetadata` (struct,
`🧰️framework/…/🔌️plugin/🦀️.rs:1427`): `{ owner, artifact_kind, artifact_schema,
artifact_schema_version, inference_schema, inference_schema_version, algorithm_version,
policy_version }` — all `&'static str`/`u32`.

**Contract, reconstructed from the builder + its own test
(`🏗️builder/🧪️tests/🔬️schema-stamping/🦀️.rs:138`,
`routed_inference_is_frozen_into_the_plugin_roster_without_a_sync_service`)**: declaring
`.routed_inference(metadata)` freezes the `(artifact_kind, inference_schema)` pair into the plugin's
roster (`PluginRuntimeRegistry.routed_inferences: Vec<ArtifactInferenceServiceMetadata>`,
`🔌️plugin/🦀️.rs:4031-4114`) **as metadata only** — it tells the host "this artifact kind's named
inference schema is answered by a cold job (started through `⚛️reactor/💼️jobs::start_job`, keyed by
the `semio.infer/<schema>` route), do not expect a synchronous `ArtifactInferenceService`/`infer()`
executable for it." The builder rejects a conflict if the same `(artifact_kind, inference_schema)` is
also declared via a *contributed* synchronous inference (`🔌️plugin/🦀️.rs:4098-4114`). The actual
cold-job execution is wired separately by registering a `ToolJobFactory` on the `ActionBus`
(`register_assembly_inference_factory`) — `routed_inference()` only makes the roster/discovery layer
(`wire_list_artifact_inference_services_with_routes`, `🔌️plugin/🦀️.rs:30593`, and the MCP inference
tool tests at `🧰️framework/…/🌉️mcp/💡️inference/🧪️tests/🔬️quick/🦀️.rs`) aware the route exists, so a
client asking "how do I get `s.assembly.solve`?" is told to call the job route rather than getting a
synchronous answer.

Note assembly uses **both** mechanisms simultaneously: the routed cold job (`AssemblyInferenceJob`,
for a UI-driven full/steppable solve with preview) *and* three ordinary `InferredField` impls
(`AssemblySolve`/`AssemblyContradiction`/`AssemblyEntropy`, for small bounded solves computed
synchronously and DepHash-cached) — the `InferredField` compute bodies call the **same**
`solve_with_job`/`WfcJob` machinery headlessly rather than being a third independent solve path.

### 2.6 Editor / Viewer surfaces

- `✏️editor/🦀️.rs` (139 lines): `AssemblyEditorCommand` enum, one variant per mutation (9 variants,
  matching §2.3 exactly — *"no synthetic 'set field' indirection, since the domain's own mutations
  are already exactly this granular"*). `AssemblyEditor` implements `ArtifactEditor` with
  `Config/Draft/Presence/Transient = No*` (no per-window state at all) and
  `DIALECT = ASSEMBLY_DIALECT`. One mode, `edit`, one window, `structure` (`TreeWindowKit`,
  editable). `create_assembly_editor()` builds the `AppDefinition`.
- `👁️viewer/🦀️.rs` (92 lines): `AssemblyViewCommand` is a single inert `Noop` variant — *"the
  viewer declares no actions... same shape as energy.model's own view-command precedent."*
  `AssemblyViewer` implements `ArtifactViewer`, same dialect/schema, one mode `view`, one window
  `structure` (read-only).
- Both windows live at
  `✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/🦀️.rs` (88 lines) and
  `👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️structure/🦀️.rs` (63 lines).

**How the solve result is projected to the UI: it currently is not.** The `structure` window's own
doc comment states this explicitly: *"a real, EDITABLE overview tree of the whole WFC problem spec
(seed/slots/edges/modules/weights/rules — **never the solved assignment, which is an inference, not
persisted state**)... no module ASSIGNMENT is ever stored on the snapshot... so a mesh view would
have nothing solved to place — a rule/slot tree is the honest first-pass representation of the
PROBLEM this artifact actually persists. A spatial view over slots' raw coordinates is a plausible
follow-up, not a purity or completeness requirement for this packet."* I grepped both `✏️editor` and
`👁️viewer` trees for `AssemblySolve|AssemblyContradiction|AssemblyEntropy|InferredField|solve(` outside
tests: **zero hits**. The `AssemblySolve`/`AssemblyContradiction`/`AssemblyEntropy` `InferredField`s
and the routed `AssemblyInferenceJob`'s `WfcPreview`/`WfcCommit` streaming exist and are tested, but
nothing in the editor or viewer subscribes to them — there is no mesh/spatial render of a solved
assignment anywhere in this artifact today. Any of the five new wfc artifacts that want a "watch it
solve" viewer experience must build that subscription from scratch; it is not something to preserve
from assembly, because assembly never had it.

### 2.7 Everything else (lower priority, read at directory-listing depth)

- `📚️examples/`: two bundled problems, `🚪️two-room-corridor` and `🧱️wall-roof-facade-strip`
  (`examples::sources()` in root file, line ~410-420), each with its own Rust builder as "the
  authority its `.dsl.semio` asset is printed from."
- `🚪️io/`: only text (`stdio.txt`) import/export serializers under
  `📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/` and the mirror export path — no
  binary/mesh codec.
- `🔮️oracles/` (artifact-level, distinct from `wfc-engine/🔮️oracles/`), `🧫️fixtures/`, `🧪️tests/`
  (`mount-contract`, `mutate-assembly-1`): present but not read in depth for this pass; an execution
  agent extracting the crate will need to walk these directly (small, self-contained fixture/test
  dirs, low risk).
- `📦️packages/🦀️rust/`: `Cargo.toml` (quoted in §2.0), `project.json` (Nx target), `script.ts` (Nx
  script codegen — also the source of the `s.assembly.solve` tool-run-payload registration row seen
  in `📜️script.ts:9816`, §3).

---

## 3. External referrers — every site outside the assembly folder that names it

Grepped the whole repo (excluding `node_modules`, `target`, `dist`, `⚡️cache`, `🗑️generated`,
`🎫️tickets`, `.git`) for `procedural_assembly`, `procedural-assembly`, `s\.assembly`,
`AssemblyEditor`, `AssemblyViewer`, `assembly_inference`, `s\.procedural\.assembly`:

| term | file | what it is |
|---|---|---|
| `procedural_assembly` | `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` | **the plugin registration site** — `ProceduralApps::AssemblyEditor/AssemblyViewer` enum variants (line 36-37), `register_assembly_inference_factory(&ActionBus::production())` (line 92), `.routed_inference(assembly_inference_metadata())` (line 97), `.artifact(...declaration())` (line 100), `.editor_with_examples::<AssemblyEditor>(...)`, `.editor_mutation_roster::<AssemblyEditor>()`, `.viewer::<AssemblyViewer>(...)`, `.viewer_mutation_roster::<AssemblyViewer>()`, `.activation(ActivationEvent::OnArtifactKind{kind: artifact_kind().id})` (lines 110-127). **This whole block is the template an extraction must replicate in a new `✏️s/🔌️plugins/🌊️wfc/🦀️.rs` root**, once per new artifact. |
| | `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🔬️surface/🦀️.rs` | Surface-law tests: `assembly_editor_and_viewer_share_dialect` (line 66), `assembly_manifest_examples_are_registered_on_the_editor_surface` (71), `assembly_apps_are_declared_on_the_plugin` (83) — asserts `editor.id == "s.assembly@1/*#editor"`, app ids on the plugin roster. Must be ported (or replaced by equivalent per-new-artifact tests). |
| `procedural-assembly` (crate-name form) | `Cargo.toml`, `Cargo.lock` (workspace root) | Workspace member registration. |
| | `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` | procedural plugin's own crate depends on `semio-s-artifact-procedural-assembly`. |
| | `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🟦️.ts:26` | **Negative** assertion — demonstrator's cargo receipt must **not** contain `semio-s-artifact-procedural-assembly` (it's deliberately excluded from the 8-pane demonstrator bundle already). Unaffected by extraction as long as the new wfc artifacts also stay out of that bundle, or the assertion is updated if one should join it. |
| `s\.assembly` (dialect string) | `📜️script.ts:9816` | Nx/tool-run-payload registration row: `{ toolId: "s.assembly.solve", root: ".../🧬️schema/💡️inferences", scope: [...], lane: "W3 (3) assembly", inventory: "§2.3" }` — a build-tooling manifest row that must gain equivalent rows for each new wfc artifact's inference tool. |
| | `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/📚️example-picker.json` (lines 5, 22) | Example-picker fixture: `{"id":"two-room-corridor","dialect":{"artifactKind":"s.assembly","standard":"1","subset":"*"}}` and an `app: {"id":"s.assembly@1/*#viewer", ...}` row — a fixture a new plugin's examples must add equivalents to (or this one stays as-is if the assembly artifact is deleted and its example dropped). |
| | `🧰️framework/…/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs:1` | Doc comment only: *"`semio.infer` cold-job bridge. Exact ActionBus routes such as `s.assembly.solve`"* — assembly is used as the canonical example in a doc comment; no functional dependency, just needs the example to stay a true statement or get swapped for a wfc-plugin example. |
| | `🧰️framework/…/🌉️mcp/💡️inference/🧪️tests/🔬️quick/🦀️.rs` (lines 111-190) | MCP inference-tool tests use `"s.assembly"`/`"s.assembly.solve"` as their concrete fixture artifact kind/schema throughout (`declared[0].artifact_kind`, `inference_get` call, `lookup_inference` assertions). **If assembly is deleted, these tests break** and need a replacement fixture artifact kind (either one of the new wfc artifacts, or a synthetic test-only fixture). |
| | `✏️s/🔌️plugins/🌀️procedural/🔣️.json` | Some generated/committed manifest JSON referencing `s.assembly` — inspect before deleting. |
| | `♻️mit-bestand/🔎️recherche/_archive/research/plattform/entwurf/knowledge/piece_bauteilpass_interface.md` | Archived research note — historical, not load-bearing. |
| | `.vscode/launch.json` (lines 3612, 3634) | Two debug-launch configs with `"SEMIO_APP": "s.assembly@1/*#editor"`. Update or add equivalents for new wfc artifacts. |
| `AssemblyEditor`/`AssemblyViewer` (Rust type names) | same two procedural-plugin files as above | No other referrers repo-wide. |
| `assembly_inference` | `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` only | Same registration lines as above. |
| `s\.procedural\.assembly` (owner-qualified identity string) | `🧰️framework/…/🔌️plugin/🏗️builder/🧫️fixtures/🪪️artifact-admission/🔣️.json:248` | Fixture: `"kind": "s.procedural.assembly"` — an admission-contract test fixture; needs an equivalent row per new artifact kind (or removal). |
| | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md`, `🔣️schema-catalog.json` (10 keys: `s.procedural.assembly.1.any`, `.1.any.mutation.change-seed`, `.change-weight`, `.connect-slots`, `.create-rule`, `.create-slot`, `.delete-rule`, `.delete-slot`, `.disconnect-slots`, and presumably `.remove-weight` truncated in this grep) | **This catalog appears to be auto-generated** by a schema-export step (candidate generator:
`🧰️framework/🔨️modules/🧬️schema/🧪️tests/📤️schema-export-entries/🦀️.rs`, found via a secondary grep for
"schema-catalog.json" references — not confirmed by reading that generator's code in this pass). If
so it should self-update on a rebuild after extraction/deletion rather than needing hand edits;
**verify this assumption before treating it as a manual-edit site.** |
| | `✏️s/🔌️plugins/🌀️procedural/🔣️.json` | Same file as above, second hit. |

**Summary of what an extraction/deletion must touch**: the procedural plugin root
(`✏️s/🔌️plugins/🌀️procedural/🦀️.rs`) and its surface test are the only two files with *structural*
Rust dependencies on the assembly artifact crate. Everything else is fixture/manifest/doc-comment/
generated-catalog surface area that references the `"s.assembly"`/`"s.procedural.assembly"` *strings*
and must be updated (or regenerated) to match whatever identity strings the five new wfc artifacts
adopt — none of it requires the assembly Rust types themselves.

---

## 4. Recommendations

### 4.1 What moves verbatim into `✏️s/🔌️plugins/🌊️wfc` as a shared engine crate

Everything in `wfc-engine/` is domain-agnostic already (it operates on `PatternId`/`NodeId`/
`RelationId`, never on assembly's `AssemblySlot`/`AssemblyRule` types) **except** two pieces that are
assembly-specific and must NOT move as-is:
- `model.rs`'s `AssemblyModelPhase`/`AssemblyModelBuild` (lines 194-397) — an incremental compile
  state machine specifically named/shaped around assembly's weights→patterns→rules pipeline (§2.4).
  The *pattern* (steppable, interruptible builder) is worth keeping generically, but it should be
  renamed and, if the five new artifacts have different compile shapes (e.g. bitmap's overlapping
  extraction vs. grid's stencil declaration vs. graph's arbitrary rules), likely needs to become
  either several builders or a more generic one parameterized over how patterns/relations are
  produced.
- `topology.rs`'s `AssemblyTopologyPhase`/`AssemblyTopologyBuild` (lines 135-282) — same situation,
  specifically an incremental *arbitrary-graph* topology builder. This one is directly reusable
  as-is by the two "arbitrary graph" artifacts (2d, 3d) since it already builds a generic
  `GraphTopology`; only its name (`Assembly*`) needs to stop implying it's assembly-only.

Everything else — `bitset`, `error`, `ids`, `weights` (production core); `search`, `propagate`,
`prop_ac3`, `prop_ac4`, `domain`, `trail`, `constraint`+`constraints_card`+`constraints_conn`,
`flow`, `nogood`, `beam`, `repair`, `hierarchy`, `parallel`, `evolve`, `soft`, `oracle`, `diag`,
`heuristics`, `serial`, `sample`, `chunk`, `motif` (currently-inert reference/experimental modules);
`grid2d`, `grid3d`, `solver_grid2d`, `solver_grid3d`, `symmetry`, `extract`, `tiled`, `sparse3d`,
`solver_graph` (the actual per-topology solvers and the overlapping-extraction pipeline) — is
generic and should move as one shared `semio-s-plugin-wfc-engine`-style crate (or a `wfc-engine`
module inside the new plugin, mirroring how it lives today) that all five artifacts depend on.
**Critical follow-up work, not optional**: remove the `#[cfg(test)]` gates from whichever of these
modules a given new artifact actually needs at runtime (grid2d/grid3d/solver_grid2d/solver_grid3d for
the two `*-grid` artifacts; extract+grid2d for `bitmap`'s overlapping model; symmetry for anything
wanting tile-orbit expansion) — today none of them run in any shipped binary, and simply copying the
directory across without lifting the gate reproduces the exact same "10,930 LOC of dead code" problem
in the new plugin. `job.rs`'s hand-rolled incremental `WfcJob`/`WfcRestore` state machine currently
duplicates `search.rs`'s logic for the interruptible case; decide once, for the new plugin, whether
to keep two independent implementations (current state) or make the interruptible job a thin
step-driver over the reference `search`/`propagate` modules (more correctness confidence, more
refactor risk) — this is a judgment call for whoever plans the actual migration, not something this
audit can resolve from reading alone.

### 4.2 What is per-artifact (new code, not moved)

- The five artifacts' own `AssemblySnapshot`-equivalent document schemas (GraphQL/Rust/mutations/
  diff), each shaped around its stencil model:
  - **bitmap**: sample bitmap(s) + `Extract2dConfig`-shaped params (N, periodic, symmetry flags) —
    needs `extract.rs` lifted out of `#[cfg(test)]` and likely extended (no 3D overlapping exists to
    reuse if a "bitmap" artifact ever wants a volumetric sibling; confirm whether the ticket's 5
    artifacts include one before assuming 2D-only is sufficient).
  - **2d-grid**: `grid2d.rs` + `solver_grid2d.rs`, lifted out of `#[cfg(test)]` — this is the closest
    1:1 existing match to a new artifact of anything in the engine.
  - **2d** (arbitrary graph, non-rectangular, graph-with-slots editor): **this is what
    `AssemblySnapshot`/`AssemblyModelBuild`/`AssemblyTopologyBuild`/editor/viewer already are**,
    minus the z-coordinate and minus assembly's specific mesh/kit module-reference mechanism if `2d`
    wants flat vector-graphic tiles instead. The slot/edge/rule/weight/seed document shape, the 9
    mutations, the editor's one-command-per-mutation pattern, and the `TreeWindowKit` structure
    window are all directly reusable *templates* (not code to literally keep using
    `Assembly*`-named types, but the closest available precedent to copy from).
  - **3d-grid**: `grid3d.rs` + `solver_grid3d.rs`, lifted out of `#[cfg(test)]` — direct match,
    exactly as 2d-grid.
  - **3d** (arbitrary graph, mesh tiles): **this is what `AssemblySnapshot` already is, essentially
    unchanged** — slots already carry full `(x,y,z)`, modules are already mesh/kit references via
    `ArtifactChildHandle` into `s.stdio.semio@v1/kit`. Per the goal context's own hint ("probably
    2d/3d arbitrary-graph") — confirmed: **assembly's actual document model is the `3d`
    arbitrary-graph artifact almost verbatim, and the `2d` arbitrary-graph artifact is assembly with
    z dropped and modules swapped for a 2D-appropriate reference.** Whichever of {2d, 3d} is
    implemented first, the other is a near-copy.
- Each artifact's own inference wiring (`InferredField` impls + a routed `*InferenceJob`/
  `ToolJobFactory`, per §2.4-2.5) and plugin-registration block (per §3's `✏️s/🔌️plugins/🌀️procedural/🦀️.rs`
  template) — five near-identical copies, one per artifact, registered on a new
  `✏️s/🔌️plugins/🌊️wfc/🦀️.rs` root analogous to the procedural root.
- A UI projection of the solve result (mesh/spatial render of a `Solved{assignments}` outcome) —
  genuinely new work for every artifact; assembly never built this (§2.6), so there is nothing to
  port here, only a gap to close from scratch, and it's probably the single highest-value net-new
  feature the extraction should deliver (a WFC plugin whose editor can't show you the solve is a
  weak pitch).

### 4.3 Delete assembly from procedural after extraction

The repo is greenfield (no compatibility layers wanted). Once the wfc-engine tree and (at minimum)
the `2d`/`3d` arbitrary-graph artifacts exist in the new `🌊️wfc` plugin with equivalent or better
functionality, **delete** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly` entirely and:
- Remove the `AssemblyEditor`/`AssemblyViewer` variants from `ProceduralApps`, the
  `register_assembly_inference_factory`/`.routed_inference(...)`/`.artifact(...)`/
  `.editor_with_examples::<AssemblyEditor>`/`.viewer::<AssemblyViewer>`/`.activation(OnArtifactKind
  {assembly})` lines, and the three `assembly_*` surface tests in
  `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` and `🧪️tests/🔬️surface/🦀️.rs` (§3).
- Remove the crate from the workspace `Cargo.toml`/`Cargo.lock` and
  `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`'s dependency list.
- Update/remove the `example-picker.json` fixture rows, the MCP inference test's `s.assembly`
  fixture (swap for a wfc-plugin artifact or a synthetic fixture — §3), the `artifact-admission`
  fixture row, `.vscode/launch.json`'s two `SEMIO_APP=s.assembly@1/*#editor` entries, and the
  `📜️script.ts` tool-run-payload row.
- Let `schema-catalog.json`/`.md` regenerate (verify the generator first, per §3's open item) rather
  than hand-editing, if confirmed auto-generated.
- No change needed to the demonstrator compile-closure test (`♻️mit-bestand/…`) — it already asserts
  assembly is absent from that bundle.

This is a clean, fully-enumerated deletion: the structural Rust dependency surface is exactly one
plugin root file plus its own surface test (§3), and every other referrer is fixture/manifest/string
data that either regenerates or needs a one-line swap.
