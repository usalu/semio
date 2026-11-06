# External Marker Propagation Census

One bounded read of OS, Job and replication inspected 5450 Rust files and recorded 49 matched files / 87 candidate lines. Raw capture is generated/external-marker-census.json. Fixed ArtifactHistoryLedger.reserve_one() calls are false positives: they reserve a fixed logical row rather than a PagedList allocation and must not acquire fabricated markers.

Actual remaining physical page paths include Store hydration, replay, original outbox, operation-wire output, snapshot export; Window registry, typed-pack load and action indexing; replication diagnostics, dictionary, causal fold/index and byte I/O; Flow compare and Neural frontier/retirement pages. Raw source methods must accept and forward the actual marker from their existing original receiving/retirement gate. Reference-only registration paths require an authored owner rather than discarding a locally created marker.

RetainedWorkLedger normal/contiguous/vector/string/shared births are also mandatory marker-last. Server owns the lower primitive/Controlled/clone graph, Guest owns page reservation/collection, Root owns its Source/Host/projection lanes. No local marker was invented, inferred from progress or discarded during this census.

The direct PagedBytes adapter now delegates the actual immutable sequence extent to all canonical reserve demand/work/step methods. CausalPayload uses its bounded declared byte count; isize::MAX is only the type ceiling.

Native compiler qualification has not run. Constructor and Source protocol migrations are active, so the captured caller set is a review aid rather than a claim that every caller is fixed.
