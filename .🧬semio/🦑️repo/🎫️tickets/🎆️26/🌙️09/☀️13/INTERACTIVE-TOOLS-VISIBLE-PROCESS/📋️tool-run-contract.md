# ⏯️ Tool run contract: interactive, transactional tools with a visible process

Ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`, phase 4b (architecture). This is a design document only: no
source was edited. It binds every phase-4 implementation lane and replaces `📋️master-plan.md` §1.2 ("locking is
committing"), §2.1's ring of 12, §2.4 `WindowMeasure::Progress` and §2.5 `fillBuild` block wherever they conflict.
Inputs were `📓️status.md`, `📋️master-plan.md`, `📓️audit-p4-transaction-primitives.md`,
`📓️audit-p4-tool-lifecycle.md`, `📓️audit-p4-tool-inventory.md` (§0, §8, §9) and `📓️audit-p4-fill-state.md` (read
after the first draft; integrated in §0.9 and wave 1). Every claim marked **verified** was re-read in source for
this document. Paths are
relative to the repo root, and these abbreviations are used throughout:

- `P` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (37 715 lines)
- `S` = `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` (21 926 lines)
- `J` = `🧰️framework/🔨️modules/🧵️job/🦀️.rs`
- `C` = `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`
- `E3` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- `WH` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`

---

## 0. Verified findings the decision rests on

### 0.1 `Emit::amend` puts provisional state into durable, replicated history on every tick

1. `Emit::amend` only sets `coalesce_key` (`P:10547-10552`). `dispatch_emit` turns that into
   `ArtifactCommand::AmendLast` and sends it straight to the live document store (`P:22938-22941`). Nothing along
   that path stages or hides the mutations.
2. `amend_command` (`S:17266-17313`) folds the ops into `self.current` right away. It records them in
   `envelope.vcs.edits` (the persisted edit ledger, reserved through `reserve_edit_history_slot`, `S:15749-15761`)
   and clears redo. It also stamps `finished_at = Some(now_iso())` on every tick (`S:17283`, `S:17300`). The audit's
   "`finished_at: None` means still open" is therefore **false**: no open-edit state exists.
3. `ArtifactStore::dispatch` sets `is_apply` only for `Apply` (`S:16416-16417`). It then calls
   `flush_outbound(is_apply)` (`S:16421-16423`), which for any non-`Apply` command (including `AmendLast`) sends a
   **full `BackboneMessage::Snapshot { pack, spr }`** to peers (`S:18146-18158`). Backbones are attached in
   production: the wgpu Shell uses them (`…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4946,5024,5070`), and so does store sync
   (`…/🏪️store/🔄️sync/🦀️.rs:1009`). **Every amend tick broadcasts the whole document to every peer.** That settles
   the audit's open question, and the answer is worse than suspected: the cost is O(document) per tick, and the
   provisional state becomes shared durable state.
4. Coalescing only works while the amended edit is the last entry of `applied_edit_ids` (`S:17269-17271`). A remote
   ingest merges edits into `applied_edit_ids` in HLC order (`S:17608-17650`). As soon as a peer's edit lands after
   the run's edit, the next tick opens a new edit. A long run therefore splits into several undo entries under
   concurrency, which breaks "one undo entry".
5. Undo of an amended edit goes through `undo_lane_position`, which pushes the edit onto `redo_edit_ids`
   (`S:17190`). The edit stays in `envelope.vcs.edits` and has already been broadcast. An "abort" built on amend
   would need a new `AbortOpenEdit` command, and peers that already received the snapshot would still need a
   compensating broadcast. Zero trace cannot be reached this way.

### 0.2 The DraftStore lane is not wired end to end

1. `Draft`/`DraftMutation` are app types separate from `Snapshot`/`Mutation` (`P:11293-11296`), so a draft cannot
   express "committed document plus provisional document ops" without a second schema.
2. `handle` receives `DraftView` (`P:11346-11355`). `render` does **not** (`P:11462`), and neither do
   `window_measures` or `tool_measures` (`P:11503-11519`). The renderer implementation builds its `ArtifactView`
   only from the committed cache (`P:28224-28236`). **No render path reads the draft.**
3. `Emit::draft` applies through `draft_store.dispatch(ArtifactCommand::Apply…)` (`P:22855-22859`), so each draft
   batch mints an `Edit` in a second full `ArtifactStore`. `ArtifactCommand::PruneDrafts` returns
   `Err("draft pruning is not implemented")` (`S:17137`). No publication path exists from draft to document.
4. Every production app uses `NoDraft`. `rg "type Draft = "` finds only the generic forwarder `P:30231` and one test
   (`…/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs:207`).

### 0.3 The vcs pattern (candidate c) contains no provisional mechanism

`PendingChangeRef` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:1177-1182`) is only the hash input used
to content-address a checkpoint. `ArtifactHistoryLedger` (`…/🌿️vcs/🦀️.rs:287-297`, capacity
`ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` at `:186`) is a fixed slot ledger. A checkpoint merely groups edits that were
already applied and already sent (`uncommitted_edit_ids`, `S:10854-10866`). `S:15400-15420` is the envelope-replace
restore path. The inventory's "never-checkpoint = implicit abort" is **false**: an `Apply` edit reaches peers on
dispatch through `flush_apply_outbound` (`S:18160-18181`), whether or not it is ever checkpointed.

### 0.4 `HostTransactionCoordinator` and `PendingTransaction` (candidate d) are synchronous cross-artifact 2PC

- `run_transaction` (`…/🔌️plugin/🖥️host/🦀️.rs:6392-6470+`) resolves foreign steps, prepares each member once, and
  commits in reverse order.
- The per-instance `transaction_prepare` folds a **private** `running` snapshot that is never exposed
  (`P:27747-27763`).
- `transaction_commit` rejects on generation mismatch and does not rebase (`P:27778-27783`). It applies one `Apply`
  and stamps `group_id` (`P:27787-27796`).
- `transaction_rollback` just clears the slot (`P:27807-27815`).
- While a transaction is pending, the freeze guard rejects every other artifact-emitting verb (`P:22808-22812`).

Useful pieces to reuse: the vocabulary, the "one member = one `Edit` stamped with a group id" rule, and "rollback =
nothing was written". Missing for tool runs: visibility while running, multi-turn lifetime, and rebase.

### 0.5 The puzzle 5d paste job (candidate e) has the right commit shape but no visibility and no user gate

`Puzzle5dPasteJob` (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2014-2262`)
collects a `Vec<Puzzle5dMutation>` across bounded steps and emits once through `completion.complete(Ok(emit))` at
`Complete` (`:2233-2241`). Nothing is rendered while it runs. It also commits as soon as the algorithm ends, so no
explicit finalize step exists.

### 0.6 How plugins keep run state today, and what the renderer can read

- Puzzle 3d keeps its fill session in a process-global `static REGISTRY: OnceLock<Mutex<Puzzle3dSessionRegistry>>`.
  It is checked out and back in around every call (`E3/🦀️.rs:2976-3088`). This is state held outside the four
  framework state classes.
- Render input is `(ArtifactView over the committed cache, ConfigView, TransientView, InteractionView)`
  (`P:28191-28236`). A precedent already exists for a framework-owned body: `FRAMEWORK_HISTORY_BODY_KEY` is served
  before any app body (`P:28193-28200`). `ArtifactView` holds `snapshot: &P` plus a render operation context
  (`P:7646-7655`, `P:7718-7720`). **The renderer can read committed ⊕ provisional as soon as the framework passes a
  folded overlay `Arc<Snapshot>` into `ArtifactView::with_render_context`.** No plugin render code has to change for
  that.
