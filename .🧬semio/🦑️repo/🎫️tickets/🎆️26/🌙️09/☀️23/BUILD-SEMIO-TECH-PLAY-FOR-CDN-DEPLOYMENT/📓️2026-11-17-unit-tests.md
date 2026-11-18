# Play unit tests and typecheck (2026-11-17)

## Commands
1. `repo run @semio-tech/semio-tech-play:test` stalled for over 4 minutes on "Reading the project graph slice" (contention with the running release build); stopped. Log: `🗑️generated/unit/test-1.log`.
2. Fallback, from the play dir: `/Users/ueli/Documents/semio/node_modules/.bin/vitest run --config 🧪️tests/🎚️config/🟦️.ts` (no `bun`/`bunx` on PATH). Log: `🗑️generated/unit/vitest-direct.log`.
3. Typecheck: `node_modules/.bin/tsc --noEmit -p <ticket>/play-tsconfig.json` (extends root tsconfig, includes only play sources; follows imports into the framework). Log: `🗑️generated/unit/tsc-1.log`.

## Results
- vitest: 8 files passed, 81 tests passed, 0 failed (before and after; no source changes were needed). Config includes only `🧪️playscopes` plus in-source tests of brand, tile-serve-mode, runtime, assets, activation, pages, fresh-build.
- Bun-only specs (`🧪️playrelease`, `🧩️release-job-order`, publication tests) are not part of the vitest include and were not run (no bun installed here).
- tsc: 665 errors, none attributable to stale imports or renamed modules in play source:
  - 37 TS2307 all target missing build products: `🤖️generated/*` (manifest, actor, tokens, shell, ui-contract), wasm packages (`@semio-tech/puzzle-wasm`, `flow-core`, `framework-editor-rs`, `framework-surface-rs`), and `bun:test`/`bun:sqlite`.
  - TS2305/TS2459 on `📦️packages/🟦️typescript/🐚️shell` cascade from the missing `🔨️modules/🐚️shell/🤖️generated`.
  - TS7006/TS2339 (`import.meta.dir`, `Bun`) come from missing Bun types in the scope config and implicit-any cascades from missing generated types.
  - Play-own: `🪧️brand.ts:77` TS7006 (cascade), `🧪️tests/🧪️playscopes/🟦️.ts:29` TS2538 (test-only strictness, `fixture.dev` possibly undefined); neither affects the Vite bundle.

## Files changed
None (only the scoped `play-tsconfig.json` and this report added to the ticket folder).
