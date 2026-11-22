# Studio Workflow artifact binding

Studio publishes the existing `os.workflow` codec under the kind `s.space.studio` and the editor surface `s.space.studio@1/*#editor`. The plugin declaration, AppIo and committed JSON and Pack descriptors carry the same identities. Hub's existing trusted catalogue obtains the codec fingerprint from the Space component. The default trusted bootstrap and the local development profile include the Space package.

The shell confirms a mounted Workflow to `openSpace` with the owning Space and the exact Workflow artifact ID. The guest publishes both IDs to Studio configuration in one mutation and checks the retained Workflow document context. The local Studio route clears the shared document binding.

A Catalogue drop in a mounted shared Workflow sends a creation request to the shell. The guest derives the node ID from the retained authoring seed. The host resolves the target from the trusted creation catalogue and correlates its Ready result with the originating Workflow and node. The continuation validates the active Space, mounted Workflow, registered target schema, and existing binding. A successful continuation emits one `AddNode` with the artifact identity and opens that document using the same node identity. Ready redelivery is idempotent. A reused node ID with a different document is refused.

The Workflow snapshot keeps its existing physical artifact-reference field. The field carries a canonical encoding of the typed document target with the artifact ID, schema and Space. `openInstance` validates that reference before emitting it across the actor boundary. Both the browser and native effect codecs preserve the target. The shipped demonstration Workflow declares its graph-only nodes with empty references.

## Verification

- The pinned Rust parser accepted the changed guest commands and effect codec owners. This check validates syntax without running the workspace type checker.
- The language-neutral effect fixture matched JSON serialization and the browser decoder for direct and tagged optional values. Malformed document identities were refused.
- The committed Space JSON and Pack descriptors round-tripped canonically after registering Studio, its Workflow codec and the hidden completion action.
- Rust coverage includes the pending creation request, Ready publication and duplicate handling, conflicting and stale completions, reopening after folding the node mutation, mounted Workflow validation, and the Studio editor's codec identity. Full Rust and browser journey verification remained with the parent ticket build.
- Component binding generation queued behind shared build and swap pressure. The queued request was cancelled without running a component build.

