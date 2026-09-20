# Ink Clipboard Ownership and Transport

## Scope

Phase 1 defined the shared clipboard contract and proved the React `InkCanvasHost` behavior before WGPU production changes. Phase 2 implements the bounded WGPU native/browser paths and the exact Ink pointer-cancellation owner. The combined scope covers selected-block copy, structured block paste, plain text, SVG, tiny raster images, editor priority, bounded asynchronous ownership, close/reopen validation, stale-completion rejection, and transient gesture retirement.

## Contract artifacts

- Schema: `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🖋️ink-clipboard/🔣️.json`
- Neutral fixture: `🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/🖋️ink-clipboard/🔣️.json`
- Mounted React oracle: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖋️ink-canvas-clipboard/🟦️.tsx`
- React test registration: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`

The schema fixes the payload discriminator as `ink.clipboard`, keeps recursive Ink item definitions strict, and records current hard limits rather than inventing compatibility behavior:

| Resource | Current bound |
| --- | ---: |
| Clipboard text payload | 16,383 bytes |
| UI action item | 16,384 bytes |
| String inside an action | 4,096 bytes |
| Selected Ink items | 254 |
| Mounted clipboard operations | 32 |

The fixture includes a recursive group/table payload, an image asset, a selected-order copy case, plain text, SVG, a 1×1 PNG, malformed payloads, wrong-schema payloads, a non-array payload, and lifecycle cases for replacement, close, and stale completion. Its camera and snap inputs resolve the paste target to `{x:72,y:64}` and pin the top-level clone offsets while descendants retain their local geometry.

## React oracle result

