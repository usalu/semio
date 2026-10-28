# Actual Encoded Movie Audit

Inspected German speaker source movies with registered FFmpeg, using the existing imageio FFmpeg executable via SEMIO_TOOL_FFMPEG. No builds or tests run. FFmpeg metadata inspection intentionally has no output target and returns 1 after printing valid metadata; extraction calls returned 0.

| Source | Duration | Encoded format | Inspected Frame |
| --- | --- | --- | --- |
| s/120/v/Beat1_LocalGlobalExtrema.mp4 | 142.40 s | H.264, 1920×1080, 30 fps | 60 s |
| s/121/v/Beat2_GridRandomSampling.mp4 | 143.47 s | H.264, 1920×1080, 30 fps | 90 s |
| s/122/v/Beat3_FactorialExperiments.mp4 | 181.03 s | H.264, 1920×1080, 30 fps | 110 s |

All source paths are beneath .🧬semio/🎓️teaching/⚡️cache/videos/de/speaker/. PNGs are ticket/🗑️generated/movie-audit/extrema60.png, sampling90.png and experiments110.png, each inspected with view_image.

- Extrema: global green square is clearly visible at x=2. Global label is readable, but turquoise curve crosses the lower-left of "globales Minimum" and meets its lower-right edge. Sent coordinator actionable remaining label-placement finding. Local marker, objective direction, domain, optimum, title and complete German caption are visible.
- Sampling: full Latin-hypercube diagram contains nine points, marginal intervals and highlighted strips. Title, axis labels, right-hand annotations and complete caption fit.
- Experiments: full response diagram, model equation, response axis label, B=-1/A direction annotation and complete German caption fit. No overlap found in this frame.
- Sources s/123/v/Beat4_NormalizedIndex.mp4, s/124/v/Beat5_ParetoFront.mp4 and s/125/v/Beat6_EvolutionarySearch.mp4 did not exist at metadata probe. Index score/winner and actual Pareto full-static-diagram checks remain pending.
- This audit inspects representative actual movie frames, not every movie frame or audio playback. Wrapper files have not yet been inspected.

The initial FFmpeg extraction argument -frames:v was incorrectly split by command argument forwarding; replacing it with -vframes fixed extraction. No source edits performed.

## Parent Correction

The actual142.40second extrema movie frame at60s confirmed the global-minimum label crossed the curve. Parent moved the label to y=1.4, aligned with the local-minimum label above the horizontal axis. This changes shared visual provenance; final renders must be refreshed after the correction. Captions/narration and mathematics remain unchanged.

## Index and Pareto Movie Follow-Up

- s/123/v/Beat4_NormalizedIndex.mp4: 167.53 s, H.264, 1920×1080, 30 fps. Inspected index105.png, index115.png, index125.png, index140.png and index150.png at corresponding seconds. Both B and C winner annotations are cleanly separated from numeric scores. Initial disappearance suspicion was retracted: parent independently opened index140.png and confirmed title/equation; FFmpeg title-region signalstats also confirms bright foreground pixels. Repeated image display in this agent context appears to expose visual deltas and must not be mistaken for missing encoded objects.
- s/124/v/Beat5_ParetoFront.mp4: 181.87 s, H.264, 1920×1080, 30 fps. Inspected pareto65.png and pareto110.png: complete title, axes, objective directions, full static points and trade-off/dominance annotations remain visible. At 65 s caption is mid-fade; 110 s complete caption fits. Earlier sparse harness captures are not established movie defects.
- Coordinator moved global label anchor above axis to resolve curve overlap; refreshed actual extrema movie frame inspection remains pending after rerender.

Parent directly inspected index140.png: title and index formula are present, scores and C winner label do not overlap. The disappearance finding was retracted after independent pixel-region verification. Parent also inspected Pareto110 and Experiments110 actual movie frames: complete diagrams and readable captions. Initial render correctly refused publication after the global-label source edit; clean refresh started after sources stabilized.

- s/125/v/Beat6_EvolutionarySearch.mp4: 287.07 s, H.264, 1920×1080, 30 fps. Inspected evolution145.png during mutation transform and evolution160.png after transform. Parents, arithmetic crossover, mutation, offspring and complete caption fit. All six raw movie metadata checks are now complete; refreshed extrema pending.

## Final Extrema Correction Verified

Re-extracted the refreshed s/120/v/Beat1_LocalGlobalExtrema.mp4 at 60 seconds to 🗑️generated/movie-audit/extrema60-final.png and inspected actual pixels. Global label is now above the horizontal axis, aligned with the local label and clearly separated from the entire curve. Global square remains visible at x=2. Previous label/curve overlap is resolved. Wrapped rendered directory is not present yet; six final wrapper metadata checks await completed envelopes.

## Wrapped Delivery Audit

Optimization_Extrema_speaker_de_1080p30.mp4: 148.40 s, H.264 1920×1080 at 30 fps, AAC stereo 48 kHz. Inspected actual encoded frames at 1.5 s (opening six-card navigation), 63 s (interior source 60 s) and 146.8 s (closing six-card navigation). PNGs wrapped-extrema-open.png, wrapped-extrema-inside.png and wrapped-extrema-close.png are under generated/movie-audit. Interior title, complete caption, all axes, constraints, local marker and global square/raised label fit inside the red window border without source cropping. Opening and closing navigation show all six optimization cards. No wrapper defect found in inspected frames.

Optimization_Sampling_speaker_de_1080p30.mp4: 149.47 s, H.264 1920×1080 at 30 fps, AAC stereo 48 kHz. Inspected actual encoded frames at 1.5 s, 93 s and 147.8 s, saved as wrapped-sampling-{open,inside,close}.png. Both navigation screens show all six cards; interior full Latin-hypercube plot, title, labels and complete German caption fit within window without crop. No defect found.

Additional final wrappers: Optimization_Algorithms_speaker_de_1080p30.mp4 (293.07 s), Optimization_Index_speaker_de_1080p30.mp4 (173.53 s), Optimization_Pareto_speaker_de_1080p30.mp4 (187.87 s), all H.264 1920×1080 at30 fps with AAC stereo48 kHz. Actual opening1.5s and closing291.5/171.9/186.2s frames each show six-card navigation. Actual interior163/143/113s frames respectively show complete titles, captions and source diagrams within the window. Offspring/mutation display, index score/winner separation, and full Pareto axes/points/annotation all fit without crop. Saved wrapped-{algorithms,index,pareto}-{open,inside,close}.png; all inspected with view_image. No defect found in these frames.

Optimization_Experiments_speaker_de_1080p30.mp4:187.03 s, H.2641920×1080 at30fps, AACstereo48kHz. Actual encoded opening1.5s/interior113s/closing185.4s frames inspected, saved wrapped-experiments-{open,inside,close}.png. Both navigation screens retain six cards. Interior title, response equation, full plot, response-axis label, B/A annotation and complete two-line caption fit inside window with no source crop.

All six final wrapped movie metadata and three representative actual encoded frames per wrapper have now been inspected. No outstanding visual defect remains from this audit. Scope remains representative-frame review; no full audio playback or frame-by-frame review claimed.
