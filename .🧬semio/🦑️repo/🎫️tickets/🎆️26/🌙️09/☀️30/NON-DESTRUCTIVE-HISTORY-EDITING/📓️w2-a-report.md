# 📓️ W2-A — Plugin runtime: non-destructive history editing

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, work package W2-A (plan items 1–8, design §4, §7, §10). Status:
**implemented and verified**.
Nothing was committed. No ticket or goal was opened or closed.

Aliases: `P` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`, `K` = `🧰️framework/🔨️modules/🎠️kernel`,
`M` = `🧰️framework/🔨️modules/🛂️manifest`.

## 1. What landed

### 1.1 `Emit.transaction` (landed first; the signature was sent to main for W2-D)

- The field `Emit.transaction: Option<protocol::TransactionRef>` and the constructor `Emit::commit_transaction(transaction, mutations)`.
  - An empty mutation list means no transaction.
  - There is no description, so the row label comes from the mutation labels.
- The retained ladder passes the transaction to `begin_apply_batch`/`begin_outbound_apply_batch`, where `fold_batch_item` stamps it. `dispatch_emit_inner` passes it to `ArtifactCommand::Apply { transaction }`.
- A transaction together with a `coalesce_key` or `child_emits` is refused as `toolTransaction.shape` (one commit is one plain edit).

### 1.2 Kernel history wire (§10)

Files: `K/🦀️.rs` region `🔖️HistoryWire`, TypeScript twin `K/🟦️.ts`.

- New types:
  - `HistoryTransaction`
  - `HistoryMutationMessage`
  - `HistoryMutationEntry`: `{mutationId, position, opIndex, label, worst?, messages, superseded, withdrawn, editable, pending, edited}`
  - `HistoryTimeTravelStage`
  - `HistoryTimeTravelReview`: `noChanges | needsReplay | blocked | ready`
  - `HistoryTimeTravel`: `{sessionId, generation, stage, target?, targetLabel?, done?, total?, worst?, blocking, fault?, acceptedCount, review?, rerunnable}`
- `HistoryEntry` gains `editId?`, `transaction?`, `worst?` and `mutations[]`. `HistoryPatch` gains `timeTravel?`.
- Row key: `HistoryEntry::key()` (TS `historyEntryKey`) is `edit:<id>`, else `seq:<seq>`.
- Serde carries `Severity` through `history_severity_serde`.
- Deviations from §10, all reported to main:
  - `editId` is optional, because session rows without an edit still exist.
  - `review` and `rerunnable` are additive, adopted from W1-B's follow-up.
- Schema: `K/🧬️schema/🔣️history-patch/🔣️.json`. Fixture: `K/🧫️fixtures/🧫️history-patch/🔣️.json` (6 valid, 6 invalid cases).
- Tests:
  - Rust `K/🧪️tests/🧪️history-patch/🦀️.rs` checks serde against the value codec, round trips, keys and refusals.
  - TS `…/🟦️.ts` validates with Ajv (third-party) and checks the row keys. It is registered in the kernel vitest include list.

### 1.3 Manifest (`history_action_definitions` region, new region `🔖️HistoryEdit`)

- Twelve reserved verbs (`HISTORY_EDIT_ACTION_IDS`):
  - `historyEditBegin{mutationId}`
  - `historyEditInput{path, value, generation?}`
  - `historyEditUseSelection{path, generation?}`
  - `historyEditWithdraw`
  - `historyEditAccept`
  - `historyEditDiscard`
  - `historyEditFinalize`
  - `historyEditCommit{choice?, name?}`
  - `historyEditBack`
  - `historyEditExit`
  - `historyEditCancelReplay`
  - `historyEditRerun` (from W1-B's follow-up)
- Every verb:
  - is `History` kind and `resumable_framework`;
  - is never in the palette and has no chords;
  - has EN/DE `describe` and `use_when`;
  - is agent-addressable (MCP-visible).
- `historyEditCommit` is `.destructive()`.
- Every other verb takes an optional hidden `generation`. Without it, the verb addresses the live session.
- `history_action_definitions()` = the seven history-lane verbs followed by the history-edit verbs.
- Constants: `HISTORY_EDIT_ARG_*`, `HISTORY_EDIT_CHOICE_OVERWRITE`, and `HISTORY_EDIT_FINALIZE_DIALOG_ID = "finalizeHistoryEdit"`.
- `history_edit_finalize_dialog()` equals W1-E's `🧫️dialog-choices` dialog except for its literal default name:
  - submit: new alternative, with a `name` arg;
  - choice: destructive `overwrite` with Danger tone;
  - cancel: `historyEditBack`.
- `noteShellCommand.label` is now an `any` argument. It takes `{en, de}` or a full `LocalizedLabel`, and plain text stays data (W2-B item 1).
- TS twin (`M/🟦️.ts`) exports `HISTORY_EDIT_ACTION_IDS` and the argument, choice and dialog constants.
- Fixture `M/🧫️fixtures/🧫️history-edit-actions/{🔣️.json,🧬️schema/🔣️.json}`. Tests: Rust `M/🧪️tests/🧪️history-edit-actions/🦀️.rs` and TS `…/🟦️.ts` (Ajv 2020 with hostile rows).
- New nx target `@semio-tech/framework-rs:test-history-edit-actions` (`🧰️framework/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`).

### 1.4 Plugin runtime (`P/⏪️time-travel/🦀️.rs`, new, mounted as `app::time_travel`)

**`TimeTravelLedger<A>`**, which sits beside `tool_runs`:
- It holds:
  - the pure `TimeTravelSession` (W1-B);
  - the draft editor: input descriptors from `mutation_input_defs`, a precompiled `OwnedJsonSchemaValidator` with the registry's cross-document `$ref`s, the kind being rebuilt, the draft payload, and the draft's own outcome;
  - the editing preview;
  - the running `EditReplay`;
  - the finished `EditReplayResult`, whose head is the reviewing preview;
  - the retirement queues.
- Snapshot aliases retire through `store.retire_snapshot_alias`, and discarded ops through `retire_cold`.
- Close ladder and terminal witness are wired.

**Verbs:**
- They are host-driven at the head of `dispatch_action`: `if is_time_travel_action_id(action) { return Box::pin(self.dispatch_time_travel_action(..)).await; }`.
- A viewer rejects them (`viewer_rejects_action`, which also filters the roster). `is_framework_reserved_action_id` includes them, and the debug branch labels them "host-driven".
- A refusal is `{rejected: code}` and repaints the history body. The codes are:
  - the session's `timeTravel.illegal|stale|blocked|empty`;
  - the plugin's `timeTravel.busy|unknown-mutation|not-editable|unknown-input|invalid-input|no-selection|name-required|name-invalid`.

**Generic input path:**
1. Pointer lookup of the descriptor (nested objects and array elements).
2. Coercion to the declared shape: numbers from text, integer rounding, booleans, typed reference ids (W1-D's edit), vectors, and colours from `#rrggbb` with the stored alpha kept.
3. Set the RFC 6901 path in `payload_value()`.
4. `mutation_input_instance` plus schema validation.
5. `with_payload_value`, then canonical `encode_op`, then `Draft`.

