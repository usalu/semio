# Run File Media Cache One-Consumer Inline Acceptance

## Scope

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs`
- This acceptance record

## Preflight

- HEAD matched `0727b80aa6a802cac1760f90fb7a148f74035413`.
- The component matched `6cadb4e0bf0300a0fcd7ce3e0e807eef8ed27daa128a5cdf79a6d73d07651ac7` and its ordinary diff contained only the accepted test-cache relocation and associated prose.
- The binary matched `f5bd7e3561a3a1fe8bb30956b966bcf5e7d23cadaa29717edda620c2cbd25b96` with no staged or ordinary diff.

## Change

- Removed public `FileMediaCache`, its `MediaCache` implementation, and `SpaceBundle::media_cache()` from the run component.
- Added private binary-local `FileMediaCache`, using the same `MediaCache` interface and the exact former JSON serialization, cache entry path, directory creation, and ignored-write-error behavior.
- The binary now constructs it from `bundle.media_cache_dir()`.
- Retained `MediaCache`, `SpaceRunner`, `SpaceBundle::media_cache_dir()`, and the accepted private `TestMediaCache` unchanged in purpose.

## Verification

- `FileMediaCache` has six Rust references, all in `📦️bin.rs`: region, private type, implementations, and the one construction.
- `SpaceBundle::media_cache` definitions and calls have zero Rust references. `media_cache_dir()` remains defined in the component and is consumed by the binary.
- Scoped ordinary and cached `git diff --check` produced no output. The cached scoped diff was empty; the ordinary scoped diff contains the accepted test cache relocation plus this one-consumer production cache move.
- The package has neither local `project.json` nor `📜️script.ts`, so no truthful package-local Nx target exists. The required structural exception was used:

  ```text
  cargo check --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml
  cargo test --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml --lib
  ```

- Both commands are blocked before the run crate compiles by active external SPR/store contract drift: missing `CommandReceipt.messages`/`worst`; absent `reconcile_with_last`; missing `HistoryLog.conflicts` and `HistoryOpMeta.messages`; a `SpaceConflict`/`Conflict` slice mismatch; and unhandled `SetMergePolicy`/`ResolveConflict` `ArtifactCommand` variants. No blocker was changed in this lease.

## Final Fingerprints

- Component SHA-256: `2ab7ef2edcd150706e9165238e039d6274998087907f6848fdb7a3ce2324f57f`
- Binary SHA-256: `cd68b4d384ea8bcc675d38d4bab16c71fbaed860e8190d3fdf968fc5249444ec`
