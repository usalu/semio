# Flatten Utility and Tool Option Trees

## Problem
Activating Puzzle 3D Transform showed "Transformieren" twice (group picker + leaf toggle) and never showed Move/Rotate option flags.

## Root causes
1. Lone `group: "transform"` utility became a one-child collection with the same label as the child.
2. `partition_window_measures` kept the tagged utility-options Group as a rendered tree root (duplicate utility name).
3. Transform had no utility-option measures / runtime gumball flags.

## Fix
- `derive_utility_nodes` / `deriveUtilityNodes`: hoist single-child groups to a top-level toggle.
- `partition_window_measures` / `partitionWindowMeasures`: unwrap tagged utility groups — children become flat `utilityOptions` (wrapper is routing-only).
- Puzzle 3D: drop unnecessary `group` on transform; add Move/Rotate toggles + `setTransformGumballFlag`; emit `gumballConfig` on selection JSON; World3d prefers plugin `gumballConfig`.
