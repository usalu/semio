# 📓️ Channel Audit — Artifact Mutations via AppCommand/AppFrame

Contract: establish exactly how an external MCP process can PREVIEW, COMMIT with revision check, and UNDO an artifact mutation using existing frames.

## 1. Channel v11 Command/Frame Types & Fields

**CHANNEL_VERSION = 11** (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs:20`)

### AppCommand variants (keyed by tag, path:line `🦀️component.rs:52-245`)

| Variant | Tag | Fields | Purpose |
|---------|-----|--------|---------|
| **Command** | 2 | `seq: u64`, `command: Vec<u8>`, `view_state: Vec<u8>` | Dispatch command against live view model (L66-71) |
| **PureCommand** | 18 | `seq: u64`, `command: Vec<u8>`, `document: Vec<u8>`, `document_spr: Vec<u8>`, `config: Vec<u8>`, `config_spr: Vec<u8>`, `draft: Vec<u8>`, `draft_spr: Vec<u8>` | Host-authoritative dry-run: guest returns Emit ops only, host applies (L136-145) |
| **ArtifactCommand** | 6 | `seq: u64`, `command: Vec<u8>` | Artifact mutation command (L87-90) |
| **ApplyEnvelopes** | 7 | `seq: u64`, `envelopes: Vec<MutationEnvelope>` | Ingest backbone mutations (L91-94) |
| **ReadDocument** | 9 | `seq: u64` | Requests document pack+spr (L100-102) |
| **ReadConfig** | 11 | `seq: u64` | Requests config pack+spr (L108-110) |
| **ReadHistory** | 21 | `seq: u64` | Reads complete history projection (L159-161) |
| **TransactionPrepare** | 22 | `seq: u64`, `txn_id: String`, `mutation_id: String`, `payload: Vec<u8>`, `prepared_ops: Vec<Vec<u8>>`, `label: String`, `origin: Vec<u8>` | Phase-1 prepare; EITHER owner-mutation form (`mutation_id`+`payload` non-empty, `prepared_ops` empty) OR pre-planned form (`prepared_ops`+`label`+`origin` non-empty, `mutation_id` empty) (L167-175) |
| **TransactionCommit** | 23 | `seq: u64`, `txn_id: String` | Phase-2 commit (L176-180) |
| **TransactionRollback** | 24 | `seq: u64`, `txn_id: String` | Abort not-yet-committed member (L181-185) |
| **TransactionUndo** | 25 | `seq: u64`, `group_id: String` | Fan group undo to committed member (L186-190) |
| **TransactionRedo** | 26 | `seq: u64`, `group_id: String` | Fan group redo (L191-195) |
| **OpenArtifact** | 27 | `seq: u64`, `artifact_ref: String`, `role: u8`, `plugin_id: String`, `app_id: String` | Open artifact in viewer/editor surface; empty plugin/app means resolve (L196-205) |
| **SetMergePolicy** | 30 | `seq: u64`, `policy: u8` | Pin connection's MergePolicy: `0`=`LaissezFaire`, `1`=`Normal`, `2`=`Vigilant` (L226-233) |
| **ResolveConflict** | 31 | `seq: u64`, `conflict_id: String`, `resolution: u8` | Resolve one Conflict: `0`=`Accept`, `1`=`Discard` (L234-240) |
| **ReadConflicts** | 32 | `seq: u64` | Reads every open Conflict (L241-244) |

### AppFrame variants (keyed by tag, path:line `🦀️component.rs:248-343`)

| Variant | Tag | Fields | Purpose |
|---------|-----|--------|---------|
| **Invocation** | 2 | `in_reply_to: u64`, `output: Vec<u8>`, `diagnostics: Vec<u8>`, `ui_scope: Vec<u8>`, `history_patch: Vec<u8>`, `messages: Vec<u8>` | Command dispatch outcome; **`messages` (CHANNEL_VERSION 11 trailing addition)** carries one packed `DispatchReport` (L256) |
| **Emit** | 14 | `in_reply_to: u64`, `document_ops: Vec<u8>`, `config_ops: Vec<u8>`, `draft_ops: Vec<u8>`, `output: Vec<u8>`, `diagnostics: Vec<u8>` | Guest Emit bytes for host-applied store authority; document/config/draft ops as packed bytes (L272-279) |
| **Document** | 7 | `in_reply_to: u64`, `pack: Vec<u8>`, `spr: Vec<u8>`, `ops: String` | Reply to ReadDocument (L261) |
| **Config** | 8 | `in_reply_to: u64`, `pack: Vec<u8>`, `spr: Vec<u8>`, `ops: String` | Reply to ReadConfig (L262) |
| **HistorySnapshot** | 18 | `in_reply_to: u64`, `history_patch: Vec<u8>` | Full history projection reply to ReadHistory (L303) |
| **TransactionProposal** | 19 | `in_reply_to: u64`, `proposal_id: String`, `local_ops: Vec<Vec<u8>>`, `description: String`, `coalesce_key: String`, `foreign: Vec<Vec<u8>>` | Guest touched foreign artifact; host mints `txn_id` and drives protocol (L308-315) |
| **TransactionPrepared** | 20 | `txn_id: String`, `foreign: Vec<Vec<u8>>`, `rejection: Vec<u8>` | Phase-1 reply; empty rejection means member is prepared (L316-321) |
| **TransactionCommitted** | 21 | `txn_id: String`, `edit_id: String` | Phase-2 commit succeeded (L322-326) |
| **TransactionRolledBack** | 22 | `txn_id: String` | Member rolled back transaction (L327-330) |
| **MergeReport** | 23 | `in_reply_to: Option<u64>`, `report: Vec<u8>` | Pushed unsolicited after ingest; one packed `MergeReport` describing batch resolution (L333-336) |
| **Conflicts** | 24 | `in_reply_to: Option<u64>`, `conflicts: Vec<u8>` | Pushed unsolicited after ingest and reply to ReadConflicts; one packed `Vec<Conflict>` (L338-342) |
| **Error** | 13 | `in_reply_to: Option<u64>`, `fault: Vec<u8>`, **`report: Vec<u8>` (CHANNEL_VERSION 11 trailing addition)** | Error response; `report` is packed `DispatchReport` of rejected dispatch when `Fault.code == "mutation.rejected"` (L270) |

---

## 2. Dispatch Semantics

### PureCommand: Dry-Run with Hydration (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`)

