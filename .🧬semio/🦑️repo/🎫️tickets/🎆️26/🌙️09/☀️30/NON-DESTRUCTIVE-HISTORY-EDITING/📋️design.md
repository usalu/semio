# 📋️ Non-Destructive History Editing — Binding Design

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Coordinator decisions, 2026-09-30. Evidence: the eight
`📓️explore-*.md` reports in this folder. Every work package (WP) implements exactly this contract; deviations
are raised to the coordinator (SendMessage `main`), never improvised.

## 0. Model in one paragraph

A document is `genesis + HLC-ordered causal log`. Mutations (ops) are immutable and addressed by `MutationId`.
Editing history never rewrites an op: it appends a `Supersede` history transition that replaces the *effective
input* of one or more ops (or withdraws them). The pure fold resolves effective inputs; every fold site folds
*effective forwards*. Editing happens in a local, ephemeral **time-travel session**: the user edits one
mutation's inputs (UI controls derived from schema metadata), sees the document as of that mutation with the
draft applied and downstream not applied, accepts or discards, then the downstream suffix replays in Report mode
and every mutation shows success / warning / error / fatal. Error and Fatal block finalizing; the user edits the
failing mutations (or withdraws them) until the suffix is clean, then finalizes as **overwrite** (unscoped
`Supersede`) or **new alternative** (`Branch` + scoped `Supersede`). Tools are state machines that yield
parametric mutations inside a `ToolTransaction`; history edits the yielded mutations, never the tool.

## 1. Vocabulary (names are binding)

| Name | Meaning |
|---|---|
| `Supersede` | new `HistoryTransition` variant, tag 6. Never call it `Amend` (collides with `AmendLast`). |
| `InputReplacement` | `Input { schema, payload }` (canonical `OpBinary` bytes of the artifact aggregate op) or `Withdrawn` (the op folds as a no-op). |
| effective forwards | `edit.forwards` with every op replaced by its effective supersession (or skipped when withdrawn). |
| Report mode replay | policy-independent, non-quarantining, non-aborting suffix replay producing per-mutation outcomes. |
| `ReplayReport` | per-`MutationId` outcomes (`worst`, `messages`) of a replay. Blocks finalize iff any `Error`/`Fatal`. |
| time-travel session | ephemeral local-only state of an ongoing history edit (framework module `⏪️time-travel`). |
| `ToolTransaction` | one interactive tool gesture/run: yields keyed mutations, commits ONE edit, or aborts with zero trace (framework module `🛠️tool-machine`). |
| `TransactionRef` | `{ id, tool }` stamped on every op a committed tool transaction produced; persisted and on the wire. |
| input descriptor | `ActionArgDef` derived from the leaf payload JSON Schema plus its `x-semio-ui` annotation (one vocabulary for action args and mutation inputs). |

## 2. Replication layer (`🧰️framework/🔨️modules/📡️replication`, crate `semio-framework-replication`, lib `protocol`)

```rust
pub enum HistoryTransition { Revert{..}, Reinstate{..}, Commit(..), Branch{..}, Checkout{..}, Repin{..},
    Supersede(TransitionSupersede) }                                   // tag 6
pub struct TransitionSupersede { pub scope: Option<String>, pub inputs: Vec<SupersededInput> }
pub struct SupersededInput { pub target: MutationId, pub replacement: InputReplacement }
pub enum InputReplacement { Input { schema: String, payload: Vec<u8> }, Withdrawn }
pub struct EffectiveSupersession { pub transition_id: String, pub actor: String,
    pub timestamp: HybridLogicalTimestamp, pub scope: Option<String>, pub replacement: InputReplacement }
// HistoryFold gains: pub supersessions: BTreeMap<MutationId, EffectiveSupersession>
```

Codec: `inputs` non-empty, targets unique within one transition, `scope` ≤ 256 bytes, payload ≤ 256 KiB; malformed
otherwise. Envelope: `dependencies = targets`, `target = []` at protocol level (the store fills the conflict
target from the replacement op), content-addressed id as for every transition.

Fold law: effective supersession of op X = the last (by `(hlc.cmp_key(), id)`) `Supersede` naming X whose `scope`
is `None` or equals the fold's final `alternative`. Unknown target → fold error (same as `Revert`). **No ownership
rule**: any actor with write permission may supersede any op (editing is non-destructive and reversible; the hub
admission/security gate decides write permission). A superseded op keeps its slot, id, HLC and edit; `Revert`,
`Reinstate`, `Commit` keep naming slots and compose independently.

`TransactionRef { id: String, tool: String }` is added to `MutationMeta` and to `MutationEnvelope`
(`transaction: Option<TransactionRef>`), in every codec (Rust binary + TS twin + batch codecs). Tool id format:
the action/tool id that authored the transaction (`<appId>#<toolId>`), transaction id `tx-<hex16>` minted from
`blake3(actor | hlc | tool)`.

Replay report types live in `⚔️conflict`:

```rust
pub struct MutationReplayOutcome { pub mutation_id: MutationId, pub edit_id: String, pub op_index: u32,
    pub worst: Option<Severity>, pub messages: Vec<MutationMessage>, pub superseded: bool, pub withdrawn: bool }
pub struct ReplayReport { pub from_position: u32, pub outcomes: Vec<MutationReplayOutcome>, pub worst: Option<Severity> }
impl ReplayReport { pub fn blocks_finalize(&self) -> bool }   // any Error or Fatal
```

Schema-first: `🧬️schema/🔣️history-transition/🔣️.json` gains `supersede`; corpus gains accepted and malformed
supersede cases; three independent encoders (Python generator in a `🧪️tests/🧪️<case>/🐍️.py`, Rust codec, TS
encoder + Ajv). Fold laws get a language-agnostic step fixture checked by Rust and a TS fold twin.

## 3. Store (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store`, `📡️spr`, `🌿️vcs`; crate `semio-framework-os-kernel`)

1. **Effective forwards everywhere.** One accessor `effective_forwards(edit, &supersessions)` used by every fold
   site: `fold_history` (both), `materialize_document_snapshot`, `project_applied`, the replay engine,
   `parse_decoded_document_spr`, `parse_document_text`, the retained initializer (`BoundedStoreInitializationPhase`
   in `🔌️plugin`), `replay_envelopes_onto_pair`/hub check-in. The original `forwards` are never modified.
2. **`EditReplay` stepper** extracted from `replay_suffix_partitioned`: cursor `(edit, op)`, `step(deadline)`,
   `cancel`, `finish`. Modes: `Merge(policy)` (today's semantics; `replay_suffix_partitioned` becomes a
   drive-to-completion wrapper) and `Report` (keep-and-record: `MutationOutcome::apply_to`, never quarantine,
   never abort, no per-edit `P::clone`, rebased inverses staged, per-mutation outcomes). Convergence early exit:
   when the running state equals the previous prefix state at the same position (`Arc::ptr_eq`, then `PartialEq`),
   splice the previous tail and stop.
3. **Prefix-snapshot ring**: `Arc<P>` states keyed by a forwards-only (effective) prefix digest, adaptive stride,
   retired through the store retirement (never a bare drop). Gives O(stride) "state before op X".
4. **Commands**: `ArtifactCommand::Supersede { scope: Option<String>, inputs: Vec<(MutationId, Option<P::Mutation>)> }`
   (ordinal 17) with ToValue/FromValue, text and binary codecs. Authoring: validate targets are applied, payload
   canonical, op not `may_emit_foreign_steps`; dry-run Report replay; refuse atomically with
   `VcsError::Rejected{policy: Normal, messages}` when the report blocks finalize; else `commit_transition`.
   `ArtifactCommand::CreateAlternativeWithSupersede { name, inputs }` = commit pending → `Branch` at head →
   scoped `Supersede`, atomically (one dispatch, one outbound batch).
5. **`reproject`** detects supersession changes (not only applied-id changes) and replays in Report mode from the
   first affected applied position; writes rebased inverses, `replace_edit_messages`, per-mutation outcomes,
   resets the tail cache, sets `ArtifactProjectionCause::Replay`. Remote supersessions converge through the same
   path.
6. **Read API** for the session: `state_before(mutation_id, drafts) -> Arc<P>`, `begin_report_replay(drafts,
   from) -> EditReplay` (stepper usable by a job), `mutation_outcomes()` (durable per-mutation messages),
   `supersessions()`, `mutation_ops()` (ordered applied ops with ids, edit ids, op indices, TransactionRef).
