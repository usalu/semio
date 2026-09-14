# ⏯️ Wave W3-F: shared force-layout run

Lane W3-F of `📋️tool-run-contract.md` (§3.7, §5 "Framework lane W3-F") and `📓️audit-p4-tool-inventory.md` §8 wave 0.2.
Status: **landed, green.** Native and wasm32-wasip2 checks reach warnings; 13/13 Rust tests, 7/7 TS conformance tests.

`L` = `🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/`. Logs: `T/🗑️generated/W3-F/`.

## 1. Placement

- `🕸️graph` is the right parent: it already owns graph drawing layouts, including the synchronous
  `🖊️drawing::force::run_force_layout` that trinity/jack calls directly and that `♾️infinite/🎲️board/➕️normal/↔️undirected`
  wraps for reasoning/wires.
- The run is a **separate crate** (`semio-framework-graph-layout-run`) in `L`, not a module inside
  `semio-framework-graph`. The run needs `semio-framework-job`, `semio-framework-tool-run` and `semio-framework-ui`
  (`LocalizedLabel`). `semio-framework-graph` is a dependency of `os-infinite`, `os-flow`, `math` and the DAG artifact,
  and none of those should inherit that stack. A sibling crate leaves the dependencies of `semio-framework-graph` unchanged.

## 2. What changed

| File | Content |
|---|---|
| `L/🧬️schema/🔣️.json` (new) | Source of record. `$defs`: `LayoutRunPoint/Node/Edge/Graph/Config/Falloff/SpringLaw/Stage/Counter/Reason/Stop`, plus the fixture types (`LayoutRunGenerator`, `LayoutRunExpectation`, `LayoutRunFixtureCase`, `LayoutRunFixture`). `x-semio-toolRun` holds the stages, the counters (with fixed point), the reasons with verdicts and EN/DE templates, the unit, the policies, the 44-byte checkpoint layout, the limits and the graph/positions digest rules |
| `L/🧫️fixtures/🎞️layout-run.json` (new) | `defaultConfig`, 4 seeded cases (grid 6×6, ring 24 with checkpoints every 16 iterations, tree 31 with a pinned root and gravity, a hand-made anchored chain with pins, anchors, unplaced nodes and weights) with exact expectations (stop, iterations, positions digest, moved ops, settled, checkpoints, first 24 iterate-stage verdicts), the step law and the oracle parameters |
| `L/🦀️.rs` (new, 1 428 lines) | Regions: Limits, Vocabulary (+ `layout_run_definition`), Graph, Config, Encoder, Checkpoint, Quadtree (arena Barnes-Hut), Job |
| `L/🧪️tests/🔬️unit/🦀️.rs` (new) | 12 laws, see §4 |
| `L/🧪️tests/🔬️oracle/🦀️.rs` (new) | The `fdg-sim` oracle law |
| `L/🧪️tests/🧩️conformance/🟦️.ts` (new) | bun + ajv: the fixture and hostile mutations against the schema, plus the consistency of the tool-run table |
| `L/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs,📜️script.ts,📋️project.json,package.json}` (new) | Crate `semio-framework-graph-layout-run`; nx project `@semio-tech/framework-graph-layout-run-rs` with targets `test`, `test-quick`, `test-long`, `test-exhaustive`, `check` |
| `Cargo.toml` (root) | Workspace member after `⏯️tool-run`; `semio-framework-graph-layout-run = { path = … }` after `semio-framework-graph` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | `members-of-members-of-modules.memberNames` gains `⏯️layout-run` |

Dependencies:
- Runtime: `semio-framework-os-kernel` (value derives), `semio-framework-job`, `semio-framework-tool-run`, `semio-framework-ui` (feature `wgpu`, declarative only) and `serde`. No third-party runtime dependency.
- Dev: `serde_json`, and **`fdg-sim = "0.9.1"`** (MIT) as the oracle.

## 3. Public API as landed (the contract for the consumer lanes)

