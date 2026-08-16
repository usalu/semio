# Run Test-Only Memory Media Cache Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Run component SHA-256: `4517e65d98ee41c73ca7b2375c2330effa52edc505506cccc4e785189913b18e`; clean.
- Applicable instructions: root and OS product `AGENTS.md`.

## Consumer Evidence

- The `MediaCache` contract is live through the public `SpaceRunner::run` boundary.
- `FileMediaCache` is live through the run binary's production `SpaceBundle::media_cache()` consumer and remains unchanged.
- `InMemoryMediaCache` has zero production consumer. Every construction is inside the same file's `#[cfg(test)]` module. Tests do not qualify a production module or public API.

## Disposition

Delete the public top-level `InMemoryMediaCache` and its production `MediaCache` implementation. Recreate the minimal map-backed implementation as a private `TestMediaCache` inside the existing test module and update the five same-component test constructions. Do not move or change `MediaCache`, `FileMediaCache`, `SpaceRunner`, persistent paths, media fingerprints, or binary behavior.

Terra writable paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- one unique Terra acceptance Markdown

## Verification

The package currently has no `project.json` or `📜️script.ts`, so there is no honest package-local Nx test target. Do not invent central glue in this lease. Run the focused package test through Cargo as a recorded repository-structure exception:

```text
cargo test --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml --lib
```

Also require active non-test references to the deleted symbol to be zero, private test fixture refs only, and scoped ordinary/cached diff checks. If external SPR channel drift blocks compilation, record the exact blocker and preserve source-static acceptance.
