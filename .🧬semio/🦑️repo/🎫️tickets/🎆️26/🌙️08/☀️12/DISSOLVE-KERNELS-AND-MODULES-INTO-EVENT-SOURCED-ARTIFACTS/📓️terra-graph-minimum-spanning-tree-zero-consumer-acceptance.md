# Graph Minimum-Spanning-Tree Zero-Consumer Acceptance

## Scope

- Ticket: `2026/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`
- Packet: `📓️sol-graph-minimum-spanning-tree-zero-consumer-packet.md`
- Baseline HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`
- Baseline graph-algorithms SHA-256: `8a44d0056f22306cb53ab57f3faf83851d2070778110cb2d7bf9210cf2b4fd18`

## Change

- Removed only public `minimum_spanning_tree` from `🧰️framework/🔨️modules/🕸️graph/🧮️algorithms/🦀️component.rs`.
- Removed only its exclusive tests: `minimum_spanning_tree_selects_cheapest_edges_without_cycles` and `minimum_spanning_tree_skips_out_of_range_edges`.
- Preserved `UnionFind`, its independent connected-components use, its contract test, and every other graph algorithm.

## Verification

- The initial graph-algorithms path was clean in both ordinary and cached diffs and matched the required baseline SHA-256.
- Post-edit SHA-256: `f00871395e1f8800989aa4ca10b9f408b173660a52aa3aeeb69ecf81b878b510`.
- Post-edit active Rust stale scan, `rg -n --glob '*.rs' minimum_spanning_tree .`, returned zero matches.
- Ordinary diff contains exactly the function and the two exclusive-test removals; cached diff is empty.

## Graph Gate

`bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache` exited with status 1 before graph tests, while compiling the external OS kernel. This unchanged OS SPR/store MutationOutcome/reconcile drift is outside this packet's lease:

- `E0432`: `crate::os_spr::ReconcileReport` is imported by `🧰️framework/🔨️modules/🏪️store/🦀️component.rs:23`, but `os_spr` has no `ReconcileReport` (the compiler identifies `RecoveryReport` as a similarly named item).
- `E0053`: `SpaceHistoryMutation::diff` at store line 5700 returns `SpaceHistoryDiff`, while the SPR trait requires `MutationOutcome<SpaceHistoryDiff>`.
- Store code expects `apply_mutation` to return `P`, but it returns `(P, Vec<MutationMessage>)`; corresponding `E0308`/`E0277` errors occur in replay, fold, materialization, application, and round-trip paths.
- Store calls `validate`, `reconcile`, and `MutationOutcome::apply` that its current generic bounds/types do not provide, producing `E0599` errors.

No external API was repaired. The zero-active-reference result and scoped ordinary/cached diffs remain the acceptance evidence for G-02.

## Files Changed

- `🧰️framework/🔨️modules/🕸️graph/🧮️algorithms/🦀️component.rs`
- `📓️terra-graph-minimum-spanning-tree-zero-consumer-acceptance.md`
