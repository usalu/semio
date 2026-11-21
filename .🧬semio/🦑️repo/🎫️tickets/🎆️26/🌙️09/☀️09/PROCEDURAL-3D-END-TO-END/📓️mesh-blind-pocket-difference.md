# Authored Blind-Pocket Mesh Difference

The existing Flow `brep.mesh.difference` and Generation3d `mesh.edit.difference` operators support blind pockets from a certified convex cutter with four common-axis walls and a unique transverse floor. The pocket must open through a single original stock polygon.

The retained partition job exposes five reversed cutter-owned polygons. It checks reciprocal edge incidence, positive volume and Euler characteristic two. Four wall joins form a cycle, and each adjacent wall pair shares exactly one bottom corner. A retained rim traversal checks that one simple opening visits all four walls and belongs to one original stock face. The four-wall through-tunnel path still requires two complete openings and Euler characteristic zero.

The wall and floor polygons preserve their own materials, independent corner UVs and flipped authored normals. Shared geometry vertices can have two wall UV samples at the rim and three independent samples at the floor corners. A pocket whose opening spans two authored stock polygons returns a validation error. The later [enclosed-cavity report](./📓️mesh-enclosed-cavity-difference.md) describes the strictly contained cutter path.

The centered and opposite cases produce one closed genus-zero shell with 28 vertices, 48 edges and 22 polygons. Both have volume 7 and area 28. The offset pocket has volume 7.625 and area 25.75.

## Checks

- Both compact native Clipping tests passed. They check the four reversed wall owners, the reversed floor owner, the opening and floor depths, reciprocal incidence, Euler characteristic two, positive derived volume, cancellation and the specific single-stock-face refusal.
- The authored Mesh facade witness passed with all five independent cutter charts, distinct materials, named normals, discontinuous corner UVs and PolygonMesh reimport.
- The portable Mesh jobs and packaging suite passed 24 tests and 4,312 assertions. Manifold independently verified one genus-zero component, both depths and all five reversed cutter normals in each of the three pocket fixtures.
- All six compact retained Flow producer/parser tests passed. Rust syntax parsing verified the registered workbench source, and scoped whitespace validation passed. The full linked workbench was outside the compact run.
