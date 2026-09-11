# Single Shared Cache Plan (2026-09-11)

Session `⚪83140bef0e504e03b0b3380912b12a0e`. The repo MCP failed to connect (`repo`: invalid initialize params),
so ticket bookkeeping for this session is manual (`🎫️ticket.json` edited on disk).

## Goal

Every build/dev/test/etc step is Nx-cached on package level (crate, npm package, …). There is ONE cache root that
every agent and dev shares — `.🧬semio/🦑️repo/⚡️cache/` — so disk is bounded by codebase size and build history,
not by the number of concurrent builders.

## Measured starting point (2026-09-11 13:10)

| location | size | problem |
| --- | ---: | --- |
| `target/` | 74 G | repo-wide cargo target, one directory lock → agents fork private dirs |
| `.🧬semio/🦑️repo/⚡️cache/cargo/browser` | 54 G | second full cargo target (`wasmBuildEnvironment` default) |
| `.🧬semio/🦑️repo/⚡️cache/agents/local` | 7.2 G | per-agent cargo test hosts |
| session scratchpads `9f5f6952…`, `9e1e818a…` | 186 G + 58 G | private `CARGO_TARGET_DIR`s of dead sessions (not ours, surfaced to the dev) |
| `~/Library/Caches/Mozilla.sccache` | 10 G | sccache, 16.8 % hit rate, single server serialises every session |
| `.nx/cache` | 7.6 G | Nx cache outside the cache root |
| per-package `target*` dirs, `target-gen3d-opt` | ~2 G | standalone/ad-hoc targets |

Disk free: 31 G of 926 G.

## Key evidence (probes in `🐚️fine-grain-lock-probe.sh`)

Toolchain `nightly-2026-07-07` (cargo 1.99) ships `-Zfine-grain-locking`, `-Zbuild-dir-new-layout`, `-Zchecksum-freshness`.

1. Two concurrent `cargo check` (`semio-framework-os-kernel` + `semio-framework-ui`) on ONE fresh build-dir: both exit 0,
   69 s total; the second only printed `Blocking waiting for file lock on proc-macro2 (9ba370…)` — per-unit waits, then reuse.
2. A different `CARGO_TARGET_DIR` with the same build-dir: `Finished … in 0.67s` — intermediates are shared, a private
   target-dir only diverts uplifted deliverables.
3. `touch Cargo.toml` with checksum freshness: `Finished … in 0.30s`, nothing rebuilt (Nx restores/checkouts are free).
4. `RUSTFLAGS=-Awarnings` creates a sibling unit hash dir (`06dadf835160ba98`) instead of overwriting — flag variants coexist,
   no ping-pong rebuilds.
5. New layout: `<build>/<triple?>/<profile>/build/<package>/<unit-hash>/{.lock,fingerprint,out}` and
   `<build>/<…>/incremental/<crate>-<hash>/`. One directory per compilation unit → exact LRU pruning.
6. File atime behaves like relatime (updated on read when older than mtime or > 24 h): `atime < now − 48 h` proves no build
   read the unit in the last 24 h. That is the lock-free safety guard for pruning.
7. `.cargo/config.toml` relative `build-dir`/`target-dir` resolve against the repo root from any subdirectory/workspace.

## Design

### Cache root layout (`.🧬semio/🦑️repo/⚡️cache/`)

| path | owner | bound |
| --- | --- | --- |
| `nx/` | Nx `cacheDirectory` (moved from `.nx/cache`) | `maxCacheSize` 16GB (Nx LRU) |
| `cargo/build/` | cargo `build.build-dir` (all workspaces, profiles, targets, agents) | pruner: unused > N days, then LRU to budget |
| `cargo/target/` | cargo `build.target-dir` (uplifted deliverables only) | pruner, same rules |
| `vite/<consumer>/` | Vite/Storybook dependency-optimizer caches (from `node_modules/.vite-*`) | pruner age rule |
| `tools/ms-playwright/` | Playwright browsers (from `node_modules/.cache/ms-playwright`) | versioned tool install |
| `go/` | `GOCACHE` for repo-spawned Go | `go clean -cache` trimming + pruner |
| existing `cmake/`, `tectonic/`, `tools/`, `tests/`, `oracles/`, … | unchanged | unchanged |

