# Result tsc2: remaining source type errors after tsc / tscp2

No `package.json` / `bun.lock` edit, no cargo, no modifying git command, nothing under `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**`. `@semio-tech/flow-core` skipped as instructed.

## Method
`NODE_OPTIONS=--max-old-space-size=12288 node node_modules/typescript/bin/tsc --noEmit -p <tsconfig> --pretty false --extendedDiagnostics` (installed typescript 5.9.3), foreground, one config at a time (`C/tsc-tools/run2.sh <tsconfig> <tag>`). Every run reports `Files` 2,514 to 14,073 and Check time 56 to 477 s (machine load average ~40 during the later runs). Logs deleted after summarizing.

## Before / after (error counts)
| config | before | after | what remains |
| --- | --- | --- | --- |
| framework `📦️packages/🟦️typescript` | 46 | 41 | 0 source: opentype.js, check-error, d3-array/color/ease/scale, pngjs (+2 derived `unknown[]`), color-convert types; tailwindcss, @tailwindcss/postcss, @dnd-kit/*, react-resizable-panels, class-variance-authority (+derived TS7006) |
| os `tsconfig.json` | 41 | 32 | 0 source: @dnd-kit/*, react-resizable-panels, katex, jsdom, pngjs, `@webgpu/types` (5), flow-core (3), stale `flow_core.js` bindings (`semioSetRuntimeAssets`) |
| repo `tsconfig.json` | 9 | 7 | 3 deps (strip-ansi + 2 derived, smart-buffer); 3 in `🏗️build/📜️script.ts` + `🧭️selection/👁️watch/🧪️tests/🎟️phase` (`childReaderPort`, `childRunnerPort`, `{initial}` without `child`): a peer's in-flight build-watch change, files modified minutes earlier, not touched |
| `✏️s/tsconfig.json` | 63 | 58 (57 after the last edit, not re-run) | 0 framework source: only katex, flow-core; plugin side: unbuilt `@semio-tech/*` outputs, brepjs, d3-geo, stale `trinity_jack_lsp.js` |
| ui react `…/⚛️react/📦️packages/🟦️typescript` | 21 | 21 | all deps (17 missing modules/types + 4 derived TS7006) |
| hub admin | 48 | 48 | all deps (13 missing modules, 31 duplicate `@types/react`, 4 derived) |

Note the framework 41 and repo/os counts include errors that a peer's concurrent edit fixed between runs (wgpu server config, store test, shard-client).

## Source fixes (file list)
- `🏃️process/📦️artifacts/🏗️native-build/🎯️recipe/🧪️tests/🟦️.ts`: the production-recipe test wraps in `withIssuedLawStorageV1` and builds all three `buildCargoArtifacts` option objects with `lawBuildOptionsV1(law, control, {environment})` (supplies `consume`, `maximumObservationBytes`, `storage`, `sourceBoundary`).
- `🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🧪️tests/🟦️.ts`: `originalCargoStreamingV1` supplies `control` (`scriptCompilerAdmissionControlV1`), `storage` (`withIssuedLawStorageV1`) and `sourceBoundary` (`lawSourceBoundaryV1`) to `cargoStreamingStatus`; storage env merged into the child environment.
- `…/library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🟦️.ts`: dropped the stale 6th argument (30000) of `withRepositoryCacheStoreV1`.
- `…/library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️purpose/📜️script.ts`: `physicalWorkspace!` at the `statfs` call (closure-assigned `string | null`).
- `…/library/🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🏺️vendored-exclusion/🟦️.ts`: rule objects typed as `IForbiddenRuleType[]` (readonly vs mutable arrays).
- `🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🟦️.ts` and `…/🗣️Interpreter/🧵️worker-cell/🧪️tests/🟦️.ts`: removed a duplicate `readModuleTreeSource` import (TS2300).
- `…/📃️UiDocumentStore/🧪️tests/🧠️resident-pool-and-metadata/🟦️.tsx`: `originalScope` typed `object | null` (it receives both resident instances and `OwnedUiResidentScope`).
- `…/🔌️plugin/🧪️tests/📦️units/🟦️.ts`: `featureLocks.find(...)!.thirdpartyPinsSha256` (3x).
- `…/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🚚️vendor-transport/🟦️.ts`: the removed repository-global `pluginOutRoot` is replaced by a local `pluginModulesRoot = builtPluginModulesRootIn(repoRoot, "prototype")` (the canonical selected-mode modules root, which the plugin route now serves); the plugin module-root loop uses it directly (no `dirname(...)/<directory>` join). A peer had started this same change (line 368 only) and left the destructure and the other uses broken; I completed it. The test still skips unless built output exists.
- `✏️s/🔌️plugins/🗄️stdio/…/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🫙️prefix/🟦️.ts`: `row.value as number` (union from the JSON fixture does not narrow).

## Still open
- Dependencies (manifests frozen): same list as result-tsc section 3 plus `strip-ansi`, `smart-buffer`, `@types/jsdom`, `@types/color-convert`, `check-error` typings, `d3-geo`, `brepjs`.
- `🌊️flow/🎁️browser-package/…/🦀️rust/🟦️.ts`: generated `🕸️bindings/flow_core.js` lacks `semioSetRuntimeAssets` (needs the flow-core wasm build; not a source defect).
- repo `🏗️build` watch peer change (3 errors above).
- Not verified at runtime: only type-checking was run, none of the edited tests were executed (they need Bun, issued invocations and, for the recipe/runtime tests, a Native storage image).
