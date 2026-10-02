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
