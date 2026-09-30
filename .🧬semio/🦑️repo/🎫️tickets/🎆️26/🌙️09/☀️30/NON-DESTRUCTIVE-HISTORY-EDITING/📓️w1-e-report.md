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
