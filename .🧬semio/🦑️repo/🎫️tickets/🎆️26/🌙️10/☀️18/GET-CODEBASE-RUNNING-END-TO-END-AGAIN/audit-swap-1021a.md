# audit-swap-1021a: how swappable are the external dependencies behind the internal interfaces?

Auditor `aud-swap-1021a`, 2026-10-21. Read-only. No builds, no `bun repo`, no git commands, no sub-agents. Reads were limited to the build library and the process framework. Searches used `/usr/bin/grep` because the `grep` alias (ugrep) errors on some emoji-heavy patterns.

## Abbreviations

- `L` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`
- `B` = `L/🏗️build` (all `file:line` cites without a prefix are under `B`)
- `P` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process`
- `S` = `P/📦️artifacts/🏗️native-build/💾️storage`
- `VERIFIED-READ` = the cited line was read in this session (directly or through a `grep -n` of the live file). `INFERRED` = derived from file names, structure, or an unread branch. Out-of-scope trees (`L/⚡️caching/...`, the Nx adapter, the dependency installer) were NOT read; they appear only as import paths quoted from in-scope files.

## 1. Verdict per dependency class

| class | neutral interface | neutral schema | shared contract tests | verdict |
|---|---|---|---|---|
| (a) build backends: nx, bazel, in-house | partial: cache, cache-probe, remote-cache and storage have interfaces; run, plan, affected, exec, watch and test have none | closed enum `BuildTool` `nx\|bazel` (`B/🧬️schema/🔣️.json:544-561`); operation catalog with per-tool `implementations` (`B/⚙️operations/🔣️catalog.json`, 19 rows) | pairwise only (parity harness, two adapters named in tests) | swap = edit the dispatch and about 11 closed unions. Not swappable without code changes in neutral files. |
| (b) JS runtime: bun | none | none | none | Bun APIs are called directly on about 40 production lines across the build library and the process framework, several of them neutral. Not swappable. Child processes use `process.execPath`, which is the one good seam. |
| (c) package manager: bun install | partial: `BuildEnvironmentInstallerV1` is an injected parameter | lock file name and install key are bun-specific | none | installer slot exists, but lock name, parser location and cache key are bun-specific. Medium-hard. |
| (d) toolchains: cargo, go, uv, dotnet, tsc, vite, playwright, tectonic, wasm | partial: data table in policy and script-provider ports | toolchain table in `B/🔣️policy.json` (`toolchains`) | `P/🧪️testing/🧰️toolchain/🧪️tests` for the test-tool resolver only | policy table and provider ports are good. About 10 framework sites spawn tool names as literals. Medium. |
| (e) storage: hdiutil DMG vs directory partitions | contract type exists, named `NativeImageStorageV1` | schema `S/🌐️platform/🧬️schema/🔣️.json` | per-backend tests (file names only INFERRED) | the backend is chosen by `process.platform` in at least 5 ternaries, the neutral contract imports the directory backend, and hdiutil lives in a shared cleanup module. Not swappable without edits in 5+ files. |

Bottom line: the cleanest seam today is the remote cache (`render` contract, policy env names, tool injected as a closure). Everything else is either a hard-coded if/else, a Bun API call, or a literal executable.

## 2. Positive seams that exist (VERIFIED-READ)

