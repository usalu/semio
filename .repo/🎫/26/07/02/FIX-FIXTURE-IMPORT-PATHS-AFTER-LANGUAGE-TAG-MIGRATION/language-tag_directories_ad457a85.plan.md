---
name: Language-Tag Directories
overview: Insert a language/framework subdirectory (`js`, `rs`, `py`, `go`, `cs`) directly under every bundle's own implementation location repo-wide, so each bundle's source can later be rewritten in a different language without moving the bundle itself.
todos: []
isProject: false
---

# Insert Language-Tag Directories Under Every Bundle

## Goal

Every implementation must start with its language tag as an immediate subdirectory of the bundle that owns it, e.g.:

- [kernel/2d/engine/lib.rs](kernel/2d/engine/lib.rs) → `kernel/2d/engine/rs/lib.rs`
- [trinity/jack/core/lib.rs](trinity/jack/core/lib.rs) → `trinity/jack/core/rs/lib.rs`
- [note/core/index.ts](note/core/index.ts) → `note/core/js/index.ts`

Scope confirmed with dev: apply to **the entire repo**, including `compose/`, `coda/`, `mit-bestand/`. Bundles that already carry the language as their own directory name (e.g. `draw/rs`, `kernel/2d/js`, `ui/styling/py`) are **already compliant and untouched**. The `cad/js/` umbrella is **dismantled** so each bundle gets its own local language folder instead of sharing an ancestor.

## Core rule (decision algorithm)

For every directory that is a "bundle" (identified by owning a `package.json`, `Cargo.toml`, `pyproject.toml`, `go.mod`, or `.csproj`):

1. If the bundle's own final path segment already IS a language tag (`js`, `rs`, `py`, `go`, `cs`) → **skip, no change**.
2. Otherwise, determine which language(s) have real source files directly in that bundle directory (not in a nested sub-bundle that has its own manifest — that nested one is handled independently by the same rule).
3. For each language present, create `<bundle>/<lang>/` and move into it:
   - all source files of that language (`.ts`/`.tsx`, `.rs`, `.py`, `.go`, `.cs`/`.csproj`)
   - that language's own manifest (`Cargo.toml`, `pyproject.toml`, `go.mod`+`go.sum`, `.csproj`) and closely-coupled files (`build.rs`, `uv.lock`, C# `Properties/`, `obj/`, `bin/`)
   - language-specific tooling config that only applies to that source (e.g. `vitest.config.ts`, `tsconfig.json` for the `js` case)