A refused value keeps the draft and records its reason on the editor.

**`historyEditUseSelection`** reads `interaction_selection_snapshot()` for the input's `Reference.domain` at its granularity. It keeps typed ids and clips to `max_items`.

**Effects:**
- `ShowPreview`: `store.state_before(target, accepted)`, then the draft folded exactly like the store folds it (an apply refusal is Fatal and a no-op).
- `StartReplay`: `begin_report_replay`, latest-wins.
- `CancelReplay`.
- `OpenFinalizePrompt`: `Effect::OpenDialog` seeded with the localized `alternativeNameDefault`.
- `CommitOverwrite`/`CommitAlternative`: `commit_finished_replay` (no second replay).
  - `Stale` becomes `FinalizeFaulted{timeTravel.stale}`, which replays again.
  - `Rejected` becomes `FinalizeFaulted{timeTravel.blocked}`.
  - Every commit answers `Finalized` or `FinalizeFaulted`.
- `Close`.
- Commit names are checked with `is_time_travel_alternative_name` and are not trimmed.

**Driver:**
- `drive_time_travel_turn` is gated by `time_travel_has_pending_work` in `advance_typed_operation_publication` and in both pending-work gates. Each turn:
  1. retirement;
  2. base watch;
  3. one replay slice of at most 4 ms (`EditReplay::step(store.replay_edits(), deadline)`).
- The base watch sends `BaseMoved` with re-resolved positions on every store generation change. If a target vanished, it exits.
- Progress and stage changes bump `log_generation` and prepare a `HistoryPatch`. The patch rides the unsolicited UI-progress `AppFrame::Invocation.history_patch` through the new `TypedOperationUiProgress.history_patch` (W2-B item 3). Progress is throttled to at least 100 ms or at least 5 % of the total.

**Render seam:** `render_snapshot_or(tool_runs, committed)` in the four overlay seams (`render`, `window_engagements`, `window_measures`, `tool_measures`).
- Editing and replaying show the draft preview.
- Reviewing, choosing and finalizing show the replayed head.
- Otherwise the tool-run overlay or the committed snapshot is shown.
- `pending_effects`, context menus and command caches stay committed.

**Freeze (`timeTravel.frozen`):**
- artifact **or child** emits, in both `dispatch_emit_inner` and the retained ladder;
- `transaction_prepare`;
- history-lane verbs and `cut`/`paste`, both at admission and in `validate_framework_reserved_commit`.
- Mutual exclusion: `historyEditBegin` is refused as `timeTravel.busy` while a mutating tool run (`ToolRunLedger::holds_mutating_run`) or an agent transaction is live. A mutating `toolRunStart` is `Busy` while a session is open.

**History model:**
- `CommandView.transaction` and `CommandView.mutations: Vec<MutationView>` are built from `store.mutation_ops()` and `mutation_outcomes()`:
  - at most 32 rows per edit, plus any flagged ones;
  - each row is labelled from its effective input;
  - labels are reused across rebuilds;
  - the incremental cache arm refreshes the tail edit's rows.
- `editable` = the op has an input schema, plans no foreign steps, and this is not a viewer.
- A transaction edit without a description is labelled from its mutations, e.g. `Set count to 2 (+1)`.
- `history_patch` overlays the session on each row (replay outcome, pending, edited) and carries `timeTravel`.
- New `ArtifactApp::mutation_label(op) -> Option<LocalizedLabel>`, also on `ArtifactEditor` and forwarded by `EditorApp`.
  - A derived aggregate answers `Some(SemanticMutation::label(op))`.
  - `None` shows the op text as data.
  - W2-D / puzzle 2d should implement it.

**`ui_history_panel(history, time_travel, controller, locale, read_only, view)`**, one producer for both hosts:
- `is_de` is replaced by `Locale`, with exhaustive EN/DE copy.
- The band section `framework.history.timeTravel` shows:
  - a polite status line built from `review()` or the stage, plus the target and the worst severity in words;
  - a progress bar with Cancel;
  - the fault, labelled through `TimeTravelLabel::for_fault`;
  - Next problem;
  - Replay again, enabled only when `rerun_refusal()` is None;
  - Finalize, disabled with its reason when refused;
  - Exit.
- The editor section `framework.history.editor` shows:
  - the draft heading with its outcome;
  - windowed input rows whose controls derive from the descriptors:
    - Slider with snaps, Dial, Stepper, Number with precision, Toggle, Select/Segmented;
    - Vector with min/max/step/precision/unit;
    - Colour, through `time_travel_color_control`, a one-line switch point for W1-E's upcoming `color_input`;
    - reference list with chips and "use selection";
    - a text fallback;
  - Accept, Discard and Withdraw.
