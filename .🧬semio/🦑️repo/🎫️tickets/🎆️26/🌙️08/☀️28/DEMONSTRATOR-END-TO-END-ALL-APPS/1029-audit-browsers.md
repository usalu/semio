# 1029 Audit: Browsers (`workspace:deps-browsers`)

**Status: INSTALLED** at `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` (`chromium-1248`, `chromium_headless_shell-1248`, `ffmpeg-1013`, all complete, dated 2026-10-20). The 02:18-02:39 sync did not install them: it ran `playwright install chromium`, Playwright found its `INSTALLATION_COMPLETE` markers and returned without downloading. Fix: none needed for the browser build.

## Answers

- **Did the sync install Chromium?** No. It was a silent no-op on a build that already existed.
- **Is the build Playwright 1.64.1 needs present?** Yes. Revisions match `browsers.json` exactly, every directory has its marker, and the binaries run.
- **Is there a receipt that made the sync skip?** Yes, Playwright's own per-directory `INSTALLATION_COMPLETE` marker. The repo layer keeps no receipt for browsers; `native/🟦️.ts:65` calls the installer unconditionally.

## 1. Code path (sync to install)

| Step | Location | Behaviour |
|---|---|---|
| Sync entry | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts:6-8` | Registers `sync` to `NativeDependenciesScript` |
| Browsers branch | `.../🏗️native/🟦️.ts:65` | `kind === "browsers"` runs `runtimeExecutableV1()` with `node_modules/playwright/cli.js install chromium` and `repoToolCacheEnv(workspace)`. No repo-level marker check. |
| Process runner | `.../📦️dependencies/🟦️.ts:74` | `capture=false` gives `stdout: "inherit"`, so Playwright output would reach `browsers02.log`. |
| Env routing | `.../🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts:32-37` | `env.PLAYWRIGHT_BROWSERS_PATH ??= repoCacheDirectory(repoRoot, "tools", "ms-playwright")`. Honours a caller-set value. |
| Cache root | `.../📚️library/⚡️caching/🟦️.ts:8-11` | `<repo>/.🧬semio/🦑️repo/⚡️cache/<segments>` |
| Re-export | `.../📦️packages/🟦️typescript/🏃️process/🟦️.ts:2` | Same `repoCacheDirectory`, used by the e2e |

Demonstrator e2e: `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🟦️.ts:23` sets `PLAYWRIGHT_BROWSERS_PATH: repoCacheDirectory(context.repoRoot, "tools", "ms-playwright")` **unconditionally**. This resolves to the same directory as the sync. The test command (`.../🎛️commands/🧪️test/🟦️.ts:187`) uses `process.env.PLAYWRIGHT_BROWSERS_PATH ?? …`.

Playwright side:
- `/Users/ueli/Documents/semio/node_modules` is a symlink to `.cache/b/d/m/j/4ccf8232dba5164499e9/node_modules` (created 2026-10-28 16:51). `node_modules/playwright` links to `.bun/playwright@1.64.1`. The `playwright-core` package is at `node_modules/.bun/playwright-core@1.64.1/node_modules/playwright-core`. There is no top-level `node_modules/playwright-core`.
- Revisions in `…/playwright-core/browsers.json`: `chromium` 1248 (browser 156.0.8078.4), `chromium-headless-shell` 1248, `ffmpeg` 1013.
- Registry directory: `coreBundle.js:34467-34474` (env var, relative paths resolved against `INIT_CWD`). Directory naming: `coreBundle.js:34051` gives `chromium-1248`, `chromium_headless_shell-1248`, `ffmpeg-1013`.
- `install chromium` expands to chromium plus headless shell, and ffmpeg is added for each browser: `coreBundle.js:35171`, `:35203`.
- Install writes a registry link `.links/<sha1(PACKAGE_PATH)>` and runs GC: `coreBundle.js:34887-34906`.
- **Skip point:** `coreBundle.js:33724-33728` returns silently if `INSTALLATION_COMPLETE` exists and `--force` is not set. The `Downloading …` message (`logPolitely`) comes after this return and was never reached.
- Per-browser call: `coreBundle.js:35041`.

## 2. Filesystem

`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/` lives on `/System/Volumes/Data` (`df`), not on an `.cache/b/<letter>/m` APFS image. The Bun executable the sync used is on partition `t` (`.cache/b/t/m/bun/…`), but the browsers are not.

| Playwright 1.64.1 requires | On disk | Marker | Dated | Verified |
|---|---|---|---|---|
| chromium 1248 | `chromium-1248/` | `INSTALLATION_COMPLETE` (11:08:33), `DEPENDENCIES_VALIDATED` | 2026-10-20 | `Google Chrome for Testing.app` present, `CFBundleShortVersionString` 156.0.8078.4 |
| chromium-headless-shell 1248 | `chromium_headless_shell-1248/` | `INSTALLATION_COMPLETE` (11:09:10), `DEPENDENCIES_VALIDATED` | 2026-10-20 | `chrome-headless-shell-mac-arm64/chrome-headless-shell` (176,747,664 bytes); `--version` returns Google Chrome for Testing 156.0.8078.4 |
| ffmpeg 1013 | `ffmpeg-1013/` | `INSTALLATION_COMPLETE` (11:08:36), `DEPENDENCIES_VALIDATED` | 2026-10-20 | `ffmpeg-mac -version` returns `n7.0.1-playwright-build-1013` |

- No other revisions are in the registry.
- `.links/ad27ee63c9bee5c88bfc360b3e8af0838a0a0e13` (mtime 2026-10-29 02:39) holds `/Users/ueli/Documents/semio/.cache/b/d/m/j/4ccf8232dba5164499e9/node_modules/.bun/playwright-core@1.64.1/node_modules/playwright-core`. This is the only link. Its 02:39 mtime shows the browsers02 run registered against this exact registry, so the sync and the e2e resolve to the same directory.
- Nothing was downloaded during 1029; the directories are unchanged since 2026-10-20.

## 3. Logs

**`browsers02.log` (exit 0, 2026-10-29 02:18:39 to 02:39:16).** 1247 `"stage":"running"` lines removed for this review; the rest is planning/capacity noise.
- Lines 1699-1707: Bazel action `workspace:deps-browsers:prototype` starts and runs on a worker (11 s).
- Line 1708: `INFO: From workspace:deps-browsers:prototype:`
- Lines 1714-1716: `native-dependencies:browsers` runs `<bun>:/Users/ueli/Documents/semio/node_modules/playwright/cli.js`. The `install chromium` arguments are not logged; only `args[0]` is.
- Line 1717: `still running after 10 s`
- Line 1718: `native-dependencies … sync complete`
- Lines 1722-1724: `Elapsed time: 21.150s, Critical Path: 18.50s`; `Build completed successfully, 2 total actions`.
- Between the sync start (1714) and completion (1718) there is no Playwright output: no `Downloading`, no progress, no error. That matches the silent marker skip at `coreBundle.js:33724-33728`.

**`browsers01.log` (exit 1, 02:08:14 to 02:18:20).** Failed before the sync step: `NX The externalDependency 'nx' for 'workspace:deps-browsers' could not be found`. This was the `nx` path, not the Bazel path, and says nothing about the browser install.

**`🗑️generated/1028/e2eprep-br.txt` (rc 1, 2026-10-28 23:32).** Contains no `native-dependencies` or `browsers` lines. It fails in storage planning (`nx:storage-planning`, phase `source.served`, around lines 2119-2130) with `error: Exact original descendant creator identity required`. The browser step was never reached, so this attempt is not evidence about the browser build. It is a cache-image receipt error, the same family as the APFS image notes in memory.

## 4. Risks (not blocking)

- **Asymmetric env override.** The sync uses `??=` (`🌿️environment/🟦️.ts:35`), which honours an inherited `PLAYWRIGHT_BROWSERS_PATH`. The e2e (`e2e/🟦️.ts:23`) always overrides it. If a devcontainer or caller exports a different path, the sync installs there and the e2e looks in the repo cache, and the e2e fails with a missing executable. No inherited value was observed in this run, and the `.links` evidence shows both resolved to the same directory.
- The test command (`commands/🧪️test/🟦️.ts:187`) is a third variant, using `??`. Harmless as long as the variable is unset.

## Verdict

INSTALLED (pre-existing since 2026-10-20). The 1029 sync was a no-op that confirmed the build. No action required for browsers. If the e2e later reports a missing browser, look elsewhere first (the ms-playwright registry is complete for 1.64.1).
