# Dev config tests: transform-freshness failures

Result: 54 cases, 52 pass, 2 fail (was 46 pass / 7 fail; the suite has 54 cases, so the earlier count was 53).

## Root cause of the 5 freshness failures: Windows path handling, not `.cache`

- `.cache` in `UNWATCHED_REPOSITORY_SEGMENTS` is unrelated. The tests use `os.tmpdir()` (`C:\Users\...\AppData\Local\Temp`) and `cacheDir: <sandbox>/.vite`; nothing lives under `.cache`, and `server.fs` is untouched.
- The tests requested `/@fs${target}`, which is only valid for POSIX paths. On Windows that is `/@fsC:\...`, a 404 with an empty body. Vite wants `/@fs/C:/...`. (Probe: `/@fs/C:/...` and `/<root-relative>` both answered 200.)
- Product bug behind it: `requestedTransformFile` returned `/C:/...` for `/@fs/C:/...`, which can never be statted, so the stat-guard silently never fired on Windows. The registry also keyed Vite ids (`C:/x/y.ts`) differently from watcher paths (`C:\x\y.ts`).
- The sed write style used BSD-only `sed -i ''`, which GNU sed (Git Bash) reads as an empty file name.

## Changes

- `🧑‍💻dev/🔌️vite-plugins/🔖️source-freshness/🟦️.ts`: `requestedTransformFile` strips the leading slash from `/@fs/<drive>:/...` and normalizes it. The registry normalizes every key (`record`, `tracks`, `movedFile`, `movedInDirectory`).
- `🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts`: added `fsUrl` helper; the `/root` case now expects `join(...)`; added a `/@fs/C:/...` case; sed uses the `''` argument only on darwin.

## Remaining 2 failures: separate drift, not touched

Both come from the 2026-11-18 restructure (commits 3d62b2ef22c5 and 3cde99d423d7), unrelated to freshness or `.cache`:

1. `vite config module graph > stays inside the declared module and source-byte bounds` (808 modules against a bound of 40). The config statically imports `catalog-view` and the playground catalog. `catalog-view` reaches `📦️packages/🟦️typescript/📇️inventory/🟨️.mjs` and from there `🕸️dependencies/🟦️typescript/🟨️.mjs`. That file now imports the whole `🏗️build/🚀️bootstrap` and `👁️observation` closure (about 445 modules), and the playground loader pulls `🧰️framework/📦️packages/🟦️typescript` (about 210). Fix: make that import lazy or split the light parts out, not raise the bound.
2. `browser entry module graph > still reaches every module...`: `🎨️styling/🌓️theme/🟦️.ts` is still listed in `🌐️browser-graph.json` `requireModules` but the browser graph no longer reaches it (the styling barrel moved).

## Follow-up: the two remaining failures fixed (54 of 54 pass)

1. Config module graph (808 modules against a bound of 40). `🏗️builder/🌐️vite/🟦️.ts` imported `readGeneratedCatalogProjection` and `loadFrameworkOsPlaygroundCatalog` only as `void` "graph anchors"; nothing used them. Both static imports and the two `void` lines are gone. `catalog-view` and the `📦️packages/🟦️typescript/🎮️playground` loader moved from `require` to `lazy` in `⚙️config-graph.json`, so the test still proves each resolves as a complete graph of its own. The bound of 40 is kept. The build/bootstrap/observation closure is no longer statically reachable from the config.
2. Browser graph. The theme module was not broken at runtime. Since the 2026-11-18 restructure the renderer imports `@semio-tech/framework-os/modules/...` subpaths, which Vite resolves through the os package's `exports` map. The test resolver only did prefix aliases, so it saw those as leaf package edges and stopped, losing the whole react, ShellHost and theme reach (it also missed `ui-react`, ShellHost and ShellFooter, not just theme). `🧪️tests/🧹️config/🟦️.ts` now resolves `@semio-tech/framework-os/<subpath>` through that exports map. All required modules, including `🌓️theme/🟦️.ts`, are reached again and the existing bound of 1600 holds. The fixture needed no change. The failing assertion now also names the missing module.

Final: 🧹️config 54 pass, 0 fail. Dev server on 6087 untouched; config changes apply on its next restart.
