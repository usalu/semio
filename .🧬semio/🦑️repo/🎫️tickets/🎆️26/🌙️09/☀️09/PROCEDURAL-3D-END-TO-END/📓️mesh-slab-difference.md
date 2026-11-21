# Authored Middle-Slab Mesh Difference

The existing Flow `brep.mesh.difference` and Generation3d `mesh.edit.difference` operators also support a convex cutter with two exactly opposing parallel support planes across the first solid. Both support planes must cross the interior, the gap must exceed the numeric tolerance, and the first solid must lie inside the cutter's other halfspaces.

The retained job clips each exterior halfspace independently, preserves the left exterior faces, and includes one reversed, cutter-owned cap in each residual. There is no partition face between components. Publication checks both residuals against their own and opposing cutter planes, verifies reciprocal edge incidence and establishes positive signed volume separately for each shell. The two-plane job retains the existing nonparallel corner behavior and cancellation window.

The cap polygons use their own authored corner UVs, material bindings and reversed normal samples. The shared authored projection and import retain both asset inventories. Inputs outside the certified topology or with unrepresentable authored channels yield a validation error.

## Checks

The neutral corpus covers the centered slab, an offset slab, and a slab along the third axis. The centered result has 16 vertices, 24 edges and 12 polygons across two components, with volume 2 for each shell. The offset result has volumes 1 and 3. Native and Manifold witnesses also retain the original three nonparallel corner cases.

- The focused native Clipping run passed both tests. It checks the two capped ranges, reversed cutter-face normals, reciprocal edges, independent signed volume per shell, and cancellation during both slab and nonparallel production.
- The native authored facade witness passed. It checks distinct materials and normal samples on the opposite caps, per-corner UV interpolation, and actual polygon-source reimport of the separate components.
- The portable Mesh jobs and packaging suite passed 24 tests and 4,170 assertions. Manifold independently decomposed all three slabs into two components with the expected bounds and per-component volumes.
- The compact Flow producer/parser passed all three selected operations including slab and nonparallel Difference. The registered workbench fixture passed Rust syntax parsing. The full linked workbench was outside the compact run. Scoped whitespace validation and all package source gates passed.
