# 🔄️ Audit — transaction primitives for a domain-neutral `ToolTransaction`

Read-only audit for phase 4 (`📓️status.md` §Phase 4). Scope: what the framework/os product already provides toward
"a tool that mutates the artifact runs inside a transaction: provisional mutations visible while running, abort
rolls them back with zero trace, finalize commits them as one durable unit," in a local-first, event-sourced CQRS
(no CRUD/CRDT), multi-user architecture. All claims are `file:line`, gathered by four parallel read-only research
passes over the actual source (not the emoji-approximate paths given in the brief — real paths verified with
`find`/`rg` first). Unverified points are called out explicitly rather than guessed.

## 0. Two things with "transaction" in the name that are NOT this

- **`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🔄️transaction/🦀️.rs`** — `FrameTransaction`/`FrameTransactionStage`
  (`:294-303`: `DrainProjectionDeltas → RouteIntents → FlushEffects → PresentSurface → ReconcileTree →
  BuildRenderPackets → PublishSnapshot`). This is a **per-frame UI rendering pipeline**, scoped to one
  `UiRuntime<S, D>` embedder (`:180-198`), coordinating `EntityStore`/`DependencyTracker`/`SurfaceReconciler`/
  `CommandGateway`/`PresenceHub`. It has no history/undo concept (`grep -rniE "history|undo|redo"` over the whole
  `🧠️runtime` tree: **zero hits**), no artifact-document mutation, and no multi-user awareness. `cancel()`
  (`:723-729`) and `supersede()` (`:678-692`) discard staged **presentation/patch** work, not document mutations —
  proven by tests `cancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision`
  (`🧪️tests/🔬️transaction-unit/🦀️.rs:392-417`) and
  `repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command` (`:366-390`). **Not a
  candidate implementation for the dev's requirement — different layer entirely.**
