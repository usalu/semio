# CollectionOperation unify

- VCS is the single shape: `Add { index, item }`, `Move { id, to_index }`
- `spr/command` reexports VCS collection types/traits/fns (no duplicate enum)
- `Patchable`: `apply_patch` + `diff_patch` (plugin-compatible)
- Call sites across ✏️s migrated off `Add { id, at }`
- `semio-s-plugin-architect` cargo check --lib GREEN
