---
goal: BUILDGRAPH
---

# Ticket: Build Functional Parity Inventory

## Summary

Bulk close

## Plan

1. Gather all root-level config files (package.json, nx.json, tsconfig.json, vitest.config.ts, pyproject.toml, Cargo.toml, go.work, Monorepo.sln, conftest.py)
2. Inventory all 27 npm workspace package.json files and their scripts
3. Inventory all Nx project detection and target defaults
4. Inventory all non-JS build configs (.csproj, pyproject.toml, go.mod, Cargo.toml)
5. Inventory VS Code tasks
6. Document the preflight/build/test/publish orchestration pipeline
7. Document the full dependency graph
8. Write comprehensive inventory

## Todos

- [x] Gather root build config files
- [x] Inventory nx.json workspace config
- [x] Inventory all package.json workspaces
- [x] Inventory non-JS build configs
- [x] Inventory CI/CD and task configs
- [x] Document all build targets/scripts
- [x] Document dependency graph
- [x] Write full inventory to ticket file

## Changes

- `.repo/tickets/2026/02/18/BUILD-FUNCTIONAL-PARITY-INVENTORY/ticket.md` — created inventory document

## Log

- Gathered all root config files: package.json, nx.json, tsconfig.json, vitest.config.ts, pyproject.toml, Cargo.toml, go.work, Monorepo.sln, conftest.py
- Extracted scripts, deps, and Nx targets from all 27 workspace package.json files
- Verified Nx project graph via `npx nx show projects` (28 projects detected)
- Analyzed repo CLI preflight orchestration logic in main.go
- Parsed .csproj project references for .NET dependency chain
- Confirmed Go workspace (go.work) members
- Confirmed Rust workspace (Cargo.toml) members
- Documented VS Code tasks from .vscode/tasks.json

---

# Build System Functional Parity Inventory

## 1. Orchestration Layer

### 1.1 Root Orchestrator: `repo/cli` (Go binary)

The root `package.json` scripts delegate to the repo CLI binary (`./repo/cli/cli`), which then dispatches to `npx nx run-many -t <target>`:

| Root Script     | CLI Command                           | Pipeline                           |
| --------------- | ------------------------------------- | ---------------------------------- |
| `dev`           | `npx nx run-many -t dev`              | Direct Nx                          |
| `analyze`       | `repo/cli/cli analyze`                | CLI analyze                        |
| `fix`           | `repo/cli/cli fix`                    | CLI fix                            |
| `preflight`     | `repo/cli/cli preflight`              | fix → analyze                      |
| `test`          | `repo/cli/cli preflight test`         | fix → analyze → `nx test`          |
| `build`         | `repo/cli/cli preflight build`        | `nx test` → `nx build`             |
| `update`        | `repo/cli/cli update`                 | npm+python+rust+go+dotnet parallel |
| `publish:test`  | `repo/cli/cli preflight publish:test` | `nx build` → `nx publish:test`     |
| `publish`       | `repo/cli/cli preflight publish`      | `nx build` → `nx publish`          |
| `pre-commit`    | `npm run preflight`                   | fix → analyze                      |
| `benchmark`     | `repo/cli/cli benchmark`              | CLI benchmark                      |
| `mcp:inspector` | `npx @modelcontextprotocol/inspector` | Direct                             |

### 1.2 Nx Configuration (`nx.json`)

**Plugins:**

- `@nx/eslint-plugin` (targetName: `lint`)
- `@nxlv/python` (Python workspace support)

**Target Defaults (all with `cache: true` except `dev`):**

| Target         | `dependsOn`     | Inputs                                      |
| -------------- | --------------- | ------------------------------------------- |
| `lint`         | `^lint`         | `{projectRoot}/**/*`, `{projectRoot}/.env*` |
| `build`        | `^build`        | `{projectRoot}/**/*`, `{projectRoot}/.env*` |
| `dev`          | _(none)_        | _(cache: false)_                            |
| `test`         | `^test`         | `{projectRoot}/**/*`, `{projectRoot}/.env*` |
| `publish:test` | `^publish:test` | `{projectRoot}/**/*`, `{projectRoot}/.env*` |
| `publish`      | `^publish`      | `{projectRoot}/**/*`, `{projectRoot}/.env*` |
| `update`       | `^update`       | `{projectRoot}/**/*`, `{projectRoot}/.env*` |

**Constraint:** `^build` means each project's `build` target depends on its upstream dependencies' `build` targets completing first (topological ordering).

