# UI Toggle and Breadcrumb Registrar Integration

## Preconditions

- Shared React registrar matched pre-edit SHA-256 `fdd7e8ec24ea5288b386bab04f2627d81194712e2461860e8e2abcead71a4a23`.
- Toggle/ToggleGroup source ownership split and Breadcrumb source/CSS deletion both completed without touching the registrar.

## Changes

- `Toggle` now imports/exports from the specific `🎚️Toggle` component; `ToggleGroup` and `ToggleGroupItem` remain under `🎛️ToggleGroup`.
- Removed the unconsumed public `toggleVariants` glue and the unused package-level Radix Toggle/ToggleGroup namespace imports.
- Removed the zero-consumer Breadcrumb import/export region.
- Reduced the mixed Breadcrumb/Command hover test to its surviving Command responsibility.
- Removed Breadcrumb-only assertions from the shared shell-border test while preserving all other assertions.

## Validation

- Final registrar SHA-256: `e82f73a9fd61e5d140d69f7df7498fa1afcd2217fde523fb6f64c9e130844e81`.
- No `Breadcrumb`, `toggleVariants`, `TogglePrimitive`, `ToggleGroupPrimitive`, old Toggle ownership, or alias residue remains in the registrar.
- The specific Toggle and ToggleGroup import/export regions resolve to their respective component paths.
- Ordinary and cached scoped `git diff --check` passed.
