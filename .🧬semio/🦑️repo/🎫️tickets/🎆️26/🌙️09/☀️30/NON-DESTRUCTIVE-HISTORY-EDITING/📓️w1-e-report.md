# 📓️ W1-E — UI contract: detents, precision, stepper, recipes, dialog choices

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, work package W1-E (see `🧭️plan.md`, `📋️design.md` §6–§7).
Status: **DONE and verified**. All five deliverables are implemented in the Rust contract (source of record), the
generated TS contract, React and wgpu. Each one has a language-agnostic fixture checked by Rust, TS and a
third-party oracle.

Aliases: `UI` = `🧰️framework/🔨️modules/🖱️ui`, `C` = `UI/🧬️contract`, `RE` =
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, `SHELL` = `RE/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`.

## 1. What was built

### 1.1 Contract (Rust source of record, `C/🧩️component`, `C/🛡️limits`, `C/🔢️number-format`)

- `SliderProps.snaps: UiFixedList<f64>` holds the detents. Snaps are strictly ascending, finite, lie within `min..=max`, and
  there are at most 32. The field is omitted on the wire when empty.
  - Validation: `slider_snaps_are_valid` in `C/🛡️limits` (TS twin `sliderSnapsAreValid` in `C/🛡️limits/🟦️.ts`).
    Both validators (`validate_core` and the streaming patch validation) refuse bad snaps with the new violation
    `UiContractViolation::InvalidSliderSnaps { node }`. A NaN snap is refused as `NonFiniteNumber` instead.
  - Shared laws in `C/🧩️component/🦀️.rs`, with the TS twin in the new `C/🧩️component/🟦️.ts`:
    - `slider_pointer_value`: clamp, then step up the ladder from `min`, then pull onto the nearest snap within
      `SLIDER_SNAP_RADIUS` (3 % of the span). The first snap wins a tie.
    - `slider_adjacent_snap`: PageUp/PageDown jumps to the next detent.
    - Keyboard arrows never snap, so a detent can never trap the arrow keys.
- `InputProps.precision` and `NumberStepperProps.precision` (`Option<u16>`, capped at `UI_NUMBER_PRECISION_MAX` = 15).
  - New number laws: `format_ui_number_fixed` (twin of JS `toFixed`: rounds ties away from zero, never prints a signed
    zero, falls back to the 12-digit format at ≥1e21) and `round_ui_number`. TS twins are `formatUiNumberFixed` and
    `roundUiNumber`.
  - The spoken value (`accessibility_value` / `uiAccessibilityValueV1`) follows the precision.
- Typed field catalog (`C/🧾️typed`), copy/compare/retirement pick up the new fields automatically.
- TS projection (`C/🧬️schema/🦀️.rs`) was updated and bumped: SliderProps v2, InputProps v2, NumberStepperProps v2,
  UiContractViolation v3. `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` was regenerated, and the `typegen_export`
  test confirms it is up to date.

### 1.2 Builders and recipes (`C/🏗️builder/🦀️.rs`)

