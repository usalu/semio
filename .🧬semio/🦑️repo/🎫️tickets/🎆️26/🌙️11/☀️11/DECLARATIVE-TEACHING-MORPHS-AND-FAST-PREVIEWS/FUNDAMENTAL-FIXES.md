# Fundamental Defects Log

## FD-001 Relative UV_PROJECT_ENVIRONMENT builds a duplicate venv

- Symptom: the first focused unittest run printed `Installed 42 packages` and took 52 s instead of ~20 s; the venv was created at `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/.🧬semio/🎓️teaching/⚡️cache/python` instead of the shared `C:\git\semio\.🧬semio\🎓️teaching\⚡️cache\python`. (A stale sibling of that path from 2026-11-01 already existed, so earlier workers hit the same trap.)
- Reproduction: from the repo root run the COORDINATION.md command with `UV_PROJECT_ENVIRONMENT=.🧬semio/🎓️teaching/⚡️cache/python`; uv resolves a relative value against the `--project` directory, not the cwd.
- Root cause: relative path in the documented env; `📜️script.ts` uses an absolute `join(cache, "python")`.
- Fix: `$T/run-tests.sh` exports absolute `UV_PROJECT_ENVIRONMENT` and `SEMIO_TEACHING_CACHE_DIR` (derived from the script location). Workers must use the runner, or absolute paths. COORDINATION.md (coordinator-owned) should replace the relative values with absolute ones; the stray gitignored venv under the package directory is harmless and was left alone (not created by a verified-current owner; safe to delete later).
- Regression evidence: after the fix, all three suites ran in ~20 s each with no install step and used the shared venv (see below).
- Owner: runtime worker.

Baseline results (run via `RUN_NAME=env bash $T/run-tests.sh <module>`):

| Module | Result |
| --- | --- |
| `🎓️teaching.🏛️architecture.🔀️morph.🧪️tests.🐍️` | 37 tests OK (19.9 s) |
| `🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧪️tests.📦️preview.🐍️` | 9 tests OK (13.5 s) |
| `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🧪️tests.🐍️` | 61 tests OK (19.2 s) |

Non-defects observed: `pydub` SyntaxWarning (invalid escape sequences) is third-party, harmless; a manim `get_points` DeprecationWarning in staffage tests line 280 comes from the test's own call and does not fail.

## FD-playback-1: presentation browser never settles, page error `isElement is not defined` (found by playback, OPEN)

- Reproduction: `$T/playback-browser-probe.py` (two root-overview frames through `🪟️presentation/🌐️browser/🟨️render.mjs`); run with the COORDINATION.md Python env. `render.mjs` exits 1 at the settle check with page errors `isElement is not defined` repeated. The identical failure occurs with the HEAD copies of `🟨️render.mjs`/`🟦️.tsx` (original files copied under `$T/🗑️generated/orig` with junctions to the siblings), so it is not caused by the playback changes.
- Impact: every browser-backed presentation test (`PresentationNativeTest`, chapter DOM tests) errors; the real-pixel purity test `BrowserFramePurity` cannot run.
- Suspected root cause: a bare `isElement` identifier in the esbuild IIFE bundle of the shared UI (a dependency exporting `isElement` that is tree-shaken or aliased wrongly, or a vendored package under `🖱️ui/.p*/p`); not yet located. Owner: foundations. Regression evidence to add when fixed: `SEMIO_TEST_BROWSER=1 ... 🪟️presentation.🧪️tests.⏱️purity.🐍️`.

## FD-002 Cast frame selection ran on a second, history-dependent clock (timeline risk 2)

