# Authored Enclosed-Cavity Mesh Difference

Flow `brep.mesh.difference` and Generation3d `mesh.edit.difference` can subtract a watertight convex cutter strictly contained in an outward closed convex stock mesh. Every cutter vertex must lie beyond the stock tolerance on the interior side of every stock support plane. If the containment certificate fails, the operation uses the existing contact, pocket or other certified Difference path.

The retained job preserves the stock positions and face rings, appends a separately indexed shell with the cutter rings reversed, and verifies positive outer volume, negative inner volume and positive net volume. Both source certificates establish watertight reciprocal shells. Their strict separation prevents shared positions or edges between the two shells. The centered fixture has 16 vertices, 24 edges and 12 polygons, outer volume 8, inner volume −1, net volume 7 and total area 30. The offset fixture has inner volume −0.375 and net volume 7.625.

Original stock corner samples and the cutter's independent materials, UVs and normals feed the retained authored projection. Intact geometrically separate shells also preserve authored Vertex attribute domains. Cut and welded geometry continues to use independent corner samples where required.

## Checks

- Both compact native Clipping tests passed. They verify the exact stock position and face prefix, reversed cutter rings and their direct point mapping, reciprocal edges, separate signed shell volumes, area, center and offset cutters, and cancellation during shell construction. A cutter exactly on the stock plane takes the certified pocket path.
- The authored Mesh facade witness passed all six independent cutter charts, original stock corner samples, flipped normals, preserved Vertex normal and color domains, native tessellation and subsequent PolygonMesh reimport. A disconnected Union check verifies the same intact Vertex-domain behavior.
- The portable Mesh jobs and packaging suite passed 24 tests and 4,388 assertions. Manifold independently reproduced net volume and area and signed triangle integration confirmed the positive stock volume and negative cavity volume for both cases.
- All seven retained Flow producer/parser tests passed. The registered workbench fixture passed Rust syntax parsing and scoped whitespace verification. The full linked workbench was outside the compact run.
