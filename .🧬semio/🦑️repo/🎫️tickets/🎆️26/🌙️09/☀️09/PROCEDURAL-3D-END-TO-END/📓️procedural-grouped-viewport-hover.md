# Procedural grouped viewport hover

Edit, Generate and View attach all directional ports that share exact preview geometry at the same pose. The scene carries one set of hover addresses for each such instance. It preserves per-port render identities and point, vector, plane, mesh and retained-handle identity. An inline mesh shares its buffer only when both its preview and component data match.

A viewport ray publishes the whole authored group. The ray winner still supplies the click target and the viewport's immediate paint. Component picking retains its own targets. The receiving hover channel clears the entire group on pointer leave. Each renderer uses the ordinary ray target if authored aliases are missing, invalid or exceed the 4 KiB action string limit.

## Verification

- Chromium WebGL mounted the production World3d renderer. It picked the shared input and output, selected one on click, cleared both on exit, picked a translated instance sharing the mesh on its own, and published a changed group with the same winning instance. Graph-originated marks addressed each rendered directional instance without replacing geometry. The separate four-instance probe verified multi-region paint, pointer override, source handoff and clear.
- Four focused Rust tests compiled the actual Edit and View builders with the real pack JSON library. They checked retained mesh reuse, exact inline grouping, component mode, viewport and export direction, and the admission of a late input tessellation. Generate uses the shared Edit builder.
- The portable fixture checks include exact inline groups via SQLite, renderer wire limits and malformed groups, unrelated targets, component edge fallback, directional graph identities, and wire endpoints. The existing 24 component and hover publication tests passed under Vitest with jsdom. Chromium supplied the real WebGL paint check.
- Four focused tests passed against freshly managed production WGPU libraries. They exercised native ray picking, selection, pointer clear and re-entry, action bounds, scene replacement and retirement. The stock test binaries are held by concurrent terrain definitions, and this witness did not mount a WGPU window. See the [native report](📓️native-grouped-hover.md).

The full linked preview evaluation and canonical Edit, Generate and View mounts remain open.
