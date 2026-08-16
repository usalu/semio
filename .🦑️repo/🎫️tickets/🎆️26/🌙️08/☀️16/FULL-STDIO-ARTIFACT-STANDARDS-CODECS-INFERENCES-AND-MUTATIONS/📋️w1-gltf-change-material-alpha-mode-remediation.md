# glTF Change Material Alpha Mode Remediation

## Boundary

The leaf was demoted after audit because its diff and inverse blindly overwrote `alphaMode`. It is not accepted merely because its facets exist.

## Remediation

- The diff now carries `expectedAlphaMode`, the target `alphaMode`, and concrete `touchedPaths`.
- Diff application rejects a missing material, forged paths, and a current value that differs from the expected pre-state.
- The inverse now carries the expected forward value, the prior value, and concrete paths.
- Inverse application rejects a missing material, forged paths, and a current value that differs from the planned forward result.
- Rust, TypeScript, JSON Schema, GraphQL, and Protobuf phase facets mirror the new fields.
- The command-root descriptor delegates decoding, planning, inverse reconstruction, application, and path reporting to the leaf phases through the schema-owned descriptor contract.
- One canonical JSON vector now drives executable Rust and TypeScript laws for mutation, direct diff, inverse, stale forward, stale inverse, forged paths, and serialization.

## Evidence

- TypeScript canonical-vector execution exited `0`; the retained debug evidence is `🧪️change-material-alpha-mode-typescript.log`.
- All four JSON facets parsed with Bun.
- `rustfmt` completed on the three modified Rust phase/contract files and the descriptor root.
- Rust compile and contract execution remain pending the glTF descriptor/glue integration barrier. The leaf remains a candidate until that gate passes.
