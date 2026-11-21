# Selected Mesh Boundary Extrusion

Flow exposes `brep.mesh.extrudeBoundary`. Generation3d exposes `mesh.edit.extrudeBoundary` with a boundary halfedge picker and a displacement vector. The retained Mesh operation orders the selected directed halfedges from the source's boundary topology and adds one quad per edge, with shared translated vertices at adjacent walls.

The first scope is a single edge or one unambiguous open chain on a supporting plane. The displacement must leave the source on the opposite side of that plane. The certificate rejects ambiguous joins, interior edges, disconnected and cyclic selections, folds, nonadjacent crossings or contacts, unreliable native coordinates, and offsets into the source. It handles in-plane outward extension of a straight exposed edge and out-of-plane extrusion of a bent planar chain.

Original faces and source surface samples retain their identities. The generated strip preserves vertex, corner, material and edge provenance. Both directions of each new shared connector use the same source edge sample. The walls receive independent flat-shaded UV charts, including through an active named UV channel.

## Verification

- The neutral fixture fixes exact output topology and authored edge provenance. Three.js independently measured area 3 for a bent two-edge unit-offset strip and area 2 for the in-plane single-edge unit extension. The Mesh job and packaging suites passed 20 tests and 3,904 assertions.
- The compact native test compiled the live Mesh Document, Reconstruction and Modeling sources. The boundary extrusion and Bridge regressions both passed. Coverage includes the exact output polygons, six open boundary edges, paired source-rim twins and authored seams, raw and indexed UVs, materials, normals and shading, folded and crossed chains, an interior halfedge, and cancellation at wall generation with byte-identical source retention.
- The shared procedural catalogue oracle passed 14 tests and 269 assertions. Both new registered widget fixture cases specify exact polygon topology and area. The procedural widget and registered Flow workbench sources passed Rust parsing. Executable Flow and Generation3d integration remain unverified.
- All four committed Flow JSON carriers and the packed descriptor published the new operator with its edge and offset inputs. The packed descriptor retained its suffix bytes. English and German catalogue labels were updated and the changed paths passed diff checking.
- The framework 3D source closure measured 261,748 bytes, 3,115 active lines and 16 files. Reconstruction measured 162,191 bytes, 1,980 active lines and 8 files. Existing packaging gates passed without changing their thresholds.
