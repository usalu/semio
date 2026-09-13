# ⏯️ Wave W0-A: `⏯️tool-run` framework module

Lane W0-A of `📋️tool-run-contract.md` (§2, §3.1, §3.2, §3.7, §5 W0-A row, §6). Status: **landed, green.**

Module root `M` = `🧰️framework/🔨️modules/⏯️tool-run/`.

## 1. What changed

| File | Content |
|---|---|
| `M/🧬️schema/🔣️.json` (new) | Source of record. `$defs` for every type listed in §3 below, plus `LifecycleLawFixture`, `TracePageFixture` and `TickFixture` |
| `M/🦀️.rs` (new, 2098 lines) | Rust contract. Regions: Limits `:16`, Identity `:51`, Lifecycle `:100`, Progress `:484`, Trace `:700`, Tick `:1024`, Codec `:1254`, Definition `:1667`, Actions `:1842`, Labels `:1957` |
| `M/🟦️.ts` (new) | TS mirror of the Rust file, plus a `Json` region with converters to and from the schema JSON form (fixtures, debugging) |
| `M/🧫️fixtures/⚖️lifecycle-law.json` (new) | The full 10 × 18 matrix (180 rows, 46 legal), 21 cases, 6 scenarios, 5 invariants, 7 actions, 22 labels, reserved reasons, 5 template cases, limits, and one sample `ToolRunDefinition` |
| `M/🧫️fixtures/📼️trace-pages.json` (new) | 3 pages and 2 deltas (JSON + hex), 9 malformed wires, 2 residency cases, 11 delivery cases |
| `M/🧫️fixtures/🎞️ticks.json` (new) | 3 ticks (JSON + hex), 4 step-ring cases, 4 writer cases |
| `M/🧪️tests/🔬️unit/🦀️.rs` (new) | 21 Rust tests over the fixtures, plus u64-range, cap and 5 000-candidate delivery laws |
| `M/🧪️tests/🧩️conformance/🟦️.ts` (new) | 18 bun tests. Oracles: **ajv** (fixtures plus hostile mutations), **xstate** (machine built from the matrix and compared on every row), **fast-check** (3 000 weighted random sequences, TS reducer vs xstate) |
| `M/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs,📜️script.ts,📋️project.json,package.json}` (new) | Crate `semio-framework-tool-run`, lib `semio_framework_tool_run`. nx project `@semio-tech/framework-tool-run-rs` with targets `test`, `test-quick`, `test-long`, `test-exhaustive`, `check` |
| `Cargo.toml` (root) | Added the workspace member after `🧵️job` and `semio-framework-tool-run = { path = … }` in `[workspace.dependencies]` after `semio-framework-job` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | `members-of-modules` gains `⏯️tool-run` after `🧵️job` |
| `T/🐍️w0a-fill-tool-run-fixture-hex.ts` (input script, kept) | One-off: fills fixture `hex` fields from the TS encoder and builds the malformed wires. Rust then independently verified byte equality through the real pack `encode_record_body` |

### Dependencies (no third-party runtime dependencies)

- `semio-framework-os-kernel`: provides `os_pack::{encode_record_body, decode_record_body_exact}`, `os_dsl::schema::{RecordSpec, FieldValue, …}` and the `ToValue`/`FromValue` derives. It is aliased as `dsl` in the glue.
- `semio-framework-ui` with feature `wgpu` (the declarative-only feature, no `wgpu` crate), imported as `ui`: provides `LocalizedLabel` and `Locale`.
- `serde`: needed because the manifest types derive `Serialize`/`Deserialize`.
- `serde_json`: dev-dependency only.

**Cycle warning for other lanes:** `semio-framework-ui`, `-ui-scene`, `-ui-contract`, `-os-kernel` and `-job` must **never** depend on `semio-framework-tool-run`.

