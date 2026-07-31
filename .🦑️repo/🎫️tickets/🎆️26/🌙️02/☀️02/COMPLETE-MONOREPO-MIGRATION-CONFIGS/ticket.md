# Ticket

## Todos

- [ ] Fix `.gitmodules` — references `examples/metabolism` but submodule is now at `compose/examples/metabolism`
- [ ] Fix `.gitignore` — old Go binary paths (`repo/cli`, `repo/go/main`, `repo/server/server`, `repo/server/main`) need verification; old `js/**/public` and `js/compose/lib` exception paths are stale; `!rb/lib` references old `rb/` location
- [ ] Fix `.prettierignore` — only has `node_modules` and `.repo/prompts/**/*.md`, may need `compose/` scoped ignores
- [ ] Fix `tsconfig.json` — `include` has old `js/compose/.storybook/**/*.ts(x)`, `exclude` has old `js/temp`
- [ ] Fix `pyproject.toml` — `testpaths` has old `py/compose`, `py/engine` (should be `compose/py`, `compose/engine`)
- [ ] Fix `Monorepo.sln` — references old `compose\net\Compose\...`, `compose\grasshopper\...` paths (backslash but already `@`-prefixed — verify correctness)
- [ ] Fix `.github/dependabot.yml` — ALL directories use old paths: `js/compose`, `js/desktop`, `js/docs`, `py/compose`, `py/engine`, `net/Compose`, `net/Compose.Grasshopper`, `net/Compose.Tests`, `net/Compose.Grasshopper.Tests`, `go/cli`, `go/mcp`, `./repo/cli`, `go/compose`, `rs/compose`
- [ ] Fix `.github/workflows/gh-pages.yml` — `path: ./js/docs` should be `compose/docs`; npm version `10.8.2` is old
- [ ] Fix `.github/workflows/playwright.yml` — generic `npx playwright test` with no path context
- [ ] Fix `devcontainer.json` — `dotnet.defaultSolution` = `net/Compose.sln` (should be `Monorepo.sln` or `compose/net/...`); `rust-analyzer.linkedProjects` = `rs/compose/Cargo.toml` (should be `compose/rs/Cargo.toml`); `sqltools.connections.database` = `./examples/metabolism/.compose/kit.db` (should be `compose/examples/...`)
- [ ] Fix `post-attach.sh` — `VSIX_PATH` = `repo/vscode/repo.vsix` (OK); but checks `js/vscode/extension.ts` and `js/vscode/package.json` (old paths); `cd js/vscode` (old); Windsurf MCP command `./repo/cli/cli` should be `./repo/cli`
- [ ] Fix `post-create.sh` — `cd ./repo/cli` (should be `cd repo/go`); `go build -o repo` should be just `go build` (default binary name from module path is already `repo`); `dotnet restore net/Compose.sln` (should be `Monorepo.sln` or `compose/net/...`); `cd js/vscode` (should be `cd repo/vscode`)
- [ ] Fix all MCP configs — `.mcp.json`, `.vscode/mcp.json`, `.cursor/mcp.json`, `.windsurf/mcp.json`, `.codex/config.toml` all reference `./repo/cli/cli` (should be `./repo/cli`); `.vscode/mcp.json` references `py/engine` (should be `compose/engine`)
- [ ] Fix `.claude/settings.json` — all permission paths use old format: `net/Compose.Grasshopper/...`, `net/Compose/...`, `py/engine.py`, `js/compose/...`, `js/js/...` (should all be `compose/...`)
- [ ] Fix `package-lock.json` — regenerate after package.json workspace paths are confirmed correct (currently has old `js/compose`, `js/docs`, `./repo/cli`, etc.)
- [ ] Fix `.vscode/tasks.json` — references `js/vscode` path (line 108)
- [ ] Verify `package.json` workspaces — some entries like `compose/net/Compose`, `compose/grasshopper/Compose.Grasshopper`, `compose/grasshopper/Compose.Grasshopper/yak` need path verification

## Changes

## Log

### 2026-02-02 — Initial audit

Audited all config files in the monorepo after the restructuring from flat paths (`js/`, `py/`, `go/`, `net/`, `rs/`, `examples/`) to scoped `@`-prefixed paths (`compose/`, `repo/`, `coda/`).

**Path mapping (old → new):**

- `js/compose` → `compose/js`
- `js/docs` → `compose/docs`
- `js/play` → `compose/play`
- `js/desktop` → `compose/desktop`
- `js/vscode` → `repo/vscode`
- `py/compose` → `compose/py`
- `py/engine` → `compose/engine`
- `./repo/cli` → `repo/go`
- `go/compose` → `compose/go`
- `go/mcp` → `repo/server` (?)
- `net/Compose` → `compose/net/Compose`
- `net/Compose.Grasshopper` → `compose/grasshopper/Compose.Grasshopper`
- `rs/compose` → `compose/rs`
- `examples/metabolism` → `compose/examples/metabolism`

**Files with outdated paths:**

1. `.gitmodules` — submodule path
2. `.gitignore` — Go binary paths, JS exception paths
3. `tsconfig.json` — include/exclude paths
4. `pyproject.toml` — testpaths
5. `.github/dependabot.yml` — all directory entries
6. `.github/workflows/gh-pages.yml` — docs path
7. `devcontainer.json` — dotnet solution, rust-analyzer, sqltools
8. `post-attach.sh` — vsix build paths, Windsurf MCP command
9. `post-create.sh` — Go build, dotnet restore, vscode build paths
10. All MCP configs (6 files) — `./repo/cli/cli` command, `py/engine` path
11. `.claude/settings.json` — all permission paths
12. `package-lock.json` — needs regeneration
13. `.vscode/tasks.json` — js/vscode path

## Summary

Bulk close
