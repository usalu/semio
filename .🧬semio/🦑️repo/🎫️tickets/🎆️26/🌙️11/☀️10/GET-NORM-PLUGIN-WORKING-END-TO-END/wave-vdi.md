# wave-vdi report (vdi3805)

Result: `check -p` of all 19 vdi3805 crates in the hub component closure via `T/wave-vdi-check.sh` (log `T/🗑️generated/wave-vdi-9.log`): 0 errors, exit 0.

## Crates, errors before -> after
- vdi3805-model 199 -> 0; diff 1 (E0117) -> 0; io-text 4 -> 0; viewer 29 -> 0; editor 1 -> 0; root `vdi3805` 38 -> 0; io, inferences, scene, mutations, schema, mut-*: clean.

## Root causes and fixes
1. BTreeMap in RetainedClone types: `Vdi3805Snapshot.{edition_profile,geometry,curves}`, `ParametricGeometry.parameters`, `ExtensionBag.fields` are `RetainedOrderedMap<String, _>` (model, schema artifact, diff rows, `ChangeGeometryParameters.new_parameters`, reference fixture, sqlite codec, inferences/scene loops `.iter()`, tests use `cold_insert`/`cold_clear`/`.get`).
2. `LocalizedText` Serialize/Deserialize: model `test-support` now enables `semio-s-plugin-norm-compliance/compliance-testing` (the serde derive on `LocalizedText` is gated on that feature).
3. Orphan impls (din pattern): sqlite codec + `impl_norm_artifact_record!` (new `io/🪞️records`) mounted in the model crate; text/binary mutation codecs mounted in the mutations crate (`OpBinaryReceiving` added); text/binary diff codecs mounted in the diff crate; io-text/io re-export them. Manifest deps added to model/mutations/diff (`pack`, `native-owned`, `contract`, `diagnostic`, `compliance`).
4. diff: `Vdi3805ProductsRows` uses `key: String = by Vdi3805ProductKeys` with a local `KeyOf` marker (foreign row type).
5. Root crate: `From<VdiQuantityKind> for QuantityKind` moved into the model crate (orphan); missing `SheetId`/`Domain`/`SHEET_ENTRIES` imports.
6. Viewer/editor: `norm_viewer_commands!` takes the full model path; `semio-framework-job` + `semio-framework-action-bus` deps; editor imports `viewport_act`, test-only macros `#[cfg(test)]`, catalogue render gets `TreeWindows`.

## Left / notes
- Test targets not checked (`--tests`); test sources updated for the new map API but not compiled. Tests under moved io files reference old `crate::` paths in places.
- Cargo.lock of the vdi3805 workspace updated by cargo.
