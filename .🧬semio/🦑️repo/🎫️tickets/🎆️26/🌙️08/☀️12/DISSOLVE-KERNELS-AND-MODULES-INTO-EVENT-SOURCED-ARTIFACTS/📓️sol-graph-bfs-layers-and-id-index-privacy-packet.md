# Graph BFS Layers and ID Index Privacy Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Writable source: `🧰️framework/🔨️modules/🕸️graph/🧮️algorithms/🦀️component.rs` only, plus a unique acceptance record.
- Required source SHA-256: `1ad85fc63b8c5aae506af613c2f1c54ca1734f50fcdf48a21a8a7bc2bc3ff210`.
- Preserve the accepted ordinary `4` additions / `289` deletions and empty cached diff from prior Graph algorithm pruning.

## Disposition

- `bfs_layers` has zero authored production consumers, imports, registrations, glue consumers, examples, or language-mirror consumers outside its own component. Delete the function and its exclusive `bfs_layers_group_by_distance` test.
- `IdIndex::edges_to_indices` has exactly one production call site, within the same component's public live `would_create_cycle_ids`. Make it private; do not inline or alter behavior.
- Preserve `bfs_distances`, the private live `bfs_order` chain, `IdIndex` public constructors/lookups, `would_create_cycle_ids`, and all other tests.

## Evidence

Use `apply_patch` only and no modifying Git command. Rehash before editing, preserve concurrent/accepted diffs, perform active-source stale scans, scoped ordinary/cached diff checks, and run `bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache` only if no external Cargo job holds the shared build directory. If Cargo remains saturated, do not start or repeat a lock-waiting gate; record source-complete/not-green and queue validation. Report final SHA and complete cumulative diff.
