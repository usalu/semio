# Shared Replication I/O Contracts

## Change

`OpText`, `OpBinary`, and `DiffCodec` are now owned by replication `🚪️io`, with representation-specific text and binary modules. Owned operation-byte backing and its runtime laws moved from semantic mutation to binary I/O. The protocol facade exports these contracts from their new owner; semantic mutation contains no codec aliases. Existing callers were updated to the canonical I/O paths.

No new runtime dependencies were introduced. Existing allocation, progress, cancellation, refusal, and ownership behavior is retained.

## Validation

- Before extraction, the new language-neutral ownership fixture failed because the prescribed I/O text owner did not exist.
- Direct Bun ownership checks passed after extraction, independently checked with minimatch and Ajv.
- `bun nx run @semio-tech/framework-replication-rs:test-source` passed.
- `bun nx run @semio-tech/framework-replication-rs:test -- quick --lib owned_operation_byte_pages -- --nocapture` passed: seven native runtime laws, zero failures. Runtime `[DEBUG]` output confirmed exact retained octets, physical backing return, canonical comparison, capacity refusal, cumulative measurement, and cancellation behavior after the move.
- Both Nx tasks waited for shared project graph and Cargo preparation ownership, then completed successfully. No global caches, locks, or other agents' processes were reset.

The ownership case runs through the existing replication source-test route, already declared in Nx and launch configuration. Its fixture and feature are co-located with the I/O owner.
