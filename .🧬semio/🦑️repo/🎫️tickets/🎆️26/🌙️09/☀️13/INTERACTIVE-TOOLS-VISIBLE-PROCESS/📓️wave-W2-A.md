# ⏯️ Wave W2-A: puzzle 2d fill as a tool run

Lane W2-A of `📋️tool-run-contract.md` (§2.4, §2.7, §3.2, §3.6, §3.7, §5 wave 2). **Status: landed.** The lane's own tests
pass at `--test-threads=4` and `=1`. The native and `wasm32-wasip2` checks reach the crate's warnings. W1-D's policy
predicates report 0 findings for puzzle 2d.

Paths below are relative to the repo root.

- `A` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `E` = `A/✏️editor`
- `F` = `E/⏳️precompute/🪣️fill`

Logs are in `T/🗑️generated/W2-A/`.

## 1. What changed

### 1.1 New files

| File | Content |
|---|---|
| `F/🦀️.rs` (1 614 lines) | Regions `🔖️Limits`, `🔖️Vocabulary` (`FillRunStage`/`FillRunCounter`/`FillRunReason`/`FillRunPlacementKey`/`FillRunCheckpoint`, `:32`), `🧱️Placement` (`:273`), `🔬️CaptureCursor` + `🔬️Capture` (the document capture moved out of `set-fill-count`, `:364`/`:574`), `⏯️RunJob` (`Puzzle2dFillRunJob`, `:995`), `🔍️RevalidateJob` (`Puzzle2dFillRevalidateJob`, `:1434`) |
| `F/🧫️fixtures/🎞️fill-run.json` | Language-neutral run law: 5 cases (document spec, run id, count → exact verdict prefix, decision prefix, counters, op/entity/checkpoint counts, stall), plus the geo oracle, resume, interactive, revalidate and finalize parameters |
| `F/🧪️tests/🔬️unit/🦀️.rs` | 9 laws (§3) |

### 1.2 Edited files

| File | Change |
|---|---|
| `A/🧬️schema/🔣️.json` | New `$defs`: `Puzzle2dFillRun` (source of record, with `x-semio-toolRun` holding the policies, trace kind, job kinds, unit, stages, counters, reason codes with verdicts and EN/DE templates, and the checkpoint layout), plus `…Stage/…Counter/…Reason/…Checkpoint`. All are `x-semio-formats: 🔣️jsonschema` |
| `E/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` | `definition()` now declares `run: Some(run_definition())`. `run_definition()` builds its labels from the terminology. `measures()` is the count `WindowMeasure::Number` only. The cancel and retry toggles, the lifecycle helpers and the stage text are deleted |
| `E/🎮️commands/🧮️set-fill-count/🦀️.rs` | Rewritten as a 15-line generic config verb: it sets `runtime.fill_count`, and a malformed count is a no-op. The session work, the capture and apply cursors and `fill_session_control` are deleted |
| `E/🎮️commands/{🏁️fill-session-begin,👣️fill-session-step,🧹️fill-session-clear}/` | **Deleted** |
| `E/🎚️config/🦀️.rs` + `E/🎚️config/🧬️schema/{🦀️.rs,🟦️.ts,🔣️.json,🔗️.graphql,🛰️.proto}` | `Puzzle2dFillLifecycle`, `Puzzle2dFillRuntime`, `Puzzle2dFillText` and every `fill_job_*` runtime field are **deleted**. `Puzzle2dConfig.fill_count: u32` (default 100) is added in all five schema languages plus the Rust config, with a hand `Default` |
| `E/🪟️window/🦀️.rs` + `E/🪟️window/🧬️schema/*` | `fill_count` is removed from `Puzzle2dWindowConfig` in all languages. `runtime`/`split` read and write the count through `Puzzle2dConfig` |
| `E/🗣️terminology/🦀️.rs` | The old `fill_progress/cancel/retry/fault/cancelled/stage_*/tested/accepted` labels and `puzzle2d_fill_stage_label` are replaced by `fill_unit`, `fill_stage_{capture,search,test,place,retract}`, `fill_counter_{tested,accepted,collisions,rejected}` and 11 `fill_reason_*` labels. Each has all four locale × terminology cells |
| `E/🦀️.rs` | See the bullet list below |
| `E/🧪️tests/🔬️unit/🦀️.rs` | See the bullet list below |
| `E/🎚️config/…/🧪️tests`, `E/🪟️window/…/🧪️tests`, `🛠️tools/🪣️fill/🧪️tests`, `🧮️set-fill-count/🧪️tests` | Updated or rewritten (§3) |
| `A/🧫️fixtures/🗄️retained-jobs/🔣️.json` | Every `brushFillSession*` entry is removed from tool ids, evidence ids, semantic cursors, hostile mutations and vectors. `setFillCount` evidence, cursors and vectors are removed too, because it is now a generic verb |
| `◻️2d/🦀️.rs` | The `editor::puzzle2d::precompute::fill` module is mounted and the three fill-session command modules are removed |
| `◻️2d/📦️packages/🦀️rust/Cargo.toml` | New deps `semio-framework-tool-run` and `semio-framework-hash`. New dev-dep `geo = { version = "0.31.0", default-features = false }` (test-only oracle) |