- **`PendingTransaction<Op>` / `transaction_prepare`/`transaction_commit`/`transaction_rollback`** in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12695-12711` (struct), `:11928-11949` (trait methods),
  `:27724-27840` (impl) — this IS an existing begin/commit/rollback protocol, but it is scoped to **cross-artifact
  composite gestures** (one parent + N foreign/child artifacts, wire-framed as
  `AppCommand::TransactionPrepare/Commit/Rollback`, `AppFrame::TransactionProposal/Prepared/Committed/RolledBack`,
  handled `:35601-35660`), not to a single tool's own multi-turn provisional editing of one artifact. `prepare`
  computes a purely local in-memory `running` snapshot without touching `self.store` (`:27762-27777`); `commit`
  applies all ops as one `ArtifactCommand::Apply` batch stamped with `group_id = txn_id` (`:27804-27810`); `rollback`
  just clears `self.pending_transaction` (`:27816-27823`) — zero-trace because nothing was ever written. A freeze
  guard (`:22817-22825`) rejects any other mutating verb while a transaction is pending
  (`transaction.instance-busy` fault). **Reuse this vocabulary/shape, but it does not itself make intermediate
  state visible to reads** — its "running" snapshot is private to the dispatch call, never surfaced. Referenced
  spec doc `contract-freeze.md §5.2/§5.6/§5.8-§5.10` was not located in this pass (only `.rs`/`.ts` were searched).

## 1. Inventory table

| Primitive | file:line | What it does | Fits which requirement |
|---|---|---|---|
| `MutationEnvelope` | `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:38-46` | Wire unit: `mutation_id, document_id, actor, dependencies: Vec<MutationId>, diff: ArtifactDiff, inverse: InverseMutation, timestamp: HybridLogicalTimestamp`. Causal order via explicit dependency DAG, not a vector clock. | substrate for "one event batch" |
| `Edit<Op>` | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1471-1485` | "One coalesced batch of operations, forward and backward": `forwards: Vec<Op>`, `inverse: Vec<Op>`, `coalesce_key: Option<String>`, `sequence_number`, `started_at`, `finished_at: Option<String>`. **This is the durable undo/history unit — already N-mutations-to-one-entry shaped.** `finished_at: None` = still open. | finalize = ONE durable unit |
| `HistoryEdit` (on-disk form) | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:102` | Same shape as `Edit` (`ops: Vec<OpPayload>`, `inverse: Vec<OpPayload>`) persisted to the `.spr` journal. | durable storage of the finalized unit |
| `MutationMeta` | `🎮️mutation/🦀️.rs:1342-1385` | Per-op metadata: `undo_policy`, `label`, **`group_id: Option<String>`** ("Composite-gesture stamp... so group undo can find and reverse every sibling member together" — explicitly documented as groundwork for **"the future `CompositionCoordinator`"**, i.e. not yet built for this purpose), `origin: MutationOrigin`. | group-undo scaffolding (unbuilt) |
| `MutationOrigin::Transaction{initiator}` | `🎮️mutation/🦀️.rs:1570-1581` | Provenance tag: "applied as one member of a cross-artifact composite gesture initiated elsewhere." | provenance only, not an executable API |
| `coalesce_key` (`Edit`) | `🎮️mutation/🦀️.rs:1481`; grouping logic `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16606-16619` (`batch_amend_target`) and `:17262-17314`/commit sites `:16732,16752,16758` (`amend_command`) | Given a key, finds whether the **last uncommitted** edit has the same key; if so, appends `forwards`/`inverse`/`mutation_meta` into it instead of minting a new `Edit`. Repeat-key ticks of one gesture collapse into one history entry. **This is the real mechanism, already load-bearing** (puzzle's `"fill-count"`, generation2d's `"generation-values"` — `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/…/🎮️commands/🧬️generation/🦀️.rs:38`). | provisional-visible + coalesce-to-one-entry |
| `Emit::amend(mutations, coalesce_key)` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:10562-10565` | Doc: *"per-tick coalesced DOCUMENT emission. The `coalesce_key` folds every tick of one live gesture (drag/scrub) into a single amendable edit, so the whole gesture is one undo."* | **closest existing "provisional + visible + one entry" API** |
| `Emit::commit(mutations, description)` | `🔌️plugin/🦀️.rs:10568-10572` | Doc: *"gesture-end commit of an app-runtime scratch draft as one described DOCUMENT edit (coalesce_key: None)."* Pattern (b): hold a draft in app-runtime state, render as overlay, commit once. | finalize-only-when-complete pattern |
| `UtilityPreviewContract` | `🔌️plugin/🦀️.rs:11063-11080` | Formalizes: exactly one interactive utility active per window; switching utilities clears in-progress preview scratch; the two blessed patterns above (`amend` per-tick vs scratch+`commit`). | **the only documented begin→provisional→end contract in the repo** |
| `ArtifactStoreBatchPublication<P, Mutation>` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14462-14486` | Doc: *"ONE user gesture, N admitted mutations, ONE staged Edit, ONE history ledger slot, ONE undo step."* Phase machine `Preparing → PreparingCursor → PreflightingCommit → Publishing → AwaitingAck → Closing → Complete` (`ArtifactStoreOneItemPublicationPhase`, `:14334-14343`). | batch→one-entry, but **invisible until `Publishing`** |
| `fold_batch_item` | `🏪️store/🦀️.rs:16843-16918` | Folds one prepared item's forward/inverse into the staged `Edit`; nothing durable yet. | staging step |
| `advance_apply_batch` (Publishing phase) | `🏪️store/🦀️.rs:16624-16920`, commit at `:16741-16746`/`16796-16797` | Only here does `self.replace_current_retained(post)` run — the live/queryable snapshot flips atomically. Before this, staged mutations are private to `publication.stage`, **never exposed to any read path** (confirmed: no query surface found reading `publication.stage`). | atomic single-write finalize |
| `cancel_apply_batch` | `🏪️store/🦀️.rs:16830-16837` | Sets `cancel_requested`, retires the staged edit via `ArtifactStoreBatchStage::close_step` (`:14395-14440`) without ever having touched `self.current`. | genuinely zero-trace abort (because nothing was ever visible) |
| `DiffAlgebra::inverse` / `Mutation::inverse` | `🎮️mutation/🦀️.rs:155-162`, `181-182` | Diff-level and op-level inverse computation; law `d.inverse(base).apply(d.apply(base)) == base`. | rollback substrate |
| `UndoPolicy` | `🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs:175-213` | `ExactBaseOnly` (default), `TransformAgainstConcurrent`, `SemanticUndo`, `CompensatingAction` — governs how an undo/rebase is computed when the document moved on. | concurrency-aware undo |
| `undo_lane_position` / `redo_lane_position` | `🏪️store/🦀️.rs:17176-17193` / `17200-17216` | Undo: pops id from `applied_edit_ids`, restores cached pre-snapshot or full `fold_current()` replay, pushes id to `redo_edit_ids`. Redo: pops `redo_edit_ids`, replays `edit.forwards`. **Undo of a finished edit leaves it redo-able — this is NOT zero-trace, and is a different operation from aborting a still-open (unfinished) edit**, for which no dedicated command was found. | undo of *finalized* edits only — gap for open-edit abort |
| `StateClass` | `🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs:289-307` | `Artifact` (persisted shared), `Config` (persisted local-only), `Presence` (ephemeral shared), `Transient` (ephemeral local-only). Doc: *"the four — and only four — state mechanisms the architecture admits."* Mirrored in `🔌️plugin/🦀️.rs:9669-9685` and GraphQL schema `🧬️schema/⚛️component/🦀️.rs:127,776-793`. | **all 4 of §6 already named and enforced** |
| `PresenceStore<P, Mutation>` | `🏪️store/🦀️.rs:4000-4022` | Backs `StateClass::Presence`. Doc: "last-writer-wins roster, NOT an event log... no history, no undo, no checkpoints, no merge." | ephemeral-shared backing store |
| `TransientStore<P, Mutation>` | `🏪️store/🦀️.rs:4803-4816` | Backs `StateClass::Transient`. Doc: "Nothing here is ever shared, persisted, packed, checkpointed or undone." Examples given: "which pane is focused, what is hovered, **an in-flight gesture**." | ephemeral-local-only backing store |
| `UiPreferences` | `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️.rs:95-110` | Backs `StateClass::Config` (persisted local-only). | persisted-local-only backing store |
| `DraftMutation` / `DraftStore` | `🔌️plugin/🦀️.rs:11308-11311` (type param), `DraftView` doc `:8727-8732`, `DraftStore<P,Mutation> = ArtifactStore<P,Mutation>` `🏪️store/🦀️.rs:3759` | A **full second `ArtifactStore`** (same undo/edit machinery) per app, documented "ephemeral *artifact* content, not UI state... real document content that simply has not been committed" (`🔌️plugin/🦀️.rs:9678-9682`). Every plugin sampled defaults to `NoDraftMutation` — no real plugin found using a non-trivial draft lane. | closest structural match for a provisional **overlay of the main document**, but currently a separate snapshot, not integrated with the main artifact's finalize path |
| `Lane::{Command, Preview}` | `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:21-28` | `Command` = causally-ordered durable batches; `Preview` = "ephemeral, best-effort UI-state broadcast." `ClientFrame::PreviewPublish{key,seq,payload}` (`:52`), `ServerFrame::Preview{actor,key,seq,payload}` (`:543-548`), durable-log tag `REC_EPHEMERAL` (`🧾️wire/🦀️.rs:115-116`, "dropped freely by compaction"). | **existing cross-user provisional-visibility channel** — a tool's ticks could broadcast here without ever touching durable history |
| `ConflictKind::{Quarantined, Degraded}` | `🧰️framework/🔨️modules/📡️replication/⚔️conflict/🦀️.rs:60-71` | Quarantined = whole batch rejected, nothing applied, replayable later. Degraded = applied but flagged, "resolving only acknowledges/dismisses the flag, it never rewrites history." | conflict handling for the finalize step |
| `MergePolicy::{LaissezFaire, Normal, Vigilant}` | `🧾️wire/🦀️.rs:215-286` | Per-authority severity threshold for accept/quarantine, never wire-carried. | authority-side gate on finalize |
| `MutationDag` / `MutationTransform` | `🔗️causal/🦀️.rs:337-343`, `704-729` | Causal DAG ordering by explicit `dependencies`; `TransformOutcome::{Unchanged,Transformed,Conflict}` — the rebase hook matching `UndoPolicy::TransformAgainstConcurrent`. | rebase against concurrent edits |
| `resume_token` / `FrontierSummary` / `Bootstrap::{Tail, Snapshot}` | `📡️wire/🦀️.rs:49, 519-525, 61-67`; `🔗️causal/🦀️.rs:546-552` | Reconnect handshake: client presents last frontier, server replies `Tail` (short-gap incremental catch-up) or full `Snapshot`/`ArtifactBootstrap` (cold start). No explicit "short outage" time/threshold constant found. | survives short connection shortages (generic, not tool-specific) |
| `with_scratch_session` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:187-195` | **False lead.** A throwaway `FlowEvalSession` (flow-graph node-eval cache: mesh identities, tessellation handles) used only when no retained owner exists; retired immediately via `begin_close`+`close_step` loop. Nothing to do with mutations. | none — name already taken by an unrelated concept |