- `snapSource` `Config{key}` (focused window config, then app config) and `Snapshot{pointer}` (the previewed document) are resolved to concrete detents, or to the step when more than 32 would be needed, before building.
- History-lane actions and per-row revert are frozen while a session is open.
- Rows have per-mutation children (flagged first, at most 8) with tone, icon and words for severity, plus an Edit row action and activation (`historyEditBegin{mutationId}`).
- Undo is enabled for every editor outside a session, matching its chord (W2-B item 2). The host must route the body's `undo` through its remote-undo route.

**Finalize dialog** is injected into every non-viewer `AppDefinition.dialogs` in `try_build_definition`.

**Stores:** `enable_convergence_early_exit()` at store construction and after every store-replacement publish.

**W2-E:** `replay_envelopes_fault` maps a `codec.replay-envelopes` `VcsError::Rejected` to Fault `ledger-not-replayable`, naming only the blocking messages.

## 2. Tests (all run in the foreground and gated; exact results)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-plugin` | Finished (latest at 06:3x), 0 warnings in `⏪️time-travel` |
| `cargo check -p semio-framework` | Finished |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w2a cargo test -p semio-framework --lib -- history_patch_tests history_edit_actions_tests dialog_choices_tests tool_run_actions_tests` | **16 passed** |
| `bun ./📜️script.ts test` (kernel TS package) | **77 passed** (6 files, including `🧪️history-patch`) |
| `bun test …/🛂️manifest/🧪️tests/🧪️history-edit-actions/🟦️.ts` | **3 pass** |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w2a cargo test -p semio-framework-plugin --lib -- time_travel_tests ui_history_panel plugin_builder_contract_tests::history_ activated_tool_factory shared_framework_actions one_framework_reserved_route_fits app_builder_tests panel_kit_tests set_history_command_filter rendering_the_history_body undo_on_empty_history` | **118 passed, 0 failed**. This includes the 11 `time_travel_tests`, the stack-headroom law, the updated bijection law and the new builder law. |
| TS oracle `timeTravelScenarioOracle` (`P/🧪️tests/🧪️time-travel/🟦️.ts`, registered in the plugin `📜️script.ts test`) | **cases=8**. Negative control: flipping one fixture `blocking` is caught. |
| `tsc` (scratch strict config over the kernel twin, the kernel/manifest tests and the plugin oracle) | exit 0 |
| `cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle` | Finished; re-checked after every follow-up (the plugin crate was rebuilt) |
| `cargo check -p semio-framework-os-renderer-wgpu` | Finished; re-checked after the kernel `review`/`rerunnable` addition |
| `bun ./📜️script.ts verify taxonomy report --scope <dir>` for all 8 new dirs | clean=true, errors=0, warnings=0 |

The 11 `time_travel_tests`:
- `every_fixture_scenario_reaches_its_status_body_and_rows`. It runs 8 scenarios: preview, accept/replay, fatal blocks, withdraw resolves, overwrite, new alternative (branch plus scoped supersede), exit leaves zero trace, discard exits.
- `an_overwrite_head_equals_a_fresh_fold_of_the_edited_log`
- `finalize_opens_the_injected_dialog_with_a_localized_default_name`
- `an_open_session_freezes_document_verbs_and_refuses_stale_or_invalid_drafts`. It includes the German band and editor render.
- `cancel_keeps_drafts_and_a_remote_edit_replays_again`. It covers the `needsReplay` review, the rerun button and verb, and a `BaseMoved` re-replay from a real `MemoryBackbone` ingest.
- `a_committed_tool_transaction_is_one_row_with_its_reference_and_mutation_rows`
- `replay_progress_rides_the_unsolicited_ui_frame_with_the_session_status`
- `noted_shell_commands_keep_every_locale_and_undo_matches_its_chord`
- `a_blocking_ledger_replay_crosses_the_guest_boundary_as_ledger_not_replayable`
- `input_pointers_and_host_values_take_the_declared_shape`
- `snap_sources_resolve_to_numbers_and_colours_take_hex_text`

## 3. Fallout fixed

- Plugin tests: the `CommandView` literals and the 12 `ui_history_panel` calls. This was done with the input script `🧪️w2-a-command-view-literals.py` in the ticket root.
- The `declaring_dialog_appends_to_definition` law now expects the injected prompt.
- The bijection law lists the history-edit verbs and the document-transfer verbs as directly routed, with source guards. The document-transfer verbs were an older failure caused by a peer.
- `✏️s/…/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs`: `CommandView` literal. The `HistoryView` literals were unaffected, since `HistoryView` did not change.
- The toy `set-slot-children` leaf schema got an `x-semio-ui` label, because `children` is not in the glossary.

## 4. Open items and notes for the coordinator

- **Descriptor regeneration (coordinator chore):** every plugin descriptor is stale: the 12 verbs, the finalize dialog and the `noteShellCommand.label` shape. Run `bun nx run <plugin-project>:describe` for each plugin, for example `bun nx run @semio-tech/puzzle-plugin:describe`.
- **Full plugin suite:** 869 passed and 16 failed. None of the failures touch history editing:
  - `merge_ui_values` ×4: `json_number` integer versus float carriage;
  - `tool_run` ×4: the `Number(0)` versus `0.0` fixture and the settings-reconfigure laws;
  - `window_kits` ×2;
  - `a_100kib_measures_section…`;
  - `a_dropped_selection_publishes…`;
  - reactor ×2: command-ingress source text and reconcile crossings;
  - `window_config` retained-pack-load ×2.
  They come from peers' changes to value carriage, the store and window config. They were not investigated beyond that.
- **Not done, by design:**
  - `PresenceTimeTravel` (skipped per scout recommendation).
  - `ArtifactView::with_time_travel` (apps do not style "editing history").
  - Reload backfill of a finalized supersede row (H8). The commit row is logged live only.