**PureCommand flow** (grep lines containing `PureCommand`):

1. **Host constructs** `AppCommand::PureCommand` carrying:
   - Current document pack + spr (hydrated from persistent store)
   - Current config pack + spr
   - Current draft pack + spr (live in-memory state)
   - The command payload bytes
   
2. **Guest decodes** command and ALL three store lanes from the packs/sprs WITHOUT persisting:
   - `document`, `config`, `draft` become live in-memory lanes, just like a normal Command dispatch
   - This is a TRUE dry-run: no edits touch the persistent store, no mutations are durably recorded
   
3. **Guest returns** `AppFrame::Emit` with:
   - `document_ops`: ops that WOULD apply to the document (encoded bytes)
   - `config_ops`: ops that WOULD apply to the config
   - `draft_ops`: ops that WOULD apply to the draft
   
4. **Host applies selectively**: the host chooses which lanes to apply the returned ops to; the store never opens during `PureCommand` exchange

**Confirmation**: "host-authoritative command: document/config/draft packs travel with the command; guest returns `AppFrame::Emit` ops only (host applies)" (L134-135, CHANNEL_VERSION 5 addition).

### TransactionPrepare: Two Forms (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs:162-175`)

**Owner-mutation form** (initiating mutation discovery):
- `mutation_id` non-empty, `payload` non-empty, `prepared_ops` empty
- Guest uses `mutation_id` to route to the owning artifact, evaluates the mutation, returns ops

