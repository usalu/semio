# r2-ty: typecheck and catalog loading

## Task A, typecheck
The staffage package already defines `typecheck` (`📦️packages/🟦️typescript/📋️project.json`, `repo ./📜️script.ts typecheck`). `repo show_targets` and the MCP graph queries hung for 22 minutes (dependency sync stale), so tsc was run directly with `repo tools exec bun -- x tsc`.

Fixed at the root, no casts or ts-ignore:
- `🧪️tests/🧍️front-projection/🟦️.ts:134,151` and `🧪️tests/🧍️plan-projection/🟦️.ts:196`: removed `{ language: "en" }` from `exportCast` calls (casts are language-independent since r2-lg).
- `🧪️tests/🧍️front-projection/🟦️.ts:129`: `indexes` typed `readonly CastIndex[]` (TS4104).
- `🧫️fixtures/🌡️thermal/🟦️.ts:58`: `shapes` gets `front: []`.

Result (ticket tsconfigs `🧰️ty/tsconfig.lib.json`, `🧰️ty/tsconfig.react.json`, extending the real ones):
- Library (barrel, schema, modules, tests, fixtures): 0 errors.
- React target: 0 errors.

Environment gaps, not code (repo infra, left alone): `d3-format` and `@types/d3-format` are declared in the root package.json but not linked in root `node_modules`; `@semio-tech/framework-transient` is unlinked. The stock `tsconfig.json` therefore reports TS2307 for `print-size:8` and in `🧰️framework/.../🖱️ui`; the ticket tsconfigs map both to the `node_modules/.bun` store.

## Task B, catalog loading
- First run: `catalog-loading` passed 7/7 (the earlier 15 failed assets were mid-edit state).
- Full catalog check: rc 0, 147/147 ready.
- Library vitest: 1120 passed, 11 skipped, 2 failed, both radiator, both from agent ap's in-flight radiator edit (not mine):
  - `front-projection`: `radiator-side height 0.71 > 0.83` (vector in fixture is stale).
  - `export-performance`: pinned digest of `radiator-transient/0000.svg` differs (pinned digest in `🧫️fixtures/🚀️export-performance/🔣️.json` is stale).
- React vitest: 116 passed, 1 skipped, 1 failed: `catalog-loading` row count 148 vs manifest 147. Cause: `🏗️machinery/solar-thermal-collector/🔣️.json` exists (modified 18:34) but is not in `🗂️catalog/🔣️.json`; the store appends it as an extra row. Another agent is authoring it; I did not touch the manifest. It stays failing until that agent adds the manifest entry (or removes the asset). Checked again 10 min later: still not in the manifest.

## Open
- Add `solar-thermal-collector` to the manifest (owner of that asset).
- Radiator owner (ap): refresh the front-projection height vector and the export-performance digest.
- Re-run `🧰️ty/final.sh` (catalog, library, React) once both settle.

Scripts kept in `🧰️ty/`: `tsconfig.lib.json`, `tsconfig.react.json`, `final.sh`, `orphans.ts`.