- **Colour:** W1-E's `color_input` recipe had not landed. Switch `time_travel_color_control` to it; it is a one-line change.
- **W2-D:** implement `ArtifactEditor::mutation_label` for puzzle 2d so rows and the editor read localized kind labels.

## 5. Files

- **Created:**
  - `P/⏪️time-travel/🦀️.rs`
  - `P/🧪️tests/🧪️time-travel/{🦀️.rs,🟦️.ts}`
  - `P/🧫️fixtures/🧫️time-travel/{🔣️.json,🧬️schema/🔣️.json}`
  - `K/🧬️schema/🔣️history-patch/🔣️.json`
  - `K/🧫️fixtures/🧫️history-patch/🔣️.json`
  - `K/🧪️tests/🧪️history-patch/{🦀️.rs,🟦️.ts}`
  - `M/🧫️fixtures/🧫️history-edit-actions/{🔣️.json,🧬️schema/🔣️.json}`
  - `M/🧪️tests/🧪️history-edit-actions/{🦀️.rs,🟦️.ts}`
  - ticket input `🧪️w2-a-command-view-literals.py`
- **Modified:**
  - `P/🦀️.rs`
  - `P/⏯️tool-run/🦀️.rs`
  - `P/📦️packages/🦀️rust/{Cargo.toml,📜️script.ts}`
  - `P/🧪️tests/{🔬️plugin-runtime-plugin-builder-contract,🔬️app-panel-kit,🔬️app-app-builder}/🦀️.rs`
  - `P/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children/🧬️schema/🔣️.json`
  - `K/{🦀️.rs,🟦️.ts}`
  - `K/🧪️tests/🎚️config/🟦️.ts`
  - `M/{🦀️.rs,🟦️.ts}`
  - `🧰️framework/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`
  - `✏️s/🧑‍💻dev/🧩️composition-laws/🧪️tests/🧊️generation3d-app-laws/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs`
- **Scratch:** `🗑️generated/w2-a/` (logs, tsconfig). It is safe to delete at close.

## 6. Follow-up (coordinator requests after the first report)

Status: **implemented and verified** (section 6.8 lists every run). Nothing was committed. No ticket or goal was opened or
closed. Extra alias: `R` = `🧰️framework/🔨️modules/📡️replication`.

### 6.1 History edits are history rows (item 1)

- **Source of truth.** Every row comes from the store's `Supersede` transitions (`envelope.transitions`). No row comes
  from a runtime-only log. The live `record_command("historyEditCommit", …)` in `commit_time_travel` is gone. A local
  commit, an ingested remote commit and a reloaded document (text or pack) therefore produce the same row.
- **`SupersedeLedger`** (`P/⏪️time-travel/🦀️.rs`, region `🔖️Supersessions`; field `VcsArtifactApp::supersedes`):
  - It decodes the transitions once per change (keyed by count and last id) and sorts them in fold order `(hlc, id)`.
  - It classifies each one per author as a history edit, the undo of one, or the redo of one (`SupersedeRole`). The
    rule uses per-target ownership: an edit owns the inputs it installed, a redo gives them back to its edit, and an
    undo gives each input back to whoever owned it before the undone edit (nobody, for an original input).
  - The same inputs give the same rows on every replica and after reload.
- **Backfill.** `backfill_command_log` adds one row per transition that is not yet logged, merged in time order with the
  edits that are missing. `CommandLogEntry`/`CommandView` gained `transition_id`, and `CommandView` gained `author`.
- **Labels (en/de, recomputed on every projection):**
  - "History edited — overwrite: N mutations" / "Verlauf bearbeitet — überschrieben: N Mutationen"
  - "… — alternative <name>: …" / "… — Alternative <name>: …"
  - The same forms with "History edit undone / redone" and "Verlaufsbearbeitung rückgängig / wiederhergestellt".
- **Row flags:**
  - `applied`: an edit or redo row while its edit owns an input the store folds. An undo row while the edit it undid is
    still undone.
  - `revertible`: the row is by this replica's author, is not an undo row, and its edit is in effect.
  - Rows that are not applied are dimmed. The panel description shows the author.
- **Wire (additive, kernel `K`).** `HistoryEntry` gained `transitionId` and `author`. `HistoryEntry::key()` and the TS
  `historyEntryKey` fold rows under `edit:<id>`, else `transition:<id>`, else `seq:<n>`. The schema and the fixture have
  a new case, "history-edit-rows-fold-under-their-transition".
- **Mutation rows** carry `superseded`/`withdrawn` from the store's effective supersession. An input restored to its
  original (by the undo of a history edit) reads as not superseded.

### 6.2 Undo and redo of a finalize (item 2)

Design §2: undoing a finalize authors a new `ArtifactCommand::Supersede` through the store's one supersede path.