**Pre-planned form** (foreign member in 2-phase):
- `mutation_id` empty, `payload` empty, `prepared_ops` non-empty, `label` non-empty, `origin` non-bytes  
- Host PRE-COMPUTED the ops via `plan_contributed` callback (see §3 for full flow)
- Guest validates revision, applies ops to a temporary in-memory store (without durability), replies with empty rejection on success
- This is the FAST PATH for multi-artifact transactions

### Error Codes for Revision Mismatch (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11915-11922`)

**"transaction.generation-mismatch"**: 
- Returned by `transaction_commit` when `pending.base_generation != self.store.generation()`
- Fields in error: message includes `"base generation {base} no longer matches current generation {current}"`
- `base_generation` is captured at `TransactionPrepare` time (L11876 in tests); `self.store.generation()` is the live counter
- Fault code: `FaultOrigin::Plugin`, code string `"transaction.generation-mismatch"` (L11921)

**"transaction.instance-busy"**:
- Returned by `transaction_prepare` when a pending transaction already exists (L2409: fake cluster test; real impl at L11843)
- Returned by `dispatch_emit` if a mutation verb would touch an artifact while transaction is pending (L10753)
- Fault carries message: `"already has a pending transaction"` or verb context

---

## 3. HostTransactionCoordinator — 2-Phase Protocol (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2172-2375`)

### run_transaction Signature (L2200-2210)

```rust
pub fn run_transaction(
    &self,
    instances: &InstanceDirectory,
    mutation_router: &ArtifactMutationRouter,
    mut exchange: impl FnMut(&str, u32, protocol::AppCommand) 
        -> Result<Vec<protocol::AppFrame>, TransactionError>,
    mut plan_contributed: impl FnMut(&str, &str, &str, &TransactionMember, &[u8]) 
        -> Result<HostArtifactMutationPlanResult, TransactionError>,
    initiator: TransactionMember,
    local_ops: Vec<Vec<u8>>,
    description: String,
    foreign: Vec<protocol::ForeignStep>,
) -> Result<TransactionOutcome, TransactionError>
```

### 2-Phase Protocol Steps (L2189-2353)

**Phase 1: Discovery & Prepare (L2227-2308)**

1. **Resolve all foreign steps** (L2224-2265):
   - Iterate frontier of `ForeignStep` structures depth-first (cycle-guarded to `MAX_PLAN_DEPTH`)
   - For each step, resolve `ForeignStep.target.artifact_id` → instance location via `InstanceDirectory::resolve`
   - Query `ArtifactMutationRouter::resolve` to determine if mutation is Owner or Contributed
   - On Owner: accumulate `step.payload` to member's `prepared_ops` list (pre-planned form)
   - On Contributed: call `plan_contributed` callback, extend frontier with plan's `foreign` steps, mark contributor as origin
   - Track discovery order (first discovered member becomes first prepared)

2. **Prepare every member in discovery order** (L2267-2308):
   - For each member, construct `AppCommand::TransactionPrepare` with:
     - **Pre-planned form**: empty `mutation_id`/`payload`, non-empty `prepared_ops` list, encoded `origin` bytes
   - Send via `exchange` callback; wait for `AppFrame::TransactionPrepared`
   - Check `rejection` field: empty means success, non-empty (fault bytes) means this member rejected
   - **On any rejection**: roll back all already-prepared members via `TransactionRollback` (reverse discovery order), return error

**Phase 2: Commit in Reverse Discovery Order (L2317-2349)**

1. **Commit sequence** (L2321-2339):
   - Iterate discovery_order in REVERSE
   - Send `AppCommand::TransactionCommit { txn_id }`
   - Wait for `AppFrame::TransactionCommitted { edit_id }`
   - Collect `edit_id` by member key

2. **On any commit failure** (L2340-2349):
   - **Compensation**: undo already-committed members via `TransactionUndo { group_id = txn_id }`
   - Rollback uncommitted members via `TransactionRollback`
   - Return error

