# Graph Algorithm Zero-Consumer Pruning Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Writable source: `🧰️framework/🔨️modules/🕸️graph/🧮️algorithms/🦀️component.rs` only, plus one uniquely named Terra acceptance record.
- Source SHA-256: `f00871395e1f8800989aa4ca10b9f408b173660a52aa3aeeb69ecf81b878b510`.
- The source has one accepted ordinary diff and no cached diff: 33 deletions for the already released `minimum_spanning_tree` function and its two exclusive tests. Preserve that diff exactly.

## Resolved Consumer Graph

Active authored Rust/TypeScript/JavaScript/JSON/TOML resolution found no reference outside the algorithms component to:

- `acyclic_edge_subset`;
- `dfs_preorder`;
- `dfs_postorder`;
- `dijkstra_path`;
- `find_cycle`;
- `in_degrees`;
- `longest_path_layers`;
- `out_degrees`;
- `root_indices`;
- `shortest_path_unweighted`.

Tests and internal calls do not create production consumers. Delete those ten zero-terminal public algorithms and their exclusive tests. Preserve the combined Dijkstra test's independent `dijkstra` assertion. Preserve private `find_cycle_among`, because live `topo_sort` and `topo_levels` use it.

`bfs_order`, `is_reachable`, and index-based `would_create_cycle` are an internal chain supporting public `would_create_cycle_ids`. That ID API is consumed through Infinite DAG by multiple terminal application components. Retain the chain but make those three implementation functions private. Their tests remain valid behavioral coverage and do not count as consumers.

Do not alter glue, Cargo files, manifests, generators, imports outside the leased file, or the external Infinite DAG wrapper.

## Required Evidence

1. Reread applicable `AGENTS.md`; verify HEAD, source hash, existing accepted diff, and cached state before editing.
2. Use `apply_patch` only. Do not use any modifying Git command.
3. Verify all ten removed symbols have zero active authored-source references. Verify the three retained helpers are private and `would_create_cycle_ids` remains public and consumed by Infinite DAG.
4. Verify `find_cycle_among`, `topo_sort`, `topo_levels`, `dijkstra`, and their surviving tests remain.
5. Run scoped ordinary and cached `git diff --check` and characterize the complete file diff, including the earlier accepted MST deletion.
6. Run:

   ```text
   bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache
   ```

   If the target is blocked before Graph tests by the actively moving SPR/store migration, report the exact external leading blocker without editing it. Do not claim green unless Graph's complete gate passes.
7. Record final SHA-256, exact deletion/visibility counts, static evidence, and command result in the unique ticket acceptance record.