Changes in `E/🦀️.rs`:

- **Removed:**
  - the 7 `BrushFillSession*` command variants;
  - their retained ids, publication contracts, tool proofs, manifest actions and interactive-job classifications;
  - the fill refusal in `handle` and the fill arm in `build_tool_job`.
- **`setFillCount` is now generic:**
  - it joined `PUZZLE2D_GENERIC_TOOL_IDS` and `puzzle2d_dispatch_emit`;
  - its lane is `Config`;
  - its `ActionKind` is `View`.
- **Added:**
  - `build_tool_run_job` (`:3637`);
  - `const TOOL_JOB_IDS = PUZZLE2D_RETAINED_TOOL_IDS` (`:869`);
  - draft owners and disposer, presence retirement factories and disposer, transient disposer (`:3471-3496`).
- **Removed `Puzzle2dImportJobFactory`** and its registration (§5.4).

Changes in `E/🧪️tests/🔬️unit/🦀️.rs`, the shared test harness:

- `app()` is now registry-backed.
- `settle` drains typed-operation completions and acknowledges local-interaction pages.
- Framework-reserved verbs settle through `settle_framework_reserved_admission`.
- The two fill-session source-contract tests are deleted.

### 1.3 Foreign edits (smallest possible)

| File | Edit |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` | Additive only. New `pub enum BoardFillCandidateVerdict { Testing, HostCollision, VirtualCollision, PortIncompatible, RuleIncompatible, Fits }` (`:932`) and `pub struct BoardFillCandidateEvent { verdict, kind_index, position, bounds }`. A `candidate_event` state slot is set at the exact decision points: construct preview, host or virtual overlap, port-shape refusal, kind-rule refusal and fits. It is taken by `pub fn take_candidate_event` (`:6307`) and cleared on close. Two private helpers (`kind_event`, `preview_event`) now also compute the preview bounds. Search behaviour, RNG and order are unchanged |
| `📜️script.ts` (W1-D requirement row for 2d fill) | `scope` becomes `🛠️tools/🪣️fill`, `🎮️commands/🧮️set-fill-count` and `⏳️precompute/🪣️fill`; `actions` becomes `["setFillCount"]`. The verbs and measures that must stay deleted are unchanged |
| `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` | 2d owner: the `brushFillSession*` routes are removed and `setFillCount` moves to the `config` group |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` | The 2d `Puzzle2dWindowConfig` neutral case drops `fillCount` |

## 2. Public API as landed

### 2.1 Vocabulary (`editor::puzzle2d::precompute::fill`)

