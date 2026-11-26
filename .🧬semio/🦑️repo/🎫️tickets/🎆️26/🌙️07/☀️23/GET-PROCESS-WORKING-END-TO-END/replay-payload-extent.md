# Replay "operation output payload extent exhausted"

## Root cause
The error was not a history-length or batch-splitting problem. A replay publishes at most 3 envelopes per batch, and the batch message limit (`DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES`) was never reached.

The failing `ArtifactPreparedOperationOutput` belongs to the replacement-wire comparison (`OriginalOperationWireComparison` -> `ArtifactOriginalOperationBytes::admit_paged(.., wire.len(), ..)`). Its extent is the supplied canonical wire length (8 bytes), by design. The comparison re-encoded the operation with the canonical JSON source (`kind=json`) instead of the persisted binary source, so the re-encoded bytes (`{"operation"...`) exceeded 8 bytes and the guard raised "extent exhausted". Instrumented run: wire `[1,0,0,1,0,3,232,2]`, output extent 8, JSON chunk of 9 bytes at offset 4.

Why the source was JSON: the Store catalog install (`store-owners-retained-reads-and-retirements`) takes `wire.source = owners.operation_wire.source`. The owners come from `A::build_document_store_owners`, whose wire is `ArtifactOperationWire::canonical()`. This overwrote the persisted source (`with_original_encoded_binary_receiving`) that the ingress had installed on the live Store. Replacement Stores built by the maintenance ladder had no persisted wire at all.

## Fix
- `ArtifactApp::persisted_operation_wire()` (artifact-app-trait): the single owner of the app's persisted wire (semantic, batch digest, persisted receiving, whole-document target).
- `ArtifactApp::persisted_document_store_owners(grant)`: `build_document_store_owners` with that wire set on the catalog owners.
- The ingress install (live Store) and both replacement-Store builds in the maintenance ladder (paged archive, recursive document archive parent) use them. The ingress `install_operation_wire` uses `A::persisted_operation_wire()`.
- The batch limit constant was not touched.

## Results
- The "payload extent exhausted" fault is gone. The replay of both target laws now completes.
- `cargo test -j 2 folder_reload_route`: 7 of 9 pass, including the new law `a_history_larger_than_one_batch_payload_replays_through_a_history_edit` (6 edits of 60,000 chars, more than one 262,144-byte batch payload: begin, input, accept and replay all complete).
- The two target laws now fail one step later, not on the extent:
  - `a_folder_bound_document_reloads_...`: the folder archive load faults in the replacement initializer leg, and the commit publishes 0 batches (expected 1).
  - `two_programs_on_one_folder_converge_...`: the commit publishes 0 batches, so no supersession or history-edit row reaches the peer.
  - Not diagnosed. Likely the finalize/publish stage (a version of the new law that also finalized stopped at "the finalize retires never settled; stage Finalizing"), and loading a 360 KB archive exceeds the fixture turn bound ("original fixture cancellation issuer birth exceeded its authored turn bound").

## Changed files
- `framework/products/os/modules/plugin/declaration/app/artifact-app-trait`
- `framework/products/os/modules/plugin/instance/staging/ingress` (`instance/stores/staging/ingress` for the owners call)
- `framework/products/os/modules/plugin/instance/docs/staging/maintenance-ladder`
- `framework/products/os/modules/plugin/tests/folder-reload-route` (new law, `ONE_BATCH_PAYLOAD_BYTES`)

All `[DEBUG]` instrumentation was removed.