### 1.3 Preflight Pipeline (in repo CLI)

```
preflight (default) = fix → analyze
preflight test      = fix → analyze → nx run-many -t test
preflight build     = nx run-many -t test → nx run-many -t build
preflight publish:test = nx run-many -t build → nx run-many -t publish:test
preflight publish   = nx run-many -t build → nx run-many -t publish
```

## 2. Workspace Projects (27 npm workspaces + detected Nx projects)

### 2.1 npm Workspaces (root `package.json` → `workspaces`)

1. `semio/assets/logo` → `@semio/logo`
2. `semio/assets/icons` → `@semio/icons`
3. `semio/assets` → `@semio/assets`
4. `semio/py` → `@semio/py`
5. `semio/engine` → `@semio/engine`
6. `semio/js` → `@semio/js`
7. `semio/docs` → `@semio/docs`
8. `semio/play` → `@semio/play`
9. `semio/desktop` → `@semio/desktop`
10. `repo/vscode` → `repo`
11. `semio/net/Semio` → `@semio/net`
12. `semio/grasshopper/Semio.Grasshopper` → _(no package.json)_
13. `semio/go` → `@semio/go`
14. `repo/cli` → `@repo/cli`
15. `repo/server` → `@repo/server`
16. `semio/rs` → `@semio/rs`
17. `semio/grasshopper/Semio.Grasshopper/yak` → _(no package.json)_
18. `semio/sqlite` → `@semio/sqlite`
19. `repo/sqlite` → `@repo/sqlite`
20. `coda/py` → `@coda/engine`
21. `semio/sketchpad` → `@semio/sketchpad`

### 2.2 Additional Nx Auto-detected Projects (no npm workspace entry)

- `.coda` (coda/examples)
- `repo/graphql`
- `semio/graphql`
- `jsonschema`
- `liveblocks`
- `openapi`
- `antlr`
- `peg`
- `rdf`

## 3. Per-Workspace Scripts & Build Behaviors

### 3.1 TypeScript/JavaScript Workspaces

#### `@semio/logo` (semio/assets/logo)

| Script      | Command             | Behavior                       |
| ----------- | ------------------- | ------------------------------ |
| `dev`       | `tsx watch logo.ts` | Watch mode, generates SVG logo |
| `build`     | `tsx logo.ts`       | One-shot logo generation       |
| `preflight` | `tsc --noEmit`      | Type-check only                |

**Dependencies:** `jsdom`
**Dev deps:** `@types/jsdom`, `@types/node`, `tsx`

#### `@semio/icons` (semio/assets/icons)

| Script      | Command                                             | Behavior          |
| ----------- | --------------------------------------------------- | ----------------- |
| `build`     | `echo 'icons build not yet migrated to TypeScript'` | No-op placeholder |
| `preflight` | `echo "No preflight checks configured for icons"`   | No-op             |

**Dev deps:** `@semio/logo` (internal)

#### `@semio/assets` (semio/assets)

| Script      | Command                                            | Behavior |
| ----------- | -------------------------------------------------- | -------- |
| `preflight` | `echo "No preflight checks configured for assets"` | No-op    |

**Dependencies:** `lucide-react`

#### `@semio/js` (semio/js) — **Core UI library**

| Script          | Command                                                  | Behavior                      |
| --------------- | -------------------------------------------------------- | ----------------------------- |
| `dev`           | `tsx dev.ts`                                             | Custom dev orchestrator       |
| `dev:storybook` | `storybook dev -p 6006 --host 0.0.0.0 --no-open --debug` | Storybook dev server          |
| `dev:sketchpad` | `vite --port 5173 --host 0.0.0.0`                        | Vite dev server for sketchpad |
| `build`         | `storybook build`                                        | Build Storybook static site   |
| `test`          | `vitest run`                                             | Run unit tests                |
| `test:unit`     | `vitest run`                                             | Same as test                  |
| `test:e2e`      | `playwright test`                                        | Run Playwright E2E tests      |
| `test:coverage` | `vitest run --coverage`                                  | Tests with coverage           |
| `preflight`     | `prettier --write . && tsc --noEmit`                     | Format + type-check           |
| `postinstall`   | Copy `sql-wasm.wasm` to `public/`                        | WASM file setup               |