## 2. Existing begin → provisional → commit/cancel flows

| Flow | Begin | Provisional/visible state | Cancel — trace? | Commit — atomic? | Notes |
|---|---|---|---|---|---|
| **Puzzle 3d gumball drag** | `transformBegin` view action, `ActionKind::View`, `✏️editor/🦀️.rs:2406-2407,8468-8469`; UI trigger `handleGumballDragStart`, `🌐️World3dHost/🟦️.tsx:6450-6458` | Live pose in a **client-local React module singleton** (`worldnByController` map, `setWorldn`/`getWorldn`), not document/network state at all | `transformBegin`/`transformEnd` are `puzzle3d_shell_only_emit` → `Emit::default()` (`✏️editor/🦀️.rs:3502`) and `ArtifactToolPublicationLane::HostOnly` (`:7108-7109`) — zero trace by construction. Explicit cancel gesture (e.g. Escape) **not found/unverified**. | Yes — `handleGumballDragEnd` (`World3dHost/🟦️.tsx:6464-6475`) dispatches **one** absolute start→end delta as a normal mutation command; comment: *"One absolute start→end delta — the app commits it directly."* | Cleanest existing "abort=zero-trace, commit=one-entry" precedent, but entirely client-side/synchronous — no multi-turn provisional-visible-to-others state |
| **Puzzle 3d single-arm dispatch (`ctx.abort`)** | Every dispatched action clones `scene`, snapshots `before` (`✏️editor/🦀️.rs:3380-3454`) | The command mutates the in-memory scene clone directly during its own call | `ctx.abort = true` → `Emit{effects,..Default::default()}`, discarding every scene mutation (`:3406-3414`, comment: *"An aborted arm emits no document/config delta."*) | `operations = puzzle3d_operations_from_fixture_change(before, &scene.fixture)` (`:3416-3417`) — diff of the whole call becomes one `Emit` | Synchronous single-turn only; not a multi-turn provisional session |
| **Energy simulation "adopt"** | `StartSimulation`→`EnergySimulationEventKind::Start`, `🧵️simulation-session/🦀️.rs:1851-1887`; verbs `Start/Configure/Cancel/Retry/Discard/Adopt` all `ArtifactToolPublicationLane::HostOnly` (`✏️editor/🦀️.rs:950`) | `EnergySimulationProjection`/`EnergyTierProjection` (`:150-164`), live-updated per packet (`install_preview`, `:1071-1098`) — visible to UI while running, entirely session-local (`is_session_command` gate, `✏️editor/🦀️.rs:812-827`: "every document verb falls through to the model reducer" i.e. session verbs never touch the document) | `Cancel`/`Discard` (`:1906-1944`) only retire the in-memory job/session shell — **no document mutation exists to roll back**, so trivially zero-trace | `AdoptSimulation` only sets `adopt_requested=true` (`:1946-1955`); actual finalize happens in `close_step`/retirement (`:1262-1286`), acknowledging exactly one commit packet — **whether/how the result becomes a document mutation was not traced within budget; flagged unverified** | Best existing state-machine vocabulary (`Idle/Admitting/Queued/Running/.../FinalReady/Adopted/Closing`) and identity-staleness guards (`MountedIdentity::matches_render/matches_request`, `:205-216`) for rejecting stale requests |
| **Puzzle 2d fill `AwaitingAdoption`/`Applying`** | `PUZZLE2D_FILL_SESSION_ACTIONS`, `🎮️commands/🧮️set-fill-count/🦀️.rs:24`; lifecycle enum `🎚️config/🦀️.rs:46-60` (`Idle,Capturing,Queued,Running,CheckpointReady,Applying,AwaitingAdoption,Closing,Completed,Cancelled,Faulted,Discarded`) | Runtime counters on a visible runtime struct, mapped to localized labels (`🗣️terminology/🦀️.rs:109`) | **Violates the requirement**: `"brushFillSessionCancel"`/`"brushFillSessionDiscard"` (`:1658-1666`) only reset the *runtime*; explicit doc `:1623-1626`: *"discarding the runtime IS discarding the session... the placements a previous session accepted are already committed to the document"* (`:1634-1636`) — **placements are pushed to the document per-step, before cancel/adopt, and cancel does NOT roll them back** | **Not one entry**: each `brushFillSessionStep` call flushes its own `Emit` (`std::mem::take(&mut self.mutations)`, `:952`, `Complete(Emit{...})` at `:955`) — a full session spanning N steps produces N history entries. `"brushFillSessionAdopt"` (`:1654-1657`) emits nothing further, it only flips a UI-lifecycle flag | **This is the concrete counter-example the dev's requirement 2 is meant to fix** — flag directly: today's fill pattern is "commit-as-you-go, adopt is cosmetic," the opposite of provisional-then-atomic-commit |
| **Procedural generation3d preview-eval** | Tick chain begun via `⏱️flow-eval-tick` (viewer command, located not fully read) | `Generation3dViewTransient.preview_eval_text` (`👁️viewer/🫧️transient/🦀️.rs:21-24`) — explicitly doc'd "ephemeral LOCAL-ONLY state" | `cancel-preview-eval/🦀️.rs:13-18`: `Ok(Emit{extension_invocations,..Default::default()})` — explicit doc: *"Emits no mutation."* Verified zero-trace. | **No adopt/commit concept exists** (`rg -ni "adopt"` under generation3d: zero hits, twice). Document changes happen only via ordinary one-shot commands (`add-generation`, etc.), entirely decoupled from this preview machinery | Prior audit `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️preview-eval-cancellation-2026-09-12.md` documents a real bug class directly relevant here: **a guest-emitted cancel queued behind the very work it meant to stop** (per-actor request serialization); the fix moved abort to host-side `AbortSignal`. **Any generalized abort must be host-driven, not guest-emitted-and-queued.** |