4. Leave at the bundle root (these describe the bundle, not a language):
   - `package.json` (bundle/nx registration — used uniformly across languages in this repo) — but **update its `exports`/`main`/`module`/`types`/`sourceRoot` fields** to point into the new `<lang>/` path
   - `project.json` (nx target definitions; `cwd` is unchanged since the bundle root itself doesn't move)
   - `script.ts` (mandated by repo rules to live "at the respective directory") — **update its internal relative paths / cargo `cwd` / working-dir args** to target the new `<lang>/` subfolder
   - `AGENTS.md`, `README.md`, `LICENSE.md`, `.env.example`, `Dockerfile`, `Caddyfile`
   - bundle-level data dirs unrelated to a specific language (`example/`, `manifest/` data, `generated/` schema output used by multiple consumers)
5. Generated build-output directories (`pkg/` from wasm-pack) are **not manually moved** — update the build script's out-dir to the new `<lang>/pkg` and let it regenerate; delete the stale copy at the old location.
6. If a bundle genuinely contains real source in **two or more languages side-by-side** (not nested sub-bundles), create one subfolder per language (see "Mixed bundles" below).

## Bundles already compliant — do not touch

Own directory name is already the language tag; leave exactly as-is:
`cad/rs`, `draw/rs`, `forms/rs`, `gis/2d/rs`, `layout/rs`, `procedural/2d/rs`, `procedural/3d/rs`, `puzzle/2d/rs`, `puzzle/3d/rs`, `puzzle/5d/rs`, `raster/rs`, `s/rs`, `shooting/rs`, `vcs/rs`, `writer/rs`, `reasoning/mindmap/rs`, `infinite/cavas/rs`, `framework/product/presentation/rs`, `kernel/2d/rs`, `kernel/2d/js`, `kernel/3d/brep/rs`, `kernel/3d/brep/js`, `ui/styling/js`, `ui/styling/rs`, `ui/styling/py`, `repo/lib/js`, `repo/lib/go`, `compose/client/lib/js`, `compose/client/lib/rs`, `compose/client/lib/go`, `compose/client/lib/py`, `compose/client/lib/sketchpad/js`, `coda/client/lib/blnbo/go`, `coda/client/lib/programming/go`.

## Dismantling the `cad/js/` umbrella (special case)

Move every child up one level out of `cad/js/` into `cad/`, then insert `js/` locally in each:

- `cad/js/core` → `cad/core/js`
- `cad/js/runtime` → `cad/runtime/js`
- `cad/js/query` → `cad/query/js`
- `cad/js/kernel/brepjs` → `cad/kernel/brepjs/js`
- `cad/js/renderer` → `cad/renderer` (+ `js/` only if it has real source of its own beyond aggregating; otherwise it becomes a plain grouping folder, mirroring how `cad/js` itself dissolves)
- `cad/js/renderer/core` → `cad/renderer/core/js`
- `cad/js/renderer/react` → `cad/renderer/react/js`
- `cad/js/module/spatial-shape` → `cad/module/spatial-shape/js`
- `cad/js/module/aec-building` → `cad/module/aec-building/js`
- `cad/js/module/aec-building-structure` → `cad/module/aec-building-structure/js`
- `cad/js/module/aec-building-energy` → `cad/module/aec-building-energy/js`
- `cad/js/machine/stately` → `cad/machine/stately/js`

`cad/rs` (already a top-level sibling, already compliant) is unaffected. After this, `cad/js` no longer exists as a directory.

## Genuinely mixed-language bundles (need multiple language folders in one bundle)

Confirmed by inspection — these have real source files of two languages directly co-located, not in separate nested bundles:

- [flow/core](flow/core) — `index.ts` (real TS logic) + `lib.rs` (real Rust engine) → split into `flow/core/js/index.ts` and `flow/core/rs/{lib.rs,Cargo.toml}`; `package.json` stays at `flow/core/` with `exports` updated (`"." : "./js/index.ts"`, `"./pkg/flow_core.js"` → `"./rs/pkg/flow_core.js"`)
- [coda/client/bin/assistant](coda/client/bin/assistant) — `main.py` (Python) + `mcp-app.tsx`/`vite.mcp-app.config.ts`/`mcp-app.html` (TS) → split into `.../py/{main.py,pyproject.toml,uv.lock}` and `.../js/{mcp-app.tsx,mcp-app.html,vite.mcp-app.config.ts}`
- [compose/client/bin/engine](compose/client/bin/engine) — same pattern as above (`main.py` + `mcp-app.tsx`/vite config) → split into `py/` and `js/`
- [repo/server/coordinator](repo/server/coordinator) — `main.go`/`go.mod`/`go.sum` (Go backend) + Next.js `app/`, `next.config.ts`, `tsconfig.json`, `next-env.d.ts` (TS frontend) → split into `go/` and `js/`; `Dockerfile`/`Caddyfile`/`.env.example` stay at bundle root since they orchestrate both

Everywhere else, a `Cargo.toml`/`package.json` pair in the same folder is just Rust-source-plus-npm-packaging-metadata for the same crate (e.g. `reasoning/mindmap`, `trinity/ram`, `mathematical/graph/dsl`, `mathematical/graph/manifest`, `mathematical/graph/port/directed/dag`, `compose/client/lib/query`, `compose/client/bin/store`, `compose/client/ui/3dm/Compose.Rhino`) — single language, `package.json`/`.csproj`-adjacent packaging stays with that language's folder or bundle root per the rule above, no split needed. Where such a folder also contains a separately-manifested nested bundle (e.g. `mathematical/graph/dsl/core`, `mathematical/graph/manifest/core`, `mathematical/graph/port/directed/dag/{core,react}`), that nested bundle is handled independently by the same rule (its own `js/`).

## Representative transformations (bulk of ~150 remaining bundles)

- TypeScript playground core, e.g. [note/core](note/core): `index.ts`, `internal.ts`, `example-slugs.ts`, `play-ids.ts` → `note/core/js/`; `package.json` (`exports: "." → "./js/index.ts"`), `project.json`, `script.ts`, `vitest.config.ts` stay at `note/core/` (vitest config likely also moves to `js/` if it only targets TS tests — verify per bundle)
- Rust engine with no wrapper, e.g. [kernel/2d/engine](kernel/2d/engine): `lib.rs`, `Cargo.toml` → `kernel/2d/engine/rs/`; `project.json`, `script.ts` stay, `script.ts`'s `cargo test -p ...` gets `cwd` updated to the `rs/` subfolder
- Rust engine with npm shim in same dir, e.g. [trinity/jack/core](trinity/jack/core): `lib.rs`, `queryable.rs`, `Cargo.toml` → `trinity/jack/core/rs/`
- Python, e.g. `ui/styling/py`-style pattern applied to any new py-only bundle: `main.py`, `pyproject.toml`, `uv.lock` → `<bundle>/py/`
- Go, e.g. [repo/client/cli](repo/client/cli): `main.go`, `main_test.go`, `go.mod`, `go.sum` → `repo/client/cli/go/`; `package.json` (pure nx metadata, no source) stays at bundle root
- C#, e.g. [compose/client/ui/3dm/Compose.Rhino](compose/client/ui/3dm/Compose.Rhino): `Compose.Rhino.cs`, `Compose.Rhino.csproj`, `Properties/`, `bin/`, `obj/` → `.../Compose.Rhino/cs/`; `package.json` stays; `Monorepo.sln` project path updated

## Config surfaces to update after every move

- Root [package.json](package.json) — bun workspaces list: **unchanged** for bundles whose `package.json` stays at the bundle root (the common case); only changes for the `cad/js` dismantling (paths move) and for bundles whose `package.json` itself moves (none, per the rule above)
- Root [Cargo.toml](Cargo.toml) — workspace `members` list: update every relocated crate's path to append `/rs`; update inter-crate `path = "../xxx"` dependencies since relative depth changes
- Root `go.work` — update `use` paths for every relocated Go module to append `/go`
- Each `package.json`'s `exports`/`main`/`module`/`types`/`sourceRoot` fields — repoint into the new language subfolder
- Each moved bundle's `script.ts` — update relative imports to `repo/lib/js` (depth may not change since script.ts itself doesn't move) and any `cwd`/`--manifest-path`/working-directory arguments passed to `cargo`/`go`/`uv`/wasm-pack commands
- `Monorepo.sln` — update the 7 `.csproj` relative paths once C# bundles gain a `cs/` folder
- Any `runWasmPackWebBuild({ rsDir, ... })` calls in `script.ts` — `rsDir` becomes `<bundle>/rs`
- Cross-bundle relative imports that bypass package `exports` (deep relative paths crossing bundle boundaries) — search and fix; imports via package name (`@semio-tech/xxx`) are insulated by the `exports` remap and need no change
- `repo/lib/js` playground registry (`PLAYGROUND_PORTS`, `folderRoot`-style path references) if any hardcode bundle-internal file paths