3. **Success**: return `TransactionOutcome { txn_id, members (discovery order), edit_ids }`

### Undo/Redo Fan-Out (L2355-2368)

- `undo_group`: sends `TransactionUndo { group_id }` to every member (best-effort)
- `redo_group`: sends `TransactionRedo { group_id }` to every member (best-effort)

---

## 4. Ops Encoding: document_ops → Vec<op-bytes>

**AppFrame::Emit.document_ops** carries a **single packed binary stream** of all document mutations to apply.

**Decoding path** (path:line not explicitly named; inferred from channel layer):
- The `document_ops: Vec<u8>` field is a varint-length-prefixed op sequence
- Each op is binary-encoded per the artifact's schema (e.g., `OpBinary` trait in `store::pack_rt`)
- There is **NO** separate `decode_ops_vec` / `encode_ops_vec` function exposed at the channel layer
- **Pattern**: ops are encoded at mutation time in the guest, returned as a single byte blob, and the host **interprets the blob as a stream of encoded ops**

**For MCP action adapter**: 
- To construct `TransactionPrepare.prepared_ops` from a mutation:
  - Encode each op individually using the artifact's `OpBinary::encode` (or equivalent `pack_rt::encode_wire_value` for the payload)
  - Collect into `Vec<Vec<u8>>` where each element is one complete op payload
  - This matches the "pre-planned form" expectation: `prepared_ops` is a vec of separate op payloads, not a single stream

**Reference**: `write_vec_bytes` (L386-391) encodes the vec as `varint count | bytes[0] | bytes[1] | ...`

---

## 5. Revision & Identity: Authoritative Revision for External Caller

### HistorySnapshot.history_patch Structure (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs:303`)

**ReadHistory reply** carries `history_patch: Vec<u8>` — a packed `HistoryPatch` struct (decoded by host).

**HistoryPatch fields** (inferred from design-abi.md §1 and store layers):
- `cursor`: current head edit ID
- `head`: commit ID of the latest applied edit
- `checkpoints`: sequence of prior sealed run snapshots
- `alternatives`: divergent edit branches

### Generation Counter Usage (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11876`, `11918`)

- **Captured at prepare time** (L11876, test): `base_generation = self.store.generation()` saved into `PendingTransaction`
- **Checked at commit time** (L11918): reject if `pending.base_generation != self.store.generation()` now
- Increments on every mutation/ingest that changes the store state
- Used to detect concurrent edits (from backbone ingest, other local commands) between prepare and commit

### MutationEnvelope Fields (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔗️causal/🦀️component.rs:34-42`)

```rust
pub struct MutationEnvelope {
    pub mutation_id: MutationId,
    pub document_id: ArtifactId,
    pub actor: ActorId,
    pub dependencies: Vec<MutationId>,
    pub diff: ArtifactDiff,
    pub inverse: InverseMutation,
    pub timestamp: HybridLogicalTimestamp,
}
```

### Recommendation for External MCP Action Adapter

**`expectedRevision` parameter should carry**:
1. **Preferred**: the `edit_id` from the last successfully applied edit (available via `ReadDocument.ops` field or `HistorySnapshot` cursor)
   - Read via `AppCommand::ReadHistory`, extract `HistorySnapshot.history_patch` bytes, decode to get `cursor` (the head edit id)
   - At prepare time, compare returned `Document.ops` string (human-readable operation log or edit id list) to ensure consistency

2. **Fallback**: the `generation` counter itself
   - Read via `Document` frame implicitly (each frame carries implicit generation state in the store metadata)
   - Less intuitive for external callers; generations are internal counters

3. **Do NOT use**: `MutationEnvelope.timestamp` or `MutationId` — these do NOT establish store revision; they are causality markers, not revision numbers

---

## 6. Undo: TransactionUndo vs. Plain Edit Undo

### TransactionUndo for Committed Transaction (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2355-2368`)

