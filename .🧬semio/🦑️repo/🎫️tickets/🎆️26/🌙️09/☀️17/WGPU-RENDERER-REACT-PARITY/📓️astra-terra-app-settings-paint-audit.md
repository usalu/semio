# Puzzle3D App Settings Retained-Paint Audit

## Scope and runtime signature

This is a read-only audit. No Cargo command, browser run, or production-source edit was made.

The paired checkpoint-8 Settings artifact records the same native fault repeatedly after the app-owned Settings leaf is mounted:

```text
ui-doc paint fault window=puzzle3d.panel.settings
phase=Some("paint-node") sync-line=0 nodes=Some(true)
```

Examples occur at lines 2638, 2654, and 2702 of [the WGPU console](./🗑️generated/astra-runtime/paired-checkpoint-8-settings/wgpu/console.txt). The surrounding host messages report `retained-fault=None`; this is a document-paint failure, not an engine presentation stall. `sync-line=0` and a present root rule out a synchronize-walk fault and a missing document root.

The captured wasm predates the expanded `paint_stall_census`, so it cannot name the node that faulted. The current census at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1939` will emit the active key, cursor phase, section, item, depth, glyph byte/line, and route state on the next checkpoint run.

## Actual app document

Puzzle3D's app-owned Settings leaf is not blank or malformed. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/⚙️settings/🦀️.rs:53` emits one open `puzzle3d-play-settings` section. Lines `:60`–`:63` emit four fields:

1. `contact-tolerance`
2. `proximity-radius`
3. `chunk-size`
4. `grid-spacing`

Each `stepper_field` creates a `NumberStepper` control with a `Change` binding at `:33`–`:48`; its control id is the field id plus `.control`. This is the expected public Component document shape.

React accepts that same shape. `NumberStepperView` in `🧱️elements/🗣️Interpreter/🟦️.tsx:1339` mounts `Stepper`, preserves `uniform`, and dispatches the `change` binding; its source comment at `:1351` explicitly identifies Puzzle3D's four Settings steppers. The document and its action arguments have therefore already crossed admission before native painting begins.

The failing native route is:

```text
Interpreter document consumer
  → Ui::frame_into_step
  → paint_node_step_with_driver
  → UiNode::NumberStepper, phase 0
  → retained_fixed_output
  → DrawList::push_solid / retained grant
  → RetainedNodePaintStep::Fault
```

`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1581` calls the retained frame route. Its frame driver calls the node painter at `⚙️engine/🦀️.rs:1713`, then records `paint-node` on a node-paint fault at `:1726`. `DrawList::push_solid` claims one retained item for every quad at `🖍️draw/🏷️types/🦀️.rs:863`; a refused ninth or tenth quad leaves the grant faulted, and `finish_retained_output` returns `LimitExceeded` at `:388`–`:393`.

## Ranked causal candidates

| Rank | Candidate | Evidence and disposition |
| --- | --- | --- |
| 1 | **NumberStepper phase-0 retained-output underbudget** | **Confirmed by source.** The retained painter gives every fixed chrome operation an eight-item grant at `🖌️paint/🦀️.rs:57` and `:354`. A NumberStepper phase-0 operation calls `push_control_border` twice at `:1163`–`:1167`. With the normal non-transparent input background, each call emits one background plus four border edges, as `🖥️chrome/🦀️.rs:55`–`:63` shows: ten draw items. The ninth push marks the eight-item retained grant exceeded; `finish_retained_output` returns an error; the painter returns `Fault` at `:1169`. The frame driver records precisely `fault_site="paint-node"` at `⚙️engine/🦀️.rs:1713`–`:1729`. This is sufficient to explain the deterministic app leaf fault. |
| 2 | Missing accepted layout for an individual record | Possible only if candidate 1 is absent in the next binary. `paint_node_step_with_driver` faults when an active node lacks accepted layout at `🖌️paint/🦀️.rs:786`–`:788`. It is lower ranked because the checkpoint reaches `paint-node` after the document's layout phase, and the settings document contains only ordinary section, field, and control nodes. A layout failure will show the keyed node with cursor phase `0`, rather than the predicted stepper phase `1`. |
| 3 | Text/glyph or cursor-identity fault after chrome admission | Low probability. The document's labels and formatted scalar values are short; retained text has a four-megabyte byte bound, while the paint cursor only faults on a changed node/origin or missing tree node at `🖌️paint/🦀️.rs:779`–`:787`. A next-run census whose stepper key reaches phase `1`, `2`, or `3` after the grant fix would distinguish this from the proven phase-0 budget defect. |

The current unit test creates a false sense of coverage: `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-paint-unit/🦀️.rs:272` drives immediate `paint_tree` and correctly expects nineteen stepper instances at `:279`–`:284`. It bypasses `begin_retained_output`, so it cannot detect the retained phase-0 eight-versus-ten failure.

## Bounded production correction

Keep the nested centre-value border. It is part of the immediate painter's nineteen-instance contract and of the React `Stepper` appearance. Do not suppress the centre border or special-case Puzzle3D.

Give NumberStepper phase 0 an exact ten-item retained-output grant, separate from the eight-item default used by simpler fixed-output arms. A narrowly named `RETAINED_NUMBER_STEPPER_CHROME_OUTPUT_ITEMS: usize = 10`, passed through a budgeted fixed-output helper, expresses the actual maximum without inflating every unrelated node's reservation. Ten covers two five-quad `push_control_border` calls when the background is visible; the existing byte bound remains sufficient.

After that correction, the next checkpoint should report the first control as approximately:

```text
paint-key=Explicit("puzzle3d-play-settings.contact-tolerance.control")
cursor=[... phase=1 ...]
```

only if another fault occurs after the phase-0 correction. It must not be interpreted as evidence against the current diagnosis before the corrected binary runs.

## Minimal actual-document native acceptance law

Add one native integration law beside the retained Interpreter/Shell document tests, named for the product document rather than for `NumberStepper` alone, for example `puzzle3d_settings_document_completes_retained_paint`.

The fixture must be the actual Puzzle3D Settings Component document: its section key, four field keys, four `.control` NumberStepper records, `uniform: true` scalar values, and the four `Change` bindings emitted by the production `render` function. It must enter through `UiDocumentTree` ingress and the real `frame_into_step` route used at `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1581`; do not use `paint_tree` or a hand-built `UiNode` leaf.

With a bounded, fixed viewport and a completion cap, require all of the following:

1. Layout, retained paint, scene pass, and hit publication reach `Ready` without `UiFrameStep::Fault`.
2. The final `paint_stall_census("puzzle3d.panel.settings")` has no fault site and no active paint node.
3. All four control keys have accepted layout and produce retained hits; each remains bound to its authored `Change` action.
4. Each control's retained output contains the outer and nested value borders plus minus, formatted value, and plus glyph runs. The nineteen-instance immediate `NumberStepper` law is the native paint oracle; a mounted React `NumberStepperView` using the same records is the behavioral oracle.

This law isolates the real app payload, the document ingress/reconcile path, and the incremental retained painter in one small fixture. It would have failed before the ten-item phase-0 grant correction and prevents a future direct-paint-only test from masking the same defect.
