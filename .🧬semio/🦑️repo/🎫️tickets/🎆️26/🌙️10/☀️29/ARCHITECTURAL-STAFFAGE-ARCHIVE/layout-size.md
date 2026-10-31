# Stable card sizes

A walking figure used to publish its paper size into the page: `fit="contain"` set an aspect ratio from the current frame, and that ratio changed the card, the grid row and the main page.

The figure now fills the box its parent already has. The drawing keeps its paper aspect inside that box (`preserveAspectRatio`) and is clipped there. Previews, friezes, the stage and the main page use `overflow: hidden` and layout containment, so the gait cannot change their size.

Checked in Chrome at 1440×1000 after the page settled: `main`, the hero, a category card and the compare card kept one size across ten samples while the figures were moving.
