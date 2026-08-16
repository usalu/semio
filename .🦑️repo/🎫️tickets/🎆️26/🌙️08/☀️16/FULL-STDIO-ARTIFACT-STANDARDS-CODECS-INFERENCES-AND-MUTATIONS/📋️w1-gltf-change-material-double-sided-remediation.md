# glTF Change Material Double Sided Remediation

## Boundary

The leaf was demoted because its diff and inverse blindly overwrote `doubleSided`. It remains a candidate until the Rust integration gate runs.

## Remediation

- Diff and inverse records now carry expected state, target/prior state, and concrete touched paths.
- Application rejects missing targets, forged paths, stale forward state, and stale inverse state before mutation.
- Rust, TypeScript, JSON Schema, GraphQL, and Protobuf facets mirror the same fields.
- One canonical JSON vector executes mutation, direct diff, inverse, forward-stale, inverse-stale, forged-path, and serialization laws in both source implementations.
- The command-root executable descriptor delegates only to the leaf phases through the schema-owned descriptor contract.

## Evidence

- TypeScript canonical-vector execution exited `0`; retained output is `🧪️change-material-double-sided-typescript.log`.
- `rustfmt` completed on the modified Rust phase, contract, and descriptor files.
- Rust compile/test is pending the glTF descriptor assembly barrier, so no runtime acceptance is claimed.
