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

**Status.** Done in Follow-up 2 (G2).

## Follow-up 2 (rejection codes, peers' history edits)

### G1. Command rejections are told from their codes

W2-B's unified contract is `CommandAckOutcome::Rejected { code, reason, messages, detail }` with ten closed codes. The
wgpu shell no longer reads the English `reason` anywhere.

- **`ArtifactEvent::CommandOutcome` (`🧊️wgpu/🦀️.rs`).** Every rejection now shows a notice through the new
  `command_rejection_notice(code, &messages, locale)`, React's `commandRejectionNoticeV1`:
  - A `local.*` code is told by its own line (`local_command_rejection_notice`, React's
    `LOCAL_COMMAND_REJECTION_NOTICES_V1`): warning or error, coded with the rejection code.
  - A hub refusal is read from its messages' codes. A history-edit refusal wins. Otherwise `mutation.invariant` or
    `mutation.clamped` is named after "Change refused by the hub"; otherwise only that line is shown, as a warning coded
    `sync.command.rejected`.
  - `Transformed` now shows the info notice "Change adjusted" (`sync.command.transformed`), as React does.
  - Before this change wgpu showed a notice only when the English reason happened to contain a history code.
- **Copy.** The 13 `ui.conflict.*` lines, in en and de at normal terminology, are now in the shell's bilingual
  `shell_chrome_string` table.
