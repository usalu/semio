# 📓️ Audit: `⏪️time-travel`, `🛠️tool-machine` and the UI contract (W1-B, W1-C, W1-E)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, read-only audit, 2026-09-30. Sources of truth: `📋️design.md` §4, §5, §6, §7, §10, `AGENTS.md`, `📓️w1-b-report.md`, `📓️w1-c-report.md`, `📓️w1-e-report.md` and the approved interpretations recorded there.
No source file was edited. The only file written to the repo is this report. Scratch lives in the session scratchpad.

Aliases: `FW` = `🧰️framework/🔨️modules`, `TT` = `FW/⏪️time-travel`, `TM` = `FW/🛠️tool-machine`, `UI` = `FW/🖱️ui`, `SHELL` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `MAN` = `FW/🛂️manifest/🦀️.rs`.
The manifest, the plugin runtime and the shell are being edited concurrently by peers; line numbers in those files are "at audit time" and drift.

## 0. What I ran, and what I only read

Ran (all foreground, no cargo):

| Command | Result |
|---|---|
| `bun test ./TT/🧪️tests/🧪️conformance/🟦️.ts` | 15 pass, 0 fail, 2110 expects |
| `bun test ./TM/🧪️tests/🧪️conformance/🟦️.ts` | 14 pass, 0 fail, 9007 expects |
| `bun ./📜️script.ts verify taxonomy report --scope` for `TT`, `TM`, `UI/🧬️contract/🧫️fixtures/🧫️number-controls`, `MAN`-side `🧫️fixtures/🧫️dialog-choices` and `🧪️tests/🧪️dialog-choices` | all `clean=true errors=0 warnings=0` |
| The ticket generator `🧪️w1-b-generate-lifecycle-law.py`, output redirected to scratch | byte-identical to the committed `🧫️lifecycle-law/🔣️.json` (`cmp`) |
| Four independent mutations of the TS `⏪️` twin, run against a scratch copy of the module and its suite | every mutation fails the matrix, case, scenario, xstate and (where relevant) fast-check tests; restored copy 15/15 |
| Five independent mutations of the TS `🛠️` twin, same method | every mutation fails at least one test; restored copy 14/14 |
| Standalone `rustc` build of `UI/🧬️contract/🔢️number-format/🦀️.rs` next to `bun` `toFixed` | one parity mismatch, see minor N-14 |
| Scratch bun scripts on the twins | reproduce M-2 (transaction leak), M-3 (cancel ambiguity), N-2 (stale fault, empty codes) |

Only read, not executed: the Rust unit suites (`cargo` is out of scope), the vitest suites (`UIDialog`, `Slider`), the wgpu shell tests, and any screen reader. Findings M-5, N-16 and N-17 rest on code reading and grep only.

## 1. Verdict

No critical findings. Both reducers, both twins, both fixtures and the number-control laws are correct against their specifications, and the oracles are demonstrably non-vacuous.

The weaknesses are at the seams:
- the **tool-machine contract** is not yet sufficient for interactive tools (no host abort, silent transaction leak, no persistence story, and its only production consumer bypasses the interactive path);
- the **time-travel driver contract** has one wrong sentence ("Reviewing without a report means no changes") that a downstream WP already built on;
- the **finalize dialog** gates the destructive choice on an unrelated required field;
- the **wgpu declared dialog** is not exposed to assistive technology.

## 2. Ranked findings

### Critical

None. Reasoning: no path found in W1-B, W1-C or W1-E that corrupts a document, loses committed history, or breaks a documented law. The reducers refuse without side effects, the fixtures reproduce from an independent generator, and the twins agree.

### Major