- Cache operations: `B/⚙️operations/🟦️.ts:24` `BuildCacheToolV1 {tool, status, clear}` and `:49` `runBuildCacheOperationV1(tool, args, context, resolveImplementation)` resolves the adapter by injection, not import.
- Cache probe: `B/⚙️operations/🟦️.ts:40` `CacheProbeToolV1` renders files and invocations for a tool-neutral spec.
- Remote cache: `B/🌐️remote-cache/⌨️command/🟦️.ts:6-10` `RemoteCacheToolV1 {tool, protocol, probePath, render(contract, env)}`. Neutral env names are in `B/🔣️policy.json:9-19` (`SEMIO_REMOTE_CACHE_*`). This is the best example of a render-only adapter.
- Tool propagation: `B/📜️script.ts:109` sets `SEMIO_BUILD_TOOL: capabilities.tool` for children. Nested builds inherit the tool.
- Script providers: `P/🔌️providers/🟦️.ts:21-29` `ScriptProviderTokenV1<Provider>` (a name plus the methods a provider must expose) with a schema-fixed env var that designates provider modules. The wasm build uses it (`P/📦️artifacts/🕸️wasm-build/🔌️provider/🟦️.ts` imports `defineScriptProviderV1` and `loadScriptProviderV1`). `B/🟦️typescript/🔌️provider/🟦️.ts` follows the same provider-module shape for the TypeScript check.
- Dependency installer injection: `B/🚀️bootstrap/🟦️.ts:18` `BuildEnvironmentInstallerV1` is a default parameter of `ensureBuildEnvironmentV1` (`:47`).
- Toolchain table: `B/📥️inputs/🟨️.mjs:207` `POLICY.toolchains[tool]` reads `files`, `environment` and `commands` per tool from `B/🔣️policy.json`.
- Test-tool resolver: `P/🧪️testing/🧰️toolchain/🟦️.ts` takes a `TestToolchainPortV1` (`which`, `run`, `platform`) so the resolver is testable without the host. Candidates and probes are declared in `P/🧪️testing/🧰️toolchain/🔣️.json`.

## 3. Leaks: external types, paths, flags and env in neutral code (ranked by spread)

### 3.1 Tool-name closed unions (about 11 sites)

Each of these hard-codes `"nx"|"bazel"` or `["nx","bazel"]`, so a third name needs an edit at every site.

- `B/⚙️operations/🟦️.ts:9` `BuildToolNameV1`
- `B/📥️caller/🟦️.ts:21,24,34` `tool:"nx"|"bazel"` in the caller capability types
- `B/📥️caller/📤️orchestration/🟨️.mjs:29` `["nx","bazel"].includes(tool)`
- `B/🧭️invocation/🟦️.ts:162` `if(tool!=="nx"&&tool!=="bazel") throw`
- `B/🔧️tools/💾️storage/🟦️.ts:6` `prepareBuildToolStorageV1(..., tool:"nx"|"bazel", ...)` with an `else throw` (`:6`, end of body)
- `B/🌐️remote-cache/⌨️command/🟦️.ts:7` `readonly tool:"nx"|"bazel"`; `:8` `protocol:"bazel-http"|"nx-self-hosted"`
- `B/📏️incremental/🟦️.ts:6` `IncrementalToolV1`; `B/🪞️parity/🟦️.ts:6` `ParityToolV1`; `B/🧾️ci-report/🟦️.ts:5` `CiToolV1`
- Schema enums: `B/📏️incremental/🧬️schema/🔣️.json:38-39`, `B/🪞️parity/🧬️schema/🔣️.json:33-34`, `B/🧾️ci-report/🧬️schema/🔣️.json:5`, `B/🧬️schema/🔣️.json:547-548`
- Catalog `B/⚙️operations/🔣️catalog.json:22` `"valueName":"nx|bazel"` and `:209` `"nx,bazel"`, plus 19 `implementations: {nx, bazel}` rows (e.g. `:366`, `:392`, `:424`)

### 3.2 Tool-name branches in neutral logic (VERIFIED-READ)

- `B/📜️script.ts:157` remote-cache tool ternary; `:188` `resolveCacheTool`; `:215` `if(selected.tool==="nx")` for the whole Nx path; `:182` bazel reset; `:231-252` bazel adapter imported directly (`./🔧️tools/🏗️bazel/📜️script.ts`) and called without an interface.
- `B/📏️incremental/🟦️.ts:85,94` `tool==="bazel"?dependsOn:consumes`: the measurement harness bakes in the difference between Bazel's dependency model and Nx's consumer model. `:157-158` pairs `nx` rows with `bazel` rows.
- `B/📏️incremental/🏃️runner/🟦️.ts:194` `if(tool==="bazel")`; `B/📏️incremental/🤝️concurrent/🏃️runner/🟦️.ts:205,217` the same.
- `B/🪞️parity/🏃️collect/🟦️.ts:146` `if(tool==="nx")`.
- `B/🧾️ci-report/🟦️.ts:152` `if(input.tool==="nx")` with an Nx log marker.