**Key dependencies:** `react`, `react-dom`, `@xyflow/react`, `@react-three/fiber`, `@react-three/drei`, `three`, `xstate`, `yjs`, `sql.js`, `zod`, `i18next`, `golden-layout`, `motion`, `@dnd-kit/*`, `@radix-ui/*`, `cmdk`, `cytoscape`, `d3-force`, `dagre`, `fuse.js`, `jszip`, `mathjax`, `react-router`, `rehype-*`, `remark-*`, `uuid`
**Dev deps:** `@semio/assets`, `storybook`, `vite`, `vitest`, `@playwright/test`, `tailwindcss`, `postcss`, `eslint`, `prettier`, `tsx`, `typescript`

#### `@semio/docs` (semio/docs)

| Script      | Command                                 | Behavior          |
| ----------- | --------------------------------------- | ----------------- |
| `dev`       | `vite --port 4321 --host 0.0.0.0`       | Docs dev server   |
| `build`     | `vite build`                            | Build static docs |
| `publish`   | `vite build && npm publish`             | Build + publish   |
| `preflight` | `echo "No preflight checks configured"` | No-op             |

**Dependencies:** `@semio/js`

#### `@semio/play` (semio/play)

| Script      | Command                                 | Behavior              |
| ----------- | --------------------------------------- | --------------------- |
| `dev`       | `vite --port 4000 --host 0.0.0.0`       | Play dev server       |
| `build`     | `vite build`                            | Build static play app |
| `publish`   | `vite build && npm publish`             | Build + publish       |
| `preflight` | `echo "No preflight checks configured"` | No-op                 |

**Dependencies:** `@semio/js`

#### `@semio/sketchpad` (semio/sketchpad)

| Script        | Command                                 | Behavior             |
| ------------- | --------------------------------------- | -------------------- |
| `dev`         | `vite`                                  | Sketchpad dev server |
| `build`       | `vite build`                            | Build static app     |
| `publish`     | `vite build && npm publish`             | Build + publish      |
| `preflight`   | `echo "No preflight checks configured"` | No-op                |
| `postinstall` | Copy `sql-wasm.wasm` to `public/`       | WASM file setup      |

**Dependencies:** `@semio/js`

#### `@semio/desktop` (semio/desktop)

| Script      | Command                                 | Behavior             |
| ----------- | --------------------------------------- | -------------------- |
| `dev`       | `electron-forge start`                  | Electron dev         |
| `build`     | `electron-forge make`                   | Build distributables |
| `publish`   | `electron-forge publish`                | Publish release      |
| `preflight` | `echo "No preflight checks configured"` | No-op                |

**Dependencies:** `@semio/js`, `@electron/fuses`, `electron-squirrel-startup`
**Dev deps:** `electron`, `@electron-forge/*` makers/plugins

#### `repo` (repo/vscode) — VS Code Extension

| Script              | Command                                                 | Behavior                      |
| ------------------- | ------------------------------------------------------- | ----------------------------- |
| `dev`               | `vite build --watch`                                    | Watch mode build              |
| `test`              | `vscode-test`                                           | VS Code extension test runner |
| `build`             | `vite build && vite build --config vite.test.config.ts` | Build prod + test bundle      |
| `package`           | `vsce package --no-dependencies --out repo.vsix`        | Package VSIX                  |
| `preflight`         | `tsc --noEmit`                                          | Type-check                    |
| `vscode:prepublish` | `npm run build`                                         | Pre-publish hook              |

**Dev deps:** `@semio/js`, `@vscode/vsce`, `@vscode/test-cli`, `@vscode/test-electron`, `vite`, `typescript`, `jsonc-parser`

### 3.2 Python Workspaces

#### `@semio/py` (semio/py)

| Script      | Command                               | Behavior             |
| ----------- | ------------------------------------- | -------------------- |
| `build`     | `uv build`                            | Build Python package |
| `test`      | `uv run pytest`                       | Run pytest           |
| `preflight` | `ruff format . && ruff check --fix .` | Format + lint        |

**Python deps:** `pydantic`, `numpy`, `networkx`, `python-dotenv`, `fastapi`, `graphene`, `graphene-pydantic`, `graphene-sqlalchemy`, `loguru`, `pytransform3d`, `sqlalchemy`, `sqlmodel`

#### `@semio/engine` (semio/engine)

| Script      | Command                               | Behavior                                                 |
| ----------- | ------------------------------------- | -------------------------------------------------------- |
| `dev`       | `uv run engine.py`                    | Run engine dev server                                    |
| `build`     | `tsx ./build.ts`                      | TypeScript build script (generates assets → PyInstaller) |
| `test`      | `tsx ./test.ts`                       | TypeScript test runner                                   |
| `preflight` | `ruff format . && ruff check --fix .` | Format + lint                                            |

