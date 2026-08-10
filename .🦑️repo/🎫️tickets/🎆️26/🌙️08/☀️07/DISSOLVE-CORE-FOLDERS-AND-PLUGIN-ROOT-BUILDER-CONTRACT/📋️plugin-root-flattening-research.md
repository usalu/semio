# Plugin Root Flattening Research

## Findings

- All 33 owners under `✏️s/🔌️plugins` contain a redundant `🔌️plugin` directory.
- Every nested contract contains `🦀️component.rs` plus `🛂️manifest`, `🎟️capabilities`, `🔧️setup`, and `🎛️apps` facet leaves.
- `🧱️block` and `🪐️space` already contain root `🦀️component.rs` files with shared domain code, so their registration functions must be merged into dedicated regions.
- Rust glue contains 40 nested-contract paths: 33 plugin entry leaves and seven additional facet leaves.
- The active contract is encoded in the taxonomy type/validator, workspace policy, plugin registry validator, Rust taxonomy assertion, and SDK documentation.
- The worktree already contains unrelated edits in many glue files; replacements must be limited to the exact nested-contract path strings.

## Intended Result

Each plugin owner directly contains `🦀️component.rs`, `🛂️manifest`, `🎟️capabilities`, `🔧️setup`, and `🎛️apps`, with no nested `🔌️plugin` directory or compatibility path.
