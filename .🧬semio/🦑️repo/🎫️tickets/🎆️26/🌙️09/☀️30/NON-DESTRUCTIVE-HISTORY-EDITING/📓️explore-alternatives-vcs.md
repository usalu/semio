# Explore: alternatives, VCS, replication and hub for non-destructive history editing

Ticket: `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Read-only audit, 2026-09-30. Nothing was compiled or run; every statement is from reading the source at the cited lines (concurrent churn may shift line numbers by a few).

## Path aliases

| Alias | Path |
|---|---|
| `REPL` | `🧰️framework/🔨️modules/📡️replication` |
| `OS` | `🧰️framework/🛍️products/💻️os/🔨️modules` |
| `STORE` | `OS/🏪️store/🦀️.rs` (23.6k lines) |
| `SYNC` | `OS/🏪️store/🔄️sync/🦀️.rs` |
| `PLUGIN` | `OS/🔌️plugin/🦀️.rs` (29k+ lines) |
| `DBART` | `OS/🛢️db/🗿️artifact/🦀️.rs` (hub-side artifact engine) |
| `TRANS` | `REPL/🔗️causal/🔀️transition/🦀️.rs` |
| `SPRH` | `OS/📡️spr/📜️history/🦀️.rs` |
| `HUBB` | `🌎️hub/🏗️bootstrap/🦀️.rs` |
| `TICKET19` | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/EVENT-SOURCED-FRAMEWORK-VERSION-CONTROL-END-TO-END` |

## 0. Verdict in ten lines

1. There is no artifact-level fork, "save as" or duplicate anywhere. `os.create-space-artifact` takes only `kindChoice` and `name`; the hub refuses client-supplied pairs.
2. "Alternative" means four different things. Only the first is version control: the `Branch`/`Checkout` transitions plus the `Alternative` ledger; the others are the space-level `SpaceAlternative`, a UI-only tree `< 1/N >` control, and the flow module's own separate VCS.
3. The document head (current checkpoint and active alternative) is a single shared value. `Branch` and `Checkout` are ordinary HLC-ordered events folded on every replica, so one user's checkout moves everybody (`TRANS:430-438`, test `checkout_governs_concurrent_edits_by_hlc`).
4. The log is already append-only and replication-safe for edits and transitions. Remote ops arrive as single-op edits keyed by mutation id. Replicas fold by `(HLC, id)`. A late older event rewinds and replays the suffix through `replay_suffix_partitioned`.
5. What is missing for history editing is a primitive that supersedes the input of an existing op. The nearest thing today is selective undo, which does not rewrite anything. `AmendLast` is unrelated: it extends the tail edit (naming collision).
6. Transition-driven reprojection (`reproject`, `STORE:18020`) drops all replay messages. Only ingest of new edits runs `replay_suffix_partitioned`, which produces warnings, quarantines and conflicts. Success/warning/error reporting per downstream mutation must therefore be added for the transition path.
7. Adding a transition kind is cheap on the wire and in `.spr` (the `REC_TRANSITION` frame is opaque, tag byte inside the payload) but wide in code: about 12 Rust sites, the JSON schema and fixture, a TS twin, a Python encoder, and hub refusal/grading rules.
8. The edit ledger is fixed at 64 slots (`ARTIFACT_HISTORY_LEDGER_CAPACITY`, `OS/🌿️vcs/🦀️.rs:196`). Copying edits into a new alternative or new document spends slots. Transitions (a `Vec`) do not.
9. Recommended primitive: one new transition `Supersede { scope, inputs[] }`. Overwrite is `scope = document`. New alternative is `Branch` at the committed head plus `Supersede` with `scope = that alternative` (copy-on-write, no duplicated edits).
10. Fork-to-new-document is the fallback when the shared-head move is unacceptable (see section 6.5). It needs a server-owned creation variant and is not required for a first cut.

## 1. The notion of "alternative" across the repo

### 1.1 Fold level: `fold_history` and the six transitions (`TRANS`)

- `HistoryTransition` (`TRANS:54-67`): `Revert`, `Reinstate`, `Commit(TransitionCheckpoint)`, `Branch{alternative_id,name,checkpoint_id}`, `Checkout{checkpoint_id,alternative_id?}`, `Repin`. Codec tags 0..5 (`TRANS:119-218`). Envelope: `diff.schema == "semio.history.transition"` (`TRANS:19`), empty inverse, id `transition-{hex16(blake3(actor|hlc|payload))}` (`TRANS:223`).
- `fold_history(edits, transitions, excluded)` (`TRANS:343-460`): a pure function of the event set, sorted by `(hlc.cmp_key(), id)`, where `cmp_key = (physical_ms, logical, actor)` (`REPL/🆔️ids/🦀️.rs:181`). Outputs `HistoryFold` (`TRANS:317`): `applied`, `redo`, `refused`, `checkpoint`, `alternative`, `changes`, `checkpoints`, `alternatives`.
- Alternative semantics in the fold:
  - `Branch` (`TRANS:430`): `checkout(checkpoint)`, register `FoldAlternative`, set `fold.alternative`.
  - `Commit` (`TRANS:415`): appends the new checkpoint id to the active alternative's chain.
  - `Checkout` (`TRANS:435`): sets `alternative` verbatim.
  - `checkout()` (`TRANS:462`): `active.clear()`, then `active = ⋃ edits of the checkpoint's change chain`, `redo.clear()`. Consequence: applied edits that belong to no change of that checkpoint (uncommitted work) become inactive on every replica.
  - Only `Revert` and `Reinstate` have an ownership rule (`foreign()`, `TRANS:378`; `refused`). `Branch` and `Checkout` have none.
