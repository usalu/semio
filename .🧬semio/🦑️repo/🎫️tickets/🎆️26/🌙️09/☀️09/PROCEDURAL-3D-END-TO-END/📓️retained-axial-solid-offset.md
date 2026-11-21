# Retained axial solid offsets

`brep.solid.offsetSolid` accepts certified finite Cylinders, Cones and closed Conical Frusta. It moves the lateral support and each native end cap by the signed normal distance. The sharp edges follow the exact intersections of those supports. A cone retains its axial apex and original opening angle, with the apex moved to the displaced conical support. The operation preserves the geometric family's exact circle, plane and cylinder or cone entities on the authored world axis.

Inset offsets require a positive remaining radius and axial interval. A frustum with a cap that would eliminate its rim is refused. There is no implicit conversion to a cone or empty solid. The operator retains the original staged publication and rollback path: each topology record takes a funded step and one complete solid becomes live. Frustum Measure now reads exact cap and side areas and its uniform-density centroid from its analytic profile.

## Evidence

- `🔣️original-axial-solid-offset.json` supplies inside and outside offsets for each family and five refusal cases. Three independently offsets the meridional support line and intersects it with the displaced cap planes or apex axis. Manifold revolves that resulting support interval and verifies world bounds, area and volume. The combined independent offset and adjacent Split tests passed 12 tests and 162 assertions.
- `🐍️axial-solid-offset-source-check.py` compiled the actual retained provenance selection, recipe, analytic material queries and Frustum topology. Its 12 passing tests cover each signed case, exact support and rim measurements, one coedge per direction, trim-to-edge fidelity, area, volume, centroid, Classify and refusals. The adjacent cylinder/cone Split compact regression passed four tests and the existing smooth solid offset regression passed its sphere and torus corpus.
- Registered `original-brep-booleans` covers Rotate → Translate → Offset for Cylinder and Cone, and Rotate → Translate → Split → Offset for Frustum. It checks unpublished Frustum rollback after private faces are inserted, one complete live solid, native rim and surface topology, preview bounds, Measure, Classify and rejection without side effects. Managed execution remains pending shared build capacity.
- Canonical source measurement passed BRep Engine at 3,940 lines and 233,703 bytes, Native Registry at 3,948 and 215,482, Solid Transaction at 3,915 and 202,510, Construction at 3,903 and 196,391, and original-booleans at 1,448 and 175,664.

The existing certified Box, Sphere and ring Torus offsets still use their original formulas and retained output contract.