7. **Revision**: `CursorRevisionAccumulator` digests effective forwards; truncates at the first affected position.
8. **Persistence**: `.spr` `HistoryOpMeta.transaction`; supersede rides `REC_TRANSITION`; `.ops` gets a
   `supersede` line; `HistoryLog::fold` resolves supersessions; persisted messages/inverses are recomputed on load.
9. `MutationMeta.semantic_kind`, `label`, `transaction` are populated by `replay_mutations` and the retained batch
   path (no more defaults).

## 4. Time-travel session (`🧰️framework/🔨️modules/⏪️time-travel`, new crate `semio-framework-time-travel`)

Pure, target-neutral (native, `wasm32-wasip2`, `wasm32-unknown-unknown`), Rust + TS twin, schema-first.

```
Stage = Inactive | Editing | Replaying | Reviewing | Choosing | Finalizing
Session { id, generation, base: BaseStamp{store_generation, content_revision}, stage,
          accepted: ordered map MutationId -> InputReplacement,       // accepted drafts
          pending: Option<{ target, original, replacement, return_stage }>,
          report: Option<ReplayReport>, progress: Option<{done,total}>, fault: Option<code> }
Events: Begin{target, original} | Draft{replacement} | Withdraw | Accept | Discard |
        ReplayProgressed{done,total} | ReplayCompleted{report} | ReplayCancelled | ReplayFaulted{code} |
        RequestFinalize | Choose{Overwrite | Alternative{name}} | Back | Finalized | FinalizeFaulted{code} |
        BaseMoved{base} | Exit
Effects: ShowPreview{target, replacement} | StartReplay{drafts, from} | CancelReplay | OpenFinalizePrompt |
         CommitOverwrite{inputs} | CommitAlternative{name, inputs} | Close
```

Laws: `Begin` is legal from `Inactive` and `Reviewing` (and `Editing` switches target only when the pending draft
equals the original); `Accept` moves the pending draft into `accepted` and starts a replay from the earliest
accepted target (downstream is never applied while `Editing`); `Discard` returns to `return_stage` (`Inactive`
exits time travel when nothing is accepted); `RequestFinalize` needs `Reviewing`, a report that does not block,
and a non-empty `accepted`; `BaseMoved` while `Reviewing`/`Replaying` restarts the replay; `Exit` from any stage
discards everything and closes; stale `generation` events are silent no-ops. Fixtures:
`🧫️fixtures/🧫️lifecycle-law/🔣️.json` (every legal/illegal pair), third-party oracles in TS: `ajv`, `xstate`,
`fast-check` (tool-run conformance pattern).

## 5. Tool machines (`🧰️framework/🔨️modules/🛠️tool-machine`, new crate `semio-framework-tool-machine`)

```
ToolYield<M> = Upsert{key, mutation: M} | Retract{key} | Commit | Abort
ToolTransaction<M> { id, tool, state: Open | Committed | Aborted, entries: ordered (key, M) }
ToolMachine: a `🔄️machine` statechart whose effects are ToolYield<M>; ToolMachineRunner drives events,
             owns at most one open ToolTransaction, returns the committed batch (TransactionRef, Vec<M>).
```

Laws: upsert replaces by key keeping first-insertion order; retract removes; commit of an empty transaction is a
no-op (no edit); abort leaves zero trace; one commit = one edit = one undo step = one history row. Rust + TS twin,
fixture `🧫️fixtures/🧫️transaction-law/🔣️.json`, xstate + fast-check oracles. Tool context lives in window
transient state and is never history-editable.

## 6. Input descriptors (`🛂️manifest` + `🗣️dsl` derive + `🎮️mutation`)

- One vocabulary: `ActionArgDef { id (= JSON pointer of the input), label, description, schema: ArgSchema,
  presentation, required, default, group, order }`. `ArgSchema::Number` gains `snaps`, `snap_source`,
  `soft_min`, `soft_max`, `precision`, `display_unit`, `display_factor`, `scale`; new `ArgSchema::Reference { kind,
  domain, granularity, many, min_items, max_items }` (selection picker); `ArgSchema::Vector { dims, unit, step }`
  replaces `Vec3`; `ArgPresentation` gains `Stepper`, `Dial`, `Segmented`. `ConfigFieldShape`/`CommandFieldSpec`
  are deleted.
- Leaf payload JSON Schemas carry hard bounds in standard keywords and UI facts in `x-semio-ui` (widget, label
  `{en,de}`, description `{en,de}`, step, precision, softMin/softMax, snaps, snapSource `{step}|{config}|{snapshot}`,
  unit, displayUnit, displayFactor, scale, group, order, role `value|target|discriminator`, ref `{kind, domain,
  granularity}`). Registered in the strict vocabulary fixture with a meta-schema.
- Inference when `x-semio-ui` is absent (mirrors `ActionArgDef::control()`); labels resolve from `x-semio-ui.label`,
  else the framework input-label glossary (field name → `{en,de}`), else lint failure. Every locale required.
- `MutationLeaf::PAYLOAD_SCHEMA: &'static str` (derive `include_str!`s `payloadSchema`); `#[derive(Mutations)]`
  generates `payload_schema()`, `payload_value()`, `with_payload_value()` on `SemanticMutation`; the manifest
  reader `mutation_input_defs(schema, resolver) -> Vec<ActionArgDef>` (Rust + TS twin); roster rows carry inputs.
- Lint `schema-mutation-input-ui` (repo test domain) drives the rollout to every leaf.
- Implementation record (authoritative where it refines this section): `📓️api-input-descriptors.md` and
  `📓️w1-d-report.md` §5–§8 (Vector replaces Vec3 with number facets, `Reference.kinds` + `idType`, `Color` presentation,
  discriminated root unions, cycle guard → structured value, hidden stop, collecting audit, `nullable`).

## 7. Plugin runtime (`🔌️plugin/🦀️.rs`, `🎠️kernel`)

- `time_travel: TimeTravelLedger<A>` beside `tool_runs`; reserved actions `historyEditBegin{mutationId}`,
  `historyEditInput{path, value}`, `historyEditUseSelection{path}`, `historyEditWithdraw`, `historyEditAccept`,
  `historyEditDiscard`, `historyEditFinalize`, `historyEditCommit{choice, name?}`, `historyEditExit`,
  `historyEditCancelReplay` (host-driven, never queued behind guest work).
- Render seam: one `render_snapshot_or(committed)` used at every render site; while a session is non-inactive
  the app renders the session preview (Editing: state before target + draft; Reviewing: replayed head).
- Freeze: artifact-lane app commands are refused while a session is non-inactive (`timeTravel.frozen`);
  interaction (selection) and view verbs keep working; remote ingests keep arriving → `BaseMoved`.
- Replay runs as a Migrated latest-wins job driven per reactor turn (≤ 4 ms slices), progress `{done,total}`,
  cancel, stale-generation guard.
- `Emit.transaction` (from a committed `ToolTransaction`) stamps `TransactionRef` on every published op.
- History: rows keyed by transaction (fallback edit id), expandable to mutations; each mutation row carries
  `mutationId`, label, severity, messages, `superseded`, `editable`; one Rust-produced `framework.body.history`
  rendered by both hosts (the React hand-built tab is deleted). `HistoryEntry`/`HistoryPatch` gain these fields and
  `timeTravel{stage, target, done, total, worst, blocking}`.
- Finalize prompt: framework-injected dialog with `choices` (Overwrite — destructive tone; New alternative + name
  field, default name from a localized label) → `historyEditCommit`.

## 8. Puzzle 2d reference (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/...`)

- New parametric leaves: `drag-selection { targets: [id], dx, dy }` (nodes and target regions), `rotate-selection
  { targets, pivotX, pivotY, angle }`, `scale-selection { targets, pivotX, pivotY, factor }`. Diff reads base
  positions (works on any base); missing/locked members → `mutation.partial`; none left → `mutation.target-missing`.
- The select utility is a `ToolMachine`; the board engine emits one gesture record per gesture (gesture id,
  kind, targets, offset/angle/factor, proximity pairs) in ONE dispatch (both coalescers: TS and wgpu Rust).
  The tool yields the parametric leaf plus recorded proximity `connect-handles` in one `ToolTransaction`.
  `translateSelection`, HUD `move`, keyboard nudge and inspector `delta` reuse the same leaves.
