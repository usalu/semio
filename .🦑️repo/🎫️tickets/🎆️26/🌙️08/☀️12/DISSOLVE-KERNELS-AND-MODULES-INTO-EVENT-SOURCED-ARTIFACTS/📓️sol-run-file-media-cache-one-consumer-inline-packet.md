# Run File Media Cache One-Consumer Inline Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Run component current SHA-256: `6cadb4e0bf0300a0fcd7ce3e0e807eef8ed27daa128a5cdf79a6d73d07651ac7`; dirty only for the accepted `TestMediaCache` relocation and its local prose.
- Run binary SHA-256: `f5bd7e3561a3a1fe8bb30956b966bcf5e7d23cadaa29717edda620c2cbd25b96`; clean.

## Consumer Evidence

`FileMediaCache` has exactly one terminal production consumer: the `os run` binary, through `SpaceBundle::media_cache()`. No other active source constructs, imports, registers, mounts, or names it. The `MediaCache` contract remains live through `SpaceRunner::run`. The persistent path contract remains `SpaceBundle::media_cache_dir()` and `<space>/cache/media/<fingerprint>.json`.

## Disposition

Move the file-backed cache implementation into the sole consuming binary as a private cohesive implementation. In the run component, delete the public `FileMediaCache` type/impl and the one-consumer `SpaceBundle::media_cache()` constructor; retain `media_cache_dir()`, `MediaCache`, `SpaceRunner`, and the accepted private test cache. In the binary, define private `FileMediaCache`, implement the repository-owned `MediaCache` interface, and construct it from `bundle.media_cache_dir()`.

Terra writable paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs`
- one unique Terra acceptance Markdown

## Validation

Preserve exact disk format/path behavior and the preceding accepted run diff. Require active `FileMediaCache` references only in the consuming binary, zero `SpaceBundle::media_cache()` calls/definition, scoped ordinary/cached diff checks, and package Cargo check/test. The package still has no Nx target; record that structural exception. If moving SPR/store prevents compilation, record exact external blockers and retain source-static acceptance.