```rust
// limits
pub const LAYOUT_RUN_TICK_FLUSH_BYTES: usize = 12 * 1024;   // estimated bytes before a tick is flushed (one 16 KiB payload page)
pub const LAYOUT_RUN_OP_BYTES_MAX: usize = 2 * 1024;        // largest encoded move op
pub const LAYOUT_RUN_NODES_MAX: usize = 32_768;             // TOOL_RUN_PROVISIONAL_OPS_MAX / 2
pub const LAYOUT_RUN_REPULSION_CHUNK: usize = 4;            // repulsion queries between deadline checks
pub const LAYOUT_RUN_LINEAR_CHUNK: usize = 128;             // tree inserts / springs / integrations between deadline checks
pub const LAYOUT_RUN_TREE_DEPTH_MAX: u32 = 32;
pub const LAYOUT_RUN_CHECKPOINT_MAGIC: u32 = 0x4C52_4331;

// vocabulary (indices/codes are the ToolRunDefinition indices)
pub enum LayoutRunStage { Initialize, Iterate, Settle }                         // ALL, index() -> u16, id(), label() -> LocalizedLabel
pub enum LayoutRunCounter { Iterations, Energy, MaxDisplacement, Settled }     // ALL, index(), id(), fixed_point() (1 | 1000), label()
pub enum LayoutRunReason { Moving /*0 testing*/, Settled /*1 success*/, Pinned /*2 success*/, Diverged /*3 danger*/, Unsettled /*4 warning*/,
    Initialized /*5 step*/, Converged /*6 success step*/, IterationLimit /*7 warning step*/, Resumed /*8 step*/ } // ALL, code(), from_code, id(), verdict(), template()
pub enum LayoutRunStop { Converged, IterationLimit }                            // serde/value camelCase
pub fn layout_run_definition(run_job: JobKindId) -> ToolRunDefinition;           // mutating, rebase Restart, reconfigure Resume, trace Entity, unit iteration/Iteration

// graph (serde + ToValue/FromValue, camelCase, deny_unknown_fields)
pub struct LayoutRunPoint { pub x: f64, pub y: f64 }                            // new(x, y)
pub struct LayoutRunNode { pub entity: u64, pub origin: Option<LayoutRunPoint>, pub radius: f64, pub pinned: bool, pub anchor: Option<LayoutRunPoint> }
pub struct LayoutRunEdge { pub source: u32, pub target: u32, pub weight: f64 }
pub struct LayoutRunGraph { pub nodes: Vec<LayoutRunNode>, pub edges: Vec<LayoutRunEdge> }
impl LayoutRunGraph { pub fn validate(&self) -> Result<(), LayoutRunGraphError>; pub fn digest(&self) -> u64 }
pub enum LayoutRunGraphError { TooManyNodes, NonFiniteOrigin { node: u32 }, NonFiniteAnchor { node: u32 }, NonPositiveRadius { node: u32 }, PinnedWithoutOrigin { node: u32 }, EdgeOutOfRange { edge: u32 }, InvalidWeight { edge: u32 } }
pub fn layout_run_positions_digest(positions: &[[f64; 2]]) -> u64;

// config (serde + ToValue/FromValue, camelCase; Default = the fixture's defaultConfig)
pub enum LayoutRunFalloff { InverseSquare, Inverse }        // repulsion 1/d² | 1/d
pub enum LayoutRunSpringLaw { Linear, Quadratic }           // d − ideal | d²/ideal (anchors: d | d²/ideal)
pub struct LayoutRunConfig {
    pub max_iterations: u32,          // 420
    pub ideal_edge_length: f64,       // 140
    pub repulsion_strength: f64,      // 49
    pub repulsion_falloff: LayoutRunFalloff,   // Inverse
    pub spring_strength: f64,         // 1
    pub spring_law: LayoutRunSpringLaw,        // Quadratic
    pub gravity: f64,                 // 0
    pub center: Option<LayoutRunPoint>,        // None = centroid of the initial positions
    pub anchor_strength: f64,         // 1
    pub time_step: f64,               // 0.85
    pub velocity_damping: f64,        // 0.88
    pub max_speed: f64,               // 48
    pub cooling_floor: f64,           // 0.08   cool = max(1 − iteration/maxIterations, floor), dt = timeStep·√cool
    pub seed: u64,                    // 1
    pub barnes_hut_theta: f64,        // 0.78   0 = exact
    pub pairwise_max_bodies: u32,     // 56     exact pairwise at or below
    pub settle_displacement: f64,     // 0.5    convergence threshold
    pub settle_iterations: u32,       // 8      consecutive iterations below it
    pub emit_displacement: f64,       // 0.25   drift before a node's move op is re-emitted
    pub checkpoint_iterations: u32,   // 64
    pub compact_ops: u32,             // 16 384 effective threshold max(compactOps, 2·nodes) ≤ 65 536
}
impl LayoutRunConfig { pub fn validate(&self) -> Result<(), LayoutRunConfigError> }
pub struct LayoutRunConfigError { pub field: &'static str }   // schema field name

// encoder (the caller's artifact op)
pub struct LayoutRunEncodeError(pub String);
pub trait LayoutRunOpEncoder: Send { fn encode_move(&mut self, node: u32, entity: u64, position: LayoutRunPoint) -> Result<Vec<u8>, LayoutRunEncodeError>; }
impl<F: FnMut(u32, u64, LayoutRunPoint) -> Result<Vec<u8>, LayoutRunEncodeError> + Send> LayoutRunOpEncoder for F

// checkpoint
pub struct LayoutRunCheckpoint { pub node_count: u32, pub graph_digest: u64, pub iteration: u32, pub settle_streak: u32, pub provisional_len: u32, pub center: [f64; 2] }
impl LayoutRunCheckpoint { pub const BYTES: usize = 44; pub fn encode(self) -> [u8; 44]; pub fn decode(&[u8]) -> Option<Self> }
pub enum LayoutRunStartError { Graph(LayoutRunGraphError), Config(LayoutRunConfigError) }
pub enum LayoutRunResumeError { Malformed, Foreign, Positions, Graph(LayoutRunGraphError), Config(LayoutRunConfigError) }

// job
pub struct LayoutRunJob<E: LayoutRunOpEncoder>;   // impl InteractiveJob; Box<dyn InteractiveJob + Send> ready (asserted by a test)
impl<E: LayoutRunOpEncoder> LayoutRunJob<E> {
    pub fn new(identity: ToolRunIdentity, graph: &LayoutRunGraph, config: LayoutRunConfig, encoder: E) -> Result<Self, LayoutRunStartError>;
    pub fn resume(identity: ToolRunIdentity, graph: &LayoutRunGraph, config: LayoutRunConfig, encoder: E, positions: &[LayoutRunPoint], checkpoint: &[u8], provisional_len: u32) -> Result<Self, LayoutRunResumeError>;
    pub fn identity(&self) -> ToolRunIdentity;
    pub fn stage(&self) -> LayoutRunStage;
    pub fn iteration(&self) -> u32;
    pub fn stop(&self) -> Option<LayoutRunStop>;
    pub fn checkpoints(&self) -> u32;
    pub fn positions(&self) -> Vec<LayoutRunPoint>;
    pub fn positions_digest(&self) -> u64;
    pub fn counters(&self) -> [u64; 4];             // LayoutRunCounter::ALL order
    pub fn checkpoint(&self) -> LayoutRunCheckpoint;
}
```

