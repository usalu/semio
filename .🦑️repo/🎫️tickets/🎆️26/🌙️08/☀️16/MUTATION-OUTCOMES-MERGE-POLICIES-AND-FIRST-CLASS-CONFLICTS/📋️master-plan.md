# Mutation Outcomes, Merge Policies and First-Class Conflicts

## Context

Today a mutation's `diff(base)` is silent: a missing/deleted target is a silent no-op (`🌿️vcs` collection helpers `retain`/`find`, taxonomy rule "missing target ⇒ inverse returns `Vec::new()`"), `validate` returns a bare `Result<(), String>` that the derive never even generates, `Mutation::reconcile` produces a `ReconcileReport{Info,Warning,Blocking}` whose severity is dropped when mapped to `SpaceConflict{kind,uri,message}`, and remote edits are **appended** in arrival order by `ArtifactStore::ingest_remote` (`🏪️store/🦀️component.rs:4395`) — never re-ordered by timestamp, never re-evaluated. A parallel CRDT layer (`📡️spr/🔀️crdt`, `MergeStrategyKind`, `ConflictRule`, `db_conflict::ResolutionPlan`) exists but is unreachable from the store path and contradicts CLAUDE.md ("MUST NOT use CRDTs").

Target: every mutation produces a **diff along with messages** (Info/Warning/Error/Fatal); **conflicts are first-class** (typed, persisted, resolvable); three **merge policies** decide what a message level prevents (LaissezFaire: only Fatal; Normal: Error+Fatal; Vigilant: Warning+); alone = illegal actions prevented up-front and rejected atomically; collaborating = remote edits merged **chronologically by HLC**, the affected suffix re-evaluated, degraded/quarantined merges surfaced as conflicts — end to end through guest store, both hosts, TS shell UI (en/de), hub + db authority. The CRDT layer is deleted; `Severity` (`Hint`→`Info`) becomes the one level vocabulary.

Decisions confirmed by the dev: delete CRDT layer; `diff()` returns `MutationOutcome{diff,messages}` (single pass, `validate` removed); rejected remote merges are **quarantined** as open conflicts; reuse `Severity` with `Hint→Info`.

Concurrent live tickets on the same files (region-lease discipline mandatory): `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` (W2 live: `🏪️store` Composition/Transaction regions, `🔌️plugin` Emit/Exchange, `🖥️host`, channel tags 19–26), `26/08/16/FULL-STDIO-…` (`🗄️stdio` artifacts, `🔌️plugin`, WIT), `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` (`🔌️plugin` app ids).

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
pub enum ConflictKind { Quarantined { policy: MergePolicy, envelopes: Vec<MutationEnvelope> }, Degraded { edit_ids: Vec<String> } }
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
8. Reject (`policy.rejects(worst)`): dag not advanced, state unchanged, push `Conflict{Quarantined{policy, envelopes}, Open, messages}` → `MergeReport{accepted:false}`.
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

## Repo gates / launch / nx (coordinator-only lease)
Root `📜️script.ts` `policy…` region: `policyMutationOutcomeBreaches` (every `🔺️diff` leaf returns `MutationOutcome<` and references ≥1 frozen code unless in `POLICY_MUTATION_TOTAL_KIND_ALLOWLIST`, shrink-only), `policyMutationMessageCodeBreaches` (only the 7 codes), `policyNoCrdtVocabularyBreaches` (`merge_strategy|MergeStrategyKind|merge_concurrent_diffs|ConflictRule|ResolutionPlan|assert_crdt_` ⇒ 0 outside `.🦑️repo/`), `policyNoValidateOverrideBreaches`, `policySeverityInfoBreaches` (no `Hint`), `policyMergePolicyParityBreaches` (Rust spine, TS host codec, TS kernel, both i18n bundles), `policyDeriveMirrorBreaches` (`✨️derive/🦀️component.rs` byte-identical to `✨️derive/📦️packages/🦀️rust/📦️glue.rs`); rolled into `verify gate`. `.vscode/launch.json` group `4_gate`: `⚖️gate🎯️mutation-outcome` (410.11), `⚖️gate🧹️no-crdt-vocabulary` (410.12), `⚖️gate🛡️merge-policy-parity` (410.13); group `3_dev` after `🛠️dev🖥️s⚛️react`: `🛠️dev🖥️s⚛️react🛡️policy-vigilant` (`SEMIO_MERGE_POLICY=vigilant`). `📋️project.json`: `mutation-outcome-law` next to `stdio-mutation-law`; `verify-gate` extended. Taxonomy `🔣️taxonomy.json` unchanged (triad shape unchanged).

