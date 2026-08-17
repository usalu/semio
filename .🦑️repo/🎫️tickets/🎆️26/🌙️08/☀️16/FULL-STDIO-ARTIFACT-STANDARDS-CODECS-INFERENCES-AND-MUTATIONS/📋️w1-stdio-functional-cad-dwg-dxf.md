# W1 Stdio Functional CAD DWG DXF

## Gate

The CAD functional source lane is frozen for the repository-wide `MutationDiff<Result>` migration. No Cargo, Bun, or Nx command was run. The current source therefore has static validation only; every runtime test below remains pending until the parent releases the serialized stdio build gate.

The already-built pre-change stdio test executable was used only to reproduce and classify the baseline failures. It does not validate the edited source.

## Reproduced Baseline Failures

### DWG AC1024 / relocated AC1018 implementation

- `artifacts::dwg::standards::v_ac1024::subsets::any::io::component::tests::codec_round_trip`
- `artifacts::dwg::standards::v_ac1024::subsets::any::io::component::tests::conformance_laws::fixture_honesty_law`
- `artifacts::dwg::standards::v_ac1024::subsets::any::io::component::tests::dwg_full_entity_set_round_trips`
- `artifacts::dwg::standards::v_ac1024::subsets::any::io::component::tests::dwg_mesh_bridge_round_trips_triangle_count_and_positions`
- `artifacts::dwg::standards::v_ac1024::subsets::any::io::component::tests::dwg_path_bridge_round_trips_cubic_control_points_exactly`
- `artifacts::dwg::standards::v_ac1024::subsets::any::io::component::tests::dwg_reader_skips_unknown_object_types_without_failing`

### Semio bridges

- `artifacts::semio::standards::v1::subsets::cad::io::import::deserializers::artifacts::dwg::v_ac1024::any::component::tests::produces_empty_but_valid_cad_snapshot`
- `artifacts::semio::standards::v1::subsets::drawing::io::import::deserializers::artifacts::dwg::v_ac1024::any::component::tests::buckets_entities_by_layer_in_entity_order`
- `artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::dwg::v_ac1024::any::component::tests::real_round_trip_through_relocated_dwg_codec`
- `artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::dwg::v_ac1024::any::component::tests::groups_polyface_mesh_by_layer_name`
- `artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::dwg::v_ac1024::any::component::tests::serialize_then_deserialize_round_trips_triangle_and_vertex_counts`

### DXF R12

- `artifacts::dxf::standards::v_r12::subsets::any::schema::inferences::bounds::component::tests::bounds_matches_hand_built_entity_extent`

## Root Causes

- The semio AC1015 writer and reader had diverged. The writer emits the structural entity layout, while the reader had been replaced by the native R2010 entity layout. Recognized entities consequently failed decoding and were silently discarded.
- The semio reader located the handle stream at the unrounded body bit length, although the writer byte-pads the entity body before appending handles.
- `DwgLogicalDrawing::from_native` retained only line, arc, and lightweight-polyline objects and discarded the remaining eight native entity kinds.
- `decode_dwg` attempted the R2004 path for AC1015 data, discarded typed decoder failures with `.ok()`, and synthesized default document sections.
- Recognized corrupt objects, missing layer references, and invalid object-map identities were skipped or defaulted instead of producing contextual typed errors.
- The unknown-object fixture inserted a new object-map entry before its bogus payload but did not shift the recorded payload address by the inserted entry length.
- The CAD bridge test supplied a header-only AC1032 byte stub rather than a codec-produced DWG.
- The demo DSL and pack fixtures predated the expanded snapshot schema.
- DXF circle and arc bounds are defined by this implementation as center plus/minus radius on all three axes; the hand-built expectation incorrectly retained the center Z minimum.

## Implemented Source

- Kept separate, explicit native R2010 and semio structural AC1015 entity decoders.
- Added the structural decoder for point, line, circle, arc, ellipse, lightweight polyline, spline, text, 3D face, 3D polyline, and polyface mesh in writer field order.
- Rounded the structural body storage size before opening its handle stream.
- Made recognized native and structural object failures contextual and fatal while retaining intentional skipping only for unknown object types.
- Made layer identities and entity layer references deterministic and validated.
- Added schema-owned `DwgGeometryEntity` projection for the eight native geometry variants not represented by dedicated DWG object bodies, retaining all eleven geometry variants without an opaque payload.
- Mirrored the projection through Rust DSL, TypeScript, JSON Schema, GraphQL, and Protocol Buffers facets.
- Made `DwgLogicalDrawing::from_native` fallible and propagated the result through DWG and Semio serializers.
- Dispatched AC1015 to the structural reader and later versions to the native R2004 reader without decoder fallback or default document sections.
- Replaced invalid test stubs with bytes produced by the structural writer.
- Corrected the unknown-object fixture address and DXF R12 Z-bound expectation.
- Regenerated the canonical demo DSL from the baseline schema-aware encoder.

## Exact Production Files

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️component.graphql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️component.graphql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️component.proto`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️component.proto`

The concurrent `no_op_diff.diff().is_empty()` change visible in the DWG I/O file belongs to the repository-wide `MutationDiff<Result>` migration and is not part of this lane.