- W0-E: the scene lane should carry the delta as an opaque base64url string. Only renderer crates above `ui` (for example `os-infinite`) should decode it.
- W0-B: the `semio-framework` crate (manifest) may depend on it. There is no cycle, because tool-run does not depend on `semio-framework`.

## 2. Tests run (exact commands, all foreground; logs in `T/🗑️generated/W0-A/`)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-tool-run` | **21 passed, 0 failed** (`cargo-test-final.txt`) |
| `cargo check -p semio-framework-tool-run --target wasm32-wasip2` | Finished, 0 errors (`check-wasip2-final.txt`) |
| `cargo clippy -p semio-framework-tool-run --tests` | 0 warnings in the crate after fixes (`clippy-2.txt`) |
| `bun ./📜️script.ts test` (in `M/📦️packages/🦀️rust`) | TS **18 pass, 0 fail, 867 expects**, then nextest **21 passed** (`script-test-final.txt`) |
| `bun ./📜️script.ts check` | native and wasm32-wasip2 checks Finished (`script-check-final.txt`) |
| `bun nx run @semio-tech/framework-tool-run-rs:test --skip-nx-cache` | Successfully ran (`nx-test.txt`) |
| `tsc --strict --noUncheckedIndexedAccess` on `M/🟦️.ts` | 0 errors. The test file shows only the repo-wide missing `bun:test` types |

### Checks that the tests actually catch mistakes

- **Type-check proof (rule 8).** A temporary `[DEBUG]` probe function made the crate emit warnings on both native and `wasm32-wasip2`, which proves type-checking reached the crate on both. The probe was then removed (`check-*-probe.txt`).
- **Mutation 1.** Changing the `complete × baseChanged` generation bump made 4 tests fail: matrix, cases, xstate row parity, and fast-check.
- **Mutation 2.** Making the staleness guard ignore the generation on `resume` was caught **only** by the fast-check vs xstate property. Both files were restored byte-identical (verified with `cmp`).

## 3. Public API as landed (Rust; the TS names mirror them in camelCase)

### Limits

```rust
pub const TOOL_RUN_STEP_RING_CAPACITY: usize = 64;
pub const TOOL_RUN_STEP_ARGS_MAX: usize = 4;
pub const TOOL_RUN_COUNTERS_MAX: usize = 8;
pub const TOOL_RUN_PROVISIONAL_OPS_MAX: u32 = 65_536;
pub const TOOL_RUN_TRACE_RESIDENT_RECORDS: usize = 1_048_576;
pub const TOOL_RUN_TRACE_PAGE_OPS_MAX: usize = 4_096;
pub const TOOL_RUN_TRACE_PAGE_BYTES_MAX: usize = 262_144;
pub const TOOL_RUN_TICK_BYTES_MAX: usize = 262_144;
pub const TOOL_RUN_TRACE_LOG_COMPACT_FLOOR: usize = 4_096;
pub const TOOL_RUN_TRACE_PAGE_OVERHEAD_BYTES: usize = 64;
pub const TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS: u64 = 2_000;
pub const TOOL_RUN_RESERVED_REASON_FLOOR: u16 = 0xFF00;
pub const TOOL_RUN_REASON_REBASING: u16 = 0xFF00;
pub const TOOL_RUN_REASON_CONFLICT: u16 = 0xFF01;
pub const TOOL_RUN_REASON_TRACE_TRUNCATED: u16 = 0xFF02;
pub const TOOL_RUN_REASON_PROVISIONAL_CAP: u16 = 0xFF03;
pub const TOOL_RUN_GROUP_ID_PREFIX: &str = "toolRun:";
```

### Identity

```rust
pub struct ToolRunId { pub app_instance_id: u32, pub run: u64 }            // Copy, Eq, Hash
impl ToolRunId { pub fn group_id(self) -> String }                          // "toolRun:<run>"
pub struct ToolRunIdentity { pub id: ToolRunId, pub generation: u32, pub base_revision: [u8; 32] }
impl ToolRunIdentity {
    pub fn new(id: ToolRunId, base_revision: [u8; 32]) -> Self;             // generation 0
    pub fn next_generation(self) -> Self;
    pub fn freshness(self, sequence: u64) -> ToolRunFreshness;
}
pub struct ToolRunFreshness { pub run: u64, pub generation: u32, pub sequence: u64 } // Ord (lexicographic)
```

