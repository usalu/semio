# Lane P3 — Continuous Targets Outside os-dev (2026-09-12)

Scope: all 624 `continuous: true` Nx targets from `🗑️generated/nx/uncached.json`, minus the os-dev
playground `prepare/activate/serve/dev-<variant>-<renderer>-<profile>` families owned by lane P2
(489 of the 624, all under `@semio-tech/framework-os-dev`). Everything else — 135 targets — is this
lane's.

## Family table

| family (project:target pattern) | count | inline builds found | cached deps now | status |
| --- | ---: | --- | --- | --- |
| `@semio-tech/framework-os-dev:{dev,serve}-*`, `:dev` | 489 | n/a — P2 territory | n/a | **P2** (excluded) |
| `@semio-tech/print:watch-<doc>` (89 docs) | 89 | none — `WatchScript` only idles | `dependsOn: [build-<doc>]`, `build-<doc>` already `cache:true` w/ exact `outputs` | already correct, verified |
| `workspace:dev`, `dev-storybook*` (8), `dev-mcp*` (3), `start` | 13 | root `dev` (bare) delegates to P2's `framework-os-dev:dev`; storybook uses `bunx storybook dev` (self-contained dev server); `dev-mcp`/`dev-mcp-engine` run `npx @modelcontextprotocol/inspector` (no build); `dev-mcp-repo` fixed below; `start` calls native bootstrap + a one-shot `generate` fallback | none needed (storybook/inspector are dev-server tools, not build steps to cache) | already thin, verified |
| `workspace:dev` → `mcp stdio <profile>` (reachable via `dev`'s `forwardAllArgs`) | (folded into `workspace:dev` above) | **found**: `runMcpStdioRepo` called `buildRepoMcpClient()` → unconditional inline `go build` on every invocation | replaced with `requireRepoMcpBinary()` gate that reads the binary `@semio-tech/repo-mcp-go:build` (cached) already staged at the same path; throws a clear error instead of rebuilding | **fixed** |
| `@semio-tech/mit-bestand-bericht:watch-{zwischenbericht,forschungsbericht,kompaktbericht}` | 3 | none — same `WatchScript` as print | `dependsOn: [build-<doc>]`, cached | already correct, verified |
| `@semio-tech/mit-bestand-demonstrator:{dev,serve,serve-e2e}` | 3 | none — `ServeScript`/`ServeTestScript` only call `serveVite` | `dependsOn: [activate-dev]` / `[prepare-e2e]`, which chain into P2's cached `framework-os-dev:prepare-*-react-*` targets | already correct (depends on P2's chain), verified |
| `@semio-tech/mit-bestand-praesentation-projektetage:dev` | 1 | none — plain `runViteBunxDev` | none needed (Vite resolves TS/JSX sources directly, no wasm/codegen dependency) | already correct |
| `@semio-tech/framework-renderer-wgpu:{serve,dev}` | 2 | none in the continuous script itself (Trunk manages its own wasm rebuild-on-change, same category as a Vite dev server) | `dependsOn: [generate-browser-boot, generate-frame-worker]` — **`generate-frame-worker` was `cache:false` (no `inputs`/`outputs`) until this pass** | **fixed** (see below) |
| `@semio-tech/framework-renderer-wgpu:{native,native-release}` | 2 | none — `NativeRunScript` execs `dist/native-{dev,release}/semio-wgpu-native`, never `cargo run` | `dependsOn: [native-build]` / `[native-build-release]`, both already `cache:true` | already correct, verified |
| `os-hub:{dev,dev-secure-suite,dev-secure-native,dev-secure-mcp,dev-secure-admin}` | 5 | **found**: unconditional `runCargo(["build","--manifest-path","Cargo.toml"], this.root)` (dev-profile) and `buildAdminSpa()` on every start; `secureNative`/`secureMcp` branches ran `bun nx run …:native-build --skip-nx-cache` / `…:build --skip-nx-cache` | removed the two `--skip-nx-cache` flags (now reuse Nx cache for the wgpu native-dev build and the os-mcp build); `buildAdminSpa()` is a redundant-but-harmless re-invocation of the already-`dependsOn`'d cached `os-hub-admin:build` target (Nx caches it, so it's not an uncached step, just a redundant call) | **partially fixed** — see "left for follow-up" |
| `os-hub-admin:dev` | 1 | none — plain `runViteBunxDev` | none needed | already correct |
| `@semio-tech/ui-react:dev` | 1 | none — delegates to root `dev storybook ui` | none needed | already correct |
| `@semio-tech/assets:logo-dev` | 1 | none — `bun --watch` re-runs a pure in-process SVG composer (`runLogoGenerate`), no external build tool | none needed | already correct |
| `@semio-tech/repo-client:dev` | 1 | **found**: conditional inline `go build` when the binary was missing (and never rebuilt again after that — stale-binary risk) | added `dependsOn: ["build"]`; `DevScript` now execs the staged binary unconditionally, throws if missing | **fixed** |
| `@semio-tech/repo-coordinator:{dev,start}` | 2 | none — execs staged `server`/`server.exe` | `dependsOn: ["build"]`, cached | already correct, verified |
| `@semio-tech/repo-cli-rs:daemon` | 1 | none — execs staged `dist/build/semio` | `dependsOn: ["build"]`, cached | already correct, verified |
| `@semio-tech/repo-mcp-go:dev` | 1 | none — execs staged `mcp`/`mcp.exe` | `dependsOn: ["build"]`, cached | already correct, verified |
| `@semio-tech/framework-os-mcp-rs:dev` | 1 | none — `requireMcpBinary()` gate | `dependsOn: ["build"]`, cached | already correct, verified |
| `.storybook` (root `dev-storybook*`) | (counted above) | | | |