### 3.3 Neutral modules importing adapter modules (VERIFIED-READ)

- `B/📜️script.ts:15` imports the Bazel test schema `./🔧️tools/🏗️bazel/🧪️tests/🧬️schema/📋️dispatch.json`.
- `B/📜️script.ts:157,169,177,183,188,221` import `L/⚡️caching/🚀️bootstrap/🛠️tools/...` (Nx adapter, out of scope; import paths only).
- `B/📥️caller/🟦️.ts:4` imports the Nx plugin schema `L/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json`. `:82-87` encodes the Nx child protocol ("Original Repo-to-Nx caller required"). The neutral caller therefore knows the Nx child wire format.
- `B/📥️inputs/🧮️digest/🟨️.mjs:5` imports `L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/📥️inputs/📋️data/🟦️.ts`.
- `B/🔧️tools/💾️storage/🟦️.ts:7` dynamically imports `L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts`.
- `B/📏️incremental/⌨️command/🟦️.ts` imports both the Nx operations module and `./🔧️tools/🏗️bazel/📜️script.ts`.
- Eleven neutral files in total import a tool adapter or an Nx path (grep count, VERIFIED-READ).

### 3.4 Tool-specific env vars and names in neutral code (VERIFIED-READ)

- `NX_DAEMON`: `B/📥️inputs/🎯️selected/📦️snapshot/🟦️.ts:49` (`environment.NX_DAEMON!=="false"` throws), `B/📥️inputs/🎯️selected/🦀️rust/🟦️.ts:30,33` (`NX_DAEMON:"false"` is written into the environment).
- `SEMIO_NX_NATIVE_SOURCE_SNAPSHOT`: `B/📥️inputs/🎯️selected/📦️snapshot/🟦️.ts:19` (a neutral wire name with a tool prefix).
- `SEMIO_BAZEL_INTEGRATION` and `SEMIO_BAZEL_FIXTURE_PHASE`: `B/📜️script.ts:62` (set in the neutral bazel-test branch).
- `CARGO_NET_OFFLINE`: `B/📜️script.ts:111` (set into every neutral build child environment) and `:169` (read in the neutral dispatch).
- Directory names: `B/🕸️graph/🟦️.ts:154` lists `.nx` among neutral generated directories; `:404` filters `nx.json` by name.

### 3.5 Tool-specific commands and names in neutral inputs and policy (VERIFIED-READ)

- `B/📥️inputs/🟨️.mjs:212` `uv run --locked --no-sync --project ... python --version` is a literal command in neutral inference.
- `B/📥️inputs/🟨️.mjs:187,192,193,202,222,229,230,247` branch on `tools.includes("cargo"|"go"|"dotnet"|"python")`. `:202` hard-codes per-tool source extensions.
- `B/📏️incremental/🟦️.ts:18` `IncrementalFamilyV1` is a closed union of tool families (`vitest`, `playwright`, `dotnet`, `latex`, `docker`, `wasm`, `cargo`, `aggregate`, ...).
- `B/🔣️policy.json:3` `defaultTool: "nx"`; `B/⚙️operations/🔣️catalog.json:6-7` "Nx with Bun (default) and Bazel"; `:23` help text "(nx)". Three places repeat the default.
- `B/🕸️graph/📥️inference/🧶️bun-lock/🟨️.mjs:18,85` `bunLockGraph` and `readBunLockGraph` (the bun lock parser) live in neutral graph inference, registered as component `"bun-lock"` (`:9`). See 4.3.

## 4. Bun coupling (class b) and package manager (class c)

### 4.1 Bun runtime APIs called directly in production (VERIFIED-READ, grep)