- An alternative therefore is: a named chain of checkpoints in one shared log. Edits are global to the log; an alternative does not own edits or content variants. Two alternatives differ only in which change sets their checkpoints name.
- A second call site of the same fold is `HistoryLog::fold` (`SPRH:213`) for `.spr` files, the CLI and hydration.

### 1.2 Store level (`STORE`)

- Commands (`ArtifactCommand`, `STORE:2906-2993`): `CommitCheckpoint`, `CreateAlternative{name}`, `SwitchAlternative{alternative_id}`, `CheckoutCheckpoint{checkpoint_id}`, `Undo/Redo(+InLane)`, `AmendLast(+InLane)`, `IngestRemote`, `SetMergePolicy`, `ResolveConflict`. Command ordinals are frozen; the binary codec is at `STORE:13483-13640` (last used ordinal 16).
- Dispatch: `CreateAlternative` (`STORE:18134`): if there are no checkpoints and applied edits exist it commits them first; otherwise it branches at `current_checkpoint_id`, so with existing checkpoints and pending edits the fold drops the pending edits from `applied`. I found no test for that case; treat it as a hazard for the finalize flow.
- `SwitchAlternative` (`STORE:18149`) checks out the alternative's last checkpoint. `CheckoutCheckpoint` (`STORE:18158`) infers the alternative as "the one whose last checkpoint is this" (ambiguous for shared prefixes).
- Every structural command authors one transition via `commit_transition` (`STORE:18000`) and materializes through `reproject` (`STORE:18020`). This is the same path `admit_remote_transitions` takes for remote transitions (`STORE:17969`).
- Persisted shape: `ArtifactEnvelopeOwners` (`STORE:2654-2699`). Notable fields: `vcs{initial_snapshot, edits, changes, checkpoints, alternatives}`, `transitions: Vec<MutationEnvelope>` (the authoritative structural log, unbounded), `conflicts`, `edit_messages`, `lanes`, `migrated_from`, `owner`. `changes`, `checkpoints`, `alternatives` and `cursor` are projections re-derived by `fold_envelope_history` (`STORE:19631`).
- Provenance precedent for a derived document: `MigrationProvenance` / `envelope.migrated_from` (`STORE:2889-2902`).
- Hard ceiling: each of the four ledgers holds at most `ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` entries (`OS/🌿️vcs/🦀️.rs:196`, `reserve_edit_history_slot` `STORE:16511`). Pinned as "the app's hard interactive budget" in puzzle unit tests (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/🧪️tests/🔬️unit/🦀️.rs:467`). One gesture of 200 mutations is one edit (`STORE` tests `🔬️unit/🦀️.rs:3678`).
- Unit of history: replicas materialize one single-op edit per wire op with edit id = mutation id (`edit_from_operation_envelope` `STORE:19504`), while the author's replica holds multi-op edits (`Apply` with several mutations, coalesced `AmendLast` runs). History editing must therefore address ops by `MutationId` (as `Revert.mutation_ids` and `Commit.mutation_ids` already do), never by local edit id.

### 1.3 The `🌿️vcs` module in os modules (`OS/🌿️vcs/🦀️.rs`, 1288 lines)

Pure data algebra, no live document: `Author` (100), `Change` (130), `CompositionPin` (157), `Checkpoint` (170), `Alternative{id,name,checkpoint_ids}` (190), fixed-capacity `ArtifactHistoryLedger` (297), `ArtifactVcs` (802), id minting `mint_alternative_id` / `mint_change_id` / `mint_edit_id` / `mint_mutation_id` (`38-67`), `apply_mutation` (1147, drops messages), content-addressed checkpoint id (1210). `ArtifactStore` in `STORE` depends on it. Its `AGENTS.md` sibling under the plugin describes the same entities.

### 1.4 The `🌿️vcs` plugin (`✏️s/🔌️plugins/🌿️vcs`)

A demonstrator artifact type `s.vcs.vcs` (snapshot: title, counter, notes, status, tags; six mutations `rename/change-counter/change-notes/change-status/add-tag/remove-tag`) that exercises the history UI. It is not the version-control engine.

- Root: `🦀️.rs` (50 lines) registers `VcsApps { Editor(VcsArtifactApp<EditorApp<VcsPlayApp>>), Viewer(VcsArtifactApp<ViewerApp<VcsViewer>>) }`.
- History surfaces: `✏️editor/📌️panels/🗿️artifact/🦀️.rs` (checkpoint and alternative tree; per-row actions `checkoutCheckpoint` / `switchAlternative`) and `…/🎭️modes/✏️edit/🪟️windows/📜️history/🟦️.ts` (`VcsHistoryColumnViewModel.alternativeIds`).
- Test corpus shape to copy: `🧪️tests/🌿️mutate-vcs-1/{🥒️.feature,🦀️.rs,🐍️.py}` (Gherkin + Rust + independent Python implementation).
- The generic wrapper that gives every plugin history is `VcsArtifactApp` (`PLUGIN:23200`). Framework-owned history actions:
  - Definitions: `history_action_definitions()` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:1205`) lists `undo`, `redo`, `commitCheckpoint`, `createAlternative`, `switchAlternative`, `checkoutCheckpoint`, `revertToCommand`.
  - Routing: mapped to `ArtifactCommand` in `history_command` (`PLUGIN:26992`) and committed in `commit_framework_history_route` (`PLUGIN:29182`).
  - Constants and rejection lists: `HISTORY_ACTION_IDS` (`PLUGIN:23604`), `VIEWER_REJECTED_ACTION_IDS` (`PLUGIN:23656`).
  - Job factories: each reserved action has a job factory (`PLUGIN:17862` for revert).
  - Descriptors: listed in about 45 plugin descriptors (`✏️s/🔌️plugins/*/🔣️.json`).
