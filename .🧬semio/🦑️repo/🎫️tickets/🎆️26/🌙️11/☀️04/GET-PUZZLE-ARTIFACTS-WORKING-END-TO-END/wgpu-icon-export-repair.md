# WGPU Icon Export Preparation

The prepared scene now uses the caller's `OriginalJobReturnOrigin` to reserve the real receiver before driving `PreparedRenderJob`. The scene receives the original descriptor or continuation with its return witness. It acknowledges a normal outcome before taking the packet, and closes the origin and producer before publishing the scene. Failure and cancellation drain pending returns through the same origin. Continuation causes remain in the retained capsule until closure, with their primary messages surfaced as the export fault.

Prepared owners use the shared UI `prepared_close_turn` ladder. It lends the caller's `RetainedAllocationEffect` and settles its completed, pending, or refused poll against the original work and retained progress. The grant-shaped rejected input closes with the grant from that effect and converts its refusal into the ladder's typed exit.

The export-batch fixture checks a denied turn, a pending turn that consumes work and retains its backing, a complete turn that releases the fixture's bytes and credits, and a typed refusal. The existing GLB material test advances and closes the icon scene and packet through a single original caller, and verifies that it can take the accepted scene once.

Rustfmt parsed the edited Rust files and `git diff --check` passed. Native compilation uses the coordinator's shared Cargo cache.
