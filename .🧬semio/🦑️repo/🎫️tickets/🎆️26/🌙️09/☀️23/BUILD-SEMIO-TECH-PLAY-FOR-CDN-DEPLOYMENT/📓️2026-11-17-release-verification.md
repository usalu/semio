# Release verification, native

Run after `native-play.sh site` succeeded: `./native-play.sh verify` (logs in `🗑️generated/native/{release-routing,release-verify,test-release}.log`, Playwright output in `🗑️generated/play-release-e2e`).

`verify` runs routing, browser installation checks, a browser GPU probe, static release verification, and the full Playwright suite. Diagnostic steps that use `step` receive a capacity grant and `native-launch.ts` envelope and run with fresh logs.
- `routing`: bun (from `.cache/b/t/m/bun/*/bun`) `test 🏢️semio-tech/🎡️play/🧪️tests/🧪️playrelease/🟦️.ts` (13 pass, ran now; uses own fixtures, needs no build; `repo test` is NOT bun test).
- `browsers`: skipped when `ms-playwright/chromium-*` and `chromium_headless_shell-*` exist (revision 1248 installed, matches playwright-core 1.64.1); else `repo <bootstrap>/📦️dependencies/🏗️native/📜️script.ts sync browsers`.
- `verify-browser` launches the exact app-local Playwright installation from the repository browser cache and verifies JSPI plus WebGL 2 and WebGPU draw/readback. An explicitly configured GPU profile is authoritative. When the default probe fails, macOS tries `verify-browser-windowed` before `verify-browser-swiftshader`. The successful profile carries into the full acceptance run.
- `release-verify`: play `🔨️modules/📦️site/📜️script.ts verify` (catalog equals descriptors, module page files equal the catalog binding).
- `test-release` reads the published cohort from the products cache store, serves the committed families on loopback listeners, and maps the three HTTPS CDN origins to their respective listeners. The Playwright gate runs the complete acceptance spec. The current catalog yields 172 tests across 18 groups, including a boot test for each of 150 panes.

## Changed
- `native-play.sh`: added `routing`, `browsers`, `verify`.
- `🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🟦️.ts` (`TestReleaseScript`): Playwright CLI was resolved from the root `node_modules` (copy e5027...), while the spec imports play's copy (ed03d...): Playwright refuses with "did not expect test.describe()". Now resolved like `TestScript`: `createRequire(play/package.json).resolve("playwright/package.json")`. Verified `--list` enumerates 173 tests with the play-local CLI.

## Release verification follow-up

- The owned capture API refused the native caller's 1 GiB and 10 million line transport grant before spawning Playwright. The release verifier now streams through the repository's owned command runner with the caller's output grant and the published pages' live deadline and cancellation signal. Playwright discovery completed through that runner with the native output grant, and the exact set of listed boot variants matched the authored catalog with no duplicates.
- The static verifier now selects the app origin by host from the committed family roster. The routing fixture lists three satellite families around the app, and both Bun fetch and Playwright's independent HTTP client prove the app base URL serves the published index with its expected headers. The test reproduced a 404 before the fix. All 13 routing cases now pass.
- The browser adapter disposes each fetched response once the browser has received it, releasing Playwright's copy of the component or asset body. The [Chromium worker probe](📓️worker-route-adapter-proof.md) passed against the updated adapter across all three satellites, including a dynamic module import.

## Browser GPU and cancellation evidence

- The actual headless shell was Chromium 156.0.8078.4, revision 1248, under `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`. The default Metal profile started Chromium and could navigate to loopback, but the GPU draw probe exceeded its 15-second limit. SwiftShader returned green pixels from both WebGL 2 and GPU canvas texture readback, with JSPI available. The probe passed through the ticket’s native step launcher.
- Aborting the owned command when the Chromium startup marker arrived sent SIGTERM to the browser tree. The owned command settled in 19 ms for headless Chromium and 62 ms for windowed Chromium. PID checks after exit found no remaining processes from either browser tree.
- `map-gpu-probe.ts` imports the cached release surface module in Chromium and exercises its actual MapSession with the Vite output’s visible raster and vector tiles. It uses the repository’s neutral map-policy fixture with paint roles for OpenMapTiles water, land cover, and boundaries. GPU attachment and tile uploads worked with SwiftShader, but composited screenshots timed out under both headless channels. The diagnostic `--disable-gpu-compositing` flag returned uniform frames.
- `PLAY_E2E_GPU=windowed` uses the same Metal launch flags and Chromium revision with a window on the existing macOS WindowServer. The production capability probe passed through the native step launcher, reporting Apple M4 ANGLE and a non-fallback Metal 3 WebGPU adapter. Independent PNG decoding of the MapSession screenshots found 20 raster colors and 513 vector colors, with 65,536 changed pixels at each transition and zero browser errors. Bash syntax and the default, windowed, software, and explicit windowed selection branches passed.
- Reproduce the successful map probe with `PLAY_E2E_GPU=windowed repo tools exec bun -- <ticket>/map-gpu-probe.ts <release surface module directory> <release Vite output>`. Use `PLAY_E2E_GPU=swiftshader` to compare software output. Append `chromium` to use the full Chromium headless channel or set `PLAY_GPU_PROBE_DISABLE_COMPOSITING=1` to reproduce the uniform screenshot diagnosis.

## Pending

- The 172 browser tests still need to run against a complete published release cohort in the products store.

## Follow-up files

- `🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🟦️.ts`
- `🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/📦️release/🟦️.ts`
- `🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/📦️release/🎭️fixture/🟦️.ts`
- `🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🎚️config/🟦️.ts`
- `🏢️semio-tech/🎡️play/🧪️tests/🧪️playrelease/🟦️.ts`
- `🏢️semio-tech/🎡️play/🧪️tests/🧪️playrelease/🔣️.json`
- `🔭️routing/📜️script.ts`
- `📓️worker-route-adapter-proof.md`
- `📓️2026-11-17-redeploy.md`
- `📓️2026-11-17-release-verification.md`
- `native-play.sh`
- `📜️script.ts`
- `map-gpu-probe.ts`
