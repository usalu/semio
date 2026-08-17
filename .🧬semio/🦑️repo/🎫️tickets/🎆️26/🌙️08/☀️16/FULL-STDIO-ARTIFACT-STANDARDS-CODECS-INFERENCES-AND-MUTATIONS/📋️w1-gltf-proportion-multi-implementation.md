# glTF Proportion Multi-implementation Remediation

## Scope

This lane repairs the TypeScript measurement contract and the four proportion inference leaves: aspect ratios, elongation, flatness, and slenderness. It does not change the schema support ledger or claim runtime acceptance before the serialized Rust gate.

## Changes

- The TypeScript geometry kernel now emits the same `GltfMeasure<T>` shape as Rust: canonical unit names, availability and validity, diagnostic IDs, complete quality metadata, and complete provenance metadata.
- Existing TypeScript geometry leaves now use the canonical `squareMetre`, `cubicMetre`, and `inverseMetre` unit values.
- All four proportion leaves execute typed TypeScript formulas over sorted extents and expose explicit unavailable behavior.
- Each proportion leaf owns an available and unavailable canonical JSON vector consumed by TypeScript and Rust test sources.
- Each JSON Schema, GraphQL, and protobuf result facet now describes the typed measurement envelope instead of metadata-only marker fields or JSON blobs.

## Source Gates

- Four Bun shared-vector executions pass.
- Rust formatting checks pass for all four leaf modules.
- All result and vector JSON files parse with `jq`.
- Scoped `git diff --check` passes.

## Pending Runtime Gate

The Rust vector tests and the stdio registry integration must execute after the repository-wide fallible `MutationDiff::apply` migration reaches a coherent compile boundary. Until then these four leaves remain unimplemented in the public support ledger.
