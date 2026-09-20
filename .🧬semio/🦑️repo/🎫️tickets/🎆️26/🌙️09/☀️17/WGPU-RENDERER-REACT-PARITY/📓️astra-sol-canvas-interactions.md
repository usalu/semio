# Astra–Sol Canvas Interactions

## Result

The available React service on `127.0.0.1:6313` is the fixed `puzzle3d` React development target. Its process environment names `serve-puzzle3d-react-dev`, `SEMIO_PLUGIN=puzzle3d`, and `VITE_SEMIO_PLUGIN=puzzle3d`. Supplying `?plugin=draw` does not change that baked variant. The measured page exposes two `.semio-world-3d-host` surfaces, `window:puzzle3d-main-top` and `window:puzzle3d-main-perspective`; it exposes no Draw `Canvas2dHost` with controller `drawing-play` and surface `drawing.play.composite`.

Storybook on `127.0.0.1:6327` returned an index with 230 entries and zero entries matching `draw` or `Canvas2dHost`. It therefore could not provide a real Draw host without an unauthorized rebuild or relaunch.

The ticket now owns a bounded physical-probe contract at `🔬️canvas-interactions/📜️script.ts` and its neutral fixture at `🔬️canvas-interactions/🧫️fixtures/🔣️.json`. It requires the exact Draw host selector and records observed surfaces before any interaction. A wrong host fails explicitly; Puzzle3D is never labelled as Draw.

## Interaction contract

The fixture records these cases for a later dedicated Draw React activation:

1. Trusted primary drag with `shapeRect`: require ordered `canvasPointerDown`, `canvasPointerMove`, and `canvasPointerUp`, one added layer, and a changed canvas image.
2. Trusted primary click with `selectDirect`: require pointer down/up, one selected layer, and a changed selection overlay.
3. Trusted Shift-click and platform-modifier click: require additive selection followed by subtractive selection. The real Draw law maps Shift to `additive`, Control/Meta to `subtractive`, and their combination to `invertive`.
4. Trusted pointer input followed by physical Escape: require `canvasEscape`, no committed layer, and removal of the draft preview.
5. Trusted Playwright double-click: require `canvasDoubleClick` and one observable draft commit.

These action names and utilities come from the Draw publication contract and `Canvas2dHost`; the default utility is `selectDirect`, and the real shape gesture returns to it after commit. No production defect was isolated because no matching host was available.

Desktop Playwright has no trusted physical `pointercancel` primitive. Synthesizing `dispatchEvent` would bypass the browser input path, so the fixture marks that case unsupported rather than fabricating evidence. Escape remains a genuine keyboard cancellation case.

## Validation

- `bun .../🔬️canvas-interactions/📜️script.ts test`: PASS, five cases and one explicit unsupported case.
- `bun .../🔬️canvas-interactions/📜️script.ts run`: expected explicit failure, exit 1. It measured only the two Puzzle3D World3D surfaces and wrote the unavailable receipt.
- `GET http://127.0.0.1:6327/index.json`: HTTP 200, 230 indexed stories, zero Draw/Canvas2dHost matches.

The canonical dedicated target exists as `@semio-tech/framework-os-dev:activate-draw-react-dev`; the play project also references `@semio-tech/framework-os-dev:prepare-draw-react-dev`. A later integration pass must activate that Draw React variant on its own port, point `SEMIO_CANVAS_REACT_URL` at it, and then calibrate the physical cases against the exact host. This packet did not activate, build, generate artifacts, run native code, or exercise WGPU. It makes no WGPU runtime claim.

## 2026-09-20 probe revision

The neutral fixture is now version 3 and names the canonical Draw, Layout, and Note surface specimens instead of treating clipboard or catalogue work as a Draw concern:

- Draw remains the five-case Canvas2d journey at React/WGPU ports 6064/6164.
- Layout owns the four-case catalogue drag journey at React/WGPU ports 6079/6179.
- Note owns the six-case Ink clipboard/editor/cancellation journey at React/WGPU ports 6080/6180.