**Python deps:** `semio` (workspace), `fastapi[standard]`, `graphene`, `lark`, `networkx`, `numpy`, `openai`, `pint`, `pydantic`, `pyside6`, `pytransform3d`, `sqlalchemy`, `sqlmodel`, `uvicorn`, `mcp[cli]` and more
**Dev deps (npm):** `@semio/assets`

#### `@coda/engine` (coda/py)

| Script      | Command                               | Behavior      |
| ----------- | ------------------------------------- | ------------- |
| `dev`       | `uv run coda.py`                      | Run coda dev  |
| `preflight` | `ruff format . && ruff check --fix .` | Format + lint |

**Nx targets:** `update` → `@nxlv/python:update`, `lock` → `@nxlv/python:lock`
**Dev deps (npm):** `@semio/assets`

#### Python Workspace Root (`pyproject.toml`)

- `uv workspace members`: `semio/py`, `semio/engine`
- Dev dependency groups: `jupyter`, `notebook`, `ipykernel`, `ruff`, `black`, `debugpy`, `pandas`, `numpy`, `matplotlib`, `seaborn`, `scipy`, `scikit-learn`
- Test dependency group: `pytest`, `pytest-cov`, `deepdiff`
- Test paths: `semio/py`, `semio/engine`
- `conftest.py` at root: patches `sys.path` for `semio` and `engine` modules

### 3.3 Go Workspaces

#### Go Workspace Root (`go.work`)

- Go 1.24.0
- Members: `repo/cli`, `repo/server`, `semio/go`

#### `@semio/go` (semio/go)

| Script      | Command            | Behavior              |
| ----------- | ------------------ | --------------------- |
| `build`     | `go build ./...`   | Build all Go packages |
| `test`      | `go test -v ./...` | Run Go tests          |
| `preflight` | `go vet ./...`     | Go vet                |

**Module:** `github.com/usalu/semio/go/semio`
**Key dep:** `gonum.org/v1/gonum`

#### `@repo/cli` (repo/cli)

| Script      | Command            | Behavior     |
| ----------- | ------------------ | ------------ |
| `dev`       | `go run`           | Run CLI dev  |
| `build`     | `go build`         | Build binary |
| `test`      | `go test -v ./...` | Run Go tests |
| `preflight` | `go vet ./...`     | Go vet       |

**Module:** `github.com/usalu/semio/repo/cli`
**Dep:** `github.com/usalu/semio/repo/go` (internal)

#### `@repo/server` (repo/server)

| Script      | Command                      | Behavior            |
| ----------- | ---------------------------- | ------------------- |
| `dev`       | `go run main.go`             | Run server dev      |
| `build`     | `go build -o server main.go` | Build server binary |
| `test`      | `go test -v ./...`           | Run Go tests        |
| `preflight` | `go vet ./...`               | Go vet              |

**Module:** `github.com/usalu/semio/repo/server`
**Key dep:** `modernc.org/sqlite`

### 3.4 Rust Workspace

#### Rust Workspace Root (`Cargo.toml`)

- Members: `semio/rs`
- Resolver: 2

#### `@semio/rs` (semio/rs)

| Script      | Command                             | Behavior            |
| ----------- | ----------------------------------- | ------------------- |
| `build`     | `cargo build --release`             | Release build       |
| `test`      | `cargo test`                        | Run Rust tests      |
| `preflight` | `cargo fmt --check && cargo clippy` | Format check + lint |

**Crate:** `semio` v0.1.0, edition 2021
**Lib type:** `cdylib` + `rlib` (supports WASM + native)
**Core deps:** `serde`, `serde_json`, `thiserror`, `uuid`, `nalgebra`
**WASM deps:** `wasm-bindgen`, `js-sys`, `web-sys`, `serde-wasm-bindgen`, `getrandom`
**Native deps:** `rusqlite` (bundled), `chrono`, `zip`, `tempfile`, `walkdir`
**Bin:** `semio-benchmark`

### 3.5 .NET Workspace

#### .NET Solution (`Monorepo.sln`)

- Configurations: Debug, Release, UnitTest