Focused command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test-long ../../../../🧪️tests/🖋️ink-canvas-clipboard/🟦️.tsx --silent=false --reporter=verbose --skip-nx-cache
```

Phase 1 receipt: 1 test file, 6 tests passed. The Phase 2 rerun below extends this to 7/7 with mounted PointerCancel coverage.

Captured outputs: `🗑️generated/astra-ink-clipboard/react-oracle.log` and `🗑️generated/astra-ink-clipboard/react-oracle-phase2.log`.

The mounted oracle proves:

1. strict Ajv 2020 schema and fixture validation;
2. selected blocks are copied in selection order, missing IDs are skipped, image assets are not embedded, and an empty selection does not write;
3. structured payloads round-trip, every pasted descendant gets a new ID, top-level items are offset to the snapped canvas center, and the pasted top-level IDs become the selection;
4. trimmed plain text becomes a text block and SVG becomes an image block backed by an SVG asset;
5. the first image clipboard item has priority and is read as a data URL before block/text parsing;
6. an active contenteditable editor retains native copy and paste ownership.

The first red receipt was 4/6: strict Ajv required `type: "object"` next to `unevaluatedProperties`, and the editor test point overlapped a group. The schema and neutral coordinates were corrected; no product behavior was weakened.

## Existing React behavior

`InkCanvasHost` serializes `{schema:"ink.clipboard",blocks: InkItem[]}` to `text/plain`. It reads the selected IDs in their declared order, skips missing blocks, and omits the asset map. Paste resolves the world point at the canvas center, applies the existing snap law, recursively re-identifies every item, offsets top-level coordinates, publishes one atomic `addBlock` event per top-level clone, then selects the new IDs.

Input precedence is image clipboard item, valid structured Ink payload, trimmed SVG, then trimmed plain text. An image uses `FileReader`, publishes the data URL as an asset, and creates a centered image block. SVG follows the same asset-and-image path. Plain text becomes paragraphs in a text block. Copy and paste both return without intercepting the browser when a contenteditable editor is active.

## WGPU source findings

### Ink scene authority

The WGPU Scenes target already owns strict Ink document validation, raw block spans, bounded selection, bounded `InkInteractionJob`, active text/table editors, atomic editor commits, and `write_ink_events_action`. The document byte ceiling is 16 KiB, the item ceiling is 256, and the selection ceiling is 254. It does not yet own a semantic clipboard parser or producer.

The renderer already has an exact `FocusedInkEditor` address containing window ID, window generation, node, and surface ID. It validates that address on every keyboard and blur event. A parallel surface-level owner is required because React focuses the Ink canvas root even when no text/table editor is active.

### Generic clipboard port

The native UI host currently exposes `ClipboardIoJob::Read | Write(String)` through `arboard`. Reads return text only. Each retained clipboard operation owns one 16 KiB payload page and the mounted registry has 32 generational slots.

The Interpreter drains generic clipboard commands and can dispatch a generic `UiEvent::Paste`, but its completion callback has no `InputState` and therefore cannot publish semantic Ink actions. Generic event routing also does not make `ComponentScene` focusable or route its block selection through retained text events. Selected Ink block clipboard behavior therefore belongs at the renderer/scene semantic boundary, while transport remains in the generic OS clipboard port.

### Browser transport

Browser boot currently inspects at most 16 clipboard items and forwards only the first `text/plain` value as `{kind:"paste",text}`. Frame transport is bounded to 64 lossless items, 256 KiB total admitted bytes, 4 KiB per message, and 1,024 code units per text chunk. The worker reconstructs a text stream, but the winit ingress discards its semantic target and feeds the generic text buffer. Image clipboard items are currently ignored.

### Image dependencies and action ceiling

The renderer already depends on the repository image codec and first-party base64 module, so native `arboard` RGBA can be PNG-encoded inside the renderer without a new runtime dependency. The generic UI host must remain codec-neutral.

`write_ink_events_action` currently stores the complete event JSON in one action string. Although an action item has 16 KiB, each action string is limited to 4 KiB. The neutral 1×1 PNG fits. Arbitrary image sizes do not. Initial WGPU parity is therefore bounded by the existing semantic action limit unless a later schema-first chunked asset event is introduced. Raising a ceiling or silently truncating images is not acceptable.

## Production ownership plan

### 1. Exact surface focus

Add `FocusedInkSurface` beside `FocusedInkEditor`, addressed by `(window_id, window_generation, node, surface_id)`. Set or replace it only after a live primary pointer-down reaches an Ink `ComponentScene` in `process_scene_interaction`. Clear it when validation fails, another focusable surface takes real focus, or its window/node/surface is closed or replaced.

The editor owner has precedence. When a live Ink editor owns the same surface, block copy and paste refuse admission so native editor text semantics remain authoritative.

### 2. Keyboard admission

After the editor keyboard branch and before Shell chords, handle exact platform Mod+C and Mod+V for a validated `FocusedInkSurface`.

- Copy synchronously reads `checked_ink_document`, serializes selected raw blocks in selection order, skips missing IDs, and writes the exact `ink.clipboard` text payload through the existing OS clipboard port.
- Native paste admits a generic clipboard read operation.
- Browser Mod+V reserves the semantic intent but waits for the actual DOM paste payload. It must not combine a navigator read with the DOM paste event and create duplicate mutations.

### 3. One generic clipboard content contract

Extend the existing clipboard port to return owned content:

- `Text(String)`
- `ImageRgba8 { width, height, bytes }`

Native reads apply React's image-first priority. The codec-neutral host returns RGBA; the renderer encodes it as a PNG data URL using dependencies it already owns. Text writes continue to support structured block copy.

Browser paste projects the first image item through `FileReader`, otherwise the first `text/plain` value. The transport discriminator is required and strict; no optional legacy field or parallel Ink-only adapter is introduced.

### 4. Semantic browser streams

Replace the ambiguous paste target with explicit `PasteText` and `PasteImageDataUrl` stream targets. Preserve the target through browser worker reconstruction and winit ingress.

At stream start, snapshot the exact Ink surface address and target point. At commit, route to the Ink semantic parser only if the address is still live. Without a live Ink owner, text continues through generic retained-text paste and image content is ignored.

### 5. Scene parser and event publication

The Scenes parser applies the fixed precedence: image, valid structured Ink payload, SVG, plain text. It validates all payload and item bounds before emitting events, reuses the monotonic Ink ID owner for recursive re-identification, and publishes only the existing event grammar:

- block payload: `addBlock…`, then selection;
- SVG/raster: `putAsset`, `addBlock`, then selection;
- plain text: `addBlock`, then selection.

The paste point is captured from the actual scene rect/camera and passed through the existing snap law. No plugin adapter or duplicate asset store is added.

### 6. Async lifetime and cancellation

Every admitted paste is an independent bounded operation. A newer user paste must not silently replace an earlier valid one. Each token carries the exact surface address, its request generation, and the target point.

Window close, node removal, surface replacement, or generation reuse retires the operation. Late success and failure complete benignly and cannot attach state to the new generation. Native completion is pumped at an existing bounded input/frame synchronization phase with `&mut InputState`; it is not published from the clipboard callback and does not create a thread.

Admission remains bounded by the existing 32 native clipboard slots and browser stream credits. Refusal is explicit and releases all reserved ownership.

## Implementation laws after epoch release

The next source cohort should add:

- Scenes fixture laws for selected-order copy, recursive re-identification, snapped offsets, text/SVG/PNG event sequences, malformed input, action-size refusal, and editor bypass;
- Interpreter/renderer laws for real pointer focus, exact Mod+C/Mod+V, multiple independently admitted reads, close/reopen generation reuse, late completion rejection, 32-slot refusal, and terminal retirement;
- browser transport laws for image/text priority, strict target preservation, chunk reconstruction, cancellation, and no duplicate DOM/navigator paste;
- an actual host integration law that focuses Ink by pointer, copies through the real generic port, completes an asynchronous paste, and observes the exact published scene action.

That statement was the Phase 1 boundary. The Phase 2 implementation and its current validation boundary follow.

## Phase 2 implementation

### Semantic scene contract

The WGPU scene target now owns the same parser and publisher as the React host:

- copy traverses the declared selection order, recursively resolves blocks, skips missing IDs, and serializes the strict `{schema:"ink.clipboard",blocks}` payload without embedding the asset map;
- structured paste validates the payload, recursively assigns fresh IDs, preserves names and descendant-local geometry, offsets top-level blocks to the snapped viewport-center target, and publishes one atomic `inkApplyEvents` action;
- text paste trims input and creates one text block with one paragraph per line;
- SVG and image-data URLs publish `putAsset`, image-block, and selection events through the same bounded action;
- an active Ink text/table editor keeps priority and accepts text through its draft owner while refusing image paste.

The scene path uses the existing 16 KiB document/input limit, 256-item document limit, 254-item selection limit, and 4 KiB action-string limit. It does not raise a ceiling or truncate content.

### Native clipboard ownership

The generic UI host clipboard port now returns `ClipboardContent::Text` or `ClipboardContent::ImageRgba8`. Native reads inspect the image first, exactly matching the React item priority. The host remains codec-neutral; the renderer uses its existing `image` and `base64` dependencies to encode admitted RGBA as PNG only after the exact Ink owner is revalidated.

`FocusedInkSurface` snapshots `(window_id, window_generation, node, surface_id, rect)`. Copy and paste re-resolve all five fields against the retained engine. Native paste reserves one of 32 semantic completion slots before submitting the existing 32-slot clipboard worker operation. The reserved slot is never replaced. Completion, empty clipboard, refusal, and worker fault all terminate that exact slot; late content is applied only if the original address remains live. Capacity refusal is recorded as `ItemCredits`, and a worker/codec failure is recorded as `Structure` rather than being silently converted into a paste.

### Browser transport

Browser paste inspects at most 16 `DataTransferItem`s. The first valid image wins and is read as a data URL; otherwise the first `text/plain` item wins. Browser boot does not also invoke `navigator.clipboard`, so one physical paste cannot duplicate a mutation.

The strict `paste-image-data-url` target now survives browser-frame transport, worker reconstruction, winit ingress, and the semantic Ink stream. The stream registry has 32 independent slots, rejects duplicate identifiers and saturation, validates declared and accumulated bytes, and snapshots the exact Ink address at start. Commit takes the slot before applying content and rejects a closed or replaced surface. Abort takes only its addressed slot.

### Pointer cancellation

The shared scene cancellation fixture and schema remain the canonical language-neutral owner contract:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🛑️scene-pointer-cancellation/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🛑️scene-pointer-cancellation/🔣️.json`

