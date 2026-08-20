# Phase 2 Design Inputs: Interactive Job Runtime Refactor

This document gathers evidence from existing code to support the design of the universal resumable job protocol (`InteractiveJob` trait and `StepContext`/`StepOutcome`).

---

## 1. `semio-framework-machine` — PersistedSnapshot/step() Round-trip

**File**: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔄️machine/🦀️component.rs`

### API Structure

#### PersistedSnapshot (lines 1286–1292)
```rust
pub struct PersistedSnapshot {
    pub version: u32,
    pub fingerprint: u64,
    pub states: Vec<String>,              // Stable state ids
    pub history: Vec<(String, Vec<String>)>, // History per compound state
    pub done: bool,                       // Terminal status
}
```

#### Persist/Restore Functions (lines 1324–1363)
- `persist<M>(snapshot: &Snapshot<M>) -> PersistedSnapshot` — Captures running snapshot with stable ids
- `restore<M, Mg>(persisted: &PersistedSnapshot, context: M::Context, migrations: &[&Mg]) -> Result<Snapshot<M>, RestoreError>` — Rebuilds snapshot, applies migrations, resolves stable ids back to dense `NodeId`s

#### Step Entry Point (lines 1721–1727)
```rust
pub fn step<M, Mg>(prior: &PersistedSnapshot, context: M::Context, event: M::Event, migrations: &[&Mg]) 
  -> Result<MachineStep<M>, RestoreError>
```
Performs: restore → run one macrostep → persist. The live `Snapshot<M>` is confined to this frame.

#### MachineStep Output (lines 1675–1694)
```rust
pub struct MachineStep<M: Machine> {
    pub entered: Vec<&'static str>,      // Union of nodes entered
    pub exited: Vec<&'static str>,       // Union of nodes exited
    pub active: Vec<&'static str>,       // Settled configuration
    pub commands: Vec<Command<M>>,       // Effects for the host
    pub report: StepReport,              // Metrics/status
    pub persisted: PersistedSnapshot,    // For next cycle
}
```

#### Command Effect Representation (lines 352–436)
Commands are accumulated during kernel execution — no preview channel or fault channel exists. Inspector pattern via `trait Inspector<M>` (lines 276–280) emits `InspectionEvent` during macrostep:
- `MacrostepStart`, `Microstep { exited, entered }`, `CommandIssued`, `Settled { microsteps }`

### Current Limitations
1. **No deadline-bounded step** — `run_to_completion` drains to quiescence counted only by microsteps (COUNT-bounded), not wall time
2. **No mid-macrostep yield** — runs to completion every time
3. **No preview channel** — effects surface only at the end
4. **No fault channel** — errors surface as `RestoreError` only on restore, not during step
5. **Inspector emits during step** but is a post-hoc observer, not integrated with deadline/fuel tracking

### Implications for `InteractiveJob`
- **Reuse**: `PersistedSnapshot` structure is solid; the stable-id-based round-trip pattern should generalize
- **Adopt**: The read-restore-step-persist cycle is the template
- **Extend**: Need to thread `StepContext { deadline, fuel, … }` through the step, yield `StepOutcome` with granular states (Yield, PreviewReady, CheckpointReady, Complete, Cancelled, Fault)
- **Remove**: No need for a separate `Inspector` trait once tracing is integrated into `StepContext`

---

## 2. Actor Layer's Existing Job Vocabulary

**File**: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` (~3664 lines)

### Payload Variants (lines 580–605)
```rust
pub enum Payload {
    Event { bytes: Vec<u8> },
    Suspend { checkpoint: bool },       // Request suspend; checkpoint bool flags data presence
    Resume { checkpoint: Option<Vec<u8>> }, // Resume with optional checkpoint data
    Cancel { seq: u64 },                // Cancel by sequence number
    JobStep { job: u64 },               // NEW: job identifier to step
}
```

### TurnStatus Enum (lines 714–747)
```rust
pub enum TurnStatus {
    Idle,                               // No more work
    MoreWork,                           // Tick again soon
    CheckpointReady,                    // Checkpoint available
    Faulted { detail: Vec<u8> },        // Error payload
}
```

### Budget Structure (lines 437–484)
```rust
pub struct Budget {
    pub fuel: u64,                      // Instruction-equivalent units
    pub wall_ms: u32,                   // Wall-clock ceiling
    pub memory_bytes: u64,              // Memory budget
    pub ui_nodes: u32,                  // UI tree nodes
    pub mailbox_len: u16,               // Queue depth
    pub max_effects: u32,               // Effect count ceiling
    pub max_patch_bytes: u32,           // UI patch byte ceiling
}
```