- `SliderBuilder::try_snap` refuses any snap that would break the detent law.
- `InputBuilder::precision` and `InputBuilder::number(f64)` (prints the value at the field's precision).
- New `number_stepper(value)` with `.step()`, `.min()`, `.max()`, `.mixed()` and `.precision()`.
- Recipe `vector_input(label)`:
  - Builds a labelled `Group` laid out as a wrapping row of `Field` axes.
  - Each axis holds one keyed number `Input`; all axes share unit (shown as the field description), step, bounds,
    precision and commit.
  - `.try_axis(key, label, value, bind)`, where `bind` attaches that axis's own bindings.
  - Costs `1 + 2 × axes` records.
- Recipe `reference_list(label, ReferenceListLabels { use_selection, empty })`:
  - A toolbar of chip buttons. Each chip reads the item, carries an `x` icon, is announced as `remove_label`, and
    activating it removes the reference.
  - An empty line when there are no chips.
  - An actions row with the "use current selection" button (`try_use_selection(action, args, enabled)`) and an
    optional candidate `Select` (`try_candidates`).
  - Labels come from the caller, already localized, because the contract carries no locale.

### 1.3 `DialogDefinition.choices` (`🛂️manifest/🦀️.rs`, `🔖️Dialog` region only)

- `DialogChoice { id, label: LocalizedLabel, description: Option<LocalizedLabel>, action: ActionRef, tone: ui_contract::Tone, destructive: bool }`.
  - Builder: `DialogChoice::new(..).description(..).tone(..).destructive()` and `DialogDefinition::choice(..)`.
- **Semantics:**
  - Choices are extra decisions offered next to the submit button. Tab order is: fields → Cancel → choices in
    declared order → submit.
  - A choice sends its own `action` with the merged staged args plus `choice: <id>`
    (`DialogDefinition::choice_args`; TS twin `dialogChoiceArgs`; `DIALOG_CHOICE_ARG = "choice"`).
  - The submit sends its action with the merged args only. Cancel and Escape send the cancel action.
- `validate_choices()` requires non-empty, unique choice ids, and refuses a staged arg named `choice`. It is wired into
  plugin assembly (`🔌️plugin/🦀️.rs` dialog validation block), which also checks that each choice action is declared.
- TS projection (`🧬️schema/📽️projection`): `DialogChoice` v1, `DialogDefinition` v2, and `Tone` imported from
  ui-contract. The generated manifest TS is fresh (framework `exports_typescript_bindings` passes).

### 1.4 Renderers

**React**

- `🎚️Slider`:
  - A pointer value goes through the shared detent law.
  - An off-step detent keeps its exact value.
  - PageUp/PageDown jump to the next detent.
  - One `data-slot="slider-tick"` (aria-hidden) is painted per snap.
- `🪜️Stepper` gets a `precision` prop (values are rounded and formatted with it).
- Interpreter:
  - `SliderView` and the declarative slider pass the snaps through.
  - `InputView` rounds the committed number and derives `step` from the precision.
  - `NumberStepperView` passes the precision through.
  - `ButtonView` now wires `accessibility.description` to `aria-describedby`. This was a gap before this change.
- `📨️UIDialog` renders the choices as buttons in that order:
  - Each choice with a description gets a visible description paragraph, referenced by `aria-describedby`.
  - A destructive choice gets danger styling plus `data-destructive` and `data-tone`.
  - Choices are disabled while any arg is unresolved. Escape still cancels.
  - New required prop `onChoose(choice, args)`.
- `OwnedShellDialog` dispatches a choice through `dialogChoiceArgs`.
- `resolveDialogDefinition` resolves choice labels and descriptions through the app label overlay.
- `UiDocumentStore` and the retained graph validation refuse invalid snaps. The retained typed wire decoder handles
  `snaps` and `precision`.

**wgpu**

- `UiSliderNode.snaps`:
  - The shared `slider_tick_rects` (in `🎚️Slider/🎯️targets/🧊️wgpu`) paints the ticks in the retained painter and in
    `render_slider`, mirrored for right-to-left layouts.
  - `slider_value_at` applies the detent law.
  - `constrain_slider_value` keeps detents exactly.
  - PageUp/PageDown jump between detents.
- `UiInputNode.precision` and `UiNumberStepperNode.precision`:
  - Reconcile derives `step` from the precision.
  - Commits are rounded (`constrain_number_field`, `constrain_stepper_value`).
  - `stepper_value_text` formats the value in the painter, events and accessibility.
- `ChromeDialogRequest` was rewritten in `SHELL` (dialog region):
  - Built by `from_definition(controller, &DialogDefinition, seed, terminology, locale)`.
  - Staged fields: text, number, select, toggle.
  - Driven by a flat paint program (`ChromeDialogPaintOp`), following the agent-approvals pattern.
  - Every button dispatches the effective args (`effective_action_args` / `unresolved_action_args`), and a choice adds
    its id. The submit and choices are gated while args are unresolved. A destructive choice is painted in the error
    colour.
  - The keyboard is owned by `handle_chrome_dialog_key`:
    - Tab walks the fixture order.
    - Typing and Backspace edit text and number fields.
    - Space flips a toggle or advances a select; arrows move a select or step a number.
    - Enter takes the focused button or submits from a field.
    - Escape cancels.
  - A veil click dispatches the cancel action. Before, `cancel_action` was ignored everywhere.
  - Choice descriptions reach the chrome accessibility projection through a new `ChromeControlPresentation.description`.
  - The literal English `"Cancel"` default was replaced by `LocalizedLabel::native("Cancel","Abbrechen")`.
  - On/Off/Choose… are localized the same way.

### 1.5 Conformance corpus (`C/🧫️fixtures/🧪️conformance`, now 69 cases)

New cases: `🧩️component/🧲️slider-with-snaps`, `🧩️component/🎯️stepper-precision`, `🖥️composite/🧭️vector-input`,
`🖥️composite/🧷️reference-list`, `🖥️composite/🔀️dialog-choices`. They are generated by the input script
`w1e-conformance-cases.py` in this ticket folder and registered in `📇️catalog.json`.

| Suite | How it covers the new cases |
|---|---|
| Rust `🔬️conformance-unit` | Walks the catalog, so the cases are covered automatically; the count was bumped to 69. |
| TS `🔬️conformance-corpus` | Ajv checks the catalog; count bumped to 69. |
| React Interpreter corpus | Loads all 69 cases. The new test `renders the number-control and recipe cases with their own semantics` renders them and checks: ticks, stepper value "2.50", axis step/value, chip names, and the destructive choice's `aria-describedby`. |
| wgpu | New suite `UI/🧪️tests/🧪️conformance-corpus/🦀️.rs`, mounted in `🔀️reconcile`. All 47 shape-group accept cases mount; the new cases paint; one tick is painted per detent; precision reaches the nodes; chips and choices mount as buttons. |
| Builder test | `the_recipes_build_exactly_the_conformance_corpus_shapes`: the Rust recipes produce exactly the corpus tree shapes. |

### 1.6 Other fixtures

- `C/🧫️fixtures/🧫️number-controls/{🔣️.json,🧬️schema/🔣️.json}` covers detent validity, the pointer law, the page law,
  fixed-precision cases (including binary ties such as 0.125, 2.5, 1.005 and 2.675), spoken values, and document
  verdicts.
  - Rust: `number_controls_fixture_pins_…`.
  - TS: `C/🧪️tests/🧪️number-controls/🟦️.ts`, wired into the contract `test` script. Ajv2020 checks the fixture, and
    **decimal.js** is the independent rounding oracle (it rounds the exact binary expansion).
  - React Slider test replays the pointer and page rows through the real element.
- `🛂️manifest/🧫️fixtures/🧫️dialog-choices/{🔣️.json,🧬️schema/🔣️.json}` holds the finalize prompt: focus order,
  dispatches, invalid dialogs.
  - Rust: `🛂️manifest/🧪️tests/🧪️dialog-choices`.
  - React: UIDialog test in en and de (Ajv2020 plus the dom-accessibility-api description).
  - wgpu: layout/focus/dispatch test and keyboard test.
- `decimal.js@10.6.0` was added as a root devDependency (test oracle only); `bun.lock` updated by `bun install`.

## 2. Verification (all foreground, gated)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-ui-contract --all-features -- --test-threads=1` (private target) | lib 214 ✔, catalogue 12 ✔, typegen_export 1 ✔, others 2 ✔ |
| `cargo check -p semio-framework-ui-contract --target wasm32-wasip2` and `--target wasm32-unknown-unknown` | ✔ |
| `cargo test -p semio-framework-ui --features testkit --lib` | 756 ✔ (includes the 4 wgpu corpus tests) |
| `cargo check -p semio-framework-os-renderer-wgpu --tests` | ✔ |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- dialog` | 13 ✔ (includes the 2 new dialog-choices tests, the restored-dialog test, and the chrome text laws) |
| `cargo test -p semio-framework --lib -- dialog` | 6 ✔ (3 new dialog_choices tests) |
| `cargo test -p semio-framework --features typegen --lib exports_typescript_bindings` | ✔ (generated manifest TS is fresh) |
| `cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle -p semio-s-plugin-forms` | ✔ (these edited literals compile; also compiles the plugin SDK validation edit) |
| ui-react vitest `📨️UIDialog 🎚️Slider` | UIDialog 9/9 ✔, Slider 15/15 ✔ (includes the new detent test) |
| renderer-react long suite, Interpreter corpus | 70 corpus tests ✔ plus the new render test ✔ |
| renderer-react exhaustive `normalizes all native component variants` (typed wire) | ✔ |
| TS self-tests (`bun -e`) | number-controls 63 checks ✔, corpus 69 ✔, a11y twin 279 ✔ |
| `tsc --noEmit` for renderer-react, ui-react and framework | 0 new errors (only the older `layered-overview-geometry` test d3-ease/polygon-clipping errors) |
| `verify taxonomy report` | `🧪️conformance-corpus` clean. `🧫️number-controls` and `🧪️number-controls` have no findings. The 5 new corpus dirs share the older `directory-kind-unresolved` finding that all 56 corpus dirs already have. |

## 3. Notes for the next packages

- **W2-A / finalize prompt.** Declare the dialog like this:
  - `submit_action = historyEditCommit`, `submit_label` "New alternative"/"Neue Alternative", plus the `name` text arg
    with a localized default.
  - One `DialogChoice::new("overwrite", …, historyEditCommit).tone(Tone::Danger).destructive().description(…)`.
  - The handler reads `choice == "overwrite"` for overwrite; a missing `choice` means new alternative.
  - The fixture `🧫️dialog-choices` is exactly this dialog.
- **W2-A / panels.** Build input editors from `ActionArgControl` like this:
  - Slider or Dial → `slider(v).try_snap(..)`.
  - Stepper → `number_stepper`.
  - Number → `input(Number).precision(p).number(v)`.
  - Vector → `vector_input`.
  - Reference → `reference_list`, with labels from `LocalizedLabel::native` resolved per locale.
- **W2-C.** The wgpu dialog now renders staged args and choices, so key routing depends on `handle_keyboard` being
  reached. If a retained content field still holds focus when a dialog opens, the content router may take keys first.
  A dialog-open focus clear is worth adding when the shell is touched.

## 4. Peer and older findings (not mine, not fixed)

- W1-D (in flight while I was working):
  - The `semio-framework` manifest `ArgSchema` failed to build for a while (`kind` tag conflict) and was fixed later.
  - `semio-framework-os-kernel` failed on `PAYLOAD_SCHEMA` missing in the store mutations, then on one store error,
    and was fixed later.
  - `🛂️manifest/🟦️.ts` imports `./🔣️input-labels.json`, which the wgpu browser allowlist rejects. This fails
    renderer-react `🧩️package-integration` (2 tests).
- Renderer-react long suite failures that were already there before this change: window-measure-controls and World3d
  projection-pane (`dir` auto vs rtl), canvas-framing timeout, engine-contract text-editor compose/paste and portal
  z-tutorial, scene-pointer-cancellation source string, window-fault labels.
- Renderer wgpu accessibility tests fail depending on the order they run in ("presented input candidate could not be
  sealed"). They pass when run alone, and fail without my tests too.
- The contract lib tests need `--test-threads=1`, because the resident and retirement tests fight over shared global
  state when run in parallel.
- The exhaustive typed-wire suite has many timeouts on this loaded machine, and a stale path (`📦️packages/🦀️rust/🎬️action.rs`).
- Two older gaps in the typed wire fixture expectations (`treeItem` inlineToolbar/detail, table
  rowLabel/columnLabel/columnWindow) were in my scope; I fixed them.

## 5. Files

- **Created:**
  - `C/🧩️component/🟦️.ts`
  - `C/🧫️fixtures/🧫️number-controls/🔣️.json` and `🧬️schema/🔣️.json`
  - `C/🧪️tests/🧪️number-controls/🟦️.ts`
  - 5 corpus case dirs (10 JSON files)
  - `UI/🧪️tests/🧪️conformance-corpus/🦀️.rs`
  - `🛂️manifest/🧫️fixtures/🧫️dialog-choices/🔣️.json` and `🧬️schema/🔣️.json`
  - `🛂️manifest/🧪️tests/🧪️dialog-choices/🦀️.rs`
  - Ticket inputs: `w1e-conformance-cases.py`, `🗑️generated/w1-e/{add_field.py,dialog_struct.rs,dialog_render.rs}`
- **Modified, contract:**
  - `C/🧩️component/🦀️.rs`, `C/🛡️limits/{🦀️.rs,🟦️.ts}`, `C/🔢️number-format/{🦀️.rs,🟦️.ts}`
  - `C/♿️accessibility/{🦀️.rs,🟦️.ts}`, `C/🧾️typed/🦀️.rs`, `C/🏗️builder/🦀️.rs`, `C/🧬️schema/🦀️.rs`
  - `C/🧵️retained/📦️wire/🧾️typed/🟦️.ts`, `C/🧵️retained/📦️wire/{🧫️fixtures/🧾️typed,🧬️schema}/🔣️.json`
  - `C/🧵️retained/🛡️validation/🔬️graph/🟦️.ts`
  - `C/🧪️tests/{🔬️component-unit,🔬️builder-unit,🔬️conformance-unit}/🦀️.rs`, `C/🧪️tests/🔬️conformance-corpus/🟦️.ts`
  - `C/📦️packages/🦀️rust/📜️script.ts`, `C/🧫️fixtures/🧪️conformance/📇️catalog.json`
  - `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`
- **Modified, React elements:**
  - `UI/🧱️elements/🎚️Slider/{🟦️.tsx,🧪️tests/🧩️component/🟦️.tsx}`
  - `UI/🧱️elements/🪜️Stepper/🟦️.tsx`
  - `UI/🧱️elements/📨️UIDialog/{🟦️.tsx,🧪️tests/🧩️component/🟦️.tsx,📖️stories/🧪️.story.tsx}`
- **Modified, wgpu:**
  - `UI/🧱️elements/🎚️Slider/🎯️targets/🧊️wgpu/🦀️.rs`, `UI/🧱️elements/🪜️Stepper/🎯️targets/🧊️wgpu/🦀️.rs`
  - `UI/🎯️targets/🧊️wgpu/{🧩️component,🔀️reconcile,⚡️events,🖌️paint,🧮️layout,🪀️widgets,♿️accessibility}/🦀️.rs`
  - UI wgpu tests (literal fields): value-round-trip, events-unit, paint-unit, engine-unit, ui-node-wire-format,
    cursor-unit, retained-control-commit
- **Modified, manifest:** `🛂️manifest/{🦀️.rs (🔖️Dialog region),🟦️.ts}`, `🧬️schema/📽️projection/🦀️.rs` (Dialog
  entries and header import)
- **Modified, renderer:**
  - `RE/🗣️Interpreter/{🟦️.tsx,📖️stories/🧪️.story.tsx,🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx}`
  - `RE/📃️UiDocumentStore/🟦️.tsx`, `RE/🛠️ShellHelpers/🟦️.tsx` (resolveDialogDefinition)
  - `RE/🏛️ShellHost/🗨️dialog-origin/🌐️browser/🟦️.tsx`
  - `SHELL` (dialog region, keyboard hook, ChromeControlPresentation description, props literals)
  - Shell tests: overlays-tour, tour-overlay, appearance-tour, theme-editor
  - `RE/../🧪️tests/🔬️engine-contract/🟦️.ts`, HubConnection wgpu, playbook
- **Literal-only field additions:**
  - `🔌️plugin/🦀️.rs` (choice validation) and `🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs`
  - Puzzle 2d settings and inspection, puzzle 3d/5d settings, forms `▶️try`
- **Root:** `package.json`, `bun.lock` (decimal.js)

## 6. Follow-up (audit `📓️audit-modules-ui.md` and coordinator requests)

Status: **DONE and verified**. This section covers:
- the audit findings on W1-E's work (M-4, M-5 and the minors);
- the coordinator's later request for `color_input` and vector facets;
- the compile fix for the consumers of the contract change.

Earlier sections describe the first round. Where this section disagrees with them, this section wins: two names
changed (see 6.3) and the dialog fixture changed shape (see 6.1).

### 6.1 M-4: a choice is gated only on the args it requires

- `DialogChoice.requires: Vec<String>` names the staged args a choice consumes. It defaults to none and is left
  off the wire when empty. Builder: `DialogChoice::requires(ids)`.
  - `DialogChoice::unresolved_args(defs, effective)` gates the choice.
  - `DialogChoice::dispatch_args(defs, effective)` builds what the choice sends: the seed context (keys that no arg
    declares), the args it requires, and `choice`.
  - `DialogDefinition::choice_args` is removed.
  - `validate_choices` also refuses a choice that requires an undeclared arg ("choice {id} requires undeclared arg
    {arg}").
- The submit is still gated on the dialog's own required args. Clearing `name` therefore disables only "New
  alternative"; "Overwrite" stays enabled and sends `{generation, choice}`.
- TS twins:
  - `dialogChoiceArgs(choice, defs, effective)` in `🛂️manifest/🟦️.ts`;
  - `unresolvedDialogChoiceArgs(choice, defs, effective)` in `🧩️action-argument-resolution/🟦️.ts`.
- Projection `DialogChoice` v2 and `DialogDefinition` v3. The generated manifest TS is regenerated.
- Fixture `🛂️manifest/🧫️fixtures/🧫️dialog-choices` now has:
  - a `seed` (`{generation: 7}`);
  - `cases`: `named` and `cleared`, each listing its `staged` values, the `enabled` and `disabled` buttons, and its
    `dispatches`;
  - a new invalid row, `requires-undeclared-arg`.
- The three suites answer it:
  - **Rust** (`🧪️dialog-choices`) adds a law for a choice that does require an arg.
  - **React:** `UIDialog` gates each choice by itself. The test runs every case in en and de. `OwnedShellDialog`
    dispatches the args `UIDialog` hands it.
  - **wgpu:** `ChromeDialogRequest::action` gates and dispatches through the same functions (W2-C had already switched
    `action()`). The fixture test checks the painted `event` and `disabled` flags per case. The keyboard test
    clears the name, finds the submit ignoring Enter, and takes Overwrite.
- W2-A's `history_edit_finalize_dialog` needs no change: its Overwrite requires nothing. Cancel still sends no args,
  and W2-A's `historyEditBack` falls back to the live session generation, so that is fine.

### 6.2 M-5: the wgpu declared dialog is an accessibility tree

- **The tree:** `dialog_accessibility_nodes` publishes the open dialog as a modal tree, the way React's `UIDialog`
  does, and `chrome_accessibility_nodes` uses it first. Nothing behind the veil is published.
  - The root node has role `dialog`, key `shell.dialog.<id>`, the title as its name and the body as its description.
  - Every painted control sits one level below, in paint order, with its name, description, role, pressed state,
    value and range.
  - A gated submit or choice is `disabled` and not actionable.
  - Focus stops are tabbable.
  - The dialog's own keyboard focus is the one `focused` node.
- **The paint op:** `ChromeDialogPaintOp::Hit` carries `disabled`. One helper, `note_chrome_dialog_control`, records
  name, description, semantics and disabled for both render paths. The test path used to record nothing.
- **Names and census:**
  - The `*` on a required field is only drawn, never part of its name.
  - The census reports the dialog at level `dialog`.
- **Operable tree:** `handle_chrome_dialog_accessibility_event`, called from `handle_accessibility_event`:
  - focus moves the dialog's focus stop;
  - activation presses the control (a gated button does nothing; an enabled one dispatches and closes);
  - a value edits a text, number, axis or colour field, or sets a slider.
- **Test isolation:** under `cfg(test)` the chrome name registry is thread-local, as it already is on wasm. Before,
  a concurrent test's frame setup could clear the names another test was about to publish. That is where the flaky
  `humanize` labels came from.
- **Tests (`🔬️wgpu-chrome-overlays-tour`):**
  - `the_open_dialog_publishes_a_modal_accessibility_tree_with_gated_and_focused_controls` (en and de);
  - `the_dialog_accessibility_tree_is_operable_like_its_keyboard`.

### 6.3 Minors

- **N-14 (`format_ui_number_fixed` ties at 2^50 and above):**
  - At an exact binary half, the value is now printed with one more digit and incremented as a decimal string
    (`decimal_increment`), so ties go away from zero exactly as JS `toFixed` does.
  - New `🧫️number-controls` `fixed` rows: 1125899906842624.25 (p1), its negative, 2251799813685248.5 (p0), and an
    odd tie. Checked by Rust, the TS twin and decimal.js.
- **N-15 (React slider ignored detents within a quarter step):**
  - `sliderValuesMatch` and `resolveSliderDraftClear` take the snaps, and a detent now matches only itself.
  - New fixture pointer row `detent-within-quarter-step`, plus a React test with a pointer landing on 5.1 next to 5.
- **N-16 (one key rule, pinned by a fixture):**
  - The contract now has `SliderKey`, `SLIDER_PAGE_STEPS`, `ui_number_key_value` (optional bounds) and
    `slider_key_value` (both bounds), with TS twins `sliderKeyValue` and `uiNumberKeyValue`.
  - Arrows walk the step ladder from `min` (from 0 without one). They never snap, and from an off-ladder value the
    first rung beyond it counts as one step. Shift walks 10 rungs.
  - Page keys go to the adjacent detent, or 10 rungs when there is none. Home and End go to the bounds.
  - An invalid step walks rungs of 1. Results are clamped and cleaned to the decimals of the ladder.
  - Fixture `keys` has 22 rows, 6 of them for unbounded number fields. Checked by Rust, TS (decimal.js ladder
    oracle), React `Slider` (physical keys, bounded rows), the wgpu retained slider (bounded rows) and W2-C's
    dialog-slider law.
  - The wgpu dialog `slide` passes Shift through.
  - The old tenth-of-travel fallback in the dialog and React's current + 10 × step fallback are both gone.
- **N-17:** the retained painter drew RTL ticks mirrored while the thumb stays LTR. The rtl parameter is removed from
  `slider_tick_rects`. The corpus test now asserts that each tick sits under the thumb resting on its detent, in LTR
  and RTL.
- **N-24:** the duplicate docstring emojis I introduced are made unique in component, limits, builder, Slider, and
  the shell and test additions. Duplicates that were already there are untouched.
- **Renames** (every call site, fixture and generated file updated):
  - `slider_snaps_are_valid` → `snaps_are_valid` (TS `snapsAreValid`);
  - `UiContractViolation::InvalidSliderSnaps` → `InvalidSnaps` (wire `invalidSnaps`, projection v4).

### 6.4 Coordinator request: `color_input` recipe and vector facets

- **Contract:**
  - `InputProps.snaps` holds the detents of a number field. It follows the detent law against optional bounds and is
    validated as `InvalidSnaps`; its page keys jump between the detents and typing never snaps. It has TS
    projection v3, typed catalog field 9 and typed-wire decoding, and is checked in the graph and store validators.
  - Colour law: `ui_color_hex(rgba, alpha)` and `parse_ui_color_hex(text)`, with TS twins `uiColorHex` and
    `parseUiColorHex`.
- **Builders:**
  - `color_input(label, ColorInputLabels { hex, alpha }).try_color(rgba, alpha, bind_hex, bind_alpha)` builds a
    Group holding:
    - a swatch (`InputKind::Color`, `#rrggbb`);
    - a hex field (text, `#rrggbb` or `#rrggbbaa`, committed on blur);
    - with alpha, an alpha slider over `0..=1` in steps of `0.01`.
  - `VectorInputBuilder::try_snap` gives every axis the shared snaps; `InputBuilder::try_snap` adds a snap to one field.
- **Fixtures and tests:**
  - New `C/🧫️fixtures/🧫️color-input`: 7 hex rows and 8 parse rows. Checked by Rust and the TS twin, with
    `color-string@1.9.1` as the oracle. It is added to root devDependencies and wired into the contract `test` script.
  - The corpus gets a new `🖥️composite/🎨️color-input` case, and `vector-input` axes now carry `snaps: [0]`. The corpus
    has 70 cases. The builder test compares every recipe component with the corpus snapshot.
- **React:**
  - The Interpreter's number fields handle page keys through the shared law. The corpus render test covers the
    colour case and vector page keys.
  - `ShellHelpers` replaces W1-D's interim `StagedColorField` with the recipe: a swatch named by the field, a hex field
    that stages on Enter or blur and keeps the alpha when the text has none, and an Opacity/Deckkraft `Slider`.
  - The new `StagedVectorField` honours the vector's bounds, precision (which also sets the step), display unit and
    factor, and detents on the page keys.
  - The labels `ui.colorInput.{hex,alpha}` are added in en and de, in both bundles and the `📚️I18n` schema.
- **wgpu retained (`UI/🎯️targets/🧊️wgpu`):**
  - `UiInputNode.snaps`, mapped by reconcile.
  - A number field's edit buffer handles PageUp and PageDown through the shared law, and a committed detent is kept
    off the ladder.
  - A `color` input paints a swatch of the colour its text parses to (`paint::color_input_swatch`), with the hex beside
    it, in both paint paths.
- **wgpu shell:**
  - `staged_arg_row` renders `Color` as swatch, hex and opacity rows instead of plain text. `staged_color_value`
    applies the colour law. Vector axes carry the vector's bounds, step, precision and snaps.
  - In the declared dialog, a `Color` field paints a swatch beside its hex draft, and an opacity field is a slider
    over the fourth component.
  - The colour is derived in `effective()` from the edited draft. This means a hex passing through a short form while
    it is being typed never loses the opacity, and an untouched colour dispatches its exact components.
  - Vector `Axis` fields carry bounds, precision, snaps and a precision step. Arrows and page keys go through
    `ui_number_key_value`, and a typed value is clamped.

### 6.5 Compile fix

`InputProps.snaps` and `UiInputNode.snaps` were added to every literal in one compile-atomic pass (a repo-wide scan
finds none missing). `cargo check -p semio-framework-ui -p semio-framework-os-renderer-wgpu -p semio-framework-plugin
--tests` has 0 errors in ui and in the wgpu renderer. The only remaining errors are 3 in W2-A's plugin test
`🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs:459-461`: `time_travel_input_at` now takes 3 args. That is W2-A's in-flight
API, not mine. Main was told.

### 6.6 Verification (all run, gated, `target-nde-w1e`)

| Suite | Result |
|---|---|
| `cargo test -p semio-framework-ui-contract --tests -- --test-threads=1` | lib 215 ✔, catalogue-carrier-map 12 ✔ |
| contract typegen (`--features typegen --test typegen_export`), regenerated then re-run fresh | 1 ✔ |
| `cargo test -p semio-framework --features typegen --lib` (dialog, history_edit, typegen) | 13 ✔ (manifest TS regenerated) |
| `cargo test -p semio-framework-ui --features testkit --lib` | 760 ✔ |
| `cargo test -p semio-framework-os-renderer-wgpu --lib`: `chrome_overlays_tour` / `time_travel` (W2-C laws) / `dialog` / `staged` | 52 / 16 / 19 / 9 ✔ |
| TS twins (`bun -e`): number-controls / color-input / corpus / a11y | 126 / 31 / 70 / 279 ✔ |
| ui-react: Slider / UIDialog / translation-totality | 17 / 9 / 1 ✔ |
| renderer-react: staged-arg-controls / Interpreter corpus and recipes | 11 / 72 ✔ |
| renderer-react UiDocumentStore (exhaustive), filtered to typed/snap/violation | 9 ✔ |
| typecheck: ui-react / renderer-react / framework TS | 0 errors each |

### 6.7 Failures I did not cause

- **Full `shell::` run (605 passed, 100 failed):** the failures are the known global-state contention:
  - "retained presented input candidate could not be sealed" (58);
  - "resident capacity exhausted, roots 64/64" (10);
  - a few "Contended" readers.
  - I re-ran `presented_input_authority` tests one at a time and they pass. `accessib` fails the same way with
    `--test-threads=1`, because each test inherits the previous one's interpreter state.
- **renderer-react UiDocumentStore (exhaustive), 3 failures:**
  - `OwnedHash` and `OwnedIntake`: a surface node and the retained schema fixtures. None of these files was touched by
    me, and they have not changed for 2 weeks.
  - The stale `📦️packages/🦀️rust/🎬️action.rs` path, already reported in section 4.

### 6.8 Files (follow-up)

- **Created:**
  - `C/🧫️fixtures/🧫️color-input/{🔣️.json,🧬️schema/🔣️.json}`
  - `C/🧪️tests/🧪️color-input/🟦️.ts`
  - corpus case `C/🧫️fixtures/🧪️conformance/🖥️composite/🎨️color-input/{📸️snapshot.json,🎯️expect.json}`
- **Modified, contract:**
  - `C/🧩️component/{🦀️.rs,🟦️.ts}`, `C/🛡️limits/{🦀️.rs,🟦️.ts}`, `C/🔢️number-format/{🦀️.rs,🟦️.ts}`
  - `C/🏗️builder/🦀️.rs`, `C/🧾️typed/🦀️.rs`, `C/🧬️schema/🦀️.rs`
  - `C/🧵️retained/📦️wire/🧾️typed/🟦️.ts`, `C/🧵️retained/📦️wire/🧫️fixtures/🧾️typed/🔣️.json`
  - `C/🧵️retained/🛡️validation/🔬️graph/🟦️.ts`
  - `C/🧫️fixtures/🧫️number-controls/{🔣️.json,🧬️schema/🔣️.json}`, `C/🧫️fixtures/🧪️conformance/📇️catalog.json`,
    corpus `🧭️vector-input`
  - `C/🧪️tests/{🔬️component-unit,🔬️builder-unit,🔬️conformance-unit}/🦀️.rs`
  - `C/🧪️tests/{🧪️number-controls,🔬️conformance-corpus}/🟦️.ts`
  - `C/📦️packages/🦀️rust/📜️script.ts`
  - `🛂️manifest/🤖️generated/{📜️ui-contract,🪪️manifest}/🟦️.ts`
- **Modified, manifest and framework TS:**
  - `🛂️manifest/🦀️.rs` (🔖️Dialog region only), `🛂️manifest/🟦️.ts`
  - `🛂️manifest/🧫️fixtures/🧫️dialog-choices/{🔣️.json,🧬️schema/🔣️.json}`, `🛂️manifest/🧪️tests/🧪️dialog-choices/🦀️.rs`
  - `🧩️action-argument-resolution/🟦️.ts`, `🧬️schema/📽️projection/🦀️.rs`
- **Modified, React:**
  - `UI/🧱️elements/🎚️Slider/{🟦️.tsx,🧪️tests/🧩️component/🟦️.tsx}`
  - `UI/🧱️elements/📨️UIDialog/{🟦️.tsx,🧪️tests/🧩️component/🟦️.tsx}`
  - `UI/🧱️elements/📚️I18n/🟦️.tsx`, `UI/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`
  - `RE/🗣️Interpreter/{🟦️.tsx,📖️stories/🧪️.story.tsx,🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx}`
  - `RE/🛠️ShellHelpers/{🟦️.tsx,🧪️tests/🧪️staged-arg-controls/🟦️.tsx}`, `RE/📃️UiDocumentStore/🟦️.tsx`
  - `RE/🏛️ShellHost/🗨️dialog-origin/🌐️browser/🟦️.tsx`
- **Modified, wgpu:**
  - `UI/🎯️targets/🧊️wgpu/{🧩️component,🔀️reconcile,⚡️events,🖌️paint}/🦀️.rs`
  - `UI/🧱️elements/🎚️Slider/🎯️targets/🧊️wgpu/🦀️.rs`
  - `UI/🧪️tests/{🧪️conformance-corpus,🔬️targets-wgpu-events-unit}/🦀️.rs`, plus the `UiInputNode` literal tests
    (retained-control-commit, ui-node-wire-format, engine-unit, paint-unit, cursor-unit, value-round-trip)
  - `SHELL`, with a tiny off-region part:
    - in the dialog region: the dialog model, paint, keys, accessibility tree and events;
    - outside it: the `handle_accessibility_event` hook, the census row, the name registry cfg, `staged_arg_row` and
      `staged_arg_value`/`staged_color_value`, and the `InputProps` literals.
  - `RE/🐚️Shell/🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs`
  - `UiInputNode` literals in HubConnection wgpu and playbook
- **Root:** `package.json`, `bun.lock` (color-string 1.9.1)
- **Ticket inputs:** `w1e-conformance-cases.py` (color case, sorted catalog insertion)

## 7. Follow-up 2: tree rows render recipe content in wgpu

Status: **DONE and verified**.

**The problem:** in wgpu, reconcile turned only the first of the nine inline controls under a `TreeItem` into the row's
`control`. Every other child became a placeholder row: a group recipe, text, progress, or a second control.
`mounted_layout` matched that placeholder by key and painted it as a tree row labelled with the node key. So the
time-travel editor's colour, vector and reference inputs could not be used in wgpu, while React (which renders every
non-row child in the row's property value column) showed them correctly.

**The fix** (one rule, deterministic before layout):
- **Reconcile** (`UI/🎯️targets/🧊️wgpu/🔀️reconcile`):
  - `tree_row_content` lists a row's content: every child that is not a nested row, its toolbar or its detail.
  - When that content is exactly one of the nine inline controls, it stays the row's `control`, as before.
  - Otherwise `UiTreeItemNode.content_lines` (new) counts the lines the content stacks to:
    - one per leaf (control, button, text, progress);
    - one per Field label, description and error;
    - one per Section title;
    - other containers count only their children.
  - Nested items are only real `TreeItem` rows now, so no placeholder row is ever made.
- **Metrics** (`🧮️layout`): `TreeRowMetrics::for_item` grows a content row to `tree_content_height(lines)`, one
  standard control per line plus one gap between lines. Paint, hit-testing and layout all read this one height.
- **Layout** (`📌️mounted_layout` and `📐️flex`):
  - A direct content child gets `TreeContentFlow::Slot`: its box in the value column, below the content before it,
    vertically centred in the row.
  - Content containers become `Column`s whose label bands take one line each.
  - Leaves become fixed `Line`s.
  - Content Fields and Sections do not take part in chrome text measurement, so their bands stay deterministic and the
    lines/runs invariant holds. Without this the layout faulted as `layout.stale` and never settled.
- **Paint:**
  - Rows with content reserve the value column for their label.
  - A Group inside a tree paints no chevron or label of its own; the row label names it and its accessibility label
    stays.
- **React:** `SliderView` now passes the record's accessibility label as `aria-label`. Before, a React slider had no
  accessible name of its own.
- **Budget:** the fixed-slot budget `🧱️boxed-fixed-slots` records `UiSurfaceRegistry` at 165872 B (+32 B), from the
  layout node's new content field.

**Corpus:** new case `🖥️composite/🌲️tree-row-recipes`: a tree section whose rows hold a colour input, a vector input
(Fields with unit descriptions and detents), a reference list, and a text with a progress bar. The corpus now has 71
cases.
- **Rust contract:** `conformance-unit` validates the case.
- **TS:** the corpus self-test covers it (71).
- **React** (Interpreter corpus test):
  - every control is named: Tint, Hex, Alpha, X, Y, "Remove Piece 3", "Remove Piece 7", "Use selection", "Add target",
    "Replay progress";
  - the swatch value and the progress `aria-valuenow` are checked;
  - each control sits inside its own `treeitem`.
- **wgpu:**
  - `tree_rows_render_recipe_content_in_their_value_column` checks:
    - each row's `content_lines` (3/6/4/2), with no placeholder rows and no `control`;
    - every leaf laid out inside its row, in the value column, one below the other, and rows below each other;
    - `hit_test` at every leaf's centre finds that leaf;
    - Tab reaches every control.
  - The corpus accessibility law now also covers the case: slider, buttons and progressbar roles, plus every name and
    description.

**Verification (gated, all run):**

| Suite | Result |
|---|---|
| `semio-framework-ui --features testkit --lib` | 761 ✔ |
| `semio-framework-ui-contract --lib` | 215 ✔ |
| TS corpus self-test | 71 ✔ |
| renderer-react Interpreter file, full | 168 ✔ |
| renderer-react typecheck | 0 errors |
| renderer wgpu `time_travel` / `tree` | 17 / 29 ✔ |
| `cargo check -p semio-framework-ui -p semio-framework-os-renderer-wgpu -p semio-framework-plugin --tests` | 0 errors |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | ✔ |

The renderer `staged` filter could not be re-run at the end, because a peer's in-flight shell presence edit
(`PresenceActivity`, `PresencePeerRow.activity`) breaks the crate right now. It passed (9) earlier, and staged rows are
spec-built, with `content_lines` always `None`.

**Files:**
- **wgpu:**
  - `UI/🎯️targets/🧊️wgpu/{🧩️component,🔀️reconcile,🧮️layout,📐️flex,📌️mounted_layout,🖌️paint}/🦀️.rs`
  - `UI/🧪️tests/🧪️conformance-corpus/🦀️.rs`
  - `UI/🧪️tests/{🔬️targets-wgpu-flex-unit,🔬️targets-wgpu-engine-unit,🎯️retained-hit-targets,🔬️targets-wgpu-component-ui-value-round-trip,🔬️targets-wgpu-reconcile-unit}/🦀️.rs`
  - `UiTreeItemNode` literals in `🐚️Shell` wgpu (none left besides `..base`) and playbook
- **Contract:**
  - corpus case `C/🧫️fixtures/🧪️conformance/🖥️composite/🌲️tree-row-recipes/{📸️snapshot.json,🎯️expect.json}`
  - `📇️catalog.json`
  - count updates in `C/🧪️tests/🔬️conformance-unit/🦀️.rs` and `C/🧪️tests/🔬️conformance-corpus/🟦️.ts`
- **React:** `RE/🗣️Interpreter/{🟦️.tsx,🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx}`
- **Budget:** `🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`
- **Ticket input:** `w1e-conformance-cases.py` (tree-row case)

## Session 2 — 2026-10-01 (G6)

Successor of W1-E (session 2, coordinator `⚪552b484a…`), gap **G6**: input-metadata rendering completeness. Repair-first check: no
half-finished W1-E edit was found (the last W1-E work, follow-up 2, closed at 18:45 on 09-30; the only later diffs in `UI/**` are a peer's
wgpu action-value refactor, left alone).

Status: **IN PROGRESS** (this section is updated at every milestone).

### S2.1 Contract (Rust source of record, `C/🧩️component`, `C/🛡️limits`, `C/♿️accessibility`) — DONE, tested

- New types:
  - `UiNumberScale { Linear (default), Log }`
  - `SliderAppearance { Track (default), Dial }`
  - `UiNumberBound { value, exclusive, refusal: Option<Label> }`
  - `UiNumberLimits { min?, max? }`
  - The refusal label is localized by the producer and names the bound in display units, because the contract carries no locale.
- `SliderProps` gains `appearance`, `scale`, `precision`, `display_unit`, `display_factor` and `limits`.
  - `min`/`max` are the travel.
  - Without `limits`, the travel is the hard range.
  - With `limits`, the travel is a soft range: a typed value may leave it while the limits admit it.
- `NumberStepperProps` gains `snaps` (detents), `unit`, `display_unit`, `display_factor` and `limits`.
- `InputProps` gains `display_factor` and `limits`. A number field's `value` stays the stored number.
- All three props derive `Default`. External literals use `..Default::default()` (coordinator request after the 13:10 compile break).
- Laws (Rust, each with a TS twin in `C/🧩️component/🟦️.ts`):
  - `slider_axis_position` / `slider_axis_value`: linear or log axis, `min`/`max` exact at the ends.
  - `dial_angle` / `dial_position`: one full counter-clockwise turn, the travel's centre at 3 o'clock, the seam at 9 o'clock. A
    `-180°..180°` angle points where it turns.
  - `slider_pointer_value(value, min, max, step, snaps, scale)`: the snap radius is measured on the axis (the log axis for `log`), and
    stepped values are cleaned to the ladder decimals.
  - `ui_number_key_value`:
    - Arrows move one rung.
    - Shift and PageUp/PageDown move ten rungs. A page key stops on the first detent it reaches.
    - Any key that lands within ladder tolerance of a detent lands on it exactly. Example: arrows from 89° land on π/2 itself.
    - Arrows still never stop on an off-path detent.
  - `ui_number_display` / `ui_number_display_text`: shown value = stored × factor, at the given precision or 12 digits.
  - `ui_number_typed_value`: a candidate whose display text equals the typed text keeps its exact stored value (typing `90` gives π/2),
    otherwise the value is rounded and divided back.
  - `ui_number_crossed_bound`: returns the crossed bound, lower first. An excluded limit refuses itself. Without declared limits the
    bounds are the inclusive `min`/`max`.
- Validation:
  - New `UiContractViolation::InvalidNumberRange`, checked in `validate_core` and the streaming validator, with TS twins in the
    retained graph validator and `UiDocumentStore`.
  - The range is invalid when:
    - the display factor is not positive;
    - a log travel is not strictly positive;
    - the limits are inverted;
    - the limits do not admit `min`/`max` and every detent.
  - Stepper snaps obey the detent law (`InvalidSnaps`).
  - New limit values and display factors count in `NonFiniteNumber`.
- Accessibility value law (Rust and TS twin):
  - Range controls announce min/max/now in display units (cleaned to 12 digits).
  - The value text is the display text plus the shown unit (`display_unit`, else `unit`). For example, the dial announces `90 °`.
- Builders (`C/🏗️builder`):
  - `SliderBuilder::{appearance, scale, precision, display_unit, display_factor, limits}`.
  - `NumberStepperBuilder::{try_snap, unit, display_unit, display_factor, limits}`.
  - `InputBuilder::{display_factor, limits}`. `number()` prints the exact stored number once a factor is set.
  - `VectorInputBuilder::{display_factor, limits}`.
- Typed catalog, copy, compare and retirement cover the new fields and types. The typed retirement depth grows from 7 to 9 (a labelled
  bound nests two levels). The fixed path holds 16; the pin in `🌳️typed` tests was updated.
- TS projection: SliderProps v4, NumberStepperProps v3, InputProps v4, UiContractViolation v5, plus the new SliderAppearance,
  UiNumberBound, UiNumberLimits and UiNumberScale (91 types). The generated `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` was
  regenerated with `SEMIO_TYPEGEN_OUT`; `typegen_export` passes.
- Retained typed-wire TS decoder: new `numberLimits`/`numberBound`, and slider, stepper and input decode every new field. The fixture
  `🧾️typed` gains 2 rows (27); the schema count was bumped to match.

### S2.2 Language-agnostic tests — DONE (Rust + TS), the third-party oracle wired

- `C/🧫️fixtures/🧫️number-controls`:
  - New sections `axis` (10), `dial` (6), `display` (6), `typed` (6) and `limits` (7).
  - New rows: `pointer` (+6: log detent, axis-relative radius, dial right angle), `keys` (+8: one-degree arrow, settle onto π/2, page
    walks ten rungs before a far detent, page stops on a detent, log, stepper page detents), `valueTexts` (+4, including
    valueNow/Min/Max), `documents` (+9: dial, log, stepper detents, `invalidNumberRange` cases).
  - The schema was extended.
  - The expectations were written by the independent Python implementation `🧪️s2-w1e-number-controls.py` (ticket root, input
    script; `decimal` for exact-binary half-up).
- TS twin test `C/🧪️tests/🧪️number-controls/🟦️.ts`:
  - d3-scale `scaleLog`/`scaleLinear` are the third-party axis oracle (position and inverse) and the display-factor/read-back oracle.
  - decimal.js remains the ladder oracle. It now cleans to the ladder decimals and applies the ladder tolerance.
  - `d3-scale@4.0.2` and `@types/d3-scale@4.0.9` were added as root devDependencies (test oracle only). `bun install --lockfile-only`
    added 2 lines to `bun.lock`.
- Conformance corpus: 5 new `🧩️component` cases, generated by `w1e-conformance-cases.py` (76 cases; counts bumped in Rust and TS):
  - `🧭️dial-with-snaps`: rad stored, ° shown, detents at ±180/±90/0.
  - `📈️log-slider`: ticks at 0.25/0.5/1/2/4, hard `> 0` refusal.
  - `🔁️display-factor`: stepper π/4 reads 45 °.
  - `📍️stepper-detents`.
  - `⛔️hard-bound-refusal`: number field, exclusive 0 / max 100.

### S2.3 Renderers — React DONE, tested (wgpu: S2.5b)

- React `🎚️Slider`:
  - New props `scale`, `appearance` (an SVG dial face with radial ticks, a needle and a thumb on the rim; the pointer angle goes
    through `dial_position`), `displayFactor`, `precision` and `limits`.
  - Ticks, the thumb and the ready extent sit at axis positions.
  - ARIA min/max/now are in display units.
  - The typed readout reads display units. A refused value keeps editing, sets `aria-invalid`, and shows the refusal in a
    `role="alert"` (`aria-describedby`). An admitted value beyond a soft travel commits unclamped.
- React `🪜️Stepper`:
  - New props `snapValues`, `displayFactor`, `unit` (suffix), `limits` and `aria-valuetext`.
  - Typing goes through a draft; refusals are visible and nothing is dispatched.
  - Arrows and page keys follow the shared key law.
- Interpreter (`RE/🗣️Interpreter`):
  - `SliderView` and `NumberStepperView` pass every facet.
  - `InputView`: number fields show and read display units, and refuse unreadable or crossing values visibly (the draft is kept).
  - The external slider unit readout is the display text plus the shown unit.
- ui-react `tsc --noEmit` is clean; the suites are in S2.6.

### S2.3b Keyboard law — coordinator decision §18 (17:00) — DONE, tested

This replaces the S2.1 page rule. `ui_number_key_value` / `slider_key_value` (Rust and TS twins) now take `precision` and
`display_factor`:

- **Step:** `step`, else `10^-precision` display units divided back by the factor, else 1.
- **Arrows:** one step on the ladder from `min`.
- **PageUp/PageDown:** the next or previous detent; without one ahead, 10 steps.
- **Home/End:** the bounds.
- **Rounding:** walked values are rounded half away from zero at `precision` in display units, divided back, then clamped. A value
  within ladder tolerance of a detent lands on the detent exactly.

All callers were updated compile-atomically: React Slider, Stepper and the Interpreter's `pageKey`; the shell `nudge_by`/`slide`
pass `precision` and `None` (S2-W2C's file); ShellHelpers' `StagedVectorField` passes `control.precision` and
`control.displayFactor` (S2-W2B's file); wgpu events pass `input.precision`, and the slider passes `None`/`None` until S2.6 wires
the node facets.

New `keys` rows:
- `vector-page-to-the-next-detent`: S2-W2B's case, x = 0.25 → 1.5.
- `vector-page-down-to-the-previous-detent`
- `page-without-snaps-walks-ten-steps`: → 0.35.
- `arrow-step-from-precision-and-display-factor`: → 0.26.
- `half-away-rounding-in-display-units`: ±0.025 → ±0.03.
- `home-reaches-the-hard-minimum` and `end-reaches-the-hard-maximum`.
- The dial rows were renamed to the detent-jump semantics. The rows retired by the generator are dropped.

The decimal.js ladder oracle applies the same display rounding. To keep the stories' and consumers' literals free of the new
facets, they are optional in the TS projection (SliderProps, NumberStepperProps and InputProps facets; the UiNumberBound refusal;
the UiNumberLimits sides). The renderer-react stories typecheck clean again.

### S2.4 Time-travel mapping (`🔌️plugin/⏪️time-travel/🦀️.rs` `🔖️Panel` region) — DONE, plugin lib compiles

`time_travel_input_row` maps every facet:
- **Slider:** scale, units, display factor, precision, detents and hard limits.
- **Dial:** a slider with `appearance(Dial)` and the same facets. Before, it dropped snaps, units and the factor.
- **Stepper:** detents, stored and shown unit, display factor and limits. An excluded schema bound is limits-only, never a key range
  end.
- **Number:** display factor and limits; the shown unit goes into the row description.
- **Vector:** display factor, limits and the shown unit.
- Select/Segmented, Toggle, Colour, Reference and Text are unchanged; they were already complete.

New helpers:
- `time_travel_unit_symbol`: `deg` reads `°`.
- `time_travel_bound_excluded`
- `time_travel_number_limits`: builds the contract limits from the schema bounds, with refusals localized through the new
  `HistoryPanelText::{AtLeast, GreaterThan, AtMost, LessThan}` (en: "Must be greater than {bound}", de: "Muss größer als {bound}
  sein"…). The bound is named in display units and the shown unit.
- `time_travel_slider`

`cargo check -p semio-framework-plugin --lib` ✔ (140 warnings, none left in the Panel region).

### S2.5 Runtime law (puzzle 2d rotate/scale descriptors → controls) — WRITTEN, run pending

The puzzle 2d corpus law `every_corpus_scenario_edits_its_leaves_in_history_and_overwrites_to_a_fresh_fold`
(`PZ2D/✏️editor/🧪️tests/🧪️select-tool-history`, S2-W2D's harness, region-scoped addition) now also checks each input's rendered
contract component in the history body against a new corpus key `component`. The schema `🔣️select-tool-history` admits it.

The corpus pins these facets:
- `/angle` is a dial over ±π with step π/180, `rad` → `°` (×57.29577951308232), detents at −π, −π/2, 0, π/2 and π, and open
  limits.
- `/factor` is a log slider over 0.1..10 with step 0.01, precision 2, detents at 0.25/0.5/1/2/4, and a `> 0` limit refusing with
  "Must be greater than 0".
- `/pivotX` is a stepper with precision 2.

### S2.5b wgpu renderer (`UI/🎯️targets/🧊️wgpu`, `UI/🧱️elements/{🎚️Slider,🪜️Stepper}/🎯️targets/🧊️wgpu`) — WRITTEN, compile/test pending

Nodes and reconcile:
- `ActionDescriptor`, `UiSliderNode`, `UiNumberStepperNode` and `UiInputNode` derive `Default`. Every literal ends with
  `..Default::default()`; the codemod is `T/🧪️s2-w1e-wgpu-node-literals.py`.
- `UiSliderNode` gains appearance, scale, precision, display unit, display factor and limits. The stepper gains detents, shown
  unit, factor and limits. The input gains factor and limits. Reconcile maps them all.

Display and geometry:
- Node helpers `readout`/`shown_unit`/`unit_label`/`axis_position` and `value_text` depend only on `ui_contract`, so the
  feature-less guest build compiles.
- The external unit readout is the display text plus the shown unit.
- Slider geometry runs on axis positions: ticks at `slider_axis_position`, thumb and range at the axis share.
- The dial: `dial_face`/`dial_point`/`dial_tick_lines`. `slider_node_pointer_value` reads the pointer angle about the face, or
  the track share. The retained and test painters share `paint_slider_travel`: a face, radial ticks, a needle and a rim thumb, or
  rail/range/ticks/thumb.

Keys and typed values:
- The keyboard law receives precision and factor. Stepper PageUp/PageDown follow the law.
- The shared verdict `events::typed_number`/`edit_refusal` drives every typed commit:
  - an unreadable text, or one crossing a bound, is refused and never dispatched;
  - the slider readout keeps its edit open on Enter;
  - blur keeps a refused buffer;
  - a stepper keeps its draft as typed and is no longer clamped while typing.
- The refusal lives in a new `WidgetState.number_refusal`:
  - the slider paints the message in its track cell and the draft in the error colour;
  - the stepper draft and the number field border and draft turn the error colour;
  - the accessibility tree puts the message in the node description.
- Number fields with a factor show and seed display text.

### S2.5c The wgpu corpus law and the laws the hard-bound rule rewrote

New law `the_g6_number_control_cases_carry_and_paint_every_facet` (`UI/🧪️tests/🧪️conformance-corpus`; the shape groups now count
54 accept cases). It pins, through the mounted retained nodes:
- **Dial:** appearance, `rad`/`°`, five detents, readout `90`, unit readout `90 °`, open limits, painted ticks, the 90° tick pointing
  up and the 0° tick at three o'clock, a pointer at 88° landing on π/2 exactly and one at −120° reading −120°, an arrow stepping 1°,
  a page key jumping to π/2.
- **Log slider:** ticks at log positions, 1 in the middle, `-5` refused with "Must be greater than 0", `20` admitted beyond the soft
  travel.
- **Steppers:** the radian stepper reads `45 °` and a typed `90` keeps π/2 exactly; the detent stepper pages 2 → 5 and refuses `12`
  with "Must be at most 10 mm".
- **Hard-bound field:** `-1`/`0`/`120`/`x` refused, `4.5` admitted.

The 23:00 run of `cargo test -p semio-framework-ui --features testkit --lib` gave 764 ✔ and 7 ✘. Six were mine and are fixed in
source:
- **Slot budget:** `UiSurfaceRegistry` element 165872 → 165920 B, because of `WidgetState.number_refusal`. The fixture
  `⏳️async/🧫️fixtures/🧱️boxed-fixed-slots` is updated.
- **Shared fixtures moved to the hard-bound law** (typed out-of-range values are refused, never clamped):
  - `⌨️number-stepper-editing`: `validEdit` is 3 → 3, and a new `refusedEdit` (9, the draft kept) is in the schema.
  - `⌨️slider-readout-editing`: `rejected[].editing` is new; `out-of-range-enter-is-refused` keeps the editor open on `11`.
  - The consumers are wgpu `🔬️targets-wgpu-events-unit` and React `⚙️settings-general-layout`.
  - The retained-control-commit law is now `a value above/below the bounds is refused, the draft kept`.
- **The Enter commit of a blur field** records its refusal.
- **A slider readout keeps its draft on blur only after a refusal was shown** (React's `handleEditBlur` parity).
- **The number-field commit** snaps to the step again, rounding in display units.
- **The slider key-row law** passes precision and factor and skips `step: 0` rows, like the React slider.
- **React Stepper arrows** dispatch on every press again (the bound at the bound, the step as a delta), as `⌨️number-stepper-editing`
  pins.

The 7th failure is not mine: `theme::tests::no_wgpu_target_paints_a_hand_written_colour_literal` flags 3 literals in
`📐️Canvas2dHost` (peer).

Re-run pending under the coordinator's CARGO HOLD (rule 26).

### S2.6 Verification so far (gated, foreground, `target-nde-s2-w1e`)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-ui-contract --tests --all-features` | 0 errors (warnings only) |
| `cargo test -p semio-framework-ui-contract --all-features --tests -- --test-threads=1` (after the hold, 10-02) | lib 215 ✔, carrier map 12 ✔, typegen 1 ✔ |
| `cargo test -p semio-framework-ui-contract --features typegen --test typegen_export` (regenerate, then pass) | 1 ✔ (91 types) |
| TS `bun -e` self-tests | number-controls 273 ✔, corpus 76 ✔, a11y twin 279 ✔, colour 31 ✔ |
| `cargo check -p semio-framework-plugin --lib` | ✔ |
| `cargo check -p semio-framework-ui --target wasm32-unknown-unknown` (default and `--features wgpu-engine`) | ✔ (guest gate); default re-checked ✔ on 10-02 |
| `cargo check -p semio-framework-artifact-playbook-playbook --target wasm32-unknown-unknown` | ✔ |
| `cargo check -p semio-framework-ui --features testkit --tests` | ✔ |
| `cargo test -p semio-framework-ui --features testkit --lib` (23:00) | 764 ✔ / 7 ✘ — 6 mine, fixed in source (S2.5c); 1 peer (`📐️Canvas2dHost` colour literals) |
| ui-react vitest `🎚️Slider` / `🪜️Stepper` (10-02, `SEMIO_VITEST_POLICY` from `repositoryVitestPolicyV1`) | 20 ✔ / 3 ✔ |
| renderer vitest `⚙️settings-general-layout` (`SEMIO_TEST_LEVEL=long`) | 13 ✔ / 1 ✘ — the ✘ is a peer's `anchorPositionStyle` source pin (panel inset refactor) |
| renderer vitest `⏪️time-travel` component / `🧪️staged-arg-controls` | 31 ✔ / 13 ✔ |
| renderer vitest `🗣️Interpreter` corpus (`SEMIO_TEST_LEVEL=long`) | 79 ✔ |
| ui-react `tsc --noEmit` | 0 errors |
| renderer-react `tsc --noEmit` | none in G6 files (remaining: peer store worker `line`, Shell `idleInstalledServiceStatusV1`) |
| `cargo test -p semio-framework-ui --features testkit --lib` (10-02, after the hold) | blocked: `semio-framework-os-kernel` does not compile (peer `RecordSpecProducer` dsl refactor, 26 errors); the `wgpu` feature depends on it |
| puzzle 2d corpus law, plugin `time_travel` tests, wgpu renderer check | blocked by the same os-kernel break |

Fixes from the 10-02 React runs:
- **React Stepper:** a step button or an arrow while editing showed an empty field; the draft is now `null` unless typed. A refused
  draft survives blur.
- **React Slider:** the readout editor again carries `min`/`max`, now in display units (the hard bounds; an exclusive bound has none).
- **`⚙️settings-general-layout`:** now also checks `⌨️number-stepper-editing.refusedEdit` in React (no dispatch, the draft kept,
  `aria-invalid`).
- **`StagedVectorField`** (S2-W2B's file): the precision-derived step is `10^-precision` display units divided back by the factor, and
  the key law gets the declared step (the law derives the rest), as §18 says. `🧪️staged-arg-controls` passes the corpus rows'
  `precision`/`factor`; the axis `step` attribute is `0.1` (one display unit of precision 1), not `1`.

Open for the coordinator:
- The live probe needs a puzzle 2d re-activation; activation is not run by this WP.
- The shell's dialog staged-arg slider (`staged_arg_row`, S2-W2C's file) still builds a linear slider without display facets.

## Session 3 — 2026-10-02

Successor S3-W1E (session 3, coordinator `⚪b7db773a…`). Focus: verify → fix → close (rule 29). Status: **IN PROGRESS** (updated at
every milestone).

Repair-first check (rule 28): files under `UI/**` newer than the session-2 section — `🪜️Stepper/🟦️.tsx` (the S2.6 React Stepper
fixes, complete; one docstring emoji made unique), `🔬️component-unit` (peer fixture move of `🎛️inline-tree-controls`), two
`📋️project.json` (peer nx targets), `🖼️raster-residency` + `🖼️scene-raster-ownership` (peer), `📚️I18n` + react `🌐️i18n` (peer).
No half-finished W1-E edit.

### S3.1 Shared number-facet helper (staged-arg slider open item) — WRITTEN, TS green, Rust check pending

One mapping from an input descriptor to its UI-contract number facets, used by the time-travel editor and by both shells' staged
dialog fields (so React and wgpu render the same control):

- Rust (`🛂️manifest/🦀️.rs`, new region `🔖️ActionArgFacets`):
  - `pub struct ActionArgNumberFacets { min, max, step: Option<f64>, appearance: SliderAppearance, scale: UiNumberScale, unit,
    display_unit: Option<String>, display_factor: Option<f64>, precision: Option<u16>, snaps: Vec<f64>, limits: UiNumberLimits }`
    plus `shown_unit()`.
  - `impl ActionArgDef { pub fn number_facets(&self, locale: Locale) -> Option<ActionArgNumberFacets> }` — `None` unless the
    control is Number/Stepper/Slider/Dial/Vector.
  - `pub fn action_arg_unit_symbol(unit: &str) -> String` (`deg` → `°`, `percent` → `%`).
  - Key range: a slider's/dial's travel; a stepper's/field's/axis' hard bounds, an excluded bound `None`. Step: non-positive →
    `None`. Precision clamped to 15. Detents: ascending, inside the key range, admitted by the limits. Limits: the schema's hard
    bounds with the refusal copy (en/de) naming the bound in display units beside the shown unit.
- TS twin (`🛂️manifest/🟦️.ts`, region `🔖️ActionArgFacets`): `actionArgNumberFacets(def, locale)`, `actionArgUnitSymbol`, type
  `ActionArgNumberFacets`.
- Fixture `🛂️manifest/🧫️fixtures/🧫️number-facets` (9 cases + schema), written by the independent Python implementation
  `T/🧪️s3-w1e-number-facets.py`. Rust test `🧪️tests/🧪️number-facets/🦀️.rs`; TS test `🟦️.ts` beside it (npm `jsonschema` validates
  the corpus, decimal.js recomputes each refusal's bound in display units, the contract detent/range laws hold on every row).
- The time-travel `🔖️Panel` region now maps every number control through `number_facets`; `time_travel_unit_symbol`,
  `time_travel_bound_excluded`, `time_travel_number_limits` and `HistoryPanelText::{AtLeast, GreaterThan, AtMost, LessThan}` are
  deleted (the copy lives in the manifest helper); `time_travel_slider(value, &facets)`.

### S3.2 Verification (gated, foreground, `target-nde-s3-w1e`)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-ui-contract --tests --all-features` | ✔ (warnings only) |
| `cargo test -p semio-framework-ui-contract --all-features --tests -- --test-threads=1` | lib 215 ✔, carrier map 12 ✔, typegen 1 ✔ |
| `cargo check -p semio-framework-ui --features testkit --tests` (native wgpu, S2.5b) | ✔ (132 warnings) |
| `cargo test -p semio-framework-ui --features testkit --lib` (S2.5b + S2.5c) | 769 ✔ / 2 ✘: peer `📐️Canvas2dHost` colour literals (theme law); `the_history_editor_controls_project_their_corpus_accessibility` "a11y.choices: layout settles" (investigating) |
| `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️number-facets/🟦️.ts` | 13 ✔ |
| ui-react `bun ./📜️script.ts test 🎚️Slider 🪜️Stepper` (`SEMIO_VITEST_POLICY` from `repositoryVitestPolicyV1`, budget 540 s) | 2 files, 23 ✔ |
| renderer-react `bun ./📜️script.ts test 🗣️Interpreter 🧪️staged-arg-controls ⏪️time-travel/🧪️tests/🧩️component ⚙️settings-general-layout` (`SEMIO_TEST_LEVEL=long`) | 248 ✔ / 3 ✘ — all peer: `⚙️settings-general-layout` `anchorPositionStyle` source pin and 2 `📐️overlay-flow` Interpreter laws (Overlay inset/size now `calc(n * var(--ui-spacing))`, the tests still expect rem) |
| `cargo check -p semio-framework --lib --tests` (manifest helper + its test) | ✔ (12:08) |
| `cargo check -p semio-framework-plugin --lib`, `cargo check -p semio-framework-ui --target wasm32-unknown-unknown`, UI lib re-run | BLOCKED since 12:21 by peer breaks: `🧬️schema/📇️registry/🦀️.rs:349` E0428 duplicate `ArtifactSchemaRegistry` + E0432 `semio_framework_os_kernel`; `📡️replication/🎮️mutation/🦀️.rs:218` E0425 `StateClass`; (12:02–12:24) os-kernel `🏪️store/🦀️.rs:3353` E0433 `os_schema_composition` — reported to the coordinator |

### S3.3 Fixes

- **`the_history_editor_controls_project_their_corpus_accessibility` "a11y.choices: layout settles"** — the corpus `mount` helper
  busy-polled the layout worker pool 200 000 times; under fleet load (load 70+) the worker had not delivered by then. The loop is
  now bounded by a 120 s wall deadline and yields the thread between polls (`🧪️conformance-corpus/🦀️.rs`). Re-run pending (tree red).

### S3.4 Gap N2 (editor limits) — WRITTEN, verification pending (tree red from peers)

Agreed with S3-W2A (one paging mechanism = tree windows, no new verb, no view state) and relayed to the coordinator.

- **Verb shape:** `historyEditInput{path, value?, edit?: "insert"|"remove", generation?}`. No `edit` sets the input, as before.
  `insert` puts `value` (absent → the item schema's default) at `<list>/<i>` or `<list>/-`; `remove` drops the item at
  `<list>/<i>`. Validation is unchanged: `minItems`/`maxItems` refuse through the normal refused row (on the list's row).
  - Manifest `🔖️HistoryEdit`: `HISTORY_EDIT_ARG_EDIT`, `HISTORY_EDIT_INPUT_INSERT`/`_REMOVE`; the `historyEditInput` definition
    has `value` optional and an `edit` select. TS twin constants, `🧫️history-edit-actions` fixture + schema, and the Rust and TS
    vocabulary laws are updated (bun 3 ✔).
- **TT `🔖️Pointer`:**
  - `time_travel_pointer_insert` / `time_travel_pointer_remove` (private `time_travel_pointer_get_mut`).
  - `time_travel_default_value`.
  - `TimeTravelInputRow.{list, item}` with `TimeTravelList { len, addable }` and `TimeTravelListItem { index, removable }`.
  - `time_travel_input_rows` expands a list of ANY item kind: the list row, then per item an item row (an object item: header
    + fields; any other item: its own value row).
- **TT `🔖️Driver`:** `draft_time_travel_input(path, value: Option, edit: Option, …)`. The list edit resolves the list input
  (`Array` item schema, or a reference list element) and inserts or removes, then runs the same verdict/rebuild tail.
- **TT `🔖️Panel`:**
  - (a) `framework.history.editor.inputs` is `tree_window_indexed_section` over every row.
    `TIME_TRAVEL_EDITOR_INPUT_ROWS` and the "N more inputs" row are deleted.
  - (b) A reference input row is a `tree_window_indexed_item`: [Use selection, one chip row per id (label = entity name; Remove =
    `edit: remove` at `<pointer>/<i>`, many only, disabled at `minItems`), Clear if nullable]. `TIME_TRAVEL_PANEL_CHIPS` and the
    overflow row are deleted; the remove args are O(1) (no list copy per chip).
  - (c) The list row holds Add item (disabled at `maxItems`, reads "Items: n"). An object item row holds Remove item; any other
    item nests a Remove item row (disabled at `minItems`).
  - (d) A select/segmented input with more than `UI_FIXED_LIST_ITEMS` options is a windowed list of option rows (check icon on
    the chosen one). ≤ 32 options stay a Select. Option search is deferred: every option is reachable by scrolling.
  - New copy `HistoryPanelText::{AddItem, RemoveItem, ItemCount}` (en/de); `MoreInputs`/`MoreReferences` are deleted.
    `time_travel_clear_args` became `time_travel_value_args`; new `time_travel_list_edit_args`.
  - `time_travel_editor_sections(.., windows, ..)`: the one call line in `ui_history_panel` passes `&windows`.
- **Laws:**
  - New `list_inputs_add_and_remove_items_within_their_bounds` (plugin `🧪️time-travel`).
  - Updated: `input_pointers_and_host_values_take_the_declared_shape` (list + item rows), the chips law (chip rows, window
    total ≥ 11, remove by index), the session-leads law (inputs are a window over every row).
  - The puzzle 2d `useSelection` corpus step finds the reference row (`….row`).

### S3.5 Resume after the usage cut + reboot (18:40) and N1 tree-disclosure parity — WRITTEN

Repair check: every S3.1–S3.4 edit is on disk (no half edit). The 12:5x checks died with the reboot.

wgpu UI tree disclosure (found by S3-W2C):
- `UiTree::disclosure_open` and `disclosure_is_interactive` (`UI/🎯️targets/🧊️wgpu/🌳️tree`) treated a tree item as a disclosure only
  with materialised `items`. A closed history row (N1: `window.total` = op count, no children) therefore had no `aria-expanded`,
  and the retained toggle refused it.
- Both now read the new `UiTreeItemNode::has_rows()` (`🧩️component`): materialised items, or `window.total > 0`. That is the
  predicate `📌️mounted_layout` already used; its private `tree_item_has_rows` is deleted in favour of the method.
- New law `a_tree_item_with_unstreamed_window_rows_is_a_closed_disclosure` (`🧪️tests/♿️retained-section-accessibility`, sharing
  a new `mounted(records)` helper). A windowed item without children is collapsed, toggles open and announces expanded; a leaf is
  no disclosure and the toggle refuses it.

### S3.6 N2 disabled list edits name their bound (coordinator item 3) — WRITTEN

- `TimeTravelList.max` and `TimeTravelListItem.min` carry `maxItems`/`minItems`.
- `time_travel_item_limit_text` gives the reason ("Maximum 8 items" / "Höchstens 8 Einträge", "Minimum 1 item" / "Mindestens 1
  Eintrag", singular at 1). The new copy is `HistoryPanelText::{MaxItems, MaxItem, MinItems, MinItem}`.
- Where the reason shows:
  - The list row reads "Items: n · Maximum n items" while Add is disabled.
  - A disabled Remove item (its own row or an object item's header) carries the reason as its row description.
  - A reference-list chip at `minItems` does the same.
- The list law pins the texts in both locales.

### S3.7 Verification after the reboot (gated, foreground)

| Command | Result |
|---|---|
| `cargo check -p semio-framework --lib` | ✔ (lib test red from peers: `🕹️interaction/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:7` `semio_framework_schema::SchemaFormat` moved) |
| `cargo check -p semio-framework-plugin --lib` (N2 + facets + Panel) | ✔ 18:50, no warning in `⏪️time-travel` |
| `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2` | ✔ |
| `cargo check -p semio-framework-plugin --lib --tests` | ✘ 58 errors, 0 in my files: peer `LocalizedLabel` crate-root re-export removed + `semio_framework_schema::artifact_schema_descriptor_registered` moved |
| `cargo check -p semio-framework-ui --features testkit --tests` | ✔ |
| `cargo check -p semio-framework-ui --target wasm32-unknown-unknown` (default and `--features wgpu-engine`) | ✔ / ✔ |
| `cargo test -p semio-framework-ui --features testkit --lib` (18:55) | 767 ✔ / 1 ✘ (my new disclosure law built its items as bare tree items, which the reconcile wraps as their own sections; fixed to a section parent) → `-- retained_section_collapse_tests conformance_corpus` 16 ✔, incl. the disclosure law and the a11y corpus law (the yield fix holds) |
| puzzle 2d `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --lib -- select_tool_history` | BLOCKED 19:0x: os-kernel red from a peer DSL crate split (`🗣️dsl/👪️family/🗂️catalog/🦀️.rs:11` `semio_framework_dsl` unresolved) |

### S3.8 Tree row semantics on both renderers (coordinator decisions 1 + 2) — React verified, wgpu written

**Contract:**
- `RowAction.reason: Option<Label>`, plus `RowAction::disabled_because(reason)`. A disabled action stays focusable
  (`aria-disabled`) and every renderer names the reason as its description.
- The typed catalog gains field `5 => reason`.
- The TS projection is regenerated (`SEMIO_TYPEGEN_OUT`; schema metadata RowAction v4, `reason?: Label | null`).
- The retained typed-wire TS decoder reads `reason`.
- Literal sites get `reason: None`: the runtime reconcile-unit test, the plugin `row_action` builder and its patches unit test,
  and the wgpu shell's row-action packing (S3-W2C's file; it has no reason to carry yet).

**React:**
- (1) `🌳️Tree`: a property row (one holding a control) now shows its `description` beside the label
  (`data-slot="tree-description"`). `useTreeRowDescribedControls` names it in the `aria-describedby` of every focusable inside the
  value slot and keeps it there across the control's own re-renders (MutationObserver). Before, the editor's refused reasons,
  units, "Cleared" and item counts were invisible in React.
- (2) `TreeRowActionButton`: a disabled tree row/header action is `aria-disabled` (not `disabled`), stays focusable, refuses
  activation, and names `reason` through `aria-describedby` (sr-only text).
- `TableRowActionButton` in the Interpreter does the same for table row actions. The Interpreter passes `RowAction.reason` to the
  Tree.
- This is product-wide, as decided. The laws that pinned native `disabled` now pin focusable `aria-disabled` + no dispatch:
  - engine-contract "gates Execute on required args";
  - time-travel N15 "a refused Edit says why";
  - Interpreter table "dispatches a tree row and a table row with one target identically".

**wgpu ARIA mirror (`♿️accessibility`):**
- A control under a tree item (not a nested row, inline toolbar or detail) carries the row's description (after its own, ` · `).
- A row action's virtual button stays focusable/tabbable when disabled, is not actionable, and its description is the reason.
  `ROW_ACTION_KEY_INFIX` is `pub(crate)`.

**Shared corpus case `🖥️composite/💬️row-semantics`:**
- Written by `T/🧪️s3-w1e-row-semantics-case.py`. It holds a property row "Scale" whose description is a refusal and which holds a
  slider, and a row "Move" with a disabled "Edit" whose reason is "Not possible right now".
- New expectation key `rowSemantics.{describedControls, disabledRowActions}`.
- Case counts 76 → 77 (Rust contract, TS contract, React Interpreter); the wgpu shape groups go 54 → 55.
- Laws:
  - React "exposes row descriptions on their controls and keeps disabled row actions focusable with their reason";
  - wgpu `row_descriptions_reach_their_controls_and_disabled_row_actions_stay_focusable_with_their_reason` (also checks that the
    contract twin `RowTarget::action_binding` refuses the action).

| Command | Result |
|---|---|
| `cargo test -p semio-framework-ui-contract --all-features --tests -- --test-threads=1` (after `reason`) | 215 + 12 + 1 ✔ |
| typegen regenerate + `--test typegen_export` | ✔ (91 types) |
| TS `conformanceCorpusSelfTests()` (bun) | 77 ✔ |
| ui-react `tsc --noEmit` | 0 errors |
| renderer-react `tsc --noEmit` | 4 errors, all peer (store worker `line` ×3, Shell `idleInstalledServiceStatusV1`) |
| ui-react vitest `🌳️Tree 🎚️Slider 🪜️Stepper` | 4 files, 78 ✔ |
| renderer vitest `🗣️Interpreter ⏪️time-travel/🧪️tests/🧩️component 🔬️engine-contract` filtered to the touched laws (`-t …`) | 7 ✔ |
| renderer vitest full five suites (`🗣️Interpreter 🧪️staged-arg-controls ⏪️time-travel/🧪️tests/🧩️component ⚙️settings-general-layout 🔬️engine-contract`) before the law updates | 939 ✔ / 14 ✘. 3 were the old `disabled` pins (now updated, green above). 11 are peer: overlay flow ×2 and declarative-forms gap (`--ui-spacing` refactor), settings `anchorPositionStyle` pin, text-editor key/paste/compose ×5, graph parameter dispatch, tutorial portal (`Worker is not defined`) |
| wgpu `cargo test -p semio-framework-ui … conformance_corpus` (row-semantics law + 55 cases) | PENDING — tree red (peer DSL crate extraction); waiting for the coordinator's TREE GREEN |
| `cargo test -p semio-framework-ui-contract --all-features --tests -- --test-threads=1` (19:27, with `💬️row-semantics`; count 77) | 215 + 12 + 1 ✔ |

### S3.9 Owed after TREE GREEN, open items, coordinator actions (19:30)

Owed verification. These wait for the coordinator's TREE GREEN; the peer DSL crate extraction keeps os-kernel red. One run each:
1. `cargo test -p semio-framework-ui --features testkit --lib`: full suite, incl. the row-semantics law and the disclosure law.
2. `cargo test -p semio-framework --lib -- number_facets history_edit_actions`. The lib-test binary was red from a peer
   (`🕹️interaction/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:7`).
3. `cargo test -p semio-framework-plugin --lib -- time_travel`, covering:
   - the N2 list law;
   - the updated chips, session-leads and pointer laws;
   - S3-W2A's editor-keys law.

   The plugin lib-test binary was also red from peers (`LocalizedLabel` crate-root export, schema-registry moves).
4. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --lib -- select_tool_history`: the S2.5 runtime law,
   with the facets now from `number_facets` and the `useSelection` step on the windowed reference row.
5. `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2` and `cargo check -p semio-framework-ui --target
   wasm32-unknown-unknown --features wgpu-engine` again, after the S3.8 edits.

Open items:
- N2 (d) option search is deferred; every option of a long choice is reachable through the windowed option rows.
- The `reference_list` contract recipe is no longer used by the time-travel editor. It stays a contract recipe with its own corpus
  case, and React's staged reference field mirrors its semantics.

Coordinator actions (not run by this WP):
- Re-activation of puzzle 2d (React 6012 + wgpu 6112) for the live probe of the editor rows (windowed inputs/chips/options,
  list add/remove).
- No descriptor, `describe`, launch.json or central schema generate is needed: the history-edit verbs stay 12, and the new
  `edit` arg is a manifest definition change only.

### S3.10 Owed runs after TREE GREEN (10-03 05:50)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-ui --features testkit --lib` (05:49) | **769 ✔ / 2 ✘**. Every W1-E law is green: `row_descriptions_reach_their_controls_and_disabled_row_actions_stay_focusable_with_their_reason`, `a_tree_item_with_unstreamed_window_rows_is_a_closed_disclosure`, `the_history_editor_controls_project_their_corpus_accessibility`, the G6 corpus law and the 55 shape cases. Both ✘ are peer: `wgpu::draw::tests::world_mesh_instance_packs_policy_and_standard_material_into_one_fixed_stride` (128 ≠ 112) and `wgpu::gpu::prepared_present_tests::every_world_color_cursor_uses_the_encoded_composite_attachment` (17 ≠ 15 pipelines) — world/GPU pipeline work |
| `cargo test -p semio-framework --lib -- number_facets history_edit_actions dialog_choices` (05:52) | BLOCKED: peer test `🎠️kernel/🧪️tests/💡️service-operation/🦀️.rs:9` (`dsl::json` gone) breaks the lib-test binary |
| `cargo test -p semio-framework-plugin --lib -- time_travel` (05:57) | BLOCKED: 81 errors in 7 peer test files (`?` cannot convert `dsl::io_schema::IoError` to `String`: `🧪️tests/🧩️composition/🦀️.rs:21`, `🏗️builder/🧪️tests/🔬️schema-stamping/🦀️.rs:187`, `🧪️tests/🔬️app-declarations-fixture/🦀️.rs:75` …), 0 in my files. Reported once to the coordinator |
| puzzle 2d `-- select_tool_history` (06:06) | BLOCKED: `semio-framework` lib red again in flight (`🧰️framework/📦️packages/🦀️rust/🦀️.rs:28` `semio_framework_io_schema` unresolved, peer DSL/io-schema extraction) |
| `cargo check -p semio-framework --lib` (06:23) | ✔ (the in-flight io-schema split settled) |
| puzzle 2d `-- select_tool_history` (06:36) | BLOCKED: peer red in the stdio dependencies of the ✏️s workspace, the same `?`/error-type migration: `🎒️zip/…/🪶️sqlite/🚦️native/🦀️.rs:41` (2 errors), `🎨️svg/…/🔰️basic/🧬️schema/🦀️.rs:198` (6), `🧊️gltf/…/♾️any/🧬️schema/🧬️mutations/🦀️.rs:135` (13) |

Status at 06:40: every W1-E change is source-complete.

- **Verified green:** the UI contract (215+12+1, typegen), the UI lib laws (every W1-E law; 769 ✔), React (ui-react 78 ✔, the
  touched renderer laws 7 ✔), the TS corpus (77), bun facets (13) and history-edit vocab (3), and `cargo check` of the plugin lib
  (native and wasip2) and the UI crate (wasm32 ×2).
- **Still blocked by peer test or dependency breaks**, all owner-guess the DSL/io-schema extraction:
  - the manifest `number_facets` law;
  - the plugin `time_travel` laws (N2 list law, updated chips/session/pointer laws);
  - the puzzle 2d S2.5 law.

  Re-run these three commands once the peer's migration reaches the test files and the stdio crates.

### S3.11 F7 — `ContextMenuItemSpec.reason` (S3-W2A request) — Rust written, React verified

- Contract (`UI/🎯️targets/🧊️wgpu/🧩️component`):
  - `ContextMenuItemSpec.reason: Option<String>` (serde/value `reason`, skipped when None).
  - `impl ContextMenuItemSpec { pub fn disabled_because(mut self, reason: String) -> Self }`.
  - The wire-format test pins the absent/present forms.
- TS twin (`UI/🎬️scene`): `readonly reason?: string`.
- React (`🖱️ContextMenu`):
  - `ContextMenuItem.reason?: UiLabel`.
  - A disabled row is `aria-disabled` (no native `disabled`) and is reached by arrow keys (`contextMenuFocusableIndices`); ordinals stay enabled-only.
  - The reason shows beside the label and is named by `aria-describedby`; click and Enter do nothing.
  - `mapContextMenuSpecs` (World3dHost) passes `reason`.
  - Law "keeps a disabled row focusable with aria-disabled, names its reason, and never runs it" (11/11 ✔).
- wgpu shell menu (hand-painted chrome, S3-W2C's file): routed to the coordinator with the exact edit points (`ContextMenuItem.reason`,
  `context_menu_enabled_indices`, reason painting + chrome accessibility description).

### S3.12 `x-semio-ui.optionSource` (design §20.11) — TS verified, Rust written (tree red)

Schema-first, end to end:
- **Meta-schema** `🛂️manifest/🧬️schema/🔣️.json` `InputUi.optionSource = {snapshot: "^/…"}`. It also covers the strict
  vocabulary (the validator `$ref`s the manifest `InputUi`). The TS `parseInputUi` + `InputUi` type are updated.
- **Rust:**
  - `pub enum OptionSource { Snapshot { pointer } }` and `ArgSchema::String.option_source`.
  - The reader (`input_option_source`, `INPUT_UI_KEYS` 19) refuses `uiInvalid` on a non-string or beside an `enum`.
  - `control()` gives Select when sourced, and the widget compatibility check accepts it.
  - Every literal construction is updated: manifest ×5, the mcp source-builders test, and the plugin time-travel test.
- **TS twin:** `mutationInputDefs` (`sourcedOptions`), `argControl`, the widget check, and the generated projection. Schema metadata
  `ArgSchema` v3 + new `OptionSource` v1 in `🧬️schema/📽️projection`, mirrored in `🤖️generated/🪪️manifest/🟦️.ts`.
- **Corpus** `🧫️mutation-inputs` +4 cases: `option-source-from-the-previewed-document`, plus three refusals (on a number, beside an
  enum, relative pointer).
- **Time-travel editor:**
  - `🔖️Pointer` gains `time_travel_option_pointer` (fills `{field}` from the draft payload, RFC 6901-escaped),
    `time_travel_document_options` (keys in document order; labelled by record `label`/`name` {en,de}/text → glossary → key) and
    `time_travel_resolve_options`. A recorded key the document lacks stays an option, and no enum is ever added, so the fold
    decides.
  - `🔖️Driver`: `resolve_time_travel_options` plus the shared `time_travel_previewed_value` (the snaps resolver now uses it too).
  - One call line in the history render (`🔌️plugin/🦀️.rs`).
  - The React and wgpu selects are unchanged; more than 32 sourced options become the N2 windowed option rows.
- **Laws:**
  - Rust and TS reader control assertions.
  - The plugin law `sourced_options_are_the_keys_of_the_previewed_document`: template escaping (`w/1` → `w~11`), labels en/de, stale
    key kept, no enum, an unfillable template offers only the recorded key.
- Told S3-PROCEDURAL (a7fd2aa4782869ebd) to declare it on `change-widget-input.channel`, and to keep the fold law refusing an
  unknown key.

| Command | Result |
|---|---|
| `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/🟦️.ts` | 88 ✔ (incl. the 4 new cases + control) |
| `python3 🛂️manifest/🧪️tests/🧪️mutation-inputs/🐍️.py` | 207 jsonschema verdicts over 40 cases ✔ |
| ui-react `tsc --noEmit` | 0 errors |
| ui-react vitest `🖱️ContextMenu` | 11 ✔ |
| renderer-react `tsc --noEmit` | 1 error, peer (`🌐️World3dHost/🟦️.tsx:2226`, surface material work at 06:47) |
| `cargo check -p semio-framework-plugin --lib` (06:56) | BLOCKED: os-kernel red again (peer: `🧬️semio/🦀️.rs:4` `os_dsl::ValueRefusalKind`, `🚪️io/🦀️.rs:2654` `PackError::TextRefusal`), so the manifest/plugin Rust is not compiled yet |

Owed on the next green tree (one run each):
- `cargo check -p semio-framework --lib --tests`, then `cargo test -p semio-framework --lib -- mutation_inputs number_facets history_edit_actions`.
- `cargo test -p semio-framework-plugin --lib -- time_travel`.
- The UI wire-format test `-- ui_node_wire_format`.
- The `semio-framework` projection test `exports_typescript` (app-label), which checks the hand-mirrored generated manifest TS.

Projection check without cargo: a Python simulation of `render_typescript()` over the 204 metadata rows of `🧬️schema/📽️projection`
matches `🤖️generated/🪪️manifest/🟦️.ts` for the `OptionSource` and `ArgSchema` edits. The one remaining diff is a peer's: projection
`TutorialDocumentEventKind` no longer has `coalesceKey` (the §20.1 CLOSURE deletion), while the generated file still does. The
projection test will fail on that until the CLOSURE owner regenerates the file (`SEMIO_TYPEGEN_OUT`).

S3-PROCEDURAL asked for record lookup: generation3d's `hostSnapshot.widgets` is an array of tagged records. The template is now
walked one segment at a time by `time_travel_option_target`, which replaces `time_travel_option_pointer`:
- A `{field}` segment on an object is that object's key.
- A `{field}` segment on an array selects the record whose own `field` equals the payload's top-level `field`.
- A literal segment indexes an array or keys an object, with RFC 6901 escapes.

The spelling stays `/hostSnapshot/widgets/{id}/params`; no extra marker is needed. The meta-schema description, the Rust/TS doc and
the corpus case (`/hostSnapshot/…`) were updated. The plugin law now pins an array of tagged widget records, keyed lookup, numeric
index, and no match. TS reader suite: 90 ✔. Spelling sent to S3-PROCEDURAL.

## Session 4 — 2026-10-04

S4-UI (Opus) inherits S3-W1E and S3-W2B (`📓️w2-b-report.md` points here). Brief: `🧭️plan.md` Session 4 row S4-UI, `📓️s4-resume.md`
§2.2/§2.5/§4 D18 D21/§6.1/§7. Scratch: `🗑️generated/s4-ui/`. Private test target `target-nde-s4-ui`.

### S4.1 Repair-first (rule 34) — done, nothing half-finished

- `git diff HEAD --stat` over `🖱️ui`, `🛂️manifest`, React `🛠️ShellHelpers`/`🗣️Interpreter`/`🏛️ShellHost`/`🔌️PluginRuntime` and plugin
  `⏪️time-travel`. Files touched between the predecessors' last report (10-03 06:59) and the cut (12:07) belong to other WPs
  (S3-GATES oracle claims, S3-NOTICES fault-notices, S3-STROKES/AGNOSTIC mutation-inputs corpus). Everything later (10-03 19:45 → 10-04 00:47)
  is Codex peer churn (`Tree/🟦️.tsx` gained `busy`; `Interpreter` gained the row-action admission). It all compiles and its tests pass; I repaired nothing.

### S4.2 Owed verification

| Command | Result |
|---|---|
| `cargo test -p semio-framework --lib -- number_facets history_edit_actions dialog_choices mutation_inputs every_corpus_case` (`fw-lib-1.txt`) | **22 ✔ / 0 ✘** (owed since S3.9; all W1E manifest laws) |
| `cargo test -p semio-framework-ui --features testkit --lib` (`ui-lib-1.txt`) | **770 ✔ / 2 ✘**. Both ✘ are the known peer failures: `wgpu::draw::tests::world_mesh_instance_packs_policy_and_standard_material_into_one_fixed_stride`, `wgpu::gpu::prepared_present_tests::every_world_color_cursor_uses_the_encoded_composite_attachment` |

### S4.3 W1E-3 kernel half — one status copy (coordinator decision, 02:20) — DONE, verified

- `FW/🎠️kernel/🦀️.rs`, new region `🔖️HistoryReprojectionStatus`:
  - `HISTORY_REPROJECTION_LABELS` (11 rows `remote|step|load.{title,progress,paused,refused}` + `reason.unnamed`, en/de);
  - `HistoryReprojectionStatus { title, text, done, total, paused, fault }`;
  - `history_reprojection_status(&HistoryReprojection, Terminology, Locale)`.

  Rules: a fault with total 0 reads `<kind>.refused` with the code's placeholder-free kernel history notice, else `reason.unnamed`
  (never a raw code); `paused` applies only to remote; anything else reads the kind's progress line. Load gets its own refused copy
  (W2A-8 found that Load reused the step copy).
- TS twin `historyReprojectionStatus` + `HISTORY_REPROJECTION_LABELS` live in `FW/🎠️kernel/🟦️.ts`, in the same-named region.
- Schema-first:
  - `🧬️schema/🔣️history-reprojection/🔣️.json`;
  - fixture `🧫️fixtures/🧫️history-reprojection/🔣️.json` (11 cases);
  - laws `🧪️tests/🧪️history-reprojection/{🦀️.rs,🟦️.ts}`. The TS law has the Ajv oracle and hostile rows, and is added to the kernel vitest include list.

| Command | Result |
|---|---|
| `cargo test -p semio-framework --lib -- history_reprojection history_notices history_patch fault_notice` (`fw-lib-2.txt`) | **10 ✔ / 0 ✘** (2 new) |
| kernel TS `bun ./📜️script.ts test 🧪️history-reprojection 🧪️history-notices` (`SEMIO_VITEST_POLICY` = `repositoryVitestPolicyV1(<package dir>)`) | **2 files, 5 ✔** |

Relayed to `main` for S4-WGPU (mirror key `shell.history.reprojection`) and S4-RUNTIME (TT section adopts it, delete the
`HistoryPanelText` replay copies).

### S4.4 `history.step-blocked` kernel notice (coordinator YES, 02:55) — DONE, verified

- `HISTORY_NOTICE_LABELS` now has 7 rows. The new row is `history.step-blocked`: "Later mutations would end with errors — fix or
  withdraw them first." / "Spätere Mutationen würden mit Fehlern enden — zuerst beheben oder zurückziehen."
- It is in the TS twin, the `🧫️history-notices` fixture, and a new `🧫️history-reprojection` case that names it. S4-RUNTIME stores the code
  in TT `step_reprojection_turn` (relayed).
- Rust `-- history_reprojection history_notices history_patch fault_notice` 10/10. Kernel TS suite: 7 files, 78 ✔. Manifest
  fault-notices bun: 5 ✔.

### S4.5 W1E-2 contract `TreeItemProps.selected` + W1E-1 `revealReason` corpus — DONE, verified (contract + React; wgpu by S4-WGPU)

- Contract (`🖱️ui/🧬️contract`):
  - `TreeItemProps.selected: Option<bool>` (serde `selected`, skipped when None) and builder `.selected(bool)`;
  - typed catalog field `13 => selected`;
  - TS projection `TreeItemProps` v3, hand-mirrored in `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` and checked by `typegen_export`;
  - retained typed TS decoder plus its fixture;
  - literal sites in Rust (contract test, runtime ownership/reconcile tests, wgpu shell ×2, plugin builder-contract test ×3) and in TS
    (24 treeItem literals in tests and stories).
- `UiNodeRecord` grew from 7072 B to 7088 B. Re-pinned (greenfield):
  - the resident fixtures/schemas (`🎟️resident`, `🗃️fixed`, `🔄️refresh`, `🌳️root`);
  - the `⏳️async/🧫️fixtures/🧱️boxed-fixed-slots` budget for `wgpu::engine::UiSurfaceRegistry`, 165920 → 165984.
- Corpus `🖥️composite/💬️row-semantics` (generator `T/🧪️s3-w1e-row-semantics-case.py`, extended):
  - `disabledRowActions[i].revealReason {on: [hover, focus, press], off: [leave, blur, escape]}` (W1E-1);
  - `selectedRows` over two new option rows (W1E-2).
- Producer: TT long-option rows (more than `UI_FIXED_LIST_ITEMS` = 32) are `tree_item(label).selected(chosen)` holding the choose
  button. The id `{id}.option.{i}.row` is unchanged.
- React:
  - (W1E-1) new `DisabledReasonHint` (`🖱️ui/🧱️elements/💡️ChromeControlHint`, exported from ui-react). The reason element is always the
    action's `aria-describedby`. It becomes a visible glass tooltip (`[data-slot=row-action-reason][data-revealed]`) on hover (400 ms delay,
    mouse/pen), keyboard focus and press, and hides on leave, blur, Escape and a press elsewhere.
  - Used by the Tree `TreeRowActionButton` and the Interpreter `TableRowActionButton`. The table's `title="label · reason"` mouse-only
    path is gone.
  - Test adapter `fireEvent` gains `pointerEnter`/`pointerLeave`.
  - (W1E-2) the Interpreter maps contract `selected` to the row's `isSelected`, giving `aria-selected` + `data-selected` + the selected fill.
- wgpu (S4-WGPU, reported green 03:4x): `RowReasonHint` router/paint and mirror `selected`. Laws
  `a_disabled_row_action_reveals_its_reason_on_hover_focus_and_press` and `option_rows_announce_and_paint_their_selected_state` —
  `cargo test -p semio-framework-ui --features testkit --lib -- conformance_corpus row_action tree_item accessibility` 40/0 (their run).

### S4.6 W1E-3 React half — DONE, verified

- `ShellHistoryProjectionV1.reprojection` is folded from every patch; absent means `null`.
- New `HistoryReprojectionStatus` (`🛠️ShellHelpers/⏪️time-travel`), mounted by ShellHost beside the band (outside the History panel) for
  the focused program:
  - `[data-semio-history-reprojection=<kind>]` with `-phase=progress|paused|refused`; the code lives only in `data-notice-code`;
  - the title + text + progress bar come from the kernel copy;
  - the `role=status aria-live=polite aria-atomic` text "<title>: <text>" is announced on kind/phase change, not on every progress step;
    the visible line and `aria-valuetext` follow every step;
  - Cancel replay / Replay again (`historyReprojectionControlV1`) appear only while no session is open.
- No raw codes: the band's unknown-fault copy is now the `replayFaulted` label. `ui.timeTravel.fault` is deleted from the schema and the
  en/de bundles. The band corpus case was renamed and re-expected. Sent to S4-WGPU, whose wgpu `format!("Replay failed ({fault})")` must
  follow.

### S4.7 React `requestMediaFrames` host cancel (D21) — DONE, verified

- `runRequestMediaFrames` is replaced by `requestMediaFramesSourceV1(accept, payload)` + `runMediaFramesV1(effect, source,
  dispatchOne, app, signal, progress)`.
  - Both tiers check the signal before each frame, and `waitForVideoEvent` rejects on abort.
  - Each dispatched frame reports progress.
  - A cancel sends no done and no fallback, and dispatches `importAbort` only when a frame reached a declaring guest (remodel's §15
    streamed transaction leaves zero trace).
- ShellHost runs it as a Tasks-window document transfer (name = picked file) with Cancel and the `import-cancelled` notice, as file
  imports do.
- Laws (engine-contract, `-t D5`): 13 ✔, incl. new frame-progress and cancel laws (after 1 or 2 frames, undeclared, pre-cancel).

### S4.8 W1E-5 layering — decided: no change

- `ActionArgDef::number_facets` returns `semio_framework_ui_contract::{SliderAppearance, UiNumberScale, UiNumberLimits, Label}`.
- `semio-framework-ui-contract` is the dependency-free UI boundary crate. It never depends on the manifest.
- The manifest already describes its UI in contract types (`Tone` ×6, `TREE_WINDOW_PATH_SEPARATOR`, `UiText`), and its TS twin imports
  `🖱️ui/🧬️contract/🧩️component`.
- manifest → ui-contract is therefore the legal downward direction, not an inversion.
- Returning contract facets is the point of the S3.1 shared helper: one mapping, no per-renderer re-mapping (time-travel, wgpu
  `staged_arg_row`, React `StagedNumberField`). Moving to plain numbers would re-create three mappings.

### S4.9 Peer fix in my file

- `🗣️Interpreter/🟦️.tsx` `ButtonView` (peer row-action-admission work, 10-04 00:47) called `.then` on any truthy `onIntent` return. A
  host callback returning a non-promise (`dispatched.push(...)`) threw `completion.then is not a function` (5 uncaught errors in the
  time-travel suite).
- Fixed by narrowing with `completion instanceof Promise`.

### S4.10 Verification (session 4, after S4.3–S4.9)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-ui-contract --all-features --tests -- --test-threads=1` (`contract-2.txt`) | **222 + 12 + 1 ✔** |
| `cargo test -p semio-framework-ui --features testkit --lib` (`ui-lib-2.txt`, before the slot re-pin) | 769 ✔ / 3 ✘. 2 are the known peer failures; the third was the `UiSurfaceRegistry` slot budget, re-pinned. Re-run owed |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` (`wgpu-check-2.txt`, 03:27) | ✔ exit 0 |
| `cargo check -p semio-framework-ui-contract -p semio-framework-plugin --lib` (`plugin-check-2.txt`, 04:26) | ✔ exit 0 (TT `.selected` caller) |
| renderer-react W2B suites (`⏪️time-travel/🧪️tests/🧩️component 🧪️staged-arg-controls 📎️local-folders/🧪️tests 🧪️command-rejection 🪟️spawned-program-session`) (`vitest-w2b-2.txt`) | **5 files, 132/132 ✔**, 0 errors (+4 reprojection laws) |
| renderer-react `🗣️Interpreter/🟦️.tsx` in-source (`vitest-interp-2.txt`) | **188/188 ✔** (row-semantics: reveal on/off ×3, selected rows) |
| renderer-react `🧾️typed/🟦️.ts 🔬️engine-contract 🗄️stdio-details-tree 🪟️tree-windows 🪪️container-node-ids 📊️table` (`vitest-related-1.txt`) | 732 ✔ / 1 ✘. The ✘ is a peer test: `🧾️typed` "normalizes the native Input draft target", where the new `draftTarget` test passes a plain object to a native-buffer cursor ("Invalid native buffer admission") |
| renderer-react `🔬️engine-contract -t D5` (`vitest-media-1.txt`) | **13 ✔** |
| ui-react vitest `🌳️Tree 🖱️ContextMenu 💡️ChromeControlHint 🎚️Slider 🪜️Stepper` (`vitest-uireact-1.txt`) | **6 files, 94 ✔** |
| ui-react translation totality | **2 ✔** |
| ui-react `tsc --noEmit` | **0 errors** |
| renderer-react `tsc --noEmit` | **0 errors** |

### S4.11 Tree sections are named groups (S4-WGPU request, approved) — DONE

- `accessibility_projection_node` (Rust) and `uiAccessibilityProjectionNodeV1` (TS twin) name a `TreeSection` by its
  `label`, after the explicit accessibility label as for every other component. The role stays `group`, which is non-interactive.
- The wgpu mirror therefore names `framework.history.reprojection`, `framework.history.editor` and the other sections, as React's headings
  do.
- TS twin self-test `accessibilityProjectionSelfTests()` 279 ✔, and a probe gives `group "Inputs"`. UI lib (below) is green.

### S4.12 Verification after S4.11 (07:0x–08:1x; rule 42 gate from 07:40, rule 43 CHECKS ONLY from ~08:10)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-ui --features testkit --lib` (`ui-lib-4.txt`, after the slot re-pin, with S4-WGPU's laws) | **772 ✔ / 2 ✘**, both the known peer world/GPU failures. Inside it: `ui_node_wire_format` 26 ✔, `conformance_corpus` 12 ✔ (incl. row-semantics reveal + selected), `presence_bar` 7 ✔, `peer_notes` 1 ✔, `row_action` 8 ✔, `option_rows` 3 ✔ |
| `cargo check -p semio-framework-ui --target wasm32-unknown-unknown --features wgpu-engine` (`ui-wasm-1.txt`) | ✔ exit 0 |
| `cargo test -p semio-framework-plugin --lib -- time_travel the_draft_editor_keys` (`plugin-tt-1.txt`) | 41 ✔ / 5 ✘. Every W1E law is green (draft editor keys, sourced options, list inputs, reference chips, filter options). The 5 ✘ are peer areas: 3 × `ValidationFailed("artifact store displaced-owner fixed retirement authority is saturated")` (S4-STORE), "pack reload" `a_history_edit_is_its_own_row…` :1404, and the paused-remote `…rerun` row :1765 (S4-RUNTIME mid-edit on TT reprojection). Routed to `main` |
| `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/🟦️.ts` | 91/1 at bun's 5 s default timeout (Ajv strict-oracle test, 7.6 s at load 80); with `--timeout 60000` **92/92 ✔** |
| `python3 🛂️manifest/🧪️tests/🧪️mutation-inputs/🐍️.py` | **216 jsonschema verdicts over 42 cases ✔** |
| `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️fault-notices/🟦️.ts` | **5 ✔** |

OWED (rule 43, Rust tests frozen; source ready):
- `cargo test -p semio-framework-ui-contract --all-features --tests -- --test-threads=1` after S4.11 (the contract compiled inside `ui-lib-4`).
- `cargo test -p semio-framework --lib -- history_reprojection history_notices` and `--features typegen exports_typescript_bindings`
  (`SEMIO_TYPEGEN_OUT` unset ⇒ compares the hand-mirrored generated manifest TS).
- The new plugin law `long_option_rows_state_their_choice_as_selected` (TT producer of W1E-2). It is drafted in
  `🗑️generated/s4-ui/draft-long-option-rows-law.rs` and lands in `🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs` region `☑️LongOptionRows` when
  tests resume: it needs a `--tests` compile, and an unverified test file edit would put the shared plugin test binary at risk.
- Puzzle 2d `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --lib -- select_tool_history` (S2.5 law; needs ✏️s
  green, ask `main`).

Raw-code fallbacks still in the Rust history body (TT, S4-RUNTIME's regions; routed to `main`): band fault `map_or_else(|| fault.to_string())`
(≈3810) and `history_code_text` `(None, _) => code` (≈3637).

### S4.13 Resume after the 08:55 cut (11:35) — nothing half-finished; final checks

- Repair-first: every S4 edit was already complete on disk; the report was current through S4.12. The plugin wasip2 check that ran across the
  cut finished green.

| Command | Result |
|---|---|
| `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2` (`plugin-wasip2-1.txt`) | ✔ exit 0 (with the TT `.selected` producer) |
| renderer-react W2B suites + `♿️retained-toggle-semantics` + `🗣️Interpreter/🟦️.tsx` + `🧪️fault-notices` (`vitest-final-1.txt`) | **8 files, 326/326 ✔** |
| contract TS twins (bun): `conformanceCorpusSelfTests` / `accessibilityProjectionSelfTests` / fixed-list / carrier map / number controls / color input | **77 / 279 / 75 / 9 / 273 / 31 ✔** |
| `bun ./📜️script.ts verify taxonomy report --scope 🧰️framework/🔨️modules/🎠️kernel` | no finding for the new `🧬️schema/🔣️history-reprojection`, `🧫️fixtures/🧫️history-reprojection`, `🧪️tests/🧪️history-reprojection` (the listed findings are pre-existing dirs) |

### S4.14 Open items, status

| Item | Status |
|---|---|
| W1E-1 disabled reason visible (contract corpus + React) | DONE, verified (React). wgpu half by S4-WGPU, reported green |
| W1E-2 `selected` row semantics (contract + TT producer + React) | DONE, verified (contract, React, plugin compile native + wasip2). Plugin law drafted, OWED (rule 43). wgpu half by S4-WGPU, reported green |
| W1E-3 live reprojection announcement (kernel copy + React status) + `history.step-blocked` | DONE, verified. wgpu half by S4-WGPU (`shell.history.reprojection`), reported green. TT adoption by S4-RUNTIME |
| W1E-5 layering | DECIDED: no change (S4.8) |
| W1E-6 probe derivation | already from `window.total` in source; selectors for the new verdicts sent to S4-E2E via `main` |
| React `requestMediaFrames` host cancel (D21) | DONE, verified (S4.7) |
| React readiness (names, en + de, no raw codes) | band raw code removed (React + corpus + wgpu). Remaining raw-code fallbacks are in TT (S4-RUNTIME), routed |
| Owed Rust tests | OWED (rule 43): see S4.12 list |

### S4.15 Coordinator actions

1. Re-activation of puzzle 2d (React 6012 + wgpu 6112) for the live probe. React shell changes: reprojection status, disabled-reason hint,
   selected option rows, media-frames cancel, band copy. The contract `TreeItemProps.selected` (typed field 13; `UiNodeRecord` +16 B)
   rides the planned rebuild/describe wave of every plugin component (rule 41). There is no channel, frame or descriptor-schema change.
2. No central `schema generate` is needed. The new kernel `🔣️history-reprojection` is a fixture schema, not a catalog leaf, and the
   ui-contract projection (`TreeItemProps` v3) is hand-mirrored and verified by `typegen_export`.
3. When "TESTS RESUMED": the S4.12 OWED list (contract tests, `semio-framework` reprojection + typegen, land and run the drafted plugin law,
   puzzle 2d `select_tool_history`).

