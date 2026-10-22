# result-cargo-home (wave 1022a, executor cargo-home)

STATUS: DONE, GREEN (live edits, verified by the real target run)

## Change
- `🏗️compiler/📍️layout/🟦️.ts` new `repositoryCompilerHostV1(repositoryRoot, workspace, platform)`: a workspace inside the repository checkout is hosted by the repository root (same Rust image, CARGO_HOME registry, toolchain, lease directory); a workspace outside hosts itself.
- `🏗️compiler/🟦️.ts` `withRepositoryCompilerStoreV1`: host derived from `readNativeCallerAuthorityV1(original.capabilities).repositoryRoot`; tooling store, rust toolchain, rust image, strict storage and headroom recovery use the host; `owner.workspace` stays the requested workspace.
- Test: `📍️layout/🧪️tests/` fixture rows `hosts` + `registry` (+ schema), Python posixpath/ntpath oracle, root vs nested -> same CARGO_HOME path.

## Run
- generate #1 pid 48665, log `C/🗑️generated/cargo-home/generate1.log`
- generate #1 (48665) finished rc 1 but PAST the cargo-metadata stage: no `no matching package named base64` any more (cargo metadata --locked --offline succeeded against the shared registry). New, unrelated fault: `cargo test --features typegen exports_typescript_bindings --unit-graph` -> `the package 'semio-framework' does not contain this feature: typegen` (the Oct-20 rust carve removed the feature; the test is `#[cfg(test)]`). Fixed live: `🧰️framework/📦️packages/🦀️rust/📜️script.ts` dropped `"--features","typegen"` at the 2 sites (generate/check + preview).
- generate #2 pid 53970, log `C/🗑️generated/cargo-home/generate2.log`

## GREEN EVIDENCE (for course-rest)
`SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool=bazel run @semio-tech/framework-rs:generate --configuration=prototype` (script `C/cargo-home-tools/generate.sh`, detached pid 53970, log `C/🗑️generated/cargo-home/generate2.log`, finished, nothing of mine still running):
- cargo stage passed: no `no matching package named base64`; cargo metadata --locked --offline resolved from the SHARED registry (`.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/cargo/home`, holds base64-0.22.1); the framework-local rust image `🧰️framework/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/cargo/home` stays empty (created 16:01 by an earlier pre-fix run, now unused; not deleted, retire it when convenient).
- `test typegen_tests::exports_typescript_bindings ... ok`, `framework typescript mirror refreshed -> .../🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` (file unchanged vs index), `INFO: Build completed successfully, 2 total actions`, `[script] repo complete`.
- Unit/language-agnostic test: `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🧪️tests/🟦️.ts` -> 2 pass, 60 expects (fixture rows `hosts` posix+win32 checked against a Python posixpath/ntpath oracle; root, nested and deeply nested workspace -> identical `cargo/home` path `<root>/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/cargo/home`; producer source pinned to `repositoryCompilerHostV1(`).

## Files changed
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts` (host passed to tooling store, rust toolchain, owned store; `owner.workspace` stays the requested workspace)
- `.../🏗️compiler/📍️layout/🟦️.ts`, `.../📍️layout/🧪️tests/🟦️.ts`, `.../📍️layout/🧪️tests/🔣️.json`, `.../📍️layout/🧬️schema/🔣️.json`
- `🧰️framework/📦️packages/🦀️rust/📜️script.ts` (dropped removed `typegen` feature flag at 2 sites)

## Open
- Not run: other compiler-store suites (`🏗️compiler/🧪️tests/🕸️units`), tsc over the module (the live run compiled it). A standalone checkout of `🧰️framework` alone (no enclosing repository root) hosts itself by design.
- No provisioning step was needed: the shared registry already satisfies the framework lock offline. If a lock ever needs a crate the shared registry lacks, the repo's own `bun repo cargo ... -- fetch --locked` step is the provisioning path (online once).
