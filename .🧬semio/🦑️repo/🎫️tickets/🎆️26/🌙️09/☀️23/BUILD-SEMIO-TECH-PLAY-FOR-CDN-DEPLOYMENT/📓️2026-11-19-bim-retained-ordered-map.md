# BIM snapshot maps on `RetainedOrderedMap`

The abandoned migration is finished by migrating the remaining fields and callers fully to `RetainedOrderedMap`; no `BTreeMap` shim was added.

- `PhaseVisibility.visible` is now `RetainedOrderedMap<String, Vec<String>>` (fixes E0277 on `RetainedClone`); `table_json` rebuilds its oracle `BTreeMap` from `iter()`; `openings` and `wall_sweeps` loops use `.iter()`.
- `ImportedIssues.issues` and `.comments` are `RetainedOrderedMap` and filled with `cold_insert`, so `BcfIntoModel` hands them to `ModelSnapshot` unchanged (fixes E0308).
- `PropertySetPatch::write` and `ClassificationSetPatch::write` edit the cold map with the framework's `cold_insert`, `cold_remove` and `get_mut` (fixes E0599; the framework deliberately has no `entry`, `insert` or `remove`, only the cold, standalone-construction API and the paid retained edit cursors).
- Framework: `impl IntoIterator for &RetainedOrderedMap` in `🗂️map-core/🦀️.rs`, so `for (id, row) in &snapshot.collection` keeps working.

Verification: `repo cargo --store=shared --configuration=release -- check -j4 --message-format short --target wasm32-wasip2 --manifest-path <bim model Cargo.toml>`, log under `🗑️generated/bim/`.

## Static review (no cargo run; the coordinator killed the deadlocked check)

- Snapshot collections are all `RetainedOrderedMap`, so downstream callers moved to the cold API: about 610 mechanical rewrites in 128 files of `<snapshot|model|i.model|base|bare|next|...>.<collection>.insert|remove|clear|extend|retain(` to `cold_insert|cold_remove|cold_clear|cold_extend|cold_retain(`. Receivers that are other structs (`ModelInference`, export links, removal sets, gbXML plan) were left alone.
- Hand-fixed `entry` sites: IFC import of `properties` and `classifications`, the place-grid-columns test material, `PropertySet::from` in set-element-property; `user_rows` now returns the retained map and uses `cold_retain`.
- `KeyedDelta::write_into` and `inverse` take and return `RetainedOrderedMap`, matching the `ModelDiff` macro.
- SQLite snapshot reader/writer (`write_map`, `read_map`, `read_properties`, `write_maps`, `read_scalar_map`, `read_maps`) read and write `RetainedOrderedMap` with `cold_insert`.
- `range(prefix..)` over `family_parameters` and `component_overrides` became `skip_while` plus `take_while`; helper signatures (`dep_types`, `present`, `field_codecs::keys/first`, `library_type`, option-scope `effective`) take `RetainedOrderedMap`.
- Framework `RetainedOrderedMap` gained read-only `IntoIterator for &`, `Index<&Q>` and `get_key_value`; still no `entry`, `insert`, `remove`.
- Not verified by a compiler: remaining risk is rewrites on same-named receivers that are not snapshots, and any `&mut`/owned iteration of snapshot maps (none found).