Removed: `cargo/browser/` (second target), `agents/*/cargo-*` targets, root `target/`, every per-package `target*`,
sccache (wrapper + bootstrap + `RUSTC_WRAPPER=""`/`SCCACHE_DISABLE` overrides — with one shared fine-grain build-dir
it only adds a serialising server and a second out-of-repo cache).

Package-manager download caches (`~/.cargo/registry`, `~/.bun/install/cache`, uv) stay at their standard user
locations: they are keyed by the lockfiles, already shared by every agent, and bounded by dependency history —
`CARGO_HOME` also hosts the rustup proxies, so moving it would break the toolchain.

### Wave A — core config (done by the coordinator)

- `.cargo/config.toml`: `build.target-dir`, `build.build-dir`, `[unstable] fine-grain-locking / build-dir-new-layout /
  checksum-freshness`; `rustc-wrapper = "sccache"` removed.
- `nx.json`: `cacheDirectory` → `.🧬semio/🦑️repo/⚡️cache/nx` (existing entries moved, hits preserved), `maxCacheSize` 16GB.
- `policy.json` cargo hash env: `CARGO_TARGET_DIR` removed (does not change deliverable bytes; it split cache keys
  between agents), `CARGO_PROFILE_WASM_{DEV,RELEASE}_DEBUG` added (they do change bytes).
- Central resolver `⚡️caching/🦀️cargo/🟦️.ts`: `cargoDirectories / cargoTargetDirectory / cargoBuildDirectory` read
  Cargo's own precedence (env over `.cargo/config.toml`) — the ONLY way scripts may locate cargo output.
  Hot paths already switched: dev `buildPluginCargo`, describe `cargoTargetRoot`.

### Wave B — execution fleet (Sonnet 5, file-disjoint lanes)

| lane | files owned | work |
| --- | --- | --- |
| S1 library core | `📚️library/📦️packages/🟦️typescript/**`, `📚️library/🏃️process/**`, `📚️library/🧪️tests/**` (not `⚡️caching/🧪️tests`), `🧪️test` module (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/**`) | `wasmBuildEnvironment` stops forcing `cargo/browser`; `devToolingEnv` drops `RUSTC_WRAPPER`, routes `GOCACHE` + `PLAYWRIGHT_BROWSERS_PATH`; `agentCacheRoot` no longer hosts cargo targets; library tests stop passing private `--target-dir` |
| S2 root script | root `📜️script.ts`, root `📋️project.json`, `package.json` | remove sccache bootstrap; ticket-scoped `CARGO_TARGET_DIR`s; playwright path; `clean` never touches the cache root (delegates to `cache-prune`) |
| S3 os dev | `🧑‍💻dev/**`, `.storybook/**` | WGPU dev path consumes Nx `component-dev`/`materialize-dev` outputs instead of raw cargo; vite `cacheDir` → cache root; storybook cacheDir; `PARITY_CARGO_TARGET_DIR`/owned private targets removed |
| S4 os modules | `🔌️plugin/**`, `🌉️mcp/**`, `📺️renderer/**`, other `💻️os/**` except `🧑‍💻dev` | describe/materialize/mcp/binary-gate/wgpu renderer use the resolver; drop `SCCACHE_DISABLE`/`RUSTC_WRAPPER` |
| S5 s + hub | `✏️s/**`, `🌎️hub/**`, `♻️mit-bestand/**` | probes, bridges, generators, tests: no private target dirs, resolver for artifact paths |
| S6 launch/devcontainer/CI | `.vscode/**`, `.claude/launch.json`, `.devcontainer/**`, `.github/**` | remove private `CARGO_TARGET_DIR`, `RUSTC_WRAPPER`, `SCCACHE_*`; persist the cache root as one volume; CI caches the cache root |
| S7 caching module | `📚️library/⚡️caching/**` (incl. `policy.json`, schema, tests, fixtures), `📚️library/🟨️.mjs`, `📚️library/🔌️nx-plugin/**` | `cache prune` / `cache report` commands + `repo:cache-prune` target, budgets in policy (schema-first), throttled auto-prune after native cargo runs, language-agnostic fixtures + third-party oracle; registry/fixtures for new Nx dir; precise `default` inputs |

### Wave C — verification

Runtime evidence for: shared build-dir concurrency through `bun nx run <crate>:check` twice in parallel, Nx cache
hit on rerun (`[existing outputs match the cache, left as is]`/`[local cache]`), pruner dry-run + real run on the live
cache, dev boot (`generation3d` react) reading plugin wasm from the shared target, disk before/after.
