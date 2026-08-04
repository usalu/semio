# Fix Table Edge Stub Lines

## Problem
Horizontal row rules overshoot the table side borders by ~0.5 hairline, producing small stubs at every row/column join on the right (and left) edge. Windowed short tables also paint duplicate L/R borders (tcolorbox frame + table `\vrule`s).

## Fix
1. `\semio@table@rule` uses natural alignment-width `\hrule` (no `width\linewidth`).
2. Window short-table colspec drops table-side borders; window frame owns L/R.
3. Remove duplicate TikZ left finish on `semio~window~table`.
