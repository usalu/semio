# Interactive-Job Migration Recipe: `BatchOnlyPendingRewrite` → `Migrated`

## 0. Scope of this research

Only **one** plugin (`lowpoly`) has an actual `🧪️interactive-job/🔣️component.json` route table
(`find … -path "*interactive-job*"` returned exactly one hit:
`✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️component.json`). No other plugin currently has this
route file, so `lowpoly` is not just "the best exemplar" — it is the **only** exemplar in the repo.
This is itself worth flagging: the recipe below generalizes lowpoly's own pattern to any future
plugin, since there is nothing else to compare it against.

Inside lowpoly, the route table lists 47 tools: 19 `Migrated`, 28 `BatchOnlyPendingRewrite` — matching
the TS oracle test's own asserted counts. Every `Migrated` route's `preparation`/`lanes` pair is one
of a fixed signature set (see §7). The best "real compute, multi-step" exemplar among the 19 migrated
tools is **`paintStrokeEnd`** — it diffs a paint-layer pixel buffer in bounded chunks across multiple
`step()` calls with a resumable cursor and a replay/digest mechanism — not a config toggle. It's used
as the primary exemplar throughout. `patchObject`/`addPaintLayer` (single-step `Artifact`-lane
commands) are used as the simpler contrast case.

Exemplar file (all line numbers below refer to this file):
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
(2144 lines).

Framework files:
- `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs` — `ToolExecutionContract`, `ToolJobFactory`, `RetainedToolWireInput`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️component.rs` — `ArtifactCommandWorkStep`, `ArtifactCommandWork`, `ArtifactRetainedCommandJob` (the state machine that drives every migrated command).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — `AppOperationContext`, `ArtifactToolCompletion`, `ArtifactToolPublicationLane`, `ArtifactToolFactoryRegistry::register`, `bounded_first_step_tool_proofs!`.
- `🧰️framework/🔨️modules/🧵️job/🦀️component.rs` — `StepOutcome`, `StepContext` (cancellation/yield/fuel primitives).
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:733-740` — `InteractiveJobClassification` enum.

---

## 1. `ToolExecutionContract` (action-bus `🦀️.rs:216-299`)

```rust
pub struct ToolExecutionContract {
    pub max_raw_wire_bytes: usize,       // cap on the raw encoded command wire payload
    pub max_decoded_items: usize,        // cap on decoded/semantic work items (== maximum_work_items)
    pub max_work_units_per_step: u64,    // work units one step() may advance (retained-command jobs always pass 1)
    pub max_output_bytes: usize,         // cap on the produced result/output payload
    pub max_step_micros: u32,            // must be < INTERACTIVE_MAX_STEP_MICROS (8_000); this is maximumPollMicros
    pub checkpoint_every_steps: u32,     // cadence of StepOutcome::CheckpointReady
    pub progress_every_steps: u32,       // cadence of progress preview emission
    pub cancellation: ToolCancellationPolicy,   // always PerOperation
    pub freshness: ToolFreshnessPolicy,         // always ValidateImmediatelyBeforeExposure
    pub shape: ToolExecutionShape,              // Resumable | BoundedFirstStep
}
```

Two constructors:

- `ToolExecutionContract::resumable(max_raw_wire_bytes, max_decoded_items, max_work_units_per_step, max_output_bytes, max_step_micros, checkpoint_every_steps, progress_every_steps) -> Self` (`🦀️.rs:248`) — `shape = Resumable`. **This is what every migrated lowpoly command uses**, e.g. `lowpoly_contract()` (component.rs:426-428):
  ```rust
  ToolExecutionContract::resumable(LOWPOLY_RETAINED_RAW_BYTES /*16_384*/, LOWPOLY_RETAINED_WORK_ITEMS /*258*/, 1, 32*1024*1024, 7_500, 1, 1)
  ```
- `ToolExecutionContract::bounded_first_step(max_raw_wire_bytes, max_decoded_items, max_work_units, max_output_bytes, max_step_micros) -> Self` (`🦀️.rs:263`) — `shape = BoundedFirstStep`, `checkpoint_every_steps = progress_every_steps = 1` fixed. For single-shot ops that never need to resume mid-work.

`validate()` (`🦀️.rs:278-298`) rejects any field of 0, and rejects `max_step_micros == 0 || max_step_micros >= 8_000`.

---

## 2. `ArtifactCommandWorkStep` — ALL variants (`retained-command/🦀️component.rs:80-85`)