- **Sent by**: `HostTransactionCoordinator::undo_group`
- **Command**: `AppCommand::TransactionUndo { seq, group_id }` where `group_id = txn_id`
- **Scope**: fans out to EVERY member that participated in the transaction
- **Effect**: marks the group (identified by `txn_id`) as undone; the guest's store implements group-level undo via `UndoGroup` mechanism
- **Best-effort**: no error return; members whose tail has moved on independently error on their own side

### Plain Edit Undo (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — design-abi.md §4)

- **Command**: `AppCommand::Command { seq, command, view_state }` with command payload `"undo"` or similar action
- **Scope**: single instance; does NOT coordinate across artifacts
- **Effect**: pops the undo stack on this instance only

### UndoPolicy & UndoGroup (path:line `semio_framework::kernel::UndoPolicy`, `UndoGroup`)

- **UndoPolicy**: governs whether consecutive edits coalesce or separate (design-abi.md §4)
- **UndoGroup**: opaque handle returned by `stamp_tail_group_id` at commit time; uniquely identifies a transaction's edits across all members for group undo
- **For transactions**: `group_id` is always `txn_id`; committed edits are stamped with this group id so later `TransactionUndo` can find and toggle them

---

## 7. Conflicts & Reports (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs:331-342`)

### MergeReport (L23) — Pushed Unsolicited

- **Timing**: after every `ApplyEnvelopes` ingest, next to `DocumentChanged`
- **Type**: packed `MergeReport` struct (DSL encoded)
- **Fields** (inferred from design-abi.md contract §C8):
  - `policy_applied`: which `MergePolicy` was used (LaissezFaire/Normal/Vigilant)
  - `resolution_count`: number of conflicting mutations resolved
  - `casualties`: edits that were reordered/simplified due to merge

### Conflict (L24) — Conflict List

- **Timing**: pushed unsolicited after ingest, and reply to `ReadConflicts`
- **Type**: packed `Vec<Conflict>` (DSL encoded)
- **Each Conflict carries**:
  - `conflict_id`: opaque handle for `ResolveConflict { conflict_id, resolution: Accept|Discard }`
  - `our_edit`: the local edit that is in conflict
  - `their_mutation`: the ingested mutation that caused the conflict
  - `merged_ops`: the result of auto-merge (if policy allowed)

### DispatchReport (L256, L270) — On Mutation Rejection

- **Carried by**: `AppFrame::Invocation { messages }` (CHANNEL_VERSION 11 trailing field) OR `AppFrame::Error { report }` when `Fault.code == "mutation.rejected"`
- **Type**: packed `DispatchReport` struct
- **Provides**: diagnostic details about why a dispatch was rejected (e.g., validation failure, permission check, constraint violation)

### MutationMessage (path:line inferred from design-abi.md §C8)

- **Type**: union of report/diagnostic message types
- **Examples**: `InferenceResult`, `MutationPlanResult`, validation error
- **Serialization**: always DSL-encoded `pack_rt::encode_wire_value`

---

## 8. Headless Driving Precedent (path:line `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs:75-89`, L32-34`)

### AppChannelHost Trait (L82-88)

```rust
pub trait AppChannelHost {
    /// artifact_ref (the node's own WorkflowNode.artifact_ref) is threaded through 
    /// so a real host can populate its InstanceDirectory at instantiate-app time
    fn open(&mut self, plugin_id: &str, app_id: &str, artifact_ref: &str) -> Result<u32, RunError>;
    fn exchange(&mut self, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError>;
}
```

**Contract**:
- `open`: mints an opaque handle (typically an instance id in a plugin runtime)
- `exchange`: takes a handle + **batch** of commands, returns frames (single synchronous duplex round-trip, NOT request-reply RPC)

### SpaceRunner / WasmtimeNodeHost (path:line `🏃️run/🦀️component.rs` L1-29)

**Script a node computes** (L7):
```
Hello → LoadConfig → LoadDocument → 
  MediaIn* → (MediaOut+MediaFingerprint)* → 
  ReadDocument → ReadConfig
```