Neutral build library:
- `Bun.Glob`: `B/🧭️selection/🟦️.ts:69` (neutral selection matcher, no fallback except extglob); `B/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:192` throws "Declared module globs require Bun.Glob" when absent; `B/🪞️parity/🏃️collect/🟦️.ts:129`; `B/🔧️tools/🏗️bazel/📜️script.ts:168,339`; `B/🔧️tools/🏗️bazel/🏃️execution/🏭️runner/🟦️.ts:49`.
- `bun:sqlite`: `B/📥️inputs/💾️storage/📒️ledger/🟨️.mjs:22` (guarded by `process.versions.bun`); `B/🧹️cache-gc/🏃️runner/🟦️.ts:29` (unguarded).
- `Bun.spawn`, `Bun.serve`, `Bun.file`: `B/🌐️remote-cache/🖥️server/🟦️.ts:78,119`; `B/📖️storybook/🏃️execution/🟦️.ts:48`; `B/🔧️tools/🏗️bazel/🏃️execution/🏭️runner/🟦️.ts:44`; `B/🏃️execution/🔒️hermetic/🔌️providers/🔷️typescript/🟦️.ts:10` (`Bun.spawn([process.execPath, compiler, ...])`).
- `Bun.stdin`, `Bun.which`, `Bun.resolveSync` in build code: `B/👷️workers/📨️protocol/🟦️.ts:46`; `B/🛠️cmake/🏃️execution/🟦️.ts:22` (`Bun.which`).
- Source admission: `B/📥️source/🔏️closure/🟦️.ts:44-45` uses `new Bun.Transpiler` and `Bun.plugin` to enforce immutable implementations. `B/📥️source/🛗️zero-touch/🟦️.ts:44` uses `Bun.Transpiler`.
- `process.versions.bun` in a cache key: `B/🚀️bootstrap/🟦️.ts:58` (the dependency receipt key includes the Bun version).
- Watch allowlist: `B/🧭️selection/👁️watch/🟦️.ts:21` `["bun","bun.exe",process.execPath]`.
- Input precision: `B/📥️inputs/🎯️precision/🟦️.ts:145` `tokens[0] !== "bun"`.
- Shebang: `B/📜️script.ts:1` `#!/usr/bin/env bun`.

Process framework:
- `bun:ffi` (Windows and Linux atomic exchange, Windows semaphores, lease physical checks): `P/🔒️leases/📁️physical/🟦️.ts:18`; `S/📁️directory/🔀️exchange/🟦️.ts:3,14,18`; `S/📁️directory/🪟️host/🟦️.ts:64-67,87`; `P/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts:93`; `S/🪟️conditional-retirement/🔌️port/🟦️.ts:42-43`; `S/📤️publication/📚️witness/🍎️events/🟦️.ts:38`. Note that `P/🔒️leases/🟦️.ts` is a shared lease module, so the coupling is not confined to storage.
- `bun:sqlite` duplicated: `P/🔒️leases/🟦️.ts:135-139`; `P/🔒️leases/📦️namespace/📒️baseline/🟦️.ts:19-21`. Three independent sqlite switches exist, with no common port.
- `Bun.TOML`: `P/🧭️routing/🎛️command/⚙️configuration/🟦️.ts:40`; `P/📦️artifacts/🏗️native-build/🎟️admission/🔖️scheduler/🟦️.ts:21`; `P/📦️artifacts/🏗️native-build/🟦️.ts:202`.
- `Bun.resolveSync("typescript/bin/tsc")`: `P/🛠️repository-tooling/🟦️.ts:62`.
- `Bun.which`: `P/🧪️testing/🧰️toolchain/🟦️.ts` (system port, injectable).

Positive: child processes are spawned with `process.execPath` (`B/📜️script.ts:42,71,247`; `B/📥️caller/📤️child/🟦️.ts:59`). Swapping the runtime would not break the spawn sites, but would break the APIs above.

Test-only coupling (not counted above): `B/🧪️tests/🕸️graph/📌️pinned-publication/🟦️.ts` and others use `Bun.CryptoHasher`, `Bun.sleep`, `spyOn`. Tests use the Bun test runner (INFERRED: `bun test`).

### 4.2 Package manager (bun install) seams

