---
name: Bun Nx Monorepo Setup
overview: Migrate the polyglot monorepo to Bun as the single Node package manager + runtime, route every script (build/test/dev/lint/publish/setup) exclusively through Nx with full dependency caching across Bun, Cargo, Go, uv and dotnet, and reduce the devcontainer + native bootstrap to one zero-touch `bun nx run workspace:setup` entrypoint.
todos:
  - id: ticket
    content: Open ticket .repo/🎫/26/05/11/bun-nx-monorepo-setup via repo/client/client
    status: completed
  - id: bun-swap
    content: Swap npm/pnpm → Bun in root package.json, delete package-lock.json + .npmrc, add bunfig.toml, run bun install
    status: completed
  - id: scripts-rewrite
    content: Replace every npm/pnpm/npx call (root + 32 workspaces + devcontainer + CI + .vscode) with bun/bunx; replace inline `node -e` blobs with bun TS scripts under each scripts/ folder
    status: completed
  - id: drop-helpers
    content: Drop cross-env, tsx, jiti, concurrently from devDeps; verify nothing else needs them
    status: completed
  - id: project-jsons
    content: "Add/align project.json for all buildable units: 32 Node workspaces, compose/hub crate, 9 Go modules, 4 .NET test/benchmark csprojs"
    status: completed
  - id: workspace-project
    content: Add root `workspace` project.json with setup/lint/format/test/build/mcp-inspector/git-setup/depcruise targets
    status: completed
  - id: nx-json
    content: "Refine nx.json: cli.packageManager=bun, per-toolchain namedInputs (cargo/go/uv/dotnet), targetDefaults caching for setup/build/test/lint/publish, outputs per project"
    status: completed
  - id: devcontainer
    content: Shrink .devcontainer/post-create.sh, post-attach.sh, install-native.ps1 to OS prereqs + bun install + `bun nx run workspace:setup`; swap Node feature for Bun in Dockerfile
    status: completed
  - id: ci
    content: Update .github/workflows/playwright.yml + gh-pages.yml to oven-sh/setup-bun + bun nx affected/run; add lockfile-keyed Nx cache
    status: completed
  - id: verify
    content: "Cold bootstrap on Linux + Windows: rm caches, run `bun nx run workspace:setup`, then `bun nx run-many -t build/test`, confirm second run hits Nx cache everywhere"
    status: completed
  - id: close-ticket
    content: Close ticket via repo/client/client with summary + file list
    status: completed
isProject: false
---

# Bun + Nx Zero-Touch Monorepo

## Goals

- Bun is the only Node toolchain (no `npm`, no `pnpm`, no `npx`, no `tsx`, no `cross-env`).
- Nx is the only entrypoint for every script. Direct `cargo`, `go`, `dotnet`, `uv`, `vite`, `electron-forge`, `vitest`, `playwright`, `eslint`, `pre-commit`, `tsc`, etc. invocations live inside `project.json` targets only.
- Caching is configured per toolchain (Cargo.lock, go.sum, uv.lock, csproj/sln, bun.lock) so re-runs hit the local Nx cache.
- `.devcontainer/post-create.sh` and `.devcontainer/install-native.ps1` collapse to: install OS prereqs + `curl bun.sh | bash` + `bun install && bun nx run workspace:setup`. Identical behavior on devcontainer / native Linux / macOS / Windows.

## Ticket

Open via `repo/client/client(.exe) ticket open` under the most appropriate goal from `repo://goals`. Folder: `.repo/🎫/26/05/11/bun-nx-monorepo-setup/`. All scratch logs/scripts go there.

## 1. Package manager swap (npm/pnpm → Bun)

