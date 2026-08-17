# glTF Topology Multi-implementation Remediation

## Scope

This lane adds executable TypeScript behavior and shared vectors for boundary loops, Euler characteristic, genus, handles, and holes. It does not change the public support ledger before the serialized Rust tests pass.

## Changes

- The typed TypeScript geometry context now carries optional topology metrics alongside watertight, manifold, and orientation quality.
- All five topology leaves expose executable available and unavailable behavior in TypeScript.
- Every leaf owns a closed-tetrahedron vector and an unavailable vector.
- Each vector is executed by its TypeScript leaf and is consumed by one Rust family test that calls the leaf functions directly.
- Genus-derived leaves distinguish invalid input from a valid non-manifold topology whose genus is unavailable.
- Every JSON Schema, GraphQL, and protobuf facet now carries the typed count measurement, quality, and provenance envelope rather than a metadata marker or JSON blob.

## Source Gates

- All five Bun vector executions pass.
- Vector JSON parsing and scoped diff hygiene pass.
- The Rust family test is source-complete and queued for the serialized stdio gate.

## Pending Runtime Gate

No topology leaf is marked implemented or verified until the current-tree Rust family test and registry service path pass after the fallible mutation migration.
