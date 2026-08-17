# UI Chrome Umbrella Registrar Integration

## Preconditions

- Shared React barrel matched pre-edit SHA-256 `e82f73a9fd61e5d140d69f7df7498fa1afcd2217fde523fb6f64c9e130844e81`.
- Storybook smoke spec matched pre-edit SHA-256 `0ed906d63e572030e6615cad3b1d2867e3d4c697e6c4446d7167c0f080c94fd1`.
- Terra distributed every old Chrome responsibility and deleted the old source without editing either registrar.

## Changes

- Window silhouette geometry imports now resolve to `🪟️WindowSilhouette` while preserving the public package symbols.
- `ChromeControlHint` now resolves to its specific component while preserving its package export.
- Window-content dead-line behavior now resolves to `🚧️WindowContentDeadLine` while preserving public symbols.
- Removed the dead standalone `LoadingRow` package surface after its implementation became private to Skeletons.
- Removed the obsolete standalone LoadingRow Storybook smoke ID.

## Validation

- Final React barrel SHA-256: `0f8def42b5703b2ab00bd31f6e7b242e334ea9f60fdd9a5d35c1a88fdf8fa401`.
- Final Storybook spec SHA-256: `6ffb3b9befd21d25ebaacda3db95a6174d49a9fa16144de5582c4b9992e50eed`.
- No old Chrome path, public LoadingRow surface, or obsolete smoke ID remains in either registrar.
- All three specific destination imports are present.
- Ordinary and cached scoped `git diff --check` passed.
