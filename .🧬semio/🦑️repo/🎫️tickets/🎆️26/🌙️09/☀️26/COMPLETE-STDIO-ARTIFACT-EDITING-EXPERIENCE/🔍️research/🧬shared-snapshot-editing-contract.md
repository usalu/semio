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

## Validated Dispatch and Bounded Admission Audit

The final dispatch graph has one validation boundary. All 88 `SnapshotEditingEditor` implementations expose their format-specific publication through `snapshot_edit_mutations`; the only call to that raw hook is the default `snapshot_edit_emit`. Direct editor handles route snapshot commands through that default method. Retained factories recognize the six shared action IDs before their native routes, verify the event action matches the requested tool, and reduce through the same default method. The two macro-supplied native factories and every explicit native factory also compare the decoded command ID with the requested tool ID. No override, recursive call, or alternate raw dispatch path remains.

Retained admission no longer converts the complete snapshot to `DslValue`. It first bounds the encoded incoming event to 16 MiB, then enforces 65,536 incoming value nodes, 128 nested levels, unique object keys, finite numbers, valid RFC 6901 escapes, and 128 pointer segments. It uses `value_shape_at_path` only for the addressed value or parent. Source replacement validates duplicate keys and parses only its submitted source. The neutral compact-patch fixture now includes the admission contract, and its Rust regression admits a metadata edit beside a 2 MiB byte sibling while refusing excess raw bytes, nodes, depth, path segments, malformed pointers, duplicate values, and duplicate source keys.

No new Cargo run was started because multiple retained native lanes remain active. The WGPU awaiting-echo run is still process `30640` / `34180` with log `🗑️generated/wgpu-table-awaiting-echo.log`; it remains in nextest discovery with no compiler or test result. The empty-enum derive run is still process `43179` / `43896` with log `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/🗑️generated/raster-adjustment-value-derive-2.log`; it also remains in nextest discovery with no diagnostic.

### Admission edited-file ledger

- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🧪️tests/🦀️.rs`

## Direct Typed-Path Value Codec

The domain-neutral value codec now exposes three bounded readers and one atomic writer. `value_at_path` projects only the requested subtree, `value_shape_at_path` reports scalar kind or container length without projecting children, `value_key_at_path` returns one serialized object key by ordinal, and `edit_value_at_path` applies `Set`, `Insert`, or `Remove` to already-decoded RFC 6901 segments. Array reads accept canonical decimal indices; only insert accepts `-`. Missing keys, duplicate insertions, noncanonical indices, fixed-container length changes, type errors, and custom-codec refusals return `ValueError` before the owning field is assigned.

Container implementations traverse vectors, deques, arrays, tuples, boxes, options, sets, ordered maps, hash maps, and `DslValue` directly. Generated records honor wire renames, skip predicates, flattened fields, and paired custom codecs. Generated external, adjacent, and internal enum representations traverse their active payload directly, including tag/content keys and internally tagged object payloads. A custom serialized composite without the matching decoder is explicitly uneditable below its wire field.

The neutral fixture is shared with the TypeScript fast-json-patch/Ajv suite. The native derive integration consumes its 14 accepted cases, eight rejected cases, custom codec cases, all three enum tagging modes, flattening, Unicode/escaped/empty keys, canonical indices, atomic refusal, and the 2 MiB unchanged-sibling witness. It also checks object-key paging and enum shape queries. Public `ValueEdit` and `ValueShape` reexports are available from protocol value, OS DSL schema, and the OS kernel crate root.

### Direct-path validation status

- TypeScript fixture/oracle result owned by the coordinator: 50 tests / 102 assertions passed before compact-patch additions; the compact-patch suite later passed 64 tests / 150 assertions.
- Native focused command: `NX_DAEMON=false NX_PLUGIN_NO_TIMEOUTS=true NX_WORKSPACE_DATA_DIRECTORY='<ticket>/🗑️generated/nx-data' bun nx run @semio-tech/value-derive-rs:test -- --test typed_path`.
- Its redirected output is `🗑️generated/value-typed-path-native.log`. Nextest run `27607ff8-c41b-4a6a-a547-950d9b2c04f8` passed all 7 tests in 35 ms; Nx completed successfully after 20m37s of shared queue and build time. The regressions include fields named `path`, `index`, and `edit`, a `PathBuf` capture witness, and a second decode trait with a colliding `from_value` method.
- A later shipping WASM compilation exposed zero-variant enums as a derive edge case: matching `self` left `&EmptyEnum` and `&mut EmptyEnum` inhabited even though the enum itself is uninhabited. The generator now emits exhaustive `match *self {}` bodies for the root encoder, all three path readers, and the path writer. Tagged and untagged empty-enum derives are compile-law fixtures; decode still returns a normal unknown-variant error. A standalone Rust compile proves the shared and mutable uninhabited match forms. A pre-existing focused Nx run is compiling `typed_path` and `deny_unknown_fields_enums` together as processes `43179` / `43896`; its log is `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/🗑️generated/raster-adjustment-value-derive-2.log`. It remains queued in Cargo with no source diagnostic. The coordinator's shipping component run is independently compiling the patched derive and had progressed through `semio-framework-ui-contract` without the former E0004 diagnostic when this worker handed off; its log is `🗑️generated/full-catalog-component-current-2.log`. The earlier 7/7 result predates this edge-case regression, so neither active run is reported as passing yet.

### Direct-path edited-file ledger

- `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧪️tests/🧭️typed-path/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs`
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

WGPU editable-table focus now owns its original persisted base, current draft, and one explicit value awaiting its scene echo. A successful retained publication enters `AwaitingEcho`; accessibility exposes the textbox as busy, and pointer blur, accessibility blur, Enter, or focus transfer cannot publish that value again while the accepted scene still shows the original base. An authoritative echo rebases the editor and releases it, a later draft can publish from the echoed base, and a different collaborator value remains a conflict. A publication fault never enters the awaiting state, so its visible fault and local draft remain retryable. The shared editable-table fixture describes accepted, refused, pending-echo, and collaborator outcomes. The WGPU runtime law keeps the same focused cell through `Focus(A) → Value(B) → Blur before echo → scene echo(B) → Blur`, asserting one publication before the echo, none after it, busy projection while pending, retained draft on cancellation, and final focus release.

Details paths that exceed the flat `List<Text>` carrier no longer silently become readonly. The coordinated Details implementation renders the provider's complete typed snapshot as an explicit Apply/Discard source draft routed through `replaceSnapshotSource`; a lazy provider with neither a bindable path nor source returns an assembly error. Its 160 KiB Unicode-key regression checks that the resulting scene remains editable and explicitly committed.

### Follow-up validation status

- React quick run attempts and their exact output are in `🗑️generated/react-explicit-conflict-rerun.log`, `react-explicit-conflict-final.log`, and `react-explicit-conflict-final2.log`. The first two exposed incomplete test-host session mocks; the third proved the full host harness was unsuitable in this jsdom suite because the asynchronous WASM loader was not ready. That harness was removed. The stable pure state-transition regression remains in the normal quick suite; `react-explicit-conflict-green.log` is the active/final rerun log.
- The WGPU focused command is session `77139`, redirected to `🗑️generated/wgpu-table-awaiting-echo.log`. It covers the same focused cell through accessibility `Focus(A) → Value(B) → Blur before echo → scene echo(B) → Blur`, with exactly one publication, busy awaiting state, authoritative echo rebase and focus release, cancellation/draft retention, focus transfer suppression, and the scene-level pending/echo transitions. The session is alive inside `bun ./📜️script.ts test-wgpu-unit`; it has no compiler diagnostic and remains behind the broad renderer Cargo lane. Bun emitted its known `NO_COLOR` / `FORCE_COLOR` `getColorDepth` warning before Nx entered the task; this did not terminate or fail the invocation.
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


## Office Primary Document Editing

The shared document window now declares an honest `set-page` command with four required typed arguments: non-negative integer `page`, non-negative integer `item`, `revision`, and empty-string-capable `text`. Its editable renderer creates a unique, prefilled `TextDraftView` for every writable item. Apply is explicit; Discard remains local; stale revisions show a localized conflict; retained execution exposes Applying, Cancel, and failure labels in English and German. The draft revision is a deterministic FNV-1a digest of the exact UTF-8 persisted text.

DOCX renders one draft for each paragraph block and addresses the original body index. Applying preserves the paragraph and run records, writes the new text into the first run, and empties later run text while retaining their formatting. Tables are not presented as a lossy primary text edit; their full structure stays editable in Details. PPTX renders one draft for each text-bearing shape, addresses slide and shape ordinals, preserves paragraph/run records where lines correspond, adds typed paragraphs for new lines, and removes only text paragraphs beyond the submitted line count. PDF renders one draft per page and replaces its faithful Unicode text projection through `SetPageContent`: the first Unicode text operand carries the submitted text, later Unicode text operands become empty, encoded text operands and every non-text operation remain intact, and a page without Unicode text receives a bounded text object only when the submitted text is nonempty. Missing addresses, unsupported targets, nonzero PDF/DOCX item addresses, and stale revisions return faults without a mutation.

Native primary actions share one retained route. `BoundedNativeEditingEditor` supplies exact tool IDs, publication contracts, payload schema, admission extent, and native mutation reduction. `BoundedNativeEditToolJobFactory<E>` validates command/tool identity, bounds and pages the wire command, and publishes only declared lanes. Adding `bounded_native: true` to `snapshot_details_editor_support!` mounts its proof, factory, and builder alongside the six shared Details actions. All 16 DOCX/PPTX/PDF editor roots roster and mount `set-page`; XLSX, EPW, and ZIP also consume this surface.

The first native helper incorrectly equated two durable edit rows with two units of computation. `BoundedArtifactCommandWork::step` called the complete reducer once, while the generic `BoundedConfigPreparation::advance` called `Mutation::inverse`, `Mutation::diff`, `MutationDiff::apply`, and `prepare_one_item` in one scheduler grant. The Store's `ArtifactStoreOneItemSealer` pages canonical encoding only after the inverse and post snapshot already exist. It therefore cannot make an artifact-sized clone or scan cooperative.

The route now exposes both real retained seams without changing existing call sites. `BoundedNativeEditingEditor::native_edit_work` supplies an editor-owned `ArtifactCommandWork<EditorApp<Self>>`; the macro accepts an optional `work:` builder. `native_edit_preparation_route` supplies an exact mutation predicate plus an app-owned `ArtifactStoreOneItemPreparationFactory`; the macro accepts an optional `preparation_route:` builder. `snapshot_details_editor_support!` installs a predicate router in front of the shared snapshot preparation factory. A recognized native mutation stays on its native factory even when preflight or begin refuses it, so an invalid or oversized native operation cannot bypass domain validation through the fallback. Unrecognized shared snapshot mutations continue through the fallback.

The proven model is Flow's phased preparation: retain the immutable base and mutation owners, advance domain hashing/copy/recipe work one semantic unit per grant, build the exact inverse and post root, then call `begin_one_item_seal`; cancellation and close retire every partial owner through bounded cursors. Stdio format owners can now implement that same model behind the shared route. Existing DOCX/PPTX/PDF snapshots still own nested `Vec`, `String`, OPC, and operation trees directly. Applying their diffs clones those aggregate owners, so merely splitting inverse, diff, and apply into three scheduler calls would remain false boundedness. A truthful office preparation needs either domain cursors for every nested owner or a model refactor to chunked/shared immutable owners. The current generic fallback remains valid only for mutations whose inverse, diff, and apply are intrinsically bounded independently of snapshot size; the new native hooks must be mounted before claiming cooperative large-document publication for the office routes.

Primary document commands now reject missing text instead of silently clearing content. Stale revisions are checked before no-op detection; an identical submitted draft produces no mutation or history row. The `set-page` action remains registered for its prefilled draft controls but is hidden from the ordinary command palette because its revision token and stable address are renderer-supplied concurrency data. Representative DOCX, PPTX, and PDF host laws invoke the registered retained action, verify one publication, stale refusal with snapshot preservation, an identical no-op with no extra history, and undo/redo restoration. These native laws remain pending Rust compilation behind the shared graph.

The language-neutral editable-document fixture carries English and German Unicode drafts, exact addresses, revisions, and labels. Its Rust scene law checks the action metadata and rendered draft settings. Ajv 2020 independently validates the fixture against its language-neutral schema, and the TypeScript implementation computes the same revision and draft descriptor from the validated vectors.

### Office validation status

- `NX_DAEMON=false NX_PLUGIN_NO_TIMEOUTS=true NX_WORKSPACE_DATA_DIRECTORY='<ticket>/🗑️generated/nx-data' bun nx run @semio-tech/plugin-window-kits:test` passed 6 files and 9 tests. Log: `🗑️generated/document-window-kit-ts-test.log`.
- `NX_DAEMON=false NX_PLUGIN_NO_TIMEOUTS=true NX_WORKSPACE_DATA_DIRECTORY='<ticket>/🗑️generated/nx-data' bun nx run @semio-tech/plugin-window-kits:typecheck` passed. Log: `🗑️generated/document-window-kit-ts-typecheck.log`.
- No new Cargo job was launched because broad native/WASM/catalog checks were already active. The shared retained factory and 16 office mounts therefore remain pending native compilation/runtime confirmation.
- `@semio-tech/stdio-snapshot-editing-js:test` passed 78 tests / 238 assertions after adding the retained native route/cancellation fixture and its independent Ajv oracle. Log: `🗑️generated/retained-native-route-ts.log`.
- Focused native command: `NX_DAEMON=false NX_PLUGIN_NO_TIMEOUTS=true NX_WORKSPACE_DATA_DIRECTORY='<ticket>/🗑️generated/nx-data' bun nx run @semio-tech/stdio-artifact-contract-rs:test -- --lib retained_native_route_refuses_without_fallback_and_lifecycle_is_cancelable`. Log: `🗑️generated/retained-native-route-rust.log`; the run ended after 50.2 seconds with two route-test inference diagnostics and one location-stripped borrowed-value diagnostic. Explicit preparation-factory trait-object annotations repair the two inference sites. The borrowed-value diagnostic awaits the already-running shared native compile for a source location, so no native pass is claimed.
- The next full component run reached malformed hand-authored escaping in four DOCX/PPTX sibling command codecs. All six DOCX/PPTX facets now use the shared structured JSON `impl_serde_op_codec!` path, two malformed PPTX newline literals were repaired, and `rustfmt --edition 2021 --check` parses all 16 DOCX/PPTX/PDF editor roots. Runtime compilation remains pending the coordinator's warm retry.
- The coordinator's full component run `full-catalog-component-current-5.log` stopped after 15m20s in unrelated PNG imports (`InteractiveJobCloseStep` and inaccessible `patch_pixel_region`) before it could validate the final office/shared-native edits.

### Office edited-file ledger

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-window-kits/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document/🧪️tests/🧪️renderdocument/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document/🧫️fixtures/✏️editable/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document/🧫️fixtures/✏️editable/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧫️fixtures/🧵️retained-native/{🧬️schema/🔣️.json,🔣️.json}`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/{🦀️.rs,🟦️.test.ts}`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/{🧱️base,📏️strict,🔄️transitional}/✏️editor/🦀️.rs` and their primary-window Rust modules/tests
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/{🧱️base,🔒️strict,🌉️transitional}/✏️editor/🦀️.rs` and their primary-window Rust modules/tests
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/{4️⃣1.4,7️⃣1.7}/🪆️subsets/*/✏️editor/🦀️.rs` and their primary-window Rust modules/tests
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`

## Retained Native Publication Preparation

The base DOCX `set-page` route now uses two independent cooperative phases. `DocxSetPageWork` checks the paragraph revision and equality in 4 KiB UTF-8 byte pages, then copies the submitted Unicode text through `RetainedTextCopy`. The Store preparation route pages the inverse text and every owned field of the post snapshot, including OPC part byte payloads, before beginning the canonical one-item seal. Cancellation retires partial strings and byte buffers in bounded grants; it does not iterate one work item per byte. A language-neutral fixture carries a 2 MiB unchanged sibling, Unicode replacement, page budget, cancellation point, and expected digest. The TypeScript implementation validates it with Ajv and Node's independent SHA-256 implementation.

The first focused native DOCX attempt exposed unrelated concurrent framework compilation, then the shipping component located an invariant lifetime in the borrowed canonical mutation encoder. `canonical_index` and the static `setRunText` discriminator now shorten into the borrowed root lifetime. The second focused attempt again stopped before DOCX at the concurrently introduced move-only tree-action argument type; that owner removed the invalid `Clone` derive. The third warm focused run is active as session `26009`, redirected to `🗑️generated/docx-retained-preparation-native-3.log`. No native DOCX pass is claimed until this run executes the two `post_copy` laws and a subsequent registered action filter executes publication, stale refusal, no-op history, undo, and redo.

The language-neutral retained-native TypeScript suite passed 81 tests and 279 assertions. Its exact log is `🗑️generated/retained-native-document-ts.log`.

## Semio Vertex Native Route Follow-up

The only two editor roots whose native command rosters lacked factories were Semio Mesh and Semio B-rep. Both now mount the shared native factory and an editor-owned retained work cursor. B-rep resolves exactly one vertex record per replay step. Mesh resolves exactly one mesh or primitive record per replay step, then validates the addressed vertex. Identical points complete without a mutation; missing targets fault without publication; localized replay previews are available in English and German. Registered host-action laws cover publication, missing-target refusal, snapshot preservation, undo, and redo.

The B-rep binary decoder now derives the vertex-id byte length with checked subtraction from the actual frame length. A forged `u32::MAX` prefix over a short frame returns `Malformed` on 32-bit and 64-bit targets without overflowing address arithmetic. Its unit regression constructs that frame without a large allocation.

These Semio source laws remain pending native execution. Their command work is cooperatively bounded, while their current generic Store preparation still derives and applies the sparse diff through aggregate-owned vectors. Large-model Store publication therefore remains an explicit unproven gap until the roots add format-specific paged post-snapshot preparation comparable to DOCX; the factory mount is not reported as complete large-model boundedness.

### Retained-native checkpoint handoff

The shipping component build `🗑️generated/full-catalog-component-current-12.log` compiled `semio-s-artifact-stdio-docx` after the canonical lifetime repair and advanced to PPTX. This proves the DOCX library compiles in the shipping component graph; it does not execute the focused Store preparation laws.

The focused DOCX command remains alive in exec session `26009` and Cargo processes `47567` / `47578`, with no rustc child while another logical task owns the active native lane. Its log is `🗑️generated/docx-retained-preparation-native-3.log`. Preserve that process. The intended next filters after it passes are `registered_page_draft` and `retained_page_edit`; neither has been launched or reported as passing.

The Semio Mesh/B-rep source checkpoint is syntax- and rustfmt-clean. The check log is `🗑️generated/semio-native-route-rustfmt-check.log`. Their registered action laws have not executed. The full component build had not yet reached Semio when this checkpoint was written.

### Structural retained preparation checkpoint

The retained-native wire admission now measures the exact `OpBinary` command before a job is constructed and rejects malformed or oversized encodings with stable faults. `RetainedTextCopy` no longer constructs a `String` from unchecked partial UTF-8 bytes; partial pages remain bytes until a complete checked conversion. The language-neutral retained-native fixture covers split multibyte code points, accepted and refused command extents, a 2 MiB untouched sibling, large Mesh/B-rep structural collections, and cancellation points. Its Rust and TypeScript laws have been authored; the extended laws have not yet executed in the current queue.

The base DOCX preparation replaced aggregate paragraph/table cloning turns with a recursive `DocxBlockCopy` cursor. It pages run text, table rows/cells, nested blocks, XML attributes/children, and cancellation retirement with a depth limit. Laws cover 4,096 runs, 1,024 rows, early cancellation, and deep cancellation. The focused DOCX process remains queued in Cargo and has not reached these assertions.

Semio now has a shared structural Store preparation driver and two domain cursors. Mesh pages schema/id/material strings, numeric vertex/index buffers, colors, UVs, and texture bytes. B-rep pages vertex/edge/loop/face/shell/solid/coedge strings plus NURBS control points, weights, and knots. Both count the complete target collection and require exactly one target before sealing, retain an exact inverse, and retire partial owners on cancellation. Their direct reducers use the same exact-target helper, so missing or duplicate targets do not emit history. The large laws use 8,192 vertices, a 2 MiB Mesh texture, and a 4,096-point B-rep NURBS payload.

The live shipping component build `🗑️generated/full-catalog-component-current-12.log` cannot validate these structural preparation modules. Its Semio rlib was written at `2026-09-27 23:36:55`, while the shared structural preparation source was written at `23:48:54` and the Mesh/B-rep domain cursors at `23:58:39`. The later plugin link reused that older Semio rlib. The current structural source is rustfmt-parse-clean only; a fresh focused Semio compile and its unit/runtime laws are required. No duplicate native job was launched while the existing component and DOCX jobs remained active.

The independent TypeScript/Ajv run `@semio-tech/stdio-snapshot-editing-js:test --excludeTaskDependencies` passed 82 tests with 305 assertions in 8.7 seconds, including the retained-native structural oracle. Log: `🗑️generated/retained-native-structural-ts.log`. The fresh Rust command `@semio-tech/stdio-semio-rs:test --excludeTaskDependencies -- --lib structural_copy -- --nocapture` is active as exec session `93040`, with log `🗑️generated/semio-structural-preparation-native-1.log`; it has entered the artifact test runner and remains queued in Cargo, so no Rust result is claimed.

Real wrapped DOCX, Mesh, and B-rep command laws now encode the complete command, admit its exact byte length, and reject a limit one byte lower. These exercise DOCX revision plus text, both Mesh target identifiers, and the B-rep vertex identifier rather than an abstract surrogate command. They are rustfmt-clean and pending native execution. The earlier DOCX registered-route process ended after 54m37s in the already-repaired upstream table-label diagnostics before its assertion. Its fresh retry is exec session `94934`, log `🗑️generated/docx-registered-route-native-2.log`, filtered to `registered_page_draft`; no result is claimed yet.

Additional scoped files:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/📬️preparation/🧱️structure/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/✏️editor/📬️preparation/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/📬️preparation/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/📬️preparation/🦀️.rs`

Current scoped files:

- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧫️fixtures/🧵️retained-native/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧫️fixtures/🧵️retained-native/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🟦️.test.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
