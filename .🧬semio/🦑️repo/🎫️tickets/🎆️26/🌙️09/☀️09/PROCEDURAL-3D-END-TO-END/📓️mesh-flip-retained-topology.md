# In-Place Winding for Retained Mesh Flip

Flip Mesh Faces now toggles the existing face winding bit in the owned native mesh. This keeps original halfedge identity, twin adjacency, per-corner UVs, seam flags, smooth flags and named surface bindings. A raw point normal is reversed when all incident faces reverse. A point shared between reversed and unreversed faces keeps its single authored point direction. Automatic Orient continues to own its separate topology reconstruction.

The producer quotes the selection backing, incidence backing, one face or vertex change and the final ownership transfer. It yields after each admitted turn. The registered Flow fixture checks effective polygon winding and retained raw surface data and includes cancellation during normal projection. Polygon JSON serializes the resolved loops. Twin backing identity is an in-memory native property.

The neutral fixture flips one of two triangles with an authored seam and different UVs. After the previous prototype dependencies were evicted during setup, a small Rust harness compiled the exact production Flip cursor against equivalent halfedge records and grant adapters. It verified the original halfedge backing pointer, both twin records, unchanged UVs, the exclusive reversed normal, and the mixed shared normal. Short capacity and copy grants left the cursor and receipts empty. The cursor copied 26 bytes over 28 events. Three.js independently verified both winding directions and traced the original twin IDs.

All 31 portable Mesh tests passed with 4,202 assertions. The 3D facade admitted 2,635 active lines and 225,065 bytes. The linked Flow cancellation and complete package compile remain pending repository build admission.