- Installer slot: `B/🚀️bootstrap/🟦️.ts:18` (`BuildEnvironmentInstallerV1`) and `:47` default parameter. The default installer is imported from `L/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` (out of scope; INFERRED to run `bun install`).
- Lock name: `B/🔣️policy.json:51` lists `"bun.lock"` as a graph member; `B/🚀️bootstrap/🟦️.ts:58` reads `join(root,"bun.lock")` directly in the neutral bootstrap; `B/🪞️parity/🔣️rules.json:92-99` names `bun.lock` in parity rules.
- Lock parser: `B/🕸️graph/📥️inference/🧶️bun-lock/🟨️.mjs` (105 lines, `bunLockGraph`, `readBunLockGraph`), registered as a graph component (`:9` `component = "bun-lock"`). This is the right shape (a component registry) but the component is bun-specific, and a yarn or pnpm sibling is not present. INFERRED: the registry consumer is `B/🕸️graph/📥️inference/🗂️components/🔣️.json`, not read in full.
- Lock reader: `B/📥️inputs/🔒️lock/📥️read/🟨️.mjs:21` comment says "nx and Bazel fold it with this one function". The lock reader is shared between tools, which is good.
- Bazel adapter: `B/🔧️tools/🏗️bazel/📜️script.ts:466,627,637,652` pass `bun` as a rule attribute to generated Bazel rules.

## 5. Toolchains (class d)

Positive: the policy table (`B/🔣️policy.json` `toolchains`, read by `B/📥️inputs/🟨️.mjs:207`), the script-provider ports (`P/🔌️providers/🟦️.ts`), and the test-tool resolver (`P/🧪️testing/🧰️toolchain/`) are the right shapes.

Literal executables in framework code (VERIFIED-READ):
- `cargo`: `P/📦️artifacts/🏗️native-build/🌊️relay/🟦️.ts:31` (`captureOwnedProcess("cargo", ...)`); `P/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts:360` (`spawn("cargo", ...)`); `P/📦️artifacts/🕸️wasm-build/🟦️.ts:121` (`invoke("cargo", ...)`).
- `rustc`: `P/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts` (name in a capture path, INFERRED to spawn).
- `go`: `P/📦️artifacts/🐹️go-build/🟦️.ts:30` (`runOwnedCommand("go", ["build", ...])`).
- `wasm-bindgen` and `wasm-pack`: `P/📦️artifacts/🕸️wasm-build/🟦️.ts:104` (label) and the wasm-bindgen step.
- `codesign`: `P/📦️artifacts/🏗️native-build/🟦️.ts:68` (`spawnSync("codesign", ...)`).
- `hdiutil`: `S/🍎️image/🟦️.ts:29` (`captureOwnedCommandV1("hdiutil", ["info", "-plist"], ...)`); `S/🧹️cleanup/🎛️command/🟦️.ts:70,190`.
- `vitest`: `P/🧭️routing/🎛️command/⚙️configuration/🟦️.ts:42` (`vitestPolicy.toolPath` from configuration).
- Dotnet configuration: `P/🏗️build/🎚️mode/🟦️.ts:25` (`dotnetConfiguration`).
- Not found in scoped trees: `uv` executable (only the literal in `B/📥️inputs/🟨️.mjs:212`), `tectonic`, `playwright` (only in schema enums and globs), `dotnet` executable. INFERRED: these are invoked from out-of-scope product trees.

Toolchain interface: none. There is no `Toolchain` type with `resolve`, `probe`, `invoke` and `provision`. Each framework area calls its own tool.

## 6. Storage substrate (class e)

