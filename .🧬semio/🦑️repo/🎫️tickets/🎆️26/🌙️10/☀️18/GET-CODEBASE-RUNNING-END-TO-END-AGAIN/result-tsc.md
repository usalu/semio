# Result tsc: TypeScript type checks (framework, teaching, hub)

`C` = this ticket folder. No `package.json`/`bun.lock` edit, no cargo, no modifying git command, no file under `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**` touched (strict freeze respected).

## 0. Method

- Scope: all 23 `tsconfig.json` under `🧰️framework/`, `🎓️teaching/`, `🌎️hub/` (plugins `✏️s/` skipped). `🏢️semio-tech/` and `♻️mit-bestand/` contain no `tsconfig*.json` at all (only `package.json`).
- Command per config, one at a time, foreground: `NODE_OPTIONS=--max-old-space-size=8192 node node_modules/typescript/bin/tsc --noEmit -p <tsconfig> --pretty false --extendedDiagnostics` (installed typescript 5.9.3; `bunx` adds nothing). Runner and config list: `C/tsc-tools/run-one.sh`, `C/tsc-tools/configs.txt` (line N = row N below). Logs: `C/🗑️generated/tsc-logs/{b,a,c,d,e}*.txt` (b = before, c/d/e = after).
- Proof that the type-checker ran (memory note): every row has `Files` of 345 to 6,602 and `Check time` > 0; zero-error rows were re-run for the four `incremental: true` configs (1, 9, 17, 18) with a fresh `--tsBuildInfoFile` (1: 0 errors, 36 s check; 18: 0 errors, 19.5 s check). Error rows carry type-dependent diagnostics (TS2322/TS2345/TS2339).
- Errors overlap between rows (the library tree is pulled in by several configs); the table counts per config.

## 1. Per-package before / after

| # | tsconfig (relative) | Before | After | Remaining is |
| --- | --- | --- | --- | --- |
| 1 | 🌎️hub/📦️packages/🟦️typescript | 49 | 0 | fixed: lib |
| 2 | 🌎️hub/🔨️modules/🛡️admin/…/typescript | 48 | 48 | 13 missing modules, 31 duplicate `@types/react` (deps), 4 derived |
| 3 | 🎓️teaching/…/❓️quiz/…/typescript | 0 | 0 | |
| 4 | 🧰️framework/📦️packages/🟦️typescript | 60 | 46 | 34 missing modules/types (deps), 5 derived TS7006, 2 pngjs-derived, 5 native-build tests (cc API) |
| 5 | framework ◻️2d/🧮️compute | 0 | 0 | |
| 6 | framework 🔄️machine | 3 | 0 | fixed: types |
| 7 | framework 🖱️ui/🌐️locale | 0 | 0 | |
| 8 | framework 🖱️ui/🎓️introduction | 0 | 0 | |
| 9 | framework 🖱️ui/🎯️targets/⚛️react (ui) | 81 | 21 | 17 missing modules/types (deps), 4 derived TS7006 |
| 10 | framework 🖱️ui/📦️packages/🦀️rust | 1 | 0 | fixed: test type |
| 11 | framework 🖼️assets | 3 | 0 | fixed: types |
| 12 | framework 🧊️3d | 0 | 0 | |
| 13 | framework products ❓️quiz/⚛️react | 6 | 6 | 5 missing types (d3-scale, aria-query, opentype.js, jsdom), 1 derived |
| 14 | framework products 🐾️pets/⚛️react | 3 | 3 | aria-query types, 1 derived |
| 15 | framework products 🐾️pets | 1 | 1 | d3-ease types |
| 16 | framework products 💻️os (play, os dev, renderer) | 54 | 33 | 27 deps/derived; 6 open (section 4) |
| 17 | renderer 🧑‍🎨engine/⚛️react | 22 | 20 | 16 missing modules/types (deps), 4 derived |
| 18 | os 🔌️plugin/🪟️window-kits | 29 | 0 | fixed: lib |
| 19 | framework products 🦑️repo | 8 | 6 | strip-ansi (deps), 2 derived, 2 open (section 4) |
| 20 | repo 💻️client/🧩️vscode | 27 | 0 | fixed: lib |
| 21 | repo 📚️library | 3 | 2 | 2 open (section 4) |
| 22 | repo 🖥️server/🎛️coordinator | 0 | 0 | |
| 23 | repo 🧪️test | 0 | 0 | |

Zero-error rows after: 1, 3, 5, 6, 7, 8, 10, 11, 12, 18, 20, 22, 23 (13 of 23).

## 2. Fixes (file list)

Config (not manifests):
- `🌎️hub/📦️packages/🟦️typescript/tsconfig.json`, `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json`: `lib` gains `DOM.Iterable`, `DOM.AsyncIterable` (146 TS2488 `NodeListOf`/`HTMLCollection`/`Headers` iteration errors). The ui config also gains `node` in `types` (the owned-execution module is reachable from ui tests).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/tsconfig.json`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/tsconfig.json`: `lib` gains `DOM`, `DOM.Iterable` (they transitively compile the ui theme and assets code, which touches `document`; 56 errors).
- `🧰️framework/📦️packages/🟦️typescript/tsconfig.json`: include `./🌿️ambient/**/*.d.ts`; `🧰️framework/🛍️products/💻️os/tsconfig.json`: include `../../📦️packages/🟦️typescript/🌿️ambient/**/*.d.ts`. The ambient file was split into per-package children (graphlib, bun-sqlite-statement `safeIntegers`, opentype hinting, d3-shape stack order) but only the root `🟦️.d.ts` stayed in `include`, so the children were invisible (graphlib TS7016, `safeIntegers` TS2339).
- `🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/tsconfig.json`, `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/tsconfig.json`: `types` gains `vitest/importMeta` (`import.meta.vitest` in-source suites).