## Ticket Evidence Files

- `derive-dwg-fixture.c`
- `derive-dwg-fixture.S`
- `derive-dwg-fixture.dylib`

These ticket-local helpers invoked the already-built baseline encoder to obtain a canonical DSL and pack trace without rebuilding the repository. They are evidence only and are not production inputs.

## Static Validation

- `rustfmt --edition 2021 --check` completed successfully for all eight edited Rust files.
- `git diff --check` completed successfully for all seventeen production files listed above.
- `jq empty` completed successfully for the edited JSON Schema facet.
- Static symbol inspection confirms `DwgGeometryEntity` is present in Rust, TypeScript, all three GraphQL facets, and all three Protocol Buffers facets.
- Cargo, Bun, Nx, and current-source runtime tests were intentionally not run under the parent migration gate.

## Pending Serialized Validation

All twelve reproduced tests listed above must run against the rebuilt current source after the gate opens.

`artifacts::dwg::standards::v_ac1024::subsets::any::io::component::tests::conformance_laws::fixture_honesty_law` additionally requires regenerating `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` from the newly compiled schema. The baseline helper produced a 729-byte pack for the pre-edit nested schema hash, so installing it would be knowingly stale. The binary pack was deliberately left unchanged until the current schema can produce and immediately verify it through the fixture law.

## Fallible Geometry MutationDiff Migration

### Scope and Contract

The STL ASCII, PLY 1.0, LAS 1.0, OBJ 3.0, STEP AP214, IFC 4, IFC 2x3, and DXF R12 root `MutationDiff` implementations now return `MutationApplyResult`. Every direct production consumer, mutation builder, and affected test in this lane handles the typed result explicitly.

Application validates all targets before cloning or mutating a candidate snapshot. Missing, duplicate, and out-of-range indexed or named targets return contextual `MutationApplyError` paths. Nested STEP arguments, PLY rows/properties/cells, DXF block entities, LAS collections, OBJ named collections, and IFC identities retain their full target paths. No application path clamps, silently skips, panics, aliases, falls back, or discards the typed error. Mutation outcomes and builders preserve the original snapshot and surface the failure as an error diagnostic.

### Exact Migration Source Files

#### LAS 1.0

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

#### PLY 1.0

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

#### OBJ 3.0

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

#### STEP AP214

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc1/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc2/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc3/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc4/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc5/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🧬️schema/🦀️component.rs`

#### IFC 4

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

#### IFC 2x3

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/🧬️schema/🦀️component.rs`

#### DXF R12

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

#### STL ASCII

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

### Direct Adversarial Tests Pending Cargo

- `artifacts::las::standards::v_1_0::subsets::any::schema::diff::component::tests::invalid_collection_targets_are_rejected_before_mutation`
- `artifacts::las::standards::v_1_0::subsets::any::schema::mutations::component::tests::out_of_range_index_mutation_is_rejected_without_mutating`
- `artifacts::ply::standards::v_1_0::subsets::any::io::component::tests::missing_element_target_is_rejected_before_mutation`
- `artifacts::obj::standards::v_3_0::subsets::any::schema::diff::component::tests::invalid_collection_targets_are_rejected_before_mutation`
- `artifacts::step::standards::v_ap214::subsets::any::schema::diff::component::tests::invalid_collection_targets_are_rejected_before_mutation`
- `artifacts::step::standards::v_ap214::subsets::any::schema::mutations::component::tests::missing_and_out_of_range_targets_are_rejected_without_mutating`
- `artifacts::ifc::standards::v_4::subsets::any::schema::mutations::component::tests::missing_entity_target_is_rejected_before_mutation`
- `artifacts::ifc::standards::v_4::subsets::any::schema::mutations::component::tests::out_of_range_entity_mutation_is_rejected_without_mutating`
- `artifacts::ifc::standards::v_2x3::subsets::any::schema::diff::component::tests::invalid_instance_order_is_rejected_before_mutation`
- `artifacts::dxf::standards::v_r12::subsets::any::schema::mutations::component::tests::missing_entity_target_is_rejected_before_mutation`
- `artifacts::stl::standards::v_ascii::subsets::any::schema::mutations::component::tests::out_of_range_triangle_mutation_is_rejected_without_mutating`

The complete scoped library suites for all eight standards, including their existing apply, inverse, between, absorb, and mutation-law tests, remain pending the serialized Cargo gate.

### Migration Static Validation

- Exact `rustfmt --edition 2021 --check` completed successfully for all forty-two migration source files.
- `git diff --check` completed successfully across the eight assigned standard directories.
- Static signature census found all eight assigned root implementations returning `MutationApplyResult` and no assigned old infallible signature.
- Static direct-consumer census found no assigned snapshot overwrite from an unchecked result and no `let _`, `unwrap_or`, fallback, or ignored-result application consumer.
- Clamp and dedup hits remaining in the assigned diff modules are confined to absorb-time index/label simulation; application validates first and uses the already-validated target unchanged.
- Cargo, Bun, Nx, and runtime tests were not run under the repository-wide migration gate.
