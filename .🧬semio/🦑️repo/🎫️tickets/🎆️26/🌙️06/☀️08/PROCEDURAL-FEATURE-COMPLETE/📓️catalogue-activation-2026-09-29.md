# Catalogue Activation and Placement

## Plan

- Define the typed add-widget contract before code; keep descriptor fields identical for drag and row activation.
- Reject malformed descriptors and surface host creation faults before producing mutations.
- Place automatic additions with the actual rendered rectangle and a 48-unit gap; preserve explicitly supplied drop coordinates.
- Assert the shared JSON cases through Rust production code and Ajv schema validation. Exercise catalogue rows through retained app dispatch, including undo/redo, rejected creation, and repeated additions.
- Coordinate editor action decoding with root; preserve catalogue row identities and viewport windows.

## Validation

- The package router now runs `generation3d-widget-creation-twin` before its native tests. Ajv validation and the independent rectangle sweep answered all 26 shared cases successfully through Nx.
- First native build attempt reached compiler errors in concurrent editor input changes and initial history-test calls. The new history tests were corrected to use `settle_history_verb`, which drives reserved-job publication; another filtered native build is running.
- The initial `--filter` invocation was refused by Nextest. The supported filter is `-E 'test(add_widget) | test(every_catalogue_row_activates)'`.
- Runtime assertions now cover descriptor fields through command decode and retained publication, returned fault without mutation, three automatic additions against every rendered rectangle, and content/layout undo/redo. These tests have not yet run successfully; runtime behavior remains unverified until compilation succeeds.

## Implemented Changes

The add-widget schema carries `kind`, optional discriminated `neuronKind`, `format`, `action`, and optional finite x/y. There is no encoded neuron string support. A registered neuron is checked against the current operator registry. Wrong combinations or unsupported kinds produce `generation3d.widget.add` faults; host add/move errors preserve their original detail in that fault.

Both catalogue drag and activation use `AddWidget::from_catalogue` and its descriptor fields. The virtualized catalogue and stable identity functions are preserved. Explicit x/y drops keep their requested position. Automatic additions use the actual DAG node width/height, including tall operator nodes, and sweep below occupied rectangles at a 48-unit gap. A supplied single axis stays fixed while the other axis is chosen.

## Files Owned

- generation3d `✏️editor/🎮️commands/🧩️add-widget/🦀️.rs` plus `🧬️schema/🔣️.json`, `🧫️fixtures/🔣️.json`, Rust and TypeScript unit tests.
- generation3d `✏️editor/📌️panels/🛍️catalogue/🦀️.rs` and its Rust unit tests.
- One existing add-widget literal in remove-widget tests.
- generation3d Rust package `📜️script.ts` twin registration.
- This ticket note.

Root owns matching editor action decoding/argument declarations, existing editor AddWidget test literals, and launch integration.

## Catalogue Language Integration

The final added `brep.mesh.*` names use Generation3dLabels in both English and German. Catalogue rows receive the chosen label set, including static inputs/outputs and section titles. Format-specific exports keep the uppercase format in the visible name so export rows remain distinguishable. `generation3d_catalogue_name` is available to the inspector through terminology. Existing arbitrary extension metadata names and summaries still come from the extension registry; the change translates the 12 newly added mesh operator names and the static widget names.

The full terminology fixture was extended without removing the concurrent inspector input labels. Its independent twin has passed 525 checks; the creation/localized-roster twin has passed 68 checks. Two intrinsic cluster input/output cases were subsequently added to creation fixtures because those neural builtins appear in the catalogue but are intentionally outside the extension operator-info registry; production accepts those builtin constants explicitly. The next current-code run must include those added cases (expected 70 creation/localized-roster checks).
