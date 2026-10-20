# 🔎️ Direct External Tool Census (audit 1021a)

READY: yes (read-only audit, no repo files changed, no builds, no `bun repo`, no git-modifying commands)

## Method and limits
- Tracked files only (`git grep -I` and `git ls-files`, under `nice -n 10`), excluding `🎫️tickets`, `node_modules`, `.cache`, lockfiles.
- Three passes: (1) function-call sites whose first string argument is a tool (`Command::new`, `spawnSync`, `run(`, `runCmd(`, `exec.Command`, ...), 2,925 lines; (2) manifests (1,794 tracked `package.json` and `project.json`, parsed as JSON); (3) config, CI, devcontainer, bootstrap scripts, README and agent docs by pattern.
- "Prod" excludes test paths (`🧪️`, `tests/`, `_test.go`, fixtures). "Adapter" is a path heuristic: `🏃️process/` (`🍞️bun/`, `🦀️cargo/`, owned-execution), `🦀️cargo/` under `⚡️caching`, `🔌️nx/`, `🏗️bazel/`, `🔧️tools/`, `🔒️git/` provider, `🍎️image/` (hdiutil), `🚀️bootstrap/📦️dependencies/🏗️native/`, `🐳️service/`. Adapter counts include tests inside adapter dirs.
- Corrections: 50 `node` hits in `.rs` are JSON/GraphQL keys (`["node"]`), not Node.js; excluded from the real count below.
- Not verified: runtime behavior (no run was made); untracked files are not searched; `git grep` warned about a symlink loop on `.gitattributes`.

## Census per tool (call sites outside manifests)
| Tool | Prod (real) | Adapter dirs | Test | Notes |
|---|---|---|---|---|
| bun | ~101 (90 ts, 7 rs, 4 go) | 19 | 585 | Launch dependency, see below |
| cargo | ~90 (81 ts, 9 mjs) | 91 | 89 | `.cargo/config.toml` rustc wrapper |
| git | ~137 (73 ts, 56 go, 8 rs) | 35 | 235 | Go `🔒️git` provider exists, bypassed |
| gh | 34 (all Go) | 0 | 2 | No adapter at all |
| playwright | 28 (ts) | 0 | 122 | os parity probes, no adapter |
| node | ~22 | 146 | 304 | Real count after removing JSON-key false positives |
| python/python3 | ~17 | 64 | 243 | Mostly plugin generators |
| uv | ~18 | 3 | 26 | |
| nx | 10 | 8 | 32 | `nx.json` runtime probes (5 direct) |
| bazel | 2 | 6 | 16 | |
| vite | 8 | 4 | 17 | |
| rustc | 7 | 21 | 72 | |
| dotnet | 6 | 6 | 8 | |
| rustup | 4 | 8 | 0 | |
| go | ~4 | 16 | 140 | |
| docker | 2 (course deploy) | 2 | 2 | |
| npx | 2 (Go) | 0 | 0 | |
| hdiutil | 0 | 6 | 1 | adapter-only, good |
| jco, wasm-opt | 1 each | 1 | 1 | |
| tectonic | 0 | 2 | 0 | adapter-only, good |
| tsc, vitest | 0 / 1 (string in planner) | 0 | 30 | |

Manifests (1,794 files): 4,603 `bun` tokens. Every command routes through `bun ./📜️script.ts <cmd>` or `bun "<path>"`. Non-bun tools appear only as arguments to routed commands (`bun … native cargo check`, `bun … sync go|dotnet|python|cargo`, `bun … setup git`). Direct non-bun invocations in manifests: 0. Exception to note: `♻️mit-bestand` uses `bun --cwd` in 706 manifest tokens.

Config and CI:
- `.github/workflows/build-system.yml`: `bun repo …` and `bun "$CI_REPORT"` (about 25 calls); direct `git` 6 (`config --system`, `rev-parse`); `bazel` and `nx` appear only as `--tool` values.
- `.github/workflows/architecture-course.yml`: `bun repo run repo-mcp:build` under sudo (3).
- `.devcontainer/Dockerfile` (lines 92-97): `ln -s bun bunx`, `bun --version`, `node --version` checks. Install-time only.
- Bootstrap `🐚️.sh` and `🔵️.ps1`: direct `rustup` (5/4), `uv` (5/4), `git` (6/6), `node` (2/1), `dotnet` (2/0), `go` (1/2), `gh` (0/1), `playwright` (0/1). Install-time, hardwired, but versions are pinned in `🧬schema/🔣️.json`.
- `nx.json` lines 15-55: runtime probes `node -p`, `bun --version`, `go`, `uv`, `dotnet` (5).
- `.cargo/config.toml`: `rustc-wrapper = "bun"` and `rustc = <script.ts>`. Every cargo build depends on bun.