- [package.json](package.json): set `"packageManager": "bun@1.2.x"`, drop the `pnpm` block, drop `engines.node`, add `engines.bun`. Keep `workspaces` (Bun is compatible).
- Delete [package-lock.json](package-lock.json) and [.npmrc](.npmrc); add new `bunfig.toml` (registry, `install.linker = "isolated"` if hoisting bites Electron/Playwright).
- Replace every `npm` / `pnpm` / `npx` invocation with `bun` / `bunx` across:
  - root [package.json](package.json) scripts
  - 32 workspace `package.json` scripts (Grep already enumerated them)
  - [.devcontainer/post-create.sh](.devcontainer/post-create.sh), [post-attach.sh](.devcontainer/post-attach.sh), [post-start.sh](.devcontainer/post-start.sh), [install-native.ps1](.devcontainer/install-native.ps1)
  - [.github/workflows/playwright.yml](.github/workflows/playwright.yml), [gh-pages.yml](.github/workflows/gh-pages.yml)
  - [.vscode/launch.json](.vscode/launch.json) (the `mcpinspector` runtimeExecutable etc.)
- Drop `cross-env`, `tsx`, `jiti`, `concurrently` from root devDeps; Bun supplies cross-platform env handling, native TS execution, and Nx supplies parallelism via `run-many`.

## 2. Replace inline `node -e "..."` with `bun` TS scripts

Every workspace currently has a `dev`/`build`/`postinstall` script with a long `node -e "...spawn npx vite..."` blob (root `postinstall`, root `git:setup`, root `mcp:inspector*`, all sites/play, sites/docs, ui, algorithms, 3dm/ui, elements/ui, repo/client, repo/server build, compose/desktop, etc.). Replace each with a small file under that workspace's `scripts/` directory (e.g. [compose/sites/doc/scripts/dev.ts](compose/sites/doc/scripts/dev.ts)) and call it via `bun scripts/dev.ts` from the matching Nx target.

A single shared [scripts/run-vite-dev.ts](scripts/run-vite-dev.ts) at repo root handles the common pattern (host = `0.0.0.0` in devcontainer else `127.0.0.1`, optional polling, configurable port) so each ui workspace becomes a one-liner.

## 3. Nx project coverage

Add or update `project.json` for every buildable unit. Each project gets the same target vocabulary:

- `setup` — fetch/install deps for that toolchain (idempotent, cacheable on lockfile inputs)
- `build`, `test`, `lint`, `dev`, `publish`, `clean` (where applicable)

Coverage matrix:

- Already covered (align target naming + caching): all 32 Node workspaces and the 9 schema `project.json` files.
- Missing — Cargo crates without `package.json`: `compose/hub` (add minimal `project.json` invoking `cargo build -p compose-hub` etc.).
- Missing — Go modules without `package.json`: `repo/mcp`, `repo/cursor`, `repo/copilot`, `repo/codex`, `repo/claude`, `repo/kiro`, `repo/go`, `coda/blnbo/go`, `coda/programming/go` (add `project.json` invoking `go build`/`go test`).
- Missing — .NET projects without `package.json`: `compose/net/Compose.Tests`, `compose/net/Compose.Benchmark`, `compose/gh/Compose.Grasshopper.Tests`, `compose/3dm/Compose.Rhino.Tests` (add `project.json` invoking `dotnet build/test`).

All targets implement via `nx:run-commands` (no third-party Nx plugin churn). Keep `@nxlv/python` for `compose/py` + `compose/engine` since it already provides `update`/`lock` executors.

## 4. Root `workspace` project (orchestration)

Add [project.json](project.json) at the repo root named `workspace` with these targets:

- `setup` — `dependsOn: ["^setup"]`, runs Bun install + every project's `setup` (cargo fetch via `cargo metadata`, go mod download, uv sync `--all-packages --all-groups`, dotnet restore Monorepo.sln, playwright install, electron-sandbox chmod on Linux, `repo/client/client configure` for git hooks, build VSCode extension once).
- `lint`, `format`, `test`, `build` — `nx run-many -t <target>` across every project (with caching).
- `mcp-inspector`, `mcp-inspector-repo`, `git-setup`, `pre-commit`, `depcruise` — replace today's loose root `package.json` scripts.

