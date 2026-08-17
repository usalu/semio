# Contract Freeze — Mutation Outcomes, Merge Policies and First-Class Conflicts

> **Binding on every lane.** A lane that needs a change to anything below **stops and reports to the coordinator** — it never improvises. Copied verbatim from `📋️master-plan.md` (sections C1–C10) plus the fan-out decision tables and the verified channel tag allocation.

## Frozen contract (the coordinator copies this into `📋️contract-freeze.md`)

### C1 Severity — `🗣️dsl/⚠️diagnostic/🦀️component.rs`
`pub enum Severity { Info, Warning, Error, Fatal }` — declaration order reversed so `derive(Ord)` IS the level order; `as_u8/from_u8` (0..3). Repo-wide `Hint`→`Info` (Rust: 4 sites incl. `🕸️graph/🗣️dsl`; TS mirrors; WIT enum; golden fixtures re-baked).

### C2 Message & outcome — new region `🔖️Message` in `📡️spr/🎮️command/🦀️component.rs` (after `🔖️Mutation`)
```rust
pub struct MutationMessage { level: Severity, code: FaultCode, message: String, target: Vec<String>, op_index: Option<u32> }
// ctors info/warn/error/fatal(code,msg) + .at(target) + .at_op(i)
pub struct MutationOutcome<D> { diff: D, messages: Vec<MutationMessage> }
// MutationOutcome::new(diff) | ::empty() | ::fatal(code,msg,target) [forces diff = D::default()] | ::error(code,msg,target) [empty diff]
// .info(..)/.warn(..)/.error(..) builders, absorb_messages, stamp_op_index, worst_level(), is_applicable(policy), map, into_parts
pub fn worst_level(messages: &[MutationMessage]) -> Option<Severity>;
```
Laws: (1) Fatal ⇒ `diff == D::default()`; (2) Error ⇒ diff carries no change for the named target; (3) deterministic — equal `(op, base)` ⇒ equal messages.

**Frozen code set (7, generic, gate-enforced — no per-plugin codes):**
| code | level | diff |
|---|---|---|
| `mutation.target-missing` | Error | empty |
| `mutation.no-op` | Warning | empty |
| `mutation.partial` | Warning | survivors only |
| `mutation.clamped` | Warning | non-empty |
| `mutation.duplicate-id` | Fatal | empty |
| `mutation.invariant` | Fatal | empty |
| `mutation.cascade` | Info | non-empty |
`message` = English prose; UI localizes by `code`. `target` = address of the offending element.

### C3 Policy — `📡️spr/🧾️wire/🦀️component.rs` region `🔖️Policies` (replaces deleted CRDT pair)
`pub enum MergePolicy { LaissezFaire, #[default] Normal, Vigilant }` + `rejects(Severity)->bool` (LF: Fatal; Normal: ≥Error; Vigilant: ≥Warning), `as_u8/from_u8`. `UndoPolicy`, `StateClass` untouched.

### C4 Traits — `📡️spr/🎮️command`
- `Mutation<P>::diff(&self, base) -> MutationOutcome<Self::Diff>`; delete `validate`, `merge_strategy`, `conflict_rule`, `reconcile`; delete `ReconcileReport`/`ReconcileSeverity`.
- `MutationKind<P,Op>::diff -> MutationOutcome<Op::Diff>`; delete `validate`. `SemanticMutation`, `inverse`, `label`, `target`, `foreign_steps` unchanged.
- `MutationDescriptor` loses `conflict_rule` (constructor param + fingerprint input) — golden pin re-baked.
- Composite: `Planner::call` folds outcomes, stops on Fatal (`PlanError::StepRejected`), exposes `messages()`; `fold_plan_diff -> MutationOutcome<..>` — **all-or-nothing**: any Error/Fatal step ⇒ empty diff, all messages kept with `target` prefixed by the step path; `PlanError` ⇒ `fatal("mutation.invariant")`. `CompositeMutationKind::validate` deleted. `#[derive(CompositeMutation)]` mirrors.
- `vcs::apply_mutation(snapshot, op) -> (P, Vec<MutationMessage>)`; `CollectionMutation` helpers report `mutation.target-missing` instead of silent no-op.