```rust
pub enum ArtifactCommandWorkStep<A: ArtifactApp> {
    Replay { stage: &'static str, preview: &'static [u8] },
    Progress { stage: &'static str, preview: &'static [u8] },
    Complete(Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>),
    CompleteWithEphemeral { emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, ephemeral: EphemeralEmit<A> },
}
```

- **`Replay`** — job is re-hydrating from a checkpoint after a resume; no new work is done this step, just re-walks state to the point it already reached. `stage`/`preview` (bilingual `{"en":…,"de":…}` JSON bytes) surface as progress text. Lowpoly example: `paint_end_step` returns `Replay { stage: "lowpoly-paint-replay", … }` while `paint_cursor < target` during restore (component.rs:625, 643).
- **`Progress`** — one bounded unit of real work was done, more remains; increments `work_progress` in the driving job (`retained-command/🦀️component.rs:486`) and triggers a checkpoint (`checkpoint_pending = true`). Lowpoly example: `paint_end_step` returns `Progress { stage: "lowpoly-paint-diff", … }` while `paint_cursor < before.len()` (component.rs:646).
- **`Complete(emit)`** — work is finished, no ephemeral (presence/transient) side output. Used by every single-step reducer arm in `lowpoly_retained_reduce` (component.rs:517-553), e.g. `patchObject`.
- **`CompleteWithEphemeral { emit, ephemeral }`** — work is finished AND also emits ephemeral presence/transient mutations (drag-state snapshots etc). Used by `paintStrokeBegin`, `transformBegin`, `setActiveUtility`, and the terminal step of `paint_end_step` (component.rs:534-549, 660).

The driving state machine (`ArtifactRetainedCommandJob::step`, phase `Work`, `retained-command/🦀️component.rs:471-504`) matches on these: `Replay`/`Progress` stay in `Work` phase and re-enter `step()` on the next poll; `Complete`/`CompleteWithEphemeral` transition to `Publish` phase.

`ArtifactCommandWork<A>` trait (the type a plugin implements) — required/overridable methods:
```rust
trait ArtifactCommandWork<A: ArtifactApp>: Send {
    fn tool_id(&self) -> &'static str;
    fn workspace_identity(&self) -> u64 { 0 }                    // stable identity of the retained mutable workspace
    fn extent(&self, command, snapshot, interaction, context) -> Option<usize>;  // semantic work-item count; None/oversized => rejected
    fn step(&mut self, command, snapshot, config, history, interaction, hover, context, operation) -> Result<ArtifactCommandWorkStep<A>, Fault>;
    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> { Ok(0) }
    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> { … }
    fn begin_close(&mut self) {}
    fn close_step(&mut self, maximum_items, maximum_bytes) -> InteractiveJobCloseStep { Complete }
    fn terminal_is_empty(&self) -> bool { true }
}
```
Two implementations exist:
- `BoundedArtifactCommandWork<A>` (retained-command/🦀️component.rs:124-163) — a thin one-shot wrapper around a plain reducer fn; always yields `Complete` and can only be stepped once (`consumed` flag).
- `LowpolyRetainedCommandWork` (component.rs:560-827) — the hand-rolled multi-step implementation with a real checkpoint format (`LPC2`, 88 bytes) and the bounded paint cursor described next.

---

## 3. The bounded operation-owned cursor (concrete type + API)

There is **no generic "Cursor" framework type** — "a bounded operation-owned cursor" is house
terminology for: a `usize`/`u64` field owned by the command's own `ArtifactCommandWork` impl,
advanced by fixed-size chunks per `step()`, capped by a compile-time constant, and round-tripped
through `checkpoint()`/`restore()` so a killed/resumed job picks up exactly where it left off. Two
concrete forms in the exemplar:

**a) The wire/checkpoint page cursor** — framework-owned, in `ArtifactRetainedCommandJob`
(retained-command/🦀️component.rs:243-270):
```rust
raw_page_cursor: usize,           // index into RetainedToolWireInput pages already consumed
checkpoint_page_cursor: usize,    // same, for the checkpoint's own wire input
work_progress: u64,               // monotonic progress counter bumped by ArtifactCommandWorkStep::Progress
```
`RetainedToolWireInput` (action-bus `🦀️.rs:109-206`) is the actual bounded page owner: fixed-capacity
`Vec<ToolWirePage>` (`try_reserve_exact`d up front to `declared_bytes / 4096` pages), each page a fixed
`[u8; 4_096]` buffer (`TOOL_WIRE_PAGE_BYTES`). `admit_page` rejects once `admitted_bytes` would exceed
`declared_bytes` or `maximum_bytes`, or once `pages.len() == pages.capacity()`.

