# Degree-Day Chart Fill

## Problem
Scene2 shaded $G_t$ with day bars / one concave year polygon. Manim triangulation left vertical black slits (especially near May/October).

## Fix
- Visual shade: $\theta_e < \theta_i$ (band between room line and outdoor curve). $G_t$ still sums Heizgrenze days only.
- `_degree_day_band` builds **convex day trapezoids** with `x_overlap = 1.25` so 480p seams do not show as dark slits.
- Fill behind the temperature curve (`z_index`), opacity ~0.72.

## Validation
- `DegreeDayFillTest` — 8 tests OK.
- Preview: `videos/preview/de/speaker/s/63/v/Scene2.mp4` (full-year frame: continuous tan band Jan–Jun and Sep–Dec; Jul/Aug empty when $\theta_e \ge 20\,°C$).
