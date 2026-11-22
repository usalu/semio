# Raster Browser Picker Routing

The Raster `importPng` action advertises a single-file `RequestFileOpen` with a 3,144,704 byte limit and the `importPngFile` callback. The mounted Rust law already checks effect emission. For the React local shard, invocation and typed-operation completion paths both decode the request and hand it to the common shell effects loop. That loop checks the effect owner before selecting files. The bounded picker builds a transient file input, calls its click method, and starts file reading once the chooser returns a selection. The WGPU browser bridge uses the same friendly WIT request decoder.

The Actions rail assigns `action.importPng` to the tree item and makes its child label a button without an ID. The prior forced row click could report success while another element received the press. Its `button[id]` diagnostic also omitted the leaf control. The file input is detached before its click, so document-level event capture cannot observe that click. A locator for the visible leaf button and a pass-through wrapper around the input click method identify both sides of the request. The wrapper can also read the remaining browser user activation.

The React local invocation already records `semio.hop.invoke` in User Timing with `actionId`, a decoded `effects` count on success, and `failed` on error. A completed `importPng` span with one effect and no input click points to shell ownership or effect handling. A completed span with zero effects points back to guest publication or renderer decoding. The shell ledger logs input refusals with their typed reasons.

Hub-mounted shared documents have a distinct route. The retained publication allowlist admits service requests and external links. It rejects `request-file-open` with `browser-actor-publication: unsupported host effect request-file-open`. The direct actor path requires an exact live document owner in the shared browser actor table, so it is relevant when the player dispatches against a hub-mounted document. Runtime evidence is still needed to identify the route used by the failing preview press.

No production code changed during this trace. The parent task owns the live Chromium retry.

## Live Pointer and Keyboard Trace

Chromium hit-tested the Actions rail row center to `framework.window.rasterComposite.engagement`. That ID belongs to the `Pane` wrapper for the Actions tree, so the hit can be the row's own outer overlay. The wrapper is pointer-enabled and the `WindowChrome` body uses a measured clip path. A leaf can still have a nonempty bounding box when its ancestors clip its hit area. The source also gates nonreserved actions while a utility disallows them, but that gate would reject keyboard activation too.

In the live retry, pressing Enter on Import PNG opened the PNG browser chooser while user activation was still active. After selection, multiple `importPngFile` commands reached the local invocation path and a final invocation failed. This confirms chooser delivery and isolates the later import failure from the original pointer behavior. Pointer diagnosis requires the target and actual hit's owner and their clip, scroll, and intersection geometry after scrolling the leaf into view.

The pane uses the shared anchor maximum height and a scrollable engagement body. The exact overlay slot also scopes the rail to its owning Composite window. Base and extra windows can render the same action ID, so an unscoped first match leaves the window owner ambiguous. Production hit-testing needs a stable pointer witness scoped to that window. No layout or picker source change was made on the available geometry.

## Expanded Actions Clip Repair

Post-scroll Chromium geometry established that the Actions wrapper remained the same owner and that every pointer-events setting was active. The expanded body measured 300 × 552 pixels, but its CSS polygon measured only 75 × 24 pixels. Three points inside the PNG action label hit the outer engagement zone. The Actions pane reused the folded stack DOM node, leaving the geometry hook's observer-driven refresh as the only invalidation when its explicit layout changed.

WindowChrome now measures on folded-to-open and open-to-folded transitions in its layout effect. The updated polygon is published before the next paint, and existing resize, mutation, and scroll observation still handles later changes to the same layout. The regression retains the original stack DOM node through open, close, and reopen. Its neutral fixture reproduces the measured folded and expanded sizes, and the existing polygon-clipping library independently verifies the entire action hit rectangle lies inside the expanded clip. The original code produced an empty intersection. All 17 Window component tests passed after the change. The parent task owns the live pointer retry.