- Selection by OS, not by config: `S/🌐️platform/🟦️.ts:21` `nativeStoragePlatformV1(platform)` maps `darwin` to `macos` and everything else to `linux` or `windows`. `:24` opens with `backend==="macos"?openNativeImageStorageV1:openNativeDirectoryImageStorageV1`. `:27` retires by the same ternary. `:38` and `:40` capture and read cleanup by the same ternary. Five or more ternaries, one per operation.
- Neutral contract imports the directory backend: `S/🌐️platform/📋️contract/🟦️.ts:3` imports `nativeDirectoryProtocolV1`, `observeNativeDirectoryStateV1`, `readNativeDirectoryAnchorFileV1` from `S/📁️directory/🟦️.ts`. The recovery modules import `flushNativeDirectoryV1` from the directory backend: `S/🌐️platform/🥾️boot/🩹️recovery/📒️ledger/🟦️.ts:4` and `S/🌐️platform/🥾️boot/🩹️recovery/🧯️act/🟦️.ts:5`. Recovery code is therefore not backend-neutral.
- Vocabulary: the neutral type is `NativeImageStorageV1` (`S/🌐️platform/📋️contract/🟦️.ts:36`). Helpers are `nativeImageRootV1`, `nativeImageBackingFilenameV1`, `nativeImageAttachedV1`, `nativeImageDirectoryHostV1` (`:48-62`). The directory backend implements an "image" interface. The artifact cache requires the image type: `S/📦️artifacts/💾️cache/💾️volume/🟦️.ts:32` `parent:NativeImageStorageV1`. The DMG concept is therefore part of the cache's type surface.
- hdiutil in a shared module: `S/🧹️cleanup/🎛️command/🟦️.ts:70,190` (the cleanup command is imported by the platform module at `S/🌐️platform/🟦️.ts:4`).
- Directory backend Bun coupling: `S/📁️directory/🔀️exchange/🟦️.ts:3,14,18` (`bun:ffi` binds the libc call that atomically exchanges two directory names), `S/📁️directory/🪟️host/🟦️.ts:66` (throws unless `process.versions.bun`). The directory backend is not Bun-free.
- Contract schema: `S/🌐️platform/📋️contract/🧬️schema/` and `S/📁️directory/🧬️schema/🔣️.json` exist (INFERRED that a shared schema governs both). Tests: `S/🌐️platform/🧪️tests/🧭️dispatch/🟦️.ts` tests dispatch (test body not read). Backend tests exist per backend (`S/🍎️image/🧪️tests`, `S/🧱️fixed/🧪️tests`). No test was found that runs one suite against both backends, but this is by file-name inspection only (INFERRED).
- Build-tool storage: `B/🔧️tools/💾️storage/🟦️.ts:6-7` is a closed `"nx"|"bazel"` switch with an `else throw`. Bazel storage is `B/🔧️tools/🏗️bazel/💾️storage/🟦️.ts`. Nx storage is in out-of-scope `L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph`.
- Sandbox tests use hdiutil: `P/🧪️testing/🏖️sandbox/🖼️images/🟦️.ts` (3 hits). Test infrastructure is coupled to DMG images.

## 7. Conformance and contract tests (cross-cutting)

- No single backend contract suite. Tests are pairwise: `B/🪞️parity/🟦️.ts` compares Nx and Bazel over the samples in `B/🪞️parity/🔣️samples.json` (27 entries with an `id`), with `ParityToolV1` fixed to `"nx"|"bazel"` (`:6`) and results keyed `{nx:{}, bazel:{}}` (`:322`).
- `B/⚙️operations/🧪️tests/🟦️.ts:67` asserts `["nx","bazel"].every(tool => ...)` over the catalog. A third tool would be silently skipped unless the list is edited.
- `B/🌐️remote-cache/🧪️tests/🟦️.ts:15-21` imports both adapters by name.
- `B/🔧️tools/🏗️bazel/⚙️operations/🧪️tests/🟦️.ts:43-47` builds an Nx oracle for Bazel results.
- The positive: `B/⚙️operations/🔣️catalog.json` is a schema-like catalog with one row per operation and an honest `implementations` state (`native|adapter|neutral|missing`).

## 8. Gaps ranked (most blocking first)