### C5 Conflicts — new module `📡️spr/⚔️conflict/🦀️component.rs` (glue entry replaces `🔀️crdt`; re-exported from `📡️spr/🦀️component.rs`)
```rust
pub struct ConflictId(pub String);            // blake3(kind, artifact id, sorted mutation ids, hlc)
pub enum ConflictKind { Quarantined { envelopes: Vec<MutationEnvelope> }, Degraded { edit_ids: Vec<String> } }
pub enum ConflictStatus { Open, Accepted, Discarded }   pub enum ConflictResolution { Accept, Discard }
pub struct Conflict { id, kind, status, messages: Vec<MutationMessage>, actors: Vec<ActorId>, timestamp: HybridLogicalTimestamp }
pub struct EditMessages { edit_id: String, messages: Vec<MutationMessage> }
pub struct DispatchReport { policy, worst: Option<Severity>, messages }               // one local dispatch
pub struct MergeReport { policy, accepted: bool, insertion_index: u32, replayed: Vec<EditMessages>, worst, conflict: Option<ConflictId> }
```

### C6 Store — `🏪️store/🦀️component.rs`, `🌿️vcs`
- `VcsError::Rejected { policy, messages }` (nothing applied) and `VcsError::UnknownConflict(String)`; `ValidationFailed` only for structural failures.
- `CommandReceipt { edit_ids, generation, messages: Vec<EditMessages>, worst }`.
- `ArtifactCommand` appended: `SetMergePolicy{policy}` (ordinal 15), `ResolveConflict{conflict_id, resolution}` (16) + text/binary command forms.
- `ArtifactStore` fields: `merge_policy` (default Normal, not on wire envelope), `conflicts: Vec<Conflict>` (replaces `Vec<SpaceConflict>`), `edit_messages: HashMap<edit_id, Vec<MutationMessage>>` (ledger), `clock: HybridLogicalTimestamp` (monotone: `tick` on every local apply, `merge` on every ingest — replaces `HybridLogicalTimestamp::new(0, now_ms())` in `replay_mutations`).
- Methods: `merge_policy/set_merge_policy`, `conflicts/open_conflicts`, `resolve_conflict(id, resolution) -> MergeReport`, `messages_for_edit`, `ingest_remote(..) -> MergeReport`.
- `replay_mutations`: per op `outcome = op.diff(&snapshot)`, stamp op index, accumulate, inverse, meta (`timestamp: clock.tick()`), `snapshot = outcome.diff.apply(..)`; after loop `if policy.rejects(worst) ⇒ Err(Rejected)` — **whole command atomic**.
- Delete `SpaceConflict`, `reconcile_with_last`, `materialize_document_snapshot_with_conflicts`, `snapshot_with_conflicts`.
- `SpaceMember::validate_wire` → `preview_wire(ops) -> Vec<MutationMessage>` (dry-run) + `merge_policy()`; `CompositionCoordinator` phase 1 unions members' messages, one global worst, `Rejected` if policy rejects; `GroupReceipt.messages`. (Negotiate this region with the PLUGIN-DEPENDENCIES ticket.)

**Invariant:** `applied_edit_ids` is always sorted by edit HLC (`min` op timestamp, order `(physical_ms, logical, actor)`). Local `Apply` appends (local clock ≥ everything merged); `Redo` inserts at HLC position.

**`ingest_remote` algorithm:**
1. `candidate_dag.insert(env)`; `AlreadyApplied` ⇒ existing equivalence check, empty report.
2. `batch = drain_applied_envelopes()` → edits (dedupe vs known ids unchanged); `clock.merge(e.timestamp)` for each.
3. Sort batch by HLC. `k` = min partition point of batch in `applied_edit_ids`, raised to `1 + index of latest causal dependency`.
4. `order` = stable HLC merge of `applied_edit_ids` and batch.
5. `base` = `current` if `k == len` (hot append) else `fold_history(order[..k])` (checkpoint prefix).
6. Replay `order[k..]`: per op `o = op.diff(&state)`, stamp, `state = o.diff.apply(&state)`, rebase inverse from `state`; collect `EditMessages` (forwards/meta never rewritten).
7. `worst` over the whole replayed suffix (incl. local edits after k).
8. Reject (`policy.rejects(worst)`): dag not advanced, state unchanged, push `Conflict{Quarantined{envelopes}, Open, messages}` → `MergeReport{accepted:false}`. The deciding `MergePolicy` is runtime-local authority state, reported through `MergeReport`/`VcsError` only; no conflict or persisted history record carries it.
9. Accept: commit `dag`, `applied_edit_ids = order`, inverses, `current`, ledger suffix, renumber `sequence_number`, `tail_undo_cache=None`, `bump()`; if `worst ≥ Warning` push `Conflict{Degraded{edit_ids}, Open}` → `MergeReport{accepted:true, insertion_index:k, ..}`.
- `resolve_conflict`: Quarantined+Accept ⇒ rerun 3–9 with `LaissezFaire` (Fatal still rejects), status Accepted, no second conflict; Quarantined+Discard ⇒ status Discarded, `dag.seed_applied` for those ids, never relayed; Degraded+Accept ⇒ ack; Degraded+Discard ⇒ `Err` (never rewrite shared history).
- `merge_remote_snapshot`: after identity checks, HLC-sort merged edits, run 5–9 from first divergence; reject ⇒ one snapshot-conflict (envelopes via `mutation_envelope_from_edit`).
- `flush_outbound` relays only applied local edits (quarantined never). Undo removes id, order kept; mid-history undo already cold-path refolds.

