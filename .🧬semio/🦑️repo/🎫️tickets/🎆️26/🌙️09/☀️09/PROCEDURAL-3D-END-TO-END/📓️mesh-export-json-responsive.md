# Responsive Polygon Mesh JSON Export

The schema-driven Export JSON widget now runs a funded polygon-source capture followed by a funded JSON encode. It yields between geometry and authored metadata events, uses separate capture and encode progress stages, and checks cancellation before publishing text.

The Mesh Engine exposes the typed polygon source to Pack's existing borrowed JSON writer. The writer measures the output, reserves its final backing under the operation's grant, then writes the same fields in bounded increments. The native projection reuses the existing JSON field, binary32, escaping and dynamic value codecs. Polygon maps are read in their canonical order. Texture bytes are emitted as JSON arrays along with the authored attributes, materials, normals, shading, UVs and seam records.

## Verification

- The scoped native Mesh Engine test compiled the live writer and matched the previous encoder byte for byte. The fixture includes a binary32 decimal, indexed attributes, nested Unicode and escaped text, a material and 1,536 texture bytes, and all four sparse surface record kinds.
- The same native test verified an insufficient allocation grant leaves progress unchanged, a real one-fuel job context publishes the identical final value, and cancellation after output reservation returns the cancellation fault and leaves text unpublished.
- The shared procedural Mesh TypeScript oracle passed 14 tests and 263 assertions. The Mesh Engine and Pack portable suites passed 4 tests and 51 assertions. Their independent output checks use Three.js and native JavaScript JSON.
- The widget fixture now expects an authored seam in the cube's exported JSON. The widget Rust source parses and the changed paths pass diff checking. The full procedural widget package was not linked in the scoped test.