```rust
pub enum FillRunStage { Capture, Search, Test, Place, Retract }   // ALL, index(), id(), label() -> fn(&Puzzle2dLabels) -> LabelText
pub enum FillRunCounter { Tested, Accepted, Collisions, Rejected } // ALL, index(), id(), label()
pub enum FillRunReason { Fits, HostCollision, VirtualCollision, PortIncompatible, KindIncompatible,
    NoOpenHandle, NoCompatibleKind, NoFreePlacement, DocumentCapacity, RequestedReached, Retracted } // code()=ordinal, from_code, id(), verdict(), label()
pub struct FillRunPlacementKey { pub key: u64, pub shape: u32 }
pub struct FillRunCheckpoint { pub requested, pub tested, pub collisions, pub rejected, pub next_key: u64, pub placements: Vec<FillRunPlacementKey> }
impl FillRunCheckpoint { pub const HEADER_BYTES: usize = 44; pub const PLACEMENT_BYTES: usize = 12; pub fn encode(&self) -> Vec<u8>; pub fn decode(&[u8]) -> Option<Self> }
```

Verdict per reason:

- `fits` and `requested-reached` are **success**.
- `host-collision` and `virtual-collision` are **danger**.
- `retracted` is **testing** (an info step).
- Every other reason is **warning**.

### 2.2 Jobs

```rust
pub(crate) const FILL_RUN_TICK_FLUSH_BYTES: usize = 8 * 1024;
pub(crate) const FILL_RUN_OPS_PER_PLACEMENT: usize = 2;
pub(crate) fn fill_run_entity(node_id: &str) -> u64;      // first 8 LE bytes of semio_framework_hash::hash(id)
pub(crate) fn fill_node_bounds(x, y, scale, rectangle, radius_or_width, height) -> [f64; 4];
pub(crate) struct Puzzle2dFillRunJob;          // impl InteractiveJob
impl Puzzle2dFillRunJob {
    pub(crate) fn new(identity: ToolRunIdentity, document: Arc<Puzzle2dPlaySnapshot>, suggestion_offset: f64, requested: u32,
                      checkpoint: Option<&[u8]>, provisional: &[Puzzle2dMutation]) -> Result<Self, &'static str>;
    pub(crate) fn counters(&self) -> [u64; 4]; pub(crate) fn checkpoint(&self) -> FillRunCheckpoint;
}
pub(crate) struct Puzzle2dFillRevalidateJob;   // impl InteractiveJob
impl Puzzle2dFillRevalidateJob { pub(crate) fn new(identity, head: Arc<Puzzle2dPlaySnapshot>, provisional: &[Puzzle2dMutation], checkpoint: Option<&[u8]>) -> Self; }
```

### 2.3 Editor wiring

```rust
// modes::edit::tools::fill
pub const PUZZLE2D_FILL_RUN_JOB: &str = "puzzle2d.fill.run";
pub const PUZZLE2D_FILL_REVALIDATE_JOB: &str = "puzzle2d.fill.revalidate";
pub fn definition(label: LocalizedLabel) -> ToolDefinition;  // run: Some(run_definition())
pub fn run_definition() -> ToolRunDefinition;                // mutating, Revalidate, Resume, Placement2d
pub fn count_measure(..) -> WindowMeasure;  pub fn measures(..) -> WindowMeasure;
// commands::set_fill_count
pub fn set_fill_count(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>);
// config
pub struct Puzzle2dConfig { pub node_kind_weights, pub handle_kind_weights, pub fill_count: u32 }
// ArtifactEditor for Puzzle2dPlayApp
fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault>; // Run → run job over request.snapshot with request.config.fill_count; Revalidate → revalidate job over the head
```

The finalize preparation factory is the existing `Puzzle2dArtifactStorePreparationFactory`. It is exercised end to end by the finalize law; no new factory was needed.

### 2.4 Run job semantics

**Capture.**

- The job streams the base document into the engine's `BoardFillSnapshotIngress`, 256 units per deadline check.
- Kind rows are the document's `meta.kindCatalogs.nodes`. If the document has none, they are the manifest catalog's `nodeKinds` (`board_kind_catalogs_json`).
- The job serial is the value after the highest `puzzle2d.fill.<n>` node id, so a second run never reuses an id.
- The seed is the run id.
- The engine runs with `max_count = u32::MAX`, so the job decides completion itself.

