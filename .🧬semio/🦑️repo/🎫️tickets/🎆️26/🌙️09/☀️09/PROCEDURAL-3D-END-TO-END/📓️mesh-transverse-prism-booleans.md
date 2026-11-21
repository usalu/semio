# Transverse Mesh Booleans on a Concave Prism

Intersect now clips a certified concave prism when the convex enclosure supplies transverse cap planes and its other supports enclose the prism. It preserves the original side faces, emits new vertices once per original joining edge, and assigns each cap to its actual enclosing face. Both operand orders yield the same closed geometry.

Difference now subtracts a convex transverse slab from a concave prism. It publishes both complement intervals as closed material pieces. Each endpoint cap carries the reversed original cutter-face owner. The callback also handles a single surviving interval, total removal, and disjoint material using the same certified interval.

The intersection oracle yields volume 1.5 and area 10. The split Difference yields two pieces, each of volume 0.75, area 8, and Euler characteristic 2. The registered native Clipping suite passed 10 tests. Focused Flow passed 17 tests and Generate3d passed eight, under 1, 7 and 1024 work items. The independent Manifold and packaging run passed 18 tests and 4,762 assertions. Oblique side crossings and transitional concave Union remain open.