- `revertToCommand` (`PLUGIN:28862`, `29102`) is not history editing: it performs repeated `Undo` until the target edit is the tail (`framework_revert_unit` `PLUGIN:29245`). It appends `Revert` transitions; it never touches inputs.
- The command log (`CommandLogEntry`, `PLUGIN:11964`) is runtime-only; `HistoryView` (`PLUGIN:12046`) is rebuilt per generation (`build_history_view` `PLUGIN:26812`). Row `op_lines` print `edit.forwards` (`PLUGIN:26832`).

### 1.5 Space-level alternatives

`SpaceCheckpoint{members: Vec<SpaceMemberPin{document_id, checkpoint_id, alternative_id}>}`, `SpaceAlternative`, `SpaceHistorySnapshot` (`STORE:21418-21484`): the `os.space.history` meta-document is itself an ordinary event-sourced artifact, and a space alternative fans out to each member's own `(checkpoint, alternative)`. This is the upper level; it does not change how a single artifact is amended.

### 1.6 UI-only "alternatives" (the two TREE tickets)

`TREE-ITEM-ALTERNATIVE-BRANCHES-NAVIGATION` and `VIOLATION-TREE-WITNESS-ALTERNATIVES` (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️03/☀️16/…/🎫️ticket.json`, both closed, ticket.json only) added `TreeDataItem.alternatives?: TreeDataItem[][]` and `TreeItem` props `branchCount`, `activeBranchIndex`, `onBranchChange` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:1104, 1560-1564, 2535-2537`, `< n/N >` at 2868 and 2973). Purely presentational; no VCS link. Reusable for the history editor: a mutation row can show its inputs as `< original | supersede 1 | supersede 2 >` using this control.

### 1.7 A separate VCS: the flow module

`OS/🌊️flow/🌿️vcs/🦀️.rs` (2100 lines) is `FlowRetainedVcs`: undo/redo/checkpoint with its own bounded credits (`FLOW_VCS_MAX_HISTORY = 256`), no alternatives, not driven by the transition fold. History editing either excludes flow explicitly or reconciles it later.

### 1.8 Artifact identity, open, save, copy

