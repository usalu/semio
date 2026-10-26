# Final Runtime Audit

Read-only source review of the new teaching rendering runtime, CLI, TypeScript launcher, optional TTS integration, and full ventilation wrapper. Render jobs and processes were not inspected or changed. Existing test results were supplied by the parent and were not repeated.

## Finding

### P2: Invalidate the completion sidecar before overwriting a cached clip

`🎓️teaching/video_runtime.py:188–199` retains an existing `.render.json` throughout a forced rerender. With unchanged settings and sources, that sidecar continues matching the requested state. If cancellation or tool failure occurs while Manim writes its final MP4, an incomplete replacement exceeding the 1,000-byte size check can be reused on the next ordinary render (`reusable_clip`, lines 132–140). This matters for the documented interrupted-render recovery behavior.

Unlink the old sidecar immediately before invoking Manim and recreate it only after successful completion. A regression test should begin with an existing matching clip/sidecar, force a rerender whose command writes a partial replacement then raises, and assert that the replacement is not reusable.

## Other Reviewed Behavior

The manifest resolves all five lesson wrappers, the CLI validates dimensions and positive frame rates, concat paths escape apostrophes and normalize separators, and the TypeScript launcher addresses process-tree cancellation on Windows and POSIX. No additional concrete correctness issue was identified in the bounded review.

## Verification And Preview Audit

### P2: Require One Final Movie Per Manifest Lesson

`verify-videos.py:17–19` accepts any five MP4 files under directories named `rendered`; line 33 maps each independently by filename prefix. Five valid output variants or copied names from one lesson can therefore pass while other lessons are missing. The loop also never asserts the required 31 distinct source clips. Require the exact five manifest output basenames for 480p15 (or assert a one-to-one mapping to the five manifest IDs), and assert the total distinct expected source paths is 31.

For each selected movie, the verifier does compare actual FFprobe frame counts with source clip frame totals, fully decode through PyAV, and require FFmpeg decoding with `-xerror` to succeed. These checks are useful and were not executed during this read-only review.

### P3: Make Missing Preview Frames An Explicit Failure

`make-previews.py:24–31` labels and saves a panel even if no decoded frame reaches its target timestamp, leaving a blank region presented as a preview. Track a selected frame and raise if absent. The script also succeeds when no movies are found (line 17); requiring five inputs would prevent an empty preview run from looking complete.

## Verification Fix Review

The current verifier requires exactly five files and the exact manifest output names ending in `_480p15.mp4`, then checks all 31 distinct expected source clip names were visited. The preview script requires the same five names and asserts each target produced a decoded frame. Both previously reported verification findings are resolved by source review.

## Installed Manim Prefix Resume Semantics

Reviewed the ticket-local Manim source under `🗑️generated/v/Lib/site-packages/manim`. `CairoRenderer.update_skipping_status` (renderer/cairo_renderer.py:273–277) skips plays whose zero-based index is below `from_animation_number`; `play` still compiles, begins, and finishes the skipped animation, and increments `num_plays` (lines 81–137). Skipped partials are registered as None (lines 111–113), so prefix movies must be independently retained and assembled in order.

Prefix skipping is not a general scene-state checkpoint. `Scene.get_time_progression` (scene/scene.py:1125–1129) uses one update at `run_time` when skipped, versus `np.arange(0, run_time, 1/fps)` during normal rendering. `Scene.play_internal` (1401–1413) then finishes animations but skips the normal final zero-dt mobject update. `update_to_time` (1714–1724) propagates these different dt sequences to animation, mobject, mesh, and scene updaters.

Timeline state also differs: skipped plays add exact `scene.duration` before simulation (cairo_renderer.py:83–86), whereas rendered frames increment time by their frame count divided by fps (207–211); `Scene.time` directly exposes that clock (scene/scene.py:286–288). Non-frame-aligned durations and time-based callbacks therefore require explicit equivalence proof.

Concrete authored exposure includes dt-accumulated clocks in `energy/demand/3_heating/1_introduction/scene_1.py:625` and `:791`, another in `3_heating/2_conduction/scene_2.py:405`, and scene-time-dependent cycling in `5_energy_balance/3_bedarf_vs_leistung/scene_3.py:1064`. A tiny comparison consisting only of endpoint Transform animations does not establish equivalence for these cases; include dt clocks and non-frame-aligned durations. This review did not render or change any scene.

## Final Finding Resolution

All three actionable findings from this audit are resolved by final source review.

