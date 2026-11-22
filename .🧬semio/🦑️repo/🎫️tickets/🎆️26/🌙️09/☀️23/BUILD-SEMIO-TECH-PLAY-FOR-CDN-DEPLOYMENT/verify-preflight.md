# Verify preflight (cohort-independent steps)

Run while `site` was still building. No failures, no code changes.

| Step | Command | Result |
| --- | --- | --- |
| Release routing | `bash native-play.sh routing` | 13 pass, 0 fail (62 expects) |
| Playwright browsers | `bash native-play.sh browsers` | present, ok |
| Browser capabilities | `native-play.sh step verify-browser ... browser-capabilities` (`PLAY_E2E_GPU` unset) | ok: JSPI true, WebGL (ANGLE Metal, Apple M4) green pixel, WebGPU (apple metal-3, no fallback) green pixel. Default GPU mode suffices; no windowed/swiftshader fallback needed |
| Play unit tests | `native-play.sh step play-unit ... repo ./📜️script.ts test` | 8 files, 82/82 pass |
| Export law | `bun test ./🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/📦️publication/📤️export/🧪️tests/🟦️.ts` | 2 pass, 43 expects |

Notes
- The export test path needs a leading `./`, otherwise bun treats it as a filter and matches nothing.
- Calling `repo ./📜️script.ts test` directly fails ("Original parent Script process invocation required"); run it through the `native-play.sh step` launcher.
- Not run: `release-verify`, `test-release` (need the built cohort).
