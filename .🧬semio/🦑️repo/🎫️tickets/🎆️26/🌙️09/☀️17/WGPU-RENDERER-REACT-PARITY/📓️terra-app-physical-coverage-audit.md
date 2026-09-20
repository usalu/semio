# Terra App Physical Coverage Audit

Read-only audit on 2026-09-20. No activation, browser run, build, or production edit was performed by this audit.

## Canonical activation and specimens

The generated playground registry is the canonical route inventory:

| Variant | Plugin app identity | React / WGPU port | Registered specimens |
| --- | --- | --- | --- |
| Draw | `s.draw.drawing@1/*#editor` | 6064 / 6164 | `🎬️demo`, `🎬️demo-session` |
| Flow | app identity is supplied by the Flow physical fixture as `s.flow.flow@1/*#editor` | 6016 / 6116 | `🎬️demo`, `🎬️demo-session` |
| Layout | `s.layout.layout@1/*#editor` | 6079 / 6179 | `🎬️demo`, `🎬️demo-session` |
| Note | `s.note.note@1/*#editor` | 6080 / 6180 | `🎬️demo`, `🎬️demo-session` |

The Draw, Layout, and Note rows and ports are in [playgrounds.json](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:347), [playgrounds.json](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:1036), and [playgrounds.json](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:1093); Flow's ports and specimens are at [playgrounds.json](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:710). The generated dev target family is `prepare-<variant>-<renderer>-dev`, `activate-<variant>-<renderer>-dev`, and `serve-<variant>-<renderer>-dev`; serving depends on activation, which depends on preparation. This is generated for React at [🟨️.mjs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1024) and WGPU at [🟨️.mjs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1040). A `?plugin=` URL must only reach an already activated target; it is not a substitute for this lifecycle.

## P0: resolve mounted surface identity through the interpreter

The first Draw physical attempt failed before exercising a gesture because the old selector treated producer constants as React host attributes. The fresh DOM receipt instead has a Canvas child under `.semio-canvas-2d-host`, with `data-controller-id="1"`, `data-surface-id="window:drawing-composite"`, and an ancestor `data-ui-node-key="drawing.play.composite"`.

This is expected from [Interpreter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:479): `surfaceHostIdentityV1(surface, record.key, record.id)` assigns the owning document surface as `surfaceId`, the per-document record id as `controllerId`, and the authored pane key as `paneId`. The interpreter renders `data-ui-node-key={record.key}` at [🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1153). Therefore the semantic app controller used by the input ledger, the transient host controller id, and the authored pane key are three different values.

The Canvas fixture is already corrected for Draw: it now uses the authored wrapper key and owning surface while retaining the semantic controller independently at [🔣️.json](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/🧫️fixtures/🔣️.json:3). Layout and Note still use obsolete host selectors at [🔣️.json](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/🧫️fixtures/🔣️.json:13) and [🔣️.json](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/🧫️fixtures/🔣️.json:24). Flow has the same issue at [🔣️.json](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️surface-interactions/🧫️fixtures/🔣️.json:8).

Required harness convention:

1. Keep `appControllerId` only for exact ledger-action ownership.
2. Call the fixture's current `surfaceId` field `paneKey`; it is the authored node key.
3. Locate exactly one host below `[data-ui-node-key="<paneKey>"]`; validate and record its nonempty runtime `data-surface-id` and, where present, its runtime `data-controller-id`. Do not hard-code record ids.
4. Canvas targets may then select the descendant canvas. NodeGraph and Ink must locate their own host below that wrapper, because they do not promise a static controller-id selector.
5. For WGPU, use the authored pane key only to locate exactly one visible component-scene structure row in the declared window, record the complete structure path, and use the reported rect. It is not the retained host's runtime surface identifier.

This is an adapter correction, not a renderer parity conclusion. The Canvas2d, Ink, and NodeGraph hosts all pass through the same component-scene bridge, so a raw `[data-surface-id="<pane key>"]` rule is not valid for any of the three families.

## P0: use the complete retained Layout tree handle

The Layout source deliberately authors the item tail `layout-catalogue.{kind}` at [catalogue/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:46), with the four-kind roster built at [catalogue/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:69). WGPU receives the complete `tree.drag.transfer.<projected item path>` id and strips only the prefix when routing it, at [🐚️Shell/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12299).

The canvas adapter's general `wgpuHit` requires full equality at [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:224), yet Layout constructs a bare `tree.drag.transfer.layout-catalogue.<kind>` lookup at [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:859). Actual retained hit identities are scoped (for example `tree.drag.transfer.framework.display.windows/framework.display.windows.puzzle3d-main.kind`), so that equality can reject a valid row.

