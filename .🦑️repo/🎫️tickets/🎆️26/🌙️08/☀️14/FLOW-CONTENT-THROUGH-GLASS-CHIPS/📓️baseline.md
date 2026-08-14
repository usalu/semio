# Baseline

- React already measures top/bottom chip spans and generates the concave silhouette border, but payload layout begins below the cap row.
- Existing CSS correctly makes gap cells transparent and applies the shared `ui-glass` formula only to chips/body cells.
- WGPU Dock currently paints a rectangular full-cap glass region and starts content below the cap.
- The current Storybook long-test runner executes only the puzzle specification rather than the configured suite.
- `bun nx run @semio-tech/ui-styling:test` previously passed 25 tests during read-only planning.
- Three targeted React silhouette geometry tests previously passed under the long-test budget during read-only planning.
- WGPU tests were not observed because another process held the Cargo build lock.

These planning baselines are not acceptance evidence for the implementation; all relevant gates must be rerun after integration.