### C7 Persistence — `📡️spr/📜️history`
`REC_CONFLICT = 0x42` (non-critical caller range); `HistoryLog.conflicts: Vec<HistoryConflict{id, kind u8, status u8, actors, hlt, edit_ids, envelopes: Vec<Vec<u8>>, messages: Vec<HistoryMessage>}>`; `HistoryOpMeta.messages: Vec<HistoryMessage{level u8, code, message, target, op_index}>` (durable ledger, dict-interned codes); `encode/decode_conflicts` mirror `encode_cursor` shape.

### C8 Channel — `📡️spr/🧵️channel` + TS mirror `💻️os/🟦️component.ts` — **CHANNEL_VERSION 10 → 11**
- `AppCommand` appended: `SetMergePolicy{seq, policy:u8}` (30), `ResolveConflict{seq, conflict_id, resolution:u8}` (31), `ReadConflicts{seq}` (32).
- `AppFrame` appended: `MergeReport{in_reply_to:Option<u64>, report:Vec<u8>}` (23), `Conflicts{in_reply_to, conflicts:Vec<u8>}` (24); `Invocation` gains trailing `messages: Vec<u8>` (pack `DispatchReport`); `Error` gains trailing `report: Vec<u8>` (pack `DispatchReport` of the rejected dispatch; `Fault.code = "mutation.rejected"`). `Invocation.diagnostics` keeps DSL meaning. `MergeReport`/`Conflicts` are pushed unsolicited after every ingest next to `DocumentChanged`.
- `📡️wire` `ApplyOutcome::Rejected{reason}` → `Rejected{reason, messages}` (tag unchanged). `MutationEnvelope`/`BackboneMessage` unchanged (policy is local/authority, never on an envelope). Golden vectors under `🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/` (Rust + vitest); tag allocation written into `📋️contract-freeze.md` and coordinated with the PLUGIN-DEPENDENCIES ticket (bump version once).

### C9 Guest / hosts / UI / hub-db
- Guest `🔌️plugin/🦀️component.rs`: `dispatch_emit` pre-fold uses outcomes (Fatal short-circuits); `store.dispatch` `Rejected` → `Fault("mutation.rejected")` + `report` in `AppFrame::Error`; `InvocationResult` carries `DispatchReport` into `Invocation.messages`; `ApplyEnvelopes` handling emits `MergeReport`/`Conflicts`; `VcsArtifactApp::preview(&self, ops) -> DispatchReport` (dry run — the single predicate apps use to grey out actions/context-menu items so "alone ⇒ no errors" holds); `SetMergePolicy`/`ResolveConflict`/`ReadConflicts` handled; `Hello.config` seeds policy.
- Rust host `🖥️host` + `🏃️run`: decode `messages`/`report`/`MergeReport`/`Conflicts`; `HostInvocation.messages`; policy from run config; e2e test.
- TS host `💻️os/🟦️component.ts` (`AppChannelClient` + codecs + `faultMessages()`), TS kernel `🎠️kernel/🟦️component.ts` types + boot policy; parity via shared golden vector.
- React shell (`📺️renderer/🧑️‍🎨️engine/🧱️elements/`): `EventFeedHost` (tone per level, `fatal`), `ChromePanels` new **Conflicts** panel (open conflicts, messages, Accept/Discard), `ShellSync` quarantine state, `DiffViewHost` incoming-vs-current for a quarantined conflict, `ShellHost` toast for rejected dispatch (one per gesture, worst level), `🖱️ui/🧱️elements/📜️HistoryTable` per-edit level badge, `Shell` settings: merge policy (persisted via new OS config triad `🛡️change-merge-policy` under `💻️os/🎚️config/🧬️schema/🧬️mutations/`, forwarded as `SetMergePolicy`).
- i18n: `🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` `UiTranslationSchema` + both bundles (de + en, `{normal, beginner}` pairs): `ui.mutation.level.*`, `ui.mutation.code.*` (7), `ui.mutation.policy.{laissezFaire,normal,vigilant}.{label,description}`, `ui.mutation.policy.setting.label`, `ui.conflict.{panel,accept,discard,quarantined,degraded}.label`, `ui.mutation.rejected.{title,body}`.
- Hub `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`: policy from config → `SubmitOptions.policy`; ack + relay frames carry messages. db `🛢️db/📄️artifact`: `SubmitOptions.policy: MergePolicy`, pipeline `validate+conflict` → one **outcome** step (op outcomes ∪ `ConflictDetector` findings: region intersection = Warning, constraint violation = Fatal) then `policy.rejects(worst)` ⇒ `DbError::Rejected` before WAL; `CommandReceipt`/`ConflictRecord.messages`; `🛢️db/⚔️conflict`: delete `ResolutionPlan`/`combine_conflict_rules`/`CommandTouch.conflict_rule`, add Degraded/Quarantined kinds + Accept/Discard; `🛢️db/⌨️cli`: `--policy`, delete `parse_merge_strategy`/`parse_conflict_rule`/`describe_merge_strategy`; `🛢️db/👁️preview` `synthetic_rule` deleted.

