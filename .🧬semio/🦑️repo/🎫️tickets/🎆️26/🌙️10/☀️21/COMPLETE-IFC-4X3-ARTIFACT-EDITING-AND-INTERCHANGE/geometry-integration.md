# IFC4x3 Model Window Integration

The IFC4x3 editor now declares the shared mesh window beside its declaration-aware details window. Both read the same document graph; the mesh route uses the existing owned generic IFC representation projector, including the live request's cancellation, yield and fuel context. It never passes ADD2 data through the IFC4 STEP codec.

The existing shared projector supplies actual representation vertices, triangles, placements, stable product identities, camera bounds and explicit representation diagnostics. Its retained render suppresses meshes for cancelled or limited projection. Unsupported representations remain diagnostics; this does not establish complete IFC4x3 geometric support.

## Verification

- Added language-neutral editor scenario before implementation and native tests for both declared windows, official ADD2 tessellation and cancellation without visible geometry.
- Added executable Bun test to the existing independent-oracle suite: official ADD2 fixture yields eight vertices, twelve triangles, source-unit bounds `[500,-500,0]` to `[1500,500,2000]`; cancelled projection emits empty mesh and instance arrays.
- Bun oracle suite without configured IfcOpenShell output: two tests passed, fourteen assertions; the two environment-dependent IfcOpenShell tests were skipped. The other executed test independently loads exported GLB with three.js and verifies world bounds and topology.
- The existing IfcOpenShell evidence for this same immutable fixture has eight vertices, twelve triangles and metre bounds `[0.5,-0.5,0]` to `[1.5,0.5,2]`; see `exchange-implementation.md` and `ifc-oracle.py`.
- New Rust editor/tests parse and format successfully with rustfmt. Native compilation and execution remain unverified because the repository compiler ingress requires the authentic launcher. Other agents own build restoration; no enforcement was bypassed.

## Files

- IFC4x3 `✏️editor/🦀️.rs`, `✏️editor/tests.rs`, `✏️editor/tests.feature`.
- IFC4x3 Rust package dependency on the existing IFC4 surface crate that owns the generic graph renderer.
- IFC4x3 `🚪️io/🧪️tests/oracles.test.ts`.

Full geometry, full EXPRESS semantics, all MVD constraints, full artifact exchange and native host end-to-end verification remain open goal work.
