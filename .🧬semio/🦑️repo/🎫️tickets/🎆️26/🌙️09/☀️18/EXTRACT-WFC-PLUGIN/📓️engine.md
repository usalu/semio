# 🀄️ Slice E — the shared engine crate `semio-s-plugin-wfc-engine`

Crate: `semio-s-plugin-wfc-engine` (Rust ident `semio_s_plugin_wfc_engine`)
Root: `✏️s/🔌️plugins/🀄️wfc/⚙️engine/🦀️.rs` · manifest `✏️s/🔌️plugins/🀄️wfc/⚙️engine/📦️packages/🦀️rust/Cargo.toml`
Rebuild script (idempotent, re-runnable from scratch): `📜️engine-extract.sh` + templates `🗂️engine-root.rs.txt`, `🗂️engine-grid-job-drive.rs.txt` in this ticket folder. Logs: `🗑️generated/engine/`.

> The plugin folder was renamed `🌊️wfc` → `🀄️wfc` mid-slice and, separately, wiped once by a peer process. The whole slice is reproducible by running `zsh "$T/📜️engine-extract.sh"` — it copies from procedural, rewrites, and drops the two template files in place. `DST` in that script is the single place the plugin folder name lives.

## 1. What was copied

`cp -R` (never `mv` — procedural's copy is untouched and slice C deletes it) from
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/`
into `✏️s/🔌️plugins/🀄️wfc/⚙️engine/`, every emoji folder name preserved, every `🦀️.rs` leaf, every
`🧪️tests/🔬️unit/` sub-tree and both fixture folders (`🧫️fixtures/{🎲️solver-contracts,🔀️topology-contracts,🔍️decoding-and-graphs}`, `💼️job/📤️publication/🧫️fixtures`) — the unit tests reach them by relative `include_str!`, so the whole tree had to move together. 91 files.

Two files are new: the crate root `🦀️.rs` (rewritten, §3) and `🧪️tests/🔬️grid-job-drive/🦀️.rs` (§6).

## 2. Manifest

```toml
[package] name = "semio-s-plugin-wfc-engine" · version/edition/rust-version.workspace = true
[package.metadata.semio] role = "s-module"
[lints] workspace = true
[lib] path = "../../🦀️.rs"
[dependencies] semio-framework-{dispatch-macros,geometry,graph,job,os-kernel,value-derive} = { workspace = true }
[dev-dependencies] serde_json = { workspace = true }
```

Derivation: `semio-framework-job` (`InteractiveJob`/`StepContext`/`RetainedJobPayload` in `job`, `search`),
`semio-framework-geometry` (`random::Rng` in `sample`, `search`, `beam`), `semio-framework-graph`
(`GraphView`/`EdgeRef`/`NodeId` in `topology::from_graph_view`), `semio-framework-dispatch-macros`
(`dyn_enum`/`dyn_enum_close` in `constraint`), `semio-framework-value-derive` (`ToValue`/`FromValue`
on `ids`, `job`, `serial`), `semio-framework-os-kernel` (the derive expands to `::semio_framework_os_kernel::{ToValue,FromValue}`, and `job` encodes previews with `semio_framework_os_kernel::json`).
No `serde`, no `pack`: the engine derives neither `Serialize` nor `Deserialize` anywhere. `serde_json`
is test-only (fixture oracles).

Root `Cargo.toml` (two single-line Edit insertions, made immediately after the crate manifest existed):
- member `"✏️s/🔌️plugins/🀄️wfc/⚙️engine/📦️packages/🦀️rust",` next to the other `✏️s/🔌️plugins/…` members
- `semio-s-plugin-wfc-engine = { path = "✏️s/🔌️plugins/🀄️wfc/⚙️engine/📦️packages/🦀️rust" }` in `[workspace.dependencies]`
Both lines were repointed to `🀄️wfc` by the coordinator during the folder rename and are correct now.

## 3. Un-gating — 34 modules lifted out of `#[cfg(test)]`

Before, 7 of 41 modules shipped. Now everything ships except two.

Un-gated at the crate root (were `#[cfg(test)]`, now plain `pub mod`):
`beam`, `chunk`, `constraint`, `constraints_card`, `constraints_conn`, `diag`, `domain`, `evolve`,
`extract`, `flow`, `grid2d`, `grid3d`, `heuristics`, `hierarchy`, `motif`, `nogood`, `outcome`,
`parallel`, `prop_ac3`, `prop_ac4`, `propagate`, `repair`, `sample`, `search`, `serial`, `soft`,
`solver_graph`, `solver_grid2d`, `solver_grid3d`, `sparse3d`, `symmetry`, `tiled` (32).
Already production and unchanged in gating: `bitset`, `error`, `ids`, `job`, `model`, `topology`, `weights`.
**Still `#[cfg(test)]` — deliberately**: `oracle` (`🔮️oracles`, the brute-force differential
reference) and `model_vectors` (`🧪️tests/🧮️model-vectors`, shared test fixtures).

Every module is now `pub` (no `pub mod(crate)` left) and every `pub(crate)` item in the crate was
widened to `pub` — 8 files also carried per-item `#[cfg(test)]` gates that were stripped, because
those items are exactly what the artifacts need:
- `🏗️model` (50 gates): `PatternInfo`, `RelationInfo`, `ModelBuilder` + its whole `impl`,
  `CompiledModel`'s `patterns/relations/supporters/base_support/tag_names/tag_ids` fields and the
  accessors `relation_count`, `pattern_info`, `relation_info`, `inverse`, `supporters`, `tag_id`,
  `tag_name`, `full_domain`, `validate`, `lint`, `stats`, `LintFinding`, `ModelStats`.
- `⚠️error` (24): the `AsymmetricInverse`/`CapacityOverflow`/`InvalidSymmetryGroup`/`SchemaVersionMismatch`
  `ModelError` variants, the `ZeroDimension`/`SizeOverflow`/`MaskShapeMismatch`/`InvalidStencil`/`TooManyNodes`
  `TopologyError` variants, all of `ConstraintError` and `SolveError` plus their `Display`/`Error` impls.
- `🎛️bitset` (13): `new_full`, `fill`, `clear_all`, `and_with`, `or_with`, `and_not_with`, `count_ones`,
  `is_all_zero`, `iter_ones`, `words`, `is_subset_of`, `intersects`, `restrict_returning_removed`.
- `⚖️weights` (12): `with_reference_columns`, `new`, `ln_w`, `w_int`, `has_integer_weights`,
  `sum_over`, `sum_int_over` and the `ln_w`/`w_int` columns.
- `🗺️topology` (6): `node_count`, `arc_count`, `in_degree`, `GraphTopologyBuilder` + impl, `from_graph_view`.
- `🆔️ids` (2 + 6 method scopes): `TileId` and `DecisionId` now exist in production, and every
  `get/index/from_index` method scope changed from `test` to `all()` (so `TileId::from_index`,
  `RelationId::from_index`, `RegionId::get` are callable from the artifacts).
- `🔄️prop-ac4` (2): `count_at`, `debug_assert_consistent`. `🚫️nogood` (1): `len`.
- `💼️job` (7): `WfcSampler::Uniform` (+ `JobRng::range` and `ChoiceCursor::ordinal` it needs, and
  both `match` arms) is now a production sampler; `WfcJob::{from_checkpoint, commit, domain_masks,
  metrics, observed}` are production `pub` read accessors.

## 4. Renames and de-aliasing

| before | after | why |
|---|---|---|
| `model::AssemblyModelBuild` / `AssemblyModelPhase` | `model::GraphModelBuild` / `GraphModelPhase` | a generic steppable weights→patterns→rules compiler, nothing assembly-specific |
| `topology::AssemblyTopologyBuild` / `AssemblyTopologyPhase` | `topology::GraphTopologyBuild` / `GraphTopologyPhase` | a generic steppable arbitrary-graph topology compiler |
| `// #region 🧵️AssemblyCompiler` | `// #region 🧵️IncrementalCompiler` | same |
| `crate::wfc_engine::…` (292 paths) | `crate::…` | it is its own crate now |
| `protocol::json::…`, `dsl::json::…` | `semio_framework_os_kernel::json::…` | those aliases were `extern crate semio_framework_os_kernel as protocol/store/dsl/vcs` lines in the **assembly crate root**, which the engine no longer has. No `extern crate` alias exists in the engine. |
| `job::tests::{payload_bytes, retire_outcome, close_job}` | `job::{payload_bytes, retire_outcome, close_job}` | `search::drive_batch_job` calls them and `search` now ships, so they had to become production helpers. The job test module still gets them through `use super::*`. |
| `job::retained_payload_bytes` (test-only) | removed, callers use `job::payload_bytes` | it was a duplicate of `payload_bytes` |

`model_vectors` used to lean on `use super::*` from inside the `wfc_engine` module; as a crate-root
module it now imports explicitly (`bitset::PatternSet`, `ids::{NodeId, RelationId}`,
`model::{CompiledModel, ModelBuilder}`, `oracle::ArcSpec`, `weights::WeightTable`).

## 5. wasm32-wasip2

`cargo check --target wasm32-wasip2` is green with **no gating needed**. The findings:
- `🧵️parallel` does **not** use `std::thread` at all, despite its name. Its own docstring says so:
  *"This keeps the solver off private CPU threads: the process-wide worker pool is the only
  production CPU-thread owner."* `multi_start` runs attempts sequentially by index and reduces
  deterministically (lowest-index `Solved` wins). Nothing to gate, nothing to fall back to — the
  sequential implementation *is* the implementation. I deliberately did not add a
  `#[cfg(not(target_arch = "wasm32"))]` that would be a no-op.
- The only non-portable API in the tree is `std::time::Instant` in `🔍️search` (5 sites: `Budget`
  deadlines in `solve`/`solve_all`/`solve_with_constraints`). `Instant` is supported on
  `wasm32-wasip2` (wasi clocks), so it compiles and works; no gate needed.
- No `std::fs`, `std::net`, `std::process`, `SystemTime`, `Mutex`-across-threads or FFI anywhere.

Artifact authors: `search::solve*` is wall-clock-budgeted and therefore blocking. In a guest, prefer
`job::WfcJob` (fuel-bounded, resumable) — that is what the inference services should drive.

## 6. Tests

`Topology` was already implemented for `GraphTopology`, `Grid2dTopology` and `Grid3dTopology`
(`🗺️topology/🦀️.rs:78`, `🔲️grid-2d/🦀️.rs:248`, `🧊️grid-3d/🦀️.rs:235`) — verified, nothing to add.
Both grid topologies already derive `Clone`, so `WfcJob<T: Topology + Clone>` accepts them directly.

New `🧪️tests/🔬️grid-job-drive/🦀️.rs` (3 tests), mirroring how the assembly inference drives
`WfcJob<GraphTopology>` through `semio_framework_job`:
1. `wfc_job_drives_a_grid_2d_topology_to_a_valid_commit` — `TiledModelBuilder` + two tiles +
   `declare_stencil_relations_tiled(Stencil2d::VonNeumann)` + `Grid2dTopology::new(6, 6, …, Boundary::Open)`,
   stepped with `StepBudget::new(8, …)` to `StepOutcome::Complete`, payloads retired, close ladder run,
   then every cell asserted against its `(x+y) % 2` checkerboard parity.
2. `wfc_job_drives_a_grid_3d_topology_to_a_valid_commit` — same over `Stencil3d::Face6` and a 4×4×4
   `Grid3dTopology`, parity `(x+y+z) % 2`.
3. `extract_2d_output_is_locally_similar_to_its_sample` — `extract_2d` over a 4×4 two-colour sample
   (2×2 colour blocks), window 2, periodic input, `SymmetryGroup2d::None`; the extracted model is
   solved on an 8×8 `Grid2dTopology` with `Boundary::Wrap` on both axes, decoded with
   `PatternDecoder2d::decode`, and **every** wrapped 2×2 window of the output is asserted to occur in
   the input's window set (plus both colours present). `extract_2d` declares its relations on a fresh
   `ModelBuilder` in `Stencil2d::VonNeumann.offsets()` order, so the test reconstructs the same
   `Vec<RelationId>` the same way for the output topology — do the same in the bitmap artifact.

**Counts**: 298 `#[test]` came across from procedural; 301 now (the 3 above). Result:
`300 passed; 1 failed` or `301 passed; 0 failed` depending on machine load — see §8.

Two stale assertions in the copied `💼️job` unit tests were repaired (they could not have passed since
`empty_job_fault()` landed on 2026-09-09, commit `599a5d8450` — the assembly crate is no longer a
workspace member so I could not run the original to confirm, but the code makes it certain):
`CommitBuild::new`/`CheckpointBuild::new` are called before any `StepContext` exists, so they cannot
page a detail message and return an empty placeholder `JobFault`. The test asserted the detail bytes
`wfc-commit-admission-exceeded` / `wfc-checkpoint-admission-exceeded`; it now asserts the admission
refusal itself (`is_err()`), which is the property that matters. Detailed faults raised from inside
`step` (e.g. `wfc-checkpoint-capacity`) still assert on their bytes and still pass.

## 7. Public API for the artifact authors

Import path is `semio_s_plugin_wfc_engine::<module>`.

**`ids`** — `PatternId(u32)` a solver *value*; `TileId(u32)` an authored tile (one tile ↔ one pattern
until symmetry expands orbits); `NodeId(u32)` a solver *variable* (grid cell or graph slot);
`RelationId(u32)` a directed adjacency relation; `DecisionId`, `RegionId`. All have
`get()/index()/from_index()`.

**`model`** — `ModelBuilder` authors the pattern universe (`add_pattern(weight) -> PatternId`,
`add_relation(name) -> RelationId`, `set_relation_inverse`, `allow`, `deny`, `allow_mirrored`,
`add_tag`, `set_tile`, `compile()`). `CompiledModel` is the immutable result: `pattern_count`,
`relation_count`, `allowed(relation, source) -> &PatternSet`, `supporters`, `inverse`, `weights()`,
`full_domain()`, `fingerprint()`, `validate()`, `lint() -> Vec<LintFinding>`, `stats() -> ModelStats`.
`PatternInfo { weight, tags }`, `RelationInfo { name, inverse }`.
`GraphModelBuild` / `GraphModelPhase` — the *incremental* compiler: `new(weights, pairs)` then
`step() -> Result<Option<CompiledModel>, ModelError>` once per fuel unit. Use it when compiling a
model inside a job step instead of blocking on `ModelBuilder::compile`.

**`tiled`** — `TiledModelBuilder`: the `TileId`-facing wrapper (`tile(weight) -> TileId`, `tag`,
`relation`, `set_relation_inverse`, `allow`, `deny`, `allow_mirrored`, `allow_where(relation, tiles, pred)`,
`pattern_of`, `tile_count`, `compile()`). This is what grid2d/grid3d/wfc2d/wfc3d should author with.

**`topology`** — `trait Topology` (`node_count`, `arc_count`, `region_of`, `out_arc_bound`,
`out_arc_at`, `for_each_out_arc`, `for_each_in_arc`, `max_in_degree`). `GraphTopology` is the
arbitrary directed-graph implementation (multiedges and self-loops allowed); `GraphTopologyBuilder::new(n)`
+ `.arc(from, to, relation)` + `.build()`. `from_graph_view(view, rel_of)` converts any
`semio_framework_graph::GraphView`. `GraphTopologyBuild` / `GraphTopologyPhase` — the incremental
(`add_arc` … `step()`) sibling, for building a topology inside a job step; the wfc2d/wfc3d graph
artifacts want this one.

**`grid2d`** — `Stencil2d::{VonNeumann, Moore, Hex, Custom}` (`offsets()` order is the relation
order); `declare_stencil_relations(&mut ModelBuilder, &Stencil2d)` and
`declare_stencil_relations_tiled(&mut TiledModelBuilder, …)` register one relation per offset with
inverses and return them *in offsets order*; `Boundary::{Open, FixedOutside(PatternId), Wrap, Mirror}`
(`Wrap` = periodic; `Mirror` under-counts AC-4 support on small grids — avoid);
`Grid2dTopology::new(width, height, &stencil, relations, boundary_x, boundary_y, mask)` with
`mask: Option<Vec<bool>>` for masked-out cells; `node_at(x, y)`, `coords(n)`, `is_active`,
`inactive_cells()`, `fixed_outside_restrictions()`.

**`grid3d`** — `Stencil3d::{Face6, Edge18, Vertex26, Custom}` (**there is no `VonNeumann` variant**;
the six-neighbour stencil is `Face6`), `declare_stencil_relations_3d[_tiled]`,
`Grid3dTopology::new(width, height, depth, &stencil, relations, bx, by, bz, mask)` with
`node_at(x, y, z)`, `coords`, `depth()`, `is_active`, `inactive_cells`, `fixed_outside_restrictions`.
`Boundary` is re-used from `grid2d`.

**`extract`** — `Sample2d::new(width, height, Vec<TileId>)`; `Extract2dConfig { window, periodic_input, symmetry }`
(default 2 / true / `None`); `extract_2d(&[Sample2d], &cfg) -> ExtractedModel2d { model, decoder }`;
`PatternDecoder2d::{window, anchor_tile, window_of, decode(&[PatternId]) -> Vec<TileId>}`. The
anchor convention: the pattern at cell `(x, y)` *is* the N×N window whose top-left is `(x, y)`, so
`decode` yields a locally-similar bitmap. Relations are always `Stencil2d::VonNeumann` in
`offsets()` order. **2D only — no `extract_3d` exists.**

**`symmetry`** — `Transform2d` + `SymmetryGroup2d` (D4 subgroups, `elements()`, `apply_window`);
`Transform3d`, `cube_rotations_24()`, `cube_symmetries_48()`, `SymmetryGroup3d`.

**`job`** (the production solver, this is what an inference service drives) —
`WfcJob::<T: Topology + Clone>::new(operation, model, topology, WfcJobConfig, initial_domains, fixed)`
implements `semio_framework_job::InteractiveJob` (the impl additionally requires `T: Send`).
`WfcJobConfig { sampler: WfcSampler::{WeightedRoulette (default), Uniform} }`.
`WfcStage` is the 10-step state machine; `WfcPreview` is the per-step UI preview (active slot,
candidates, tested tile, propagation wave, changed domains, contradiction, backtrack path,
`incomplete_grid: Vec<Option<u32>>`, counters) and rides `StepOutcome::PreviewReady` as JSON;
`WfcCommit { assignment: Vec<u32>, observations, compatibility_edges, backtracks }`.
Read accessors: `commit()`, `domain_masks() -> Vec<PatternSet>` (entropy maps), `metrics()`,
`observed()`. Resume: `WfcRestore::new(…bytes)` stepped to completion then `take_job()`, or the
one-shot `WfcJob::from_checkpoint(…)`. `MAX_CHECKPOINT_BYTES = 1 MiB`.
Drive helpers, now production: `payload_bytes`, `retire_outcome`, `close_job` — **every job must be
run through `close_job` (or an equivalent close ladder) before it is dropped**.

**`search`** (blocking reference driver; prefer `job` in a guest) — `SearchConfig`, `SearchMode`,
`RestartSchedule`, `Budget`, `CancelToken`, `solve`, `solve_cancellable`, `solve_all`,
`solve_with_constraints`, `solve_all_with_constraints`.

**`solver_grid2d` / `solver_grid3d` / `solver_graph`** — builder facades over `search` that fold
masks and `Boundary::FixedOutside` into domain overrides: `Grid2dSolverBuilder::new(model, topology)`
`.fix(x, y, p)` `.domain(x, y, set)` `.config(cfg)` `.constraint(c)` `.build()`, then
`Grid2dSolver::{solve, solve_cancellable, solve_all, solve_chunk, get, decode_tiles, model, topology}`.
`GraphSolver` additionally has `resume(&Checkpoint)`, `repair`, `solve_beam`, `solve_multi_start`.

**`outcome`** — `SolveOutcome::{Solved(Solution), Unsatisfiable(UnsatReport), Contradiction(ContradictionReport), BudgetExceeded(PartialState), Cancelled(PartialState)}`; a failed solve is an
outcome, never an `Err`. **`error`** — `ModelError`, `TopologyError`, `ConstraintError`, `SolveError`,
flat and non-nested. **`weights`** — `WeightTable` (`w`, `ln_w`, `w_int`, `sum_over`, `len`, `is_empty`).
**`bitset`** — `PatternSet`, the word-packed domain bitset (`iter_ones`, `count_ones`, `and_with`,
`restrict_returning_removed`, …). **`constraint*`, `flow`, `nogood`, `beam`, `repair`, `hierarchy`,
`chunk`, `evolve`, `soft`, `motif`, `sparse3d`, `diag`, `serial`, `heuristics`, `sample`, `domain`,
`trail`, `propagate`, `prop_ac3`, `prop_ac4`, `parallel`** are all available now too — see each
module's `//!` doc for scope and the deliberate gaps it declares.

## 8. Commands run (all foreground, `-j 4`, logs in `🗑️generated/engine/`)

| command | result | log |
|---|---|---|
| `cargo check -p semio-s-plugin-wfc-engine --lib --tests -j 4` | **GREEN**, 0 errors, 0 warnings from this crate | `check-final.txt` |
| `RUST_MIN_STACK=33554432 cargo test -p semio-s-plugin-wfc-engine --lib -j 4 -- --test-threads=4` | **301 passed / 0 failed** on 3 of 6 runs; **300 / 1** on the other 3, always the same timing test (below) | `test-run1..3.txt`, `test-final*.txt` |
| `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-wfc-engine --target wasm32-wasip2 -j 4` | **GREEN**, 0 errors, 0 warnings | `check-wasm-final.txt` |
| `cargo clippy -p semio-s-plugin-wfc-engine --lib -j 4` | **GREEN**, 0 warnings (6 were introduced by widening `pub(crate)` → `pub` and were fixed: `is_empty()` added to `DomainStore`, `NogoodIndex`, `Trail`, `WeightTable`; two `obfuscated_if_else` chains in `grid2d`/`grid3d` `out_arc_bound`) | `clippy-2.txt` |

### The one flaky test — `job::tests::every_large_domain_unit_including_checkpoint_stays_below_watchdog`

Pre-existing, load-dependent, **not** introduced by this extraction. The test drives an 8,192-node
checkerboard through ~324k one-fuel steps and asserts `p99 < 2 ms` **and** `max < 8 ms`. The p99
assertion always passes; the single worst sample is what flips. Measured maxima across runs:
7.2 ms (pass), 16.5 ms, 18.4 ms, 21.2 ms, 23.2 ms (fail). Instrumenting the loop shows the outlier is
always a `PropagateCompatibilityEdge` unit deep in the run (index ≈ 324k), never a checkpoint unit —
i.e. an unlucky scheduling/allocator hiccup in a debug build on a machine that is running four other
cargo waves, not an algorithmic cliff. I added the measured value to the assertion message
(`"WFC unit maximum exceeded 8 ms: {maximum:?}"`) so the next failure is self-diagnosing, and left
the budget alone — weakening a watchdog to make a suite green is the wrong trade. W2 should re-run it
on an idle machine before judging it.

## 9. Not done / left to others

- `✏️s/🔌️plugins/🌀️procedural/…/🧩️wfc-engine/` is **untouched and still on disk** — slice C deletes it.
- No `README.md` in the engine folder (plan says docstrings only).
- No `📋️project.json` / nx target for the engine crate: it is an `s-module` library with no describe
  step; it is built transitively by `@semio-tech/wfc-plugin`.