### Lane Defaults (lines 487–502)
- `Interactive`: 4ms, 2M fuel
- `UserVisible`: 16ms, 6M fuel
- `Background`: 50ms, 20M fuel
- `Maintenance`: 200ms, 80M fuel

### TurnResult Output (lines 773–799)
```rust
pub struct TurnResult {
    pub ui_patches: Vec<u8>,            // Opaque UiPatch blobs
    pub effects: Vec<u8>,               // Opaque Effect blobs
    pub next_wake: Option<u64>,         // Deadline for next turn
    pub status: TurnStatus,             // Result status
    pub usage: Usage,                   // { fuel, wall_us, memory_bytes }
}
```

### Usage Tracking (lines 749–767)
```rust
pub struct Usage {
    pub fuel: u64,
    pub wall_us: u64,
    pub memory_bytes: u64,
}
```

### Implications for `InteractiveJob`
- **Direct fit**: `Budget` structure should become `StepContext` fields (fuel, deadline_ms → wall_ms, maybe split to fuel/deadline)
- **Adopt**: `TurnStatus` state machine is close to `StepOutcome` — `Idle`/`MoreWork` map to `Yield`, `CheckpointReady` is direct, `Faulted` maps to `Fault`
- **Reuse**: `Usage` should flow back in `StepOutcome` as well
- **Thread through**: Job's own `step()` must accept a `StepContext` and report back `StepOutcome`
- **Conflict**: `Suspend { checkpoint: bool }` is a payload instruction, not a result — need to clarify: suspend is the *request* to pause the actor, not the job's internal checkpoint mechanism
- **Generalize**: Lane enum (Interactive, UserVisible, Background, Maintenance, Io, Timer) is actor-layer specific; `InteractiveJob::step()` must accept a `StepContext` that carries priority, not assume a lane

---

## 3. Puzzle 3D Resumable Session — Best Existing Template

**File**: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️component.rs`

### Session Structure (lines 62–79, 389–460)
```rust
pub(crate) struct Puzzle3dCollision {
    pub(crate) scene: Option<SceneConfig>,
    scene_json: Option<String>,         // Byte-identical check for no-op resync
    meshes: HashMap<String, CollisionBody>,
    pub(crate) brush_cache: HashMap<String, BrushCollisionFreeResult>,
    pub(crate) brush_queue: VecDeque<String>, // Brush lane cursor
    fill_steps_remaining: usize,        // Fill lane fuel counter
    pub(crate) fill: Option<FillBuilder>,
}
```

### FillBuilder State (lines 24–64)
```rust
pub(crate) struct FillBuilder {
    pub(crate) base: Fixture,
    pub(crate) fixture: Fixture,
    pub(crate) applied_count: usize,    // Prefix-stable checkpoint
    pub(crate) sequence: Vec<BrushPlacePayload>, // Growing plan
    pub(crate) appended_objects: Vec<FixtureObject>,
    pub(crate) appended_attractions: Vec<AttractionProps>,
    pub(crate) placed: Vec<PlacedCollisionEntry>, // Collision cursors
    pub(crate) candidate_cache: HashMap<String, Vec<BrushCompatibleCandidate>>,
    pub(crate) seed_object_ids: std::collections::HashSet<String>,
    pub(crate) rng_state: u32,          // Seeded RNG for determinism
    pub(crate) stalled: bool,
    pub(crate) max_count: usize,
}
```

### Step Budget (line 58)
```rust
const PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS: f64 = 12.0;
```

### Step Functions (lines 389–460)
- `async fn precompute_step_lane(&mut self, lane: PrecomputeLane, budget: u32) -> bool` — Steps one lane (brush or fill), returns true if work remains
- `async fn precompute_step(&mut self, budget: u32) -> bool` — Steps both lanes within budget

The step function:
1. Checks time budget (line 58: 12ms soft ceiling)
2. Attempts work on first lane
3. If time permits, attempts additional work
4. Returns whether more work remains

### Progress Reporting (lines 467–474)
```rust
pub(crate) async fn fill_progress_summary(&self) -> FillProgressSummary {
    self.fill.as_ref().map_or(
        FillProgressSummary { count: 0, applied_count: 0, max_count: FILL_COUNT_MAX, done: true },
        |fill| FillProgressSummary {
            count: fill.sequence.len(),
            applied_count: fill.applied_count,
            max_count: fill.max_count,
            done: fill.stalled || fill.sequence.len() >= fill.max_count,
        }
    )
}
```

### Cursor & Resumption
- **Brush lane cursor**: `brush_queue: VecDeque<String>` — pop front, process, continue next step
- **Fill lane cursor**: `fill.applied_count` — prefix-stable checkpoint; unapplied tail discarded on weight replan (line 118: `soft_replan_fill_tail`)
- **RNG state**: `fill.rng_state: u32` — preserved across steps for determinism

### Implications for `InteractiveJob`
- **Template**: This is EXACTLY what Phase 2 must generalize — two independent lanes, per-lane progress, resumable cursors, prefix-stable progress
- **Adopt**: `applied_count` pattern (committed vs. tentative) should become standard in `StepOutcome::CheckpointReady`
- **Reuse**: 12ms budget constant is a data point for `StepContext`'s default deadline
- **Plan**: Phase 4 will migrate this to `InteractiveJob` directly — the architecture is proven

---

## 4. New WorkerPool — Scheduling Substrate

**File**: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs`

