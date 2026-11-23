# Energy course tests, final run

Per-module runs (`python -m unittest -v <module>`, `SEMIO_TEST_MANIM=1`, up to 3 in parallel, 20 min timeout each). Logs: `🗑️generated/tests-final/logs`. Runner: `tests-final/run-one.sh`, `run-one-path.sh`; module list `tests-final/modules.txt` (66 modules, `tests/inventory.py` finds none unregistered).

## Environment
- No render profile is active outside `script.ts test`, which sets `SEMIO_TEACHING_PROFILE=final` and `SEMIO_TEACHING_LANGUAGE=en`. Pass 1 without them gave dozens of ProfileError/language errors (environment only); the final column is from the run with these two variables (the visuals integration module ran with `preview`/`en` as asked).
- `repo` on PATH and the installed prototype are too old for `tools exec --light` (`unknown flag: --light`). Modules that spawn `repo` (browser lifecycle, output, hierarchy, interaction) ran with `SEMIO_REPO_BINARY` and `PATH` pointing at `%LOCALAPPDATA%/Semio/repo/aa03a414...` which supports it. This is machine state, not code.

## Result
66 modules, 590 tests run, 1 module(s) not green.

| Module (below `⚡️energy`) | Tests | Result | Seconds |
|---|---|---|---|
| `❄️cooling.🎬️clip.👥️internal-gains.🧪️tests.🐍️` | 3 | OK | 2s |
| `❄️cooling.🎬️clip.📐️system-sizing.🧪️tests.🐍️` | 4 | OK | 18s |
| `❄️cooling.🎬️clip.🧪️tests.🐍️` | 4 | OK | 4s |
| `🌬️ventilation.🎬️clip.📦️volume.🧪️tests.🐍️` | 2 | OK | 4s |
| `🌬️ventilation.🎬️clip.🧪️architecture.🐍️` | 8 | OK | 5s |
| `🌬️ventilation.🎬️clip.🧪️architecture.🐍️numbers` | 4 | OK | 1s |
| `🌬️ventilation.🎬️clip.🧪️architecture.🐍️transitions` | 7 | OK | 2s |
| `🏗️authoring.🎨️visuals.🧍️staffage.🧪️tests.🐍️` | 64 | OK | 24s |
| `🏗️authoring.🎨️visuals.🧍️staffage.🧪️tests.🐍️lod` | 21 | OK | 7s |
| `🏗️authoring.🎨️visuals.🧪️tests.🌍️captions.🐍️` | 2 | OK | 1s |
| `🏗️authoring.🎨️visuals.🧪️tests.🎨️semio-style.🐍️` | 10 | OK | 223s |
| `🏗️authoring.🎨️visuals.🧪️tests.🐍️` | 7 | OK | 2s |
| `🏗️authoring.🎨️visuals.🧪️tests.🐍️course_staffage` | 5 | OK | 7s |
| `🏗️authoring.🎨️visuals.🧪️tests.🐍️icons` | 8 | OK | 11s |
| `🏗️authoring.🎨️visuals.🧪️tests.🐍️integration` | 9 | OK | 172s |
| `🏗️authoring.🎨️visuals.🧪️tests.🐍️labels` | 5 | OK | 2s |
| `🏗️authoring.🎨️visuals.🧪️tests.🐍️media_staffage` | 2 | OK | 1s |
| `🏗️authoring.🎨️visuals.🧪️tests.🐍️scenes` | 15 | OK | 12s |
| `🏗️authoring.🎨️visuals.🧪️tests.🐍️section_consumers` | 9 | OK | 10s |
| `🏗️authoring.🎨️visuals.🧪️tests.🐍️sections` | 9 | OK | 4s |
| `🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only` | 41 | OK | 107s |
| `🏗️authoring.🎨️visuals.🧪️tests.🐍️transforms` | 2 | OK | 7s |
| `🏗️authoring.🎨️visuals.🧪️tests.📟️readout.🐍️` | 5 | OK | 6s |
| `🏗️authoring.🎬️clip.🎚️profile.🧪️tests.🐍️` | 17 | OK | 3s |
| `🏗️authoring.🎬️clip.🎞️playback.🧪️tests.🐍️` | 6 | OK | 2s |
| `🏗️authoring.🎬️clip.👥️audience.🧪️tests.🐍️` | 7 | OK | 39s |
| `🏗️authoring.🎬️clip.📁️output.🧪️tests.🐍️` | 9 | OK | 10s |
| `🏗️authoring.🎬️clip.📤️export.🎞️course.🧪️tests.🐍️` | 4 | OK | 3s |
| `🏗️authoring.🎬️clip.📤️export.🧪️tests.🎬️e2e.🐍️` | 1 | OK | 9s |
| `🏗️authoring.🎬️clip.📤️export.🧪️tests.🐍️` | 21 | OK | 6s |
| `🏗️authoring.🎬️clip.🧪️tests.🐍️` | 32 | OK | 35s |
| `🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🎥️camera.🐍️` | 1 | OK | 59s |
| `🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🐍️` | 11 | OK | 319s |
| `🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.💾️persisted.🐍️` | 4 | OK | 1s |
| `🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.📚️closure.🐍️` | 1 | OK | 0s |
| `🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🕰️motion.🐍️` | 1 | OK | 1s |
| `🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🖱️hover.🐍️` | 1 | OK | 15s |
| `🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🧹️staging.🐍️` | 1 | OK | 1s |
| `🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🪡️axis.🐍️` | 1 | OK | 1s |
| `🏗️authoring.🎬️clip.🪟️presentation.📐️geometry.🧪️tests.🐍️` | 8 | OK | 13s |
| `🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction.🧪️tests.🐍️` | 21 | FAIL | 472s |
| `🏗️authoring.🎬️clip.🪟️presentation.🖼️posters.🧪️tests.🐍️` | 22 | OK | 231s |
| `🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🧪️tests.🐍️` | 8 | OK | 2s |
| `🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🧪️tests.🧩️chapters.🐍️` | 12 | OK | 482s |
| `🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🧪️tests.🧩️chapters.🐍️pointer` | 1 | OK | 19s |
| `🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.⏱️purity.🐍️` | 6 | OK | 2s |
| `🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🐍️` | 15 | OK | 461s |
| `🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🖼️stream.🐍️` | 3 | OK | 6s |
| `🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🪵️diagnostics.🐍️` | 2 | OK | 1s |
| `🏗️authoring.📏️running-example.🧪️tests.🐍️` | 10 | OK | 1s |
| `🏗️authoring.📚️references.🧪️tests.🐍️` | 10 | OK | 13s |
| `🏗️authoring.📝️narration.🧪️tests.🛡️guard.🐍️` | 3 | OK | 8s |
| `🏗️authoring.📝️narration.🧪️tests.🧮️tex.🐍️` | 9 | OK | 4s |
| `🏗️authoring.📝️narration.🧪️tests.🪟️caption-layout.🐍️` | 8 | OK | 9s |
| `🏗️authoring.🔊️speech.🧪️tests.🗣️lexicon.🐍️` | 12 | OK | 1s |
| `🏗️authoring.🔤️fonts.🧪️tests.🐍️` | 14 | OK | 15s |
| `🏗️authoring.🔤️fonts.🧪️tests.📑️heading.🐍️` | 3 | OK | 15s |
| `🏗️authoring.🗺️roadmap.🧪️tests.🐍️` | 11 | OK | 3s |
| `📊️demand.🎬️clip.⚖️regulations.🧪️tests.☀️sun.🐍️` | 2 | OK | 3s |
| `📊️demand.🎬️clip.🏭️primary-energy.🧪️tests.🐍️chain` | 6 | OK | 4s |
| `📊️demand.🎬️clip.🏭️primary-energy.🧪️tests.🐍️scale` | 5 | OK | 3s |
| `📊️demand.🎬️clip.🔁️recap.🧪️tests.🧮️balance.🐍️` | 4 | OK | 3s |
| `📊️demand.🎬️clip.🧪️tests.🔗️chain.🐍️` | 8 | OK | 2s |
| `📊️demand.🎬️clip.🧪️tests.🛡️guard.🐍️` | 8 | OK | 3s |
| `🔥️heating.🎬️clip.🧱️conduction.🧪️tests.🐍️` | 2 | OK | 7s |
| `🧲️physics.🎬️clip.🧲️fundamentals.🧪️tests.🐍️` | 13 | OK | 6s |