**Candidates.** One unit of fuel is one final verdict.

- An engine `Testing` event produces `Upsert{key, testing, fits, Placement2d{shape: kind index, position, 0}}`.
- A host or virtual overlap produces `danger` with the matching reason.
- A port or kind refusal produces `testing` then `warning` on a fresh key.
- `Fits` is remembered. The success upsert, the two `OpBinary` ops (`create_node`, `connect_handles`) and the entity are appended when the engine's `CheckpointReady` hands over the placement.
- The placement is then closed and the engine checkpoint re-adopted.

**Ticks.** A tick is flushed when the fuel is exhausted, the deadline passes or about 8 KiB is pending. A placement flushes a tick, and `CheckpointReady` follows on the next call. At completion:

1. a closing step is written: success `requested-reached [n]`, or warning `no-open-handle` / `no-compatible-kind` / `no-free-placement` / `document-capacity [n]`;
2. the tick carries progress `Complete`;
3. the next call returns the final `CheckpointReady`;
4. the call after that returns `Complete`.

**Resume** (reconfigure `resume`).

- A job built with a checkpoint and provisional ops replays the same search silently, with no trace, no ops and no fuel. It compares every re-derived placement's op bytes with the provisional prefix.
- **Raise:** after the kept placements, the job continues live.
- **Lower:** `retract_to(2·kept)`, the checkpoint keys past `kept` are retired, and an `info retracted [kept]` step is written.
- **Divergence** (for example after a rebase) retracts to the first differing placement and continues live.

**Revalidate** (one unit of fuel per placement). A placement conflicts when any of these holds:

- its node id is already in the head;
- its source handle is neither in the head nor on an earlier survivor;
- its footprint overlaps a head node.

Each placement gets a `testing` then `success`/`danger(TOOL_RUN_REASON_CONFLICT)` upsert under its checkpoint key. The final tick uses `retract_to(2·first conflict)`, re-appends the ops and entities of the later survivors, and adds a `danger` conflict step with the count.

## 3. Tests (TDD: fixture and laws first; expected fixture values filled from the first deterministic run, cross-validated by the count laws and the geo oracle)

### 3.1 Laws in `F/🧪️tests/🔬️unit/🦀️.rs`

| Law | Proves |
|---|---|
| `fill_run_job_matches_the_language_neutral_fill_run_fixture` | 5 cases; see the list below. Every case also checks the run laws |
| `fill_run_checkpoint_codec_round_trips_and_refuses_malformed_lengths` | Codec exactness |
| `fill_run_job_collision_verdicts_agree_with_the_geo_oracle` | See §3.2 |
| `fill_run_job_step_with_one_unit_of_fuel_reaches_exactly_one_candidate_verdict` | Every tick has ≤ 1 final verdict, exactly `tested` ticks have one, and the verdict map equals a free run |
| `fill_run_job_resume_raise_continues_the_sequence_and_lower_retracts_the_tail` | Raise 6→9: ops, verdicts and counters equal a direct run to 9, and no earlier candidate is re-announced. Lower 6→3: `retractTo = 6`, the op prefix is kept, exactly the checkpoint keys 3..6 are retired, and there is a retracted step, `Complete` and completed = 3 |
| `fill_revalidate_job_retracts_conflicting_placements_and_reappends_survivors` | Clean head: no retract and 6 successes. Intruder on placement 3: `retractTo = 6`, the survivors 4 and 5 are re-appended, `danger` for exactly key 3, and a conflict step `[1]` |
| `fill_run_job_drive_step_stays_below_the_interactive_ceiling_for_nakagin` | Largest example (Nakagin, all 180 nodes, edges detached, count 120), `drive_step` under `INTERACTIVE_LANE_WALL_US`. Per turn, the best of 5 cold runs replaying one recorded clock. Last run: **13 072 turns, worst 1 141 µs < 2 000 µs** |
| `fill_run_start_complete_finalize_is_one_undo_entry` | Registered app, Nakagin reduced to its first node, count 8. Runs `toolRunStart` until `complete`: the committed document is unchanged, the history is unchanged, and trace pages exist. `toolRunFinalize` until `finalized`: nodes +8 and **history +1**. **One `undo` removes all 8 placements** |
| `fill_run_abort_leaves_the_document_byte_identical` | After ≥ 2 provisional placements, `toolRunAbort` leads to `aborted`. `document_pack` pack+spr are **byte-identical** and the history is unchanged |

