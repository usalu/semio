# WP-Z2 — Zero-Touch Cross-Platform (Windows / Linux / Devcontainer)

Slice Z2, session 11 (2026-09-25), Opus 5.5 executor. Sources: `📓️audit-s11-cross-platform.md`,
`📓️audit-s11-build-health.md` §2 + P0-3, `.tmp-ticket-0918/📓️z1-zero-touch-and-launch-rows.md`, `📓️g4-zero-touch-and-run-paths.md`.
Captures: `wp-z2/generated/`. One-off probes/codemods: `wp-z2/*.py`, `wp-z2/*.ts`, `wp-z2/linker-probe/`, `wp-z2/linux-proof/`,
`wp-z2/ps-lint/`. Ports: none bound (hubs 8100–8109, serves 6600–6609 reserved for item 3). Cut once by a network outage
(~03:1x); resumed 03:26, state re-verified (no container was left running; my stalled `verify taxonomy report`, pid 55715, killed).

## Status

| # | Item | Status | Evidence |
|---|---|---|---|
| 1a | Windows long paths: git `core.longpaths`, `LongPathsEnabled` detection + en/de message, MAX_PATH budget | LANDED + TESTED (path law RED only on peer-staged ticket data, §4) | `setup git` writes `core.longpaths` repo-local — tested in a scratch clone with `GIT_CONFIG_GLOBAL` sentinel byte-identical; `🔵️.ps1` guidance; law `🥾️cross-platform-bootstrap` |
| 1b | Native Linux `mold` hard requirement | LANDED + MEASURED (§3: os-hub link rust-lld 6.6 s vs mold 3.3 s vs GNU ld 37.8 s) | `.cargo/config.toml` → toolchain `rust-lld` for every glibc Linux; `generated/linker-probe-{arm64,amd64}.txt` |
| 1c | `🔵️.ps1` BOM-less under Windows PowerShell 5.1 | LANDED + ORACLE | `generated/ps-lint.txt` (PowerShell 7.6.6 + PSScriptAnalyzer 1.24): HEAD script → `PSUseBOMForUnicodeEncodedFile` + **4 parse errors** when decoded as CP1252 (what 5.1 does without BOM); new script BOM, 0 parse errors, 0 analyzer findings incl. `PSUseCompatibleSyntax` 5.1/7.0 |
| 1d | `🔵️.ps1` overwrites user-global `~/.cargo/config.toml` | LANDED | block deleted; `rustup toolchain install` (both wasm targets from `rust-toolchain.toml`) replaces the one-target `target add` |
| 1e | Unguarded Linux `apt-get` lines | LANDED + PROVEN in container | `🐚️.sh` `NativeToolchain` region (apt-get/dnf/pacman/zypper), every install fail-soft + scoped message; container toolchain phase green 3× (135 s cold, 45–57 s warm) |
| 1f | Hub single-PID child kills → process-tree kill (POSIX + Windows) | LANDED + TESTED | `terminateOwnedChildTree` (library `🏃️process`) at 9 hub sites + local-bootstrap; law `🪓️process-tree-termination` 5/5 ×3, nx target green; hub `🧱️foundation-source` law 12/12 after the edit |
| 1g | P2s | LANDED / NOTED | wasip2 via `rustup toolchain install`; go.work pin, Neo4j apt-only, devcontainer `bash` hooks: §5 |
| 1h | NEW P0s found by the fresh-clone Linux run | 7 LANDED; B1–B4 gated on W2's Hub Handoff (§2) | §2: `bun install` "Workspace not found" (`🕸️bindings`), `🤖️generated/🎚️ui-axes` missing for every script, styling generator cycle through `🤖️generated/🔤️tokens`, `**dist*` ignore swallowing source (`🚚️distribution`), Node missing for Nx tooling, `workspace:setup` racing 11 script tasks against `deps-javascript`, `repo-mcp:build` missing its `framework-schema:generate` edge |
| 2 | Linux proof in a plain container | **GREEN end to end with B1–B3 seeded and B4 patched in the copy**: `🐚️.sh setup` → `workspace:setup` 27/27 tasks (cold ≈ 19 min, warm 45 s), `bun install`, nx graph 1016 projects, `cargo check -p semio-hub` 3 m 40 s, `cargo build --bin os-hub` 8 m 27 s, os-mcp binary 4 m 15 s. Without them: 22/26 setup tasks (B1–B4 below) | §2, `generated/linux-fresh-e2e-{2..8}.txt` |
| 3 | Devcontainer zero-touch timed end to end | ON HOLD (coordinator); host blocker measured and B4 also breaks the devcontainer, §7 | |
| 4 | Windows static correctness + tests; unproven list | LANDED (laws) / list §6 | |