**Driver pattern**:
1. Construct `Hello { channel_version, app_id, actor, config }`
2. `LoadConfig`, `LoadDocument` pass current artifact bytes and SPR metadata
3. Issue `MediaIn` for each input port (converted to artifact descriptor + blob data)
4. Exchange `MediaOut`/`MediaFingerprint` to collect outputs
5. `ReadDocument`, `ReadConfig` retrieve mutated bytes after the node runs
6. Never write back to source (read-only over workflow source; mutations land in `RunSink`)

**Batch exchange semantics**: all commands in one `exchange` call are processed **in order by the guest** in a single reactor turn; replies are returned as a single batch. This is NOT a request-reply loop; it is one bidirectional duplex per turn.

---

## 9. Channel v12 Risk Analysis (path:line `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-abi.md:85-86`, `status.md:14-16`)

### Design-abi.md §2: "exchange collapse" (L85-86)

**Channel v12 removes from AppCommand**:
- `Hello`, `Bye`, `AttachBackbone`, `DetachBackbone`, `RefreshUi`

**Channel v12 removes from AppFrame**:
- `Welcome`, `UiSection`, `Effects`, `Events`

**Channel v12 adds**:
- Revisioned `ui-patch{surface, kind, revision, base_revision, ops: pack}` with `PatchOp::{Replace, InsertChild, RemoveChild, SetProps}`
- Ephemeral keeps its generations

