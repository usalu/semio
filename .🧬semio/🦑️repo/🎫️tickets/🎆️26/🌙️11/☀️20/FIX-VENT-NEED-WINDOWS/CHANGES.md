# Richer Windows On Ventilation Need

Scene: `Beat1_Lueftungsbedarf`.

## Problem
`_window` forced `window_opening` at construction, so both openings read as flat teal wedges.

## Changes
1. Closed casement by default: thicker wall, glass pane, sill/lintel rails, cross mullion.
2. Fresh-air beat opens both sashes with `open_window(..., fraction=0.55)`.