- Ghosts reach React through the World3d scene lane `brushPreviewJson` (`WH:5370-5395`, lanes list at
  `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:519-541`). `FillTriedGhosts` is mounted at `WH:7136`. The
  native wgpu world (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`) reads only
  meshes/instances JSON (`:1881`, `:9514`, `:9620-9627`). It never reads `brush_preview_json`.

### 0.7 wgpu window measures: the dead-code claim is confirmed, and the root cause is deeper

- `render_window_measure_select/_slider/_toggle/_number/_progress` (`…/🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:457-551`)
  have zero callers anywhere in the repo.
- The **native** wgpu backend's `window_measures` returns `Ok(HashMap::new())` on purpose. Its doc says the
  retained-document channel v12 "has no engagement/measure record"
  (`…/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:384-398`).
- The wasm32 JS backend does fetch measures (`:582-589`, `:1038-1041`). The wgpu Shell stores them in
  `self.window_measures` (`…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3921`), but that field is **never read** afterwards;
  the only other hits are the declaration (`:2638`) and the initializer (`:3145`).
- Window bodies on wgpu are the retained `ui_contract` document: `ui_contract::Component`
  (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:495-514`), reconciled to `UiNode` in
  `…/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:543,752` and painted by `…/🖌️paint/🦀️.rs:567,1685`.
- **Result: tool options and progress are invisible on wgpu today.** Neither `Component` nor `UiNode` (`C:3248-3268`)
  has a progress element.

### 0.8 Reusable job and progress primitives (verified)

- `StepContext` provides fuel, deadline, cancel and stage (`J:928-939`).
- `StepOutcome` is `Yield | PreviewReady(RetainedJobPayload) | CheckpointReady | Complete | Cancelled | Fault`
  (`J:1092-1103`).
- `InteractiveJob` (`J:1163-1172`) is driven by `drive_step` under the 8 ms watchdog (`J:1186-1199`).
- `ProgressEvent` already models `StageChanged`, `CandidateTested`, `PreviewPatch` and friends (`J:1688-1759`).
  Channel policies (coalesced preview, lossless commit, diagnostic ring, byte-credit large geometry,
  `LARGE_PREVIEW_PATCH_BYTES = 256 KiB`) are at `J:1808-1850`.
- Bounded atomic publication `begin_apply_batch` / `advance_apply_batch` (`S:16448-16460`, `S:16624+`) keeps
  staged work "never exposed to any read path" until one atomic swap, and `cancel_apply_batch` exists. It requires
  an explicit app-owned `ArtifactStoreOneItemPreparationFactory` (`S:16457-16460`).
- Local-first delivery: the sync actor keeps `DocumentBackbone` bytes "until its authoritative Hub command
  acknowledgment" (`…/🏪️store/🔄️sync/🦀️.rs:163-165`). Presence and preview frames travel the uncredited
  `Lane::Preview` (`:1890-1897`).

### 0.9 Puzzle 3d fill after phase 3 (from `📓️audit-p4-fill-state.md`; runs recorded under `T/🗑️generated/p4-fill-*.txt`)

- The crate compiles with 0 errors and 96 warnings. Fill tests: 95/100 pass. One failure
  (`fill_build_tick_locks_planned_placements_into_the_document_in_bounded_chunks`) is a parallel-run artifact of
  the process-global session registry. Four failures are real and reproduce deterministically:
  1. `fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin` (`E3/🧪️tests/🔬️unit/🦀️.rs:6187`):
     the worst turn takes 5.09 ms against a 2 ms budget. The cause is the per-tick whole-fixture document diff
     (`fillBuildTick` is a document-intent action that emits `create_object`/`connect_vortices` under `AmendLast`
     `"fill-count"` every tick).
  2. `cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run` (`:4240`) and
  3. `engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one` (`:4372`): pieces are already
     locked in the document while `fill_job_identity()` is `None`, so no cancel affordance exists.
  4. `set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count` (`:2005`): a stale
     literal (0 instead of the default 100).
- The tried ring (12 entries of full `BrushPreviewState`, about 250 B of JSON each) and the 16 KiB **full-snapshot**
  preview wire cannot carry all tested candidates, and history that scrolls out of the ring is lost.
