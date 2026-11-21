# Mounted region deformation

The FEM 2D results overlay now borrows the analysis nodes and CST elements from the same retained CSR assembly that supplied the displayed fields. If the window displays the previous validated fields while a replacement runs, it also borrows the previous assembly's mesh. No preview mesh or batch solver is called to create the overlay.

One borrowed-ID map projects retained node positions with the static displacement or normalized mode-shape components. Authored members and triangle edges share the existing deformation color and polyline layer. Triangle edges require published values at both endpoints. Canonical endpoint pairs deduplicate shared edges, and first occurrence in assembly order determines layer order. The work scales with the admitted nodes, authored elements, and triangle edges.

The 40-triangle bound applies to each region's mesh. The accumulated model and Canvas2d transport retain their existing bounds. The overlay applies no additional per-window triangle cutoff.

The focused results-window law reads a shared JSON fixture with endpoint coordinates, reverse-edge deduplication, and omission of edges with unpublished endpoints. An independent NumPy check reproduced all three edges in layer order. The native results-window suite passed all 22 tests, including this deformation fixture.
