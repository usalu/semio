# Sourcing pool window — clean curated controls

## Problem

The **Pool** table (`sourcing-pool`) stacked three overlapping ways to change curated count:

1. **Kuratiert** column — table stepper (`curationSetCount`) with minus/plus controls.
2. **Aktionen** column — duplicate `curationAdd` / `curationRemove` row buttons with plus/minus icons.
3. **Renderer bug** — stepper buttons passed both `icon="plus|minus"` and literal `+` / `−` children, so each control read as `+ +` or `- -`.

That made the window noisy and crowded the right side of the grid.

## Fix

- **Pool plugin** — drop the `actions` column entirely; the **Kuratiert** stepper is the single control (same command family as the curated table's count column).
- **Table host** (`Table/🟦️.tsx`) — stepper increment/decrement buttons are icon-only, matching row-action buttons.

## Tests

- `pool_row_has_no_actions_column_and_only_a_curated_stepper`
- `pool_scene_names_columns_by_id_and_drops_onto_the_pool` (five columns, ends at `curated`)
- `curation_add_updates_the_pool_stepper_and_curated_table`

## Out of scope

The **Curated** window still pairs a count stepper with quick-add / remove-all row actions by design (different workflow). Pool is catalogue browsing — one stepper per row is enough.