| # | Where | Finding | Fix |
|---|---|---|---|
| M-1 | `TM/🦀️.rs:231-283` (`impl ToolMachineRunner`), `TM/🟦️.ts:105-146` | **No host abort.** The runner exposes `start`, `send`, `timer_elapsed`, `transaction`, `snapshot`. `snapshot()` is read-only and `transaction` is private. The host cannot drop an open transaction or reset the tool unless the tool happens to declare an abort event, and the `ToolMachine` trait does not require one. Needed for: window blur, pointer-capture loss, a remote change to the gesture's targets (`BaseMoved`), entering a time-travel session (freeze), plugin retirement. | Add `fn abort_event() -> Self::Event` to `ToolMachine` and `ToolMachineRunner::abort(clock) -> ToolStep` that sends it, then drops any still-open transaction and returns `Aborted(ref)`. Add fixture rows (abort while `dragging`, abort while idle = `Idle`, abort while `pressed`) checked by Rust, TS and the xstate oracle. |
| M-2 | `TM/🦀️.rs:285-324` (`settle`), `TM/🟦️.ts:147-172` | **Silent transaction leak across gestures.** Nothing ties "a transaction is open" to "the tool is not resting". If a statechart returns to its initial state without yielding `Commit` or `Abort`, the transaction stays open and the next gesture's upserts join it (same ref, same keys replaced). Reproduced with the TS twin (scratch `leak.ts`): gesture 1 press/move/release without commit leaves `tx-1c454bdb158cb48f` open with the tool `idle`; gesture 2's move replaces gesture 1's `move` entry inside the same transaction. Two gestures become one edit and one undo step. The fixture has no such scenario, and the "tool author contract" in the W1-C report does not forbid it. | Give the runner a resting predicate (`ToolMachine::RESTING: &'static str` stable id, checked with `snapshot.matches`, or `fn is_resting(&Snapshot)`). After each settle, an open transaction on a resting tool is aborted fail-closed and reported (`Aborted(ref)` or a new refusal). Add a fixture chart variant "release without commit" and the matching scenario. |
| M-3 | `TT/🦀️.rs:17` and `TT/🟦️.ts:13` (driver contract), reducer arms `TT/🦀️.rs:447-457`; consumer `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs:365`, `:1342` | **"`Reviewing` without a report means no changes" is false after a cancelled or faulted replay.** `ReplayCancelled` leaves `Reviewing`, `accepted ≠ ∅`, `report = None`, `fault = None`. W2-A implemented exactly the documented rule (`no_changes: stage == Reviewing && report.is_none()`), so the panel prints `noChanges` ("No changes: showing the current history") while drafts exist and the preview is not the replayed head. `RequestFinalize` is then `timeTravel.blocked` ("resolve the pending change or the errors first") with nothing to resolve. There is no event to rerun; the only retry is `Begin` on a target followed by `Discard` (the fixture scenario "a cancelled replay is rerun when the next edit is discarded" is the workaround) or a base move. Reproduced with the TS twin. | Redefine no-changes as `accepted.is_empty()` in both contract docs and in the W2-A panel. Add `Rerun { generation }` (legal in `Reviewing` with `accepted ≠ ∅ ∧ report = None`, moves to `Replaying` with `StartReplay`), both twins, matrix rows and a scenario. Make `ReplayCancelled` record a fault marker (`timeTravel.cancelled`) so hosts can label it. |
| M-4 | `UI/🧱️elements/📨️UIDialog/🟦️.tsx:137` (`disabled={!canSubmit}` on every choice), `SHELL` `ChromeDialogRequest::action` (`ChromeDialogStop::Choice` arm uses `self.resolved()`, ~line 24569), `MAN` `history_edit_finalize_dialog` (`name` arg `.required()`, ~line 2457) | **A choice is gated by every required arg of the dialog.** In the one production dialog, clearing the pre-filled `name` disables **Overwrite**, although overwrite ignores the name. Both renderers, same defect. The `🧫️dialog-choices` fixture pins only the fully-staged case, so no test can see it. The choice dispatch also forwards the unrelated `name`. | Add `DialogChoice.validates` (`all` default, `none`, or an explicit id list) and use it in `unresolved_action_args` calls of React and wgpu. Overwrite = `none`, and its dispatch args are `{choice}` plus the generation only. Add fixture rows: empty `name` gates the submit and leaves Overwrite enabled. Update TS `dialogChoiceArgs` and Rust `choice_args` together. |
| M-5 | `SHELL` `chrome_surface_census` (~15821-15837: tour and palette only), `render_chrome_dialog_step` `Hit` arm (~30079-30083: `note_chrome_control_name/description/semantics` only), `ChromeDialogRequest::focus` (~24200), `handle_chrome_dialog_key` (~30156) | **The wgpu declared dialog is not exposed to assistive technology as a dialog.** `dialog_stack` is not part of the accessibility projection or the census; no `role=dialog`, no title, no body description, no modal flag. Gated submit and choices are painted dimmed with `event: None` but never call `note_chrome_control_disabled`, so they are announced as enabled no-ops. The keyboard focus index lives only in the dialog request; nothing feeds it into `accessibility_focused_control_id` (that is set from AT events only). The corpus case `🖥️composite/🔀️dialog-choices` proves the retained contract dialog *does* carry label and description on its root, so the shell chrome dialog is the odd one out. Verified by grep and reading only. | Emit a `dialog` projection node per `dialog_stack` entry (label = title, description = body, modal), call `note_chrome_control_disabled` for gated stops, publish the focused stop as the accessibility focus, and add a shell test that asserts these three. |

### Minor

Numbered for reference; grouped by module.

**W1-B `⏪️time-travel`**

| # | Where | Finding | Fix |
|---|---|---|---|
| N-1 | `TT/🦀️.rs:489-500` (`Choosing` in `BaseMoved`), `:296` (`Close`), no dialog effect | No effect dismisses the finalize prompt. `BaseMoved` in `Choosing` jumps to `Replaying`, and `Exit` from `Choosing` emits only `Close`, yet the prompt opened by `OpenFinalizePrompt` must be closed by the host inferring it from the stage. | Add `CloseFinalizePrompt` (emitted by `Back`, `BaseMoved` in `Choosing`, `Exit` in `Choosing`, `Choose`), or state the inference rule in the driver contract. |
| N-2 | `TT/🦀️.rs:618-630`, `:452-457`, `:468-476`; schema `TT/🧬️schema/🔣️.json:275` (fault `minLength 1`), `:313` (name unconstrained) | Three small state-hygiene gaps, all reproduced on the TS twin. (a) After `ReplayFaulted`, reverting the only accepted draft leaves `Reviewing`, `accepted = 0`, `fault = "boom"`: `settle` clears `fault` only when accepted is non-empty, and no invariant covers it. (b) `ReplayFaulted{code:""}` and `FinalizeFaulted{code:""}` are accepted and produce `fault:""`, which the schema forbids. (c) `Choose{Alternative{name:""}}` is accepted and emits `CommitAlternative{name:""}`; the design's scope limit (≤ 256 bytes, §2) is enforced only after the store round trip and the schema has no `minLength`/`maxLength` on `name`, no payload bound (design: ≤ 256 KiB). | Clear `fault` in `settle`'s empty branch and add an invariant `fault-needs-work`. Refuse empty codes and names with `Illegal` or `Blocked`, add `minLength 1` and `maxLength 256` to the schema, add fixture cases. |
| N-3 | reducer has no `TargetLost` event (`TT/🦀️.rs:487-501`) | A remote `Revert` of the edited or an accepted target: `BaseMoved.positions` simply omits it and the session keeps a draft on a mutation that is no longer applied. The store then refuses `Supersede`, the replay faults, and only `Exit` (which drops all drafts) remains. | Add `TargetsLost { mutations }` that drops those drafts (and the pending draft, returning to `return_stage`) and restarts the replay. |
| N-4 | `TT/🦀️.rs:502-505`, design §4 "Exit from any stage" | `Exit` is `Blocked` in `Finalizing` and `Illegal` in `Inactive` (approved deviation 7). If the commit driver never answers, the session cannot be left. | Contract note for W2-A: `Finalized` or `FinalizeFaulted` must be sent on every path including cancellation and timeout. |
| N-5 | `TT/🦀️.rs:116` and `:354` | Docstring emoji `🕰️` used twice in one file (`TimeTravelSession`, `TimeTravelRefusal::Stale`). Violates "unique emoji". | Change one. |
| N-6 | `TT/🧪️tests/🧪️conformance/🟦️.ts:246-252`, `:293-296`, `:374` | The xstate machine's targets and the expected effects are taken from the fixture matrix rows; guards and assignments are independent. Random sequences compare effect **kinds** only, and `startReplay` order. `showPreview` and `commit*` payloads are covered only by the 30 cases. Not vacuous (the mutations above were caught), but payload drift in a rarely-hit branch would slip past the fuzzer. | Compare effect payloads through the independent model as well (`showPreview.replacement`, `commit*.inputs`, `commitAlternative.name`). |
| N-7 | TS `TT/🟦️.ts:16` imports `../📡️replication/🟦️.ts` (relative, cross-module); `TT/🧪️tests/🧪️conformance/🟦️.ts:7` deep-imports a sibling schema | `TM/🟦️.ts:9` uses the package name `@semio-tech/framework-replication`. Two styles for the same dependency. The relative form bypasses the package's public surface. | Use the package specifier in both. |
| N-8 | `TT/🦀️.rs:658-748` labels | Five label rows (`Frozen`, `ChoiceOverwrite`, `ChoiceOverwriteDescription`, `ChoiceAlternative`, `ChoiceAlternativeDescription`) have no consumer at audit time. The finalize dialog (`MAN` `history_edit_finalize_dialog`) re-declares divergent copies ("Overwrite" vs "Overwrite history", different description sentence), and the frozen refusal message at `🔌️plugin/🦀️.rs` (~23894) is a hard-coded English `format!`. W2 is still in flight, so this is a risk, not yet a defect. | One owner for these strings. Either the dialog is built from `TimeTravelLabel`, or the dead rows are deleted. |

**W1-C `🛠️tool-machine`**

| # | Where | Finding | Fix |
|---|---|---|---|
| N-9 | production consumer `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs:155-178` | The contract is **unproven for interactive tools**. The only consumer builds a fresh runner per dispatch, feeds one `Records` event, passes a fake `HybridLogicalTimestamp{0,0,0}` and hides the real uniqueness in the `authoring_seed` used as "actor". Multi-dispatch gestures, the preview overlay (`transaction().entries()`), abort, timers and persistence are exercised only by the example drag chart. Also the "reusable module" rule wants two production consumers; there is one. | Keep as an explicit risk in the ticket. See section 5 for what to add before the second tool. |
| N-10 | `TM/🦀️.rs:272-283`, `TM/🟦️.ts:134-145` | The API forces a `HybridLogicalTimestamp` on **every** event although only the opening upsert uses it. Guest plugins have no clock; W2-D passes zeros. | Replace the per-event clock by a lazily invoked minter supplied at `start` (`FnMut() -> TransactionRef` fed by the host's seed or HLC). The id law stays in replication. |
| N-11 | `TM/🦀️.rs:223`, W2-D `SelectToolHost` (6 empty methods) | Every tool must implement the whole `Host` trait even though `execute_effect` is never called by the runner. | Provide `InertHost<T>` in the module and let `start` default to it. |
| N-12 | `TM/🧬️schema/🔣️.json:17-24` vs `TM/🦀️.rs:26-32`, `:234` | The schema demands `YieldKey`/`ToolId` `minLength 1` and the `<appId>#<toolId>` shape; neither implementation checks (`upsert("")`, `start("")` accepted). | Check in `start` and `ToolYield::upsert`, or drop the constraint from the schema. |
| N-13 | `TM/🟦️.ts:41-44` | `entries()` returns the live internal array and `upsert` mutates tuples in place, so a caller that caches the result sees later changes without a new reference (React memoization hazard). Rust returns a borrow. | Return a fresh frozen copy, or document. |
| N-13b | fixture `TM/🧫️fixtures/🧫️transaction-law/🔣️.json` | `oneClosePerEvent`, "yield while entering" and timers are laws stated in `invariants` but checked only by hand-written probe machines duplicated in each language. The xstate/random oracle mirrors the settle policy line by line, so it cannot detect a wrong policy. The random gesture fuzz almost never produces an `empty` commit (the `m1` mutation, empty commit reported as `committed`, was caught only by the deterministic scenario). | Put the probe charts into the fixture (like `gesture.chart`) and generate an `empty`-biased fuzz arbitrary. |