- The `success` token exists (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🔣️.json:11`, generated
  `🤖️generated/🔤️tokens/🟦️.ts:10`), but `WH` `MeshStyleKind` (`:493`) has no `success` arm and maps
  non-collision candidates to `highlighted`.
- `engagementAbort` publishes only on `WindowTransient` (`E3/🦀️.rs:7134`), so it cannot roll back locked pieces.

These facts all point the same way as the decision: the performance red, the missing cancel and the durable
per-tick commits come from one code path, the one this contract deletes.

---

## 1. Decision

**Chosen: candidate (b), rebuilt as a framework-owned `ToolRunLedger`.** It is not the DraftStore lane.

- A tool run keeps its provisional **document** mutations (`A::Mutation`, the same type the document uses) in an
  ephemeral local-only ledger owned by `VcsArtifactApp`.
- The framework folds them incrementally into an overlay snapshot, and the renderer reads that overlay as the
  document (committed ⊕ provisional).
- The algorithm's thinking (tested candidates and their verdicts) streams as delta trace pages into a generic
  viewport layer.
- **Abort** discards the ledger. The store is never touched: no `Edit`, no redo entry, no command-log row, no
  backbone frame, no archive write.
- **Finalize** is legal only from `Complete`. It rebases and revalidates against the head, then publishes **one**
  `Edit` through the bounded atomic batch publication:
  - that `Edit` is stamped `group_id = "toolRun:<runId>"`;
  - it produces one command-log row, one undo step, and one `BackboneMessage::Mutations` batch.

### 1.1 Rejected alternatives

| Candidate | Rejected because (verified) |
|---|---|
| (a) `Emit::amend` + new `AbortOpenEdit` | Every tick lands in durable history and replicates a full document snapshot to every peer (§0.1.3). Coalescing breaks under concurrent remote edits (§0.1.4). Undo leaves a redo trace, and broadcast state cannot be retracted without compensating traffic (§0.1.5). It costs O(document) per tick. It violates zero-trace abort, one undo entry, max performance and multi-user correctness all at once. |
| (b) as written: DraftStore/DraftMutation lane | Separate `Draft` type, no render access, no prune, no publication, a second full `ArtifactStore` minting edits per batch (§0.2). It would need all of those built and would still hold a different schema. The ledger reuses `A::Mutation`/`A::Snapshot` directly. |
| (c) vcs checkpoint pattern | No provisional mechanism exists. Checkpoints group edits that were already broadcast (§0.3). |
| (d) `HostTransactionCoordinator`/`PendingTransaction` | Single-turn 2PC across artifacts, with a private invisible fold, no rebase, and a freeze of all other edits (§0.4). Its group-id stamping and "rollback = nothing written" semantics are **reused** at finalize. |
| (e) 5d paste job accumulator | Invisible while running, and it commits at algorithm end without a user gate (§0.5). Its "accumulate typed mutations inside a bounded job, publish once" shape is **reused** as the job side of a run. |
| Plugin-owned registries (puzzle 3d `static REGISTRY`) | This is state held outside the four `StateClass` mechanisms (§0.6). The ledger is a framework `Transient`-class owner with retirement and residency budgets. |

Consequence: `Emit::amend` stays only for cheap direct-manipulation gestures (camera/opacity drags, per the
`UtilityPreviewContract`, `P:11049-11066`). No algorithmic tool may use it. Its per-tick full-snapshot broadcast
(§0.1.3) is a separate performance defect, recorded in §7 (out of scope).

---

## 2. Domain-neutral, schema-first contract

Module: `🧰️framework/🔨️modules/⏯️tool-run/` (new; `⏯️` is unused in the taxonomy). The JSON Schema
`🧬️schema/🔣️.json` is the source of record. The Rust `🦀️.rs` and TS `🟦️.ts` mirror it and are validated against the
same fixtures. All wire names are camelCase.

### 2.1 Identity and staleness guard

```text
ToolRunId        = { appInstanceId: u32, run: u64 }        // run: per-instance monotone, never reused
ToolRunIdentity  = { id: ToolRunId, generation: u32, baseRevision: bytes32 }
```

- `generation` increments on every reconfigure, rebase and conflict-return. Every action except `start` carries
  `{ runId, generation }`.
- A mismatch makes the action a **silent no-op**: fault code `toolRun.stale`, logged at trace level only and never
  shown to the user. This generalizes energy's `request/operation/generation` guard (`audit-p4-tool-lifecycle.md`
  §3.4) and mirrors `interactive-job.stale` (`P:20373,20386`).
- `baseRevision` is the committed `content_revision` the overlay was folded from.
- Renderers drop any trace page, progress snapshot or panel whose `(run, generation, sequence)` is older than the
  newest one they have seen. This is the same lexicographic freshness rule `WH:5381-5395` already uses.

### 2.2 Lifecycle states and legal transitions

`ToolRunState = starting | running | paused | complete | finalizing | finalized | aborting | aborted | faulted`.
There is no `idle` state: "no run" is the absence of a ledger slot.

| From | Event | To | Effect |
|---|---|---|---|
| none, or a terminal state | `start` | `starting` | Allocates a new run id (discarding any terminal slot), captures the base, spawns the run job |
| `starting` | job admitted | `running` | |
| `running` | `pause` | `paused` | The driver stops scheduling steps; the job stays resident |
| `paused` | `resume` | `running` | |
| `paused` | `step` | `paused` (or `complete`) | Exactly one algorithm unit: one `drive_step` with `StepBudget.fuel = 1` |
| `running`, `paused` | job `Complete` | `complete` | Provisional result held; **nothing is committed** |
| `running`, `paused`, `complete` | settings changed | same state (`complete` goes to `running`), `generation+1` | Per the `reconfigure` policy (§3.3) |
| `running`, `paused`, `complete` | base changed (local or remote edit, undo, redo) | same state, `generation+1` | Overlay refold (bounded); the plugin is notified per the `rebase` policy |
| `complete` | `finalize` | `finalizing` | Freshness check, then revalidate job, then batch publication |
| `finalizing` | publication `Complete` | `finalized` | One `Edit`; the ledger's provisional ops are released |
| `finalizing` | revalidation found conflicts | `complete`, `generation+1` | Conflicting ops are retracted and shown as `danger` trace records plus a step entry; the user finalizes again or aborts |
| `finalizing` | store rejection (`VcsError::Rejected`) | `complete`, `generation+1` | A `danger` step carries the merge messages; provisional ops are kept |
| `starting`, `running`, `paused`, `complete` | `abort` | `aborting` then `aborted` | Job `begin_close`/`close_step`; provisional ops and overlay retired; **store untouched** |
| `finalizing` before the batch reaches `Publishing` | `abort` | `aborting` then `aborted` | `cancel_apply_batch` |
| `finalizing` at or after `Publishing` | `abort` | unchanged | Returns `toolRun.stale`; the swap is atomic |
| `starting`, `running`, `paused` | job `Fault` | `faulted` | Provisional ops discarded; trace kept for inspection |
| `finalized`, `aborted`, `faulted` | `dismiss` | none | Trace and panel cleared |
| any | document closed, instance retired or reloaded | none | Same as abort; ephemeral state vanishes |

Invariants, asserted by the lifecycle law fixture:

1. `finalize` is accepted only in `complete`.
2. The store generation changes only on the `finalizing → finalized` transition.
3. After `aborted` or `faulted`, the store generation, the length of `envelope.vcs.edits`, the command log and the
   memory-backbone outbox equal their values at `start`.
4. Pause and resume are justified by the dev requirement to see the algorithm think. `step` lets a user inspect a
   single collision verdict. Neither one creates a durable trace.
5. At most **one** non-terminal run exists per document instance per local actor. A second `start` while one is
   non-terminal fails with `toolRun.busy`.

### 2.3 Progress payload

```text
ToolRunProgress = {
  identity: ToolRunIdentity, sequence: u64, state: ToolRunState,
  stage: u16,                       // index into ToolRunDefinition.stages (localized there, never per tick)
  completed: u64, total?: u64,      // total absent = indeterminate; percentage is derived, never sent
  counters: [{ counter: u16, value: u64 }],   // ≤ 8, ids into ToolRunDefinition.counters (tested/locked/collisions…)
  unitsPerSecond: f32,              // framework-measured from driver timestamps
  conflicts: u32,
  steps: ToolRunStepRing            // newest ≤ 64 entries, overwrite-oldest (DiagnosticRing policy, J:1808-1850)
}
ToolRunStep = { sequence: u64, kind: "info"|"success"|"warning"|"danger", stage: u16, reason: u16,
                subject?: u64 /* trace key */, repeat: u32 /* coalesced identical consecutive entries */,
                args: [f64 | u64] /* ≤ 4, substituted into the reason's localized template */ }
```

- The status text is derived by the renderer from `state` plus `stage`, for example
  "Running · Testing candidates (2/3)". Plugins never send prose.
- Percentage = `completed / total`. The stage position is `stage + 1` of `definition.stages.length`.

### 2.4 Static declaration (manifest, schema-first)

```text
ToolRunDefinition = {
  mutating: bool,                                   // false = read-only run (brush suggestions, simulation preview)
  rebase: "revalidate" | "restart" | "freeze",      // freeze = reject local artifact emits while non-terminal (P:22808-22812 pattern)
  reconfigure: "resume" | "restart",
  unit: LocalizedLabel,                             // "candidate" / "Kandidat"
  stages:   [{ id, label: LocalizedLabel }],
  counters: [{ id, label: LocalizedLabel }],
  reasons:  [{ code: u16, id, verdict: ToolRunVerdict, template: LocalizedLabel }],
  trace: "instance3d" | "placement2d" | "entity" | "none",
  runJob: JobKindId, revalidateJob?: JobKindId      // registered in ArtifactToolFactoryRegistry, classification Migrated
}
```

- It is attached as `run: Option<ToolRunDefinition>` to both `ToolDefinition` and `UtilityDefinition`
  (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:1288-1327,1488-1513`).
- Declaring it auto-injects the generic actions, the same way `history_action_definitions` injects undo and redo
  (`…/🛂️manifest/🦀️.rs:998-1009`).

### 2.5 Generic actions (framework-reserved, routed like `undo`, never queued behind the run)

Abort must be host-driven. A guest-queued cancel waits behind the very work it wants to stop: see
`📓️audit-p4-transaction-primitives.md` §2 generation3d row and
`26/09/09/PROCEDURAL-3D-END-TO-END/📓️preview-eval-cancellation-2026-09-12.md`. The actions are therefore added to
`is_framework_reserved_action_id` (`P:20027`).

| Action id | Args | Legal in | Chord | EN | DE |
|---|---|---|---|---|---|
| `toolRunStart` | `toolId` (required), `windowId?` | none or terminal | `mod+enter` | Start | Starten |
| `toolRunPause` | `runId, generation` | running | `mod+alt+enter` (toggle) | Pause | Pausieren |
| `toolRunResume` | `runId, generation` | paused | `mod+alt+enter` (toggle) | Resume | Fortsetzen |
| `toolRunStep` | `runId, generation` | paused | `mod+alt+arrowright` | Step | Einzelschritt |
| `toolRunAbort` | `runId, generation` | starting, running, paused, complete, finalizing (pre-publish) | `mod+.` | Abort | Abbrechen |
| `toolRunFinalize` | `runId, generation` | complete | `mod+shift+enter` | Finalize | Abschließen |
| `toolRunDismiss` | `runId` | terminal | `escape` (panel focus only) | Dismiss | Schließen |

Chord notes:

- `mod+enter`, `mod+.` and `mod+shift+enter` reuse energy's test-guarded chords
  (`✏️s/🔌️plugins/🔋️energy/…/✏️editor/🦀️.rs:1343-1345`).
- Shifted punctuation is avoided: `event.key` for shift+`.` differs by layout (`>` on US, `:` on DE).
- A framework law test pins these chords wherever the actions are bound. Each plugin's local cancel chord (energy
  and generation3d both use `mod+.`) is deleted from the plugin once that plugin migrates.

Label table, framework-owned `LocalizedLabel::native(en, de)`, no default locale:

