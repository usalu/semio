# Reset Document Ownership

Wires and Rewriting currently create an owned ArtifactEnvelope only to print an edit-free SPR history. Current Store ownership requires explicit envelope retirement, so this temporary owner can panic on ordinary drop. Store already provides `empty_document_spr(doc_id, schema)` without allocating or cloning a document envelope; the reset helpers should call that existing abstraction.

Neutral reset fixtures and focused native codec/runtime regressions were added first for both families. The tests compare each source and decoded reset pack through independent serde_json values, and compare the decoded SPR identity and empty history collections with the neutral fixture. The executable is registered through root script, root/ticket Nx and both launch catalogs.

`reset-document-ownership-red-1.log` failed to compile because WiresSnapshot intentionally has no serde::Serialize implementation. Its regression now projects through the first-party JSON boundary and parses with serde_json. `reset-document-ownership-red-2.log` then exposed a concurrent crate-name regression in Rewriting: the dependency package is semio-framework-replication but its actual lib name remains protocol. The root extern was corrected to `extern crate protocol as replication` after inspecting Cargo.toml. RED3 is queued; production reset helpers are still unchanged until the regression reaches the lifecycle failure.

## Native Lifecycle RED and Fix

`reset-document-ownership-red-3.log` compiled both test crates and ran the Wires regression. It failed exactly at Store's envelope Drop invariant: the fresh shell reached Drop before its bounded retirement authority detached all nested owners. Compile plus test took 17 minutes 36 seconds; the failing test itself took 0.01 seconds. Rewriting's test was not run after the first package failure.

Both reset helpers now encode their existing pack and call `store::empty_document_spr` for fresh history. They no longer clone a snapshot or manufacture a temporary envelope just to obtain a history log. The registered two-family verification is queued as `reset-document-ownership-green-1.log`; no green result is claimed yet.

## Latest Dependency Checkpoint

`reset-document-ownership-green-3.log` terminated before the reset tests on three intermediate Jack errors (one unsupported owner constant and two unresolved results-window constants). The Jack owner subsequently corrected all three. Its next compile encountered concurrent BRep visibility errors outside this reset change; runtime proof remains pending until the dependencies compile.
