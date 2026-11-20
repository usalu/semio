# Revise Tank to Chart Morph

## Goal
ENERGY — Beat1_Leistung 1 kWh tank → power-time area.

## Problem
The `fade` crossfade left two overlapping ghosted rectangles mid-morph.

## Fix
1. Tank frame dissolves first.
2. Tank fill reshapes into the chart area with recipe `replace` (single Transform, no crossfade ghosts).
3. Chart area is a `Rectangle` (same family as the tank fill) so the reshape stays axis-aligned.
4. The `1 kWh` label relocates with `ReplacementTransform`.
5. Preview profile runs full morph for `replace` / `hull` / `move` (not hard PreviewSwap).

### Verify
`videos/preview/de/speaker/rendered/Beat1_Leistung_480p15.mp4`
