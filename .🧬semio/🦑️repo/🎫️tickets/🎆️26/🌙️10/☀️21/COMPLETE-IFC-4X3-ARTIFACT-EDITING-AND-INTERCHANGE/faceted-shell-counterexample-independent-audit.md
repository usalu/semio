# Declared Faceted Shell Counterexample Oracle

The frozen ten-row shell matrix was independently constructed as actual IFC4X3_ADD2 geometric entities through the existing IfcOpenShell0.9.0.post1 interface. The first attempt encountered the concurrent correction from two- to three-component join offsets and stopped after two rows; the final complete run consumes corrected xyz offsets, including true single-vertex contact [2,1,3]. All final10 inputs and normalized observations are under generated/faceted-shell-independent; authored inputs are faceted-shell-physical-oracle.py and faceted-shell-mesh-oracle.ts. No original official asset or frozen18 fixture data was changed.

A confirmed fixture defect was reported before production: original pinched offset[2,1] shared a vertical edge because both prisms occupied z0..3. Parent corrected the contract/test to [2,1,3], producing only shared vertex(2,1,3). This now tests a different topology from four-incidence edge contact.

| Case | IfcOpenShell full EXPRESS diagnostics | Normal geometry | Independent Manifold observation |
|---|---:|---|---|
| open | 0 | accepts10 triangles | rejects Not manifold |
| duplicate face | 0 | accepts12 triangles | rejects Not manifold |
| two components | 0 | accepts24 triangles | NoError, volume12 |
| pinched vertex | 0 | accepts24 triangles/15vertices | NoError, volume12 |
| outward void | 0 | compound failure | NoError, volume6.5 |
| outside void | 0 | compound failure | NoError, volume5.5 |
| crossing void, centre inside | 0 | compound failure | NoError, volume5 |
| touching outer | 0 | compound failure | NoError, volume5.5 |
| nested voids | 0 | compound failure | NoError, volume3.25 |
| intersecting voids | 0 | compound failure | NoError, volume4.5 |

These are deliberately geometric counterexamples, not ten valid physical solids. Full EXPRESS does not cover all informal ClosedShell/faceted-solid topology, orientation and containment requirements. The unmodified external backend also fails the valid compound WithVoids fixture, so its compound failure here cannot discriminate invalid cases. Manifold's constructor/status is similarly insufficient as a shell-validity oracle: it may repair input topology and can accept oriented triangle soups without checking the IFC declared role contract. Its resulting signed-volume arithmetic alone does not prove Boolean occupancy. Three182 BoxGeometry provides the separately constructed oriented cube meshes; these external observations are test-only and no runtime dependency is added.

The valid disconnected compound is especially important: two disjoint positively oriented boxes can form an acceptable generic compound mesh, but putting both face sets into one declared IfcClosedShell violates its connected-shell requirement. Vertex-only contact can retain two-manifold edge counts while creating a disconnected vertex link. An outward cavity preserves closed incidence but violates inward void normals. A centre-inside test misses the crossing cavity's exterior portion. Shell surface intersections/touching and void-void separation require complete geometric checks, beyond AABBs or one point alone. Existing shared solid validation plus explicit IFC shell/void role checks must retain these distinctions.

Generated physical files are minimal geometric entity populations, not complete project/product exchange fixtures. Exact FILE_SCHEMA is ADD2 without retagging; no whole-view, native compiler/runtime, or physical validation proof follows. All processes have exited. This research does not audit parent's in-progress Faceted evaluator implementation.
