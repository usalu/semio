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

## Item 41 path attribution

The contemporaneous report records roughly 610 call-site changes in 128 files. No independent path manifest from that edit has been recovered. The read-only archived patch from `3d62b2ef22c` to `5db034ffe53` includes that migration and concurrent work. The reconstruction below retains only hunks that match this report.

### Documented hand edits

The archived hunks agree with the named changes to phase visibility, BCF issue maps, IFC data, material creation, map helper signatures, SQLite, the set-element-property fixture value, and keyed delta application. Paths are repository-relative here.

- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏛️place-grid-columns/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🔤️field-codecs/🦀️.rs`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💬️bcf/🦀️.rs`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🏗️ifc/🪑️components/🦀️.rs`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🏗️ifc/🧬️data/🦀️.rs`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎭️phase-visibility/🦀️.rs`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭️option-scope/🦀️.rs`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪑️components/🦀️.rs`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧮️algebra/🦀️.rs`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/🔺️diff/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🗂️map-core/🦀️.rs` adds borrowed iteration, keyed indexing and `get_key_value`. Include those three hunks.
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️effect/🧳️frame/🎛️driver/🦀️.rs` is the separate value helper reported in the same redeploy item. Include the `drive_original_effect_bound_with` wrapper and its dispatch, as documented in the [value refactor report](📓️2026-11-18-value-refactor-completion.md).

The later BIM compile audit covers inference-owned map fields and nested fixture type adjustments. For shared files, the Item 41 hunk scope here covers snapshot-map calls and the hand edits above. The original `ModelSnapshot` field type migration was already present when the work began. The archive also contains unrelated BIM source and lockfile changes, so file-wide attribution would overstate the edit. [Later compile audit](📓️2026-11-19-bim-compile-audit.md)

### Archived snapshot-map call-site roster

There are 132 paths with at least one archived removed and added line that differs by an `insert`, `remove`, `clear`, `extend` or `retain` call becoming `cold_*` on a named retained field of `ModelSnapshot`. The paired-line scan ignores whitespace and can include edits from concurrent passes when they produced the same replacement. This is a corroborated path roster for the matching call hunks, with code and test scope stronger than a directory diff. It cannot recover the exact original 128-path set or individual authorship in an archived hunk.

One path sits outside the standard subset. It is `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🧪️tests/🔬️unit/🦀️.rs`. All paths below are relative to `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.

- `✏️editor/🎭️modes/✏️edit/🪟️windows/📐️section/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️family/✏️edit/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🧮️schedule/✏️edit/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🧮️schedule/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/🎚️set-override/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/🏛️place-grid-columns/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/📊️export-schedule-csv/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/📋️edit-schedule/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/📚️edit-classification/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/🔍️search-classification/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/🔭️create-view/🦀️.rs`
- `✏️editor/🎮️commands/🗂️set-classification/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/🗄️remove-classification/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/🤝️coordinate/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/🧮️edit-template/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/🧰️apply-template/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/📌️panels/🌳️outliner/🖼️views/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/📌️panels/🌳️outliner/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/📌️panels/🔍️properties/🎚️overrides/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/📌️panels/🔍️properties/🏷️data/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/📌️panels/🗂️classification/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/📌️panels/🤝️coordination/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/📌️panels/🪑️families/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧩️entities/🌀️mep/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧩️entities/🏗️frame/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧩️entities/🏷️psets/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧩️entities/🕰️phasing/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧩️entities/🤝️coordination/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧩️entities/🧷️wall-sweeps/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧩️entities/🪑️components/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧪️tests/⌨️completeness/🦀️.rs`
- `✏️editor/🧪️tests/🧰️fixture/🦀️.rs`
- `✏️editor/🧵️gestures/✂️reshape/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧵️gestures/🎯️select/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧵️gestures/🏗️frame/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧵️gestures/📏️annotate/🦀️.rs`
- `✏️editor/🧵️gestures/🧩️area/🦀️.rs`
- `✏️editor/🧵️gestures/🧩️area/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧵️gestures/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧵️gestures/🧱️chain/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧵️gestures/🧲️snap/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧵️gestures/🧷️sweep/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧵️gestures/🪑️place/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧵️gestures/🪟️opening/🧪️tests/🔬️unit/🦀️.rs`
- `📚️examples/🏢️office/🧪️tests/🧩️example/🦀️.rs`
- `🚪️io/💬️bcf/🦀️.rs`
- `🚪️io/💬️bcf/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🎨️svg/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/🌀️mep/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/🏘️zoning/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/🏰️walls/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/🏰️walls/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/📚️ifc4/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/🔥️energy/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/🔲️ceilings/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/🧪️tests/🧰️testkit/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/🧬️data/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/🧷️wall-sweeps/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🏗️ifc/🪑️components/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/📄️sheets/✒️ink/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/📄️sheets/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/📄️sheets/🧪️tests/🧰️testkit/🦀️.rs`
- `🚪️io/📤️export/📊️csv/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📤️export/🧱️holders/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/⬜️horizontal/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🌀️mep/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🏗️elements/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🏘️zoning/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🏘️zoning/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🏛️spatial/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🔥️energy/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🔲️ceilings/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🔲️ceilings/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🛝️ramps/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🛝️ramps/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🧬️data/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🧭️options/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🧱️walls/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🧷️wall-sweeps/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🪑️components/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🪜️circulation/🦀️.rs`
- `🚪️io/📥️import/🏗️ifc/🪟️curtain/🦀️.rs`
- `🧬️schema/💡️inferences/⚖️rule-results/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/⚠️diagnostics/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🌀️mep/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🌡️energy-envelope/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🎨️finishes/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🎭️phase-visibility/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🏘️zones/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🏠️spaces/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🏷️effective-properties/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/📄️sheet-layout/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/📋️schedules/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/📋️schedules/🧪️tests/🕸️graph/🦀️.rs`
- `🧬️schema/💡️inferences/📋️schedules/🧪️tests/🗂️rows/🦀️.rs`
- `🧬️schema/💡️inferences/🕸️model-graph/🧪️tests/📈️incremental/🦀️.rs`
- `🧬️schema/💡️inferences/🕸️model-graph/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🕸️model-graph/🧪️tests/🧬️families/🦀️.rs`
- `🧬️schema/💡️inferences/🕸️model-graph/🧳️instance/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🖼️view-linework/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🗺️plan-linework/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🛝️ramp-runs/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🦴️analytical-members/🦀️.rs`
- `🧬️schema/💡️inferences/🧊️element-solids/➖️beams/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🧊️element-solids/🌀️mep/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🧊️element-solids/🚪️fillers/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🧊️element-solids/🛤️railings/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🧊️element-solids/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🧊️element-solids/🪟️curtain-walls/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🧪️tests/🧰️schedule-kit/🦀️.rs`
- `🧬️schema/💡️inferences/🧬️families/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🧬️families/🩺️findings/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🧭️option-scope/🦀️.rs`
- `🧬️schema/💡️inferences/🧮️quantities/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🪜️stair-runs/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🪞️curtain-layout/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/💡️inferences/🪧️annotation-layout/🦀️.rs`
- `🧬️schema/💡️inferences/🪧️annotation-layout/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/📏️authored/🪜️storeys/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/📏️authored/🪟️sizes/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/📸️snapshot/📄️sheets/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/📸️snapshot/🖼️views/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/📸️snapshot/🤝️coordination/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/🧬️mutations/🔌️component-rules/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/🧬️mutations/🔩️family-rules/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/🧬️mutations/🧙️modify/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/🧬️mutations/🧪️tests/occurrence.rs`