**Implication for action adapter**:
- v11 frame contract (`ReadDocument`, `ReadConfig`, `ReadHistory`, `TransactionPrepare`/`TransactionCommit`/`TransactionUndo`, `ReadConflicts`, `MergeReport`, `Conflicts`) **SURVIVES** v12
- `Hello` is **REMOVED** — v12 session setup differs (not yet designed in this ticket's scope)
- `RefreshUi` is **REMOVED** — v12 uses event-driven patch protocol instead
- **Safe to depend on**: ALL transaction frames, artifact I/O frames, conflict/merge frames

### Status.md §2 Packet Landing Status (L14-16)

- **A3-kernel-types** (dispatched): new `Effect`/`Event`/`UiPatch`/`Budget`/`TurnResult`/`Broker`/`Quota` types + `HostEffect`→`Effect` rename, workspace stays green
- **A4-channel** (queued, not yet landed): v12 frame changes — dispatch together with A2-abi-sdk + B1-host-native to confine red window to W1→G1 gate
- **A2-abi-sdk** and **B1-host-native** (held as of status.md timestamp 21:10): waiting on peer session to complete its work in overlapping files

**Current state (session 26/08/17 LLM-FIRST-OS)**: The transaction protocol frames remain **stable** in v11. Channel v12 is design-frozen but implementation is queued. An MCP action adapter written to v11 will **NOT break** on v12's arrival (v12 only removes UI-specific frames, not transaction frames).

---

## Action Adapter Contract — Exact Frame Sequence

```rust
// === PREVIEW: Dry-run without commit ===

// 1. External process sends PureCommand with current state
send AppCommand::PureCommand {
    seq: <autoincrement>,
    command: <artifact_command_bytes>,
    document: <current_pack>,
    document_spr: <current_spr>,
    config: <current_config_pack>,
    config_spr: <current_config_spr>,
    draft: <current_draft_pack>,
    draft_spr: <current_draft_spr>,
}

// 2. Guest returns what WOULD happen
recv AppFrame::Emit {
    in_reply_to: <seq>,
    document_ops: <op_bytes_to_apply>,
    config_ops: <op_bytes_to_apply>,
    draft_ops: <op_bytes_to_apply>,
    output: <diagnostics>,
    diagnostics: <errors_if_any>,
}

// === PREPARE: Acquire revision ===

// 3. Read current revision BEFORE preparation
send AppCommand::ReadHistory { seq: <autoincrement> }

recv AppFrame::HistorySnapshot {
    in_reply_to: <seq>,
    history_patch: <packed_HistoryPatch>,  // decode to get cursor (edit_id)
}

// === COMMIT: 2-phase transaction with revision check ===

// 4. Phase 1: Prepare (pre-planned form for direct artifact mutation)
send AppCommand::TransactionPrepare {
    seq: <autoincrement>,
    txn_id: <mint_txn_id>,
    mutation_id: "",  // empty: pre-planned form
    payload: vec![],  // empty: pre-planned form
    prepared_ops: vec![
        <document_ops>,  // same bytes from Emit, or new ops if different
        // ... one Vec<u8> per op payload
    ],
    label: <description>,
    origin: <encode_MutationOrigin::Owner>,
}

recv AppFrame::TransactionPrepared {
    txn_id: <txn_id>,
    foreign: vec![],
    rejection: vec![],  // MUST be empty for success
}

// 5. Phase 2: Commit (checks expectedRevision)
send AppCommand::TransactionCommit {
    seq: <autoincrement>,
    txn_id: <txn_id>,
}

recv AppFrame::TransactionCommitted {
    txn_id: <txn_id>,
    edit_id: <the_committed_edit_id>,
}

// === VERIFY: Confirm result ===

// 6. Read updated document to verify
send AppCommand::ReadDocument { seq: <autoincrement> }

recv AppFrame::Document {
    in_reply_to: <seq>,
    pack: <new_pack>,
    spr: <new_spr>,
    ops: <stringified_ops>,  // can cross-check against what was committed
}

// === UNDO: Revert committed transaction ===

// 7. When rolling back, fan undo to all affected members
send AppCommand::TransactionUndo {
    seq: <autoincrement>,
    group_id: <txn_id>,  // same as the transaction id
}

recv AppFrame::Done {
    in_reply_to: <seq>,
}  // best-effort; no error means "undo marked for this member"

// === ERROR HANDLING ===

// On generation-mismatch at commit (concurrent edit between prepare and commit):
recv AppFrame::Error {
    in_reply_to: <seq>,
    fault: <encode_Fault(
        origin=Plugin,
        code="transaction.generation-mismatch",
        message="transaction {txn_id}'s base generation X no longer matches current Y"
    )>,
    report: vec![],  // may carry DispatchReport if mutation.rejected
}
// Action: rollback via TransactionRollback { seq, txn_id }; retry with fresh prepare

// On instance-busy (transaction already pending):
recv AppFrame::Error {
    in_reply_to: <seq>,
    fault: <encode_Fault(
        origin=Plugin,
        code="transaction.instance-busy",
        message="already has a pending transaction"
    )>,
    report: vec![],
}
// Action: wait for pending to complete, then retry
```

### Key Invariants

1. **expectedRevision check**: captured generation at prepare time, verified at commit time; mismatch aborts transaction
2. **Revision identity**: use `HistorySnapshot.history_patch.cursor` (edit_id) or generation counter; do NOT use `MutationEnvelope` timestamp
3. **Ops encoding**: prepared_ops is Vec<Vec<u8>> where each element is one complete op payload (NOT a single stream)
4. **Undo semantics**: TransactionUndo fans to all members; uses same txn_id as group_id
5. **Frame availability in v12**: all transaction/conflict/merge frames survive; only UI frames (Hello, RefreshUi, Welcome, UiSection, Effects, Events) are removed

---

## Files & Artifacts

| Path | Purpose | Line Range |
|------|---------|-----------|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` | Channel codec + AppCommand/AppFrame types | 52-343, 1-20 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` | HostTransactionCoordinator, 2-phase protocol | 2172-2375 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | AppChannelHost dispatch, generation-mismatch rejection | 11915-11922 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` | AppChannelHost trait, headless driver precedent | 75-89, 1-29 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔗️causal/🦀️component.rs` | MutationEnvelope structure, envelope encoding | 34-42, 1-56 |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-abi.md` | Channel v12 design, effects/events, SDK layers | §2, §4 |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️status.md` | Packet A2/A3/A4/B1 landing status | §0-1 |
