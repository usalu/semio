# Parallel Rendering

Eight additional Manim workers render complete authored chapters at 480p15. Each job owns a short numbered directory under `🗑️generated/pre/`, with independent configuration, text cache, partial movies, output video, and `render.log`. The four original in-flight scenes remain untouched.

After measurement showed 51% CPU use and 35.14 GiB free memory, the staging pool was expanded to eleven workers. The final controller isolates individual job failures and appends attempt logs instead of overwriting them. Controller replacement can cause tool-owned Windows process cleanup even when native child handles are retained; no further controller handoffs occur while children are alive. A separate read-only observer holds native query handles and records actual process IDs, parent IDs, job numbers, and exit codes. Completed attempt logs are copied to immutable PID-specific files. All active render/controller roots remain owned by the active agent until their work exits.

The initial scheduling process published the completed Ventilation intro. It was then replaced to ensure staging remains isolated while original composition jobs are active. Only the scheduling Python process was terminated; all eight Manim child processes continued rendering. `parallel-monitor.py` adopts their Windows process handles without sending signals, then schedules remaining chapters as worker slots become free. It never publishes staged clips into active composition directories.

`🗑️generated/pre/jobs.json` records each job's scene, source, video index, initial PID, and staging directory. `handoff.json` records the native process inventory captured before scheduler replacement. Completed clips receive settings and authored-source SHA256 provenance after their process exits successfully and source fingerprints remain unchanged. The dependency selection matches the product runtime, including intro assets.

| Staging Job | Video | Chapter |
| --- | --- | --- |
| 1 | Ventilation | Series intro |
| 2–7 | Ventilation | Chapters 1–6 |
| 8–12 | Heating | Chapters 2–6 |
| 13–15 | Cooling | Chapters 4–6 |
| 16–21 | Energy Balance | Chapters 2–7 |
| 22 | Heating | Chapter 1 replacement |
| 23 | Energy Balance | Chapter 1 replacement |

The original active scenes excluded from this queue are Physical Fundamentals' full chapter, Heating chapter 1, Cooling chapter 3, and Energy Balance chapter 1. Their original intros and Cooling chapters 1–2 were already complete. Final publication and concatenation occur only after original composition processes stop. The final verification checks every frame of every assembled video.

Owning-agent finalization unexpectedly ended three original rendering sessions and their child processes. Global native process inventory confirmed that Heating and Energy Balance chapter 1 remained incomplete, while Cooling chapter 3 had completed and chapter 4 had only begun. No detachment command terminated these processes: the guarded command refused to act after the original PIDs were absent. Their partial movies remain intact for recovery research. Full replacements for Heating and Energy Balance chapter 1 were started in isolated jobs 22 and 23 while a controlled Manim resume test evaluates recovery of completed original partials.

The controlled Manim resume test found that `-n` skipping changes frame rounding and updater history: its full reference had 29 frames, while retained partials plus resumed output had 30, with a different first frame after the splice. The full replacement renders therefore continue. Complete XML-validated SVG text caches accelerate repeated text shaping through private per-job byte copies published only when the destination does not exist. Existing active cache files are never replaced by this reuse helper.

An initial shared-inode cache experiment exposed a real Manim behavior: `Text` calls `PangoUtils.remove_last_M` even on cache hits, truncating and rewriting the SVG. Cross-job hard links therefore caused an empty-SVG parse error in Heating chapter 6. All 15,311 shared cache paths were atomically isolated into private files without interrupting renders; six persistently empty generated SVG entries were removed, and the failed full chapter was retried. No authored visual source changed. Local primary-source inspection confirms that the rewrite is idempotent when successful, so already-complete movies retain their normal decode and visual validation requirements.

The Physical Fundamentals process remains owned by the active video-runtime agent. `pf-watchdog.py` keeps its original wrapper and stdout pipe alive until the current Manim child exits successfully, then stops only the composition wrapper before it can enter redundant Ventilation renders. It records the completed Physical Fundamentals chapter's provenance after success.

`publish-videos.py` independently computes per-lesson clip coverage in `🗑️generated/pre/coverage.json`. It publishes completed staged clips only when their settings and dependency fingerprints match the current runtime. Original completed clips are decoded completely with PyAV before provenance is recorded. A lesson is assembled through the product runtime's `--concat-only` path as soon as its own clips are complete; lessons 2–5 do not wait for the active Physical Fundamentals process. Lesson 1 is gated by the watchdog's successful-child metadata. Publication results and original decoded-frame counts are recorded under `🗑️generated/pre/publication.json`.

A PyAV inspection of completed partial `uncached_00564.mp4` confirmed that Physical Fundamentals had reached beat 7, “Sensible und latente Wärme,” with the authored dark-background graph, equation panel, and kettle diagram visible at the expected 854×480 resolution. Its temporary progress screenshot is stored only beneath `🗑️generated/` and is removed with other generated validation output.

## Assembled First-Pass Full Lessons

| Lesson | Frames | Declared Format | Status |
| --- | ---: | --- | --- |
| Physical Fundamentals | 15,068 | 854×480, 15 fps | Complete |
| Ventilation | 24,332 | 854×480, 15 fps | Complete |
| Heating | 25,922 | 854×480, 15 fps | Complete |
| Cooling | 16,139 | 854×480, 15 fps | Complete |
| Energy Balance | 31,833 | 854×480, 15 fps | Complete |

Manim's native partial concatenation produces a small rational difference between average frame rate and its exact declared 15 fps rate. Publication validates the declared rate plus an average-rate difference below 0.001 fps and preserves every source timestamp and frame. Final verification uses the declared FFprobe `r_frame_rate` and independently decodes every final movie frame.

The five first-pass movies contain 113,294 frames in total. Visual QA subsequently identified missing math/arrow glyphs in several authored labels. Acceptance is held for a portable symbol-font diagnosis and actual rerenders of any affected clips; visual font changes are never rebased as nonvisual provenance changes.

## Active Validation Ownership

All first-pass render roots, retries, publisher, and native observer have finished. Heating chapter 1 and chapter 6 private-cache retries both exited successfully. The eleven-worker staging controller exited naturally after all its children finished, reporting its four historical failures: three command-only dependency guards were independently audited and rebased, and the SVG-cache failure was resolved by the full private-cache retry. The original Physical Fundamentals runner and its watchdog both exited successfully. There are no remaining first-pass children that could be lost on agent finalization; any glyph-corrected rerenders will use a new stable controller with private caches.