## 3. The precise gap

No existing primitive gives, together, in one place: **(a)** provisional mutations that are real/visible on the
live document while a multi-turn tool runs, **(b)** an abort that removes them with zero trace (not even a redo
entry), and **(c)** a finalize that is exactly the caller's own choice of "now" rather than an implicit boundary.
Two halves already exist separately:

- **Visibility while running** is solved by `Emit::amend(mutations, coalesce_key)` (`🔌️plugin/🦀️.rs:10562-10565`) —
  each tick already writes into the live document (`self.current`) via `batch_amend_target`/`amend_command`
  (`🏪️store/🦀️.rs:16606-16619`), so intermediate state is genuinely visible, not staged-and-hidden like
  `ArtifactStoreBatchPublication`.
- **Coalescing to one durable unit** is also solved by the same mechanism: repeated same-`coalesce_key` ticks fold
  into one still-open `Edit` (`finished_at: None`).
- **What is missing is abort of that still-open `Edit`.** The only removal operation found,
  `undo_lane_position` (`🏪️store/🦀️.rs:17176-17193`), is built for **already-finished** edits and explicitly
  preserves them for redo (`redo_edit_ids.push`) — i.e. running `undo()` on an in-progress amend chain would leave
  a redo-able trace, which fails "no trace in durable history." No `ArtifactCommand::AbortOpenEdit` (or equivalent)
  exists anywhere I found — a genuine gap, not a naming issue.