**b) The app-owned paint cursor** — `LowpolyRetainedCommandWork.paint_cursor: usize`
(component.rs:569). `paint_end_step` (component.rs:610-661) advances it by
`LOWPOLY_RETAINED_PAINT_CHUNK_BYTES` (16_384) bytes per `step()`:
```rust
let end = self.paint_cursor.saturating_add(LOWPOLY_RETAINED_PAINT_CHUNK_BYTES).min(before.len());
for index in self.paint_cursor..end { /* bounded per-byte diff, digest accumulation */ }
self.paint_cursor = end;
```
It is "operation-owned" because `workspace_identity()` (component.rs:670) binds it to
`operation_id.rotate_left(17) ^ generation.rotate_left(31) ^ context_identity.rotate_left(43)` — a
checkpoint from a *different* operation/generation/context cannot be replayed into this cursor
(`restore` checks these in `retained-command/🦀️component.rs:762` and errors
`lowpoly-retained-checkpoint-identity-mismatch` on drift).

`checkpoint()`/`restore()` (component.rs:729-775) hand-encode an 88-byte `LPC2` binary format
(magic, disposition byte, complete flag, stage, tool identity digest, operation id, generation, base
revision, context identity, `paint_cursor` as u64, `paint_digest` as u64) — this is the exact wire
form a "resumable" migrated command must produce. `paint_replay_target: Option<(usize, u64)>`
(cursor position + expected digest) is how `Replay` steps re-walk to the restored cursor and verify
they land on the same digest before resuming real work (component.rs:619-627).

---

## 4. "Exact Store publication authority"

This means: before a mutation is allowed to reach the document/config store, an app-owned
"preparation factory" computes its **exact retained byte footprint** and rejects it outright if it
(or the resulting post-state) exceeds a fixed cap — no partial/approximate admission.

Concrete API, `store::ArtifactStoreOneItemFootprint { work_items: usize, retained_bytes: usize }`,
produced by an `admit_*` fn and consumed by `prepare_*`:

```rust
// component.rs:943-949 — exact per-mutation admission
fn admit_lowpoly_artifact_mutation(mutation: &LowpolyMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = lowpoly_artifact_mutation_retained_bytes(mutation)?;  // exact byte accounting per mutation variant
    if retained_bytes > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES { return Err(…); }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

// component.rs:951-963 — base AND post-mutation snapshot both re-checked
fn prepare_lowpoly_artifact(base: &LowpolySnapshot, mutation: LowpolyMutation) -> Result<(LowpolySnapshot, Vec<LowpolyMutation>, LowpolyMutation), String> {
    admit_lowpoly_artifact_mutation(&mutation)?;
    if !lowpoly_snapshot_admitted(base) || lowpoly_snapshot_retained_bytes(base) > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES { return Err(…); }
    let inverse = mutation.inverse(base);
    let diff = mutation.diff(base).into_parts().0;
    let post = diff.apply(base).map_err(…)?;
    if !lowpoly_snapshot_admitted(&post) || lowpoly_snapshot_retained_bytes(&post) > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES { return Err(…); }
    Ok((post, inverse, mutation))
}
```
`lowpoly_artifact_mutation_retained_bytes` (component.rs:926-940) is a match over every `LowpolyMutation`
variant with a hand-written byte count per field — anything not explicitly enumerated is rejected
(`_ => Err(…)`), i.e. fail-closed, not fail-open. The equivalent pair exists for config:
`admit_lowpoly_config_mutation`/`prepare_lowpoly_config` (component.rs:992-1010) against
`LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES` (16_384).

These two factories are wired into `ArtifactApp` via:
```rust
fn build_artifact_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Snapshot, Mutation>>> { Some(Arc::new(LowpolyArtifactStorePreparationFactory)) }
fn build_config_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Config, ConfigMutation>>> { Some(Arc::new(LowpolyConfigStorePreparationFactory)) }
```
(component.rs:1394-1400). This is exactly what the JSON route's `blocker` string names: *"Reducer
lacks a bounded operation-owned cursor or exact Store publication authority"* — i.e. a batch-only
command either has no bounded `step()`/checkpoint cursor (§3), or has no `admit_*`/`prepare_*` pair
enforcing an exact byte cap on what it writes to the store (this section), or both.