The fixture's five cases:

- Nakagin first node, run 1, count 5 and count 12;
- Nakagin with detached edges, run 3, count 40;
- Concrete Forest (no kind catalog), which stalls with `no-compatible-kind`;
- the empty board, which stalls with `no-open-handle`.

The per-case run laws:

- 2 ops and 1 entity per placement;
- the success, danger and warning counts equal the counters;
- every final verdict was preceded by `testing`;
- ops alternate `CreateNode`/`ConnectHandles`;
- entity = `fill_run_entity(node id)`;
- no document id is reused;
- the run ends in `Complete` with a closing step whose argument is the placement count;
- the final checkpoint equals the counters.

Tests elsewhere in the crate:

- `🛠️tools/🪣️fill`: 4 tests — the count is the only unbounded measure with default 100; the declared `ToolRunDefinition` equals `$defs.Puzzle2dFillRun` (policies, job kinds, ids, codes, verdicts, EN/DE texts); the 7 framework actions are injected.
- `🧮️set-fill-count`: 1 test — the config count is rendered by the tool measure, the document is untouched, and malformed counts are no-ops.
- `🎚️config`: the fill count is in the config and no window or operation state is.

### 3.2 geo oracle

The oracle is **third-party `geo` polygon intersection**, run on Nakagin with detached edges, run 7, count 60.

**How it recomputes a verdict.**

- **Candidate footprint:** the kind-catalog size (96·scale) at the traced position.
- **Board nodes:** their footprints are recomputed from the document JSON independently.
- **Order:** host nodes are tested first, then the placements accepted before the candidate.
- **Ambiguity:** a verdict whose deciding depth lies within the `f32` position precision counts as ambiguous.

**Result:** 27 163 decisive, **0 disagreements**, 0 ambiguous; 27 157 collisions, 6 fits.

### 3.3 Commands run

All runs used `RUST_MIN_STACK=134217728` and ran in the foreground.

