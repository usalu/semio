# Result tscp2: TypeScript typecheck target for all of `✏️s/` and the remaining source errors

Executor Sonnet 5.5, foreground, no cargo, no sub-agents, no modifying git command, no `package.json` / `bun.lock` / storage / revision edit (freeze respected).

## Decision (AGENTS.md: every package typechecked, zero-touch)
One family `tsconfig` covering every TypeScript source of `✏️s/`, registered as a `typecheck` target. No per-package `tsconfig.json` is generated for the ~170 plugin packages: no tsconfig generator or schema exists in the repo (searched), the existing precedent for a family config is `🧰️framework/🛍️products/💻️os/tsconfig.json`, and 140 hand-copied files would only be a drift surface. The 5 existing package tsconfigs (dev/cad, 4 cad extensions) stay untouched and independent.
- Target `@semio-tech/s-dev-composition:typecheck` (the project that already owns `✏️s/🧑‍💻dev` and is registered in `📋️build-owners.json`, so no owner registry change): `bun ./📜️script.ts typecheck` -> `typecheckRepositoryProjectV1` with `project: "../tsconfig.json"` (same helper the quiz uses), `timeoutMs` 30 min (a whole-tree check takes about 4 minutes).
- `dependsOn` lists the build outputs the sources import by package name: nine `:build` targets (`writer-writer`, `gis-gisterrain`, `gis-gismap-js`, `lowpoly-lowpoly`, `reasoning-wires`, `forms-js`, `space-home`, `space-space`, `stdio-docx`) and two wasm targets (`trinity-jack-lsp:wasm`, `puzzle-plugin:wasm`). Not run (builds are the coordinator's); this is the part still unverified, see below.
- launch.json and seed: `🛠️dev✏️s🪁️typecheck`, group `3_dev`, order 390.3, right after the `🛠️dev💻️os🪁️typecheck` entry (same shape as its siblings).
- The package manifests need no `typecheck` script: `✏️s/🧑‍💻dev` has no `package.json`, the project is a pure `📋️project.json` project.

## Files changed
- `✏️s/tsconfig.json` (new): extends the root config, includes `**/*.ts(x)`, the root's exclusions re-listed (extending replaces `exclude`), `paths` for `@semio-tech/puzzle-wasm` -> `🌎️hub/…/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.d.ts` (the package lives in the hub workspace, which `✏️s` does not contain).
- `✏️s/🧑‍💻dev/📜️script.ts`: `TypecheckScript` + `.register("typecheck", …)` + one import.
- `✏️s/🧑‍💻dev/📋️project.json`: `typecheck` target.
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`: one entry each (launch.json re-parsed: 13,816 configurations).
- Source fix, architect: `✏️s/🔌️plugins/🏛️architect/…/🪆️subsets/✳️any/🧬️schema/🟦️.ts` re-exports (type-only) the row types it already imports from `🏗️program-structure`, `📋️requirements`, `🗃️decision-records`. The snapshot schema declares this file as the TypeScript row owner (`x-semio-collection-catalog.typescript.rows = "../../🟦️.ts"`), and the regenerated `🗄️registers/🗂️catalog/🟦️.ts` imports all 65 row types from it; they were imported but not exported (64 errors).
- Source fix, puzzle stories: `🎭️2d-fixtures` and `🎭️3d-world` imported `…/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.js`, a directory that no longer exists (the wasm package moved to the hub composition); they now import `@semio-tech/puzzle-wasm` like `🎭️5d-timeline` and their docstrings already say. This also removes the 2 TS2307 on the old path; the 3rd (5d) resolves through the new `paths`.

## Verification (installed typescript 5.9.3, `node --max-old-space-size=12288 node_modules/typescript/bin/tsc --noEmit -p ✏️s/tsconfig.json --extendedDiagnostics`, foreground)
Proof the checker ran: Files 14,089 / 14,092, 607k lines of TypeScript, Check time 177 s / 179 s, total 247 s / 218 s, 4.3-4.8 GB.
| | errors | in `✏️s/` | in `🧰️framework/` (imported by `✏️s/`) |
|---|---|---|---|
| before (new config, nothing fixed) | 152 | 106 | 46 |
| after | 63 | 38 | 25 |
The previous report's 70 was before a peer's catalog regeneration added the 64 architect errors. One transient error in the stdio `🫙️prefix` test (`hex.cancelled.repeat` possibly undefined) is a peer's in-flight test: its fixture `🧵️hex-octets/🔣️.json` was created at 22:55 with `repeat: 1024`, which resolves it; not touched.

## Remaining errors (none is a source defect I can fix in `✏️s/`)
For the deps executor (manifests frozen, nothing edited):
- Missing types: `@types/pngjs` (13 files in wfc, procedural, draw, stdio png), `@types/graphlib` (2), `@types/opentype.js` (1), `@types/d3-hierarchy` (raster, 2 + 6 implicit-any follow-ups).
- Missing packages: `brepjs` (cad `⚙️engine/🧱️brepjs` + 3 stdio step/semio generator scripts; only `brepjs-opencascade` is declared), `d3-geo` (gismap tiles test), `katex`, `@dnd-kit/core|sortable|utilities`, `react-resizable-panels` (framework ui, 14 + 4 implicit-any).
Build outputs (covered by the new `dependsOn`, not run, so unverified that they clear): `@semio-tech/<self>` imports in 9 `🪶️sqlite`/consumer tests and `@semio-tech/stdio-docx`; `JackLspSession` missing from the stale `trinity_jack_lsp.js` wasm pkg (needs `trinity-jack-lsp:wasm`).
For the tsc/framework executor (not in `✏️s/`, not touched): `🧰️framework/🛍️products/💻️os/…/🧊️wgpu/🌐️server/🎚️config/🟦️.ts` (missing exports `semioServeCloseVitePlugin`, `semioBackboneVitePlugin`) and `…/🌐️server/🟦️.ts` (`staticDirVitePlugin`, +1 implicit any); `🧱️elements/📃️UiDocumentStore/🧪️tests/🧠️resident-pool-and-metadata` line 117 (`OwnedUiResidentScope` vs `OwnedUiResidentInstance`); `@semio-tech/flow-core` unresolved in os `🧑‍💻dev/🪝️host-ports/🕸️wasm-bindings` (the `node_modules` link points at an absent `🕸️bindings` output).
Not done: a language-agnostic test for the target; the target is itself the gate and `verify typescript` already lists sources from the live checkout. The `bun repo run @semio-tech/s-dev-composition:typecheck` route itself was not executed (graph load and builds are serialized by the coordinator); the helper call mirrors the quiz and os scripts verbatim apart from `project`/`owner`/`timeoutMs`.
Scratch output deleted.