`ArtifactToolCompletion<A>::complete(emit, ephemeral)` (component.rs:13087-13094) is the final
publication handoff out of the retained job into the app's store — it can only be called once
(`duplicate-output` fault on a second call) and only when `has_mounted_consumer()` is true (an Arc
strong-count check that a live consumer is actually attached) — see `Publish` phase,
retained-command/🦀️component.rs:505-517.

---

## 5. Progress & cancellation surfacing

`StepOutcome` (job `🦀️component.rs:1300-1307`):
```rust
pub enum StepOutcome { Yield, PreviewReady(RetainedJobPayload), CheckpointReady(Checkpoint), Complete(CommitCandidate), Cancelled, Fault(JobFault) }
```
Every `ArtifactRetainedCommandJob::step()` call (retained-command/🦀️component.rs:382-389) starts with:
```rust
if cx.is_cancelled() { return StepOutcome::Cancelled; }
if cx.should_yield() || cx.fuel_remaining() == 0 { return StepOutcome::Yield; }
cx.consume_fuel(1);
```
So **cancellation** is a per-step poll of `StepContext::is_cancelled()` — every phase checks it before
doing any work, and it always wins over everything else including a pending checkpoint. **Progress**
is surfaced by returning `StepOutcome::PreviewReady` with a small bilingual JSON payload
(`{"en":"…","de":"…"}`) set via `cx.set_stage(stage)` + `self.preview(cx, preview)`
(retained-command/🦀️component.rs:479-489) — every `Replay`/`Progress` work-step, and every phase
transition (decode, preflight, publish), emits one of these. `progress_every_steps` /
`checkpoint_every_steps` on the contract control how often a full `CheckpointReady` (vs just a
preview) is required. Yielding (`StepOutcome::Yield`) is the interactive-job scheduler's own
cooperative timeslice bound — driven by `max_step_micros` (≤ 8_000, `INTERACTIVE_MAX_STEP_MICROS`)
from the contract.

---

## 6. `AppOperationContext` (component.rs — framework, `🦀️component.rs:8003-8037`)

```rust
pub struct AppOperationContext {
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub operation_id: u64,
    pub generation: u64,
    pub canonical_base_revision: [u8; 32],
}
```
Built via `AppOperationContext::from_operation(app_instance_id, parent_document_id, operation, canonical_base_revision)`.
`canonical_base_revision_hex()` gives a fixed-width lowercase hex encoding for persisted fields.
This is the "operation-owned" identity that `LowpolyRetainedCommandWork::step` revalidates on
**every** step (component.rs:702-707) — if `operation_id`/`generation`/`canonical_base_revision`
drift from what the work was constructed with, it hard-fails
(`lowpoly-retained-operation-freshness-drift`) rather than silently continuing against a stale base.
There is a sibling `AppRenderOperationContext` (no invocation id) for renderer-observed background
jobs, not used by command reducers.

---

## 7. `*_admitted` gating functions

Two-tier admission, both must pass or the command never reaches a retained job at all:

```rust
// component.rs:430-440 — is the CURRENT document snapshot itself within bounds?
fn lowpoly_snapshot_admitted(snapshot: &LowpolySnapshot) -> bool {
    snapshot.schema.len() <= LOWPOLY_RETAINED_FIELD_BYTES
        && snapshot.objects.len() <= LOWPOLY_RETAINED_OBJECTS
        && snapshot.objects.iter().all(|object| /* every string field <= LOWPOLY_RETAINED_FIELD_BYTES, paint layers <= LOWPOLY_RETAINED_PAINT_LAYERS_PER_OBJECT, each layer.pixels <= LOWPOLY_RETAINED_PAINT_LAYER_BYTES */)
}

// component.rs:442-467 — is THIS command's payload within bounds, given the snapshot/config are admitted?
fn lowpoly_command_admitted(command: &LowpolyCommand, snapshot: &LowpolySnapshot, config: &LowpolyConfig) -> bool {
    lowpoly_snapshot_admitted(snapshot)
        && lowpoly_config_retained_bytes(config) <= LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES
        && match command {
            LowpolyCommand::PatchObject(payload) => field(&payload.object_id) && field(&payload.field) && …,
            /* one arm per migrated command's payload strings, checked against LOWPOLY_RETAINED_FIELD_BYTES / LOWPOLY_RETAINED_RAW_BYTES */
            _ => false,   // fail-closed: unlisted commands are NOT admitted
        }
}
```
Invariants enforced: every string field ≤ `LOWPOLY_RETAINED_FIELD_BYTES` (4_096), object count ≤
`LOWPOLY_RETAINED_OBJECTS` (64), paint layers/object ≤ `LOWPOLY_RETAINED_PAINT_LAYERS_PER_OBJECT` (8),
each layer's pixel buffer ≤ `LOWPOLY_RETAINED_PAINT_LAYER_BYTES` (4 MiB), raw JSON payloads (import/
fixture) ≤ `LOWPOLY_RETAINED_RAW_BYTES` (16_384), config retained bytes ≤
`LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES` (16_384).

