# P9h Owned Errors in Machine, Pack, 3D, and Server

## Outcome

- Replaced `thiserror` derives with owned `Display` and `Error` implementations in the Machine statechart kernel, Pack streaming JSON parser, 3D mesh kernel, and Server authority/gateway/storage layers.
- Removed the direct `thiserror` dependency from all four packages.
- Removed already-unused `thiserror` dependencies from Editor and Graph after their earlier owned-error conversions.
- Corrected Server's permanent `📜️script.ts` Cargo invocation ordering.
- Cleared three stale awaits on synchronous Pack varint/CRC helpers exposed by the package gate.

## Verification

- Machine: Nx `test-quick`, `31 passed; 0 failed`.
- Pack: Nx `test`, `66 passed; 0 failed`.
- 3D: Nx `test-quick`, `62 passed; 0 failed`.
- Server: Nx `test`, `73 passed; 0 failed`.
- Editor, Graph, and Surface were compiled through their isolated Nx gates, but their final gates were blocked upstream by the concurrently active OS Store registry de-async repair. No pass is claimed for those three packages yet.

## Scope Boundary

This packet removes direct package dependencies only. Transitive dependencies and the remaining Phase 9/P10 replacement families remain open.
