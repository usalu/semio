# Tapered Solid Drawing Projections R16

## Native Geometry Propagation

Drawing providers now retain optional top_outline instead of extruding the lower ring vertically. The shared drawing-core part_faces helper returns library-independent lists of actual3D side polygons, outward unit normals and the real upper cap. Corresponding lower/upper rings keep their vertex alignment, including clockwise source winding. Axon generic stairs, details, furnishings, railings, beams and foundations wrap these faces through _part_faces(part,role). Staged service solids interpolate both caps at staging boundaries. Live extrusion transforms inclined normals consistently with the vertical scale.

Plan cuts read the actual interpolated ring at the cut elevation. Wholly lower solids show the real upper cap; wholly higher solids show the underside. Tapered footprint bounds include both caps. Default architectural ground plans hide fully buried foundations; structure-only views and explicit highlights can reveal them.

Sections classify tapered solids by their full footprint, then use the model's actual convex section polygon. A plane intersecting only the lower shoulder therefore stops at the real inclined boundary rather than generating a full-height rectangle. Beyond-cut silhouettes use the actual projected hull. Existing multi-interval handling for concave straight prisms is preserved.

Elevations emit and paint actual world-space side polygons with true face normals and geometric depth. All four facade directions preserve the tapered silhouette. Profile details preserve taper during partial reveal and read the horizontal profile at the declared section station; the structural strip path retains optional inclined profiles under its explicit collapse transform.

Parent-owned column splitting at terrain and axon ground painter ordering were preserved by fresh scoped edits. No source JSON, schema or model geometry helper was edited in this phase.

## Test-First Evidence

The initial production-recorder tests failed for absent _part_faces, the old plan lower-outline extrusion (1.835m² symmetric difference), the full-height section rectangle (.7m² difference), and elevation painter dependence on vertical edge endpoints. Added reveal/complete-provider tests exercise production class methods through a geometry-recording drawing interface, with no replacement of the model calculations.

Two additional regressions failed before their fixes: architectural plans exposed buried caps, and live extrusion retained stale sloping normals. A staged-equipment test separately failed when its band cap was replaced by the lower outline. All nine new recorder tests pass after correction.

Portable fixtures contain two tapered solids, including an offset pentagonal profile. NumPy independently verifies interpolated cut/profile rings; Shapely verifies projected occupied polygons; SciPy ConvexHull independently verifies actual section intersections, face planes, outward normals and exposed surface area. The complete elevation provider is exercised in all four directions.

## Final Validation

The established integrated geometry batch was rerun after clearing previously discovered test modules, so the final test count includes the new staged-service regression rather than a cached test class. Result:44tests,0failures,0errors against the published382-element source. All nine new production recorder tests pass. Every one of the22 actual source foundation frustums was additionally checked: each emitted production face lies on an independent SciPy hull plane.

These checks validate geometry and drawing-consumer propagation. Native Manim rendering/playback remains a separate blocked gate and is not claimed here. The prior19-test dense analytical proof used the340-element freeze; changed foundation births warrant the coordinator's focused analytical birth/profile rerun, without repeating dense sampling unless load-area geometry changes.

## Files

- 🎨️visuals/🏛️savoye/🧱️core/🐍️.py: actual3D face interface.
- 🎨️visuals/🏛️savoye/📋️plan/🐍️.py: cut/cap rings and buried foundation visibility.
- 🎨️visuals/🏛️savoye/✂️section/🐍️.py: true section/silhouette polygons and inclined elevation faces.
- 🎨️visuals/🏛️savoye/🔩️detail/🐍️.py: taper-preserving reveal/profile/strip paths.
- 🎨️visuals/🏛️savoye/🧊️axon/🐍️.py: generic actual faces, staged service caps and transformed normals only.
- 🎨️visuals/🏛️savoye/🧪️ortho-remodel/frustums.json and test_frustums.py: portable cases and production recording regressions.