## Execution approach

1. Open a ticket via the repo MCP (after checking `repo://goals`) for this restructuring.
2. Build a generated inventory (script placed in the ticket folder, kept afterward per repo convention) that enumerates every bundle, its detected language(s), and computes the exact move-list — this is mechanical and error-prone to hand-write for 250+ bundles.
3. Execute moves domain-by-domain (batches: `kernel`, `trinity`, `flow`+`mathematical`, `cad`, `framework`+`ui`+`repo`, remaining `*/core`+`*/react` playground technologies, `compose`, `coda`, `mit-bestand`) using `git mv` so history is preserved, immediately followed by the config-surface fixes for that batch.
4. After each batch: `bun install`, `cargo build --workspace`, `go build ./...` (where touched), relevant `vitest`/`uv` runs, and a check for dangling relative imports, before moving to the next batch.
5. Close out with a full-repo `nx run-many` test sweep and a search for any remaining hardcoded old paths (docs, launch.json commands, dev server configs, playground registry).
6. Close the ticket with a summary and the full list of touched files.
   </plan>
   <todos>
   [{"id":"ticket-open","content":"Open repo ticket for the language-tag restructuring after reviewing repo://goals"},{"id":"inventory-script","content":"Write ticket-scoped script to enumerate bundles, detected languages, and compute exact move-list"},{"id":"cad-js-dismantle","content":"Dismantle cad/js umbrella: relocate all 12 child bundles up a level and insert local js/ folders"},{"id":"mixed-bundles","content":"Split the 4 genuinely mixed-language bundles (flow/core, coda/client/bin/assistant, compose/client/bin/engine, repo/server/coordinator) into per-language subfolders"},{"id":"batch-kernel-trinity","content":"Restructure kernel/ and trinity/ bundles, update Cargo.toml members and path deps"},{"id":"batch-flow-mathematical","content":"Restructure flow/, mathematical/, imperative/, neural/, sequence/, lowpoly/ bundles"},{"id":"batch-framework-ui-repo","content":"Restructure framework/, ui/, repo/ bundles including go.work updates"},{"id":"batch-playground-tech","content":"Restructure remaining */core + */react playground technologies (draw, note, forms, gis, layout, procedural, puzzle, raster, reasoning, s, shooting, vcs, writer)"},{"id":"batch-compose-coda-mitbestand","content":"Restructure compose/, coda/, mit-bestand/ bundles including .csproj/.sln updates"},{"id":"update-config-surfaces","content":"Update root package.json workspaces, Cargo.toml, go.work, per-bundle package.json exports, script.ts paths, Monorepo.sln"},{"id":"verify-build","content":"Run bun install, cargo build --workspace, go build, vitest/uv sweeps per batch and fix dangling imports"},{"id":"ticket-close","content":"Close ticket with summary and full list of touched files"}]
