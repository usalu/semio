# P9n Final Ts-Rs Retirement

## Outcome

`ts-rs` has been retired from every non-Compose Rust source and manifest. Framework, async, actor, OS-shell, and UI-contract projections now use their owned versioned schema metadata and deterministic renderers. The dependency ratchet therefore no longer classifies `ts-rs` as a current direct dependency.

## Migrated Owners

- Root framework contract: 172 declarations.
- UI contract: 79 declarations.
- Async runtime contract: 8 declarations.
- Actor runtime contract: owned actor/job/scheduler declaration table.
- OS-shell contract: owned shell declaration table.

Each owner validates metadata before rendering and compares generated output with its committed mirror through existing Nx/`📜️script.ts` targets.

## Verified Gates

- Root framework release and both wasm type-generation checks: passed.
- Framework, async, actor, OS-shell, and UI-contract focused type-generation/test gates: passed.
- Whole-repository non-Compose source/manifest census for `ts-rs`, `ts_rs`, `#[ts(...)]`, and `TS` derive sites: zero.
- Dependency ratchet lists `ts-rs` among removed dependencies and reports no newly introduced third-party package.

## Deliberate Scope

Historical dependency-inventory records remain immutable audit evidence and may still contain the string `ts-rs`; they are not source, manifests, generated bindings, or active dependency declarations.
