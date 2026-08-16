# Hash Retained Contract Hardening Acceptance

## Baseline

- HEAD: `07873f842a5a99ac2f69c1648c21f36ebf260bdb`.
- `🧰️framework/🔨️modules/#⃣hash/🦀️component.rs` was clean at SHA-256 `b2a4e93df9604705736a503a24402e3fdd6b978c270024ff0529f857e1f9179f`.
- This lease is limited to adding tests within the existing Rust test region; runtime behavior and public API remain unchanged.

## Implementation

- Added four contract tests in the existing `#[cfg(test)]` Rust region of `🧰️framework/🔨️modules/#⃣hash/🦀️component.rs`.
- The tests cover numeric normalization (`-0`, integral, and fractional values), part-boundary delimiter separation, deterministic Merkle child ordering, and `NaN`/positive-infinity/negative-infinity normalization.
- No runtime definition, public API, Cargo metadata, consumer, glue, or registrar changed.

## Validation

- `bun nx run @semio-tech/framework-rs:test-quick --skip-nx-cache` exited `1` overall. Its Rust nextest phase compiled `semio-framework` and passed `137` tests (`137 passed`, `0 skipped`, `0 failed`).
- The subsequent TypeScript Vitest phase failed before finding tests because the unrelated `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:495` has a parser error: `Expected ")" but found ":"`. That path was read-only and untouched by this lease.
- `git diff --check -- 🧰️framework/🔨️modules/#⃣hash/🦀️component.rs` exited `0`.
- The final source diff is exactly `27` added test lines and `0` deletions; no post-edit drift was observed.

## Final State

- `🧰️framework/🔨️modules/#⃣hash/🦀️component.rs` SHA-256: `ef6c104c523aab38a3d036fc32c43522f784b8bad00192ad699a7af6cf498e3a`.
- No Git-mutating command was used.