### Lifecycle (§2.2)

```rust
pub enum ToolRunState { Starting, Running, Paused, Complete, Finalizing, Finalized, Aborting, Aborted, Faulted }
// serde/ToValue camelCase; ALL, as_str, parse, ordinal() -> u8, from_ordinal(u64), is_terminal(), label() -> ToolRunLabel
pub struct ToolRunSlot { pub run: u64, pub generation: u32, pub state: ToolRunState }
pub enum ToolRunEvent {
    Start { run: u64 },
    JobAdmitted { run: u64, generation: u32 }, Pause { run: u64, generation: u32 }, Resume { run: u64, generation: u32 },
    Step { run: u64, generation: u32 }, JobComplete { run: u64, generation: u32 }, JobFault { run: u64, generation: u32 },
    SettingsChanged { run: u64 }, BaseChanged { run: u64 },
    Finalize { run: u64, generation: u32 }, PublicationComplete { run: u64, generation: u32 },
    RevalidationConflicts { run: u64, generation: u32 }, StoreRejected { run: u64, generation: u32 },
    Abort { run: u64, generation: u32, publishing: bool }, AbortComplete { run: u64, generation: u32 },
    Dismiss { run: u64 }, Closed,
}
impl ToolRunEvent { pub fn key(self) -> ToolRunEventKey; pub fn target(self) -> (Option<u64>, Option<u32>) }
pub enum ToolRunEventKey { Start, JobAdmitted, Pause, Resume, Step, JobComplete, JobFault, SettingsChanged, BaseChanged,
    Finalize, PublicationComplete, RevalidationConflicts, StoreRejected, Abort, AbortWhilePublishing, AbortComplete, Dismiss, Closed } // ALL, as_str, parse
pub enum ToolRunEffect { SpawnJob, Schedule, StopScheduling, DriveOneUnit, HoldResult, Reconfigure, Refold, BeginFinalize,
    ReleaseProvisional, RetractConflicts, KeepProvisional, CloseJob, CancelBatch, RetireProvisional, DiscardProvisional, ClearTrace, RetireAll }
impl ToolRunEffect { pub fn commits(self) -> bool /* only ReleaseProvisional */; ALL, as_str, parse }
pub enum ToolRunRejection { Stale, Busy, Illegal }        // code(): "toolRun.stale" | "toolRun.busy" | "toolRun.illegal"; Display + Error
pub struct ToolRunTransition { pub slot: Option<ToolRunSlot>, pub effect: ToolRunEffect }
pub struct ToolRunMachine;
impl ToolRunMachine { pub fn apply(slot: Option<ToolRunSlot>, event: ToolRunEvent) -> Result<ToolRunTransition, ToolRunRejection> }
```

Reducer rules, applied in this order:

1. `Start` on a non-terminal slot is `Busy`. `Start` with `run <= slot.run` is `Illegal`. Otherwise the result is `Starting` with generation 0 and effect `SpawnJob`.
2. `Closed` always yields slot `None` with effect `RetireAll`.
3. With no slot, or on a run/generation mismatch (generation is only checked when the event carries one), the result is `Stale`.
4. Otherwise the §2.2 table decides. `finalizing × abort{publishing:true}` is `Stale`. Every pair not in the table is `Illegal`.

### Progress (§2.3)