### Step semantics

- **Initialize** (stage 0).
  - First page: one `info`/`initialized` step `[nodes, edges]`, or `resumed` `[iteration]` on resume.
  - Then `Upsert{key: node index, testing/moving | success/pinned, Entity{entity}}` for every node, spread over as many
    ticks as needed.
  - No fuel is consumed. A resumed job emits only its step.
- **Iterate** (stage 1).
  - One iteration is Tree (arena quadtree) → Repulse (Barnes-Hut or exact pairwise) → Attract (springs) → Integrate
    (gravity, anchors, damping, speed clamp, pins).
  - Every phase is resumable by cursor and checks the deadline after each chunk. An expired deadline returns `Yield`.
    The result is independent of the slicing: forces are Jacobi, and positions are only written in Integrate.
  - A completed iteration consumes **1 fuel** and ends the step with exactly **one tick**. The tick carries:
    - progress (`completed = iteration`, `total = maxIterations`, 4 counters);
    - a `danger`/`diverged` step when nodes produced non-finite values (they are reset to their last finite position);
    - a round-robin, byte-bounded batch of nodes whose drift since their last emitted position is ≥ `emitDisplacement`.
      Each such node gets its move op, its entity (once per epoch) and its trace upsert (`testing/moving` or
      `success/settled`).
    - trace upserts for nodes whose verdict changed.