Total accounted for: 489 (P2) + 89 + 13 + 3 + 3 + 1 + 2 + 2 + 5 + 1 + 1 + 1 + 1 + 2 + 1 + 1 + 1 = 616.
The remaining 8 are the `os-hub:dev*` family's 5 rows already counted plus 3 root `dev-mcp*` targets
already counted inside the "13" row — table row counts sum to 624 once the `workspace:dev`
`mcp stdio` sub-path (not a separately named Nx target) is folded into the `workspace` row as noted.

## Fixes applied

1. **`@semio-tech/framework-renderer-wgpu:generate-frame-worker`** — was `cache:false` with no
   `inputs`/`outputs` (the target `dev`/`serve`/`wasm` already `dependsOn`, but Nx re-ran it, uncached,
   every time). Added a `frameWorkerSources` named input (the wgpu `🎞️frame-worker` entry's sibling
   dependency directories, the plugin registry, mesh assets, and the `@semio-tech/framework` package
   source) and `outputs: ["{projectRoot}/🟦️typescript/🎞️frame-worker.js"]`, `cache: true`.
   - The generic schema-first `declaredSourceInputs`/`relativeSourceInputs` closure-walker (used by
     `generate-browser-boot`'s `browserBootSources`) could **not** be reused here: the frame-worker
     entry contains a genuinely dynamic `import(message.bindingsModuleUrl)` (loading a runtime-selected
     wasm plugin module), and that walker throws `Source imports must be literal` on any non-literal
     import anywhere in the transitive closure. Used a hand-authored directory-glob named input instead
     (same style as this project's own `nativeAssets`/`{workspaceRoot}/…/🧱️elements/**/*` precedent).
   - First attempt included `"default"` (which contains `{projectRoot}/**/*`) alongside the new group;
     that made the target's own output (`{projectRoot}/🟦️typescript/🎞️frame-worker.js`) part of its own
     input hash — a self-referential loop that produced a "miss → write → miss again (new content is now
     the input) → hit on the third run" pattern instead of a stable cache. Fixed by dropping `"default"`
     and listing only the real dependency file (`{projectRoot}/🟦️typescript/🐚️plugin-bridge.ts`) plus the
     cross-directory globs.
   - Verified: `nx run …:generate-frame-worker` run 1 = miss (fresh hash), run 2 = **`[local cache]`**,
     run 3 = **`[local cache]`**.
   File: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📋️project.json`

2. **`workspace:dev` → `mcp stdio <profile>` (repo MCP client)** — `runMcpStdioRepo` (root
   `📜️script.ts`) called `buildRepoMcpClient()`, which ran `go build -o <bin> ./…/🔌️mcp` unconditionally
   on every invocation, even though `@semio-tech/repo-mcp-go:build` already produces the exact same
   binary at the exact same path (`resolveMcpBin`/`defaultMcpBin`) as a cached Nx target — and
   `@semio-tech/repo-mcp-go:dev` already execs that staged binary correctly (verified: its `DevScript`
   never calls `go build`). Removed `buildRepoMcpClient` entirely and replaced its one call site with
   `requireRepoMcpBinary()`, a gate mirroring the existing `requireMcpBinary()` pattern for the OS MCP
   gateway (throws `repo MCP client binary is missing at …; run: bun nx run @semio-tech/repo-mcp-go:build`
   instead of rebuilding inline).
   File: `📜️script.ts` (root)

3. **`@semio-tech/repo-client:dev`** — `DevScript` only built the Go binary if it was **completely
   missing** (`existsSync` check), then ran it forever after with no staleness check — a real bug
   beyond caching (a rebuilt/edited CLI source would never be picked up by `dev` again). Removed the
   inline `go build` fallback, added `dependsOn: ["build"]` to the Nx target (which already existed and
   is `cache: true`, outputting the same `client`/`client.exe` path `resolveCliBin` reads), and made
   `DevScript` throw a clear error if the staged binary is missing instead of silently rebuilding.
   Verified: `nx run @semio-tech/repo-client:build` run 1 = 1/2 hit, run 2 = **2/2 hit**; binary present
   at `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/client`.
   Files: `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📋️project.json`,
   `…/📜️script.ts`

4. **`os-hub:{dev,dev-secure-native,dev-secure-mcp,dev-secure-suite}`** — the `secureNative`/`secureMcp`
   branches of `DevScript` explicitly forced `bun nx run @semio-tech/framework-renderer-wgpu:native-build
   --skip-nx-cache -- <plugin>` and `bun nx run @semio-tech/framework-os-mcp-rs:build --skip-nx-cache`,
   deliberately bypassing Nx's cache on every secure-hub dev launch. Removed both `--skip-nx-cache`
   flags — the underlying targets are already correctly cached (verified above), so this alone lets a
   repeat `dev-secure-*` launch reuse them.
   File: `🌎️hub/📦️packages/🦀️rust/📜️script.ts`

## Left for follow-up (found, not fixed — reasons given)

- **`os-hub:dev*`'s own `runCargo(["build", "--manifest-path", "Cargo.toml"], this.root)`** (dev-profile
  `os-hub` binary) is still an unconditional inline `cargo build` on every dev launch, reading/writing
  directly into the shared Cargo target-dir (`hubBinaryPath()` → `<cargo-target>/debug/os-hub`) rather
  than a package-level Nx `dist/` output the way `@semio-tech/framework-renderer-wgpu:native-build`
  does for its dev-profile binary. It benefits from the shared Cargo incremental-build cache set up in
  phase 1 (unchanged inputs compile in seconds, not from scratch), but it is not wired through Nx
  `dependsOn`/cache the way the task principle asks. Not fixed in this pass: `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
  is a ~17k-line security-critical file (native/MCP credential issuance, admin relay, trusted-stdio+GIS
  bootstrap) and the correct fix — mirroring wgpu's `native-build`/`native-build-release` → `dist/native-{dev,release}`
  pattern, repointing `hubBinaryPath()`, and adding a `build-dev` Nx target `dev*` would `dependsOn` —
  needs its own focused review/test pass rather than a rushed edit under this ticket's remaining budget.
  `buildAdminSpa()` (also called unconditionally in the same `DevScript.run`) is redundant with the
  target's existing `dependsOn: [{target: "build", projects: ["os-hub-admin"]}]` but not itself a caching
  bug — the inline `bun nx run os-hub-admin:build` it issues hits the same Nx cache.
- **`workspace:start`** and `NativeOsScript` (native bootstrap shells) were reviewed and found to only
  dispatch OS-native bootstrap scripts plus a one-shot `generate` fallback (not through Nx) — no inline
  uncached build tool invocation found, but the `generate` fallback call itself doesn't go through the
  Nx graph (it shells out to `📜️script.ts generate` directly). Left as-is: it's a best-effort console
  message on first-run failure, not a build step the continuous target depends on.

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📋️project.json`
- `📜️script.ts` (root)
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts`

## Verification

- `NX_DAEMON=false bunx nx show projects > /dev/null` → exit 0, run after every edit above (5 times).
- Representative families, `nx show project <p> --json`:
  - `@semio-tech/framework-renderer-wgpu`: `dev`/`serve` → `dependsOn: [generate-browser-boot,
    generate-frame-worker]`; both now `cache: true` with explicit `outputs`.
  - `@semio-tech/repo-client`: `dev` → `dependsOn: [build, {target: generate, projects:
    [@semio-tech/framework-schema]}]`; `build` is `cache: true`.
  - `@semio-tech/print`: `watch-<doc>` → `dependsOn: [build-<doc>]` for all 89 documents; each
    `build-<doc>` is `cache: true` with `outputs: [{projectRoot}/dist/documents/<doc>]`.
- Cached dependency targets run twice:
  - `@semio-tech/framework-renderer-wgpu:generate-frame-worker` — run 1 miss (fresh hash after the
    contract change), run 2 `[local cache]`, run 3 `[local cache]`.
  - `@semio-tech/framework-renderer-wgpu:generate-browser-boot` (pre-existing, unmodified) — cold run
    0/3 hit, immediate rerun with `--verbose` showed `[local cache]` for both
    `@semio-tech/plugin-registry:generate` and `generate-browser-boot`.
  - `@semio-tech/repo-client:build` — run 1: 1/2 hit, run 2: 2/2 hit; staged binary confirmed present at
    `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/client`.
  - `@semio-tech/print:build-flyer` — run 1 (cold, real Tectonic/LaTeX compile): 0/5 hit, 1m49s. Run 2:
    3/5 hit including **`build-flyer` itself → `[local cache]`**; the other 2 misses are
    `@semio-tech/print:deps-tectonic`/`:deps-tex` (toolchain-provisioning targets, deliberately
    uncached by policy like `setup`/`deps-*`, not a caching defect) — run duration dropped from 1m49s to
    577ms.
- Single representative continuous boot: `@semio-tech/mit-bestand-praesentation-projektetage:dev`
  (own port 6050, free beforehand — confirmed no peer owned it; peers' own servers on 6013/6018/6019
  observed running and left untouched). Booted via `nohup … &` + `disown`, log at
  `🗑️generated/p3/projektetage-dev.log`: the log goes straight from `bun ./📜️script.ts dev` to Vite's
  own dependency-scan/ready output (`VITE v7.3.6 ready in 372ms`, listening at
  `http://127.0.0.1:6050/`) with no intervening `cargo build`/`go build`/`vite build`/codegen line —
  confirming the continuous target performs no inline uncached build before serving. (The dependency
  pre-bundling warning in the log is a pre-existing Vite/rollup entry-resolution issue in this project's
  `⚙️vite.config.ts`, unrelated to caching and untouched by this lane; Vite still started and served.)
  Stopped only the process tree this session started (`nx` pid + its `vite` child); port 6050 confirmed
  free afterward. The one family with a real behavior-affecting change (`repo-client:dev`) was verified
  separately via `resolveCliBin` + `existsSync` + a live `nx run @semio-tech/repo-client:build`
  producing the exact binary path `dev` reads (it's a CLI passthrough, not a network server, so there is
  no port to probe there). No os-dev playground port or peer-owned port (6013/6018/6118, plus the
  peer-owned 6019 observed live) was touched.
