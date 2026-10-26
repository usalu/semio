# Manim Partial Recovery Proof

## Decision

Do not recover these authored chapters by concatenating retained uncached partials with a bare Manim -n N suffix. Real Manim 0.22.0 fails exact timeline and decoded-frame equivalence for the features used by this curriculum. Keep the complete replacement chapter renders running. No long render was launched, stopped, or modified by this experiment, and no authored asset changed.

## Local Implementation Evidence

The installed CairoRenderer.play implementation (manim/renderer/cairo_renderer.py, lines 83–86) advances skipped renderer.time by scene.duration. Normal frame writing instead advances renderer.time by emitted frames / camera.frame_rate (lines 206–211). Thus requested durations that do not align to frame boundaries produce different clocks when skipped. Scene.get_time_progression uses one [run_time] sample when skipping (manim/scene/scene.py, lines 1125–1128), which also changes the update history for dt-dependent mobjects. The local manim_visuals.hold_for uses renderer.time to charge visible subtitle time and calculate its remaining narration budget.

Parent's independent authored-source inspection also found dt-dependent clocks in heating chapter 1 (clock.increment_value(dt), source lines 625 and 791) and self.time-dependent cycling in energy-balance chapter 3. The experiment tests those exact categories of behavior rather than relying on static object endpoints alone.

## Real Experiment

The language-neutral recovery-cases.json defines a 320×180, 15 fps fixture with three animation calls. Two ValueTracker animations run for 0.10 and 0.17 seconds. A separate visible object updates its position using dt. The third call uses the actual authored manim_visuals.hold_for narration-budget helper.

Three real Manim invocations render: a complete reference; the first two animations using -n 0,1; and the suffix using -n 2. The completed uncached_00000.mp4 and uncached_00001.mp4 prefix files are concatenated with the suffix partial using the production FFmpeg concat interface. Every decoded RGB frame is hashed with SHA-256 using independent PyAV decoding and compared in order.

| Measurement | Complete Reference | Prefix Plus Resumed Suffix |
| --- | ---: | ---: |
| Decoded frames | 29 | 30 |
| Prefix frames | 5 | 5 |
| Suffix frames | 24 | 25 |
| Renderer clock after two plays | 0.3333333333333333 | 0.27 |
| ValueTracker after two plays | 2 | 2 |
| dt-updater X after two plays | -2.6 | -2.46 |

All retained prefix frames match the full reference. The first decoded mismatch is zero-based frame 5, exactly the splice boundary. Boundary offsets −1, 0 and +1 compare as true, false and false. The resumed result has one extra frame and a different visible updater state. Matching ValueTracker endpoints alone therefore does not establish recovery equivalence.

## Retained Inputs and Generated Evidence

Inputs retained at the ticket root: recovery-cases.json, recovery-scene.py, test-recovery.py, and run-recovery.ts. The run log, trace JSON, tiny partial MP4s, reference/recovered videos, and result.json are generated artifacts under 🗑️generated/recovery (log at 🗑️generated/recovery.log), to be removed with the other generated outputs after final task audits. This Markdown preserves the result after cleanup.

## Restart Preparation Addendum

The subsequent shared render_config fix removes only empty or malformed direct generated text-cache SVGs before fresh chapter simulation. Valid SVG caches remain available, while Manim partial movie caching stays disabled. This addresses interrupted SVG writes without changing the unsafe -n recovery conclusion above. Full fixture evidence, link-preservation boundaries, and final test results are in cache-fix.md. No live QA cache or authored visual source was changed by the agent implementing this preparation.
