# Graph Minimum-Spanning-Tree Zero-Consumer Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`
- Graph algorithms SHA-256: `8a44d0056f22306cb53ab57f3faf83851d2070778110cb2d7bf9210cf2b4fd18`; clean.

## Consumer Evidence

`minimum_spanning_tree` has zero production references, imports, wildcard imports, or reexports across active Rust source. Its only calls are two same-file tests. Tests do not qualify as production consumers. The shared `UnionFind` remains independently live in connected-component computation and its contract test.

## Lease

Delete only `minimum_spanning_tree` and its two exclusive tests. Preserve all other graph algorithms, `UnionFind`, graph glue, Cargo, generated files, and the active T-01 graph-drawing delta.

Writable paths:

- `🧰️framework/🔨️modules/🕸️graph/🧮️algorithms/🦀️component.rs`

Validation:

```text
bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache
```

If the graph gate remains blocked before graph tests by the external OS SPR/store MutationOutcome/reconcile drift, record the exact unchanged blocker and rely on zero active references plus ordinary/cached diff checks. Do not repair unrelated APIs.