- **Undo** (in the history edit's own scope): every input the edit still owns goes back to the effective input before
  it, or to its original input when there was none.
- **Redo:** re-authors the edit's own inputs over the state its undo left, while that state still stands.
- **Plain Undo** (`dispatch_supersede_history_action`, first in `commit_framework_history_route`): it takes the newest
  own target in the command log. When that target is a history-edit row, it undoes it. A newer own document, config,
  child or shell target keeps Undo on its own lane.
- **Plain Redo:** re-authors the newest undone history edit unless one of these holds:
  - an own document edit is newer than the undo;
  - a pending document redo was undone later;
  - a child redo tail exists.
- **Backwards** on a history-edit row (`revertToCommand{entrySeq}`, `revert_history_edit_row`) undoes that history edit
  and leaves document edits alone. Another author's row is never revertible.
- `can_undo` / `can_redo` include history edits.

### 6.3 Presence (item 3)

**Field name: `PresencePeer.history_edit`, JSON `historyEdit`, binary flag bit 13.** TS: `ArtifactPresencePeer.historyEdit`.

- **Payload.** `PresenceHistoryEdit { mutation_id, stage, drafts }`:
  - `mutation_id` is the replica-independent id of the mutation being edited, else the newest accepted draft's. Hosts
    label it from their own history rows, in their own locale, so no text in one locale travels.
  - `stage` is editing, replaying, reviewing, choosing or finalizing (the `HistoryTimeTravelStage` spelling).
  - `drafts` is the number of accepted drafts.
  - Binary layout: `mutation_id str | stage u8 | drafts varint`. JSON: `{mutationId, stage, drafts}`. The standalone
    functions are `encode/decode_presence_history_edit` (Rust) and `encode/decodePresenceHistoryEdit` (TS).
- **Path:**
  1. The guest publishes `TimeTravelLedger::presence()` through `EphemeralSnapshot.history_edit`.
  2. It travels as `AppFrame::Ephemeral.history_edit`, a new trailing bytes field (Rust channel codec and TS twin
     `💻️os/🟦️.ts`; golden hex updated). This follows the `tool_run` precedent, which had no channel-version bump.
  3. The native host decodes it into `ProgramEphemeralSnapshot.history_edit`, and the wgpu shell heartbeat stamps it on
     the local `PresencePeer`.
  4. The hub keeps it through admitted-presence normalization (`history_edit: input.history_edit`).
- **React host (for W2-B).** It does not stamp it yet: `BrowserActorEphemeralSnapshotV1` carries neither `toolRun` nor
  `historyEdit`. The hook is `stampSession` in `🏪️store/👷️worker/🟦️.ts`.
- **Every `PresencePeer` literal in the repository was updated in one compile-atomic pass**, using ticket input
  `🧪️w2-a-presence-history-edit.py`, then fixed by hand:
  - the MCP literal, which the script skipped;
  - the hub's two literals in `🌎️hub/🏗️bootstrap/🦀️.rs`, outside the first scan. You reported that build break; it is
    fixed and `cargo check -p semio-hub` passes.
- **Codec contract changes:**
  - The unknown-flag law moved to bit 14, in both codecs, the neutral fixture and the Rust test.
  - Four fixture vectors were added: one accepted, plus unknown stage, drafts above u32 and truncated.
  - The replication schema declares `historyEdit`.
  - The hub's independent presence oracle (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`) encodes bit 13 and checks that the
    hub keeps the field.

### 6.4 The plugin-suite failures (item 4)

The full plugin suite now reports **904 passed, 13 failed**. No failure is caused by this fleet, so none was changed.

- **Baseline evidence.** Peer report `☀️23/END-TO-END-OS-HUB-COLLABORATION-MCP/📓️wp-c13.md` classifies these reds as
  baseline on the pre-P4 SDK on 2026-09-29, about 19:30. That is before this ticket's `📋️design.md` was born
  (09-30 02:49). HEAD `3eeee4f9119` (09-30 00:11) holds no file of this ticket.

| Failure(s) | Classification and evidence |
|---|---|
| `merge_ui_values_tests` ×4 | Baseline (wp-c13). `UiCommandJsonProducer` now emits `DslValue::json_number` (UInt/Int for integral floats). This is the WG11 `wg11-json-number-patch.py` of ticket 26/09/23 END-TO-END…, in commit `3eeee4f9119`. The tests (last touched 09-15) still expect `Float`. |
| `tool_run_panel_of_a_running_run_is_the_shell_fixture` | Baseline (wp-c13). The same numeric rule, in `UiValue`'s `Serialize` (`json_integer`, `3eeee4f9119`): `generation: 0` against the fixture's `0.0` (fixture last changed 09-19). |
| `tool_run_reconfigure_resume…`, `tool_run_settings_changed…`, `tool_run_window_settings_reads…` | Baseline (wp-c13). Mechanism measured with temporary `[DEBUG]` lines (since removed): a 256-op tick displaces 255 intermediate overlays (`apply_tick`). Each alias retirement takes about 5 driver turns, and `drive_tool_run_turn` returns early while any retirement is pending, so the settings watch never runs within the tests' 8-turn budget. The same pending retirement keeps `has_pending_work()` true for the complete run. The code dates from `5bcb2da23da` (09-27). This fleet changed only `holds_mutating_run` and the busy refusal there. |
| `tool_run_overlay_append_per_tick…` (below 2 ms) | Timing law. It failed in the parallel full run and passes alone (`test-flaky-rerun.log`). This is peer load. |
| `window_kits_tests` ×2 | Baseline (wp-c13): required arguments, and the command id `textEdit` against `replace-text`. |
| `command_ingress_terminal_tests::every_admitted_command_page…` | Baseline (wp-c13): a source-text law about the admission chain's terminal else. |
| `more_work_drive_tests::the_production_reconcile_ladder…` | Red only in the full parallel run (wp-c13: "560 ≠ 569"). It passes alone. |
| Earlier reds that are gone | `a_100kib_…`, `a_dropped_selection_…` (wp-c13 lists them as full-run flakes) and `window_config retained_pack_load` ×2 all pass in this run. |

- **Unrelated os TS failure.** The os TS suite has 549 of 550 passing. The one failure is
  `🏷️schema-vocabulary` (`x-semio-fixture`), caused by `🌎️hub/🧫️fixtures/🐳️docker-image-v1/🔣️.json`, committed 09-27.

### 6.5 Input addressing, unions, snaps and nullable (audit `📓️audit-inputs-leaves.md` §2.3, §2.8, §2.2)

- **Full JSON-pointer addressing.** `time_travel_input_at(inputs, value, pointer)` follows object members and array
  items to any depth; an array item takes the array's item schema. `/handles/1/angle` resolves.
- **Unions resolve against the active variant:**
  - `time_travel_variant_selector` finds the selector: the ungrouped string input whose options name every group.
  - `time_travel_active_inputs` resolves against the selector's current value. glTF's `/value` declared twice now
    resolves to the variant the op is in.
  - Layout `group`s of a payload that is no union hide nothing.
  - Changing the selector switches the variant (`time_travel_switch_variant`): members only other variants declare are
    dropped, and declared defaults fill the new variant.
- **Editor rows.** The editor renders `time_travel_input_rows`: the selector plus the active variant's fields. Objects
  are flattened into their fields, and arrays of objects into one row per field of each item. Hidden inputs, such as
  glTF's restore diff, never get a row. Each row carries its pointer and a path label, for example
  "Handles 2 · Angle" / "Griffe 2 · Winkel".
- **snapSource (confirmed and tested):**
  - `time_travel_resolve_snaps` is the pure resolver; `resolve_time_travel_snaps` feeds it the window config, then the
    app config, and the previewed document for `Snapshot` pointers.
  - A puzzle-shaped schema read by the real reader (`newX`, `snapSource: {config: "gridFactor"}`, grid 0.5) steps by
    0.5, and a bounded input snaps to its 21 multiples.
  - A `Snapshot` pointer reads the open session's preview (`count` 5 gives detents 0…20).
- **Nullable (W1-D's `ActionArgDef.nullable`):**
  - Each nullable row has an accessible Clear button, "Clear <label>" / "<label> leeren" (icon `eraser`), that drafts
    `null` at the row's pointer.
  - A cleared row reads "Cleared (no value)" / "Geleert (kein Wert)".
  - Array items never inherit their array's nullability.
- **Colour inputs (W1-E).**
  - They render with the UI contract's `color_input` recipe: swatch, hex text, and an opacity slider bound to
    `pointer/3`.
  - The colour law comes from the contract's `parse_ui_color_hex` / `ui_color_hex`. A hex text without alpha keeps the
    stored opacity. The local helper `time_travel_color_text` was removed.
  - Vector axes get the vector's snaps.
- **Fail closed.** A payload schema that compiles no validator, or that describes no inputs, refuses every draft with
  the typed `timeTravel.schema-unavailable` and names the reason on the editor. `.ok()` is gone. The fixture's refusal
  table has the new code.
- **`Restore`-phase ops** are non-editable automatically, because editability is `input_schema().is_some()` and no
  foreign steps.
- **Labels of superseded ops (W1-G's `AppliedMutation.effective`).** The label and editability still come from the
  decoded effective input: its op carries the values the label shows. The descriptor's `display_name` is
  single-locale, so it is not used as a label.

### 6.6 Tests added in this follow-up

- **`P/🧪️tests/🧪️time-travel/🦀️.rs`** (20 tests in total now), new laws:
  - `a_history_edit_is_its_own_row_locally_remotely_and_after_reload` (two replicas, text and pack reload);
  - `undo_and_redo_of_a_finalize_author_restoring_supersedes_on_both_replicas` (two replicas: undo, redo, ordering
    against a newer document edit, Backwards, another author refused);
  - `an_alternative_history_edit_is_undone_within_its_alternative`;
  - `union_inputs_resolve_against_the_active_variant`;
  - `snap_sources_resolve_from_the_grid_factor_and_the_previewed_document`;
  - `colour_inputs_render_the_contract_recipe_and_vector_axes_keep_their_snaps`;
  - `a_nullable_input_offers_a_clear_control_that_drafts_null`;
  - `a_schema_without_a_validator_refuses_every_draft`;
  - the extended `input_pointers_and_host_values_take_the_declared_shape`.
- **Language-agnostic supersede-ledger law.**
  - Fixture: `P/🧫️fixtures/🧫️supersede-ledger/{🔣️.json,🧬️schema/🔣️.json}`, with 7 cases.
  - Rust `P/🧪️tests/🧪️supersede-ledger/🦀️.rs` runs the fixture against the ledger.
  - TS twin `…/🟦️.ts` validates the fixture with Ajv 2020 and classifies it with its own implementation. It is
    registered in `P/📦️packages/🦀️rust/📜️script.ts`.
- **Replication:** `presence_peer_history_edit_round_trips_every_stage_with_wire_spelling`; "every field present" now
  sets `history_edit`.
- **Channel:** a third `Ephemeral` round trip; golden hex updated in Rust and TS.

### 6.7 Notes

- **Projection gap.** The fixture projection (`project_and_retire_fixture_tree`) drops a button's `disabled` flag, so
  assertions such as `rerun["disabled"] != true` are vacuous. The Clear law therefore asserts the cleared description
  instead.
- **Presence tool run.** The `tool_run` presence precedent is itself not stamped by the wgpu heartbeat (`tool_run: None`
  there). `history_edit` is stamped.
- **`semio-s-composition-laws`.** Its test target `generation3d-app-laws` does not compile, with 105 errors, all
  unrelated to this work: unresolved `semio_framework_tool_run` / `semio_s_artifact_procedural_generation3d`, private
  fields, missing `evaluate_host_snapshot`. No error names `CommandView` or its new fields. The `CommandView` literal
  in its `🧵️preview-eval` unit test carries the two new fields.

### 6.8 Verification (all gated and in the foreground; `cargo test` with `CARGO_INCREMENTAL=0`, `target-nde-w2a`)

| Run | Result |
|---|---|
| `cargo check -p semio-framework-plugin -p semio-framework-os-renderer-wgpu` | Finished, no warning in the files I own |
| `cargo check --tests -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-plugin -p semio-framework-os-renderer-wgpu -p semio-hub` | Finished (`check-followup-tests.log`) |
| `cargo check -p semio-hub` | Finished |
| `cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle` | Finished |
| `cargo check --tests -p semio-s-composition-laws` | 105 unrelated errors (section 6.7), none in the `CommandView` literal |
| `cargo test -p semio-framework-plugin --lib -- time_travel_tests supersede_ledger_tests` | **20 passed** |
| `cargo test -p semio-framework-plugin --lib` (full) | 904 passed, 13 failed (section 6.4) |
| `cargo test -p semio-framework-replication --lib -- presence` | **20 passed** |
| `cargo test -p semio-framework-os-kernel --lib -- os_spr::channel::tests store_sync presence` | **113 passed** |
| `cargo test -p semio-framework-os-kernel --features sync --lib -- sync::` | **83 passed** |
| `cargo test -p semio-framework --lib -- history_patch history_edit` | **8 passed** |
| `bun ./📜️script.ts presence-peer-codec-check --oracle-only` (R) | 36 vectors, 28 hostile rejected |
| `bun ./📜️script.ts presence-normalization-check source` (hub) | 19 vectors |
| Kernel TS `bun ./📜️script.ts test` | **77 passed** |
| os TS `bun ./📜️script.ts test` | 549 of 550 (the unrelated `x-semio-fixture`) |
| TS oracles (supersede ledger, time-travel scenarios) | 7 and 8 cases |
| `tsc` (scratch config over every TS file this follow-up touched) | exit 0 |
| Taxonomy of the two new directories | clean |

### 6.9 Files (follow-up)

- **Created:**
  - `P/🧫️fixtures/🧫️supersede-ledger/{🔣️.json,🧬️schema/🔣️.json}`
  - `P/🧪️tests/🧪️supersede-ledger/{🦀️.rs,🟦️.ts}`
  - ticket input `🧪️w2-a-presence-history-edit.py`
- **Modified, this ownership:**
  - `P/🦀️.rs`, `P/⏪️time-travel/🦀️.rs`, `P/🧪️tests/🧪️time-travel/🦀️.rs`
  - `P/🧫️fixtures/🧫️time-travel/{🔣️.json,🧬️schema/🔣️.json}`, `P/📦️packages/🦀️rust/📜️script.ts`
  - `P/🧪️tests/{🔬️plugin-runtime-plugin-builder-contract,🔬️app-panel-kit}/🦀️.rs`
  - `P/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs`
  - `K/{🦀️.rs,🟦️.ts}`, `K/🧬️schema/🔣️history-patch/🔣️.json`, `K/🧫️fixtures/🧫️history-patch/🔣️.json`
- **Modified, the presence wire across the repository** (all additive):
  - `R/📡️wire/🦀️.rs` and `R/📡️wire/🧪️tests/🔬️presence-codec/🦀️.rs`
  - `R/🟦️.ts`, `R/🧬️schema/🔣️.json` and `R/🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json`
  - `R/👕️peer-overlay/🧪️tests/🔬️unit/🦀️.rs`
  - `💻️os/🔨️modules/📡️spr/🧵️channel/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
  - `💻️os/🟦️.ts` and `💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`
  - `💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/{🔬️unit,🔬️backbone-parity}/🦀️.rs`
  - `💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
  - `💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/{🌉️ProgramBridge/🎯️targets/🧊️wgpu,🐚️Shell/🎯️targets/🧊️wgpu,🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence,👕️canvas-presence/🧪️tests/🔬️wgpu-unit}/🦀️.rs`
  - `🌎️hub/🏗️bootstrap/🦀️.rs` and `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- **Modified, fallout:** `✏️s/🧑‍💻dev/…/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs`, which has the `CommandView` literal.
- **Scratch:** `🗑️generated/w2-a/` (logs, tsconfigs) and `🗑️generated/w2a-supersede-oracle.ts`. Both are safe to
  delete at close.

## 7. Follow-up 2

Status: (5), (1), (c), (d), (a), (2) and the generation fix are landed and tested. (3) host events and (4) the
Alternatives section are **not started**.

### 7.1 (5) One tree vocabulary for the band and the draft editor

- **Before.** The band and the editor were `section()` containers. React's `TreeView` keeps only `treeSection`
  children, so it dropped both.
- **Band.** `time_travel_band_section` returns the `tree_section` `framework.history.timeTravel`. It has one
  `tree_item` per row:
  - the stage row `framework.history.timeTravel.status`, whose label carries the stage, edited mutation and worst
    severity;
  - the replay progress row `….progress`, whose label is the localized "done of total";
  - the fault row `….fault`;
  - one button row each for cancelReplay, nextProblem, finalize, rerun and exit (`{id}.row` holds button `{id}`;
    Finalize is disabled and carries its refusal reason as the row description).
- **Editor.** `time_travel_editor_sections` builds two tree sections:
  - `framework.history.editor`: the target row, the refused row, and the accept, discard and withdraw button rows;
  - the windowed `framework.history.editor.inputs`: one row per input.
  A nullable input nests a Clear row (`{id}.clear.row` holding button `{id}.clear`). The input row is `default_open`.
- **wgpu finding (why rows carry text in labels).** wgpu's `tree_item` projection (`🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile`,
  `tree_item_control`) mounts only one of nine single controls per row. Every other non-row child becomes a nested
  placeholder row labelled with its record key: text, progress, groups, and a second control. Both hosts therefore
  get rows with at most one single control. The polite live region and the progress bar now live only in each
  shell's own band, which React and wgpu both already announce.
- **Open wgpu gap (not mine to fix).** The composite input recipes (`color_input`, `vector_input`, `reference_list`)
  are container groups. They render in React but become placeholder rows in wgpu. The fix belongs in the wgpu tree
  row: mount a group child like React does, or add a group `UiControlNode`.
- **Law.** `the_band_and_the_editor_rows_hold_at_most_one_single_control` walks both sections while editing and
  while reviewing. It asserts that each is a `treeSection` and that no row has more than one non-row child, of the
  nine wgpu kinds.

### 7.2 (1) Window-transient drop

- The fix and its law (`a_refused_window_transient_emission_keeps_its_mutation_and_faults`) landed earlier: a refused
  `begin` hands the mutation back. The puzzle 2d `select_tool` is boxed so it fits the ephemeral inline bound.
- The puzzle 2d rerun (`cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib`) gives
  **1061 passed, 5 failed**. The window-transient and ui_scope laws now pass.
  - `leaving_the_select_utility_retires_an_open_gesture` needs (3), the `UtilityChanged` host event.
  - 4 brush/fill engine laws fail inside the board host (`♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal`: "fill
    placement must reach exact terminal-empty before Drop"). Commit `48d881aa7ab` (09-30 12:00) changed that host. I
    did not bisect it. It is not in my ownership, and the puzzle engine directories are unchanged since 09-23.

### 7.3 (c) `[DEBUG]` removal

- There is no `[DEBUG]` left in P or K (checked with grep).

### 7.4 (d) Interaction and pure view verbs are no longer history rows

- `history_row_is_recorded(kind, edited, inverse)` is the one predicate. `record_command` applies it:
  - an interaction is never a row;
  - a `View` is a row only with a store edit (document or config) or an inverse;
  - everything else always is.
  So hover, pick and clear selection, and an op-less view verb, leave no row.
- **Laws:**
  - `interaction_verbs_never_become_history_rows`: pick, hover and clear selection after a seed edit leave only the
    seed row;
  - `a_pure_view_verb_is_no_history_row`;
  - the table law `history_rows_exclude_interactions_and_pure_views`;
  - the three fold laws now assert that no interaction row exists.

### 7.5 (a) No raw op-lines under labelled mutations

- `ui_history_panel` shows the `op_lines` description only when the row has no mutation children. The wire keeps
  `op_lines` as data.
- **Law:** an extension of `a_committed_tool_transaction_is_one_row_with_its_reference_and_mutation_rows`. The rendered
  row, without its children, carries none of its op-lines.

### 7.6 (2) ui_scope widening

- **Root cause.** `take_operation_progress_scope` returned `Full` on every progress change, so the framework
  widened every app to `Full`.
- **Fix.**
  - `ArtifactApp`, `ArtifactEditor` and `ArtifactViewer` gain `operation_progress_scope() -> UiDirtyScope`, which
    defaults to `None`. The raster editor declares its layers panel.
  - `operation_progress_dirty_scope(changed, declared)` is the rule: the declared scope, never wider, and nothing
    for `None`.
  - Cancellation also returns the declared scope instead of `Full`.
- **Law:** `a_progress_change_dirties_exactly_the_declared_scope`. The puzzle 2d ui_scope laws pass.

### 7.7 The new generation ships in the frame where the stage flips

- **Root cause:** three compounding faults.
  1. A retained-surface intent (the history body's buttons in React) replies with an `Emit` frame, which has no
     history patch.
  2. `dispatch_time_travel_action` dropped the owed patch.
  3. `drive_time_travel_turn` prepared a patch only after snapshot retirement, and a progress frame shipped a
     prepared patch only alongside some UI scope. Begin from Reviewing retires the review's snapshots, so the new
     generation waited for an unrelated later scope, which was the ~800 ms. Meanwhile the React band still stamped
     the old generation, and its verbs were `timeTravel.stale`.
- **Fix (Rust).**
  - `prepare_time_travel_patch` runs in the dispatch and first in every driver turn. A patch that has not shipped is
    superseded by one that still carries its rows.
  - `take_typed_operation_ui_progress` ships a pending patch even with no scope queued (scope `None`).
  - `has_pending_work` counts an owed patch, so the continuation of the same reactor turn ships it.
- **Fix (React, one line).** `onBrowserActorIntent` in `🏛️ShellHost/🟦️.tsx` now applies `result.historyPatches`,
  exactly as `dispatchDirectBrowserActorCommand` does. Before this, it ignored them.
- **Law:** `a_stage_change_ships_its_generation_in_the_same_turn_and_a_verb_stamped_with_it_applies`.
  - Begin from Reviewing bumps the generation.
  - The reply and the first progress frame, taken before any driver turn and while snapshots still retire, both
    carry `(editing, new generation)` with rows.
  - A discard stamped with the old generation is stale; one stamped with the new generation applies.
  - The return to review ships at once.

### 7.8 Not started

- **(3)** Typed host-event delivery (`ArtifactApp::host_event`, `HostEvent`, dispatch on time-travel begin, base
  move and utility switch, blur and capture-loss forwarding from both hosts, the puzzle select tool). The failing
  puzzle law above waits for it.
- **(4)** The Alternatives section with a Switch row action and the `alternativeId` manifest arg. Note that
  `switchAlternative` already exists as a history-lane verb in M.

### 7.9 Verification (gated, foreground; `cargo test` with `CARGO_INCREMENTAL=0`, `target-nde-w2a`)

| Run | Result |
|---|---|
| `cargo check -p semio-framework-plugin -p semio-framework-os-renderer-wgpu` | Finished |
| `cargo test -p semio-framework-plugin --lib -- time_travel supersede` | **23 passed** |
| `cargo test -p semio-framework-plugin --lib` (full) | 912 passed, 14 failed. The same baseline set as section 6.4; `a_dropped_selection…`, `tool_run_scene_render…` and `…reconcile_ladder…` pass when run alone |
| the new (d), (a), (2) and generation laws, run alone | pass |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- time_travel` | **17 passed** |
| `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib` | 1061 passed, 5 failed (section 7.2) |
| `bunx tsc --noEmit -p tsconfig.json` (React renderer package, includes ShellHost) | 0 errors |

### 7.10 Files (Follow-up 2)

- `P/🦀️.rs`: history row predicate, op-lines description, `take_typed_operation_ui_progress`, an unused
  `ActionArgOption` import removed.
- `P/⏪️time-travel/🦀️.rs`: band and editor rows, patch preparation, `has_patch`.
- `P/⏳️operation-progress/{🦀️.rs,🧪️tests/🦀️.rs}`.
- `P/🧪️tests/{🧪️time-travel,🔬️plugin-runtime-plugin-builder-contract}/🦀️.rs`.
- `💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`: one line plus a dependency.
- The puzzle 2d window and select-tool tests (the earlier `Box` change).
