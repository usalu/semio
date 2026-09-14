# ⏯️ Wave W3-F1: force-layout consumers (reasoning, dag, trinity)

Lane W3-F1 of `📋️wave-3-lane-brief.md` / `📓️wave-W3-F.md` §3.

**Status**
- The three layout consumers are converted and green: reasoning/wires, dag, and trinity/jack. Each `reorganize` is now a mutating ToolRun driven by `LayoutRunJob`. The old synchronous commands are deleted.
- The DAG topology cost flag is measured. It stays below the ceiling, and a real endpoint bug was fixed along the way.
- **Not converted, needs a coordinator decision (§7.1):** trinity/jack `runQuery` and the trinity/rewriting rule application.

Abbreviations:
- `RW` = `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `DG` = `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `JK` = `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `RR` = `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `L` = `🧰️framework/🔨️modules/🕸️graph/⏯️layout-run`
- `TOOL` = `✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize`

Logs are in `T/🗑️generated/W3-F1/`.

## 1. What changed

### 1.1 Shared, in `L` (foreign: the W3-F crate; additive)

| Where | Change |
|---|---|
| `L/🦀️.rs` Job region end | `LayoutRunResume<'a>`, `layout_run_job(identity, graph, config, encoder_factory, resume)` (the consumer entry: resume when the checkpoint still fits, otherwise fresh; graph and config errors are refused), `layout_run_entity(id)` (W1-A hash convention) and `layout_run_overlay_positions(graph, moves)`. This keeps the three consumers from repeating the resume and fallback logic. |
| `L/🦀️.rs` new region `🔖️Testing` | `pub mod testing`, behind `cfg(any(test, feature = "testing"))`. It contains the fdg-sim oracle moved out of the oracle test (`LayoutRunOracleParameters`, `layout_run_hop_distances`, `layout_run_normalized_stress`, which now skips disconnected pairs, and `layout_run_fdg_layout`). It also adds `layout_run_longest_path_layers` (a petgraph toposort, via fdg-sim's re-export), `LayoutRunRecording`, `layout_run_drive(job, fuel)` (drive_step with a frozen clock, so slicing is deterministic and every step is timed whole) and `layout_run_close`. |
| `L/🦀️.rs` `layout_run_definition` | `settings: ToolRunSettingsReads::default()`. A peer added a required field to `ToolRunDefinition` mid-lane, and the crate no longer compiled without it. |
| `L/📦️packages/🦀️rust/Cargo.toml` | Dependency `semio-framework-hash`. Optional `fdg-sim` plus feature `testing = ["dep:fdg-sim"]`; plugins enable it only from their dev-dependencies. |
| `L/🧪️tests/🔬️oracle/🦀️.rs` | Uses `testing::*`. The test module was renamed `oracle_law`, because the name `oracle` collided. |
| `L/🧪️tests/🔬️unit/🦀️.rs` resume law | New assertions for the consumer entry: own checkpoint resumes, malformed starts fresh, unplaced positions start fresh, the config error is refused, the entity hash is checked. |

### 1.2 Reasoning / wires

| Where | Change |
|---|---|
| `RW/TOOL/🦀️.rs` (new) | `TOOL_ID = "reorganize"`, `LAYOUT_RUN_JOB = "reasoning.wires.layoutRun"`, `definition()`, `WiresLayoutGraph`, `layout_graph(snapshot)`, `move_encoder`, `build_job`. The graph maps visible nodes, keeps the first id when ids repeat, uses half the diagonal as radius for rectangles and otherwise `radius` (default 32), pins `locked` placed nodes, resolves handle and node endpoints, and removes duplicate springs. Config is the FR defaults. |
| `RW/TOOL/🧫️fixtures/🎞️layout-run.json` (new) | Language-neutral law: tool, mapping case, metabolism run (164 iterations, 2 checkpoints, 7 ops, digest `de00acc369a8e484`, verdict prefix), oracle, interactive and lifecycle sections |
| `RW/TOOL/🧪️tests/🔬️unit/🦀️.rs` (new) | 7 laws, see §3 |
| `RW/✏️editor/🦀️.rs` | `forceLayout` and `reorganize` command rows, manifest actions and classifications deleted. Adds `.tool(reorganize::definition())` and `build_tool_run_job`. |
| `RW/✏️editor/🎭️modes/✏️edit/🦀️.rs` | `tools: [reorganize]` |
| `RW/✏️editor/🎮️commands/{⚛️force-layout,🗂️reorganize}` | **Deleted** |
| `RW/🧬️schema/🦀️.rs` | `force_layout_board` deleted |
| `🗿️artifacts/🔌️wires/🦀️.rs` | Module paths updated; the unused `infinite_board_port_directed` alias removed |
| `RW/✏️editor/🧵️retained/🦀️.rs:313` | `preflight` declared `work_items: MAXIMUM_WORK_ITEMS`. The store counts footprint work items as *edit rows* and sums them across a batch, so any multi-move finalize was refused. It now uses `ArtifactStoreOneItemFootprint::for_one_invertible_item(MAXIMUM_RETAINED_BYTES)`. |
| `RW/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | Removed rows, pinned hex `01080000`→`01060000`, and route count 10→8. The context now uses a registry-backed `new_app` bound to the instance (the registry-less one faulted with `catalog-authority`). New `close`. `metabolism_app` now sets the envelope dialect (undo faulted without it) and retires its seed envelope (the drop panicked). |
| `RW/✏️editor/🧫️fixtures/🛣️retained-command-routes.json` | `forceLayout` and `reorganize` routes removed |
| `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-tool-run`. The `dyn_enum_close!` expansion needs it; without it the plugin component did not build. |
| `✏️s/🔌️plugins/💡️reasoning/{🔣️.json,🛂️.descriptor.semio}` | Regenerated with `describe`. They now carry the `reorganize` tool and its `run`, and no `forceLayout`. |

### 1.3 DAG

| Where | Change |
|---|---|
| `DG/TOOL/🦀️.rs` (new) | `LAYOUT_RUN_JOB = "dag.dag.layoutRun"`, `definition()`, `layout_config()` and `layered_targets(snapshot, config)`. The layered targets come from `DagHost::reorganize` once per run; the fixture schema is set to `DAG_DOCUMENT_SCHEMA`, because the old command silently no-opped on the `dag.dag` schema. Also `layout_graph(snapshot, targets)` (anchor = layered target, radius = half the diagonal), `move_encoder` and `build_job`. The anchor-only config uses repulsion 0, spring 0, linear law, anchor 1 and settle 0.01. |
| `DG/TOOL/🧫️fixtures/🎞️layout-run.json`, `DG/TOOL/🧪️tests/🔬️unit/🦀️.rs` (new) | 7 laws |
| `DG/✏️editor/🦀️.rs` | `reorganize` row, action, classification and context-menu entry deleted. Adds `.tool`, `build_tool_run_job` and `build_artifact_store_one_item_preparation_factory` (framework `bounded_config_store_one_item_preparation_factory::<DagSnapshot, DagMutation>`, 4 KiB). Finalize had no factory before. Also adds the document, config, draft, presence and transient owners and disposers; finalize faulted "returned snapshot read requires its exact owned-snapshot retirement factory", and close faulted without them. |
| `DG/✏️editor/🎭️modes/✏️edit/🦀️.rs`, `🗿️artifacts/🕸️dag/🦀️.rs` | Tool mounted in the mode; module paths updated |
| `DG/✏️editor/🎮️commands/🗂️reorganize` | **Deleted** |
| `DG/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | Row count 12→11; reorganize rows removed |
| `DG/🧬️schema/💡️inferences/🧭topology/🦀️.rs` | **Bug fix:** edge endpoints are `node@port`, so every real document had a topology with no edges. The algorithm is rewritten on dense, id-sorted indices with zero-allocation endpoint parsing, and the output is identical. |
| `DG/🧬️schema/💡️inferences/🧭topology/🧫️fixtures/⏱️cost.json`, its unit tests | New laws: `port_endpoints_resolve_to_their_nodes`, and `topology_of_a_large_layered_dag_stays_below_the_interactive_ceiling` (64×64 = 4 096 nodes, 8 064 edges; depth equals the petgraph longest-path layers) |
| `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-tool-run` |
| `DG` artifact `Cargo.toml` | `semio-framework-graph-layout-run`, `semio-framework-tool-run`; dev: layout-run `testing` |
| `✏️s/🔌️plugins/🕸️dag/{🔣️.json,🛂️.descriptor.semio}` | Regenerated |

### 1.4 Trinity

| Where | Change |
|---|---|
| `JK/TOOL/🦀️.rs` (new) | `LAYOUT_RUN_JOB = "trinity.jack.layoutRun"`, and `layout_config()` (FR defaults with `max_iterations: 120`, per W3-F §3.5). Radius is `(w.max(48)+h.max(24))·0.25`; ports are stripped from endpoints. |
| `JK/TOOL/🧫️fixtures/🎞️layout-run.json`, `JK/TOOL/🧪️tests/🔬️unit/🦀️.rs` (new) | 7 laws. The Nakagin run: 9 nodes, 6 edges, `iterationLimit` at 120, 1 checkpoint, 9 ops, digest `8b754fec81d05db0`, all 9 nodes `warning/unsettled`. |
| `JK/✏️editor/🦀️.rs` | `Reorganize` enum variant, id, handle arm, action, classification and menu entry deleted. Adds `.tool`, `build_tool_run_job`, the bounded one-item factory, and the draft and presence owners and disposers (close faulted without them). |
| `JK/✏️editor/🎮️commands/🧭️reorganize` | **Deleted**; mode, crate root and command round-trip test updated |
| `JK` artifact `Cargo.toml` | Optional `semio-framework-graph-layout-run` and `semio-framework-tool-run` under `component-app-assembly`; dev: layout-run `testing` |
| `RR/✏️editor/🌍️world/🦀️.rs` | Deleted the dead synchronous force layout: `TrinityBridge::reorganize`, `force_layout_reposition_operations`, `apply_force_layout_to_trinity_graph`, `trinity_graph_to_force_layout_fixture` and `apply_force_layout_positions_to_trinity_graph`. `TrinityBridge` has no production caller. Their 3 world tests are gone too. |
| `🗿️artifacts/♻️rewriting/🦀️.rs` | `TrinityRewritingError::{Layout, ForceLayoutFixtureMissingNodes}` deleted |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-tool-run` |
| `✏️s/🔌️plugins/🔱️trinity/{🔣️.json,🛂️.descriptor.semio}` | Regenerated |

**Inventory correction.** The rewriting `reorganize` command (`RR/✏️editor/🎮️commands/🧹️reorganize`) only removes the rule layout points. It is one-shot cheap and not a force layout, so it stays a command and its W1-D row is removed.

### 1.5 Other foreign edits

- **Root `📜️script.ts`, `INTERACTIVITY_TOOL_RUN_REQUIREMENTS`:**
  - reasoning: the `forceLayout` row is deleted; the `reorganize` row's scope is now `TOOL`, with `verbs: ["forceLayout", "force-layout"]`;
  - the jack and dag rows now have scope `TOOL`;
  - the rewriting `reorganize` row is deleted;
  - the lane label is now "W3-F1 layout-run consumers".
- **`taxonomy.json`:** `members-of-tools.memberNames` gains `🗂️reorganize`.

## 2. Public API as landed

```rust
// semio-framework-graph-layout-run (additive)
pub struct LayoutRunResume<'a> { pub checkpoint: &'a [u8], pub positions: Vec<LayoutRunPoint>, pub provisional_len: u32 }
pub fn layout_run_job<E: LayoutRunOpEncoder>(identity: ToolRunIdentity, graph: &LayoutRunGraph, config: LayoutRunConfig, encoder: impl Fn() -> E, resume: Option<LayoutRunResume<'_>>) -> Result<LayoutRunJob<E>, LayoutRunStartError>;
pub fn layout_run_entity(id: &str) -> u64;
pub fn layout_run_overlay_positions(graph: &LayoutRunGraph, moves: impl IntoIterator<Item = (u32, LayoutRunPoint)>) -> Vec<LayoutRunPoint>;
#[cfg(any(test, feature = "testing"))] pub mod testing {
    pub struct LayoutRunOracleParameters { pub fdg_scale: f32, pub fdg_cooloff: f32, pub fdg_dt: f32, pub fdg_updates: u64 }
    pub fn layout_run_hop_distances(graph: &LayoutRunGraph) -> Vec<Vec<u32>>;
    pub fn layout_run_normalized_stress(positions: &[[f64; 2]], hops: &[Vec<u32>]) -> f64;
    pub fn layout_run_fdg_layout(graph: &LayoutRunGraph, initial: &[LayoutRunPoint], parameters: LayoutRunOracleParameters) -> Vec<[f64; 2]>;
    pub fn layout_run_longest_path_layers(count: usize, edges: &[(u32, u32)]) -> Option<Vec<u32>>;
    pub struct LayoutRunRecording { pub ops, pub entities, pub upserts, pub trace, pub steps, pub progress, pub ticks, pub checkpoints, pub step_micros, pub complete } // + entity_set()
    pub fn layout_run_drive<J: InteractiveJob + ?Sized>(job: &mut J, fuel: u64) -> Result<LayoutRunRecording, String>;
    pub fn layout_run_close<J: InteractiveJob + ?Sized>(job: &mut J);
}
// each consumer: editor::<plugin>::modes::edit::tools::reorganize
pub const TOOL_ID: &str = "reorganize"; pub const LAYOUT_RUN_JOB: &str;   // reasoning.wires.layoutRun | dag.dag.layoutRun | trinity.jack.layoutRun
pub fn definition() -> ToolDefinition;                                    // run: layout_run_definition(LAYOUT_RUN_JOB)
pub fn layout_graph(snapshot: &Snapshot /* dag: , targets */) -> <Plugin>LayoutGraph { graph, node_ids }
pub fn move_encoder(node_ids: Vec<String>) -> impl LayoutRunOpEncoder + 'static;
pub fn build_job(identity, snapshot, /* dag: config, */ checkpoint: Option<&[u8]>, provisional: &[Mutation]) -> Result<ToolRunJob, Fault>;
pub fn layout_config() -> LayoutRunConfig;                                // dag and jack
pub fn layered_targets(snapshot: &DagSnapshot, config: &DagConfig) -> Result<HashMap<String, LayoutRunPoint>, Fault>; // dag
```

## 3. Tests (all foreground)

| Command | Result |
|---|---|
| `bun nx run @semio-tech/framework-graph-layout-run-rs:test --skip-nx-cache` | TS **7 pass**; nextest **13/13** (`final-layout-run-nx-test.txt`) |
| `cargo test -p semio-s-artifact-reasoning-wires --lib -- tools::reorganize --test-threads=4` and `=1` | **7/7** each (`final-reasoning-threads{4,1}.txt`) |
| `cargo test -p semio-s-artifact-dag-dag --lib -- tools::reorganize inferences --test-threads=4` and `=1` | **15/15** each (`final-dag-threads{4,1}.txt`) |
| `cargo test -p semio-s-artifact-trinity-jack --features component-app-assembly --lib -- tools::reorganize --test-threads=4` and `=1` | **7/7** each (`final-jack-threads{4,1}.txt`) |
| `cargo check -p semio-s-artifact-{reasoning-wires,dag-dag,trinity-jack,trinity-rewriting} --features …component-app-assembly --target wasm32-wasip2` | Finished; warnings reached in the dag, jack and rewriting crates (`wasip2-check-1.txt`) |
| `bun ./📜️script.ts describe` in the reasoning, dag and trinity plugin packages | All three plugin components **build for wasm32-wasip2** (`--profile wasm-dev`) and describe; descriptors regenerated (`*-describe-*.txt`) |
| `bun T/🐍️w3f1-policy-probe.ts` (W1-D repo-wide predicates) | amend, local-lifecycle, legacy-trace and reserved-action: 0. Declaration: 17 total, **0 in lane roots** (`policy-probe-2.txt`) |
| `bun ./📜️script.ts verify dependencies literal-external` | fdg-sim and petgraph are not in `oracle-conflicts`. The gate itself is red for pre-existing reasons (198 literal-external) (`deps-literal-external.txt`) |

**Laws per consumer (the same 7 shapes):**
1. **Declaration.** The tool declares exactly `layout_run_definition`, its policies match the fixture, the mode references it, the framework actions are injected, and the old verb no longer resolves.
2. **Language-neutral mapping** from the fixture: radius, pin and anchor rules; handle and port resolution; self-loop, dangling, hidden and duplicate springs dropped.
3. **Fixture run** with fuel 1, so every iteration is one step: stop, iterations, checkpoints, a 24-entry verdict prefix, the bit digest (reasoning, jack), exactly one `move-node` per moved node carrying the final position, entity set equals the moved entities, and the final verdict census. DAG additionally requires the landed layout to sit within 0.5 of every layered target (measured 0.012).
4. **Third-party oracle.**
   - reasoning and jack: **fdg-sim** FR stress ratio ≤ 1.05 and improvement ≥ 30 %. Metabolism: 0.3774 → 0.0135 vs fdg-sim 0.0135. Nakagin: 0.0495 → 0.0036 vs 0.0037.
   - dag: a **petgraph** longest-path layering oracle, run from the demo mirrored left to right. No edge may point against the layer axis, sources must share the leftmost column, and the deepest layer must hold the rightmost column. Stress is not meaningful for a layered layout (FR ratio 19.7), see §6.
5. **Worst whole `drive_step`** (best of 5) < 2 000 µs on the largest example. Measured: reasoning 131 µs, dag 124 µs, jack 148 µs.
6. **start → complete → finalize** through the real framework actions. Nothing is committed before finalize, trace pages are published, history gains +1, every node moves, and one `settle_history_verb("undo")` restores every committed position.
7. **Abort** once provisional moves exist: document pack and spr byte-identical, history unchanged.

**DAG topology:** at 4 096 nodes and 8 064 edges the best of 5 is 6.2 ms at opt-level 0. That is below the 8 ms ceiling, so no stepped run is needed; it is above the 2 ms target (the first version measured 30 ms). The depth matches the petgraph oracle.

**Mutation checks** (each file restored and verified with `cmp`):
- reasoning encoder writing every move to `node_ids[0]` → the run fixture law failed;
- dag anchors removed → mapping, landing, finalize and the strengthened petgraph law failed. The first petgraph law version passed this mutation, which is why it now starts from a mirrored layout.

**Pre-existing reds, same set with and without this lane's changes:**
- **jack** full lib: 41 failures. Measured by removing this lane's owner and factory additions: the identical 39 non-lane failures plus this lane's 2 lifecycle laws (`jack-lib-without-owners.txt` vs `final-jack-lib-full.txt`).
- **dag:** 5 render/topology tests fail with "artifact store reached Drop"; identical without the store owners (`dag-baseline-owners.txt`).
- **reasoning** full lib aborts on `BatchOnlyPendingRewrite` commands (`interactive-job.missing-factory`), a retained-route fixture mismatch that predates this lane (`nodeGraphViewport` is Migrated in code but BatchOnly in the fixture), and destructor panics in older tests.
- **rewriting world:** `TrinityBridge` store tests fail (Drop witness / retirement factory).

## 4. Commands to register in launch.json

- `cargo test -p semio-s-artifact-reasoning-wires --lib -- tools::reorganize`
- `cargo test -p semio-s-artifact-dag-dag --lib -- tools::reorganize inferences`
- `cargo test -p semio-s-artifact-trinity-jack --features component-app-assembly --lib -- tools::reorganize`
- `bun ./📜️script.ts describe` in `✏️s/🔌️plugins/{💡️reasoning,🕸️dag,🔱️trinity}/📦️packages/🦀️rust` (component build plus descriptor)

## 5. Framework synchronous force layout: not deleted

`rg` shows live users, so none of it was removed:
- `🕸️graph/🖊️drawing::force::{run_force_layout, seed_positions}` is used by `♾️infinite/🎲️board/➕️normal/↔️undirected` `apply_force_graph_layout_*`.
- That module is used by `🔌️ports/➡️directed` `force_graph` and the redraw `mode: force-graph` path.
- Puzzle 2d `🎮️commands/⚛️force-layout` and its `⚙️engine/📐️layout` tests use them.

After this lane, **no reasoning, dag or trinity code calls any of them.** Remaining owners: puzzle 2d (W2-A area), and flow and sequence if they still reach the directed wrapper.

## 6. Deviations

1. **Reasoning has one tool.** `forceLayout` and `reorganize` were byte-identical synchronous handlers, so they are one `reorganize` tool. The `forceLayout` W1-D row is removed and its verbs are now forbidden on the `reorganize` row.
2. **Shared consumer entry and test support in `L`.** Without them the resume and fallback logic, the fdg-sim oracle and a tick-folding harness would be copied into three plugins, and CLAUDE.md requires repeated code to sit together.
3. **The DAG oracle is petgraph layering, not stress.** The layered layout is not stress-optimal, so FR stress is no quality bound there.
   - `DagHost`'s Buchheim layering puts a node that has several parents under its *shallowest* parent. In the demo, `scale → combine` is therefore drawn inside one column, whereas petgraph puts `combine` one layer deeper.
   - The law asserts the monotone property that holds and records this defect (§7.3). It does not paper over it.
4. **Jack keeps `max_iterations: 120`,** as W3-F §3.5 prescribes. Nakagin ends at `iterationLimit`, so every node is `warning/unsettled` in the trace.
5. **Finalize factories.**
   - dag and jack use the framework bounded one-item factory; it clones the snapshot once per move.
   - reasoning keeps its bounded retained factory, but its footprint declaration is corrected.

## 7. Open items

1. **trinity/jack `runQuery` and trinity/rewriting rule application were not converted. This needs a coordinator decision.**
   - **`runQuery` is `algorithmic-mutating`, not read-only.** SET/CREATE/DELETE/MERGE clauses emit `TrinityGraphMutation`s (`JK/✏️editor/🎮️commands/▶️run-query/🧵️job/🦀️.rs:258`); the inventory is wrong.
   - **It conflicts with a tested invariant.** `jack_graph_window_config_query_ownership_isolates_two_editor_result_pairs…` runs two editor→results pairs concurrently, while the ToolRun contract allows **one** non-terminal run per document instance (§2.2 invariant 5). Converting would remove a tested feature.
   - **There is no result sink.** A run job cannot publish window transients. Result rows would need a port hop command addressing the results window (W3-2 pattern), and `ToolRunJobRequest` carries no window roster from which to find the paired results window.
   - **Rewriting shares the executor.** Rewriting's `after_fixture_json` solves inside render through the same executor (`RR/🧬️schema/🦀️.rs:279`). A match trace needs an observable `QueryExecution` (binding matched, WHERE kept or filtered, mutation applied, row returned) in `JK/🧬️schema/🧮️executor/🪜️execution`, plus a pending-effects-driven read-only run like W3-2's preview eval.
   - **Proposal:** one follow-up lane for both. Decide first whether concurrent query pairs survive: either per-window runs in the framework, or a single run with a results-window target.
2. **No provisional stamp on 2d node graphs.** The wires, dag and jack graph canvases are not `world-3d`/`canvas-2d` scene surfaces, so W0-H does not inject the `toolRunTrace` lane there, and no 2d `provisional` node style exists. The run is visible through the overlay (nodes move live) and through the panel trace list, but the canvas has no per-node verdict tint yet.
3. **`DagHost` layering defect** (§6.3), in foreign `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` `buchheim_positions`. It is shared with flow and sequence.
4. **Tool-run crate path in a framework macro.** `dyn_enum_close!` expands to `semio_framework_tool_run` paths, so every plugin crate needs that dependency (added here for 3 plugins). The macro should re-export through `semio_framework_plugin`.
5. **`TrinityBridge`** (`RR/✏️editor/🌍️world`) has no production caller and red store tests; it is a deletion candidate.
6. **DAG topology** is 6 ms at 4 096 nodes. Watch it if DAG documents grow.
7. **Pre-existing plugin test debt** (§3): reasoning and dag `BatchOnlyPendingRewrite` commands are dispatch-dead, and older tests drop apps without the close protocol.
8. **Not verified:** `bun ./📜️script.ts verify taxonomy report` did not finish within 600 s and was stopped, so the taxonomy registration of `🗂️reorganize` is unverified by the gate.

Scripts in `T`: `🐍️w3f1-consumer-entry-refactor.py`, `🐍️w3f1-policy-probe.ts`.