- **Cross-user provisional visibility** ("ideally flagged as provisional/ephemeral-shared") has an existing wire
  channel (`Lane::Preview`/`PreviewPublish`, `📡️wire/🦀️.rs:21-28,52,543-548`, tagged `REC_EPHEMERAL` and "dropped
  freely by compaction") that nothing currently wires a tool's amend-ticks through — today an `amend` tick's
  mutation, once applied to `self.current`, would presumably replicate as a normal (durable) `Command`-lane
  envelope to other clients, which is the opposite of "provisional/ephemeral" for anyone but the local user.
  **Unverified**: I did not trace whether `amend`-lane mutations are in fact broadcast over `Command` immediately
  per tick or only at some later sync point — this needs confirming before design, since it changes how bad the
  gap is for the multi-user requirement.
- **Concurrency/rebase** and **short-disconnect survival** are architecturally already generic (`MutationDag`,
  `MergePolicy`, `UndoPolicy::TransformAgainstConcurrent`, `resume_token`/`FrontierSummary`/`Bootstrap::Tail`) and
  need no new mechanism — a finalized tool-transaction `Edit` is just another `Edit` to this machinery. The one
  soft spot: no explicit "how long is a short shortage" constant was found in `📡️replication`; if one is needed
  it likely belongs at the hub/session-timeout layer, not in the transaction primitive itself.

## 4. Proposal — smallest domain-neutral `ToolTransaction`

Build on what exists rather than adding a new document-mutation model:

1. **Naming.** Do not reuse "transaction" bare (already means the UI frame pipeline in `🧠️runtime/🔄️transaction`)
   or "scratch session" (already means the generation3d flow-eval cache). Call the new SDK-facing primitive
   `ToolTransaction`, and its lifecycle enum `ToolTransactionLifecycle` — deliberately distinct from
   `PendingTransaction`/`TransactionProposal` (§0, the cross-artifact composite-gesture protocol, which this can
   sit beside but must not collide with).

2. **Begin.** A tool calls a new SDK helper (proposed:
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-transaction/🦀️.rs`, sibling to `🧵️retained-command/`)
   that mints a `txn_id` and a derived `coalesce_key = format!("tool-txn:{txn_id}")`. Every subsequent tick calls
   the existing `Emit::amend(mutations, Some(coalesce_key))` — no change needed to `Emit`, `amend_command`, or
   `batch_amend_target`; this is the "provisional mutations applied and visible" requirement, already built.

3. **Cross-user provisional visibility.** Route amend-lane ticks belonging to an open `ToolTransaction` over the
   existing `Lane::Preview`/`PreviewPublish` channel (`📡️wire/🦀️.rs:21-28,52`) instead of `Lane::Command`, keyed by
   `txn_id`, so peers see live progress as best-effort/droppable state (`REC_EPHEMERAL`) — matching "ideally
   flagged as provisional/ephemeral-shared" — without it ever entering durable history. **This needs verifying**
   against whatever currently decides `Command` vs `Preview` lane routing for `amend` (not traced in this pass).

4. **Abort — the actual gap to fill.** Add `ArtifactCommand::AbortOpenEdit { coalesce_key: String }` (new variant,
   store-side, `🏪️store/🦀️.rs`) that: finds the tail edit whose `coalesce_key` matches and whose `finished_at` is
   `None` (open) and which is not yet in a `Change`/checkpoint (same guard `batch_amend_target` already computes,
   `:16606-16619`); removes it from `applied_edit_ids` **without** pushing to `redo_edit_ids`; refolds `current`
   via `fold_current()` over the remaining ids (same machinery `undo_lane_position` already uses, `:17176-17193`,
   just without the redo side-effect); and never writes anything to the `.spr` durable history journal for that
   edit (it was never sent as a `Command`-lane envelope per point 3, so there is nothing to retract on other
   peers either). This is the one genuinely new piece of code; everything else reuses existing primitives.

5. **Finalize.** The tool calls `Emit::commit(mutations, description)` on its last tick (or simply lets the open
   `Edit` stand and marks it finished by dispatching with a *different* `coalesce_key`/`None` on the next
   unrelated action, per the existing implicit-closure behavior of `amend_command`) — at that point the coalesced
   `Edit` is exactly the "one durable unit"/"one history/undo entry," and (per point 3) is the *first* point at
   which the mutation batch is sent as a real `Command`-lane `MutationEnvelope` batch to other clients — so
   finalize is also naturally the point where normal conflict handling (`⚔️conflict` Quarantine/Degraded,
   `MergePolicy`, `MutationDag` causal ordering, `UndoPolicy::TransformAgainstConcurrent` rebase) applies unchanged,
   satisfying "concurrent edits by other users during the run are handled" without any new conflict logic — the
   run itself was invisible to the durable/conflict machinery until this moment.

6. **Complete-only finalize.** "Finalize only when complete" is enforced the same way `dispatch_emit`'s freeze
   guard already enforces exclusivity for `PendingTransaction` (`🔌️plugin/🦀️.rs:22817-22825`): while a
   `ToolTransaction` with a given `txn_id` is open, reject a `finalize` call whose tool-reported status isn't
   `Complete` — reuse the `WindowMeasure::Progress`/lifecycle vocabulary phase 3 already added, or the
   `EnergySimulationStatus`-style enum (`🧵️simulation-session/🦀️.rs:118-129`) as the template for a
   `ToolTransactionLifecycle { Idle, Running, Complete, Aborting, Finalizing, Finalized, Aborted, Faulted }`.

7. **Short-disconnect survival.** Falls out for free: all ticks before finalize are local-only (amend into
   `self.current`, optionally best-effort-broadcast per point 3); only the single finalize dispatch needs
   connectivity, and it goes through the existing `resume_token`/`FrontierSummary`/`Bootstrap::Tail` reconnect path
   (`📡️wire/🦀️.rs:49,519-525,61-67`) like any other durable command — no new resilience code needed.

8. **Schema-first placement.** `ToolTransactionLifecycle` and the wire-visible progress/status object belong in
   `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/🔣️.json` (framework-level, domain-neutral, alongside
   `MutationMeta`/`Edit`'s own schema) so every plugin gets it for free; `AbortOpenEdit` and the `ToolTransaction`
   SDK wrapper are necessarily os-product-level (`🏪️store`, `🔌️plugin`) since that's where `ArtifactStore`/
   `ArtifactApp` already live — this mirrors how `StateClass` (framework-level enum) is consumed by os-level
   `PresenceStore`/`TransientStore`/`UiPreferences` today.

### What this deliberately does NOT add

No new document-mutation model, no CRDT, no new durable log format, and no change to `Edit<Op>`/`MutationEnvelope`/
`coalesce_key`/conflict machinery — every piece except `AbortOpenEdit` and the Preview-lane routing for amend-ticks
already exists and is exercised in production plugin code today.

## 5. Explicitly unverified (flagged, not guessed)

- Whether `amend`-lane mutations are currently broadcast to other clients over the durable `Command` lane
  immediately (making today's live-preview-via-amend already leak provisional state into other users' durable
  history) — not traced.
- Whether the energy simulation's `Adopt` finalize step actually writes a document mutation, and where — the
  2381-line `🧵️simulation-session/🦀️.rs` was not read to completion.
- Whether any plugin uses the `DraftMutation`/`DraftStore` lane for anything beyond the default `NoDraftMutation` —
  only ~30 sample sites were checked, absence of a counter-example is not proof of none.
- The exact runtime location/threshold (if any) distinguishing a "short" connection shortage from a "long" offline
  period — not found in `📡️replication`; may live at the hub/session layer, not inspected.
- `contract-freeze.md §5.2/§5.6/§5.8-§5.10`, cited by doc comments as the spec for the `PendingTransaction`
  protocol (§0) — not located in this pass (only `.rs`/`.ts` were searched).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history` — confirmed to exist on disk (referenced by multiple
  doc comments as the durable ledger keyed by `edit_id`) but its contents were only partially read (via the
  `HistoryEdit` struct at `:102`), not audited end-to-end.
