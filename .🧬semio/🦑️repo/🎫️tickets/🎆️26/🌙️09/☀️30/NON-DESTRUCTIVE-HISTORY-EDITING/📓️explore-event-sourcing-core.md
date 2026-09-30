# Explore: event-sourcing / VCS core for non-destructive history editing

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Read-only audit. No source file was edited.
Peer report on the same ticket (alternatives, hub, persistence cost of a new transition): `📓️explore-alternatives-vcs.md`. Its `Supersede` primitive and my `Amend` below are the same idea; this report goes deeper on the trait layer, the fold sites and the replay mechanics.

## 0. Method, aliases, caveats

- Read every cited region directly. Line numbers were verified on 2026-09-30 ~02:00 and drift by a few lines: `🏪️store/🦀️.rs` is edited concurrently by peers.
- Ran once: `cargo test -p semio-framework-replication --lib transition` -> **16 passed, 0 failed** (6 s build with a warm shared cache). The previous ticket's status recorded 14 transition tests; the filter now matches 16, including the durable-collaborative-redo and language-agnostic fixture tests.
- Not run: the `semio-framework-os-kernel` store suite. A rebuild of that very large crate is long (machine load was 14+, the store file is edited concurrently), and a read-only audit did not justify starting it (fleet rule 9). The previous ticket's figures (367 pass / 51 fail, all pre-existing debt) are quoted as claims, not re-verified.
- Aliases:
  - `REPL` = `🧰️framework/🔨️modules/📡️replication`
  - `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`
  - `STORE` = `OS/🏪️store/🦀️.rs`
  - `VCS` = `OS/🌿️vcs/🦀️.rs`
  - `SPR` = `OS/📡️spr`
  - `CMD` = `SPR/🎮️command/🦀️.rs`
  - `HIST` = `SPR/📜️history/🦀️.rs`
  - `PLUG` = `OS/🔌️plugin/🦀️.rs`
  - `DSL` = `OS/🗣️dsl`
  - `DB` = `OS/🛢️db`
  - `TRANS` = `REPL/🔗️causal/🔀️transition/🦀️.rs`
  - `CAUSAL` = `REPL/🔗️causal/🦀️.rs`
  - `MUT` = `REPL/🎮️mutation/🦀️.rs`
- Crates: replication is `semio-framework-replication` (lib name `protocol`, manifest `REPL/📦️packages/🦀️rust/Cargo.toml`). Store, vcs and spr are mounted as `os_store`, `os_vcs`, `os_spr` inside `semio-framework-os-kernel` (`OS/../📦️packages/🦀️rust/🦀️.rs:159-330`).

## 1. The mutation contract

### 1.1 Traits (who owns what)

| Item | Where | Shape |
|---|---|---|
| `MutationDiff<P>` | `MUT:103` | `apply(&self,&P)->Result<P,MutationApplyError>`, `absorb(&mut self, Self)` (sequential coalesce only), `retire_cold(self)`, `retire_projection(P)` |
| `DiffAlgebra<P>` | `MUT:155` | `inverse`, `between`, `is_empty` (adopted per type, shrink-only allowlist) |
| `Mutation<P>` | `MUT:174` | `type Diff`, `const DESCRIPTORS`, `descriptor()`, `diff(&self,&P)->MutationOutcome<Diff>`, `inverse(&self,&P)->Vec<Self>`, `retire_cold`, defaults `mutation_id()->None`, `dependencies()->[]`, `conflict_target()->[]`, `base_version()`, `author_id()`, `timestamp()`, `undo_policy()=ExactBaseOnly`, `state_class()=Artifact`, `may_emit_foreign_steps()=true`, `foreign_steps()->[]` |
| `MutationLeaf` | `MUT:928` | `const DESCRIPTOR: MutationLeafDescriptor` (14 fields, `MUT:371`), `const PROVENANCE` |
| `MutationKind<P,Op>` | `CMD:219` | one leaf: `SEMANTICS: SemanticDescriptor{verb,entity,kind,record}`, `diff`, `inverse`, `label()->LocalizedLabel` (en+de exhaustive), `timestamp`, `target()`, `may_emit_foreign_steps` (default false), `foreign_steps` |
| `SemanticMutation<P>` | `CMD:261` | aggregate refinement, implemented only by the derive: `kinds()`, `semantics()`, `label()`, `target()` |
| `CompositeMutationKind` | `CMD:789` | `plan(&self,&P,&mut Planner)`; `Planner` `CMD:678`, `MAX_PLAN_DEPTH = 8`; `fold_plan_diff` `CMD:822` (all-or-nothing: any `Error`+ or `PlanError` => empty diff, messages kept, step messages target-prefixed `step-N`); `fold_plan_inverse` `CMD:857`; `plan_foreign_steps` `CMD:879` |
| `OpText` / `OpBinary` / `DiffCodec` | `MUT:1209` / `MUT:1223` / `MUT:1245` | one-line text, canonical binary, diff twin |
| `MutationDescriptor` registry | `CMD:437-590` | runtime `schema#kind` -> descriptor, `register_mutation_descriptors` |

`P` is the artifact snapshot. The aggregate `Op` is an enum of leaf payload structs (example: `WorkflowMutation` in `OS/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🦀️.rs`, 18 leaves, each in its own folder with `🦀️.rs`, `🔣️.json` descriptor, `🧬️schema/🔣️.json` payload schema, `🧪️tests`).

### 1.2 `#[derive(Mutations)]` / `#[derive(CompositeMutation)]` / `#[derive(MutationLeaf)]`

