# 📓️ W3-T2-CONTROLS Report: Scrub Machine, Continuous-Control Hosts, ToolRun Transactions

Executor W3-T2-CONTROLS. Scope: `📓️audit-remaining-tools.md` §7 row 1 (F-1, F-3, F-4, §5.2, §6.2) and design §13.1.
Nothing was committed by me; no ticket or goal was opened or closed. API for other executors: `📓️api-scrub-machine.md`.

Aliases: `FW` = `🧰️framework/🔨️modules`, `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `PLG` = `OS/🔌️plugin/🦀️.rs`,
`RE` = `OS/📺️renderer/🧑‍🎨engine/🧱️elements`, `TM` = `FW/🛠️tool-machine`.

## 1. Census (before this work)

| Control | Host behaviour | Guest commit |
|---|---|---|
| React `SliderView`, continuous number `InputView` | lane `{value, gesture, commit}` (no abort, no unmount/blur cancel) | per plugin: amend with key, or one edit per tick |
| React `NumberStepperView` (absolute) | one bare `change {value}` per click/keystroke | one edit per dispatch |
| React declarative `renderUiControl` slider/stepper (test-only path) | bare value per change | – |
| React text `Input` without `commit` | one `change` per keystroke | one edit per keystroke (F-4) |
| wgpu `Slider` | bare `Change {value}` on down, on every move, and on release only when released OVER the slider; keyboard/readout bare | same as React |
| wgpu `NumberStepper` (absolute), number `Input` (no commit) | bare value per click/keystroke | one edit per dispatch |
| gis terrain exaggeration | – | `Emit::amend(…, "gis3d-exaggeration")` static key |
| energy inspector sliders/numbers | – | one described edit per tick (no key) — ledger exhaustion class |
| energy name fields | per keystroke | one edit per keystroke |
| forms `patchQuestions`/`patchStep`/`patchQuestionOptions`/`patchVectorField`/`updateForm` | text per keystroke, numbers per tick | amend with static per-field keys (`patch:{field}:{ids}`, `patch-step:…`, `patch-option:…`, `patch-vector:…`, `change-form-title`) |
| norm `setField` text inputs | per keystroke | `Emit::commit` described edit per change |
| playbook title | palette/agent only (no field) | `Emit::amend(…, "playbook.title")` |
| ToolRun finalize | – | one edit, `transaction: None`, English-only description |

## 2. What landed

### 2.1 F-1: ToolRun rows are tool transactions
- `OS/🔌️plugin/⏯️tool-run/🦀️.rs`: `publish_tool_run` publishes with `Some(tool_run_transaction(app_id, tool_id, actor, run))` and NO
  description; `tool_run_transaction` mints `TransactionRef::mint(actor, {0, now_ms, run}, "<appId>#<toolId>")`.
- `tool_run_transaction_label(app_id, transaction)`: a transaction whose tool is `<appId>#<toolId>` of a declared tool run is
  labelled with the tool's own `LocalizedLabel` (all locales); `PLG::build_history_view` uses it ahead of the mutation-label
  rule, so the row reads the same after a reload (no single-locale description stored any more).
- Law `🧪️tests/🔬️tool-run` `tool_run_finalize_is_one_transaction_labelled_by_its_tool_in_every_locale`.

### 2.2 `ScrubMachine<M>` in `🛠️tool-machine` (Rust + TS twin, schema-first)
- Rust (`TM/🦀️.rs` region `🔖️Scrub`): `SCRUB_GESTURE_ARG/SCRUB_COMMIT_ARG/SCRUB_ABORT_ARG`, `ScrubPhase::parse/gesture/input`,
  `ScrubInput<M>`, `ScrubContext`, `ScrubEvent<M>` (`StatechartEvent`), `ScrubMachine<M>` (a `ToolMachine`, Effect
  `ToolYield<M>`, chart `idle → scrubbing`), M-independent tables + generic definition via an associated const (promoted to
  `'static`), `SCRUB_FINGERPRINT`/`SCRUB_MANIFEST_JSON` pinned against the `statechart!` compilation of the same chart,
  `ScrubHost`, `ScrubState<M>`, `Scrub<M>` (start/resume/send/persist; another press = host abort `captureLost` first),
  `ScrubLedger<M>` (per window; closed-press memory; base or tool change reopens; abort/abort_all/retain_windows/provisional).
