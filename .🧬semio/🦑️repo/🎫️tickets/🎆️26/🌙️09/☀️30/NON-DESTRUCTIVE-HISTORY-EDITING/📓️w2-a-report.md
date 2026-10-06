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

## Session 2 — 2026-10-01

Successor S2-W2A (coordinator `⚪552b484a…`). Nothing committed, no ticket or goal touched, no describe/activate/serve run.
Status: **source complete for (a)–(c) and G3/G4/i18n; targeted laws green except the items in 8.3**. This section is
updated at every milestone (last update at the bottom).

Aliases as above (`P`, `K`, `M`, `R`), plus `FWT` = `🧰️framework/🔨️modules/⏪️time-travel`.

### 8.1 Repair (rule 21)

- Nothing was half-written. HEAD `4e36b2b5012` (11:16) already held every file of the predecessor; the 07:39–07:44
  `Edit.verb` scripts (`🧪️w2-a-edit-verb-*.py`) had all applied (Rust + TS + schema + fixtures + `.spr`/`.ops`/db
  cursor + store stamping + projection label rule). Report §7.8 is stale: (3) host events and (4) the alternatives
  section were landed by the predecessor at 22:43 (see `📓️resume-core.md` §4.6).
- Peer fallout repaired in my crate so the test target compiles again:
  - `P/🧪️tests/{🛰️declaration-channels,🛰️declaration-channels-unit,🌐️standard-one-any-set-value-unit,🔒️standard-one-strict-set-value-unit,🌐️standard-two-any-set-value-unit}/🦀️.rs`:
    `#[path]`/`include_str!` re-pointed from `1️⃣standard-1`/`2️⃣standard-2` to the dirs REPO-PATH-BUDGET renamed at
    12:16 (`1standard`, `2standard`; 8 references).
  - `P/🧪️tests/🧪️time-travel/🦀️.rs` (S2-W1G's streamed-transaction law, line 476): E0502 borrow fixed by hoisting the
    mutation id (semantics unchanged).

### 8.2 Changes

**(a) Locale-neutral edit verb — verified, no new code needed.** `Edit.verb` / `MutationEnvelope.verb` (trailing flags
bit 1) / `HistoryEdit.verb` (`.spr` field 6, REC_EDIT presence bit 7, `.ops` header) / store `set_authoring_verb` stamped
at every runtime publish / `backfilled_edit_label` + `authored_row_label` (verb registry label → description → leaf label).
Added one proof: the `.spr` identity sample now carries a verb (`OS/📡️spr/📜️history/🧪️tests/🔬️unit/🦀️.rs`, edit-1
`verb: Some("typeText")`), so every `.spr` encode/decode identity law round-trips it.

**i18n of every `timeTravel.*` code (coordinator-approved strings, byte for byte), framework vocabulary `FWT`:**
- `FWT/🦀️.rs` + TS twin `FWT/🟦️.ts`: code constants `TIME_TRAVEL_{BUSY,UNKNOWN_MUTATION,NOT_EDITABLE,UNKNOWN_INPUT,
  INVALID_INPUT,NO_SELECTION,NAME_REQUIRED,NAME_INVALID,SCHEMA_UNAVAILABLE,REPLAY_FAULTED,COMMIT_FAILED}_CODE`; 12 new
  `TimeTravelLabel`s (`refusalBusy … refusalSchemaUnavailable`, `replayFaulted`, `commitFailed`, `outcomeIntroduced`);
  `TIME_TRAVEL_CODE_LABELS` (17 codes → label: the 4 session refusals, frozen, cancelled, 9 host refusals, 2 driver
  faults); `TimeTravelLabel::for_code` / TS `timeTravelCodeLabel` **replace** `for_fault` / `timeTravelFaultLabel`.
  `reportBlocking` already read "Errors must be fixed or withdrawn before finalizing" (S2-W2B, §16.1).
- Schema `FWT/🧬️schema/🔣️.json`: label enum 23 → 35, `labels` 35, new required `codeLabels` (17). Fixture
  `FWT/🧫️fixtures/🧫️lifecycle-law/🔣️.json` + generator `T/🧪️w1-b-generate-lifecycle-law.py` (`LABELS`, `CODE_LABELS`);
  regenerating to scratch reproduces the fixture byte for byte.
- Plugin: the local code constants are gone (`P/⏪️time-travel/🦀️.rs` imports them from `FWT`); the band's fault row
  reads `TimeTravelLabel::for_code` (driver faults no longer show raw codes).
- wgpu fallout (one token, S2-W2C's file): `…/🐚️Shell/🎯️targets/🧊️wgpu/⏪️time-travel/🦀️.rs` `for_fault` → `for_code`.

**G4 — `HistoryMutationEntry.introduced` (kernel wire change, see 8.5):** `K/🦀️.rs` field + doc, TS twin `K/🟦️.ts`,
schema `K/🧬️schema/🔣️history-patch/🔣️.json`, fixture `K/🧫️fixtures/🧫️history-patch/🔣️.json` (valid
`reviewing-session-marks-an-introduced-warning`, invalid `introduced-not-a-boolean`). Runtime: `history_mutation_entry`
sets it when the session's replay outcome carries a `(level, code)` the durable pre-edit outcome of the same mutation
lacks (`history_outcome_introduced`); the row description adds "New since this edit" / "Neu durch diese Bearbeitung".
Session-scoped by definition (no session → false); the warning itself persists (durable outcomes recomputed on load).
Fallout: wgpu test literal `…/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs` gets `introduced: false`.

**G3 — selection editing:**
- `time_travel_selection_value(input, selection)` (pure, `P/⏪️time-travel/🦀️.rs` 🔖️Pointer): domain absent →
  `unknown-input`; no / empty / other-granularity selection → `no-selection`; `many` clips to `maxItems`; id type
  integer stages integers, unspellable ids skipped (all skipped → `no-selection`, before it drafted `[]`).
  `draft_time_travel_selection` delegates to it.
- Entity labels on chips: new hook `ArtifactApp::entity_label(snapshot, kinds, id) -> Option<LocalizedLabel>` (+
  `ArtifactEditor::entity_label`, forwarded by `EditorApp`) in `P/🦀️.rs`; `VcsArtifactApp::resolve_time_travel_reference_labels`
  fills `TimeTravelEditorPanel.reference_labels` from the previewed document (member-store sessions keep ids); chips
  and their remove labels read them.
- Overflow: ids beyond 8 chips are named by one nested row `{input}.more.row` ("N more: a, b" / "N weitere: a, b").
- The board highlight is S2-W2D's (`InteractionView::draft_references`), not duplicated here.
- Toy fixture: `set-slot-children` `children` is now a reference input (`role: target`, `ref {kind: s.test.child,
  domain: test.slot, granularity: child}`), toy `SetLabel` emits Warning `mutation.no-op` when the label is unchanged.

**(c) Acceptance laws added (`P/🧪️tests/🧪️time-travel/🦀️.rs`, region `🧭️AcceptanceLaws`; fixture scenario
`several-drafts-accumulated-from-a-review-finalize-once` in `P/🧫️fixtures/🧫️time-travel/🔣️.json`, also run by the TS
reducer oracle):**
- `selection_values_follow_the_reference_domain_granularity_count_and_id_type`
- `editing_targets_through_the_selection_resolves_a_blocked_review` (Next problem opens the first blocking mutation from
  the band's own binding; use selection refuses then drafts; German entity-label chips; overflow row; chip remove binding
  drafts the rest; accept → ready)
- `a_warning_an_edit_introduces_stays_visible_after_finalize_and_reload` (introduced while reviewing; after finalize, text
  reload and pack reload the Warning row persists, en + de words)
- `several_drafts_from_a_review_finalize_as_one_overwrite_supersede` (exactly one unscoped `Supersede` naming both,
  head = fresh fold)
- `a_new_alternative_is_one_branch_then_one_scoped_supersede`
- Already covered by existing laws: preview (downstream not applied), accept → Report replay, Fatal blocks, withdraw,
  exit zero trace, frozen verbs, progress frames, cancel + rerun, alternatives section + switch. Error-level blocking is
  the replication `blocks_finalize` law + reducer fixture (the toy has no Error-level outcome).

### 8.3 Verification (gated, foreground, one cargo at a time; `cargo test` with `CARGO_INCREMENTAL=0`, `target-nde-s2-w2a`)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-plugin --lib --tests` (11:54, before my edits) | Finished |
| `cargo test -p semio-framework-time-travel` | **13 passed** |
| `bun test ./FWT/🧪️tests/🧪️conformance/🟦️.ts` | **19 pass** (new code-label law) |
| `cargo test -p semio-framework-replication --lib` | **316 passed, 0 failed** (envelope verb flags, backbone batch fixture) |
| kernel vitest (`K`, config `🧪️tests/🎚️config/🟦️.ts`, no budget) | **77 passed** (6 files, incl. the new history-patch cases) |
| replication vitest | 20 passed, 1 failed — `trailing-flags-invalid`: the PER-VIEWER peer made trailing flag bit 2 (`line`) legal in the TS decoder; the fixture's flag-4 refusal must move to flag 8 (theirs) |
| `cargo test -p semio-framework-os-kernel --lib -- os_spr::history canonical_edit` | 76 passed, 1 failed — `fold_falls_back_to_positional…` (`alternative` None vs `alt-1`): PER-VIEWER's per-viewer head, not the verb; `.spr` identity with a verb + 25 canonical-edit (digest vectors with verb) pass |
| `cargo test -p semio-framework --lib -- history_patch history_edit` | blocked at 12:56 by a peer's `DslValue::Bytes` E0004 in `M/🧪️tests/🧪️mutation-inputs/🦀️.rs` (rerun pending) |
| TS oracles (scratch runner): time-travel / supersede-ledger / alternatives / label-reload | **9 / 7 / 4 / 5 cases** |
| `tsc` scratch config `🗑️generated/s2-w2a/tsconfig.s2.json` (FWT, K, plugin oracles, R, worker) | 3 errors, all PER-VIEWER's in-flight `line` field (worker + backbone-parity); none in my files |
| `cargo test -p semio-framework-plugin --lib -- time_travel supersede history_label_reload history_alternatives ui_history_panel rendering_the_history_body activated_tool_factory` (14:03) | **41 passed, 3 failed**: all 5 new laws pass; failures: `history_labels_survive…` (manifest verbs unclassified for interactive jobs), `the_open_draft_references_its_reference_inputs_per_domain` (S2-W2D's new law), bijection law (`hostEvent`) — being fixed |

### 8.4 Regions owned by others inside my files

- `P/⏪️time-travel/🦀️.rs` `time_travel_input_row`: the `Slider`/`Dial`/`Stepper`/`Number`/`Vector` arms (facets snaps,
  unit, displayUnit/displayFactor/scale) → S2-W1E. I changed only the signature (`references` map) and the `Reference` arm.
- `ArtifactApp::mutation_label` default + the per-app overrides + label gate → S2-AGNOSTIC (G7); untouched.
- `tool_transaction_shape_fault` + retained initializer (§15) → S2-W1G; the streamed law in my test file is theirs.
- `InteractionView::draft_references` + its law → S2-W2D.

### 8.5 For S2-W2B / S2-W2C (wire and vocabulary)

- `HistoryMutationEntry.introduced: bool` (JSON `introduced`, default false) — Rust, TS twin, schema, fixture.
- `TimeTravelLabel::for_code` / `timeTravelCodeLabel` + `TIME_TRAVEL_CODE_LABELS`: one lookup for every `timeTravel.*`
  code (17); `for_fault`/`timeTravelFaultLabel` are deleted. Label keys: `refusalBusy`, `refusalUnknownMutation`,
  `refusalNotEditable`, `refusalUnknownInput`, `refusalInvalidInput`, `refusalNoSelection`, `refusalNameRequired`,
  `refusalNameInvalid`, `refusalSchemaUnavailable`, `replayFaulted`, `commitFailed`, `outcomeIntroduced`.

### 8.6 Coordinator actions

- Descriptor regeneration (`describe`) stays owed (history-edit verbs, finalize dialog, `hostEvent`); no new verb this session.
- Activation needed for live proof of G3/G4 (no new wire beyond `introduced`).

#### Update 16:55–17:30 (after the 13:30–16:30 usage cut)

- `P/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` bijection law: `hostEvent` joins the directly-routed set with
  a source guard on its boxed `dispatch_host_event_action` arm; the history-edit guard now tolerates the
  `freeze_tool_machines()` call a peer put into that arm (reads the arm body, not a byte-exact snippet).
- `P/⏪️time-travel/🦀️.rs` `settle_time_travel_owners`: the draft editor closes whenever the session leaves `Editing`
  (accept, discard, exit, replay). Before, a review kept rendering the last editor (its Accept/Discard refused as
  illegal) and `targetLabel` named a mutation no longer edited; S2-W2D's law
  `the_open_draft_references_its_reference_inputs_per_domain` ("an exited session references nothing") needs it.
- `P/🧪️tests/🧪️history-label-reload/🦀️.rs`: the never-run law failed twice on runtime rules it was written against
  (manifest verbs must be classified; every typed command needs an exact declaration). Fix: the manifest classifies its
  declared verbs (`BatchOnlyPendingRewrite`, data only); an undeclared verb's edit is authored through the store with that
  verb stamped (`set_authoring_verb`) — the realistic case of an edit from a peer or another app version — and logged by
  the backfill. Fixture and TS twin unchanged (twin docstring updated).
- Run `test-tt-2` (17:00, filters `time_travel supersede history_label_reload history_alternatives ui_history_panel
  rendering_the_history_body activated_tool_factory document_archive`): **48 passed, 2 failed** (the two above, fixed
  since); `document_archive` 7/7 pass incl. R2-2 `a_document_archive_round_trip_lists_every_history_row_of_its_source`.
- Rerun of the two fixed laws blocked at 17:25 by a peer's in-flight `🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`
  (E0433 `wgpu::layout` / `wgpu::stepper`), not mine.
- 17:52: `history_label_reload` law **passes** (1/1) and the bijection law **passes** (1/1). Final shape of the label
  law: every fixture command publishes through `dispatch_emit` (the seam every typed command's emit lands in, where
  the verb is stamped and the live row recorded) under its verb, declared or not; the typed admission lane itself cannot
  run here without a full Migrated factory/job harness (`BatchOnlyPendingRewrite` is refused `interactive-job.not-ui-safe`
  by UI dispatch), and its emit seam is what carries the verb.

#### Update 21:40–22:10 (after the 18:00–21:30 cut): G9 runtime adoption + notices — SOURCE WRITTEN, compiling next

- `P/🦀️.rs`: the document store defers remote replays (`defer_remote_replays(Some(TIME_TRAVEL_REMOTE_REPLAY_OPERATIONS = 256))`)
  at construction and after every store-replacement publish; `history_patch` carries `remote_replay`;
  `ui_history_panel(history, time_travel, remote_replay, …)` (every call site updated, 11 in tests) shows the remote section.
- `P/⏪️time-travel/🦀️.rs`: `step_remote_replay` per driver turn (adoption → cache reset, `BaseMoved`, full refresh; refusal
  → code on the row); `historyEditCancelReplay` / `historyEditRerun` while no session is open pause / resume it (no new
  verb, no descriptor change); `time_travel_remote_section` (`framework.history.remoteReplay`: status row + Cancel replay /
  Replay again, en/de); `author_supersede` (undo/redo of a finalize, Backwards on a history-edit row) is now
  `begin_report_replay` + per-turn steps + `commit_finished_replay(…, Overwrite | Scope{alternative_id})`, a stale base
  replays the same inputs again, `historyEditBegin` and a second undo/redo answer `timeTravel.busy` meanwhile.
- Kernel wire (for S2-W2B/S2-W2C): `HistoryPatch.remoteReplay?: {done, total, paused?, fault?}` (Rust `HistoryRemoteReplay`,
  TS twin, schema, fixture: 2 valid + 1 invalid). Notices: `K` `HISTORY_NOTICE_LABELS` / `history_notice(code)` (TS
  `historyNotice`) for `toolTransaction.open`, `toolTransaction.unknown`, `history.full` (W1-G's texts; German
  "abschließen"), fixture `K/🧫️fixtures/🧫️history-notices/`, schema `K/🧬️schema/🔣️history-notices/`, Rust + TS laws.
- New laws: `a_long_remote_history_change_replays_over_turns_and_pauses_on_cancel`,
  `the_undo_of_a_finalize_over_a_long_history_lands_through_the_driver`. Ticket inputs `🧪️s2-w2a-g9-runtime.py`,
  `🧪️s2-w2a-g9-panel.py`.

#### Update 22:10–23:06 and 02:40 (cargo hold since 02:45, rule 26)

- Peer fallout repaired in my crate: `P/🧪️tests/🖥️test-app-mutations-document/🦀️.rs` — a retirement-macro sweep (18:57)
  wrote `crate::semio_framework_value::retirement::…` (6 paths; the crate is extern) → `semio_framework_value::retirement::…`;
  it broke the whole lib-test target (E0433 + the `TestSnapshot: RetireOwned` bound in the builder-contract test).
- Run `test-tt-7` (23:06; filters `time_travel supersede history_label_reload history_alternatives ui_history_panel
  rendering_the_history_body activated_tool_factory document_archive`): **51 passed, 1 failed**. Green: every Session-2
  law (selection, editing targets, introduced warning after finalize + reload, several drafts → one Supersede, Branch +
  scoped Supersede, label reload, bijection, `the_undo_of_a_finalize_over_a_long_history_lands_through_the_driver`), the
  existing undo/redo-of-finalize laws on the new resumable path, S2-W1G's streamed-transaction law, S2-W2D's
  draft-references law, `document_archive` 7/7 (R2-2 included).
  Red: `a_long_remote_history_change_replays_over_turns_and_pauses_on_cancel` — its fixture, not the runtime: relaying a
  3000-op edit to a fresh replica exceeds the store's fixed displaced-owner retirement authority (`module.vcs` "…retirement
  authority is saturated"). Rewritten (02:42) to the plain seed with the remote store at `defer_remote_replays(Some(1))`;
  rerun queued for after the hold.
- The remote section now also shows for a viewer and during a session, without controls there (a viewer rejects the verbs;
  a session's Cancel/Replay again address the session).
- Queued after the hold (one cargo at a time): the targeted plugin run above; `cargo test -p semio-framework --lib --
  history_patch history_notices history_edit`; kernel vitest; `cargo check -p semio-framework-plugin --target wasm32-wasip2`;
  `cargo check -p semio-framework-os-renderer-wgpu --tests` (my two wgpu fallout edits); the full plugin lib suite vs the
  918/13 baseline.
- 03:03 (hold lifted): `cargo test -p semio-framework --lib -- history_patch history_notices history_edit` **blocked** by a
  peer's in-flight `RecordSpecProducer` dsl refactor (E0308/E0618 in `🗣️dsl`, `🏪️store`); every plugin/kernel cargo run
  waits for it. Kernel vitest (needs `SEMIO_VITEST_POLICY` since a peer's 🏃️process change; scratch policy under
  `🗑️generated/s2-w2a/vitest-*`): **6 files, 75 passed**, incl. the new `🧪️history-notices` (2) and `🧪️history-patch` (3,
  with the `remoteReplay` + `introduced` cases).

## Session 3 — 2026-10-02

Successor S3-W2A (coordinator `⚪b7db773a…`). Nothing committed, no ticket or goal touched, no describe/activate/serve run.
Scratch: `🗑️generated/s3-w2a/`. Aliases as above (`P`, `K`, `M`, `R`, `FWT`), plus `PZ2D` (plan alias).
Status: **in progress** — this section is updated at every milestone.

### 9.1 Repair (rule 28)

- Owned files newer than §8's last update (03:03): `K/🟦️.ts` (03:20: notices + `remoteReplay` twin, plus a peer's move of the
  ephemeral lane into `K/🫧️transient`), `P/⏪️time-travel/🦀️.rs` (03:24), `P/🦀️.rs` (07:56, peers), test files (07:52, peers).
  Checked by compiling (9.3).

### 9.2 Coordinator items received in session 3 (12:00–12:40) — SOURCE WRITTEN, verification in 9.3

- **§19.1 intent-labelled transaction rows.** `ArtifactApp::tool_intent_kinds(tool: &str) -> &'static [&'static str]`
  (default `&[]`; `tool` = the stamped `TransactionRef.tool`, `<appId>#<toolId>`), mirrored on `ArtifactEditor` and forwarded
  by `EditorApp`. `P/🦀️.rs` `history_intent_label::<A>(tool, ops, mutations)` picks the first op whose
  `SemanticDescriptor::kind` the tool declares (label from its mutation row = effective input); the `transaction.is_some()`
  label arm of `build_history_view` uses it, else the first leaf as before. No wire change. Law
  `a_transaction_row_reads_its_declared_intent_leaf_before_and_after_reload` (toy declares `#gesture` → `["set-label"]`).
- **N1 every mutation reachable.** A history row is now a `tree_window_indexed_item` over ALL its mutations: projected ones
  first (flagged leading), then the rest of its edit in op order, materialised on demand from the store for the slice a host
  window opened (`VcsArtifactApp::history_mutation_pages` → `HistoryMutationPages`, new `ui_history_panel` parameter;
  `history_row_mutation_extent`, `history_row_window_path`, `history_row_requested_rows`, `history_mutation_view_of` in
  `P/⏪️time-travel/🦀️.rs` region `🔖️MutationPages`). `HISTORY_PANEL_MUTATION_ROWS` (8) is deleted; the projection/wire cap
  (32 + flagged ≤ 64) is unchanged, so memory stays bounded. Rows default closed (as before) and announce `window.total`.
  Open item: child-member rows beyond a member's own projection are not paged (parent ops only).
- **N15 Edit follows the Begin law.** Reducer query `TimeTravelSession::begin_refusal()` (+ TS twin `timeTravelBeginRefusal`,
  `FWT`), law `begin_is_refused_exactly_where_the_reducer_refuses_it` (Rust, every fixture context) and the TS conformance
  twin; `TimeTravelPanel.begin_refusal`; the mutation row's Edit action is `disabled` with label "Edit: <reason>" and no row
  activation while refused (replaying/choosing/finalizing → "Not possible right now"; changed draft → "Blocked: …").
- **N3 generic chip default.** `resolve_time_travel_reference_labels`: app hook `ArtifactApp::entity_label` (the refinement
  hook, unchanged) → the entity's own `label|name|title|text` in the previewed document (`time_travel_entity_names`, bounded
  walk, `{en,de}` pairs honoured; member sessions read the member preview) → `time_travel_reference_fallback_label` =
  glossary kind word (en/de) + short id. Law `reference_chips_fall_back_to_the_document_name_then_the_kind_and_a_short_id`.
- Ticket inputs: `🧪️s3-w2a-n1-time-travel.py`, `🧪️s3-w2a-n1-panel.py`, `🧪️s3-w2a-n3-entity-labels.py`.
- **N1 refinement (12:45).** A row opens by default while one of its (session-overlaid) mutation rows carries an outcome
  (`history_row_opens_by_default`), so a warning stays visible without expanding; the page builder mirrors the first-paint
  slice (`history_row_window_rows`: host request, else default-open first `min(total, 64, viewport rows)`). Rows without
  outcomes stay closed and announce `window.total`. Agreed with S3-W1E: ONE paging mechanism (tree windows) for history rows,
  editor inputs, chips and long option lists; no `historyEditView` verb; `historyEditInput{edit?: insert|remove}` is W1E's.
- **§20.6 no hand-written runtime label.** `dispatch_catalogue_example` no longer sets `description: "Set Active Example"`;
  `runtime_verb_label(verb)` (framework en/de: "Load example" / "Beispiel laden") backs `verb_label` for runtime verbs no app
  declares (`CATALOGUE_EXAMPLE_ACTION_ID`). Law `the_catalogue_route_reads_the_framework_label_never_a_hand_written_one`
  (toy gains `catalogue_example_document("five")`).
- **Puzzle 2d witness (item 3), SOURCE WRITTEN.** New language-agnostic corpus
  `PZ2D/✏️editor/🧫️fixtures/🧫️history-edit-runtime/🔣️.json` (5 scenarios) + schema `PZ2D/✏️editor/🧬️schema/🔣️history-edit-runtime/🔣️.json`;
  Rust laws `PZ2D/✏️editor/🧪️tests/🧪️history-edit-runtime/🦀️.rs` (mounted as `history_edit_runtime_tests` in `✏️editor/🦀️.rs`):
  `every_corpus_scenario_reaches_its_session_outcomes_rows_and_heads` (several drafts from a review → ONE overwrite + undo/redo;
  replay cancelled → needsReplay → rerun → ready; introduced Warning after finalize + text reload + pack reload, en/de words;
  Fatal `duplicate-id` + Error `target-missing` → Next problem → withdraw → Next problem → Use selection → ready → overwrite
  (undo/redo) and new alternative "Fixed" (undo/redo within it); every head = fresh fold of its edited log),
  `replay_progress_rides_the_ui_frames_over_a_long_downstream` (900 downstream drags), and
  `a_long_remote_history_change_replays_over_turns_pauses_and_resumes_on_the_board` (G9 store half through the runtime:
  `remoteReplay` on the wire, Cancel replay pauses, Replay again resumes, adoption = author head). Independent Python oracle
  `🐍️.py` beside it (jsonschema + shapely, reuses the select-tool oracle's `fold`): **5 scenarios, 20 head nodes agree**;
  negative control (one head x off by 1) exits 1. Taxonomy report on the 3 new dirs: clean ×3.

### 9.3 After the reboot (18:40–19:25)

- **Repair (rule 28):** every session-3 edit was intact (§19.1, N1, N3, N15, §20.6, the puzzle corpus and laws). S3-W1E's
  N2 runtime regions (`time_travel_pointer_insert/remove`, `time_travel_default_value`, the `edit` arm) landed in
  `P/⏪️time-travel/🦀️.rs`. I reviewed them while reading the file and found nothing touching my regions.
- `cargo check -p semio-framework-plugin --lib` at 18:46: **Finished**, with no warning in `⏪️time-travel`.
- **Plugin test-target repair (coordinator GO).** The peer locale move (auto-commit 202c4b7b5b1: a private
  `use semio_framework_ui_locale::{…}` in the os-kernel root) broke 58 sites.
  - Ticket input `🧪️s3-w2a-plugin-locale-imports.py` migrated 29 plugin test/fixture files, idempotently, to
    `semio_framework_ui_locale::{LocalizedLabel, Locale, Terminology}`.
  - The same pass moved `artifact_schema_descriptor_registered` to its new home `semio_framework_schema_registry::`.
  - S3-W1G's N17 reload law needs the private `ArtifactStoreInitializationJob::take_candidate`. Rather than widen it, I
    moved the law, unchanged, into `app` as `P/🧪️tests/🧪️bounded-reload/🦀️.rs`, mounted beside `time_travel_tests`, and
    removed it from `🧾️document-archive-load-legs`.
  - Result: `cargo check -p semio-framework-plugin --lib --tests` **Finished** at 18:57.
- **Kernel TS vitest:** **6 files, 75 passed** (18:5x), and **75 passed** again after the N17 wire rename (19:1x).
- **N17 runtime adoption — SOURCE WRITTEN, ticket input `🧪️s3-w2a-n17-reprojection.py`:**
  - **Kernel wire, rename:** `HistoryRemoteReplay` → `HistoryReprojection {done, total, local, paused, fault?}`, and
    `HistoryPatch.remoteReplay` → `reprojection`. Rust, TS twin, schema and fixture changed together; the fixture gains
    the valid case `a-local-history-step-replays-before-it-is-adopted` and the invalid case `reprojection-local-not-a-boolean`.
  - **New notice `history.replaying`:** en "History is still replaying — wait for it or cancel it first." / de "Der Verlauf
    wird noch neu angewendet — abwarten oder zuerst abbrechen." It is in `HISTORY_NOTICE_LABELS` (4 rows), the TS twin and
    the fixture.
  - **Store settings:** `defer_local_replays(Some(TIME_TRAVEL_REPLAY_OPERATIONS))` is set beside both
    `defer_remote_replays` sites. The constant was renamed from `…_REMOTE_REPLAY_…`.
  - **Driving:** one turn step, `step_reprojection_turn`, drives local and remote. A local step is always driven; a remote
    change only while it is not paused.
  - **Cancel:** Cancel replay on a local step calls `discard_local_step()`, which leaves zero trace.
  - **Blocked finalize:** a local finalize whose report blocks reads "History step refused: Errors must be fixed or
    withdrawn before finalizing".
  - **Body:** the section `framework.history.reprojection` reads "History step / Replaying history: {done} of {total}
    mutations" (de "Verlaufsschritt / Verlauf wird neu angewendet: …").
  - **Busy:** `historyEditBegin` answers `timeTravel.busy` while a local step waits.
  - **History row:** the row of a deferred history verb is held back (`defer_history_step_row`). It is recorded on
    adoption and dropped on cancel or refusal, so a cancelled step leaves no row either.
  - **Laws:**
    - `an_interior_undo_over_a_long_history_replays_over_turns_and_cancel_leaves_zero_trace` (toy: the author's undo under
      another author's 600-op edit);
    - `switching_to_a_long_alternative_replays_over_turns_and_cancel_leaves_zero_trace` (puzzle 2d, 360 drags, ≥ 2
      turns, at most one budget per turn).
  - **Open:** routing `load_document_pack/text` and `hydrate_document_lane` through
    `begin_persisted_document_store_replacement` (§3.5 of `📓️api-deferred-history-replays.md`).
- **Blocked since 19:00:** os-kernel peers. First the `🗣️dsl` crate extraction, then
  `📡️spr/🧵️channel/🦀️.rs:216` (`crate::Fault*` gone) and `🚪️io/🦀️.rs:7` (`dsl::Diagnostic`). No plugin or kernel cargo run
  is possible until they land.
- **N15 per S3-W1E's new contract (19:35):** the refused Edit keeps label "Edit"/"Bearbeiten" and carries
  `RowAction::disabled_because(<refusal text>)`, which both renderers announce as the description. The law
  `every_mutation_of_a_long_transaction_is_reachable_and_edit_follows_the_begin_law` asserts `reason`.
- **Review of S3-W1E's N2 runtime regions:** `time_travel_pointer_insert/remove`, `time_travel_default_value`, and the `edit`
  arm of `draft_time_travel_input`. They are sound:
  - every list edit is judged by the payload schema;
  - refusals land on the list row;
  - the regions are fail-closed when no validator compiles.
  
  One nit: array indexes with leading zeros (`/targets/01`) are accepted, which RFC 6901 forbids. No blocker.
- **Reload routing (N17 §3.5): NOT STARTED, needs a coordinator decision.** `load_document_pack/text` and
  `hydrate_document_lane` are reached from the component host through `resolve_ready(..)` (`P/🦀️.rs` ≈41852, ≈44056), which
  panics on any suspension. A stepped reload therefore needs a multi-turn load/hydrate protocol between host and guest
  (an operation handle plus completion, like the archive load), not a local change. `parse_document_pack` also folds the
  history before any initializer runs. The archive path already avoids that with the stepped `PersistedDocumentHydration`.

### 9.4 Stepped whole-document load (coordinator decision 19:40) — API on disk, guest source in progress

- **API:** `T/📓️api-stepped-document-load.md`. Every whole-document load becomes the existing stepped, ACK-owned
  document-archive load with no members. `AppCommand::LoadDocument` is to be deleted; that frame-layout change rides the
  §20.7 bump wave. Progress and cancel are covered, cancel leaves zero trace, and a `document.loading` refusal holds while
  the load runs.
- **Wire changes (not yet consumed by any host), ticket input `🧪️s3-w2a-n17-reprojection-kind.py`:**
  - `HistoryReprojection.kind: remote|step|load` (`HistoryReprojectionKind`) replaces `local: bool`;
  - fixture cases `a-whole-document-load-replays-before-it-is-adopted` and `reprojection-kind-unknown`;
  - kernel notice `document.loading`: en "The document is still loading — wait for it or cancel it first." / de "Das
    Dokument wird noch geladen — abwarten oder zuerst abbrechen." (`HISTORY_NOTICE_LABELS` now has 5 rows).
  - Kernel vitest: **75 passed**, also after these changes.
- **Guest source (unverified while the tree is red):**
  - `DOCUMENT_LOADING_CODE`;
  - `document_loading_refusal`, wired at the head of `dispatch_action` and of `dispatch_typed_command_inner`;
  - `live_document_load`;
  - `reprojection_status`, which reports a live load as kind `load` using the archive status;
  - `historyEditCancelReplay`, which cancels a live load first;
  - the body section text "Document load / Loading document: {done} of {total}".
- **Open, in this order:**
  1. cold-pair ingress and checkpoint restore onto the archive load (multi-turn `ColdPairIngressStatus::Loading`);
  2. the MCP workspace and `🏃️run` senders onto archive admit + poll;
  3. op-level `completed/total`, which needs a progress hook on the retained initializer (S3-W1G region);
  4. removing `load_document_pack/text` from the `PluginApp` surface (55 files and 84 test call sites, done as a sweep to
     `artifact_app_laws::load_document`);
  5. the decision on `PureCommand`: its `.pack` is the INITIAL snapshot, so a head-only hydration is impossible on today's
     wire. I proposed that the sender send head-snapshot packs;
  6. laws §6.

### 9.5 Pure command head-only (decision §4 = b, 20:00) — SOURCE WRITTEN, ticket input `🧪️s3-w2a-pure-head.py`

- `hydrate_document_lane(pack, spr)` now has three cases:
  - **Empty lanes:** the live document is kept, and nothing changes. This is what the MCP workspace sends today.
  - **A history lane (`spr` non-empty):** refused with `pure.history-unavailable` (`PURE_HISTORY_UNAVAILABLE_CODE`).
  - **A head pack alone:** decoded straight into a genesis-only store. That is O(snapshot), with no fold and no suspension.
    The ledger is then marked `history_unavailable`.
- **In head-only mode:**
  - the history-lane verbs, `revertToCommand` and every `historyEdit*` verb answer `pure.history-unavailable`;
  - `history_snapshot` (ReadHistory) answers the same;
  - `load_document_pack/text` lift the mode.
- **Kernel notice** (6 rows now): en "This is a head-only evaluation without history — open the document in a live instance to
  use its history." / de "Dies ist eine Auswertung nur des aktuellen Stands ohne Verlauf — für den Verlauf das Dokument in
  einer laufenden Instanz öffnen."
- **Law:** `a_pure_lane_hydrates_the_head_without_history_and_history_verbs_refuse` (240 edits).

### 9.6 State at 20:05 — waiting for TREE GREEN

The tree is red: os-kernel and the replication ↔ dsl crate cycle are a peer's DSL extraction, and the coordinator tracks it.
**Nothing written after 18:57 has been compiled.** Owed, in order, once TREE GREEN:
1. `cargo check -p semio-framework-plugin --lib --tests`, and the puzzle crate check (`--manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --features component-app-assembly --tests`).
2. The targeted plugin laws: `time_travel supersede history_label_reload history_alternatives ui_history_panel
   rendering_the_history_body activated_tool_factory document_archive composed_child_history scrub history_ bounded_reload`.
   These include the new §19.1, N1/N15, N3, §20.6, N17 interior-undo and pure-head laws.
3. `cargo test -p semio-framework --lib -- history_patch history_notices history_edit` (kernel and manifest wire).
4. The puzzle 2d laws `history_edit_runtime_tests` (corpus, progress, remote, alternative switch).
5. `cargo check --target wasm32-wasip2` of the plugin and the puzzle plugin.
6. The full plugin lib suite against the baseline.

**Open items** (owners in brackets):
- Stepped load: cold-pair ingress (multi-turn `Loading` needs host and actor protocol coordination), checkpoint restore,
  the MCP/`🏃️run` `LoadDocument` senders, removal of `load_document_pack/text` from `PluginApp` (55 files / 84 test call
  sites), and op-level progress (waiting for S3-W1G's initializer hook) [S3-W2A].
- Delete `AppCommand::LoadDocument` in the §20.7 bump wave [coordinator].
- React mapping of the notice codes through `historyNotice()` [S3-W2B].

**Coordinator actions:**
- Descriptor regeneration: none new. The history-edit verb set is unchanged, because the stepped load reuses
  `historyEditCancelReplay`.
- Activation is needed for live proof.
- The §20.7 bump wave must carry the `LoadDocument` deletion.
- **20:15 op-level load progress (source).** S3-W1G landed `ArtifactStoreInitializationJob::progress() -> (folded, to_fold)`,
  plus linear edit validation and linear applied/redo lookup (no more N² reload). In my regions:
  - `ActiveArtifactStoreReplacement.fold_progress` captures it at every checked-out initializer outcome.
  - `ActiveDocumentArchiveLoad.fold` copies it while awaiting the replacement.
  - `DocumentArchiveLoadStatus` reports `completed + folded` of `total + to_fold`. That is real per-operation progress on
    the wire and in `reprojection {kind: load}`.

### 9.7 Resume 2026-10-03 05:47 (after the ~21:00 usage cut; TREE GREEN from the coordinator)

- Repair: every edit from 9.2–9.6 is intact. Nothing was half-written: the 19:58 kernel check was green and nothing was in
  flight at the cut. Running the owed list now (9.6 order).
- `cargo test -p semio-framework --lib -- history_patch history_notices history_edit` (06:04): **10 passed, 0 failed**. This
  includes the reprojection kind rename and the 6-row notice table. Fix in my region first:
  `K/🧪️tests/💡️service-operation/🦀️.rs:9`, where `dsl::json::from_json_str` (gone after the DSL extraction) became
  `semio_framework_pack_json::from_json_str(.., JsonMemberPolicy::Reject)`.
- The plugin TEST target is red from the peer sqlite-snapshot `ValueError` change (05:36; 81 errors in fixture impls).
  S3-INFRA owns that sweep, per the coordinator.
- Puzzle 2d `history_edit_runtime_tests` (06:27): blocked by the same peer sqlite `ValueError` change, now in the stdio zip
  artifact crate (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/…/🪶️sqlite/🦀️.rs`), which the puzzle test build pulls in. S3-INFRA's
  sweep covers it.
- New: `HistoryPatch.editCount: u32` (Rust, TS twin, schema, fixture: valid case and invalid `edit-count-negative`). It is
  filled by the runtime from `vcs.edits.len()` and is the structured `{n}` of the `history.full` notice. S3-W2B should
  read it instead of matching `\d+` in the fault message (`🛠️ShellHelpers/🟦️.tsx:2362`). Kernel vitest: **75 passed**.
- F6 (`📓️audit-s3-tools.md`) is already resolved on disk: `commit_time_travel` applies `Finalized` before
  `publish_time_travel_member`, and the member row label is the `TimeTravelLabel::MemberEdited` glossary entry.
- Cold-pair ingress analysis: the browser host (`🏪️store/👷️worker/🟦️.ts` `transferColdPair`) requires the LAST page's turn
  to answer `Applied` (`owner.assertApplied`). A multi-turn `Loading` therefore needs a host change: keep invoking empty
  poll turns while the status is `loading`. The wgpu renderer and gis cold-pair tests need the same. Guest and host must
  change together, so this is now a coordinated item in `📓️api-stepped-document-load.md` §4.
- `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2` (06:29): **Finished**, with no warning in my regions.
- `cargo test -p semio-framework-time-travel` (06:32): **14 passed, 0 failed**, including the new N15 law
  `begin_is_refused_exactly_where_the_reducer_refuses_it`. FWT TS conformance: 20 passed (session 3).
- 06:40: handoffs sent to `main`:
  - cold-pair needs a host change first (store worker, wgpu);
  - the MCP/`🏃️run` senders belong to other WPs; the recipe is in `📓️api-stepped-document-load.md` §8;
  - F7 plan: `Menu::of(registry, locale)`, a glossary count phrase, and a disabled delete row with a reason once S3-W1E
    adds `ContextMenuItemSpec.reason`. Waiting for a decision.

### 9.8 Stepped load hosts, publication gate, F7 (2026-10-03 06:40–07:05) — SOURCE WRITTEN, compile pending TREE GREEN

- **Publication regression (puzzle 3d 22 vs 1590 units, `6f33e313da9`)**: the fix was agreed with S3-W1G and is applied in
  `P/🦀️.rs`. `PUBLICATION_RETURNED_ROOT_ALLOWANCE = 1` lets one returned root-snapshot read through before the
  `reclaim_document_snapshot_read_returns` deferral, and the deferral now applies only to `PendingArtifactStorePublication::Artifact`.
  S3-W1G measures it on TREE GREEN.
- **F7 approved** (coordinator). W1E's contract is on disk: `ContextMenuItemSpec.reason` + `disabled_because(String)`, the
  TS `reason?`, React done, wgpu with W2C. The sweep runs **after** the owed laws: `Menu::of(registry, locale)`,
  `selection_count_phrase(locale, &[(count, kind_key)])` with glossary kind words, and a disabled delete row
  "Nothing selected" / "Nichts ausgewählt". It covers every `Menu::of` caller (procedural gen2d/gen3d, flow, sequence,
  layout, trinity rewriting/jack, dag, puzzle 2d/5d, hub space, plugin `🧪️node-graph-delete-row`, the builder-contract test).
- **In-guest archive load helpers** (`P/🦀️.rs` `plugin_runtime`): `plugin_begin_document_archive_load`,
  `plugin_poll_document_archive_load` (one poll wall via `drive_self_waking_ready`) and `plugin_acknowledge_document_archive_load`.
  They are the in-guest twins of `LoadDocumentArchive` / `Poll` / `Acknowledge`.
- **Cold-pair ingress, guest step 2** (ticket input `🧪️s3-w2a-cold-pair-stepped.py`). Host contract per S3-W1G
  (`settleColdPairLoading`); S3-W2C confirmed wgpu never sends cold pairs.
  - The verified pair is admitted as an archive load (`members: []`) under `COLD_PAIR_DOCUMENT_LOAD_OPERATION | transfer_generation`
    (bit 63). It is held in the reactor's `COLD_PAIR_DOCUMENT_LOAD` and stepped by one poll per turn
    (`⚛️reactor/🔄️turn` `step_cold_pair_document_load`).
  - While running, every turn answers `ColdPairIngressStatus::Loading(last page cursor)`, sends **no UI patch**, and reports `MoreWork`.
  - At the terminal state: acknowledge, then `finish_load`. `Ready` answers `Applied`. `Cancelled` answers `cold-pair.load-cancelled`
    (the previous document is kept, so the guest has restored). A fault carries its bytes. A dead lifetime answers `cold-pair.not-live`.
- **Checkpoint restore** (ticket input `🧪️s3-w2a-checkpoint-restore-stepped.py`).
  - `checkpoint::restore` re-creates each instance under its **checkpointed id**. It now uses `plugin_create_app_with_id`;
    before, a fresh auto id disagreed with the `INSTANCE_METADATA` and task-restart rows that `restore_now` keys by the old id.
  - It admits the document as an archive load under `RESTORE_DOCUMENT_LOAD_OPERATION = 1 << 62` and answers
    `RestoredCheckpoint { pack, document_loads }`.
  - `restore_now` queues the loads in `RESTORED_DOCUMENT_LOADS`. Each turn, `step_restored_document_loads` polls each one:
    - a terminal load is acknowledged;
    - a fault reaches the instance's shell as an error frame;
    - a cancel keeps the initial document;
    - a closed instance's load is dropped.
  - While any load runs, the instance answers `document.loading`.
- **Named MoreWork source** `TurnMoreWorkSources.document_load` (cold pair loading or restore loads pending) replaces the
  ad-hoc `|| cold_pair_loading` in the status, so an idle law names it.
- **Tree at 06:53** (`check-plugin-tests-12.txt`): `semio-framework-os-kernel` red, 31 errors, all peer work. They are the
  `IoError` / `ValueError` / `PackError::into_value_error` / `ValueRefusalKind` reshape in `🚪️io/🦀️.rs:2470`, `🏪️store/🦀️.rs:10917`,
  `🧬️semio/🦀️.rs:4` and the sqlite snapshot files. Nothing in my regions; S3-INFRA's sweep.
- **Cold-pair supersession** (S3-W1G review; ticket input `🧪️s3-w2a-cold-pair-supersede.py`). A cold-pair page of another
  `transfer_generation` for an instance whose earlier pair still loads means the host abandoned that transfer.
  `supersede_cold_pair_document_load` cancels the stale archive load, using the new `plugin_cancel_document_archive_load`,
  the in-guest twin of `CancelDocumentArchiveLoad`. The step then ends it as `cold-pair.load-cancelled`, so a stale fold
  never publishes. An instance close is already covered: the app close ladder (`drive_document_archive_load_retirements`,
  `closing`) cancels and drops every non-terminal load, and the turn step finishes the pair as `cold-pair.not-live`.
- **10:45 resume (usage cut 07:15).** All 9.8 edits are intact. `cargo check -p semio-framework-plugin --lib`: **Finished** (10:50).
  `--target wasm32-wasip2`: **Finished** (10:55). No warning in the cold-pair, checkpoint or turn edits.
  - The reactor's `thread_local!` block is the framework macro, which refuses `const { … }`. A peer had wrapped my two
    initializers in parentheses, which only moved the problem to a warning. Both are now plain `RefCell::new(..)`, like
    their neighbours.

### 9.9 F7 context menus, archive load lifts head-only (2026-10-03 11:00–11:40) — SOURCE WRITTEN; ICU oracle green, Rust pending TREE GREEN

- **New domain module `P/🖱️context-menu/`.** The `🖱️MenuBuilder` region moved out of `P/🦀️.rs`, mounted as `app::context_menu` and
  re-exported unchanged at the crate root. It holds:
  - `🦀️.rs`;
  - `🧬️schema/🔣️.json` (`semio.plugin.context-menu-selection/v1`);
  - `🧫️fixtures/🔣️.json`: the glossary of 5 kinds × en/de `one`/`other`, the `deleteSelection`/`nothingSelected` labels,
    20 phrase cases and 5 delete-row cases;
  - `🧪️tests/🟦️.ts`: the language-agnostic oracle. Ajv checks the schema, with negative controls. **ICU via `Intl.PluralRules` +
    `Intl.ListFormat` + `Intl.NumberFormat`** reproduces every phrase and delete row: `bun test` **27 pass / 0 fail**
    (`ts-context-menu-1.txt`);
  - `🧪️tests/🦀️.rs`: 4 Rust laws. The glossary equals the fixture both ways, every phrase case and delete row reproduces, and
    `Menu` resolves rows and reasons in the caller's axes.
- **API**, with no default language anywhere:
  - `Menu::of(registry, axes: &impl LabelAxes)` resolves every declared row in the call's **locale and terminology**. The approved
    plan said `locale`; terminology rides along, because labels are terminology-dependent.
  - `Menu::disabled_because(&LocalizedLabel)` disables the most recent row with its reason, for example
    `.destructive("clearSelection").when(empty, |m| m.disabled_because(&nothing_selected()))`. The reason-less
    `Menu::disabled(id, bool)` is gone; it had no callers.
  - `SelectionKind {Node, Edge, Frame, Item, Part}` and `selection_count_phrase(locale, &[(count, SelectionKind)]) -> Option<String>`
    follow CLDR `one`/`other`, ICU list conjunctions ("a, b, and c" / "a, b und c") and no digit grouping.
  - `delete_selection()` / `nothing_selected() -> LocalizedLabel`.
  - `node_graph_delete_selection_spec(label, axes, nodes, edges, dispatch) -> ContextMenuItemSpec`: an empty selection is now a
    **visible, disabled row with reason "Nothing selected" / "Nichts ausgewählt" and no action/args** (W1E's `ContextMenuItemSpec.reason`),
    never an omitted row.
- **Sweep** (ticket input `🧪️s3-w2a-f7-plugins.py`): 76 edits over 19 files, compile-atomic per crate. Covers writer, gen2d, gen3d, flow,
  gismap, sequence, process3d, layout, cad, trinity rewriting/jack, dag, and puzzle 2d/5d/3d.
  - Every `is_de: bool` helper now takes `view_state`.
  - Flow's hand-built delete row uses the shared spec (`Direct`).
  - Sequence and trinity lost their English-only "Delete selection" literal (now `delete_selection()`).
  - Disabled `clearSelection` (gismap, layout) and writer copy/cut rows now carry the reason.
  - Plugin-crate callers too: the builder-contract test, `🧪️node-graph-delete-row`, and the `TestApp` menu.
  - Tests that pinned "an empty selection omits delete" now pin the disabled row: gen2d unit, three flow unit asserts.
- **Archive load lifts head-only** (S3-LOAD §6 item 5; ticket input `🧪️s3-w2a-archive-lifts-head-only.py`). The committed store replacement
  (`P/🦀️.rs` replacement commit, beside `retire_displaced_document_rows`) calls `set_history_unavailable(false)`.
  - Every whole-document archive load restores history exactly like the old `load_document_pack/text`.
  - The law `a_pure_lane_hydrates_the_head_without_history_and_history_verbs_refuse` now ends through the stamped archive load
    over real polls: `Ready`, head-only lifted, source head + 240 edits.
- **Tree 11:20** (`check-plugin-lib-17.txt`): os-kernel red from a peer, 4 errors. `🏪️store/🦀️.rs:17806/17823` `shared_prefix_len` not found;
  `:22122` a 5→6-argument call. Nothing in my regions; the plugin lib with the moved module compiled green at 11:06
  (`check-plugin-lib-16.txt`) before the label refinement.
- **W2A-4 cold-pair load slot** (ticket input `🧪️s3-w2a-cold-pair-serialized.py`). The TurnResult carries ONE `ColdPairIngressStatus`,
  so a turn can answer only one transfer. The guest therefore runs **one cold-pair document load per actor at a time** and answers each
  turn for the page that arrived:
  - **Serialization.** While another lifetime's load runs, a terminal page answers `Backpressure(its own cursor)` *before* `accept_page`,
    so the page is not consumed. W1G's `retryColdPairBackpressure` resends it fresh until the running load ends. Non-terminal pages
    still buffer.
  - **Answer precedence.** A load's step answers a turn only when that turn carried no page or a page of the load's own lifetime.
    A non-terminal page of instance B no longer gets instance A's `Loading`.
  - **Supersession.** A newer `transfer_generation` of the same lifetime marks the running load `superseded` and cancels it. Its terminal
    answer is never sent. The registry (`accept_page`) then retires the finished owner in bounded steps, one page wiped per attempt,
    each answering `Backpressure(stale cursor)`, before taking the newer page 0. Before this, a second transfer on a lifetime got
    `Backpressure` forever.
  - `document_load` MoreWork now reads "a cold-pair load is pending", so a silenced load is still driven.
  - New registry law `a_newer_transfer_of_the_same_lifetime_retires_the_finished_pair_in_bounded_steps`.
  - **Residual, needs the bump wave:** in a pooled actor (opt-in, unused today; the browser host runs one document per actor child), a
    load that ends during another transfer's turn has no lifetime-addressed answer channel. A per-lifetime status list on `TurnResult`
    would close it. A turn-level two-instance law is still owed; it needs a cold-pair header builder over a real instance's pack in
    the native lifecycle harness.

## Session 4 — 2026-10-04

Successor S4-RUNTIME (Opus), session-4 coordinator `⚪487b04ad…`. Owns `OSM/🔌️plugin/**` time-travel/history regions,
`FW/🎠️kernel/**`, `FW/⏪️time-travel`. Scratch: `🗑️generated/s4-runtime/`.

### 10.1 Repair (rule 34) — done (02:30)

- No half-edit of S3-W2A found: every file I own that changed after the 9.9 section (11:40 10-03) changed through peer sweeps
  (value/DSL, sqlite fixtures, 23:30–01:31) or session-3 owners that landed callee-first (S3-W1G `ReplayTurnBudget` 12:06).
- L4 (W2A-2) predicate is already on disk (landed between 11:40 and the 12:07 cut, not reported): `dispatch_emit` records no row for a
  config-only emit (`!in_transaction && !config_edited`), `record_settled_typed_operation_command` skips `published_config`, and
  `record_typed_operation_lane` runs only on the artifact and child lanes. The three pinned laws no longer exist under those names
  (`git grep` 0 outside a 08-06 ticket backup). Verification: energy L4 law + plugin `history_` laws (10.3).
- First tree check 02:07–02:24 (`check-plugin-1.txt`): plugin lib 3 errors / lib test 12 errors, ALL in S4-BUMP's in-flight frame
  and `begin_*apply_batch(description)` removal (`🦀️.rs` 9025/9049/34728/43671/45133, builder-contract 4718–4749,
  mutation-fixtures-transaction 535–564). Not mine; re-checked after BUMP lands.

### 10.2 Source changes (02:30–03:20) — WRITTEN, compile pending BUMP

- **W2A-1 incremental `HistoryView` (design §20.14).** `P` CommandLog region: `HistoryViewStamp`, `HistoryViewCache`,
  `HistoryViewPatch`, `HistoryBackfillMark`, `HistoryRowReads`; runtime fields `history_view` (survives `self.cache = None`) and
  `history_backfill`. `refresh_cache` drops the cache's share of the view, then `refresh_history_view` patches it in place
  (`Arc::make_mut`: same allocation unless a command job holds it) or rebuilds:
  - `TT` region `🔖️HistoryViewStamps`: `HistoryStoreStamp` (generation, ledger/transition/checkpoint/alternative counts, line, checkpoint,
    applied/redo tails of ≤ 8 ids, tail op count), `history_store_change` (Append/amend → applied positions `[prev-1, now)`; undo/redo
    of ≤ 8 tail edits → the moved ids + the uncovered/covered top; anything else → `None` = rebuild), `MemberHistoryStampVisitor`.
  - `history_view_patch`: new log rows (`partition_point` on seq), the previous newest row (the log amends only `last_mut`), the held rows
    of touched document + member edits (found from the log tail), shell rows whose undo flipped; rows retired / member set changed /
    supersede records changed → rebuild.
  - One row builder `history_row` for full build and patch: per-edit slice reads `applied_edit_mutations/outcomes(position)` (W1G-3),
    position from the touched set or the previous row (hint checked against the stack, else `applied_edit_position`). The per-row
    `mutation_ops()`/`mutation_outcomes()` whole-history reads are gone from the runtime (MemberHistoryVisitor too: per-edit slices,
    `wanted` filter for a patch).
  - `backfill_command_log` is O(change): past a `HistoryBackfillMark` whose document and member ledger prefixes still stand (length + last
    id, `MemberLedgerTailVisitor`) only new edits / member edits / supersede records are read against rows logged since the mark's row;
    `retire_displaced_document_rows` drops the mark.
  - `history_patch` shares the cached view (was a deep clone per patch) and finds dirty rows by binary search on `seq`.
  - The snapshot-override render reads the cached view (was a full build per render). The streamed-tick special arm and
    `history_edit_mutation_views` are deleted (the patch covers a growing tail edit).
  - `history_mutation_pages` reads only each paged row's edit slice.
- **W2A-11 / D13.** `history_projected_operations(len, flagged)`: the first 32 + at most 32 later flagged, most severe first → a row's
  `worst` is the worst over the whole edit. Member-only rows (flow/sequence/dag child drags) page past their projection
  (`history_row_mutation_extent` + `MemberMutationPageVisitor`). Residual: a MIXED row's member edits past their own projection
  (CommandView carries no member op count) — see 10.6.
- **W2A-10.** A page miss renders a dimmed placeholder row (`HistoryPanelText::MutationUnavailable` en/de), never a failed body.
- **W2A-3.** One wall deadline per driver turn on the ledger's injectable `turn_clock` (`time_travel_turn_deadline`), passed to the
  session replay (`StepReplay { deadline_us, clock }`), the finalize-undo authoring and `store.step_reprojection(Some(deadline))`.
- **Laws (plugin `🧪️time-travel`)**: `the_history_view_patches_only_the_rows_a_change_touched` (200 edits; append/undo/redo ≤ 3 rows,
  render 0 rows, patched == full rebuild, a history edit rebuilds), `the_projected_rows_carry_the_worst_severity_of_the_whole_edit`,
  `a_deferred_history_step_ends_its_turn_at_the_wall_deadline_not_an_operation_count` (1 ms counted clock: ≤ 8 ops / 4 ms turn).

### 10.3 Minors, relays and vocabulary (03:20–04:25) — WRITTEN; plugin `--lib` GREEN 04:23

- **W2A-8** (with S4-UI's kernel region): `time_travel_reprojection_section` takes title + line from
  `kernel::history_reprojection_status` (one copy for both shells and the body; a refused adoption reads its named notice, never a raw
  code). Deleted `HistoryPanelText::{RemoteReplay, RemoteReplayProgress, RemoteReplayPaused, RemoteReplayRefused, LocalReplay,
  LocalReplayProgress, LocalReplayRefused, DocumentLoad, DocumentLoadProgress}` and the raw-code `reason()` fallback. Icon by kind
  (remote cloud-download, step undo, load download). A refused local step stores `HISTORY_STEP_BLOCKED_CODE = "history.step-blocked"`
  (S4-UI kernel notice). The pause is lifted when nothing waits any more (adoption, or at the start of a driver turn); a refused
  adoption's row is dismissed by the next non-view/non-interaction dispatch (`dismiss_refused_reprojection`, head of `dispatch_action`).
- **W2A-9**: `time_travel_busy()` is the one predicate behind the `timeTravel.busy` refusal and the panel: `ui_history_panel` gained
  `busy: bool` (callers: runtime + 12 test call sites incl. wgpu `🧪️wgpu-time-travel`); the Edit row action is disabled with the
  `RefusalBusy` reason (`history_panel_mutation_row` now takes a `TimeTravelLabel`).
- **W2A-13**: `document_loading_refusal` exempts declared `View` AND `Interaction` verbs and `INTERACTION_ACTION_IDS` (hover/pick no longer
  spam faults during a restore) via `declared_action_kind`. (The "emit history event on adoption" half: the adoption already calls
  `note_time_travel_changed(true, true)` → patch + body refresh.)
- **W2A-14**: `time_travel_pointer_index` (RFC 6901: `0` or a digit run without leading zero, no sign) replaces every `parse::<usize>()` of a
  pointer token in TT (get/set/get_mut/insert/remove/input_at/optionSource templates).
- **W2A-12** (tool-id parsing): NOT changed — `tool_intent_kinds(tool)` keeps the stamped `<appId>#<toolId>` (two adopters, gen3d + flow,
  parse it; changing the trait signature touches their crates — routed to `main` as a D-item for S4-TOOLS-B / S4-FLOWCAD, not done here).
- **History filter vocabulary** (E2E routing): the declared `setHistoryCommandFilter` options were `withoutOperations/onlyOperations` while the
  body dispatched `withoutMutations/onlyMutations`. One vocabulary now: manifest consts `HISTORY_COMMAND_FILTER_{ALL,WITHOUT_MUTATIONS,
  ONLY_MUTATIONS}` are the declared options (labels en/de "Without/Only Mutations", "Ohne/Nur Mutationen"); `HistoryCommandFilter::{ALL,
  value, from_value}` is the only reader (body select, `HistoryPatch.commandFilter`, dispatch); an undeclared value is refused
  `history-filter.unknown` (was: silently `All`). Law `every_declared_history_filter_option_is_dispatched_and_echoed`. Generated hub
  descriptors still carry the old options → describe wave (coordinator).
- **D6 review (S4-GRAPHS' fix in my region)**: `close_streamed_transaction_unit` keeps `emit.transaction` when `child_emits` is non-empty
  (commit), strips it on abort — correct: `PendingChildGroupPublication::new(…, emit.transaction.take())` stamps every member op, the
  backfill groups member edits by transaction id. Law owed (child-only commit = one row with the ref; abort = zero trace) — 10.5.
- Rule-39 incidents caused by me: TT:3901 `ui_label(&String)` (03:02–03:13) and the manifest-constant ordering (callers seen before the
  manifest crate rebuilt, 03:41). Both fixed; PLUGIN GREEN 04:23 (`check-plugin-5.txt`, exit 0, 259 warnings, none in TT/my regions).

### 10.4 Resume 06:45 → owed runs and fixes (06:45–08:50)

- Resume (coordinator 06:45): nothing was half-written at the 04:15 cut; the filter vocabulary was complete on disk. PLUGIN GREEN 04:23
  (`check-plugin-5.txt`).
- `cargo check -p semio-framework-plugin --lib --tests` 07:34–07:37 **exit 0** (`check-plugin-tests-9.txt`) after fixing my two test
  errors: the pure-lane law onto S4-LOAD's `PluginApp::hydrate_pure_head(head)` (the history-carrying refusal cannot exist any more by
  type), `ArgSchema` path in the filter law. (A 07:15 red at PLG 17563–17665 was a stale queued build — S4-STORE's callees were on disk.)
- **W2A law run** 07:37–07:53: `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s4-runtime cargo test -p semio-framework-plugin --lib --
  time_travel supersede history_label_reload history_alternatives ui_history_panel rendering_the_history_body activated_tool_factory
  document_archive composed_child_history scrub history_ bounded_reload` → **75 passed / 15 failed**. Green incl.: all L4 laws
  (`a_config_lane_edit_is_never_a_history_row_and_undo_never_steps_over_it`, `a_config_only_view_action_is_not_a_history_row`,
  `consecutive_config_only_view_dispatches_add_no_history_rows`, `history_rows_exclude_interactions_and_pure_views`), the new
  `every_declared_history_filter_option_is_dispatched_and_echoed`, `the_projected_rows_carry_the_worst_severity_of_the_whole_edit`,
  fixture scenarios, overwrite == fresh fold, alternatives, §19.1, N1 long transaction, finalize-undo over a long history, replay progress.
  The 15 failures and what I did (laws OWED under rule 43 — no test builds until TESTS RESUMED):
  1. 5 × "artifact history ledger reached Drop before every exact entry owner was retired" (history_label_reload, intent-leaf reload,
     warning reload, history-edit row reload): `artifact_app_laws::load_document_text` / `plugin_load_document_text` dropped the parsed
     envelope after printing its pack → now `envelope.retire_unadopted()` (exact owners). [shared helper, told `main` for S4-LOAD]
  2. `a_history_edit_is_its_own_row…after_reload` (S4-STORE root cause): the archive-load adoption took the local actor from the tail
     edit → the replacement commit now carries the replaced store's `local_actor_id()` into the adopted store.
  3. `a_streamed_tick_extends_cached_history_in_place`: `toolTransaction.shape` "streamed … no tool transaction" — the retained
     publication took `emit.transaction` for the artifact lane but left `transaction_phase = Stream`, so the next lane step's shape check
     refused the same emission. Fixed: the phase becomes `Commit` with the take (one line, PLG retained publication).
  4. `a_long_remote_history_change_replays_over_turns_and_pauses_on_cancel`: (a) the law asserted "no driver work" while the cancel's own
     patch was still due — moved after the 4 driver turns; (b) the paused body had no Replay-again row because the section showed controls
     only while `total > 0` and a cancelled replay counts 0 → Replay again shows whenever a remote change is paused (S4-UI routing (c)).
  5. `an_interior_undo…` and my `a_deferred_history_step_ends_its_turn_at_the_wall_deadline…`: "the interior undo waits for its replay"
     failed — the store steps a replay per EDIT, so one 600-operation edit replays inside the first budget; both laws now put 600 one-op
     edits of another author downstream (`apply_other_edits`).
  6. `undo_and_redo_of_a_finalize…` replicas converge: a relayed change may now span turns → `relay_adopted` (relay + pump until the
     store's reprojection settles) in that law.
  7. `the_history_view_patches_only_the_rows_a_change_touched`: an undo/redo is a `Revert`/`Reinstate` transition, and the store stamp
     counted transitions → every undo rebuilt. Transitions dropped from `HistoryStoreStamp` (stacks show undo/redo; supersede records
     are counted by the view stamp). Also: the stamp carries the content revision (a replaced store may restart at the same generation),
     a fresh row whose edit no change named finds its position from the stack tail, and the history-patch wire marks exactly the rows
     whose projection changed (patch: rebuilt rows that differ + fresh rows; rebuild: diff against the previous view) — so
     `record_command(History)` no longer re-sends every row on each undo/redo (O(change) wire).
  8. `sourced_options_are_the_keys_of_the_previewed_document`: resolved `optionSource` options became an `enum` in the row's JSON schema
     ("never narrows" breach) → the row keeps its `optionSource`; manifest `arg_schema_json_schema` emits `enum` only for declared
     options (Edit tool, region-scoped). Law expectation updated (`option_source: Some(_)`).
  9. `a_pure_lane_hydrates…`: expected 240 edits, the source holds seed + 240 → compares with the source's count.
  10. `a_document_archive_round_trip_lists_every_history_row_of_its_source`: expected edit DESCRIPTIONS as row labels (§20.6 breach in
      the law) — someone fixed the expectation to leaf labels at 07:53.
  11. Not mine → S4-STORE (routed by `main`): `bounded_reload` "supersession … targets an operation that plans foreign steps",
      `retained_window_input_recursive_document_archive…` ("adopted initialization runtime released its seeded operation ids").
- **No raw codes** (S4-UI routing): `time_travel_band_section` reads `ReplayFaulted` for an unknown fault code; `history_code_text` answers
  the frozen-code words, else the framework's placeholder-free notice (`history_notice`/`framework_fault_notice`), else "Could not apply" /
  "Nicht anwendbar" — never the code.
- **K3 (written, WRITTEN BUT UNVERIFIED — rule 43 blocks the law build):** `transient_root!` and `window_transient_owners!` (exported,
  `🪟️window/🫧️transient/🦀️.rs` region `🔖️TransientRoot`) generate a whole-root transient's `Snapshot` mutation (wire
  `{"kind":"snapshot","transient":…}`), descriptor, diff/inverse, JSON op/DSL/pack codecs in its semio envelope, retirement, the
  ephemeral transfer (`footprint` = encoded root bytes, `into_state`), the per-kind owners and `register`/`from_snapshot`/`current`/
  `addressed` (refusals `window-transient.window-required|window-stale|kind-unknown`). Hidden re-exports `__kernel`, `__value`,
  `__pack_json`, `__diagnostic` at the crate root. Law `🧪️tests/🧪️transient-root/🦀️.rs` (4 tests: wire + inverse, codecs + envelope,
  footprint, owners register/address). Adoption in flow/fem/remodel/cad/lowpoly/layout/forms/draw/raster/wfc needs their owners
  (`main` asked) — each replaces ~150 hand-written lines by one invocation; wire shapes unify (local-only state, no persisted bytes).

### 10.5 Resume 11:35 → K3 conversions (§21.5) — WRITTEN; framework GREEN, plugin crates OWED (rules 43/44)

- Repair: nothing half-written at the 08:55 cut (content-revision guard was on disk). The 08:4x wasip2 check died with the session.
- `cargo check -p semio-framework -p semio-framework-plugin --lib` 12:04–12:28 **exit 0** (`check-plugin-13.txt`; two earlier runs died:
  build dir swept by the disk guard, then SIGKILL by the deadlock breaker). Fixed 3 "unnecessary qualification" warnings in TT afterwards.
- **Macros final shape** (`🪟️window/🫧️transient/🦀️.rs` region `🔖️TransientRoot`, exported):
  - `transient_root!` — mutation enum (wire `{"kind":"snapshot","transient":…}`), descriptor, diff/inverse, JSON op text/binary, the
    state's DSL/pack in its envelope. Enough for an artifact-level transient (fem gumball, lowpoly session).
  - `window_transient_transfer!` — the mutation's owned retirement, `footprint` (encoded bytes, refused above the one-item bound),
    `into_state`, and the **compile-time inline-size assertion** (audit F8: state and mutation ≤ `ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES`).
  - `window_transient_owners!` — the transfer + one owner per window kind + `register`/`from_snapshot`/`current`/`addressed`.
  - Kernel framework notices for the generic refusals: `window-transient.window-required|window-stale|kind-unknown` (en/de) in
    `FRAMEWORK_FAULT_NOTICE_LABELS` (17 rows), TS twin, fixture `🧫️framework-notices`, schema namespace pattern. `bun test
    ./🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️framework-notices/🟦️.ts` **4 pass / 0 fail**.
- **Conversions** (ticket inputs `🧪️s4-runtime-k3-transient-root.py` + `🧪️s4-runtime-k3-tidy.py`; every state, its retirement and domain
  helpers kept byte for byte; owner names kept, so registrations elsewhere stand):
  flow (4 window kinds), cad (4 kinds; the four per-window `🫧️transient/🦀️.rs` owner stubs DELETED with their `mod transient;` lines:
  `📐️shape`, `🏢️building`, `🔥️energy`, `🏛️structure-classic` — zero references left), layout (2), draw (1), forms (1), raster (1),
  remodel (3; the generic `RemodelingWindowTransientOwner<K>` + kind markers replaced by `RemodelingModelWindowTransientOwner`,
  `…Frames…`, `…Report…`), wfc bitmap input (1; app registers via the generated `register`), wfc grid2d (`transient_root!` +
  `window_transient_transfer!`; its own owners stay — one config + transient owner pair per window kind), fem 2d gumball (`transient_root!`,
  artifact transient; fem 3d reuses it), lowpoly session (`transient_root!`, artifact transient). Callers switched from
  `current(snapshot)` to `from_snapshot(snapshot)` (raster paint-stroke, remodel editor, wfc bitmap editor).
  Wire changes (local-only, never persisted): remodel/raster/wfc bitmap/fem/lowpoly mutation op text now the tagged JSON form (lowpoly
  lost its `snapshot ` prefix); descriptors: lowpoly's transient leaf emoji 🖌️ → 🫧️ (describe wave regenerates).
  Not converted (not in the §21.5 list): puzzle 2d/3d/5d, note, sequence window transients (same pattern; S4-PUZZLE/TOOLS-A/GRAPHS).
- **OWED (rules 43/44)**: `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-flow-flow -p semio-s-artifact-cad-cad
  -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d -p semio-s-artifact-lowpoly-lowpoly -p semio-s-artifact-remodel-remodeling
  -p semio-s-artifact-raster-raster -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-layout-layout
  -p semio-s-artifact-draw-drawing -p semio-s-artifact-forms-forms --lib --features semio-s-artifact-fem-2d/component-app-assembly,
  semio-s-artifact-fem-3d/component-app-assembly,semio-s-artifact-wfc-grid2d/component-app-assembly,semio-s-artifact-wfc-bitmap/component-app-assembly`
  (+ the same with `--target wasm32-wasip2`), then `cargo test -p semio-framework-plugin --lib -- transient_root` (K3 law) and the
  10.4 W2A list.
- 12:3x rule-39 incident (mine): following rustc's "unnecessary qualification" lint I unqualified `crate::dsl::HistoryPageStack` in TT
  1884/1925 under the freeze → E0425; reverted, **PLUGIN GREEN 12:36** (`check-plugin-14.txt`, the one check the coordinator allowed).

### 10.6 State at 12:40 (cargo frozen, rule 44) — open items and coordinator actions

Done in source this session (verified by checks where named): W2A-1 incremental history view + O(change) backfill + per-edit slices +
exact-row wire; W2A-2 L4 verified green by laws (10.4); W2A-3 one wall deadline per turn; W2A-8 (kernel status copy, pause/fault
lifecycle, `history.step-blocked`); W2A-9 busy reason; W2A-10 placeholder row; W2A-11/D13 worst over the whole edit + member-only paging;
W2A-13 interaction exemption; W2A-14 canonical pointer indices; filter vocabulary; no raw codes; optionSource never an enum; archive-load
actor adoption; streamed-tick phase; law-helper envelope retirement; K3 macros + ten plugin conversions + window-transient notices.

Open (not done):
- **OWED runs (rules 43/44)**: plugin-crate checks of the ten K3 conversions (native + wasip2, command in 10.5); plugin wasip2 lib check;
  plugin `--tests` check; W2A law list (10.4) + `transient_root` law; `cargo test -p semio-framework --lib -- history_patch history_notices
  history_edit framework_notices`; `cargo test -p semio-framework-time-travel`; kernel vitest + FWT TS conformance.
- **D8** turn-level two-instance cold-pair law: not written (needs a reactor-turn harness feeding two lifetimes' pages; cannot be compiled
  under rules 43/44). The serialization/supersession code it proves is on disk since 9.9.
- **D7** publication gate: held for S4-PUZZLE's clean thread-local census numbers (coordinator relay 02:50).
- **D13 residual**: a row with BOTH a parent edit and member edits pages only its parent edit past the projection (CommandView carries no
  member op count).
- **W2A-12** (`tool_intent_kinds` gets the bare tool id): not changed — touches gen3d + flow adopters; proposal for `main`.
- Puzzle 2d/3d/5d, note and sequence window transients use the same hand-written pattern (not in §21.5's list).

Coordinator actions:
- Describe wave must regenerate: hub descriptors (old `withoutOperations/onlyOperations` filter options), lowpoly transient leaf emoji
  (🖌️ → 🫧️), and every converted plugin (owner/descriptor unchanged otherwise).
- At CARGO OPEN: let S4-RUNTIME run the OWED list above first (K3 crates native + wasip2 → COMPOSITION GREEN per plugin).

## Session 5 — 2026-10-05

S5-RUNTIME (successor of S4-RUNTIME, agent `ab0af284ec8750474`). Scratch: `🗑️generated/s5-runtime/`. Priorities per rule 54:
live-probe findings → §22.1 / §22.2 / §22.6 / §22.7 guest half → owed verification → inherited open items.

### 11.1 Repair (rule 46) — nothing half-written (00:24)

- `git diff --stat HEAD` over `TT`, `FW/⏪️time-travel`, `🔌️plugin/🧪️tests/🧪️time-travel`, `🧫️fixtures/🧫️time-travel`,
  `🪟️window/🫧️transient`, `FW/🎠️kernel`: **no difference to HEAD** (commit 670, 23:07). `PLG` differs in 32 hunks
  (12901–13395 Emit, 20283, 21147, 28475, 31128, 32223) — none in my regions (`🔖️CommandLog`, `🔖️HistoryPanel`, history view,
  K3). So the session-4 source of 10.2–10.5 is what HEAD holds; nothing to repair.
- Launch state: `landing` + `serve` HELD by `COORDINATOR-ACTIVATION` (00:14) → no saves under `🧰️framework/**`; waves are
  prepared under `🗑️generated/s5-runtime/` and land one at a time under the lock (rule 51).

### 11.2 Contracts sent to `main` before landing (00:3x)

1. **§22.2 wire shape**: `HistoryPatch.timeTravel.nextProblem?: { mutationId: string, store?: string }` (the args of
   `historyEditBegin`); kernel `HistoryTimeTravel.next_problem: Option<HistoryTimeTravelProblem { mutation_id, store }>`;
   present iff the session's report blocks finalizing. A keyed value-map field (default + skip-if-none) — **no channel frame
   layout change** (the status is named nowhere in `📡️spr/🧵️channel`), nothing for wave B.
2. **§22.1 verb shape**: no 13th verb — `historyEditWithdraw{mutationId?, store?, generation?}`; with `mutationId` it is the
   row action. Wire row gains `HistoryMutationEntry.withdrawable?: boolean`.
3. **§22.1 STORE LAW GAP** (S5-STORE / coordinator decision): `admit_replacement` (`STORE` ≈ :23705) refuses every
   supersession of an operation that plans foreign steps, `Withdrawn` included (it folds as a `mutation.invariant` Fatal). A
   foreign-step blocker is therefore NOT resolvable by a withdrawal today. The row predicate asks the store's own law, so it
   follows the store the moment it admits such a withdrawal.
4. **§22.7 names** asked of S5-UI: `ActionArgControl::Multiline` → `input(InputKind::LongText)`; `Segmented` →
   `select(..)` + a contract facet `SelectAppearance::Segmented`; `IconSelect` → a contract builder `icon_select(..)`
   (`Component::IconSelect` has props but no builder).

### 11.3 Coordinator decisions received (00:3x–00:5x) — binding for the waves below

- `nextProblem?: { mutationId, store? }` confirmed; band order while blocked `[nextProblem, rerun, finalize, exit]`, label key
  `ui.timeTravel.nextProblem`; **the kernel field + schema + TS twin land FIRST** (S5-UI's corpus validates against it).
- `historyEditWithdraw{mutationId?, store?, generation?}` + `HistoryMutationEntry.withdrawable?` approved; I own the manifest
  `🔖️HistoryEdit` hunk. `rowActions = [Edit?, Withdraw?]` (Edit stays index 0); I add `withdrawable` to the three literals of
  other WPs' tests (wgpu `🧪️wgpu-time-travel` :822 / :1304, plugin `🧪️composed-child-history` :27).
- Store law (S5-STORE, design note in `📓️w1-g-report.md`): `Withdrawn` is admitted for every operation; a foreign-step unit =
  every applied op with its `MutationMeta.group_id`; STORE lands `unit_operations(mutation_id)` / `unit_id(mutation_id)`; my
  Withdraw row drafts `unit_operations(target)` together and `editable` is false inside a unit. Cross-instance units refused by
  the store with a localized reason. The row predicate keeps asking the store's own law.
- Two codes + one label approved: `timeTravel.not-withdrawable` (`refusalNotWithdrawable`), `timeTravel.editor-closed`
  (`refusalEditorClosed`), `refusalReadOnly`; reducer event `beginWithdrawn`.
- **Restore (decision 5):** an accepted draft is individually revertible — reducer event `Restore{target}` (legal in
  `Reviewing`; removes the target from `accepted`, replays the rest, or leaves time travel with zero trace when it was the
  only draft), verb `historyEditRestore{mutationId, store?, generation?}`, row action "Restore" / "Wiederherstellen" that
  replaces Withdraw on a row whose mutation has an accepted draft. Fixture + TS twin first.
- Guest half of "reveal + scroll": status `nextProblem`, stable row key, entry open, blocking rows first among an entry's
  children. Hosts scroll.
- P4 names confirmed by S5-UI (it lands contract + manifest first): `ActionArgControl::Multiline` →
  `input(InputKind::LongText)` + `commit("blur")`; `Segmented` → `select(value).appearance(SelectAppearance::Segmented)`;
  `icon_select(value, classifier_kind)`. S5-PUZZLE owns one hunk in `history_code_text` (`mutation.precondition-drifted`,
  `mutation.inverse-refused`) — not mine.

### 11.4 Landing mechanism and staged waves (lock held by the activation since 00:14)

- `🧪️s5-runtime-land.py <check|land|revert> <wave>` (ticket input): exact-anchor patches (each anchor must occur once in the
  live file) or whole-file producers per wave module `🧪️s5-runtime-wave-<x>.py`; `check` writes the result under
  `🗑️generated/s5-runtime/wave-<x>/after/` and touches nothing; `land` keeps every replaced file under `…/before/` (rule 51
  rollback) and writes the tree; `revert` restores.
- **Wave A — STAGED, schema-verified** (`🧪️s5-runtime-wave-a.py`): kernel `HistoryTimeTravelProblem` +
  `HistoryTimeTravel.next_problem` (`FW/🎠️kernel/🦀️.rs`), schema `🧬️schema/🔣️history-patch` (`HistoryTimeTravelProblem`,
  `nextProblem`), TS twin, wire fixture (blocked case names its problem; new valid case with a member store; two invalid
  cases), `TT` (`TimeTravelLedger::next_problem`, the one predicate `ReplayReport::blocks_finalize` reads, feeds status and
  panel). Ran: python `jsonschema` over the staged schema + fixture — 14 valid accepted, 13 invalid refused, fixture schema ok.
- **Wave B — STAGED, TS-verified** (`🧪️s5-runtime-wave-b.py` + generator `🧪️w1-b-generate-lifecycle-law.py`, which also
  regains the `memberEdited` label it had lost): pure session `beginWithdrawn{target, original}` (one `open` path shared with
  `begin`; legal exactly where `begin` is) and `restore{generation, target}` (`restore_refusal`; `Reviewing` + accepted draft
  → replay the rest, or close when it was the only one), codes `timeTravel.not-withdrawable` / `timeTravel.editor-closed`,
  label `refusalReadOnly`; fixture 18 → 20 event keys, 24 → 27 guards, 22 → 23 contexts, 130 → 145 matrix rows, 45 → 57 cases,
  10 → 14 scenarios, 37 → 40 labels, 18 → 20 codes (every previous row kept, checked by script); schema counts; Rust reducer +
  unit laws (3 new); TS twin + conformance (3 new laws; xstate guards/assignments for both events).
  Ran: `bun test ./…/wave-b/after/…/🧪️tests/🧪️conformance/🟦️.ts` on the staged tree → **22 pass / 0 fail** (ajv fixture
  schema, xstate agreement on every matrix row, fast-check random sequences, pinned seed visits every legal row incl. the new
  ones). Rust half compiles and runs only at landing.
- **Wave A also carries** (coordinator 00:5x) the kernel history notice row `history.unit-spans-documents` (S5-STORE's
  cross-document unit refusal; `HISTORY_NOTICE_LABELS` 7 → 8, TS twin, `🧫️history-notices` fixture) so the store finds it.
- **Wave C — STAGED, partly verified** (`🧪️s5-runtime-wave-c.py`, 16 files, applies on A + B; `check c a b`):
  - Manifest `🔖️HistoryEdit` (Rust + TS twin + `🧫️history-edit-actions` fixture + its schema): `historyEditWithdraw` args
    `[mutationId?, store?, generation?]`; 13th verb `historyEditRestore{mutationId, store?, generation?}` after
    `historyEditDiscard` (icon `undo-2`, "Restore Mutation" / "Mutation wiederherstellen").
  - Kernel wire `HistoryMutationEntry.withdrawable` (Rust, schema, TS twin, fixture: one valid row, one invalid case).
  - `TT`: `TimeTravelStoreCommand::Open{withdraw}` — a row's Withdraw asks the STORE's supersede law
    (`time_travel_admits_withdrawal` = `store::admit_replacement(op, Withdrawn)`), needs no input schema and opens an editor
    without inputs (`schema: Option`); `begin_time_travel(withdraw)` sends `BeginWithdrawn`, or the editor's own `Withdraw`
    on the mutation already being edited; `historyEditRestore` → `Restore`; refusals `NotWithdrawable` / `EditorClosed`; both
    `.expect(` of the user-input path are gone (§22.6: `open` asks the schema as data, a draft whose editor vanished answers
    `timeTravel.editor-closed`, a draft on a mutation without inputs `timeTravel.not-editable`); the editor's withdrawn state
    is the session's pending draft (the stored flag was never set by the editor's Withdraw — a latent bug: the heading never
    read "Withdrawn" and the button stayed enabled); panel `accepted` set + `editable`; `MutationView.withdrawable`;
    `mutation_row_refusals` (one place for the Edit / Withdraw / Restore refusals of a row, incl. a session on another store).
  - `PLG` `🔖️HistoryPanel`: row actions `[Edit?, Withdraw | Restore]` on the one row target, each disabled with its localized
    reason (viewer → `refusalReadOnly`, store law → `refusalNotWithdrawable`, session → its refusal, busy); blocking rows lead
    an entry's children (guest half of "reveal + scroll", §22.2).
  - Laws: plugin scenario fixture 9 → 15 scenarios (`withdrawRow`, `restore`, `status.nextProblem`) + schema + Rust driver +
    TS oracle; `a_mutation_row_offers_withdraw_and_restore_takes_an_accepted_draft_back` (app level, rendered rows + verbs);
    `a_blocking_mutation_without_editable_inputs_is_withdrawn_and_the_review_becomes_ready` (store-level, over
    `InertLabelOp`: a kind without input schema that an upstream edit breaks → not editable, withdrawable, review ready,
    one overwrite).
  - Other WPs' literals (approved): wgpu `🧪️wgpu-time-travel` (`withdrawable`, panel `accepted`, editor `editable`), plugin
    `🧪️composed-child-history` (`withdrawable`) — edited by me, NOT compiled by me (wgpu `--tests`).
  - Ran on the staged tree: python `jsonschema` — kernel wire 14 valid / 14 invalid as expected, manifest fixture valid (13
    verbs), plugin scenario fixture valid; `bun run …/run-plugin-oracle.ts` → `time-travel-scenario-oracle cases=15` (the
    staged scenarios through the staged reducer twin). Rust compiles and runs only at landing.
- **LANDED 01:16–01:23 (landing + serve held 7 min): waves A + B.** Ran on the landed tree:
  `CARGO_BUILD_JOBS=3 cargo check -p semio-framework-time-travel -p semio-framework -p semio-framework-plugin --lib
  --message-format=short` → **exit 0** (`check-wave-ab-1.txt`; plugin lib 283 warnings = type-checked);
  `bun test ./🧰️framework/🔨️modules/⏪️time-travel/🧪️tests/🧪️conformance/🟦️.ts` → **22 pass / 0 fail**;
  `bun test ./🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️history-patch/🟦️.ts` → **3 / 0**; `…/🧪️history-notices/🟦️.ts` →
  **2 / 0**. OWED for A + B (test builds): `cargo test -p semio-framework-time-travel` (Rust reducer against the new
  fixture) and `cargo test -p semio-framework --lib -- history_patch history_notices`.
- P5 started while the lock was held (01:01–01:08, gate v3): the owed K3 native batch (command of §10.5) → **exit 101**, one
  error, NOT K3 and not mine: `✏️s/🔌️plugins/🌊️flow/…/🎮️commands/🩹️patch-flow-widgets/🦀️.rs:12:45 E0432 unresolved import
  semio_framework_plugin::ChildEmitPreparation` (the type is exported only inside `semio_framework_plugin::app`, PLG:384, a
  peer hunk newer than HEAD; told `main`). fem-2d and remodel compiled (warnings only) before cargo stopped; the other
  K3 crates need the batch again with `--keep-going` (`check-k3-native-1.txt`).
- **LANDED 02:16–02:25: wave C** (landing 9 min; serve 02:24–02:25 for the two bundled TS twins, wave `c-ts`), with
  three additions made while waiting for the lock: design §22.20 (a mutation is editable only when its payload schema shows
  at least one input row — `time_travel_schema_shows_inputs`, memoized per schema; `open` refuses an editor with zero rows;
  the row ALWAYS names Edit, disabled with "The inputs of this mutation cannot be edited" / the viewer reason), §22.7 guest
  arms (`SelectAppearance::Segmented`, `icon_select`, `InputKind::LongText`; a text over `UI_TEXT_MAX_BYTES` is read-only so a
  commit never writes it clipped — label `TooLong`), and S5-UI's drafted law `long_option_rows_state_their_choice_as_selected`
  adopted with the new struct shapes. Row actions are now `[Edit, Withdraw | Restore]` (a withdrawn row: `[Edit]`).
  Ran on the landed tree: `CARGO_BUILD_JOBS=3 cargo check -p semio-framework -p semio-framework-plugin --lib` → **exit 0**
  twice (`check-wave-c-1.txt`, `-2.txt`; plugin lib 283 warnings = the baseline after I unqualified 5 new paths);
  `cargo test -p semio-framework-time-travel` → **16 passed / 0 failed** (`test-fwt-1.txt`, after waves A + B);
  bun: conformance **22/0**, manifest `🧪️history-edit-actions` **3/0**, kernel `🧪️history-patch` **3/0**, plugin scenario
  oracle `cases=15`.
- **02:33 RED OF MINE, tests only** (`check-plugin-tests-1.txt`, exit 101): 2 × E0502 in my new law
  (`verb(&mut app, …, seeded_mutation(&app, 0))`, `🧪️tests/🧪️time-travel/🦀️.rs:2206/:2220`). Repair = wave `c-fix`
  (3 lines, tests-only), told `main` at 02:34; the fleet was cut at ~02:40 before a lock came. **Saved 04:24** on the
  coordinator's order (no lock: tests-only file). Its verifying check (`check-plugin-tests-2.txt`, started 04:24:03) sat
  childless in the post-prune cargo flock cycle for 21 min (1.56 s CPU) → killed by me at 04:45 (rule 63), exit 143 = NO
  VERDICT. `foundation.status` = `BUILDING` (S5-INFRA warms the closure alone; its pass includes plugin `--lib --tests`).
  **OWED until GREEN**: `CARGO_BUILD_JOBS=3 cargo check -p semio-framework-plugin --lib --tests --features
  artifact-app-testing --message-format=short`, then `CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s5-runtime
  cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- a_mutation_row a_blocking_mutation
  long_option_rows every_fixture_scenario`.
- Live probe findings against my trees (rule 54 priority 1):
  - F3 (React/en batch G: Edit natively `disabled` with no reason while `replaying`): **not guest** — the guest has one path
    for every refusing stage (`render(history)` passes `time_travel.panel()` for every non-inactive stage; `begin_refusal` is
    `Illegal` in replaying / choosing / finalizing alike) and can only disable a row action through
    `RowAction::disabled_because(reason)`; the stage edge and the body's dirty scope leave in the same reply. The probe's
    timeline shows the button it PRESSED at t=150 still natively disabled at t=803 in `reviewing`, where the guest offers
    Edit again → a host in-flight state of the row action button that hides the guest's reason and is not cleared after a
    `{rejected}` reply (S5-UI). Told `main` 04:46.
  - O3 (transient `timeTravel.stale` / `timeTravel.illegal` notices on Discard-then-Begin): a late blur-commit of an editor
    field after Discard is answered `timeTravel.stale` by design (§4: a silent no-op) — a host must not surface `stale` as
    a notice (S5-UI); for `illegal` E2E has to name the press.
  - Withdrawn by the coordinator (probe timing faults): "Edit not disabled behind the finalize prompt", "no undone/redone
    rows". Live on record before the cut (B0 guest, React/en): batches A 72/0, B 67/0, C 48/1, D 44/0, G green but F3.
  - **F4** (first two-peer run, React/en batch H: A edits a draft, B's edit arrives through the folder; A never lists it,
    one `timeTravel.stale`). Store + route halves are S5-STORE's / S5-LOAD's (the folder route replaced the store under the
    session; a replaced document exits the session by design, which is where the generation changed). SESSION half,
    decided with the coordinator (design §22.24): (a) a base move while `Editing` keeps the generation (reducer law
    `editing × baseMoved`), so the editor's controls stay valid; (b) the remote edit is listed downstream "Not applied
    while editing"; (c) an open session does NOT pause remote adoption — a tail edit is adopted at ingest, a change that
    needs a replay shows "Replaying a remote history change: n of m", "paused" only after the user cancelled one;
    (d) Accept replays the remote edit too. No runtime law covered a remote edit while EDITING (only while reviewing) →
    **wave D, STAGED** (`🧪️s5-runtime-wave-d.py`): law `a_remote_edit_while_editing_keeps_the_draft_and_accept_replays_it_too`
    (two toy replicas over memory backbones; asserts (a)–(d), an input stamped with the pre-move generation applies, and the
    overwrite converges on the other replica); one real bug fixed: the pending rows were counted from the position the
    EDITOR recorded when it opened, so a remote edit sorting before the edited mutation made the edited row itself read
    "not applied" — a base move now re-resolves the editor's position with the session's (no law pins the upstream case:
    the toy harness cannot order a remote edit before a local one); `open` asks the store's `unit_id(target)` (Withdraw of
    a cross-document unit op → the store's `history.unit-spans-documents`, Edit → `timeTravel.not-editable`; const
    `HISTORY_UNIT_SPANS_DOCUMENTS_CODE` in `PLG` `🔖️CommandLog`). Asked S5-STORE (approved, relayed) for
    `AppliedMutation.unit` on the applied row so `editable` / `withdrawable` can read it per row without re-deriving store
    law (`unit_id` re-locates by a linear scan per call).
- **PLUGIN TESTS GREEN 04:55:59** (S5-INFRA's warm pass) and by my own run 04:56:46–04:56:47:
  `CARGO_BUILD_JOBS=3 cargo check -p semio-framework-plugin --lib --tests --features artifact-app-testing` → **exit 0**
  (`check-plugin-tests-3.txt`). Targeted laws started 04:57:14 (`test-plugin-laws-1.txt`), verdict below when it exists.
- **Owed laws of wave C RAN** (gate v5, 40 GiB free, uplift `target-nde-s5-runtime`):
  `cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- a_mutation_row a_blocking_mutation
  long_option_rows every_fixture_scenario` → **5 passed / 0 failed** (`test-plugin-laws-1.txt`, 05:09). Families
  `-- time_travel supersede history_label_reload history_alternatives ui_history_panel rendering_the_history_body
  composed_child_history` → **65 passed / 2 failed** (`test-plugin-laws-2.txt`, 05:16; session-4 baseline 75/15 on a wider
  filter). The 2 reds = my own law helper `apply_other_edits` (600 edits seeded straight on the store saturate its
  displaced-owner retirement authority of 1024 → `ValidationFailed("… authority is saturated")`), repaired in wave D.
- **K3 native batch RAN** (12 converted plugin crates, `--keep-going`, command of §10.5): **exit 0** at 05:28
  (`check-k3-native-2.txt`, 8 m 28 s cold). OWED: the same with `--target wasm32-wasip2`.
- **LANDED 05:35:52–05:39:55: waves D + E** (landing 4 min; serve 05:38:40–05:38:49 for the served half `d-serve`).
  - Wave D, on top of the staged content above, by coordinator decisions of 05:2x–05:3x: the session's base is the CONTENT
    REVISION alone — `TimeTravelBase { content_revision }` (pure crate, TS twin, schema, generator + lifecycle-law fixture,
    both law drivers, plugin base query, plugin TS oracle); a document port attaching or detaching moves the store's local
    generation and no event, which used to be a `BaseMoved` and a `stale` for the user's next input; law
    `a_backbone_attach_and_detach_while_editing_is_no_base_move`. `row.unit` (S5-STORE's `AppliedMutation.unit`, landed
    05:20): `editable` / `withdrawable` are false inside a cross-document unit; `open` refuses it without a second scan.
  - Wave E (approved 04:5x): `apply_time_travel_event` marks the rendered document dirty only when the identity of the shown
    snapshot changed across the event and its effects; `shown(Replaying)` falls back to the head reviewed last; law
    `a_session_edge_republishes_every_window_only_when_the_shown_document_swaps`.
  - Ran on the landed tree: `CARGO_BUILD_JOBS=3 cargo check -p semio-framework-time-travel -p semio-framework-plugin --lib
    --tests --features semio-framework-plugin/artifact-app-testing` → **exit 0** after D (`check-wave-d-1.txt`, 05:38:22)
    and after E (`check-wave-e-1.txt`, 05:39:47); bun conformance **22/0**, plugin scenario oracle `cases=15`.
  - NOT in wave D: "a replay finished before a port change still commits" — needs S5-STORE's wave RB
    (`commit_finished_replay` still refuses `finished.generation != self.generation`, store ≈ :21625). Until RB: a port
    change while REVIEWING → one `timeTravel.stale` finalize fault, automatic re-replay, second Finalize commits.
  - Law runs on D + E: started 05:40 (`test-plugin-laws-3.txt`), verdict below.
- **Law run on D + E, 05:44:27** (`test-plugin-laws-3.txt`, the family filter above): **68 passed / 2 failed**. Every law
  of waves C/D/E passes (row Withdraw + Restore, row refusal reasons, §22.20, `long_option_rows…`,
  `a_remote_edit_while_editing_keeps_the_draft_and_accept_replays_it_too`,
  `a_backbone_attach_and_detach_while_editing_is_no_base_move`,
  `a_session_edge_republishes_every_window_only_when_the_shown_document_swaps`). The 2 reds = the two N17 laws, now past
  the harness saturation and failing at their real assertion `local_step_pending()` ("the interior undo waits for its
  replay").
- **N17 diagnosis — a LAW HARNESS fault, no runtime or store regression.** Both laws were re-written in session 4 onto
  `apply_other_edits` and never ran green since (rule 43, then the saturation). A mutation dispatched by a plain
  `ArtifactCommand::Apply` names no author; the store authors it as `local` (`replay_mutations` ≈ :22224,
  `author_id().unwrap_or("local")`) and takes that actor as its local one (≈ :22061). So
  `set_local_actor_id("other")` + 600 Applies never made "600 edits of another author": they were the fixture actor's own,
  its undo hit the applied TAIL, and the store's tail step (design §20.14, D22) adopts that inside the dispatch — correct,
  O(change), nothing waits. First repair attempt (wave `f`, 05:53): the acting author's edit through the runtime's
  `dispatch_emit` — measured "0 of 605 rows" authored by that actor (`test-plugin-laws-4.txt`, 05:56): the runtime's plain
  emit route is an `Apply` too. Wave `f-fix` (06:00): helper `bury_edit_of(app, author, count)` publishes the acting
  author's one edit through the store's batched publication (`begin_apply_batch(.., actor, ..)` → `advance_apply_batch`
  → acknowledge → close; the retained tool route, the only one that stamps the acting actor), puts `count` one-op edits
  of the fixture actor downstream and asserts exactly one interior history row is that author's. Both laws undo as that
  author.
- **RB commit clause** (S5-STORE's wave RB on disk 05:42: `commit_finished_replay` refuses `Stale` on the content revision
  alone): law `a_replay_finished_before_a_backbone_attach_and_detach_still_commits` — reviewing → backbone attach + detach
  (store generation moved, content revision not) → 4 driver turns: the session is unchanged, no second replay → Finalize +
  Overwrite are accepted, the session closes, the head and every window show the reviewed head. The "one
  `timeTravel.stale`, re-replay, second Finalize" window noted under wave D is closed.
- **Waves `f` + `f-fix` saved 05:53 / 06:00** (tests only, without lock by coordinator order): `🧪️time-travel/🦀️.rs`.
  **Targeted run 06:08** (`test-plugin-laws-5.txt`): `-- an_interior_undo_over_a_long_history a_deferred_history_step_ends
  a_replay_finished_before a_backbone_attach_and_detach` → **4 passed / 0 failed**.
- **Full W2A law list 06:13:50** (`test-plugin-laws-6.txt`): `cargo test -p semio-framework-plugin --lib --features
  artifact-app-testing -- time_travel supersede history_label_reload history_alternatives ui_history_panel
  rendering_the_history_body composed_child_history transient_root` → **74 passed / 1 failed**: the whole W2A list is
  green (71 = 68 + the 2 N17 laws + the RB clause), and of the four K3 `transient_root` laws (first run ever) three pass.
  The red, `the_snapshot_mutation_replaces_the_whole_root_and_inverts_to_the_base`
  (`🪟️window/🫧️transient/🧪️tests/🧪️transient-root/🦀️.rs:52`), is my own law's last clause "unknown members are refused":
  no contract carries it — `transient_root!` parses with `JsonMemberPolicy::Reject`, the authority over REPEATED member
  names (`🎒️pack/🔤️json/🦀️.rs` ≈ :1806 `DuplicateMember`), and a member the root does not declare is read past by the
  value derive for every root alike. Every assertion before it passed (wire text, text and binary round trip, diff,
  inverse, descriptor). Wave `g` (tests only, CHECKED, NOT LANDED — stand-back for wave B since 06:12) states what the
  macro promises: a repeated member is refused. OWED after "LOCKS OPEN": `python3 🧪️s5-runtime-land.py land g`, then
  `cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- transient_root`.
- **Observation handed to S5-STORE through `main`** (measured, not mine to change): the runtime's legacy emit route
  (`dispatch_emit` → plain `Apply`, `PLG` ≈ :28825) authors every edit as `local` whatever `meta.actor` is; only the
  retained batch route stamps `meta.actor`. A document verb still on the legacy route in a two-peer session would make
  `undo` (`edit_is_local`) find nothing of its own (host actor ≠ `local`) or take the peer's tail edit (both `local`).
- Observations for a later wave (not done): (a) [DONE in wave E] any session effect marks the rendered document dirty, so RequestFinalize /
  Back / Choose / Accept-into-replay answer `UiDirtyScope::Full` although the document a window shows does not change —
  narrowing to the shown-slot change (committed ↔ preview ↔ replayed head) would re-publish only the history body on those
  edges (the ~450 ms the probe measured on the `choosing` edge); (b) `TimeTravelStoreState::shown(Replaying)` is the draft
  preview only — a replay started from `Reviewing` (Rerun, Restore, BaseMoved) has none, so windows fall back to the
  committed document until the new head arrives.
- Store law since S5-STORE's landing (read 04:45): `admit_replacement` admits `Withdrawn` for every operation;
  `ArtifactStore::unit_id` / `unit_operations` exist; a cross-document unit is refused at authoring
  (`VcsError::UnitSpansDocuments` → `history.unit-spans-documents`). My row predicate follows the admit law automatically;
  the unit half is the follow-up below.
- Unit withdraw (S5-STORE `unit_operations` / `unit_id`): NOT in wave C — the API is not on disk. Today every product
  operation is its own unit, so the single-target path is complete for them; a real multi-op unit would be refused by the
  store's authoring law at finalize (`timeTravel.commit-failed`) until the follow-up wave expands a withdrawn draft to
  `unit_operations(target)` and makes `editable` false inside a unit.

### 11.5 State at 06:20 — stand-back for wave B (S5-CHANNEL holds all five locks since 06:12; no cargo, no landings)

On disk and verified by runs named in 11.4: waves A–E (runtime, pure session, kernel, manifest, fixtures, twins) and the
tests-only waves `c-fix`, `f`, `f-fix`. Plugin `--lib --tests --features artifact-app-testing` check exit 0 (05:39:47);
W2A law list 71/0 inside the 74/1 run of 06:13:50.

OWED — every step is one call of `zsh 🧪️s5-runtime-owed.sh <step>` after "LOCKS OPEN" (gate v5 inside the script):

| step | what | why owed |
| --- | --- | --- |
| `g` | lands tests-only wave `g` (K3 law clause: a repeated member is refused) | checked 06:15, not landed (stand-back) |
| `transient` | `cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- transient_root` | 3/1 at 06:13; expect 4/0 after `g` |
| `fwt` | `cargo test -p semio-framework-time-travel` | last run 16/0 was before wave D (base = content revision, new corpus rows) |
| `kernel` | `cargo test -p semio-framework --lib -- history_patch history_notices history_edit framework_notices` | never run this session (waves A, C touched the rows) |
| `k3-wasip2` | the K3 batch of 10.5 with `--keep-going --target wasm32-wasip2` | native half exit 0 at 05:28; a wasm32 build needs the coordinator's word on the wasm build mutex |
| `w2a` | the full law list again | only if wave B or the B1 activation touched `TT` / `PLG` history regions |

Not verified by me: the wgpu shell's `--tests` compile of the three literals I edited in
`🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs` (S5-WGPU's build). My uplift dir `target-nde-s5-runtime` holds no executables
(tests link in the shared build dir), so rule 48 leaves nothing of mine to delete.

Coordinator actions: the next activation must re-describe (wave C added the 13th HistoryEdit verb `historyEditRestore` and
changed the args of `historyEditWithdraw`); waves C/D/E reach the browser with build B1.

### 11.6 After the 07:45 usage cut — waves H + I (09:37–10:07)

Repair-first (rule 64): nothing of mine was half-applied. Wave `g` was on disk (07:28); the `transient` build started 07:28
was killed at the cut (`exit=137`, 07:43, no verdict).

**Wave H — an instance always acts as someone (runtime half of S5-STORE's widened P3; LANDED 10:06:37–10:06:50, apply-only
hold, rule 67; NOT compiled by me — the train's verdict in `🗑️generated/coord/train.status` is the check).**
What was missing, read from code:
- nothing gave an instance its actor at open (`plugin_open_actor_instance` kept the admitted actor in the runtime's registry
  only): the document store acted as nobody until the first verb and, after a load, as the author of the loaded history's
  newest edit (the store's initializer takes the tail edit's actor). `deliver_base_moved` answered nothing without a store
  actor, `revertible` read another author's rows, and the routes without an `ActionMeta` (text ingest, media import,
  transaction undo/redo, conflict resolve) authored as whoever wrote last;
- the app constructor applied `A::genesis()` on a store without an actor — with STORE's refusal on disk every app with
  genesis edits would have panicked at construction, hence the order "H before the refusal" (coordinator 10:0x);
- `begin_framework_revert_route` read `revertible` and undid as whoever wrote last; `hydrate_pure_head` reset the store and
  lost the actor;
- two actor sources: `plugin_handle_action` / `plugin_handle_command` read `context.actor` with a literal `local` fallback
  while every other entry point used the admitted `instance_actor`.
Landed (`🔌️plugin/🦀️.rs`, 10 exact-anchor hunks): `LOCAL_ACTOR_ID` (`🔖️CommandLog` region); the constructor binds it to
the fresh document store before genesis (genesis stays authored `local`, explicit and identical on every replica —
coordinator decision); `PluginApp::bind_actor` (default: nothing; `VcsArtifactApp` sets the document store's actor) called by
`plugin_open_actor_instance` right after `bind_instance_id`; `instance_actor` falls back to the const; both `plugin_handle_*`
act as the admitted `instance_actor` (coordinator decision: one actor source); the revert route sets `meta.actor` before it
reads the log; `hydrate_pure_head` carries the actor across its reset. Folder attach / reload already carried it (archive-load
commit; the `pack reload` clause of `a_history_edit_is_its_own_row_locally_remotely_and_after_reload` proves it).
Law `an_instance_always_acts_as_someone_and_every_edit_names_its_actor` (boot, bind, text ingest, plain emit, reload, pure
hydrate, revert, undo). Its ingest and emit clauses need STORE's half (a plain `Apply` authoring as the store's actor).
Left alone, named to `main`: PLG media import ≈ :36066 / :36079 still spell `unwrap_or("local")` on the store's actor (LOAD's
region; harmless with a bound identity); codec `apply_ops` binds `LOCAL_ACTOR_ID` on its bare store in STORE's wave; the
later-turn commit of an authored undo/redo of a finalize (`step_supersede_authoring`) acts as the store's actor at that turn
(the instance's, since nothing else changes it); `revertToCommand` under a foreign tail undoes the target itself instead of
"everything after it" (pre-existing multi-actor semantics of `framework_revert_unit`, no decision asked yet).
Restore: `python3 🧪️s5-runtime-land.py revert i && python3 🧪️s5-runtime-land.py unland h` (`unland` = new verb: takes a
wave's exact-anchor replacements out of the LIVE files and keeps what peers landed since).

**Wave I — tests only, landed with H:** `an_inverse_refusal_is_one_mutations_fatal_that_its_row_names_and_resolves`
(§22.5 + §22.17: Report replay completes; one `Fatal` `mutation.inverse-refused` on the mutation; the operation after it
applies; row line "Fatal: Cannot be reversed" / "Kritisch: Nicht umkehrbar"; withdraw → finalizes),
`hostile_history_edit_input_is_answered_never_panicked_on` (§22.6: 13 verbs × 5 hostile argument sets without a session,
15 hostile calls inside an editing session; an answer every time, stage / document / revision / session untouched);
`InertLabelOp`'s children kind has no inverse under a count below -1; the two `InertLabelOp` laws share `inert_document` /
`close_inert`. §22.6's `timeTravel.editor-closed` branch (TT ≈ :3011) is a guard after an await and cannot be driven through
the verbs; the law covers every refusal a user can reach.

**Owed after the train's GREEN that includes H+I** (`zsh 🧪️s5-runtime-owed.sh <step>`, gate v6 inside, shared build dir):
`transient` (running since 09:51, compiles the tree with H+I), `fwt`, `kernel`, `w2a`, `s22` (the two wave-I laws + the
refactored §22.20 law), `actor` (after STORE's half), `k3-wasip2 1|2|3`.

### 11.7 USAGE STOP 11:30 — the ONE plugin test binary (build running since 11:09:13, not linked when I stopped; resume after 14:20)

- Build: `CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 cargo test -p semio-framework-plugin --lib --features artifact-app-testing --no-run` on the shared dirs; log `🗑️generated/s5-runtime/owed-plugin-test-build-2.txt`. Verdict = its `exit=` line; binary path = its `Executable unittests … (<path>)` line (expected `<shared target>/debug/deps/semio_framework_plugin-<hash>`; NOT seen by me). No `exit=` line = killed, NO VERDICT: re-issue once.
- Invocation (not run by me): `cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust && CARGO_MANIFEST_DIR="$PWD" <path> <filter…>` with whatever `[env]` the repo's cargo config sets for tests (check it; I did not).
- RUNTIME: `transient_root` · `an_inverse_refusal_is_one_mutations_fatal hostile_history_edit_input a_blocking_mutation_without_editable_inputs` · `an_instance_always_acts_as_someone` (ingest + emit clauses red until STORE's P3 is on disk) · W2A `time_travel supersede history_label_reload history_alternatives ui_history_panel rendering_the_history_body composed_child_history`.
- LOAD: `folder_reload_route a_merge_archive_command document_backbone` · TOOLS: `gesture_laws` · GATES: `a_blocking_ledger_replay` · NESTED: its P1 / registry / recursive filters · CHANNEL: its funnel law (their names are theirs; not known to me).
- The 10:48 build is NO VERDICT (interim tree, the 11 `ChildPackEntry.owner` / arity errors of the 10:56 train RED, none in my files). Nothing of waves H / I / g has a test verdict yet.
- Staged, not landed (B3, after B2 is live): wave J + `j-serve` (§22.33; TS conformance 23/0 and kernel fixture 14 valid / 15 invalid on an overlay, Rust half not compiled), wave K (`.expect` → refusal). `🧪️w1-b-generate-lifecycle-law.py` is already edited for J: do not run it standalone before J lands (it would write the new corpus under the old reducer).
- Owed after the reset, in order: the filters above, `zsh 🧪️s5-runtime-owed.sh fwt`, `… kernel`, `… k3-wasip2 1|2|3`; then J + K in one `landing` + `serve` hold.

### 11.8 After the reboot — shared plugin law run, wave H2, owed laws (16:18–16:43)

Repair-first: H + I + g on disk, J + K staged only (checked 16:18). The 11:09 build had linked at 11:30 (`exit=0`); the
tree moved since, so I built again.

- **ONE plugin test binary** (`CARGO_BUILD_JOBS=4 RUST_MIN_STACK=268435456 cargo test -p semio-framework-plugin --lib
  --features artifact-app-testing --no-run`, 16:19–16:23, exit 0; rebuilt 16:31–16:38 after H2, exit 0):
  `.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-framework-plugin/b80951f2dbb7024a/out/semio_framework_plugin-b80951f2dbb7024a`.
  Every owner's filters ran on it through `🧪️s5-plugin-law-run.py`; table and attribution notes: `📓️s5-plugin-law-run.md`
  — **142 passed / 11 failed** over the owner rows (1 mine and expected, 1 LOAD, 8 NESTED / tool-run family, + 1 overlap).
- **My verdicts (second run, tree with H2):** `transient_root` 4 / 0 (wave g) · wave I's laws + the §22.20 law 3 / 0 ·
  `an_opened_instance_acts_as_its_admitted_actor_across_reload_and_on_the_revert_route` 1 / 0 · `w2a` 79 / 1 ·
  `time_travel` module 66 / 1. The single red is `every_route_authors_as_its_acting_actor` — the store half, expected
  until S5-STORE's §22.34 ("a route without an actor of its own authors as the instance's": got `local`, wanted `ada`).
- **Wave H2 (fix-forward of H, landed 16:31:02, 2 files, restore `python3 🧪️s5-runtime-land.py revert h2`).** The first
  run showed nothing of mine red but the expected clause, and eight foreign reds whose shape ("work never ends", "store
  reached Drop without its witness") made me read what H does to a fixture: H gave every constructed document store the
  actor `local`, and `replace_local_actor_retained` retires a replaced actor string as a displaced owner — so every
  directly constructed app gained one displaced owner on its first verb, and an opened instance one at its bind. Fixed:
  the constructor binds nobody; genesis alone is authored `LOCAL_ACTOR_ID` (the state a genesis `Apply` left before H);
  `plugin_open_actor_instance` binds the admitted actor (nobody → actor, nothing displaced). The foreign reds did NOT move
  (NESTED 39 / 8 before and after), so they were not H's; the fix stands on its own reason. The actor law is split:
  runtime half (bind, reload, pure hydrate, revert route, undo — edits authored through `publish_as`, the batched
  publication) and store half (text ingest + plain emit). The first run's law had died at its ingest clause, so the
  clauses behind it had never executed; they are green now.
- **Owed laws RAN:** `cargo test -p semio-framework-time-travel` → **16 passed / 0 failed** (16:41, `owed-fwt.txt`);
  `cargo test -p semio-framework --lib -- history_patch history_notices history_edit framework_notices` → **14 passed /
  0 failed** (16:42, `owed-kernel.txt`).
- **Still owed:** K3 batch `--target wasm32-wasip2` (`zsh 🧪️s5-runtime-owed.sh k3-wasip2 1|2|3`, on the coordinator's word
  for the wasm mutex); waves J + `j-serve` + K (B3) with `fwt`, `kernel`, bun conformance and the plugin law
  `accept_needs_a_change…` after them; §22.31 (iii) after S5-PUZZLE's declaration. The train lanes were not running at
  16:31 — H2's lib half has my test build (exit 0) as its only check.

### 11.9 Wave L — shared schema documents in every instance's resolver (design §23; 2026-10-06 01:25–01:41)

- **Cause.** The runtime resolver `manifest::registered_input_schema_document` reads two process-wide lists: the
  `schema://` export registry and the documents an app's own leaves declare (`Mutation::INPUT_SCHEMA_DOCUMENTS`, registered
  by the app constructor). The framework's shared documents entered neither on their own: nobody published
  `framework/value/schema.json` (the manifest reader embeds it only for its numeric-transport shape test), and the store's
  and io vocabulary's documents were published only by a plugin ASSEMBLY
  (`PluginRuntimeRegistry::publish_declared_catalogs`) — an instance constructed without one held none of them.
- **Fix (landed 01:37:18, apply-only hold, train line; restore `python3 🧪️s5-runtime-land.py unland l`).**
  `🛂️manifest/🦀️.rs`: `FRAMEWORK_INPUT_SCHEMA_DOCUMENTS` (the value schema) is part of the resolver itself.
  `🔌️plugin/🦀️.rs`: `PluginRuntimeRegistry::publish_framework_schema_documents` (store child / owner / link / blob + io
  vocabulary; the catalog tolerates exact duplicates) runs at every app construction; the assembly calls the same function.
- **Always present now:** `framework/value/schema.json` (every process); `framework/io/schema.json`,
  `os/store/child/schema.json`, `os/store/child/owner/schema.json`, `os/store/link/schema.json`,
  `os/store/blob/schema.json` (every instance). Other framework / os schema ids (`kernel`, `interaction`, `manifest`,
  `tool-run`, `time-travel`, `os/store/schema.json`, …) are contracts, not input vocabularies, and are not published.
- **RAN:** the one allowed `CARGO_BUILD_JOBS=3 cargo check -p semio-framework-plugin --lib` → exit 0 (01:41, 3 m 43 s,
  warnings present = type-checked; `check-wave-l.txt`). It compiles the manifest (`semio-framework`) and the plugin lib.
- **NOT RUN (no test build this turn):** law `an_input_that_references_a_shared_framework_schema_resolves_on_any_instance`
  (`🧪️tests/🧪️time-travel/🦀️.rs`, region `🪣️SharedSchemaDocuments`) — neither compiled nor executed; and the raster
  acceptance law `history_edit_inputs_resolve` that found the gap. OWED: rebuild the plugin test binary, then
  `python3 🧪️s5-plugin-law-run.py <binary> RUNTIME` plus the filter `an_input_that_references_a_shared_framework_schema`.
- **Noted for later (U-o1, live journey):** the generic `index` stepper has min 0 and no max, so a step past the collection
  is caught only by the replay (Fatal); the reference / index control should carry the collection's bound when the
  descriptor names the collection.