**Two call sites, both must reject on failure:**
1. `build_tool_job` (component.rs:1437-1446) — admission-time gate, when the UI dispatches the
   command: `if !lowpoly_command_admitted(…) { return Err(Fault::from("lowpoly-retained-command-capacity")); }`
2. `LowpolyRetainedCommandWork::step` (component.rs:708-710) — re-checked on **every** step (not just
   the first), so a snapshot that grew stale/oversized mid-flight is caught too:
   `if !lowpoly_command_admitted(command, snapshot, config) { return Err(Fault::from("lowpoly-retained-command-capacity")); }`

If not admitted: the job faults (`ArtifactCommandWorkStep` is never produced; `Result::Err` propagates
to `ArtifactRetainedCommandJob`'s `Work` phase match arm `Err(_) => self.fault(cx, …)`,
retained-command/🦀️component.rs:502), which transitions the job to `Fault` phase and returns
`StepOutcome::Fault(JobFault { detail })` — a terminal, non-resumable outcome.

`build_tool_job` also does the **classification-based routing gate**:
```rust
let Some(disposition) = lowpoly_command_disposition(&request.tool_id) else { return Ok(None); };
```
`None` disposition (i.e. tool not in `LOWPOLY_MIGRATED_TOOL_IDS`, or in `LOWPOLY_BATCH_ONLY_TOOL_IDS`)
means "not mine to build a retained job for" — the caller falls through to the batch-only /
non-interactive path instead. This is the actual enforcement point of
`InteractiveJobClassification::Migrated` vs `BatchOnlyPendingRewrite` at runtime, on top of the
compile-time registration check in `ArtifactToolFactoryRegistry::register`
(`component.rs:12636-12638`: `if factory.classification() != Migrated { return Err(…) }`) and the
publication-contract shape check (`component.rs:12629-12635`: every registered tool must have a
non-empty `lanes` list, and `HostOnly` must be the *only* lane if present).

---

## 8. `LowpolyCommandDisposition` — how to choose

```rust
#[repr(u8)]
enum LowpolyCommandDisposition { Artifact = 1, Config = 2, HostOnly = 3, Transient = 4, ConfigTransient = 5, ArtifactTransient = 6 }

fn lowpoly_command_disposition(tool_id: &str) -> Option<LowpolyCommandDisposition> {
    Some(match tool_id {
        "patchObject" | "addPaintLayer" => LowpolyCommandDisposition::Artifact,
        "paintStrokeEnd" => LowpolyCommandDisposition::ArtifactTransient,
        "importSnapshotJson" | "setFixtureJson" => LowpolyCommandDisposition::HostOnly,
        "paintStrokeBegin" | "transformBegin" => LowpolyCommandDisposition::Transient,
        "setActiveUtility" => LowpolyCommandDisposition::ConfigTransient,
        tool_id if LOWPOLY_MIGRATED_TOOL_IDS.contains(&tool_id) => LowpolyCommandDisposition::Config,
        _ => return None,
    })
}
```
Choice rule, read off the exemplar's own assignments:
- **`Artifact`** — the command mutates the persisted document snapshot only (undo-history, versioned). `patchObject`, `addPaintLayer`.
- **`Config`** — the command mutates only the per-session UI/view config (not undo-tracked document state); this is the **default** for any migrated tool not otherwise special-cased (the catch-all arm).
- **`HostOnly`** — the command's effect is entirely host-side (e.g. loading a whole new snapshot/fixture from JSON) — not a normal document mutation lane at all. `PUBLICATION_CONTRACTS` enforces `HostOnly` can never coexist with any other lane (component.rs:12632).
- **`Transient`** — the command only emits ephemeral (ungoverned, non-undo, non-store) transient/presence state — e.g. beginning a drag gesture. `paintStrokeBegin`, `transformBegin`.
- **`ConfigTransient`** — emits BOTH a Config mutation AND a Transient/ephemeral mutation in the same step. `setActiveUtility` (sets active-utility config + resets gesture transient).
- **`ArtifactTransient`** — emits BOTH an Artifact mutation AND a Transient mutation. `paintStrokeEnd` (finishes a paint stroke: commits pixel-run mutations to the Artifact AND clears the drag transient).

