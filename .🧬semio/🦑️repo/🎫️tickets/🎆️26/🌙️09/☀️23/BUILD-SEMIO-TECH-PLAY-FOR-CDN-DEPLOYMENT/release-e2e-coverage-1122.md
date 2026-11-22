# Play release gate coverage audit (2026-11-22)

Read-only audit. No builds, tests or processes were run. Paths are relative to `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play` unless absolute.

## 1. Source of truth for the release app/pane list

- Pane catalog: `🔨️modules/🧩️runtime/🔣️.json` (`groups[].panes[]`, 18 groups). **150 panes**, unique variants, 123 with `example`/`exampleLabel`, 27 without (the 10 `pdf:stdio-*`, 10 `office:stdio-*`, 2 `semio-files:stdio-*`, 5 `binary:stdio-*`; no example).
- Read by `🔨️modules/🧩️runtime/📇️catalog/🟦️.ts` (`PLAY_RUNTIME_PANES`, lines 31-32) and by the overview/grid.
- Registry cross-check: `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎮️playgrounds/🟦️.ts` has **158 rows**. Remove the host `s` (`hostVariant`), the 7 excluded rows (`demonstrator` plus six `entwerfen-mit-bestand-*` brands). That gives **150**, identical to the catalog (`playExpectedVariants`, `🔨️modules/🧩️runtime/🟦️.ts` lines 11-13). Verified with a node script: zero missing and zero extra variants.
- Published release site: `🔨️modules/📦️site/📄pages/` (families `play`, `map`, `media`, `modules`). Overview cards and module bindings are built from the same catalog.

## 2. What test-release and release-verify enumerate

**test-release** (`🔨️modules/🧪️e2e/📜️script.ts` line 7 registers it; `🔨️modules/🧪️e2e/🟦️.ts` lines 61-90 `TestReleaseScript`; project target `📋️project.json` line 662):
- Runs Playwright with config `🔨️modules/🧪️e2e/🎚️config/🟦️.ts`, whose `testMatch` (line 31) is `🎭️acceptance/🟦️.ts` only. It does **not** run `🧪️playrelease` (that is `test-release-routing`, bun test, a separate target, not chained).
- Serves the published families statically (`🔨️modules/🧪️e2e/📦️release/🟦️.ts` `servePublishedPlay`, line 22; missing file returns 404 `text/plain`, line 41, no SPA fallback).
- Spec `🧪️tests/🎭️acceptance/🟦️.ts`, 957 lines. Test count, from the file's describe structure:
  - overview: 1 (line 384, asserts `panes.length` cards and "N apps")
  - 3d: 2 locales x 3 (`edit` ~401, `analyze` 477, `cancel` 496) = 6
  - forms: 2 locales x 7 (`authoring`, `source refusal`, `long answer`, `flow isolation`, `windows`, `cancel`, `submission`) = 14 (describe at 525)
  - language choose fr-FR: 1 (854); GIS terrain: 1 (868)
  - **per pane: 150** (loop at lines 919-957, `playPanes()` reads all 150, no filter)
  - Total about **173 tests**.
- Config: `workers: 1` (`🎚️config/🟦️.ts` line 37), `fullyParallel: false` (line 32), `retries` 2 only when `CI` is set (line 34), timeout from `playwrightTestTimeoutMs()`, per-test `TEST_TIMEOUT_MS` 240 000 and `SHELL_READY_TIMEOUT_MS` 120 000 (acceptance spec lines 105-106), single project `chromium`.

**release-verify** (`🔨️modules/📦️site/📜️script.ts` `verify` router; `🔨️modules/📦️site/🟦️.ts` `runVerifyScript` lines 51-62; project target `📋️project.json` line 748):
- Does **not boot any app**. Checks (a) the generated catalog equals a refusing render of the committed descriptors, (b) `playModuleCatalogFindingsV1` (`📦️site/📄pages/🟦️.ts` lines 153-169): every module file exists, byte size and sha256 match the bound catalog, and the published receipt matches. This covers all runtime modules, including extension modules that no pane boots directly.
- Its README row (`README.md` line 49) lists it together with the routing contracts, which is misleading: the routing contracts are `test-release-routing`.

## 3. Diff and gaps

**Coverage:** every one of the 150 panes is booted by `test-release` (per-pane loop, 150 = 150). No pane or app is uncovered by the boot loop. Module files are covered only byte-wise by `release-verify`; a module is booted only if a pane loads it.

**Per-pane checks that exist** (lines 919-957): deep-link `./#variant`, shell outcome must be `ready` (waitForPaneShellOutcome, 120 s), error boundary absent, `pageerror`, `console.error`, HTTP >=400 and `requestfailed` all empty, refused inputs empty (regex line 139, applied to every non-error console type, so warnings are included), chrome names the curated example, paint witness, example picker round trip, role switch round trip, overview button, SPA-fallback assets empty.

**Assertion gaps:**
1. **Pixel census only for generation3d.** `paintWitness` line 257: `if (variant !== "generation3d" || ...) continue`. For the other 149 panes a blank WebGPU/WebGL canvas passes when the retained `data-meshes-json` or `data-layers-json` is non-empty. The raster witness (`readPaintWitnesses` 233-239) checks metadata only: a visible layer with an asset at least 8 px, not drawn pixels.
2. **DOM witness accepts any text.** Lines 214-216: `text > 0 || elements >= 10`. A window whose body shows only an error or placeholder string passes. Only `aria-busy` status and hidden nodes are stripped.
3. **"Any window paints" rule.** Line 197 design: a pane passes if one window passes, so a dead secondary window does not fail the pane.
4. **Refusals are regex-matched on console text only** (line 139, emitter `🏛️ShellHost/🎯️input-ledger/🟦️.ts` line 213). A refusal logged after the 2 s `BOOT_SETTLE_MS` (line 110) window, or one throttled by `createRefusalNoticeThrottleV1` before it reaches the console, is not seen. Not verified in a run.
5. **SPA-fallback check** (lines 155, 158-170) covers only image/font/media/pdf extensions. `.wasm`, `.js`, `.json`, `.css` answered as text/html are not checked. In the static release server this cannot occur for missing files (404 text/plain, which `resourceErrors` catches only for requests the pane issues).
6. **Locale:** all 150 pane boots run in `en-US` only (`test.use({locale:"en-US"})` line 383 wraps the loop). German boot of panes is not in the gate except forms and 3d.
7. **Routing and serving contracts** (`🧪️tests/🧪️playrelease/🟦️.ts`) are not part of `test-release`.
8. **release-verify** asserts catalog and bytes, not boot, and is not chained to test-release.

## 4. Expected duration and workers

- Workers: **1**, serial (`🎚️config/🟦️.ts` line 37). Retries 0 locally, 2 in CI.
- No recorded run exists. `README.md` line 80 states the chain is "not yet verified".
- Estimate (not measured): fixed sleeps alone are about 6 s per pane (2 s boot settle, 2 s around the example switch, 2 s around the alternative), so 150 panes spend roughly 15 min in waits. Adding cold WASM plugin boots (up to 120 s each) and paint polling, expect on the order of one hour serial for the full run. Treat this as an estimate.

## Summary

- 150 panes in catalog, 158 registry rows minus host and 7 excluded = 150, zero diff. All 150 are booted by `test-release` (about 173 tests total, `workers: 1`).
- `release-verify` boots nothing; it checks catalog and module bytes only.
- Gaps: pixel check only for generation3d; DOM and raster witnesses accept text or metadata; refusals depend on console timing; boot only in en-US; SPA-fallback check misses wasm/js/json; routing contracts not in the gate.
