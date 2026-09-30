# 📓️ W2-C — wgpu shell: time-travel band, indicator, chords, finalize prompt, staged editors

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, packet W2-C. Contract: `📋️design.md` §7 and §10, plus the additive
wire W2-A relayed later (`review`, `rerunnable`, `historyEditRerun`, `timeTravel.cancelled`, `timeTravel.name-invalid`,
throttled progress patches). Coordinator follow-ups also handled here: the hub refusal codes, the staged-arg editors
(W1-D), the `review`/rerun model, and the dialog choice API change (W1-E).

Aliases:

| Alias | Path |
|---|---|
| `SHELL` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell` |
| `WGPU` | `SHELL/🎯️targets/🧊️wgpu/🦀️.rs` |
| `TT` | `SHELL/🎯️targets/🧊️wgpu/⏪️time-travel/🦀️.rs` (new submodule `time_travel`) |
| `TESTS` | `SHELL/🧪️tests/🧪️wgpu-time-travel/🦀️.rs` (new) |
| `CORPUS` | `…/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band/🔣️.json` (W2-B's, shared with React) |

Status: **done**. The last full compile and the 16/16 run are green (§3); the latest edits are **unverified** (§3.1).

## 1. What the wgpu shell does now

### 1.1 Wire consumption (`WGPU` 🔖️CheckInPure / 🔖️CheckIn, `TT`)

- History rows now fold under `HistoryEntry::key()` (`edit:<id>`, else `seq:<n>`) instead of `seq`. A new
  `history_rows_oldest_first` gives the uncommitted-edit count, and the `🔗️hub-projection-workspace` law, their order
  back.
- `HistoryPatch.time_travel` is picked up from three places:
  - the session-mount snapshot (`seed_history_snapshot`);
  - every non-stale dispatch reply (`observe_invocation_history`, `cursor >= history_cursor`). This also covers the
    uncorrelated progress frames W2-A pushes (`AppFrame::Invocation{in_reply_to:0}`): the ProgramBridge folds those into
    the next exchange.
  - on native, a `read_history` poll at most every 150 ms while the stage is replaying or finalizing
    (`poll_time_travel_progress`, from `pump_sync_events`). It folds the whole snapshot, so an older reply can no
    longer roll the band back.
- A missing status closes the session.
- The first status of a session reveals the History tab (`reveal_dock_tab(FRAMEWORK_PANEL_TAB_HISTORY_ID)`), the twin of
  the tool-run panel reveal. Later statuses of the same session leave a closed panel closed.

### 1.2 Persistent band (`TT`, new chrome phase `TimeTravelBand`)

- **Placement:** bottom-centre, 8 px above the footer, clear of the transient-notice stack at the top. It is painted
  between Footer and Overlay, so an open dialog's veil covers it, and it registers no hits while a modal owns the
  pointer.
- **Model:** a line-for-line port of React's `🛠️ShellHelpers/⏪️time-travel`, pinned by `CORPUS`.
  - Lines, joined with " · ": stage · target · progress · review · worst outcome · fault · accepted.
  - The `review` line comes from `HistoryTimeTravel.review`; a missing report is never read as "no changes".
  - Faults: `timeTravel.cancelled` via `TimeTravelLabel::for_fault`, the `timeTravel.*` and hub `history.*` refusals,
    otherwise "Replay failed (code)".
- **Controls:** each stage offers these, in order, each dispatching its `semio_framework::HISTORY_EDIT_*_ACTION_ID` with
  `{generation}` on the session controller:

  | Stage | Controls |
  |---|---|
  | editing | Accept draft, Discard draft, Exit |
  | replaying | Cancel replay, Exit |
  | reviewing | Replay again (enabled iff `rerunnable`), Finalize… (disabled by empty, blocked or illegal), Exit |
  | choosing | Back, Exit |
  | finalizing | none |

- A disabled control:
  - stays painted, muted;
  - registers a hit with no event;
  - is announced as `disabled`;
  - is described by its refusal text. `ChromeControlPresentation` gained `disabled`, `value_text` and `value_range`
    for this.
- **Accessibility:**
  - The live node `shell.time-travel.status` is a polite `status`, or a `progressbar` with done/total while replaying.
    It is `busy` while replaying or finalizing and speaks exactly the painted message.
  - Chorded buttons get `aria-keyshortcuts` and their inline badge from the remappable table.
- **Tone:** error while the report blocks; warning for a fault, a needed replay or a worst outcome of warning; neutral
  otherwise.

### 1.3 Per-window indicator

- Every pane gets a chip at its free bottom-middle anchor, painted right after the pane chips. It shows a clock and
  "Time travel" / "Zeitreise".
- It is `role=note`, not focusable, and its name is React's `timeTravelIndicatorTextV1` ("Time travel: document before X"
  while editing, else the stage).
- Its id is `framework.window.<id>.timeTravel.indicator`. No pane chip resolves it.

### 1.4 Keyboard

- `SHELL_SHORTCUT_ROWS` has three new rows: `ui.timeTravel.accept = alt+enter`, `ui.timeTravel.discard = alt+backspace`,
  `ui.timeTravel.exit = alt+shift+backspace`. W2-B adopted the same ids and chords in React.
- They are remappable, reserved against app chords, listed in Settings → Keybindings, and run on the async funnel
  (`ShellShortcut::TimeTravel`).
- A chord dispatches only a control the stage offers and enables, and never while a retained text or number field has
  focus.
- No chord is Escape. Escape never reaches a history-edit verb, and in the finalize prompt Escape means Back.
- An open chrome dialog now owns every key in `handle_keyboard_async`, ahead of the focused-chrome, content, shortcut
  and app rungs. Before this, retained content focus could take keys from a modal (the W1-E note).

### 1.5 Finalize prompt

- `Effect::OpenDialog{finalizeHistoryEdit, {name, generation}}` opens the framework-injected dialog from
  `session.app.dialogs`.
- Keyboard alone does everything:
  - typing edits the seeded name;
  - Enter submits `historyEditCommit{generation, name}`;
  - clearing the name gates the submit;
  - Tab, Tab, Enter takes the destructive Overwrite, which dispatches `{generation, choice:"overwrite"}` and needs no name;
  - Escape sends `historyEditBack`.
- For W1-E's new choice API (`requires`, `unresolved_args`, `dispatch_args`) I only changed
  `ChromeDialogRequest::action`, so the crate compiles again. W1-E owns the dialog region from here (told them).

### 1.6 Hub and session refusals

`time_travel::history_refusal_notice` maps the three hub `history.*` codes (error) and the seven session `timeTravel.*`
codes (warning, including `cancelled` and `name-invalid`) to React's exact en/de texts. They surface in three places:

- `ArtifactEvent::Conflict`
- `CommandOutcome::Rejected { reason }`
- `classify_dispatch_fault_notice`

In each case the result is a transient notice carrying the raw code.

### 1.7 Staged editors: Actions form and command palette (`WGPU`)

- `staged_command_arg_row` and `staged_action_arg_row` are unified into one `staged_arg_row` that renders every
  `ActionArgControl`:
  - Stepper → `NumberStepper`
  - Slider and Dial → slider with valid detents as ticks
  - Select
  - Toggle
  - Number with precision
  - Segmented → one pressed Toggle row per option (`option`)
  - Vector → one labelled number row per axis (`index`/`dims`/`tuple`, spliced by `staged_arg_value`)
  - Reference → removable chip buttons ("Remove {item}") plus a "Use current selection" button that stages the live
    `interaction_selection` of the argument's domain and granularity
- A peer (W1-D) has since added typed reference ids (`id_type`) to that arm.
- The copy goes through `LocalizedLabel::native` and matches React's `ui.referenceList.*`.

### 1.8 Staged editors: chrome dialog (`WGPU` 🗨️ChromeDialog)

The dialog has a field per kind:

- number and stepper, with −/+ Nudge buttons and `spinbutton` semantics;
- slider or dial: rail, one tick per detent, arrows step, PageUp/PageDown jump detents, Home/End go to the ends, and the
  pointer uses the shared `slider_pointer_value`;
- select;
- segmented: pressed segment buttons, arrows move;
- toggle;
- one `Axis` field per vector component ("Offset x");
- reference: Enter/Space stages the selection; chips are focus stops of their own (`ChromeDialogStop::Chip`) that
  Enter/Space/Backspace/Delete remove.

Paint ops carry role, pressed and value semantics into the chrome accessibility projection.

### 1.9 History body in wgpu (item 1 of the brief)

The body is the guest's `framework.body.history`, rendered by the retained UI. I added a UI-crate law,
`the_history_editor_controls_project_their_corpus_accessibility`, to `🖱️ui/🧪️tests/🧪️conformance-corpus/🦀️.rs`. For
the five W1-E cases (slider-with-snaps, stepper-precision, vector-input, reference-list, dialog-choices) it checks that
the wgpu accessibility projection keeps each record's liveness and visibility, names and describes every interactive
record exactly as the corpus declares, and gives the number controls their `slider`/`spinbutton` roles. It is
**written but not run** (§3.1).

## 2. Files

- **Created:**
  - `TT`
  - `TESTS`
  - the kept fixture generator, the fixture and the schema I first wrote, all since deleted in favour of W2-B's
    shared `CORPUS`
- **Modified:**
  - `WGPU`, in these regions:
    - check-in fold
    - ShellState fields and init
    - native sync pump
    - chrome phases
    - shortcut table and dispatch
    - keyboard router
    - pane chips
    - staged args and their handlers
    - ChromeDialog
    - dialog step, click and key handling
    - accessible names
    - the `default_dock` doc
  - `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` (dependency `semio-framework-time-travel`)
- **Tests adjusted for the new behaviour:**
  - `SHELL/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs` (3 rows, backspace chord, 31 chords)
  - `SHELL/🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs` (Chip stop name)
  - `SHELL/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` (rows oldest first)
  - `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️conformance-corpus/🦀️.rs` (new law)
- **Taxonomy:** `verify taxonomy report --scope SHELL` finds 77 before and 77 after. The new `⏪️time-travel` and
  `🧪️tests/🧪️wgpu-time-travel` directories add none.

## 3. Verification

All runs were foreground and gated. `cargo test` used `CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-w2-c`.

| Command | Result |
|---|---|
| `cargo check -p semio-framework-os-renderer-wgpu --tests` | ✔ (run `check-5`, before the review/rerun update) |
| `cargo test … --lib -- time_travel` | 16/16 ✔ (before the review/rerun update and the new choice API) |
| `cargo test … --lib -- time_travel dialog` (after the choice-API fix) | 26 ✔, 3 ✘. W1-E's two dialog-fixture laws fail because their fixture's shape changed (W1-E owns them). My finalize law failed because it still expected the name in the overwrite dispatch; I have since fixed it (not re-run). |
| Full `-- shell::` in parallel | 424 ✔, 54 ✘, 233 hung for more than 60 s. I killed my own run after 30 min. |
| The first 27 of those failures, serially | 21 still ✘. Every one reads "retained presented input candidate could not be sealed" or "ArenaFull": process-global UI state leaking between tests in one process, which W1-E already reported. |
| 6 of those, one process each (keyboard ring, document retirement, presented input, bilingual-table law, pointer ownership, board presence) | 6/6 ✔ |

The bilingual-table law was a real regression from my reference-list copy. It is fixed (the copy now goes through
`LocalizedLabel::native`) and passes.

### 3.1 Not verified yet

After the review/rerun update, `cargo test … -- time_travel` did not compile. The cause is outside my files: W1-E is
mid-edit in `🖱️ui/🧬️contract` (`snaps_are_valid` missing, `InputProps.snaps` missing). My last edits are therefore
**WRITTEN BUT UNVERIFIED**:

- the review/rerun band model, pinned by the updated `CORPUS` (11 cases, 10 refusals);
- the finalize law with the generation seed;
- the reference copy through `LocalizedLabel::native`;
- the new UI-crate accessibility law.

Re-run once the contract compiles:

```
until [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt 8 ]; do sleep 20; done
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w2-c cargo test -p semio-framework-os-renderer-wgpu --lib -- time_travel
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w2-c cargo test -p semio-framework-ui --features testkit --lib -- the_history_editor_controls_project
```

## 4. Findings for peers and the coordinator

1. **Remote undo (W2-A item 4; precise account in Follow-up F3).** The wgpu shell has no remote-undo route. React's `shellHistoryUndoRouteV1` is the Hub
   inference-history durable-inverse route, and wgpu has no inference-history owner at all. The body's `undo` therefore
   dispatches `undo` to the guest, exactly as the undo chord does. Porting that route is a separate task.
2. **Progress frames (W2-A item 5; closed by Follow-up F2).** The ProgramBridge (not my file) folds uncorrelated `Invocation` frames only
   during an exchange, so between dispatches only the native poll moves the band. The browser build has no `ReadHistory`
   door; its band advances on the next dispatch reply. A push lane for guest-initiated frames in the wgpu bridge would
   close this gap.
3. **Order-dependent shell laws.** The wgpu shell test module cannot run in one process: dozens of laws fail or hang
   together but pass one per process. This was already there before W2-C.
4. **Typed reference ids in the dialog.** A peer added `id_type` to the staged-row Reference arm, but the chrome
   dialog's `stage_references` still stages strings. W1-E or W1-D should mirror it.
5. **W1-E's own dialog-fixture laws (closed; W1-E migrated them, see F1).** `dialog_choices_*` in `🔬️wgpu-chrome-overlays-tour` must follow their fixture's
   new `seed`/`cases` shape.

## Follow-up (after W1-E's contract change)

W1-E's renames (`slider_snaps_are_valid` → `snaps_are_valid`, `InvalidSliderSnaps` → `InvalidSnaps`) needed no change in
my files: the shell calls `ui_contract::snaps_are_valid` at all four sites, and neither old name occurs anywhere under
`📺️renderer/` or `🖱️ui/`.

### F1. Laws after the contract change

| Run | Result |
| --- | --- |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- time_travel dialog_choices` | **19/19** (17 `time_travel`, including the new F2 law and the pointer law a peer added to my module, plus W1-E's 2 `dialog_choices_*`) |
| `cargo test -p semio-framework-ui --features testkit --lib -- conformance_corpus` | **6/6**, including `the_history_editor_controls_project_their_corpus_accessibility` |

W1-E had already moved the two `dialog_choices_*` laws to the fixture's new `seed`/`cases` shape. Nothing was left for
me there, and I did not touch the dialog region.

### F2. Unsolicited UI-progress frames on the wasm/web build

**Before.** React folds every uncorrelated `AppFrame::Invocation` that carries a `history_patch` through
`subscribeOperationProgress` → `applyHistoryPatch`. The wgpu browser bridge let those frames fall into its
`pendingTurnEffects` leftover lane, so the band only moved on the next dispatch reply.

**Now.** The same frames reach the band on every frame:

- **Bridge (`🧊️wgpu/🐚️plugin-bridge/🟦️.ts`).** The typed-operation drain (`drainTypedOperations`) is the wgpu
  equivalent of React's progress subscription.
  - Every uncorrelated shell frame it does not hand to a caller goes to `stashProgressHistoryPatch(instanceId, frame)`,
    which decodes the `Invocation`, keeps only a non-empty `history_patch`, and queues it per instance, oldest first,
    bounded at 64.
  - The frame still reaches the leftover fold as before.
  - `takeProgressHistoryPatches` drains the queue. It is exposed on `WgpuPluginHandle` and on `WgpuJsBridge`, the latter
    as JSON with the bigint replacer `invocationResponseJson` uses.
  - `releaseInstance` drops the queue.
- **ProgramBridge (`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`).**
  - `ProgramBridgeEntry::take_progress_history_patches(instance_id)` reads the JS door on wasm through
    `take_progress_history_patches_js`, parsing `Vec<HistoryPatch>` with `dsl::os_pack::json`.
  - On native it returns empty: native frames are already folded into the next exchange, and
    `poll_time_travel_progress` covers the gap between exchanges.
  - A `cfg(test)` hook, `install_fixture_progress`, lets a law feed patches.
- **Shell (`⏪️time-travel/🦀️.rs`, `🧊️wgpu/🦀️.rs`).**
  - `pump_sync_events` calls `ShellState::drain_progress_history_patches` every frame. It takes the session instance's
    queue without cloning the entry and returns early when the queue is empty.
  - Each patch folds through `observe_invocation_history`, the same stale-cursor guard a reply uses, so an older
    patch can never roll the band back.
  - The native-only poll state (`TIME_TRAVEL_POLL_MS`, `time_travel_polled_at_ms`) is now
    `cfg(not(target_arch = "wasm32"))`, which removed its two wasm dead-code warnings.

Laws and checks:

| Run | Result |
| --- | --- |
| Rust `unsolicited_progress_patches_move_the_band_between_dispatches` (`🧪️wgpu-time-travel`): empty take is no change; a queued replaying patch moves the band; a stale patch behind it closes nothing; the drain empties the queue | pass (counted in F1's 19) |
| vitest `🧩️package-integration` (`bun ./📜️script.ts test-preview-generated`): new law "queues the history patch of an uncorrelated progress frame and hands it to Rust once, as JSON". It encodes real frames with `encodeAppFrame`/`encodePackValue`; a frame without a patch queues nothing; a take drains; the JS bridge answers `[{"cursor":2}]` / `[]` | **28/28** in the file |
| `tsc --noEmit` on the bridge and `🧩️package-integration` with their whole import closure (`🧪️w2-c-typecheck-progress-bridge.tsconfig.json` in the ticket folder, run with `node_modules/.bin/tsc -p <it>`) | **0 errors** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | ok, no warning in touched code |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` (native) | ok, no warning in touched code |

To make the test file typecheck, `fakeHandle` gained the fields `WgpuPluginHandle` now requires:
`takeProgressHistoryPatches` (mine), plus `readAppDocumentIdentity` and the four `MediaTransportPort` members, which had
drifted before this packet.

Not verified: a live browser run of a replay (no dev servers under the fleet rules).

### F3. Remote undo: why wgpu has no host-side route, and what is missing

**What React does.** `shellHistoryUndoRouteV1(remote, local)` picks `remote` only when a **Hub GIS-inference approval
undo owner** is mounted for the document and is at least as new as local history. It then posts
`inference-history-undo` to the backbone worker, which calls `POST …/inference/gis-map/approval-undos` with a
`GisMapApprovalUndoRequestV1`.

That worker owner (`retainInferenceApprovalUndo` / `bindInferenceApprovalUndoToMountedPair`) exists only after all of
the following:

- an applied `GisMapInferenceApprovalReceiptV1`;
- a **closed-browser-actor** execution-target lease;
- a verified cold pair whose frontier equals `receipt.undo.expectedCurrent`.

In every other state the route is `local` (the guest's own `undo`) or `none`.

**Why wgpu can never reach `remote` today:**

1. **Native: the port never starts.**
   - `ShellState::document_execution_target_lease` is only ever initialised to `None` (`🧊️wgpu/🦀️.rs` ~7273).
   - No code assigns it. The lease the open path resolves goes to `document_host.set_document_execution_target_lease`
     or into the detached `ShellDocumentOpenAnswer`, never onto the shell.
   - So `execution_target_lease_verified()` is always `false`, and `open_inference_port` stops at `LeaseUnverified`
     before any submit.
   - Result: no approval, and so no receipt to undo.
2. **Native: even an approval would lose its undo handle.**
   - `ShellInferenceRunner` folds the receipt through `reduce_gis_map_inference_port_v1`
     (`GisMapInferencePortEventV1::Approval`).
   - That reducer keeps only the phase, job and hash. `GisMapInferencePortStatusV1` has no `undo` field, so the
     `GisMapApprovalUndoHandleV1` is dropped.
3. **No client call.** `DirectoryClient` (`📇️directory/🔌️client/🦀️.rs`) has submit, events, cancel and approval, but no
   `approval-undos` call; `gis_map_inference_call` is private. The only Rust implementation is
   `🌉️mcp/💡️inference::undo_hub_inference_approval`, which runs over the MCP `InferenceHubTransport` and is not a
   dependency of the renderer.
4. **Browser wgpu has no inference port at all.** `GisMapInferenceDriverV1` and `ShellInferenceRunner` are
   `cfg(not(target_arch = "wasm32"))`.
5. **No mount binding.** Native wgpu loads programs natively (`load_resolved_program`). It has neither a
   closed-browser-actor lease nor a verified-cold-pair frontier witness, and React requires both before offering the
   undo.

**Parity as things stand.** For every state wgpu can reach, the remote owner is `null`, so React's own function answers
`local` or `none`. wgpu does the same: the history body's `undo` and the undo chord dispatch `undo` to the guest.

Check-in already goes through the host on wgpu. `framework.checkin` is intercepted in `dispatch_action`
(`handle_checkin_action` → `request_hub_check_in`, a `ShellHubCheckInV1` carrying the head frontier), and a landed
checkpoint also triggers `request_hub_check_in` from `observe_invocation_history`.

**What is missing for a faithful port, in dependency order (owners outside W2-C):**

- (a) Retain the verified native `DocumentExecutionTargetLeaseFieldsV1` on the shell when a hub document opens
  (execution-target-lease lane).
- (b) A retained approval-undo owner. It holds:
  - `receipt.undo` (`GisMapApprovalUndoHandleV1`);
  - a minted 32-hex idempotency key, kept across retries;
  - a history epoch and order;
  - a phase: awaiting-mount, available, submitting, applied or failed/retryable.

  It is bound to the scope, the client instance and the lease fields, and retired on close, rebootstrap or authority
  change. This mirrors the worker's `InferenceApprovalUndoOwnerV1` and needs the receipt surfaced past the reducer.
- (c) `DirectoryClient::undo_gis_map_approval(ctx, scope, &GisMapApprovalUndoRequestV1) -> Result<GisMapApprovalUndoReceiptV1, GisMapInferencePortCodeV1>`
  posting `/approval-undos` (📇️directory owner).
- (d) A native mounted-frontier witness equal to `receipt.undo.expectedCurrent`.
- (e) A Rust twin of `shellHistoryUndoRouteV1` consulted in `dispatch_action` for `undo`, including the `blocked` refusal
  while submitting. Its law should use a corpus shared with React.
- (f) A wasm inference port, if the browser build is to have the route at all.

I added no dead route code: without (a)–(c) it could only ever answer `local`.

### F4. Counts

- **Rust shell laws:** 19/19 (`time_travel` + `dialog_choices`).
- **UI conformance corpus:** 6/6.
- **wgpu package-integration vitest:** 28/28, one of them new.
- **tsc:** 0 errors.
- **cargo check:** wasm32 ok and native ok.
- **New laws this follow-up:** 2 (1 Rust, 1 TS).
- **Remote-undo route:** not implemented. F3 documents why, with six missing prerequisites.


### F5. Open parity gap: peers' history-edit presence (W2-B, presence bit 13)

**What React does now.** It shows a peer's history-edit presence in three places:

- **Roster chip:** a ⏪ badge, and an accessible name using `ui.timeTravel.peer.editingTarget` or `.editingHistory`.
- **History rows:** `framework.history.mutation.<id>` and `framework.history.entry.<seq>` get the description note
  `ui.timeTravel.peer.editingRow`.
- **Publishing:** React publishes the local `toolRun` and `historyEdit` from `AppFrame::Ephemeral`.

**What wgpu has.** The wgpu shell decodes the field (`PresencePeer.history_edit = ephemeral.history_edit`, `🧊️wgpu/🦀️.rs`
~27062). It does not yet render the badge, the chip's accessible name or the row note. I have not checked whether the
wgpu presence heartbeat publishes the local `history_edit`.

**Status.** Not in this packet's brief, so it is left for the coordinator to assign.