| Project                   | Target Frameworks | Key Dependencies                                                                                     |
| ------------------------- | ----------------- | ---------------------------------------------------------------------------------------------------- |
| `Semio`                   | net8.0; net48     | FluentValidation, Humanizer, Newtonsoft.Json, QuikGraph, Refit, Svg, UnitsNet, Microsoft.Data.Sqlite |
| `Semio.Grasshopper`       | net7.0; net48     | Grasshopper (Rhino), System.Drawing.Common → references `Semio`                                      |
| `Semio.Tests`             | net8.0; net48     | xunit, Microsoft.NET.Test.Sdk → references `Semio`                                                   |
| `Semio.Grasshopper.Tests` | net7.0; net48     | xunit → references `Semio.Grasshopper` + `Semio`                                                     |
| `Semio.Benchmark`         | net8.0            | Newtonsoft.Json → references `Semio`                                                                 |

#### `@semio/net` (semio/net/Semio) — npm wrapper

| Script      | Command                                         | Behavior                         |
| ----------- | ----------------------------------------------- | -------------------------------- |
| `build`     | `tsx ./build.ts`                                | TypeScript build script for .NET |
| `test`      | `dotnet test ../Semio.Tests/Semio.Tests.csproj` | Run xunit tests                  |
| `preflight` | `dotnet build`                                  | Build .NET solution              |

**Dev deps:** `@semio/assets`

### 3.6 Schema-only Workspaces (no scripts)

| Workspace      | Name            | Type   |
| -------------- | --------------- | ------ |
| `semio/sqlite` | `@semio/sqlite` | schema |
| `repo/sqlite`  | `@repo/sqlite`  | schema |

## 4. Dependency Graph

### 4.1 Internal npm Dependency Chain

```
@semio/logo ← @semio/icons
@semio/logo, @semio/icons ← @semio/assets
@semio/assets ← @semio/js
@semio/assets ← @semio/engine (devDep)
@semio/assets ← @semio/net (devDep)
@semio/assets ← @coda/engine (devDep)
@semio/js ← @semio/docs
@semio/js ← @semio/play
@semio/js ← @semio/sketchpad
@semio/js ← @semio/desktop
@semio/js ← repo (vscode ext, devDep)
```

### 4.2 Cross-Language Dependencies

```
semio/py → semio/engine (Python workspace dep via uv)
semio/net/Semio → semio/gh/Semio.Grasshopper (.NET ProjectReference)
semio/net/Semio → semio/net/Semio.Tests (.NET ProjectReference)
semio/net/Semio → semio/net/Semio.Benchmark (.NET ProjectReference)
semio/gh/Semio.Grasshopper → semio/gh/Semio.Grasshopper.Tests (.NET ProjectReference)
repo/go → repo/cli (Go module require)
```

### 4.3 Build-to-Build Dependencies (Nx `^build`)

Since `build.dependsOn = ["^build"]`, build order is topologically sorted:

1. `@semio/logo` → 2. `@semio/icons` → 3. `@semio/assets` → 4. `@semio/js` → 5. `@semio/docs`, `@semio/play`, `@semio/sketchpad`, `@semio/desktop`, `repo`

Other chains build independently in parallel:

