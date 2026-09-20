# InkCanvas Text and Table Editing

## Scope

This packet closes the text and table-cell editing part of gap 2 in `📓️astra-terra-surface-completeness.md`. Clipboard copy/paste remains a separate follow-up because it has a distinct OS clipboard boundary and image/block identity laws.

## Production Repair

- Added bounded local `InkEditState` ownership to each native InkCanvas surface. It retains the source block, text draft, text/table-cell identity, replacement state, and current screen rect.
- Extended the existing resumable `InkInteractionJob` hit scan to retain the top block and recognize a second primary press on the same editable text block or table cell within the double-click interval.
- Opening an editor focuses an owned input identity and publishes only the existing selection intent. Editing itself stays local until a commit boundary.
- Painted the active editor above the canvas and registered it last as `HitKind::Input`. The same hit therefore owns pointer resolution and Shell's retained accessibility projection exposes it as a textbox.
- Routed keyboard input to the focused Ink editor before the shell's Space handling and before other surface owners. Character, Space, Backspace/Delete, and multiline text Enter update only the local draft.
- Escape retires the editor without publishing a document action. Text Tab and pointer blur outside the editor commit. Table Enter and Tab each commit and advance to the next cell; the final cell retires focus.
- Each commit reserves and publishes exactly one `inkApplyEvents` action with `phase: "atomic"` and exactly one `updateBlock` event, then updates the local override only inside the successful publication transaction.
- The focus address records window id, document generation, retained node id, and surface id. A retired/reopened generation, removed node, changed kind, or changed surface cancels local edit state and cannot mutate the new document.
- The renderer commits on a primary press outside the active editor, then continues routing that press to its intended target.

## Shared Law and React Oracle

The language-neutral fixture contains a text block `"a"`, a two-cell table `"b"/"c"`, viewport and gesture coordinates, and the exact expected atomic `updateBlock` events for text blur, table Enter, and table Tab. Escape and window-generation retirement require no action.

The React law mounts the actual `InkCanvasHost`, drives real double-click, blur, Enter, Tab, and Escape events, and compares its semantic actions with the shared fixture. The first successful behavior run exposed a React defect: the uncontrolled table input was reused when advancing to the next cell, leaving the prior cell's DOM value visible. Keying the overlay by block/row/column remounts and focuses the correct cell. The shared test adapter now exposes its underlying semantic `doubleClick` event alongside its existing pointer and keyboard events.

The native law enters through actual retained `UiCommand::Scene` pointer commands and the production focus/key/blur functions. It checks exact atomic events, Enter and Tab advancement, Escape cancellation, and stale generation rejection. A separate paint law verifies the active editor is a topmost retained `HitKind::Input` over the generic canvas hit.

## Verification

The focused React law passed through the repository's Bun/Nx entry point after the per-cell identity repair:

```text
nx run @semio-tech/framework-renderer-react:test-long ../../../../🧪️tests/🖋️ink-canvas-editing/🟦️.tsx --silent=false --reporter=verbose
Test Files  1 passed (1)
Tests       1 passed (1)
NX Successfully ran target test-long for project @semio-tech/framework-renderer-react
```

The second focused run, including the explicit Tab commit/advance assertion, also passed 1/1. Cargo and native build commands were intentionally not run by this subtask; the root integration agent owns the sequential native and browser gates.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/🖋️ink-canvas-editing/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖋️ink-canvas-editing/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`