```rust
pub enum ToolRunVerdict { Testing, Success, Warning, Danger }     // ordinal u8, is_rejected() (warning|danger), serde/ToValue camelCase
pub enum ToolRunStepKind { Info, Success, Warning, Danger }
pub enum ToolRunStepArg { Unsigned(u64), Float(f64) }              // PartialEq bitwise on floats; to_plain_string()
pub struct ToolRunStep { pub sequence: u64, pub kind: ToolRunStepKind, pub stage: u16, pub reason: u16, pub subject: Option<u64>, pub repeat: u32, pub args: Vec<ToolRunStepArg> }
impl ToolRunStep { pub fn new(sequence, kind, stage, reason) -> Self /* repeat 1 */; pub fn coalesces_with(&self, other: &Self) -> bool }
pub struct ToolRunStepRing;  // new(), push(step) (coalesce: repeat += step.repeat, sequence = newest; else overwrite-oldest at 64),
                             // from_steps(Vec) -> Result<Self, ToolRunCodecError>, len, is_empty, iter, oldest, newest, clear
pub struct ToolRunCounter { pub counter: u16, pub value: u64 }
pub struct ToolRunProgress { pub identity, pub sequence: u64, pub state, pub stage: u16, pub completed: u64, pub total: Option<u64>,
    pub counters: Vec<ToolRunCounter>, pub units_per_second: f32, pub conflicts: u32, pub steps: ToolRunStepRing }
impl ToolRunProgress { pub fn fraction(&self) -> Option<f64> }
```

### Trace (§3.2)

```rust
pub enum ToolRunTraceSubject { Instance3d { mesh: u32, position: [f32; 3], rotation: [f32; 4], scale: f32 }, Placement2d { shape: u32, position: [f32; 2], rotation: f32 }, Entity { entity: u64 } }
pub enum ToolRunTraceOp { Upsert { key: u64, verdict: ToolRunVerdict, reason: u16, subject: ToolRunTraceSubject }, Retire { key: u64 }, Clear }
impl ToolRunTraceOp { pub fn wire_bytes(self) -> usize }            // 13+36 | 13+16 | 13+8, retire 9, clear 1
pub struct ToolRunTracePage { pub identity: ToolRunIdentity, pub page: u32, pub ops: Vec<ToolRunTraceOp> }
impl ToolRunTracePage { pub fn wire_bytes_estimate(ops: &[ToolRunTraceOp]) -> usize; pub fn encode(&self) -> Result<Vec<u8>, ToolRunCodecError>; pub fn decode(bytes: &[u8]) -> Result<Self, ToolRunCodecError> }
pub struct ToolRunTraceDelta { pub identity: ToolRunIdentity, pub clear: bool, pub next: u32, pub pages: Vec<ToolRunTracePage> } // encode/decode
pub struct ToolRunTraceCursor { pub run: u64, pub generation: u32, pub page: u32 }  // page = next page the renderer expects
pub struct ToolRunTraceRecord { pub verdict, pub reason: u16, pub subject /* + private stamp */ }
pub struct ToolRunTraceApply { pub logged: u32, pub evicted: u32, pub overflowed: u32, pub compacted: bool }
pub struct ToolRunTraceStore;
impl ToolRunTraceStore {
    pub fn new(identity: ToolRunIdentity) -> Self;                                     // capacity 1 048 576, compact floor 4 096
    pub fn with_limits(identity: ToolRunIdentity, capacity: usize, compact_floor: usize) -> Self;
    pub fn identity(&self) -> ToolRunIdentity;
    pub fn rebind(&mut self, identity: ToolRunIdentity);                               // new generation keeps records; new run resets
    pub fn len(&self) -> usize; pub fn is_empty(&self) -> bool;
    pub fn record(&self, key: u64) -> Option<&ToolRunTraceRecord>;
    pub fn records(&self) -> impl Iterator<Item = (u64, &ToolRunTraceRecord)>;
    pub fn log_base(&self) -> u32; pub fn next_page(&self) -> u32; pub fn log_page(&self, page: u32) -> Option<&[ToolRunTraceOp]>;
    pub fn apply_page(&mut self, page: &ToolRunTracePage) -> Result<ToolRunTraceApply, ToolRunRejection>; // Stale on run/generation mismatch
    pub fn apply_ops(&mut self, ops: &[ToolRunTraceOp]) -> ToolRunTraceApply;
    pub fn delta_after(&self, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> ToolRunTraceDelta;
}
```