1. `video_runtime.py:196` now unlinks the completion sidecar before Manim can overwrite the clip. Lines 198–199 reject a render if the fingerprinted authored sources changed during execution, before line 202 publishes completion metadata. The language-neutral `render_cache_publication` cases and `test_render_cache_publication_requires_completed_unchanged_sources` exercise both an interrupted forced overwrite larger than the reuse size threshold and a successful command accompanied by an authored source edit; both require the sidecar to remain absent and concatenation not to run. This closes the interrupted-render cache finding. The supplied cache-fix report records all eight tests passing with real Manim enabled; tests were not rerun during this read-only final review.
2. The verifier now enforces the exact five manifest movie names and coverage of 31 distinct expected clips, closing the movie selection finding.
3. The preview script now requires the exact five inputs and a decoded selected frame for each requested target, closing the blank/empty preview finding.

The installed-Manim prefix-skip analysis above remains evidence about recovery semantics, not an unresolved defect in these reviewed fixes. No implementation changes or render/process operations were performed by this audit agent.

## Final Cancellation And UTF-8 Review

The final owned-process cancellation change passes bounded source review. `run_command` starts POSIX tools in a new session and targets only the resulting process group using its owned Popen PID; it attempts SIGTERM and then SIGKILL for surviving descendants. Windows cancellation targets only `/PID <owned Popen PID> /T /F`, covering the virtual-environment redirector tree without process-name or global process termination. The original exception is re-raised after waiting for the owned child. The ordinary success branch still returns None, and nonzero exit still raises CalledProcessError with the actual command and exit code.

`render_videos.main` reconfigures both stdout and stderr to UTF-8 where supported, keeping the change in CLI entry rather than imported library behavior. Reviewed the supplied cancellation report and regression test coverage for a real owned child/grandchild tree, success/failure statuses, and hostile-encoding CLI help/error output. The report records 11 tests passing with actual Manim enabled on native Windows; POSIX behavior was reviewed in source, not executed on a POSIX host. No additional actionable finding arose in this bounded review, and no process or background render operation was performed.

## Partial Cache Temporal Semantics

A final direct inspection of installed Manim 0.22 confirms that partial cache hits share the skipped-prefix temporal shortcut. `CairoRenderer.play` (renderer/cairo_renderer.py:104–110) sets `skip_animations = True` and adds exact `scene.duration` when `is_already_cached` succeeds. It still begins animations (line 123) and invokes `Scene.play_internal` for non-frozen plays (line 134). `Scene.get_time_progression` (scene/scene.py:1125–1129) consequently evaluates one `[run_time]` update instead of normal frame steps; `play_internal` (1412–1413) also omits the final zero-dt mobject update. This is source-confirmed behavior, not an assumed equivalence.

Therefore every actual chapter render should disable Manim partial caching to retain the normal simulation path for the authored dt/time-dependent updaters already identified above. Reuse of a complete chapter with matching provenance can remain, because it does not reconstruct scene state before rendering later plays. The pipeline agent was notified of this source evidence. The parent reports current QA jobs already disable partial caching, so this review adds no finding against their output. No render or process operation was performed.

## Pango SVG Cache Mutation Check

Installed primary source confirms the reported hardlink race. `manim/mobject/text/text_mobject.py:540–541` calls `PangoUtils.remove_last_M` unconditionally after `_text2svg`; `_text2svg:847–848` returns the existing SVG path on a cache hit. Installed `manimpango/utils.py:43–49` reads that path, removes a trailing `Z M ...` path fragment with a regular expression, then opens the same path in write mode and writes the result. Write mode truncates the file even when normalization made no change. Hardlinked per-job paths therefore share a mutable file and its temporary empty state.

The normalization itself is idempotent: its replacement omits the matched M fragment and does not create another match. Serial cache hits retain the same normalized SVG geometry. Breaking generated hardlinks into private per-job copies is the appropriate fix for the observed cross-job empty-XML failure. There is no source-based reason to rerender already-successful movies solely because they used linked caches; rerender failed/incomplete jobs, and retain normal full decode and visual QA for completed outputs.

Product `render_config` uses `{media_dir}/t`; compose defaults media_dir to the distinct lesson index and renders its chapter loop sequentially. The reviewed product runtime has no hardlink creation. This finding concerns the parallel QA cache provisioning rather than an additional defect in that sequential product path. Portability agent received the source evidence. This review did not read, modify, or repair live cache files and did not operate render processes.
