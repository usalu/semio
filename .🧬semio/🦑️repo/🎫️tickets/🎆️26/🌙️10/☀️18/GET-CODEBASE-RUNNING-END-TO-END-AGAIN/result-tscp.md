# Result tscp: TypeScript type check of `✏️s/` (plugins and dev)

Executor Sonnet 5.5, foreground, no cargo, no sub-agents, no modifying git command, no `package.json` / `bun.lock` / `💾️storage/**` / `🏗️build/📥️inputs/🔏️revision/**` edit (strict freeze respected).

## Scope finding
Only 5 `tsconfig.json` exist under `✏️s/` (all `📦️packages/🟦️typescript/tsconfig.json`); the other ~170 plugin TS packages have none and are covered only by the root `tsconfig.json`. So I ran both: the 5 package configs, and a whole-`✏️s/` run with a scratch config extending the root one (include `✏️s/**/*.ts(x)`, excluding node_modules, generated, dist).

## Runs (`bunx tsc --noEmit -p …`, typescript 5.9.3)
Proof the checker ran: `--extendedDiagnostics` Check time 8-25 s, 1,044-1,256 files, 74-78k lines per package config.

| config | errors before | after |
|---|---|---|
| dev/cad | 0 | 0 |
| plugins/cad aec-building, aec-building-structure, aec-building-energy, spatial-shape | 1 each (`Cannot find module 'brepjs'`) | 1 each (missing package); 0 when `paths` maps `brepjs` to the older store copy `brepjs@18.164.3` (scratch config, run in this session) |
| whole `✏️s/` (14,038 files, 607k lines, Check 501 s, 4.2 GB) | 70 | 70 minus 11 fixed below (re-verified by a 7-file scratch config: only missing-package and framework errors remain) |

## Source fixes (real errors from refactors)
- `🌍️gis/…/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:219`: `readModuleTreeSource(path,"utf8")` passed an encoding as `ModuleTreeOptions`; now module-tree reads for the inference worker and the (split) store worker.
- `🗄️stdio/…/🧿️semio/📦️packages/🦀️rust/📜️script.ts:44`: `prepareJavascriptDependencies` is no longer exported from `…/📦️dependencies/🟦️.ts`; the owner is `…/📦️dependencies/💾️storage/🟦️.ts` (signature gained `selection`); both imports now come from the owner, call passes `{kind:"all"}`.
- `🖍️draw` svg write and svg document tests: `documentElement.localName` -> `documentElement?.localName` (2 TS2531).
- `🗄️stdio/…/💬️bcf/…/🧬️schema/🔺️diff/🟦️.ts`: `BcfDiff.bytes` was `number[]`, schema/graphql/proto/parser say string -> `string`.
- `🗄️stdio/…/🧿️semio/…/🔺️mesh/🧬️schema/🔺️diff/🟦️.ts`: three parser members cast to their `NamedTripleDiff` types (schema only says `object`; 3 TS2322).
- `🗄️stdio/…/☁️las/…/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:68`: `safeIntegers` is not in the bun:sqlite types; cast like the repo's other 9 uses.

## Still open
For the deps executor (manifest freeze, nothing edited):
- Packages with no type resolution: `brepjs` (never in `✏️s/package.json`/`bun.lock`; only `brepjs-opencascade`; plugins/cad `⚙️engine/🧱️brepjs` and the stdio step/semio generator scripts import it; older store generations hold 18.164.3), `d3-geo`, `katex`, `@dnd-kit/core|sortable|utilities`, `react-resizable-panels` (framework `🖱️ui`, 14 errors incl. 5 implicit-any follow-ups).
- Missing declarations: `@types/pngjs` (11), `@types/graphlib`, `@types/opentype.js`, `@types/d3-hierarchy` (+4 implicit-any follow-ups in `🖨️raster`).
Build outputs not present (not source errors; need `<pkg>:build` / wasm build): `@semio-tech/*` self-imports in 8 `🪶️sqlite` tests (writer-writer, gis-gisterrain, gis-gismap-js, lowpoly-lowpoly, reasoning-wires, space-home, space-space, forms-js) and `@semio-tech/stdio-docx` (no `dist`, used by `✏️s/🧑‍💻dev/🗄️stdio/🟦️.ts`); `@semio-tech/puzzle-wasm` and `semio_puzzle.js` pkg (3 puzzle story/wasm files); trinity `🔌️jack/🧠️lsp/…/pkg` is a stale Aug wasm-pack output without `JackLspSession` (needs wasm rebuild).
For the `tsc` executor (framework, not touched): `🧰️framework/🛍️products/💻️os/…/📃️UiDocumentStore/🧪️tests/🪛️typedwire-helpers/🟦️.tsx(30)`: `MAINTENANCE_LANE_DEFAULT_BUDGET` no longer exported by the module it imports.
Note: the remaining ~170 plugin packages have no tsconfig; a per-package `tsconfig` is not added (would be new config files; whole-tree run above is the coverage).
Scratch output (`🗑️generated/tscp-out`) deleted.