- `@semio/py` → `@semio/engine`
- `@semio/go`, `@repo/cli`, `@repo/server` (Go)
- `@semio/rs` (Rust)
- `@semio/net` (C#/.NET)

## 5. Test Infrastructure

### 5.1 Vitest (root `vitest.config.ts`)

- Test projects: `./semio/js/vite.config.ts`
- Only `@semio/js` is registered for Vitest unit tests

### 5.2 Playwright

- E2E tests via `@semio/js` → `playwright test`
- Config: `semio/js/playwright.config.ts`

### 5.3 pytest (root `pyproject.toml`)

- Test paths: `semio/py`, `semio/engine`
- Pattern: `test_*.py`, `*_test.py`, `*.test.py`
- Import mode: `importlib`
- Coverage via `pytest-cov`

### 5.4 Go tests

- `go test -v ./...` in `semio/go`, `repo/cli`, `repo/server`

### 5.5 Rust tests

- `cargo test` in `semio/rs`

### 5.6 .NET tests (xunit)

- `dotnet test` for `Semio.Tests` and `Semio.Grasshopper.Tests`

## 6. Dev Modes

| Label                    | Command                  | Background | Port     |
| ------------------------ | ------------------------ | ---------- | -------- |
| `dev` (aggregate)        | `npx nx run-many -t dev` | Yes        | Multiple |
| `repo/cli dev`           | `go run`                 | No         | —        |
| `repo/server dev`        | `go run main.go`         | No         | —        |
| `repo/vscode dev`        | `vite build --watch`     | Yes        | —        |
| `semio/engine dev`       | `uv run engine.py`       | Yes        | —        |
| `semio/js dev`           | `tsx dev.ts`             | Yes        | —        |
| `semio/js dev:storybook` | `storybook dev -p 6006`  | Yes        | 6006     |
| `semio/js dev:sketchpad` | `vite --port 5173`       | Yes        | 5173     |
| `semio/docs dev`         | `vite --port 4321`       | Yes        | 4321     |
| `semio/play dev`         | `vite --port 4000`       | Yes        | 4000     |
| `semio/desktop dev`      | `electron-forge start`   | No         | —        |
| `semio/logo dev`         | `tsx watch logo.ts`      | No         | —        |

## 7. Preflight Behaviors per Workspace

| Workspace          | Preflight Command                     | Effect                    |
| ------------------ | ------------------------------------- | ------------------------- |
| `@semio/logo`      | `tsc --noEmit`                        | TypeScript type-check     |
| `@semio/icons`     | `echo` (no-op)                        | —                         |
| `@semio/assets`    | `echo` (no-op)                        | —                         |
| `@semio/py`        | `ruff format . && ruff check --fix .` | Python format + lint      |
| `@semio/engine`    | `ruff format . && ruff check --fix .` | Python format + lint      |
| `@semio/js`        | `prettier --write . && tsc --noEmit`  | JS format + TS type-check |
| `@semio/docs`      | `echo` (no-op)                        | —                         |
| `@semio/play`      | `echo` (no-op)                        | —                         |
| `@semio/sketchpad` | `echo` (no-op)                        | —                         |
| `@semio/desktop`   | `echo` (no-op)                        | —                         |
| `repo` (vscode)    | `tsc --noEmit`                        | TypeScript type-check     |
| `@semio/net`       | `dotnet build`                        | .NET build                |
| `@semio/go`        | `go vet ./...`                        | Go vet                    |
| `@repo/cli`        | `go vet ./...`                        | Go vet                    |
| `@repo/server`     | `go vet ./...`                        | Go vet                    |
| `@semio/rs`        | `cargo fmt --check && cargo clippy`   | Rust format check + lint  |
| `@coda/engine`     | `ruff format . && ruff check --fix .` | Python format + lint      |

## 8. Update Mechanism

The `repo/cli update` command manages dependency updates across all ecosystems in parallel:

- **npm:** `npm update` per workspace
- **Python:** `uv` updates
- **Rust:** `cargo update`
- **Go:** `go get -u` per module
- **Dotnet:** `dotnet outdated`

Supports `--dry-run` (default) and `--apply` flags. Reads constraints from dependabot config.

## 9. Key Constraints & Behavioral Notes

1. **Node.js ≥ 22.13.1** required (root `engines`)
2. **npm 11.7.0** packageManager pinned
3. **Python ≥ 3.14** (root pyproject.toml)
4. **Go 1.24.0** (go.work)
5. **Rust edition 2021** (Cargo.toml)
6. **.NET multi-target:** net8.0/net48 (Semio), net7.0/net48 (Grasshopper)
7. **WASM support:** `@semio/rs` builds as `cdylib` for WASM target; `sql.js` WASM file copied via postinstall in `@semio/js` and `@semio/sketchpad`
8. **TypeScript strict mode** enabled globally; `noEmit: true` (type-check only, Vite handles bundling)
9. **Nx caching** enabled for all targets except `dev`
10. **Root `overrides`:** `diff: ^8.0.3`, `tmp: ^0.2.5` (forced versions across all workspaces)
11. **Root `devDependencies` hoisted:** `nx`, `typescript`, `vitest`, `tsx`, `react`, `react-dom`, `@types/react`, `@types/node`, `lint-staged`, `esbuild`
12. **Pre-commit hook:** Runs full `npm run preflight` (fix → analyze) via global git hooks
13. **Two workspaces lack package.json:** `semio/grasshopper/Semio.Grasshopper`, `semio/grasshopper/Semio.Grasshopper/yak` — these are managed purely via .NET/csproj
14. **`@semio/js` has 3 dev modes:** `dev` (main orchestrator), `dev:storybook`, `dev:sketchpad`
15. **Hierarchical VS Code task naming** follows pattern `project/bundle target` (e.g., `semio/js build`, `repo/cli test`)
16. **Root-level `dev:js:js:storybook` and `dev:js:js:sketchpad`** expose multi-dev-mode scripts with colon separators
