# 🧭 Verification recipes on this Mac (second pass, 2026-11-25)

Scope: which verification paths work right now on this 16 GB Mac, with exact commands, results and durations. No source file edited, no git write, no cargo build. Logs are in `🗑️generated/` of this ticket.

Shorthand used below:
- `R` = `/Users/ueli/Documents/semio`
- `T` = `$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS`
- `G` = `$T/🗑️generated`
- Playwright always needs `export PLAYWRIGHT_BROWSERS_PATH="$R/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"`

## Summary table

| # | Path | Status | Wall time |
| --- | --- | --- | --- |
| 1 | UI vitest, full stock config | Works, 8 failing tests | 137 s |
| 2 | WindowChrome / WindowFrame / Select subset | Works, all pass | 8 s |
| 3 | OS renderer vitest (full) | Runs, 38 failing tests, 19 files | 262 s |
| 4a | Storybook dev, UI scope (port 6090) | Works | ready in about 9 s |
| 4b | Playwright story suite | Lists 135 tests; window-frame story tests time out on `goto` | 38 to 121 s |
| 5 | Vite sites from `.claude/launch.json` | Tested 2 of 12 entries, both work | ready in 1 to 5 s |
| 6 | tectonic / LaTeX report | Binary works; no working Mac build script; direct build fails | 21 to 51 s |
| 7 | Python / manim | Course env works; `.venv` has no manim; still frame and clip unittest pass | 5 s and 56 s |
| 8 | `repo` CLI | `--help` works; `repo run` stalls in graph reading | 180 s cap hit |

## 1 UI vitest suite (works)

Command (ticket wrapper, no filter, stock config `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`):

```sh
export PLAYWRIGHT_BROWSERS_PATH="$R/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
sh "$T/compositor-vitest.sh" > "$G/ui-full.txt" 2>&1
```

Result: exit 1, 137 s. Test Files 4 failed | 73 passed (77). Tests 8 failed | 1633 passed (1641). Log: `$G/ui-full.txt`.

Failing tests (file, then first error line):
1. `🧪️owned-locale-detector-retirement/🟦️.tsx` > Layout paints one continuous base floor: `expected 'panel' to be 'base'`
2. same file > Navbar, Footer, Canvas, Mode-body paint ui-surface at base standalone: `expected '<nav id="ui.navbar"…' to contain 'ui-surface'`
3. `🃏️CardStack/chip-row.test.ts` > reads the token in `../🗂️WindowChrome/🟦️.tsx`: `expected … not to contain 'min-h-medium'`
4. `📐️Layout/🧪️tests/🧩️component/🟦️.tsx` > Shell band borders > keeps the navbar seam on hover over a toggle: `expected Set{ 1, 27 } to deeply equal Set{ 27 }`
5. same > keeps the footer seam on hover over a toggle: `expected Set{ 1, 27 } to deeply equal Set{ 1 }`
6. same > Layout scroll canvas > keeps tinted bars in reach: `desktop long page: expected 0 to be greater than 0`
7. same > Shared native panel overlay contract > keeps the window canvas full width: `Test timed out in 30000ms`
8. `🔣️Icons/🧪️tests/🧩️component/🟦️.tsx` > updates the mounted info glyph and mask on theme alias change: `expected '<svg …' to be '<svg …'` (serialisation whitespace differs)

Note: the earlier handoff counted 1362 tests in 55 files for `"🧪️tests/"`. The full stock config now has 1641 tests in 77 files, so the numbers are not comparable.

## 2 WindowChrome / WindowFrame / Select subsets (works)

```sh
sh "$T/compositor-vitest.sh" WindowChrome WindowFrame Select > "$G/subsets-windowchrome-frame-select.txt" 2>&1
```

Result: exit 0, 8 s. Test Files 11 passed (11), Tests 167 passed (167). Arguments are substring filters on test file paths.

## 3 OS renderer vitest (runs, not green)

```sh
sh "$T/os-vitest.sh" > "$G/os-renderer-full.txt" 2>&1
```