The exact Ink journey and its validation boundary are recorded in `📓️astra-sol-ink-clipboard.md`. The Draw cases remain preserved unchanged and are reported as `ready-for-physical-calibration` until root runs the freshly activated dedicated Draw target. The fixture test now also executes Chromium's browser-level CDP cancellation producer and proves its events are trusted and contain no forged pointer-up.

## Layout catalogue physical adapter

`🔬️canvas-interactions/🧬️schema/📐️layout-catalogue/🔣️.json` fixes the Layout journey to the real `demo` document and the exact four-item catalogue roster: `page`, `rect`, `text`, and `image`. It also fixes the base MIME `application/x-semio-catalogue-item`, each kind MIME, each exact JSON payload, the Catalogue and Artifact tab identities, and the seed counts of two pages and three total frames. The schema permits no extra cases or fields.

The ticket-owned `runLayout` adapter uses genuine Playwright mouse input. For each case it performs this complete sequence:

1. Reload the `demo` fixture with the physical example picker and observe the seed count in the Artifact tree.
2. Open the Catalogue panel and start a browser drag from the real transfer handle. React requires the `data-slot="drag-handle"` and `data-drag-role="transfer"` witness; WGPU requires the exact `tree.drag.transfer.<sourceId>` hit.
3. Move onto the exact Layout Canvas2d surface, require `canvasDragOver`, verify the editor controller plus base and kind MIME values, and capture the preview pixels.
4. Move outside the surface and release. Require `canvasDragLeave`, no synthetic `canvasDrop`, restored pixels, and an unchanged Artifact count.
5. Repeat the physical drag and drop. Require terminal action order `canvasDragLeave` then `canvasDrop`, the canonical JSON payload, and exactly one new page or frame in the Artifact tree.

Page preview is deliberately pixel-stable because the page operation has no frame preview. Rectangle, text, and image previews must change the canvas pixels. All four cases require leave retirement before the committed mutation.

The neutral fixture test includes an independent Chromium HTML Drag and Drop oracle. It uses a real draggable DOM node and Playwright mouse movement, observes trusted `dragstart`, `dragover`, `dragleave`, and `drop` events, checks both MIME types, and checks that the exact JSON payload is readable at drop. Chromium deliberately withholds `getData` during dragover, so the adapter uses the MIME roster for preview and the payload for the terminal drop, matching browser policy.

The contract is grounded in the production sources:

- Layout catalogue publication supplies the exact roster, MIME types, and payloads in `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs`.
- The Artifact panel publishes the page and frame section identities consumed for mutation counts.
- React Tree exposes the physical transfer handle and mirrors its MIME/payload attributes.
- `Canvas2dHost` admits the base MIME, publishes the kind roster during preview, publishes leave, and publishes leave before the exact raw payload on drop.
- WGPU input exposes the same transfer owner as `tree.drag.transfer.<item.id>`.

### Runtime boundary

The canonical targets are:

- `@semio-tech/framework-os-dev:prepare-layout-react-dev`
- `@semio-tech/framework-os-dev:activate-layout-react-dev`
- `@semio-tech/framework-os-dev:serve-layout-react-dev`
- `@semio-tech/framework-os-dev:serve-layout-wgpu-dev`

This packet did not activate or serve them. The Layout adapter therefore has a strict execution-ready contract and an explicit unavailable runtime boundary; it has no passing physical-app claim yet. Root can run `📜️script.ts run layout react` against `http://127.0.0.1:6079/?plugin=layout` and the WGPU counterpart against port 6179 after the corresponding canonical targets are active.

### Layout validation receipt

- `bun .../🔬️canvas-interactions/📜️script.ts test`: PASS. The fixture reports version 3, the exact four Layout cases, and the trusted Chromium drag event sequence with the exact terminal payload.
- `bun x tsc .../🔬️canvas-interactions/📜️script.ts --noEmit --module esnext --moduleResolution bundler --target es2022 --allowImportingTsExtensions --skipLibCheck`: PASS.
- No native build, WGPU activation, artifact generation, or Layout application serve was started by this packet.