The disposition is threaded into `LowpolyRetainedCommandWork::workspace_identity()`
(`u64::from(self.disposition as u8) << 56`) so a checkpoint from one disposition can never be
misapplied under a different one, and it's checked byte-for-byte on `restore` (`checkpoint[4] != self.disposition as u8`).

---

## 9. `PUBLICATION_CONTRACTS` / `ArtifactToolPublicationLane`

Framework enum (`component.rs:12484-12492`):
```rust
pub enum ArtifactToolPublicationLane { HostOnly, Artifact, Config, Draft, Presence, Transient, Child }
```
Note: this framework enum has MORE variants (`Draft`, `Presence`, `Child`) than lowpoly's app-level
`LowpolyCommandDisposition` uses — lowpoly just doesn't have Draft/Presence/Child-lane commands yet.
Each migrated tool declares its exact lane set in `ArtifactOwnedToolJobFactory::PUBLICATION_CONTRACTS`
(component.rs:884-904), e.g.:
```rust
ArtifactToolPublicationContract { tool_id: "paintStrokeEnd", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Transient] },
```
`ArtifactToolFactoryRegistry::register` (component.rs:12629-12635) validates: every `TOOL_IDS` entry
has exactly one contract, no contract has an empty `lanes`, and `HostOnly` is never combined with
another lane. This must line up 1:1 with the JSON route's `lanes` array (checked by the TS test, §12).

---

## 10. Ordered migration checklist — moving tool `X` from BATCH_ONLY to MIGRATED

Given a command `X` currently only in `LOWPOLY_BATCH_ONLY_TOOL_IDS` (or the plugin-specific
equivalent), to migrate it:

1. **Design the bounded work shape first.** Decide: is `X` single-step (→ implement as one arm in
   the `_retained_reduce` match, returning `ArtifactCommandWorkStep::Complete`/`CompleteWithEphemeral`,
   like `patchObject`) or genuinely multi-step (→ needs its own `ArtifactCommandWork` impl with a
   bounded cursor field + `checkpoint()`/`restore()`, like `paintStrokeEnd`'s `paint_cursor`)? For an
   expensive mesh/geometry op, budget the per-`step()` unit of work so it completes well inside
   `max_step_micros` (< 8_000µs) — chunk by a fixed byte/item count constant analogous to
   `LOWPOLY_RETAINED_PAINT_CHUNK_BYTES`.

2. **Write the exact admission check.** Add an arm to `*_command_admitted` for `X`'s payload
   (bound every variable-length field against an existing or new `*_FIELD_BYTES`/`*_RAW_BYTES`
   constant). If `X` reads/writes new document fields, extend `*_snapshot_admitted` too.

3. **Write the exact Store publication check.** If `X`'s disposition is `Artifact`/`ArtifactTransient`:
   add an arm to `*_artifact_mutation_retained_bytes` for every new `Mutation` variant `X` emits, and
   confirm `prepare_*_artifact`'s pre/post admission still holds (base AND result both re-checked
   against `*_ARTIFACT_STORE_MAXIMUM_BYTES`). Same for `Config`/`ConfigTransient` against
   `*_config_mutation_retained_bytes`/`*_CONFIG_STORE_MAXIMUM_BYTES`. Without this, `X` cannot be
   migrated — this exact-footprint check is literally what the `BatchOnlyPendingRewrite` blocker
   string names as missing.

4. **Move the id.** Remove `"X"` from `*_BATCH_ONLY_TOOL_IDS` and add it to `*_MIGRATED_TOOL_IDS`
   (component.rs:351-401 for lowpoly).