Config: `SEMIO_TEST_LEVEL=long`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine` config. Result: exit 1, 262 s (transform 200 s, tests 952 s, workers included). Test Files 19 failed | 222 passed (241). Tests 38 failed | 3563 passed (3601). Log: `$G/os-renderer-full.txt`.

Failing areas (files): `🧪️hidden-tab-surfaces` and `🧪️node-graph-hosts` (suite fails to load), `🧪️embedded-mount` (about 12 tests, Worker and boot lifecycle, 30 s timeouts), `🧪️package-integration` (wgpu worker, about 8 tests), `🧪️chrome-palette` (2), `🧪️icon-svg-lighting` (WebGPU Chromium), `🔨️modules/🖱️ui/🧪️tests/📤️prepared-readback` (WebGPU readback), `🧪️diff-view-produced-surface`, `♿️editable-controls` (4), `🧪️procedural-descriptors`, `🔒️shell-option-locks`, `🧺️turn-patch-batch`, `🎬️MediaTransportHost` (native media in Chromium), `🖼️IconRenderHost` (4 files: gpu-export, png-export, download, frame-presentation), `🛠️ShellHelpers/⏪️time-travel` geometry (2). Many jsdom errors `Not implemented: HTMLMediaElement.prototype.pause/load` are logged but are not the failing assertions. Exact first error line per test is in the log.

Important: other agents use a lock (`lockf -k /private/tmp/claude-501/semio-vitest.lock`). This run did not take that lock.

## 4 Storybook UI scope and Playwright

### 4a Dev server (works)

Launch entry `os-shell-frames-native` (`.claude/launch.json`, port 6090), run as a background `sh -c` of its `runtimeArgs[1]`:

```sh
cd "$R" && STORYBOOK_SCOPE=ui STORYBOOK_DISABLE_TELEMETRY=1 SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY="$R/.cache/b/p/m/os-shell-frames-sb-vite" exec "$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)" node_modules/storybook/dist/bin/dispatcher.js dev -c .storybook -p 6090 --exact-port --host 127.0.0.1 --no-open --ci
```

Ready after about 9 s (`8.52 s for preview`). `curl http://127.0.0.1:6090/iframe.html` returns 200. Stopped by pid (21517), done.

Note: port 6090 is now held by another process (`bun` pid 44341, not mine, left alone). A Playwright run against 6090 will test that process, not mine.

### 4b Browser binaries (present)

`$R/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` contains `chromium-1248`, `chromium_headless_shell-1248`, `ffmpeg-1013`, `firefox-1555`, `webkit-2370`. `~/Library/Caches/ms-playwright` is absent, so `PLAYWRIGHT_BROWSERS_PATH` must be set (the wrapper does it).

### 4c Playwright story suite

```sh
sh "$T/story-playwright.sh" --list                       # 135 tests in 12 files, seconds
sh "$T/story-playwright.sh" --grep "window" > "$G/story-pw-window.txt" 2>&1
```

Result of `--list`: works. Result of `--grep "window"` (exit 1, 121 s): 3 failed, all in `📚️storybook-new-stories` (window frame reduced-transparency, forced-colors, default glass chips). Each fails with `goto: Test timeout of ~33 s` while navigating `iframe.html?id=…&viewMode=story` with `waitUntil: domcontentloaded`. A retry on a warm server (`--grep "window frame chips"`, 38 s) failed the same way. `curl` to the same iframe returns in 0.13 s, so the timeout is inside Playwright/Chromium, not the server. Not diagnosed further. Note: `--grep` is a regex on the test title, case-sensitive, and story ids are kebab-case, so `--grep WindowChrome` matches nothing (`No tests found`).

Static Storybook (`repo run repo-storybook:build-storybook`, `env-gaps-storybook-build.sh`) was not re-run: `repo run` stalls (see 8).

## 5 Vite and sites from `.claude/launch.json`

Entries present (12): `staffage-site` (6071), `staffage-site-native` (6072), `os-shell-frames-native` (6090), `process3d-react` (6022), `stdio-pdf-react` (6260), `generation3d-react-native` (6018), `generation3d-wgpu-native` (6118), `sourcing-react` (6081), `architecture-course-native` (6061), `architecture-course-cdn-native` (6062), `architecture-course-rehearsal-long` (6063), `draw-react-native` (6064).

Tested:
- `staffage-site-native` (port 6072): works. Vite v7.3.7 ready in 1.3 s. It binds only to `localhost` and `[::1]`: `http://localhost:6072/` and `http://[::1]:6072/` return 200, `http://127.0.0.1:6072/` refuses. Stopped by pid 26818, port freed.
- `architecture-course-native` (6061, `node site-direct.ts dev` in the ticket `☀️23/ARCHITECTURE-COURSE-REDEPLOY-READINESS`): works. Ready in 0.7 s, `http://127.0.0.1:6061/` returns 200 after about 5 s. Stopped by pid 32140, exited.

Not tested (entries exist, no run): `staffage-site` (`repo run @teaching/staffage-site:dev`, depends on `repo`, which stalls), `sourcing-react` (`repo run`), `process3d-react` (`repo ./📜️script.ts dev …`), the rest of the list.

No launch entry exists for: the play app (`🏢️semio-tech/🎡️play/package.json`), the demonstrator (`♻️mit-bestand/🧺️demonstrator/package.json`, plugin copy at `✏️s/🔌️plugins/🎪️demonstrator`), or the hub admin (`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript`). Those need a `.claude/launch.json` entry first. Not tried.

## 6 tectonic / LaTeX

