# 2D Dead Parallel Cargo Closure Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- 2D Cargo manifest SHA-256: `d496e5cb1a5708999eda1807f510695214eb9fd92da35dd90fe4868e450fa62d`; clean.
- Root Cargo lock SHA-256: `f1d857b9bf1614e3f791dfb4d7963d319d527a0c3e9acbcd3f836d43803ccffd`; clean.
- Active 2D source/config search finds `parallel`, `rayon`, and `futures` only in this manifest after the accepted `run_blocking` deletion.

## Disposition

Delete the dead `parallel` feature, remove it from the default feature set, and delete the 2D crate's optional `rayon` and `futures` dependencies. In `Cargo.lock`, remove only `futures` and `rayon` from the `semio-framework-2d` package dependency list; both registry packages remain because other workspace packages still consume them.

Coordinator writable paths:

- `🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/Cargo.toml`
- `Cargo.lock`
- one unique coordinator acceptance Markdown in this ticket

## Verification

```text
cargo metadata --offline --format-version 1 --manifest-path 🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/Cargo.toml
bun nx run semio-framework-2d:test-quick --skip-nx-cache
```

Require active 2D stale search zero, exact lock diff, unchanged lock registry packages, and scoped ordinary/cached diff checks.