## Workforce (Fable = author of this plan, not an executor)

Roles: **1 Opus 5 coordinator** (opens ticket, writes `📋️contract-freeze.md` + `📋️ownership-and-handoffs.md` + `📌️important.md` (workspace-break window notice with the one-line adaptation `MutationOutcome::new(..)` for other sessions), allocates leases, spawns lanes, runs serial cargo/nx gates at barriers, reviews reports, closes the ticket with explicit path + full file list, clears `📌️important.md` last). **Sonnet 5 executors** (one per lease; write only inside the lease; region-local `Edit`, re-read region right before every edit, never whole-file rewrites, never revert foreign changes; report to `📓️<lane>-report.md`; never `ticket_close`; no worktrees; no git-modifying commands; `.txt` not `.log` for scratch). **Haiku 4.5 Explore scouts/auditors** (read-only) before W1 and at every barrier. Rule from memory: coordinator spawns workers with `run_in_background: false`, many `Agent` calls per message; never `isolation: worktree`.

Ticket: `ticket_open` goal `🎯aioptimizedrepo`, client `claude-code`, llm `fable-5`, emoji `💬️`, title "Mutation Outcomes, Merge Policies and First-Class Conflicts"; copy this plan as `📋️master-plan.md`.

**Compile-window rule:** the `diff` return-type change is landed atomically (no staging methods — CLAUDE.md forbids compat layers). Plugin crates are red from W0 until their W3 lane finishes; therefore **W3 starts immediately after the W0 barrier, in parallel with W1/W2**, and `📌️important.md` tells other sessions how to adapt.