- Symptom: `staffage/🐍️.py` monkeypatched `Scene.update_mobjects`/`update_to_time` at import and read `renderer.time`, a float running sum that skip mode advanced by raw `scene.duration` and render mode by `frames/fps`; the motion anchored itself at its first update, which is the play END in a skipped play and the play START in a rendered one.
- Reproduction: `clock-repro.py` (stock) and `🧰️interop/🕐️clock/🧪️tests` `test_skipped_plays_end_on_the_rendered_clock`, `test_time_is_the_exact_frame_total`.
- Root cause: no canonical clock; scene time was a side effect of whatever had been rendered or skipped before.
- Fix: new shared module `🎓️teaching/🧰️interop/🕐️clock/🐍️.py` (`SCENE_CLOCK`, `frame_count`, `scene_seconds`, `stopwatch`, `scene_timer`): scene time is an integer frame count over the exact frame rate (`Fraction`), advanced per play by `frame_count(duration)` for rendered, skipped, cached and trimmed plays alike; `renderer.time` is overwritten with it. `SCENE_CLOCK.origin` is the play start while a play runs. Staffage now imports `SCENE_CLOCK` from it, anchors a motion at `origin`, and its private patch is deleted (`🎨️visuals/🧍️staffage/🐍️.py`, narrow edits: import, clock block, `sync` anchor + docstring).
- Regression evidence: clock tests (9 OK); staffage suite 60 of 61 OK, including `test_a_real_scene_keeps_the_motion_on_video_time_through_waits_and_plays`; the one error is `lod.id is not an object` from the bun staffage exporter in `test_progress_reaches_the_caller_and_the_cache_gets_the_cast` (in-flight LOD work of the staffage owner, not this change; owner: sonnet-staffage).
- Owner: runtime worker; staffage owner informed through this log.

## FD-003 dt-accumulating timers diverge between skip and render (timeline risk 1)

- Symptom: `ValueTracker` + `increment_value(dt)` clocks gave `sum(dt)`, which differs for a skipped play (one dt of the whole run time) and a rendered one (frame dt, quantised) and depends on callback history.
- Reproduction: `🧪️tests` `test_timer_is_rate_times_scene_time_in_every_mode`, `test_only_the_rate_integrators_still_accumulate_dt` (guard).
- Fix: `scene_timer(rate)` / `stopwatch(rate)` evaluate `rate · (scene time − start)` analytically. Replaced in `🔥️heating/🎬️clip/🧲️fundamentals` (2 sites), `🧱️conduction` (1), `🏗️bim/🎬️clip/🔗️consistency` (2, rates 0.8 and 0.3) and `📊️demand/🎬️clip/⚖️regulations/_level_flow`. Import check of all four clips passes.
- Residual resolved (FD-005): the `🧪️practice` rate integrators now use `rate_integral` (piecewise-linear rate track); the guard test's allowlist is empty.
- Note: a timer is not advanced during frozen waits (Manim runs no updaters there); it corrects itself at the next update, and the frozen frame shows no change by definition.

## FD-004 Frozen waits wrote floor(T·fps) frames, plays ceil (timeline risk 3, CONFIRMED at runtime)

- Symptom: `wait(0.5, frozen_frame=True)` at 15 fps wrote 7 frames and advanced `renderer.time` by 7/15, while `wait(0.5)` or a 0.5 s play write 8. Video length drifted a frame per frozen wait against narration.
- Reproduction: `$T/clock-repro.py` on stock Manim prints `written: 7`, time 1.5333 for the frozen wait; with `clock` argument it prints 8 and 1.6.
- Root cause: `CairoRenderer.freeze_current_frame` uses `int(duration/dt)`; `np.arange(0, T, 1/F)` for plays also yields a float-noise-dependent count.
- Fix (superseded by FD-005, coordinator design correction: plain `ceil` per play adds up to one frame per play): the clock rounds against the ABSOLUTE grid, see FD-005.
- Regression evidence: `test_written_frames_match_the_declared_count` compares against the frames Manim's own writer receives (independent oracle), JSON cases in `🧪️tests/🔣️.cases.json`.
- Owner: runtime worker.

## FD-005 PreviewSwap's terminal target never reaches the exported frame (found in actual exported output)

- Symptom: in an exported preview movie of a 10-frame transition at frames 5..14 the source was still drawn at frame 14 (blue 180 px, red 0) and the target first appeared at frame 15, one frame late. Scene-level tests (`🎭️stage` spy on `scene.mobjects`) passed because `scene.mobjects` was already correct.
- Reproduction: `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📤️export.🧪️tests.🎬️e2e.🐍️` with `SEMIO_TEST_MANIM=1` (renders the stage test document through `runtime.render_segment` at 160x90 and decodes the exported mp4).
- Root cause: Manim's Cairo renderer draws the flat `scene.moving_mobjects` list captured when the `play` call begins (`Scene.get_moving_mobjects`, read per frame at `scene.py` `self.renderer.render(self, t, self.moving_mobjects)`), not the animation group's current children. `PreviewSwap` only edited `stage.submobjects`, so the source leaves stayed in the list (still drawn) and the target leaves (never in the list) were not drawn until the next play.
- Fix: `PreviewSwap._redraw(hidden, shown)` edits the list in place at the swap (sources leave, targets join), called from `_show_target` and from the partial-lane swap in `_interpolate` (`🔀️morph/🖼️preview/🐍️.py`, narrow patch).
- Regression evidence: exported preview frames 5..13 have red == 0 and blue > 0, frame 14 has blue == 0 and red 282; final frames have genuine intermediates (mixed colour at frame 10) and the exact target at frame 14; e2e test `test_exported_preview_and_final_movies`. Owner: preview worker (informed through this log; verify no other animation relies on stage.submobjects for drawing).
- Open question for the final-morph owner: the same constraint applies to any custom animation that adds mobjects mid-play; `PartMorph` draws correctly because it creates its parts at `begin`.

