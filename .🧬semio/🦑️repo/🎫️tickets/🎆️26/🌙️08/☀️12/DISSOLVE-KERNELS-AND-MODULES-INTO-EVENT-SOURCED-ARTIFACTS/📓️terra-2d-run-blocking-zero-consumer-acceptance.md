# 2D Run-Blocking Zero-Consumer Acceptance

## Scope

- Baseline engine SHA-256: `d63d2dd3636dea3795d9d1ad4a9e01167c81e1bc57d9de52c32f87da845ea59c`.
- Post-edit engine SHA-256: `90d3b3a400460424250d339c6c64ffb9770ceb50be42f5cdd15789fb43641938`.
- Changed only `🧰️framework/🔨️modules/◻2d/⚙️engine/🦀️component.rs` and this acceptance record.

## Result

- Removed `compute::run_blocking` and its public reexport.
- Retained `compute::block_on`, its type/error vocabulary, and every other 2D source surface.
- Updated the compute module documentation to describe synchronous waiting for async kernel calls.
- A Rust/TOML active-source scan has zero `run_blocking` hits; `block_on` remains defined and publicly reexported.
- The `parallel` feature and its optional `rayon`/`futures` dependencies are now dead configuration. Cargo and lockfile ownership is outside this lease; root Cargo-lock authority should remove them in a follow-up.

## Verification

- Scoped ordinary and staged diffs contain only the requested engine deletion, reexport removal, and documentation update; both scoped whitespace checks are clean.
- Repository-wide ordinary and staged whitespace checks remain blocked by concurrent unrelated ticket-note and library-test changes. Scoped ordinary and staged checks for this lease are clean; all reported global paths remain untouched.
- `bun nx run semio-framework-2d:test --skip-nx-cache` was run and did not reach 2D tests. It is blocked while compiling `semio-framework-os-kernel` by unchanged external OS store/SPR errors: `MutationOutcome<…>` lacks `apply` at store component lines 1036, 1105, and 1172. No OS or SPR file was changed.
