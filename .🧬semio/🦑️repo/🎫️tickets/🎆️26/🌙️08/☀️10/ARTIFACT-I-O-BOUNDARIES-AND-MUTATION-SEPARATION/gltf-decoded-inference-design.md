# glTF Decoded Inference Boundary

The geometry kernel currently decodes three accessor roles through the engine alias, which resolves to physical buffer decoding. The schema will instead own a decoded inference input that borrows the authored snapshot and owns typed accessor results plus input fingerprints. Physical inference preparation will decode each accessor once for that request, expose progress and cancellation between accessors, and then invoke the pure geometry DAG.

The projection is request scoped and is recomputed from the exact current snapshot. Mutations therefore do not maintain a second persisted cache, and buffer/accessor edits cannot leave stale inferred values. Existing inference dispatch will prepare the projection in I/O; the schema computes only on typed values. Empty defaults construct an empty typed input directly. Native glTF and GLB serialization stays in I/O, and no schema forwarding aliases will be added.

Neutral inference fixtures and registered native runtime tests will verify the numerical output is preserved. This extends the Raster approach of decoded typed inputs without adding persisted derived state.
