# Generation3D Preview Window Transient Ownership

## Result

The edit-mode computed flow evaluation is owned by the exact registered `procedural-preview` window. `Generation3dConfig` no longer contains `preview_eval_text`, and `Generation3dConfigMutation` no longer exposes `SetPreviewEval`.

The generate-mode preview remains independently app-transient as `Generation3dTransient::generation_preview_text`.

## Authority and publication

- `Generation3dFlowEvalWindowWork` accepts `flowEvalTick` only when the retained operation captured both a `ViewModel` and a `WindowTransientSnapshot`.
- The selected `ViewModel.window_id` must equal the snapshot window id.
- The snapshot must have registered kind `procedural-preview` and contain `Generation3dPreviewWindowTransientOwner`.
- Completion publishes one `CompleteWithEphemeral` mutation addressed from the captured snapshot. No payload window or surface id is accepted.
- Rendering reads `preview_eval_text` only from the addressed window owner.

## Retained ownership

`Generation3dPreviewWindowTransientOwner` uses `store::ArtifactEphemeralTransferPreparationFactory` with direct owned transfer. Admission charges `size_of::<Generation3dPreviewWindowTransient>() + String::capacity()`; a short string with retained capacity above the Store maximum rejects while returning the exact allocation owner. State and mutation disposal use explicit `RetireOwned` cursors. Tests cover an accepted 64 KiB logical value under one-byte retirement grants and the rejected oversized reserved-capacity owner under the same grants.

## Schema and oracle

The window transient has Rust, JSON Schema, TypeScript, GraphQL, and proto facets. JSON Schema classifies `previewEvalText` as `ephemeral-local-window`. The app config facets omit the field.

The permanent language-neutral test compares the production TypeScript parser with Ajv for two accepted and two rejected cases, rejects a payload-owned `windowId`, and confirms the app config schema rejects `previewEvalText`.

- Ticket oracle: `🗑️generated/generation3d-preview-window-transient-oracle-2.log` — terminal green.
- Root permanent handler: `🗑️generated/generation3d-preview-window-transient-root-oracle-2.log` — terminal green.
- Earlier red oracle: `🗑️generated/generation3d-preview-window-transient-oracle-1.log` — Ajv lacked the referenced artifact schema; the final oracle explicitly registers the canonical artifact schema.
- Native validation was not launched because the parent owns the shared Cargo queue.

## Registered commands

- Root Nx targets: `workspace:generation3d-preview-window-transient`, `workspace:generation3d-preview-window-transient-native`.
- Ticket Nx targets with the same suffixes.
- Root permanent command: `bun ./📜️script.ts verify generation3d-preview-window-transient [native]`.
- Ticket permanent command: `bun ./📜️script.ts generation3d-preview-window-transient [native]`.
- Launch orders: 311.164 and 311.165 in both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`.
- Native command uses ticket `🗑️generated/cargo-trinity`, `CARGO_INCREMENTAL=0`, package `semio-s-artifact-procedural-generation3d`, feature `component-app-assembly`, and test filter `preview_eval_`.

## Exact file ledger

Created:

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🦀️.rs`
- `.../🫧️transient/🧬️schema/🔣️.json`
- `.../🫧️transient/🧬️schema/🟦️.ts`
- `.../🫧️transient/🧬️schema/🔗️.graphql`
- `.../🫧️transient/🧬️schema/🛰️.proto`
- `.../🫧️transient/🧬️schema/🦀️.rs`
- `.../🫧️transient/🧫️fixtures/🔬️unit/🔣️.json`
- `.../🫧️transient/🧪️tests/🔬️contract/🟦️.ts`
- `.../🫧️transient/🧪️tests/🔬️unit/🦀️.rs`

Removed:

- `.../👁️preview/🫧️transient/📌️.empty.md`

Updated:

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `.../✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `.../✏️editor/🧪️tests/🔬️testkit/🦀️.rs`
- `.../✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs`
- `.../✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`
- `.../✏️editor/🎚️config/🦀️.rs`
- `.../✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `.../✏️editor/🎚️config/🧬️schema/🔣️.json`
- `.../✏️editor/🎚️config/🧬️schema/🟦️.ts`
- `.../✏️editor/🎚️config/🧬️schema/🔗️.graphql`
- `.../✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `.../✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `📜️script.ts`
- `📋️project.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- ticket `validation/📜️script.ts`
- ticket `validation/project.json`

