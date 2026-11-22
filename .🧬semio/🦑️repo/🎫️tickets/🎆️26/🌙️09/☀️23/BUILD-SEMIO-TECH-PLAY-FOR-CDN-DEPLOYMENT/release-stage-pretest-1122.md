# Release-stage pretest (2026-11-22)

Runner: `$BUN test ./<path>` with `SEMIO_BUILD_MODE=release`. Raw output: `.🧬semio/🌐hub/play-cdn-1122/pretest/`.

| Test | Result |
|---|---|
| `🧪️tests/🧪️playrelease/🟦️.ts` (routing) | 13 pass, 0 fail |
| `📦️site/📄pages/📦️publication/📤️export/🧪️tests/🟦️.ts` (export) | 2 pass, 0 fail |
| `📦️site/📄pages/📦️publication/🧪️tests/🟦️.ts` | 5 pass, 0 fail |
| `🧪️tests/🧩️release-job-order/🟦️.ts` | 2 pass, 0 fail |
| `🧪️tests/🧪️playscopes/🟦️.ts` | 5 pass, 0 fail |

Total: 27 pass, 0 fail.

Not runnable under bun test (0 tests registered, vitest `registerTests*` or Playwright-only; they belong to the `test` / `test-e2e` targets, not to the later stages): playpanecoverage, playmaptiles, playfreshbuild, playgrid, playpages, playpanedefaults, playactivation, playdevmounts, `🎚️config` (bun: 0 tests), `🎭️acceptance` (Playwright describe, needs Playwright runner). `🔨️modules/🧪️e2e` has no unit tests (script, config, release fixture only).

## Type-check
No typecheck target exists in `📋️project.json`. Ran tsc (strict, noEmit, bundler resolution) over `🧪️e2e`, `📦️site` and `🧪️playrelease`: zero errors in play files. Transitive errors outside play (not touched, peers' areas, none reached by release-verify/test-release/release-export paths as far as the load graph shows):
- `🧰️framework/.../🏗️native-build/🧩️component-pass/🟦️.ts(514)`: `UnitRecordV1` not found
- `.../🖨️describe/🛠️emitter/🟦️.ts(2)`: `RepositoryCompilerStoreV1` export missing
- `.../⚡️caching/🌐️vite/🟦️.ts(121)`: `ResourceLeaseControl` not found
- three `isWellFormed`/`toWellFormed` lib-target errors (artifact of my tsconfig lib).

## Fixes
None required; no files in the repo changed.