- `DSL/✨️derive/🦀️.rs:1596` `expand_derive_mutations` -> `:1616` `expand_mutations`. Attributes: `#[mutations(snapshot = P, diff = D, schema = "os.x" [, retire_cold = path])]` (`:1557`).
- Requires every variant to be `Variant(LeafPayload)`. Generates `impl Mutation<P> for Aggregate` where each of `diff`, `inverse`, `timestamp`, `conflict_target` (from `MutationKind::target`), `may_emit_foreign_steps`, `foreign_steps` is a `match` delegating to `MutationKind` (`:1783-1806`). `mutation_id()`, `dependencies()`, `base_version()`, `author_id()`, `undo_policy()` are NOT overridden, so they keep the trait defaults.
- Also generates `impl SemanticMutation`, `From<Leaf> for Aggregate`, `register_<aggregate>_descriptors(state_class)`, and const-time roster validation (`DESCRIPTORS` uniqueness, verb in `APPROVED_VERBS` `CMD:112`, semantic kind == kebab(variant), source provenance under the aggregate's taxonomy root).
- `DSL/✨️derive/🦀️.rs:1889` `expand_composite_mutation` wires `MutationKind` for a `CompositeMutationKind` (`fold_plan_diff`/`fold_plan_inverse`/`plan_foreign_steps`).
- `DSL/✨️derive/🦀️.rs:540` `expand_mutation_leaf` reads the leaf's `🔣️.json` descriptor at compile time (`payloadSchema`, `textOpcode`, `binaryTag`, `invertibility`, `diffParticipation`, `outcomeClasses`, `composition`, `requiredLanguageSurfaces`).
- `#[derive(dsl::DslOps)]` (`:1528`) emits only `DslVariants`; `OpText`/`OpBinary` on the aggregate are handcrafted per artifact, usually delegating to `dsl::variants_binary::{encode_op,decode_op}` (`DSL/🦀️.rs:535`), example `workflow 🦀️.rs:1514-1545`.

### 1.3 Outcome, message, severity, policy

```rust
// MUT:1066                                   // MUT:953
pub struct MutationOutcome<D>{diff:D,         pub struct MutationMessage{ level:Severity, code:FaultCode,
                              messages:Vec<MutationMessage>}   message:String, target:Vec<String>, op_index:Option<u32> }
// REPL/⚠️diagnostic/🦀️.rs:125   enum Severity { Info, Warning, Error, Fatal }   // derive(Ord) = level order
// REPL/🧾️wire/🦀️.rs:212       enum MergePolicy { LaissezFaire, Normal(default), Vigilant }
//   rejects(level): LaissezFaire>=Fatal, Normal>=Error, Vigilant>=Warning   (:251)
```

- Builders: `MutationOutcome::{new, empty, fatal, error, info, warn, absorb_messages, stamp_op_index, into_parts, worst_level, is_applicable(policy), apply_to(&mut P)}` (`MUT:1101-1201`). `apply_to` converts a `diff.apply` failure into a `Fatal` message with `D::default()` and keeps going. This is the right primitive for a non-aborting replay.
- Laws (`MUT:1046-1053`): Fatal => diff is default; Error => no change to the named target; messages are deterministic in `(op, base)`.
- Exactly seven codes with fixed levels (validated at persistence, `STORE:12055` `expected_mutation_message_level`, `STORE:12065` `validate_persisted_message` also requires `op_index < forwards.len()`): `mutation.target-missing` Error, `mutation.no-op` / `mutation.partial` / `mutation.clamped` Warning, `mutation.duplicate-id` / `mutation.invariant` Fatal, `mutation.cascade` Info.
- `validate` no longer exists; every check is a message on `diff`.
- Reports: `EditMessages{edit_id, messages}` `REPL/⚔️conflict/🦀️.rs:240`, `DispatchReport` `:275`, `MergeReport{policy, accepted, insertion_index, replayed: Vec<EditMessages>, worst, conflict}` `:322`. Local dispatch returns `CommandReceipt{edit_ids, generation, messages, worst}` (`STORE:3826`).

### 1.4 `MutationMeta`, `Edit`, and how a forward op is identified

```rust
// MUT:1362  per-op metadata                       // MUT:1488  one coalesced gesture
pub struct MutationMeta{ mutation_id:Option<MutationId>, dependencies:Vec<MutationId>,
  base_version:u64, author_id:Option<ActorId>, timestamp:HybridLogicalTimestamp,
  undo_policy, payload_hash:Option<PayloadHash>, semantic_kind, label, group_id:Option<String>, origin }
pub struct Edit<Op>{ id:String, actor:Option<String>, forwards:Vec<Op>, inverse:Vec<Op>,
  mutation_meta:Vec<MutationMeta>, description, coalesce_key, sequence_number:i32, started_at, finished_at }
```

- **Forward op = the leaf payload inside `Edit.forwards[i]`.** This is the mutation's "input". A composite mutation stores only its own payload; its plan steps are re-derived from `base` on every `diff` (so replay re-plans automatically).
- **Identity chain** (single source: `mutation_ids_for_edit`, `CAUSAL:790`): `mutation_meta[i].mutation_id`, else `op.mutation_id()`, else `"{edit.id}#{i}"`. In practice three id families coexist:
  - multi-op local `Apply`: content hash `mutation-<hex16>` = `mint_mutation_id(encode_op bytes, hlc)` (`VCS:67`, called in `replay_mutations`, `STORE:18473`);
  - single-op local edit: op 0 is stamped with the edit id itself (`stamp_primary_operation_identity`, `STORE:11526`);
  - retained batched path: `edit.id` for position 0 and `{edit.id}#{position}` after (`fold_batch_item`, `STORE:17747`).
- **Edit id**: `mint_edit_id(replica, sequence, forwards_fingerprint)` (`VCS:38`), fixed at authoring. It is not re-verified against content afterwards.
- **Op index** = index in `edit.forwards` = `MutationMessage.op_index`. `mutation_meta.len()` must equal `forwards.len()` for persisted edits (`stable_mutation_ids_for_edit`, `STORE:12080`).
- **Edit HLC** = `mutation_meta[0].timestamp` (`edit_timestamp`, `STORE:19667`), the order key of every fold.
- `Edit.inverse` is a flattened, per-op-reversed list (`replay_mutations` does `back.reverse(); inverse.extend(back)`). It is **not** used by store undo any more (undo is a `Revert` transition plus refold). Its consumers: `.ops`/`.spr` persistence (`STORE:11815`, `:12349`), plugin `edit_mutations()` for kernel mutations (`PLUG:27106`, `:27740`), and `MutationEnvelope.inverse` where it is zipped by index with `forwards` (`CAUSAL:845`). That zip misaligns whenever an op has 0 or 2+ inverse ops (pre-existing).

### 1.5 Serialized forms of a forward op

| Form | Representation | Where |
|---|---|---|
| in memory | aggregate enum of leaf structs; `ToValue`/`FromValue` (`DslValue` tree) | `REPL/🌱️value/🔁️codec/🦀️.rs:21,42` |
| text | one line `keyword k=v ...`; `OpText::print_op`/`parse_op`; specs from `DslVariants::variants()` -> `RecordSpec{keyword, layout, fields: Vec<FieldSpec{key, shape, optional,...}>}` | `DSL/🦀️.rs:293`, `DSL/🧬️schema/🦀️.rs:33,120,181`; `.ops` file |
| binary | `format u8 (=1)`, then `variant ordinal varint`, then `record body`; **canonical** (`decode_op` re-encodes and rejects non-identical bytes); tagged variant `encode_tagged_op` via `📡️.protocol.semio` | `DSL/🦀️.rs:535-600` |
| wire | `MutationEnvelope{mutation_id, document_id, actor, dependencies, observed, target, diff:ArtifactDiff{schema,payload=encode_op}, inverse:InverseMutation{schema,payload}, timestamp}`; one envelope per forward op; codec `encode_envelope`/`decode_envelope` | `CAUSAL:48,119,149,887,914` |
| `.spr` | `REC_EDIT` (critical) carrying `HistoryEdit{id, actor, started_at, finished_at, coalesce_key, description, ops:Vec<OpPayload{text?,binary?}>, inverse, meta:Vec<HistoryOpMeta>, lane}`; `HistoryOpMeta` has `op_id, dependencies, hlt, payload_hash, group_id, origin, messages` | `HIST:105-175`, `encode_edit :774` |
| JSON | `crate::os_pack::json::to_json_string` over `ToValue`, used for revision digests, `.ops` `message`/`conflict` lines and fixtures | `STORE:13860` |
| leaf schema | per-leaf `🧬️schema/🔣️.json` (JSON Schema draft-07); observation to verify: workflow `move-node` schema lists `node_id` while the Rust payload is `#[value(rename_all = "camelCase")]` (`nodeId`), so the JSON Schema may not equal the `ToValue` wire shape | `OS/🔁️workflow/.../↔️move-node/🧬️schema/🔣️.json` |

### 1.6 Schema-first hooks for "edit the inputs"

- `ToValue::value_at_path(path)`, `FromValue::edit_value_at_path(path, ValueEdit::{Set,Insert,InsertAt,Remove})` and `edit_through_value(&mut T, path, ValueEdit)` (`REPL/🌱️value/🔁️codec/🦀️.rs:21-125`). The last one runs the edit on the value tree and decodes back, so every `from_value` invariant holds and `target` is unchanged on failure. This is the ready-made, schema-driven "change input X of mutation M" primitive, independent of the concrete leaf.
- `DslVariants::variants()` gives the typed field list (`key`, `Shape`, `optional`) of every mutation kind, enough to build a generic inputs form.
- Result of an edit must round-trip through `OpBinary::encode_op` (canonical) to be a valid replacement payload.

## 2. The causal log, transitions, fold, projection

### 2.1 Envelope, DAG, HLC

- `HybridLogicalTimestamp{actor, physical_ms, logical}` (`REPL/🆔️ids/🦀️.rs:149`), total order key `cmp_key() = (physical_ms, logical, actor)` (`:181`). Store clock: `tick(now_ms)`/`merge` per local op in `replay_mutations` (`STORE:18487,18491`), `merge` per remote envelope in `ingest_remote` (`STORE:18894`).
- `MutationDag` (`CAUSAL:364`) buffers envelopes until every dependency is known. **Fixed capacity 8192** (`MUTATION_DAG_CAPACITY`, `:192`), ids <= 256 bytes. `insert` -> `Applied | Pending | AlreadyApplied(env)`, `Err(Duplicate|Capacity|IdentifierTooLong)`. `seed_applied` marks locally authored or out-of-band-known ids.
- Every semantic event is a `MutationEnvelope`. Operation envelopes carry the artifact's schema; transitions carry `HISTORY_TRANSITION_SCHEMA = "semio.history.transition"` (`TRANS:19`, routed by `is_history_transition`, `TRANS:259`).
- `event_log()` (`STORE:18593`) = every edit's operation envelopes in ledger order, then `envelope.transitions` (HLC order). `announce_history` (`STORE:18667`) sends `Genesis{pack}` then that log in `Mutations` batches (<= 128 KiB each).

### 2.2 Transitions (`TRANS:54`)

| tag | variant | payload |
|---|---|---|
| 0 | `Revert` | `{ mutation_ids: Vec<MutationId> }` (undo, author's own ops only) |
| 1 | `Reinstate` | `{ mutation_ids }` (redo) |
| 2 | `Commit(TransitionCheckpoint)` | `{ checkpoint_id, parent_id?, change_id, mutation_ids, description?, saved_at, authors: Vec<TransitionAuthor{id,name,avatar?}>, message?, timestamp }` |
| 3 | `Branch` | `{ alternative_id, name, checkpoint_id }` |
| 4 | `Checkout` | `{ checkpoint_id, alternative_id? }` |
| 5 | `Repin` | `{ checkpoint_id, pinned_checkpoint_id, pins: Vec<TransitionPin{child_uri, checkpoint_id}> }` |

- Codec: `encode_history_transition` (`TRANS:119`, `tag varint | fields`), `decode_history_transition` (`:173`, rejects trailing bytes and unknown tags).
- Envelope: `history_transition_envelope` (`TRANS:241`): `mutation_id = "transition-" + hex16(blake3(actor|hlc|payload))` (`:223`, content-addressed), `inverse.payload` empty (a transition is undone by a later transition), `target = []`. Store authoring path: `commit_transition` (`STORE:18000`).
- There is **no** variant that changes an operation's input. `AmendLast` (`STORE:18335`) only appends to the tail edit (name collision, unrelated).

### 2.3 `fold_history` (`TRANS:343`)

`fold_history(edits: &[FoldEdit], transitions: &[MutationEnvelope], excluded: &HashSet<String>) -> HistoryFold`:

- Pure function of the event set. Events sorted by `(hlc.cmp_key(), id)`. `FoldEdit{id, actor, timestamp, mutation_ids}` (`:276`) knows nothing about op payloads.
- Rules: edit joins `active` and clears only its author's redo entries; `Revert`/`Reinstate` map mutation ids to owning edits via `owners`, and are `refused` (listed by transition id) if any owning edit's actor differs from the transition's actor; `Commit` registers change and checkpoint facts; `Branch` and `Checkout` call `checkout()` (`:462`) which **replaces** `active` with the edits of the checkpoint's change chain and clears redo; `Repin` re-identifies a checkpoint.
- Output `HistoryFold{applied (edit ids, HLC order), redo, refused, checkpoint, alternative, changes, checkpoints, alternatives}` (`:317`).
- Store wrapper: `fold_event_log` (`STORE:19641`) builds `FoldEdit`s (`edit_timestamp`, `mutation_ids_for_edit`) and derives `excluded` from non-accepted `Quarantined` conflicts (owners of quarantined mutation ids). `HistoryLog::fold` (`HIST:207`) does the same for a decoded `.spr`.
- Consequence for the feature: `owned()` errors with `transition references unknown operation` for a mutation id no edit owns, so **an edited mutation must keep its `MutationId`**; identity stays, payload changes.

### 2.4 Materializing the document, caches, cost

Document = `ArtifactVcs.initial_snapshot: P` (`.pack`, `VCS:802`) folded over `edit.forwards` of `applied_edit_ids` (`materialize_document_snapshot`, `STORE:11440`; `Self::fold_history`/free `fold_history`, `STORE:17020`/`:19672`, both `apply_mutation(&*snapshot, op)?.0`, **messages discarded**).

Live path (`ArtifactStore`, `STORE:15605`):
- `current: Arc<P>` incrementally maintained.
- `tail_undo_cache: Option<(edit_id, Arc<P>)>`: the snapshot before the current tail edit only.
- `revision_accumulator: CursorRevisionAccumulator` (`STORE:13763`): per applied position `CursorRevisionRecord{id_digest, edit_digest, prefix_digest}` (a prefix-authenticated hash chain), `content_revision = revision(checkpoint)` (`:13951`), `generation` (bumped by `bump`, `STORE:19387`).
- `reproject()` (`STORE:18020`): `fold_envelope_history` -> `adopt_history_facts` (`:18065`) -> if `applied` changed, `project_applied(next)` (`:18042`) with three modes: tail-undo O(1) via the cache, one edit appended at the tail O(edit), otherwise full fold from genesis; then replace `redo`, `checkpoint`, `active_alternative_id`. **It only recomputes the payload when the applied id list changed.** It records no messages.
- `commit_transition` (`STORE:18000`): tick clock, `history_transition_envelope`, `insert_transition` (HLC position), `reproject`, `dag.seed_applied`, push to `pending_report.outbound`, `bump`. `admit_remote_transitions` (`STORE:17969`) is the receive twin (content-equality check for known ids, then `reproject`).

Cost of replay today:
- **No intermediate snapshot cache exists** (only `current` and the single tail entry). Undo of a non-tail edit, redo, checkout, alternative switch, `reset`/`set_state` (`STORE:16151-16200`), load (`construct`, `STORE:15922`) all do a full fold from genesis: O(total ops), one fresh `P` per op.
- `replay_suffix_partitioned` clones the whole state once per edit (`candidate_state = state.clone()`, `STORE:17057`), so O(|P|) per edit plus op cost; `ingest_remote` additionally clones **every** edit into `edits_by_id` (`STORE:18959`).
- The persisted side has an unused facility: `SPR/💎️materialize` (`resolve_plan`, `MaterializeTarget::{LatestOnActive, AtCheckpoint, AtEditOrdinal}`, `REC_PROJECTION` snapshots, `CheckpointPolicy` every 512 edits) is referenced by no store or plugin code path (grep: only its own tests and cli).
- Absolute bound: every ledger (edits, changes, checkpoints, alternatives) is a fixed **64-slot** `ArtifactHistoryLedger` (`VCS:196,297`); a 65th edit fails `edit history ledger is saturated` (`STORE:16511-16523`) and there is no compaction. Ops per edit are unbounded (coalescing extends `forwards`). Other ceilings: DAG 8192, edit-message ledger 8192 entries x 4096 bytes (`STORE:14266-14269`), envelope decode 256 KiB / 64 history items (`STORE:8233,8456`), open conflicts 256, resolved 512 (`STORE:17136-17137`).

## 3. `ingest_remote` and suffix re-evaluation

`ArtifactStore::ingest_remote(envelope) -> MergeReport` (`STORE:18835`). Steps:

1. `ensure_durable_group_idle`; reserve 2 displaced-retirement slots (`:18837`).
2. Clone the causal DAG (`candidate_dag`), `insert(envelope)`. `AlreadyApplied` -> equivalence check (`assert_equivalent_remote_mutation`, `STORE:19116`: same id needs identical schema+payload, else refuse) and a no-op report. Duplicate/capacity -> `VcsError::Backbone`.
3. Drain the DAG (`take_next_applied`): transition envelopes go to `ready_transitions`; operation envelopes become **single-op edits** via `edit_from_operation_envelope` (`STORE:19504`, `id = mutation_id`, `mutation_meta[0]` from the envelope, actor from the envelope). A known edit id must match (`assert_equivalent_remote_envelope`).
4. Transitions only: `admit_remote_transitions` -> `reproject`, return a no-op report.
5. Sort the batch by edit HLC. **k** = binary search over `applied_edit_ids` for the first position whose edit HLC >= the batch's minimum, raised to `max(position(dependency)+1)` for dependencies present in `applied` (`STORE:18916-18935`; note dependencies are compared to edit ids, correct only for single-op ids).
6. `order` = applied ids with the batch spliced at HLC positions. `base` = current state if `k == len`, else `fold_history(envelope, &order[..k])` from genesis (uncached).
7. `edits_by_id` = clone of every edit plus the batch. `replay_suffix_partitioned(base, order, k, edits_by_id, policy)` (`STORE:17038`) walks `order[k..]`; per edit: `candidate_state = state.clone()`, for each op `outcome = op.diff(candidate).stamp_op_index(i)`, collect messages, `back = op.inverse(candidate)` reversed, `diff.apply(candidate)` (an apply `Err` **aborts the whole replay** with `Err`), then `policy.rejects(worst_of_edit)` decides per edit: reject -> candidate dropped, edit id goes to `quarantined_ids`; accept -> state advances, inverse recorded in `rebased_inverse`, id in `committed_ids`. Returns `(state, committed, quarantined, rebased_inverse, replayed: Vec<EditMessages>)`.
8. Degraded = committed edits whose worst message >= Warning. Guard `ensure_open_conflict_capacity` (`:17143`). Quarantined -> `Conflict{Quarantined{envelopes}}` (id content-addressed, deterministic HLC from the edits, not the ticking clock), messages of quarantined edits cleared; committed batch edits inserted into the ledger.
9. Write-back (`STORE:19040-19075`): swap each committed edit's `inverse` for `rebased_inverse` (forwards and `mutation_meta` untouched), `replace_applied_edit_ids_retained(order[..k] ++ committed)`, `replace_edit_messages` per committed edit, renumber `sequence_number`, `edit_sequence = len`, `replace_tail_undo_cache_retained(None)`, `replace_current_retained(state)`, mint `Degraded` conflict, `admit_remote_transitions(ready_transitions)`, `pending_report`, `bump`, `last_projection_cause = RemoteIngest`.
10. Every scratch `P`, `Edit`, inverse and DAG goes through cold-retire helpers (`retire_replayed_projection`, `retire_scratch_edits`, `retire_scratch_operations`, `refuse_with_candidate_dag`); a refused ingest leaves the store unchanged.

Accept/quarantine/degrade is decided **per edit by the local `MergePolicy`**, which is authority-local and never on the wire. Two replicas with different policies can hold different `applied` sets for the same events (peer report §2.3).

`resolve_conflict` (`STORE:19167`) / `accept_quarantined_events` (`:19203`): re-fold with the conflict marked accepted and replay from genesis under `LaissezFaire`; a `Fatal` outcome changes nothing.

### Can it be reused for "replace op at index i, re-evaluate i..n"?

Reusable as is:
- `replay_suffix_partitioned` is a pure static function over `(base, order, k, edits_by_id, policy)`. Feeding it an `edits_by_id` in which the target edit's `forwards[i]` is replaced already re-evaluates `diff`, `inverse` and messages of everything from `order[k]`. It needs no store state.
- Base computation `fold_history(envelope, &order[..k])`, the retire helpers, the write-back block (inverses, edit messages, sequence numbers, tail cache reset, `bump`), `MergeReport`/`EditMessages`, `Degraded` conflict minting and the conflict caps.
- `reproject` already is the single convergence path for local and remote transitions.

Not reusable as is:
- Forward ops are never rewritten (by design and by the immutability checks below), so the replacement must live in a new event plus an override map.
- Per-edit rejection drops an edit that has Error/Fatal messages and leaves it out of `applied`. History editing needs "keep the edit, record the error, continue"; this must be identical on every replica, hence policy-independent.
- An apply error aborts the replay (`return Err(error.into())`) instead of turning into a `Fatal` message. `MutationOutcome::apply_to` (`MUT:1138`) is the existing non-aborting alternative.
- Synchronous, uncancellable, no progress (AGENTS.md requires progress and cancellation for expensive work).
- k is derived from an HLC splice; for an amend it is the position of the owning edit in `applied`.
- The transition path (`reproject` -> `project_applied` -> `fold_history`) discards messages; only edit ingest records them.

## 4. Store, vcs, replication, spr, alternatives

### 4.1 Store API (`ArtifactStore<P,Mutation>`, `STORE:15605`)

- Single write gate `dispatch(ArtifactCommand) -> CommandReceipt` (`:17205`): `ensure_durable_group_idle` -> `pump` (drain backbone) -> `dispatch_inner` (`:18114`) -> `flush_outbound` (`:19334`, one `BackboneMessage::Mutations` batch per command) -> receipt.
- `ArtifactCommand` (`STORE:2906`, `#[value(tag="kind")]`): `Apply`, `Undo`, `Redo`, `UndoWithPolicy`, `ApplyInLane`, `AmendLastInLane`, `UndoInLane`, `RedoInLane`, `CommitCheckpoint`, `CreateAlternative`, `SwitchAlternative`, `CheckoutCheckpoint`, `AmendLast`, `IngestRemote`, `PruneDrafts`, `SetMergePolicy`, `ResolveConflict`. Ordinals are frozen and appended (the last two are 15 and 16). Three codecs must grow for a new variant: derive `ToValue`/`FromValue`, text `print_command`/`parse_command` (`STORE:13253`/`:13342`, `CommandHeaderLine`), `OpBinary for ArtifactCommand` (`STORE:13459`).
- Read side: `snapshot()`, `snapshot_ref()`, `snapshot_read()` (leased `SnapshotRead`), `applied_edit_ids()`, `redo_edit_ids()`, `current_checkpoint_id()`, `envelope()`, `event_log()`, `conflicts()`, `open_conflicts()`, `messages_for_edit(id)`, `history_columns()` (`build_history_columns`, `STORE:13728`), `projection_stamp()` (`ArtifactRevision{applied, redo, checkpoint}` + generation), `invalidate_after_replay()` (`STORE:16066`, cause `ArtifactProjectionCause::Replay`, unused by any replay yet).
- Local apply path: `apply_command` (`:18286`) / `amend_command` (`:18335`) -> `replay_mutations` (`:18473`, the only place that records messages for local edits and refuses atomically with `VcsError::Rejected{policy, messages}` when `merge_policy.rejects(worst)`), then `insert_reserved_edit_history`, `record_edit_messages`, `announce_operations`, `reproject`, `bump`.
- Retained/batched path (what plugins use): `begin_apply_batch` -> `advance_apply_batch` (`STORE:17501`) -> `fold_batch_item` (`:17747`); domain code prepares each item in a step-budgeted `ArtifactStoreOneItemPreparation` (`STORE:14957`, grant/checkpoint/cancel/close_step). The store never calls the generic `Mutation::diff` there. The only generic implementation is the config lane (`PLUG:16576`), which **discards messages** (`Mutation::diff(..).into_parts().0`, `PLUG:16592`). I found no message recording in `fold_batch_item`.
- `VcsError` (`VCS:876`): `Rejected{policy, messages}`, `ValidationFailed`, `UnknownEdit`, `ForeignEdit`, ... A structural failure is `ValidationFailed`; a policy refusal is `Rejected`.

### 4.2 Replication and other modules

- `REPL/🎮️mutation`, `🔗️causal`, `🔗️causal/🔀️transition`, `⚔️conflict`, `🧾️wire` (`UndoPolicy` `:169`, `MergePolicy` `:212`, `StateClass`), `🆔️ids`, `📡️wire` (client/server frames), `🚧️apply-refusal`. No fold of edits over payloads lives there; that is the store's job.
- Framework sync actor `OS/🏪️store/🔄️sync/🦀️.rs`: `persist_operations` (`:2681`) appends edits and transitions to the folder `.spr` append-only (`append_history_events_to_spr`, `STORE:12478`); `handle_external_change` (`:2720`) detects divergence **by op-id sets only** (`known_op_ids` vs file ids). A payload changed in place under the same id is invisible to it.
- `🌿️vcs` (`VCS`): `Author`, `Change{id, edit_ids, description, saved_at}` (`:130`), `Checkpoint{id, change_ids, parent_id, authors, message, timestamp, composition_pins}` (`:170`), `Alternative{id, name, checkpoint_ids}` (`:190`), `ArtifactVcs{initial_snapshot, edits, changes, checkpoints, alternatives}` (`:802`), id minting (`:18-79`), `apply_mutation` (`:1147`), `content_addressed_checkpoint_id` (`:1263`).
- **Checkpoint identity hashes `Change` JSON (edit ids, description, saved_at), message, authors, timestamp and pins, never op payloads** (`VCS:1210-1262`). Op payloads are not part of any checkpoint id.

### 4.3 `.spr` persistence (`SPR/📜️history`)

- `HistoryLog{doc_id, schema, edits, transitions, composition, conflicts}` (`HIST:25`); records: `REC_DOC`, `REC_EDIT`, `REC_TRANSITION = 0x43` (critical), `REC_COMPOSITION = 0x41`, `REC_CONFLICT = 0x42`; hash-chained (`REQUIRED_HASH_CHAIN`), append via `HistoryAppender` (`:1601`), whole-file `encode_history` (`:1474`), `decode_history` (`:1593`).
- Changes, checkpoints, alternatives, cursor and active alternative are **derived** by `HistoryLog::fold` (`:207`); only edits (with `inverse` and `meta`, including persisted `messages`), transitions, conflicts and the composition overlay are stored.
- `print_document_spr` (`STORE:12515`) / `parse_document_spr` (`:12560`) / `parse_decoded_document_spr`. **The loader validates by replaying `log.edits` in file order with their stored ops** (`snapshot.advance(apply_mutation(..))`), separate from the fold-based projection.
- `.ops` text mirror: `OpsHeaderLine` (`STORE:11636`: `Doc, Edit, Revert, Reinstate, Commit, Branch, Checkout, Repin, Message, Conflict`); `ops_line_from_transition` (`:11861`), `transition_from_ops_line` (`:11895`).
- The `.spr` payload of a transition is opaque to the codec; `RetainedHistoryDecode` validates that it decodes as a `HistoryTransition`.

### 4.4 Branch / Checkout / alternatives: mechanics

- Data: an `Alternative` is a named, ordered list of checkpoint ids. All edits of all alternatives live in the single flat `vcs.edits` ledger. Which ones are applied is the fold: `Branch{alternative_id, name, checkpoint_id}` runs `checkout(checkpoint)` (applied := edits of that checkpoint's change chain, redo cleared), registers the alternative with `checkpoint_ids = [checkpoint_id]`, makes it active. Later `Commit`s append their checkpoint to the active alternative's chain (`TRANS:423-428`). `Checkout{checkpoint_id, alternative_id}` moves to any checkpoint and sets the active alternative.
- Local commands: `CreateAlternative{name}` (`STORE:18131`): if no checkpoint exists it first commits pending edits, then `Branch` at **`current_checkpoint_id`** (or the last checkpoint), id = `mint_alternative_id(name, [checkpoint])`, dependencies = the checkpoint's origin mutation. `SwitchAlternative` -> `Checkout` at the alternative's last checkpoint. `CheckoutCheckpoint` -> `Checkout` (alternative = the one whose last checkpoint it is).
- Consequences (verified in `fold_history`): `Branch`/`Checkout` deactivate every applied edit that is not in the target checkpoint's chain, including uncommitted ones. They are HLC-ordered events folded on every replica, so the head (current checkpoint + active alternative) is shared; one user's checkout moves everybody (peer report §2.2).
- What "overwrite" and "new alternative" can map to (details in §10 H9): overwrite = an unscoped supersession of the input, effective on every alternative and every checkpoint containing the mutation; new alternative = `Branch` at the current head plus a supersession scoped to that alternative id, no duplicated edits (duplicating edits would spend the 64 ledger slots).

## 5. Transactions

- **In one replica** the transaction is an `Edit`: one `Apply{mutations: Vec<Mutation>}` (or one coalesced gesture) becomes one `Edit` with N `forwards`, N `mutation_meta`, one `id`. It is the atomic unit of undo (`undo_lane_position`, `STORE:18238`, reverts all its mutation ids) and of policy verdicts in `replay_suffix_partitioned`.
- **Coalescing**: `AmendLast{mutations, coalesce_key}` (`amend_command`, `STORE:18335`) and the retained `batch_amend_target` (`STORE:17484`) fold new ops into the tail edit only if it is uncommitted (no `Change` references it) and carries the same `coalesce_key`. The edit grows (`forwards.extend`), one edit slot is consumed for a whole gesture family. This is the only mitigation of the 64-edit ledger (memory note `project-engagement-history-coalescing-and-64-edit-ledger`).
- **On the wire the grouping disappears**: one `MutationEnvelope` per op, no group field, ops of one edit are not chained by `dependencies` (`Mutation::dependencies` defaults to empty). A receiver materializes **one single-op edit per wire op** (`edit_from_operation_envelope`). So author and peers hold different edit boundaries, different `op_index` values and different edit ids for the same mutations; only `MutationId`s are shared. `Revert`/`Commit` name mutation ids for that reason. Consequence: per-edit message ledgers are replica-local; anything shared or compared across replicas must be keyed by mutation id.
- **Change / Checkpoint**: `commit_pending_checkpoint` (`STORE:18256`) groups all applied-but-uncommitted edits into one `Change`, then one `Checkpoint` (content-addressed id), emitted as a single `Commit` transition naming their mutation ids.
- **Cross-artifact composite gesture**: `MutationMeta.group_id` and `origin: MutationOrigin::{Owner, Contributed, Transaction{initiator}}` (`MUT:1584`); `TransactionCoordinator::{dispatch_group, dispatch_peer_group, undo_group, redo_group}` (`STORE:22442`, methods `:22513+`); channel `AppCommand::{TransactionPrepare, TransactionCommit, TransactionRollback, TransactionUndo, TransactionRedo}` (`SPR/🧵️channel/🦀️.rs:2601-2630`). `group_id` and `origin` are persisted in `.spr` meta but are **not** on `MutationEnvelope`.
- "Engagement ledger" in project vocabulary = the config-lane edits that a tool emits for ephemeral UI state (hover, engagement); they are ordinary edits in the same 64-slot ledger and are why coalescing exists.
- Lanes: `HistoryLane::{Document, Interaction}` (`STORE:2640`), sparse `envelope.lanes`; default `Undo` skips non-document lanes.

## 6. Undo/redo, revert-to-position, views of past states

- **Undo/Redo** = `Revert`/`Reinstate` transitions (author's own edits only; foreign ones `refused` in the fold and refused before the WAL at the hub). Undo picks the last local applied edit of the lane (`undo_with_policy`, `STORE:18210`); redo restores from the shared redo stack. O(1) only for the tail edit, otherwise full fold.
- **Revert to here** (`REVERT_TO_COMMAND_ACTION_ID`, "Backwards"): plugin-level, repeats `Undo` until the target edit is the tail (`begin_framework_revert_route` / `framework_revert_unit`, `PLUG:~29215-29260`). It walks the runtime-only `CommandLogEntry` list (`PLUG:11964`, never persisted) and only for revertible rows (applied + local).
- **History panel**: `ui_history_panel` (`PLUG:12133`), `CommandView`/`HistoryView` (`PLUG:12006,12046`), swimlanes `HistoryColumn` (`STORE:13656`), React `GraphTimelineHost` (checkpoint checkout only).
- **Checkout of a checkpoint** is the only "state as of X" in the store and it is destructive to the shared head (a `Checkout` event). There is **no** non-mutating "view the document as of event N" in the store.
- Facilities elsewhere: `SPR/💎️materialize` (`MaterializeTarget::AtEditOrdinal`, `AtCheckpoint`, `REC_PROJECTION` anchors; unused by store/plugin), `DB/📽️projection` ("state as of frontier X" historical queries via `db_index::ProjectionIndex`), `DB/🔮️preview` (`PreviewStore`: ephemeral speculative overlays that never enter the WAL), `SpaceMember::preview_wire` (phase-1 of transactions). None is wired to artifact history editing.
- `ArtifactStore::invalidate_after_replay` (`STORE:16066`) is an existing seam for "replay changed derived content but not the id lists".

## 7. Language twins and schema-first sources

Rust (authoritative): everything above.
TypeScript:
- `REPL/🟦️.ts`: `MutationEnvelope` type (`:18`), wire envelope codec (`encodeEnvelope :880`), batch codec `encodeCausalEnvelopeBatch`, exact document-backbone batch codec (`:1050+`), bootstrap assembler (`:1313`), client/server frame codecs (`:1584-1800`), `HISTORY_TRANSITION_DIFF_SCHEMA = "semio.history.transition"` (`:951`). **No TS fold, no TS mutation traits.**
- `REPL/🧪️tests/🧪️history-transition/🟦️.ts`: independent TS encoder of all six transitions + Ajv validation of the corpus. `REPL/🧪️tests/🗄️durable-collaborative-redo/🟦️.ts`.
- `REPL/../🎠️kernel/🟦️.ts:~2030`: twins of `Severity`, `MergePolicy` (+ its ordinal twin of `as_u8`), `MutationMessage`. Shell channel twin `OS/🖥️shell/🟦️.ts`; renderer `HistoryTable`, `ShellHost`, `GraphTimelineHost`, Conflicts panel, `EventFeedHost`.
- Per-artifact mutation TS twins exist only for OS config leaves (`OS/../🎚️config/🧬️schema/🧬️mutations/🟦️.ts`, pure `diff`/`inverse` functions per leaf). Artifact leaves require only `rust`, `json-schema`, `text`, `binary` surfaces (`requiredLanguageSurfaces`).
Python: the language-agnostic fixture generator `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/EVENT-SOURCED-FRAMEWORK-VERSION-CONTROL-END-TO-END/🧪️generate-history-transition-fixture.py` (independent encoder).
Schema-first sources:
- `REPL/🔗️causal/🧬️schema/🔀️history-transition-v1/🔣️.json` (draft-07, `Transition.oneOf` of the six kinds), corpus `REPL/🔗️causal/🧫️fixtures/🔀️history-transition-v1/🔣️.json` (payload hex + expected transition, accepted/malformed cases), checked byte for byte by Rust (`the_language_agnostic_fixture_matches_the_codec_byte_for_byte`) and TS.
- `REPL/🔗️causal/🧬️schema/🧮️document-backbone-batch-v1`, `🗄️durable-collaborative-redo-v1`, `REPL/🎮️mutation/🧬️schema/🔣️.json` (+ fixtures), per-leaf `🔣️.json` + `🧬️schema/🔣️.json`, `REPL/🧬️schema/🔣️.json` (bootstrap and presence fixtures).
- `HIST` fixtures: `SPR/📜️history/🧫️fixtures/🔀️transition-record/🔣️.json`.
- No Go/other-language twin found for these types.

## 8. Hub (db) facts that constrain the design

- `DB/🗿️artifact/🦀️.rs` `submit` (`:2037`) refuses a known mutation id with different content (`already committed with different content`) and dedupes by command id: the WAL is append-only and edits are immutable by id. Undo at the hub is a compensating envelope (`undo`, `:~2245`), never a rewrite.
- The hub does not interpret domain diffs; it grades concurrent writes by `MutationEnvelope.target` overlap and `observed` (`unseen_concurrent_writes :341`, `grade_concurrent_write`). `touch_writes_fields` (`:331`) declares transitions as never writing fields, and `unseen_concurrent_writes` never grades a transition. `foreign_history_transition_refusal` (`:2004`) knows only `Revert`/`Reinstate` (any other variant decodes to `None` and passes).
- `SubmitOptions{durability, policy}` (`:111`); a policy rejection happens before any WAL append.
- Hub check-in replays envelopes through `ingest_remote` under `Normal` (`replay_envelopes_onto_pair`, `STORE:~10704`, peer report §2.5).

## 9. Test suites and how to run them

Run from the repo root, foreground; builds are shared and slow (cold `os-kernel` build exceeds 10 minutes).

| Area | Crate / package | Files | Command |
|---|---|---|---|
| Fold, transition codec, fixture | `semio-framework-replication` | `REPL/🔗️causal/🔀️transition/🧪️tests/🔬️unit/🦀️.rs`, `REPL/🔗️causal/🧪️tests/🔬️unit/🦀️.rs`, fixture `🔗️causal/🧫️fixtures/🔀️history-transition-v1` | `cargo test -p semio-framework-replication --lib transition` (**16 passed**, ran); `... --lib causal` for the DAG |
| Conflict types, mutation contract | same | `REPL/⚔️conflict/🧪️tests/🔬️unit`, `REPL/🎮️mutation/🧪️tests/🔬️mutation-leaf-metadata` | `cargo test -p semio-framework-replication --lib conflict` / `mutation` |
| TS twin | `@semio-tech/framework-replication` | `REPL/🧪️tests/🧪️history-transition/🟦️.ts`, `🗄️durable-collaborative-redo`, `🧪️document-backbone-envelope-batch` | `bun nx run @semio-tech/framework-replication:test` |
| Store, ingest, merge laws | `semio-framework-os-kernel` | `STORE/🧪️tests/🔬️unit/🦀️.rs` (9237 lines, 515 fns; e.g. `modify_vs_delete_*` `:4772`, `chronological_determinism_any_arrival_order_converges` `:4811`, `quarantine_*` `:4867-4900`, `checkpoint_transitions_resolve_against_each_replicas_own_edits` `:4702`), `STORE/🧪️tests/📤️outbound-announcement`, `🔗️backbone-detach`, `🗄️durable-owned-group` | `cargo test -p semio-framework-os-kernel --lib <test_name_substring>` (add `--features sync` for `🔄️sync` tests); not run here |
| spr / history / protocol laws | same | `SPR/📜️history/🧪️tests/🔬️unit`, `SPR/🧪️tests/⚖️protocol-laws/🦀️.rs`, `SPR/💎️materialize/🧪️tests`, `SPR/🔌️io/🧪️tests` | `cargo test -p semio-framework-os-kernel --lib os_spr:: -- --skip os_spr::channel` |
| Derive / leaf metadata | os-kernel + `DSL` | `DSL/✨️derive/🧪️tests/*` (`mandatory-mutations`, `mutation-attrs`, `composite-attrs`) | `cargo test -p <dsl derive crate> --lib` (crate name not resolved here) |
| Hub | `semio-framework-os-kernel-db` | `DB/🗿️artifact/🧪️tests/🔬️unit`, `⚔️concurrent-write` | `cargo test -p semio-framework-os-kernel-db --lib` |
| Gates | root | `mutation-outcome-law` (7 gates), `verify dependencies literal-external` | `bun ./📜️script.ts verify mutation-outcome-law` (per previous ticket; register via `.vscode/launch.json`) |

Existing oracles worth reusing: `test_support::{assert_live_equals_replay, assert_store_roundtrip, assert_document_text_round_trip, assert_document_pack_round_trip, assert_op_text_binary_equivalence, mutation_report_json}` (`STORE:22795-23440`) and `protocol_laws::{assert_chronological_determinism, assert_merge_convergence, assert_ledger_matches_replay, assert_outcome_deterministic, assert_outcome_policy_matrix}` (`SPR/🧪️tests/⚖️protocol-laws/🦀️.rs:691-965`).

## 10. Hook points for mutation-input editing + suffix replay

Recommendations first (H1-H11), then the ranked gaps.

### H1. One new event kind: `Amend` (replication crate, schema-first)

- Add `HistoryTransition::Amend{ mutation_id, replacement: {schema, payload (OpBinary bytes)}, alternative_id: Option<String>, note: Option<String> }`, tag 6, appended after `Repin` (`TRANS:54`, codec `:119`/`:173`).
- Why an event and not an in-place rewrite of `edit.forwards[i]`:
  - AGENTS.md: event-sourced, no CRUD.
  - Immutability checks: `same_operation_identity_and_payload` / `assert_equivalent_remote_*` (`STORE:19106-19140`), hub `already committed with different content`, hash-chained append-only `.spr`.
  - The folder sync detects divergence only by id sets (`handle_external_change`), so an in-place change would be silent.
  - Checkpoint ids do not cover payloads.
- `mutation_id` stays the logical identity, so `Revert`/`Commit`/`fold owned()` keep working. Original operation envelopes are never re-sent with a different payload (`event_log`/`operation_envelopes` must emit originals; the amendment travels as its own envelope).
- Envelope: `history_transition_envelope` with `dependencies = [mutation_id]` (like `Revert`), `target = original op's conflict_target()` so the hub can grade it, content-addressed id (identical amendments dedupe).
- Fold (`fold_history`): overrides = per `mutation_id` the last non-refused `Amend` in `(hlc, id)` order, honouring `alternative_id == fold.alternative` (scoped) or none (global). Extend `HistoryFold` with the effective override list. Decide the authorship rule and reuse the existing `refused` mechanism (`TRANS:378`).
- Sites that must change together: `TRANS` codec + fold + tests; JSON Schema `🔀️history-transition-v1` and its corpus (Python generator, Rust test, TS encoder/Ajv); `.ops` `OpsHeaderLine` + `ops_line_from_transition`/`transition_from_ops_line` (`STORE:11636,11861,11895`); `.spr` `RetainedHistoryDecode` (payload validation) and `HistoryLog::fold`; hub `foreign_history_transition_refusal`/`touch_writes_fields`/grading (`DB/🗿️artifact/🦀️.rs:331,2004`); sync `rollback_envelope` (transitions have no inverse rollback, a hub-refused amend needs a designed answer).

### H2. Store command and authoring path

- New `ArtifactCommand::AmendMutation{...}` (next ordinal 17) with the three codecs listed in §4.1, and an arm in `dispatch_inner` (`STORE:18114`) next to `Undo`.
- Authoring = validate (mutation id known, in `applied` or reachable, `edit_is_local` or the chosen authorship rule, replacement decodes and re-encodes canonically, refuses `may_emit_foreign_steps()` mutations, see gap 8) -> **dry-run** the replay (H3, pure, no store mutation) -> if the local policy rejects the resulting worst level, return `VcsError::Rejected{policy, messages}` atomically (same contract as `replay_mutations`, `STORE:18473`) -> `commit_transition(Amend, [mutation_id])` (`STORE:18000`) -> `flush_outbound` announces it as one `Mutations` batch.
- Peers converge through `admit_remote_transitions` (`STORE:17969`) -> `reproject`: no new receive path needed.

### H3. Replay core: generalize `replay_suffix_partitioned` (`STORE:17038`)

- Extract a shared `replay_suffix(base, order, k, edits, mode)` where `mode = Merge(policy)` (today's behaviour for `ingest_remote`/`accept_quarantined_events`) or `Report` (history editing).
- `Report` mode: never rejects or quarantines (so `applied` is policy-independent and identical on every replica), uses `MutationOutcome::apply_to` so an apply failure becomes a `Fatal` `mutation.invariant` message instead of aborting, keeps every edit applied, returns per-op messages with `op_index`, rebased inverses, final state.
- `edits` only needs the suffix edits with overrides applied; do not clone the whole ledger.
- Extract the write-back block of `ingest_remote` (`STORE:19040-19075`) into one function used by both paths (AGENTS: repeated code must be close).
- Start position `k` = index in `applied_edit_ids` of the first edit owning an overridden mutation id (recompute from the fold's override diff before/after); base = `fold_history(envelope, &order[..k])` with overrides applied (H4).

### H4. Effective-forwards accessor at every fold site

Every place that folds `edit.forwards` must see the same effective ops. Introduce one accessor `effective_forwards(edit, &overrides)` and replace direct `forwards` iteration in all of:
1. `Self::fold_history` / free `fold_history` (`STORE:17020`, `:19672`)
2. `materialize_document_snapshot` (`STORE:11440`)
3. `project_applied` one-edit branch (`STORE:18042`)
4. `replay_suffix_partitioned` (`STORE:17038`)
5. `parse_decoded_document_spr` validation replay (`STORE:~12575`, file order!) and `parse_document_text` (`:12921`)
6. the retained initializer `BoundedStoreInitializationPhase::ApplyForward` (`PLUG:~16932`)
7. `replay_envelopes_onto_pair` / hub check-in path (goes through `ingest_remote`, must fold overrides from the envelope's transitions)
8. `db` projection if it ever interprets domain diffs (currently opaque).
Greenfield rule: refactor all sites at once, no compat shim.

### H5. `reproject` must detect override changes

`reproject` (`STORE:18020`) only recomputes when `applied != self.applied_edit_ids`. Add: `if overrides_changed { k = first affected position; replay_suffix(...); write back; }`. Also feed messages into `replace_edit_messages` and set `last_projection_cause` (existing `ArtifactProjectionCause::Replay` fits, `STORE:16066`). Reset `tail_undo_cache` (as `ingest_remote` does).

### H6. Revision / digest blindness for interior edits

`CursorRevisionAccumulator::reconcile_stack` (`STORE:13899`) matches records by edit-id digest and re-hashes only the **tail** record, so a changed interior edit (or override) leaves `content_revision`, `prefix_digest` and `SnapshotRead::commit_authority_matches` unchanged; `bump()` only advances `generation`. Fold the effective override digest into `edit_digest` and truncate `records` at the first affected position, so `content_revision` moves. Check consumers keyed on `ArtifactRevision{applied, redo, checkpoint}` (ids only) and inference caches.

### H7. Progress and cancellation

- Model the replay as a step machine like the retained initializer (`BoundedStoreInitializationPhase`, `PLUG:16682`: `FindApplied`, `ApplyForward{position, edit, mutation}`, `Cancelled`, `Fault`, fuel via `cx.consume_fuel`, `StepOutcome::Yield`) and like `begin_apply_batch`/`advance_apply_batch`/`cancel_apply_batch` (`STORE:17357-17710`) with `ArtifactStoreOneItemCheckpoint{cursor, completed_items, completed_bytes, digest}` (`STORE:14742`) as the progress record.
- All state is scratch until one atomic commit; cancel = retire scratch (`retire_replayed_projection`, `ReplayProjection` RAII wrapper, `STORE:19555`), store untouched.

### H8. Per-mutation results

- `MergeReport.replayed: Vec<EditMessages>` is edit-keyed and edit boundaries differ per replica (§5). Add a mutation-id keyed `ReplayReport{ per_mutation: [{mutation_id, edit_id, op_index, worst, messages}], worst }` next to `MergeReport` (`REPL/⚔️conflict/🦀️.rs:322`) with `ToValue`/`FromValue`, a channel frame (channel version 20 -> 21, `SPR/🧵️channel/🦀️.rs:24`, TS mirror), and UI (History panel `PLUG:12133`, `HistoryTable`). Success = no messages; warning/error/fatal come from `Severity`.
- `CommandReceipt.messages` already carries `Vec<EditMessages>` for a dispatch; reuse via `pending_report`.

### H9. Overwrite vs new alternative (semantics)

- Overwrite = `Amend` with `alternative_id = None`: effective on every alternative and every checkpoint containing the mutation. Warning: checkpoints are content-addressed by edit ids, not payloads (`VCS:1210-1262`), so an existing checkpoint id would silently denote different content. Either accept and document, or refuse overwrite of a committed edit and route to a new alternative.
- New alternative = existing `CreateAlternative` (`Branch` at the current head) + `Amend{alternative_id = new}`. Zero duplicated edits (respects the 64-slot ledgers); switching alternatives (`Checkout{alternative_id}`) switches the override set. `CreateAlternative` today can only branch at `current_checkpoint_id` (`STORE:18131`), though the transition accepts any checkpoint.
- Heavier alternative (not recommended): branch before the edit and re-author every downstream edit through `Apply`; consumes ledger slots and mints new ids.
- Shared-head hazard: `Branch`/`Checkout` are global HLC events (peer report §2.2). A "preview the amended history before committing" mode must be a local-only overlay, not a `Checkout`.

### H10. Persistence

- Amend rides `REC_TRANSITION` (opaque payload, no format bump). `HistoryLog::fold` picks the overrides up once `HistoryFold` carries them.
- `Edit.inverse` and per-edit `messages` in `.spr` are derived caches (messages are a pure function of `(op, base)` by the outcome law); recompute at load after applying overrides instead of persisting stale values, otherwise `validate_persisted_message` (`STORE:12065`) may reject reloaded ledgers.
- `HistoryLog` decode drops unknown records; `compact` and `open_append` in `SPR/🔌️io` drop composition and conflicts (pre-existing debt per the previous ticket's summary, not re-verified).

### H11. Tests to write first (AGENTS.md: language-agnostic + independent implementation)

1. Payload corpus for `Amend`: extend `🔀️history-transition-v1` (Python encoder, Rust codec test, TS/Ajv test) — same three-way pattern as today.
2. Fold law in `SPR/🧪️tests/⚖️protocol-laws/🦀️.rs` next to `assert_chronological_determinism` (`:920`): last-wins by `(hlc,id)` under any arrival order; scoped vs global; refused authorship.
3. Store laws with the demo fixtures (`STORE/🧫️fixtures/🧬️mutations/🧮️demo`, `🚦️severity`, `🛂️validated`, harness `mutation_envelope_at`/`fresh_demo_store`, `STORE/🧪️tests/🔬️unit/🦀️.rs:4720-4870`): amend an interior edit; assert snapshot == fresh replay of the edited log (`test_support::assert_live_equals_replay`, `STORE:23368`), messages == replay (`assert_ledger_matches_replay`, `:956`), Warning/Error/Fatal downstream cases from the `🚦️severity` set, two-store convergence, cancel leaves the store untouched, `.ops`/`.spr` round trip (`assert_document_text_round_trip :23177`, `assert_document_pack_round_trip :23292`).
4. Independent oracle: a Python or TS replay of `SetN/AddN/DeleteN` (pure `(initial, ops) -> state + messages`) compared to the Rust result, as `mutation_report_json` (`STORE:22808`, oracles in `OS/../✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law`) does for single mutations.

### Gaps and risks (ranked)

1. **64-slot ledgers, no compaction** (`VCS:196`): history beyond 64 edits cannot exist today, so "every mutation editable" holds only inside that window; coalescing is the only mitigation. Raising it touches fixed-capacity owner catalogs (`retain_id_capacity`, revision accumulator, batch publication).
2. **Retained batched path bypasses `replay_mutations`** and records no messages; the config lane discards them (`PLUG:16592`). After an amend, replay messages exist that the original run never recorded. Decide whether the message ledger is a cache (recomputable) or a record.
3. **No snapshot cache, `P: Clone` per edit**: prefix fold from genesis on every rewind. `revision_accumulator.applied[i].prefix_digest` is a ready cache key. Any cached `Arc<P>` must retire through `snapshot_retirement_factory` (fail-closed `Drop`, displaced-retirement capacity 1024).
4. **Edit boundaries differ between author and peers** (single-op edits on receivers): per-edit messages are replica-local; key shared results by mutation id.
5. **Interior-edit blindness** of `CursorRevisionAccumulator` (H6).
6. **Synchronous, uncancellable replay** in `ingest_remote`/`replay_suffix_partitioned` (H7).
7. **Local `MergePolicy` decides `applied`** in the merge path (authority-local, not on the wire); history editing must not inherit that (use `Report` mode).
8. **Foreign steps**: `MutationKind::may_emit_foreign_steps` defaults to false, but every `#[derive(CompositeMutation)]` kind returns true (`DSL/✨️derive/🦀️.rs:1921`) and plans foreign steps that were dispatched to other artifacts under a `group_id` two-phase transaction. Editing such a mutation cannot be replayed inside one store; refuse it or cascade through the coordinator.
9. **`Edit.inverse` alignment** with `forwards` is pre-existing fragile (`CAUSAL:845`); inverse recompute must keep the same flattened, reversed layout.
10. **Edit-message ledger caps**: 4096 bytes per entry, 8192 entries; a long suffix replay with many messages fails the whole write-back (`ValidationFailed`). Needs a truncation or summarization rule.
11. **Quarantine exclusion is edit-based** (`fold_event_log` excludes owners of quarantined mutation ids); a quarantined *transition* envelope has no owner. If remote amendments may be quarantined, exclusion by transition id is needed.
12. **Leaf JSON Schema vs `ToValue` shape** may drift (snake_case vs camelCase observation, §1.5); verify before generating input forms from `🧬️schema/🔣️.json`.
13. **Hub**: transitions are not graded for field writes; author-only refusal exists only for `Revert`/`Reinstate`; a hub-refused transition has no rollback (`rollback_envelope`, peer report §2.4).
14. Fold sites are duplicated (eight, H4); missing one yields silent divergence between live state, reload and hub.
15. Prior-ticket test debt (51 failing store tests, `compact`/`open_append` dropping conflicts) is unverified here but will mask regressions; run the suite before and after.
