# Complete Monorepo Readiness Matrix

Static inventory on 2026-10-28 found 1,489 project manifests, 3,978 declared targets and 2,151 distinct target names. These are authored manifest counts, not a successfully resolved runtime build graph. Counts include organizational projects and the workspace root; they do not imply 1,489 independently buildable applications.

## Declared Target Coverage

| Target | Project declarations |
|---|---:|
| build | 251 |
| test | 516 |
| check | 232 |
| lint | 11 |
| verify | 2 |
| test-e2e | 3 |
| test-native | 40 |
| wasm | 11 |
| package | 26 |

The inventory contains many named native, oracle, snapshot SQLite, source, ownership and runtime targets beyond generic test/build. Passing only generic targets cannot establish all named requirements without checking aggregate target closures.

## Domain Inventory

| Top-level domain | Manifests |
|---|---:|
| ♻️mit-bestand | 3 |
| ✏️s | 326 |
| 🌎️hub | 115 |
| 🎓️teaching | 2 |
| 🏢️semio-tech | 1 |
| 👴️leutwiler | 1 |
| 📋️project.json | 1 |
| 🧰️framework | 1040 |

## Language and Runtime Gates

| Family | Manifest mentions | Required closure |
|---|---:|---|
| Rust | 1242 | pinned rustup/toolchain, rust-lld, Cargo locks/offline registry custody, native compiler partitions, wasm32 target, generated packs, native tests |
| JavaScript/TypeScript | 267 | launcher Node/Bun registry, frozen workspace dependencies, generated bootstrap assets, Nx entrypoints, type checks, React/Electron/browser startup |
| Go | 30 | pinned Go, go.work modules, canonical runner compilation and publication, retained physical lease mount |
| Python | 8 | uv pin, Python interpreter, lock consistency, managed environment and fixture/oracle execution |
| .NET | 6 | global.json SDK pin, ICU on Linux, NuGet managed cache and native test execution |
| C/C++ | 1 | compiler, pkg-config, CMake/Ninja, Cairo/Pango/UUID/ICU and root cpp setup/configure/build/test/toolchain targets |

Mentions are overlapping lexical classifications of project path/target declarations; they are not precise package counts. Browser dependencies, SQLite, graphics/headless display, fonts, numerical oracle tools, Docker-backed services and project-specific generated assets add runtime prerequisites beyond language installation.

## Comprehensive Execution Gates

Run through the native repo frontend, with an explicit supported language. These targets exist in the root manifest:

1. repo preflight, then repo run workspace:setup and dependency ownership checks.
2. repo run workspace:test-workspace-build, repo run workspace:verify-zero-touch, repo run workspace:verify-build-owners-sync, repo run workspace:verify-dependencies-locks and repo run workspace:generate-config-check.
3. repo run workspace:schema-verify, repo run workspace:schema-test and repo run workspace:verify-typescript.
4. repo run workspace:build, repo run workspace:test and repo run workspace:lint. Inspect resolved closure to prove all appropriate project build/test/check targets participate.
5. repo run workspace:cpp-test-toolchain and repo run workspace:cpp-test, plus native/wasm/browser product acceptance targets that aggregates do not select.
6. repo run workspace:test-contract, workspace:test-oracle, workspace:test-subject and workspace:test-parity; verify named source/native/oracle ownership target closure instead of assuming generic tests include them.
7. Exercise all three declared test-e2e targets with their prepare/serve dependencies, browser startup and cancellation; exercise Docker-backed hub services on a Docker-capable host.
8. Repeat cold and warm devcontainer creation/start/attach and the corresponding native host gates on Windows/macOS. Source fixtures alone are insufficient platform execution evidence.

These are recommended commands, not completed runs. Do not run publish/release/deploy merely to validate build readiness. Resolve all missing system prerequisites before interpreting native build failures.

## Independent Change Verification

Independently ran current Go lease law plus native bootstrap toolchain laws through repo tools exec bun using explicit test paths and ticket artifact directory: 6 tests passed, 0 failed, 104 assertions. Included browser prerequisites routing every Linux package manager through the owned privileged installer, stable version third-party semver parity, Linux compiler/ICU inputs, Unix/Windows rustup profile safety, and Go compiler/publication physical lease partition settlement. These are focused behavior/source contract tests; they do not certify real package installation across distro images or a full compiled monorepo.

Machine-readable static inventory is temporary generated output and will be removed during final ticket cleanup. No source edits were made by this audit.