- Every puzzle 2d leaf carries complete `x-semio-ui` (en/de labels; dx/dy snap to the grid via `snapSource`).

## 9. Decisions recorded

1. `Supersede` has no ownership rule (hub write permission governs). 2. Error and Fatal block finalize
(= `MergePolicy::Normal` = hub check-in strictness); warnings persist in history. 3. The replay evaluates the
whole suffix (Error/Fatal ops fold as no-ops), so all problems are visible at once. 4. Withdraw is the resolution
for ops whose inputs cannot fix an error. 5. Overwrite applies to every alternative and checkpoint containing the op
(checkpoint ids name slots, not payloads); a new alternative preserves the original. 6. Time travel is never a
`Checkout`. 7. Composite ops with foreign steps are not editable. 8. Out of scope, flagged as separate tasks:
per-viewer alternative head (Branch/Checkout move every replica today), 64-slot ledger compaction, flow's separate VCS.

## 10. Wave 2 wire contract (binding for W2-A producer, W2-B/W2-C consumers, W2-D apps)

Kernel `HistoryEntry` (Rust `🎠️kernel/🦀️.rs` + TS twin + schema), one row per transaction (fallback: edit):

```
HistoryEntry { seq, editId: string, transaction?: { id, tool }, actionId, label: LocalizedLabel, kind, timestamp,
  opLines, opCount, applied, revertible, count, worst?: Severity,
  mutations: HistoryMutationEntry[] }
HistoryMutationEntry { mutationId, position, opIndex, label: LocalizedLabel, worst?: Severity,
  messages: { level, code, message, target: string[], opIndex? }[],
  superseded, withdrawn, editable, pending /* downstream, not applied while Editing */, edited /* has a draft */ }
Severity = "info" | "warning" | "error" | "fatal"
HistoryPatch.timeTravel?: { sessionId, generation, stage: "editing" | "replaying" | "reviewing" | "choosing" |
  "finalizing", target?: mutationId, targetLabel?: LocalizedLabel, done?, total?, worst?: Severity,
  blocking: boolean, fault?: string, acceptedCount }            // absent = inactive
```

`log_generation` bumps on every time-travel stage/progress change, so the patch is always delivered.

Reserved verbs (framework controller, host-driven at the head of `dispatch_action`): `historyEditBegin{mutationId}`,
`historyEditInput{path, value}`, `historyEditUseSelection{path}`, `historyEditWithdraw`, `historyEditAccept`,
`historyEditDiscard`, `historyEditFinalize`, `historyEditCommit{choice?: "overwrite", name?}` (dialog: submit =
new alternative with `name`; destructive choice `overwrite`), `historyEditBack`, `historyEditExit`,
`historyEditCancelReplay`. Refusal codes: `timeTravel.frozen`, `timeTravel.illegal`, `timeTravel.stale`,
`timeTravel.blocked`, `timeTravel.empty`.

