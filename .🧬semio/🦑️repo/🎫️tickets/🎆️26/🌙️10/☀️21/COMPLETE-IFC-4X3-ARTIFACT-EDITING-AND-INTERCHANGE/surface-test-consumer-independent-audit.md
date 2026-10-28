# Surface Test Consumer and Primary Contract Gap Review

Read-only static inspection of surface-fixtures.ts/surface.test.ts and the copied22 inputs found no confirmed fixture-role or fixed-row Three area defect. Real product identity placement is present. Wrong-bound is a valid Plane, wrong-loop a valid Polyline, wrong-point a valid Direction. Required Orientation unset is deliberately invalid. The independent reference projects each face to its plane and uses Three ShapeUtils for holes; its first-three-vertices basis is adequate for these authored noncollinear-prefix rows, not a general future collinear-prefix oracle. Official area10/bounds/8v12t and single-face X-edit area2→3 with both inverses match actual source geometry. No changing production module was imported, no public Surface after result is claimed.

The canonical ticket matrix is now extended30 rows in surface-geometry-cases.json, SHA256 c0d91a3d3b84faf9759ea9a213d20a0c3e92f4b2ee0bcc4be662c67c1f99d419. The existing closed schema is unchanged SHA256996b3d915d8162a4772b93ba4564929bceab92874e6e270d42f5c87568fa6334. Actual Ajv strict validation passes30. Source copied22/hashes remain historical; root must copy new30 bytes and update source test length/hash after accepting the explicit added contract. New surface-geometry-boundaries.py/open-boundary.py create only eight additional physical observations through the same owned interface; earlier22 results were retained, not rerun. All30 results are in generated/surface-geometry/oracle.json. All external processes exited.

New allowed rows and normal IfcOpenShell0.9.0.post1 observations (all formalEXP0):

- Common-edge separate groups:8v4t, area2.
- Common-vertex separate groups:8v4t, area4.
- Same source face referenced by two distinct ConnectedFaceSets:8v4t, area4.
- Distinct face entities with identical boundaries in separate groups:8v4t, area4.

The last two measure the sum over retained source face occurrences, matching normal backend and the current Three consumer. They do not claim Boolean union area2, forced face deduplication or a new solid. A global face-reference uniqueness restriction would reject valid common-source-face reuse incorrectly: uniqueness applies inside each group's SET, while the model's SET contains distinct group entities. Geometric group comparisons may recognize identical complete boundaries as permitted overlap without rewriting source ownership or using epsilon welds.

New negative rows all also pass formalEXP0 and normal meshing, so external acceptance is not a geometric certificate:

- Vertex-only-pinched OpenShell:7v4t/area4; generic ConnectedFaceSet with exactly the same face domains remains a positive.
- Two groups crossing through face interiors:8v4t/area8.
- Partial coplanar face overlap with different boundaries:8v4t/area8.
- Closed box declared as OpenShell:8v12t/area22.

Pinned primary source paths/hashes are in surface-geometry-independent-preparation.md. FaceBasedSurfaceModel permits groups to meet at common complete faces, edges or vertices, with only exact identical-boundary face overlap allowed by its definition; arbitrary interior crossings/partial coplanar overlap violate its stated model conditions. ConnectedFaceSet requires arcwise connectivity and is concrete; shared vertex contact suffices, and generic sets are not required to be2-manifolds. OpenShell has the stronger oriented2-manifold-with-boundary contract and must not be a closed surface. ClosedShell retains its stronger closed-manifold contract. Applying the generic solid validator to every group would wrongly reject valid open/vertex-connected generic surfaces; ignoring declared shell subtype would wrongly accept the negative rows above.

For the proposed bounded planar implementation, preserve required declared owners and raw point/face/group identities. Use private exact-coordinate keys for geometric checks only. Inside a generic group, connectivity may be through common edges OR vertices; no universal edge2/opposite-normal/positivevolume rule applies. For OpenShell require a real boundary, internally compatible oriented edge incidence and a single manifold vertex-link component, without requiring closed volume. For ClosedShell retain existing actual shell rules. Across groups, classify exact common boundary/face contact independently of forbidden interior intersection. If some robust intersection classification is not implemented, identify the unsupported case explicitly rather than return silent success or claim complete SurfaceModel semantics.

Before source closure, add genuine private/public output reservation limits with source admission separately funded, mapping/reflection/units, original error/undefined/cancel through extraction and composed GLB, and declared source-shape/late unknown checks. The current25 tests are a genuine red checkpoint only. No production/testregistration/build files changed, no native execution or complete IFC/MVD/precision proof is claimed.
