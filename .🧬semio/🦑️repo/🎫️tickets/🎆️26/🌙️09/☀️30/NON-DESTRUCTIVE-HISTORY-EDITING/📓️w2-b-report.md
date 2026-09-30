# 📓️ W2-B — React shell: time-travel band, interpreted history body, chords, i18n

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, work package W2-B (`🧭️plan.md`, `📋️design.md` §7 and §10).

**Status: DONE and verified** (the typechecks and every suite named below were run in the foreground).

Aliases:

- `RE` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`
- `UI` = `🧰️framework/🔨️modules/🖱️ui`

## 1. What changed

### 1.1 The history body is the Rust-produced body, rendered by the interpreter

**Deleted from `RE/🏛️ShellHost/🟦️.tsx`:**

- `frameworkUtilitiesHistoryTab`, which was the hand-built tree: undo, redo, checkpoint, the inline check-in form, the status, and the `↶` rows.
- The check-in message dialog state (`checkinDialog`, `submitCheckin`).
- The undo-route memo that only that tab used (`mountedInferenceHistory`, `shellUndoRoute`, `shellCanUndo`).
- The dead `historyRowLabelText`.

**Deleted from `RE/🛠️ShellHelpers/🟦️.tsx`:**

- `HISTORY_PANEL_LABELS` and `historyPanelText`.
- The check-in form's frozen labels: `checkinActionText`, `checkinMessagePlaceholderText`, `checkinSubmitText`, `checkinCancelText`.
- `SHELL_OWNED_PANEL_TAB_IDS` and `shellRendersPanelTabItself`.
- The matching exports in the renderer-react barrel (`RE/../🎯️targets/⚛️react/🟦️.tsx`).

**New: `partitionFrameworkHistoryPanelTab`.**

- `framework.panel.history` is now mounted through `panelTabDefinitionToNode`, using the focused program's own panel body store. That puts it on the interpreter, the same as every other panel body.
- The dock keeps it as a bottom-right leaf beside Settings, Marketplace and Task manager, which matches the wgpu dock partition.
- Spawned programs now refresh the history body as well; the old filter had excluded it.

**Check-in keeps working through the Rust body and host state:**

- The body's `#s-checkin` button dispatches `framework.checkin` `submit`.
- `onAction` intercepts that controller: `checkinSubmitMessageV1` takes the trimmed message, falling back to `"check-in"`, then calls `dispatchCheckpoint`. This is the same behaviour as the wgpu shell's `handle_checkin_action`.
- Any other verb on that controller is refused as `undeclared-action`.
- The hub Check In status and cancel moved out of the tab into a persistent band (`role=status`, `aria-live=polite`):
  - It shows `#s-checkin-status`, a `<progress>` bar and `#s-checkin-abort` while the check-in runs.
  - Once it finishes, `#s-checkin-dismiss` closes it.

### 1.2 The time-travel chrome (`RE/🛠️ShellHelpers/⏪️time-travel/🟦️.tsx`, new)

**History projection:**

- The React history projection now folds rows under the kernel's `historyEntryKey` (`edit:<id>`, else `seq:<n>`).
- It carries `timeTravel: HistoryTimeTravel | null`, taken in full from every patch. The real W2-A kernel types are used.

**`TimeTravelBand`:**

- It is rendered at the bottom centre whenever the focused program has a session, whichever panel is open.
- It is a `role=status`, `aria-live=polite` region.
- Its lines, from `timeTravelBandTextV1`:
  - the stage text
  - the target label, resolved with `historyEntryLabelText` on the shell's axes
  - the replay progress, as `<progress>` plus text
  - the worst outcome, in words via `ui.mutation.level.*` (never colour alone)
  - the blocking reason
  - the fault (a known refusal code reads as its localized refusal)
  - the accepted count
- Its controls, from `timeTravelBandControlsV1`:

| Stage | Controls |
|---|---|
| editing | Accept, Discard, Exit |
| replaying | Cancel replay, Exit |
| reviewing | Finalize, Exit |
| choosing | Back, Exit |
| finalizing | none |

