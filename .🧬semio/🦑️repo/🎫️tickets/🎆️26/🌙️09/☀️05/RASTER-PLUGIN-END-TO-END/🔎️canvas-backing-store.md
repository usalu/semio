# Raster Canvas Backing Store

Chromium reported GPU texture validation errors at `CreateView` and `canvas_surface_blit` after the Actions and chooser interactions. The React raster host assigned both canvas extent properties before each attachment and at every ResizeObserver delivery. Assigning an unchanged property resets the canvas backing. The Rust render session only reconfigures its surface when the physical extent changes. The JavaScript host also allowed DPR below 1 while both Rust entry points clamped it to 1.

Paint2dHost now uses the shared synchronous canvas backing helper. Both canvas attributes and CSS dimensions are assigned only when their own value changes. The host compares logical dimensions and DPR before notifying the Wasm session, keeps the initial paint after attachment, and ignores queued callbacks from the retired Observer. The existing NodeGraph API still uses the shared helper. Rust attachment and teardown remain outside this repair.

The mounted navigator regression first reproduced three assignments to each dimension where the logical and physical extents already matched the canvas defaults. It then passed with zero redundant assignments, a width-only resize, unchanged rounded measurements, fractional DPR, and a queued callback after unmount. The fixture compares physical extents against the existing Three.js vector arithmetic. All three tests in the navigator suite passed. The parent task owns live GPU verification.

The hidden-tab renderer suite stopped before collecting tests because Vite could not resolve the generated Flow WebAssembly binding `flow_core.js`. The suite requires the Flow browser publication.
