# 3D Dead Parallel Cargo Closure Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- 3D Cargo manifest SHA-256: `7fc2fe4cd2f6ec4de42611efa7d8ddfbaa0571422175888d2d8677e55f57a63a`; clean.
- Cargo lock SHA-256: `f6b19ca66228b424ab1cc19e6b97cb7a5a1e43fb5c0a96c750d2f7e676a75427`; dirty only for the coordinator-owned, accepted 2D removal of the same two dependency edges.

## Consumer Evidence

Active 3D authored Rust contains no `rayon`, `futures`, `run_blocking`, or parallel-feature implementation. Workspace consumers use the crate's defaults or its independent `brep` feature; none explicitly requests `parallel`. The feature and two optional dependencies have zero implementation consumer. Other workspace packages still consume the registry crates, so their lock packages remain.

## Disposition

Remove `parallel` from the 3D default feature list, delete the feature, and delete the optional `rayon`/`futures` manifest edges. Remove only those two names from the `semio-framework-3d` Cargo lock package block, preserving the accepted 2D lock hunk and all registry packages.

Coordinator writable paths:

- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml`
- `Cargo.lock`
- one unique coordinator acceptance Markdown

Validation:

```text
cargo metadata --offline --format-version 1 --manifest-path 🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml
bun nx run semio-framework-3d:test-quick --skip-nx-cache
```

Require active stale refs zero, exact lock diff, and scoped ordinary/cached diff checks.