- Finalize stays visible when it cannot run: it is disabled, its title names the refusal (`blocked` or `empty`), and `aria-describedby` points at the blocking line.
- Every control dispatches its reserved `historyEdit*` verb on the focused app's controller with `{ generation }`, so a stale press is refused as `timeTravel.stale`.

**`TimeTravelWindowIndicator`:**

- It is passed as `controls` on every window descriptor of the focused program: spawned, base and extra windows.
- It shows an icon plus the word "Time travel", with `role=note`.
- Its accessible name says "document before X" while editing; otherwise it names the stage.
- It is memoised on the stage and its text, so replay progress steps do not rebuild the window descriptors.

### 1.3 Chords

- `SHELL_KEYBINDINGS` gained three rows, agreed with W2-C:
  - `ui.timeTravel.accept` = `alt+enter`
  - `ui.timeTravel.discard` = `alt+backspace`
  - `ui.timeTravel.exit` = `alt+shift+backspace`
- None of them is Escape, and none fires from a form field.
- Each chord is enabled only while its control is offered and enabled.
- They go through `useControlKeybinding`, so user remaps apply. The Settings ▸ Hotkeys rows get their names from the new `ui.timeTravel.*` keys.
- The chords are published on `aria-keyshortcuts`.
- `ariaKeyshortcutsText` now maps `backspace`/`delete` to `Backspace`/`Delete`; it used to output them in lowercase (`UI/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts`).

### 1.4 i18n (`UI/🧱️elements/📚️I18n/🟦️.tsx` and both bundles in `UI/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`)

The type makes every key compile-time complete in en and de.

- `ui.timeTravel.*`:
  - band, indicator, indicatorTarget
  - `stage.*` for all 5 stages
  - target, progress, worst, clean, blocking, fault, accepted
  - accept, discard, exit, finalize, back, cancelReplay
  - `refusal.{frozen,illegal,stale,blocked,empty}`
  - Every stage and refusal text is exactly the `⏪️time-travel` module's `TIME_TRAVEL_LABELS` en/de; a test enforces this.
- `ui.history.refusal.{malformedTransition,unknownTarget,transitionRefused}` uses W2-C's keys and texts byte for byte.
- `ui.referenceList.{useSelection,remove,empty}` is used by the staged reference field.

### 1.5 Refusals surfaced wherever the shell shows faults

`HISTORY_REFUSAL_LABEL_KEYS` maps these 8 codes to their keys:

- `history.malformed-transition`, `history.unknown-target`, `history.transition-refused`
- `timeTravel.frozen`, `timeTravel.illegal`, `timeTravel.stale`, `timeTravel.blocked`, `timeTravel.empty`