## FD-005 Per-play rounding accumulated time; clock and timeline now round on the absolute grid (coordinator correction)

- Symptom: rounding each play up (`ceil(T·F)`) adds up to one frame per play, so a clip of many short plays runs longer than its authored total (six 0.1 s plays at 15 fps would be 12 frames, authored total 9); the timeline `count` pinned stock Manim's float-noise `np.arange` (0.28 s at 25 fps = 8 frames).
- Root cause: no absolute reference; every interval was rounded in isolation.
- Fix: authored time is an exact `Fraction` sum (microsecond-snapped); a play spanning `[t0, t0+T]` emits `ceil((t0+T)F) - ceil(t0 F)` frames (the grid points inside the interval), so any split totals `ceil(total·F)`. Sub-frame intervals (impossible through Manim, whose `validate_run_time` clamps to one frame, but valid timeline windows) borrow one frame and the next interval gives it back: `emitted(end, done) = max(done + 1, ceil(end·F))`. Frames of a play are sampled at their true absolute grid time (`t_k = (done+k)/F - t0`, `SCENE_CLOCK.phase` = frames from `t0` to the first sampled frame), so the last grid point inside the authored interval is the terminal sample. Files: `🎓️teaching/🏛️architecture/🔀️morph/⏱️timeline/🐍️.py` (`exact`, `rate`, `grid`, `emitted`, `count`, `first`, `frame_of_alpha(..., phase)`, `sample_alpha(..., phase)`), `🧰️interop/🕐️clock/🐍️.py` (rewritten state machine), `🔀️morph/🖼️preview/🐍️.py` (`_interpolate` uses `SCENE_CLOCK.phase`) and `🔀️morph/🎞️full/🐍️.py` (`_counted`), `INTERFACES.md` (timeline and scene clock sections).
- Rate integrators: `rate_integral(speed)` in the clock module (piecewise-linear rate track from the previous play's end knot, a stream starts at rest) replaces the `dt * speed * rate` sums in `🧪️practice` `_stream` and `_gas_meter`; rendered and skipped plays end on the same phase.
- Test expectation changes (with justification): `terminal.json` float-noise case 0.28 s at 25 fps is now 7 frames / `[0, 6]` (exact; stock Manim's arange adds an 8th); new absolute-grid cases (0.1 s from 0 and from 0.1 s at 15 fps, sub-frame borrow); `chain.json` "shared timestamp" case now uses a real overlap (adjacent windows partition the grid, so they no longer share a frame) plus a new adjacent-partition case; `CountOracle` replaced the numpy-arange pin with a `decimal` oracle (arange agrees except float-noise cases where it is exactly one more) and a random split test (sum of window counts = `ceil(total·F)`); `ManimOracle` now runs the real, clock-patched Manim and adds chained off-grid plays and skip parity.
- Regression evidence: `🧰️interop.🕐️clock.🧪️tests` 12 OK; morph `⏱️timeline` 39 OK, `🖼️preview` 28/29 (the one failure, `test_recipe_correspondence_uses_one_group_lane_without_full_morph`, is the preview owner's in-flight change of `_pairs` recipe handling: `recipe` now infers key/order lanes), `🎞️full` 31 OK, `📋️spec` 29 OK, `📍️placement` 10 OK, `✏️authoring` 30 OK, `🎭️stage` 8 OK, `🎬️clips` 7 OK; presentation purity, playback and export suites OK.
- Owner: runtime worker (change to the timeline module agreed with the coordinator; timeline owner busy with ventilation).

## FD-006 The cast cache key hashed every module of the archive, so any unrelated edit re-exported the whole course (staffage)

- Symptom: editing the DXF, PDF, video, ZIP, sheet or command-line exporters (or any test-only helper under `🔨️modules`) changed `_source_digest` and therefore the key of all 99 casts: one cold re-export of about 15,000 thermal frames and 270 MB per edit, so no one could iterate on the exporter or on a preview level without a multi-hour rebuild.
- Reproduction: `touch` any file under `🧍️staffage/🔨️modules/📤️export/📐️dxf`, call `S.cast_key(entry)` before and after: the keys differ.
- Root cause: `_source_digest` hashed `🔨️modules/**` and `🧬️schema/**` wholesale instead of the code the cast exporter actually loads.
- Fix (approved, coordinator decision 7): `_source_digest` hashes the transitive relative-import closure of `🎭️cast/🟦️.ts`, the whole `🎭️cast` folder (schema, levels file), `🧬️schema/**` and the bundled print profiles. A module the exporter never imports no longer re-keys any cast; one it imports (evaluation, depiction, thermal, transport, structure, revision, catalog loader, frames, svg) still does.
- Regression evidence: `🧍️staffage/🧪️tests/🐍️.py` `Cache.test_each_input_changes_the_key` (imported module changes the key, an unimported `dxf.ts` does not) and `test_the_key_ignores_line_endings_and_the_case_rule_of_the_path_order` (hash order over the closure); 61 tests OK.
- Owner: staffage worker.

## FD-007 Three staffage defects found while tracing the pipeline (staffage)

- `fitted_staffage` built a complete `_Motion` (slots, frame 0 parse) only to read `.width` and `.height` and threw it away (50 to 196 ms per call on the course casts). It now sizes from the logical descriptor (`staffage_extent`).
- `_slot_count` read the bytes of every frame file of a cast at construction (126 ms for a 600 frame cast). The exporter now writes `logical.maxPaths` into the index and the bridge reads that.
- `staffage_parts` parsed straight-line SVG paths with regexes and raised on a curve; the exporter now writes the polygons of frame 0 (`logical.partsM`, world metres) from the prepared shapes.
- Plus: the exporter loaded and parsed all 152 catalog assets (40 MB) for every batch; it now reads only the asset folders the casts name.
- Pre-existing test defects fixed on the way: four `🎭️cast-export` cases built manifests without the required `scale`, `fps` and timeline `ease`; `scenes.py` read the old index fields.
- Outside scope, not fixed (reproduce with `repo tools exec --light bun -- x vitest run --config 🧪️tests/🎚️config/🟦️.ts plan-projection`): `🧍️plan-projection` fails two cases on the current tree (`rigid minX` of the stride capsule, and `plan-car` `logical.heightM` 1.497 against a plan depth 2.08 (`plan-car: heightM is the physical figure height, not the plan depth`)); `heightM` is computed as before (`state.bounds` height), so the cause predates the LoD work.
- Owner: staffage worker.

## FD-005b Moving-list patch reviewed: state-based sync replaces the transition-edge `_redraw` (preview owner)

- Review finding: `_redraw(hidden, shown)` only edited `scene.moving_mobjects` at the swap edges. Any earlier evaluation that reached the target state (for example `begin()` evaluating with a stale `SCENE_CLOCK.phase`, or a seek to the target and back) removed the source family from the list, and the next source-state frames never re-added it, so the source was not drawn ("frame 5 lost the source", flaky because it depended on the clock phase at `begin`). It was also order dependent (hidden/shown sets were derived from the call sequence).
- Fix: `PreviewSwap._sync(key)` makes the list equal to the current stage state on every state change (key = target state or the set of swapped lanes): managed mobjects (source and target families) that are hidden leave it, visible ones missing from it join it in stage-family order; unmanaged entries are untouched. The result depends only on the current state. `begin` resets the key.
- Regression evidence (`🔀️morph/🧪️tests/🖼️preview/🐍️.py`, `MovingListTests`): premature target evaluation then source frames keep the source in the moving list; rendered Cairo pixels show only the source (blue) before f1 and only the target (red) at f1 for transitions starting on and off the frame grid (waits 0, 0.25, 0.23 s); four repeated renders are byte-identical, also with a `run_time` override; a 2.0 s override on a 1.0 s transition gives 19 source frames then the target.
- Also: `PreviewSwap` stretches its windows uniformly when `scene.play(..., run_time=...)` overrides the run time (stock Manim `setattr`s it on every animation), exactly as the former Swarm/Morph objects stretched; the lane builder now uses `spec.uses_parts` and infers key/order lanes for `recipe` correspondences with matching counts.
