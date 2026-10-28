# Swept Rectangle And Mapping Exchange Next Slice

Read-only source/primary-spec exploration plus executed independent geometry oracle. No runtime dependencies, production edits or swept/mapped/BREP completeness claim.

## Smallest general implementation

Current IO geometry.ts extractIfc4x3Triangles privately captures through IfcGeometryGraph.capture and iterates actual Product→ProductDefinitionShape→ShapeRepresentation.Items; it only evaluates IfcTriangulatedFaceSet and reports geometry.unsupported-item for others. glTF export consumes extracted meshes through existing controlled source/export codec. Add one general item evaluator beside existing geometry domain: exact IfcExtrudedAreaSolid with exact IfcRectangleProfileDef AREA, positive finite XDim/YDim/Depth, optional profile Axis2Placement2D, optional swept solid Axis2Placement3D and valid normalized ExtrudedDirection. Generate rectangular prism8positions/12triangles; no arbitrary polygon triangulator or external runtime needed.

The [official extrusion definition](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcExtrudedAreaSolid.htm) places direction in swept solid Position coordinates, allows oblique direction with nonzero local z component and defaults absent Position to object coordinates. Do not assume positiveZ only, multiply raw direction magnitude by depth, or put profile position into global coordinates. Compose profile2D→solid3D→product placement→representation context→SI units exactly once; retain paired axis semantics and orientation/winding. Refuse unsupported profile types/holes/tapered/revolved forms explicitly. This general subset supports original extruded-solid.ifc without fixture-ID branching.

Original official graph: product1000 placement1001→1002 at1000mmX, body1020 typeSweptSolid, item1021 rectangle1022 X/Y1000mm, Depth2000mm, absent profile/solid Position and direction1034 +Z. Pinned source SHA256c488f0367106a17289468409531b1b687f72c25cea8e1b46924f59972ad7a8e9; exact headerADD2.

## Mapping is a separate next boundary

[IfcMappedItem](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcMappedItem.htm) reuses a representation definition under MappingTarget; nested maps are permitted. Exact role path: MappedItem.MappingSource→RepresentationMap.MappingOrigin/MappedRepresentation→items; apply mapping origin inverse followed by target transform under parent placement. Authenticate formal transform composition from released catalog functions before implementation; do not conflate MappingOrigin with an extra forward placement. Retain depth/cycle limits and each occurrence independently; visited geometry IDs cannot suppress genuine reused occurrences.

The actual transformation fixture uses CartesianTransformationOperator3DnonUniform (0.5,0.5,1), so uniform-scale-only support will not admit it. Without-transformation uses the same rectangle extrusion via representation map; multiple-items has four mapped occurrences. Each mapped source item must retain actual occurrence/product identity and transform. Nonuniform determinant/reflection affects normals/winding and volume. Mapped-source contexts and world unit scale must not be applied twice.

## Executed independent mesh oracle

Authored swept-geometry-oracle.py uses existing IfcOpenShell.geom0.9.0.post1 normal create_shape API with USE_WORLD_COORDS, existing ticket dependency and guarded Windows shortpath. No reinstall. All4 original manifest hashes checked; generated/swept-geometry/oracle.json stores URLs/hash/header/product IDs/bounds/count/volume. No geometry errors.

| Original file | Vertices/triangles | SI bounds | Mesh volume |
| --- | --- | --- | --- |
| extruded-solid |8/12|[0.5,-0.5,0]→[1.5,0.5,2]|~2m³|
| mapped without transformation |8/12|same|~2m³|
| mapped with transformation |8/12|[0.6464466094,-0.3535533906,0]→[1.3535533906,0.3535533906,2]|~0.5m³|
| mapped multiple items |32/48|[0.6464466094,-0.3535533906,0]→[2.3535533906,1.3535533906,2]|~2m³|

Volume is absolute signed tetrahedral sum of oracle mesh, not a general Boolean union oracle. Triangle topology/winding may differ while representing same surface; compare bounds/volume/edge closure/orientation with tolerance and exact original triangle semantic content where appropriate. These observations do not certify full IFC schema/view.

## Test-first contract and controls

Neutral schema-first rows: default rectangle, translated/rotated profile, solid/product/context placement, millimetres/metres, oblique and negativeZ direction, nonunit direction normalization, dimensions/depth invalid, axis paired/default/parallel, unsupported profile/hole/tapered. Mapping rows separately cover nonzero origin, identity/uniform/nonuniform/reflection, repeated source occurrences, nested/cyclic maps and unsupported branches. Thirdparty oracle compares same input physical graphs through normal geometry API. Actual official model must export GLB, pass independent Khronos and be read back with geometry bounds/volume preserved; no changed fixture graph.

Keep source capture original ownership and private graph. Item traversal, transform normalization and emission must checkpoint original batch including completion; no uncontrolled full map/some/reflection pass or secondary whole mesh arrays. Rectangular item work is constant but source representation/map traversal is not. Existing geometry.ts hardcoded2048 vertex/triangle intervals are a separate older throughput/cancel frontier; new evaluators should use original captured batch policy rather than copy it. Cancellation during source/map/profile/vertex/index/export phases must publish no artifact. Native matching evaluator remains separate original-funded work and cannot be claimed implemented by TS success.