Supporting helpers: `historyRefusalCodeV1`, `historyRefusalOfFaultV1` (matches the fault's code or the code of any of its causes), and `historyRefusalNoticeV1`. A `history.*` refusal is always shown as an error; a session refusal keeps the fault's own severity.

These notices are wired into four places in ShellHost:

- **Hub command outcome:** the new `hubCommandRejectionNoticeV1` replaced `hubCommandRejectionReasonKeyV1`. A refused history step is named on its own, with the raw code as the notice code; a graded conflict keeps the old wording.
- **Sync `conflict` events:** those carrying one of the codes, such as the actor's `history.transition-refused`.
- **`handleAction` fault catch.**
- **`handleCommand` fault catch.**

`mutationCodeLabelKey` falls back to the same table.

### 1.6 Staged arg controls (coordinator follow-up from W1-D)

`renderStagedArgControl` (`RE/🛠️ShellHelpers/🟦️.tsx`) now renders each W1-D control kind properly:

| Kind | Rendering |
|---|---|
| Stepper | The real `Stepper` element, named through `aria-labelledby`. Its Increase/Decrease buttons are now named (via `ui.tableStepper.*`), and it gained a `disabled` prop. |
| Dial and Slider | One detented `Slider` with the snaps and ticks. It is read in display units (`stagedNumberDisplayText`: `displayFactor`, `precision`, `displayUnit`) through both `aria-valuetext` and the readout. |
| Segmented | A `ToggleGroup` of pressed buttons, instead of the Select it fell back to. |
| Vector | A labelled group with one number field per axis, named "x (mm)" and so on, carrying the step. |
| Reference | `StagedReferenceField`, with the W1-E `reference_list` semantics: chips announced "Remove <id>" that remove on activation, an empty line, and "Use current selection". |

- "Use current selection" reads the reference's own `domain` through a new optional `StagedArgContextV1.selection`. ShellHost feeds it `interactionSelectionIdsV1(localInteraction.get(), domain)` for dialogs.
- The picked selection is deduplicated, capped at `maxItems`, and reduced to the first id for a single reference.
- The UIDialog story (`UI/🧱️elements/📨️UIDialog/📖️stories/🧪️.story.tsx`) now:
  - renders integer `quantity` as a Stepper
  - handles slider/dial, segmented, vector and reference
  - has a new "Mutation inputs" story.

## 2. Tests (language-neutral corpus plus third-party oracles)

**New fixture:** `RE/🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band/🔣️.json`, with its schema at `🧬️schema/🔣️time-travel-band/🔣️.json`. It has 9 cases covering every stage, blocking, empty, a known fault and an unknown fault, plus `refusals` rows (code, en/de text, severity) for the three `history.*` refusals. It is the only band corpus: W2-C deleted its own fixture, and the wgpu law `the_shared_band_corpus_holds_on_wgpu` reads this one.

| Suite (added to the renderer-react config include list) | Tests | Oracles |
|---|---|---|
| `RE/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx` (new) | 13 ✔ | Ajv (the corpus schema and the kernel `history-patch` schema); `dom-accessibility-api`; `@testing-library/user-event`; `TIME_TRAVEL_LABELS` |
| `RE/🛠️ShellHelpers/🧪️tests/🧪️staged-arg-controls/🟦️.tsx` (new) | 8 ✔ (7 mine; the 8th was added concurrently by someone else and also passes) | `dom-accessibility-api`; testing-library role queries |
| `RE/🛠️ShellHelpers/🧪️tests/⚔️hub-command-rejection/🟦️.ts` (extended; the fixture gained 3 history rows plus `kind`/`code`) | 3 ✔ | Node `TextEncoder` payloads |

What the time-travel suite checks:

- Band lines per stage in en and de.
- The polite status region, progress, control order, disabled state and title.
- Each verb's dispatch with its generation.
- The indicator's name.
- The chords, including:
  - Escape dispatches nothing
  - a chord the current stage does not offer does not fire
  - nothing fires from a form field
  - a remap is followed and republished
- The bundle texts equal the `⏪️time-travel` labels.
- The Rust-shaped history body mounted through `panelTabDefinitionToNode` dispatches `undo`, `framework.checkin` `submit` and `historyEditBegin{mutationId}`. A row whose only child is a button becomes that row's activation.
- A source guard: ShellHost and ShellHelpers no longer contain the deleted names.

Commands run (renderer-react package dir, `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts <filters>`, and the `bun nx` typechecks):

| Command | Result |
|---|---|
| `bun nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache` | exit 0, no errors |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | exit 0 |
| The 3 suites above (last run) | 24/24 ✔ |
| Related existing React suites (11 files) | 136/136 ✔ |
| `🔬️engine-contract` | 689 ✔, 3 ✘ (below) |
| ui-react (translation-totality, UIDialog, Slider, keybinding-context) | 30/30 ✔ |

- The 11 related React suites are: chrome-history-locale, ShellHelpers component, keybinding-glyphs, window-scope, surface-switch, settings-general-layout, puzzle3d-settings-document, command-panel, ChromePanels, os-command-shortcuts and spawned-program-session.
- `🔬️engine-contract` includes the rewritten dock test. Its 3 failures were already there before this change and have nothing to do with it (W1-E listed them too): text-editor paste and compose ("Artifact" vs "Editor"), and the portal z-tutorial test (`Worker is not defined`).

## 3. Not verified or not mine (for the coordinator)

- **Taxonomy:** `verify taxonomy report --scope RE/🛠️ShellHelpers` crashes before it reports anything. The cause is unrelated: `📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json` "document digest does not match registered bytes". So the new directories are **unverified**:
  - `⏪️time-travel/🧪️tests/🧩️component`
  - `🧪️tests/🧪️staged-arg-controls`
  - `🧫️fixtures/🧫️time-travel-band`
  - `🧬️schema/🔣️time-travel-band`

  They follow the existing sibling (`⏯️tool-run-panel`) and open-pattern names.
- **For W2-A, chrome history rows are now frozen in their original locale.** React used to relabel `shell.*` and `os.*` rows from their `actionId`; that was the deleted `historyRowLabelText`. The Rust body renders the `LocalizedLabel::data` that `noteShellCommand` stores, so these rows keep the locale they were recorded in. The clean fix is to have `noteShellCommand` carry a full `LocalizedLabel` (en and de).
- **For W2-A, the undo button no longer enables for remote undo.** The Rust body's undo button is enabled from `history.can_undo` only. The old React tab also enabled it for a remote (inference) undo route. The chord and palette undo still route remotely.
- **For W2-A, replay progress may only update the band on completion.** The band updates from `HistoryPatch.timeTravel` on dispatch responses and completions. If replay progress arrives only through spawned-job progress frames, which React does not refresh on (a known memory note), the band will update only when the replay completes.
- **For W3 e2e probes, React DOM ids changed.** They are now namespaced, e.g. `panel:framework.panel.history/s-checkin` and `.../framework.history.entry.<seq>`, where wgpu uses key suffixes. The explicit check-in message dialog no longer exists in React, the same as in wgpu.
- **W2-C, done:** W2-C fixed the stale wgpu doc comment near `default_dock` (it now names `partitionFrameworkHistoryPanelTab`) and uses the shared band corpus, refusal texts and `ui.referenceList.*` copy byte for byte. The one difference: wgpu shows `timeTravel.*` refusals as warnings, because its dispatch faults carry no severity; React keeps the fault's own severity.

## 4. Files

- **Created:**
  - `RE/🛠️ShellHelpers/⏪️time-travel/🟦️.tsx`
  - `RE/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx`
  - `RE/🛠️ShellHelpers/🧪️tests/🧪️staged-arg-controls/🟦️.tsx`
  - `RE/🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band/🔣️.json`
  - `RE/🛠️ShellHelpers/🧬️schema/🔣️time-travel-band/🔣️.json`
- **Modified:**
  - `RE/🏛️ShellHost/🟦️.tsx`
  - `RE/🛠️ShellHelpers/🟦️.tsx`
  - `RE/🛠️ShellHelpers/🧪️tests/⚔️hub-command-rejection/🟦️.ts`
  - `RE/🛠️ShellHelpers/🧫️fixtures/⚔️hub-command-rejection/🔣️.json`
  - `🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx` (barrel)
  - `🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (include list)
  - `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
  - `UI/🧱️elements/📚️I18n/🟦️.tsx`
  - `UI/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`
  - `UI/🔨️modules/🕹️control-keybinding-context/🟦️.tsx`
  - `UI/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts`
  - `UI/🧱️elements/🪜️Stepper/🟦️.tsx`
  - `UI/🧱️elements/📨️UIDialog/📖️stories/🧪️.story.tsx`
- **Command output:** `🗑️generated/w2-b/`.

## 5. Follow-up: adopting W2-A's additive wire (coordinator request)

All five items are done. Both typechecks are clean: renderer-react exits 0 and ui-react exits 0. The framework-os typecheck still reports 26 errors, but none of them is in a file I touched. The shared band corpus model was agreed with W2-C, and W2-C reads the same corpus.

### 5.1 Rerun

- The band's reviewing controls are now, in order:
  - `rerun` (`historyEditRerun{generation}`, caption "Replay again" / "Erneut anwenden"). It is enabled only when `timeTravel.rerunnable` is true; otherwise it is disabled with `ui.timeTravel.refusal.illegal`.
  - `finalize`
  - `exit`
- The verbs are typed against the manifest's `HistoryEditActionId`, and the argument key is `HISTORY_EDIT_ARG_GENERATION`.

### 5.2 Review status

- The new `review` line reads the session's own `review` through `ui.timeTravel.review.{noChanges,needsReplay,blocked,ready}`. The texts are exactly the `⏪️time-travel` labels `noChanges`, `needsReplay`, `reportBlocking` and `readyToFinalize`, and a test enforces this.
- Nothing is inferred when the report is missing:
  - The `outcome` line appears only when `worst` is set.
  - The "No problems" and "Errors block finalizing" lines and their keys (`clean`, `blocking`) are deleted.
- Finalize is disabled for these reasons, checked in this order:

| Condition | Disabled by |
|---|---|
| `noChanges`, or `acceptedCount` is 0 | `refusal.empty` |
| `needsReplay`, `blocked`, or `blocking` is set | `refusal.blocked` |
| `ready` | enabled |
| no `review` stated | `refusal.illegal` |

- A disabled Finalize is described by the review line (`aria-describedby`).
- New refusal keys:
  - `ui.timeTravel.refusal.cancelled` ("Replay cancelled" / "Neu anwenden abgebrochen", the `replayCancelled` label). It is also what the band shows as the fault after a cancel.
  - `ui.timeTravel.refusal.nameInvalid` ("Invalid alternative name: use 1 to 256 characters" / "Ungültiger Name der Alternative: 1 bis 256 Zeichen verwenden", agreed with W2-C).
- Both codes are now in `HISTORY_REFUSAL_LABEL_KEYS`, which has 10 codes.
- A session refusal with no severity of its own is now a warning, which matches wgpu.
- A reserved verb's silent `{rejected: <code>}` result now raises the localized notice (`historyRefusalOfOutputV1`).

### 5.3 Body Undo goes through the host

- For a local document this already worked: the body's `undo` goes through `onAction`'s undo branch, which uses the remote inference route when one is mounted.
- For an actor-bound (hub) document, panel gestures used to go straight to the actor. `panelActionRoutesThroughHostV1` now sends `undo` and `framework.checkin` to the host's `onAction`, where the remote-undo route applies. Every other gesture, including `historyEdit*`, still goes to the actor.

### 5.4 Replay progress reaches the band while the replay runs

- `AppChannelClient` (`🧰️framework/🛍️products/💻️os/🟦️.ts`) routes an unsolicited `Invocation` frame to the progress lane when it carries a `ui_scope` **or** a `history_patch`. Before, it required a `ui_scope`, so a patch-only frame fell into the waiter lane and was dropped.
- The listener now receives `AppChannelOperationProgressV1 {uiScope?, historyPatch?}`, and the PluginRuntime `subscribeOperationProgress` type follows.
- Both ShellHost progress subscribers (the session's and every spawned program's) now fold the patch with `applyHistoryPatch` and refresh only a scope the frame actually carried (`operationProgressPartsV1`). A frame without a scope is never read as a full refresh.
- The projection type, its empty value and the fold moved to ShellHelpers (`ShellHistoryProjectionV1`, `EMPTY_SHELL_HISTORY_PROJECTION_V1`, `shellHistoryProjectionAfterPatchV1`), so the fold can be tested on its own.

### 5.5 `noteShellCommand` carries both languages

- `noteShellCommand` and `buildNoteShellCommandAction` take a `ShellLabelTextV1 = Record<ShellLocale, string>`.
- All 11 chrome call sites use `shellLabelTextV1("ui.shellCommand.*")`, backed by the new locale-fixed `UiI18nPort.tIn`.
- Both OS-command call sites use `manifestLabelTextV1(label, terminology)`.
- The now-dead `shellChromeCommandLabel` and its key table are deleted.

### 5.6 Tests (all run in the foreground)

| Suite | Result |
|---|---|
| `RE/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component` | 15 ✔ |
| `RE/🛠️ShellHelpers/🧪️tests/🧪️staged-arg-controls` | 9 ✔ (2 of them added concurrently by others) |
| `RE/🛠️ShellHelpers/🧪️tests/⚔️hub-command-rejection` | 3 ✔ |
| `RE/🛠️ShellHelpers/🧪️tests/🌐️chrome-history-locale` (rewritten) | 3 ✔ |
| engine-contract plus 5 related renderer-react suites | 770 ✔, 3 ✘ |
| ui-react (translation-totality, UIDialog, keybinding-context) | 15 ✔ |
| framework-os in-source `../../🟦️.ts` | 244 ✔, 1 ✘ |

- **Time-travel suite:**
  - The corpus now has 11 cases:
    - ready, blocked, warning-only, noChanges
    - a cancelled replay that needs a rerun
    - a review with no stated `review`, which is never read as ready or no-changes
    - an unknown fault
  - The `refusals` rows cover all 10 codes, each with its default severity.
  - The bundle ↔ `TIME_TRAVEL_LABELS` law now also covers the review, rerun and cancelled texts.
  - New: actor-bound history body routing (Undo and check-in go to the host, `historyEditBegin` goes to the actor).
  - New: the progress fold (patch-only frames move the band `1/8 → 5/8 → reviewing`; a bare frame refreshes nothing; a patch without a session closes it).
- **engine-contract:** the `noteShellCommand` tests now use `{en, de}`. The 3 failures are the same pre-existing ones (text-editor paste/compose, and `Worker is not defined`).
- **framework-os:** the updated `backbone-envelope-io` progress test passes; it delivers a scope-only frame and a history-patch-only frame. The one failure is unrelated: "document backbone batch: truncated" in `📡️replication` `readDocumentBackboneEnvelopeBatchAtExact`, which looks like fallout from a peer's codec change.

### 5.7 Files (follow-up)

- `RE/🛠️ShellHelpers/⏪️time-travel/🟦️.tsx`
- `RE/🛠️ShellHelpers/🟦️.tsx`
- `RE/🏛️ShellHost/🟦️.tsx`
- `RE/🔌️PluginRuntime/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`
- `UI/🧱️elements/📚️I18n/🟦️.tsx`
- `UI/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`
- The corpus and schema under `🧫️time-travel-band` and `🔣️time-travel-band`
- Tests:
  - `⏪️time-travel/🧪️tests/🧩️component`
  - `⚔️hub-command-rejection`
  - `🌐️chrome-history-locale`
  - `🔬️engine-contract`

## 6. Follow-up 2: peers' history-edit presence (coordinator request)

Done: the React host now publishes the local user's history edit (and tool run) in presence, and shows other peers' history edits in the roster and on the history rows. Both typechecks I own are clean (renderer-react and ui-react exit 0). The framework-os typecheck still reports the same 26 errors; none of them is in a file I touched.

### 6.1 Publishing (the local session's presence carries it)

- **Two paths for the snapshot.** For a local instance, the snapshot comes from `AppChannelClient.ephemeral()` (`🧰️framework/🛍️products/💻️os/🟦️.ts`). For an actor-bound document, it comes from the browser actor's publication (`BrowserActorEphemeralSnapshotV1`, `🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication`).
- **What changed in both.** Both snapshot types now also carry the guest's `AppFrame::Ephemeral.tool_run` and `.history_edit` bytes, and the PluginRuntime `ephemeralSnapshot` type follows them.
- **Local documents.** The React heartbeat (`ShellHost`) now adds the decoded `toolRun` (bit 10) and `historyEdit` (bit 13) through `presenceEphemeralPeerFieldsV1`. That function also decodes the interaction slice, and it only adds the fields a snapshot actually carries.
- **Actor-bound documents.** The worker's `stampSession` (the TS twin of the actor's `stamp_session`) now publishes the actor's own `toolRun`/`historyEdit`. It drops the shell's local copies, the same way it already handles the presence pack and the interaction slice.