**W1-E UI contract**

| # | Where | Finding | Fix |
|---|---|---|---|
| N-14 | `UI/🧬️contract/🔢️number-format/🦀️.rs:34-58` (`exact_decimal_half`, `format_ui_number_fixed`) vs `🟦️.ts:6-10` | Rust is not a twin of `toFixed` for fractional ties at magnitude ≥ 2^50. Executed: `1125899906842624.25` at precision 1 gives Rust `1125899906842624.2`, JS `1125899906842624.3`. Cause: the round-down test parses the rounded text back to `f64`, which equals the input when the decimal step is below one ulp. Harmless for UI values, but the fixture claims the law. | For a detected tie format with `digits + 1` fraction digits (exact) and round the decimal string half-up; add the case to `🧫️number-controls`. |
| N-15 | `UI/🧱️elements/🎚️Slider/🟦️.tsx:120-124` (`sliderValuesMatch`, epsilon `step * 0.25`), `:234` (`changed`) | A detent within a quarter step of a ladder value cannot be reached from it, and cannot be left towards it. Example: step 1, snap 3.2, value 3: `publishValues(3.2)` is judged unchanged and dropped (arithmetic from the code, not executed). After an off-step detent an arrow key also skips the adjacent ladder value (3.2 + 1 → 4). wgpu has no such epsilon, so the renderers differ. | Compare detents exactly (`snaps.includes`) before the epsilon test. Add a pointer row with an off-step detent to `🧫️number-controls`. |
| N-16 | `SHELL` `slide` (~24505-24519), `set_slider` (~24495) vs `UI/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:672-688` and `Slider/🟦️.tsx:389` | Three slider keyboard laws. React and the retained wgpu slider page-jump **ten steps** when no detent lies ahead; the wgpu **dialog** slider jumps a tenth of the travel and does not normalize arrow results onto the step ladder (after a detent at 33.3, ArrowRight gives 34.3). Fixture `adjacent` pins `null` but not what a renderer does with it. | Add a `pageFallback` law to the number-controls fixture and use one helper in all three. |
| N-17 | `UI/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:1504` vs `:1502-1507`, `UI/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:442-455`, `:468` | In the retained painter the detent ticks are mirrored for right-to-left flow (`inline.is_rtl()`), but rail, range, thumb and `slider_value_at` are the documented "LTR value track". In an RTL flow a tick and the thumb of the same value disagree. React and the two `render_slider` call sites pass `false`. | Pass `false` (the track is LTR), or mirror the whole track including the pointer law. |
| N-18 | `UI/🧬️contract/🧩️component/🦀️.rs:446-457` (`SliderProps`) vs `:378`, `:483` | `InputProps` and `NumberStepperProps` carry `precision`; `SliderProps` does not, though design §6 gives `ArgSchema::Number` `precision` and the wgpu dialog slider honors it. Contract sliders show a 12-digit readout (`formatNumber`, `format_ui_number`) and cannot round commits. | Add `precision` to `SliderProps` (Rust, TS, projection, fixture). |
| N-19 | `UI/🧬️contract/🏗️builder/🦀️.rs:1371-1377` (`try_snap`), `:2184-2192` (`try_use_selection`) | `try_snap` validates against the bounds set so far, but `min`/`max` are infallible and `From` does not revalidate, so `slider(..).try_snap(5.0).min(6.0)` builds an invalid node found only at admission. `try_use_selection` binds `actions.children[0]` by position. | Validate in `From`, or make bounds a constructor argument; store the selection button by key. |
| N-20 | `UI/🧱️elements/📨️UIDialog/🟦️.tsx:127-131`, `SHELL` `chrome_dialog_paint_ops` (~29888, ~30008-30011) | Choice descriptions are rendered as an unlabeled block of muted lines **above** the button row in both renderers. With two described choices a sighted user cannot tell which sentence belongs to which button. The programmatic association (`aria-describedby`, `ChromeControlPresentation.description`) is correct. | Prefix each description with its choice label, or render it under its own button. |
| N-21 | `SHELL` `handle_chrome_dialog_key` (~30220-30229) | Text, number and axis fields in the wgpu dialog are append-only: no caret movement, selection, paste or IME. A keyboard user replaces the 14-character default name by pressing Backspace 14 times. | Reuse the retained text editor for staged text fields. |
| N-22 | `UI/🧬️contract/🛡️limits/🟦️.ts:13-20`, `UI/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts:341` (the literal `32`), `MAN` `validate_choices` (~4729), `MAN`-TS `🟦️.ts:1348-1354` | Twin gaps. TS `sliderSnapsAreValid` omits the 32-snap cap (only the wire decoder has it, as a magic number). `validate_choices` and the `🧫️dialog-choices` `invalid` cases exist only in Rust, so the TS renderer would render duplicate choice ids. `DialogChoice::destructive` promises "an agent must ask before taking it", but no agent lane reads `choice.destructive`; only the whole `historyEditCommit` action carries `.destructive()`. | Export the cap as a shared constant, add a TS `validateDialogChoices` run against the fixture, and either enforce or reword the destructive promise. |
| N-23 | `UI/🧬️contract/🏗️builder/🦀️.rs:2021-2121`, `:2124-2246`; corpus cases `🖥️composite/🧭️vector-input`, `🧷️reference-list` | Recipe accessibility: (a) the vector unit is only the `Field` container description, the number input's own `accessibility.description` is `null` in the corpus expectation, so the unit may not be announced with the input; (b) the chip list is `role=toolbar` without any roving-focus contract and shares its label with the group; (c) after a chip activation the chip unmounts and React focus is lost (the wgpu dialog restores it, `remove_chip`); (d) `remove_label` is a free string, so nothing enforces "label in name"; (e) the wgpu dialog drops the vector `unit` and bounds and shows raw ids on chips. | Give the input `describedBy` the unit, use `role=list` (or implement roving focus), restore focus to the next chip or the container, and require `remove_label` to contain the chip label in the builder. |
| N-24 | docstring emoji uniqueness, W1-E-added docs | Duplicates involving new lines: `UI/🧬️contract/🔢️number-format/🦀️.rs` and `🟦️.ts` (`🎯️` twice each), `🧩️component/🦀️.rs` (`🧲️` 460/486, `🎚️` 439/494), `🛡️limits/🦀️.rs` (`🧲️` 177/319), `🏗️builder/🦀️.rs` (`🧷️` ×3 in the ReferenceList region, `🧭️` ×3 with the VectorInput region, `🔢️`, `🎯️`, `⬇️`, `⬆️`, `➕️` reused from older builders). | Make each unique. |
| N-25 | pre-existing, adjacent to W1-E | `Slider/🟦️.tsx:527` the value readout is `role="button"` with only `onDoubleClick` and no `tabIndex`/key handler, so exact-value entry has no keyboard path; the thumb has `focus-visible:ring-0` (focus shown by background colour only). Not introduced by W1-E, but the history-edit input editors (W2) will depend on this slider. | Give the readout `tabIndex=0` with Enter/F2 to edit; restore a focus ring. |