### ProcessKind (lines 413–428)
```rust
pub enum ProcessKind {
    InteractiveNative,  // max(1, cores-1) workers
    HeadlessBatch,      // cores workers
}

pub fn worker_count_for(process_kind: ProcessKind, cores: usize) -> usize {
    // Interactive: 1 worker on single-core; Background leaves one core for UI
    // Batch: all cores
}
```

### Lane Enum (lines 444–495)
```rust
pub enum Lane {
    Interactive,        // weight=8, priority 0
    UserVisible,        // weight=4, priority 1
    Background,         // weight=2, priority 2
    Maintenance,        // weight=1, priority 3
    Io,                 // weight=4, latency-sensitive
    Timer,              // weight=3, between UserVisible and Background
}
```

Deficit-round-robin scheduler: weights are accrued and scanned in fixed order. Work with insufficient deficit is skipped (not starved forever).

### PermitLedger (lines 537–559)
```rust
pub struct PermitLedger {
    remaining: AtomicU32,
    trace_permits: semio_framework_trace::PermitLedger,
}

pub fn checkout(&self, n: u32) -> Result<PermitGuard<'_>, PermitError>
// Checked compare-exchange; over-allocation always Err
```

Sized to `worker_count_for()` permits. Guards returned on drop.

### ChannelPolicy (lines 278–283)
```rust
pub enum ChannelPolicy {
    LatestWins { max_bytes: u64 },
    Coalesced { key: String, max_items: u32, max_bytes: u64 },
    LosslessBounded { max_items: u32, max_bytes: u64 },
    ByteCredit { max_items: u32, max_bytes: u64 },
}
```
**Phase 1a requirement**: Every variant bounds both items and bytes.

### OperationContext (lines 70–92)
```rust
pub struct OperationContext {
    pub actor: u64,
    pub generation: u16,
    pub trace: TraceId,
    pub lane: u8,                  // Mirrored from actor::Lane
    pub deadline_ms: Option<u64>,
    pub cancel: CancelToken,
    pub capability: Option<CapabilityTokenId>,
}
```

### CancelToken (lines 137–192)
Tri-state (Live, Park, Cancelled) with optional parent link. State is max-severity fold of ancestors.

### Implications for `InteractiveJob`
- **Adopt**: `Lane` enum directly; or accept `lane: u8` to avoid crate coupling
- **Thread**: `deadline_ms` and `cancel: CancelToken` become core fields in `StepContext`
- **Integrate**: `PermitLedger` for admission control; jobs submit to `WorkerPool::submit(lane, work)` and receive a permit or reject
- **Backpressure**: `ChannelPolicy` drives preview/checkpoint/result channel capacity — each must declare both item and byte bounds
- **Scope discipline**: Jobs must belong to a `ScopeHandle` (lines 243–253); cancelling the scope cancels every descendant job

---

## 5. Tracing Crate — Instrumentation Hooks

**File**: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏱️trace/🦀️component.rs`

### Watchdog RAII Guard (lines 334–371)
```rust
pub struct Watchdog {
    site: &'static str,
    operation: OperationId,
    generation: Generation,
    stage: InteractiveStage,
    start_us: u64,
}