5. **Add the disposition arm.** In `*_command_disposition`, add `"X" => *CommandDisposition::<lane>`
   — pick per §8: pure document mutation → `Artifact`; pure session/view state → `Config` (or omit —
   it's the catch-all for anything in `*_MIGRATED_TOOL_IDS`); host-only load → `HostOnly`; ephemeral
   gesture-only → `Transient`; combined document+ephemeral → `ArtifactTransient`; combined
   config+ephemeral → `ConfigTransient`.

6. **Add the reduce arm.** In `*_retained_reduce` (or a dedicated `*RetainedCommandWork::step`
   arm for a multi-step command), add `Command::X(payload) => x_handler::handle(payload, &doc, &cfg, …)`
   returning the right `ArtifactCommandWorkStep` variant (§2). For a multi-step command, implement
   `ArtifactCommandWork` directly (mirror `LowpolyRetainedCommandWork`): own the cursor field(s),
   implement `extent()` (returning `None` or an oversized count must make the job reject before any
   work starts — see the `Preflight` phase check in retained-command/🦀️component.rs:465), and
   implement `checkpoint()`/`restore()` round-tripping the cursor plus a magic/version tag, disposition
   byte, tool identity digest, operation id/generation/base-revision, and context identity — reject on
   any mismatch (this is what makes the resume "exact" rather than best-effort).

7. **Register the publication contract.** Add `ArtifactToolPublicationContract { tool_id: "X", lanes: &[...] }`
   to `PUBLICATION_CONTRACTS` (component.rs:884-904) — lanes must exactly match the disposition chosen
   in step 5 (`Artifact`→`[Artifact]`, `ArtifactTransient`→`[Artifact, Transient]`, etc.), and must be
   non-empty; `HostOnly` must be the sole lane if used.

8. **Register the execution contract / proof.** Add `"X" => ToolExecutionContract::resumable(max_raw_wire_bytes, max_decoded_items, 1, max_output_bytes, max_step_micros, 1, 1)` to the
   `bounded_first_step_tool_proofs!` macro invocation's `tools: { … }` map (component.rs:1409-1428) —
   or `bounded_first_step` if `X` is genuinely one-shot and never needs mid-flight resume.

9. **Flip the manifest action registration.** Change
   `.action_interactive_job("X", InteractiveJobClassification::BatchOnlyPendingRewrite)` to
   `.action_interactive_job("X", InteractiveJobClassification::Migrated)` in the `EditorApp` builder
   chain (component.rs:1757-1803).

10. **Update the JSON route.** In `🧪️interactive-job/🔣️component.json`, change `X`'s route:
    `"classification": "Migrated"`, `"lanes"` = the same array as step 7 (as strings), `"preparation"`
    = `["Artifact"]` and/or `["Config"]` per which `admit_*`/`prepare_*` pair(s) step 3 wired up
    (`HostOnly`/pure-`Transient` commands get `"preparation": []`), `"blocker": null`.

11. **Re-run/extend the TS oracle test** (`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts`,
    `TestScript.run`) — it is exactly "a test that reads `🔣️component.json`" and cross-checks it
    against the Rust source via regex:
    - `validateOwnedFixture` requires `fixture.routes.length` to match the plugin's total tool count,
      and the migrated/batch counts (`migrated === 19 && batch === 28` for lowpoly today — these
      literals must be bumped when `X` moves categories) plus a fixed lane/preparation signature
      allowlist (line 44: `["Artifact|Artifact","Config|Config","HostOnly|","Transient|","Config+Transient|Config","Artifact+Transient|Artifact"]`
      — add a new signature here only if `X` introduces a genuinely new lane/preparation combination).
    - It regex-scans the Rust source for `.action_interactive_job("X", InteractiveJobClassification::…)`
      (line 70) and asserts the count equals the total route count (line 71) and that every JSON route's
      `toolId`+`classification` pair is found in source (line 73).
    - For `Migrated` routes it string-searches the source for the literal
      `ArtifactToolPublicationContract { tool_id: "X", lanes: &[…] }` (line 76) and
      `"X" => semio_framework::ToolExecutionContract::resumable` (line 77) — so steps 7/8 must produce
      byte-identical literal text to what this test greps for.
    - It also asserts a fixed list of "structural needles" are present in source (line 80-100) — a
      genuinely new multi-step command should extend this list with its own new checkpoint-format
      marker / cursor field name, mirroring how `paintStrokeEnd`'s needles
      (`"fn paint_end_step(&mut self"`, `"paint_replay_target"`, `"LOWPOLY_RETAINED_PAINT_CHUNK_BYTES"`)
      were added.
    - Ajv (`schema.json`) hostile-fixture checks (lines 108-121) validate the schema itself rejects
      duplicate routes, empty lanes on Migrated, empty blocker on BatchOnly, and lane/preparation
      mismatches — no change needed here unless a new signature is added to the allowlist.

12. **Update the two test-list assertions inside the Rust file itself** (component.rs:1884-1885):
    ```rust
    assert!(LOWPOLY_MIGRATED_TOOL_IDS.iter().all(|tool_id| lowpoly_command_disposition(tool_id).is_some()));
    assert!(LOWPOLY_BATCH_ONLY_TOOL_IDS.iter().all(|tool_id| lowpoly_command_disposition(tool_id).is_none()));
    ```
    These self-verify automatically once steps 4-5 are done correctly — no edit needed, but run them.

---

## 11. Framework-enforced numeric constraints

| Constraint | Enforced where | Overflow behavior |
|---|---|---|
| `max_step_micros` (aka `maximumPollMicros`) | `ToolExecutionContract::validate()` (action-bus 🦀️.rs:291) | Contract registration itself errors `"max_step_micros must be strictly below 8000"` if 0 or ≥ `INTERACTIVE_MAX_STEP_MICROS` (8_000). At runtime, `StepContext::should_yield()`/`fuel_remaining()` force `StepOutcome::Yield` once the step budget is spent (retained-command 🦀️component.rs:386). |
| `max_raw_wire_bytes` (aka `maximumRawBytes`) | `RetainedToolWireInput::try_new`/`admit_page` (action-bus 🦀️.rs:119-140); re-checked per-factory in `create_job_from_wire_pages_with_payload` (component.rs:870) | `declared_bytes > maximum_bytes` → `ToolJobFactoryError::new("declared tool wire extent exceeds its admitted contract")` at construction; a page that would push `admitted_bytes` over the cap, or a full page-Vec, → `"tool wire page exceeds its pre-admitted extent"`, page handed back to caller. |
| `max_decoded_items` (aka `maximumWorkItems`) | `ArtifactRetainedCommandJob` `Preflight` phase (retained-command 🦀️component.rs:465): `work.extent(...)` must be `Some(n)` with `0 < n <= maximum_work_items` | Otherwise faults: `"retained command exceeds semantic work capacity"` (terminal `StepOutcome::Fault`, non-resumable). |
| `max_output_bytes` | Declared per contract (e.g. `32 * 1024 * 1024` for lowpoly); enforced by the output `RetainedJobPayload`/segmented-output machinery (`ARTIFACT_OUTPUT_CHUNK_BYTES` = 4_096, component.rs:12665) chunking result bytes | Payload construction is bounded by pre-reserved fixed capacity; oversized output is rejected at that boundary. |
| `artifactStoreMaximumBytes` / `configStoreMaximumBytes` | `admit_*_mutation` + `prepare_*` pair (§4), checked against `LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES` (16 MiB) / `LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES` (16_384) | Returns `Err(String)` from `prepare_*`, rejecting the mutation before it reaches the store — both the pre-mutation base AND the post-mutation result are independently checked, so growth-through-mutation cannot slip past the cap. |
| Checkpoint capacity | `ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES` = 512 (retained-command 🦀️component.rs:10); lowpoly's own `LPC2` checkpoint is a fixed 88 bytes, well under this | Encoding a checkpoint over-capacity faults with `"retained-command-checkpoint-capacity"`. |
| Wire page size | `TOOL_WIRE_PAGE_BYTES` = 4_096 (action-bus 🦀️.rs:78) | `ToolWirePage::try_copy_from` rejects `bytes.len() > TOOL_WIRE_PAGE_BYTES` with `"tool wire page exceeds its fixed byte capacity"`. |

All of the above are declared per-plugin at the top of the route JSON
(`maximumPollMicros: 7500, maximumRawBytes: 16384, maximumWorkItems: 258, artifactStoreMaximumBytes: 16777216, configStoreMaximumBytes: 16384` for lowpoly) and must match the Rust-side constants
(`LOWPOLY_RETAINED_RAW_BYTES`, `LOWPOLY_RETAINED_WORK_ITEMS`, `LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES`,
`LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES`) and the `resumable(...)` contract call args (`16_384, 258, 1,
33_554_432, 7_500, 1, 1`) — the TS oracle (`script.ts:34`) hard-asserts the JSON top-level numbers
equal these exact literals.