## 3. Twin parity and fixture completeness

### 3.1 W1-B reducer, Rust vs TS

Compared branch by branch (`TT/🦀️.rs:407-508` against `TT/🟦️.ts:242-307`):

| Aspect | Result |
|---|---|
| Generation fence before any stage check | identical (`:410` / `:244`) |
| All 17 events × 6 stages, refusal code per pair | identical; the matrix (119 rows) is the shared witness |
| `Begin` (Inactive next id, Reviewing keep id, Editing switch/blocked, else illegal) | identical |
| `Accept` (unchanged, revert-to-original removes, insert in history order) | identical: Rust `partition_point` vs TS `sort`, equal for unique ordered keys |
| `settle`, `close`, `resume`, `reposition` | identical, including fault handling and the u32 wrap (`wrapping_add` vs `>>> 0`) |
| `BaseMoved` per stage and the stale-base refusal | identical |
| `Exit` per stage | identical |
| History order `(position, id by UTF-8 bytes)` | Rust `String::cmp`, TS `compareCodePoints`; covered by a dedicated test |
| Labels | 18 rows equal across Rust, TS and the fixture (`labels_mirror_the_fixture`, `labels mirror the fixture`); en and de, placeholders `{done}`/`{total}` in both |
| No default language, no `is_de`, no `FrozenLabel` | confirmed by grep in both modules; `TimeTravelLabel::localized` hands both locales to the carrier |
| Wire `ToValue` | only `TimeTravelStage` (bare camelCase string, tested); session, events and effects have no `ToValue` and are shaped by the schema and the test converters |