| Wave | Lane | Model | Exclusive lease | Deliverable / acceptance |
|---|---|---|---|---|
| W0 | 0-A kernel spine | Sonnet | `📡️spr/🎮️command`, `📡️spr/🧾️wire`, `📡️spr/🦀️component.rs`, `📡️spr/🔀️crdt` (delete), new `📡️spr/⚔️conflict`, `🗣️dsl/⚠️diagnostic`, `🕸️graph/🗣️dsl`, `🌿️vcs` (`apply_mutation`, `Errors`), mechanical return-type adaptation of every in-crate `impl Mutation/MutationKind` (`🏪️store` `replay_mutations`/`ingest_remote` minimal wrap, `🪐️space`, `🔁️workflow`, `🌊️flow/🌿️vcs`, `♾️infinite … dag`, `💻️os/🎚️config`, `📡️spr/🧪️testkit` fixtures + delete crdt helpers/bench), glue `📦️glue.rs` `#[path]` | C1–C5, C10; kernel crate compiles; `cargo test -p semio-framework-os-kernel --lib -- os_spr::command` green |
| W0 | 0-B derive | Sonnet | `🗣️dsl/✨️derive/🦀️component.rs` **and** `🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` (byte-identical) | `#[derive(Mutations)]`/`#[derive(CompositeMutation)]` emit outcome-returning `diff`, no `validate`, no `ConflictRule` in `register_calls`; MiniMutation fixture in command tests green |
| W0 | 0-I | Opus + Haiku×1 | ticket docs, tag allocation letter to PLUGIN-DEPENDENCIES ticket | contract freeze; barrier: `cargo check -p semio-framework-os-kernel -p semio-framework-os-kernel-dsl-derive`; Haiku audit: 7-code/policy-matrix parity contract↔code |
| W1 | 1-A store | Sonnet | `🏪️store/🦀️component.rs` regions `🔖️ArtifactStore`, `🔖️Authority`, `🔖️Schemas` (ArtifactCommand), `🔖️Backbone`; `📡️spr/🔗️causal` | C6 full: clock, HLC invariant, chronological `ingest_remote`, quarantine/resolve, ledger, `SetMergePolicy`/`ResolveConflict`, two-peer tests |
| W1 | 1-B history | Sonnet | `📡️spr/📜️history/**` | C7 + round-trip tests |
| W1 | 1-C channel | Sonnet | `📡️spr/🧵️channel/🦀️component.rs`, `💻️os/🟦️component.ts` (`AppChannelCodec` region + tests), `🧫️fixtures/📡️channel/**`, `📡️wire` `ApplyOutcome` | C8 + golden vectors both sides (after PLUGIN-DEPENDENCIES 0-B closure, confirmed by coordinator) |
| W1 | 1-D testkit | Sonnet | `📡️spr/🧪️testkit/🦀️component.rs` `🔖️Laws` | all law helpers above + self-tests |
| W1 | 1-E composition | Sonnet | `🏪️store` `🔖️Composition`/`🔖️Space`/`🔖️CompositionCoordinator` (negotiated with PLUGIN-DEPENDENCIES ticket) | `preview_wire`, phase-1 policy gate, `GroupReceipt.messages` |
| W1 barrier | Opus + Haiku×2 | | `cargo test -p semio-framework-os-kernel --lib`, `bun nx run @semio-tech/os:test`; audits: derive mirror identity, ledger-vs-replay law | |
| W2 | 2-A guest SDK | Sonnet | `🔌️plugin/🦀️component.rs` regions `VcsArtifactApp`, `Emit`, `Exchange`, `plugin_runtime`, testkit; `🔌️plugin/🏗️builder` | C9 guest: report frames, preview, policy commands, testkit txn tests adapted |
| W2 | 2-B Rust host | Sonnet | `🔌️plugin/🖥️host/🦀️component.rs`, `🏃️run/**` | decode/expose messages, policy config, wasmtime e2e (delete-missing under Normal ⇒ Error frame + unchanged doc; under LaissezFaire ⇒ applied + Degraded conflict) |
| W2 | 2-C TS host+kernel | Sonnet | `💻️os/🟦️component.ts` (`AppChannelClient`, public api), `🎠️kernel/🟦️component.ts`, renderer boot | policy plumb, `faultMessages`, vitest parity vs golden vector |
| W2 | 2-D shell UI + i18n | Sonnet | `📺️renderer/…/🧱️elements/{ChromePanels,EventFeedHost,DiffViewHost,ShellSync,ShellHost,Shell}`, `🖱️ui/🧱️elements/📜️HistoryTable`, `🖱️ui/…/⚛️react/📦️index.tsx` (i18n only), `💻️os/🎚️config/🧬️schema/**` | Conflicts panel, toasts, feed tones, history badges, policy setting, en+de |
| W2 | 2-E hub + db | Sonnet | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, `🛢️db/{📄️artifact,⚔️conflict,⌨️cli,👁️preview}` | C9 hub/db; `cargo test -p semio-framework-os-kernel-db --lib` |
| W3 (starts after W0 barrier) | 3-A..3-H plugin fan-out | Sonnet ×8 | 3-A `📕️norm` {din16798,en1998,en1999,en1997} (159); 3-B `📕️norm` {en1992,en1991,en1996,en1994,en1990} (121); 3-C `📕️norm` {din4108,iso16757,en1995,vdi3805,en1993,din18599} (112); 3-D `🏛️architect` (266); 3-E `🗄️stdio` (125 + 34 legacy enums, coordinate FULL-STDIO); 3-F `🧱️block`+`🧩️puzzle` (193); 3-G `🏗️fem`,`🌀️procedural`,`📸️remodel`,`🗒️note`,`🎥️shooting` (185); 3-H remaining 20 plugins (239). Norm lanes never touch `📕️norm/🎚️config` or `👥️presence` (coordinator) | recipe applied to every leaf, `validate` overrides removed, facet tests, `cargo test -p semio-s-plugin-<id> --lib` green per lane; report `📓️w3-<lane>-<plugins>-report.md` |
| W2/W3 barrier | Opus + Haiku×2 | | `cargo check --workspace` (serial, long), per-plugin tests, `bun nx run-many -t test` for touched TS packages, `bun ./📜️script.ts verify gate`; audits: Rust↔TS codec parity, en/de completeness | |
| W4 | 4-I gates/launch/close | Opus + Haiku×3 | root `📜️script.ts` policy region, `.vscode/launch.json`, `📋️project.json`, ticket docs | gates + launch entries; audits (1) schema/taxonomy/open-closed, (2) CQRS/atomicity/determinism/security, (3) evidence honesty; remediation leases; `ticket_close` explicit path + full file list |

