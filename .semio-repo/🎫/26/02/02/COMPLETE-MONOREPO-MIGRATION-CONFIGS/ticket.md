# Ticket

## Todos

- [ ] Fix `.gitmodules` — references `examples/metabolism` but submodule is now at `semio/examples/metabolism`
- [ ] Fix `.gitignore` — old Go binary paths (`semio-repo/cli`, `semio-repo/go/main`, `semio-repo/server/server`, `semio-repo/server/main`) need verification; old `js/**/public` and `js/semio/lib` exception paths are stale; `!rb/lib` references old `rb/` location
- [ ] Fix `.prettierignore` — only has `node_modules` and `.semio-repo/prompts/**/*.md`, may need `semio/` scoped ignores
- [ ] Fix `tsconfig.json` — `include` has old `js/semio/.storybook/**/*.ts(x)`, `exclude` has old `js/temp`
- [ ] Fix `pyproject.toml` — `testpaths` has old `py/semio`, `py/engine` (should be `semio/py`, `semio/engine`)
- [ ] Fix `Monorepo.sln` — references old `semio\net\Semio\...`, `semio\grasshopper\...` paths (backslash but already `@`-prefixed — verify correctness)
- [ ] Fix `.github/dependabot.yml` — ALL directories use old paths: `js/semio`, `js/desktop`, `js/docs`, `py/semio`, `py/engine`, `net/Semio`, `net/Semio.Grasshopper`, `net/Semio.Tests`, `net/Semio.Grasshopper.Tests`, `go/cli`, `go/mcp`, `./semio-repo/cli`, `go/semio`, `rs/semio`
- [ ] Fix `.github/workflows/gh-pages.yml` — `path: ./js/docs` should be `semio/docs`; npm version `10.8.2` is old
- [ ] Fix `.github/workflows/playwright.yml` — generic `npx playwright test` with no path context
- [ ] Fix `devcontainer.json` — `dotnet.defaultSolution` = `net/Semio.sln` (should be `Monorepo.sln` or `semio/net/...`); `rust-analyzer.linkedProjects` = `rs/semio/Cargo.toml` (should be `semio/rs/Cargo.toml`); `sqltools.connections.database` = `./examples/metabolism/.semio/kit.db` (should be `semio/examples/...`)
- [ ] Fix `post-attach.sh` — `VSIX_PATH` = `semio-repo/vscode/semio-repo.vsix` (OK); but checks `js/vscode/extension.ts` and `js/vscode/package.json` (old paths); `cd js/vscode` (old); Windsurf MCP command `./semio-repo/cli/cli` should be `./semio-repo/cli`
- [ ] Fix `post-create.sh` — `cd ./semio-repo/cli` (should be `cd semio-repo/go`); `go build -o repo` should be just `go build` (default binary name from module path is already `repo`); `dotnet restore net/Semio.sln` (should be `Monorepo.sln` or `semio/net/...`); `cd js/vscode` (should be `cd semio-repo/vscode`)
- [ ] Fix all MCP configs — `.mcp.json`, `.vscode/mcp.json`, `.cursor/mcp.json`, `.windsurf/mcp.json`, `.codex/config.toml` all reference `./semio-repo/cli/cli` (should be `./semio-repo/cli`); `.vscode/mcp.json` references `py/engine` (should be `semio/engine`)
- [ ] Fix `.claude/settings.json` — all permission paths use old format: `net/Semio.Grasshopper/...`, `net/Semio/...`, `py/engine.py`, `js/semio/...`, `js/js/...` (should all be `semio/...`)
- [ ] Fix `package-lock.json` — regenerate after package.json workspace paths are confirmed correct (currently has old `js/semio`, `js/docs`, `./semio-repo/cli`, etc.)
- [ ] Fix `.vscode/tasks.json` — references `js/vscode` path (line 108)
- [ ] Verify `package.json` workspaces — some entries like `semio/net/Semio`, `semio/grasshopper/Semio.Grasshopper`, `semio/grasshopper/Semio.Grasshopper/yak` need path verification

## Changes

## Log

### 2026-02-02 — Initial audit

Audited all config files in the monorepo after the restructuring from flat paths (`js/`, `py/`, `go/`, `net/`, `rs/`, `examples/`) to scoped `@`-prefixed paths (`semio/`, `semio-repo/`, `coda/`).

**Path mapping (old → new):**

- `js/semio` → `semio/js`
- `js/docs` → `semio/docs`
- `js/play` → `semio/play`
- `js/desktop` → `semio/desktop`
- `js/vscode` → `semio-repo/vscode`
- `py/semio` → `semio/py`
- `py/engine` → `semio/engine`
- `./semio-repo/cli` → `semio-repo/go`
- `go/semio` → `semio/go`
- `go/mcp` → `semio-repo/server` (?)
- `net/Semio` → `semio/net/Semio`
- `net/Semio.Grasshopper` → `semio/grasshopper/Semio.Grasshopper`
- `rs/semio` → `semio/rs`
- `examples/metabolism` → `semio/examples/metabolism`

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
10. All MCP configs (6 files) — `./semio-repo/cli/cli` command, `py/engine` path
11. `.claude/settings.json` — all permission paths
12. `package-lock.json` — needs regeneration
13. `.vscode/tasks.json` — js/vscode path

## Summary

Bulk close