Fixture completeness: all `6 × 17` pairs present (`every_stage_event_pair_is_covered_once_per_guard`), 18 guards, no repeated guard per pair, 30 cases, 8 scenarios, 10 invariants with counterexamples, stale generations tested `±1` in all 20 contexts. Reducer branches without their own matrix row but covered by a case or scenario: accept-changed with `return_stage = Inactive`, base move while editing with a kept report, discard to `Reviewing` with nothing accepted. Reducer branches with no coverage at all: the empty-name and empty-code inputs (N-2).

Oracle strength: four fresh mutations caught, see section 0. The pinned-seed test visits 34 of 34 legal rows.

### 3.2 W1-C runner and reducer, Rust vs TS

| Aspect | Result |
|---|---|
| Transaction reducer (12-cell matrix, 11 cases) | identical; `Map` oracle and the `into_parts`/`mutations` batch agree |
| Runner `settle` | identical control flow (`TM/🦀️.rs:285-324` vs `TM/🟦️.ts:147-172`), including refuse-and-drop, `Empty`, and non-yield commands still routed after a refusal |
| Id minting | both call replication (`TransactionRef::mint`, `mintTransactionRef`); 7 fixture ids pinned against the first-party TS BLAKE3 and against the `blake3` crate |
| `ToolStep::kind()` strings and JSON shape | equal to the schema `ToolStep` |
| Naming clash | `machine::ActorId` (kernel) vs `protocol::ActorId` (replication) both appear in `TM/🦀️.rs:10-11,188`; the const is fully qualified, still confusing |

