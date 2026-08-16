# Terra Graph Algorithm Zero-Consumer Pruning Acceptance

## Scope

- Ticket: `2026/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`
- Leased source: `🧰️framework/🔨️modules/🕸️graph/🧮️algorithms/🦀️component.rs`
- Baseline HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`
- Pre-edit source SHA-256: `f00871395e1f8800989aa4ca10b9f408b173660a52aa3aeeb69ecf81b878b510`
- Final source SHA-256: `1ad85fc63b8c5aae506af613c2f1c54ca1734f50fcdf48a21a8a7bc2bc3ff210`

## Changes

- Deleted the ten zero-terminal public algorithms: `acyclic_edge_subset`, `dfs_preorder`, `dfs_postorder`, `dijkstra_path`, `find_cycle`, `in_degrees`, `longest_path_layers`, `out_degrees`, `root_indices`, and `shortest_path_unweighted`.
- Deleted 12 exclusive test functions for those algorithms.
- Preserved the independent Dijkstra assertion from the combined out-of-range Dijkstra/Dijkstra-path test; it is now `dijkstra_out_of_range_from_returns_empty`.
- Changed exactly three retained implementation helpers from public to private: `bfs_order`, `is_reachable`, and index-based `would_create_cycle`.
- Retained public `would_create_cycle_ids`, `topo_sort`, `topo_levels`, and `dijkstra`; retained private `find_cycle_among`.

## Static Evidence

- Exact-word active authored Rust/TypeScript/JavaScript/JSON/TOML scan for all ten deleted symbols returned no matches (ripgrep exit 1 for no matches).
- `would_create_cycle_ids` remains public and is consumed by Infinite DAG at `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:1149`.
- `find_cycle_among` remains private and is called by `topo_sort` and `topo_levels`; both APIs and their tests remain. `dijkstra` and its three surviving tests remain.

## Diff Evidence

- Pre-edit ordinary diff was exactly the accepted uncached MST change: 33 deletions, removing `minimum_spanning_tree` and its two exclusive tests.
- Final ordinary diff for the leased file is 4 additions and 289 deletions. The accepted MST 33-line deletion remains unchanged; this pruning/visibility hunk contributes 4 additions and 256 deletions.
- `git diff --check -- <leased source>` exited 0 with no output.
- `git diff --cached --check -- <leased source>` exited 0 with no output.
- `git diff --cached --numstat -- <leased source>` was empty.

## Required Graph Gate

Command run twice:

```text
bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache
```

Both invocations reached `cargo nextest run --no-tests warn --profile quick -p semio-framework-graph -- --skip long:: --skip exhaustive::`, then exceeded the runner's 30,000 ms budget while waiting on Cargo's shared build-directory lock. Each exited 1 with:

```text
[budget] cargo nextest run --no-tests warn --profile quick -p semio-framework-graph -- --skip long:: --skip exhaustive:: exceeded 30000ms — killed.
```

The second attempt compiled `semio-framework-graph` successfully before the timed `nextest` invocation was killed. No Graph test result was emitted, so this change is source-complete but the required Graph gate is not green. The observed warnings were unrelated existing diagnostics in OS kernel, SPR, store, and DSL sources; no SPR/store edit was made.