After this, root [package.json](package.json) `scripts` collapses to a single `"setup": "bun nx run workspace:setup"` convenience alias (everything else is `bun nx run <project>:<target>`).

## 5. `nx.json` — caching done right

Refine [nx.json](nx.json):

```json
{
  "cli": { "packageManager": "bun" },
  "namedInputs": {
    "default": ["{projectRoot}/**/*", "sharedGlobals"],
    "sharedGlobals": [
      "{workspaceRoot}/package.json",
      "{workspaceRoot}/bun.lock",
      "{workspaceRoot}/bunfig.toml",
      "{workspaceRoot}/nx.json",
      "{workspaceRoot}/tsconfig*.json",
      "{workspaceRoot}/Cargo.toml",
      "{workspaceRoot}/Cargo.lock",
      "{workspaceRoot}/rustfmt.toml",
      "{workspaceRoot}/go.work",
      "{workspaceRoot}/go.work.sum",
      "{workspaceRoot}/pyproject.toml",
      "{workspaceRoot}/uv.lock",
      "{workspaceRoot}/Monorepo.sln",
      "{workspaceRoot}/eslint.config.mjs",
      "{workspaceRoot}/.dependency-cruiser.cjs"
    ],
    "production": [
      "default",
      "!{projectRoot}/**/?(*.)+(spec|test|e2e|integration|stories|story|mdx).[cmjtshx]?([sx])?(.snap)?",
      "!{projectRoot}/.storybook/**"
    ],
    "cargo": ["{projectRoot}/**/*.rs", "{projectRoot}/Cargo.toml", "{workspaceRoot}/Cargo.lock"],
    "go": ["{projectRoot}/**/*.go", "{projectRoot}/go.mod", "{projectRoot}/go.sum", "{workspaceRoot}/go.work*"],
    "uv": ["{projectRoot}/**/*.py", "{projectRoot}/pyproject.toml", "{workspaceRoot}/uv.lock"],
    "dotnet": ["{projectRoot}/**/*.cs", "{projectRoot}/*.csproj", "{workspaceRoot}/Monorepo.sln"]
  },
  "targetDefaults": {
    "setup":   { "cache": true, "inputs": ["sharedGlobals"], "outputs": ["{workspaceRoot}/node_modules", "{projectRoot}/.venv", "{projectRoot}/target", "{projectRoot}/obj", "{projectRoot}/bin"] },
    "build":   { "cache": true, "dependsOn": ["^build", "setup"], "inputs": ["production", "^production"] },
    "test":    { "cache": true, "dependsOn": ["^test"],  "inputs": ["default", "^default"] },
    "lint":    { "cache": true, "dependsOn": ["^lint"],  "inputs": ["default", "^default"] },
    "publish": { "cache": true, "dependsOn": ["^publish", "build"] },
    "dev":     { "cache": false },
    "clean":   { "cache": false }
  },
  "plugins": [{ "plugin": "@nxlv/python", "options": {} }]
}
```

Per-project `project.json` overrides `inputs`/`outputs` to the right toolchain (e.g. Rust crates use `["cargo", "^cargo"]` and emit `{projectRoot}/target/release/...`; Go uses `["go", "^go"]` and emits `{projectRoot}/<binary>(.exe)`).

`defaultBase` stays `⛳wip`. The existing `build-wasm` target stays special-cased.

## 6. Devcontainer + native bootstrap (zero-touch)

[.devcontainer/devcontainer.json](.devcontainer/devcontainer.json):
- Replace `ghcr.io/devcontainers/features/node:1` with Bun install in [.devcontainer/Dockerfile](.devcontainer/Dockerfile) (`curl -fsSL https://bun.sh/install | bash` + symlink). Keep the Node feature only if Electron explicitly needs system Node — verify with a smoke build; if `bun --bun` runs `electron-forge`, drop Node entirely.
- Keep all other features (Go, Python, uv, ruff, dotnet, Rust w/ wasm32 target, git, gh, sqlite, nx).
- Update the `node_modules` mount label if naming changes (keep volume name).