Mutations `m1`..`m5` of the TS twin: empty commit reported as committed, retract that keeps the entry, abort that keeps entries, id minted from a shifted clock, refusal that keeps the transaction. All caught.

### 3.3 W1-E laws, three implementations

The pointer law (`slider_pointer_value`, `sliderPointerValue`) and the page law (`slider_adjacent_snap`, `sliderAdjacentSnap`) agree line by line: clamp, ladder from `min`, snap within 3 % of the span, first snap wins ties (`min_by`/strict `<`), strictly-above/below neighbour. `Math.round` and `f64::round` coincide because the operand is non-negative. Snap validity `slider_snaps_are_valid`/`sliderSnapsAreValid` agree apart from the count (N-22). Fixed formatting agrees except N-14. `choice_args`/`dialogChoiceArgs` agree (choice last, prior `choice` replaced).

## 4. Rule checklist

| Rule | Result |
|---|---|
| Schema-first | JSON Schema of record present for both modules and for the two W1-E fixtures; Ajv strict passes. Hand-written twins follow the `⏯️tool-run` pattern. Gaps: constraints in the schema that no implementation enforces (N-2, N-12) |
| Target neutrality | no `std::time`, `Instant`, `thread`, `fs`, `env`, `process`, `net`, `tokio` or `async` in either owner file (grep). Only `std::fmt`, `std::error` and `std::cmp`. The module reports say native, `wasm32-wasip2` and `wasm32-unknown-unknown` checks passed; not re-run by me |
| No external runtime dependencies | time-travel: replication and the value derive; tool-machine: `machine` (path) and replication. `blake3`, `serde_json`, `ajv`, `xstate`, `fast-check`, `decimal.js` are dev/test only; `blake3 = "1.8.2"` is pinned per crate exactly as in `hash` and `replication` |
| Docstrings start with a unique emoji | modules: one duplicate (N-5); UI: several (N-24) |
| No comments inside definitions | none in the four module owner files or their tests; none in the W1-E ChromeDialog region, builder additions or UIDialog (pre-existing block comments in `⚡️events/🦀️.rs` around `pointer_commit_action` are older) |
| `[DEBUG]` leftovers | none found in the module files |
| Concise code | yes; the ChromeDialog region is long but flat |
| No legacy shims | none seen in W1-B/C/E; `ConfigFieldShape`/`CommandFieldSpec` removal belongs to W1-D |
| Registration | root `Cargo.toml` members (lines 236-237) and workspace deps (372-373), taxonomy `members-of-modules` (12149-12151), nx projects present, `launch.json` lists both `@semio-tech/framework-time-travel-rs` and `…-tool-machine-rs` in the aggregate rows (same coverage as `tool-run`) |
| Multi-implementation | Rust, TS, Python generator for the B fixture; the C fixture ids come from a TS BLAKE3 plus the `blake3` crate |
| Test dir naming | new modules use `🧪️tests/🧪️conformance`, `⏯️tool-run` uses `🧩️conformance`; both pass the taxonomy, cosmetic |
| TS twins in production | neither twin has a runtime consumer yet (W2-B only `@see`s the time-travel twin); acceptable for a twin, but tool-machine has one Rust consumer where the reusable-module rule wants two (N-9) |

