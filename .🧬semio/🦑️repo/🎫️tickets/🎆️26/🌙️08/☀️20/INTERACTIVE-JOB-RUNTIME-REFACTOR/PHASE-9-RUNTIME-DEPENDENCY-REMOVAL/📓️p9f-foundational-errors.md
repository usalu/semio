# P9f — Foundational Owned Error Types

## Outcome

Replaced `thiserror` derives with explicit owned `Display`, `Error`, `source`, and `From` implementations for:

- manifest version, version-requirement, dependency-graph, and media errors;
- shared diagnostic `TextError`;
- editor JSON/pack errors;
- clipboard errors;
- graph manifest and graph DSL errors.

The implementations preserve the existing messages and typed source chains. No public error variant or conversion behavior changed.

## Verification

```text
CARGO_TARGET_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-core-errors \
  bun nx run @semio-tech/framework-rs:test-quick --skip-nx-cache

Rust:       161 passed, 0 skipped
TypeScript:  87 passed
NX Successfully ran target test-quick for project @semio-tech/framework-rs
```
