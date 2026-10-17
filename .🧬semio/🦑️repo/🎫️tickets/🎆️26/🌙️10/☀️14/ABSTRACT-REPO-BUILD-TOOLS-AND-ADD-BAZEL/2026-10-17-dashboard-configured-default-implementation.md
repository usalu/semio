# Dashboard Configured Neutral Default Implementation

The dashboard now pins the effective backend from the actual workspace neutral policy before reuse and session publication. The schema owns the policy location, version, allowed tools, byte bound, Windows file-opening constants, and exact root/build script launchers. There is no Rust/dashboard default backend literal. Both actual authored launcher routes use the `repo` namespace, as confirmed by root routing and package.json.

The existing build graph Rust authority reads a bounded regular policy file from the actual absolute workspace root. It refuses missing, malformed, invalid-version/tool, changed descriptor/name, symlink and supported reparse paths. Descriptor/name stamps remain strict. On Unix they include device/inode/mode/link count/size/mtime/ctime. On Windows the stable standard-library path uses FILE_SHARE_READ without WRITE/DELETE sharing and OPEN_REPARSE_POINT, checks ordinary descriptors and reparse attributes, and validates known policy ancestry down to the supplied root. Windows stamp fields are attributes/creation/write/size; this is not a volume/file-ID proof. No Native/cache/storage publication code or guard was edited.

The central batch binder reads the default lazily, at most once per launch batch. Explicit outer argv, declared command environment, then inherited hello environment retain precedence; invalid explicit tools refuse. An already pinned child survives a subsequent workspace policy change. An unresolved build process cannot be reused. CLI and daemon independently invoke the same binder before querying/publishing sessions. Only `bun repo` or exact schema-owned script paths with `repo` are build contexts. Script arguments resolve lexically against command cwd and the actual root, so unrelated same-basename scripts neither read policy nor receive injected tool environment. Literal `--tool` after `--` remains child data.

## Evidence

- Schema-first configured-default semantic RED on earlier production: expected configured Bazel and no cross-tool reuse; observed hardcoded Nx and reuse. One test failed after3 assertions in1.90s. This is separate from test-harness failures.
- The launcher-ownership fixture was extended before its production repair. The first filtered Bun invocation crashed133 before any test result, with a Bun1.3.14 segmentation-fault report (19.572s). This is an unavailable harness run, not semantic RED. Unfiltered explicit `./` test paths completed normally.
- Intermediate Source failures were corrected openly: draft2020 schema needed Ajv2020; the test schema wrapper must reference BuildPolicy without inheriting graph-root required fields; Rust extracted visibility and JSON-port decoding were harness repairs. Initial strict command also lacked allowImportingTsExtensions and found two optional fixture cwd typings; final strict repaired both.
- Final combined runtime: **11pass/0fail/858assertions/9.29s**, terminal0, across reuse, existing launch projection, and command-audit Source tests. Runtime emitted `[DEBUG] Actual Rust identity and pinned environment agree with SQLite for 27 explicit/inherited/default/argv-boundary cases` and `[DEBUG] Actual Rust policy reader/batch binder exercised 21 actual moved-root files; schema/Ajv/SQLite agree; JSON decoder is an explicit Source port; no Native effects`.
- Reuse execution compiles exact actual Rust selection/binding, CLI identity and supervisor reuse bodies using rustc; registry/group execution retains six portable required-service variants. Identity default-provider/model records are explicit Source ports. The actual workspace-policy IO gate executes exact Rust policy/read/bind bodies against21 actual generated files under moved Unicode roots. Its JSON decoder and embedded-schema include are explicit fixture-backed Source ports, not actual serde_json qualification.
- Independent oracles: schema fixtures validated with Ajv; policy schema with Ajv2020; SQLite independently derives option/env/default selection and Windows attribute masks; actual TypeScript public invocation AST independently checks tool selection; Node path resolution independently decides schema-owned lexical launcher ownership. Seven Windows attribute/open-flag models pass. No actual Windows OS, full Cargo crate, live dashboard daemon, or physical publication acceptance is claimed. Actual serde_json crate-native unit coverage was authored but not run under the parent Source-only restriction.
- Focused strict TypeScript: terminal0 with ES2022 target, ES2024/DOM libs, Bundler resolution, strict, allowImportingTsExtensions, resolveJsonModule, esModuleInterop, skipLibCheck, bun types, noEmit. Scoped diff whitespace check terminal0 with Git fsmonitor disabled after its daemon reported termination on the earlier default invocation.

Commands were the existing direct Bun test entry for reuse/launch plus ticket command-audit tests, using caller-owned SEMIO_TEST_ARTIFACT_DIR under ticket Generated/dashboard-policy. No executable target or Source file was added; existing registered test routes cover these cases. Own generated fixtures/binaries were removed after execution. Other agents' Generated/physical retained provenance was untouched.

## Windows Design References

[Rust MetadataExt](https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html) still marks volume/file-index fields experimental; [stable OpenOptionsExt](https://doc.rust-lang.org/std/os/windows/fs/trait.OpenOptionsExt.html) provides share_mode/custom_flags. The implementation uses that stable standard-library surface rather than adding FFI or nightly requirements. Source models support the logical configured-default repair; actual Windows qualification remains outstanding.

## Captured Source Bytes

Paths below are relative to the workspace. Hashes identify this implementation checkpoint and do not claim the repository is otherwise unchanged.

| Path | Bytes | SHA256 |
| --- | ---: | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🔣️.json` | 15471 | `404275c04bf60f0b6a15d3a1e0e21e6ecec9c33844530c0b02b5b67ef160a804` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🦀️.rs` | 6930 | `8d1bbecaabf51994fc9384716230549c4443205b7c4136b1d2fd7d59cdc8d104` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs` | 31566 | `29d2fa0da409f3fe5efd10b21825070ad14bf90e41559aaeff64a8d9e6d3d460` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📎️connection/🦀️.rs` | 14139 | `c6c269c9ad804b566b591a5215d9ed6a2dedc3b7e603f4651b91fdd1794a61c3` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/🦀️.rs` | 29050 | `dc760470b92acd29747c96c2b1f79231c2a8ebdcf57daf5e33a59d2626b7b533` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🧠️supervisor/🦀️.rs` | 44091 | `ede97a595f1112d720c24523bb8df9d0f58be929fd35350ff1e5240eeca3ee1c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json` | 22456 | `a2469ef5a3cf10559bad06c8865fa5a797de9ec92642233317dc37cfdccbffc3` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/🧪️tests/🔬️unit/🦀️.rs` | 17152 | `847060b7691d9b7fe35e380ef3e3a5cf614f3f4c1d150199854b3641f3bac480` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/♻️reuse/🟦️.ts` | 27396 | `c4922a710c10a348fb676a930a92f6f4dbb409c19fc6f158310d7288fbb8b1fb` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧫️fixtures/♻️reuse/🔣️.json` | 18202 | `9c1fe469e0fd8d22f3455e562e67bc1731842e36a3fcf4d060e36896eff1c9ba` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧫️fixtures/♻️reuse/📋️schema.json` | 4396 | `41645febee90280011277190186039aa9ca8b1061807e97ea7817459df04bee5` |
