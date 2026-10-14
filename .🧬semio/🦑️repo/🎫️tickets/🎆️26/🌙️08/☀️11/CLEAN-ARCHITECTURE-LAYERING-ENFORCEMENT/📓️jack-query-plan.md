# Jack Retained Graph Query Plan

Jack owns topology, root choice and displacement accumulation. Stdio owns literal graph/value encoding. A schema-first Jack query port therefore exposes retained ArtifactChild identity plus node, endpoint and displacement queries; it does not duplicate Semio graph records or public types. Exact parent/child identity admission belongs to Jack, while Semio dialect selection and typed u/v conversion belong to the specific s dev Trinity composition.

Portable fixtures will exercise non-Semio dialects, missing/wrong child identity, duplicate nodes/edges, dangling endpoints, shuffled deterministic traversal, disconnected components, cycles and offsets. Graphology independently computes topology/depth and reachability-based positions. The topology API continues reporting cycles with cycleFree=false; it does not invent cycle-free order or depths for blocked nodes. Flat inference preserves the existing deterministic component walk on cycles.

The only current TypeScript callers are two Rewriting tests; those cross-owner integration laws will move to the specific composition. Intrinsic tests live in Jack and its existing Nx test surface. Specific integration receives an Nx target and launch registration. Rust Jack currently owns broader Semio-backed child materialization; this TypeScript public-boundary change does not claim Rust deletion independence.