**Residency policy.** When a new key arrives at capacity:

- the oldest resident `warning`/`danger` record is evicted, and a `Retire` op is logged for it (`evicted += 1`);
- if there is no evictable record and the incoming record is rejected, the incoming record is dropped (`evicted += 1`);
- if there is no evictable record and the incoming record is `success`/`testing`, it is refused (`overflowed += 1`). `success`/`testing` records are never evicted.

The caller turns `evicted > 0` into one coalesced `warning` step with reason `TOOL_RUN_REASON_TRACE_TRUNCATED` and args `[evicted]`. It turns `overflowed > 0` into the §2.8 "end as complete" handling.

**Page log.** Logged pages are numbered by the store (job page numbers are ignored) and split at 4 096 ops. When `logged_ops > 2 * max(resident, compact_floor)`, the log is replaced by snapshot pages. The snapshot starts with a `Clear` op and holds the resident records in insertion order, with `log_base = old next_page`.

**`delta_after`.** It resends (`clear = true`, starting from `log_base`) when any of these holds:

- the cursor is missing;
- the run or generation differs;
- `page < log_base`;
- `page > next_page`.

Otherwise it continues from `cursor.page`. It adds pages until the next page would exceed `byte_budget`, but always delivers at least one pending page. `next` is the cursor page to echo back. An idle, caught-up cursor gets an empty delta with `clear = false`.

### Tick (§3.2, §3.7)

```rust
pub struct ToolRunTick { pub identity, pub sequence: u64, pub progress: Option<ToolRunProgress>, pub steps: Vec<ToolRunStep>,
    pub trace: Vec<ToolRunTracePage>, pub append_ops: Vec<Vec<u8>>, pub append_entities: Vec<u64>, pub retract_to: Option<u32> }
impl ToolRunTick { pub fn encode(&self) -> Result<Vec<u8>, ToolRunCodecError> /* ≤ 256 KiB */; pub fn decode(bytes: &[u8]) -> Result<Self, ToolRunCodecError> }
pub enum ToolRunLimitError { ProvisionalOps, StepArgs }
pub struct ToolRunTickWriter;
impl ToolRunTickWriter {
    pub fn new(identity: ToolRunIdentity) -> Self; pub fn identity(&self) -> ToolRunIdentity; pub fn rebind(&mut self, identity: ToolRunIdentity);
    pub fn provisional_len(&self) -> u32;
    pub fn upsert(&mut self, key: u64, verdict: ToolRunVerdict, reason: u16, subject: ToolRunTraceSubject);
    pub fn retire(&mut self, key: u64); pub fn clear_trace(&mut self);
    pub fn step(&mut self, kind: ToolRunStepKind, stage: u16, reason: u16, subject: Option<u64>, args: &[ToolRunStepArg]) -> Result<(), ToolRunLimitError>;
    pub fn append_op(&mut self, op: Vec<u8>) -> Result<(), ToolRunLimitError>;   // refuses at TOOL_RUN_PROVISIONAL_OPS_MAX
    pub fn append_entity(&mut self, entity: u64);
    pub fn retract_to(&mut self, len: u32);                                         // below committed base → tick.retract_to; else truncates pending appends
    pub fn progress(&mut self, progress: ToolRunProgress);
    pub fn is_empty(&self) -> bool; pub fn pending_bytes(&self) -> usize; pub fn should_flush(&self) -> bool; // ≥ 128 KiB estimate
    pub fn finish(&mut self) -> Option<ToolRunTick>;                                // monotone sequence, page and step numbering
}
pub enum ToolRunCodecError { Pack(String), Malformed(&'static str), Limit(&'static str) }
```