- TS twin (`TM/🟦️.ts`): `parseScrubPhase`, `scrubMachine<M>()`, `scrubEvent`, `ScrubHost`, `Scrub`, `ScrubLedger`.
- Schema (`TM/🧬️schema/🔣️.json`): `ScrubGesture`, `ScrubPhase`, `ScrubArgs`, `ScrubInput`, `ScrubOpen`, `ScrubStep`,
  `ScrubScenario`, `ScrubChart`, `ScrubLawFixture` (additive; formatting preserved).
- Fixture `TM/🧫️fixtures/🧫️scrub-law/🔣️.json` (phases, chart incl. fingerprint, 13 scenarios) authored by
  `🧪️w3-t2-controls-scrub-law-fixture.ts` (hand-written expectations; transaction ids minted INDEPENDENTLY with first-party TS
  BLAKE3 + hand LEB128).
- Since then W3-T2-TEXT added a Typing region and W3-T-FLOWCAD a NodeDrag region to the same crate (theirs).

### 2.3 Runtime glue (plugin runtime)
- New module `OS/🔌️plugin/🛠️tool-machine/🦀️.rs` (my `ScrubRuntime`; W3-T2-TEXT later generalized it to `ToolMachineRuntime`
  with typing runs — the scrub path is unchanged). PLG seams (unique-anchor edits):
  - `dispatch_action`: after the registry/classification check, `admit_tool_dispatch` reads `gesture/commit/abort`; an abort is
    settled by the runtime (zero trace, never reaches the app); a press while time travel freezes is dropped and refused
    `timeTravel.frozen`; otherwise the tag rides `ingress` into `dispatch_typed_command_inner`, which binds it to the operation id.
  - time-travel verbs first `freeze_tool_machines()` (`frozen`); windows missing from the roster retire their press (`retired`).
  - `publish_mounted_typed_operation_unit`: at the one point the completion becomes the publication, `settle_tool_operation`
    moves the emit's `artifact_mutations` into the window's press (base = the operation's canonical revision). A tick or empty
    press publishes no artifact edit and logs no row (`command_logged`); the release publishes the committed batch as ONE edit
    with `emit.transaction = Some(ref)`, `coalesce_key`/`description` cleared. Other lanes publish as usual (ui_scope kept).
  - Render seams (render, window engagements, window measures, tool measures): `render_snapshot_or(tool_runs, overlay_or(..))` —
    committed ⊕ provisional leaves, refolded on every press change and on store generation change (`refresh_cache`); displaced
    aliases are retired through `store.retire_snapshot_alias` (never a plain drop).
- `VcsArtifactApp.tool_machines` field + constructor; `semio-framework-tool-machine` dependency of `semio-framework-plugin`.
- Laws `OS/🔌️plugin/🧪️tests/🧪️scrub/🦀️.rs`: overlay fold / untouched committed / refold-only-on-change / displaced aliases /
  bounded operation tags; plus the F-4 lint law.

### 2.4 Hosts
- Lane (`FW/🖱️ui/🎬️scene/🟦️.ts`): `ContinuousGestureLane.abort(reason)` + `open()`; port `abort?(reason)`; cancel is sent after
  the round trip in flight and before later offers; the cancelled press's release is ignored; a value-less `commit()` releases
  only an open press.
- React Interpreter (`RE/🗣️Interpreter/🟦️.tsx`): `useContinuousTriggerLane` sends `{gesture, abort}` and aborts `retired` on
  unmount; `SliderView` aborts on pointer cancel (`captureLost`) and blur (`blur`); `NumberStepperView` absolute changes ride
  the lane (button release / blur = release); declarative `renderUiControl` slider and stepper now use the same press protocol.
- wgpu (`FW/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`, `WidgetState.scrub_gesture` in `🌳️tree`): a slider press ticks on down/move
  and releases ONCE on pointer up wherever the pointer lets go; pointer cancel → `{gesture, abort:"captureLost"}`; a11y blur on
  a pressed slider → `blur`; keyboard steps, readout edits and stepper clicks/keys are one-shot presses; a number `Input` without
  a commit policy and a stepper's typed value tick per keystroke and release on blur/Enter; Escape cancels the typed press.
- Shared corpus `FW/🖱️ui/🧫️fixtures/🎛️retained-control-commit/🔣️.json`: rule `continuousPress` + per-case `press`
  (`released`/`open`); Rust law and TS twin check it.

### 2.5 F-4 input commit-mode lint
- `artifact_app_laws::document_input_commit_findings(projection, is_document_verb)` + `document_input_commit_lint(app, body, view)`
  (Mutation-kind verbs); fixture `OS/🔌️plugin/🧫️fixtures/🧫️input-commit-lint/🔣️.json`; law in `🧪️scrub`. Enforced in energy
  (`no_inspector_field_edits_the_document_per_keystroke`). CLOSURE can wire it into every app's tests.

### 2.6 Plugins
- gis terrain: `set_exaggeration` emits the absolute `change-exaggeration` (`Emit::mutations`); static key deleted.
- energy: inspector sliders/numbers unchanged (absolute `change-*`/`update-*` leaves from `model_edit`) and now scrub; name
  fields `Trigger::Commit` + blur.
- forms: all five verbs emit `Emit::mutations`; static keys deleted; inspector text-like fields commit on blur
  (`controls::input_row`, `row_on`), number fields scrub.
- norm: free-text `setField` inputs commit on blur (`bind_on`); number inputs scrub. No fixture touched.
- playbook: `updatePlaybook` emits one `change-title` per dispatch; amend deleted; describe text (en/de) and law updated.

## 3. Verification

See §3 table (filled below as each run completes).

## 4. Decisions and deviations

1. The scrub glue routes every dispatch carrying `gesture` (design §13.1). Late ticks of a closed press are silent; a moved
   document reopens the press on the new revision instead of dropping the rest of the drag (leaves are absolute).
2. The runtime captures `artifact_mutations` only; `child_emits` pass through (flow F6: W3-T-FLOWCAD extends the seam after §12).
3. Forms keeps `replace-block` (absolute, already history-editable). A field-parametric leaf needs a design decision (one
   generic `change-block-field{blockId, field, value}` vs per-field leaves; heterogeneous field types) — raised, not improvised.
4. `Ring` controls are not presses (React's `RingView` is not a continuous lane either).
5. Fail-closed mutation types (`OrderedMap`, flow) are dropped plainly inside `ToolTransaction`/`ScrubLedger` (as for every tool
   machine since W1-C); overlay snapshots are retired through the store.

## 5. Open items

- Descriptor regeneration: playbook (`updatePlaybook` describe text).
- Forms parametric leaf decision (§4.3). NodeGraph `setSlider` lane: W3-T2-PROCEDURAL moves gesture/commit to top level.
- Derived previews reading the overlay (F-7): W3-T2-PROCEDURAL adds `ArtifactOwnedToolJobContext::provisional()`.

## Session 2 — 2026-10-01

Successor S2-CONTROLS (coordinator `⚪552b484a…`). Ownership per the 12:xx coordinator message: `PL/🔋️energy/**`,
`PL/📋️forms/**`, `PL/📖️playbook/**`, `PL/🌍️gis/**`, `PL/📕️norm/📇️registry/🧬️contract/🖥️app-surface/**`,
`RE/🗣️Interpreter/🟦️.tsx` (+ tests). `FW/🖱️ui/**` is S2-W1E's and `OS/🔌️plugin/**` S2-W2A's: requests for them are in
§S2.6. Nothing committed; no ticket tool called. Scratch: `🗑️generated/s2-controls/`. Status: IN PROGRESS (this section is
rewritten in place as runs complete).
