# Shared Snapshot Editing Contract

## Existing seams

Stdio has 88 editor roots. Their persisted write path is `ArtifactEditor::command_from_action` → exact retained tool factory/proof → reducer → the artifact's typed mutation. `TextWindowKit`, `TableWindowKit`, and `TreeWindowKit` only cover whole text, one string cell, and one string node respectively. Most non-text roots only retain `setActiveExample`.

Every `ArtifactEditor::Snapshot` already implements `Clone + PartialEq + ToValue + FromValue + ArtifactDsl`. This is the stable generic boundary for ordinary sized details documents. A candidate `DslValue` tree can be edited independently, then decoded into the concrete snapshot. Decode failure returns an error before any mutation is emitted, so the persisted snapshot is preserved. Large raster, media, mesh, and archive roots need domain-specific admission and mutation emitters because converting their complete snapshot into one `DslValue` is not a bounded first step.

The host merges a declarative control's reading into authored action arguments under `value`. Controls and palette/API calls use that canonical typed argument. Missing or malformed arguments are rejected.

## Frozen protocol

The domain module is `semio_s_artifact_stdio_contract::editing`, stored under `✏️editing`.

The six actions are `setSnapshotValue`, `insertSnapshotValue`, `removeSnapshotValue`, `moveSnapshotValue`, `renameSnapshotKey`, and `replaceSnapshotSource`.

Paths use strict RFC 6901 JSON Pointer syntax. The typed event variants are `SetValue`, `InsertValue`, `RemoveValue`, `MoveValue`, `RenameKey`, and `ReplaceSource`. `SetValue` requires an existing target. `InsertValue` inserts a new object key or an array element, including `/-` append; an existing object key is rejected so insertion cannot silently replace. `MoveValue` removes first and resolves its destination against the resulting document. Root replacement is supported. Root removal, root insertion, root movement, and root rename are rejected.

The complete candidate is decoded with `FromValue` after every structural edit and compared with its re-encoded typed value while ignoring object order. This validates scalar widths, optionals, enum tags, required fields, unknown fields, duplicate keys, finite numbers, and the artifact's owned snapshot shape without lossy numeric widening. `ReplaceSource` uses strict, lossless typed snapshot JSON for every format, including binary formats whose native DSL is not a lossless snapshot encoding. It never falls back to `Default`.

## Retained integration

The shared command wrapper is `SnapshotEditingCommand<C>::Native(C) | Edit(SnapshotEditEvent)`. Its text codec frames the native binary codec, so a native command only needs `OpBinary`. `SnapshotEditingEditor` requires the owner to extract an event, admit its bounded work from the concrete snapshot, and emit its artifact-owned mutation. The shared retained factory owns the six editing routes, their artifact publication contracts, their wire bound, and exact proof rows. The ordinary `snapshot_edit_value_is_admitted` helper limits dynamic snapshots to 65,536 nodes. Large roots must provide a cheap domain-specific admission and direct emit rather than claiming whole-snapshot conversion as bounded.

Roots retain event-sourced history and framework undo/redo because no helper mutates a store or loads a replacement document directly.

## Verification

The language-neutral fixture applies scalar, map, sequence, optional, enum, large integer, escaped pointer, movement, prototype-sensitive object key, and rejection cases. Rust consumes the fixture and compares successful structural operations with the third-party `json-patch` crate before concrete snapshot validation. Rejection cases assert that the original typed snapshot is unchanged. The TypeScript twin consumes the same fixture through an explicit artifact codec callback; source replacement requires that callback and cannot fall back to generic JSON parsing. Its Nx target passed 22 cases on 2026-09-26.

## Explicit Draft and Editable Table Milestone

`TextDraftView` carries a unique surface ID, prepopulated text, explicit Apply/Discard labels, a localized conflict label, the target action/argument, and static action arguments. React and WGPU preserve local drafts across ordinary rerenders, keep refused drafts visible, route undo/redo into the focused local text buffer, and block Apply when the persisted base changed. `TextWindowKit::render_read_only` gives readonly long values distinct scene identities.

