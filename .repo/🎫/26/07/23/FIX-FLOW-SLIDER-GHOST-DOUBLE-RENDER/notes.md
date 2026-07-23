# Fix Flow Slider Ghost Double Render

## Bug

At rest, flow graph sliders showed a pixelated second track+knob above the crisp DOM slider. Only while dragging did the correct (HTML) slider become clearly visible.

## Root cause

After `GraphSliderOverlays` was added for interactive editing, the DAG GPU paint path still drew track + knob (+ value). That GPU chrome sat misaligned above the DOM control (especially because the overlay used the full Slider with a value column that compressed the track).

## Fix

1. `infinite/board/port/directed/dag/rs/lib.rs` — stop painting track/knob on GPU; keep left value readout.
2. `ui/js/react` Slider — add `showValue` (default true) for track-only mode; apply `className` when not labeled.
3. `GraphSliderOverlays` — `showValue={false}` so the overlay fills track bounds exactly.
