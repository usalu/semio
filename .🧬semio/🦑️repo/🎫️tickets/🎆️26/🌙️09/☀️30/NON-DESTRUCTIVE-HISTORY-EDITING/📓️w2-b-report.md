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

## Follow-up 3 — typed rejections, folder re-attach, R2-2/R2-4/R2-6, dev reload and hub-less serve

### F3.1 One typed rejection contract (finding 9)
- **Before.** `CommandAckOutcome.rejected.messages` had two producers that disagreed. Hub acks carried the JSON bytes of a `MutationMessage` array. Local refusals carried counters: `[envelopes]`, `[count, bytes, limit]` or `[bytes]`. The shell JSON-decoded both, so a local counter such as 69 became the text "E", and `JSON.parse` threw out of `worker.onmessage`.
- **Now, one contract in both twins.** `rejected = { code, reason, messages: MutationMessage[], detail?: { envelopes?, bytes?, limit? } }`.
  - The codes are a closed set of 10: `hub.refused`, `hub.unreadable`, `local.read-only`, `local.queue-full`, `local.backbone-capacity`, `local.backbone-duplicate`, `local.backbone-pair-unavailable`, `local.backbone-scope-mismatch`, `local.backbone-malformed`, `local.socket-frame-ceiling`.
  - Schema: `🏪️store/🔄️sync/🧬️schema/🔣️command-rejection/🔣️.json`, whose `messages` item refers to the kernel's `HistoryMutationMessage`.
  - Shared fixture: `🏪️store/🧫️fixtures/🧫️command-rejection/🔣️.json` (10 hub rows, 8 local rows).
- **TypeScript (`💻️os/🟦️.ts`, region `🚫️CommandRejection`).**
  - Types: `COMMAND_REJECTION_CODES_V1`, `CommandRejectionV1`, `CommandRejectionDetailV1`.
  - Functions:
    - `decodeHubRejectionMessagesV1` and `hubCommandRejectionV1` decode the hub bytes exactly once and never throw.
    - `localCommandRejectionV1` builds a local refusal.
    - `commandAckOutcomeOfValueV1` reads a `commandOutcome` coming from either actor, including Rust's exact-integer carriers; the batch id `u64::MAX` reads as -1.
  - `parseArtifactEvent` validates `commandOutcome`.
  - Every producer in the worker goes through `rejectLocally`.
- **Rust (`🏪️store/🔄️sync/🦀️.rs`).**
  - New types: `CommandRejectionCode` and `CommandRejectionDetail`.
  - `CommandAckOutcome::Rejected{code, reason, messages: Vec<MutationMessage>, detail}` with `hub_rejected` and `local_rejected` constructors.
  - The `reject_document_backbone` callers in both actors pass the typed outcome, and the retention's `retain` returns it too.
  - `CommandOutcome.batch_id` now goes on the wire as `batchId`.
- **Shell.** `commandRejectionNoticeV1(rejection)` is total. It gives a localized notice per code, with the new en/de keys `ui.conflict.local.*`. The suite moved to `🛠️ShellHelpers/🧪️tests/🧪️command-rejection` with fixture `🧫️command-rejection`.
- **wgpu (W2-C, told).** Line 10974 still compiles because `reason` is kept. The history refusal code is now available in `messages[].code`.