`TableWindowKit::render_editable(view, controller_id, columns)` emits `editableText` cells with stable displayed row/column ordinals and artifact-owned actions. React renders a native multiline textarea: Enter and blur commit, Shift+Enter inserts a newline, Escape cancels, clipboard/IME use the browser control, and a changed persisted base marks a conflict without replacing the draft. WGPU parses the same cell wire shape, paints it with the native input widget, and routes focus through the retained Table component bridge because the outer component surface intentionally shadows paint-time cell hits. The bridge re-resolves row and column against the accepted live scene before commit, merges the draft as `value`, blocks stale-base commits, handles Enter/Shift+Enter/Escape, and publishes virtual multiline textbox accessibility nodes.

### Edited-file ledger

- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧫️fixtures/🪆️snapshot-edits/🔣️patch-cases.json`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🟦️.test.ts`
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-component-ui-value-round-trip/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧬️schema/📝️explicit-draft/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧫️fixtures/📝️explicit-draft/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/📝️explicit-draft/🟦️.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🧬️schema/✏️editable-text/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🧫️fixtures/✏️editable-text/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🧪️tests/✏️editable-text/🟦️.test.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-table/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚡️quick/🟦️.ts`

### Validation ledger

- React focused command: `NX_DAEMON=false NX_PLUGIN_NO_TIMEOUTS=true NX_WORKSPACE_DATA_DIRECTORY='<ticket>/🗑️generated/nx-data' bun nx run @semio-tech/framework-renderer-react:test -- quick --run --testNamePattern='table editable text cells|text editor explicit drafts'`.
- The first run reached all focused tests: three passed and one table blur test exposed an Enter suppression bug. After fixing it, the second run reached four focused tests and only failed because this suite does not install the `toHaveValue` matcher; the assertion now reads the native textarea value directly. The final rerun is recorded after completion below.
- WGPU focused command: `NX_DAEMON=false NX_PLUGIN_NO_TIMEOUTS=true NX_WORKSPACE_DATA_DIRECTORY='<ticket>/🗑️generated/nx-data' bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit -- --no-fail-fast -E 'test(text_editor_explicit_draft_preserves_local_text_until_apply_or_discard)'`.
- That WGPU run did not compile the renderer. It stopped in concurrent stdio XML/PDF dependencies with `SetSnapshot: MutationLeaf` and missing generated source-authority files. No WGPU result is claimed until those format-owned errors clear and the expanded editable-table focus tests run.
- Final React rerun: one file passed, four focused tests passed, twelve unrelated tests skipped; Nx exited successfully in 16.3 seconds.

## Retained Action Transport Follow-up

WGPU's bounded action ring admits at most 4 KiB per string and 16 KiB per action item. Serializing an explicit source draft into that ring made ordinary 64 KiB typed snapshots impossible to apply even though the artifact command itself admitted them. The shared WGPU action layer now owns `RetainedStringAction`: it validates the fixed descriptor and total extent up front, copies at most one 4 KiB UTF-8 page per step, exposes progress, accepts cancellation while copying, and yields one atomic `QueuedActionDescriptor` only after the complete value exists. The frame action ledger owns that descriptor directly, so the bounded input ring never receives the large string.

Explicit drafts of every size use this lane. Apply preserves the local draft until the artifact receipt accepts it, refused/cancelled receipts leave the draft and a localized visible error, and the button only advertises Cancel while the copy can still be cancelled. Ordinary editable `TextWindowKit` documents use the same transport for values over 4 KiB; the final `textEdit` and `textSelect` enter the frame ledger as an atomic ordered pair with correlated receipts. Editable table cells publish through the retained input lane. Their native focused accessibility projection now follows the current local draft instead of announcing the stale persisted base.

The action contract also exposes `pathChunks` and `fromChunks` as first-class `List<Text>` alternatives when an RFC 6901 pointer exceeds one `UiText`. The parser requires exactly one direct or chunked representation, joins chunks without transcoding, bounds the combined pointer to 16 MiB, and applies the same strict pointer decoder afterward.

### Follow-up validation

- `@semio-tech/ui-rs:test-quick -- retained_string_action` reached the shared UI crate but failed in unrelated concurrently edited WGPU module imports before the retained action unit mounted; no passing result is claimed.
- `@semio-tech/framework-renderer-react:test-quick -- TextEditor` was an invalid Vitest file filter and found no test files; it is recorded as a command-shape failure, not a product failure.
- `@semio-tech/framework-renderer-react:test-quick` passed one file and all 16 tests in 14.5 seconds.
- The focused renderer WGPU law remains in progress while concurrent Cargo graph work holds the shared build queue.

The running WGPU command is session `50192`:

`NX_DAEMON=false NX_PLUGIN_NO_TIMEOUTS=true NX_WORKSPACE_DATA_DIRECTORY='<ticket>/🗑️generated/nx-data' bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit -- text_editor_large_explicit_draft_pages_cancel_and_refusal_without_losing_text`

It has emitted no compiler diagnostic yet. Two renderer Cargo jobs were waiting concurrently (`cargo-nextest`/Cargo process pairs 62262/62299 and 63302/63376 at the last observation), so this is a pending result rather than a known source failure. No standalone log file exists because the Nx process remains attached to the exec session. The earlier invalid WGPU `--exact` invocation and invalid React `TextEditor` file filter both completed and are not running.

### Follow-up edited-file ledger

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎬️action/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪢️text-edit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-action-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📦️retained-string-action/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📦️retained-string-action/🧬️schema.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-table/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🦀️.rs`