- Identity: hub scope `DocumentScope{space_id, document_id}` (`OS/📇️directory/🧬️schema/🦀️.rs:92`); immutable `DocumentDescriptor` with `bootstrap_snapshot_hash` and `bootstrap_frontier` (1445); `ArtifactFrontier` (2724) and `ArtifactCheckpoint{parent_checkpoint_id, baseline_frontier, pack, spr}` (2794) form a linear checkpoint lineage. Local: `ArtifactDocumentKey::{Local, Hub}` (`SYNC:187`).
- Persistence classes (`SYNC:90-95`): the four required classes exist as `PersistenceDataClass`. `PersistenceBinding::Folder` = persisted local-only, `::Hub` = persisted shared (`SYNC:144-168`). `Lane` `command` is `PersistedShared`; `preview` and `presence` are `EphemeralShared` (`SYNC:125`).
- Open: `os.open-artifact` (`🧰️framework/🛍️products/💻️os/🎮️commands/🗿️open-artifact/🦀️.rs`, constants only) and `os.open-artifact-with`; the Space index guest relays them as `ReplayShellCommand` (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗿️open-artifact/🦀️.rs`) with `{artifactRef, documentId, spaceId, schema}`; wire `AppCommand::OpenArtifact{seq, artifact_ref, role, plugin_id, app_id}` (`OS/📡️spr/🧵️channel/🦀️.rs:2637`).
- Create: `🌱create-artifact` relays `os.create-space-artifact{kindChoice, name}` (same commands folder). ShellHost accepts exactly the args `kindChoice,name` (`OS/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1606`). The hub saga takes `SpaceArtifactCreateV1{schema, request_id, expected_catalog_generation_id, kind_id, name}` only (`OS/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs:129`). Genesis is derived server-side by `materialize_genesis` (`🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs`), creation pair capped at 1 MiB (`ARTIFACT_CREATION_PAIR_MAX_BYTES`).
- There is no "save as", "duplicate" or "fork" for artifacts (repo-wide search of commands, the space plugin and the hub: no match). The only "duplicate" is `duplicateAppInstance` for studio nodes, which is unrelated. The space commands also include `🏷️rename-artifact`, `🗑️delete-artifact`, `🕒touch-artifact`.
- Building blocks for a copy exist: `snapshot_pack()` / `print_document_pack` (`STORE:18555`, `12688`) print pack+spr; `parse_document_pack` (`12933`); `replay_envelopes_onto_pair` (`STORE:10704`) folds an envelope stream onto a base pair (used by hub Check In). The `.spr` header carries `doc_id` and transition/edit envelopes rebuild `document_id` from it (`HistoryTransitionRecord::to_envelope`), so re-stamping a copy's document id is a header change, but composed children are separate documents (section 6.5).

## 2. Multi-user reality today

### 2.1 How history reaches other replicas

- Author side: an edit's ops become one `MutationEnvelope` per op (`mutation_envelopes_from_edit_since`, `REPL/🔗️causal/🦀️.rs:816`); a structural command's transition goes through `commit_transition` (`STORE:18000`, seeds the causal DAG, pushes to `pending_report.outbound`); `flush_outbound` (`STORE:19334`) sends one `BackboneMessage::Mutations` batch per command. No snapshot ever crosses the wire (`BackboneMessage::{Genesis, Mutations, Ack, Member}`, `STORE:19707`).
- Attach or join: `attach_backbone` (`STORE:18580`) announces `Genesis` (initial pack, identity only, `verify_genesis` at `STORE:19325`) then the whole event log (edits' ops in ledger order, then transitions in HLC order; `event_log` `STORE:18593`, `announce_history` `STORE:18667`).
- Hub path: the local sync actor queues envelopes in an `outbox` and sends `ClientFrame::Commands` (`SYNC:2840`); the hub admits, commits to its per-document engine and relays `ServerFrame::Commands{envelopes, origin, frontier}` to peers (`HUBB:5287-5310`, `5369-5412`); late joiners get a catch-up tail from `HUB_CATCH_UP_ORIGIN = "hub.catch-up"` (`HUBB:5279`); a replica beyond retention reseeds from the canonical checkpoint pair (`ArtifactActorMsg::Reseed`, `SYNC:242`) and re-sends unacked ops.
- Receiver: `pump_with_reports` (`STORE:19286`) → `ingest_remote` (`STORE:18835`): causal DAG insert (dependencies hold an envelope until known), transitions collected as `ready_transitions`, op envelopes turned into single-op edits (`edit_from_operation_envelope`). An edit whose HLC is older than the applied tail causes a rewind: `fold_history(order[..k])` then `replay_suffix_partitioned(base, order, k, edits, policy)` (`STORE:17038`) recomputes each op's diff, inverse and messages against the shifted state and decides accept versus quarantine per edit, giving the same verdict as a fresh in-order arrival (`STORE:17026-17036`).
- Idempotence and immutability: a known mutation id or transition id must repeat its exact payload (`assert_equivalent_remote_*`, `STORE:19106-19140`, `admit_remote_transitions` `STORE:17973`); the hub refuses `mutation id … already committed with different content` (`DBART:2047-2053`). Edits are therefore immutable by id; any rewrite of an input needs a new event.

### 2.2 Global head (the important multi-user finding)

`Checkout` and `Branch` are folded unconditionally on every replica. Fold law (test `checkout_governs_concurrent_edits_by_hlc`, `REPL/🔗️causal/🔀️transition/🧪️tests/🔬️unit/🦀️.rs:158-165`): an edit older (by HLC) than a checkout is subject to it, a newer one lands on top, on every replica. `createAlternative`'s catalog text says "edited without disturbing the current one" (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:1217`); that holds single-user only. If user A branches or switches, user B's projection jumps too, and B's uncommitted edits older than the event are deactivated on all replicas (recoverable by switching back, since the edits stay in the log). The hub does not restrict who may check out (`foreign_history_transition_refusal` only covers `Revert`/`Reinstate`, `DBART:2004-2010`).

Consequences for history editing:
- Time-travel mode ("show the edited mutation with downstream not applied") must not be a `Checkout` transition. It must be an ephemeral local-only overlay, otherwise it yanks collaborators.
- A "new alternative" published as an in-document alternative is a shared, all-user event unless the head is split per viewer (section 6.4).

### 2.3 If user A rewrites mutation #5 while B keeps appending: today

There is no rewrite primitive. The closest sequence is A undoing own edits and re-applying:
- `Revert` names only the author's own ops; foreign ones are `refused` on every replica (`TRANS:378-410`) and refused before the WAL at the hub (`DBART:2086`).
- Selective undo works because each revert is a per-edit fold change, so B's later edits stay applied and replay over the changed state. But this replay is `reproject` → `project_applied` → `fold_history` (`STORE:18042-18060`, `19672-19690`), which calls `apply_mutation(...)?.0` and discards the messages. A downstream op that now fails is silently degraded: no `EditMessages`, no `Degraded`/`Quarantined` conflict, no per-edit verdict. Only `ingest_remote` of edits (and `accept_quarantined_events`) produces those. This is the largest functional gap for "replay reports success, warnings or errors".
- Conflict resolution is replica-local: `resolve_conflict` (`STORE:19167`) changes local `Conflict` state and seeds the local DAG; it is not an event. `MergePolicy` is also authority-local and never on the wire (`STORE:2979-2985`). Two replicas with different policies can hold different `applied` sets for the same event set. A fix for a failing downstream mutation must therefore be a replicated event (another supersession), not a local discard.

