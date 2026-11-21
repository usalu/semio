# Large Layout Browser Import and Viewport Route

The large-sheet JSON contains `document`, a single 8192 × 4096 page, and 32 rectangle frames. The committed file is a test corpus envelope. No matching archive was found among the tracked files or generated outputs in the current Layout ticket and its earlier browser-journey ticket.

## Exact Fixture Import

The browser’s Layout Actions panel invokes `importArtifactDocument` and opens one `.semio-archive` in a fresh Layout owner. The earlier browser journey exercises that path at lines 368–377 of `26/09/16/LAYOUT-PLUGIN-END-TO-END/browser-journey.mjs`. The Artifact panel can verify the 32 frame IDs by requesting successive tree windows.

The runnable native archive law at `✏️editor/🧪️tests/🔬️unit/📦️archive.rs` encodes the snapshot as a Layout Pack, makes an empty SPR history, stamps the root identity and submits the archive to the real owner. The TypeScript outer archive encoder is `encodeDocumentArchiveBytes` in `🧰️framework/🔨️modules/🪢️wire-contracts/🟦️.ts`. The independent JavaScript value encoder writes a JSON-bridge record. Layout Pack admission requires the Layout-specific envelope and record format. The Layout guest’s `artifact:in` media entry can issue `ReplaceDocument` from typed intrinsic Layout data. The normal browser natural-file route submits registered binary media under `artifact:native`, and the shipped Layout descriptor has no raw Layout JSON file format. No build-free normal picker route from this exact JSON corpus to a Layout owner was found.

## Viewport Motion

The world viewport is the Layout Blueprint inside `#semio-wgpu-canvas`. A middle-button drag in the Blueprint changes the Canvas2d pan, and a wheel gesture changes zoom and settles the camera through the WGPU input path. Capture the same Blueprint region before and after each gesture and compare a page edge or a known frame’s projected pixel position. The Artifact panel’s stable frame IDs distinguish camera motion from document edits. A direct Actions-panel `setCamera` is available for deterministic positioning before the gesture.

A separate, build-free browser exercise can resize a genuine existing Layout to 8192 × 4096 using `patchPage` on `page-1` with string values for the `width` and `height` fields. That verifies motion on the requested page dimensions and uses the current document’s existing frames. The exact fixture route still needs an authored native archive or a publicly exposed path to Layout’s intrinsic media entry.

## Opt-in Browser Check

The existing browser journey accepts `--large-sheet-preview`. It uses a fresh Demo owner and the normal page Inspector to commit the fixture width and height. It creates 32 rectangles through `addFrame`, verifies selection by the unique X and Y pair in the Inspector, then commits the fixture width and height. The exported SVG must match the page extent and the full multiset of rectangle bounds. The check uses fresh frame IDs and retains the Demo layer and paint.

The check sends physical wheel and middle-button input to the Blueprint. It requires a stable camera sequence and three consecutive identical pixel observations for each gesture. The settled pixels must differ from the preceding baseline. The report includes each fixture frame’s action and commit duration, the published UI turn, and viewport input, camera, and paint timings. The existing interaction timeout bounds each observation.

JavaScript syntax and whitespace checks passed. The SVG oracle accepted all 32 fixture rectangles and rejected an omitted rectangle. A Sharp-backed mock supplied intermediate camera positions and paints and confirmed that both gestures select the final camera. Live execution remains pending on port 6179.