Apps: `Emit.transaction: Option<TransactionRef>` and `Emit::commit_transaction(transaction: TransactionRef,
mutations: Vec<M>) -> Emit` (artifact mutations published as ONE edit stamped with the transaction; no
description → the row label comes from the mutations' `MutationKind::label`).

Landed deviations (W2-A): `HistoryEntry.editId` is optional (rows without an edit remain); hosts key rows with
`HistoryEntry::key()` / `historyEntryKey(entry)` = `edit:<editId>` else `seq:<seq>`. `timeTravel.sessionId` is a decimal
string. Every `historyEdit*` verb takes an optional `generation` (absent = live session). Finalize dialog id
`finalizeHistoryEdit`.
Additive (W2-A, 06:20): verb `historyEditRerun{generation?}` (12 verbs total); `HistoryPatch.timeTravel` gains
`review?: noChanges|needsReplay|blocked|ready` and `rerunnable: bool`; invalid alternative names refused
`timeTravel.name-invalid`; replay progress ships `HistoryPatch` on unsolicited UI-progress invocation frames (≥100 ms / ≥5 %).

## 11. Evidence rule (coordinator decision, 07:50)

Every mutation leaf has at least one committed wire witness, validated against its leaf schema by the
`schema mutation-payloads` lint (`unwitnessed` is a strict finding). Witness forms: fixture quintets
(`🦠️mutation/🔣️.json`) or wire-form `🥒️.feature` rows whose `params` are exactly the leaf wire payload, decoded
generically by the adapters (no hand-mapped params→op code). Wrapped phase leaves witness their `Apply` payload
(`#[mutation_leaf(payload = Apply)]`); `Restore` ops are inert and not editable.

Negative witnesses (11:45): a fixture whose outcome is a refusal with `mutation.invariant` must FAIL its leaf schema.
Value ranges are hard bounds in the schema; state-dependent refusals use `target-missing`/`duplicate-id`; payload-intrinsic
cross-field invariants that draft-07 cannot state are declared in the leaf schema's `x-semio-invariant: [{id,
description{en,de}}]` and the refusal outcome names `"invariant": "<id>"`.

Outcome-code vocabulary extension (18:20): the frozen set grows from 7 to 9 generic codes — `mutation.target-referenced`
(Error; the target is still referenced) and `mutation.target-mismatch` (Error; the payload is inconsistent with the
target's current state). No per-plugin codes. Enforced at persistence (`expected_mutation_message_level`), in the
outcome-law gate, the TS twins, i18n (en/de) and the oracles.

## 12. Composed children (coordinator decision, 22:10)

Every mutation in history is editable, including ops that land in an owned composed-child store (flow's
`s.stdio.semio@v1/flow` content child is the first consumer; owner of the change: W3-T-FLOWCAD).

- **Transactions span parent + owned children.** `Emit.transaction` together with `child_emits` is legal; the same
  `TransactionRef` is stamped on every op of the gesture in whichever store it lands (parent and child lanes).
  `tool_transaction_shape_fault` keeps refusing only a coalesced amend and children the app does not own.
- **History lists child-member mutations**, grouped by `TransactionRef`, labelled from the child leaf, keyed by the owning
  store: `HistoryMutationEntry.store` (member id `<slot>/<childId>`; absent = the app's own document store).
- **Time travel on a child mutation** targets that member store (`historyEditBegin{mutationId, store?}`):
  `state_before` / `begin_report_replay` / `commit_finished_replay` run on the child store; after accept/finalize the parent
  re-derives from the replayed child (a stale parent scene never re-mints or retires the live child). Downstream parent
  mutations that depend on the child replay report outcomes the same way.
- **Flow**: relative child leaf `translate-nodes {targets, dx, dy}` in the stdio semio flow subset; flow drag tool machine;
  React host journals node drags as `move` rows like wgpu; no per-tick amend and no whole-snapshot `SetSnapshot` for drags.

## §13 Wave-2 tool decisions (coordinator, from `📓️audit-remaining-tools.md` §6)

1. Continuous controls: ONE `ScrubMachine<M>` in `🛠️tool-machine` (idle → scrubbing; Tick/Commit/Abort{blur|captureLost|frozen|baseMoved|retired};
   Upsert replace-by-key + Commit). The runtime routes any action whose args carry `gesture` (+ `commit`) into a runner persisted in the
   window transient (like puzzle 2d `select_tool`) and emits `Emit::commit_transaction` on commit. The leaf is the ABSOLUTE set
   (`set-x{target, value}`) — for a slider the intent is the value. Plugins only supply the leaf constructor. No plugin-local scrub machines.
2. Typing: a typing run is a tool machine yielding ONE net leaf per commit (prose: net `splice-text`; single buffers: final text;
   structured text: explicit Apply via `TextWindowKit`). Commit on idle (≤ 1 s), caret jump, blur, Enter (single-line), pagehide /
   visibilitychange, explicit apply, any non-typing verb; abort only on baseMoved conflict / frozen time travel. The pending run is
   published as EPHEMERAL SHARED preview (presence) so co-editors see typing live without history micro-mutations. Short single-line
   fields: `commit("blur")` + Enter = one dispatch. `Emit::amend`/`AmendLast` and static coalesce keys are deleted at closure.
3. Node-graph guests: the flow executor defines the gesture record `{gestureId, kind: move, nodeIds, dx, dy}` + composed-child
   transactions (§12); guests implement `move-nodes{ids, dx, dy}` + reuse the node-drag machine.

## §14 Norm path budget (coordinator, from `📓️norm-path-budget.md`)

Option B: mutation evidence case directories are ONE emoji + a short slug (≤ ~12 bytes, still descriptive: `🧪️applies`,
`🚫️refuses`, `↩️inverse`, …), norm-wide; the taxonomy tree (standards/subsets segments, bundle layout, U+FE0F) stays
unchanged. Kinds that still cannot fit get precise shorter kind names (all references at once; drop redundant words before
using abbreviations; abbreviations only if they are the standard's own notation). The repo-wide over-budget census outside norm
(energy, stdio, architect, …) is out of this ticket's scope and handed off separately.

## §15 Streamed-ingest transactions (coordinator, from W3-T2-STROKES remodel analysis)

Large streamed tools (remodel video/image-sequence import: up to 200 frames, 40–80 MB) cannot hold their yield until commit
(1 MiB transient bound, 256 KiB per dispatch) and cannot be N edits (64-slot ledger). Mechanism: TRANSACTION-SCOPED AMEND —
while a tool transaction is open, the store appends each tick's ops into the open edit carrying the same `TransactionRef`
(every appended op stamped with it; keyed by transaction id, never by a static coalesce key); commit closes the edit; abort
reverts the open edit (zero trace). The plugin runtime admits that shape (`tool_transaction_shape_fault` refines, not lifts).
This is also the replacement for the static-key `Emit::amend` path that W3-T2-CLOSURE deletes. Owner: W1-G (store + runtime
shape rule, Rust + TS twin, laws); W3-T2-STROKES converts remodel on top.

## §16 Session-2 decisions (coordinator, 2026-10-01, from `📓️resume-gap.md`)

1. **Blocking rule confirmed (G14).** The goal reads "the process is repeated until all downstream mutations are error
   free": Error AND Fatal block finalize, Warnings never do (unchanged `ReplayReport::blocks_finalize`, hub check-in
   strictness `MergePolicy::Normal`). UI copy names the rule ("Errors must be fixed or withdrawn before finalizing").
2. **Generic labels (G7).** `ArtifactApp::mutation_label` defaults to `SemanticMutation::label` (en/de); per-app overrides
   are deleted; a repo gate fails when any applied leaf resolves to the `print_op` fallback or lacks a `de` label.
3. **Editable-everything gate (G8) + cross-plugin harness (G12).** Every `Mutation` aggregate has an input schema and a
   generic payload round trip, or a declared + tested non-editable reason (`may_emit_foreign_steps`, inert `Restore`);
   one generic runtime law per plugin runs begin → input → accept → finalize (overwrite + alternative) on a
   representative editable leaf and compares the head with a fresh fold of the edited log.
4. **Fatal loop by editing (G3).** Resolution by editing targets ("Use selection") is a first-class path next to Withdraw;
   chips show entity labels (not raw ids) and the preview highlights the referenced entities.
5. **Warnings introduced by an edit (G4).** `HistoryMutationEntry.introduced: bool` marks outcomes that are new relative
   to the pre-edit outcome of the same mutation; warnings persist through finalize and reload.
6. **Long-history share (G9).** `dry_run` and `reprojection_replay` become resumable jobs with progress + cancel; until the
   paged ledger (separate ticket) lands, the 65th edit is refused with a localized notice naming the ceiling.

## §17 Session-2 tool decisions (coordinator, 2026-10-01, from `📓️resume-tools.md` §8)

1. **Forms:** one generic `change-block-field {blockId, field, value}` leaf (typed value union, hard bounds in the schema,
   full `x-semio-ui`) replaces absolute `replace-block` for field edits; `replace-block` survives only where a whole block is
   genuinely replaced by intent.
2. **Raster:** parametric `paint-stroke {layer, tool, brush {size, hardness, opacity, color}, points[]}` with ONE deterministic
   Rust rasterizer shared by both hosts (wasm) — history edits brush and points; replay re-rasterizes; PNG blobs are no
   longer the mutation payload. Preview from the window transient during the stroke.
3. **Typing runs:** accepted as designed (§13.2): commit on idle ≤ 1 s etc.; peers see the pending run via presence only.
4. **CAD:** streamed gumball through a CAD window transient (stream/commit/abort like puzzle 2d); `CadEngagementScratch`
   moves from window config into the window transient (per-frame state never lives in config).
5. **Config lane at closure:** `amend_config` stays ONLY for pure view/config state that never appears as a history row
   (camera, playback cursor, viewer camera); every artifact-lane `Emit::amend`, artifact `coalesce_key`, `AmendLast*` and
   `UtilityPreviewContract` is deleted (CLOSURE).
6. **Lowpoly:** relative leaves `move-selection` / `rotate-selection` / `scale-selection` (approved verbs, same shape as puzzle
   2d / fem); `transform-mesh` is not used.
7. **Fold footprint:** one framework helper derives a leaf's fold-footprint declaration from its inverse length, removing the
   under-declared-footprint hazard class (owner: CLOSURE, with a law over every plugin).

## §18 Number-control keyboard law (coordinator, 2026-10-01)

Slider, dial, stepper and vector axes share one law (UI contract corpus, Rust + TS + both shells): Arrow keys = one step
(facet `step`, else 10^-precision in display units) then snap within tolerance; PageUp/PageDown = next/previous detent when
`snaps` exist, else ±10 steps; Home/End = hard min/max; decimal round-half-away-from-zero at `precision` in display units
(`displayFactor`), converted back without float noise.

## §19 Session-3 decisions (coordinator, 2026-10-02)

1. **Transaction row label = the transaction's declared intent leaf.** A tool transaction may carry structural support
   leaves before its intent (generation3d's first gumball: `create-widget` + synapse splice, then `drag-transforms`).
   `ArtifactApp::tool_intent_kinds(tool: &str) -> &'static [&'static str]` (default empty) declares, per tool id
   (`TransactionRef.tool`), the mutation kinds that name the gesture; the history row is labelled by the FIRST op of the
   transaction whose kind is declared, else by the first op (today's rule). No wire change: tool id and op kinds are already
   persisted, so the label survives reload. Neither "first leaf" nor "last leaf" positional conventions (puzzle 2d's drag is
   first, its `connect-handles` follow). Owner: S3-W2A (runtime), adopters declare (S3-PROCEDURAL first).
2. **Operator inputs are edited as typed absolute leaves.** generation3d `change-widget-input {id, channel, value}` with
   `value` a discriminated root union (`type`: number | text | boolean | point | vector), hard bounds and full `x-semio-ui`
   per variant (same pattern as forms `change-block-field`, §17.1). P8 (operator input field, commit on blur) commits it;
   P9 mesh edits insert the operator with DEFAULT params and append one `change-widget-input` per user-set channel (no
   duplicated params inside the inserted record), so time travel edits "extrude distance = 0.1" directly.
3. **Static records are edited per field.** A whole-record leaf committed by an inspector press masks a history edit of an earlier
   press of another field (energy `update-site` / `update-ground-temperature` / `update-run-period`). Static records get field-granular
   absolute leaves (`change-site-<field>`, …); a field value inconsistent with the record's CURRENT other fields refuses with
   `mutation.target-mismatch` (Error). Whole-record leaves survive only for genuine whole-record intents (import). Dynamic,
   user-defined fields keep the generic typed-union leaf (forms `change-block-field`, §17.1).

## §20 Closure decisions (coordinator, 2026-10-02, from `📓️s3-closure-census.md`)

1. **No amend on any lane (D1).** Continuous view/config inputs (camera, playback cursor, viewer camera, engagement) stream
   through the WINDOW TRANSIENT while the gesture runs and commit ONE config edit when it ends (release, pause, seek end, blur) —
   the same scrub/tool-machine pattern as artifact gestures, on the config lane. `ArtifactCommand::AmendLast`/`AmendLastInLane`,
   `Emit::amend`, `Emit::amend_config`, `Emit.coalesce_key`, `Edit.coalesce_key` (wire, digest, `.spr`, `.ops`, channel, manifest)
   and their codecs are deleted; fixtures re-sealed (greenfield, no compat). Config-lane edits never appear as history rows (L4).
   Playback advancing per frame is transient state; the cursor persists on pause/stop/seek end.
2. **reasoning/wires drag (D2)** → S3-GRAPHS: node-drag machine + relative `move-nodes`.
3. **Generic snapshot editors (D3)** commit a path-scoped patch leaf (`snapshot_edit_patch`: JSON pointer + value whose input
   schema is the snapshot sub-schema at that pointer, so the time-travel editor renders the right control) instead of a
   whole-snapshot `SetSnapshot`; domain leaves are used wherever they exist. `snapshot_edit_set_snapshot` survives only for genuine
   whole-document replacement intents (import / revert to file). Owner: S3-STDIO (non-text), S3-TEXT (text), S3-FLOWCAD (flow).
4. **Hand-written commit labels (D4)** (`Emit::commit(mutations, "label")`) are deleted: labels come from `SemanticMutation::label`
   (G7); the G7 gate fails on any literal commit label. Owners: S3-DRAW (10), S3-NORM (2), S3-SPATIAL (1); gate: S3-AGNOSTIC.
5. **Derived fold footprint (§17.7)** schema-first: the leaf schema declares `x-semio-inverse-rows` (`fixed:N | perTarget:<field> |
   bounded:<const>`), the mutation derive emits `inverse_rows()`, `ArtifactStoreOneItemFootprint::for_leaf` replaces every hand
   declaration; a hand-written declaration is a gate failure. Owner: S3-CLOSURE.
6. **No hand-written emission labels at all.** `Emit { description }` literals are the same G7 breach as `Emit::commit(…, label)`
   (a non-localized string becomes the row label; 220 sites at 12:3x, mostly stdio `SetActiveExample` emits and per-editor commit
   helpers). Every row label comes from the leaves (`SemanticMutation::label`, en/de) or, for runtime verbs (load example), from a
   framework-localized label. Once the sites are gone, `Emit.description` and the label parameter of `Emit::commit` are DELETED from
   the runtime (S3-CLOSURE), so the breach cannot be written again; gate class `labelHandwritten` (S3-AGNOSTIC).
4. **§19.2 applies to every inserted operator.** Whenever a gesture inserts an operator/widget record (generation3d component
   gumball's first grab included), the record is inserted with DEFAULT params and each user-set channel (mode, selection/targets,
   pivot, values) is its own appended `change-widget-input`, so every channel stays individually editable in history.
7. **Guest↔host frame layout changes ride ONE channel bump.** Deleting `AppFrame::TransactionProposal.coalesce_key` (frame tag 15)
   changes the frame layout; it lands with an `APP_CHANNEL_VERSION` bump, the TS twin, the regenerated frame-worker and a rebuild +
   describe of every plugin component, in one final coordinator wave (a silently misread frame is never acceptable). Until that
   wave the field is sent empty and never read.
8. **Pure commands are head-only.** A pure (stateless) command runs against a HEAD-snapshot pack with an empty history — never a
   fold of `.spr` history; verbs that need history refuse with a named localized code and require a live instance. Document loads
   ride the history wire (`HistoryPatch.reprojection {kind: remote|step|load}`); `historyEditCancelReplay` cancels a live load, else a
   deferred step, else pauses a remote change.
9. **Folds are pure over the snapshot; inserted records are self-describing.** A fold never reads a runtime registry (installed
   contributions differ per replica). An inserted operator/widget materializes every declared input's default literal into its
   record at authoring time, so validity (unknown channel → `mutation.target-missing`, wrong literal type/wired → `target-mismatch`)
   is decided from the snapshot alone and the same history folds identically on every replica.
10. **A wire shadows the recorded literal.** With inserted records self-describing (§20.9), evaluation precedence per input port is
    wire > recorded literal > declared default: the engine drops from a neuron's `params` every key that is the `to_port` of an
    incoming synapse before merging (one helper at the compute merge sites). A literal set in history stays recorded and becomes
    effective again when the wire is removed.
11. **Snapshot-sourced options.** `x-semio-ui.optionSource: {snapshot: "<pointer template over the payload>"}` (mirroring
    `snapSource`) makes a select's options the keys found at that pointer in the previewed snapshot (state before the edited
    mutation), e.g. `change-widget-input.channel` over `/host_snapshot/widgets/{id}/params`. Schema-first in the `InputUi`
    meta-schema; readers (Rust + TS), the time-travel panel and both renderers resolve it; the fold still refuses unknown keys.
12. **App fault notices are localized like framework notices.** A guest refusal reaches the user through its `code`: apps declare
    `ArtifactApp::fault_notices()` (`code → LocalizedLabel`, every locale, structured placeholders), published in the descriptor;
    both shells resolve framework codes first, then the app table. A code without labels is a gate failure, never a raw-code UI.
13. **L4 holds: config-lane edits are never history rows.** View/config state (camera, playback cursor, viewer settings, window
    config) is persisted local configuration, not artifact history: the history projection never lists a config-lane edit (not as
    a row, not as a mutation), undo/redo of the artifact never steps over them, and the law observes ALL rows (no `edit_id` filter).
14. **Budgets are wall-time.** Every stepped replay/reprojection/load runs against the reactor's per-turn wall budget (≈ 4 ms), with an
    operation cap only as a secondary bound; per-mutation and per-render work is O(change), never O(history) or O(document).
15. **Composed content is edited only on the child lane (§12 completed).** A composed parent's own (parent-lane) leaves never read
    the owned child's content (`ArtifactChild::local_owner`); every content edit is a child-lane leaf in the child's store; readers
    compose parent + child on read. A decoded/reloaded/remote parent therefore needs no materialization step, and history replay of
    either lane never depends on the other being loaded. Gate: no parent-lane leaf of a composed parent reads `local_owner`. Law per
    composed plugin: save → fresh load renders and folds identically (incl. time travel over the reloaded document).

## §21 Session-4 decisions (coordinator, 2026-10-04)

1. **Tool runs may target an owned composed-child member.** `⏯️tool-run` is generic over its target (parent | owned member): the run's
   ledger is typed by the target's mutation vocabulary, provisional ops overlay the composed read (§20.15 compose on read), stepped slices
   report progress and cancel with zero trace, and finalize publishes ONE edit in the target store stamped with the run's `TransactionRef`
   (child targets via the ChildEmit path, §12). No composed app downgrades a stepped tool run to a plain command job. Owner: S4-WIRES-MATH
   (mechanism + wires Reorganize); S4-GRAPHS converts dag onto it.
2. Docstring-emoji uniqueness is enforced per file for every file this ticket touched; no repo-wide sweep (S4-GATES).
3. Compat pins are deleted, not kept: `optional_field_rows_keep_their_pre_migration_bytes` (flow) is removed or re-sealed with a reason (S4-FLOWCAD).
4. **Registry generation degrades per plugin; release gates stay strict.** Since the guest↔host channel handshake (`admit_guest_channel_version`,
   `plugin.channel-mismatch`) refuses a mismatched component at instantiation, the catalog descriptor const is no longer the only version gate.
   Dev `plugin-registry:generate` therefore EXCLUDES every committed descriptor whose `executionProtocol.appChannelVersion` differs from the host's
   (`stale-channel` diagnostic per plugin in the generated catalog + console summary; the plugin is simply not offered until re-described),
   instead of failing the whole registry. `plugin-registry:check`, `verify-staged` and the trusted-catalog preflight/publish still refuse ANY stale
   descriptor. Law: a fixture registry with one stale descriptor generates without it (diagnostic present) and `check` fails on it. Owner: S4-INFRA.
5. Kit-wide window-transient partitions use `semio_framework_plugin::transient_root!` / `window_transient_owners!` (K3, S4-RUNTIME); S4-RUNTIME
   converts the hand-written whole-root transients in flow, cad, fem, lowpoly, remodel, raster, wfc, layout, draw, forms itself (one invocation
   each, wire `{"kind":"snapshot","transient":…}`), compile-atomic per crate, owners informed.

6. **§21.6 Whole-document carrier is a stepped export job (S4-LOAD F11, 2026-10-04).** `artifact:out` never encodes O(document) inside one `MediaOut` reply: it is produced by the framework-owned stepped media-export job (submit → poll with progress → take chunks → cancel), the size admitted by the same bound the load side enforces and refused with the localized `plugin.document-load.too-large`/`too-many-members` codes. `composed_artifact_media` and its `export_media` override are deleted; the single carrier is the full recursive archive with the parent `.spr`.

7. **§21.7 Playbook content child is the forms artifact (S4-TOOLS-B F2, 2026-10-04).** Playbook steps and blocks live in a composed forms child whose field-granular leaves (`change-block-field`, block insert/remove/move) carry typed `x-semio-ui`; the whole-list `blocksJson` set leaves the vocabulary. Nested composition (forms itself composing stdio children) is admitted by the generic recursive archive; no playbook-specific path.

8. **§21.8 Uninhabited parents carry no diff facet (S4-WIRES-MATH F13, 2026-10-04).** `ArtifactSchemaDescriptor` admits an absent diff facet exactly when the parent mutation aggregate is uninhabited; every §20.15 parent (wires, dag, sequence, flow, imperative) deletes its `🔺️diff` schema and codecs instead of keeping an empty placeholder.

9. **§21.9 Composition is recursive (S4-TOOLS-B F2 admission finding, 2026-10-04).** A composed member may itself compose children, including derived ones; every live maintenance path recurses over member projections with the member as `owner.parent`: boot genesis, derived-child follow (after every child-lane publication, not only root generations), archive-load genesis, replacement genesis and `open_child`. Members carry their own derivation authority (`MemberFactory::genesis_child_pack(member_snapshot, slot, child_id)` generated by `space_members!` from the member artifact's app). `ChildMemberRegistry`, `ChildContentView` and the per-member lanes are keyed by an owner PATH, never by `(slot, child_id)`. Laws: a two-level composed fixture through boot, child-lane edit + re-mint, save → fresh load, member history edit and time travel over the reloaded document. Playbook F2 (§21.7) lands on top of it; no interim flow-node encoding.

## §22 Session-5 decisions (coordinator, 2026-10-05, from `📓️audit-s5-goal.md`, `📓️audit-s5-parity.md`, `📓️s5-gates-census.md`)

1. **Withdraw is a row action on every applied mutation (goal gap 4).** Withdrawing needs no input schema and no editor: the history
   body offers `Withdraw` on every applied mutation row the actor may author on (viewer role and busy stages still refuse, naming why).
   It begins a session when none is open (`Begin` with a `Withdrawn` draft) or stacks into the open review, exactly like the editor's
   Withdraw. A blocking mutation that is not editable (foreign steps, no input schema) is therefore always resolvable. Owner: S5-RUNTIME.
2. **"Next problem" is band vocabulary.** The time-travel wire status carries `nextProblem` (the `MutationId` + store of the first
   blocking outcome, absent when the review does not block); the shared band corpus (`🧫️time-travel-band`) lists a `nextProblem`
   control in `reviewing` while blocked, and both hosts render it (dispatching `historyEditBegin`). Hosts never compute it. A replay that
   completes blocked reveals and scrolls to the first blocking row. Owners: S5-RUNTIME (status + guest), S5-UI (corpus + React), S5-WGPU.
3. **The viewed alternative is per-replica state (goal gap 6; supersedes §9.8 "out of scope").** `Branch` stays a shared history
   transition (the alternative exists for everyone). Which alternative a replica shows is persisted local-only: `Checkout` never moves
   another replica's head, and finalizing as a new alternative moves only the author's head. Remote replicas see the new alternative
   listed and stay where they are. Laws: two replicas, A finalizes a new alternative → B's head and view are unchanged, B lists the
   alternative, B can check it out locally; reload restores each replica's own viewed alternative. Owner: S5-STORE (store/vcs/spr/wire),
   S5-RUNTIME (history rows + alternatives switcher), S5-CHANNEL if the wire layout changes (rides the wave-B bump).
4. **Wave B lands with channel 22.** The persisted/wire `Edit` layout change bumps `CHANNEL_VERSION` to 22 in the same wave; every later
   activation re-describes puzzle anyway (68 descriptors are withheld at channel 20 today). Order: Run 5 on the current build first,
   then wave B in the first landing window. Owner: S5-CHANNEL.
5. **One inverse failure is one mutation's fatal, not a faulted session.** `EditReplay::replay_operation` records `InverseRefused` as a
   `Fatal` outcome on that mutation (`mutation.inverse-refused`, localized) and keeps replaying in Report mode; the session stays
   repairable through that row. Owner: S5-STORE.
6. **No panics on the user-input path.** The two guest `.expect(` calls of the time-travel runtime become refusals with localized codes
   (`timeTravel.schema-unavailable`, `timeTravel.editor-closed`). `edit_is_local` stops counting unauthored edits as local (no legacy
   edits exist; an unauthored edit is refused at admission). Owners: S5-RUNTIME, S5-STORE.
7. **Multiline text is a control kind.** `ArgPresentation` gains `Multiline`; `x-semio-ui.widget: "multiline"` maps to it in the manifest
   reader (Rust + TS twin) and in `time_travel_input_row`; both renderers render a multi-line text control with the number-control
   keyboard law's text counterpart (Enter inserts a line, Ctrl/Cmd+Enter commits, Escape reverts). `widget: "dictionary"` (layout
   `change-data-fields`) is replaced by declared vocabulary (a keyed list of text inputs); no new widget name outside the strict
   vocabulary fixture. Segmented and IconSelect render as themselves in the editor (no fallback to select/text). Owners: S5-UI
   (contract, manifest, React), S5-WGPU, S5-RUNTIME (guest row), S5-TOOLS (layout leaf).
8. **The input gate measures declarations, not inference.** `schema mutation-inputs` reports `declared` vs `inferred` separately and
   fails an interactive numeric input (role `value`, widget slider/stepper/dial) that declares neither `step` nor `snaps`/`snapSource`,
   and any input whose label resolves only through the glossary in one locale. Bare stdio inputs get declared `x-semio-ui` from their
   leaf owners. Owners: S5-GATES (gate), S5-TEXT-STDIO (stdio leaves), other owners per the gate's routing list.
9. **Board gestures are a framework contract, not puzzle constants (goal gap 8).** The transient / flush-now classification of board
   events lives in the shared `board-event-coalescing` schema per event kind; React `Board2dHost` and wgpu `EngineCanvas` read it from the
   schema-generated table (no `PUZZLE2D_*` names, no puzzle payload types in the framework renderers); drag payload parsing is declared by
   the app's descriptor. A second producer proves it: block 2d emits `gesture` rows through the same host. Owner: S5-PUZZLE (board
   engine + both hosts' board regions), S5-TOOLS (block 2d producer).
10. **Frozen and gesture persistence are framework-owned (goal gap 7).** The plugin runtime maps `HostEvent::TimeTravelFrozen` to
    `ToolAbortReason::Frozen` for every open tool machine of the window (no per-editor arm), and offers ONE window-transient slot for a
    persisted `GestureTool` (`GestureTool::persist/resume` wire owned by the framework); the hand-written gesture wrappers (layout
    transform, note ink, draw canvas pointer, fem 3d gumball, puzzle 2d `transform_selection`, and the others the census finds) move
    onto `drive_gesture`. Owner: S5-TOOLS (framework + its plugins), owners of the other plugins adopt.
11. **Replay cost is O(change) per repair iteration (goal gap 9).** The session caches the live digest chain (no two O(n) digest
    chains per draft keystroke) and a repair iteration resumes from the previous report's longest unaffected prefix instead of the
    first accepted target; `locate_mutation`/`mutation_ops` use the existing id index. Laws pin op-count budgets on a 100 k-op log.
    Owner: S5-STORE (after decisions 3 and 5).
12. **Shared-tree landings are serialized by the fleet locks** (`🔐️lock.sh`, rules 51–52): the live tree is green between waves and the
    coordinator activates from it; the hub-puzzle wasip2 canary (S5-INFRA) is the activation gate.
13. **A recorded proximity connect states its precondition (goal gap 13).** The `connect-handles` a drag records from proximity carries
    the tolerance it was recorded under; on replay, when the two handles are no longer within that tolerance (the drag's offset or
    targets were edited), the mutation still connects and reports `Warning` `mutation.precondition-drifted` (localized, naming both
    handles) — the user sees it in the history as a new warning and may withdraw the connect. The drag never re-runs the interactive
    proximity search (tools are not replayed by history). Owner: S5-PUZZLE.
14. **Correction to 22.3 (S5-STORE, read from code):** the per-replica viewed alternative is already on disk (ticket
    `26/10/01/PER-VIEWER-ALTERNATIVE-HEAD`): `Branch` only registers, `Checkout` is validate-only and never authored, the head persists
    local-only in `.spr` `REC_VIEWER`. Audit G9.1 and §9.8 are stale. 22.3 becomes the PROOF (two-replica corpus + schema + independent
    TS model + Rust law incl. `.spr` reload and shuffled arrival) and the deletion of the dead `HistoryTransition::Checkout` (tag 4) as
    its own wave.
15. **Withdrawing a foreign-step operation withdraws its unit (S5-RUNTIME finding, S5-STORE design).** Foreign steps are materialized at
    authoring as one edit per member store stamped `MutationMeta.group_id`; the unit is every applied op with that group id.
    `admit_replacement` admits `Withdrawn` for every op; `Input` on a unit op is refused except its own recorded input (the undo of a
    withdrawal); a supersede naming a unit op names the store's whole unit. `group_id` travels on the wire (`MutationEnvelope`, wave B /
    channel 22). Composed members of one instance withdraw through `CompositionCoordinator::withdraw_group`. Cross-instance units are a
    recorded limit this session (no product calls `call_foreign`): refused with a localized code, pinned by a law.
16. **An accepted draft is individually revertible.** Reducer event `Restore{target}` (legal in `Reviewing`; drops the target's accepted
    draft, restarts the replay, or closes with zero trace when it was the only one), verb `historyEditRestore{mutationId, store?}`, row
    action "Restore" / "Wiederherstellen" on every row whose mutation has an accepted draft in the open session. Row actions of a
    mutation row: `[Edit?, Withdraw? | Restore?]`. Owner: S5-RUNTIME (fixture + TS twin first), hosts follow the row actions.
17. **Outcome codes gain two members in one wave (S5-PUZZLE):** `mutation.precondition-drifted` (Warning) and `mutation.inverse-refused`
    (Fatal), in the closed framework vocabulary (replication `OutcomeCode`, fixtures, schemas, TS twin, guest words, both hosts' labels).
18. **Band copy has one source.** `🧫️time-travel-band/🔣️.json` gains a `labels` table (keys = the React i18n keys, `normal`/`beginner` ×
    en/de, single-brace placeholders; `refusals[]` rows name their label key, control rows their `controlId`). React's catalogue is pinned
    to it by test; the wgpu shell embeds it (no band text in Rust). Host chrome says "History editing" / "Verlaufsbearbeitung"
    (the kernel's term), never "Time travel". Owners: S5-UI (table), S5-WGPU.
19. **Descriptor packs are canonical by the descriptor layer (S5-CHANNEL finding).** The general value encoder follows the pack owner's
    contract (schema map keys canonical, intrinsic objects keep authored order). `describe` canonicalizes the descriptor value (object
    keys sorted by key bytes, recursively) before encoding and pins it with a law against the TS canonical re-encode, so descriptor
    emission is indifferent to either value twin's order policy. No consolidation of the kernel/framework value twins by this ticket.
20. **A leaf is editable or declared withdraw-only (S5-GATES finding: 44 leaves open an editor with zero rows).** A leaf descriptor may
    declare `"editable": false`; such a leaf derives no input schema, its history row offers Withdraw/Restore only, the inputs gate does
    not judge it and the editability gate reports it `inert`; the source census checks descriptor ⇔ Rust consistency. An editable leaf
    shows at least one input row: the gate rule `inputless` fails the rest, and the runtime's `editable` flag requires a visible input
    row. `hidden` is "an input without a row", never "not an input". Whole-document snapshot leaves are withdraw-only.
21. **A bound folder always holds the document (S5-LOAD/S5-STORE finding).** Binding an existing document to an empty folder persists the
    archive at once; a replica that only receives persists (coalesced) after an ingested batch; the folder actor reports
    `documentArchiveAbsent` positively. Owners: S5-LOAD (route + both shells' persistence hunks), S5-STORE (actor event).
22. **A folder read-back merges; it never replaces the document a replica shows (S5-STORE finding).** Today a read-back of another
    writer's bytes is a whole-document replacement: the receiver's own events are dropped, it adopts the writer's viewer head and an
    open time-travel session ends. `ArtifactStore::merge_persisted_pair(pack, spr) -> PairMerge { merged, ahead }` ingests what the store
    lacks exactly like remote events and never reads the pair's viewer head; only a different document or a cold program loads the
    archive. Persist iff the replica's log holds events the folder lacks (`ahead > 0` after a merge; always, coalesced, after a hub
    ingest). Owners: S5-STORE (primitive + two-replica law), S5-LOAD (route + shell policy).
23. **Correction to 20:** the derive already reads the leaf descriptor strictly, so the descriptor is the single source: the optional key
    `editable` (default true) makes the derive emit `input_schema() -> None` and drop the leaf from `INPUT_SCHEMAS`; no Rust-side
    consistency census. Owner: S5-GATES (descriptor schema + derive region + gates), S5-AGNOSTIC (`inert`), S5-RUNTIME (visible-row rule).
24. **Two peers on one folder must converge while one edits history (live fault F4, 2026-10-05 04:46).** Acceptance sequence: A attaches
    a folder and drags; B attaches the same folder; A begins a history edit; B drags another node. Required: A ingests B's edit as a
    remote event (22.22 merge), the open session sees a base move, B's edit is listed downstream "Not applied while editing" with the
    "Remote history change paused" status, A's next input is not answered `stale`, Accept replays B's edit too, A's finalize is
    persisted to the folder (`ahead > 0`), B merges it, both show the same document and rows. Owners: S5-STORE (merge primitive +
    two-replica law), S5-LOAD (merge route, persist after every local publication incl. a finalize), S5-RUNTIME (session half),
    S5-CHANNEL (wire: `MergeDocumentArchive`, `ahead`), S5-E2E (probe batch H is the live proof).
25. **Refinement of 24 (S5-RUNTIME, S5-LOAD, S5-STORE triage of F4).** An open session never pauses remote adoption: a remote tail edit is
    adopted at ingest and listed "Not applied while editing"; "Replaying a remote history change: n of m" shows for a remote change that
    needs a replay (interior by HLC, a remote supersede/undo); "paused" appears only after the user cancelled such a replay. A base move
    while `Editing` keeps the session generation (the editor's controls stay valid). Root causes of F4: (store) a fetched pair has no
    store entry but whole replacement → `merge_persisted_pair`; (route) a rejected read-back load detaches the document port for good,
    so nothing published afterwards reaches the folder → `replaceAttachedDocumentV1` keeps the document attached and bound, then the
    merge route (first read after an attach loads, every later read-back merges); (session) pending rows are positioned from the
    session's re-resolved targets after a base move.
26. **A history-editing session watches content, not generations (F4 follow-up).** The session base is the content revision alone;
    a store generation that moves without any event (a document-port attach/detach) is no base move, and a finished replay is refused
    `Stale` only when the content revision moved. Owners: S5-RUNTIME (`TimeTravelBase`, Rust + TS + corpus + plugin law), S5-STORE (RB).
27. **A stepped load reports and clears itself (F5/F6).** The host folds its own load progress into the shell's reprojection status
    (`kind: load`) and clears it on settle, failure and cancel (S5-UI); the guest's load machine retires a cancelled or failed load in
    its own turns, without host polls, and reports real progress units (S5-LOAD).
28. **The content revision is a pure function of (log, head) (S5-STORE finding, 2026-10-05 07:30).** `sequence_number`, a replica-local
    ordinal assigned by applied position (live) or ledger position (reload), leaves the edit's revision digest; until then the remote
    ingest re-digests from the first renumbered position (`ingest-renumber-dirt`). Owners: S5-STORE (store + laws), S5-CHANNEL
    (canonical-edit schema, twin, reseal; a channel bump if a persisted or wire byte changes).
29. **Gesture press identity is framework state.** `GestureLedger` keeps the opened and closed host press ids per slot: a tick of a
    press the slot ended elsewhere is dropped with zero trace, a tick of another press interrupts the open one first. No per-plugin
    press memory (raster adopts after S5-TOOLS lands it).
30. **A composed member is keyed by its owner edge and addressed by its owner path (corrects §21.9's "keyed by an owner PATH";
    S5-NESTED, read from the candidate ladder).** Archives admit members unordered, so a path cannot be computed when a member
    enters the registry: the registry / content-root key is `MemberKey { owner: artifact id of the owning member | "" = the
    document, slot, child_id }` (= the persisted `OwnerRef`), never `(slot, child_id)` alone. `MemberPath` (`slot/childId` per
    step, `%25`/`%2F` escapes; depth-1 text unchanged) is the public address — history `store`, tool-run `member`, `child:` keys,
    and the `owner` string that `BackboneMessage::Member`, `ChildEmit`, `ChildPackEntry` and `ChildHeadPackEntry` gain with
    channel 23 — resolved step by step (`resolve`, `path_of`). Nothing persisted changes. Owner: S5-NESTED (N1 keys, N2 layout, N3 recursion).

31. **Board host contract is artifact-neutral** (10:12; closes `📓️audit-s5-goal-2.md` gap 6; this is the concrete form of §22.9).
    (a) Names: no framework identifier names an artifact. `Puzzle2d*` / `puzzle2d*` / `PUZZLE2D_*` in React `🖥️Board2dHost`,
    `classifierKind === "puzzle2d"` and the `vortex` context-menu key in `🗣️Interpreter`, `puzzle_board_*` /
    `puzzle3d_catalogue_drag_payload*` in the wgpu `⚙️EngineCanvas` / `🐚️Shell` become `board*` / `catalogueDrag*`.
    (b) Facts are declared, never named: (i) every board event kind declares its `delivery` (`transient` | `flushNow` | `coalesced`) in
    the ONE board event schema; the generated Rust + TS tables replace `PUZZLE2D_TRANSIENT_EVENT_NAMES`, `PUZZLE2D_FLUSH_NOW_EVENT_NAMES`
    and the wgpu `board_event_transient` / `board_event_flush_now`; (ii) the catalogue drop payload is a contract value the board node
    declares (`dropPayload { mime, schema }`), read by one generic parser; (iii) the selection domain and its granularity classifier come
    from the app's declared selection domains (descriptor) — the runtime's literal `"vortex"` (plugin runtime ≈ :29433–29692), the
    Board2dHost classifier (≈ :203) and the board classifier kind read the declaration.
    (c) Proof: block 2d produces `gesture` rows through the same contract (law), and a gate refuses `puzzle` / `vortex` in non-test,
    non-fixture source under `🧰️framework/**` outside doc comments. Owners: PUZZLE (schema + declarations + puzzle side, writes the
    exact field list first), UI (React hosts), WGPU (wgpu hosts), RUNTIME (runtime literal), GATES (gate), TOOLS (block 2d producer).
32. **Every interactive tool is a machine inside a transaction** (10:12; closes gap 5). (a) puzzle 2d's non-drag board tools
    (`regionCreate`, `regionResize`, `brushPlace`, `edgeCreate`, `edgeDelete`, `nodeDelete`, `apply-board-events` ≈ :108–152) become
    `GestureTool` machines driven by `drive_gesture` on the framework gesture slot: press → stream → release yields the mutations inside
    ONE `ToolTransaction`; an aborted or frozen gesture yields none; a one-click tool is a one-step machine (one transaction, one row).
    (b) The same for wfc grid2d / grid3d Select / Pin / Mask, block 3d surface brush (TOOLS) and architect program graph edits through
    the shared node-graph row helper (GRAPHS-WIRES). (c) The nine private wrappers (layout, note, drawing, puzzle 2d / 3d / 5d, cad,
    shooting, fem 3d) move onto `drive_gesture`; the four redundant per-plugin `TimeTravelFrozen` arms (raster, generation3d, layout,
    puzzle 2d) are deleted. (d) Law per tool: one gesture = one transaction = its mutations, cancel = none; gate: no artifact with a
    `UtilityDefinition` lacks machine vocabulary.
33. **Accept needs a change** (10:12; gap 9): while the draft equals the applied input (`status.changed == false`) Accept is refused
    with a reason (en + de) on both bands and in the guest section — one row in the shared band control table (`disabledBy: unchanged`);
    Discard stays. Lands with B3 (the probe adapts first).
34. **A store always acts as someone — by construction** (10:50; S5-STORE; replaces §22.6's refusal; own wave after B2 with a
    kernel + plugin test window). (a) `ArtifactStore::new(envelope, actor: ActorId)` and every retained initializer take the actor
    as a REQUIRED argument; the store holds `local_actor: ActorId` — no `Option`, no refusal path, no `"local"` literal in the
    store, no guessing the actor from the last applied edit on reload. (b) Authoring is total: an operation is authored by
    `mutation.author_id()` when it names one, else by the store's actor; transitions by the store's actor. (c) An app's genesis,
    the codec route (`apply_ops` over a bare store built from a pack) and plain test stores pass `LOCAL_ACTOR_ID`; an admitted
    instance rebinds; sync / mcp / space / host routes pass the actor they were opened with. (d) `edit_is_local` is authored-only;
    an unauthored edit is refused by `validate_durable_history` and the paged `.spr` decoder. (e) `cargo check --tests` enumerates every unbound site.
35. **A document is identified by its genesis pack's STORED bytes, never by a re-encode** (11:10; S5-STORE, from S5-LOAD's finding;
    B3, one wave with 34 — same construction sites). Today every route derives `initial_digest` from
    `initial_snapshot.encode_pack()` (store new / replace, hydration, eight plugin initializers) and compares it with the hash of
    a peer's stored bytes (`merge_persisted_history`, backbone `verify_genesis`), and every persist re-encodes the genesis — so two
    builds whose encoders order members differently read one document as two. Decision: (a) the genesis pack is encoded ONCE, at
    creation; the envelope keeps those bytes (immutable, shared) beside the decoded snapshot; identity = blake3 of them. (b) Every
    load decodes from the stored bytes and keeps them; persist and the backbone `Genesis` emit them verbatim; no route re-encodes
    the genesis for identity or persistence (persist gets cheaper). (c) A canonical key-ordered form is REJECTED: it needs a second
    encoder contract (member order, number forms) that drifts exactly like this one; a recorded digest without the bytes is
    rejected too (the stored pack could no longer be checked against it). (d) Law: two encoders, one document → merge, same
    revision; a genesis whose bytes differ is another document. A document born from text is encoded once at that birth.
36. **An input that names an owned child is identity, not a parameter** (19:16; found by the acceptance law on cad). A leaf that
    hands out an owned member (`childId`, `target` of an owned-child creation) publishes those inputs as NOT editable (the leaf is
    withdraw-only when nothing else remains), and — defence in depth — finalize checks the ownership closure of the replayed
    document: a replay that leaves a declared member without its store is that mutation's Fatal (`mutation.ownership-incomplete`,
    en + de), never a document the archive loader refuses later. Owners: S5-RUNTIME (finalize check + notice), S5-FLOWCAD (cad's six
    leaves), S5-GATES (gate: an input whose schema role is `owned-child` identity is never editable).

## §23 Goal of 2026-10-06 00:22 — every single editor

The dev's directive: "The timetravel feature that inputs for mutations can be changed with conflict resolution, etc must be general
and implemented for every single editor. End to end."

1. **Definition of done per editor (artifact crate with an editor):** (a) every mutation leaf publishes its inputs with UI metadata
   or is declared withdraw-only (`schema mutation-inputs` + `inputless` gates at zero for the crate); (b) the cross-plugin acceptance
   law passes on the real app runtime: begin edit → draft an input → preview == document as of that mutation (downstream not
   applied) → accept → replay report → withdraw / restore zero trace → finalize overwrite AND new alternative → both reload
   identically; one passing case per input-control kind; child-lane edits where the editor composes members; (c) the generic
   conflict case passes (item 2); (d) where a dev serve of the editor exists, the universal live journey passes (item 3).
2. **Generic conflict-resolution case in the acceptance law ("cascade")** — artifact-agnostic, no plugin knowledge: on the editor's
   demo / seeded history, withdraw (or edit to an invalid reference) an upstream mutation that later mutations depend on; Accept;
   the replay report must classify every downstream mutation (applied / warning / error / fatal), a fatal blocks the review,
   `nextProblem` names it, withdrawing or editing each blocker in turn reaches "ready", finalize → reload == fresh fold. A history
   without dependents says so (census), it does not fail.
3. **Universal live journey (probe batch U)** — manifest-driven, no editor-specific selectors: run the editor's first mutating action
   with its declared defaults (Actions rail), History shows its row, Edit → band `editing` + an editor with ≥ 1 input control of a
   contract role, change one input, Accept → replay → review, Finalize → prompt offers Overwrite and New alternative → Overwrite,
   then Withdraw → Restore on a row, Exit zero trace. Runs against ANY dev serve URL (`?plugin=<id>`), React and wgpu.
4. **Order of work:** the unattended acceptance batch over all remaining families (running since 00:22) → classification of every
   non-PASS row by owner → fixes plugin by plugin → the cascade case → live journeys on every serve that exists (puzzle 2d :6012 /
   :6112, a peer's draw serve :6064, further activations as disk allows).
