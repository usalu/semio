# Fix Flow Slider Knob Zoom Scaling

## Bug

Flow graph slider knob (DOM thumb) stays fixed CSS `size-small` while the overlay box width/height and GPU-painted nodes/text scale with camera zoom.

## Root cause

`GraphSliderOverlays` multiplies track bounds by zoom for the overlay box, but `Slider` chrome (thumb `size-small`, track `h-single`) uses unscaled design tokens.

## Fix

Lay out the overlay in world units and apply `translate(-50%, -50%) scale(zoom)` so thumb, track thickness, and box all scale with the canvas camera — same model as other zoomed canvas DOM content.
