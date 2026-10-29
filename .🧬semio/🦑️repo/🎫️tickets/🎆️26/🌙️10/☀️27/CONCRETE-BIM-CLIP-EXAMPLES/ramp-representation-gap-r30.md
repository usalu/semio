# Ramp Product Representation Gap R30

A direct current public API call to represent(first authored IfcRampFlight,'3d',source) raisesValueError:No representation forIfcRampFlight. The source has physical ramp occurrences/flights and quantity derivation, and the villa drawings derive their inclined slabs through _flights. The missing product representation is distinct from an absent ramp in those drawings.

The current _flights contract returns source path,start/end elevations, width and thickness; quantities treat thickness as a vertical slab offset and compute run×width×thickness. Before any metric change, this convention must be compared with the archival section to establish whether nominal6cm is a vertical or perpendicular dimension. No new perpendicular thickness is inferred here.

Next scope: expose native physical ramp/flight parts through a public geometry helper and all seven standard product representations, reuse those parts in physical vehicle/drawing consumers, and support native selected-product IFC export. Preserve source class/GUID and real inclined top/bottom faces, including a diagonal synthetic path and authored landings. Portable JSON cases and independent SciPy/GEOS volume/slice tests must precede implementation. Avoid a proxy-class adapter or legacy fallback. Source station positions, dimensions and IDs need no speculative relocation.

This is queued after the active beam/post/bore producer repair; no ramp production code was edited during this inspection.

A second directcheck confirms both parentIfcRamp and occurrenceIfcRampFlight reject the public body representation. The gap is therefore both products, not only standalone flights.
