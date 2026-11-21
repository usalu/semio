# Mesh Orientation Ownership Contract

The retained orientation cursor writes a component identifier and a stack entry when it discovers a root face. Discovering a neighbor also writes its winding parity. On the 64-bit test target those writes cost 16 and 17 bytes. The fixture already recorded these amounts. The schema still described a one-word stack entry plus a visited flag.

The corrected schema validates the fixture through Ajv. The portable test independently counts the backing writes on each root and neighbor visit before it compares the totals. Independent Node verification reproduced 16, 101, and 33 bytes from the same fixture. This covers the one-face open surface, the closed cube, and two open faces with inconsistent winding.

A compact executable linked the current production orientation cursor through the same repository toolchain as the Bevel check. The actual receipts were 16, 101, and 33 bytes for the three fixtures. The inward cube charged 102 bytes, including the outward sign change, and reported six flipped faces. Each attempt with a one-byte-short copy grant returned an empty receipt and left the public progress unchanged.

All 11 tests in the portable Mesh jobs file passed in a TypeScript-to-Node assertion replay, with 269 assertions. The replay used the fixture, Ajv, Three.js and JSON Patch from the workspace. A separate Bun rerun waited on host memory.
