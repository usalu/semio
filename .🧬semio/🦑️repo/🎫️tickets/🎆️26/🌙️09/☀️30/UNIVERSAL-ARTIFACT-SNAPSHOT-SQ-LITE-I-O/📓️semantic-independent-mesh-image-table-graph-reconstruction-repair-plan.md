# Mesh, Image, Table and Graph Narrow Reconstruction Repair Plan

Current held after signatures and SHA256 digests are captured in the region-plan JSON. Preserve existing paid FloatRow helper vectors, borrowed lookup names, first-party reserve/heapsort, FK/group/ordinal validation and shared value forest. FloatRow requires no new RetireOwned implementation: paid temporary vectors may stay ordinary scratch; do not wrap them in typed Owned.

All following domain types have actual root RetireOwned implementations: Mesh Snapshot/Mesh/Primitive/Material/Texture; Image Snapshot/Frame/MetadataEntry; Table Snapshot/Column/Row; Graph Snapshot/Node/Port/Edge/GraphNodeId/GraphEdgeId; shared SemioValueEntry and SemioValue. Owned<Vec<T>> is eligible only for these actual eligible T. Numeric geometry types already occur inside root Mesh retirement macros and primitive vectors. No new runtime API required.

## Mesh

Create empty guarded SemioMeshSnapshot before material assembly; assign paid materials/meshes/textures capacities directly into it. For Material, create a complete placeholder guard before copying ID, then assign base_color/metallic/roughness and each optional texture reference in original order. This protects prior strings against later Binary32/optional failures. For each Mesh create its guard before reconstructing its primitive list; reserve exact group length into guarded primitives. Create Primitive guard with empty ID/lists, parsed topology and material_id None before reserve/fill of positions/normals/UV/colors/indices. Assign each paid vector into that guard immediately before its fill. Reconstruct primitive ID and optional material in original late order, then push take. Assign mesh ID in its original late position. Texture guard before ID then MIME then blob. Root schema assigned last under snapshot guard; move final checkpoint after schema assignment, return take.

Existing float_rows/ordered_float_rows reserve their exact cardinality; relationships/group helpers are paid. Do not replace them with maps. Scalar scratch remains ordinary Drop and gives no physical release claim.

## Image

After existing width/height/bit_depth/colorspace validation, create empty guarded Snapshot carrying those scalars before ICC blob reconstruction. ICC belongs immediately to this guard. Reserve frames/metadata into guarded fields. For each frame create Frame{delay_ms:0,rgba8:empty} guard before byte vector reserve/fill; retain original delay validation after sample traversal. MetadataEntry guard with empty key/value before key then value copy. Final root schema belongs to live Snapshot before final checkpoint/take. Preserve sample ordinal/channel modulo4 and exhaustive consumption checks; paid row/group machinery remains.

## Table

Create guarded Snapshot with empty schema/columns/rows before paid columns reserve. Columns themselves contain name String plus scalar kind. Existing kind parse precedes sole name copy and insertion, so individual column has no later fallible construction field; root guard protects completed columns during downstream relational/value reconstruction. Keep existing guarded value forest, row and cell vectors, transferring completed rows into Snapshot only after all checks. Assign root schema under Snapshot, then final checkpoint/take. Preserve exact property/cell root ownership and contiguous row cell widths. Do not redundantly charge scalar cells already admitted by check_database.

## Graph

Current row/reference/identity/forest/index scratch already uses reserve, heapsort and controlled compare_text. Preserve it. Start guarded Snapshot before typed node assembly. Use actual partial Node, Port and Edge guards with empty identity wrappers/strings/lists and harmless scalar placeholders before their first owned field. Retain original field/check order: Port kind validation then name/category/properties; Node ports then id/kind/label/position/width/height/properties; Edge id/source/target/kind/label/optionalports/properties. Assign each successful copy or restored properties list directly into live guard before any next fallible operation. Transfer completed ports into Node and completed nodes/edges into Snapshot without taking a protected owner prematurely.

restore_properties has one exact additional anchor in the plan: key is copied before property root lookup/value take. Guard an actual SemioValueEntry with empty key and SemioValue::Null before key copy; assign taken value and insert. Preserve consumed Option values and duplicate root errors. Guard root schema before final checkpoint, return Snapshot take. Current graph schema String is otherwise ordinary between copy and cancellation checkpoint.

These repairs address admitted allocation capacity and typed partial owners; native::Owned retirement allocates its own boxed cursor scratch. Exact producer ledger observation must exclude successful output teardown, while failure aggregate cleanup must distinguish that scratch from reconstruction admission. No 4096 physical release, parent handback, type compilation or runtime credit follows from this static plan.