### 6.2 Showing it (labelled from the local rows; no locale text on the wire)

- **Roster.** `scopedPresencePeersV1` now passes each peer's `historyEdit` through as `ScopedPresencePeerV1`.
- **Labelling.** `timeTravelPeerPresenceV1` (`⏪️time-travel` module) labels each editing peer from the focused program's own history rows.
- **Roster chip.** The peer gets an `activity` with a ⏪ badge. The text is:

| Case | English | German |
|---|---|---|
| mutation found in local rows | "Ada is editing Drag selection in time travel" | "Ada bearbeitet Auswahl ziehen in der Zeitreise" |
| mutation not in local rows | "… is editing the history in time travel" | "… bearbeitet den Verlauf in der Zeitreise" |

- **`PresenceBar` element.** It gained the optional `activity {text, badge}`. The text is part of the chip's accessible name and title, marked with `data-presence-activity`, so it never relies on colour alone.
- **History rows.** The mutation row `framework.history.mutation.<id>` and its history row `framework.history.entry.<seq>` get a note: "Ada is editing this in time travel" / "Ada bearbeitet dies in der Zeitreise".
  - The note travels through the interpreter's presence overlay. `UiPresenceOverlayEntry` gained `notes`, and a tree row appends the notes to its description.
  - The overlay merge now merges the host's and the guest's entries field by field, instead of the guest replacing the host.
  - ShellHost provides the overlay at the shell root.