### C10 Deletions (W0)
`📡️spr/🔀️crdt/**` + glue `#[path]` + `pub use merge_concurrent_diffs` (`📡️spr/🦀️component.rs:34`); `MergeStrategyKind`, `ConflictRule`; `Mutation::{merge_strategy,conflict_rule,validate,reconcile}`; `MutationKind::validate`; `CompositeMutationKind::validate`; `ReconcileReport/ReconcileSeverity`; `SpaceConflict` & friends; testkit `assert_crdt_commutative/idempotent` + bench group; `MutationDescriptor.conflict_rule`; all `absorb` docstrings referencing `protocol_crdt` (7 sites) rewritten.

## Fan-out recipe (per `🧬️mutations/<kind>/` triad; reference facet `✏️s/🔌️plugins/🕸️dag/…/🧬️mutations/`)
- `🔺️diff` leaf: `pub fn diff(payload, base) -> protocol::MutationOutcome<XDiff>`; detect per the verb-family table below; `MutationOutcome::new(diff)` on success; `.info("mutation.cascade", ..)` for cascades.
- `🦠️mutation` leaf: only the return type of `fn diff` changes; delete any `fn validate` override (its checks move into the diff leaf as Error/Fatal). `↩️inverse` untouched (`Vec::new()` on missing target now matches the Error rule). Composite `🧩️plan` kinds: nothing beyond the derive.
- Dispatch enums: 131 derived — nothing hand-written; hand `impl Mutation<..>` (config/presence enums, framework internals: `🌊️flow/🌿️vcs FlowMutation`, `🪐️space`, `🏪️store SpaceHistoryMutation`, `♾️infinite … DagMutation`, `🔁️workflow` Workflow/Run, `💻️os/🎚️config OpeningConfigMutation`, `🗄️stdio` 34 legacy artifact enums, `🧩️puzzle` 3) get the return-type change; stdio's legacy enums are FULL-STDIO's charter — our lane wraps them minimally and records a `sharedFileRequest` if that ticket is mid-flight.
- Verb-family rules (frozen): create ⇒ Fatal `duplicate-id` on existing id / `invariant` on unknown container; delete(+plural) ⇒ Error absent / Warning `partial`; insert ⇒ Warning `clamped`; remove ⇒ Error; add ⇒ Error owner absent / Warning `no-op` present; rename ⇒ Error / `no-op` same / Fatal key collision; change/set/update ⇒ Error / `no-op` same / Fatal domain; move/drag/rotate/scale/resize ⇒ Error / `no-op` / Fatal non-finite or non-positive; reorder ⇒ Error index / `no-op` / Fatal non-permutation; edit/replace ⇒ Error / `no-op`; duplicate ⇒ Error source / Fatal target exists; connect/bind ⇒ Error endpoint / `no-op` parallel-forbidden / Fatal dup id, cycle, incompatible; disconnect/unbind ⇒ Error; group ⇒ Error none / `partial` / Fatal dup or ancestor; ungroup ⇒ Error; flatten/unflatten ⇒ Error / `no-op` / Fatal dangling; split ⇒ Error / Fatal <2 parts or collision; merge ⇒ Error <2 / `partial`; extract/inline ⇒ Error / Fatal collision; clear ⇒ `no-op` empty; fix/toggle/apply ⇒ Error / `no-op`; domain verbs analogously. Total kinds (root-scoped `clear-*`, root `change-<artifact>-<field>`) may return message-free outcomes via a shrink-only allowlist.
- Tests per facet `🧪️Tests`: `assert_missing_target_is_error`, `assert_fatal_never_applies`, `assert_outcome_policy_matrix` (one per verb family per facet), existing `assert_mutation_inverse_law` also asserts no Error/Fatal in the forward outcome.