pub fn start(site: &'static str, operation: OperationId, generation: Generation, stage: InteractiveStage) -> Watchdog
// RAII: on drop, records elapsed and checks contract
```

Records elapsed time; on drop, checks against contract ceiling for that `(site, stage)`.

### StepTimer (lines 270–307)
Callback-based timer: `on_exceeded` fires when budget is exceeded.

### Percentile Ring (lines 216–237)
```rust
fn percentile(&self, p: f32) -> u32  // 50th (p50), 95th (p95), 99th (p99)
```

Tracks operation latencies in a ring buffer with percentile queries — no external allocation.

### Operation/Generation Tracing (lines 628–656)
```rust
pub fn record_operation_started(operation: OperationId, generation: Generation) -> TraceEvent
pub fn record_stage_changed(operation: OperationId, generation: Generation, label: &'static str) -> TraceEvent
pub fn record_preview_published(operation: OperationId, generation: Generation) -> TraceEvent
pub fn record_checkpoint(operation: OperationId, generation: Generation) -> TraceEvent
pub fn record_committed(operation: OperationId, generation: Generation) -> TraceEvent
pub fn record_cancelled(operation: OperationId, generation: Generation) -> TraceEvent
```

Thread-role API: `set_thread_role()` marks OS thread identity.

### Trace Ring & Queries
- `operation`/`generation` pair coordinates preview-latency and cancellation-latency queries
- Trace events are logged to a ring (no unbounded allocation)

### Implications for `InteractiveJob`
- **Interoperate**: `StepContext` must carry `operation: OperationId` and `generation: Generation` to wire into trace recording
- **Don't duplicate**: `StepContext` should call `record_stage_changed()` at macrostep entry/exit rather than own a separate timer
- **Integrate**: Preview publishing and checkpoint recording happen via trace calls, not a separate channel
- **No new API**: The trace crate's existing `record_*` functions should be called from within the step

---

## 6. Existing Progress/Preview Reporting Vocabulary

### Puzzle 3D Progress (lines 467–474 in precompute/component.rs)
```rust
pub struct FillProgressSummary {
    count: usize,           // Planned so far
    applied_count: usize,   // Committed/materialized
    max_count: usize,
    done: bool,
}
```

### Puzzle 3D Progress/Outcome (schema)
**File**: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`

Schema defines: `FillBuildProgress`, `FillProgressSummary`, `BrushCollisionFreeResult`, `BrushCompatibleCandidate` — all passed through command dispatching (line 727: `async fn dispatch(...)`).

### Template Vocabulary
The app's precompute session already streams:
- **Started** — on `set_scene`
- **StageChanged** — when switching between lanes (brush → fill)
- **CandidateTested** — each brush/fill candidate outcome
- **PreviewPatch** — UI updates (brush preview, fill plan visualization)
- **Diagnostic** — warnings, stalled reasons
- **Checkpoint** — `applied_count` snapshot
- **CommitCandidate** — candidate ready to materialize
- **Completed** — fill plan done
- **Cancelled** — user abort
- **Failed** — error (collision, out-of-memory)

### Implications for `InteractiveJob`
- **Reuse**: This vocabulary is proven; `StepOutcome` should emit the same event stream
- **Channel**: Progress events flow through a bounded channel (likely `ChannelPolicy::Coalesced` with preview-only dedup)
- **Trace integration**: Each event type should wire into `record_stage_changed()`, `record_preview_published()`, etc.

---

## 7. Determinism Machinery

### InferredField & DepHash Caching
**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️component.rs`

```rust
pub struct DepHash(pub [u8; 32]);

impl DepHash {
    pub async fn chain(field_id: &str, schema_version: u32, input: &[u8], parents: &[DepHash]) -> Self
    // Chains parent hashes to compute stable, reproducible hash
}

pub trait InferredField<P>: Send + Sync + 'static {
    type Value;
    fn plan(&self, cache: &InferenceCache<Self::Value>) -> Option<Self::Value>;
    fn recompute(&self, input: &P) -> Self::Value;
}
```

Used in Puzzle artifacts (e.g., `puzzle3d::schema::inferences`) to cache flat-position and topology inference results. The `plan` method checks `cache` first; only cache misses call `recompute`.

### Seeded RNG Handling
**File**: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs`

```rust
pub(crate) struct FillBuilder {
    pub(crate) rng_state: u32,          // Seeded from scene.seed
}

// In new():
let mut placed = Vec::new();
for obj in &base.objects {
    if let Some(mesh_url) = resolve_object_kind_mesh_url(obj.object_kind.as_deref().unwrap_or(""), catalogs, &base) {
        if meshes.contains_key(&mesh_url) {
            placed.push(PlacedCollisionEntry { object_id: obj.id.clone(), mesh_url, world: pose_isometry(...) });
        }
    }
}
```

The RNG state is preserved across steps in `FillBuilder.rng_state`.

### Determinism Law Tests
**Files**: Various puzzle artifact test suites (e.g., `puzzle3d/schema/inferences/component.rs:118`)

```rust
async fn inference_determinism_law() {
    // Run inference twice with identical inputs; assert outputs are byte-identical
    let result1 = infer_field(...);
    let result2 = infer_field(...);
    assert_eq!(result1, result2);
}
```

### Stable Orderings
Puzzle uses `BTreeMap` and `BTreeSet` for all collections in schemas (not `HashMap`/`HashSet`), ensuring iteration order is stable regardless of insertion order.

### Implications for `InteractiveJob`
- **Snapshot determinism**: Identical snapshot, inputs, and seed must produce identical results regardless of worker count
- **RNG ownership**: Each job owns its RNG state; seeding must happen at job creation, not per-step
- **Caching discipline**: `InferredField` pattern should be available to jobs; `StepContext` or the job itself should hold `InferenceCache` references
- **Ordered collections**: Jobs must use stable-order types (BTreeMap, BTreeSet, or sorted Vec) for any state persisted in checkpoints
- **Test harness**: Phase 2 should provide a golden-test harness (determinism law test) that runs the same job snapshot twice and asserts byte-identical results

---

## Consolidated Design Decisions for Phase 2

### Decision 1: Step Function Signature and Bounds

**Proposal:**
```rust
pub trait InteractiveJob: Send {
    type State: Serialize + Deserialize;
    
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome;
}

pub struct StepContext<'_> {
    pub operation: OperationId,
    pub generation: Generation,
    pub deadline_ms: u64,           // Absolute wall-clock deadline (not remaining!)
    pub fuel: &mut u64,             // Remaining, decremented by job
    pub cancel: CancelToken,
    pub now_ms: fn() -> u64,        // Caller-supplied clock fn
    pub traces: &mut TraceRecorder, // or just call trace module directly?
    pub inference_cache: Option<&mut InferenceCache>,
}

pub enum StepOutcome {
    Yield,                              // More work remains; call again
    PreviewReady(Vec<u8>),              // Preview available (pack-encoded)
    CheckpointReady(Checkpoint),        // Committed progress snapshot
    Complete(CommitCandidate),          // All work done
    Cancelled,                          // CancelToken transitioned to Cancelled
    Fault { detail: Vec<u8> },          // Error (pack-encoded)
}

pub struct Checkpoint {
    pub state: Vec<u8>,                 // Persisted job state
    pub applied_progress: u64,          // Progress metric (e.g., FillBuilder.applied_count)
}

pub struct CommitCandidate {
    pub state: Vec<u8>,                 // Final state
    pub output: Vec<u8>,                // Job's output
}
```

**Reason:** Reuses Puzzle 3D's proven pattern (applied_count, two lanes) and threads `StepContext` uniformly. Deadline is absolute so fuel logic doesn't need to know wall-clock math. `CancelToken` is checked by the job; job yields on cancel. Traces are recorded by the job or via macro wrapper. 

**Conflicts to resolve:** Should `StepContext` own `TraceRecorder`, or does the job call `record_stage_changed()` directly? The latter avoids a new type but couples the job to the trace module.

---

### Decision 2: Checkpoint and State Persistence

**Proposal:**
- `Checkpoint { state: Vec<u8>, applied_progress: u64 }` represents a pause point where work is resumable but not committed
- `CommitCandidate { state: Vec<u8>, output: Vec<u8> }` represents completion
- State is opaque (pack-encoded by the job); the runtime only routes it
- `applied_progress` is a typed value (u64 for now) to track partial commitment (e.g., fill objects actually placed vs. planned)

**Reason:** Puzzle 3D's `applied_count` pattern is powerful — it lets the job advertise "these 100 objects are committed; the next 900 are tentative." On weight replan (line 118), the tail is discarded and re-queued. This must generalize.

**Implications:** The actor layer's `Payload::Checkpoint` and `TurnStatus::CheckpointReady` need expansion — `TurnStatus::CheckpointReady` should carry the `applied_progress` metric, not just a bool.

---

### Decision 3: Fuel Accounting and Wall-Clock Bounding

**Proposal:**
- `StepContext` carries absolute `deadline_ms` (wall-clock time the step must finish by)
- Job checks both `fuel` and elapsed time; yields when either is exceeded
- Budget defaults per lane stay as in actor layer (4ms Interactive, 16ms UserVisible, 50ms Background, 200ms Maintenance)
- Puzzle 3D's 12ms constant becomes a data point; new jobs use the lane default

**Reason:** Phase 1 WorkerPool uses deficit-round-robin on lanes with weights; the deadline represents the lane's scheduler guarantee, not a global clock. Two-bound (fuel + wall) ensures progress against both memory and latency.

---

### Decision 4: Preview Channel and Effect Serialization

**Proposal:**
- Preview is a pack-encoded `Vec<u8>` (job-specific schema)
- Previews are routed to UI via `record_preview_published(operation, generation)` in the trace module
- Each preview is bounded by `StepContext` budget (fuel, wall, patch bytes)
- Job can yield with `Yield` multiple times; on `PreviewReady`, a preview is available (and the job may have stalled, not finished)

**Reason:** Puzzle 3D already streams previews (brush collision-free result, fill plan visualization). The trace module's `operation`/`generation` pair provides the correlation id. No new channel needed; trace records are the preview delivery mechanism.

---

### Decision 5: Determinism and RNG Seeding

**Proposal:**
- Job state must include its RNG seed and current state (e.g., `fill.rng_state`)
- Jobs must use only stable-order collections (BTreeMap, BTreeSet) for any state
- Golden-test harness: run identical snapshot twice with same seed, assert byte-identical results
- `InferredField` caching is optional; jobs that don't use it pass `None` for inference_cache

**Reason:** Puzzle 3D already does this; seeded RNG in state, BTreeMap for collections. Multi-worker determinism requires this discipline. Tests will catch violations.

---

### Decision 6: Job Ownership and Cancellation

**Proposal:**
- Each job is owned by a [`ScopeHandle`] (from async crate)
- `StepContext.cancel` is a child token of the scope's token
- Cancelling the scope cascades to all child jobs via parent-chain fold
- Job checks `cancel.is_cancelled()` on entry; if true, returns `Cancelled` without doing work

**Reason:** Async crate's scope discipline is proven (lines 243–253 of async/component.rs). Jobs don't need to register themselves; the parent cancellation fold (lines 170–179) handles it. No child registry needed.

---

### Decision 7: Tracing and Watchdog Integration

**Proposal:**
- On step entry: caller invokes `record_operation_started(operation, generation)` (or job does via macro)
- On stage change (e.g., brush → fill lanes): job calls `record_stage_changed(operation, generation, label)`
- On preview publish: job calls `record_preview_published(operation, generation)`
- On checkpoint: job calls `record_checkpoint(operation, generation)`
- On complete: job calls `record_committed(operation, generation)`
- On cancel: job calls `record_cancelled(operation, generation)`
- Watchdog timing is checked by caller post-step (e.g., in actor turn logic)

**Reason:** Trace module already has these functions; no duplication. The job can call them directly (one module dependency) or via a callback in `StepContext`. The latter avoids coupling but requires `StepContext` to grow a trace field. Decision: job calls directly (simpler).

---

## Summary: Concrete Phase 2 Decisions & Recommendations

| Decision | Recommendation | Reason |
|----------|---|---|
| **Step signature** | `fn step(&mut self, cx: &mut StepContext) -> StepOutcome` | Proven by Puzzle 3D; bounded microsteps; thread context uniformly |
| **State persistence** | Opaque pack-encoded `Vec<u8>` + typed `applied_progress: u64` | Reuses `FillBuilder.applied_count` pattern; enables partial commit |
| **Fuel & deadline** | Absolute `deadline_ms`; job checks both fuel and wall time | Two-bound ensures progress on memory and latency; deadline is lane guarantee |
| **Preview channel** | Via trace module's `record_preview_published()` | No new channel; correlation via operation/generation |
| **Determinism** | Seeded RNG in state; BTreeMap/BTreeSet only; golden-test harness | Puzzle 3D pattern; tests catch violations |
| **Cancellation** | Via parent-chain fold on `CancelToken` from `ScopeHandle` | Async crate's scope discipline; no registry needed |
| **Tracing** | Job calls trace functions directly (minimal coupling) | Trace crate already has operation/generation functions |
| **Error handling** | Pack-encoded `Vec<u8>` in `Fault { detail }` | Opaque to runtime; job-specific error schema |

**Most important:** The step function must be synchronous, bounded (fuel or wall), and explicitly resumable. Puzzle 3D's precompute session proves this architecture works at scale (fill planning, brush caching, two lanes). Phase 2 generalizes it.

