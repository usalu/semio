# Graph BFS Layers and ID Index Privacy Acceptance

## Completion

- Removed the unconsumed `bfs_layers` API and its exclusive `bfs_layers_group_by_distance` test from `🧰️framework/🔨️modules/🕸️graph/🧮️algorithms/🦀️component.rs`.
- Restricted `IdIndex::edges_to_indices` to the component-private visibility required by its sole live caller, public `would_create_cycle_ids`.
- Retained `bfs_distances`, the private `bfs_order` reachability chain, public `IdIndex` construction and lookup APIs, `would_create_cycle_ids`, and all unrelated tests.

## Evidence

- Required pre-edit SHA-256 matched: `1ad85fc63b8c5aae506af613c2f1c54ca1734f50fcdf48a21a8a7bc2bc3ff210`.
- Final source SHA-256: `f2be094489268159fe7002789e160cc81216d808c76623357c9587451f97a168`.
- Scoped ordinary diff: `5` additions and `321` deletions. This preserves the accepted prior `4` additions and `289` deletions; this lease adds one visibility change and removes the 31-line BFS-layers implementation/test surface.
- Scoped cached diff: empty.
- Active-source scan found no `bfs_layers` occurrence. It found one private `edges_to_indices` definition and one retained internal call from `would_create_cycle_ids`.

## Validation Queue

At completion, external Cargo and rustc jobs were actively using the shared `target` build directory. Per the packet, `bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache` was not started or retried. Source status is complete and not green; run that queued gate after the shared Cargo workload clears.