- **Compaction** (checkpoint every `checkpointIterations`, or when provisional ops reach the threshold).
  - The first page carries `retractTo(0)` and velocities are quenched.
  - The pages then re-append exactly one op plus entity per moved node (a node moved when it is unplaced or its position
    differs from its origin).
  - The next `step` returns `CheckpointReady` (44 bytes, `applied_progress = iteration`); that call does not use up a
    single step.
- **Settle** (stage 2).
  - Convergence means the largest displacement stayed below `settleDisplacement` for `settleIterations` consecutive
    iterations with no divergence; otherwise the run stops at `maxIterations`.
  - The same compaction runs, plus a final upsert per node: `success/settled`, `success/pinned`, `danger/diverged` or
    `warning/unsettled`.
  - The last tick has a `success`/`converged` or `warning`/`iterationLimit` step `[iterations, maxDisplacement, energy]`
    and progress `state: Complete`.
  - The next call returns `Complete`. The provisional list then holds **exactly one op per moved node**, so finalize
    publishes one `Edit` with only the moved nodes.
- **Cancellation** is checked on entry and between chunks. **Close** releases one owned buffer per `close_step`.
  **Faults**: an encoder error, an op above 2 KiB or a tick above one page gives `Fault` with a UTF-8 message payload.

### How a consumer lane wires it (reasoning, dag, trinity/jack, flow, sequence)

1. **Definition.**
   - `run: Some(layout_run_definition(JobKindId::new("<plugin>.layoutRun")))` on the `reorganize`/`forceLayout` tool or utility.
   - Delete the synchronous command handler, per §3.6.
   - EN/DE labels come from the definition; the plugin adds none.
