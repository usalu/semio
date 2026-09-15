# Puzzle 3D object tree labels

## Problem

The document outliner showed object **kind ids** (or fell back to opaque ids) while the 3D view showed authored **labels**. Fill, brush, and suggestion placements created objects with `label: None`, so the tree could not distinguish instances and did not match what users saw in the viewport.

## Fix

- **`puzzle3d_object_display_label`** — same precedence as the 3D instance lane: `label` → catalog display name → `id`.
- **`puzzle3d_next_object_label`** — assigns the catalog name for the first instance of a kind; further instances get ` 2`, ` 3`, … using the root taken from existing peer labels (e.g. `Hexagonal Cut Concrete Forest Left 2`).
- Outliner **`object_row`** now uses `puzzle3d_object_display_label` instead of raw `objectKind`.
- Fill **`fill_run_ops`** stamps labels on `create_object` mutations; brush / add-object / accept-suggestion / duplicate paths set labels at publish time.

## Tests

- `puzzle3d_next_object_label_increments_from_the_authored_seed_name` in editor unit tests (Concrete Forest seed → `… Left 2`, then `… Left 3`).