**Wire.** Every value is a pack record body produced by `encode_record_body` and decoded with `decode_record_body_exact`. There are no strings, so the symbol table is always empty.

| Record | Fields |
|---|---|
| identity | 1 appInstanceId UInt, 2 run UInt, 3 generation UInt, 4 baseRevision Bytes(32) |
| step | 1 sequence, 2 kind, 3 stage, 4 reason, 5 subject?, 6 repeat, 7 args Bytes (9 B each: kind u8 + LE u64/f64) |
| progress | 1 identity, 2 sequence, 3 state, 4 stage, 5 completed, 6 total?, 7 counters Bytes (10 B each), 8 unitsPerSecond F64, 9 conflicts, 10 steps List(Record) |
| page (columnar) | 1 identity, 2 page, 3 opKinds u8, 4 keys u64le, 5 verdicts u8, 6 reasons u16le, 7 subjectKinds u8, 8 meshes u32le, 9 shapes u32le, 10 entities u64le, 11 floats f32le |
| delta | 1 identity, 2 clear Bool, 3 next UInt, 4 pages List(Bytes page body) |
| tick | 1 identity, 2 sequence, 3 progress?, 4 steps List(Record), 5 trace List(Bytes page body), 6 appendOps List(Bytes), 7 appendEntities Bytes u64le, 8 retractTo? |

Empty lists and columns are omitted.

### Definition (§2.4)

All of these derive `Serialize`, `Deserialize`, `ToValue` and `FromValue` with camelCase names; the structs also use `deny_unknown_fields`.

```rust
pub struct JobKindId(pub String);                           // transparent; new, as_str
pub enum ToolRunRebasePolicy { Revalidate, Restart, Freeze }
pub enum ToolRunReconfigurePolicy { Resume, Restart }
pub enum ToolRunTraceKind { Instance3d, Placement2d, Entity, None }
pub struct ToolRunStageDefinition { pub id: String, pub label: LocalizedLabel }
pub struct ToolRunCounterDefinition { pub id: String, pub label: LocalizedLabel }
pub struct ToolRunReasonDefinition { pub code: u16, pub id: String, pub verdict: ToolRunVerdict, pub template: LocalizedLabel }
pub struct ToolRunDefinition { pub mutating: bool, pub rebase: ToolRunRebasePolicy, pub reconfigure: ToolRunReconfigurePolicy, pub unit: LocalizedLabel,
    pub stages: Vec<ToolRunStageDefinition>, pub counters: Vec<ToolRunCounterDefinition>, pub reasons: Vec<ToolRunReasonDefinition>,
    pub trace: ToolRunTraceKind, pub run_job: JobKindId, pub revalidate_job: Option<JobKindId> /* skipped when None */ }
impl ToolRunDefinition { pub fn validate(&self) -> Result<(), ToolRunDefinitionError>; pub fn stage(&self, u16); pub fn counter(&self, u16); pub fn reason(&self, code: u16) }
pub enum ToolRunDefinitionError { NoStages, TooManyStages, TooManyCounters, DuplicateStageId(String), DuplicateCounterId(String), DuplicateReasonId(String), DuplicateReasonCode(u16), ReservedReasonCode(u16) }
```

### Actions, chords and labels (§2.5)