Docs:
- `README.md`: no direct tool dev instructions except `git clone` (line 714). 38 lines use `repo …`; 2 use `bun repo` or `bun ./`.
- `.cursor` (429 tracked files), `.agents`, `.codex`, `.claude`: 219 files, 508 direct-tool command lines (cargo, npx, nx, uv, go, docker and more). Mostly plans and skills, not runtime.

## Does the repo binary need bun? Yes.
- Native `repo` launcher is Go (`🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/`). `🚪️entrypoint_unix.go` runs `exec.LookPath("bun")` then `syscall.Exec`. `🚪️entrypoint_windows.go` runs `exec.Command("bun", …)`. Routing is in `🐹️.go` at lines 205 and 210 (`nativeBunBuildTool`, `nativeRootScriptTool`). The Rust `repo-cli` also runs `Command::new("bun")` (`🧭️repo-cli/🚪️entrypoint/🦀️.rs:24`).
- About 46 Go-native cobra subcommands (ticket, goal, todo, loc, mermaid, ...) should work without bun. Not verified at runtime.
- Every build, run, setup, verify, dev and dependency-sync verb needs bun.
- Install: `🐚️.sh:517` runs `bun repo run repo-mcp:install`, so bun is required even to install the launcher. Bootstrap installs a pinned bun.
- Starting it: the README and agents use `repo …` (native, after bootstrap). CI and the root `package.json` `repo` script use `bun repo …` / `bun ./📜️script.ts`. The CLI-level "NativeScriptTool" port exists (`🐹️.go`), but its only implementation is bun.

## Adapter status
- Adapters that exist: bun `📚️library/🏃️process/🍞️bun/`, cargo `🏃️process/🦀️cargo/` and `⚡️caching/🦀️cargo/`, nx `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/`, bazel `🏗️build/🔧️tools/🏗️bazel/`, build-tool storage `🏗️build/🔧️tools/💾️storage/`, git provider `🧩️providers/📦️packages/🐹️go/🔒️git/`, hdiutil `🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/`, docker `🏗️build/🏃️execution/🐳️service/`, bootstrap `📦️dependencies/🏗️native/`.
- No adapter: gh, playwright, vite, rustc direct, dotnet builder, most plugin generators.

## Top offenders (prod, outside adapters)
- bun: `📚️library/🎛️commands/✅️verify/🟦️.ts` (20), `✅️verify/🏁️gate-lanes/🟦️.ts` (12), `✏️s/🔌️plugins/🌀️procedural/…/generation3d/…/📜️script.ts` (5), `🧰️framework/🔨️modules/🗺️surface/…/📜️script.ts` (4), plus about 14 plugin generators `✏️s/🔌️plugins/🗄️stdio/…/🏭️generator/🟦️.ts` (`spawnSync("bun", [probes…])`).
- cargo: `✏️s/🔌️plugins/🗄️stdio/…/🏭️generator/🟦️.ts` (48), `🏗️build/📥️inputs/🟨️.mjs` (9), `os/🔨️modules/🌉️mcp/…/📜️script.ts` (5).
- git: `⌨️cli/🔒️git-devcontainer-providers/🐹️.go` (15), `🏠️workspace/…/🌿️git/🐹️.go` (7), `⌨️cli/🛠️path-time-exec-utils/🐹️.go` (6), `📝️commit/🌳️git/🟦️.ts` (5).
- gh: `⌨️cli/🐙️github-issue-cli/🐹️.go` (17), `❄️goal-management/🐹️.go` (9), `🚀️ticket-open-create/🐹️.go` (5).
- playwright: `💻️os/…/🧑‍💻dev/⚖️parity/` (probe 9, execution 4, structure 3, pixels 3).
- python3: plugin generators `✏️s/🔌️plugins/🗄️stdio/…/🏭️generator/🟦️.ts` (13).
- uv: `💻️os/…/📺️renderer/…/World3dHost/…` (about 5), `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts` (3).
- nx: `🏗️build/🪞️parity/🏃️collect/🟦️.ts` (2), `🕸️dependencies/⚖️truth/🟦️.ts` (2).
- dotnet: `🎨️styling/🏗️builder/🔷️dotnet/📜️script.ts` (3).

## Ranking by breaks-swap-ability (worst first)
1. bun: launcher, install, cargo wrapper, all 1,794 manifests, CI, about 101 prod sites. Hard runtime dependency with a single implementation.
2. gh: 34 prod sites, no adapter.
3. cargo: about 90 prod sites plus `.cargo/config.toml` rustc override. Adapter exists but is bypassed.
4. git: about 137 prod sites (largest raw count). Provider interface exists but is bypassed in about 60 sites.
5. playwright: 28 prod sites, no adapter.
6. nx and bazel: about 12 prod sites, `nx.json` probes, CI `--tool` values.
7. python, uv, node, dotnet, go, rustup: about 60 prod sites combined, mostly generators and bootstrap.
8. vite, rustc, docker, npx, jco, wasm-opt: 1 to 8 sites each. hdiutil and tectonic are adapter-only.