1. **Build-backend seam absent (a), high.** `B/📜️script.ts:157-252` is an if/else on `selected.tool` with direct adapter imports (`:231`). No `BuildBackend` interface covers run, plan, affected, exec, watch, test or reset. Each new backend touches the dispatch, 11 closed unions, 19 catalog rows and the test lists.
2. **Nx and Bazel names and env in neutral code (a, cross-cutting), high.** Section 3.3 and 3.4. Examples: `NX_DAEMON` in `B/📥️inputs/🎯️selected/📦️snapshot/🟦️.ts:49`; `SEMIO_BAZEL_*` in `B/📜️script.ts:62`; the Nx plugin schema import in `B/📥️caller/🟦️.ts:4`. These break the claim that the neutral layer is tool-free.
3. **Storage backend by OS ternary (e), high.** Section 6. The neutral contract imports the directory backend, the recovery modules are directory-specific, and the vocabulary is "image". Changing the backend means editing five or more ternaries and two recovery modules.
4. **Bun APIs in production, several neutral (b), high.** Section 4.1. `Bun.Glob` is required by neutral selection and graph inference (`B/🧭️selection/🟦️.ts:69`, `B/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:192`). `bun:ffi` is used by the shared lease module (`P/🔒️leases/📁️physical/🟦️.ts:18`) and the directory backend. Three separate `bun:sqlite` switches exist.
5. **Lock file and install key are bun-specific (c), medium-high.** Section 4.2. The installer slot is good; the lock name (`B/🔣️policy.json:51`), the bootstrap read (`B/🚀️bootstrap/🟦️.ts:58`) and the parser component are not.
6. **Toolchain executables are literals (d), medium.** Section 5. The policy table and provider ports are good, but cargo, go, codesign, hdiutil and wasm-bindgen are called from framework code, and `uv` is a literal in `B/📥️inputs/🟨️.mjs:212`.
7. **No per-backend conformance suite (all), medium.** Section 7. Tests compare pairs, and a new backend would be skipped by the hard-coded lists (`B/⚙️operations/🧪️tests/🟦️.ts:67`).
8. **Default tool hard-coded in three places (a), low.** `B/🔣️policy.json:3`, `B/⚙️operations/🔣️catalog.json:6-7`, `:23`.

## 9. What it would take to add a third backend (for example "buck2")

Minimum file set, based on the leak inventory:

- Schema: add the name to `B/🧬️schema/🔣️.json:547` `BuildTool`. Replace the 11 closed unions with one derived type (TS types from the schema, or a single exported union that the others import). Update `B/📏️incremental/🧬️schema/🔣️.json:38-39`, `B/🪞️parity/🧬️schema/🔣️.json:33-34`, `B/🧾️ci-report/🧬️schema/🔣️.json:5`.
- Adapter: implement `BuildCacheToolV1` (`B/⚙️operations/🟦️.ts:24`), `CacheProbeToolV1` (`:40`), `RemoteCacheToolV1` (`B/🌐️remote-cache/⌨️command/🟦️.ts:6`), a storage owner for `B/🔧️tools/💾️storage/🟦️.ts:6`, and the run, plan, affected, exec, watch and test entry points that the dispatch currently calls on the Bazel adapter (`B/📜️script.ts:231-252`).
- Catalog: add a key to all 19 `implementations` rows (`B/⚙️operations/🔣️catalog.json`).
- Dispatch: replace the three ternaries (`B/📜️script.ts:157`, `:188`) and the if/else (`:215`) with a registry lookup.
- Caller: extend `B/📥️caller/🟦️.ts:21,24,34` and the child protocol at `:82-87`.
- Measurement and parity: `B/📏️incremental/🟦️.ts:6,85,94,157`; `B/📏️incremental/🏃️runner/🟦️.ts:194`; `B/🪞️parity/🏃️collect/🟦️.ts:146`; `B/🧾️ci-report/🟦️.ts:152`.
- Storage: a new branch in `S/🌐️platform/🟦️.ts` and the cleanup command, or better, a registry keyed by platform and backend.
- Tests: add the backend to `B/⚙️operations/🧪️tests/🟦️.ts:67` and to a new shared contract suite driven by the registry.

Estimated scope: about 15 source files plus the schema and catalog, all in neutral code. No file under `P` needs to change for a build backend. A storage or toolchain change does touch `P`.

## 10. Limits

- Out-of-scope trees were not read: `L/⚡️caching/...` (the Nx adapter, the dependency installer, `🌐️remote-cache` Nx side), and the test bodies in most `🧪️tests` folders. Statements about them are INFERRED from import paths.
- `else` branches of the sqlite guards (`B/📥️inputs/💾️storage/📒️ledger/🟨️.mjs:22`, `P/🔒️leases/🟦️.ts:139`) were not read in full. They are assumed to fall back to `node:sqlite` (INFERRED).
- Nothing was run. All "live" claims are source-level.