## 1. Landed source changes

| File | Change |
|---|---|
| `.cargo/config.toml` | `[target.x86_64/aarch64-unknown-linux-gnu] -fuse-ld=mold` → one `[target.'cfg(all(target_os = "linux", target_env = "gnu"))']` `-Z threads=8 -C linker-features=+lld -C link-self-contained=+linker -Z unstable-options` (the toolchain's own `rust-lld`; also restores `-Z threads=8`, which target rustflags silently replaced on Linux) |
| `.devcontainer/Dockerfile` | `mold` removed from the apt layer |
| `🧰️…/🔩️native/🥾️bootstrap/🐚️.sh` | `NativeToolchain` region: `run_privileged`, `linux_package_manager` (apt-get, dnf, pacman, zypper), per-manager packages, `report_missing_tools`, `ensure_node` (pinned `package.json` engines.node, sha256 vs `SHASUMS256.txt`), `ensure_rust_toolchain` (rustup-init `--default-toolchain none` + `rustup toolchain install`), `ensure_go` (latest stable, sha256), `ensure_dotnet` (dotnet-install.sh channel 8.0 = the only `TargetFramework`), all fail-soft; global `git config --global --add safe.directory` removed; `bun install` dropped (workspace:setup's `deps-javascript` owns the frozen install); repo-client build after setup; stale `⌨️script.sh` text fixed; dead apt-only Java helpers and the pre-install `cpp setup` call removed |
| `🧰️…/🔩️native/🥾️bootstrap/🔵️.ps1` | UTF-8 BOM; UTF-8 console encodings; `Test-WindowsLongPathsEnabled` + `Write-WindowsLongPathsGuidance` (`[en]`/`[de]`, exact elevated command, start + end); user-global cargo config write deleted; `rustup toolchain install`; global safe.directory writes deleted; `OpenJS.NodeJS.LTS` winget; `bun install` dropped, repo-client after setup |
| `🧰️…/💻️os/🔨️modules/🧬️semio/🖥️associations/🪟️windows/🔵️.ps1` | UTF-8 BOM |
| `📜️script.ts` | `REPO_LOCAL_GIT_CONFIG` (`core.symlinks`, `core.longpaths`) written by `setup git` |
| `🧰️…/📚️library/🏃️process/🟦️.ts` (+ `🟦️.js` twin) | `terminateOwnedChildTree(child)` |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | 9 child kills → `terminateOwnedChildTree`; `:9346` is a fixture child killing itself on purpose and stays |
| `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts` | failed-start kill → tree kill; `finishLocalHub` keeps SIGTERM grace, then tree-kills a hub that ignored it (leaked before) |
| `🧰️…/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts` + `🌱️sources/🔣️.json` (+ `🛂️schema`) | `publishBootstrapSources`: every `bun nx` first publishes the declared fresh-clone sources through their owners' system-only publishers (lazy `require`, Nx bootstrap eager closure unchanged — law `testNxBootstrap` PASS) |
| `🧰️framework/🔨️modules/🖱️ui/🎚️axes/🌱️bootstrap/🟦️.ts` | `bootstrapUiAxes`: publishes missing UI axis projections (`@semio-tech/framework` value-imports them, so every script died on a fresh clone) |
| `🧰️…/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🟦️.ts` | `bootstrapFlowCorePackage`: the static `@semio-tech/flow-core` manifest (root workspace member) |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🏛️model/🟦️.ts` (new) + `🌓️theme/🟦️.ts` + `📽️projection/🟦️.ts` | theme model split out dependency-free; the token generator imports the model, not the barrel that imports the tokens it generates (its own docstring demanded this; `check-generated` fresh) |
| `.gitignore` | `**dist*` (+2 re-includes) → `📤️dist`, `📤️distribution`: the glob silently ignored 34 source files (the whole `🧑‍💻dev/🚚️distribution` module the plugin registry imports, a hub admin test, two print `.sty`, a repo-lib test); simulated over 22 982 `*dist*` paths: 0 newly ignored |
| `package.json` | `engines.node: "24.15.0"` — the single Node pin (== devcontainer `ARG NODE_VERSION`) |
| `⚡️caching/🚀️bootstrap/📜️script.ts` (`NxScript`) | on a fresh clone (no `node_modules/nx`) runs `workspace:deps-javascript` to completion before the requested graph (`NxScript.javascriptEnvironment`); cancellable through the same launch/stop path; the bootstrap-source publish moved to the entrypoint so the class keeps its injected-dependency contract (cache law `testNxCoordinator` PASS) |
| `🧰️…/💻️client/🔌️mcp/📋️project.json` | `repo-mcp:build` `dependsOn` `@semio-tech/framework-schema:generate` — its Go package imports the generated, gitignored `⌨️cli/🏷️entity-kinds/🐹️.go` (`repo-client:build` already had this edge) |
| `README.md` | Windows: `git clone -c core.longpaths=true`, BOM-safe `powershell -File …🔵️.ps1 setup`, short clone root; Unix: `bash …🐚️.sh setup`; bootstrap sources |
| tests | `📚️library/🧪️tests/🥾️cross-platform-bootstrap` (12 laws; 11/12 green, the red one is §4's staged ticket data — `generated/test-cross-platform-8.txt`) + fixture + schema; `📚️library/🧪️tests/🪓️process-tree-termination` + fixture + schema; repo-lib `test cross-platform-bootstrap` / `test process-tree-termination`, nx targets, launch rows `📦️test🥾️bootstrap🪟️cross-platform` (206.176), `📦️test🏃️process🪓️tree-termination` (206.177) in seed + `launch.json` (hand-inserted identically; W2 owns `plugin-registry:generate`) |

Checks run: cache laws `testNxCoordinator`, `testDependencyBootstrap`, `testNxBootstrap` PASS after the wrapper change
(`testWatcherReadiness`, `testWorkspaceWatchIgnores` fail on Nx daemon/native-watcher environment assertions in code I did
not touch — `generated/cache-laws-2.txt`; not re-verified at HEAD); `bash -n` (bash 3.2 + 5), `bun build --no-bundle` on every edited TS file, targeted `tsc` (only pre-existing
Bun-global/`import.meta.dir` noise; no new errors), `styling check-generated` fresh, hub `🧱️foundation-source` 12/12,
cache `testNxBootstrap` PASS, path-emoji statutes on all 10 new directories: 0 findings.

## 2. Linux proof (plain `ubuntu:24.04`, aarch64, `--cpus 4 --memory 7g`)

The tree is streamed into a Docker volume (Docker Desktop cannot bind `~/Documents` on this host): tracked +
untracked-not-ignored files, tickets excluded, submodule empty and `🕸️bindings` absent, exactly as a fresh clone.

| Run | Result | Capture |
|---|---|---|
| toolchain (`🐚️.sh setup`, repo bootstrap skipped) | green 135 s cold: apt toolchain, Bun 1.3.14, rustup + nightly-2026-07-07, Go 1.27.1, .NET 8.0.425, uv 0.12 | `linux-ubuntu-p1.txt` |
| `bun install --frozen-lockfile` | **red**: `Workspace not found "…/🕸️bindings"` → fixed (bootstrap source) → green 48 s | `p1`, `p2` |
| `bun nx graph` | green 102 s, 1016 projects / 18 433 edges | `p2` |
| `cargo check -p semio-hub` | red: TLS CA missing in the image (fixed by the toolchain's `ca-certificates`), then **red** on missing `🤖️generated` Rust sources (`🔤️tokens/🦀️.rs`, `🌐️locale/🤖️generated/🦀️.rs`) — `prepare` must run first | `p3`, `p5` |
| `workspace:prepare` | **red**: every generator died on `Cannot find module './🤖️generated/🎚️ui-axes/🟦️.ts'` (fresh-clone chicken-and-egg), styling also on `🤖️generated/🔤️tokens` | `p5` |
| full `🐚️.sh setup` | **red**: Nx 23.2.0 tooling crashed under Bun (`Cannot find module '../analytics'`) — no Node on the machine → Node pin + provisioning added | `linux-fresh-e2e.txt` (1st) |
| full `🐚️.sh setup` (fresh copy, Node fixed) | toolchain green; `workspace:setup` **red after 15 m 24 s**: 9/26 tasks green (deps-javascript, cargo, go, dotnet, wasm-opt, trunk, wasm, tools, …); 11 failed because they started before `bun install` finished (all generators, `generator-inputs`, `energy-oracle-py:deps`, `cpp-setup`, `deps-browsers`) or on a missing edge (`repo-mcp:build`: `undefined: EntityKindCatalog`); `setup-git` failed only because this copy had no `.git` (harness gap, fixed: `clone` phase) | `linux-fresh-e2e.txt` |
| full run again: `clone` (git only) → `🐚️.sh setup` → check/build/link, fresh volumes, all fixes | `clone` 59 s; `🐚️.sh setup` 19 m 7 s: **22/26 setup tasks green** (all `deps-*` except cargo, `setup-git`, `cpp-setup`, `repo-mcp:build`, schema/ui-rs/styling/generator-inputs generators); red: `deps-cargo` + `os-mcp-rs:build` (`--locked` — a peer's manifest edit was captured before its `Cargo.lock` update; host lock is in sync again, `cargo metadata --locked` exit 0), `framework-graph:generate` (B2/B3), `assets:build` (B1); `cargo check -p semio-hub` red on **winit: "platform not supported"** (B4) | `linux-fresh-e2e-2.txt` |
| run 3: copy re-synced; B1–B3 outputs seeded from this host (so the rest of the chain is observable); B4 patch applied **in the container copy only** | `🐚️.sh setup` warm 87 s: **`workspace:prepare` green incl. `plugin-registry:generate`**; only `deps-cargo` + `os-mcp-rs:build` red (the B4 patch needs new lock entries; `--locked`); **`cargo check -p semio-hub` GREEN in 3 m 40 s** on native aarch64 Linux with rust-lld (cargo added the winit backend lock entries: `smithay-client-toolkit`, `x11-dl`, …); **`cargo build -p semio-hub --bin os-hub` GREEN in 8 m 27 s** (287 MB, `.comment` = LLD 22.1.8) | `linux-fresh-e2e-3.txt` |
| runs 4–8 | link timing (§3); os-mcp binary standalone green 4 m 15 s; `workspace:setup` **27/27 green** twice (under strace 247 s, plain warm 45 s) | `-4` … `-8` |

**Open observation (not root-caused):** in three cold runs the `@semio-tech/framework-os-mcp-rs:build` cargo process ended
~20–40 s into compiling with no compiler error, and the whole `nx run workspace:setup` exited 130 (SIGINT). The same build is
green standalone and inside `workspace:setup` once warm; a traced run (`strace -f -e kill,tgkill,tkill,execve`) went green with
zero SIGINT sends, and the Docker VM kernel log shows no OOM kill. It coincides with sibling tasks finishing
(`deps-javascript`, `deps-browsers`); treat as a timing-dependent cold-start flake until reproduced with the sender captured.

**Fresh-clone blockers left for their owners** (each measured in run 2; none can be fixed while W2 publishes without touching
`🔣️taxonomy.json` / `Cargo.toml` + `Cargo.lock`, which feed its generator inputs):

- **B1 `assets:build`** — `external-emoji-shortcodes` (taxonomy `generatorContracts`) is an external input with no producer and
  `inclusion: ignored` at `🖼️assets/🔣️icons/🤖️generated/🔣️shortcodes.json`; a clone never has it (“missing external
  🔣️shortcodes.json snapshot”). Fix: keep the pinned snapshot as a tracked input outside `🤖️generated` (contract path,
  `🔣️icons/🏗️builder/📽️projection/🟦️.ts:338`, `♾️infinite` project input).
- **B2 `framework-graph:generate`** (and every `loadTaxonomy()` caller) — contract `wgpu-frame-worker` declares
  `🎞️frame-worker/🤖️generated/🟨️.js` `inclusion: "tracked"`, but `.gitignore` `**/🤖️generated/` ignores it and it is not in
  git; `validateGeneratorContractsAgainstWorkspace` throws “tracked output … is missing”. The `indexed-generated-output` law
  forbids tracking `🤖️generated`, so the contract must say `ignored` and `prepare` must run `framework-os:generate-wgpu`.
- **B3** — same for `scale-fixture` output root `🧫️fixtures/⚖️scale/🤖️generated` (ignored by `**/🧫️fixtures/**/🤖️generated/**`).
- **B4 native Linux Rust** — `winit` is declared `default-features = false, features = ["rwh_06"]` in `🖱️ui`, `🖱️ui/🖥️host` and
  the wgpu target crate; on Linux that is no backend, so winit's `compile_error!` stops every crate that enables
  `semio-framework-ui/wgpu-engine` (semio-hub included) — natively **and in the devcontainer**. Patch ready:
  `wp-z2/pending/winit-linux-backends.py` (Linux-only target table, runtime-loaded `x11` + `wayland` + `wayland-dlopen`, no
  system dev packages; adds lock entries such as `x11-dl`, `smithay-client-toolkit`). Proven by run 3. Land after W2's Hub Handoff.

## 3. Linker decision

Probe (`generated/linker-probe-*.txt`, pinned `nightly-2026-07-07`): `lib/rustlib/<host>/bin/gcc-ld/ld.lld` ships in the
toolchain on both hosts. x86_64 already links with LLD 22.1.8 by default; aarch64 defaults to GNU ld; `-C linker-features=+lld
-C link-self-contained=+linker -Z unstable-options` gives LLD on both (`-Z linker-features` no longer exists).

Measured link of the real `os-hub` debug binary (287 MB, aarch64 Linux container, `--cpus 4`, fleet load on the host): the
exact command rustc runs (`--print link-args -C save-temps`, Rust's `\u{fe0f}` path escapes decoded) re-executed three times
per linker, only `-fuse-ld` swapped (`generated/linux-fresh-e2e-5.txt`):

| linker | run 1 | run 2 | run 3 | median |
|---|---|---|---|---|
| rust-lld 22.1.8 (toolchain) | 3.38 s | 6.62 s | 7.86 s | **6.6 s** |
| mold 2.30.0 (distro) | 2.58 s | 3.49 s | 3.30 s | **3.3 s** |
| GNU ld 2.42 (distro default on aarch64) | 45.39 s | 37.80 s | 32.29 s | **37.8 s** |

Decision stands on root cause: the P0 was a hard dependency on a package only apt systems received. rust-lld ships inside the
pinned toolchain for every host, so no distribution, release or package manager can break linking again; it costs ~3 s
over mold on the largest binary and saves ~31 s over the GNU ld aarch64 would otherwise use.

## 4. Findings for other slices / the coordinator

- **C10 / W2:** hub data directories are staged in git under this ticket (`wp-c10/catalog-a-seed/trusted-catalog/validation/*/
  candidate-data/artifact-cas/v1/<64hex>/…`, 93 paths up to 310 UTF-16 units; `wp-w2/hub-7800/**`, 8 paths). They are tool
  output (AGENTS.md: delete), and a Windows clone cannot check them out. HEAD itself is inside the budget (max 239, 0 over,
  `generated/path-budget-head.txt`); the path law fails only on these staged paths.
- **Index hygiene (needs someone allowed to run git):** while `.gitignore` briefly re-included `**/🕸️bindings/package.json`
  (reverted within minutes in favour of the bootstrap source), something staged
  `🧰️…/🌊️flow/🫀️core/🕸️bindings/package.json` (`A` in the index; I ran no git write). It is ignored again and would trip the
  `indexed-generated-output` law if committed: `git restore --staged` it.
- The `**dist*` glob also ignored untracked peer work: `🧑‍💻dev/🚚️distribution/**` (25 files) now shows as untracked source —
  its owner should confirm it is meant to be committed.

## 5. P2 notes

- `go.work` `go 1.25` vs devcontainer Go 1.26 / native latest (1.27.1): `go` is a minimum and toolchain switching covers it; kept.
- Neo4j Desktop AppImage stays apt/x86_64-only with its manual-install message (optional; `SKIP_NEO4J_DESKTOP`).
- Native Linux does not receive Playwright's browser system libraries (`deps-browsers` stays green but prints Playwright's
  host-validation warning in the container); only headless probes need them, the devcontainer image has them.
- Devcontainer `postStartCommand`/`postAttachCommand` use `bash` inside the fixed image: correct as is.
- Still user-global and deliberately untouched (outside the audit, product decisions): `🐚️.sh` appends the `NEO4J_*` block to
  shell profiles; `🔵️.ps1` sets user env vars (`EDITOR`, `NEO4J_*`, `VCPKG_ROOT`, …) and `bun add --global` CLIs.

## 6. Windows: statically covered vs. unproven

No Windows machine is reachable from this host; nothing below was executed on Windows.

| Path | Static status | Covered by |
|---|---|---|
| Clone | tracked tree max 239 UTF-16 units (HEAD); stock Git for Windows checks out below a clone root ≤ 20 units; README clones with `-c core.longpaths=true`, `setup git` keeps it repo-local | law `keeps every tracked path inside the stock Windows MAX_PATH budget` (iconv-lite UTF-16 oracle), law `writes only repo-local git configuration` |
| `🔵️.ps1` under Windows PowerShell 5.1 | UTF-8 BOM, UTF-8 console, parses clean; the HEAD file fails to parse when decoded the 5.1 way | law `saves every non-ASCII PowerShell script with a UTF-8 BOM` + `generated/ps-lint.txt` (pwsh 7.6.6 parser, PSScriptAnalyzer `PSUseCompatibleSyntax` 5.1) |
| `LongPathsEnabled` | detected, en + de guidance with the exact elevated command, never changed silently | law `guides Windows long paths in every supported language` |
| User-global config | no `~/.cargo/config.toml`, no global git config | laws (forbidden fragments) |
| Rust targets | `rustup toolchain install` installs both wasm targets from `rust-toolchain.toml` | law (required fragment) |
| Node for Nx | `OpenJS.NodeJS.LTS` via winget — the current 24.x LTS, not byte-for-byte the `engines.node` pin the Unix script installs | law `pins one Node.js …` (fragment only) |
| Child-process trees | `terminateOwnedProcessTree` → `taskkill /T /F` on win32 | law `🪓️process-tree-termination` runs the Windows branch only on Windows (unexecuted here) |
| `setup git` aliases | `symlinkSync` → `linkSync` fallback on win32 (pre-existing) | law exercises the POSIX branch only |
| Bootstrap sources | `join(root, module)` normalizes separators; publishers are `node:`-only | laws `publishes bootstrap sources …`, `loads every setup-path script …` |

**Unproven on real Windows:** the whole winget chain; `bun` resolving emoji paths and `powershell.exe` passing an emoji
`-File` argument; Git for Windows checkout of emoji + ZWJ names; Cargo/MSVC with the shared build-dir — the deepest build
path measured here is **246 UTF-16 units below the clone root** (`generated/cargo-build-dir-depth.txt`), so MSVC tools that
are not long-path aware only fit below a ≤ 13-unit clone root (`C:\src\semio`); `node_modules` reaches 239
(`generated/node-modules-depth.txt`, a Playwright macOS bundle); Docker Desktop devcontainer on a Windows host.

## 7. Devcontainer (item 3) — prepared, not run

- On hold until the coordinator's go.
- B4 (winit without a Linux backend) stops `cargo` for semio-hub / os-mcp inside the devcontainer exactly as on native Linux, so the timed run should follow the B4 landing; B1–B3 likewise fail `workspace:setup` in any fresh container.
- **Host blocker, measured:** Docker Desktop cannot read `~/Documents` on this Mac (`mkdir /host_mnt/Users/ueli/Documents:
  operation not permitted`, also for `ls` through a `/Users/ueli` mount) — macOS privacy permission for Docker Desktop. The
  devcontainer bind-mounts `..`, so it cannot open from the repo's current location. Without the user granting that
  permission (a system setting I must not change), the run uses a clean copy under `/Users/ueli/` with a distinct basename
  (fresh `${localWorkspaceFolderBasename}-*` volumes) and `@devcontainers/cli up` in place of "Reopen in Container".

## 8. Next, gated

1. After W2's Hub Handoff shows the new catalog: land `wp-z2/pending/winit-linux-backends.py` (Cargo.toml ×3 + lock entries),
   compile-atomic `cargo check -p semio-hub` on macOS; owners (or I, if assigned) fix B1–B3 in `🔣️taxonomy.json` / assets.
2. On the coordinator's go: devcontainer timed run (§7) from a clean copy outside `~/Documents`, ports 8100–8109 / 6600–6609.
3. Docker volumes `z2-src-fresh` (17.7 GB) and `z2-home-fresh` (7.4 GB) are kept for a re-run after (1); removed when Z2 closes.