The smallest correct resolver is limited to tree transfer controls: require exactly one visible `dumpChrome.hits` entry whose id begins `tree.drag.transfer.` and whose suffix after that prefix ends in the exact authored `layout-catalogue.<kind>` tail. Record and use that discovered complete id for the gesture. It must not weaken general `wgpuHit` matching or accept multiple matches.

## Existing coverage and smallest additions

### Draw

The current canvas adapter already drives trusted rectangle creation, direct selection, additive and subtractive modifier selection, engagement rename outcomes, Escape draft retirement, and double-click commit in [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:585). Its fixture requires layer-count and cropped-pixel consequences in [🔣️.json](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/🧫️fixtures/🔣️.json:104).

After the corrected mount resolver has produced one paired green run, the single material missing cancellation case is a CDP touch start/move/cancel while a pen or shape draft is active. It should prove that neither layer count nor the crop changes, then prove a new independent down/up commits exactly once. This covers loss of pointer ownership; existing Draw coverage currently proves only Escape cancellation.

### Layout

The physical adapter is already a substantive end-to-end Layout specimen: it loads Demo, exposes Catalogue and Artifact panels, moves page/rect/text/image entries with genuine drag motion, checks both raw JSON and the typed kind MIME, observes preview versus leave restoration, and requires one created document item only on drop ([📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:859)). The producer supplies `application/x-semio-catalogue-item` plus `application/x-semio-catalogue-kind.{kind}`; that typed witness is verified in the authored unit law at [catalogue unit/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs:31).

No new product gesture is required before a paired run: repair the scene identity and scoped transfer-handle resolution first. The former report's missing-kind conclusion is obsolete for the current producer and bridge.

### Note

The Note probe covers selected-block copy, structured/plain text paste, image asset paste, text-editor priority, and exact CDP touch cancellation ([🔣️.json](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/🧫️fixtures/🔣️.json:148)); `runNote` executes the six cases at [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:572).

Two bounded physical gaps remain:

* The probe must always establish a fresh Demo receipt. Its current React shortcut treats an existing `text-semio` block as sufficient, and WGPU accepts any historical `setActiveExample` row ([📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:306)). Capture a cursor before opening the fixture control and require a later matching Demo selection on both sides.
* Add a physical edit/blur and edit/Escape case for the already seeded `text-semio` block: double-click it, replace text, blur outside, and require exactly one `inkApplyEvents/updateBlock` plus changed visible text; then edit again, press Escape, and require no added event and restored text. The shared React oracle explicitly covers blur commit and Escape cancellation at [ink-canvas-editing/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖋️ink-canvas-editing/🟦️.tsx:23), and the host makes the actual `updateBlock` atomic at [InkCanvasHost/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/🟦️.tsx:1400).

The same oracle also checks table-cell Enter/Tab behavior. It cannot be made a valid physical product test by injecting test-only document JSON: first identify a registered specimen or app-visible command that creates a table through the ordinary UI. No such user route was established in this audit, so no control id is proposed.

### Flow / NodeGraph

The Flow adapter already performs selection plus Escape, cancelled and committed node drags, wheel zoom, and middle-button pan, with action ownership and before/after geometry receipts ([🔣️.json](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️surface-interactions/🧫️fixtures/🔣️.json:20), [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️surface-interactions/📜️script.ts:306)).

Its missing end-to-end mutation is a valid output-port to input-port connection. React allows editable connections only after `nodeGraphConnectionIsValid` ([NodeGraph/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:1194)) and publishes exactly one `nodeGraphEdit` `connect` operation with the node and port ids ([NodeGraph/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:1210)).

The next physical case should discover one compatible visible port pair under the resolved Flow pane wrapper, record the real scoped ids and pre-existing edge count, drag between the two physical port centers, require exactly one `connect` operation naming those captured ids, and require a new visible/introspected edge. The probe should reject a fixture that exposes no unique compatible pair; it must not hard-code assumed port names.

## Execution order

1. Complete identity resolution for Layout, Note, and Flow, then rerun the already-authored Draw/Layout/Note/Flow acceptance gestures.
2. Add only the scoped WGPU Layout tree-transfer resolver; this unblocks all four existing Catalogue kinds.
3. Strengthen Note's fresh-demo receipt, then add text blur/Escape.
4. Add the NodeGraph valid connection case after fresh discovery yields a unique compatible pair.
5. Add Draw pointer-cancel after its corrected baseline run is green.

These are test-harness and evidence packets. This audit found no basis to alter the corresponding application producers.
