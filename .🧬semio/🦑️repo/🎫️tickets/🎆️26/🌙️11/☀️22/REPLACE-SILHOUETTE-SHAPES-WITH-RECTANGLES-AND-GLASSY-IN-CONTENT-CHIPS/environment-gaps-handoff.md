# 🧩 Environment gaps handoff

Everything below ran through project routes only: the provisioned Playwright cache, `provisionNative` (pinned micromamba + conda-forge cairo) and `repo tools exec uv` with the locked `uv.lock` of the course environment. Nothing was downloaded by another route. No source file was changed.

## Findings in one table

| Item | Result |
| --- | --- |
| 1 Python teaching tests | 39 tests over the three modules; 2 fail, both unrelated to the migration (below). All migration assertions run and pass. |
| 2a UI React Chromium tests | 5 of 5 pass (Layout scroll canvas, Tree text-flow x3 and the Layout suite) and the whole stock config passes: 55 files, 1362 tests, 0 failures. |
| 2b Storybook/Playwright story tests | NOT run: the project graph refuses to load (error below). |
| 3 Failures caused by the migration | None found. |

## 1 Python (teaching visuals, clip presentation)

Interpreter: the provisioned CPython 3.12.13 (`.cache/b/t/m/python/interpreters`) and the course environment `.🧬semio/🎓️teaching/⚡️cache/python` created by `uv run --project …/🏗️authoring/📦️packages/🐍️python --group test` (manim 0.22.0, skia-pathops 0.9.2, numpy 2.5.4, locked). The workspace `.venv` (3.14) has numpy but no manim/pathops/pytest, so it was not used. pycairo got its native libraries from the pinned micromamba install of `🧱️native` ("[native] install" ok).

Script: `env-gaps-python-tests.ts` (run as `repo env-gaps-python-tests.ts <unittest modules>`; `--script <file.py>` runs a diagnostic). Environment as in `authoring/📜️script.ts test` (`SEMIO_TEST_MANIM=1`, language `en`, profile `final`), but only the migrated modules.

| Module | Tests | Result |
| --- | --- | --- |
| `…📐️geometry.🧪️tests.🐍️` (incl. `test_frame_is_one_rectangle_with_glass_chips_inside_it`, Skia union cross-check) | part of 29 | 1 fail (a), the rest pass |
| `…🖱️interaction.🧪️tests.🐍️` | part of 29 | 1 fail (b), the rest pass |
| `…🎨️visuals.🧪️tests.🎨️semio-style.🐍️` (Manim `window_frame_*`, frames cases, Skia union) | 10 | 10 pass (first run had 2 errors only because `SEMIO_TEACHING_LANGUAGE` was unset in my script; rerun with `en`: OK) |

First combined run: 39 tests, failures=2, errors=2 (the 2 errors were the language variable). The split of the other 29 tests between geometry and interaction was not recorded (logs deleted after the summary); the totals are from the unittest footer (`Ran 39 tests`, `failures=2, errors=2`; second run `Ran 10 tests OK`).

(a) `ExactUIParityTests.test_production_browser_metadata_and_pixels_match_independent_real_ui_oracle` stops at its first rectangle assert: the Chromium title chip row is 24 px but the Python oracle says 22.4 px (`titleHeight`). Cause: `--chip-row-height: round(nearest, max(1.5rem, 7 * --ui-spacing), --device-pixel)` = 24 px; the line and the `min-h-chip-row` class on the cap are unchanged by the migration (present in `HEAD`), so this oracle mismatch predates it. Same family: navbar 29 px vs 28.8 px (device-pixel rounding). Because the assert aborts the test, I ran it with a tolerant wrapper (`env-gaps-geometry-diag.py`, the test source untouched): then the test passes, i.e. every new migration assertion holds in the real browser: `gap_hit_in_content`, `content_clip_path == "none"`, chip regions inside the frame with `dock == top`, `blur(` backdrop filter, chip background different from the glass background, body pixel, floor pixel, chip composite pixel versus plain content, zoomed frame differs. Tolerated mismatches: title height 24/22.4, body y 59.17/57.6, body height 805.6/807.2 (all consequences of the 24 px chip row), navbar and footer 29/28.8 twice.

Open for the owner: decide whether the oracle (`titleHeight` 22.4 in `📐️geometry/🧪️tests/🔣️.cases.json`, `panelHeaderHeightUiSpacing` in `geometry_metrics`) or the CSS (24 px floor of `chip-row-height`) is the law. I did not change either: it is not a migration regression.

