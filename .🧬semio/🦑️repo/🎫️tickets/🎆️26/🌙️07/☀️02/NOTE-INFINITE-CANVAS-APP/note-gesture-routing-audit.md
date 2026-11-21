# Note Gesture Routing Audit

Source audit of `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost`, 2026-11-21. No live interaction was performed; these findings follow from React event routing and the current source.

## Tool routing over blocks

`🧱️ink-block-views/🟦️.tsx` lines 88–101 define a pointer-active common block div and attach `onPointerDown`; text, math, table, image and group use it. `👆️ink-pointer/🔶️abort-gesture/🟦️.ts` lines 102–109 unconditionally stop propagation, then publish a selection before checking the active utility. This prevents the root handler at lines 51–86 from starting pencil, stroke/point eraser, pan (including middle button and Alt), marquee, or block placement when the pointer starts over one of these blocks. Locked blocks also stop the event before returning. Hyperlinks additionally stop propagation in `ink-block-views` line 48.

Stroke blocks are different: lines 156–164 render a `pointer-events-none` SVG without the common div or its handler. An exposed stroke therefore does not itself intercept an eraser start; an overlapping pointer-active block can.

## Multiple selection on drag

`beginMove` at `abort-gesture` lines 12–26 uses the current `selectedSet` and `selectedIds`, so pressing a selected member initially gathers the full selection as move origins. However the block handler publishes a replacement selection containing only that member immediately beforehand (line 108), unless Shift is held. The root hit-test path also publishes replacement at lines 94–96. The group can initially move because the callback captures the pre-update selection, while the selection and subsequent operations collapse to one member.

## Coverage

The initial filename-based test search missed tests named `🟦️.tsx` under `🧪️tests`; the renderer does contain ink canvas editing and interaction tests. Their exact coverage of these cases was not established. A real pointer interaction starting on a non-stroke block and a multi-selection drag should be covered to verify routing and selection retention.

## Eraser Follow-up

React geometry `📐️ink-geometry/🟦️.ts` lines 215–221 removes recorded vertices within the point eraser radius; it does not intersect segments. A visible segment with both endpoints farther than the radius remains unchanged when clicked in its middle, even though whole-stroke hit testing at lines 194–203 uses point-to-segment distance. The point eraser can therefore miss sparse or fast strokes.

The point eraser flattens grouped strokes and emits `removeBlock` followed by `addBlock` fragments without parent or index (lines 238–245). Local application `🌳️ink-document-ops/🟦️.ts` lines 145–156 appends those fragments to root, so a split nested stroke leaves its group and changes stacking order. Native WGPU scan `🔁️ink-interaction-job-scan/🦀️.rs` lines 236–245 also emits fragment add events without a parent.

React begins an eraser stream at pointer down if it has events (`abort-gesture` lines 64–68), streams further events during movement, and commits on pointer up (`handle-pointer-move` lines 68–70, 128–129; `ink-gestures` lines 69–101). The probe draws 12 explicit sampled movements and clicks sample 6 for point erase and sample 2 for whole erase (`🐍️note-acceptance.mjs` lines 443–475). Assuming the scene is clear, transform and persistence work, and both clicks hit their intended samples, the expected counts and three undo order are consistent with splitting into two fragments, removing the first, restoring it, restoring the original, then removing the drawn stroke. The sampled midpoint does not cover the sparse-segment defect, and count-only checks cannot establish fragment identity, geometry, hierarchy, or history contents. Native plugin mutation/history internals were not located in this follow-up; the sequence remains an expectation rather than observed proof.

## Native WGPU Point Eraser Trace

All paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state`.

`🔁️ink-interaction-job-scan/🦀️.rs::prepare_eraser_block` lines 250–263 checks the stroke hit, then calls `🧾️ink-document-model-and-edit-helpers/🦀️.rs::erase_ink_stroke_points_in_item`. The latter lines 298–311 only tests distances to recorded vertices and returns the original block when all vertices survive. A click on a long segment beyond the radius from its endpoints consequently performs no point erasure even after the segment hit succeeds.

`🔎️ink-interaction-document-and-block-scanning/🦀️.rs::ink_collect_block_object` lines 168–180 recursively collects group children for scanning. `🔁️ink-interaction-job-scan/🦀️.rs::push_pending_eraser_event` lines 236–245 emits remove followed by fragment additions. `📤️ink-event-pages-and-edit-routing/🦀️.rs::push_add_block_raw` lines 132–143 serializes exactly an `addBlock` operation and its `block`, without `parentId` or `index`; this payload does not preserve the original group or stacking position.