### 2.4 Short outage handling (rides on any new transition unchanged)

`DocumentLink` state machine (`SYNC:1189-1309`): `Linked → Unlinked{backoff} → Expired` at a bounded ceiling (`2 × HUB_RECONNECT_MAX_MS` plus one capped backoff); `admits_local_edits()` is true while `Unlinked`, so local edits and transitions keep applying and queue in `outbox` (`queue_outbox` `SYNC:2793`, `flush_outbox` `2840`); after reconnect a Commands frame that already contains queued ids settles them without resend (`settle_committed_envelopes`, `SYNC:1331`). No rollback exists for a rejected transition: `rollback_envelope` returns `None` for transitions (`SYNC:1401-1404`, test `a_history_transition_has_no_inverse_rollback`), so a hub-refused `Supersede` would leave the author ahead of the hub until reseed. Preflight it locally with the same predicate the hub uses.

### 2.5 Ceilings that matter to the design

- 64-slot edit/change/checkpoint/alternative ledgers (section 1.2). Design must not copy edits.
- Hub check-in runs `replay_envelopes_onto_pair` (`STORE:10704`), which folds through `ingest_remote` under the default `MergePolicy::Normal` (`STORE:15826`) and errors when any envelope is quarantined (`STORE:10727`: `replay-envelopes quarantined a ledger envelope as a conflict`). Automatic policy Check Ins (`observe_checkpoint_policy` `HUBB:4665`) therefore fail on a log whose replay produces `Error`/`Fatal` under `Normal` (rejects `Error` and `Fatal`; `REPL/🧾️wire/🦀️.rs:247-252`). A finalize gate must be at least as strict as `Normal`.
- `edit_messages` ledger: 8192 entries × 4 KiB (`STORE:14266-14269`), keyed by edit id.

## 3. Evaluation of the three designs

Notation: OW = overwrite, NA = new alternative.

| Criterion | (a) Supersede transition (append-only, fold resolves amended input) | (b) Branch-off alternative with re-authored suffix | (c) New document |
|---|---|---|---|
| Log stays append-only | Yes. Original ops untouched (`forwards` never rewritten, same contract as quarantine, `STORE:17032`). | Yes. | New log per doc; source untouched. |
| New wire kind | `HistoryTransition` tag 6 (schema, Rust, TS, Python, hub rules). | None. Uses `Branch`, `Apply`, `Commit`. | None in transitions; new creation variant in hub saga and directory. |
| Edit ledger cost | 0 slots (transitions are a `Vec`). | One slot per copied edit; a 40-edit suffix spends 40 of 64. Rules it out while the cap stands. | New ledger; copy is bounded by 64. |
| Provenance | Original and each supersession retained. | Lost (new ids, new authors). | Needs a `forked_from` record (precedent `migrated_from`). |
| Foreign-authored downstream ops | Replayed, not copied. | Cannot be re-authored (`Revert` ownership and `actor` on edits break). | Copy keeps actors. |
| Concurrent collaborators | Overwrite: no head move; downstream replays through existing rewind. | `Branch` moves the shared head for all (section 2.2). | Fully isolated. |
| Short outage / offline | One event in the outbox; deterministic fold on arrival. | N events. | Local copy works offline; hub creation needs the hub. |
| Hub impact | Opaque payload; needs foreign-author and grading rules. | None. | New saga variant (server derives the pair, as for Check In), pair cap 1 MiB, three directory events. |
| Composed children (own single owner, separate envelopes) | Unaffected (pins by checkpoint id). | Unaffected. | Owned children must be deep-copied and child ids inside domain ops remapped: not framework-generic. |
| UI reuse | History rows, tree `< 1/N >`. | Existing `alternativeIds` panels. | New concept in the space index. |

Design (a) is the only one that satisfies "overwrite" without spending edit slots. Design (b) alone is not viable while the 64-slot cap stands and carries the shared-head hazard. Design (c) is the safe isolation option for the "new alternative" product prompt but needs new hub machinery and cannot copy composite artifacts generically. Extension (a+): give the transition a `scope`, so the same primitive also yields a copy-on-write alternative without duplicating edits (section 6).

Naming caution: do not call the new transition `Amend`. `Amend` already means "extend the last uncommitted coalescing edit": `ArtifactCommand::AmendLast` (`STORE:2967`), `amend_command` (`STORE:18335`), `CommandHeaderLine::Amend` (`STORE:12992`), `Emit::amend_config`, hub comments (`STORE:19347`). Use `Supersede`.

## 4. Persistence formats and the cost of a new transition kind