2. **Graph.**
   - One `LayoutRunNode` per visible node: `entity` = first 8 LE bytes of `semio_framework_hash::hash(node_id)` (the W1-A
     convention, so `provisional: true` stamping matches), `origin` = document x/y (`None` when unplaced), `radius`,
     `pinned` = locked ids, `anchor`.
   - Edges resolve handles to node indices (reuse the plugin's existing resolver) and deduplicate as today.
3. **`build_tool_run_job(request)`.**
   - `Run` without a checkpoint: `LayoutRunJob::new(request.identity, &graph_from(request.snapshot), config, encoder)`.
   - `Run` with `request.checkpoint = Some(bytes)`: fold `request.provisional` over `request.snapshot` to get the current
     positions, then call `LayoutRunJob::resume(identity, &graph_from(base), config, encoder, &positions, bytes,
     request.provisional.len() as u32)`.
   - On `Err(Foreign | Malformed | Positions)`, build a fresh `new` instead.
   - No revalidate job: `rebase: restart` discards and restarts on any head change.
4. **Encoder.** A closure `move |node, _entity, p| OpBinary::encode_op(&<Plugin>Mutation::move_node(ids[node].clone(), p.x, p.y))`.
   - reasoning: `crate::mutations::move_node`
   - trinity/jack: `schema::mutations::move_node`
   - dag/sequence/flow: their `move-node` op
5. **Config mapping.**
   - The defaults are the Fruchterman-Reingold laws (§5.1).
   - trinity/jack's old behaviour was `ForceLayoutOptions { iterations: 120, ..}`: set `max_iterations: 120` and use radii `(w.max(48) + h.max(24)) · 0.25`.
   - reasoning's board used `apply_force_graph_layout_*` with gravity 0: keep the defaults and pass `locked_node_ids` as `pinned`.
   - Only select `repulsion_falloff: InverseSquare, spring_law: Linear, repulsion_strength: 6500, spring_strength: 0.028` if a lane has to preserve the old look. The oracle shows that look lands in local minima (§5.1).
6. **Layered consumers (dag, flow, sequence).**
   - Their `reorganize` is a layered DAG layout (`DagHost::reorganize`), not a force layout.
   - Compute the layered targets once (cheap, O(V+E)) and pass them as `anchor`s.
   - Use `spring_strength: 0` or small, `anchor_strength` ≥ 1 and `repulsion_strength` small enough to only resolve
     overlaps. The run then animates and relaxes toward the layered layout with the same trace and finalize path.
   - If a lane needs exact layered positions, set `pinned` targets instead: origin = target, which emits no op. That
     lane then relies on the anchors only for the unpinned nodes.
7. **Finalize.** Supply `ArtifactStoreOneItemPreparationFactory` for the move op (W0-D). Stamp `provisional: true` from
   `ArtifactView::tool_run()?.provisional_entities` (W0-H §1.5).

## 4. Tests (all foreground)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-graph-layout-run -- --test-threads=4` | **13 passed, 0 failed** (`test-11.txt`) |
| `cargo test -p semio-framework-graph-layout-run -- --test-threads=1` | **13 passed, 0 failed** (`test-final-threads1.txt`) |
| `bun test ./🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/🧪️tests/🧩️conformance/🟦️.ts` | **7 pass, 0 fail, 132 expects** (`ts-conformance-final.txt`) |
| `bun nx run @semio-tech/framework-graph-layout-run-rs:test --skip-nx-cache` | TS 7 pass, then nextest **13/13 passed** (`nx-test-2.txt`, rerun after the last edit and after the rate-limit resume re-check) |
| `bun nx run @semio-tech/framework-graph-layout-run-rs:check --skip-nx-cache` | native and wasm32-wasip2 Finished (`nx-check-1.txt`) |
| `cargo check -p semio-framework-graph-layout-run` / `--target wasm32-wasip2` with a temporary `[DEBUG]` probe fn | both Finished, and both reached the probe's dead-code **warning in this crate** (type-check proof, rule 8); the probe was removed and the file verified byte-identical with `cmp` (`check-final-native.txt`, `check-final-wasip2.txt`) |
| `cargo clippy -p semio-framework-graph-layout-run --tests` | 0 warnings in the crate (`clippy-final.txt`) |

Laws:
1. `…converges_deterministically_to_the_language_neutral_fixture`. For all 4 fixture cases:
   - the exact expectation matches, and a second run gives the same bits and the same ops;
   - every tick is ≤ 16 KiB; ops are exactly one per moved node, and the overlay equals the final positions bitwise;
   - the entity set equals the moved entities; there is one resident trace record per node with its final verdict;
   - the last progress is `Settle`/`Complete`, and the final step matches the stop;
   - there is one iteration tick per iteration, and every op in an iteration tick carries its node's upsert;
   - retracts = checkpoints + 1.
2. `…step_with_one_unit_of_fuel_advances_exactly_one_iteration_per_iterate_tick`: fuel 1 gives the same digest, and every iteration reports its own tick.
3. `…is_independent_of_deadline_slicing`: an always-expired deadline yields between chunks and gives the same digest.
4. `…resumes_from_its_checkpoint…`:
   - resume at the first checkpoint gives the same iterations, digest and final ops as the uninterrupted run, with a `resumed` step;
   - `Foreign` (other graph, or a shorter provisional list), `Malformed` (43 bytes, bad magic) and `Positions` are refused;
   - changed settings resume to a different valid layout.
5. `…barnes_hut_repulsion_tracks_exact_pairwise_repulsion` (2 000 random bodies):
   - with a closed angle, the tree equals exact to 1e-9;
   - at θ 0.78, `Inverse` has median 0.49 %, p95 1.7 %, aggregate 0.5 %, and `InverseSquare` has aggregate 0.7 %;
   - the corner case "a body inside an approximated cell" at θ 2 stays < 5 % error.
6. `…step_stays_below_the_interactive_target_on_a_5000_node_grid`:
   - a 100×50 grid, default config, `drive_step` with `INTERACTIVE_LANE_WALL_US`, measured through init, 3 iterations and a full 5 000-node compaction;
   - attempt 1 records the clock and attempts 2–5 replay it, so the steps are identical; the law takes the best time per step;
   - result: **worst 1 019–1 263 µs over 109–112 steps < 2 000 µs** at opt-level 0 under load average 17–20;
   - all 5 000 ops are published once, and the digest is identical across attempts.
7. `…emits_the_settled_trace_and_pinned_nodes_never_move`: the pinned node is never moved and gets no op; `iterationLimit` gives `warning` records; the anchor pulls node 2 toward x = 300.
8. `…rejects_invalid_graphs_and_configs`: 6 graph errors and 2 config errors; an empty graph settles immediately.
9. `…encoder_faults_and_cancellation_end_the_run`.
10. `…close_releases_every_owned_buffer_in_bounded_steps`.
11. `…definition_matches_the_schema_tool_run_table`: stages, counters, reasons, unit, policies, checkpoint, limits and default config against the schema; value and serde round trips; the job boxes as `dyn InteractiveJob + Send`.
12. `…resets_diverged_nodes_and_never_calls_them_settled`.
13. Oracle `…stress_matches_the_fdg_sim_fruchterman_reingold_oracle`:
   - fdg-sim FR (scale 45, cool-off 0.975, dt 0.035, 1 500 updates) runs from the same seeded positions;
   - the law requires the stress to stay ≤ 1.05× the oracle's and to improve the initial stress by ≥ 30 %;
   - measured normalized stress:

| Graph | Initial | Layout run | fdg-sim |
|---|---|---|---|
| grid 6×6 | 0.4679 | 0.0231 (277 iterations) | 0.0231 |
| ring 30 | 0.6397 | 0.0158 (267 iterations) | 0.0158 |
| tree 31 | 0.5126 | 0.0663 (284 iterations) | 0.0663 |

**Mutation checks** (each restored and verified byte-identical with `cmp`):

| Mutation | Tests that failed |
|---|---|
| Repulsion chunk 1 000 000 (no slicing) | the step law |
| Barnes-Hut inside-guard removed | the corner-case assertion |
| Compaction without `retractTo(0)` | fixture, resume and step law |
| No velocity quench | fixture and resume |

The initial inside-guard mutation passed. That led to the corner-case law, which catches it (`mutation-*.txt`).

## 5. Deviations and decisions

1. **Force model defaults are Fruchterman-Reingold** (`repulsionFalloff: inverse`, `springLaw: quadratic`, strengths 49 / 1).
   - With the inherited `drawing::force` physics (1/d², Hooke, 6500 / 0.028), 8 seeds per graph gave median stress:
     ring 0.157, grid 0.030, tree 0.124. That is 2–10× the oracle, with tangled local minima.
   - The FR laws matched the oracle on every seed (`probe-2.txt`…`probe-4.txt`).
   - Both laws stay selectable (maximum control); consumers wanting the old look pick them explicitly.
2. **Checkpoints are compaction points and quench velocities.** A full-state checkpoint does not fit one 16 KiB page for
   large graphs.
   - The provisional overlay carries the exact positions, so the checkpoint is 44 bytes (iteration, streak, provisional
     length, center, graph digest).
   - Quenching momentum every 64 iterations is negligible, since damping 0.88⁶⁴ ≈ 3·10⁻⁴.
   - Resume from the checkpoint is proven bit-identical to the uninterrupted run.
3. **Fuel is consumed per iteration only.** Initialize and compaction pages are ticks without fuel. A paused single step
   on such a page advances one page. Iteration ticks are exactly one per step and one per iteration.
4. **`rebase: restart`, no revalidate job.** A head change can add or remove nodes, which invalidates the graph and the
   index-keyed trace.
5. **Anchors** (spring to a target) are an addition for the three layered consumers, whose current `reorganize` is not
   a force layout (§3, step 6).
6. **No TS mirror.** Jobs are Rust-only, and renderers consume ticks through the existing `⏯️tool-run` TS codecs. No
   TS consumer needs the layout types. The language-neutral part is the JSON schema, the fixture and the bun/ajv
   conformance test.
7. **Node cap 32 768.** One op per node must fit in half of `TOOL_RUN_PROVISIONAL_OPS_MAX`, so compaction always leaves
   emission headroom. Graphs above the cap are refused with `TooManyNodes`.

## 6. Foreign edits

- `🔒️dependencies.json`: one `fdg-sim` entry (`test-runner`, not production-reachable), sorted.
  - `dependencyFreezeCheck` reported it as new, alongside 21 other unregistered peer dependencies I did not touch
    (`dependency-freeze-before.txt`).
- `Cargo.lock`: cargo resolution added `fdg-sim`, `glam 0.21`, `hashlink`, `quad-rand`, `ahash`, `hashbrown` and the new crate.

## 7. Commands to register in launch.json

- `bun nx run @semio-tech/framework-graph-layout-run-rs:test`: ajv conformance plus Rust laws and the oracle
- `bun nx run @semio-tech/framework-graph-layout-run-rs:check`: native and `wasm32-wasip2`

## 8. Open items

1. **wasm32-unknown-unknown.**
   - `cargo check --target wasm32-unknown-unknown` stops in `semio-framework-ui` (`🎟️prepared/🦀️.rs:3232`, `web_sys`
     unresolved when the crate is built standalone with feature `wgpu`).
   - `semio-framework-tool-run` fails identically, so this is not this lane (`check-tool-run-wasm32-unknown-unknown-baseline.txt`).
   - Consumer plugin crates pull `ui` with their web features and should compile.
2. **Compaction flicker for very large graphs (ledger, W0-D).**
   - A compaction that spans more than one driver turn can briefly render nodes that were not yet re-appended at their
     committed position.
   - Cause: the retract refold completes before the tail pages arrive.
   - Scope: one page holds about 220 nodes with 20-byte ops, and a 4 ms turn holds several pages, so typical plugin
     graphs are unaffected.
   - Proposed fix: the ledger defers the retract refold until the next tick without `retractTo` from the same job.
3. **Wave 4 deletion** once the consumers are re-pointed:
   - `🕸️graph/🖊️drawing::force::run_force_layout` and `seed_positions`;
   - `♾️infinite/🎲️board/➕️normal/↔️undirected` `apply_force_graph_layout_*`, and its `➡️directed` wrapper.
4. **More synchronous force-layout callers than the five named.**
   - `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/🎮️commands/⚛️force-layout/🦀️.rs` (puzzle 2d, W2-A's directory).
   - `🔱️trinity/🗿️artifacts/♻️rewriting/…/✏️editor/🌍️world/🦀️.rs`.
   - Both should use this run too.
5. **Guest throughput.** At opt-level 0 a 5 000-node iteration costs about 20 ms of stepped work (≈ 20 bounded steps).
   - If a consumer lane measures sluggish large layouts in `wasm-dev`, add
     `[profile.wasm-dev.package.semio-framework-graph-layout-run] opt-level = 2` next to the existing per-package
     overrides.
   - Not added here, because it is unmeasured in a guest.
