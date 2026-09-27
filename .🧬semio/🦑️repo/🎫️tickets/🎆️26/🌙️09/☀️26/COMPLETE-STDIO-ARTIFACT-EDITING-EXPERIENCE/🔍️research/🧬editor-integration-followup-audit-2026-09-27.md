# Editor Integration Follow-up Audit

Read-only audit of the shared snapshot-editor implementation on 2026-09-27. This is a source review of the live workspace; no product files changed and no tests were run by this audit. The already-reported React quick suite is outside this audit's validation, and the WGPU Cargo run remained pending.

## Findings

### P1 — An accessibility table edit traps focus after its own successful publish

[`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1080) handles an accessibility `Value` event by immediately publishing with the entry's `target.value` as its base (lines 1088–1095), while retaining `FOCUSED_TABLE_EDITABLE_TEXT`. The retained focus still has the old `base` (lines 1994–2006). Later `Blur`, pointer blur, or a focus transfer calls `commit_focused_table_editable_text_with_engine`, which reuses that old base and the input text (lines 2034–2046). The scene regards a persisted value different from the base as a conflict ([`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4117), lines 4133–4140), and the blur/focus-transfer branches deliberately keep focus on conflict (interpreter lines 1080–1086 and 2068–2072).

Reproduction:

1. Navigate with a screen reader to an editable TableWindowKit cell whose current value is `A` and focus it.
2. Send the accessibility `Value` event `B`; its retained action publishes and the scene refreshes that cell to `B`.
3. Move accessibility focus to another cell or emit `Blur`.

The second commit compares `B` with stale base `A`, reports `Conflict`, and preserves the old focused cell. The user cannot leave that cell through the normal accessibility focus transfer even though the value they supplied was the value that was accepted. Rebase or clear the focused record once its owned value publish is accepted, and cover `Value → scene echo → Blur/focus transfer`.

### P2 — React SourceDraft remains conflicted after the draft matches the collaborator value

[`🧱️elements/✏️TextEditor/🟦️.tsx`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx:331) correctly clears a conflict when reconciliation sees `state.draft === sceneBuffer`. But local typing copies `current.conflicted` unchanged (line 1176). The control row hides Apply while `conflicted` is true and disables Discard for a clean draft (lines 1208–1214).

Reproduction:

1. Begin with persisted source `A`; type local draft `D`.
2. Receive a collaborator scene update `B`; reconciliation preserves `D` and marks it conflicted.
3. Edit the local text to exactly `B`.

The draft becomes clean (`dirty: false`) but remains conflicted. Apply and Discard are both disabled while the conflict alert remains visible. This differs from the WGPU explicit-draft reconciliation, which accepts the exact persisted echo. Recompute `conflicted` during local edits or normalize the draft immediately when it equals the current scene buffer.

### P2 — Details silently makes valid long RFC 6901 paths read-only far below the contract limit

The editing action contract accepts `pathChunks`, combines up to 16 MiB ([`✏️editing/🦀️.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:516), `SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES` at line 604), and publishes the chunk argument in every snapshot mutation definition (lines 579–592). Details, however, declares a path unbindable after more than 256 512-byte `UiText` chunks ([`✏️editing/🪟️details/🦀️.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs:595), lines 599–615), a 131,072-byte ceiling. When unbindable, `detail_node` substitutes a passive `read_only_value` and omits the collection controls (lines 923–944), without a source-editor affordance or explanation.

Reproduction:

1. Open a valid JSON snapshot containing an object key whose RFC 6901 pointer exceeds 131,072 bytes, for example 131,073 ASCII key bytes beneath the root.
2. Open Details for that property or collection.

Its value is shown read-only and its add/remove/move controls disappear, although the same path is valid for the action contract's 16 MiB chunked transport. The Details limit should either be raised to the shared transport capability or expose an explicit source-edit path and an honest user-facing reason that structural editing is unavailable.

## Checked without a new defect

WGPU source-draft publication now uses the retained, paged string transport and its 16 MiB aggregate limit; this audit did not find the former small-source action ceiling in that path. Details numeric scalar editing serializes the displayed number as JSON-encoded text before dispatch, so this source review found no new JavaScript-number precision loss in the current Details controls. Neither observation is runtime validation.

## Leads deliberately left unfiled

The React editable table component appears capable of sending its `onBlur` action after an Enter-triggered action before an authoritative scene echo arrives. This audit did not trace the owning mutation/undo path far enough to prove duplicate persisted commands, so it is not recorded as a finding. WGPU runtime behavior, the text worker's newly added Details creation tests, and media publication caps were not run or claimed here.