| Key | EN | DE |
|---|---|---|
| state `starting` | Starting | Wird gestartet |
| state `running` | Running | Läuft |
| state `paused` | Paused | Pausiert |
| state `complete` | Complete, ready to finalize | Fertig, bereit zum Abschließen |
| state `finalizing` | Finalizing | Wird abgeschlossen |
| state `finalized` | Finalized | Abgeschlossen |
| state `aborting` | Aborting | Wird abgebrochen |
| state `aborted` | Aborted, nothing was changed | Abgebrochen, nichts wurde geändert |
| state `faulted` | Failed, nothing was changed | Fehlgeschlagen, nichts wurde geändert |
| finalize disabled description | Available once the run is complete | Verfügbar, sobald der Lauf fertig ist |
| rebasing step | Document changed, re-applying provisional result | Dokument geändert, vorläufiges Ergebnis wird neu angewendet |
| conflict step | {0} provisional changes conflict with the current document | {0} vorläufige Änderungen stehen im Konflikt mit dem aktuellen Dokument |
| trace truncated step | Oldest {0} rejected attempts are no longer shown | Die ältesten {0} verworfenen Versuche werden nicht mehr angezeigt |
| progress valuetext | {stage} ({i}/{n}): {completed} of {total} {unit} ({pct} %) | {stage} ({i}/{n}): {completed} von {total} {unit} ({pct} %) |

### 2.6 Accessibility

- The panel is a `Container` with role `group`, labelled by the tool label.
- The bar is the new `Component::Progress`, which implies role `progressbar` (§4.1):
  - determinate: `aria-valuemin=0`, `aria-valuemax=total`, `aria-valuenow=completed`, `aria-valuetext` per §2.5;
  - indeterminate: no value attributes, `aria-busy=true`.
- The status line is `Liveness::Polite` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:18-26`).
  It announces at most once every 2 s and immediately on a state transition. `Assertive` is used only for
  `faulted` and for conflicts.
- The step log is role `log` with `Liveness::Off`, so it does not flood the screen reader; it can be navigated by
  keyboard.
- All buttons are real buttons with `aria-keyshortcuts`. Finalize is `aria-disabled` and carries the disabled
  description instead of being hidden.
- The viewport trace is not accessible by itself, so the panel includes a keyboard-navigable **trace list**: newest
  first, verdict chip plus reason text. Focusing an entry frames it in the viewport through the ordinary
  window-config camera action.
- wgpu speaks the same roles through `accessibility_role` (`…/♿️accessibility/🦀️.rs:70-80`).

### 2.7 Transaction binding

1. **Start.** The ledger captures `base = committed Arc<Snapshot>` together with `baseRevision` and `generation`.
   The run job is spawned (`Effect::SpawnJob` or the typed worker path), placement per the plugin.
2. **During the run.** The job reports through `StepOutcome::PreviewReady(RetainedJobPayload)` pages carrying a
   `ToolRunTick` (§3.2).
   - The driver decodes each `appendOps` entry through `OpBinary::decode_op` and folds it into `overlay` in
     O(new ops). The fold is the same `op.diff(&running).diff().apply` loop `dispatch_emit` uses for proposals
     (`P:22820-22829`).
   - `retractTo(n)` truncates the provisional list and refolds from `base`.
   - Render, measures and interaction topology for this document instance read `overlay`.
   - Commands (other tools, the user's own edits) keep reading **committed**.
   - The provisional entity set drives a `provisional` style token (§4).
3. **Abort.** The driver sets the job cancel token, runs `begin_close`/`close_step` to empty, and drops the
   provisional ops and the overlay `Arc`. Zero durable trace follows by construction; the §2.2 invariant 3 test
   proves it.
4. **Finalize.** This is one bounded job phase with three steps:
   - (a) If the head revision differs from `baseRevision`, refold on the head (§2.8).
   - (b) If `revalidateJob` is declared, run it over the provisional ops against the head. It returns a tick that
     retracts conflicting ops and emits `danger` trace records with reason `conflict`. If anything was retracted,
     go back to `complete` with `generation+1`.
   - (c) Otherwise, `begin_apply_batch(operation, expected_generation, expected_revision, actor, provisional,
     description = definition label, HistoryLane::Document, factory)` and advance it in ≤ 8 ms turns
     (`S:16448-16460`, `S:16624+`). It stays invisible until the single atomic swap. Then
     `stamp_tail_group_id("toolRun:<runId>")` (`S:19561`), one `record_command` row, and one outbound mutation
     batch.
   - This yields **one durable `Edit`, one undo entry, one event batch.** A later `undo` reverts the whole run as
     one edit.
5. **Concurrent edits.**
   - `rebase = revalidate` (the default for mutating runs): the overlay refolds on every head change, and
     semantic validity is checked at finalize (step 4b).
   - `restart`: a head change restarts the job, keeping the run id and incrementing `generation`.
   - `freeze`: local artifact emits on this instance fail with `toolRun.busy` while a run is non-terminal. Remote
     edits are still ingested, and they force `revalidate` at finalize.
6. **Short connection shortages.** Runs are entirely local, so no connectivity is needed until finalize. Finalize
   applies locally first. The sync actor keeps the mutation batch until the hub acknowledges it
   (`…/🔄️sync/🦀️.rs:163-165`). The presence summary (§3.4) is last-writer-wins and heals on reconnect. Long
   offline periods are not specially supported: a run is ephemeral and dies with the page.

### 2.8 Rebase cost control

- An overlay refold runs as a bounded driver job (≤ 8 ms per turn). Until it completes, the previous overlay stays
  rendered and a `warning` "rebasing" step is shown.
- Incremental append is O(k). Retract and rebase are O(provisional ops).
- Provisional ops are capped at `TOOL_RUN_PROVISIONAL_OPS_MAX = 65 536` per run. Reaching the cap ends the run as
  `complete` with a `warning` step, never silently.

---

## 3. File placement, renames and removals, extension points

### 3.1 Framework module `🧰️framework/🔨️modules/⏯️tool-run/` (new; domain-neutral, pure, no async, no store)

| File | Content |
|---|---|
| `🦀️.rs` | `ToolRunId`, `ToolRunIdentity`, `ToolRunState`, `ToolRunEvent`, `ToolRunMachine::apply(state, event) -> Result<Transition, ToolRunRejection>` (pure reducer, §2.2 table), `ToolRunProgress`, `ToolRunStep(Kind)`, `ToolRunStepRing`, `ToolRunVerdict`, `ToolRunTracePage` + `ToolRunTraceDelta` codec, `ToolRunTick` codec, `ToolRunDefinition` value types, action id constants, chord constants, framework `LocalizedLabel` table, `TOOL_RUN_*` limits |
| `🟦️.ts` | Mirror of the same types, reducer and codecs (React consumes the trace codec and reducer for optimistic button gating) |
| `🧬️schema/🔣️.json` | `$defs`: all types above plus `LifecycleLawFixture`, `TracePageFixture`, `TickFixture` |
| `🧫️fixtures/⚖️lifecycle-law.json` | Every legal and illegal `(state, event)` pair, invariants, stale-generation cases |
| `🧫️fixtures/📼️trace-pages.json` | Encoded delta pages (hex) with decoded expectations, residency eviction cases |
| `🧫️fixtures/🎞️ticks.json` | Encoded ticks with `appendOps`/`retractTo`/steps/progress |
| `🧪️tests/🔬️unit/🦀️.rs` | Rust reducer and codecs against the fixtures |
| `🧪️tests/🧩️conformance/🟦️.ts` | TS reducer and codecs against the same fixtures; **oracles**: `xstate` machine built from the fixture table, `ajv` fixture validation, `fast-check` random event sequences (TS reducer vs xstate) |
| `📦️packages/🦀️rust/Cargo.toml` | Crate `semio-framework-tool-run` |
| `📜️script.ts`, `project.json`, `package.json` | Per repo rules (`project.json` calls only `📜️script.ts`) |

### 3.2 Trace and tick wire (delta pages, never full JSON)

```text
ToolRunTracePage  = { identity, page: u32, ops: [ Upsert{ key: u64, verdict: u8, reason: u16, subject } | Retire{ key } | Clear ] }
subject           = Instance3d{ mesh: u32, position: [f32;3], rotation: [f32;4], scale: f32 }
                  | Placement2d{ shape: u32, position: [f32;2], rotation: f32 }
                  | Entity{ entity: u64 }