## 5. Is the tool-machine contract enough for real tools?

| Need | Verdict |
|---|---|
| Multi-dispatch gestures (press, N moves, release) | Works in the runner and the example chart, but unused in production: the puzzle 2d tool receives one `Records` event per gesture. Gap: M-2 (leak) becomes reachable the moment a tool spans events. |
| Preview overlay | `runner.transaction().entries()` gives the ordered provisional mutations and `reference()` the ref; `ToolStep::Open` carries no data by design. Sufficient. The host must apply the entries over `state_before(...)`; no helper. TS `entries()` aliases the live array (N-13). |
| Abort from host | Missing (M-1). |
| Persistence of tool context in window transient | Not addressed. `ToolMachineRunner` is neither `Clone` nor `ToValue`, holds a `Host`, and the context type is arbitrary (W2-D's holds an `Arc<Puzzle2dSnapshot>`). The W1-C report states "ephemeral, no persistence API"; that is acceptable only if retirement, hot swap and window handover are *defined* as abort-with-zero-trace, which needs M-1. Add an explicit `checkpoint()`/`restore()` if a tool ever needs to survive a plugin swap. |
| Timers | Routed to the host and settled like events; the id is minted from the opening event, a documented and tested choice. Needs an HLC per timer tick from the host. |
| Multiple closes per event | Strictly refused (`oneClosePerEvent`). An exit action that yields `Abort` before a transition action yields `Commit` therefore drops the whole event. Documented, but a footgun. |

Suggested order before the second tool: M-1, M-2, N-10, N-11, then the fixture rows.

## 6. Fix order

1. M-4 and M-3 first: both change contracts that W2 already consumes (the finalize dialog and the panel).
2. M-1 and M-2 together (one runner change, one fixture chart).
3. M-5 (wgpu dialog projection).
4. N-2, N-1, N-3 (reducer hygiene, one fixture regeneration through `🧪️w1-b-generate-lifecycle-law.py`).
5. N-15, N-16, N-17, N-14 (one `🧫️number-controls` pass, all renderers).
6. Remaining minors, including the emoji clean-up (N-5, N-24).

## 7. Verified OK

- Focus order in the React dialog is fields, Cancel, choices, submit, matching the fixture; Enter on a choice button takes the choice, Enter in a field submits the non-destructive path; the destructive choice is never the default focus or Enter target.
- A destructive choice carries an icon and its consequence text as well as colour (`triangle-alert`, `aria-describedby`); wgpu paints it in the error colour and registers the description.
- Escape, veil click and the Cancel button all send the declared cancel action in both renderers.
- Page keys jump to detents in React, retained wgpu and the wgpu dialog; arrows never snap, so a detent cannot trap the keyboard.
- Ticks are `aria-hidden` in React; one tick per detent in every renderer.
- Snap admission is enforced in Rust (both `validate_core` and the streaming path) and in TS (`sliderSnapsAreValid`), with `InvalidSliderSnaps` and a fixture verdict table.
- The bun conformance suites of both new modules pass as reported, and a fresh regeneration of the lifecycle fixture is byte-identical.

## 8. Files

Created: this report. Scratch (outside the repo): `leak.ts`, `fault.ts`, `fault2.ts`, `gen-lifecycle.py`, `nf/`, `mut/`, `mut2/` in the session scratchpad.