Sizing: W0 = 2 Sonnet + Opus; W1 = 5 Sonnet; W2 = 5 Sonnet; W3 = 8 Sonnet (overlapping W1/W2); W4 = Opus + 3 Haiku. Keep ≤ 8 concurrent writers; cargo gates never in parallel; repo-wide cargo failures may be another live session's churn (attribute via `git log --date=iso` before chasing).

Haiku scout questions before W1 (do not block W0): whether PLUGIN-DEPENDENCIES lane 0-B/2-B has closed on `AppChannelCodec`/`AppChannelClient`; whether FULL-STDIO is mid-conversion on any legacy stdio enum; which shell element currently owns the settings surface for the policy toggle; how the TS shell keeps `DocumentChanged` handling so `MergeReport` can be routed to `EventFeedHost`/`ChromePanels`.

## Verification (end to end)
1. Unit laws: `cargo test -p semio-framework-os-kernel --lib`, `cargo test -p semio-framework-os-kernel-db --lib`, `cargo test -p semio-framework-plugin --lib`, `cargo test -p semio-framework-plugin-host`, `cargo test -p semio-s-plugin-<id> --lib` for all 33 plugins.
2. Two-peer store tests (`🏪️store` `🧪️Tests`): modify-vs-delete under each policy; chronological determinism over permuted arrival; quarantine→Accept == LaissezFaire; Discard preserves state; ledger == replay; `.spr` conflict round-trip.
3. Channel: Rust + vitest golden vectors decode identically (`bun nx run @semio-tech/os:test`).
4. wasmtime host e2e: plugin deleting a missing id ⇒ `Invocation.messages` carries `mutation.target-missing`… under Normal ⇒ `AppFrame::Error` + report and unchanged document; under LaissezFaire ⇒ applied + `Degraded` conflict frame.
5. Browser proof via the preview browser on `🛠️dev🖥️s⚛️react` (`🕸️dag`): delete a node twice ⇒ second gesture rejected under Normal with toast (en, then de), Vigilant rejects a same-value rename (Warning), LaissezFaire records a Degraded conflict visible in the Conflicts panel; `[DEBUG]` console logs of decoded `MutationMessage`/`MergeReport` captured into the ticket folder as `.txt`, then removed. Two-tab session on the same doc: concurrent modify vs delete ⇒ quarantine appears in the other tab, Accept/Discard converge both.
6. `bun ./📜️script.ts verify gate` ⇒ 0 breaches (new gates included); every lane report links its logs; `📓️final-summary.md`; ticket closed with explicit path.

## Kickoff (on approval)
1. Fable opens the ticket (`ticket_open`, goal `🎯aioptimizedrepo`, client `claude-code`), copies this plan to `📋️master-plan.md`.
2. Fable spawns exactly one Opus 5 coordinator (`Agent`, `general-purpose`, `model: opus`, `run_in_background: false`) with: plan path, ticket path, contract sections C1–C10, roster/lease table, shared-tree/barrier rules, "spawn Sonnet 5 (`model: sonnet`) executors per lease and Haiku 4.5 (`model: haiku`, `Explore`) scouts/auditors, run_in_background false, never worktrees, never git-modifying commands, subagents never call `ticket_close`; close the ticket yourself with the explicit ticket path and full file list; clear `📌️important.md` last".
3. Fable monitors, relays blockers to the dev, does not execute lanes itself.