- Binary: `$R/.cache/b/t/m/print/tectonic/aarch64-apple-darwin/tectonic`, Tectonic 0.16.9 (`--version` works). Not on `PATH` (`which tectonic` empty). `pdftoppm` is not on `PATH` either.
- Fixture build scripts: `COMPLETE-…-LA-TE-X-TEMPLATES/build.ts` (used by `build-all.py`, `build-gallery.py`) hard-code `C:/semio` and the Windows tectonic. They do not run on this Mac.
- Mac route: `ticket ☀️22 print-lane-build.sh` needs a scratch dir `S=/private/tmp/claude-501/-Users-ueli-Documents-semio/0fec…/scratchpad` from another session. Not usable as is here.
- Direct build of the report template (21 s, failed):

```sh
cd "$R/🧰️framework/🛍️products/📓️print/🧾️template/📋️report" && env HOME="$G/home" TECTONIC_CACHE_DIR="$G/tectonic-cache" SOURCE_DATE_EPOCH=1782864000 TZ=UTC TEXINPUTS="$R/🧰️framework/🛍️products/📓️print/🖋️latex//:$R/🧰️framework/🛍️products/📓️print//:$R/.cache/b/p/m/print/prototype/fonts//:" "$R/.cache/b/t/m/print/tectonic/aarch64-apple-darwin/tectonic" --bundle "file://$R/.cache/b/t/m/print/tex" --keep-logs -Z search-path=… --outdir "$G/tectonic-out2" "📋️report.tex"
```

  Format generation works. The main file is ignored (`(.tex File ignored)`) because of the emoji prefix in the file name, so the staging step of `build.ts` (strip leading pictographs, including inside TeX sources) is required. Without the isolated `HOME` the first run also printed `access to the path /dev/null is forbidden` (`tectonic-report.txt`).
- Conclusion: the binary works. There is no working Mac report-fixture build yet. Needed: a Mac port of `build.ts` (drop `C:/semio`, use the aarch64 tectonic) or a staging step before the direct call.

## 7 Python / manim

- Repo `.venv` (`$R/.venv` → cpython 3.14): `import manim` fails (`ModuleNotFoundError`). Not usable for manim.
- Course env `$R/.🧬semio/🎓️teaching/⚡️cache/python/bin/python` (cpython 3.12.13, manim 0.22.0): works.
- One still frame (5 s, works), scene in `$G/manim-probe/probe_scene.py`:

```sh
cd "$G/manim-probe" && "$R/.🧬semio/🎓️teaching/⚡️cache/python/bin/python" -I -m manim render -ql -s --format png --media_dir "$G/manim-probe/media" -o probe_still probe_scene.py ProbeStill
```

  Output: `$G/manim-probe/media/images/probe_scene/probe_still.png`. For the real clips, the course route is `authoring/📜️script.ts posters --language en --profile final` (not run).
- Clip unit tests (works, 56 s, 8 tests OK):

```sh
env SEMIO_TEST_MANIM=1 SEMIO_TEACHING_LANGUAGE=en SEMIO_TEACHING_PROFILE=final PYTHONUTF8=1 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEACHING_OUTPUT_ROOT="$G/clip-test-out" SEMIO_TEACHING_CACHE_ROOT="$G/clip-test-cache" "$R/.🧬semio/🎓️teaching/⚡️cache/python/bin/python" -m unittest "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️icons"
```

  Log: `$G/clip-visuals-icons.txt`. The full list is the `test` command of `authoring/📜️script.ts` (not run). The earlier handoff recorded 10 `semio-style` tests passing.

## 8 `repo` binary

- `repo --help` works (`/Users/ueli/.local/bin/repo`, `--tool=nx|bazel|semio`).
- `repo run @semio-tech/ui-react:lint` with a 180 s cap: does not get past preparing. It stays in `Reading the project graph slice` (124 s, 121,505 source observations, 45 owner checks, 79 owners admitted), then `still working after 172 s; last phase started: door.script`, then it was cancelled by SIGTERM (exit 124). Log: `$G/repo-run-lint.txt`. Consistent with the graph failure in `environment-gaps-handoff.md`.

## Process hygiene

- Started by me and stopped: Storybook dev (pid 21517), staffage vite (pid 26818), architecture course dev (pid 32140). All ports freed except 6090, which is held by another process (pid 44341).
- Finished by themselves: UI vitest, WindowChrome subset, OS renderer vitest, Playwright runs, tectonic runs, manim runs, `repo run` (cancelled by its own cap).
- Not mine, left alone: `lockf` and vitest processes of another agent (`semio-vitest.lock`, pids 54434 to 54488), `semio-manim.lock` (pid 51162), `bun` on 6090 (pid 44341).
- Memory free was 43 to 75 % during the runs (`memory_pressure`). No cargo, no git writes, no source edits.
- `🗑️generated/` in this ticket holds all logs and probe output. Delete it when the ticket closes.
