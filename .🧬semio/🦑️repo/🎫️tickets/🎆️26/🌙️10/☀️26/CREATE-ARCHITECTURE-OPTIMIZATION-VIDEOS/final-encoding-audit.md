# Final Encoding Audit

All six final German speaker MP4s and their nonempty .render.json certificates exist. Each published movie was decoded fully, one at a time, with the existing FFmpeg 7.1 executable through repo tools exec --light ffmpeg. Every decode exited 0; no media decode errors were reported. No tests or builds were run.

All videos are H.264 (High 4:4:4 Predictive), yuv420p progressive, 1920×1080, 30 fps; audio is AAC LC, 48 kHz stereo. This confirms encoded stream presence and integrity, not spoken narration or perceptual audio quality.

The authored focus and restore timings each total 3.00 s at 30 fps (90 frames), so expected final duration is raw source +6.00 s. Durations below use FFmpeg container metadata rounded to 0.01 s; every final matches this expected padding. Raw durations were previously recorded in movie-audit.md.

| Exact final filename | Raw seconds | Final seconds | Padding seconds | Bytes | Certificate bytes | Full decode |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Optimization_Extrema_speaker_de_1080p30.mp4 | 142.40 | 148.40 | 6.00 | 17873589 | 755562 | PASS |
| Optimization_Sampling_speaker_de_1080p30.mp4 | 143.47 | 149.47 | 6.00 | 21608735 | 755567 | PASS |
| Optimization_Experiments_speaker_de_1080p30.mp4 | 181.03 | 187.03 | 6.00 | 25902815 | 755584 | PASS |
| Optimization_Index_speaker_de_1080p30.mp4 | 167.53 | 173.53 | 6.00 | 28905895 | 755537 | PASS |
| Optimization_Pareto_speaker_de_1080p30.mp4 | 181.87 | 187.87 | 6.00 | 21321371 | 755550 | PASS |
| Optimization_Algorithms_speaker_de_1080p30.mp4 | 287.07 | 293.07 | 6.00 | 39721291 | 755577 | PASS |

Final directory: .🧬semio/🎓️teaching/⚡️cache/videos/de/speaker/rendered/.

Audit input script retained: audit-final-encoding.cjs. Temporary metadata JSON and FFmpeg logs are under 🗑️generated/encoding; parent owns their required cleanup.
