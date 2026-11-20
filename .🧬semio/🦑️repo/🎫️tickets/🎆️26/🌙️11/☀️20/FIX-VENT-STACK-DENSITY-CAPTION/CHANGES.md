# Shorten Beat4 Density Caption For Sideview

## Cause
`caption_bar` max 2 lines; density DE clause wrapped to 3 → `ValueError` mid-render.

## Fix
Drop the second sentence about $\Delta\rho(\theta)$ (covered in the next beat).
