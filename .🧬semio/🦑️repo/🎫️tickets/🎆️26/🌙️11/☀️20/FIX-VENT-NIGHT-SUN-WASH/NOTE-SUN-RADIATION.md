# Sun Radiation Wash

## Problem
Opacity `0.08` made the wash muddy/nearly invisible; extra guide rays stacked yellow fills.

## Fix
- One exterior `Polygon` wash onto the blinds at `0.22`
- One interior leak through the window at `0.10` (no overlap with exterior wash)
- Removed stroke guide rays

## Check
Shade beat (~16s): clear soft yellow beam from sun onto Verschattung; faint leak into Speichermasse.
