# Remodel Native Export Verification

The checked `mesh:out` route now registers and admits the editor's existing resumable GLB producer. The job reads from the exact captured snapshot, indexes the original bounded content leaves, preserves texture coordinates, and returns its retained read and other owners on completion or cancellation. Empty meshes produce a refusal. The snapshot Pack route continues admission after its producer takes the original request.

The active durable content store is a retained ordered map with fixed pages. The mesh reader compares the retained child ID at one byte per turn and reads the matching content by ordinal from the same snapshot. It never takes ownership of the content leaves. The neutral triangle fixture now records texture coordinates. Its checked-export law also covers an empty document.

## Verification

- The mutation bridge compiled and its Cargo test target succeeded with pinned Rust nightly, offline. Its target contains zero built-in tests. Refreshing the bridge lock was required for `--locked` and preserved all 64 registry package versions and checksums.
- Ajv 2020 accepted the edited neutral mesh fixture against its schema.
- The repo MCP Cargo and test calls timed out. Local `repo cargo` first encountered read-only host state and then failed writing the source-admission auxiliary descriptor after the host state was redirected. The authorized native fallback used the installed pinned Cargo toolchain and the repository's cached crates. Zig caches and build outputs were placed in the ticket's generated directory.

The native snapshot Pack producer still returns its captured read and drops its child view and operation context during request transfer. Mesh keeps all three until bounded close. The native Pack export law is the next check for whether that distinction affects the Remodel journey.
