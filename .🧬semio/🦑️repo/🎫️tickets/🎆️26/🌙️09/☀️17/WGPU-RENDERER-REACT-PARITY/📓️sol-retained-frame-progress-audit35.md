# Retained Frame Progress and Candidate Append Audit

## Scope

Review and repair of `Ui::frame_into_step`, the Interpreter's `UiDocumentFrameCursor` stall watchdog, `FrameworkSceneHost`, retained glyph/synchronization cursors, and `DrawList::append_retained_candidate`. Production changes began only after the coordinated WASM27 build completed.

## Finding: the paint progress scalar is an opportunity counter

`frame_into_step` increments `RetainedPaintFrame::progress` immediately after freshness validation and before it asks the active synchronization, node-paint, or scene-paint cursor to advance. The Interpreter snapshots `paint_frame_progress`, calls `frame_into_step`, and resets `stalled` whenever the scalar changed.

Consequently every admitted call against a fresh in-flight paint frame is reported as progress regardless of its result. Once a frame exists, a repeated `Pending` cannot increment the watchdog even when the owning cursor and candidate remain unchanged. The recent `stalled > 0` logging guard removes the zero-modulo notice, but it does not restore stall detection because the unconditional scalar increment keeps `stalled` at zero.

The present source has one explicit `Pending` route that does not advance `ScenePaintCursor`: generic engine phase 5 calls `advance_raster_composite(scene)` and returns `Pending` while the Paint2d composite is incomplete. The current `RasterStackJob` receives a positive 65,536-unit grant and either advances, finishes, or errors, so this audit did not prove a current infinite loop in that implementation. Its real progress lives in `PixelProgress`, outside the retained frame scalar. If that authority ever returns incomplete without increasing `completed`, the watchdog remains silent indefinitely.

The other reviewed routes have explicit bounded movement today:

- text `Pending` advances a UTF-8 byte, line, measure cursor, phase, or output;
- retained interactive synchronization advances its phase, scan pointer, record index, retirement, or close cursor;
- retained tree walks advance a node/scalar/phase;
- the first scene-host bind stores the node before returning `Pending`;
- ordinary component-scene identifier phases advance byte/phase/item counters;
- dirty layout returns before a paint frame step and the Interpreter routes it back to `Layout`;
- candidate admission failures and missing scene authority are terminal `Fault`, not retry-shaped `Pending`.

The correct long-term contract is cursor progress rather than opportunity count. A clean repair should make every retry-shaped producer expose whether its owned cursor moved, including the Paint2d compositor's `PixelProgress.completed`, and increment the frame scalar only for a changed retained phase/walk/node-sync/node-paint/scene-paint state. A controlled `SceneHost` that returns unchanged `Pending` must grow `stalled`; a host that reports increasing external progress must reset it; glyph and traversal work must continue to reset it; dirty layout remains handled by the dedicated phase transition.

## Candidate append geometry and clipping

The current append transform is coherent:

- `frame_into_step` adds the host viewport origin to retained paint, scene-slot, tooltip, overlay, hit, scissor, and clip geometry before publication;
- the candidate is therefore already in the target draw list's logical coordinate space and must not receive another translation at append;
- the caller's active scissor and clip are inherited per appended layer, intersected with any candidate-local scissor/clip;
- a candidate layer without its own restriction inherits the caller restriction;
- scene-pass and glass-region layer indices are rebased by the target layer count;
- local glass foreground indices are rebased by the target glass count, while a candidate layer without a local glass owner inherits the caller's active foreground.

This matches the direct append architecture. An older `FrameworkSceneHost` docstring still says the candidate is window-local and later composited with an offset; that text is stale because `frame_into_step` now applies `bounds.x/y` before drawing and appends directly.

One failure-atomicity limitation remains: `append_retained_candidate` consumes the target retained-output claim before checked index rebasing. A subsequent numeric overflow would leave the claim charged and the candidate partly rebased. That path is terminal and requires near-`usize::MAX` layer/glass counts, so it is not the observed stall concern, but a future cleanup should preflight every checked offset before mutating either owner.

## Same-size viewport origin freshness

The origin question is a confirmed defect, independent of the possible scene stall. `set_viewport` intentionally owns only width and height because x/y do not affect local layout. Before this packet, `RetainedPaintFrame` freshness likewise carried only `viewport_revision`. `frame_into_step` nevertheless adds the current call's x/y to every painted node, scene slot, tooltip, overlay, clip and hit. Moving a pane without resizing it could therefore leave already-painted candidate rows at the old origin and append later rows and hits at the new origin before one atomic publication.

The repair stores `[x,y]` on the retained paint frame itself. A changed origin makes the candidate stale without forcing a new layout generation. The old draw route and synchronization owners close, the unpublished candidate is retired, and a fresh candidate restarts entirely at the new origin. Width/height still use the existing layout revision.

## Implemented progress contract

The opportunity counter is removed from both retained frame entry points. Internal phases increment the scalar only after a bounded walk, synchronization, paint, phase, hit, or publication operation actually changes owned state. `ScenePaintCursor` exposes an exact cursor witness and one monotone `external_completed` lane. A scene host returning `Pending` with the same node/phase/item/page/byte/external tuple no longer changes the scalar.

Paint2d now projects the real `PixelProgress.completed` value returned by `RasterStackJob` into that external lane. An increasing pixel count is observable work; an unchanged incomplete count is backpressure and lets the Interpreter's existing `stalled` counter grow. Phase changes reset the external lane, so a later producer cannot alias an earlier phase's count.

The neutral `framework.ui.retained-frame-progress/v1` fixture fixes the three essential transitions: bind changes progress, repeated external zero is stable, external one changes progress. It also fixes the same-size origin move and accepted-only draw/hit publication. The Rust laws replace the obsolete premise that every admitted `Pending` must change the scalar, inject a controlled unchanged scene host, then release it with real external progress, and move a partially painted candidate before checking every accepted rect and hit uses only the new origin.

## Verification

The language-neutral fixture and an actual Chromium layout oracle passed together, 2/2, through the explicit single-project React target. Chromium mounted a fixed-size absolute child, moved its parent from `[12,18]` to `[112,118]` without resizing it, and observed only the new final origin. This independently fixes the browser geometry expectation the Rust retained candidate must match.

The focused Rust UI law is still queued behind the coordinated Cargo lock; no Rust pass claim is made yet.