### F3.2 Folder re-attach (e2e finding 7) — root causes fixed
1. **Every edit refused.** The TypeScript actor refused every document-backbone batch of a folder document ("canonical pair unavailable"), because admission required a hub-verified cold pair. It now asks for one only on hub documents, like the Rust actor.
2. **Wrong document id (R2-4).** After (1), each batch was refused as `local.backbone-scope-mismatch`. The folder card addressed the document by `syncDocumentId` = `${pluginId}-${instanceId}`, a runtime counter, while the guest stamps its store id (`puzzle.2d.fixture`, read from the Run 2 archive). `openSyncTarget` now addresses the document by the program's own identity, `readAppDocumentIdentity`, which is new on the React plugin handle (`ReadDocumentIdentity`). A program with no document gets the notice `ui.sync.documentUnidentified`. `syncDocumentId` is deleted.
3. **Own-write echo.** Every PUT was read back by the watch and replaced the document with an older state. The worker now keeps `folderArchive` (the TypeScript twin of Rust's `last_written_hash`), so a read that returns the document's own write replaces nothing.
4. **No hydration after load.** A loaded archive was never hydrated. `restoreDocumentArchiveV1` (in `🛂️admission/📄️document`) loads the archive, re-reads the program's history and refreshes every surface. ShellHost uses it for the folder read-back and for tutorial restores; the `loadDocument` effect now re-reads history too.

### F3.3 R2-2 (History collapses after reload)
- **New law.** `a_document_archive_round_trip_lists_every_history_row_of_its_source`, in `🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs`.
  - Source: three edits, an overwrite Supersede with Revert and Reinstate, two history edits finalized as new alternatives (c, then b), and two edits on b.
  - Round trip: `document_archive` → a fresh instance that already holds a "Set Active Example" edit → archive load.
- **What it found.**
  - The runtime and store keep every edit and supersede row, including both alternatives.
  - One real defect: the displaced document's own rows survived the load.
- **Fix.** `retire_displaced_document_rows` (`🔌️plugin/⏪️time-travel/🦀️.rs`) runs after all three whole-document replacements (the archive commit, `load_document_text`, `load_document_pack`). With it, the law passes, along with the other 6 archive tests.
- **Remaining e2e difference, not reproduced natively (needs a re-probe).**
  - Run 2's archive for `puzzle-1` holds all 5 edits and 11 transition ids.
  - After reload, the probe saw only `["Drag 1 item by (70, 70)", "create-node node {…}"]`.
  - One real, separate gap is labels. A command whose app emits no `description` (Duplicate Selection, Set Active Example) returns as op text after reload, because only the live command log holds its label. Transaction labels survive. This needs the invoking verb persisted with the edit, a store-schema change (W1-G/W2-A).
  - The probe also compares rows by label and keeps only rows that are expandable or labelled as history edits. For a re-probe, the guest's `readHistory` upserts after reload are the evidence to capture.

### F3.4 R2-6 — check-in fired during Finalize
- `dispatchCheckpoint` does nothing while the focused program is in time travel. An explicit check-in shows `timeTravel.frozen`; an automatic one waits.
- The close check-in effect is keyed on the program's identity (`pluginId`/`instanceId`) and the document, no longer on the session object. Every view-state rewrite, including the New-alternative submit, minted a new session object and fired it.

### F3.5 Dev reload `useShellScope called outside a ShellScopeProvider` (e2e finding 4)
- **Cause.** `FrameworkOsShellInner` has exactly one render site, and it is inside its provider. The error can only arise from a second evaluation of the ShellScope module creating a second context that no mounted provider supplies.
- **Fix.** `ShellScopeContext` is now page-wide, created once under `Symbol.for("semio.ui.elements.ShellScope.context")`.
- **Law.** "one shell-scope context per page", in `🐚️ShellScope/🧪️tests/🧩️component`, imports the module a second time under another URL. It fails without the fix (checked) and passes with it.
- **Not reproduced.** The live reload itself was not reproduced; no dev servers were started.

### F3.6 Hub-less serve requests the trusted catalog (500 ×8)
- **Cause.** `ensureDevLocalHub` set `process.env.S_HUB_URL` before any hub answered and left it set when it returned `null`. Vite inherited it, so it proxied `/_semio/hub/*` to nothing and got 500. It also defined `VITE_S_HUB_URL`, so the shell built its hub plugin source.
- **Fix.** `S_HUB_URL` is set only once a hub answers, and cleared on the local-only paths.
- **Law.** The explicit-hub law in `🧑‍💻dev/🧪️tests/🚀️local-hub` now asserts both outcomes.

### F3.7 Folder binding persistence — not implemented, and why
1. The playground route (`?plugin=`) has no space and no document in its URL. `resolveDocumentOpeningBindings` returns `[]` without a `spaceId`.
2. The only persisted local-only store for this is the local catalog (`os.config.local-catalog`). It lives under `${S_DATA_DIR}/os`, and it rehydrates into the landing app (host mode, `applyLocalCatalogDocument`), not into a `?plugin=` program.
3. Doing it properly needs a route parameter for a local folder document, or an event-sourced config mutation "attach local folder (surface, documentId)" with TS and Rust twins. The document id is now stable (the store id), so either design is viable. It is a design decision for the coordinator.

### Verification (all run, foreground)
- **framework-os vitest.**
  - CommandRejection: 4/4.
  - Folder archive restore: 2/2. It fails with the admission fix reverted (checked).
  - Worker folder/backbone suites: 108/108.
  - Full-suite run: killed at the 300 s budget, with one unrelated failure: `🏷️schema-vocabulary` flags `x-semio-fixture` in `🌎️hub/🧫️fixtures/🐳️docker-image-v1`.
- **Rust.**
  - `semio-framework-os-kernel --features sync`: `os_store::sync` 87/87, including the 4 new command-rejection tests.
  - `semio-framework-plugin`: archive tests 7/7, including the new law.
  - Full plugin lib test (after a peer finished the `APPLY_OUTCOME_CODE_PREFIX` export): 920 passed, 14 failed. None of the failures touches history, archives or document loads. They are `merge_ui_values` (UInt vs Float), `tool_run` fixtures, `window_kits`, fixture projection retirement, activated tool keys, command ingress terminal and the neutral checked diff, all peer areas.
- **React.** The command-rejection, time-travel and staged-arg-controls suites pass 32/32.
- **ui-react.** The ShellScope identity law passes. Three popover-contrast tests fail with "no :root { block containing --base", which comes from peer CSS edits, not this work.
- **dev.** The local-hub explicit-join law passes 1/1.
- **Typechecks.**
  - renderer-react: 0 errors. This included fixing the `PluginWasmHandle` fakes that needed `readAppDocumentIdentity`.
  - ui-react: passed after the identity-law import fix. A later run shows 1 new error, `🔬️translation-totality/🟦️.ts:19` (`trim` on `never`). It comes from a peer's outcome-code keys, not from this work.
  - framework-os-dev has no `typecheck` target.
  - framework-os: the same 26 errors as before, none in these files.
- **Rust check.** `cargo check -p semio-framework-os-kernel --features sync`: clean apart from 4 pre-existing warnings.

## Follow-up 4 — remembered local folder binding (event-sourced, persisted local-only)

### F4.1 Vocabulary (schema first; Rust + TypeScript twins)
- **Facet.** `os.config.local-folders` holds `LocalFolderBindings { bindings: LocalFolderBinding[] }`. Each binding is `{ documentId, pluginId, appId, folder: { kind: "path", path } }`, keyed by the document's own identity: the store id that `ReadDocumentIdentity` answers. Bindings are ordered by document id.
- **Persistence class.** Persisted local-only. A binding never enters a shared lane or a URL.
- **Mutations.**
  - `attach-local-folder` (`📎️attach-local-folder`): upsert keyed on `documentId`. An identical binding is a warned `mutation.no-op`. The inverse is the prior binding, or a detach.
  - `detach-local-folder` (`✂️detach-local-folder`): forgets the folder. An unbound document is a warned no-op with no undo.
- **Files.** Each leaf has:
  - a payload schema with `x-semio-ui` labels in en and de;
  - a leaf manifest;
  - a committed fixture quintet (`📎️remembers-the-folder-beside-another-document`, `✂️forgets-the-folder-and-keeps-its-sibling`);
  - Rust unit and fixture tests.
- **Registrations.**
  - Rust: the `LocalFoldersConfigMutation` aggregate, generating descriptors and the payload law `semio_payload_law_local_folders_config_mutation`, registered in `register_os_config_mutation_descriptors`.
  - TypeScript: the barrel region `🔖️LocalFolders`.
  - JSON: the aggregate `🔣️.json`.
  - Oracles (`🎚️config/🔮️oracles/🔣️.json`): a no-oracle decision, a mutation catalog and a mutation manifest.
  - Taxonomy: member names for the leaves, fixtures, tests, the host case and `📎️local-folders`.
  - Schema catalog: regenerated.
- **Exhaustive host case.** `🔌️plugin/🖥️host/🧪️tests/📎️mutate-os-config-local-folders` (feature + Rust adapter + TypeScript adapter), mirroring the local-catalog case.
- **Why a path, not a browser handle.** A folder reference names a path only. The browser has no folder transport that takes a `FileSystemDirectoryHandle`: the worker's folder backbone is the host's `/semio-backbone` route over a path, and `requestBackboneFolderPath` answers only native paths. A `handle` variant arrives together with such a transport, and the schema's closed union then grows by that one variant. The law rejects a `handle` reference today.

### F4.2 Browser shell (React)
- **Store.** `🏛️ShellHost/📎️local-folders/🟦️.tsx` keeps the facet's event log `{version:1, events}` in this shell's own local storage (`OsShellConfig.setPreference`). Each persisted event is validated against the payload schemas on read: an invalid log reattaches nothing, and a mutation that changes nothing is not recorded.
- **Attach and detach.**
  - `openSyncTarget` records `attachLocalFolder` once a folder (or file-folder) attach has committed.
  - `detachSyncBackbone` records `detachLocalFolder`.
- **Boot.**
  - The focused program's document identity is read once per program, retried while its channel comes up.
  - If this device remembers a folder for that identity (same plugin and app) and the document is not attached, the bottom band shows `LocalFolderReconnectBand`: a polite `role=status` region named "Folder of this document" / "Ordner dieses Dokuments" that names the folder, with "Reconnect folder" / "Ordner wieder verbinden" (`#s-folder-reconnect`, the person's gesture) and "Forget folder" / "Ordner vergessen" (`#s-folder-forget`). Both are disabled while reconnecting.
  - Reconnect runs `openSyncTarget` with the remembered path; the folder read-back is then restored through `restoreDocumentArchiveV1` (head plus history rows).
  - The new en/de keys are under `ui.sync.reconnect.*`.
- **Routes.** It works on the `?plugin=` playground and on space routes alike, because bindings are keyed by document identity, never by route.
- **Native (wgpu, W2-C).** Reattaches directly on boot with the Rust twin; W2-C has the API.

### F4.3 Laws
- **Rust (`semio-framework-os-config`).** 19 new tests and the payload law; the crate passes 193/193.
- **React.** `🏛️ShellHost/📎️local-folders/🧪️tests/🧩️component` passes 3/3. It covers accessible names en/de (computed by `dom-accessibility-api`), the polite status region, the clicks (`user-event`), the busy state and folder naming. The suite is registered in the renderer-react config.
- **framework-os (`🧪️folder-archive-restore`).**
  - "a remembered folder binding survives the reload and its reconnect restores the head and the history rows": attach → edit → reload → reconnect offer → reattach → restore.
  - "a detached folder is forgotten across the reload, and a binding is written to this device's local-only facet alone": detach → no offer. The only preference written is `os.config.local-folders`, an unchanged attachment is not recorded, and a foreign log reattaches nothing.

### F4.4 Verification (all run, in the foreground)
- **framework-os vitest.** `folder archive restore` passes 4/4, including the two binding laws. CommandRejection passes 4/4.
- **renderer-react.** `🏛️ShellHost/📎️local-folders/🧪️tests/🧩️component` passes 3/3.
- **Rust, `semio-framework-os-config`.** 193/193, including the 19 new tests and the derive payload law.
- **Repository host case `📎️mutate-os-config-local-folders`.** The subject phase (exhaustive) executed 12 of 12 and passed 12, for both the Rust and the TypeScript subjects. The contract phase reports for this case exactly the breaches its `local-catalog` sibling also has:
  - an empty no-oracle `capabilities` list;
  - no runtime inventory;
  - no third-party library.

  These are the honest-tracking convention. An earlier layout finding is fixed: `📁️` is a generic emoji, so the directories became `📎️mutate-os-config-local-folders` and `📎️local-folders`.
- **Typechecks** (the nx project graph was broken by peers, so the package scripts ran directly):
  - renderer-react: 4 errors, all peer work in progress (`topicContributions`, `idleInstalledServiceStatusV1`, `backbone-envelope-io`).
  - ui-react: 2 errors, both peer (`backbone-envelope-io`).
  - framework-os: no error in any file of this follow-up.
- **Schema catalog.** Regenerated with no diagnostics for the new schemas.
- **Not done here: native wgpu reattach.** It is recorded by W2-C in `📓️w2-c-report.md` Follow-up 4 for the coordinator to route. The Rust twin and its API are ready.
- **e2e.** W3-E2E's probe step 5 now uses the reconnect flow: offer → reconnect → compare → detach → reload → no offer. It is waiting for activation #5.

## Session 2 — 2026-10-01

Successor executor S2-W2B. Aliases as above: `RE` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, `UI` = `🧰️framework/🔨️modules/🖱️ui`, `ENG` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`.

### S2.1 Repair (rule 21)

- No half-finished W2-B edit. Every W2-B file was clean against HEAD (`4e36b2b5012`, 11:16). The peer's 11:12 ShellHost refactor was already committed and compiles.
- The fake `PluginWasmHandle.readAppDocumentIdentity` error is gone; session 1's follow-up 3 fixed it.
- The renderer-react typecheck has no error in any W2-B file. Every remaining error belongs to a peer:
  - `RE/🐚️Shell/🟦️.tsx:1112`: `idleInstalledServiceStatusV1` is undefined (inference WIP).
  - The stale generated registry, `🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts` (05:49). It still calls `moduleDirectoryName` with 1 argument; the 08:14 signature takes 2.
  - From 12:08: the replication envelope gained `line`, which `🏪️store/👷️worker` and `🔄️sync/🧪️tests/🔬️backbone-parity` do not set yet.
  - From 12:34: G6 numeric facets (`UiNumberLimits`, `SliderAppearance`, and `SliderProps.precision/displayUnit/displayFactor/limits`) break `UI/🧬️contract`, `UI/🧱️elements/🎚️Slider` and `RE/🗣️Interpreter/📖️stories`.
- **Blocker for every React suite that imports ShellHost.** The stale registry throws at module load ("Cannot read properties of undefined (reading 'filter')"), so all five W2-B suites failed with 0 tests (`🗑️generated/s2-w2b/vitest-mine-1.txt`).
  - Bridge: a ticket-local shim, `🗑️generated/s2-w2b/{registry-plugins-shim.ts, vitest-shim.config.ts}`. It aliases only the generated module to a copy whose module URL ignores the directory name. It is not referenced from any repo file.
  - Every React count below was taken with that shim. **To redo without the shim** after `plugin-registry:generate`, or once S2-INFRA's item 4 lands.

### S2.2 Changes

**Plugin-level `timeTravel.*` refusal notices (coordinator item 1).** These 10 codes are now localized by code, in both shells' vocabulary:
`busy`, `unknown-mutation`, `not-editable`, `unknown-input`, `invalid-input`, `no-selection`, `name-required`, `schema-unavailable`, and the driver faults `replay-faulted` and `commit-failed`.
- `HISTORY_REFUSAL_LABEL_KEYS` (`RE/🛠️ShellHelpers/🟦️.tsx`) now has 20 codes.
- New keys `ui.timeTravel.refusal.{busy,unknownMutation,notEditable,unknownInput,invalidInput,noSelection,nameRequired,schemaUnavailable,replayFaulted,commitFailed}` in `UI/🧱️elements/📚️I18n/🟦️.tsx` and both bundles of `UI/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`. The texts were agreed with the coordinator, including its two German tweaks.
- S2-W2A landed the same strings as `TIME_TRAVEL_LABELS`/`TIME_TRAVEL_CODE_LABELS` (`FW/⏪️time-travel/🟦️.ts`). A new React law requires every vocabulary code to exist in React with byte-equal en/de text, and no React `timeTravel.*` code outside the vocabulary.
- Shared band corpus (`RE/🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band`):
  - `refusals` now has 20 rows.
  - New case "a faulted replay names its fault in words and can replay again".
  - The unknown-fault case now uses `store.replay-unreadable`, since `replay-faulted` is now a known code.
- `🧪️command-rejection` now reads its code list from the band corpus instead of a hard-coded 10.

**Reveal and focus (G13, design §16).** New in `RE/🛠️ShellHelpers/⏪️time-travel/🟦️.tsx`:
- `timeTravelTransitionV1(previous, next) → {reveal, focus}`:
  - The edge into a new session (any `sessionId` change from none) reveals the History panel.
  - A draft that starts (Begin, Next problem, another target) focuses the editor.
  - Entering replaying or reviewing focuses the band.
  - Choosing focuses the prompt.
  - Progress steps, the commit and the close move nothing.
- `timeTravelFocusElementV1`: the first focusable control under `…/framework.history.editor.input.<pointer>`, else Accept (its row button), the band, or the dialog's first control.
- `timeTravelFocusIsHeldV1`: an editable field outside the History panel and the prompt keeps focus, so an agent-begun session never steals someone's typing.
- `scheduleTimeTravelFocusV1(root, target, frames)`: retries per animation frame until the target mounts.
- The band is now `tabIndex=-1` (focusable, not a tab stop). Its controls carry `min-h-medium px-tiny` (touch targets) and the band wraps within `max-w-[90vw]`.
- ShellHost (`RE/🏛️ShellHost/🟦️.tsx`): `revealHistoryPanelRef` reveals the History tab through the dock's anchor, or the merged mobile panel when `mobile`. One session effect applies the transition, using up to 120 frames.
  - A pending focus is cancelled only by a newer focus or by the close, never by a progress patch.
- Corpus: a new required `transitions` array (13 rows) in the band corpus, with a matching `Transition` definition in `🧬️schema/🔣️time-travel-band`.

**Blocking-rule copy (§16.1).** A peer changed it at 12:11: `review.blocked` now reads "Errors must be fixed or withdrawn before finalizing" / "Fehler müssen vor dem Abschließen behoben oder zurückgezogen werden". This changed `TIME_TRAVEL_LABELS.reportBlocking`, the corpus, the bundles and wgpu. The existing React law (bundle == vocabulary) holds.

**Interpreter accessibility.** Gaps found and fixed in the React rendering of the Rust history body (`RE/🗣️Interpreter/🟦️.tsx`, `UI/🧱️elements/🪜️Stepper/🟦️.tsx`):
- Tree rows dropped `style.tone`, so outcome rows had no severity colour. New `TREE_ROW_TONE_CLASSES` (info/success/warning/danger → `text-info`/`text-success`/`text-warning`/`text-destructive`) is applied as the row `className`. The icon and words were already there; tone is now the third cue.
- An interpreted `numberStepper` had no accessible name. Its `aria-labelledby` pointed at a label element that only exists with `showLabel`.
  - `Stepper` gained an `aria-label` prop. When one is given, the dangling `labelledby` is dropped.
  - `NumberStepperView` forwards `accessibility.label` and `disabled`.
- The interpreted `slider` now publishes `aria-valuetext` (value plus unit) through `uiAccessibilityValueV1`, as the declarative path already did.

**R2-6 (no check-in during finalize) is extracted so it can be tested.** New in `RE/🛠️ShellHelpers/🟦️.tsx`:
- `checkpointGateV1(timeTravel, message)` → dispatch, wait (`"auto"`) or frozen.
- `checkpointOnCloseKeyV1(editor, program, documentId)`.
- `useCheckpointOnCloseV1(key, onClose)`: the closure of the render that opened `key` runs when `key` changes.
- ShellHost's `dispatchCheckpoint` and the close effect now use these three. The behaviour is the same as session 1's fix, now pinned by laws.

**R2-4 (document identity) is extracted the same way.** `syncAttachDocumentIdV1(target, identity)` (`RE/🛠️ShellHelpers/🟦️.tsx`) is used by ShellHost `openSyncTarget`:
- a folder or file attach addresses the program's own store id;
- a hub target uses its own document id;
- a program without a document attaches nothing.

**`📎️local-folder-bindings` schema and React consumer (coordinator item 2).**
- New schema `ENG/🧬️schema/🔣️local-folder-bindings/🔣️.json`. It references the `📎️attach-local-folder` and `✂️detach-local-folder` leaf payload schemas by `$id`.
- Taxonomy for that directory is clean. It first sat under `📎️local-folder-bindings`, which the taxonomy rejected as `directory-kind-unresolved`, so it moved to the open `🔣️<slug>` pattern.
- The React suite `RE/🏛️ShellHost/📎️local-folders/🧪️tests/🧩️component` now asserts the shared corpus (see S2.3).
- Reconnect band: `max-w-[90vw]` and touch-size buttons.

**framework-os** (`🧰️framework/🛍️products/💻️os/🧪️tests/🧪️folder-archive-restore/🟦️.ts`):
- New law: a folder-bound document admits 3 batches stamped with its own id, and refuses a batch addressed by the old runtime counter (`puzzle2d-1`) with `local.backbone-scope-mismatch`. Nothing is added to the pending log.
- The envelope literal gained `line: null`, adopting the peer's 12:08 wire field.

### S2.3 Verification (foreground)

**Final run, without the shim (16:4x).** S2-INFRA re-emitted the generated registry at 16:36, so these runs use the real renderer-react config (`../../🧪️tests/🎚️config/🟦️.ts`). The ticket-local shim files are deleted; no repo file ever referenced them.

| Command (renderer-react package dir) | Result |
|---|---|
| `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ⏪️time-travel/🧪️tests/🧩️component 📎️local-folders/🧪️tests 🧪️command-rejection 🧪️staged-arg-controls 🌐️chrome-history-locale 🛠️ShellHelpers/🧪️tests/🧩️component 👥️scoped-presence` (`vitest-noshim-1.txt`) | **7 files, 80/80 ✔** |
| same config: engine-contract, editable-controls, retained-control-commit, puzzle3d-settings, section-collapse, toggle-semantics, Interpreter in-source, keybinding-glyphs, command-panel, ChromePanels (`vitest-related-2.txt`) | 953/967 ✔, 14 ✘, none from W2-B. The same peer and pre-existing set as below, except: W1-E's conformance corpus grew from 71 to 76 cases ("loads all 71 corpus fixtures"), and the catalog-feedback timeout did not recur |
| `bunx tsc --noEmit -p tsconfig.json` (`tsc-react-4.txt`) | 6 errors, 0 in W2-B files: the peer's `line` field in the worker and backbone-parity, `idleInstalledServiceStatusV1`, and W1-E's `SliderProps` in Interpreter stories. TS2554 from the registry is gone |

**Earlier runs (with the ticket-local shim, see S2.1):**

| Command (package dir) | Result |
|---|---|
| renderer-react `bunx tsc --noEmit -p tsconfig.json` (`tsc-react-3.txt`) | 10 errors, 0 in W2-B files (all peer, S2.1) |
| ui-react `bunx tsc --noEmit -p tsconfig.json` (`tsc-ui-1.txt`) | 6 errors, 0 in W2-B files (peer G6 numeric, lease) |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config T/🗑️generated/s2-w2b/vitest-shim.config.ts ⏪️time-travel/🧪️tests/🧩️component` | **31/31 ✔** (was 17; `vitest-tt-6.txt`, 16:4x after the peers' 13:42/13:49 Stepper/Interpreter edits) |
| same, `📎️local-folders/🧪️tests` | **9/9 ✔** (was 3) |
| same, `🧪️command-rejection` + `📎️local-folders` | 12/12 ✔ |
| same, `🧪️staged-arg-controls` (`vitest-staged-1.txt`) | **13/13 ✔**. The vector page-key law now reads the shared `UI/🧬️contract/🧫️fixtures/🧫️number-controls` `keys` rows (every `pageUp`/`pageDown` case) instead of a local expectation, as the coordinator decided (16:4x). When S2-W1E moves the corpus and `uiNumberKeyValue` to "PageUp/PageDown go to the next/previous detent", this law follows without an edit. |
| same, the 5 W2-B suites, before any change (`vitest-mine-2.txt`) | 38/38 ✔ |
| same, W2-B + related (staged-arg, chrome-history-locale, ShellHelpers component, scoped-presence, keybinding-glyphs, command-panel, ChromePanels) (`vitest-mine-3.txt`) | 92/94. Both reds are fixed or not mine: `🧪️command-rejection`'s hard-coded list (fixed, now 12/12), and staged-arg "vector axis … detents on the page keys" (expected 1.5, got 1.2), caused by the peer's 12:34 `uiNumberKeyValue`/scale rework in `UI/🧬️contract/🧩️component` |
| same, engine-contract, editable-controls, retained-control-commit, puzzle3d-settings, section-collapse, toggle-semantics, Interpreter in-source (`vitest-related-1.txt`) | 931/945 ✔, 14 ✘, none from W2-B (list below) |
| ui-react `bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts …translation-totality… 🪜️Stepper 📨️UIDialog` | 2 files, 11/11 ✔ |
| framework-os `bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🔨️modules/🏪️store/👷️worker/🟦️.ts -t "folder archive restore"` | **5/5 ✔** (new R2-4 law included) |
| `bun ./📜️script.ts verify taxonomy report --scope ENG/🧬️schema/🔣️local-folder-bindings` | clean |
| same, `--scope RE/🛠️ShellHelpers` | 22 errors, all in directories W2-B did not create (`🎭️browser-actor-panels`, `📌️panel-carriage`, `🌐️instance-title` and others), plus one "preimage changed" because I edited a file during the scan |

The 14 failures in the related run:
- 4 in puzzle3d-settings: the intent `input` is now `{value, gesture, commit}`. That is the peer's continuous-lane protocol; the steppers' accessible names pass.
- 6 in engine-contract text-editor key/paste/compose, and the portal z-tutorial test: pre-existing and peer-owned.
- 1 SpaceToken gap CSS and 2 Interpreter Overlay layout: peer layout tokens.
- 1 catalog-feedback test: timed out under load.

New React laws (time-travel suite):
- **Transitions:** the corpus transitions; focus-element resolution in the shell's real DOM, including the Accept fallback through the row's lone button; scheduled focus waits for mount, keeps the prompt's own focus and never takes focus from a field elsewhere.
- **Controls:** every input control is named (stepper "dx" with min/max/step; slider "Angle" with `aria-valuetext` "0 °" and one tick per snap; select; vector axis "x" with bounds; reference chips "Remove node-1"; "Use selection"). Each dispatches `historyEditInput{generation, path, value}` or `historyEditUseSelection{generation, path}`.
- **Alternatives:** the section lists the main line and the alternatives, says "Current" in words, and switches with a named "Switch" button that dispatches `switchAlternative{alternativeId}`.
- **Keyboard:** Tab reaches Accept, dx, the slider, mode, both vector axes, both chips and "Use selection", in row order.
- **Outcomes:** each outcome row shows words, an icon and a tone class.
- **Phone fit:** the band fits a phone (90vw, wraps, touch-size controls).
- **Finalize prompt:** rendered from the shared `🛂️manifest/🧫️fixtures/🧫️dialog-choices` fixture, the prompt opens with focus on its "Alternative name" field, has a destructive "Overwrite" and "Back", and shows 0 ARIA findings.
- **ARIA oracle:** an axe-like structural check (aria-query role model plus dom-accessibility-api) over band and body finds 0 findings. It checks for unknown or unsupported `aria-*`, dangling `labelledby`/`describedby`, unnamed controls and duplicate ids.
- **R2-6:** the checkpoint gate for every corpus stage; the close hook fires once per document left and never on a new session object of the same program (the New-alternative submit); while the history was being edited, it dispatches nothing.
- **R2-2 / G5:** a read-back snapshot replaces the displaced document's rows. Every label `data-history-json` publishes is the `🧫️history-label-reload` expectation in en and de, and none is op text.

### S2.4 Open items and routing

- **Coordinator:**
  - Re-activation of React 6012 for the live re-probe of steps 5, 7 and 9 (R2-2, R2-4, R2-6 and the reconnect band are proven only at unit and worker level). Probe verdicts to add: `history-panel-reveals-on-session-start` and `focus-moves-to-the-editor`.
  - `axe-core` is not installed in the repo. Adding it as a devDependency of renderer-react needs a repo-wide `bun install`, which needs the coordinator's or the dev's consent. Until then, the aria-query and dom-accessibility-api structural oracle stands in.
- **S2-W2A (Rust history body):**
  - the R2-2 law `a_document_archive_round_trip_lists_every_history_row_of_its_source` was not re-run here (plugin lib-test target red from in-flight edits, 21 rustc at the time); request sent to S2-W2A;
  - the `Dial` arm drops `snaps`, `unit` and `displayFactor` (R29/G6);
  - stepper rows carry no snaps;
  - reference chips show raw ids, not entity labels (G3, §16.4);
  - optional: a Rust law tying `time_travel_editor_sections` keys to the React selectors `…editor.input.<pointer>` and `…editor.accept.row`.
- **S2-W2A wire adopted (their §8.5).** `HistoryMutationEntry.introduced` is rendered by the Rust body, and React adds nothing to it. `TIME_TRAVEL_CODE_LABELS` is enforced by the new React law. React never used `timeTravelFaultLabel`, which is now deleted.
- **S2-W2C (told through the coordinator):**
  - the band corpus now has 20 refusals and 12 cases, and a new `transitions` array (wgpu could assert reveal and focus against it);
  - the local-folder corpus now has a schema.
- **Peers:** `idleInstalledServiceStatusV1` (inference); the replication envelope `line` call sites in the worker; the G6 slider/number facets.

## Session 3 — 2026-10-02

Successor executor S3-W2B. Aliases as above (`RE`, `UI`, `ENG`). Scratch output: `🗑️generated/s3-w2b/`.

**Status: IN PROGRESS** (this section is kept current at every milestone).

### S3.1 Repair (rule 28)

The session-2 section ends at 17:00 on 2026-10-01. The cut session-2 resumption (03:20–03:40 on 2026-10-02, for audit wave A) left
these W2-B edits on disk without a report entry. Each was read in full against HEAD `25bb77059d6`; none was half-finished:

- **B-1 (staged vector axes, §18).** `RE/🛠️ShellHelpers/🟦️.tsx`: `StagedVectorAxis` routes ArrowUp/ArrowDown (Shift = ten steps),
  PageUp/PageDown and Home/End through `uiNumberKeyValue`; a typed display value reads back through `uiNumberTypedValue`; a value
  crossing a hard bound (`uiNumberCrossedBound`) is refused visibly (`aria-invalid`, `role=alert` line, draft kept, nothing staged).
  New keys `ui.numberField.{belowMin,aboveMax}` (en/de, schema + bundles). Laws in `🧪️staged-arg-controls`: every `keys`, `typed`
  and `limits` row of `UI/🧬️contract/🧫️fixtures/🧫️number-controls` on a one-axis vector.
- **B-2 (reveal/focus wiring).** The ShellHost effect moved into `useTimeTravelRevealV1` + `revealHistoryPanelV1`
  (`RE/🛠️ShellHelpers/⏪️time-travel/🟦️.tsx`); ShellHost calls the hook. Laws (`⏪️time-travel/🧪️tests/🧩️component`, region
  `🛰️ShellReveal`) drive the shell's real `shellReducer`: desktop anchor, merged mobile panel, no History tab, the focus surviving
  progress patches and cancelled by the close, held typing vs. the modal prompt, and an xstate model of the transition table checked
  against every corpus `transitions` row.
- **B-3** Stepper `aria-labelledby` only with `showLabel` (+ law). **B-4** reconnect band phone-fit law. **B-5** unique docstring
  emoji, `draftBody` docstring moved, orphan docstrings deleted. **B-6** `timeTravel.member-gone` in `HISTORY_REFUSAL_LABEL_KEYS`, both
  bundles and the band corpus (the vocabulary already has it). **B-7** the modal prompt always takes focus. **B-8** band-corpus
  transitions carry `blocking` + `acceptedCount`, required by the corpus schema.

### S3.2 Verification (foreground)

| Command | Result |
|---|---|
| renderer-react `bunx tsc --noEmit -p tsconfig.json` (`tsc-react-1.txt`) | 4 errors, 0 in W2-B files: worker/backbone-parity `line` (×3), `idleInstalledServiceStatusV1` (`RE/🐚️Shell/🟦️.tsx:1112`) |
| ui-react `bunx tsc --noEmit -p tsconfig.json` (`tsc-ui-1.txt`) | exit 0 |
| renderer-react `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ⏪️time-travel/🧪️tests/🧩️component 📎️local-folders/🧪️tests 🧪️command-rejection 🧪️staged-arg-controls 🌐️chrome-history-locale 🛠️ShellHelpers/🧪️tests/🧩️component 👥️scoped-presence` (`vitest-mine-1.txt`) | **7 files, 89/89 ✔** |

### S3.3 B-1 completed across every React number control (design §18)

The audit's cross-file half: the history editor's own number rows and vector axes are interpreted `input` nodes (`InputView`), its
steppers interpreted `numberStepper` nodes (`Stepper`); both handled only part of the law.

- **One key mapping** `uiNumberFieldKey(key, shiftKey, min, max)` in the UI contract (`UI/🧬️contract/🧩️component/🟦️.ts`, next to
  `SliderKey`): ArrowUp/ArrowDown (Shift = large), PageUp/PageDown, Home/End **only toward a bound the field has** (the WAI-ARIA
  spinbutton pattern: without one the caret keeps the key), `null` for everything else (ArrowLeft/Right stay the caret's). ShellHelpers'
  local `numberFieldKeyV1` is deleted; `StagedVectorAxis`, `InputView` and `Stepper` all use the contract mapping.
- **`InputView`** (`RE/🗣️Interpreter/🟦️.tsx`): `pageKey` → `lawKey`, every law key through `uiNumberKeyValue` (draft of a
  blur-committed field, lane of a continuous one). **`Stepper`** (`UI/🧱️elements/🪜️Stepper/🟦️.tsx`): Home/End added; `handleKey`
  takes a `SliderKey`.
- **Laws.** New `RE/🗣️Interpreter/🧪️tests/🧪️number-keyboard-law` (registered in the Interpreter's in-source block): the mapping, and
  for a blur-committed number field AND an interpreted stepper every `keys` row (dispatched intent = law value; unbound Home/End not
  prevented, nothing dispatched), every `typed` row, every `limits` row (refused: `aria-invalid`, draft kept, nothing dispatched).
  `🧪️staged-arg-controls` and the Stepper component law now take all six keys (they skipped Home/End).

### S3.4 N12 (coordinator, from `📓️s3-gap.md`): `os.config.attach-local-folder` `/folder` was not editable

- Cause: the `folder` object (`LocalFolderRef {kind: "path", path}`) declared `widget: text`, which cannot edit an object.
- Fix (schema-first, `🎚️config/🧬️schema/🧬️mutations/📎️attach-local-folder/🧬️schema/🔣️.json`): `folder` keeps its label/description
  (no widget); `$defs.LocalFolderRef.properties.path` gets `x-semio-ui {widget: text, role: value, label {Path, Pfad}, description}`.
  The const `kind` is the discriminator, never an input. The time-travel editor flattens the object into one row
  "Folder · Path" / "Ordner · Pfad" (`/folder/path`). Payload shape, fixtures and the TS/Rust leaf types are unchanged.
- Laws: Rust `the_folder_is_edited_in_history_through_its_path_and_its_kind_is_no_input` (leaf unit tests, `mutation_input_defs` over
  `PAYLOAD_SCHEMA`); TS twin in `📎️local-folders/🧪️tests/🧩️component` (`mutationInputDefs` + `argControl`).

| Command | Result |
|---|---|
| test module `bun ./📜️script.ts schema mutation-inputs --under 🧰️framework/🛍️products/💻️os/🎚️config` (`inputs-config-2.txt`) | **0 findings**, 51/51 inputs of 20 leaves (strict) |
| renderer-react vitest `📎️local-folders/🧪️tests` (`vitest-folders-1.txt`) | **11/11 ✔** |
| renderer-react vitest `🗣️Interpreter/🟦️.tsx 🧪️staged-arg-controls -t "number-control law\|staged mutation-input controls"` (`vitest-interp-law-2.txt`) | **23/23 ✔** (7 new Interpreter laws + 16 staged) |
| ui-react `SEMIO_VITEST_POLICY=<repositoryVitestPolicyV1> SEMIO_TEST_BUDGET_MS=540000 bun ./📜️script.ts test 🪜️Stepper 🔬️translation-totality` (`vitest-ui-3.txt`) | **5/5 ✔** |

Note: the ui-react vitest config now refuses to load without `SEMIO_VITEST_POLICY` (peer change, `🏃️process/🧪️testing/🧪️vitest`); the
policy was derived with the repo's own `repositoryVitestPolicyV1(<package dir>)`. The default 15 s budget kills the run at load ~100.

### S3.5 Staged-arg number facets (coordinator, from S3-W1E)

`renderStagedArgValueControl` (`RE/🛠️ShellHelpers/🟦️.tsx`) now draws every staged number control from the manifest's
`actionArgNumberFacets` (TS twin of the Rust `ActionArgDef::number_facets`, the wgpu shell's source) through one wrapper
`stagedNumberFacetsV1(def)` (refusals in the shell's current language, read from the label port):

- **Slider/dial**: travel `min/max`, `step`, `snapValues`, `scale`, `appearance` (a dial is a dial now), `displayFactor`,
  `precision`, `limits` (hard bounds; the travel is soft), readout and `aria-valuetext` with the display unit. The value is never
  clamped to the travel.
- **Stepper**: key range, step, precision, `snapValues`, `displayFactor`, unit symbol, `limits`, `aria-valuetext`.
- **Number field and vector axes**: one `StagedNumberField` (replaces `StagedVectorAxis` and the raw `<Input type=number>`):
  display units, §18 keys, typed read-back, refusal = the facet's localized limit refusal; a lone field shows its unit; an
  emptied optional field clears. The shell-side fallback copy `ui.numberField.{belowMin,aboveMax}` is deleted (schema + both
  bundles) — every limit now carries the framework refusal ("Must be at most 30 °" / "Darf höchstens 10 mm sein").
- `stagedNumberDisplayText` reads through `uiNumberDisplayText`; ShellHelpers' unused `number-format` import is gone.
- Law (`🧪️staged-arg-controls`): every case of `🛂️manifest/🧫️fixtures/🧫️number-facets` — the shell's facets equal the corpus's
  (en/de per case); sliders/dials show the dial look, one tick per admitted detent, `aria-valuetext`, spoken `aria-valuemin/max`;
  steppers/number fields/vector axes show the display value and unit and refuse a typed value beyond the limit with the
  corpus's localized refusal; the two no-facet cases have none.

### S3.6 N1 / N15 / editor keys (coordinator, from S3-W2A §9.2)

- **N1 (paged mutation rows).** React needs no change: the History tab is mounted with the shell's `treeWindowHost`, and the
  interpreter's generic nested windows file `TreeWindowRequest`s under the container path. Law (`⏪️time-travel/🧪️tests/🧩️component`
  region `🪟️MutationPages`): a history row windowed over 40 mutations with 8 materialised exposes
  `data-tree-window-path = framework.history.commands␟framework.history.entry.4`, total 40, length 8, a 32-row trailing spacer.
- **N15 (refused Edit).** React honours `rowActions[].disabled` as it is: the Tree renders the action natively `disabled`
  (exposed to assistive technology as unavailable), its accessible name is the Rust label naming the reason, and the row has no
  activation. Law: Edit dispatches `historyEditBegin{mutationId}` while legal; while refused (en "Edit: Not possible right now",
  de "Bearbeiten: Derzeit nicht möglich", texts from the band corpus) all 8 Edits are disabled and named with the reason, and
  neither the buttons nor the rows dispatch anything; 0 ARIA findings. A focusable `aria-disabled` variant was tried in
  `🌳️Tree` `renderTreeHeaderAction` and reverted: it changes every tree header/row action of the product (Execute/Reset of the
  action panes, command panel) and broke their engine-contract laws — a UI-wide decision for W1-E/the coordinator (open item).
- **Editor keys (S2.4 optional law).** The band corpus gained `editorKeys` (schema `EditorKey`/`EditorInputKey`): the History
  panel id, Accept + its row, and per input pointer (object fields flattened) control + row keys. React law: focus resolution on
  those keys in the interpreter's real DOM (first input, else Accept), inside the History panel (never "held"). Rust law
  `the_draft_editor_keys_are_the_shared_corpus_keys_every_shell_focuses` appended (own region `🗝️EditorKeys`) to
  `🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs`: renders the editor for `/dx` and `/pivot/x` and requires exactly the corpus keys
  (panel id = `FRAMEWORK_PANEL_TAB_HISTORY_ID`). WRITTEN, run pending (plugin lib-test build, see S3.7).

### S3.7 Verification so far (foreground; renderer-react package dir unless noted)

| Command | Result |
|---|---|
| `bunx tsc --noEmit -p tsconfig.json` renderer-react (`tsc-react-3.txt`) | 4 errors, **0 in W2-B files** (peer: worker/backbone-parity `line` ×3, `idleInstalledServiceStatusV1`) |
| `bunx tsc --noEmit -p tsconfig.json` ui-react (`tsc-ui-2.txt`) | **exit 0** |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ⏪️time-travel/🧪️tests/🧩️component 🧪️staged-arg-controls command-panel` (`vitest-tt-4.txt`) | **3 files, 61/61 ✔** |
| same, `🔬️engine-contract -t "staging and single dispatch\|command panel\|window action"` (`vitest-p1p2-2.txt`) | **13/13 ✔** (two staged-form laws went red on the first facet rewrite — Execute gating and Reset — fixed: the reverted Tree change, and a staged number draft now yields to an outside value change) |
| same, `🗣️Interpreter/🟦️.tsx 🧪️staged-arg-controls -t "number-control law\|staged mutation-input controls"` (`vitest-interp-law-2.txt`) | **23/23 ✔** |
| same, `📎️local-folders/🧪️tests` (`vitest-folders-1.txt`) | **11/11 ✔** |
| ui-react `SEMIO_VITEST_POLICY=… SEMIO_TEST_BUDGET_MS=540000 bun ./📜️script.ts test 🌳️Tree 🪜️Stepper 🔬️translation-totality` (`vitest-ui-4.txt`) | **4 files, 60/60 ✔** |
| related set: `🔬️engine-contract editable-controls retained-control-commit puzzle3d-settings section-collapse toggle-semantics 🗣️Interpreter/🟦️.tsx keybinding-glyphs command-panel 📌️ChromePanels` (`vitest-related-1.txt`) | 19 ✘ in the first run; after the fixes none is W2-B's: puzzle3d-settings ×4 (peer continuous lane), text-editor key/paste/compose ×5 and portal `Worker is not defined` (pre-existing), SpaceToken ×1 + Overlay ×2 (peer layout), catalog-feedback timeout, `FlowGraphCanvasHost` "graph parameter keyboard" (peer NodeGraph/flow, `🕸️NodeGraph/🟦️.tsx` 12:01; red in isolation too, `vitest-recheck-1.txt`); ChromePanels tree order and the Interpreter surface-host law were load timeouts and pass in isolation |
| test module `bun ./📜️script.ts schema mutation-inputs --under 🧰️framework/🛍️products/💻️os/🎚️config` | **0 findings** (51/51 inputs, 20 leaves) |
| `cargo test -p semio-framework-os-config --lib -- local_folder` (private target, `cargo-os-config-{1,2}.txt`) | **BLOCKED by a peer**: `semio-framework-schema-registry` (28 errors at ~12:20) then `semio-framework` `🛂️manifest/🦀️.rs:1266-1269` (`with_schema_export_registry`, `SchemaFormat`, `registered_referenced_schema_documents` not in `semio_framework_schema`) — reported to the coordinator |