(b) `CursorNativeFrameTest.test_four_fps_transitions_capture_current_native_clicks_at_every_required_level`, phase `focus`: targets `architecture` and `physical_fundamentals` log `pointerdown, pointerdown, pointerup, click`. Trace (`env-gaps-click-diag.py`): the cursor is `pressed` in two consecutive sampled frames (4 and 5, 15 and 16) and `render.mjs` opens a fresh page per frame and presses again, so the second page logs a second `pointerdown`; `energy` and the chapter are pressed in one frame only. This depends on the 4 fps sampling of the press interval, not on window geometry. The migration touched only the hit-test (polygon removal) and not the press timing or `phaseSeconds`. Unrelated. `restore` passes.

## 2 Playwright Chromium and the Storybook tests

Chromium was already provisioned by the project route (`repo run workspace:deps-browsers`, 2026-10-20): `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` holds `chromium-1248`, `chromium_headless_shell-1248` (binary present), `ffmpeg-1013`, `firefox-1555`, `webkit-2370`. `~/Library/Caches/ms-playwright` does not exist, so the browsers were not found by default; setting `PLAYWRIGHT_BROWSERS_PATH` to the project cache is enough. I did not run `deps-browsers` again.

- `PLAYWRIGHT_BROWSERS_PATH=<project cache> sh compositor-vitest.sh "Layout" "text-flow"`: 3 files, 45 tests, all pass (includes `Layout scroll canvas > keeps tinted bars in reach as the canvas scrolls underneath them` and the three `Tree label reading order` tests).
- `sh compositor-vitest.sh "🧪️tests/"` (every file of the stock config under a `🧪️tests/` path), same variable: 55 files, 1362 tests, 0 failures. The 10 failures of `uitests-handoff.md` (icon-render-camera, shared-border-dock, Tree text-flow x3, Tabs boundary x2, Layout x2, LayeredOverview) all pass now; they were environment failures (no Chromium), not migration failures.

Storybook story tests (`📚️storybook-new-stories`, WindowChrome, Window, Panel, CardStack, OverviewCard, ContextMenu, Canvas stories): NOT run. They need the static Storybook of `repo run repo-storybook:build-storybook` (`STORYBOOK_SCOPE=ui,styling`), served by `repo-storybook:test-storybook`, via `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts`. The graph step fails before anything builds:

```
The resident service met an unexpected failure (reference 3500ee92): Error [BUILD_SOURCE_UNPARSEABLE]: Cannot find module '/Users/ueli/Documents/semio/✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts' (imported by .storybook/🧪️tests/🧪️browser-runner/📚️suites/🟦️.ts)
  at ownerSourceFailureV1 … (service log /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T/semio-repo/501/host-659f709f2f10.log)
```

`.storybook/🧪️tests/🧪️browser-runner/📚️suites/🟦️.ts` imports `✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts`; that directory exists neither on disk nor in git (`✏️s/🧪️tests` only has `🏘️plugin-composition`, `🗂️native-ownership`, `🧪️mutation-vectors`). It is not a migration artefact. Fix needed by the owner of `✏️s`: add the suite or drop the import. Trying the build script directly (`repo …/🏗️build/📜️script.ts build storybook`) fails with `Build mode must be prototype or release` (that script is the product build, not a storybook command), so there is no graph-free project route; I did not run Storybook or Playwright by any other route. Still unverified in a browser: the story `🖱️ui⚛️react-mode--content-through-glass` (chip and content plane inside one `[data-window-frame]`), the reduced-transparency and forced-colors story tests, and the glass-chip story test.

The course/demonstrator/play Playwright e2e (`prepare-e2e`, `serve-e2e`, `test-e2e`) also go through the graph and were not attempted for the same reason. The extra repo graph instability ("Reading the project graph slice" waiting for capacity) was seen once (about 12 s), it did not crash.

## 3 Failures from the migration

None. Items (a) and (b) predate it (cause named above); nothing was edited.

## Files

Scripts kept here: `env-gaps-python-tests.ts`, `env-gaps-geometry-diag.py`, `env-gaps-click-diag.py`, `env-gaps-storybook-build.sh` (the failing graph attempt). Provisioned outside the worktree index: `.🧬semio/🎓️teaching/⚡️cache/python` (git-ignored course environment, reusable).
