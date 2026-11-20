# Original sharp solid offset

The retained Solid Offset node handles signed distances on certified boxes. The operation follows the cold kernel's sharp-corner policy for planar solids. It moves all six support planes along their outward normals and reuses the funded Box publisher to construct the result. Positive distance expands the box and negative distance erodes it. The result keeps the source's world placement and has independent topology labels. Zero or nonfinite distance, collapse at the kernel tolerance, coordinate overflow and unsupported shapes return explicit refusals.

The original Flow fixture creates and translates a box, offsets it with both signs, previews it and measures its exact volume. It checks the world bounds and complete face, edge and vertex references, confirms the source volume and verifies that collapse, zero distance and an uncertified operation family produce no live result.

The independent Manifold oracle uses a centered cubic Minkowski sum and difference for the same world-coordinate fixture. It passed ten assertions including the contracted solid's genus and empty result at collapse. Rust syntax parsing passed across the seven touched Rust files. The native Flow fixture remains unexecuted because the host has insufficient space for Cargo admission.