| Command | Result |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly -j 4 --message-format short` | Finished; the crate reached its 4 (pre-existing) warnings (`check-native-final.txt`) |
| `… --target wasm32-wasip2` | Finished, 0 errors; the crate reached its 4 warnings (`check-wasip2-final.txt`) |
| `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib -j 4 -- --test-threads=4` (whole lib) | 520 passed, 193 failed (`test-lib-threads4.txt`). **All 17 lane tests pass.** The failures are listed below |
| `cargo test … --lib -j 4 -- fill --test-threads=1` | 19 passed, 9 failed, 318 s (`test-fill-threads1.txt`). The 9 failures are the same `engine::brush` ones; every lane test passes single-threaded |
| `cargo test -p semio-framework-os-infinite --lib -- fill` (foreign engine edit) | 6/6, including the fill ingress/handback source-contract laws (`test-os-infinite-fill.txt`) |
| `bun T/🐍️w1d-policy-probe.ts` | Self-tests 110 cases pass. **Puzzle 2d: 0 findings** in amend, local-lifecycle, legacy-trace, declaration and reserved-action. The remaining findings belong to other lanes (`policy-probe.txt`) |
| `bun ./📜️script.ts publication-authority-audit Puzzle2dPlayApp` (puzzle TS package) | exit 0, validated (`publication-authority.txt`) |

The 193 whole-lib failures, none in lane files:

- **154 `schema::mutations` fixture laws.** The produced JSON has `3.0` where the committed fixture has `3`. `cargo tree -e features -i serde_json` shows the serde_json features unchanged by the `geo` dev-dep.
- **23 `component::unit_tests`** in the older harness. The causes are framework behaviours: `edit history insertion requires its exact mutation retirement factory`, `batched publication has no retained outbound backbone encoder/sender authority`, a publication-lane mismatch on `setCamera`, and store Drop without close. Before this lane they already failed earlier, at app construction (see §5.4).
- **11 `engine::brush`.** `MountedWorkerJobSession::pump_one` faults or `EventCredits` unwrap; they fail identically at threads 1.
- **2 `panels::document`, 1 `overview` render, 2 `schema::snapshot`.** `BuiltChildren requires retained page transport`, store Drop, and the history retirement factory.

## 4. Commands to register in launch.json

- `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib -- precompute::fill tools::fill set_fill_count` — the W2-A laws, about 2.5 min in debug.
- `cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --target wasm32-wasip2`

## 5. Deviations, with reasons

1. **The count moved from window config to `Puzzle2dConfig`.**
   - `ToolRunJobRequest` carries only the app config, not the starting window's config.
   - The driver's `settingsChanged` watch sees config-store generations, so a count change now reconfigures the run.
   - The count is a mode-level tool measure, so a shared preference fits.
2. **The suggestion offset of a run is `PUZZLE2D_DEFAULT_SUGGESTION_OFFSET`, not the window's brush offset**, for the same request gap. See open item 1.
3. **Resume is a silent deterministic replay, not a state restore.** `BoardFillJob` state is private to the framework engine and cannot be serialized into a checkpoint page.
   - The 44 + 12·n byte checkpoint carries the counters and the placement keys and shapes.
   - Correctness is guarded by byte-comparing the replayed ops with the provisional prefix.
4. **`Puzzle2dImportJobFactory` was removed and `TOOL_JOB_IDS`, draft/presence/transient owners and disposers were added.** Without them no 2d app could be constructed or closed on the current framework, so no app-level law could run.
   - Since 26/09/10 the framework registers its own reserved `import-media` factory. The duplicate registration faulted `interactive-job.owner-registration`.
   - `import-media` still routes through `build_reserved_tool_job`.
5. **The shared 2d test harness was updated** (registry app, completion drain, reserved-verb settle) for the same reason.
6. **Candidates include rule refusals.** Port-shape and kind-rule refusals are traced `warning` candidates (one unit of fuel each), so the trace shows why an open handle got no kind. On Nakagin they dominate: 4 869 of 5 397 verdicts in the first-node case.
7. **Test documents are reduced.** The shipped examples cannot be filled as shipped:
   - Concrete Forest has no kind catalog; its manifest `concrete-forest` is not registered.
   - Nakagin has no open handle.

   The fixture therefore reduces Nakagin to its first node (`keepNodes: 1`) or detaches all its edges. Both stalls are pinned as cases.
8. **The trace subject shape index is the capture-order kind index.** A renderer must use the same kind-row order (open item 2).

## 6. Open items

1. **W0-H / W0-D.** Add the starting window's config to `ToolRunJobRequest`, for example `window_config: Option<WindowConfigSnapshot>`. Then the fill run can use that window's `suggestionOffset`, and the count could return to window config if wanted.
2. **W0-E / host.**
   - The 2d windows render a `Board2dScene` surface, but `tool_run_scene_surface` only injects the `toolRunTrace` lane into World3d and Canvas2d scenes. Placement2d trace pages are therefore produced but not delivered to the 2d board.
   - Board2d needs a `toolRunTrace` lane and a mount of `ToolRunTrace2dLayer` whose `pathForShape` indexes the fill kind rows.
   - The same host must draw `ArtifactView::tool_run()` provisional entities, which are digests of the node ids.
3. **Coordinator.**
   - Triage the pre-existing 2d reds (§3.3).
   - `✏️s/🔌️plugins/🧩️puzzle/🔣️.json` (describe output) still lists the removed 2d verbs; regenerate it after wave 2.
   - Taxonomy member names `🏁️fill-session-begin`, `👣️fill-session-step` and `🧹️fill-session-clear` are now unused by 2d.
4. **Product.** Concrete Forest needs a registered manifest or document kind catalogs before fill or brush can place anything on it.