Formats:
- `.pack`: initial snapshot (`ArtifactPack::encode_pack`), the genesis; `.dsl` is its text twin.
- `.spr`: binary event log. Frames: `REC_DOC`, `REC_EDIT`, `REC_TRANSITION = 0x43` (critical; one frame per transition; `SPRH:905`; payload `format u8 | id | actor | hlt | dependencies | payload` with the transition bytes opaque, `SPRH:910-948`), `REC_CONFLICT = 0x42`, `REC_COMPOSITION = 0x41` (non-critical). `RetainedHistoryDecode` validates the payload via `decode_history_transition` (`SPRH:1351`). Older readers must refuse unknown critical frames, which is correct for a greenfield repo.
- `.ops` text: two grammars.
  - The `spr` level prints one generic line `transition <id> actor= hlc= dependencies=[…] payload="<base64>"` (`SPRH:293-297, 419`), independent of the kind.
  - The store's document text mirror `OpsHeaderLine` has a semantic line per kind (`revert/reinstate/commit/branch/checkout/repin`; `STORE:11636-11729`), with ids re-derived and checked on parse (`STORE:11895`).
- Hub WAL: `protocol::encode_envelope` bytes (`DBART:2095-2097`), opaque.
- `folder://` (multi-document event log) and `*.json` single-blob export are the local bindings (`SYNC:145`).

Change list for `Supersede` (paths relative to repo root, aliases above):

| Layer | What changes | Where |
|---|---|---|
| Rust codec + fold | enum variant; tag 6 encode/decode; `HistoryFold.superseded`; fold arm with author rule and `refused`; scope resolved after the loop | `TRANS:54, 119, 173, 317, 390-453` |
| Rust unit tests | extend `every_transition`, round-trip, malformed, id, fold laws | `REPL/🔗️causal/🔀️transition/🧪️tests/🔬️unit/🦀️.rs:3-60, 98-200` |
| JSON Schema | add `supersede` to `Transition.oneOf` (edit in place: `history-transition-v1` id, no v2, greenfield) | `REPL/🔗️causal/🧬️schema/🔀️history-transition-v1/🔣️.json` |
| Payload corpus | new byte cases (`payloadHex` + `expect`) | `REPL/🔗️causal/🧫️fixtures/🔀️history-transition-v1/🔣️.json` |
| Python oracle | extend the independent encoder; it currently lives only in the ticket-19 folder | `TICKET19/🧪️generate-history-transition-fixture.py` |
| TS twin (test only) | Ajv validation runs the corpus; add supersede shape | `REPL/🧪️tests/🧪️history-transition/🟦️.ts` (94 lines) |
| Scenario fixture + TS fold twin | new `…-supersede-v1` steps (edit, supersede, expect, reload, hub-restart) mirroring the redo fixture | `REPL/🔗️causal/🧫️fixtures/🗄️durable-collaborative-redo-v1/🔣️.json`, `REPL/🧪️tests/🗄️durable-collaborative-redo/🟦️.ts` (131 lines), Rust test `…/🧪️tests/🔬️unit/🦀️.rs:243` |
| `.spr` module | no frame change; re-exports in `OS/📡️spr/🦀️.rs` (line 31); maybe one fixture case | `OS/📡️spr/📜️history/🧫️fixtures/🔀️transition-record/🔣️.json` |
| Store text and commands | `OpsHeaderLine::Supersede`; `ops_line_from_transition`; parse arm; new `ArtifactCommand` variant (ordinal 17), `projection_cause`, `CommandHeaderLine`, text and binary codecs | `STORE:11636, 11861, 12780-12798, 2906, 2997, 12968, 13281-13403, 13483-13640` |
| Store semantics | effective-forwards overlay (original `forwards` kept) in `fold_history`, `project_applied`, `replay_suffix_partitioned`, `accept_quarantined_events`; transition reprojection through `replay_suffix_partitioned`; dry-run preview; digest and revision must include effective inputs | `STORE:19672, 18042, 17038, 19203, 17969, 18020`; `CursorRevisionAccumulator` via `bump` `STORE:19387` |
| Retirement discipline | every new owned heap type needs `artifact_retire_struct!`/retire impls and drop-witness tests | pattern `OS/🏪️store/🧩️composition/🚪️open/🦀️.rs:349` |
| Hub rules | generalize foreign-author refusal to `Supersede.targets`; grade `Supersede` by declared `target` instead of skipping it | `DBART:2004, 331, 340-347`; fixtures `OS/🛢️db/🗿️artifact/🧫️fixtures/⚔️concurrent-write/🔣️.json:75` |
| React/wasm worker | no change (schema filter only) | `OS/🏪️store/👷️worker/🟦️.ts:2308`, `REPL/🟦️.ts:951` |
| Action surface (only if exposed as a framework action) | `history_action_definitions`, `HISTORY_ACTION_IDS`, reserved job factories, ~45 plugin descriptor JSONs, test allowlists, `📜️script.ts:920` | large fan-out; prefer a host-level history-editor command instead of an eighth plugin action |
| e2e | supersede scenario next to foreign-undo scenario | `OS/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts:618-731`, `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:4178` |

The compiler finds the exhaustive matches on `HistoryTransition` (`ops_line_from_transition`, the parse arms, the fold); that is the reliable checklist.