[.devcontainer/post-create.sh](.devcontainer/post-create.sh) shrinks to:

```bash
#!/bin/bash
set -e
WORKSPACE="${containerWorkspaceFolder:-/workspaces/semio}"
sudo apt-get update && sudo apt-get install -y ripgrep jq unzip
configure_emoji_fonts
git config --global --add safe.directory "$WORKSPACE"
curl -fsSL https://bun.sh/install | bash
export PATH="$HOME/.bun/bin:$PATH"
bun install
bun nx run workspace:setup
```

Per-toolchain build/install steps (uv sync, cargo wasm config, dotnet restore, go build, playwright install, vscode extension build, git hooks, antigravity MCP config, GitKraken install) all move into Nx targets:
- Cargo wasm config writing → `compose/rs:setup` (writes `.cargo/config.toml` if missing)
- Playwright browsers → `compose/sketchpad:setup`
- VSCode extension build → `repo/vscode:build` (called by `workspace:setup`)
- Git hooks via `repo/client` → `workspace:git-setup`
- GitKraken/Antigravity → keep as opt-in `.devcontainer/post-attach.sh` only

[.devcontainer/install-native.ps1](.devcontainer/install-native.ps1):
- `winget install Oven-sh.Bun` (drop `OpenJS.NodeJS.LTS` unless Electron requires it).
- After all `winget`/`rustup`/`uv` baseline, end with `bun install; bun nx run workspace:setup`.
- Delete the per-toolchain bootstrap helpers that Nx targets now own.

[.devcontainer/post-attach.sh](.devcontainer/post-attach.sh) and [post-start.sh](.devcontainer/post-start.sh) drop their `npm install` / per-package build calls and rely on `workspace:setup` having already run.

## 7. CI

[.github/workflows/playwright.yml](.github/workflows/playwright.yml) and [gh-pages.yml](.github/workflows/gh-pages.yml):

```yaml
- uses: oven-sh/setup-bun@v2
  with: { bun-version: latest }
- run: bun install --frozen-lockfile
- run: bun nx run workspace:setup
- run: bun nx affected -t test     # or run @compose/docs:build for pages
```

Add Nx cache persistence via `actions/cache` keyed on `bun.lock`, `Cargo.lock`, `uv.lock`, `go.sum` aggregate hash.

## 8. Verification (no test suites are added; we exercise existing ones)

- `bun nx graph` lists every project with dependencies via Nx auto-detection.
- `rm -rf node_modules .venv target .nx/cache && bun nx run workspace:setup` succeeds zero-touch.
- `bun nx run-many -t build` produces all artifacts, second run reports `[local cache]` everywhere.
- `bun nx run-many -t test` green on existing suites (we don't touch test logic).
- Devcontainer rebuild + native PowerShell run both end with `workspace:setup` exit 0.
- Verify Bun runs Electron-Forge (`bun nx run @compose/desktop:build`); fall back to a single `node` install in Dockerfile if blocked.

## Out of scope

- Nx Cloud connection (left for the user).
- Replacing `vitest`/`playwright` test runners (kept; they run fine under Bun).
- `AGENTS.md` / `CLAUDE.md` (forbidden by rules).
- Touching domain code in any bundle.

## Notes

- This is one focused refactor; no need to delegate. Estimated 4-6 hours of edits + verification.
- `pnpm.overrides` (`@compose/asset`, `@compose/js` workspace pinning) becomes `overrides` at root (Bun supports the `overrides` field).
- `defaultBase = "⛳wip"` is unusual but kept.
- The repo MCP `ticket_open` is unavailable in this Cursor session's MCP set; the ticket folder will be created via `repo/client/client(.exe) ticket open` once available, or scaffolded manually under `.repo/🎫/26/05/11/bun-nx-monorepo-setup/`.