- **`time_travel::history_refusal_notice`.** It now matches one exact code only; prose never matches.
- **New `time_travel::history_refusal_of_fault`.** React's `historyRefusalOfFaultV1`. It reads the dispatch-fault string
  (`code: message — code: message [t]; …`, or with the browser bridge's prefix) token by token: the fault code first,
  then the report codes. A code inside a word is not a code.
  - `classify_dispatch_fault_notice` uses it.
  - The band's fault line and `ArtifactEvent::Conflict` already pass exact codes.
- **Law.** `every_command_rejection_is_told_from_its_codes_in_both_locales` runs all 18 rows of W2-B's shared
  `🛠️ShellHelpers/🧫️fixtures/🧫️command-rejection`. Each `rejection` is decoded into the Rust `CommandAckOutcome`, and the
  law checks code, text (en and de) and severity, and that all 10 closed codes are covered.
- **Updated law.** `history_refusals_are_localized_notices_carrying_their_code` now covers exact codes, prose that must
  not match, and three real fault-string shapes.

### G2. Peers' history-edit presence (React parity, W2-B Follow-up 2)

- **Publishing.**
  - Native: the heartbeat already published `history_edit`, and now also publishes `tool_run` from the guest's
    `AppFrame::Ephemeral`, as React does.
  - Browser: `WgpuEphemeralSnapshot` (bridge TS) now carries `toolRun` and `historyEdit` bytes, and
    `browser_ephemeral::observe` (ProgramBridge) decodes them. Before, the browser build published neither.
- **Labelling.** `time_travel::time_travel_peer_presence` is React's `timeTravelPeerPresenceV1`. It labels each editing
  peer from this replica's own `history_entries`, never from wire text:
  - "Ada is editing Drag selection in time travel", or "… the history in time travel" when the mutation is not a
    local row, with badge `⏪`;
  - notes on `framework.history.entry.<seq>` and `framework.history.mutation.<id>` ("Ada is editing this in time
    travel"), in en and de, sorted by key, one key's lines joined by ` · `.
- **Roster.**
  - `footer_presence_rows` gives each editing peer on the attached surface its `activity`.
  - The wgpu `PresenceBar` (`🖱️ui/🧱️elements/👥️PresenceBar/🎯️targets/🧊️wgpu`) gained `PresenceActivity` and
    `PresencePeerRow.activity`, a `peer-activity-badge:<actor>` node, and the badge in the painted chip ("Ada ⏪ · Cy").
  - The new `presence_bar_chip_accessible_text` is React's per-row `aria-label` ("Ada (Ada bearbeitet Skalieren in der
    Zeitreise)"). The footer chip's status node now announces this text.
- **History rows.**
  - The wgpu UI has no host presence overlay, so I added one: `UiTree::{presence_note, set_presence_notes}` and the engine
    `Ui::set_presence_notes`, which fans the notes out to every window's candidate and presented trees.
  - A retained tree row paints `description · note` (`🖌️paint` step 5), and `accessibility_projection` announces it the
    same way. The document is never touched.
  - The shell pushes the notes every frame from `pump_sync_events` through
    `crate::interpreter::set_ui_presence_notes`, which with nobody editing is one empty comparison per window.
- **New language-neutral corpus: `🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-peers/🔣️.json`.**
  - Contents: two history rows and four cases (local mutation, foreign mutation, two peers sharing one row plus a
    non-editing peer, nobody editing), with the expected chips and notes in en and de, copied from React's i18n bundle.
  - React reads it too since Follow-up 3 (H1), so it is the parity contract for both shells, as the band corpus is.
- **Laws.**
  - `the_shared_peer_history_edit_corpus_holds_on_wgpu`: every case and locale.
  - `the_roster_badges_and_announces_an_editing_peer_and_notes_its_rows`: runs through `ShellState`. It checks the
    badge, the painted chip, the announced chip and the published notes, and that a peer on another surface is neither
    shown nor noted.
  - UI crate `peer_notes_are_announced_after_a_tree_rows_description`: mounts the `🌲️tree-row-recipes` corpus document
    and checks the appended description, that an unchanged table changes nothing, and that an empty table restores the
    row.
  - UI crate `an_activity_badges_its_row_and_is_announced_in_the_accessible_name`.

### G3. History-body order (W2-A R2-1)

The body is the guest's `framework.body.history`, built by the shared `🔌️plugin/🦀️.rs` `ui_history_panel`. wgpu renders
that retained document as it arrives, so W2-A's reordering reaches wgpu with no shell change. The band is shell chrome
and stays pinned to React by the band corpus.

### G4. Compile state and counts

W1-E saw the renderer crate broken while `PresencePeerRow.activity` existed before the shell and test literals had it;
that was mid-edit. All of the following were run in the foreground after re-reading every touched file:

| Run | Result |
| --- | --- |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- time_travel dialog_choices` | **22/22** (20 `time_travel`, of which 3 are new, plus W1-E's 2) |
| `cargo test -p semio-framework-ui --features testkit --lib -- conformance_corpus presence_bar` | **15/15** (2 new) |
| vitest `🧩️package-integration` (`test-preview-generated`) | **28/28** (the ephemeral-snapshot test now covers `toolRun`/`historyEdit`) |
| `tsc -p 🧪️w2-c-typecheck-progress-bridge.tsconfig.json` (bridge plus import closure) | **0 errors** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib`, native and `wasm32-unknown-unknown` | both ok, no warning in touched code |

Two things got in the way:

- A peer's in-progress `semio-framework-os-kernel` edits (`🏪️store`, `📡️spr/📜️history`) blocked the first two runs. I
  waited until those files were quiet for four minutes and did not touch them.
- Another peer renamed `mutation.rejected` to `app.command.rejected` in my law file and the dispatch funnel in the
  meantime. My edits keep that rename.

**Not verified:**

- The `Rejected` and `Transformed` event arms in a live sync channel. The tests have no `ShellSyncChannel` harness; the
  pure function is proven by the corpus.
- The painted note pixels. The paint step compiles; the note lookup and the accessibility projection are proven.
- Any browser run.

**Files (Follow-up 2):**

- Renderer: `…/🐚️Shell/🎯️targets/🧊️wgpu/{🦀️.rs, ⏪️time-travel/🦀️.rs}`
- Shell tests: `…/🐚️Shell/🧪️tests/{🧪️wgpu-time-travel, 🌓️appearance-tour-and-footer-pills}/🦀️.rs`
- Interpreter: `…/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
- ProgramBridge: `…/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`
- Bridge TS: `…/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`
- Bridge TS test: `…/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts`
- New fixture: `…/🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-peers/🔣️.json`
- UI crate:
  - `🖱️ui/🧱️elements/👥️PresenceBar/{🎯️targets/🧊️wgpu, 🧪️tests/🔬️wgpu-unit}/🦀️.rs`
  - `🖱️ui/🎯️targets/🧊️wgpu/{🦀️.rs, 🌳️tree/🦀️.rs, ⚙️engine/🦀️.rs, 🖌️paint/🦀️.rs, ♿️accessibility/🦀️.rs}`
  - `🖱️ui/🧪️tests/🧪️conformance-corpus/🦀️.rs`

## Follow-up 3 (the peers corpus is React's contract too)

### H1. React asserts `🧫️time-travel-peers`

- **What changed.** In `🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx`, W2-B's hand-written peer test ("marks the
  rows a peer edits…") is replaced by "marks the rows and chips of peers editing in time travel exactly as the shared
  peers corpus says, in both languages".
  - It reads `🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-peers/🔣️.json` through a new `peersCorpus` constant and two corpus
    types in the file header.
  - It builds the replica's `HistoryEntry` rows from the corpus rows.
  - Nothing else in the file changed, apart from one line in its docstring.
- **What it checks, per case and locale:**
  - `timeTravelPeerPresenceV1` returns exactly the corpus chips `{actor, text, badge}`, and no `historyEdit` wire field
    reaches the roster peers.
  - The overlay notes, joined with ` · ` and sorted by key, equal the corpus notes; this is the same representation the
    wgpu law compares.
  - The interpreter-rendered history body shows each row's note.
  - The `PresenceBar` chip carries `data-presence-activity` exactly for editing peers, its accessible name
    (`dom-accessibility-api`) includes the activity text, and its `peer-activity-badge:<actor>` shows the corpus badge.
- **Result.** Both shells now assert the same expected chips and notes: React through this test, wgpu through
  `the_shared_peer_history_edit_corpus_holds_on_wgpu` and the UI-crate notes law.
- **Coordination.** W2-B (a1e3416acfb374001) was told what changed. I did not touch ShellHost or the worker.

### H2. Runs (foreground)

| Run | Result |
| --- | --- |
| React suite, explicit include at `long` level: `bun ./📜️script.ts test long "⏪️time-travel/🧪️tests/🧩️component" --reporter=verbose` | **17/17**; the new test ran (195 ms) |
| React typecheck (`bun ./📜️script.ts typecheck`) | exit 2, **2 errors, neither mine**: `🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx:1140` and `:1445`, where the `PluginWasmHandle` fakes lack `readAppDocumentIdentity` after someone's handle change. `⏪️time-travel`'s test typechecks clean (the tsconfig includes `🧱️elements/**`). |

The `fundamental`/`quick` levels include only `⚡️quick`, so this suite has to be run at `long` or above with a file
filter.

## Follow-up 4 (W3-E2E prerequisites for `--renderer=wgpu`, port 6112)

### I1. P2: the transient notice reaches the ARIA mirror

Before this change the wgpu notice was only painted; only its `shell.notice.close` hit reached the mirror.

- **New node.** `ShellState::transient_notice_accessibility_node` (`🧊️wgpu/🦀️.rs`, key constant
  `TRANSIENT_NOTICE_STATUS_ID = "shell.notice"`) projects the showing notice as one node, React's
  `[data-semio-transient-notice]`:
  - `role=status`, `live=polite`;
  - **label** = the localized message, **description** = `ShellTransientNotice.code` (no description when the notice has
    no code);
  - not focusable, not actionable.
- **Where it appears.** `chrome_accessibility_nodes` projects it beside the chrome and also over an open modal dialog or
  palette, so a refusal answered while the finalize prompt is open is still told.
- **When it disappears.** A dismissed notice, or one past its 4 s deadline (`transient_notice_expired`), projects nothing.
- **Mirror.** The ARIA mirror already renders such a node as `<div role="status" aria-live="polite" aria-label=…
  aria-describedby=…>`, so the probe's `rejection-notices-carry-their-code` can read `data-node-key="shell.notice"`. The
  mirror needed no change.
- **New fixture:** `🧑‍🎨engine/🧫️fixtures/🧯️wgpu-transient-notice/🔣️.json`. It holds the expected node plus three notices: de
  with a code, en with a code, and en without one.
- **Laws:**
  - Rust `every_transient_notice_is_a_polite_status_named_by_its_message_and_described_by_its_code`
    (`🧪️wgpu-time-travel`) runs the fixture through `ShellState`: no node before, exactly one node after, the right
    role/live/label/description, still present over the finalize dialog, gone past the deadline.
  - vitest "mirrors every transient notice as the polite status React renders, named by its message and described by
    its code" (`🧪️tests/♿️wgpu-accessibility-interaction`) mounts the production mirror with the same fixture. Through
    `dom-accessibility-api` it checks role `status`, `aria-live=polite`, the accessible name equal to the message, the
    accessible description equal to the code, `tabIndex -1`, and that a click sends nothing.

### I2. P1: `semioWgpuIntrospection.dumpBoard2d(windowId?)`

The shape is exactly the one in `📓️w3-e2e-report.md` W.3 P1:

```
{surfaces:[{surfaceId, windowId, rect:[x,y,w,h], camera:{x,y,zoom}, positions:{nodeId:[x,y]}, selection:[id], nodes, edges, handles, parsed}]}
```

- **Rust (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`).** A new `🔬️Board2dStats` region next to `🔬️MeshStats`:
  - `board2d_surface` reads one `Board2dScene` with the fixture lane already merged by the reconcile, React's
    `board2dVitals`:
    - `fixture_json` gives every node's `[x, y]`, plus the node, edge and handle counts;
    - `camera_json` gives `{x, y, zoom}` (`null` if unreadable);
    - `selection_json` gives a string array; `{ids:[…]}` is accepted too;
    - an unparseable fixture reads `parsed:false`, `-1` counts and no positions, exactly as React's vitals do.
  - `walk_board2d` uses the same absolute-page-rect walk as `walk_mesh_stats` and names the window each board was found
    in.
  - `build_board2d_dump` walks every live window, or the one window named.
  - The `#[wasm_bindgen(js_name = dumpBoard2d)]` export sits in `🔬️IntrospectionExports` beside `dumpMeshStats`.
- **TS.**
  - Probe kind `"board2d"` in `BrowserFrameIntrospectionProbe`.
  - `RendererBindings.dumpBoard2d` and the worker's probe → hook mapping (`🎞️frame-worker`).
  - The page shim `dumpBoard2d: probe("board2d")`. A peer has since moved the shim from `🚀️browser-boot` into
    `🌐️browser-host` (`mountIntrospection`), and it carried this entry over.
- **Cost and gating.** Read-only and answered on request only, like `dumpMeshStats`: nothing is collected per frame, so it
  costs production nothing. It is not additionally gated on `SEMIO_RUNTIME_DIAGNOSTICS`, because `dumpMeshStats`, the
  sibling it mirrors, is not, and the probe sets that flag anyway.
- **Camera semantics.** As in React's `data-board-camera-json`, `camera` is the published scene camera, not a local pan
  that has not been republished.
- **Laws:**
  - Rust `board2d_dump_reads_the_published_board_like_reacts_vitals`: the exact serialized wire shape, plus the
    unparseable case.
  - Rust `walk_board2d_finds_nested_boards_at_their_absolute_rect_in_their_window`: nested absolute rect, window id, no
    row for a window without a board, and an empty engine answering `{"surfaces":[]}`.
  - vitest `📨️browser-frame-transport`: now also pins the worker's `"board2d" ? bindings.dumpBoard2d` mapping and the
    page shim.

### I3. Runs (foreground; the fleet reset killed the first attempt, and every file was re-read before rerunning)

| Run | Result |
| --- | --- |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- time_travel dialog_choices introspection_tests` | **40/40**, including the 3 new laws |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | ok, no warning in touched code; the `dumpBoard2d` export compiles |
| vitest `♿️wgpu-accessibility-interaction` + `📨️browser-frame-transport` (`test-browser`) | **86/86**, both new tests confirmed by name |
| native `cargo check -p semio-framework-os-renderer-wgpu --lib` (non-test) | **blocked by peers' in-progress edits**, not by renderer code (details below) |

- **Why the native check is blocked.**
  - `semio-framework-plugin-host`: unclosed delimiter at `🔌️plugin/🖥️host/🦀️.rs:3513`/`3625`.
  - `semio-framework-plugin`: `dsl::Edit` has no field `verb`, at `🔌️plugin/🦀️.rs:16920` and
    `🪟️window/🎚️config/🦀️.rs:150`.
- **Why the native build is still covered.** The test build of the same lib compiled natively minutes earlier with these
  changes. The non-test difference in my code is cfg-only: the board builders are `cfg(any(wasm32, test))` and the export
  is `wasm32`.
- **Other notes.**
  - Two peers' workspace edits (the `🗄️stdio` composition move) also blocked cargo for a few minutes. I waited for
    `cargo metadata` to resolve rather than touch them.
  - The bridge `tsc` closure now reports 2 errors in a peer's `🧪️backbone-envelope-io` test (`InferencePortClosedV1`); my
    files are clean.
- **Coordination.**
  - I told W1-G which corpora assert the `history.transition-refused` wording they are changing: the band corpus, the
    command-rejection corpus, and React's `ui.history.refusal.transitionRefused`.
  - W2-B relayed a native local-folder reattach request (remembered `os.config.local-folders` binding). It is not in my
    brief; I left it for the coordinator to route.


## Session 2 — 2026-10-01

Successor of W2-C (S2-W2C). Briefs: `🧭️plan.md` session-2 roster, `📋️design.md` §16, `📓️resume-core.md` §4.8, `📓️resume-gap.md`
G2/G5/G13, and the coordinator's relays (refusal codes, S2-W2B's `transitions` and local-folder schema). Cut by the usage limit at
~13:30 and resumed 16:35; this section is updated at every milestone.

Status (10-02 03:05): S2.1–S2.6 and S2.8 **run and green**: the targeted wgpu laws are **52/52** (03:02, the binary linked
at 23:25), including the two S2.8 host-route laws. The native `--lib --tests` and `wasm32-unknown-unknown --lib` checks were green at
22:16 and 22:37. One edit is newer than that binary: the 02:47 restructure of `direct_reattach_candidate` to an early return. Its
rebuild at 03:00 is **peer-blocked**: `semio-framework-os-kernel` does not compile (27 errors, `RecordSpecProducer`, from a peer's
in-progress dsl refactor). It will be retried once the kernel is green.

### S2.1 Repair: follow-up 5 (folder re-attach) was source-complete, never run

- `📎️local-folders/🦀️.rs` and `🧪️wgpu-local-folders/🦀️.rs` were whole files (balanced, mounted, wired at the attach, detach,
  reconnect and forget sites, the chrome phase `FolderReconnectBand` and the a11y projection). Nothing was half-written.
- **Decision (G5, native re-attach parity), now coded:** the shell reattaches *directly, once per document* whenever its build
  serves a folder transport (`SHELL_DOCUMENT_TRANSPORTS.folder`, native), and restores the folder's archive (rows, head) like a
  fresh load; a failed reattach leaves the accessible "Reconnect folder / Forget folder" band (React's band). The browser build is
  covered by S2.8 (host-route folder door, band always offered like React's browser shell). The predecessor's `Unavailable` copy,
  the `document-binding.folder-unavailable` branch and `note_folder_reconnect_fault` are deleted (a failure goes through
  `note_dispatch_fault`); the reattach mode is now the bool `LOCAL_FOLDER_DIRECT_REATTACH` (native true, browser false) held on
  the shell as `local_folder_direct_reattach`.
- Per-frame cost: `folder_reconnect_offer` answers before reading the session identity when the device remembers no folder.
- Laws (`🧪️wgpu-local-folders`): shared corpus (event log, offers, names, copy), the config payload fixtures, native lifecycle
  (attach → restart → direct reattach → archive restores rows `edit:e-1, edit:e-2` and head `cp-2`; detach → nothing), the band
  only after a failed reattach (polite status + two buttons, en/de, Forget retires it), and the compact law (S2.4).
- `📎️local-folder-bindings` now has S2-W2B's schema (`🧑‍🎨engine/🧬️schema/🔣️local-folder-bindings`); the wgpu law reads the same
  fixture, and the ticket script `🧪️s2-w2c-validate-corpora.py` validates it with Python `jsonschema` (os.config payload schemas
  registered): **PASS**.

### S2.2 Refusal codes by code (relay: S2-W2A strings, S2-W2B corpus)

- `time_travel::history_refusal_notice` now answers every `timeTravel.*` code of the framework's one table
  `semio_framework_time_travel::TIME_TRAVEL_CODE_LABELS` (17 codes: session refusals, frozen, cancelled, the plugin's busy …
  schema-unavailable, replay-faulted, commit-failed) as a warning notice carrying its code, plus the hub's three `history.*` as
  errors. The shell's own `SESSION_REFUSALS` list and its private name-invalid copy are deleted. The same text reaches the band's
  fault line, dispatch-fault notices and the ARIA mirror (`shell.notice`, P2).
- Law `history_refusals_are_localized_notices_carrying_their_code` now also asserts that the corpus's `timeTravel.*` codes are
  exactly the framework table (and the hub's three beside them).
- 10-02 03:25 (coordinator relay): `timeTravel.member-gone`. S2-W2A added it to the table at 03:24
  (`TIME_TRAVEL_MEMBER_GONE_CODE`, `TimeTravelLabel::RefusalMemberGone`, key `refusalMemberGone`, en "The part this history edit
  targets was closed"; the de text came with it). wgpu therefore renders it as a warning notice carrying its code, on the band's
  fault line and in the ARIA mirror, with **no shell change**: `history_refusal_notice` reads the table, and no wgpu `match` over
  `TimeTravelLabel` is exhaustive. Until S2-W2B adds the matching band-corpus row, the law above is red **by design**: 18
  table codes against 17 corpus codes, and that equality is the gate.

### S2.3 Focus on Begin and on stage changes, in the ARIA mirror (G13, design §16)

- I first wrote a separate focus corpus; S2-W2B then added `transitions` to the shared band corpus, so I deleted mine and wgpu now
  implements React's `timeTravelTransitionV1` exactly: `time_travel::time_travel_transition(previous, next) → {reveal, focus}`.
  The edge into a *new session* (also an already-open session the shell first sees, and a second session after a first) reveals
  the History panel; a draft that starts (Begin, Next problem, another mutation) focuses the editor; a replay or a review focuses
  the band; choosing focuses the prompt; progress, a draft edit, the commit and the close move nothing.
- Resolution (`resolve_time_travel_focus`, called in `acknowledge_presented_input` after the frame's documents are published and
  before the chrome projection is built, so the mirror sees the move in the same frame):
  - editor: the first enabled control under `framework.history.editor.inputs` (never its row), else the editor's Accept —
    React's `timeTravelFocusElementV1("editor")` — focused through `dispatch_accessibility_event` in its retained window (new
    interpreter helper `visible_retained_accessibility_target`);
  - band: the band's live node `shell.time-travel.status` becomes the focused chrome node (focusable, never a Tab stop) — React
    focuses its `[data-semio-time-travel]` region;
  - dialog: the open finalize prompt already focuses its first stop.
  - React's `timeTravelFocusIsHeldV1`: a chrome text field outside the prompt, or a retained input outside the History panel,
    keeps its focus; a target not on screen within 1 s is given up (React retries 30 frames).
- Phone width: below the mobile breakpoint the reveal opens the one mobile panel on the History tab.
- Laws (`🧪️wgpu-time-travel`): `the_shared_band_transitions_hold_on_wgpu` (every row: reveal + focus, and the observed reveal),
  `the_editor_target_is_the_first_enabled_input_control_else_accept`,
  `a_replay_and_a_review_focus_the_band_the_prompt_takes_focus_and_typing_elsewhere_keeps_it`,
  `beginning_an_edit_focuses_the_first_editor_input_in_the_published_projection` (a real retained document painted, published,
  and the stepper — not its row — focused in the published projection), `on_a_phone_the_session_start_opens_the_mobile_panel_on_history`.

### S2.4 Blocking-rule copy and compact layout (design §16.1, G13)

- §16.1 copy, one change in every shared source: `TimeTravelLabel::ReportBlocking` (Rust + TS twin + lifecycle-law fixture +
  its ticket generator `🧪️w1-b-generate-lifecycle-law.py`), React's `ui.timeTravel.review.blocked` (en/de, normal + beginner),
  the band corpus case, the wgpu law and the probe's regex: "Errors must be fixed or withdrawn before finalizing" / "Fehler müssen
  vor dem Abschließen behoben oder zurückgezogen werden" ("zurückziehen" is the editor's Withdraw).
- Compact layout: `time_travel::chrome_band_layout` is the one bottom-band layout (React's `max-w-[90vw] flex-wrap`): one row
  while it fits min(max width, 90 % of the viewport); else the message wraps (at " · " segments for the time-travel band, at words
  for the folder band) over full-width lines with the buttons flowing in rows below. Both the time-travel band and the folder
  band use it; their plans now carry `lines` instead of one clipped message.
- Laws: the band layout law gained 375 px / 320 px cases (inside 90 %, still above the footer, every word kept, no line starting
  or ending on the separator, buttons below and non-overlapping, the replay track on the lower edge);
  `the_reconnect_band_is_one_row_on_a_desktop_and_wraps_compact_on_a_phone`.

### S2.5 Order-dependent renderer tests (G2)

- Root cause: the renderer's three `WorkerCell`s (interpreter, scenes, engine canvas) became process-global
  `OnceLock<Mutex<T>>` so state can resume on any pool worker; in one `cargo test` process every law then shares one UI engine,
  one chrome registry and one scene map — the "presented input candidate could not be sealed", `ArenaFull` and hang reports.
- Fix: under `cfg(test)` every `WorkerCell` resolves to the calling test thread's own state
  (`crate::interpreter::test_worker_cell`; libtest runs each law on its own thread). The engine canvas's duplicate `WorkerCell` is
  deleted in favour of the interpreter's one (which gained `try_borrow_mut`). Production behaviour is unchanged.
- Remaining limit (not fixed, outside the renderer): the UI contract's own arenas (`UI_VALUE_ARENA` with one live page,
  `UI_DOCUMENT_ARENA` with 64 slots) are process-global by design, so a single process running hundreds of laws can still exhaust
  them. The repo's gate already runs each law in its own process: `test-wgpu-unit` → `runCargoTestBudgeted` → `cargo nextest`
  (process per test). Measured (S2.6): in one process 78 of 1626 laws still fail and 13 hang; 72 of those 78 pass in their
  own process. So the gate is `test-wgpu-unit` (nextest, process per law); a one-process run is not a gate until the contract
  arenas get a per-test mode (UI-contract owner, not this WP).

### S2.6 Verification so far

| Command | Result |
| --- | --- |
| vitest `♿️wgpu-accessibility-interaction` + `📨️browser-frame-transport` (P1 `dumpBoard2d` transport, P2 notice mirror), `bun …/vitest.mjs run --config …/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts …` from the wgpu TS package | **86/86** |
| vitest `🧩️package-integration` (16:48, after the registry was re-emitted) | **22/28**: every plugin-bridge law passes, including W2-C's progress-patch queue and the ephemeral `toolRun`/`historyEdit` bytes; the 6 failures are all in "generated worker" (package artifact authority drift, worker/entry render, devcontainer `deps-javascript` pin) — environment and generated artifacts, not W2-C code |
| `python3 T/🧪️s2-w2c-validate-corpora.py` (band + local-folder corpora vs their schemas) | **PASS, PASS** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` (13:23 run) | my files clean; 3 errors in peer files: `🎬️media-slots` calls `reconcile::media_transport_contract_valid` which a peer made `pub(crate)`; `🧊️renderer/🦀️.rs:19177` lacks the new `MutationEnvelope.line`; `🎞️Scenes` `request_stale_canvas_authority_cancel` passes an `FnMut` where `canvas2d_gumball::request_cancel` wants `Fn` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` (16:51, after the two peer-break fixes below) | **Finished**, 0 errors; no warning in my files after the clean-up (an unused `local_folder_bindings()` accessor deleted, Scenes paths unqualified) |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s2-w2c cargo test -p semio-framework-os-renderer-wgpu --lib --no-run`, then the binary with filters `time_travel local_folders introspection_tests dialog_choices` (17:31) | **50/50** — 29 time-travel laws (incl. `the_shared_band_corpus_holds_on_wgpu` over 12 cases, `history_refusals_…` over the 20 refusal rows, `the_shared_band_transitions_hold_on_wgpu` over 13 transitions, the focus, compact and mobile laws, `beginning_an_edit_focuses_the_first_editor_input_in_the_published_projection`), 5 local-folder laws, 14 interpreter introspection laws (P1 `dumpBoard2d` ×2), 2 dialog-choice laws. The first run (17:24) was 48/50: the restart law asserted the band after a restore (moved before it) and the focus law's tree items lacked `icon` |
| Whole renderer lib in ONE process, 4 threads, per-thread `WorkerCell` isolation (17:32, killed by a 25-min alarm) | 1242 passed, 78 failed, 13 hung; of the 78, **72 pass when run alone** (each its own process) and 6 fail alone for peer reasons: `engine_canvas_slot_tables_…` (registry element 85 024 vs committed 82 656 bytes), `every_boot_door_names_the_same_axes`, `every_page_door_hands_the_census_across`, `the_browser_appearance_door_is_wired_end_to_end`, `focused_text_editor_clipboard_…`, `the_puzzle3d_app_carries_the_introduction_…` |
| vitest `🧪️wgpu-backbone-folder-door` (new) + `🧩️package-integration` (21:52) | door **4/4**; package-integration **23/29**: all plugin-bridge laws pass incl. the new `readHistory` law; the same 6 "generated worker" environment failures |
| `tsc -p T/🧪️s2-w2c-typecheck-folder-door.tsconfig.json` (bridge, host-io, wgpu Vite config, both tests) | **0 errors** |
| `python3 T/🧪️s2-w2c-validate-corpora.py` | band, local-folder bindings and folder door corpora: **PASS ×3** |
| `bun ./📜️script.ts verify taxonomy report --scope <dir>` for the three new dirs (`🧫️fixtures/🧫️wgpu-backbone-folder-door`, `🧬️schema/🔣️wgpu-backbone-folder-door`, `🧪️tests/🧪️wgpu-backbone-folder-door`), 10-02 02:45 | **clean ×3** (0 errors, 0 warnings) |
| The same binary (`target-nde-s2-w2c` unit `55f243d0bd8af08a`, linked 23:25), copied to `T/🗑️generated/s2-w2c/renderer-tests.bin` and run from the renderer crate dir with `RUST_MIN_STACK=67108864` and filters `time_travel local_folders introspection_tests dialog_choices` (10-02 03:02, after the cargo hold was lifted) | **52/52**: the 50 above plus `the_shared_backbone_folder_door_corpus_holds_on_wgpu` and `a_browser_folder_keeps_its_archive_through_the_host_route_and_comes_back_after_a_restart` |
| Rebuild for the 02:47 `direct_reattach_candidate` restructure (`cargo test … --lib --no-run`, 10-02 03:00) | **peer-blocked**: `semio-framework-os-kernel` has 27 errors (`🚪️io/🧬️schema/🔗️reference` E0308 and `🎒️pack/🌱️value` E0618, `RecordSpecProducer`), from a peer's in-progress dsl refactor; the renderer was not reached |

Peer-break fixes I made to unblock the crate (coordinator-approved for the last two, 16:50):

- 13:23 — board DAG `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` (untouched since 06:26): the two `DslValue::Bytes` arms
  the intrinsic-bytes rollout missed (retirement via its existing `bytes`, preview summary `<n bytes>`), mirroring the rollout's
  own fix in `🗿️artifacts/🕸️dag/🧵️retained`.
- 16:50 — `🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:1065`: a peer narrowed `media_transport_contract_valid` to `pub(crate)` at
  11:32 with no public replacement, breaking its live cross-crate consumer `📺️renderer/…/🎬️media-slots/🦀️.rs:164` (E0603) for
  5 h; restored to `pub`.
- 16:50 — `📺️renderer/…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:19177`: the probe's `MutationEnvelope` literal lacked the new
  `line` field (E0063, per-viewer head); added `line: None` like the field's other sites.

### S2.8 Browser folder transport (coordinator request 17:50, G5 on 6112)

Why it was missing: the browser build's kernel `ArtifactHost` actor has no filesystem (`SHELL_DOCUMENT_TRANSPORTS.folder =
false`), and the shell runs in the frame Worker, which has no `fetch` with a page origin's cookies. React keeps folder documents in
its worker through the dev host's backbone route (`GET|PUT /semio-backbone?uri=&documentId=&schema=`, Vite
`semioBackboneVitePlugin`, archives in `<folder>/.semio/documents.db`). The wgpu browser build now does the same:

- **Dev host.** The wgpu Vite config (`🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts`) now installs `semioBackboneVitePlugin()`, so
  6112 serves the same route React's 6012 serves.
- **Page door.** `🚪️host-io/🟦️.ts` gains the op `backbone-folder` (`backboneFolderHop`): a read answers `{status: 200,
  bodyBase64}` or `{status: 204}`, a write PUTs the archive bytes the shell handed over, any other status is answered as itself and
  a fetch that never reached the host as `{error}`; 15 s deadline like React's `FOLDER_FETCH_TIMEOUT_MS`.
- **Bridge.** `🐚️plugin-bridge/🟦️.ts` gains `readHistory` (the guest's `AppChannelClient.readHistory` → `HistorySnapshot`), and
  `ProgramBridgeEntry::read_history` now answers on both builds, so `seed_history_snapshot` is one target-neutral function: a
  restored archive (and a session start) shows its rows and head on the browser too. `ProgramFixtureDocument` gained an `archive`
  hook for the laws.
- **Shell (`📎️local-folders`, region `🔖️HostRouteFolder`).** `attach_host_route_backbone`: read the stored archive and restore it
  (`restore_document_archive_bytes`, shared with the native restore), or write the document's own archive first when nothing is
  stored; keep `HostRouteFolder {uri, documentId, schema, written digest, written cursor}`; remember a folder binding.
  `flush_host_route_folder` (each frame on the browser) writes the archive again after every history change, never while a
  history edit is open and never byte-identical; `detach_host_route_folder` writes the last change and forgets the folder. The
  sync card's attach for `folder://`/`file://` routes there on the browser build; the band is always offered there (gesture),
  and Reconnect goes the same way.
- **Corpus (language-neutral, both halves):** `🧑‍🎨engine/🧫️fixtures/🧫️wgpu-backbone-folder-door` + schema
  `🧬️schema/🔣️wgpu-backbone-folder-door`: three requests (the Rust JSON and the page's one fetch) and six answers (page answer per
  HTTP outcome and what the shell reads).
- **Laws:** vitest `🧪️tests/🧪️wgpu-backbone-folder-door` (Ajv schema check, every request → its fetch, every outcome → its answer,
  a write without bytes refused before fetching; registered in the wgpu vitest include list); Rust
  `the_shared_backbone_folder_door_corpus_holds_on_wgpu` and
  `a_browser_folder_keeps_its_archive_through_the_host_route_and_comes_back_after_a_restart` (fake host in `cfg(test)`: read → 204
  → write under the schema; bound and remembered; one write per history change, none during a history edit; detach writes and
  forgets; after a restart the band is offered, Reconnect restores the archive with rows `edit:e-1, edit:e-2` and head `cp-2`).
- Not covered: external edits to the folder while attached (React listens on the dev stream mux `backbone.folder` route; the wgpu
  browser re-reads only on attach). The live 6112 run needs the coordinator's activation + serve (the Vite config change takes
  effect on the next serve).

### S2.9 Open items and coordinator actions

- **Resolved 22:37 (peer-break fixes, coordinator-approved):** the renderer was red on two stalled peer rollouts, and I completed
  each minimally:
  - 22:05: the `semio_framework_value` rollout (19:40–19:46). `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:47–73` and
    `🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs:708` needed the crate, so the renderer `📦️packages/🦀️rust/Cargo.toml` gained
    `semio-framework-value = { workspace = true }`. `🕹️wgpu-reserved-verb-answer` called `.expect` on a now-plain `Vec<u8>`; both
    calls are dropped.
  - 22:18: the execution-target lease rollout (21:43). `ShellState.document_execution_target_lease` was native-only, but the
    peer's three new assignments are target-neutral. The field and its init are now on both builds, and the type is spelled by
    its full path because its import is native-only.
  - Result: `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` (native) **Finished** at 22:16, and
    `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` **Finished with 0 errors** at 22:37.
    No warning in my code; the remaining warnings belong to the peers (an unused `checkpoint`, an unnecessary qualification at
    7181).
- **Done 03:02:** the built binary ran 52/52 (S2.6). **Pending:** rebuild and re-run once the peer's os-kernel dsl refactor
  compiles, so that the 02:47 `direct_reattach_candidate` restructure is covered too.
- **Coordinator:** one activation + serve of 6112 so the probe's `--renderer=wgpu --explore` can calibrate; the new Vite plugin and
  host-io op need that rebuild. React should adopt the focus contract (S2-W2B already owns `timeTravelTransitionV1`).

### S2.10 S2-W2A wire adopted, and what the `--renderer=wgpu` probe needs (item d)

- **S2-W2A wire (`📓️w2-a-report.md` §8.5), adopted:** `TimeTravelLabel::for_code` / `TIME_TRAVEL_CODE_LABELS` (S2.2: wgpu's one
  refusal table); `HistoryMutationEntry.introduced` ("New since this edit" is in the Rust-built row description, so wgpu shows it
  with no shell code; S2-W2A added `introduced: false` to the wgpu peers-corpus literal); chip entity labels, the overflow row
  and the editor closing on leaving Editing all arrive in the Rust-built body.
- **P1 still holds.** `semioWgpuIntrospection.dumpBoard2d(windowId?)` (interpreter `🔬️Board2dStats`, worker probe kind `board2d`,
  `🌐️browser-host` shim) now also carries `highlighted` (S2-W2D, G3).
- **P2 still holds.** The `shell.notice` polite status names the notice by its message and describes it by its code. Every
  `timeTravel.*` code is now localized (S2.2).
- **New or changed mirror facts the probe can rely on:**
  - **Focus (G13).** `focus-moves-to-the-editor` can read the focused mirror node:
    - on Begin and Next problem, the first control under `…/framework.history.editor.inputs` (e.g. `…editor.input.dx`, a
      `spinbutton`);
    - during a replay or review, `shell.time-travel.status` (`data-focused`);
    - while choosing, the prompt's first stop.
    - A person typing elsewhere keeps focus.
  - **Reveal.** `history-panel-reveals-on-session-start`: the History anchor opens on every new session. Below 768 px the mobile
    panel opens on History instead.
  - **Phone width (375 px).** The band and the folder band wrap within 90 % of the viewport: message lines first, then the
    buttons. Mirror keys are unchanged.
  - **Folder on 6112 (G5).** The sync card's `framework.sync.folder`, then a typed path, then `framework.sync.attach` now binds the
    document through the dev host's backbone route (S2.8). After a reload the band offers the folder (`s-folder-reconnect-message`
    status, `s-folder-reconnect`, `s-folder-forget`), and Reconnect restores the archive with its history rows. S2-E2E's
    wgpu-only exemption `reload-restores-the-edited-document` ("no folder route", R3.1) can go: the strict reload verdicts apply
    to wgpu too.
- **Needs before the run:** the activation must include the wgpu Vite config (the backbone plugin), host-io, bridge and shell
  changes. Activation #7 compiles from the live tree, so it does. Then serve 6112 and run `--explore` first.

### S2.7 Files (session 2)

- wgpu shell: `🐚️Shell/🎯️targets/🧊️wgpu/{🦀️.rs, ⏪️time-travel/🦀️.rs, 📎️local-folders/🦀️.rs}`;
  tests `🐚️Shell/🧪️tests/{🧪️wgpu-time-travel, 🧪️wgpu-local-folders}/🦀️.rs`.
- Renderer: `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` (`visible_retained_accessibility_target`, `WorkerCell` test isolation,
  `try_borrow_mut`), `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (test isolation), `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` (duplicate cell
  deleted).
- Copy: `🔨️modules/⏪️time-travel/{🦀️.rs, 🟦️.ts, 🧫️fixtures/🧫️lifecycle-law/🔣️.json}`, `🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`,
  `🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band/🔣️.json`, ticket `🧪️w1-b-generate-lifecycle-law.py`, `🔍️time-travel-probe.ts`.
- Peer files (S2.6, S2.9): `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`, `🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs` (one
  visibility), `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` (one field), the renderer
  `📦️packages/🦀️rust/Cargo.toml` (one dependency line), `🌉️ProgramBridge/🧪️tests/🕹️wgpu-reserved-verb-answer/🦀️.rs` (two calls),
  and the lease field in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`.
- Browser folder transport (S2.8): `🎯️targets/🧊️wgpu/{🚪️host-io/🟦️.ts, 🐚️plugin-bridge/🟦️.ts, 🌐️server/🎚️config/🟦️.ts,
  🧪️tests/🎚️config/🟦️.ts}`, `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`, `🧑‍🎨engine/{🧫️fixtures/🧫️wgpu-backbone-folder-door,
  🧬️schema/🔣️wgpu-backbone-folder-door}/🔣️.json`, `🧑‍🎨engine/🧪️tests/{🧪️wgpu-backbone-folder-door, 🧩️package-integration}/🟦️.ts`.
- Ticket: `🧪️s2-w2c-validate-corpora.py`, `🧪️s2-w2c-typecheck-folder-door.tsconfig.json`. Created then deleted (superseded by S2-W2B's `transitions`):
  `🛠️ShellHelpers/{🧫️fixtures/🧫️time-travel-focus, 🧬️schema/🔣️time-travel-focus}`.

## Session 3 — 2026-10-02

Successor S3-W2C (session 3, coordinator `⚪b7db773a…`). Focus: verify → fix → close (rule 29). Status: **IN PROGRESS** (updated at
every milestone).

### S3.1 Repair-first check (rule 28)

- Files this WP owns, newer than the session-2 section (03:26): none. `🧊️wgpu/🦀️.rs` 22:17, `⏪️time-travel/🦀️.rs` 16:34,
  `📎️local-folders/🦀️.rs` 02:47 (the `direct_reattach_candidate` early-return restructure, complete and balanced),
  `🧪️wgpu-time-travel` 17:24, `🧪️wgpu-local-folders` 21:45. No half-finished edit.
- React's `🏛️ShellHost/📎️local-folders` changed at 03:35–03:37 (S2-W2B): three docstring emojis and one phone-width test; the
  event log, offer, name and copy the wgpu twin mirrors are unchanged, so folder parity needs no wgpu change.
- The band corpus now carries `timeTravel.member-gone` (S2-W2B, 03:35): 18 corpus codes equal the framework's 18-code table, so the
  S2.2 equality gate is expected green again (confirmed in S3.2).

### S3.2 Verification (foreground, gated; scratch in `🗑️generated/s3-w2c/`)

| Command | Result |
| --- | --- |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-w2c cargo test -p semio-framework-os-renderer-wgpu --lib --no-run` (10:57→11:22, load ~80) | **Finished**, 0 errors (`test-build-1.txt`); no warning in `⏪️time-travel`, `📎️local-folders` or their test files |
| The binary (copied to `renderer-tests-1.bin`), from the renderer crate dir, `RUST_MIN_STACK=67108864`, filters `time_travel local_folders introspection_tests dialog_choices` | **52/52** (`run-1.txt`) — now covers the 02:47 `direct_reattach_candidate` restructure (S2.9 owed item closed), `history_refusals_…` with the 18-code `member-gone` gate, the band/transition/focus/mobile laws, the shared folder corpus + native restart reattach, the backbone folder door, `shell.notice`, `dumpBoard2d` (incl. `highlighted`) |
| Same binary, filters `chrome_overlays_tour_tests board2d_engine_tests board_presence_tests agent_overlays_tests shortcuts_palette window_actions_search_panes panel_anchor_model hub_projection_workspace canvas_presence` | **177/178** (11 ignored) (`run-2.txt`); the one ✘ fails alone too and is a peer's: `shell_shortcuts_palette_tests::find_publishes_filters_and_activates_with_a_physical_row_click` panics at `♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs:629` "bundled DAG demo DSL is valid DagSnapshot text: expected LBrace, found Ident 'id'" (dsl text-grammar refactor; not W2-C) |
| vitest `bun ./📜️script.ts test-browser 🧪️tests/♿️wgpu-accessibility-interaction/🟦️.ts 🧪️tests/📨️browser-frame-transport/🟦️.ts 🧪️tests/🧪️wgpu-backbone-folder-door/🟦️.ts` (wgpu TS package) | **90/90**, 3 files (`vitest-browser-1.txt`) |
| vitest `bun ./📜️script.ts test-preview-generated` (`🧩️package-integration`) | **26/29** (`vitest-package-integration-1.txt`): every plugin-bridge law passes (progress-patch queue, ephemeral `toolRun`/`historyEdit`, `readHistory`); the 3 ✘ are "generated worker" environment checks (package artifact authority drift ×2, devcontainer `deps-javascript` pin) — generated artifacts, not W2-C code (was 6 ✘ in S2.6) |
| `CARGO_INCREMENTAL=0 … cargo test -p semio-framework-ui --features testkit --lib -- conformance_corpus presence_bar peer_notes` (12:02→12:20) | **16/16** (`ui-test-1.txt`): 7 `presence_bar` (incl. W2-C's activity badge + accessible name), 9 `conformance_corpus` (incl. `the_history_editor_controls_project_their_corpus_accessibility`, `peer_notes_are_announced_after_a_tree_rows_description`, W1-E's `the_g6_number_control_cases_carry_and_paint_every_facet`) |

### S3.3 Staged-arg number facets (`staged_arg_row`, `🧊️wgpu/🦀️.rs`) — SOURCE WRITTEN, compile pending (peer break)

S3-W1E's helper landed at 12:08 (`semio_framework::ActionArgDef::number_facets(locale) -> Option<ActionArgNumberFacets>`, manifest
region `🔖️ActionArgFacets`, corpus `🛂️manifest/🧫️fixtures/🧫️number-facets`, 9 cases). `staged_arg_row` (the Actions form and the
command palette's staged form) now calls it once per row and maps it exactly as relayed:

- **Slider / Dial** → `UiSliderNode` with travel `min`/`max`, `step` (else 1), stored unit, detents, `appearance` (Dial for a dial),
  `scale` (log), `precision`, `display_unit`, `display_factor` and `limits: Some(facets.limits)` — the hard bounds with their
  localized refusals, which make the travel soft. The staged value is **no longer clamped** to the travel.
- **Stepper** → key range from the facets (an excluded bound is no key end), step, precision, detents, shown unit, display factor,
  limits.
- **Number field** and **every vector axis** → `UiInputNode` min/max/step/precision/snaps/`display_factor`/`limits`; an axis value is
  formatted at the precision only when no display factor applies (with a factor the field shows display units itself).
- Deleted: the row's private `precision` closure and both ad-hoc `snaps_are_valid` filters (the facets already filter detents);
  one unnecessary `semio_framework::` qualification in the Reference arm.
- New law `every_staged_number_row_carries_the_shared_corpus_facets` (`🧪️wgpu-time-travel` 🎛️StagedEditors): every corpus case
  through `staged_action_arg_row` in its locale — dial, log slider, both steppers, number field, vector (every axis), the
  step/precision edge case — compared field by field with the corpus facets (refusals en/de included); text and colour carry
  no limits; the log slider keeps a staged 20 beyond its soft travel (10) because the limits admit it. The existing
  `the_actions_form_renders_every_editor_kind` keeps its expectations.
- Not mine (W1-E's 🗨️ChromeDialog region): `ChromeDialogFieldKind::Slider` (chrome dialogs' staged sliders) still carries only
  min/max/step/precision/unit/snaps — the same helper should feed it.

### S3.4 History body N1/N15 on wgpu (coordinator request 12:35) — LAW WRITTEN, run pending (peer break)

New region 📚️HistoryBody in `🧪️wgpu-time-travel`, law `the_guest_history_body_opens_pages_and_refuses_edit_with_its_reason`. It
builds the body with the guest's REAL producer `semio_framework_plugin::app::ui_history_panel` (one transaction row of 120
operations, 3 projected; the store page past the projection supplied like `history_mutation_pages`), flattens the `BuiltNode`
into retained records, paints it on the History panel surface and reads the ARIA mirror projection:
1. a closed row announces `expanded=false` (window total 120, no children materialised);
2. the mirror's activation opens it; `observe_tree_windows` then files a `TreeWindowRequest` for body `framework.body.history`
   and node path `framework.history.commands␟framework.history.entry.1` — the guest's own `history_row_window_path(1)`;
3. the guest's answer for that request renders `m-0…` in op order past the projection;
4. a window at offset 100 shows exactly `m-100…m-107`; with a running replay every Edit is `button`, disabled, not
   focusable/actionable, named "Edit: Not possible right now…" / "Bearbeiten: Derzeit nicht möglich…", and activating it or
   its row dispatches no `historyEditBegin`;
5. with Begin allowed the Edit button is enabled and dispatches `historyEditBegin{mutationId: m-100}`.
The paint loop of the focus law is now the shared harness helper `paint_retained_body` (+ `close_retained_body`).

Expected red from reading the code (to be confirmed by the run): step 1–2. `🖱️ui/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs`
`UiTree::disclosure_open` (548) and `disclosure_is_interactive` (561) treat a tree item as a disclosure only when it has
materialised `items`, while the layout (`📌️mounted_layout` `tree_item_has_rows`, 270) and the widget path treat `window.total > 0`
as expandable-but-not-yet-streamed (React's `TreeDataWindow` rule). A closed history row therefore has no `aria-expanded` and
cannot be opened through the retained tree on wgpu.

### S3.5 Resume after the usage cut (~13:05) and the machine reboot (~17:00) — 18:40

- Repair check: every S3.3/S3.4 edit is on disk and whole (`staged_arg_row` facet mapping, the corpus-facets law, the
  📚️HistoryBody region, the shared `paint_retained_body`/`close_retained_body` harness; the focus law already calls the helper).
  A peer's locale refactor (committed 17:04) rewrote `ShellState::new(…, Locale, Terminology)` / `ViewModel::new(…)` in my test
  file, including `history_body`'s view; nothing of mine was half-written.
- Peer break seen at 12:49: the `#[derive(Mutations)]` expansion named `::semio_framework_schema_state::StateClass`, which the dag
  artifact (and other derive users) did not depend on; now the derive names `::semio_framework_os_kernel::StateClass` (S3-INFRA's
  schema split). Re-checking.

### S3.6 Completing the peer's `WorkerCell` initializer refactor (coordinator GO 18:52) — SOURCE COMPLETE, compile pending

**Cause.** At 14:22 `🖱️ui/🎯️targets/🧊️wgpu/⚙️engine` changed `Ui::new()` to `Ui::new(locale)` and deleted `impl Default for Ui`
(no default language). At 16:57 a peer, cut by the reboot, gave the interpreter's `WorkerCell` an initializer
(`new(initialize: fn() -> T)`, `test_worker_cell(key, initialize)`) and stopped there. The result is in commit 202c4b7b5b1:
`state()` and every caller were left on the old shape, which gave 236 errors in the renderer test build (18:49).

**Fix, one wave:**
- **Interpreter `WorkerCell`:**
  - `impl<T: 'static>` (no `Default` bound).
  - `state()` builds through `(self.initialize)()`.
  - Under `cfg(test)`, `test_worker_cell(addr, self.initialize)` builds from `initialize()`.
- **Statics.** All 21 `WorkerCell::new()` statics now pass `Default::default`: Interpreter ×14, EngineCanvas ×5, Shell `CHROME_PREFS` and
  `CHROME_CONTROL_NAMES`. The edit is the counted script `T/🧪️s3-w2c-worker-cell-initializers.py`, which refuses on any count
  mismatch.
- **Scenes.** Its own cell passes `|| RefCell::new(T::default())` to the test hook.
- **No default language for the engine** (coordinator decision):
  - `static UI_ENGINE: UiEngineCell` wraps `WorkerCell<Option<Ui>>`.
  - `UiEngineGuard` dereferences to `Ui`, so the 181 `UI_ENGINE.with(|cell| cell.borrow()…)` sites are unchanged.
  - An access before installation is refused by the named constant `UI_ENGINE_LOCALE_UNRESOLVED`
    (`ui.engine.locale-unresolved: …`), the same `expect`-by-name convention as `ShellState::active_locale`.
  - `install_ui_engine_locale(locale)` builds the engine on the first resolution. Every later resolution calls the new
    `Ui::set_locale` (→ `Shell::set_locale`, which repaints role chrome).
  - The shell installs it as the first statement of `ShellState::new(…, locale, …)`, and after every `locale_id`
    resolution: `setLocale`, the preferences phase, and the preferences reload.
  - Only under `cfg(test)` does a law that builds no shell start from the named fixture `TEST_UI_ENGINE_LOCALE` (En). A
    production build has no initial engine.
- **Files:**
  - `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` (WorkerCell, UI engine cell, statics)
  - `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (test hook)
  - `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` (5 statics)
  - `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (2 statics, 4 installs)
  - `🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` (`Ui::set_locale`, additive)

**Compile state:**
- 18:57: the renderer was down to 3 errors, all mine: `ui_wgpu::wgpu::Locale` is private. Fixed to
  `semio_framework_ui_locale::Locale`.
- 18:58–19:03: the build was blocked by a peer editing `🎒️pack/🔤️json/🦀️.rs:1109/1516` live.
- 19:04: blocked by `semio-framework-os-kernel`, which has 12 errors from the peer dsl/schema split: `🗣️dsl/🦀️.rs:15-16`
  `semio_framework_dsl` is unlinked, plus `🏪️store/🦀️.rs:12182/13688/23452` and `🗄️durable-group:149/166` `&[FieldValue]`.
  The renderer is not reached. I'll retry once the kernel is green.

### S3.7 N2 on wgpu (coordinator 12:5x; S3-W1E's source landed) — LAW WRITTEN, run pending (kernel red)

- `history_body` now takes the session panel (`replaying_panel()` for N15, `list_editor_panel()` for N2) instead of a flag.
- New law `the_guest_editor_offers_list_and_chip_edits_within_their_bounds`. It builds the editor through the REAL
  `ui_history_panel` with a `TimeTravelEditorPanel` holding three inputs:
  - `/points`, a full list: two items, `maxItems` = `minItems` = 2;
  - its two number items;
  - `/targets`, a many-reference list with `minItems` 1, where `n1` is labelled "Corner"/"Ecke".
- The law checks, in the ARIA mirror and in en and de:
  - "Add item" / "Element hinzufügen" is a disabled button, and its row reads "Items: 2 · Maximum 2 items" /
    "Elemente: 2 · Höchstens 2 Einträge".
  - Each item's "Remove item" is disabled, and its row reads "Minimum 2 items" / "Mindestens 2 Einträge".
  - The chips are enabled buttons, "Remove Corner" / "Entfernen Ecke" and "Remove n2".
  - Activating the disabled Add dispatches nothing.
  - Chip 0 dispatches `historyEditInput{path: "/targets/0", edit: "remove"}`.
- The inputs section and the chip list are the plugin's `tree_window_*` containers, so they open on the first-paint allowance.

### S3.8 Contract follow-ups (S3-W1E relays) and the state at 19:35

- **Disclosure parity (N1):** S3-W1E's fix has landed (`UiTreeItemNode::has_rows()`), which is what my
  `the_guest_history_body_opens_pages_and_refuses_edit_with_its_reason` needs for steps 1–2.
- **`RowAction.reason` (S3-W1E):** disabled row actions are now focusable `aria-disabled` nodes and carry the reason as their
  description. My N15 assertions were adapted to that: the Edit button must be disabled and not actionable, and its name or
  description must contain both "Edit"/"Bearbeiten" and the reason. That holds whether S3-W2A keeps the label "Edit: <reason>"
  (`.disabled(true)`, as on disk) or switches to `disabled_because(reason)`.
- **Shell row actions:** the shell packs `UiTreeItemAction` into `RowAction` (`🧊️wgpu/🦀️.rs` ~5956, `reason: None`). Only the
  marketplace roster disables shell row actions: own program, install in flight, failed extension. React's marketplace shows
  no reason for those either (`canUninstall`, `🏛️ShellHost`). The packing therefore stays reasonless, for parity.
  Cross-shell follow-up, outside this ticket: give both marketplaces localized reasons; that also needs
  `UiTreeItemAction.reason` in the UI crate.

### S3.9 Verification owed — waiting for "TREE GREEN"

The coordinator has paused polling; the peer DSL crate extraction keeps the kernel, plugin and pack red. I'll run these once the
coordinator sends TREE GREEN:

1. `cargo check -p semio-framework-os-renderer-wgpu --lib --tests`, native and `--target wasm32-unknown-unknown --lib`. This covers
   S3.3, S3.4, S3.6 and S3.7.
2. A test binary filtered to `time_travel local_folders introspection_tests dialog_choices`. The 52 laws of S3.2, plus the new
   `every_staged_number_row_carries_the_shared_corpus_facets`,
   `the_guest_history_body_opens_pages_and_refuses_edit_with_its_reason` and
   `the_guest_editor_offers_list_and_chip_edits_within_their_bounds`.
3. The wider suites of S3.2, as a regression check on the WorkerCell/UI-engine wave: `chrome_overlays_tour_tests
   board2d_engine_tests …`, plus the Interpreter and Scenes laws that build no shell (`ui_command_wiring`, `scenes::`), which
   exercise the cfg(test) fixture locale.

### S3.10 S3-W2A's reprojection / history-lane wire on wgpu (coordinator relay 19:4x) — SOURCE + LAWS WRITTEN, run pending

1. **`HistoryPatch.reprojection`.** No wgpu code read the old `remoteReplay`, so nothing changes in the shell. The section is
   Rust-built and renders generically. New law `a_replaying_history_step_shows_its_progress_and_cancels_in_the_mirror` runs the
   REAL `ui_history_panel` with a local step (12 of 400) and with a refused one (`timeTravel.blocked`), in both locales. It checks:
   - the section reads "History step" / "Verlaufsschritt";
   - the status reads "Replaying history: 12 of 400 mutations" / "Verlauf wird neu angewendet: 12 von 400 Mutationen";
   - Cancel replay is an enabled button that sends `historyEditCancelReplay` with no `generation`;
   - a refused step reads "History step refused: <reason>" and offers no control.

   `history_body` now also takes the `reprojection`.
2. **New `history.replaying` notice.** `time_travel::history_lane_notice_of_fault` maps a dispatch fault carrying any code of the
   kernel's `HISTORY_NOTICE_LABELS` to a warning notice from the kernel's one copy: `toolTransaction.open`,
   `toolTransaction.unknown`, `history.full` (with `{n}` set to the count the fault's message names), and `history.replaying`.
   `classify_dispatch_fault_notice` uses it right after the hub/session refusals, so wgpu shows these notices for refused
   undo/redo/checkpoint/alternative verbs, also behind the browser bridge's prefix. Before this change wgpu showed the raw
   fault string as an error. The shared token splitter is `fault_tokens`.
   - New law `every_history_lane_refusal_is_a_notice_carrying_its_code`. It reads the kernel fixture
     `🎠️kernel/🧫️fixtures/🧫️history-notices` (row order equals `HISTORY_NOTICE_LABELS`) and checks, in both locales, the
     text, the code, the warning severity and the `shell.notice` mirror node (label = message, description = code). A code
     inside a word is not matched.
   - React has no consumer of `historyNotice()` yet (no `🧑‍🎨engine` TS hit). Its owner is S3-W2B.
3. **`historyEditBegin` → `timeTravel.busy`.** This code is already in `TIME_TRAVEL_CODE_LABELS`, so wgpu shows it as a warning
   notice, and `history_refusals_are_localized_notices_carrying_their_code` covers it through the band corpus.
4. **N15 label + `RowAction.reason`.** Covered by S3.8's adapted assertions.
5. **Recommendation, not done (parity first).** Neither shell announces a long local history step outside the History panel:
   it is a body row, not a live region. Both shells could show `HistoryPatch.reprojection` as a polite progress status in
   their bottom band, the way they do for the session. That needs one cross-shell decision (S3-W2B + S3-W2C).

### S3.11 Stepped document load, `kind`, window blur, importAbort (coordinator relays 19:5x–20:xx) — SOURCE WRITTEN, run pending

- **`HistoryReprojection.kind`** (`remote | step | load`, replacing `local`): the reprojection law is now
  `a_replaying_history_step_or_document_load_shows_its_progress_and_cancels_in_the_mirror`. It covers four cases: step en/de
  ("History step"/"Verlaufsschritt") and load en/de ("Document load"/"Dokument laden", "Loading document: 12 of 400" /
  "Dokument wird geladen: 12 von 400"). In every case Cancel replay sends `historyEditCancelReplay` without a generation, and a
  refused step names its reason. Per the S3-W2A correction there is no `cancelDocumentLoad` verb and no `documentLoad` section.
- **One whole-document load (`📓️api-stepped-document-load.md` §4, wgpu host):**
  - `ProgramBridgeEntry::load_app_document_pack(instance, pack, spr)` is now a thin call of the stepped, ACK-owned
    `load_app_document_archive` with `DocumentArchivePack { parent_pack, parent_spr, members: [] }`. It covers all four shell
    callers on both builds: hub checkpoint seed, the `Effect::LoadDocument` effect, the rebootstrap reseed and the
    `H3-wgpu-native` effect replay.
  - Deleted: the native `wasm_program_exchange::load_app_document_pack` (`AppCommand::LoadDocument`), and the bridge TS
    `WgpuPluginHandle.loadAppDocumentPack` (`channel.loadDocument`) with its JS twin `WgpuJsBridge.loadAppArtifactPack`. The
    browser door is `loadAppDocumentArchive`, which uses `AppChannelClient.loadDocumentArchive`: admit, poll each turn, ack.
  - No renderer code sends `LoadDocument` any more. The command tag itself is deleted in the bump wave (S3-W2A).
  - Law `a_plain_document_load_is_the_stepped_archive_load_without_members` (`🧪️wgpu-local-folders`).
- **`document.loading` notice:** it joins the kernel's `HISTORY_NOTICE_LABELS` (S3-W2A). `history_lane_notice_of_fault`
  reads that table, so wgpu shows the notice with no further change, and the S3.10 law iterates the kernel fixture.
- **Window blur (S3-SPATIAL N9):**
  - New region 🫥️HostWindowBlur in `🧊️wgpu/🦀️.rs`: `note_host_window_blur()` ends every typing run (`Blur`) and sets a worker-cell
    flag. The shell's drain (`arm_host_window_blur`) then arms `hostEvent{windowId: <active pane>, kind: blur}` (React's pane
    `onBlur` twin). `settle_pump_pending` counts a pending blur as armed work.
  - Native: winit `Focused(false)` calls it and requests a redraw.
  - Browser: the page's `window` blur goes to `BrowserFrameTransport.setHostWindowBlur()`, which posts `host-window-blur` to the
    Worker; the Worker calls the door `semioWgpuHostWindowBlur`.
  - Agreed with S3-SPATIAL: once its `WorldInteractionIntent::cancel(WorldCancelReason::Blur|CaptureLost)` lands, the same
    drain enqueues `Blur` into every `world3d_states` entry, and the world close path enqueues `CaptureLost`.
- **`importAbort`:** wgpu's file-open import has no cancel today. The native picker future and the browser
  `run_file_open_request` dispatch every chunk, and there is no Tasks-window task (the wgpu Task Manager is the `no-runtime`
  body). React's `importOpenedFilesV1` dispatches `importAbort` only after a person's cancel. A faithful twin first needs a
  cancellable wgpu document-transfer task. That is open, see S3.9.

### S3.12 Session 3, day 2 (10-03 05:48–06:10): verification after the core went green

- **World cancel (S3-SPATIAL's API landed: `WorldCancelReason::{Blur, CaptureLost}`, `WorldInteractionIntent::cancel`).**
  `arm_host_window_blur` now enqueues `cancel(Blur)` into every `world3d_states` entry before it arms the pane `hostEvent`.
  `CaptureLost` on window close is **not** wired, because `retire_closed_world3d_windows` retires the state in the same pass,
  so a queued cancel never publishes. The runtime's `retired` abort already leaves zero trace (told to S3-SPATIAL).
  - New Rust law `a_host_window_blur_blurs_the_active_pane_for_its_program_once`.
  - New vitest law "posts the page's window blur to the Worker at once, outside any batch, and nothing once closed".

| Command | Result |
| --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` (05:49→05:53) | **10 errors, none mine.** All are in `🧊️renderer/🦀️.rs:18985-18998` (`NativeSocketProbeSnapshot`): `?` from `ValueError` into a `Result<_, String>`. That is a peer's in-flight `🚪️io/🪶️sqlite-snapshot` → `ValueError` migration, which S3-INFRA is finishing in one sweep. Nothing else errors, so this session's wave type-checks: WorkerCell/UI-engine locale, staged facets, history-body, N2, reprojection, notice, stepped load, blur. |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` (05:54→06:00) | **Finished, exit 0** (`check-wasm-1.txt`); no warning in touched code |
| vitest `test-browser` ♿️ + 📨️ + door | **92/92** (`vitest-browser-2.txt`), new blur law included |
| vitest `test-preview-generated` | **27/29** (`vitest-package-integration-2.txt`). Every plugin-bridge law passes. The 2 ✘ are the frame-worker render: "WGPU browser import is not schema-owned: `🌱️value/⚠️refusal/🟦️.ts` in `🚪️io/🪶️sqlite-snapshot/🟦️.ts`", the peer's ValueError migration. |
| `tsc -p T/🧪️s3-w2c-typecheck-wgpu-host.tsconfig.json` (bridge, host-io, wgpu Vite config, transport, browser-host, package-integration, folder-door and transport tests) | **0 errors** |
| `bun ./📜️script.ts check-browser-worker` | **blocked by a peer**: "WGPU browser import is not schema-owned: `🖱️ui/🌐️locale/🟦️.ts` in `🚀️browser-boot/🟦️.ts`" (locale refactor). The worker file's own `tsc` needs the webworker lib this ad-hoc config lacks. |
- **CaptureLost on window close (S3-SPATIAL's request, 06:1x):** plugin-local World3d tool machines (fem/lowpoly/gen3d
  transients) are not retired by the runtime when a window closes, so the shell must send the abort itself. The flow:
  1. `retire_closed_world3d_windows` asks `world3d_close_cancel_settled(host_id)` first.
  2. On the first sighting of a closed window, the shell enqueues `cancel(CaptureLost)` and zeroes the state's bounds, so no
     pointer reaches it.
  3. The frame's `World3dAuthority` phase drives the cancel, which publishes the gumball or paint abort.
  4. The window retires once its queue drained (`world3d_interaction_front_generation` is `None`), or after 8 sightings
     (`WORLD3D_CLOSE_CANCEL_SIGHTINGS`).
  - A full queue retires the window at once.
  - Three shell laws that assumed same-pass retirement now drive the sightings: `🪟️window-lifecycle-template-drag`
    `closed_world3d_retires_every_input_and_scene_owner_before_id_reuse` (it also checks the closed world is off the pointer),
    `🧭️wgpu-navbar-footer-parity` `a_focused_world_window_does_not_retire_its_hidden_sibling`, and the `🔬️wgpu-shell-input`
    close law.
- Files this step: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (🫥️HostWindowBlur region, `arm_host_window_blur`, close-cancel field and
  helper), `🎯️targets/🧊️wgpu/{🪟️winit-app/🦀️.rs, 🌐️browser-host/🟦️.ts, 🚚️browser-frame-transport/🟦️.ts,
  🎞️frame-worker/🟦️.ts}`, `🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts`, the three shell laws above, and ticket
  `🧪️s3-w2c-typecheck-wgpu-host.tsconfig.json`.
- **`importAbort` and a cancellable import (React's `importOpenedFilesV1` + `documentTransferTasksV1`), written 06:2x.** A
  picked import is now one `ImportTransfer`. Before this, wgpu queued every chunk at once on native and awaited them all
  inside one browser call, with no cancel.
  - **Creation.** The native picker future answers `ShellIoCompletion::Import(PickedImport)`, the browser's
    `run_file_open_request` hands the pick over the same way, and both go to `begin_import_transfer`.
  - **Dispatch.** The drain's `step_import_transfers` dispatches the oldest import's chunks in order, one awaited at a time,
    within an 8 ms turn budget, so a Cancel lands between chunks. A refused chunk ends the import with the `import-failed`
    error notice. While an import waits, `settle_pump_pending` stays owed.
  - **Task Manager.** The section `os.task-manager.tasks` sits above the `no-runtime` actors and mirrors React's
    `TaskManagerTasksPanel`:
    - "Running tasks" / "Laufende Aufgaben", and "No task is running." while nothing runs;
    - per import: files · "Document import" · owner · "Running";
    - a progressbar `os.task-manager.progress.documentTransfer:<id>` of the delivered chunks;
    - a button `os.task-manager.cancel.documentTransfer:<id>` reading "Cancel <files>" / "<files> abbrechen", which sends
      `framework/cancelImportTransfer{transfer}`.
  - **Cancel.** `cancel_import_transfer` drops the remaining chunks. It arms `importAbort {}` only when a chunk was delivered
    and the app declares the verb. The notice is "Import of “<file>” cancelled." / "Import von „<file>“ abgebrochen."
    (`shell.documentTransfer.import-cancelled`).
  - **Law.** `a_picked_import_is_a_cancellable_task_that_frees_a_started_import`.
- 06:11–06:14 re-checks after the import and close edits: none of the errors is in my code.
  - native `--lib --tests`: 11 errors, the 10 ValueError probe lines plus a peer's 06:08 `⚙️EngineCanvas/…:6164:60` E0716 in
    `write_paint2d_edit` (`check-native-tests-11.txt`).
  - `wasm32-unknown-unknown --lib`: only that EngineCanvas E0716 (`check-wasm-2.txt`).
  - Coordinator told; it is waiting for "renderer test target green".
- 06:2x, after S3-SPATIAL's `world3d_cancel_owed(&World3dState)` landed: a closed world now defers its retirement only while
  it owes a cancel (a live gumball that streamed, or an open paint stroke). Every other close retires in the same pass, as
  before, so the three shell laws touched above are back to their original text (no diff).
- `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` (06:28→06:33): **Finished, exit 0**
  (`check-wasm-3.txt`). No warning in my new regions (import transfer, 🫥️HostWindowBlur). The S3-STROKES E0716 is fixed. The
  native lib-test target still waits on S3-INFRA's ValueError sweep.
- **Stepped-document-load §8 (1), "the wgpu host's cold-pair transfer asserts `Applied`" (coordinator 06:4x):** wgpu has no
  cold-pair transfer, so there was nothing to change.
  - No wgpu code sends `Event::ColdDocumentPairPage` or matches `ColdPairIngressStatus::{Applied, Loading}` (checked:
    `🧊️renderer`, Shell, ProgramBridge, wgpu TS). The senders are the native `🔌️plugin/🖥️host` and the browser
    `🏪️store/👷️worker`, the latter owned by S3-W1G.
  - wgpu's whole-document loads are the stepped archive load (S3.11).
  - The kernel turn loop's "non-idle cold pair is not more work" rule (`🧊️renderer/🦀️.rs` ~9187) never meets an open transfer.
  - Told S3-W2A (a636b4d488c628fff) directly.

### S3.13 F7: disabled context-menu rows with their reason (S3-W1E's `ContextMenuItemSpec.reason` landed), 06:5x — SOURCE + LAW WRITTEN

All edits are in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`.
- **Data:**
  - `ContextMenuItem.reason: Option<String>`, copied from the spec in `shell_context_menu_item_from_spec`.
  - `ContextMenuItem::painted_label` gives the label and, beside it, the reason of a disabled row ("Paste · The clipboard is
    empty"). Both the production stepped renderer (`render_context_menu_step`) and the test level renderer paint it, muted.
- **Hit and accessibility:** both renderers now call `ContextMenuItem::register_hit`.
  - A disabled row registers a `ContextMenu` hit with no event, where before it registered nothing.
  - Every row now notes its name, its disabled state and, when disabled, its reason as the description. Before this,
    menu rows were announced by their humanized id.
  - `chrome_accessibility_nodes` makes a disabled chrome control non-actionable everywhere (the marketplace / dialog law
    convention), so a disabled row is a focusable `menuitem` that is `disabled`, `actionable: false` and described by its
    reason, exactly like `RowAction`.
- **Keys:**
  - `context_menu_enabled_indices` became `context_menu_focusable_indices`, which skips separators only. Arrow keys and
    submenu entry therefore reach disabled rows.
  - Hover and focus by id (`context_menu_path_for_item_id`) reach them too.
  - Digit ordinals stay enabled-only.
  - Enter on a disabled row is still refused. Its hit activation now returns early and keeps the menu open, where before
    the menu closed.
- **Law** `a_disabled_context_menu_row_is_reachable_and_tells_its_reason` (`🧪️wgpu-time-travel` 🍔️ContextMenuReason).
- **Compile:** the wasm32 re-check at 06:57 stopped in `semio-framework-os-kernel` (11 errors from the in-flight ValueError
  sweep: `🧬️semio` `ValueRefusalKind`, `🚪️io:2654` `PackError::TextRefusal`, store `into_value_error`). The renderer was not
  reached. I'll retry after TREE GREEN.

### S3.14 Resume after the second usage cut (07:15 → 10:42)

- Repair check: every wave is whole on disk, and the shell file is unchanged since 06:56. That covers the import transfer
  (struct, begin/step/cancel, Task Manager rows, native `ShellIoCompletion::Import`, browser `run_file_open_request`,
  `cancelImportTransfer` arm, drain step, settle-pump term), F7 context-menu reasons, close/blur cancels, staged facets and the
  UI-engine locale. The coordinator's "cut mid import wave" had already landed whole at 06:2x.
- `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` (10:42→10:45): **Finished, exit 0**
  (`check-wasm-5.txt`). No warning in any region touched this session: import transfer, 🫥️HostWindowBlur, `ContextMenuItem`
  and its key helpers, close-cancel.
- Cold-pair host `loading` (§8 (1)): nothing to change on wgpu (S3.12). Stepped load: done (S3.11).
- Still owed: every native `cargo test` run (the S3.9 list plus the laws of S3.3, S3.4, S3.7, S3.10–S3.13). They wait on the
  native test target, which the peer stdio/sqlite ValueError migration blocks.

### S3.15 Open items, coordinator actions, blockers (current at 10:46)

- **Owed native runs (need the native renderer test target green):** build the test binary with
  `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-w2c cargo test -p semio-framework-os-renderer-wgpu --lib --no-run`,
  then run it with these filters:
  - `time_travel local_folders introspection_tests dialog_choices`: the 52 laws of S3.2 plus the new
    `every_staged_number_row_carries_the_shared_corpus_facets`,
    `the_guest_history_body_opens_pages_and_refuses_edit_with_its_reason`,
    `the_guest_editor_offers_list_and_chip_edits_within_their_bounds`,
    `a_replaying_history_step_or_document_load_shows_its_progress_and_cancels_in_the_mirror`,
    `every_history_lane_refusal_is_a_notice_carrying_its_code`, `a_host_window_blur_blurs_the_active_pane_for_its_program_once`,
    `a_picked_import_is_a_cancellable_task_that_frees_a_started_import`, `a_disabled_context_menu_row_is_reachable_and_tells_its_reason`
    and `a_plain_document_load_is_the_stepped_archive_load_without_members`;
  - the S3.2 regression set (`chrome_overlays_tour_tests board2d_engine_tests … canvas_presence`);
  - `context_menu`, `window_lifecycle`, `navbar_footer`, `shell_input`, `ui_command_wiring`, `scenes::`, for the WorkerCell /
    UI-engine locale wave.
- **Coordinator actions:**
  - one activation + serve of 6112 for the `--renderer=wgpu` probe;
  - the wgpu Vite config, host-io, bridge (`loadAppDocumentPack` removed), transport/worker (`host-window-blur`) and shell
    changes take effect on the next activation.
  - No descriptor or schema regeneration is needed for W2-C's changes.
- **Blockers (peer):**
  - native `--lib --tests`: `🧊️renderer/🦀️.rs:18985-18998` `NativeSocketProbeSnapshot`, `?` from `ValueError` into
    `Result<_, String>` (S3-INFRA's sweep);
  - `check-browser-worker`: `🚀️browser-boot` imports `🖱️ui/🌐️locale/🟦️.ts`, not schema-owned (locale refactor);
  - `test-preview-generated` 2 ✘: `🚪️io/🪶️sqlite-snapshot/🟦️.ts` imports `🌱️value/⚠️refusal/🟦️.ts`, not schema-owned.
- **Cross-shell recommendations (not done, parity first):**
  - announce `HistoryPatch.reprojection` as a polite progress status in both shells' bands (S3-W2B + S3-W2C);
  - give the marketplace's disabled row actions localized reasons (needs `UiTreeItemAction.reason` in the UI crate);
  - feed `ActionArgDef::number_facets` into W1-E's ChromeDialog staged slider (`ChromeDialogFieldKind::Slider`).
- 10:5x: S3-NOTICES (design §20.12 app fault notices) asked about `classify_dispatch_fault_notice`. I'm not editing it, and I
  told them the branch order (read-only → history refusal → history-lane notice → their app notice → `app.command.rejected`
  → generic). If their `kernel::fault_notice` framework table subsumes `HISTORY_NOTICE_LABELS`, branch 3 should be replaced
  rather than duplicated, with `{n}` taken from `Fault.params`. I also listed my regions for them to avoid.

### S3.16 After TREE GREEN (11:38) — 11:39–11:46

- The `NativeSocketProbeSnapshot` sqlite lines were already on the store trait's `ValueError` (the peer sweep landed at
  10:22), so there was nothing to convert.
- **ProgramBridge wasm32 (coordinator's first item).** S3-NOTICES' `ProgramFault` change left two `String` tails, in
  `handle_action_js` and `handle_command_js`. Both now end in
  `.map_err(|error| ProgramFault::from(format!("… result parse failed: {error}")))`.
- **Shared `DocumentArchiveLoadHost` adopted (S3-LOAD).** The native `wasm_program_exchange::load_app_document_archive`
  (its hand-written admit/poll/ack loop deleted) now drives `protocol::DocumentArchiveLoadHost` with `next_seq` and
  `exchange`, the same machine the MCP gateway and `🏃️run` use.
  - Outcomes: `Ready` → Ok; `Cancelled` → the named refusal `document.load-cancelled: … the previous document is unchanged`
    (`DOCUMENT_LOAD_CANCELLED`); `Fault` / `Refused` → the guest fault's message; an unanswered step → named.
  - Cancel on wgpu is the person's Cancel in the history body (`historyEditCancelReplay`, the guest's own cancel).
  - Progress is the guest's `HistoryPatch.reprojection` (`kind: load`), already rendered (S3.11).
  - The browser path is the TS twin `AppChannelClient.loadDocumentArchive`.
- **Blocked again (peer):** both renderer checks, native `--lib --tests` (11:40) and wasm32 (11:44), stop in
  `semio-framework-graph`:
  - `🕸️graph/⚙️engine/🦀️.rs:434,493,560`: `PropertyBag` has no `extend` / `clear`;
  - `🕸️graph/🗣️dsl/🦀️.rs:443`: expected `PropertyBag`, found `BTreeMap`.

  The cause is a peer's `🕸️graph/🛂️manifest/🗂️properties/🦀️.rs` change at 11:39 that has not reached its callers yet.
  Coordinator told.
- 11:51–11:55: the `PropertyBag` callers in `🕸️graph` were fixed at 11:51. wasm32 now stops on 6 errors in the peer's new
  (11:30, uncommitted) `🕸️graph/🛂️manifest/🪆️binding/🦀️.rs:25-39` (`RecordSpecProducer` / `Shape::RecordProducer` /
  `Box<RecordValue>`), part of the graph change still in flight (`check-wasm-7.txt`, `check-wasm-8.txt`). Coordinator told.
- 12:01–12:04: the graph binding was fixed (12:00), and wasm32 got further. It now stops in `semio-framework-os-infinite`:
  `🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:770,2243,5408,6732`, where `DagExpandedPaths` is not found and `&BTreeSet<String>`
  is expected (`check-wasm-9.txt`). That is a peer's in-flight board-DAG change. The renderer has not been reached since 11:40,
  so my 11:4x edits (ProgramFault tails, `DocumentArchiveLoadHost`) are still unchecked.

## Session 4 — 2026-10-04

Agent: S4-WGPU (Opus executor), successor of S3-W2C. Scratch: `🗑️generated/s4-wgpu/`. Brief: `🧭️plan.md` row S4-WGPU, `📓️s4-resume.md` §7 S3-W2C.

### S4.1 Repair-first check (rule 34, 02:2x)

- `git diff HEAD --stat` over the owned paths: Shell `🧊️wgpu/🦀️.rs` +589/−…, `⏪️time-travel` +7, `🧊️renderer` +210, plugin bridge (−`loadAppDocumentPack`,
  −`loadAppArtifactPack`), browser host / transport / frame worker (`host-window-blur`). Every wave of S3 is whole on disk.
- Unstaged (peer) change since then: `🧊️renderer/🦀️.rs:19032` `NativeSocketProbeSnapshot::record_spec` now names
  `semio_framework_dsl_record::{FieldSpec, Shape}` (DSL-record peer). Shell file mtime 10-03 23:30 equals the index (peer sweep).
- S3-NOTICES replaced `time_travel::history_lane_notice_of_fault` by `kernel::fault_notice` (branch 3 of `classify_dispatch_fault_notice`)
  as agreed in S3.15; the shared `fault_tokens` stays (used by `history_refusal_of_fault`).
- `DocumentArchiveLoadHost` adoption lives in `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:417` (S3.16), unchecked until now.
- No wgpu TS caller of `AppChannelClient.loadDocument` remains (S4-BUMP's wave A has nothing to touch in wgpu TS).

### S4.2 First native check (02:08→02:36) and the fixes it asked for

- `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` (`check-native-1.txt`): **20 errors, all in owned files**, all fallout of the
  peer value/DSL extraction: `protocol::{FromValue, ToValue}` is private now (Shell `🦀️.rs:144` + its `DocumentHttpPortDeclarationV1::from_value`),
  `dsl::os_dsl::schema::Number` is gone (`🧪️tests/📤️wgpu-file-open-import` ×4), `store::{ToValue, FromValue, DslValue}` are private
  (`🧑‍🎨engine/🧪️tests/🧊️wgpu-renderer-standalone` ×14). Fixed: the shell imports `semio_framework_value::{DslValue, FromValue, ToValue}`;
  the two tests name `semio_framework_value::{Number, ToValue, FromValue, DslValue}`.

### S4.3 `requestMediaFrames` host cancel (D21) — SOURCE + LAW WRITTEN

- A decoded video's frame dispatches (`frameAction` × n, then `doneAction`, or the single `fallbackAction`) no longer go to
  `deferred_actions` all at once: they become one `ImportTransfer` on the new `TransferLane::VideoFrames` lane (`begin_media_frames_transfer`;
  native `ShellIoCompletion::MediaFrames {controller_id, frames}`, wasm32 directly). The shared drain steps it within the 8 ms import budget,
  the Task Manager lists it ("clip.mp4 · Video frames · <owner> · Running" / "Videobilder … Läuft") with its progress and "Cancel clip.mp4",
  and a Cancel after a delivered frame drops the rest (the `doneAction` never fires) and arms the app's `importAbort {}` once (remodel's
  streamed import, design §15) with the `shell.documentTransfer.import-cancelled` notice. `begin_import_transfer` and the new begin share
  `queue_transfer`.
- Law `a_decoded_video_is_a_cancellable_task_that_frees_a_started_stream` (`🧪️wgpu-time-travel` 📥️ImportTransfer).
- React twin is S4-UI's (`runRequestMediaFrames`).

### S4.4 Marketplace disabled-row reasons (W1E-1 on the marketplace) — SOURCE + LAW WRITTEN

- UI crate (shared, region-scoped): `UiTreeItemAction.reason: Option<Label>` (serde/value default, skipped when `None`), mapped from the
  contract's `RowAction.reason` in `🔀️reconcile::row_action`; the shell maps it back into `RowAction.reason` (only while disabled) in
  `tree_item_row_target`, so `row_action_accessibility_nodes` describes the disabled action by it and keeps it focusable.
- Marketplace verbs carry a localized reason instead of a bare flag: `plugins.reason.installing` (mid-install), `plugins.reason.inUse`
  (the open document's program), `plugins.reason.extensionUnavailable` (a failed extension), en + de.
- Literal sites given `reason: None`: UI tests (`🔬️targets-wgpu-paint-unit`, `🔬️targets-wgpu-component-ui-value-round-trip` ×2,
  `🔬️targets-wgpu-reconcile-unit`), `📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs` ×2, `✏️s/🔌️plugins/🪵️sourcing/…/🧺️curated/🦀️.rs` ×2; the
  generated TS twin `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` `UiTreeItemAction` gains `reason?: Label` (hand-aligned to the typegen shape).
- Law `the_marketplace_announces_a_disabled_uninstall_as_a_disabled_row_action_button` now asserts the reason (en/de), focusability and no
  reason on the enabled sibling.

### S4.5 W1E-3 wgpu half — live reprojection status outside the History panel — SOURCE + LAW WRITTEN (compile owed: plugin/kernel peer reds)

- Input: S4-UI's kernel copy `semio_framework::kernel::history_reprojection_status` (+ fixture `🎠️kernel/🧫️fixtures/🧫️history-reprojection`, 11 cases).
- Shell state `history_reprojection` (seeded by `read_history`, folded by every stale-guarded patch, the progress-patch drain and the native poll,
  which now also polls while a reprojection replays); `⏪️time-travel` region 📡️HistoryReprojectionBand:
  - chrome node `shell.history.reprojection` (`HISTORY_REPROJECTION_STATUS_ID`): `progressbar` over done/total + `aria-busy` while it replays,
    polite `status` while paused or refused; named by exactly the localized status line; per coordinator correction it carries **no**
    description (a refusal's raw code is never announced);
  - its own bottom band (new chrome phase `HistoryReprojectionBand` after `TimeTravelBand`), wrapped at ": " on phones, with the replay track
    while running, Info tone (Warning for a refusal), stacked above the session band when one is open; it dispatches nothing (Cancel replay /
    Replay again stay in the History body's reprojection section);
  - the session band's frame painting is now the shared `paint_chrome_band_frame` (7 steps) both bands use.
- Law `every_history_reprojection_is_announced_outside_the_history_panel` (`🧪️wgpu-time-travel` 📡️HistoryReprojectionStatus): every fixture case,
  en + de: label = fixture text, polite, no raw code, progressbar/status by running, one painted line + track, the band step paints; stacked
  above an open session band with both live nodes; gone once adopted.

### S4.6 W1E-1 + W1E-2 wgpu halves (S4-UI contract `💬️row-semantics` `revealReason` / `selectedRows`) — VERIFIED

- W1E-1 (UI crate `⚡️events`, `⚙️engine`, region-scoped): router `RowReasonHint {row, index, hovered}` + `disabled_row_action_reason` (reads the
  node's `UiTreeItemAction.reason`, filled only from contract `RowAction.reason`, only while disabled). On: pointer entering the icon
  (`hover_row_reason` on move), pointer press on it, accessibility Focus and Activate of the virtual `::row-action::i` (Activate of a disabled one
  dispatches nothing). Off: pointer leaving a hint it revealed, PointerCancel, accessibility Blur, Escape (consumed by the hint first). The engine
  presents it through the retained tooltip path (`PresentedTooltip.row_action`, `synchronize_presented_row_reason`), anchored to the icon slot
  (`EventRouter::row_action_icon_rect`, the slot paint draws and the pointer hits); a dwell hover tooltip never replaces it. Test accessors
  `Ui::revealed_row_reason`, `Ui::row_action_icon_rect`.
- W1E-2: `♿️accessibility` projects `TreeItemProps.selected` as the tree item's `selected` (arena presence only when the contract says nothing);
  `🔀️reconcile::tree_item` paints the chosen option with the selected fill (`presence.selected = selected == Some(true)`). Shell-built legacy
  rows stay `selected: None` (S4-UI's two literal lines).
- Laws (UI crate `🧪️conformance-corpus`): `a_disabled_row_action_reveals_its_reason_on_hover_focus_and_press` (all six triggers + click on the icon,
  description stays the reason) and `option_rows_announce_and_paint_their_selected_state`.

| Command | Result |
|---|---|
| `cargo check -p semio-framework-ui --lib --tests --features testkit` (`check-ui-1..4.txt`) | 1: 2 errors (mine: `PresentedTooltip` test literals) → fixed; 2, 3, 4: **exit 0** |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s4-wgpu cargo test -p semio-framework-ui --features testkit --lib -- conformance_corpus row_action reconcile_unit component_ui_value tree_action` (`test-ui-1.txt`) | **18 ✔ / 0 ✘** |
| same `-- conformance_corpus row_action tree_item accessibility` (`test-ui-2.txt`) | **40 ✔ / 0 ✘** (incl. both new laws + the marketplace-shared row-action laws) |
| `cargo check -p semio-framework-artifact-playbook-playbook --lib` (`check-playbook-1.txt`) | blocked: os-kernel red in `🚪️io/🦀️.rs:2089/2095/2719/2736` (`IoOutcome` vs `(Cow, ArchiveChildren)`, peer W-a seam); my `reason: None` literals there unverified |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` (`check-native-2.txt`, 02:44→02:54) | **exit 0** (includes S4.2–S4.4; S4.5 landed during it) — re-check `check-native-3.txt` stopped in peer `🔌️plugin/🦀️.rs:43416/35871` |
| wgpu TS `bun ./📜️script.ts test-browser ♿️ 📨️ door` (`vitest-browser-1.txt`) | **92 ✔ / 0 ✘** |
| `bun ./📜️script.ts test-preview-generated` (`vitest-preview-1/2.txt`) | 1: 28/29 (✘ "input changed during generation: `🎠️kernel/🟦️.ts`", a peer write mid-run); 2: **29 ✔ / 0 ✘** |
| `bun ./📜️script.ts check-browser-worker` (`check-browser-worker-1.txt`) | browser-boot + renderer-boot checks pass; **frame-worker stale** ("run the generate-frame-worker target") → coordinator regeneration at activation |

### S4.7 Native test target, first batch, and the usage cut (03:41 → 04:15; resumed 06:45)

- `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` (`check-native-4.txt`, 03:41→03:52, after CORE GREEN): **exit 0** — covers
  S4.2–S4.6 (media cancel, marketplace reasons, W1E-3 band/mirror, the UI-crate hint/selected changes).
- `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s4-wgpu cargo test -p semio-framework-os-renderer-wgpu --lib --no-run` (`test-build-1.txt`,
  04:04→04:11): **Finished**; binary pinned at `…/target-nde-s4-wgpu/renderer-wgpu-tests` (run from the crate dir, `RUST_MIN_STACK=67108864`).
- Batch 1 `time_travel local_folders introspection_tests dialog_choices` (`run-batch-1.txt`): **59 ✔ / 4 ✘** — all four in laws S3 wrote and
  never ran, or mine:
  1. `every_history_reprojection_is_announced_outside_the_history_panel` (mine): a long German refusal wraps over two band lines →
     the band now wraps at words and the law joins the lines;
  2. `a_replaying_history_step_or_document_load_shows_its_progress_and_cancels_in_the_mirror`: the `framework.history.reprojection`
     **TreeSection** carries no accessible name on wgpu — `ui_contract::accessibility_projection_node` names no `TreeSection` (asked S4-UI);
  3. `the_guest_editor_offers_list_and_chip_edits_within_their_bounds`: asserted `!actionable` on a disabled retained button, which
     contradicts the shared accessibility fixture (`#card` disabled + actionable); law aligned (disabled + dispatches nothing). A first
     attempt to force `actionable &= !disabled` in `♿️accessibility` broke `a_mounted_document_publishes_the_accessibility_tree_the_shared_fixture_declares`
     (`test-ui-3.txt` 45/1) and was reverted;
  4. `the_guest_history_body_opens_pages_and_refuses_edit_with_its_reason`: the window scrolled to 100 shows no mutation rows — under
     diagnosis (temporary `[DEBUG]` mirror print in the law).
- After the resume: S4-UI's band corpus now expects `TimeTravelLabel::ReplayFaulted` for an unknown replay fault (no raw code) → the wgpu
  band's fault line reads that label; the reprojection band mirrors React's status (label `<title>: <text>`, Cancel replay
  `shell.history.reprojection.cancel-replay` / Replay again `shell.history.reprojection.rerun` while no session is open, no generation),
  law rewritten accordingly.

### S4.8 After the second resume (06:45 → 08:55) and the third (11:35)

- **Rule 43 (CHECKS ONLY, coordinator 08:0x):** no renderer `cargo test`/`--tests`/`--no-run`; my running test build was stopped and my
  uplift dir `⚡️cache/cargo/target-nde-s4-wgpu` (252 MB, incl. the pinned test binary) deleted. Every renderer test run below is
  **OWED (rule 43)**.
- **UI crate:** reverted the `actionable &= !disabled` experiment (shared fixture says a disabled `#card` stays actionable);
  `cargo check -p semio-framework-ui --lib --tests --features testkit` (`check-ui-7.txt`) **exit 0**. `the_guest_editor_offers_list_and_chip_edits_within_their_bounds`
  now asserts disabled + "dispatches nothing" (no `!actionable` on a retained disabled button). Temporary `[DEBUG]` prints removed again.
- **S4-UI contract follow-ups (agent `a2a0ed242ef111cc3`):** `TreeSection` is now named by its label in `ui_contract::accessibility_projection_node`
  (S4-UI owns it), which resolves `a_replaying_history_step_or_document_load_shows_its_progress_and_cancels_in_the_mirror`'s unnamed section;
  the band's unknown-fault line reads `TimeTravelLabel::ReplayFaulted` (no `({code})` any more); the reprojection band mirrors React
  (label `<title>: <text>`, Cancel replay / Replay again controls while no session is open).
- **`plugin.channel-mismatch` on wgpu (S4-BUMP handshake, coordinator relay 08:20) — SOURCE + LAW WRITTEN:**
  - `ProgramBridgeEntry::create_app` answers `Result<u32, ProgramFault>`; the browser path keeps the bridge's rejection `fault`
    (`js_program_fault`, i.e. `createActorApi`'s `Object.assign(new Error(..), { fault })`), the native path wraps the kernel string.
  - Shell: `program_fault_text` (the framework's `kernel::fault_notice` text of a structured refusal, else the call's text) and
    `ShellState::note_refused_open` (records the fault for the dispatch-fault funnel → localized `shell.notice` with the code as
    description). Boot opens tell it and record the plugin fault with the localized text; document opens, session switches, app
    activation and spawned workflows carry the localized text in their status.
  - Law `a_refused_guest_channel_is_told_as_its_localized_notice` (`🧪️wgpu-fault-notices`) over `📡️spr/🧵️channel/🧫️fixtures/🧫️channel-handshake`:
    every refused case, en + de, both channels named, mirrored politely with the code as description, never the raw code or English.
- **Checks:** `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` (`check-wasm-1.txt` 08:18→08:37): 2 errors,
  mine (callers expecting `String` from `create_app`) → fixed; `check-wasm-2.txt` (08:45→08:52) **exit 0**. Native `--lib` (`check-native-5.txt`,
  08:09→08:15) **exit 0** before the handshake change; `check-native-6.txt` 11:35 died on the disk guard sweeping build units
  ("failed to write … invoked.timestamp"), re-issued as `check-native-7.txt`.

### S4.9 Probe readiness: the History body is a scroll region on wgpu (11:5x) — SOURCE + LAW WRITTEN (cargo frozen, rule 44)

- Gap found reading `🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts`: the wgpu probe's `scrollHistory` / `scrollHistoryBy` (used by `pageHistory`,
  `allHistoryRows`, `findMutationRow`, `revealHistory`, `readAlternatives`) look for a `dumpChrome` hit whose kind matches `/scroll/` and whose
  control or window id matches `/history/`. A docked/mobile panel's retained body registered only its row hits, so no such hit existed and
  the probe could never page the windowed History body (only the first screen of rows was reachable).
- Fix: `ShellState::register_retained_panel_scroll_region` registers the panel's whole content rect as `HitKind::ScrollRegion`
  `<surface>.scroll` (`framework.panel.history.scroll`), BEFORE the row hits (rows stay on top), owned by the surface in
  `retained_hit_windows`, so a wheel over the gap below the rows scrolls the body like React's overflow container, and `dumpChrome` names it
  (kind `ScrollRegion`, owner window = the panel). Called in `render_panel_step` and `render_mobile_panel_step`; dock windows unchanged.
- Laws: `a_panel_body_is_one_scroll_region_under_its_rows` (new); `the_guest_history_body_opens_pages_and_refuses_edit_with_its_reason`
  now wheels the window at 100 into view (`wheel_history_window_into_view`) — the mirror projects what the body shows, which is why the
  window at 100 read `[]` in batch 1.
- Native `cargo check --lib` re-issue (`check-native-8.txt`) was stopped at the CARGO FREEZE (rule 44) before reaching the renderer.
  OWED (rule 44): native `--lib` check (wasm32 green 08:52 covers everything but S4.9), then the rule-43 test runs.

### S4.10 Nested-cargo catalog drift (S4-GATES assignment, low priority) — ANALYZED, law green, re-seal NOT done

- `💥️nested-cargo-collision-authority` (`bun test ./…/📚️library/🧪️tests/💥️nested-cargo-collision-authority/🟦️.ts`, `nested-cargo-1.txt`):
  **26 ✔ / 0 ✘** — the law does not require catalog destinations to exist.
- The 10 of 32 wgpu `mappings[].destinationPath` rows of `📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json` that name no
  live file, with what happened to each (git history):
  - moved by `8add1df1473` (the sealing commit itself): `📦️packages/🦀️rust/{package.json, 📋️project.json, 📜️script.ts}` →
    `📦️packages/🟦️typescript/…`;
  - `b2064cc237b`: `📦️packages/🦀️rust/🌐️.html` → `🌐️server/🌐️.html`; `📦️packages/🦀️rust/Trunk.toml` deleted (Trunk retired);
  - `7bea15c349b`: `🪢️kernel-seam/🦀️.rs` deleted; `025ec86a429`: `📦️packages/🦀️rust/🟦️typescript/🧪️test/🟦️s.ts` deleted;
  - `🧪️tests/🟦️{browser-frame-transport, browser-interactive-job-port, package-integration}.ts` → the engine test tree
    (`🧑‍🎨engine/🧪️tests/{📨️browser-frame-transport, 🎮️browser-interactive-job-port, 🧩️package-integration}/🟦️.ts`).
- Why not re-sealed now: the ledger's reverse law (`🕰️historical-json-source-encoding`) only reverses DIRECTORY renames (`to` followed by
  `/`); whole-file moves and deletions cannot be expressed. A truthful re-seal needs a schema + law extension of `🧫️frozen-seal-ledger`
  (`moves: [{from, to}]` exact JSON-string paths, reversible; deleted rows need a decision: keep as historical rows with a `retired`
  marker, or drop them with the removed fragment recorded). Proposal sent to `main`; implement on GO.

### S4.11 Nested-cargo re-seal attempt (coordinator GO 12:3x) — REVERTED, ledger extension kept

- Kept (backward compatible, verified): `🧬️schema/🔣️frozen-seal-ledger` catalog re-seal rows may carry exact-path `moves [{from, to, revision}]` and
  `deletions [{path, reason, revision, packageId, index, row}]` beside `renames` (`anyOf` one of the three); law `🕰️historical-json-source-encoding`
  reverses deletions (re-inserting each removed row at its index, canonical `JSON.stringify(…, null, 2)`), then moves (exact JSON strings), then
  renames, and requires the previous seal byte for byte. Input script `T/🧪️s4-wgpu-nested-cargo-reseal.py` (starts from the index copy; `--check`).
- Applied then reverted (12:39 → 12:5x): the re-seal itself. It broke every taxonomy run ("Nested Cargo catalog digest drift", then
  "boundary drift", then "joined-path authority: consumer must have one exact mapped owner"): `🔍️discovery/🟦️.ts` hard-codes the contract
  counts (`sourceLeafCounts [32, 4]`, `purityCount 27`) and the `🧪️browser-frame-transport.test.ts` row is the consumer of the one pinned
  `joinedPathBinding` (with the authored-fragment census). Moving files out of the wgpu owner root is also refused by the member check
  (`members-of-wgpu-target` has no `🌐️server`; engine tests are outside the owner). Catalog, taxonomy pin/counts, discovery and ledger fixture are
  back to the index bytes (`git diff` empty); `verify taxonomy report --scope …/📽️nested-cargo-package-projection` exit 0 (1 pre-existing
  `directory-kind-unresolved`), 🕰️ **25/0**, 💥️ **26/0**.
- Needed for a real re-seal (library contract owner): retire or re-point the joined-path binding and its authored fragments, decide the
  `🌐️server` member registration, then re-seal catalog + counts + taxonomy pin + ledger row in ONE write.
- Other TS this hour: wgpu `test-browser` **92/0** (`vitest-browser-2.txt`); `test-preview-generated` **27/29** (`vitest-preview-3.txt`), both ✘
  are a peer's `🚪️io/🪶️sqlite-snapshot/🟦️.ts` importing `⏳️async/🪃️continuation/🟦️.ts`, not schema-owned for the WGPU browser build.

## Session 5 — 2026-10-05

Agent: S5-WGPU (Opus executor), successor of S4-WGPU. Scratch: `🗑️generated/s5-wgpu/`. Brief: coordinator launch message (P1–P8 = the wgpu rows of
`📓️audit-s5-parity.md` §4 + design §22.2/§22.7), `📓️s5-resume.md` §1.6. Rules 1–54.

### S5.1 Repair-first check (rule 46, 00:20–00:30)

- `git diff HEAD --stat -- 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer`: **empty** — every S4 wave of this WP
  is inside commit 670 (10-04 19:39). All mtimes read 10-04 18:27 (the stash/pop incident), so mtimes prove nothing; the index is the evidence.
  No half-finished edit found. One staged peer file in the UI crate: `🖱️ui/🌐️locale/🏷️label/🦀️.rs` (not mine).
- State at launch: `landing` + `serve` HELD by COORDINATOR-ACTIVATION (00:14) → no save under `🧰️framework/**`; reading and staging only.
- Still owed from S4 (unchanged): native `cargo check -p semio-framework-os-renderer-wgpu --lib` after S4.9 (History scroll region), every renderer
  `cargo test` batch of `📓️s5-resume.md` §1.6.

### S5.2 Wave 1 (P1 + the projection halves of P2/P6) — STAGED, anchors verified, not landed (landing HELD) — 00:55

Staged as one exact-anchor script: `🗑️generated/s5-wgpu/wave1/land.py` (`--check` / `--land` / `--revert`; 21 files, +1085/−155 lines; pre-wave bytes are
saved to `wave1/orig/` at landing). `--check` at 00:52: **21 files planned, 0 anchor failures**.

Findings that shaped it (read on disk, not in the audit):
- The mirror's `Value` path was also wrong in UNITS: the mirror speaks display units (`value_now` = stored × `displayFactor`), the engine clamped the
  typed text against STORED bounds — a degree dial written "90" through the mirror committed 90 rad clamped to π.
- A native `<input type=range>` cannot hold a text outside its travel (the browser sanitises; Playwright's `fill` throws "Malformed value"), so a typed
  out-of-bounds slider value can only go through the typed readout. The readout existed in the mirror only while a canvas double-click had opened it.
- The canvas stepper path was not §18-complete: Home/End moved the caret, Shift+Arrow was one rung, Arrow bypassed `ui_number_key_value`, and a key after a
  refused draft started from the refused text and kept the stale refusal. Number `Input`s answered only PageUp/PageDown.

What the wave does:
1. **One typed path** (`🖱️ui/🎯️targets/🧊️wgpu/⚡️events`): a mirror `Value` on an `Input`/`IconSelect`/`NumberStepper` writes the edit buffer as typed and runs
   the canvas keystroke path (`normalize_stepper_edit` → `push_buffer_change`: `typed_number` → refusal or exact dispatch, press semantics included); on a
   `Slider` it runs the typed-readout law (`finish_slider_readout_edit`: display units read back, a detent kept exactly, beyond the soft travel admitted
   while the limits admit it, beyond a hard limit refused with the draft kept). No clamp is left on the mirror path.
2. **Field-key law** `number_field_key` (twin of `uiNumberFieldKey`): stepper keys = `ui_number_key_value` for ArrowUp/ArrowDown (Shift = ten rungs),
   PageUp/PageDown, Home/End toward a bound the stepper has; every press reports (absolute: the law's value; delta: step × rungs, or the move); the law's
   value is shown and an earlier refusal cleared. Number `Input`s answer the same keys in their buffer. A refused draft is never a stepper's live value.
3. **Slider readout always addressable**: an enabled bound slider projects `<key>::editor` (spinbutton, Tab stop only while a draft is open, min/max = the
   hard limits in display units); focus opens the draft on the shown value, a value replaces it, Enter commits/refuses, Escape reverts, blur drops an
   unrefused draft — the existing canvas law. The engine gate that refused the address while no edit was open is removed.
4. **Projection fields** (contract region 🔖️AccessibilityProjection, coordinator GO 00:4x): `value_step` (display units; declared step, else 10^-precision,
   else 1 for slider/stepper), `invalid` (a refused open draft), `set_size`/`pos_in_set` (a windowed row's place in the WHOLE list: section rows, nested
   rows of a windowed row, table rows), `tone` (tree rows: info/success/warning/danger); `AccessibilityValue.step`; TS twin; shared fixture rows.
   Every row action is projected (`<row>::row-action::<i>`), Menu-placed ones included (gap 5).
5. **DOM mirror + keyboard wire** (`♿️accessibility-mirror`, `🎮️input-wire`): `step` (else `any`), `aria-invalid`, `aria-setsize`/`aria-posinset`,
   `data-tone`; a control that publishes its step lists its engine-owned keys in `data-engine-keys` (slider: arrows/page/Home/End; stepper: ArrowUp/Down,
   PageUp/Down, Home/End toward a bound, Enter, Escape; `::editor`: Enter, Escape). Those keys are forwarded to the engine instead of the browser, after the
   mirror re-asserts `accessibility-focus` on the same lossless lane (so an engine-side blur — Enter on a stepper — cannot leave a dead key).
6. **Shared corpus first** (`🧫️number-controls`, schema then rows): `fieldKeys` (15 rows: the law key a physical key names) and `valueStep` on every
   `valueTexts` row; `⌨️browser-keyboard-scope` gains target `mirror-number` + 9 cases.

Laws written (run after landing): UI crate `🔬️targets-wgpu-events-unit` region 🪞️MirroredNumberLawTests — `every_field_key_row_names_the_law_key_the_shared_corpus_declares`,
`stepper_keys_answer_the_shared_keyboard_law_rows`, `a_mirrored_stepper_value_takes_the_typed_path_of_the_canvas`, `a_mirrored_slider_value_takes_the_typed_readout_path`;
`🔬️targets-wgpu-accessibility-projection` — `windowed_rows_project_their_place_in_the_whole_list` + step/invalid/tone assertions; contract Rust + TS
twin laws read `valueStep`/`tone`/`fieldKeys`; TS `⌨️browser-keyboard-scope` — the 9 fixture cases + one mirror/wire integration test.

Pre-landing verification that needs no save (bun, staged files against the LIVE twins): `bun 🗑️generated/s5-wgpu/wave1/precheck.ts` — number-controls
fixture valid against its staged schema, **15/15** `fieldKeys` rows = `uiNumberFieldKey`, keyboard-scope fixture valid (33 cases);
`bun …/precheck2.ts` — staged contract TS twins over the staged fixtures: accessibility-projection **315 checks**, number-controls **296 checks**, 0 failures.
Rust is unverified until the lock frees (no build is possible outside the live tree).

Mirror contract changes for the probe (sent to `main` 00:4x): the refusal is the CONTROL node's description + `aria-invalid` (never the `.row`'s);
sliders are typed through `<key>::editor` then Enter; rows carry `aria-setsize`/`aria-posinset` (wgpu totals are now readable); Menu-placed row actions
(Backwards) appear as `<entry row>::row-action::<i>` buttons.

### S5.3 Wave 1 LANDED (05:00–05:15) — native + TS verified; wasm32 and Rust tests running next

Timeline: staged 00:55; the lock was unreachable until 04:59 (activation hold, five first-come losses, then FIFO rule 60, the 02:40–04:22 usage cut, the
BUILDING warm-up). Since staging the wave grew by: P3 (stable band `status` + separate `progressbar` nodes, both projected beside a modal), the live React
findings F2 (band controls ≥ 24 × 24 px: `CHROME_BAND_CONTROL_MIN`) and O5-surface (the band fill is composited opaque over the menu surface;
a Warning band was a 10 % tint over the canvas), and an exclusive-bound readout law. Anchors re-checked against the tree after S5-UI v1 and S5-RUNTIME A–C: 0 failures.

| Step | Command | Result |
|---|---|---|
| part a (12 Rust files) under `landing` 04:59:39 → 05:11:41 | `python3 wave1/land.py --land a` | landed |
| native verifying check (gate v5, `CARGO_BUILD_JOBS=3`; cold after the 04:22 prune) | `cargo check -p semio-framework-ui-contract -p semio-framework-ui -p semio-framework-os-renderer-wgpu -p semio-framework-plugin --lib --message-format=short` (`check-native-s5-1.txt`, 04:59:58 → 05:11:17) | **exit 0**, `Finished` in 11m 18s; warnings present (ui 48, renderer 274, plugin 283 — the type-check ran); no new warning in my files |
| part b (11 TS/JSON files) under `serve` 05:11:56 → 05:14:49 | `python3 wave1/land.py --land b` | landed |
| contract TS twins on the landed files | `bun …/🧪️tests/🔬️accessibility-projection/🟦️.ts`; `bun -e 'numberControlsSelfTests()'` | **315 checks**, **296 checks**, 0 failures |
| wgpu mirror suites | `bun ./📜️script.ts test-browser ⌨️ ♿️ --reporter=verbose` (wgpu TS package; `vitest-browser-s5-3.txt`) | **48 ✔ / 0 ✘** (`♿️wgpu-accessibility-interaction` 27, `⌨️os-command-shortcuts` 23… 2 files) |
| keyboard wire + mirror law | `bun ./📜️script.ts test long browser-keyboard-scope --reporter=verbose` (React renderer package owns that suite; `vitest-keyboard-scope-s5-2.txt`) | **47 ✔ / 0 ✘** incl. the 9 new `number-*` corpus cases and `hands a mirrored number control's law keys to the renderer…` |
| strict tsc of the 4 edited TS files | `bunx tsc --noEmit -p 🗑️generated/s5-wgpu/tsc-wave1.json` | **exit 0, 0 errors** |

Files (23): contract `♿️accessibility/{🦀️.rs,🟦️.ts}`, `🧪️tests/{🔬️accessibility-unit/🦀️.rs,🔬️accessibility-projection/🟦️.ts,🧪️number-controls/🟦️.ts,🔬️component-unit/🦀️.rs}`,
`🧫️fixtures/{♿️accessibility-projection.json,🧫️number-controls/🔣️.json,🧫️number-controls/🧬️schema/🔣️.json}`; UI target `🎯️targets/🧊️wgpu/{⚡️events,⚙️engine,♿️accessibility}/🦀️.rs`,
`🧪️tests/{🔬️targets-wgpu-events-unit,🔬️targets-wgpu-accessibility-projection}/🦀️.rs`; renderer `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` (8 literals),
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (4 literals + band nodes), `🐚️Shell/🎯️targets/🧊️wgpu/⏪️time-travel/🦀️.rs`, `🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs`,
`🎯️targets/🧊️wgpu/{♿️accessibility-mirror,🎮️input-wire}/🟦️.ts`, `🧬️schema/⌨️browser-keyboard-scope/🔣️.json`, `🧫️fixtures/⌨️browser-keyboard-scope/🔣️.json`,
`🧪️tests/⌨️browser-keyboard-scope/🟦️.ts`. Pre-wave bytes: `🗑️generated/s5-wgpu/wave1/orig/`.

### S5.4 After wave 1 (05:15–05:40): wasm32 green, UI test build blocked by a peer red, waves 2 + 3 staged

- Renderer wasm32: `CARGO_BUILD_JOBS=3 cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format=short`
  (`check-wasm-s5-1.txt`, 05:20:18 → 05:24:05) **exit 0**, 241 warnings. With S5.3: "WGPU WAVE 1 GREEN" at `--lib` level, sent to `main` 05:25.
  (Correction to S5.3: the `test-browser ⌨️ ♿️` run is 2 files / **48 tests**, `⌨️os-command-shortcuts` + `♿️wgpu-accessibility-interaction`; the per-file counts quoted there were line counts.)
- UI-crate laws: `RUST_MIN_STACK=67108864 CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-s5-wgpu cargo test -p semio-framework-ui --features testkit --lib --message-format=short -- events:: accessibility conformance_corpus row_action table_row scrub control_commit dispatch_accessibility`
  (`test-ui-s5-1.txt`, 05:24:55 → 05:26:46) **did not build**: `semio-framework-pixels` `🔲️pixels/🧩️compositing/🗂️layers/🦀️.rs:125:76` E0599 (no method `into_result` on
  `compositing::CompositeJob`) — a peer's crate the ui TEST build reaches and the `--lib` check does not. Reported to `main`. **OWED**: the same command once pixels is green.
- Staged `🗑️generated/s5-wgpu/wave2/land.py` (`--check a|b`: 12 Rust + 1 generated TS, 0 anchor failures at 05:35), two modules:
  - `edits_tone` (P2, gap 2): `UiTreeItemNode.tone` (the four outcome roles), `reconcile::tree_row_tone`, `Theme::tone_ink` (info = token `colors::INFO`,
    success/warning/danger = the theme's outcome inks), both row painters colour label and icon, every literal without a `..base` spread gains
    `tone: None` (component 8, reconcile 1, 4 UI tests, playbook 3 — counts asserted), the generated manifest TS twin; law `a_tree_row_carries_only_a_semantic_tone`.
  - `edits_copy` (P4 + P5, gap 7, coordinator relays 05:10/05:27): the shell embeds the shared corpus `🧫️time-travel-band` and resolves `band_label(key, locale)`
    from its `labels` table and the refusal table from its `refusals` rows — every hand-written EN/DE band string in `⏪️time-travel/🦀️.rs` is deleted
    (`TimeTravelVerb::label`, band lines, indicator, severity words, peers, `HISTORY_REFUSALS`); `TimeTravelVerb::NextProblem`
    (`shell.time-travel.next-problem`, `historyEditBegin {mutationId, store?}` from the session's own `next_problem`, offered first in a blocked review);
    `TimeTravelTransition.scroll_to` (the edge into a review that names its first blocking mutation reveals the panel again and names
    `framework.history.mutation.<id>`); `history_refusal_is_silent` guards both notice sites (`timeTravel.stale`); band controls are `Theme::size_large()`
    (9 × ui spacing = 28.8 px) on both axes. Laws updated: captions and every corpus label read from the corpus, controls compared with `controlId`/`args`,
    transitions with `scrollTo`, peers copy.
- Not in these waves (wave 4): the `beginner` label tier (the functions still take `(terminology, locale)`; the driver's `label_tier` is not read yet),
  scrolling the named row into view, a reserved band row above the footer (O5 layout half), keyed retained control state across a snapshot refresh (React F1 twin).

### S5.5 Waves 2 + 3 LANDED (05:42–05:52), first renderer law run, wave 4 staged (06:30)

| Step | Command | Result |
|---|---|---|
| part a (12 Rust files) under `landing` 05:42:40 → 05:52:12 | `python3 wave2/land.py --land a` | landed |
| native verifying check | `CARGO_BUILD_JOBS=3 cargo check -p semio-framework-ui -p semio-framework-os-renderer-wgpu -p semio-framework-artifact-playbook-playbook -p semio-framework-plugin --lib --message-format=short` (`check-native-s5-2.txt`, 05:46:49 → 05:48:17) | **exit 0**, renderer 276 warnings |
| renderer wasm32 | `… cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` (`check-wasm-s5-2.txt`, 05:50:06 → 05:52:04) | **exit 0**, 243 warnings |
| part b (generated manifest TS twin) under `serve` 05:52:13 → 05:52:23 | `python3 wave2/land.py --land b`; `bunx tsc --noEmit -p 🗑️generated/s5-wgpu/tsc-wave2.json` | landed; **exit 0, 0 errors** |
| renderer laws type-check (mine + S5-RUNTIME's literals) | `… cargo check -p semio-framework-os-renderer-wgpu --lib --tests` (`check-tests-s5-1.txt`, 05:53:02 → 05:54:35) | **exit 0** (lib test: 938 warnings) |
| renderer laws, filter `time_travel` | `RUST_MIN_STACK=67108864 CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-s5-wgpu cargo test -p semio-framework-os-renderer-wgpu --lib --message-format=short -- time_travel` (`test-renderer-s5-1.txt`, finished 06:08) | **34 ✔ / 3 ✘** (1624 filtered out) |

Green among the 34 (all on the landed waves 1–3): `the_shared_band_corpus_holds_on_wgpu` (new copy, `controlId`, `nextProblem` args), `the_shared_band_transitions_hold_on_wgpu`
(`scrollTo`), `every_control_reads_reacts_caption_and_names_its_reserved_action` (every corpus label + silent refusals), `the_band_announces_its_message_politely`
(stable status + progress child), `every_history_reprojection_is_announced_outside_the_history_panel`, `the_band_lays_out_above_the_footer_with_its_buttons_in_stage_order`
(`size-large`, opaque), `the_shared_peer_history_edit_corpus_holds_on_wgpu`, `history_refusals_are_localized_notices_carrying_their_code`, `the_finalize_prompt_is_operated_by_keyboard_alone`,
the S4 laws never run before: `a_decoded_video_is_a_cancellable_task_that_frees_a_started_stream`, `a_replaying_history_step_or_document_load_shows_its_progress_and_cancels_in_the_mirror`,
`beginning_an_edit_focuses_the_first_editor_input_in_the_published_projection`, `every_staged_number_row_carries_the_shared_corpus_facets`.

The 3 ✘ (none asserts anything waves 1–3 changed; all three are inherited laws that had never passed):
1. `a_panel_body_is_one_scroll_region_under_its_rows` (S4.9) — harness: `ui-doc begin refused … fault=Deadline`, then "retained presented input candidate could not be sealed".
2. `the_guest_history_body_opens_pages_and_refuses_edit_with_its_reason` (S3/S4.9) — "`m-100` never scrolled into the History body", with `begin refused … fault=InterruptedClose` repeated.
   Both look like the stepped document ingress missing its deadline under fleet load (parallel test threads, ~10 rustc); a single-threaded re-run was started at 06:12 and
   did not build — S5-CHANNEL's wave B was in flight (`🌊️flow/🖥️host/🦀️.rs:2448` E0063 missing field `description`, theirs). **OWED** after "LOCKS OPEN":
   `… cargo test -p semio-framework-os-renderer-wgpu --lib -- a_panel_body_is_one_scroll_region the_guest_history_body_opens_pages the_guest_editor_offers_list --test-threads=1`.
3. `the_guest_editor_offers_list_and_chip_edits_within_their_bounds` — "the list row names its count and ceiling": the row description the guest publishes no longer contains
   `Items: 2 · Maximum 2 items`; wave 4 makes the law print the row so the next run names the guest's actual text (owner of the copy: S5-RUNTIME's editor rows).

Wave 4 staged (`🗑️generated/s5-wgpu/wave4/land.py`, 3 files, 0 anchor failures at 06:28; lands after "LOCKS OPEN"): the `beginner` label tier — `BandTongue { locale, beginner }`,
every band-copy function takes `impl Into<BandTongue>` (a bare `Locale` = `normal`, so no existing caller changes meaning), `ShellState::band_tongue()` reads the UI driver's
`label_tier` (`UiDriverLabelTier` re-exported from the UI target), the corpus table keeps all four texts per key; law: every key × locale reads its `beginner` text and the
painted Accept caption follows the driver.

Still open after wave 4 (in priority order): (1) whatever the first wgpu probe run reports; (2) design §22.7 wgpu half — Segmented (contract projection `radiogroup` + always-projected
`radio` options, engine activation without the popup, segment paint), multi-line text (`text_input_key` law: Enter = newline, Ctrl/Cmd+Enter = commit, Escape = revert; multi-line
layout/paint/caret), IconSelect already a first-class retained control; (3) the reserved subfooter row (bands under the footer; needs one `ShellState::subfooter_height` read by `body_rect`,
the footer step and the three band plans — session, reprojection, folder offer — and the position asserts of `🧪️wgpu-time-travel` + `🧪️wgpu-local-folders`); (4) scrolling the row a
blocked review names into view (`TimeTravelTransition.scroll_to` is computed and pinned; nothing consumes it yet); (5) a law that an open readout draft and focus survive a snapshot
refresh keyed by node KEY; (6) the automatic check-in twin (S5-LOAD corpus `🧫️automatic-checkin`).

### S5.6 Wave 4 LANDED across the 07:45 usage cut (07:38 landed, 07:49 green, lock released 09:36); wave 5 staged

- **Repair-first after the cut (rule 64, 09:36):** I still held `landing` (07:38:48). Evidence on disk: `wave4/orig/` holds the 6 pre-wave files, every wave-4 marker is
  present in its file (`BandTongue` ×21 and `subfooter_height` in `⏪️time-travel/🦀️.rs`, 3 `subfooter_height` reads in the shell, 0 `TIME_TRAVEL_BAND_GAP` left,
  `UiDriverLabelTier` re-exported, the folder plan on `height`), and the verifying check had finished before the cut:
  `CARGO_BUILD_JOBS=3 cargo check -p semio-framework-ui -p semio-framework-os-renderer-wgpu -p semio-framework-plugin --lib --message-format=short`
  (`check-native-s5-3.txt`, 07:39 → 07:49) **exit 0**, `Finished` in 2m 20s, renderer 276 warnings. Nothing was half-written; nothing restored. Lock released 09:36:44,
  "LANDING RELEASED" sent to `main`.
- **Wave 4 content** (6 Rust files: UI target `🦀️.rs` re-export, `⏪️time-travel/🦀️.rs`, shell `🦀️.rs`, `📎️local-folders/🦀️.rs`, laws `🧪️wgpu-time-travel`, `🧪️wgpu-local-folders`):
  (a) `beginner` label tier — `BandTongue { locale, beginner }`, every band-copy function takes `impl Into<BandTongue>`, `ShellState::band_tongue()` reads the UI driver's `label_tier`,
  the corpus table keeps `normal`/`beginner` × en/de per key; (b) the reserved subfooter row — `chrome_band_layout` puts a band's lower edge on its floor, the session band's floor is the
  screen bottom, the reprojection band stacks on it, the folder offer on both; `ShellState::subfooter_height` is subtracted by `body_rect`, the footer step and the error line;
  laws `the_bands_reserve_the_subfooter_row_under_the_footer` (desktop + phone: body, footer, band never overlap; the body gives exactly the reserved row) and the tier law; the list-row
  law now prints the row it read.
- **OWED for wave 4** (no verdict yet): renderer wasm32 `--lib` (my 09:37 attempt never left the old gate loop — the Codex peer's private-dir cargos kept `pgrep -x cargo` ≥ 4; stopped at 09:56,
  no cargo started); it is folded into wave 5's verification (same crate, same target). Coordinator action when green: "re-activate wgpu" with build B2 (waves 4 + 5).
- **Wave 5 staged** (`🗑️generated/s5-wgpu/wave5/land.py`, 5 Rust + 3 non-Rust files, anchors verified 09:57 against the tree with wave 4 in it) — design §22.7 Segmented, wgpu half:
  contract role `radiogroup` for `SelectProps.appearance == Segmented` and no `expanded` (Rust + TS twin + one shared fixture row); the wgpu walk always projects the options as `radio`
  nodes (`<key>::option::<value>`, checked, `aria-setsize/posinset`, ONE Tab stop = the chosen option, the first while none is chosen), the group itself no Tab stop, no listbox;
  the engine admits a radio's activation without an open popup; ArrowRight/ArrowDown and ArrowLeft/ArrowUp move the choice from the published one (wrapping, RTL-mirrored) and dispatch
  `Change` at once; the DOM mirror moves focus and activates on arrows/Enter/Space inside a radio group; law `a_segmented_select_projects_a_radio_group_with_one_tab_stop`
  (projection + mirror activation + arrow keys, no popup opened). Not in wave 5: the canvas still paints a segmented select as its menu trigger (segment-row paint and pointer hit are open).
- Gate v6 (rule 66) from here on: `zsh T/🚦️gate.sh && CARGO_BUILD_JOBS=3 cargo …`; test builds `zsh T/🚦️gate.sh 3 25 && …` on the SHARED target (no private `CARGO_TARGET_DIR`).

### S5.7 Waves 5–9 LANDED (10:1x–11:13): live faults F10 + F11 on wgpu, the publication lane, loud drain faults

| wave | train line | files | what | verified by a run of mine |
|---|---|---|---|---|
| 5 segmented | (S5.6) | 8 | `radiogroup` + `radio` options, one Tab stop | wasm32 `--lib` exit 0 (`check-wasm-s5-3.txt`, 17 m 58 s, 244 renderer warnings) |
| 6 text keyboard law + total `band_label` | `10:2x wave6` | 6 `.rs` | `text_field_key` consumes `ui_contract::text_input_key`; multi-line Enter inserts, Ctrl/⌘+Enter commits, Escape reverts; caret-line view; `band_label` total (`BAND_LABEL_MISSING` = U+FFFD is a law failure) | same wasm32 run, exit 0; train FRAMEWORK GREEN 10:37:26 |
| 7 F10 | `10:47:49 wave7-F10` | 2 `.rs` | `resolve_language_axes`: locale = lock → stored → host locale; terminology = lock → stored valid id → `Native` (React's seed). A fresh profile boots | train GREEN 10:50:43 (native `--lib`); law `a_first_visit_derives_both_language_axes_and_boots` **not run** |
| 8 F11 boot | `10:49:47 wave8-F11b` | 4 `.rs` | `settle_boot` → `announce_session_example()` (React's `useInitialExampleReadiness`), `apply_boot_example` deleted | train GREEN 10:50:43; source-shape law **not run** |
| 9a publications (Rust) | `11:12:29 wave9a-publications` | 4 `.rs` | see below | train FRAMEWORK GREEN 11:17:45 (coordinator relay); wasm32 + laws **not run** (rule 68, activation B2) |
| 9b publications (TS + corpus) | `11:12:51 wave9b-publications` | 4 | see below | `tsc -p 🧪️w2-c-typecheck-progress-bridge.tsconfig.json` exit 0, 0 errors (`tsc-wave9-1.txt`); `bun ./📜️script.ts test-preview-generated` **29 ✔ / 0 ✘** (`vitest-preview-s5-9.txt`) |

**F11, read from code and from `s5-e2e/wgpu/wgpu-contact-console.txt`.** Two independent causes, plus one silence:

1. *Boot never announced the example* (wave 8). The wgpu shell resolved the boot example for its own picker only; React
   dispatches `setActiveExample` for every session whose document is not initialized. Evidence: one
   `plugin_exchange entry instance=1 command=true` at 5.2 s and no `setActiveExample` in the boot console.
2. *The host dropped everything a typed operation publishes after its host call* (wave 9). A typed operation's admitting
   reply carries no outcome; its `AppFrame::OperationCompleted` is the only carrier of the final `UiDirtyScope` and
   command-log delta. On wgpu that frame reached `turnOutcomes`, matched no waiter in
   `AppChannelClient.pumpOutcomes`, and `publishOperationCompletion` returned at `completionListeners.size === 0` — this
   target subscribed nowhere. The uncorrelated UI-progress `Invocation` of the standing drain went to the leftover
   ledger (`pendingTurnEffects`), folded only by the NEXT dispatch; only its history patch was carried per frame
   (`stashProgressHistoryPatch`). An example load (~370 one-item steps, far past `WGPU_TYPED_OPERATION_SETTLE_LIMIT = 64`)
   therefore changed the document and left the surface and the History rows at the pre-load render. The replay behind
   an accepted history edit takes the same drain: same hole by construction (read from code, not observed live).
3. *A drain failure was silence.* `drainTypedOperations`' `catch` pushed `{ instanceId, error }` onto `turnOutcomes`;
   the pump does `this.pending.shift()?.reject(error)` — with no waiter nothing, with an unrelated in-flight command a
   misattributed rejection. In-call throws were already loud (they reject the waiter → `note_dispatch_fault`).
   A second silence sat in Rust: `take_progress_history_patches_js` ended in `.unwrap_or_default()`.

**Wave 9 mechanism.** One per-instance lane replaces the history-only progress queue:
`WgpuOperationPublication { completed, uiScope?, historyPatch?, fault?, resync? }` (bridge) ↔
`program_bridge::OperationPublication` (Rust), JS door `takeOperationPublications` (was `takeProgressHistoryPatches`).

- Completions: the bridge subscribes the channel at `createApp`
  (`channel.onOperationCompleted(… stashOperationPublication(instanceId, operationCompletionPublication(completion)))`),
  so every completion that reaches `turnOutcomes` — in a host call, on the standing drain, from a job drain, from a
  backbone turn's `unsolicited` frames — is delivered by the pump's own classification. `shellFrameAnswersACaller` is
  unchanged (the coordinator's sketch flipped it; the subscription needs no reroute and keeps `Ephemeral` on the frame lane).
- Progress: the standing drain hands each uncorrelated `Invocation` (scope + patch) to the lane
  (`stashOperationProgress`) and CONSUMES a frame that says nothing else, so it is no longer folded a second time over
  the next dispatch's own scope; a frame with an output, diagnostics or mutations keeps its place in the leftover.
- Bounded at 64 entries. A history patch is a delta, so overflow never drops the oldest: the queue collapses into one
  `{ uiScope: full, resync: true }` and the shell re-reads `read_history` (`reread_history_projection`, split out of
  `seed_history_snapshot`).
- Shell: `pump_sync_events` → `drain_operation_publications`: each patch through `observe_invocation_history`, the
  union of `operation_publication_refresh(publication)` to `owe_refresh` — the settle lane refreshes with no further
  dispatch. A completion with a patch and no scope owes `framework.body.history` (React's
  `typedOperationCompletionRefreshV1`); progress with a patch alone re-renders nothing.
- Loud: the drain `catch` does `console.error` and queues a `fault` publication → `note_dispatch_fault` (notice) +
  `[TRACE]` log; an unreadable bridge answer is a fault publication with `resync`.
- Shared corpus (schema-first, language-agnostic): `🛠️ShellHelpers/🧫️fixtures/🧫️operation-publication/🔣️.json`
  + `🧬️schema/🔣️operation-publication/🔣️.json`, 8 rows; read by the TS law (producer: what is queued) and the Rust law
  (consumer: what is read back and owed). React is not wired to it.

**Canonical re-encode checks on guest packs in the wgpu host: none.** `grep -i 'noncanonical|canonical('` over
`🎯️targets/🧊️wgpu/**`, `🌉️ProgramBridge` and the wgpu `🐚️Shell`: 0 hits. The bridge decodes with `decodePackWire`
(decode + exact-JSON projection), Rust re-parses JSON with `JsonMemberPolicy::Reject` (unknown members, not key order).
The one canonical check on the path is the shared worker's `historyPatchBytes`
(`🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts`), S5-CHANNEL's wave C.

**New laws (written, NOT run — rule 68 closed the gate at 11:18):** `🧪️wgpu-time-travel`:
`a_typed_operation_that_ends_after_its_host_call_refreshes_and_publishes_its_rows`,
`a_refused_completion_is_told_as_a_notice`, `every_operation_publication_owes_what_the_shared_corpus_says`,
`unsolicited_progress_patches_move_the_band_between_dispatches` (re-based on the lane). The wasm32 check of waves 7–9
never started: my gated shell waited from 11:15:56, the activation flag appeared at 11:18, I killed the waiting shell and
its gate at 11:26 before any cargo ran.

**Known cost, not measured:** a completion that arrives inside its host call and carries a scope refreshes once more one
frame after the dispatch's own refresh (React has the same double pass). **Not covered:** the native `Wasm` backend
takes nothing from the lane (its exchange folds such frames into the next reply) — a long typed operation on the native
shell was not examined; leftover host effects of drain turns still wait for the next host call (React carries them on the
completion as `requestedEffects`).

**Owed after "SERVE UP (B2)" (exact commands):**
- `zsh T/🚦️gate.sh && CARGO_BUILD_JOBS=3 cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format=short`
- `zsh T/🚦️gate.sh 3 18 && RUST_MIN_STACK=67108864 CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-renderer-wgpu --lib --message-format=short -- operation_publication a_typed_operation_that_ends a_refused_completion unsolicited_progress a_first_visit_derives the_boot_example_query time_travel`
- the three inherited single-threaded re-runs and the UI-crate laws of S5.5.

### S5.8 USAGE STOP (11:3x) — state at the stop
- On disk, complete, no half-applied wave: waves 1–9 (wave 9 = 4 `.rs` via the train 11:12:29 + bridge TS, corpus, schema, package law under `serve` 11:12:51). Restore of wave 9: `(cd T/🗑️generated/s5-wgpu/wave9 && python3 land.py --revert a)` / `--revert b`.
- STAGED AND NOT LANDED: nothing. The four items of the 11:2x brief are NOT started: (1) ARIA findings (`aria-expanded` on `role=group` ×3, `aria-valuetext` on `role=combobox` ×2, mirrored bottom panels, sync chip after attach), (2) `FolderReadBackRouteV1` twin, (3) §22.31 board names, (4) Ctrl/⌘+Enter + Escape in the mirror textarea.
- RUN for wave 9: tsc 0 errors, `test-preview-generated` 29 ✔ / 0 ✘. NOT RUN: every Rust law of waves 7–9 and the wasm32 check of waves 7–9 (rule 68; no cargo of mine is running — the waiting gate shell was killed at 11:26).
- OWED 1: `zsh T/🚦️gate.sh && CARGO_BUILD_JOBS=3 cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format=short` (B2's own wasm build answers the same question first).
- OWED 2: `zsh T/🚦️gate.sh 3 18 && RUST_MIN_STACK=67108864 CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-renderer-wgpu --lib --message-format=short -- operation_publication a_typed_operation_that_ends a_refused_completion unsolicited_progress a_first_visit_derives the_boot_example_query time_travel`
- OWED 3: `… cargo test -p semio-framework-ui --features testkit --lib --message-format=short -- events:: accessibility conformance_corpus row_action table_row scrub control_commit dispatch_accessibility reconcile`; the three single-threaded re-runs of S5.5.
- Live proof still missing: boot (announce → long load → board filled, Commands row present) and Accept replay completion on wgpu — B2 / Run 7.

### S5.9 After the 11:34 reboot (16:40–17:10): live faults F14, F16, F17, F18, O-w3 on build B2

B2 proved waves 7–9 live (S5-E2E: fresh profile boots, 180 nodes / 179 edges at boot, History lists the example load).

| wave | train line | files | what | run by me |
|---|---|---|---|---|
| 10 F14 | `16:53:31 wave10-F14` | 6 `.rs` | `settle_board_pointer_authority_into` before every board input | none (native `--lib` covered by train FRAMEWORK GREEN 17:07:19) |
| 11a F17 + F18 | `17:07:32 wave11a-rail` | 4 `.rs` | pane republish after rail presses; row `actionable` | none (native `--lib` covered by train FRAMEWORK GREEN 17:09:18, through `wave11b-rail`) |
| 11b F18 | `17:08:09 wave11b-rail` | mirror TS + law | Enter / Space on an actionable row | `test-browser ♿️` **27 ✔ / 0 ✘** (`vitest-browser-s5-11.txt`); `tsc` exit 0 (`tsc-wave11-1.txt`) |

**F14 — "board retained authority faulted" on the first click.** Read from the trace (`s5-e2e/wgpu/b2-click`, 18.8 s: PointerMove +
PointerDown + PointerUp dispatched in ONE input drain, fault at 19.5 s) and the code; not reproduced by a run of mine.
Every idle move is a retained commit: `plan_hover_pointer` (`♾️infinite/🎲️board/…/➕️normal/🦀️.rs`) always answers a
`Hover` plan, `requires_retained_commit()` is `!Idle`, so the wgpu host calls `begin_pointer_commit` and the frame steps it
later, in `AppFrameTransactionPhase::BoardAuthority`. The press goes direct (`pointer_down_screen`) and bumps
`interaction_revision`; `step_pointer_commit` then finds `interaction_revision != plan.revision` at phase 0 and answers
`Fault`. The same hole, read from code: a wheel after a move (`commit_wheel` bumps the revision), a double click (press
under the first release's `FinishDrag`), a release with no gesture open (`commit_pointer`), a key chord, the pinch
takeover; and a release that shares a drain with an area-select or pan move is refused (`begin_pointer_commit` chains
`DragMove` → `DragMove | FinishDrag` only) and lost. None of the four suspects named in the brief is on this path.

Fix (wgpu host only; engine and renderer phase untouched): `settle_board_pointer_authority_into(surface_id, input)` in
`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` runs the frame phase's own ladder in place — `drive_board_authority_step` to
`Complete`, `publish_board_pointer_step` through the commit's pre-reserved claim, `release_board_pointer_claim` on
`Cancelled`, `Err(Structure)` on `Fault` or after `8 × BOARD_POINTER_ITEM_CAPACITY + 64` steps — and is the first call of
every board input entry (press, move, release, leave, wheel, key chord, pinch takeover). Order is kept: the pending
commit's events leave before the new input's. Changed signatures: `puzzle_board_pointer_down` →
`puzzle_board_pointer_down_into(…, input) -> Result<(), _>`, `puzzle_board_yield_to_pinch` → `…_into`,
`scenes::puzzle_board_pointer_down` → `…_into`, `scenes::puzzle_board_touch_pointer_down` → `…_into`.
The coordinator approved a per-surface FIFO replayed by the BoardAuthority phase; I changed the mechanism and said so
before landing (same law; the FIFO would have covered pointers only and needed owned key payloads in the close ladder).
Not covered: the paint-time `sync_board_engine` setters bump the same revision; I read no interleaving with a pending
commit there and did not prove it.

Laws, written and NOT run (`🧪️tests/🔬️wgpu-board2d-engine`): `a_direct_press_under_a_pending_pointer_commit_faults_the_engine`
(the cause, on the bare engine), `a_click_dispatched_in_one_input_drain_selects_the_node`,
`a_double_click_dispatched_in_one_input_drain_keeps_the_authority`, `a_wheel_after_a_move_in_one_input_drain_keeps_the_authority`;
three existing laws re-pointed to the new signatures.

**F16 — rail "Select All" selects nothing: guest, not wgpu.** `InteractionVerb::SelectAll` (`🔌️plugin/🦀️.rs` ≈ :30134)
selects `resolve_domain_topology(def).ordered` and only `if !ids.is_empty()`; puzzle 2d's `vortex` domain is
`HierarchyProvider::Flat` (`◻️2d/…/✏️editor/🦀️.rs:310`), which answers an empty topology. The verb still answers its
declared scope, hence the refresh in the trace. Same on every renderer. Read from code only.

**F17 — "Move…" opens no form.** `setActionExpanded` / `stageActionArg` / `resetActionArgs` / `toggleSearchPossibles`
changed shell state and returned; the pane documents are republished inside a refresh pass and nothing owed one. Fix:
`ShellState::republish_window_action_panes()` after each. Law (not run):
`a_press_on_an_arg_carrying_row_republishes_the_pane_with_its_staged_form` — the published revision
(`window_measures_minted`) changes on the press, on a staged argument, returns on reset and on the second press.

**F18 — rail rows not operable through the mirror.** (1) a row's click is `RowTarget.activation`, the contract projection
derives `actionable` from record bindings only → no `data-actionable`, no forwarded click. The wgpu walk
(`🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs`) now stamps it for a `TreeItem` with a target activation that is not
disabled; the engine already fires a leaf row's activation on `AccessibilityUiEvent::Activate` (rows mount as activatable
`Stack`s, `reconcile::row_activation`). (2) the mirror handled Enter / Space for native buttons, tabs and radios only;
`accessibilityMirrorActivatesByKey` adds every other actionable, focusable element. Laws:
`a_row_with_an_activation_projects_actionable` (Rust, not run), "activates an actionable tree row by click, Enter and
Space…" (TS, ✔). Open: table rows; the contract function itself (S5-UI's file) still reads such a row as inert; inside the
canvas the engine's own focus order still skips leaf rows (`is_focusable`).

**O-w3 — the unasked "Commit Checkpoint" row** is the automatic check-in: `poll_auto_checkin` dispatches
`commitCheckpoint { message: "auto" }` 20 s after the last uncommitted edit (the boot example load), React's constants.
The row is the guest's. Open on wgpu: the "waits while loading or attached-and-unbound" twin; the full refresh it costs.

**Owed (disk was 15–16 GiB free, below the gate's 18 GiB test floor — no cargo started by me since the reboot):**
- `zsh T/🚦️gate.sh 2 18 && RUST_MIN_STACK=67108864 CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-renderer-wgpu --lib --message-format=short -- one_input_drain a_direct_press_under saturated_graph_and_board the_wgpu_rotate_ring board_and_map_two_touch a_press_on_an_arg_carrying_row operation_publication a_typed_operation_that_ends a_refused_completion unsolicited_progress a_first_visit_derives the_boot_example_query`
- `zsh T/🚦️gate.sh 2 18 && RUST_MIN_STACK=67108864 CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 cargo test -p semio-framework-ui --features testkit --lib --message-format=short -- a_row_with_an_activation accessibility events:: conformance_corpus row_action`
- `zsh T/🚦️gate.sh && CARGO_BUILD_JOBS=3 cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format=short` (waves 7–11; an activation answers it first).
- Not started: F15 (`aria-pressed` on chrome tabs with no panel shown), O-w2 (History panel paints bottom-up, labels twice).

### S5.10 F15 and O-w2 (17:10–17:25): one withdrawn fix, one open finding

**F15 — `aria-pressed=true` on Artifact / Inspection / Display / Tool / Settings at boot with no panel shown: parity with
React, NOT a wgpu defect. Nothing landed.** The mirror takes the state from `ShellState::chrome_accessibility_pressed`
(tab on its anchor's path, whether or not the anchor is open). I staged a fix (`anchor_open && on path`) and withdrew it
before landing: React's `PanelTabBar` sets `aria-pressed={isActive}` from the path selection and uses `visible` for the
fill only (`🧭️PanelTabBar/🟦️.tsx:434`, `showActiveColor={visible}`), the shared fixture pins it
(`🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json` `settingsTabStrip.pressedFollowsSelectionWhenFolded: true`), and the
wgpu law `accessibility_activation_updates_settings_pressed_button_projection` asserts exactly that ("folding removes the
active fill while retaining React's pressed selection"). Whether a folded tab should read pressed is one decision for both
renderers (S5-UI: React + fixture + wgpu together).

**O-w2 — History panel bottom-up, labels twice: cause half found, not fixed.**
- Bottom-up and right-aligned: the wgpu shell hands the docked panel's retained document its anchor flow
  (`set_ui_document_flow(window, anchor.flow())`, shell ≈ :28380; `UiFlow::for_anchor(BottomEnd)` = inline RTL, block Up),
  and the UI target lays the whole document out in it. React's `Panel` also sets `dir="rtl"`, `flex-col-reverse` and a
  `FlowProvider` for that anchor (`🖼️Panel/🟦️.tsx:546–566`), yet React's own screenshot
  (`s5-e2e/run5/probe-react-…-s6-undo-redo.png`) shows the History tree top-down and left-aligned. Where React keeps the
  tree body out of the mirrored flow I did not find; without that rule a wgpu change would be a guess.
- Label twice: every action row paints its label and, beside it, its `control` — a button carrying the same label (a
  select for Filter). React shows a control on Redo and Filter only. The rule that hides the others was not identified.
- Needed to finish: the React DOM (`outerHTML`) of the open History panel at the bottom-right anchor, from S5-E2E.

### S5.11 Build B2w (18:05–18:55): F22 measured and fixed, F24 fixed, number inputs are spinbuttons, the owed laws RUN

B2w proved waves 10 + 11 live (S5-E2E: click selects, drag moves, 0 hard faults; first history edit on wgpu).

| wave | train line | files | what |
|---|---|---|---|
| 12 F22 | `18:25:23 wave12-F22` (serve) | mirror TS + law | `refresh()` is O(1) while a pull is owed |
| 13 F24 | `18:29:39 wave13-F24` | canvas `.rs` + law file | every published board row passes the delivery table |
| 14a number input | `18:43:08 wave14a-number-input` | 3 `.rs` | contract role `spinbutton` for `Input(kind: number)`; wgpu draft as `valueNow` |
| 14b number input | `18:43:09 wave14b-number-input` (serve) | TS twin + corpus row | `uiAccessibilityRoleV1`; `a-number-input-is-a-spinbutton` |

**F22 — measured before anything was changed.** `🗑️generated/s5-wgpu/f22/census.ts` (Playwright on the live lane; result
`f22/census.json`, console `f22/census-console.txt`) takes 12 s windows with a main-thread CPU profile, a census of worker
messages by kind (count, bytes, handler time), animation-frame gaps, mirror rebuilds and page-read latency:

| window | `frame` messages from the worker | main-thread self time in mirror `refresh` | mirror nodes | rAF gaps > 50 ms | page read max |
|---|---|---|---|---|---|
| idle, no session | 145 105 (24.7 MB) | 2 597 ms of 12.5 s | 75 | 0 | 26 ms |
| after one drag | 156 912 | 3 240 ms | 75 | 0 | 25 ms |
| History open | 153 898 | 5 830 ms | 108 | 0 | 68 ms |
| session open #1 | 143 726 | 6 168 ms | 115 | 0 | 31 ms |
| session open #2 | 153 002 | 6 413 ms of 12.7 s | 115 | 24 (1 883 ms) | 103 ms |

The frame worker posts one `frame` message per frame STEP (`runFrameTurn` after every `runtime.tick`): ~12 000 a second,
~195 per animation frame, in every state. Each runs `onDirectives` → `accessibility.refresh()`, and `refresh()` walked the
whole mirror (`querySelectorAll("[data-node-id]")` + `domOwns` per node) before its own 400 ms throttle. Cost = mirror
nodes × 12 000/s; with the History rows F24 added and the editor the main thread stops draining its queue — the 5–27 s page
reads. My run stayed below that point (115 nodes), so the 27 s stall itself is extrapolated, not reproduced. Not found: a
mirror rebuild storm (0–1 per window), a `resync` loop, a per-frame history read, a worker refresh storm.

Fix: `refresh()` returns at once while a pull is owed; the ownership walk (`retireOwned`) runs on the first call of a
burst and when the owed pull starts. The existing law "synchronously retires duplicate canvas AX ownership" holds.
Law: "spends one ownership walk and one projection pull on a storm of refresh calls". NOT fixed: the 12 000 messages/s
themselves (≈ 1.5 s of main thread per 12.5 s after the fix, independent of mirror size) — a frame-protocol change.

**F24 — "Apply Board Events" rows for hover and preview.** Two wgpu publication paths sent raw rows: the retained pointer
page (`begin_board_pointer_commit` / `publish_board_pointer_step`: every idle move's `hover`, every `preselect`, brush
preview, link ring → its own `applyBoardEvents`) and the frame pump (`publish_board_event_step`: one raw engine event per
step). Only the direct lane coalesced. Fix: `board_page_dispatch_rows` (transient and `camera` rows out, the rest in
order and byte for byte) before claim and publish; the frame pump is the buffered coalesced flush. Three unreachable
raw-publish tails deleted. Old gap now visible: the board hover reaches the framework hover lane only on leave.

**Number inputs.** dx / dy are `Input(kind: number)`; the contract said `textbox` for every input, React's
`<input type="number">` is a `spinbutton`. Both twins and the shared role table now say so; the mirror therefore builds
`<input type="number">` with value, bounds, step and the number-law keys for every number field.

**Runs (gate v6 `2 12`, shared target, one cargo at a time):**

| run | result |
|---|---|
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- board2d_engine_tests saturated_graph_and_board board_and_map_two_touch a_press_on_an_arg_carrying_row operation_publication a_typed_operation_that_ends a_refused_completion unsolicited_progress a_first_visit_derives the_boot_example_query` (`test-renderer-s5-3.txt`, build 10 m 53 s) | **23 ✔ / 0 ✘** — every new law of waves 7–13, incl. `a_direct_press_under_a_pending_pointer_commit_faults_the_engine` (F14's cause on the bare engine) and the 18-case delivery corpus |
| `cargo test -p semio-framework-ui --features testkit --lib -- accessibility a_row_with_an_activation a_number_input every_component_implies events:: conformance_corpus row_action table_row scrub control_commit reconcile` (`test-ui-s5-2.txt`) | **171 ✔ / 0 ✘** — the UI-crate laws owed since wave 1 |
| `cargo test -p semio-framework-ui-contract --lib -- accessibility every_component_implies` (`test-contract-s5-1.txt`) | **15 ✔ / 0 ✘** |
| `bun ./📜️script.ts test-browser ♿️ 🎬️` after wave 12 (`vitest-browser-s5-12.txt`) | **55 ✔ / 0 ✘** |
| `test-browser ♿️` after wave 14 (`vitest-browser-s5-14.txt`); contract TS twin; `tsc` | **28 ✔ / 0 ✘**; 319 checks; 0 errors |

**Still owed / open:** wasm32 `--lib` check of waves 7–14 (an activation answers it); the three inherited single-threaded
renderer laws of S5.5 and the full `time_travel` filter (not in this build's filter); live re-measure of F22 after
activation (`bun T/🗑️generated/s5-wgpu/f22/census.ts <url> <outDir> 12` from the dev TS package). Not started: F21 (f32
pointer coordinates: `ui_render` events carry `x, y: f32`; the engine offset is exact only if the pointer crosses the
host in f64), F23 / O-w2 (needs React's panel DOM in `s5-e2e/react-panel-dom/`), F20 (row expansion through the mirror).

**F23 / O-w2, the flow half — diagnosed from React's saved DOM (`s5-e2e/react-panel-dom/react-panel-history-bottom-right.html`), not fixed.**
React's bottom-right panel mirrors its CHROME only: the panel root is `dir="rtl"` + `flex-col-reverse` (tab bar at the
bottom, chips from the right), but the body's scroll viewport is `flex min-h-full flex-col justify-end` and the tree inside is
`dir="auto"` — rows in authored order, top-down, read left-to-right, and the whole body bottom-aligned when it is shorter than
the panel. The wgpu shell hands the BODY document the anchor's whole flow (`set_ui_document_flow(window, anchor.flow())`,
shell ≈ :28380 → inline RTL, block Up), so the UI target lays the History tree out bottom-up and right-aligned. The fix is the
body flow of a docked panel (authored order, content direction, bottom alignment) while the tab bar keeps the anchor flow;
it changes every bottom- or right-docked panel on wgpu and existing wgpu laws pin the mirrored body, so it is its own wave.
The overlap half (editor controls over the Commands rows) was not examined.

### S5.12 Wave 15 (21:27–): the wgpu text-editor host takes the editor engine's typed scene

**Why.** At 19:59 an outside peer replaced the editor engine's scene API: `EditorHost::sync_from_scene_json` /
`sync_from_scene_pack` (a JSON object of `*Json` string members, each re-parsed leniently inside the engine) are gone;
the engine now takes `EditorHost::synchronize_scene(EditorScene)`, and an `EditorScene` (`✍️editor/🧬️scene/🦀️.rs`, typed
members, `deny_unknown_fields`) is constructed only by decoding its canonical document
(`framework_editor::scene::from_json` / `scene::decode` under a `NativeDecodeControl`). The wgpu canvas called the removed
method at three sites (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`: scene sync, resync after a refused edit, discard of an
explicit draft) → train FRAMEWORK RED 20:36:13, three E0599.

**What the source of that scene is today (read, not changed).** The UI contract's `TextEditorScene`
(`🖱️ui/🎬️scene/🎬️scenes/🦀️.rs`, untouched since 10-04) STILL carries its members as `*Json` strings (`selectionJson`,
`tokensJson`, `diagnosticsJson`, `placeholdersJson`, `occurrencesJson` — an object of two further encoded strings —,
`extraCaretsJson`, `selectableSpansJson`, `settingsJson`, `cameraJson`, `overlaysJson`, `hoverJson`). The peer's React twin
(`🧱️elements/✏️TextEditor/🟦️.tsx`, in their working tree) builds the typed document from those strings in one function,
`textEditorScenePacket`, and calls the wasm binding `synchronizeScene(pack, 0, 16_777_216, 16_777_216, progress)`.

**The wgpu twin.** One builder beside `EditorSyncCache`, region `✍️EditorSceneDocument`:
- `text_editor_scene_document(editor, carries_text) -> (String, Vec<String>)` — the canonical document
  (`EditorSceneDocument`, serde) from the contract scene; every `*Json` member is expanded
  (`store::pack_rt::scene_field_json_text`, React's `parseSceneJsonField`) and read into its typed form in this one place.
- `synchronize_text_editor_host(host, editor, carries_text)` — builder → `framework_editor::scene::from_json` under
  `NativeDecodeControl::new(TEXT_EDITOR_SCENE_OWNED_BYTES = 16_777_216, …)` → `host.synchronize_scene`. All three sites
  call it; `engine_canvas_fault_log` (console ERROR on wasm, stderr natively) tells every refusal — no `let _ =` left.

Decisions inside the builder, each different from "pass the strings through":
- **Settings are projected.** `settingsJson` also carries the HOST's members (`readOnly`, `commit`, `editAction`, the
  explicit-draft labels — `text_editor_explicit_draft_settings` reads them); the canonical scene refuses any member it does
  not declare, so only `fontPx`, `lineHeight`, `showLineNumbers`, `tabSize` cross. React's `textEditorScenePacket` passes
  the whole settings object: for a scene with explicit-draft settings its decode would be refused (read from the two
  files, not run) — the peer's side.
- **An echo carries no text.** Where the old code re-applied the host's own text and caret for a scene that only echoes
  the host's edit, the document now leaves `buffer` and `selection` out (omission = unchanged), React's
  `sceneWithoutEchoedTextV1`.
- **An unreadable member is named and left out**, the rest of the scene is committed; the document's own rule makes an
  omitted member "unchanged". The old engine dropped such a member silently; refusing the whole scene instead would stop
  the editor on one malformed adornment.
- Kept from the old engine: a caret-only selection is a collapsed range, a camera without `y` scrolls to 0, a hover that
  is not a full range clears.

**Laws** (`⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine`, region `✍️EditorSceneDocumentLaws`):
`the_contract_scene_builds_the_editor_engines_canonical_scene` — over the editor engine's own corpus
(`✍️editor/🧬️scene/🧫️fixtures/🔣️.json`, every accepted case): the contract scene carrying the case's members as `*Json`
strings (plus host settings members) builds exactly that document, the engine decodes it, and the echo form drops buffer
and selection; `the_editor_host_takes_the_typed_scene_and_names_what_it_cannot_read`.

**Wave 15 — landed and runs (21:34–22:05).** Train line `21:34:41 S5-WGPU wave15-editor-scene`, 2 `.rs`
(`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`, `🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs`). Restore:
`(cd T/🗑️generated/s5-wgpu/wave15 && python3 land.py --revert a)`.

| run | result |
|---|---|
| `zsh T/🚦️gate.sh 2 10 && CARGO_BUILD_JOBS=4 cargo check -p semio-framework-os-renderer-wgpu --lib --message-format=short` (`check-native-s5-15.txt`, 21:34:48 → 21:42:19) | **exit 0**, `Finished` in 7 m 31 s; the renderer crate was checked (277 warnings), `semio-framework-editor` checked before it |
| train FRAMEWORK lane | **GREEN 21:37:57** through `21:34:41 S5-WGPU wave15-editor-scene` |
| the same check with `--target wasm32-unknown-unknown` | **NOT RUN**: the gate stayed closed for its whole 900 s (21:42:32 → 21:57:33, "shared-cargo=1 free=9GiB") — `coord/activation.flag` is present and the disk is at 9 GiB, under the 10 GiB floor. No cargo of mine started. |
| the text-editor laws (one test build) | **NOT RUN**, same reason |

The only target-gated line of the wave is `engine_canvas_fault_log`'s wasm arm (`web_sys::console::error_1`, beside the
existing `log_1`); everything else is target-neutral. That is a reading, not a compile.

**Owed, exact (after the flag is gone and the disk is ≥ 10 GiB):**
- `zsh T/🚦️gate.sh 2 10 && CARGO_BUILD_JOBS=4 cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format=short`
- `zsh T/🚦️gate.sh 2 10 && RUST_MIN_STACK=67108864 CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-renderer-wgpu --lib --message-format=short -- paint2d_engine_tests text_editor_tests text_editor populated_graph_map_editor_surface_closes`