Every queued scene intent carries its `surface_generation` and optional `PointerId`. `InkInteractionJob` copies those witnesses when it is created. A pointer cancel can mark the job retiring only when pointer ID, surface ID, window ID, and current surface generation all match. Sibling pointers and reused generations cannot retire it. The exact live Ink owner clears drag, `pointer_was_down`, marquee points, and live stroke/shape overrides without emitting PointerUp, selection, camera, or `inkApplyEvents`. Ordinary window teardown retains the existing bounded close path even when no pointer owner remains.

React now has a dedicated `onPointerCancel` path. It cancels the pending animation frame, discards queued live events and draft/drag/marquee state, and does not call the PointerUp commit path. `onPointerLeave` no longer commits or cancels a gesture.

## Phase 2 validation receipts

Focused React command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test-long ../../../../🧪️tests/🖋️ink-canvas-clipboard/🟦️.tsx --silent=false --reporter=verbose --skip-nx-cache
```

Receipt: 1 file, 7 tests passed. The seventh mounted law proves PointerCancel publishes no stroke and a subsequent Down/Up gesture commits normally.

Focused browser-frame transport through Nx: 1 file, 42 tests passed. The new law proves `paste-image-data-url` is retained through lossless chunk admission and reconstruction.

Focused browser input projection through Nx: 1 file, 27 tests passed. The new DOM-item law proves a valid image wins even when text appears first, text wins without an image, the scan stops at 16 items, and an empty list admits nothing. Receipt: `🗑️generated/astra-ink-clipboard/browser-input-image-priority.log`.

Root's UI-host census after the `ClipboardContent` API and event-queue changes: 79 tests passed, 0 skipped. This proves the host API compiles and its bounded page owner remains accounted; it is not a full renderer or runtime acceptance receipt.

Rust source laws added for the next root-owned renderer census:

- `shared_clipboard_fixture_copies_order_and_publishes_text_svg_and_recursive_blocks`
- `ink_cancel_requires_the_exact_surface_generation_and_pointer_owner`
- `ink_pointer_cancel_retires_only_the_exact_in_progress_owner_without_publishing`
- `native_clipboard_page_decodes_text_and_exact_rgba_without_conflating_them`
- `native_rgba_clipboard_content_encodes_a_natural_png_data_url`

No Cargo, native renderer, wasm activation, or browser runtime was run from this worker. WGPU runtime clipboard acceptance remains unclaimed until root executes the native census and activated browser journey.

## Deliberate raster boundary

The current generic clipboard page can carry only a tiny RGBA image under 16 KiB, and the semantic `eventsJson` string can carry only 4 KiB. The neutral 1×1 PNG fits and is implemented end to end. Arbitrary raster images require a future schema-first chunked asset transfer whose chunks have independent ownership and cancellation. This packet explicitly refuses oversize input; it does not stream, truncate, resize, or claim large-image parity.

## Prepared physical Note journey

The canonical app boundary is now explicit in the existing ticket probe:

- Draw is `Canvas2d`, controller `drawing-play`, surface `drawing.play.composite`, on the dedicated React/WGPU ports 6064/6164.
- Note is `InkCanvas`, controller `s.note.note@1/*#editor`, surface `note.play.composite`, on the dedicated React/WGPU ports 6080/6180.

Ink clipboard acceptance therefore belongs to Note. The probe does not relabel Draw as Ink or accept a listener merely because a `?plugin=note` query was appended to another activated variant.

The existing `🔬️canvas-interactions/📜️script.ts` and its neutral `🧫️fixtures/🔣️.json` now describe both targets. The Note journey:

1. requires the exact composite Ink surface and the `demo` example with `text-semio`;
2. physically selects the demo block, sends the platform copy chord, reads the browser clipboard, and requires one strict `ink.clipboard` payload without assets;
3. writes a structured block payload and plain text to the real browser clipboard, sends the platform paste chord, and requires one `inkApplyEvents` plus the visible re-identified/text result;
4. writes a natural 1×1 PNG through `ClipboardItem`, sends paste, and requires `putAsset` followed by `addBlock`; React additionally checks the mounted image's natural dimensions;
5. opens the real text editor, pastes through the browser clipboard, and requires editor content ownership with no block-level `inkApplyEvents`;
6. selects Pencil, uses Chromium CDP `touchStart → touchMove → touchCancel`, snapshots the action cursor immediately before cancel, and requires no terminal `inkApplyEvents` or `setSelection`; a following independent touch gesture must reach a commit.

The WGPU path resolves the scene through `dumpStructure` and verifies bounded action arguments through `dumpChrome`; React resolves the exact DOM Ink surface and visible block consequences. Runtime receipts are written per target/renderer below `🗑️generated/canvas-interactions`.

The probe's offline contract test uses the installed Playwright Chromium as the independent browser oracle. It observed trusted `pointerdown`, `pointermove`, and `pointercancel` events sharing pointer ID 2, and no `pointerup`. Receipt: `🗑️generated/astra-ink-clipboard/draw-note-probe-fixture.log`. Its isolated TypeScript check is `draw-note-probe-typecheck.log`.

Prepared root-owned commands after fresh activation:

```text
SEMIO_CANVAS_TARGET=note SEMIO_CANVAS_RENDERER=react bun nx exec --projects=workspace -- bun <ticket>/🔬️canvas-interactions/📜️script.ts run note
SEMIO_CANVAS_TARGET=note SEMIO_CANVAS_RENDERER=wgpu bun nx exec --projects=workspace -- bun <ticket>/🔬️canvas-interactions/📜️script.ts run note
```

This worker did not start either listener, activate an artifact, or execute the app journey. The prepared probe is not a runtime-parity receipt. Browser PNG acceptance will cover the browser image-data-url lane; the native RGBA lane remains covered by the source law and awaits root's native census rather than being inferred from browser PNG behavior.

## Native37 and Native38 reconciliation

Root's Native37 census reached the new Ink laws and exposed three real failures: the exact in-progress cancellation fixture never observed a matching job, the pre-existing text/table lifecycle never reached its terminal assertion, and the clipboard fixture compared JSON `56.0` and `56` structurally. Native38 confirmed the first two after the first reconciliation. These are red receipts, not runtime acceptance.

The text/table hang was caused by a rejected scene interaction being returned to the queue indefinitely. `drive_scene_interaction_step` now retires and boundedly closes a rejected intent and records its fault instead of requeuing it forever. This keeps the fault observable and prevents one malformed operation from owning the worker lane permanently.

The cancellation fixture now establishes a real document generation before driving the scene worker. Generation zero is an invalid identity throughout the retained runtime: `UiWindow.accessibility_generation` begins at zero, document-tree publication rejects it, and accessibility dispatch refuses it. `InkInteractionJob::new` therefore continues to reject zero. The test publishes generation one, proves the worker owns that exact identity, and advances to generation two through a real close/reopen sequence for the stale-owner case. The shared cancellation fixture records `zeroGeneration: invalid` so this boundary is language-neutral rather than test folklore.

Clipboard coordinates remain JSON numbers whose lexical integer/floating representation is irrelevant. The source law now compares their numeric value through `as_f64`; it does not normalize or loosen the clipboard schema.

The three focused native laws were retained in the later full renderer censuses:

- `shared_clipboard_fixture_copies_order_and_publishes_text_svg_and_recursive_blocks`
- `ink_pointer_cancel_retires_only_the_exact_in_progress_owner_without_publishing`
- `ink_canvas_text_and_table_editing_matches_the_react_host_lifecycle`

Root's Native44 census executed 1,228 renderer laws: 1,226 passed and the only two failures were the independently owned intermittent Tree and Actions laws. The three Ink laws above therefore passed there. Root's subsequent Native48 full census passed 1,231/1,231 with zero skips, retaining all Ink clipboard and cancellation coverage in the clean complete binary. This is a native ownership/semantics receipt; activated Note clipboard acceptance remains pending the prepared browser journey.

## Physical Note evidence boundary

The prepared Note journey now retains cropped Ink-surface screenshots for every clipboard/cancellation case rather than treating an action-ledger row as visible acceptance:

- block copy captures the unselected and selected surface and requires a pixel change before accepting the strict clipboard payload;
- structured text, plain text, and tiny PNG paste capture immediately before the physical paste and wait for a changed surface after the expected action;
- React still proves re-identification, localized text, and natural 1×1 image dimensions from its mounted DOM; WGPU claims only the operation names its real action introspection exposes plus a visible surface change, not an unexposed asset dimension;
- editor priority captures the active editor before paste, requires no block action, exits editing, and requires changed block pixels;
- exact Chromium `touchCancel` captures the pre-gesture surface, a visible live draft, exact restoration after cancel, and a changed surface after the next independent commit. The trusted CDP producer law remains unchanged.

The isolated script TypeScript check passed through Bun+Nx after the shared Layout target was merged into the same script. The installed Playwright Chromium contract check also passed fixture version 3 with all three targets (`draw`, `layout`, `note`), five Draw cases, four Layout cases, and six Ink cases. Its trusted-event oracle observed pointer down, move, and cancel with pointer ID 2 and no pointer up. The exact receipts are `🗑️generated/astra-ink-clipboard/draw-note-probe-typecheck.log` and `draw-note-probe-fixture.log`. Neither app listener was started, so these are probe-contract receipts rather than React/WGPU runtime acceptance.