## Follow-up Interaction Audit Fixes

The React explicit-draft state transition now recomputes conflict status on every local edit. When a local draft becomes byte-equal to the collaborator's current persisted text, it rebases the draft to that value and clears both `dirty` and `conflicted`; Apply and the conflict alert can no longer remain latched around a clean draft. The language-neutral explicit-draft fixture carries the `A` / local `D` / collaborator `B` / matching `B` sequence.

WGPU editable-table focus now owns both its original persisted base and its current draft. Commit resolution first recognizes `draft == persisted` as the accepted authoritative echo, then detects a true collaborator conflict, and otherwise starts a retryable retained publication. Pointer blur, accessibility blur, and focus transfer release the cell after an accepted echo. Refused or cancelled publication keeps the draft and cell focus; a focus transfer also waits while the retryable publication remains pending. The shared editable-table fixture describes accepted, refused, and collaborator outcomes and is consumed by both React reconciliation tests and WGPU scene/runtime tests.

Details paths that exceed the flat `List<Text>` carrier no longer silently become readonly. The coordinated Details implementation renders the provider's complete typed snapshot as an explicit Apply/Discard source draft routed through `replaceSnapshotSource`; a lazy provider with neither a bindable path nor source returns an assembly error. Its 160 KiB Unicode-key regression checks that the resulting scene remains editable and explicitly committed.

### Follow-up validation status

- React quick run attempts and their exact output are in `🗑️generated/react-explicit-conflict-rerun.log`, `react-explicit-conflict-final.log`, and `react-explicit-conflict-final2.log`. The first two exposed incomplete test-host session mocks; the third proved the full host harness was unsuitable in this jsdom suite because the asynchronous WASM loader was not ready. That harness was removed. The stable pure state-transition regression remains in the normal quick suite; `react-explicit-conflict-green.log` is the active/final rerun log.
- The WGPU runtime test is queued behind three already-running renderer Cargo/Nx jobs. No duplicate Cargo job was started. It covers accepted echo → blur, refused publication → cancellation with draft retention, and refused focus transfer.
- The Details 160 KiB pointer fallback test is queued in the stdio contract run owned by the Details integrator.

### Follow-up edited-file ledger

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧫️fixtures/📝️explicit-draft/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/📝️explicit-draft/🟦️.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🧫️fixtures/✏️editable-text/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🧪️tests/✏️editable-text/🟦️.test.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-table/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`