ToolRunVerdict    = testing | success | warning | danger
ToolRunTick       = { identity, sequence, progress?: ToolRunProgress, steps: [ToolRunStep],
                      trace: [ToolRunTracePage], appendOps: [bytes /* OpBinary */], appendEntities: [u64], retractTo?: u32 }
```

- Encoding: `🎒️pack` record bodies (`encode_record_body`/`decode_record_body`; not `encode_json_value`), columnar
  per page.
- Page caps: 4 096 ops and 256 KiB per page (`LARGE_PREVIEW_PATCH_BYTES`). Ticks travel as
  `RetainedJobPayload` pages.
- Residency: `TOOL_RUN_TRACE_RESIDENT_RECORDS = 1 048 576` records, about 40 B each.
  - Overflow evicts the oldest `danger`/`warning` records first and never `success`/`testing`.
  - Each eviction emits one coalesced `warning` step with the evicted count.
  - "Show all tested meshes" therefore holds up to about 1 M attempts per run.
- Delivery to a window: the renderer echoes `toolRunTraceCursor { run, generation, page }` in its window instance
  view state.
  - The ledger answers with pages after the cursor, at most a per-refresh byte budget.
  - A run or generation mismatch triggers a `Clear` followed by a resend from page 0.
  - This survives reloads and multiple windows, and costs nothing when idle.

### 3.3 OS product runtime `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs` (new) and `P`

- `ToolRunLedger<A>`: at most one slot per instance.
  - Holds identity, state, `base: Arc<A::Snapshot>`, `provisional: Vec<A::Mutation>`, `entities`,
    `overlay: Arc<A::Snapshot>`, `trace: ToolRunTraceStore` (resident, page log), progress, steps, the job handle
    and the cancel lease.
  - Implements the retirement and close protocol like the other owners.
- `ToolRunDriver`:
  - admits and steps the run and revalidate jobs through the reactor interactive lane, using `drive_step`
    (`J:1186-1199`) and the typed worker admission (`P:25590-25640`);
  - honours pause and single-step fuel;
  - watches store generation, config and window-config generations to emit `base changed` and
    `settings changed`.
- Edits in `P`:
  - a `tool_runs` field on `VcsArtifactApp`;
  - framework-reserved routing of §2.5 actions in `is_framework_reserved_action_id` (`P:20027`);
  - overlay substitution in `render`, `window_measures`, `tool_measures` and `window_engagements`
    (`P:28191-28310`);
  - `FRAMEWORK_TOOL_RUN_BODY_KEY = "framework.body.toolRun"`, served before app bodies next to
    `FRAMEWORK_HISTORY_BODY_KEY` (`P:28193-28200`);
  - scene lane injection (§4);
  - the finalize publication (§2.7.4).
- Reconfigure policy `resume`: the driver restarts the job from its last `StepOutcome::CheckpointReady` checkpoint
  with the new settings bytes. Puzzle fill relies on this, because its sequence has the prefix property. A lowered
  target makes the resumed job emit `retractTo`.

### 3.4 Presence summary (ephemeral shared)

- New field `PresencePeer.tool_run: Option<PresenceToolRun { toolId, state, stage, completed, total }>` in
  `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:1351-1376`, mirrored in `📡️replication/🟦️.ts`. Peers see
  "Alice · Fill · 42 %".
- Provisional geometry is **not** shared with peers: it would cost bandwidth and it is not durable-relevant.

### 3.5 Manifest and UI contract

- `🛂️manifest/🦀️.rs` and `🟦️.ts`: `ToolRunDefinition` on `ToolDefinition` and `UtilityDefinition`,
  `tool_run_action_definitions()`, the chord law.
- `🛂️manifest/🧬️schema/🔣️.json`: the definition `$defs` (by `$ref` to the `⏯️tool-run` schema).
- `🧬️schema/📽️projection/🦀️.rs` and the regenerated `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` via
  `bun nx run @semio-tech/framework-rs:generate`. That target name is taken from `📋️master-plan.md` §2.4; confirm
  it with `bun nx show project @semio-tech/framework-rs`, because `.vscode/launch.json` registers only `build`,
  `preview-generated` and `test-wire-retirement-*`.
- `🖱️ui/🧬️contract/🧩️component/🦀️.rs`: new `Component::Progress(ProgressProps { completed: f64, total: Option<f64>,
  value_text: Label })`. Its mirror lives in `🧬️contract/🧬️schema/🦀️.rs`; limits in `🛡️limits/🦀️.rs`; role
  `progressbar` in `♿️accessibility/🦀️.rs`.
- wgpu: `UiNode::Progress` in `C` (`C:3248`), reconcile in `🔀️reconcile/🦀️.rs`, paint in `🖌️paint/🦀️.rs`.
- React: a `progress` case in `…/🧱️elements/🗣️Interpreter/🟦️.tsx`.
- The framework ToolRun panel is built once in `🔌️plugin/⏯️tool-run/🦀️.rs` as a `ComponentTree`. It therefore
  paints on React and wgpu without per-target panel code.

### 3.6 Renames and removals (no compatibility layer)

| Removed | Where | Replacement |
|---|---|---|
| `WindowMeasure::Progress`, `MeasureProgressStep`, `MeasureProgressStepKind`, `measure_progress_cancel_label` | `C:1040-1060,1134-1165,1208-1224`; projection `📽️projection/🦀️.rs:870-884,1817-1885`; generated manifest `:1451,1475`; ui wire and round-trip tests | `ToolRunProgress` plus the framework ToolRun panel body |
| `WindowMeasureProgress` component and its `progress` cases | `…/🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx:161-209`; `…/🛠️ShellHelpers/🟦️.tsx:3346-3353,3387-3400` | Panel body through the Interpreter |
| Dead `render_window_measure_*` block | `…/🪀️widgets/🦀️.rs:456-551` | Measures projected to `ui_contract` components (W0-G) |
| `Shell.window_measures` field that is never read | `…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2638,3145,3921` | Measures painted through W0-G |
| Plugin progress and cancel measures | `E3/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` (`progress_measure`, `cancel_measure`), `◻️2d/…/🛠️tools/🪣️fill/🦀️.rs:94-118`, `🖐️5d/…/☑️options/🪣️fill/🦀️.rs:69-85`, `🧊️3d/…/🪛️utilities/🖌️brush/🦀️.rs:67-121` | `ToolRunDefinition` plus ticks |
| Per-plugin run verbs | puzzle `fillBuildTick`, `cancelFillBuild`, 2d `brushFillSession{Begin,Step,Cancel,Discard,Adopt}` and `fill-session-*` commands, 5d `cancel-fill-build`, energy `start/cancel/retry/discard/adopt-energy-simulation`, generation3d `cancel-preview-eval` | §2.5 generic actions |
| Per-plugin lifecycle enums | `Puzzle2dFillLifecycle` (`◻️2d/…/🎚️config/🦀️.rs:42-60`), `EnergySimulationStatus` | `ToolRunState` |
| `FILL_TRIED_RING`, `FillTriedCandidate`, `tried` wire, `fillBuildPreview` tail | `E3/…/🧬️schema/🦀️.rs:668-785`, `E3/⏳️precompute/🪣️fill/🦀️.rs:460,554`, `WH:349-376,406,1703-1721,3776,5381-5395,7136` | Trace pages plus `ToolRunTraceLayer` |
| "Locking is committing": `take_locked_into_fixture`, `FILL_LOCK_PLACEMENTS_PER_TICK`, the `fill-count` amend key | `E3/🎮️commands/🧮️set-fill-count/🦀️.rs:61-77`, `E3/⏳️precompute/🦀️.rs` | `appendOps` provisional plus finalize |
| Fill session held in the process-global registry | `E3/🦀️.rs:3052-3088` (`Puzzle3dSessionState.fill`) | The ledger owns the run job; the registry keeps geometry caches only |
| DraftStore lane (wave 4) | `P:11293-11296,10566-10569,22855-22859`, `DraftView`, `draft_store`, `🔌️plugin/📝️draft/**`, `ArtifactCommand::PruneDrafts` (`S:2780,17137`), every app's `type Draft = NoDraft` | Nothing (unused, §0.2) |

### 3.7 Plugin extension points

1. Declare `ToolRunDefinition`: stages, counters, reasons with verdict and EN/DE template, trace subject kind,
   policies.
2. Register `runJob` (an `InteractiveJob` that emits `ToolRunTick` pages via `StepOutcome::PreviewReady`,
   checkpoints through `CheckpointReady`, and ends with `Complete`) and optionally `revalidateJob`, both
   `Migrated`.
   - Use the domain-neutral writer `ToolRunTickWriter` from `⏯️tool-run` (bytes only: plugins pre-encode ops with
     `OpBinary`).
   - Honour `consume_fuel(1)` per algorithm unit so `step` means exactly one unit.
3. Mutating runs supply their `ArtifactStoreOneItemPreparationFactory` for the finalize publication
   (`S:16457-16460`).
4. Visualization: trace subjects reference meshes or shapes by index into the plugin's existing mesh lane. Verdict
   colours are framework tokens. A plugin never paints ghosts itself.
5. Settings stay ordinary config or window-config measures (`WindowMeasure::Number` etc.). The driver detects the
   change; plugins do not dispatch reconfigure.

---

## 4. Generic "visible process" rendering contract

### 4.1 Layers

1. **Provisional document.** `ArtifactView.snapshot` is the overlay, so provisional objects render as ordinary
   instances.
   - `ArtifactView::tool_run()` exposes `{ identity, state, provisionalEntities }`.
   - The framework stamps the `provisional` style token onto instances whose entity id is in that set. React and
     wgpu both have a mesh style palette (danger/highlighted already exist in `WH:493-566`); `provisional` is a new
     theme token: success hue at reduced opacity with a dashed or animated outline, where `prefers-reduced-motion`
     keeps it static.
   - Provisional entities are **not** selectable or hoverable: interaction topology excludes them.
2. **Trace layer.** A new World3d scene lane `toolRunTrace` (key `framework.scene.world3d.toolRunTrace`, added to
   `WORLD3D_SCENE_LANE_NAMES/FIELDS/BODY_KEYS/OPTIONAL` in `…/🎬️scene/🎬️scenes/🦀️.rs:519-590`) carries a
   base64url `ToolRunTraceDelta` string, so it pages through the existing lane-spine mechanism (`:275-283`).
   - A matching `toolRunTrace` lane is added to the canvas2d scene (`…/🎬️scene/🖼️canvas2d-snapshot/`).
   - Renderers keep a keyed record store per window: GPU-instanced batches per `(mesh, verdict)` in 3d, and
     batched quads/paths per `(shape, verdict)` in 2d.
   - Verdict tokens: `testing` = neutral/progress at 60 % opacity; `success` = theme success; `warning` = theme
     warning; `danger` = theme error. Semantic tokens only, never literals.
   - The most recent `testing` record gets the highlighted outline, so "the mesh being tested right now" is always
     visible.
   - Records fade down to a floor opacity with age (sequence distance), so recent reasoning stands out while the
     full history stays visible.
   - Legend and toggles (show rejected / show accepted / show testing) are window-config options, persisted local
     only.
3. **Panel.** `FRAMEWORK_TOOL_RUN_BODY_KEY` provides progress, status, step log, trace list and buttons (§2.5, §2.6).

### 4.2 Targets

| Target | Provisional document | Trace layer | Panel |
|---|---|---|---|
| React | Existing instance path (overlay is just the snapshot) plus a `provisional` style in `WH` mesh styles | New `…/🧱️elements/🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx` (`InstancedMesh` per mesh×verdict, codec from `⏯️tool-run/🟦️.ts`); 2d twin `…/📐️Canvas2dHost/⏯️tool-run-trace/🟦️.tsx` | Interpreter `progress` plus existing components |
| wgpu | `♾️infinite/🌍️world/🦀️.rs` instances plus a `provisional` draw flag | `♾️infinite/🌍️world/⏯️tool-run-trace/🦀️.rs` (instanced draws per mesh×verdict) plus render-plan validator limit in `…/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:99-111` | `UiNode::Progress` paint |

Per-frame cost is O(delta records), with no JSON parse of history. An idle run produces no pages.

---

## 5. Implementation waves (disjoint file ownership)

Prompt rules for every agent:

- Opus, **foreground builds only**, no background processes, no modifying git commands.
- Never touch `T/🗑️generated` except its own new subfolder. Never sweep it.
- Do **not** close or reopen the ticket.
- Write the report to `T/📓️wave-<lane>.md`.
- TDD: the fixture and the failing test come first.
- `cd /Users/ueli/Documents/semio` in every Bash call.
- Grep via Bash `rg`.

### Wave 0: framework (seven parallel lanes)

| Lane | Owned files | Deliverable | Tests (TDD) | Verify |
|---|---|---|---|---|
| **W0-A `⏯️tool-run` module** | All of `🧰️framework/🔨️modules/⏯️tool-run/**` (new); root `Cargo.toml` workspace member; `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (member name `⏯️tool-run`); `.vscode/launch.json` entries for every wave-0 command (existing order and grouping) | §2, §3.1, §3.2 types, reducer, codecs, labels, limits | Rust `🔬️unit` + TS `🧩️conformance` over the three fixtures; xstate/ajv/fast-check oracles | `cargo test -p semio-framework-tool-run`; `cargo check -p semio-framework-tool-run --target wasm32-wasip2`; `bun ./📜️script.ts test` in the module |
| **W0-B manifest + projection** | `🧰️framework/🔨️modules/🛂️manifest/{🦀️.rs,🟦️.ts,🧬️schema/🔣️.json,🧪️tests/**}`; `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs`; `🛂️manifest/🤖️generated/**` (regenerate only); `…/🛠️ShellHelpers/🟦️.tsx` + `…/🎚️measure-controls/🟦️.tsx` (Progress removal) | §2.4 definition, §2.5 action injection + chord law, projection of the new types, removal of the `progress` measure TS arms and React component | Manifest unit: injection only when `run` is declared, chord law; projection snapshot | `cargo check -p semio-framework-manifest` (confirm the crate name in `🛂️manifest/📦️packages`); framework-rs generate then check; `bun nx run workspace:verify -- interactivity` |
| **W0-C UI contract Progress** | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/{🧩️component,🧬️schema,🛡️limits,♿️accessibility}/🦀️.rs` + contract tests; `C` (`UiNode::Progress`, removal of `WindowMeasure::Progress`/`MeasureProgressStep*`); `…/🧊️wgpu/{🔀️reconcile,🖌️paint}/🦀️.rs`; `…/🧊️wgpu/🦀️.rs:242` re-export line; `🖱️ui/🧪️tests/🔬️targets-wgpu-component-layout-*` and `🔬️targets-wgpu-accessibility-projection`; `…/🗣️Interpreter/🟦️.tsx` | `Component::Progress` end to end on both targets | Accessibility projection: role `progressbar`, value attributes; React Interpreter test (valuenow/busy); contract fixture + serde oracle | `cargo test -p semio-framework-ui`; `cargo check -p semio-framework-ui --target wasm32-wasip2`; Interpreter vitest via its `📜️script.ts` |
| **W0-D runtime ledger + driver** | `🔌️plugin/⏯️tool-run/**` (new); `P` (fields, routing, overlay, body key, lane injection, finalize); `🔌️plugin/🧪️tests/🔬️tool-run/**` (new); `S` only if a missing hook appears (none expected) | §2.7, §2.8, §3.3, panel `ComponentTree` | Artifact-app fixture: start, ticks, overlay visible to render, abort invariant (store generation, `vcs.edits` length, command log, memory-backbone outbox unchanged; memory backbone `S:18775`); finalize gives one `Edit` + group id + one `Mutations` message; concurrent remote ingest then rebase then conflict return; stale generation no-op; pause/step fuel = 1; reconfigure resume from checkpoint | `cargo test -p semio-framework-plugin --features artifact-app-testing -- tool_run`; `cargo check -p semio-framework-plugin --features component-guest --target wasm32-wasip2`; `cargo check -p semio-framework-os-kernel` |
| **W0-E trace lane + renderers** | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/{🎬️scenes/🦀️.rs,🖼️canvas2d-snapshot/**,🟦️.ts,🧪️tests/**}`; `…/🌐️World3dHost/⏯️tool-run-trace/**` (new); `…/📐️Canvas2dHost/⏯️tool-run-trace/**` (new); `♾️infinite/🌍️world/⏯️tool-run-trace/**` (new) + the mount in `♾️infinite/🌍️world/🦀️.rs`; `…/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` (validator); theme token `provisional` in `🖱️ui/🎨️styling/**` | §4 layers 1–2 on both targets | Lane spine tests; TS store vs the Rust codec over `📼️trace-pages.json`; three.js InstancedMesh count oracle; wgpu draw-count test | `cargo test -p semio-framework-ui-scene`; world crate tests; `cargo check --target wasm32-wasip2` for the scene crate |
| **W0-F presence summary** | `🧰️framework/🔨️modules/📡️replication/{📡️wire/🦀️.rs,🟦️.ts,🧪️tests/**}`; `…/🏪️store/🔄️sync/🦀️.rs` presence assembly; hub mirrors found by `rg "drag_ghost_json\|dragGhostJson" 🌎️hub` | §3.4 | Wire round trip Rust↔TS over a fixture | replication crate tests + TS tests |
| **W0-G wgpu measures home** | `…/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`; `…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (measures region only); `…/🧊️wgpu/🪀️widgets/🦀️.rs` (dead block removal); the retained contract record for measures in `🖱️ui/🧬️contract/📃️document/**` | Measures (Number/Slider/Select/Toggle/Group) get a v12 wire home and paint as `ui_contract` components in the Measures overlay; the never-read field and dead widgets are deleted | Native bridge returns measures; Shell paint test; dock partition tests stay green | `cargo test` for the Shell/Dock wgpu units; `cargo check --target wasm32-unknown-unknown` for the web wgpu shell |

Integration gate at the end of wave 0 (single coordinator, foreground): framework-rs generate + check, then every
command above, then §6.

### Wave 1: puzzle 3d fill end to end (after wave 0)

| Lane | Owned files | Deliverable | Tests | Verify |
|---|---|---|---|---|
| **W1-A fill run job** | `E3/⏳️precompute/🪣️fill/**`, `E3/⏳️precompute/🦀️.rs`, `E3/⏳️precompute/🧪️tests/**`, `E3/⏳️precompute/📐️geometry/**`, fill types in `✏️s/…/🧊️3d/…/✳️any/🧬️schema/{🦀️.rs,🔣️.json}` | `FillBuilder` as a `ToolRun` `runJob`: every candidate becomes `Upsert testing`, then `danger/collision`, `warning/<rule>` or `success/fits`; accepted placements become `appendOps` (`create_object` + `connect_vortices`) + entities; stall becomes a `warning` step and `Complete`; checkpoint + resume with a new requested count (a raise continues, a lower retracts); `revalidateJob` re-tests collisions of provisional placements against the head | Fixture `🪣️fill/🧫️fixtures/🎞️fill-run.json` (seeded document + seed → exact verdict sequence and op count); **oracle: `parry3d`** collision verdicts per candidate; the existing serde oracle adapted to ticks | `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4`; `bun ./📜️script.ts test -- fill` in `…/🧊️3d/📦️packages/🦀️rust`; `--target wasm32-wasip2` check |
| **W1-B editor wiring** | `E3/🦀️.rs`; `E3/🎮️commands/{🪣️fill-build-tick (delete),🧮️set-fill-count,🛑️engagement-abort,📨️engagement-submit,🔂️engagement-repeat-last}/**`; `E3/🎭️modes/✏️edit/🛠️tools/🪣️fill/**`; `E3/🗣️terminology/🦀️.rs`; `E3/🎚️config/**`; `E3/🎭️modes/✏️edit/🪟️windows/🧊️main/{🦀️.rs,🟦️.ts}`; `E3/🧪️tests/**`; `E3/🪟️window/🦀️.rs` | `ToolRunDefinition` for fill (stages, counters tested/locked/collisions/rejected, reasons EN/DE); job factory registration; count stays `WindowMeasure::Number` (config); the removals from §3.6 (tick command, amend key, progress/cancel measures, `fillBuild` interaction block, fill in the session registry); `engagementAbort` (Escape) maps to `toolRunAbort` only while a fill run is non-terminal; preparation factory for finalize | Editor tests: start→complete→finalize gives one undo entry; abort leaves the document byte-identical; undo after finalize removes all pieces | same cargo commands; `bun nx run workspace:verify -- interactivity` |
| **W1-C React host cleanup** | `WH` only | Delete `FillTriedGhosts`, `WorldFillTriedRecord`, `fillBuildPreview` parsing and the identity ref; mount `ToolRunTraceLayer`; add the provisional mesh style | Existing World3dHost tests updated | host vitest via its `📜️script.ts` |
| **W1-D policy predicates** | root `📜️script.ts` (`…Failures()` predicates for fill), `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-*/**` | Predicates: no `Emit::amend` in tool-run tools; no plugin-local cancel/progress measures; no `FILL_TRIED_RING`; `ToolRunDefinition` present for every `algorithmic-mutating` tool | Mutation self-tests | `bun nx run workspace:verify -- interactivity` |
| **W1-F red→green owner** (runs inside W1-B's files; W1-B's report must tick each row) | the §0.9 tests in `E3/🧪️tests/🔬️unit/🦀️.rs` (W1-B owned) | See the table below | as listed | `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- fill --test-threads=4`, then the same with `--test-threads=1`; both must report 0 failures |
| **W1-E verification** (after A–D) | `T/⏯️tool-run-probe.ts` (new; it may import `26/09/02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts` helpers), `T/🗑️generated/w1e/**` | Deploy and runtime evidence (§6.5, §6.6) | n/a | §6 |

#### Wave 1 red→green obligations (`📓️audit-p4-fill-state.md` §2)

| Red today | Root cause | Green under this contract (asserted by a rewritten test, never a loosened budget) |
|---|---|---|
| `fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin`: 5.09 ms against 2 ms | The whole-fixture diff plus `create_object`/`connect_vortices` amend on every 120 ms tick | `fillBuildTick` is deleted. Replacement `fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin` measures, over the same 771+ turns: (a) worst `drive_step` of the fill run job < 2 ms (W1-A); (b) worst ledger overlay append of one tick's `appendOps` < 2 ms (O(k) fold, W0-D). **Same 2 ms budget, same Nakagin document.** |
| `cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run`: identity `None` | Cancel was gated on `fill_job_identity()`, which races with commit | The ledger identity exists from `starting` onwards, so the panel's `toolRunAbort` is present in every non-terminal state by construction. The rewritten test waits for provisional pieces > 0, asserts abort is enabled with the current `{runId, generation}`, dispatches it, and asserts `aborted`, zero provisional ops, and a document byte-identical to the one at start. |
| `engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one`: identity `None` | Same; also `engagementAbort` is `WindowTransient`-only (`E3/🦀️.rs:7134`) | `Escape` (`engagementAbort`) maps to the framework-reserved `toolRunAbort` while a fill run is non-terminal. Teardown is `aborting` across bounded `close_step` turns; the test keeps its "never inside one turn" assertion against the driver's close steps. No `Artifact` lane is needed, because nothing was ever written. |
| `set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count`: expects 0, gets 100 | Stale literal from before the default-100 decision | Assert the default is 100. The test now also asserts that `setFillCount` during a run triggers `reconfigure: resume` (same run id, `generation+1`) and never emits artifact mutations. |
| `fill_build_tick_locks_planned_placements_into_the_document_in_bounded_chunks`: fails only in parallel | Process-global `Puzzle3dSessionRegistry` shared across test threads | Deleted together with lock-is-commit. Its successor `fill_run_finalize_publishes_one_edit_with_every_provisional_placement` uses the instance-owned ledger (no process-global run state), so it must pass under `--test-threads=4` **and** `=1`. |

Also required in wave 1:

- The preview wire caps (`FILL_PREVIEW_JSON_MAX_BYTES`, `WORLD_FILL_PREVIEW_JSON_MAX_BYTES`, 16 KiB full snapshot)
  are deleted together with the `fillBuildPreview` tail. Tested candidates travel only as §3.2 delta trace pages.
  W1-A adds a fixture law that a 5 000-candidate run delivers every record, checked by comparing resident key sets.
- Verdict mapping is `collision → danger`, `fits/accepted → success` (`tokenVar("success")`), a rule rejection →
  `warning`, and the live candidate → `testing` with a highlighted outline. The theme tokens come from W0-E's trace
  layer; `WH` keeps no fill-specific `MeshStyleKind` arms.

### Wave 2: puzzle 2d / 5d / brush (parallel)

| Lane | Owned files | Deliverable |
|---|---|---|
| **W2-A puzzle 2d** | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/**` | Fill session as a run: delete the `fill-session-{begin,step,clear}` commands and `Puzzle2dFillLifecycle`; `placement2d` trace; `geo` crate collision oracle |
| **W2-B puzzle 5d** | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/**` (fill option, `🧠️precompute`, `set-fill-count`, delete `cancel-fill-build`; the 2d-window fill utility) | Reuses the 3d `runJob` through its existing planner; both 3d and 2d trace subjects; paste job moves onto the tick contract only if it exceeds one frame (it already accumulates then emits once) |
| **W2-C puzzle 3d brush suggestions** | `E3/🎮️commands/{⏱️suggestions-tick,🔓️open-vortex-suggestions,🔒️close-vortex-suggestions}/**`, `E3/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/**`, brush parts of `E3/⏳️precompute/🖌️brush/**` | `mutating: false` run; trace verdicts; accept stays a separate one-shot command |

Coordination: W2-C shares `E3/⏳️precompute/🦀️.rs` with nothing in wave 2, because W1-A already finished it. Brush
edits inside that file go through W2-C only.

### Wave 3+: other plugins (one lane per plugin directory, parallel; inventory §8 order)

1. Energy simulation: map `adopt` to finalize and drop the five verbs (`✏️s/🔌️plugins/🔋️energy/**`).
2. Procedural generation3d preview eval: read-only run, delete `cancel-preview-eval`
   (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/**`).
3. Assembly WFC: wire `WfcJob` preview into ticks, after its process-abort defect is fixed
   (`…/🧩️assembly/**`).
4. Remodel reconstruction (`✏️s/🔌️plugins/📸️remodel/**`).
5. Layout export (`✏️s/🔌️plugins/📏️layout/**`).
6. Trinity/jack run-query streaming (`✏️s/🔌️plugins/🔱️trinity/**`).
7. FEM 2d solve (`✏️s/🔌️plugins/🏗️fem/**`).
8. Raster composite (`✏️s/🔌️plugins/🖨️raster/**`).
9. Process3d brep replay (`✏️s/🔌️plugins/🏭️process/**`).
10. Imperative and sequence run (`📜️imperative/**`, `🎬️sequence/**`).
11. Animate video export (`🎞️animate/**`).
12. Architect analysis (`🏛️architect/**`).
13. Space exports (`🪐️space/**`).
14. Note ink (`🗒️note/**`).
15. Lowpoly decimate (`💠️lowpoly/**` + `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/**`).

Framework lane **W3-F**: a shared stepped force-layout `runJob` in a new
`🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/**`, re-pointed by the reasoning, dag, trinity/jack, flow and sequence
lanes, which run after W3-F.

### Wave 4: cleanup (single owner, after wave 3)

- Delete the DraftStore lane (§3.6 last row) across `P`, `S`, `🔌️plugin/📝️draft/**` and every app declaration.
- Remove `Emit::amend` usage from any remaining algorithmic tool.
- Add the root policy predicate that forbids reintroducing either.

---

## 6. Verification gates (all run in the foreground; outputs to `T/🗑️generated/<lane>/`)

1. **Native type check with warnings as proof.**
   - `cargo check -p semio-framework-tool-run -p semio-framework-ui -p semio-framework-ui-scene -p semio-framework-job -p semio-framework-plugin -p semio-framework-os-kernel`
   - `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4` (current 0 errors / 96
     warnings, `📓️audit-p4-fill-state.md` §1).
2. **wasm targets.** Cfg-gated code never compiles natively.
   - `cargo check -p semio-framework-tool-run --target wasm32-wasip2`
   - `cargo check -p semio-framework-plugin --features component-guest --target wasm32-wasip2`
   - `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --target wasm32-wasip2`
   - the web wgpu shell's wasm32 check.
3. **Tests.**
   - `⏯️tool-run` Rust + TS conformance (xstate/ajv/fast-check).
   - Plugin runtime `tool_run` suite: abort zero-trace invariant, one-edit finalize, rebase/conflict, stale no-op.
   - `bun ./📜️script.ts test -- fill` (parry3d oracle). The nextest watchdog cancels on the first failure, so the
     full count comes from `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- fill`
     at `--test-threads=4` and `=1`. Pass criterion: 0 failures in both runs, including every §5 wave-1
     red→green row.
   - ui contract accessibility projection.
   - scene lane + trace codec parity.
4. **Schema pipeline.** framework-rs generate then check (manifest mirror in sync); ajv validates every new fixture
   against `⏯️tool-run/🧬️schema/🔣️.json`.
5. **Policy.** `bun nx run workspace:verify -- interactivity` gives 0 blocking findings, including the W1-D
   predicates.
6. **Deploy.** `component-dev` → `support-dev` → `materialize-dev` → `prepare` → `activate-puzzle3d-react-dev`.
   The served `core.wasm` sha256 must equal the disk copy (`📋️master-plan.md` §4.5).
7. **Browser runtime evidence** (React :6013, then the wgpu shell), captured through in-page hooks. The console
   buffer survives reload, so console text alone is not proof.
   - (a) Start fill on Concrete Forest. Within 30 s the trace layer instance count grows monotonically, and at
     least one `danger` and one `success` record are observed via `data-tool-run-*` attributes. The document store
     generation is **unchanged** while the run is going.
   - (b) Pause, step: the trace grows by exactly one record.
   - (c) Abort: store generation, command-log length and outbound frame count are unchanged; provisional instances
     are gone; the trace stays until dismissed.
   - (d) Restart, run to `complete`, finalize: store generation +1, exactly one history row labelled with the tool,
     one `Mutations` frame; `undo` removes every piece.
   - (e) Change the count 100 → 250 during the run: it continues without restart (`generation+1`, same run id).
     100 → 40: provisional tail retracted.
   - (f) A second tab edits the document during the run: a rebase step appears, and finalize either commits or
     returns to `complete` with a conflict count.
   - (g) Keyboard only: `mod+enter`, `mod+alt+enter`, `mod+alt+arrowright`, `mod+.`, `mod+shift+enter` reach every
     transition; the progressbar exposes `aria-valuenow`/`aria-valuetext` in EN and DE.
   - (h) wgpu shell: panel, progress and trace instances paint (screenshot + draw-count hook); the count measure is
     visible (W0-G).

---

## 7. Out-of-scope findings (flagged, not planned here)

1. `Emit::amend` gestures broadcast a **full document snapshot per tick** to peers (`S:16416-16423`,
   `S:18146-18158`). Gesture coalescing should send per-op envelopes like `Apply`, or send nothing until the
   gesture ends.
2. `amend_command` appends new inverse ops **after** the older ones (`edit.inverse.extend(new_inverse)`,
   `S:17280-17282`). If any consumer replays `Edit.inverse` sequentially (undo itself uses cached or refolded
   snapshots, `S:17182-17189`), multi-tick edits whose ops do not commute would invert in the wrong order. Not
   traced further.
3. `…/🔋️energy`, `…/🌀️procedural/🧊️generation3d` and puzzle each bind `mod+.` locally. The wave-3 migrations remove
   those bindings in favour of the framework chord law.
