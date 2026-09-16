# Sourcing grid shows curation only

## Problem

The **Grid** window (`sourcing-grid` / `sourcing.grid`) rendered `filtered_stock` — the same catalogue slice as the **Pool** — so the 3D layout showed the full demo stock on a fresh document instead of the curated pick list.

## Fix

- `grid::render` now lays out slots from `curated_rows` (shared with the **Curated** table), repeating each row by its `count`.
- Mesh collection deduplicates non-box kinds when counts expand the same `object_id`.
- Demonstrator acceptance: `sourcing-grid` expects empty scene on boot (aligned with `sourcing-curated`).

## Tests

- `grid_instance_count_matches_curated_counts_and_normalizes_scale`
- `grid_renders_empty_when_nothing_is_curated`
- `renders_via_the_app` curates one kind before asserting the grid scene
