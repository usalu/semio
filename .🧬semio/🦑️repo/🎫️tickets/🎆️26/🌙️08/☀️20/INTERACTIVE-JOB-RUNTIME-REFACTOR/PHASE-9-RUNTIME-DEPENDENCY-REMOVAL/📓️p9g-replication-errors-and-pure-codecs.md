# P9g Replication Errors and Pure Codecs

## Outcome

- Replaced the replication crate's `thiserror` derives with owned `Display`, `Error`, `source`, and `From` implementations for pack, protocol, and mutation-apply errors.
- Removed the replication crate's direct `thiserror` dependency.
- Made pure scalar timestamp/id codecs and dictionary operations synchronous.
- Made the pure `RecordHasher::hash` seam synchronous while retaining asynchronous signer and verifier boundaries.
- Updated exact replication, OS SPR, and server callers without changing I/O boundaries.
- Removed the dead `typegen`/`ts_rs` annotations from the replication wire model.
- Corrected the replication package's permanent `📜️script.ts` Cargo argument ordering so Nx forwards Cargo flags correctly.

## Verification

Command:

```text
CARGO_TARGET_DIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-DEPENDENCY-REMOVAL/🧪️target-replication-errors' bun nx run @semio-tech/framework-replication-rs:test-quick --skip-nx-cache
```

Result: `184 passed; 0 failed`, doc tests `0 failed`, and no compiler warnings.

## Scope Boundary

This packet does not claim the Phase 9 dependency gate. Other runtime dependencies and the full cross-product gate remain open.