## What I changed
- `🏗️authoring/🎨️visuals/🧪️tests/🐍️staffage_only.py` `clip_sources`: `ROOT.rglob` walked into `.venv` and `node_modules` before filtering and hung for 20 minutes (the first test needing it, `test_every_call_site_mirror_is_declared_with_a_reason`); now `os.walk` with pruning of `skip_folders`, same result set, module takes about 100 s.
- `🏗️authoring/🎨️visuals/🧪️tests/🔣️staffage_only.json`: allowlisted `ventilation/🌡️heat-flow::unit_block` in `rule_a.allowed` (a framed formula card from the new unit comparison beat, not a physical object).

## What is left
1. `🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧪️tests/🐍️.py`, `test_four_fps_transitions_capture_current_native_clicks_at_every_required_level`, subtest `phase='focus'`: events per target are `['pointerdown','pointerdown','pointerup','click']`, expected one `pointerdown`. Reproduced twice (about 470 s). Cause is in runtime, not in the test: `🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟨️render.mjs` line 157 (`if(frame.cursor?.pressed&&!down){await page.mouse.down();down=true}`) runs for every pressed frame because each frame opens a fresh page (line 140-141, `down=false`), so a press that spans two or more frames reports one `pointerdown` per frame. Fix there: when `previous?.cursor?.pressed` and the current frame is pressed too, replay the press as the release branch does (lines 143-150: render previous, move, `mouse.down()`, then clear `window.semioPresentation.pointerEvents.length=0`) before rendering the frame. I did not change it, since the final videos were just rendered from the current sources.