## 5. Hub handling of transitions

- The hub-side engine (`DBART`) is domain-blind: `diff_entries` returns nothing for any schema other than the DB path-map schema (`DBART:176-180`), so plugin ops and transitions are both opaque payloads.
- Admission per envelope: `document_id` match, actor equals socket actor (`HUBB:5387-5396`), declared-batch limits (`undeclared_batch_refusal`, `HUBB:6214`), security gate (`admit_writes`, `HUBB:5320`), `dependencies` must already be applied or be earlier in the same batch (`plan_one`, `DBART:1975-1985`), immutability check by id (`DBART:2047-2053`), foreign-undo refusal (`DBART:2086`).
- Grading: `unseen_concurrent_writes` returns nothing for any history transition and `touch_writes_fields` excludes them (`DBART:331-347`); other writes are graded by declared `target` overlap and `observed` (`DBART:349-373`). A `Supersede` writes domain fields, so it needs the opposite treatment or concurrent same-target writes go ungraded.
- Ordering: the hub commits in arrival order and assigns ledger positions; replicas fold in HLC order. Causality is carried only by `dependencies`.
- Check In: `materialize_check_in` folds the committed ledger onto the active checkpoint pair through the plugin codec (`🌎️hub/🗿️artifact-authority/📌️check-in/🦀️.rs:114-142`); the resulting pair carries the full event log including transitions, so a `Supersede` is preserved and re-folded automatically. Failure mode: a supersede that makes replay quarantine under `Normal` makes Check In fail (section 2.5).
- Retention and lag: `ArtifactRetention{retained_floor, checkpoint_lineage_head}` (`OS/📇️directory/🧬️schema/🦀️.rs:2810`); the checkpoint pair contains the edit history, so an op below the ledger floor is still supersedable through its pair.
- `🖥️server` (`🧰️framework/🛍️products/🖥️server`) is the generic authority-actor and CQRS command-bus framework; it does not see history transitions.
- `📡️spr` is the os-side `.spr` codec, CLI (`verify`/`log`/`inspect`, folds the history) and materialize module; transitions there are opaque records plus the same `fold_history`.

## 6. Recommendation

### 6.1 One primitive: `Supersede`

```
HistoryTransition::Supersede {
    scope:  Option<String>,           // None = document scope; Some(alternative_id) = only while that alternative is active
    inputs: Vec<SupersededInput>,     // atomic batch, one HLC, one event
}
SupersededInput { target: MutationId, payload: Vec<u8> }   // OpBinary bytes of the artifact's own Mutation
```

Envelope: `diff.schema = HISTORY_TRANSITION_SCHEMA`; `dependencies = all targets` (causal hold-back, hub `plan_one` requirement); `target` = union of the replacement mutations' `conflict_target()` (hub grading); `observed` as for any op.

Fold rules (pure function of the event set, same style as `Revert`):
1. Ownership: every `target` must be an op authored by the transition's actor, otherwise the whole transition is `refused` (mirror of `TRANS:378`). Relaxation worth deciding: a scoped supersede (`scope = Some`) is confined to its alternative and could be allowed on foreign ops, because it does not change what anyone else sees.
2. Effective input of op X = payload of the last non-refused supersede naming X, in `(hlc, id)` order, whose scope is `None` or equals the final `fold.alternative`; otherwise the original payload. The original op is never modified.
3. A supersession does not move X in the applied order; X keeps its edit slot, id and HLC. Downstream = everything after X in applied order.
4. `Commit.mutation_ids`, `Revert`, `Reinstate` keep naming slots (mutation ids); they compose with `Supersede` independently.
5. Unknown target or repeated transition id is a fold error (same as `Revert`, `TRANS:196-207` tests).

Undo of a finalize is a new `Supersede` carrying the previous effective payload; no new kind and no in-place deletion.

### 6.2 Time-travel session and the finalize gate

- Time-travel mode and pending input edits are ephemeral local-only state (`PersistenceDataClass::EphemeralLocalOnly`; persisted local-only draft if crash survival is wanted). No transition is authored until finalize. Never a `Checkout` (section 2.2).
- Add `preview_supersede(inputs) -> ReplayReport{ per-edit EditMessages, worst, quarantined }` in the store, a pure function over `replay_suffix_partitioned` (it is already a pure static function of `base, order, k, edits_by_id, policy`, `STORE:17038`) with an `edits_by_id` map whose forwards are replaced by the candidate inputs. This is the success/warning/error report per downstream mutation.
- Finalize gate: no message at or above what `MergePolicy::Normal` rejects (`Error`/`Fatal`); warnings pass and are kept in the durable `edit_messages` ledger so history can show them. `Normal` is what hub Check In replays under.
- Authoritative path: `dispatch(Supersede)` and `admit_remote_transitions` must both run the same `replay_suffix_partitioned` from the first superseded slot (not the message-dropping `reproject`), record messages with `replace_edit_messages`, and raise the same `Degraded`/`Quarantined` conflicts as an ingested edit would.
- Store gotchas to cover in tests: `reproject` compares only `applied`/`redo`/`checkpoint` (`STORE:18024-18034`), so an overlay-only change would not reproject; `content_revision` (`STORE:19399`) and the revision accumulator must digest effective inputs, or stale-revision commit validation stays green after an amend; `Edit.inverse` is stale after a supersede (used only by the tail undo cache, which is dropped on a cold reproject, and by `rebased_inverse`, which replay recomputes).

