# Audit: Non-TypeScript, Non-Rust Build and Test Targets (aud-langs-1020e)

Read-only audit by `aud-langs-1020e`. No builds, no `bun repo`, no git writes, no repo-wide grep. Evidence came from the root listing `📋️project.json` (single Nx-style project `semio`, 322 `repo:script` targets), member `📋️project.json` files (1,475 found by a name walk with cache and build directories pruned), manifest name-finds, and scoped reads of the bazel, caching, hermetic, cmake, print, and bootstrap modules.

Tags: **VERIFIED-READ** = read in this audit (file, line, or count). **INFERRED** = derived from the above, not executed.

Repo-level Bazel facts (VERIFIED-READ): `MODULE.bazel` declares only `module(name="semio")` with no `bazel_dep`, and no `BUILD`/`BUILD.bazel` exists in the repo. `.bazelversion` = 8.4.2, and the bootstrap pins `BAZEL_VERSION=manifest.version` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/📜️script.ts` line 16). BUILD rules are generated per run (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` ~lines 444-469: `repo_task`, `repo_test`, `repo_launch`, `repo_hermetic_task`, `repo_hermetic_modules`).

## 1. Summary

| Area | Member projects | Targets (root + member) | Bazel family | Hermetic under Bazel | Toolchain pin (mac / linux / win) | Declared-input caching |
|---|---|---|---|---|---|---|
| Go | 22 (`📦️packages/🐹️go`) | 27 (test 21, build 4, dev 1, test-entrypoint-source 1) | `go` (own class) | No | Go 1.26.9 pinned w/ sha256 for 6 platforms (bootstrap schema) / yes / Windows not verified | 25 of 27 cached with inputs |
| Python | 2 (styling, energy oracle) | 12 (oracle-* 6, test 2, build 1, deps 2, generate noop 1) | `python` (no class) | No | uv 0.11.6 pinned; interpreter system-only / yes / ps1 pins 3.14 | 3 cached (1 without inputs), 8 uncached |
| .NET (C#) | 2 (`🔷️dotnet`) | 4 (deps 2 uncached, build 2 cached) | `dotnet` (no class) | No | SDK via commit-pinned `dotnet-install.sh`, SDK version not pinned | build cached, deps uncached |
| C/C++ (CMake) | 0 member, 8 root `cpp*` targets | 8 | none (falls to `bun-script`) | No | cmake/ninja unpinned OS packages; MSVC via vswhere | 2 of 8 cached; **0 native sources** |
| Schema codegen (`.proto`, JSON) | none (root `schema-*`) | 14 root (12 cached, 2 uncached) | `bun-script` | No | bun 1.4.2, node 24.15.0 pinned | cached |
| TeX / print (tectonic) | 1 (`♻️mit-bestand/📋️bericht`) | 13 (build/check/test/akteursnetz 9 cached, watch 4 uncached) | `latex` | No | tectonic 0.16.9 pinned w/ sha256 | 9 of 13 |
| WIT / wasm components | root `components` + wasm-family targets (approx 27 targets, 23 projects) | see section 3.7 | `wasm` / cargo `component` | No | wasm-opt (binaryen) pinned; wasm-pack 0.15.0 literal; wasm-tools unpinned | mixed |
| Shell / PowerShell | none (bootstrap only) | 0 build targets | n/a | n/a | `🐚️.sh` (mac/linux), `🔵️.ps1` (win) | n/a |
| Lean | 0 (unwired) | 0 | not wired | n/a | `lean-toolchain` present | n/a |
| Swift | 0 | 0 | none | n/a | none found | n/a |

Family census (VERIFIED-READ method, INFERRED counts): applying the ordered rules in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/🎯️coverage.json` to all listings, with command- and script-based rules skipped, gives go 27 targets (21 projects), python 11 (2), dotnet 4 (2), latex 19 (4), wasm 27 (23), docker 7 (2), playwright 13 (7), tsc 29 (17), vite 2 (2), bun-script 881 (180), bun-test 704 (227), cargo 2096 (333). Rust and TypeScript are out of scope.

## 2. Bazel mechanics shared by all languages (VERIFIED-READ)

- Family classifier: ordered rules in `🎯️coverage.json`. Go is `root` `(?:^|/)🐹️go(?:/|$)`; Python is `🐍️python`; dotnet is `🔷️dotnet`; LaTeX is root `📓️print|📋️bericht` with targets `build|watch|fonts|deps-te|akteursnetz|generate-actor`; WASM is targets `wasm|check-wasm|component|jco`. There is **no rule for C++ or for `schema-*`**, so both fall through to `bun-script`.
- Bazel toolchain keying: only `🐹️go/🟦️.ts` (`bazelGoIdentityV1`: hash of the `VERSION` file and binary; binary resolved from `GOROOT` then `PATH`) and `🦀️cargo/🟦️.ts` (`BAZEL_CARGO_EXCLUSIVE_V1`, `parallelism: 1`) have identity classes. The bazel tree has **0 mentions** of python, dotnet, cmake, cpp, tectonic, or wasm-opt (scoped grep, excluding tests).
- Hermetic table `🏃️execution/🔒️hermetic/🔣️.json`: only 5 classes, all TypeScript (`ts-typecheck`, `ts-test`, `ts-generate`, `ts-build`, `ts-check`), limited to 3 projects (`@teaching/architecture-course`, `@semio-tech/mathematical-equation`, `@semio-tech/writer-js`); every class has `"evidence": "assumed"`; `exceptions` is `[]`.
- Uncached reasons are recorded per action (`uncached_reason`). Policy `refused = [undeclared, unbounded]`, `interactive = [continuous]` (`🎯️coverage.json` Policy).
- Session concurrency: the Bazel tree has slot and server planning (`bazelSlotPortV1`, `planBazelServersV1`, `collectBazelSessionsV1`, `BazelSlotKindV1` in `🧵️sessions/`) and a shared repository store under `.cache` (VERIFIED names; the concurrency semantics are INFERRED).
- `.bazelrc`: `--enable_bzlmod`, disk-cache GC 30d idle 10s, repo-contents GC 3d (VERIFIED-READ). No disk cache path is pinned, so the cache location is the Bazel default (INFERRED).

## 3. Per language

### 3.1 Go

- Manifests (VERIFIED-READ): 32 `go.mod` files under `🧰️framework/…/🔨️modules/…` (list held in the ticket scratch, not repeated). `go.work` (`go 1.25`) `use`s 31 of them.
- Targets: 22 member `📋️project.json` under `…/📦️packages/🐹️go/` (for example `…/🎫️tickets/📦️packages/🐹️go/📋️project.json`, `…/⌨️cli/📦️packages/🐹️go/📋️project.json`). 27 targets; test is `bun ./📜️script.ts test` with `cache: true` (21 of them).
- Cache: 25 of 27 cached with declared inputs; 2 uncached (VERIFIED-READ).
- Under `--tool=bazel`: classified `go`; `bazelGoClassV1` keys each task by its sources plus toolchain identity, so caching is per task and therefore per package (VERIFIED-READ). Not hermetic: the Go binary is the host one (`bazelGoBinaryV1` resolves from `GOROOT` or `PATH`).
- Concurrency: deps sync is `go mod download` with `GOWORK` set to the shared root `go.work` (`…/⚡️caching/…/📦️dependencies/🏗️native/🟦️.ts` line 43, VERIFIED). Go's module and build caches are safe for concurrent processes (INFERRED from Go design).
- Toolchain: `go1.26.9` pinned with sha256 for darwin amd64/arm64, linux amd64/arm64, windows amd64/arm64 (`…/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json`, VERIFIED). Bootstrap `🐚️.sh` installs it (header, VERIFIED). Windows path: `🔵️.ps1` exists; Go installation there was not verified.
- Gaps: `go.work` says `go 1.25` while the pin is 1.26.9 and the devcontainer installs 1.26 (VERIFIED). `…/🖥️server/🎛️coordinator/go.mod` is not in `go.work` (VERIFIED). No `toolchain` directive in `go.work`.

### 3.2 Python

- Manifests (VERIFIED-READ): root `pyproject.toml` (`requires-python >=3.14,<3.15`, `[tool.uv.workspace] members = []`, pytest testpaths over three trees); `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/pyproject.toml` (`>=3.12`); `✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python/pyproject.toml` (`>=3.12,<3.13`). Each has a `uv.lock`.
- Targets: 2 member projects, 12 targets. Styling: build (cached, inputs), test (cached, inputs), deps (uncached), generate (noop). Oracle: setup, status, run, native, epjson, emit, deps (uncached), test (cached, no inputs).
- Under `--tool=bazel`: family `python` by root, no Python class, so a generic `repo_task` with host `uv` and host interpreter. Not hermetic.
- Concurrency: native deps run `uv sync --locked --all-packages --all-groups` (`…/🏗️native/🟦️.ts` line 40, VERIFIED). That mutates the single root `.venv` shared by every agent (VERIFIED `.venv` exists at the repo root). Because root `members = []`, that call does not sync the styling or oracle environments. No per-environment lock was found (INFERRED: concurrent `uv sync` on `.venv` is a race).
- Toolchain: uv `0.11.6` pinned with sha256 in the bootstrap schema (VERIFIED). The styling builder requires `UV_PYTHON_DOWNLOADS=never`, `--no-managed-python`, and `uv python find` of the authored version, and rejects a uv identity that differs from the pin (`…/🎨️styling/🏗️builder/🐍️python/🟦️.ts`, VERIFIED). So the interpreter is system-only on every OS. Windows `🔵️.ps1` sets `PythonKind = "3.14"` (line 32, VERIFIED).
- Gaps: three conflicting `requires-python` ranges (VERIFIED). Interpreter not provisioned, so the "zero-touch" claim fails on a bare machine (INFERRED). 8 of 12 targets uncached (VERIFIED), so no incremental Python builds under Bazel.

### 3.3 .NET (C#)

- Manifests (VERIFIED-READ): `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🔷️dotnet/🔷️.csproj` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/🧪️Semio.Repo.Test.csproj`. Both `TargetFramework net8.0`, `Microsoft.NET.Sdk`. 16 `.cs` files in total.
- Targets: 2 member projects (`📋️project.json` in each dotnet dir), 4 targets. `deps` is uncached in both; `build` is cached with inputs in both.
- Native sync for `dotnet` only logs `[deps-dotnet] Build-tool project restores completed` (`…/🏗️native/🟦️.ts` line 44, VERIFIED). The real restore is in each project's own `deps` script, which I did not read (INFERRED).
- Under `--tool=bazel`: family `dotnet`, no class, generic `repo_task`, not hermetic.
- Concurrency: `obj/` and `dist/build/{prototype,release}` sit inside the source packages (VERIFIED on disk). Two agents building the same csproj write the same `obj/` (INFERRED race).
- Toolchain: SDK through `dotnet-install.sh` pinned by commit `5147e32…` (bootstrap schema, VERIFIED). No SDK version pin (no `global.json` found, VERIFIED absence), so the channel floats (INFERRED). Devcontainer has `dotnet 8.0` plus `9.0`, `10.0` (VERIFIED).

### 3.4 C/C++ (CMake)

- Manifests (VERIFIED-READ): root `CMakeLists.txt` (`LANGUAGES CXX`, C++23, `include(CTest)`, no `add_library` or `add_executable`) and `CMakePresets.json` (v6, cmake minimum 3.28).
- Sources: **0** `.c`, `.cc`, `.cpp`, `.cxx`, `.h`, `.hh`, `.hpp` files in the repo (VERIFIED, scoped find).
- Targets: 8 root (`cpp`, `cpp-build`, `cpp-configure`, `cpp-custody` cached, `cpp-setup`, `cpp-test`, `cpp-test-toolchain` cached, `deps-cpp`). No member project. Command scripts live at `…/🏗️build/🛠️cmake/📜️script.ts` (785 bytes, delegates to `🟦️.ts`).
- Under `--tool=bazel`: no C++ rule, so `bun-script`. `cpp-test` runs CTest over no tests. `deps-cpp` only logs (VERIFIED, `…/🏗️native/🟦️.ts` line 45).
- Toolchain: `🐚️.sh` installs `build-essential cmake ninja-build pkg-config` (apt), the `gcc gcc-c++ make cmake ninja-build` set (dnf), and similar for pacman, zypper, and brew (lines 272-308, VERIFIED). No version pin for cmake or ninja. Windows MSVC via `vswhere` (`…/🪟️toolchain/🟦️.ts` line 22, VERIFIED).

### 3.5 Schema codegen (`.proto` and JSON schemas)

- Inputs: 2,128 `🛰️.proto` files under `…/🧬️schema/` across plugin and framework trees (VERIFIED count). Example: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/…/🧬️schema/🛰️.proto`.
- Targets: 14 root `schema-*` (audit, check, compile, docs, entries, fixture-boundary, generate, oracle, parsers-apply, parsers-check, parsers-report, parsers-restate, test, verify). 12 cached; `parsers-apply` and `parsers-restate` uncached (VERIFIED-READ). Root `prepare` depends on `@semio-tech/framework-schema:generate` (VERIFIED).
- Codegen tool: no `protoc` or `buf` invocation found in the scoped schema module (`🧰️framework/🔨️modules/🧬️schema`), where `.proto` appears in only 2 TS/Rust files (VERIFIED). So proto consumption is in-house TypeScript or Rust (INFERRED; not traced further). Generated output is under `🧬️schema/🤖️generated` (VERIFIED, one entry).
- Bazel: `bun-script` family, generic `repo_task`, not hermetic. Toolchain: bun `1.4.2` (`package.json` packageManager, VERIFIED), node `24.15.0` pinned (bootstrap schema, VERIFIED).

### 3.6 TeX and print (tectonic)

- Inputs: 241 `.tex` files, mostly under `♻️mit-bestand/📋️bericht/` (VERIFIED count).
- Member project: `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json` (13 targets). Build, akteursnetz, test, generate-actor-network, preview-generated, check-generated are cached (9). `watch`, `watch-zwischenbericht`, `watch-forschungsbericht`, `watch-kompaktbericht` are uncached (4). Watch runs `…/📓️print/🔨️modules/🖨️tectonic-template-compilation/📜️script.ts watch` (VERIFIED).
- Engine: `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🔣️.json` pins tectonic `0.16.9` with release URL and sha256 per platform: 7 entries (darwin arm64 and x64, linux arm, arm64, ia32 and x64, win32 x64) (VERIFIED-READ).
- Under `--tool=bazel`: family `latex` by root `📓️print|📋️bericht`, generic `repo_task`, not hermetic. Concurrency: tectonic keeps a shared package cache under the user's cache dir (INFERRED).

### 3.7 WIT and WASM components

- Inputs: 29 `.wit` files (VERIFIED count), e.g. `…/🧬️schema/📜️world.wit`, `…/🧬️schema/📜️types.wit`.
- Targets: root `components` (`bun ./📜️script.ts components`, cache false), `deps-wasm`, `deps-wasm-opt`, `deps-trunk`; WASM-family targets approx 27 across 23 projects (INFERRED from census).
- Toolchain (`…/🏗️native/🟦️.ts` lines 47-56, VERIFIED): `wasm-pack 0.15.0` by literal, `wasm-bindgen-cli` version read from `Cargo.lock`, target `wasm32-wasip2` via rustup, `cargo-nextest 0.9.140` and `cargo-llvm-cov 0.8.7` via `cargo install --locked`. `wasm-opt` is binaryen with a pinned `release` URL and sha256 for 6 platforms (`…/🔗️bootstrap/🛠️tools/🕸️wasm/🧬️schema/🔣️.json`, VERIFIED). `wasm-tools` and `wit-bindgen` are not in the bootstrap pin list (INFERRED unpinned).
- Under `--tool=bazel`: the Cargo class maps `bun … component --manifest` to a cargo `build` action with `BAZEL_CARGO_EXCLUSIVE_V1` (VERIFIED). `jco` is consumed on the TypeScript side (INFERRED).

### 3.8 Shell, PowerShell, Lean, Swift

- Shell: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh` (bootstrap for mac and linux, pinned and sha256-verified installs, VERIFIED); Windows counterpart `…/🥾️bootstrap/🔵️.ps1` (VERIFIED existence, winget install path VERIFIED). One fixture `…/🗣️languages/🧪️tests/🧫️fixtures/🐚️sample.sh`. Not build targets.
- Lean: `👴️leutwiler/💤️realparts-of-powers-z-n/🧘️lean/` with `lakefile.toml` and `RealPartsOfPowers.lean` (VERIFIED). No `📋️project.json`, no root target mentions it, and `.nxignore` only excludes `**/.lake/**`. Unwired.
- Swift: no `.swift` file and no `Package.swift` in the repo (VERIFIED).

## 4. Ranked gaps for many agents in one folder with a shared incremental cache

1. **Shared mutable Python environment.** `uv sync --locked --all-packages --all-groups` mutates one root `.venv` (VERIFIED), with no per-environment lock (INFERRED race), and does not cover the styling or oracle environments (VERIFIED `members = []`).
2. **In-tree build outputs.** dotnet `obj/` and `dist/build/*` and python `dist/build/*` live inside source packages (VERIFIED). Concurrent builds of one package collide (INFERRED).
3. **No hermetic class outside three TypeScript projects.** The table's classes are all `assumed` with no exceptions (VERIFIED). Bazel keys toolchain identity only for Go and Cargo (VERIFIED). Python, dotnet, tectonic, cmake, and wasm-opt identities are absent from the Bazel tree, so a cached action can go stale after a tool upgrade (INFERRED).
4. **Cache coverage holes.** 8 of 12 Python targets uncached; 1 cached Python test declares no inputs and is therefore unbounded; dotnet `deps` and `cpp-build`, `cpp-configure`, `cpp-setup`, `cpp-test` uncached; 4 tectonic `watch` targets uncached (VERIFIED).
5. **Version drift.** Python `requires-python` in three ranges (VERIFIED). Go `go.work` at 1.25 versus pin 1.26.9 (VERIFIED). `🖥️server/🎛️coordinator/go.mod` outside `go.work` (VERIFIED).
6. **Unpinned native toolchains.** cmake and ninja from OS packages (VERIFIED). .NET SDK channel floats (INFERRED). Python interpreter system-only (VERIFIED). These break zero-touch on a bare macOS, Linux, or Windows machine (INFERRED).
7. **C++ is a phantom.** Eight root targets over zero sources, with no family rule (VERIFIED). `cpp-test` runs CTest over nothing.
8. **dotnet restore is opaque.** Native `dotnet` sync only logs (VERIFIED); the restore lives in per-project scripts not read here (INFERRED).
9. **Proto codegen path not located.** 2,128 `.proto` inputs, no `protoc` or `buf` invocation in scope (VERIFIED absence; INFERRED in-house).
10. **WIT tooling unpinned.** `wasm-tools` and `wit-bindgen` absent from the bootstrap pins (INFERRED).

## 5. Not verified and limits

- Windows: `🔵️.ps1` was not read end to end, so Go, dotnet, and tectonic installation on Windows is unverified.
- Member `deps` and build scripts (`…/📜️script.ts`) for Python, dotnet, and WASM were not read beyond the cited lines.
- Family census applies rules by target and root only; command- and script-based rules were skipped, so counts are approximate.
- No build, test, or `bun repo` command was run, per the audit constraints.
- Scratch lists were written to the session scratchpad, not this folder. One accidental list written into this ticket folder was deleted.
