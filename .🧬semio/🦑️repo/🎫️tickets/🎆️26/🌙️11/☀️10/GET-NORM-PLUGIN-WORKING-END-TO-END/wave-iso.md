# wave-iso report (iso16757)

Result: `check -p semio-s-artifact-norm-iso16757` (aggregate root, transitively every iso16757 crate: model, schema, diff, mutations, inferences, checks, io-text, io, scene, viewer, editor, examples) via the hub component package: 0 errors, exit 0 (log `T/🗑️generated/iso-7.log`).

## Errors before -> after
- iso16757-model 473 -> 0; then surfaced and fixed: scene 3, checks 3, io-text 3, io 1, editor 22, root 12 -> 0.

## Root causes and fixes (all under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757`, snapshot taxonomy = `🏅️standards/🔖️1/🪆️subsets/✳️any`)
1. BTreeMap in RetainedClone types: new `IdMap<V> = RetainedOrderedMap<String, V>` in model `🧱️catalogue-core` (re-exported at crate root) used by ExtensionBag, ProductSeries/ProductVariant values, Catalogue accessories/compositions, GeometryNode::Primitive, GeometryObject bindings, GeometryCatalogue objects, PartNumberRule::Table rows, snapshot and schema `part_number_inputs`. Callers moved to `cold_insert`/`cold_remove`/`.iter()`/`into_entries()` (diff keyed rows, part-libraries, scene, checks, sqlite).
2. Missing `DslField`/`BorrowedDslField` for `CatalogueId`/`CatalogueValue`/`PartNumberRule`: they were orphan impls in io-text. Moved with their native decoder into the model (`🧬️schema/📸️snapshot/🛬️native-syntax`), `PartNumberRule` rows built as `IdMap`.
3. Stale `retire_with` path on `GeometryNode` fixed to `crate::retire_decoded_catalogue_geometry`.
4. Orphan impls (E0117): the din/en1991 pattern. Model mounts `io/🪶️sqlite` and new `io/🪞️records` (`impl_norm_artifact_record!`, with `extern crate self as ..._model`); mutations crate mounts `io/📝️text|💾️binary/🧬️mutations` (`extern crate self`, `OpBinaryReceiving` via `original_pack_value_operation_receiving!`); io-text/io re-export (`pub use ..._mutations::text as mutations`, `..._mutations::binary as mutations`, `..._model::sqlite`, `..._mutations::mutation_bridge::*`). Manifest deps added to model (pack, native-owned, artifact-reference) and mutations (diagnostic, compliance, dsl-record(-derive), contract).
5. `LocalizedText` Serialize/Deserialize under `test-support`: **shared file** `⚖️compliance/📦️packages/🦀️rust/Cargo.toml` `test-support` now forwards `semio-s-plugin-norm-compliance-core/compliance-testing` (vdi3805 hits the same error).
6. Misc: `Shape::kind()` added to solid-volume (scene diagrams read `leaf.kind` that no longer exists), `Vec3` array index cloned in scene layout, missing `doc` parameter in `check_geometry_ports_and_bindings`, editor imports `viewport_act` and gates `norm_live_acceptance!` with `#[cfg(test)]`, `semio-framework-job`/`-action-bus` in editor/viewer manifests, viewer macro takes the full model path, `semio-framework-value` in the root manifest.

## Left / notes
- Test targets not checked. Test sources were updated mechanically (BTreeMap -> IdMap, `insert` -> `cold_insert`, indexing -> `get`) in schema compliance tests, cross-checks fixture + its python generator, scene, mutations, sqlite tests; unverified.
- Moved io files keep `#[cfg(test)]` mounts that reference `crate::text`/`crate::mutations` of the old io crate layout; test targets may need path touch-ups.
- `🧬️schema/📸️snapshot` `SNAPSHOT_LEAVES` now includes the new native-syntax file text.
