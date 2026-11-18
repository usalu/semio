# Release verification, native

Run after `native-play.sh site` succeeded: `./native-play.sh verify` (logs in `🗑️generated/native/{release-routing,release-verify,test-release}.log`, Playwright output in `🗑️generated/play-release-e2e`).

`verify` = `routing` -> `browsers` -> `release-verify` -> `test-release`, each through `step` (capacity + `native-launch.ts` envelope), always forced (no done-marker skip):
- `routing`: bun (from `.cache/b/t/m/bun/*/bun`) `test 🏢️semio-tech/🎡️play/🧪️tests/🧪️playrelease/🟦️.ts` (13 pass, ran now; uses own fixtures, needs no build; `repo test` is NOT bun test).
- `browsers`: skipped when `ms-playwright/chromium-*` and `chromium_headless_shell-*` exist (revision 1248 installed, matches playwright-core 1.64.1); else `repo <bootstrap>/📦️dependencies/🏗️native/📜️script.ts sync browsers`.
- `release-verify`: play `🔨️modules/📦️site/📜️script.ts verify` (catalog equals descriptors, module page files equal the catalog binding).
- `test-release`: play `🔨️modules/🧪️e2e/📜️script.ts test-release` (MEM 4000): reads the published cohort from the products cache store (not `dist/pages`), serves the 4 families statically on 127.0.0.1 listeners, maps https://{map,media,modules}.assets.semio-tech.com to them via Playwright routes (`PLAY_E2E_RELEASE_ORIGINS`), runs `🧪️tests/🎭️acceptance/🟦️.ts` (173 tests; boots every catalog pane, 151 panes / 18 groups).

## Changed
- `native-play.sh`: added `routing`, `browsers`, `verify`.
- `🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🟦️.ts` (`TestReleaseScript`): Playwright CLI was resolved from the root `node_modules` (copy e5027...), while the spec imports play's copy (ed03d...): Playwright refuses with "did not expect test.describe()". Now resolved like `TestScript`: `createRequire(play/package.json).resolve("playwright/package.json")`. Verified `--list` enumerates 173 tests with the play-local CLI.

## Static check
Catalog shape (`groups[].panes[]`, `variant`, `example`) matches the spec reader; hosts/regex match `playPageHost`; no other drift found.

## Risks
- GPU panes (map/wgpu) need a real adapter; set `PLAY_E2E_GPU=swiftshader` otherwise (map pane then not gradable).
- Not executed end to end (needs the build); a SEMIO_BUILD_MODE=release cohort must exist in the products store.
