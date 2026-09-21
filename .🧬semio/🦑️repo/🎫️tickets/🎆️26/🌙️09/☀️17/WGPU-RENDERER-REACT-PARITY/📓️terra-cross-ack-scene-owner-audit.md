# Cross-ACK Scene Owner Audit

## Scope and evidence

Read-only source audit after the host-ID rollout. No build or browser run was performed. The native law below is newly registered and has no pass receipt in this report.

The current renderer law is valid fail-first construction: `an_accepted_sibling_insertion_rebases_the_exact_renderer_scene_capture` builds document records `A`, then `A+B`, claims `B`, then accepts `A+C+B`. It asserts both that the preserved `B` host identity is the same and that the arena `NodeId` differs before requiring captured and released ownership to name the accepted `B`. See `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:31` through `:80`.

The protocol reverse half is now bounded: `UiTree::document_id` reads a node-owned protocol `UiNodeId` in O(1), then `document_node` does the candidate binding binary search in O(log n). `Ui::candidate_scene_node_for_presented_node` is properly witness-gated and scene-only at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1136` through `:1146`; its `UiTree` primitives are at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs:669` through `:675`. It is not an O(1) whole mapping.

## Confirmed defects

### Focused TextEditor loses focus after a valid accepted sibling insertion

`FocusedTextEditor` retains `(window_id, NodeId, host_id)` at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1510` through `:1521`. On the next key, `apply_focused_text_editor_key` requires the old `NodeId` in the now-accepted tree at `:1532` through `:1562`. Alternating-arena acceptance makes that lookup fail even when the exact keyed TextEditor and host remain mounted. The failure clears focus and returns `false`, allowing the key to fall through to global shell handling.

Reachable sequence: focus keyed TextEditor B; publish and accept `A+C+B` while B remains keyed and has the same component host; send a key. The DOM/React identity for B remains mounted, whereas WGPU clears it because the stored arena location is stale.

### Focused Ink edit is silently cancelled after the same accepted change

`FocusedInkEditor` carries the same obsolete `NodeId` plus a stable host at `:1569` through `:1590`; key and blur lookup require that exact old node at `:1593` through `:1656`. Their stale branch calls `clear_focused_ink_editor`, which calls `cancel_ink_edit(host_id)` at `:1588` through `:1590`. Thus the equivalent unchanged-B sequence cancels a live edit instead of retaining React-equivalent focus.

`InkClipboardAddress` has the same address shape at `:1662` through `:1680`; `with_live_ink_surface` requires the old node at `:1682` through `:1692`. An OS clipboard completion after the accepted B insertion therefore clears its focus and drops the otherwise valid paste at `:1701` through `:1728`.

## Small coherent repair

Use the same witness-qualified mapper as renderer scene capture, immediately before the UI swap:

1. For each retained scene owner—at most 16 pointer owners, plus the singleton Text focus, Ink edit focus, and Ink clipboard address—ask for a candidate node only when its window participates in the sealed witness.
2. Build the candidate `ScenePointerTarget` and require `same_component_host`. This verifies key, kind, wire surface ID, host ID, component generation, window ID and window generation before replacing only `NodeId`.
3. If a participating address has no candidate or fails that identity check, leave the current presented address authoritative until the normal removal/blur/cancel lifecycle retires it. Do not redirect it to a sibling and do not manufacture an Up, edit cancellation, or clipboard action.
4. Commit the prepared replacements with the existing acknowledgement; untouched windows are not queried or changed.

This is a finite preparation set. It reuses the O(1) node-owned reverse binding plus O(log n) candidate lookup and avoids a document scan or an extra tree clone. The renderer acknowledgement coordinator is the appropriate common commit point; a helper that only fixes `SCENE_POINTER_OWNERS` leaves focus and clipboard wrong.

## Required native and browser laws

Extend the existing neutral `scene-pointer-owner` fixture with a keyed unchanged TextEditor and InkCanvas sibling insertion row. Native laws should perform accepted `A+B` -> focus/open B -> accept `A+C+B`, assert B host preservation plus `NodeId` change, then:

- Text: one key is consumed by B, emits no global action, and focus remains B.
- Ink: one edit key and blur commit exactly one B update; no cancel action occurs.
- Async Ink clipboard: begin native clipboard read before acceptance, complete it after acknowledgement, and assert exactly one B paste; a remove/retype control asserts zero paste and no redirected action.

The React oracle should mount keyed B, focus it, insert C before B, then type/paste after rerender. It must assert B remains `document.activeElement`/receiver and a remove/retype target receives no terminal action.

## SceneIntentQueue: unproven cross-ACK risk

`SceneInteractionIntent` stores old `tree_revision`, `surface_generation`, `NodeId`, and host ID at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:403` through `:548`. `process_scene_interaction` rejects a changed tree/node at `:2087` onward; no candidate mapping exists.

That alone does not establish a user-visible failure. `AppFrameTransaction` drains one scene interaction before moving forward from `InputEvents` at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13013` through `:13017`, so ordinary queued events appear to settle before a frame can be presented. A test must hold an `InkInteractionJob` or another pending intent across a candidate acknowledgement before classifying this as a defect.

If that test demonstrates an overlap, preserve the event's old presented target through completion, or witness-map it only after the same `same_component_host` validation. A stale intent must retire silently; it must never apply a successful terminal event to a replacement.

## Separate already-observed external retirement handoff

Not a focus/capture mapper issue, but relevant to the new scene lifetime packet: Interpreter pops a UI retirement before submitting it to the sole external close owner at `.../🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2707` through `:2711` (and document-close `:376` through `:382`). `retire_scene_identity` takes the `SCENE_STATE` row before asserting the sole `SCENE_SURFACE_RETIREMENT` slot empty at `.../🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1271` through `:1283`. Two accepted removals while the first external close is held therefore need a bounded try-admit/queued external lane; do not pop the second UI retirement before that lane owns it.
