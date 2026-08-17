# Run Test-Only Memory Media Cache Acceptance

## Scope

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- This acceptance record

## Preflight

- HEAD matched `0727b80aa6a802cac1760f90fb7a148f74035413`.
- The unmodified run component matched SHA-256 `4517e65d98ee41c73ca7b2375c2330effa52edc505506cccc4e785189913b18e` and had no staged or ordinary diff.
- The shared repository contained unrelated concurrent changes; none were touched.

## Change

- Deleted public top-level `InMemoryMediaCache` and its `MediaCache` implementation.
- Added private, map-backed `TestMediaCache` within `#[cfg(test)] mod tests`.
- Updated the five test cache constructions to `TestMediaCache::default()`.
- Retained `MediaCache`, `FileMediaCache`, `SpaceRunner`, media fingerprints, persistent paths, and binary behavior.

## Verification

- Coordinator-supplied final source SHA-256: `bee805a35f56452a561b6dcfb0e89c3868b8cbbc80e2b920d8106f290fddfffa`. A fresh no-write verification observed `6cadb4e0bf0300a0fcd7ce3e0e807eef8ed27daa128a5cdf79a6d73d07651ac7`; this indicates a concurrent source-state divergence after the supplied checksum. No source edit was made during this update.
- The run package has neither a local `project.json` nor `📜️script.ts`, so no truthful package-local Nx target exists. The required structural exception was used:

  ```text
  cargo test --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml --lib
  ```

- Cargo exited `101` before compiling the run component because active SPR/store channel changes are incomplete outside this lease:
  - `HistoryOpMeta` is initialized without its new `messages` field in store `component.rs:2385`.
  - `HistoryLog` is initialized without its new `conflicts` field in store `component.rs:2523` and SPR testkit `component.rs:216`.
  - `SpaceMember::validate_wire` is invoked but absent in store `component.rs:6436` and `:6451`.
  - `GroupReceipt` is initialized without its new `messages` field in store `component.rs:6546`.
- `rg -n 'InMemoryMediaCache' --glob '*.rs' .` returned zero results after the change.
- `TestMediaCache` has one private definition and five constructions, all inside this component's `#[cfg(test)]` module.
- Scoped ordinary and cached `git diff --check` produced no output. The cached scoped diff was empty; the ordinary scoped diff contains only the intended cache relocation.
