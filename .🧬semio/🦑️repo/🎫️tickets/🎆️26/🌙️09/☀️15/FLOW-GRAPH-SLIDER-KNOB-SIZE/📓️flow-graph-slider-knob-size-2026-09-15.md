# Flow Graph Slider Knob Size

## Report

Slider knobs on procedural 3D (and other flow) graph nodes looked oversized relative to the thin track.

## Cause

`GraphSliderOverlays` scales the whole `Slider` with camera zoom, but the thumb still used the default `size-small` token (~16px) on an `h-single` track (~3px). The overlay layout box also enforced a 16px minimum height, larger than the DAG `slider_track_bounds` hit height (8 world units).

## Fix

- `Slider` gained optional `thumbClassName`; graph overlays pass `size-tiny` instead of default `size-small`.
- Match layout height minimum to DAG track bounds: `Math.max(slider.h, 8 / zoom)` instead of `16 / zoom`.

## Verification

- Renderer engine-contract vitest: graph slider zoom scaling case asserts compact thumb class.
