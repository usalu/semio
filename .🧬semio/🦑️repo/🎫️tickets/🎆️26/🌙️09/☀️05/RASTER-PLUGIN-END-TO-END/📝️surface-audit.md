# Raster Surface and Integration Audit

Read-only audit of current source and ticket evidence. Source was not edited.

## Actionable Breadth Gap

Non-destructive adjustment layers still support only brightness/contrast. `✏️editor/🎮️commands/🩹️patch-layer/🦀️.rs` accepts only those two numeric parameter names; `🧬️schema/🧬️mutations/🎛️change-layer-adjustment-parameter/🦀️.rs` refuses other parameters and requires brightnessContrast; `🧬️schema/🔺️diff/🌲️layer-tree-apply/🦀️.rs` caps adjustment patches at two entries and the same names. Paths are relative to `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any`. Destructive apply-filter operations do not establish additional non-destructive layer kinds. This is an implementation breadth gap already retained by acceptance.md, independent of the native selection/filter controls other agents are finishing.

`🧬️schema/🧬️mutations/🎚️change-layer-adjustment-kind/🔺️diff/🦀️.rs` currently writes the requested arbitrary kind string for an adjustment layer. This deserves a schema-vocabulary/refusal test before expanding kinds: otherwise a name change can create a layer whose parameters cannot be edited by the retained parameter command. No unsupported-kind rendering outcome was tested in this audit.

## Acceptance Route Still Needed

The latest acceptance record still lacks current-source end-user evidence for the complete import → edit transformed nested pixels/masks/adjustments → undo/redo → editable save/reload → independently decode flattened export sequence. Two-client conflict/disconnect recovery, keyboard-only operation, locale/focus/customization retention and realistic large-image responsiveness remain explicit open rows. They should remain acceptance work rather than being inferred from algorithm, mounted DOM or retained-history passes. Historical activation 18283 predates current changes and cannot establish current runtime acceptance.

The editor Composite/Navigator TypeScript files are view-model twins, not standalone mounted UI entry points. Existing Composite records surfaceId raster.play.composite and native paint-2d rendering; locate the current host/launcher and execute its actual file controls for runtime acceptance. The historical editor wasm bridge file is comment-only and explicitly records that it was deleted; do not attempt to revive it as a launch route.

## Live Handle Inspection

Read-only /proc cmdline inspection at this audit found no live Raster test, Vitest or nextest command. The only raster-matching process was an unrelated concurrent shell editing the November well-known-artifact ticket. Accordingly none of the historical sessions 18283, 88342, 20371, 8631, 10202 or 8363 was independently confirmed live here. Parent tool handles can still be pending orchestration, so consult their owning tool result before relaunching. No process or lock was modified.

No generated log directory is present at the September ticket root; verification currently contains .nx, nx.json, package.json and project.json. Historical log names in the notes are not present as current ticket-root generated outputs. No logs were fabricated or deleted.
