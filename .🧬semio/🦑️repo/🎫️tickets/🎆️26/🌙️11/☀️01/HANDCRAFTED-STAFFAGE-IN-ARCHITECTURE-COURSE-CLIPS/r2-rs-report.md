# Presentation UI batch performance (rs)

Scripts: `T/🧰️rs/` (`capture.py` captures the real 26-frame batch, `make_profile.py` injects per-phase timers into a copy of `render.mjs`, `make_cpu.py`, `make_diag.py`, `make_fast*.py`, `cmp.py`, `bbox.py`, `timing.py`, `prof.py`, `membership.py`). Generated output deleted.

## Measurements (26-frame batch, 320x180, machine under 20-agent load)

Node `render.mjs` before the fix: about 112 to 135 s.

| Phase | Time |
|---|---|
| Startup: esbuild, Tailwind compile and scan, CSS build, UI bundle | 9.5 s |
| Chromium launch, page load, font and script injection, configure | 1.5 s |
| Per frame: `page.clock.runFor` (virtual 256 ms) | 2.7 to 5 s each, 84 s total |
| Screenshot | 0.3 s each, 7 to 9 s total |
| `mouse.move` | 0.23 s each |
| DOM info `evaluate` | 0.1 s each |
| React render, fonts.ready and image decode, settle | 0.15 s each |

So 75 percent of the time is `page.clock.runFor`. A CDP CPU profile of the page during `runFor` shows 95 percent `(idle)`, with about 100 ms of JS. A diagnostic counter shows 182 rAF callbacks per 256 ms. These are the 11 LayeredOverview `tick` loops (`🧱️elements/🥞️LayeredOverview/🔶️panes/🟦️.tsx:159`), each firing 16 times.

Root cause is Playwright's `ClockController._callFirstTimer`. After every fake timer it awaits a real `setTimeout`. That is clamped to 4 ms when nested, and on Windows it rounds to the 15.6 ms timer resolution. 182 timers times about 15 ms gives about 2.7 s per frame, and load stretches it further. This is not a browser-start, font, screenshot or encode problem: the browser is launched once, the page is loaded once, and the bundles are built once per node run.

## Did this ticket make it slower?

No. `🟨️render.mjs`, `🧰️tools.mjs` and the browser `🐍️.py` had no uncommitted changes before this work (git status clean). Staffage casts do not enter this batch: every frame has one small PNG `body_image`, and no SVG is parsed. The slowness is pre-existing.

## Fix 1: node frame loop (`🌐️browser/🧰️tools.mjs`, `🟨️render.mjs`)

- `accelerateClock(page)` replaces the clock's real-timer yield with message-channel tasks (6 hops, so the React scheduler still flushes between timers). It does nothing if the hook is missing.
- `finishTransitions(page)` waits (cap 5 s, real time) for every finite CSS transition to end after `runFor`. CSS transitions run in real time outside the fake clock. The old slow path had hidden that dependency by waiting seconds.
- Result: `runFor` total 84.6 s to 1.3 s. The batch takes about 55 s end to end (about 10 s of it startup), most of the rest being the loaded machine.

Output identity (PNG byte comparison and metadata against the old path):
- 24 of 26 frames byte-identical, and metadata identical.
- Without `finishTransitions` the frames 7, 10, 14 to 17 differ by 1 level on 40 to 94 pixels, which proves the real-time transition dependency. With it they are identical.
- Frames 12 and 13 (the energy to physical_fundamentals reveal) differ, but the old path itself is not deterministic there. Two old-path runs against each other differ at 12 and 13 (max diff 239, one with a hierarchy mismatch, one showing a half-finished transition). The new path is deterministic and equals the settled old frame visually.

## Fix 2: Python side (the test spends more time here than in the browser)

A cProfile of the test showed, in 411 s total: `_handoff_identity` 166 s and `ffmpeg_command` 50 s.

- `⚙️runtime/🐍️.py` `ffmpeg_command`: `shutil.which` scanned PATH (5700 `exists` calls, 17 s per call under load) on every call, and it ran many times. Now resolved once per (`SEMIO_FFMPEG_BINARY`, `PATH`).
- `🌐️browser/🐍️.py` `_repo`: same caching for `shutil.which("repo")`.
- `🌐️browser/🐍️.py` `_membership`: per file it built several `Path` objects and called `is_relative_to`, `relative_to`, `resolve` (170k `is_relative_to` calls). Now string prefixes, `os.path.islink` and `realpath` only for symlinks. Verified on the real scan sources: 655 members, identical to the old function, 2.3 s against 4.6 s.

Test `test_encoded_chapter_preserves_body_camera_handoffs_audio_and_contextual_boundaries`: 411 to 421 s before, 291 s after (same load, profile run). The remaining 226 s is the 6 node invocations. The first two are the `--sources` discovery passes (esbuild and Tailwind only, about 15 s each) and four are renders. The Python side is now small.

## Timeout (`🪟️presentation/🧪️tests/🐍️.py:305`)

`threading.Timer(1500 -> 900, session.cancel)`. It guards only `render_chapter_source`, which measured 399 s before and 281 s after the fixes under 20-agent load. 900 s is about 3 times the loaded measurement and still catches a hang. The earlier 300 s bound was below even the unloaded cost.

## Tests run

- `PresentationNativeTest.test_encoded_chapter_...` plus all `PresentationContractTest`: 10 tests OK (921 s total under load).
- `🌐️browser/🧪️tests/🟨️.mjs` (launch lifecycle): all PASS.

## Open

- Each node invocation rebuilds esbuild, Tailwind and the UI bundle (about 10 s). Six invocations per test; a cache or a long-lived worker would save about 60 s.
- Frames 12 and 13 of the old path were racy by design; the new path fixes that, but frame bytes differ from any single old-path run there.
- `accelerateClock` touches the private `_embedder` of Playwright's clock (version 1.64); it is guarded and falls back to stock behaviour.