- **New i18n keys:** `ui.timeTravel.peer.{editingRow,editingTarget,editingHistory}`, in en and de.

### 6.3 Tests (foreground)

| Suite | Result |
|---|---|
| React: time-travel | 17 ✔ |
| React: staged-arg-controls | 12 ✔ (the extra tests were added by others) |
| React: hub-command-rejection | 3 ✔ |
| React: chrome-history-locale | 3 ✔ |
| React: `👥️scoped-presence` | 12 ✔ |
| engine-contract + `👕️canvas-presence` | 700 ✔, 3 ✘ (the same pre-existing failures: text-editor paste/compose, and `Worker is not defined`) |
| ui-react: translation-totality + presence | 6 ✔ |
| framework-os in-source (`🟦️.ts` + `👷️worker/🟦️.ts`) | 382/382 ✔ |

- **New time-travel laws:**
  - Peer rows and chips are checked in en and de, including the note on the history row as rendered by the interpreter, and the PresenceBar name and badge.
  - A peer with no history edit gets no activity and no overlay entry.
  - The heartbeat field decoder is checked against the replication encoders.
- **framework-os updates:**
  - `AppChannelClient.ephemeral()` now carries `history_edit`.
  - The publication snapshot shape is updated.
  - `stampSession` now publishes the actor's tool run and history edit and drops the local ones.
  - The earlier replication "truncated" failure has been fixed by its owner in the meantime.

### 6.4 Files (follow-up 2)

- `🧰️framework/🛍️products/💻️os/🟦️.ts`
- `🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts`
- `🏪️store/👷️worker/🟦️.ts`
- `RE/🔌️PluginRuntime/🟦️.tsx`
- `RE/🏛️ShellHost/{🟦️.tsx, 👥️presence-scope/🟦️.ts}`
- `RE/🛠️ShellHelpers/{🟦️.tsx, ⏪️time-travel/🟦️.tsx, ⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx}`
- `RE/🗣️Interpreter/🟦️.tsx`
- `UI/🧱️elements/👥️PresenceBar/🟦️.tsx`
- `UI/🧱️elements/📚️I18n/🟦️.tsx` and `UI/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧪️tests/{🧪️backbone-envelope-io, 🧪️space-artifact-creation-owner}/🟦️.ts`