```rust
pub const TOOL_RUN_START_ACTION_ID = "toolRunStart"; TOOL_RUN_PAUSE_ACTION_ID = "toolRunPause"; TOOL_RUN_RESUME_ACTION_ID = "toolRunResume";
pub const TOOL_RUN_STEP_ACTION_ID = "toolRunStep"; TOOL_RUN_ABORT_ACTION_ID = "toolRunAbort"; TOOL_RUN_FINALIZE_ACTION_ID = "toolRunFinalize";
pub const TOOL_RUN_DISMISS_ACTION_ID = "toolRunDismiss"; pub const TOOL_RUN_ACTION_IDS: [&str; 7];
pub const TOOL_RUN_START_CHORD = "mod+enter"; TOOL_RUN_PAUSE_RESUME_CHORD = "mod+alt+enter"; TOOL_RUN_STEP_CHORD = "mod+alt+arrowright";
pub const TOOL_RUN_ABORT_CHORD = "mod+."; TOOL_RUN_FINALIZE_CHORD = "mod+shift+enter"; TOOL_RUN_DISMISS_CHORD = "escape";
pub const TOOL_RUN_ARG_TOOL_ID = "toolId"; TOOL_RUN_ARG_WINDOW_ID = "windowId"; TOOL_RUN_ARG_RUN_ID = "runId"; TOOL_RUN_ARG_GENERATION = "generation";
pub struct ToolRunActionArg { pub name: &'static str, pub required: bool }
pub enum ToolRunAction { Start, Pause, Resume, Step, Abort, Finalize, Dismiss }
impl ToolRunAction { ALL; pub fn id(self); pub fn from_id(&str) -> Option<Self>; pub fn chord(self); pub fn label(self) -> ToolRunLabel; pub fn args(self) -> &'static [ToolRunActionArg]; pub fn is_legal_in(self, state: Option<ToolRunState>) -> bool }
pub enum ToolRunLabel { StateStarting … StateFaulted, ActionStart … ActionDismiss, FinalizeDisabled, RebasingStep, ConflictStep, TraceTruncatedStep, ProvisionalCapStep, ProgressValueText }
impl ToolRunLabel { ALL: [Self; 22]; pub fn key(self); pub fn parse(&str); pub fn text(self, locale: Locale) -> &'static str; pub fn localized(self) -> LocalizedLabel /* ::native(en, de) */; pub fn for_reason(code: u16) -> Option<Self> }
pub fn tool_run_format(template: &str, value: impl Fn(&str) -> Option<String>) -> String;   // single-pass {name}; unknown/unterminated stay literal
```

### TS mirror (`M/🟦️.ts`)

Values of Rust type `u64` are `bigint` in TS.

| Area | Exports |
|---|---|
| Constants | Same names as Rust; `TOOL_RUN_STATES`, `TOOL_RUN_EVENT_KEYS`, `TOOL_RUN_EFFECTS`, `TOOL_RUN_VERDICTS`, `TOOL_RUN_STEP_KINDS` |
| Lifecycle | `ToolRunMachine.apply(slot, event): { ok: true, transition } \| { ok: false, rejection }`; `toolRunEventKey`, `toolRunEffectCommits`, `isToolRunTerminal`, `isToolRunVerdictRejected`; `toolRunGroupId`, `compareToolRunFreshness` |
| Steps and progress | `class ToolRunStepRing`, `toolRunStepsCoalesce`, `toolRunProgressFraction` |
| Trace | `class ToolRunTraceStore(identity, capacity?, compactFloor?)` with `applyPage` (returns `"toolRun.stale"`), `applyOps`, `deltaAfter`, `rebind`, `record`, `records`, `logPage`, `logBase`, `nextPage`, `size`; `toolRunTraceOpWireBytes`, `toolRunTracePageWireBytes` |
| Codecs | `encode`/`decodeToolRunTracePage`, `encode`/`decodeToolRunTraceDelta`, `encode`/`decodeToolRunTick`; `class ToolRunCodecError(kind, what)` |
| Actions and labels | `TOOL_RUN_ACTIONS`, `isToolRunActionLegal`, `TOOL_RUN_LABELS` (key → {en, de}), `toolRunStateLabel`, `toolRunReasonLabel`, `toolRunFormat` |
| JSON form | `toolRun{Identity,Slot,Event,Step,TraceOp,TracePage,TraceDelta,Progress,Tick}{From,To}Json`, `toolRunHexToBytes`, `toolRunBytesToHex` |

The TS writer is not mirrored, because jobs are Rust.

## 4. Commands to register in launch.json

