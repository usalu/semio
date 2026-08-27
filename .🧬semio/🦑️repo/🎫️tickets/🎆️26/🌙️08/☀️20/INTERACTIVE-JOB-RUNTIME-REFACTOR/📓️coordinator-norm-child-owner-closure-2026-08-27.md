# Norm Child Owner Closure

## Scope

Removed the two Norm process-global payload maps: EN 1990 variable-action rows and DIN 18599 monthly climate data.

## Exact changes

- `En1990QkWorkingTable` is attached to the exact `En1990QkChild` returned by `en1990_qk_child_from_entries`.
- `Din18599ClimateWorkingData` is attached to the exact `Din18599ClimateChild` returned by `din18599_climate_child_from_data`.
- Both read paths resolve only the addressed child's typed local owner and fail soft for wire-only handles awaiting host materialization.
- Equal serialized identities cannot observe another child owner's payload; owner state retires with the child.
- One language-neutral JSON fixture and one serde_json oracle test were added for each artifact.

## Validation

- Bun parsed both fixtures and checked all exact booleans: green.
- Removed-symbol/source scan for both payload statics, `thread_local!`, `RefCell`, and `HashMap`: green.
- `git diff --check -- '✏️s/🔌️plugins/📕️norm'`: green.
- Rust compiler/rustfmt validation remains queued behind Flow's exclusive compiler lease; no compiler-green claim is made here.
