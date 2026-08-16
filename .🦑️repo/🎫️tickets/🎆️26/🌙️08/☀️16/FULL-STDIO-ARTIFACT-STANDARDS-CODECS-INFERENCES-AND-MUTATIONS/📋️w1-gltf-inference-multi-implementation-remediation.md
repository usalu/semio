# GLTF Inference Multi-Implementation Remediation

## Scope

This remediation owns the first literal batch of 20 inference leaves: size (7), area-volume (8), and compactness (5). It does not touch mutations.

## Source Progress

A non-public TypeScript geometry kernel now exposes typed point/triangle contexts, bounds, triangle surface area, signed volume, canonical exact/unavailable measure envelopes, quality, diagnostics, and provenance. The 20 assigned TypeScript leaf modules now provide executable `inferGltf*` and `unavailableGltf*` functions. They derive their own typed measures from that context instead of exposing descriptor-only metadata.

The following source-only Bun import gate passed after this step:

```sh
bun -e "await Promise.all(process.argv.slice(1).map((file)=>import(file)))" $(find ... -name component.ts | rg '/(📦️size|🧱️area-volume|⚪️compactness)/')
```

No leaf is accepted yet: complete typed JSON/GraphQL/Proto result schemas, root result-facet parity, shared Rust/TypeScript vectors, and direct test matrix evidence remain required before an accepted count can be reported.

## Quality And Validity Correction

The TypeScript kernel no longer fabricates `watertight`, `manifold`, or `consistentlyOriented` quality claims. A caller may provide those topology facts only when it has established them; otherwise the fields remain absent. Exact denotes deterministic evaluation of the supplied typed geometry context, not a claim that the source glTF is closed or manifold. Unavailable results retain the supplied diagnostics, use zero coverage, and contain no value.

The bounded Bun import command was rerun after this correction and passed. The 20 leaves still have opaque/metadata-only result schemas and no shared runtime vectors, so the exact accepted count remains **0/20**.