The coordinator does the registration; per the lane rules this lane did not edit launch.json.

- `bun nx run @semio-tech/framework-tool-run-rs:test`: TS conformance (ajv/xstate/fast-check), then Rust fixture tests
- `bun nx run @semio-tech/framework-tool-run-rs:check`: native and `wasm32-wasip2` type check

## 5. Deviations from the contract, with reasons

1. **Reducer signature.** `apply(slot: Option<ToolRunSlot>, event)` instead of `apply(state, event)`. The staleness guard and the monotone run id need `run` and `generation`, and "no run" is `None`, as §2.2 requires.
2. **Event vocabulary made explicit.**
   - `Abort` carries `publishing: bool`, filled in by the driver from the batch phase. The matrix keys the `true` case as `abortWhilePublishing`.
   - A new `AbortComplete` event models the two-phase `aborting → aborted`.
   - `SettingsChanged`, `BaseChanged` and `Dismiss` carry only `run`. The other non-start events carry `run` and `generation`, so results from jobs of an older generation are stale.
3. **Added rejection `toolRun.illegal`** (for example `finalize` while running), next to `stale` and `busy`. Every rejection stays a silent no-op.
4. **`ToolRunTraceDelta` gained `clear` and `next`.** "Clear followed by resend" became a flag instead of a synthetic op page, so pages are not re-encoded. The cursor's `page` is the *next expected* page, which makes an idle refresh free.
   - Delta and tick pages travel as `List(Bytes)` of standalone page bodies.
5. **Residency when nothing is evictable.** An incoming `success`/`testing` record is refused and reported as `overflowed`. The contract forbids evicting it and only a bounded store is acceptable.
6. **Log compaction.** A page log that replays from page 0 would otherwise grow without bound, so it is compacted into a `Clear` + snapshot of the resident records (§3 Trace, "Page log").
7. **Framework-reserved reasons.**
   - Reason codes `0xFF00..=0xFF03` are reserved; plugin reason codes must be below `0xFF00`, and `validate()` rejects others.
   - A new label **`provisionalCapStep`** (EN "Provisional change limit of {0} reached, run completed", DE "Grenze von {0} vorläufigen Änderungen erreicht, Lauf abgeschlossen") covers §2.8's warning step, which had no label in §2.5.
8. **`JobKindId`** is defined here as a transparent `String` newtype. No such type existed; `ArtifactToolFactoryRegistry` keys are `String`.
9. **launch.json not edited.** The §5 W0-A row lists it, but `📋️lane-rules.md` rule 10 (binding) forbids it. See §4 for the commands.
10. **TDD order.** Fixtures and schema were written first, but the implementation landed before the first test run, which was green immediately. To show the tests are not vacuous, two deliberate mutations were run (§2).

## 6. Foreign edits

None outside W0-A's owned set. Root `Cargo.toml` and `🔣️taxonomy.json` are W0-A-owned per §5.

## 7. Open items

- **W0-D.** `ToolRunTickWriter::retract_to` does not trim `append_entities`, because entities are not index-aligned with ops. The ledger should recompute the provisional entity set on retract, or the contract should align entities per op.
- **W0-D.** `ToolRunTraceStore::apply_page` ignores the job's page numbers; the store numbers pages itself. The `evicted` and `overflowed` counts must be turned into steps by the driver (§3 Trace).
- **W0-B.** Reference the `⏯️tool-run` schema `$defs` (`ToolRunDefinition`, `LocalizedLabel`) by `$ref` from `🛂️manifest/🧬️schema/🔣️.json`. The Rust types already derive the manifest's dual serde + value derives.
- **W0-E.** Keep `semio-framework-ui-scene` free of a dependency on this crate (cycle, see §1). Decode the base64url delta with `decodeToolRunTraceDelta` in TS, and in renderer crates above `ui`.
- **Cleanup.** The ticket's `🗑️generated/W0-A/*.txt` logs stay for the coordinator; this lane swept nothing.
