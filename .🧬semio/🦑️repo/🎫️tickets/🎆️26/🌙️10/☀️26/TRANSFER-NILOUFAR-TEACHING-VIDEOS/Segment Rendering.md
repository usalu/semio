# Declared Segment Rendering

The final manifest declares each series intro as `{script, scene}` and every logical chapter as an ordered list of authored `{script, scene}` beat segments. Its single schema has no alternate chapter-rendering representation. Original scene classes, playlists, PDF, timing files, and visual helpers remain authored inputs; the font helper separately fixes unsupported math and arrow glyphs.

| Video | Logical Chapters | Ordered Beat Counts | Global Render Ordinals |
| --- | ---: | --- | --- |
| Physical Fundamentals | 1 | 8 | 1–9 |
| Ventilation | 6 | 3, 3, 4, 6, 4, 2 | 10–32 |
| Heating | 6 | 9, 6, 1, 5, 8, 6 | 33–68 |
| Cooling | 6 | 3, 5, 3, 5, 2, 3 | 69–90 |
| Energy Balance | 7 | 4, 4, 4, 4, 4, 3, 3 | 91–117 |

The plan contains 112 authored beats and five intros, assembled into 26 chapters, 31 logical clips, and five movies. Actual Manim discovery verifies every beat's source identity and exact original playlist order. An independent AST comparison also verifies all 112 source/class pairs.

`render_videos.py` and the five series wrappers use this same product runtime. `--jobs` bounds all selected videos through one global worker pool; filtering videos preserves the global ordinals. Each whole segment has a private `s/<ordinal>/` media directory and is rendered without Manim partial-cache reuse or animation skipping. Successful segment provenance contains settings, declared source/class identity, and SHA256 hashes of the scene, timings/assets, and shared visual helpers. Logical chapter provenance contains the ordered segment states, and movie provenance contains the ordered logical chapter states.

Completed segments and chapters can be reused independently. Interrupted or source-changing renders publish no successful provenance. Individual worker failures preserve other results. Ctrl+C cancels queued futures first, then stops all active Windows descendant trees or POSIX process groups without waiting for unfinished expensive tasks to drain normally. Standard Manim frame progress is enabled; optional immutable per-segment attempt logs record the exact command, PID, and exit code.

Twenty runtime tests pass, including actual Manim discovery, a real three-segment composition with exact frame totals and decoded color order, actual child/grandchild cancellation, queued-work cancellation, bounded concurrency, failure isolation, SVG-cache preparation, and whole-chapter reuse without retained segment files. One OS symbolic-link capability test is skipped where Windows does not permit creating symlinks. Language-independent cases specify the counts, first ordinals, expected scheduling results, cache behavior, and composition frame/color contract.

## Corrected Validation Run

The corrected stable controller runs the product CLI at full 480p15 with a global limit of 14 workers. Its output root is `🗑️generated/g`, separate from all first-pass evidence. Valid first-pass SVG bytes seed only private copies with link count one. Supported German text retains its original cache identity and geometry; missing-symbol text receives new cache keys through the frozen font helper, so old boxed-symbol SVGs are not reused.

The stable controller completed with exit status 0 and no handoffs. All 117 native segment attempts exited successfully, with recorded peak concurrency 14 and 63.67 minutes of measured raw render wall time. A final native inventory found zero remaining owned render or assembly processes. Raw files are `g/s/<ordinal>/v/<scene>.mp4`; logical clips retain `g/1` through `g/5` output directories, and final movies remain in `g/rendered`.

The corrected movies contain 113,285 header-counted frames in total. Exact per-movie metrics and the frozen font-helper hash are retained in `corrected-execution.md`. These are execution results; the parent separately verifies every decoded raw segment, chapter totals against ordered segments, movie totals against ordered logical clips, and corrected glyph visuals before accepting the overall transfer.