Source (broken by splits and refactors):
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/🎯️stack-drop-destination/🟦️.tsx`: `applyModeDrop`, `modeDockOutLayout` now come from `../../🎯️drop/🟦️.ts` (the Canvas split moved them out of the barrel).
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧭️native-command/🟦️.ts`: `script` typed as `typeof import(".../📜️script.ts")` (missing `createUiNativeRouter`).
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts`: `admitted` typed `NodeJS.ReadableStream` (union call TS2349).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🪛️typedwire-helpers/🟦️.tsx`: `MAINTENANCE_LANE_DEFAULT_BUDGET` imported from `shard-client/📜️wire` (the redesigned ShardClient barrel no longer exports it).
- `…/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🧾️reserved-response-settlement/🟦️.ts` (private `recomputeOldestPending` via `Reflect.get`), `…/🔓️captured-return-admission/🟦️.ts` (`instanceLifecycles.get(...)!`, `owner.host!`).
- `…/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🧰️test-kit/🟦️.ts`: `External01.getWorkspaceRoot()` to `getWorkspaceRoot()` (import was renamed in the consolidation).
- `…/🖌️Paint2dHost/✍️editing/🧪️tests/🖱️selection-focus/🟦️.tsx` (`as unknown as typeof setTimeout`), `◻️2d/📝️text/…/↔️bidi/🧪️tests/🟦️.ts` (direction cast), `…/📚️library/🧪️tests/🧱️rust-source-direction/🔔️deferred-wake-ownership/🟦️.ts` (typed `neutralGroups`).

## 3. Needed manifest changes (for deps, freeze respected)

Root `package.json` must declare (isolated linker does not hoist; versions already in the store unless noted):
- Types visible at root: `@types/d3-array@3.2.2`, `@types/d3-scale@4.0.9`, `@types/d3-color@3.1.3`, `@types/d3-ease@3.0.2`, `@types/pngjs@6.0.5`, `@types/aria-query@5.0.4`, `@types/opentype.js@1.3.9`.
- Runtime/type packages: `react-resizable-panels@4.14.5`, `@dnd-kit/core@6.3.1`, `@dnd-kit/sortable@10.0.0`, `@dnd-kit/utilities@3.2.2`, `class-variance-authority@0.7.1`, `tailwindcss@4.3.3`, `katex@0.16.47`, `strip-ansi` (6.0.1 / 7.2.0 in store), `@webgpu/types@0.1.74`.
- Not in the store (new install): `@types/jsdom`, `smart-buffer`, `@tailwindcss/postcss`, `@types/color-convert`, a typings source for `check-error`.
- Duplicate React types: hub admin `node_modules` resolves `@types/react` from `.cache/b/d/m/j/02ad8f36…/node_modules/.bun`, the root from `…/4151d2262cb4a4e7a25a/…`. Two physical copies of 19.3.0 make 31 `Ref<HTMLXElement>` errors (TS2322). One generation must serve both.

## 4. Still open (not fixed)

- `native-build/🎯️recipe/🧪️tests/🟦️.ts` (3x) and `native-build/💾️storage/🧪️tests/🟦️.ts` (2x): tests built before the cargo launcher API (`CargoArtifactBuildOptionsV1` now needs `consume`, `maximumObservationBytes`, `storage`, `sourceBoundary`; `NativeStoragePortV1.withOperation` generic). Both files were being edited by peers (cc). Owner: cc.
- `📚️library/🔍️discovery/🕸️runtime/🧪️tests/🟦️.ts:496` (`cargoStreamingStatus` needs `control, storage, sourceBoundary`, already known) and `…/🔌️nx/📦️graph/🧪️tests/📥️purpose/📜️script.ts:10` (`string | null`; Nx pool, nxpart area).
- `🔌️plugin/🧪️tests/📦️units/🟦️.ts` (3x TS2769, `string | undefined`) and `⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts:166` (`boolean | undefined`): peer edits in flight (files modified in the working tree).
- `🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🚚️vendor-transport/🟦️.ts`: `pluginOutRoot` was removed in the plugin build refactor (`pluginModulesRoot` no longer exists either); the test needs a decision on the vendor root, not a type fix.
- `🌊️flow/🫀️core/…/🦀️rust/🟦️.ts:1`: generated `🕸️bindings/flow_core.js` lacks `semioSetRuntimeAssets` (stale wasm-bindgen output; regenerate with the flow-core build).
- During the last pass a peer was mid-split on `🧊️wgpu/🚚️browser-frame-transport` (importers briefly broken, 55 errors in `d16.txt`); the final os run `e16.txt` is clean of it.
- Not run: plugins (`✏️s/`), as instructed.
