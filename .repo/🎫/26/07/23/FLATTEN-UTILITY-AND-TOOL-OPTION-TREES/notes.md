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

## Verification
- `cargo test -p ui_wgpu --lib -- partition_window_measures derive_utility_nodes` → **7 passed** (isolated `CARGO_TARGET_DIR` under this ticket).
- `bunx vitest run framework/renderer/react/index.test.ts -t "partitionWindowMeasures|deriveUtilityNodes|gumball"` → **8 passed**.
- Full `puzzle-plugin` suite could not be compiled in this session: concurrent UiPresence migration left plugin tree constructors mid-refactor (`loading`/`waiting`/`selected_ids` → `presence`). Transform measure/action code is in place and will compile once that migration finishes.