## Testkit laws (`📡️spr/🧪️testkit/🦀️component.rs`, region `🔖️Laws`)
`assert_missing_target_is_error`, `assert_fatal_never_applies`, `assert_outcome_deterministic`, `assert_policy_matrix` (3×4), `assert_merge_convergence`, `assert_modify_vs_delete(policy)` (Normal/Vigilant ⇒ quarantined & state pre-merge; LaissezFaire ⇒ applied, Degraded conflict, Error message, part absent), `assert_chronological_determinism` (any arrival order ⇒ same state/order/conflicts), `assert_quarantine_accept_equals_laissez_faire`, `assert_quarantine_discard_preserves_state`, `assert_ledger_matches_replay`, `assert_conflict_spr_round_trip`, channel frame round-trip corpus (Rust + vitest).

---

## Frozen tag allocation (verified against the live tree by the coordinator, W0)

The peer ticket `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` is
**closed** (`🎫️ticket.json.status == "closed"`), so its tag range is final and nothing needs to be
renegotiated. The coordinator counted the live declaration order and confirms every ordinal the plan
froze is in fact the next free one:

| Enum | File | Live variant count | Frozen new ordinals |
|---|---|---|---|
| `AppCommand` | `📡️spr/🧵️channel/🦀️component.rs:55` | 30 (tags 0–29, last `ClearDefaultApp`) | `SetMergePolicy` = **30**, `ResolveConflict` = **31**, `ReadConflicts` = **32** |
| `AppFrame` | `📡️spr/🧵️channel/🦀️component.rs:232` | 23 (tags 0–22, last `TransactionRolledBack`) | `MergeReport` = **23**, `Conflicts` = **24** |
| `ArtifactCommand` | `🏪️store/🦀️component.rs:191` | 15 (ordinals 0–14, last `PruneDrafts`) | `SetMergePolicy` = **15**, `ResolveConflict` = **16** |
| `CHANNEL_VERSION` | `📡️spr/🧵️channel/🦀️component.rs:20` | `10` | → **11** (bumped exactly once, by lane 1-C) |

**Wire law (inherited, unchanged):** `tag: u8` = variant declaration order, fields in declaration
order, no per-field tags, new variants **appended only**. `Invocation.messages` and `Error.report`
are **trailing** field additions to existing variants — the tag does not change.

Only lane **1-C** may bump `CHANNEL_VERSION`, and only from 10 to 11. Any other lane that finds
itself needing a wire change stops and reports.

## Severity is the one level vocabulary

`Hint` is gone repo-wide. `Severity { Info, Warning, Error, Fatal }` in declaration order, so
`#[derive(PartialOrd, Ord)]` IS the level order and `as_u8/from_u8` are 0..3. Any lane that finds a
`Hint` in its lease rewrites it to `Info`; a `Hint` outside every lease is reported to the
coordinator, not edited.

## The three merge policies

| Policy | `rejects(level)` is true for |
|---|---|
| `LaissezFaire` | `Fatal` |
| `Normal` (default) | `Error`, `Fatal` |
| `Vigilant` | `Warning`, `Error`, `Fatal` |

A policy is **local/authority state**. It is never carried on a `MutationEnvelope` or a
`BackboneMessage`, and it is never part of an artifact's shared history.

## No improvisation clause

- No code outside the frozen 7 may be introduced. There are **no per-plugin codes**. A lane that
  believes a leaf needs an eighth code reports it — the coordinator either maps it onto one of the 7
  or takes the decision to the dev.
- No `validate` may survive anywhere: not on `Mutation`, not on `MutationKind`, not on
  `CompositeMutationKind`, not as an override in any leaf. Its checks move into the `🔺️diff` leaf as
  `Error`/`Fatal` messages.
- No compatibility shim, no staging method, no deprecation, no adapter: the `diff` return-type change
  is atomic (CLAUDE.md forbids compat layers). Plugin crates are legitimately red between the W0
  barrier and their W3 lane.
- CRDT vocabulary (`merge_strategy`, `MergeStrategyKind`, `merge_concurrent_diffs`, `ConflictRule`,
  `ResolutionPlan`, `assert_crdt_*`) must reach **zero** occurrences outside `.🦑️repo/`.
