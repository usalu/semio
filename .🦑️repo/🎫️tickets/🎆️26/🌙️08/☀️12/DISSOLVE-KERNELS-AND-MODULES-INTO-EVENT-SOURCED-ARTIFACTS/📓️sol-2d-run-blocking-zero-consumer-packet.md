# 2D Run-Blocking Zero-Consumer Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`
- 2D engine SHA-256: `d63d2dd3636dea3795d9d1ad4a9e01167c81e1bc57d9de52c32f87da845ea59c`; clean.

## Consumer Evidence

`compute::run_blocking` has exactly two active Rust hits: its definition and its own public reexport. It has no production call, import, or external reexport. `compute::block_on` remains independently live in the Flow draw extension and must remain.

## Lease

Delete only `run_blocking`, remove it from the public reexport, and update the compute module description to match the retained synchronous `block_on` capability. Do not touch `block_on`, 2D booleans/trace, Cargo, root lock/configuration, OS/Flow, the dirty Draw artifact, renderer, or stdio.

The package's `parallel` feature and optional `rayon`/`futures` dependencies become dead configuration after this source deletion. They are intentionally reserved for the root Cargo-lock authority because `Cargo.lock` remains protected; record this follow-up rather than modifying Cargo surfaces in this lease.

Writable path:

- `🧰️framework/🔨️modules/◻2d/⚙️engine/🦀️component.rs`

Validation:

```text
bun nx run semio-framework-2d:test --skip-nx-cache
```

The current gate is independently baselined as failing before 2D tests in external OS store/SPR code. Acceptance still requires zero active `run_blocking` references, retained `block_on` references, exact scoped diff, and ordinary/cached diff checks.
