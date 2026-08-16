# Graph Tidy Tree One Consumer Dissolution Acceptance

## Result

The sole production consumer now owns the Buchheim tidy-tree implementation privately within its existing DAG Layout region. The framework graph drawing component no longer exposes a tidy-tree module.

## Implementation

- Removed the complete TidyTree region from framework graph drawing.
- Removed the framework tidy-tree import from the OS directed DAG board component.
- Moved the Buchheim helpers and BuchheimNode implementation into a private TidyTree subregion of DAG Layout.
- Retained the two-node Buchheim contract as the private DAG layout test tidy_tree_tests::buchheim_tree_two_nodes.

## Current Source Hashes

- Framework graph drawing: f90afdc32c779b15369252ef1620e9c7c5eb2fe5a4345d5bec5fe07a825d349d.
- OS DAG component: 32b17350f49e2bfededa059bd442f5109e674c0a657b90306dbd4c735b386724.

## Acceptance Evidence

- A hidden Rust-source scan found zero references to drawing::tidy_tree and zero public tidy_tree modules.
- Buchheim symbols now occur only in the private DAG Layout subregion and its retained DAG test.
- Scoped ordinary and cached diff checks are clean.
- The pre-existing staged acceptance report remains staged; no index mutation was performed. The two leased source paths have no staged changes.

## Required Validation

- bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache: blocked before graph tests by shared OS SPR/store compilation drift.
- bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache: blocked by the same shared OS SPR/store compilation drift.
- Both commands report missing OS SPR reconciliation exports and incompatible MutationOutcome, validate, reconcile, and raw-diff call sites outside this ticket scope. Neither command reported a TidyTree move error.

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- `🧰️framework/🔨️modules/🕸️graph/🖊️drawing/🦀️component.rs` was clean at SHA-256 `39ebcf6e71018dc28386886b31c3d1eba9d3dee4b9d454ef28712e40b9286b1b`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs` was clean at SHA-256 `955e7f921feb9091e09060ef021ef1b99d8b791889135197a25736ae87c41bff`.
- `graph::drawing::tidy_tree::buchheim_positions` has one production consumer: the OS directed DAG board component. Its other reference is its own test.