### 6.3 Overwrite

`Supersede{scope: None}` with all edited inputs of the session. Own ops only. One event replicates through the existing `Mutations` batch; every replica rewinds to the first target's HLC position and replays; hub Check In folds it into the next checkpoint pair. The log keeps the originals, so this is a supersession, not destruction. True destruction (privacy or size) would be a separate compaction that bakes inputs into the checkpoint pair and drops the prefix; out of scope here.

Concurrency table (A supersedes #5, B appends):
- B's later edits replay through the existing rewind; failures become per-replica `Quarantined`/`Degraded` conflicts by local `MergePolicy` (pre-existing property, section 2.3).
- B reverts own edit concurrently: commutes with the supersession (different slots).
- Two supersedes of one op: same author only; last `(hlc, id)` wins on every replica.
- A foreign author supersedes A's op: refused on every replica and at the hub.
- A offline: event waits in the outbox, deterministic on arrival (section 2.4).

### 6.4 New alternative

Mapping (in-document, copy-on-write, no duplicated edits):
1. Commit any pending edits (the fold drops uncommitted edits at a `Branch`, section 1.2).
2. `Branch{alternative_id, name, checkpoint = committed head}`. Branch at the head, not before the edited op, so all current edits stay applied and only the overlay differs.
3. `Supersede{scope: Some(alternative_id), inputs}`.
4. Optional `Commit` on the new alternative for the scoped state.

The existing alternative UI (`alternativeIds`, `switchAlternative`, the tree `< n/N >` control) applies unchanged. Requirements this adds beyond overwrite: per-alternative message recomputation on switch (the `edit_messages` ledger is keyed by edit id only), overlay-aware `reproject` on `SwitchAlternative`, and an explicit `alternative_id` on `Checkout` (the `CheckoutCheckpoint` inference at `STORE:18162` is ambiguous for shared prefixes).

Shared-head limitation and choice for the coordinator (this is the real decision):
- Accept and guard (v1): `Branch` and `Checkout` stay shared events. The UI confirms "switches every collaborator" when presence shows another author (`PresencePeer` lane, `EphemeralShared`), and finalize commits first. Document that `createAlternative`'s "without disturbing the current one" text is single-user only.
- Head split (follow-up, principled): make the head per-viewer persisted local-only state and keep `Commit`, `Branch` registration and `Supersede` as shared facts. Needs an alternative tag on edits (a sparse map like `envelope.lanes`, `STORE:2690`) and a canonical trunk head for hub Check In. It fixes the contract for all users, but touches `reproject`, hydration, the hub materialization and `HistoryView`; not a prerequisite for v1.

### 6.5 Fallback: new alternative as a new document

Use when an in-document head move is unacceptable or the amended set touches many foreign ops. Needs: a server-owned creation variant next to `os.create-space-artifact` (`SpaceArtifactCreateV1` gets a fork source: document id plus checkpoint or frontier), a genesis derived by the hub (fold of source ledger plus supersessions, printed with `print_document_pack`, the same shape as `materialize_check_in`), `forked_from` provenance on the envelope (precedent `migrated_from`), and the three creation directory events. Local artifacts: write a new pair with a new `REC_DOC` id (copy of `snapshot_pack`). Blockers: composed artifacts own separate child documents (single owner, ids inside domain ops), so a generic deep copy is not possible; and the 1 MiB creation pair cap.

### 6.6 Tests to write first (per AGENTS.md: language-agnostic plus independent implementation)

1. Payload corpus cases for `Supersede` (bytes generated by the Python encoder, checked by the Rust codec and TS Ajv).
2. Step fixture `…-supersede-v1` (edit, supersede, foreign refusal, revert interleaved, reload, hub-restart) run by the Rust fold and the TS twin.
3. Store law: superseding #5 then folding the same event set in shuffled order yields identical applied set, state and messages on two replicas (extend the existing two-replica convergence tests).
4. Preview equals authoritative: `preview_supersede` report equals the report of `dispatch(Supersede)` for the same inputs.
5. Hub: foreign supersede refused; supersede graded against a concurrent same-target write; Check In of a log with a supersede prints and reloads.
6. Ledger: 200 supersessions do not consume edit slots.
7. Two-human e2e next to the existing foreign-undo scenario (`OS/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts:618`).

### 6.7 Open questions

1. Confirm ownership scope: own ops only for unscoped supersede, foreign allowed inside scoped alternatives.
2. Accept the shared-head guard for v1, or schedule the head split first.
3. Whether the flow module's separate VCS (`OS/🌊️flow/🌿️vcs`) is in scope.
4. Expose finalize as a framework action (large descriptor fan-out) or as a host-level history-editor command.
5. Composed artifacts: history editing per member document with `MutationMeta.group_id` composite gestures (`REPL/🎮️mutation/🦀️.rs:1378`) is not analysed here.
