# Distribution Encoding Audit

All six compact distribution exports passed a complete one-at-a-time FFmpeg decode with exit 0 and no decode errors. The existing registered FFmpeg 7.1 was invoked through repo tools exec --light ffmpeg. No tests/builds or source edits were performed.

Each export is H.264 High profile, level 4.1 (AVC configuration profile_idc=100, level_idc=41), yuv420p progressive, 1920×1080 at 30 fps. The complete decoded frame count and container duration match its corresponding certified master exactly. AAC LC 48 kHz stereo metadata matches; SHA256 hashes of copied AAC packet payloads match the masters exactly.

These are distribution derivatives and intentionally have no master .render.json certificate. The original master encoding audit remains in final-encoding-audit.md.

| Exact distribution filename | Duration | Frames | Bytes | Decode / Master Match |
| --- | --- | ---: | ---: | --- |
| Optimization_Extrema_speaker_de_1080p30.mp4 | 00:02:28.40 | 4452 | 4285844 | PASS |
| Optimization_Sampling_speaker_de_1080p30.mp4 | 00:02:29.47 | 4484 | 4998097 | PASS |
| Optimization_Experiments_speaker_de_1080p30.mp4 | 00:03:07.03 | 5611 | 6065509 | PASS |
| Optimization_Index_speaker_de_1080p30.mp4 | 00:02:53.53 | 5206 | 6334447 | PASS |
| Optimization_Pareto_speaker_de_1080p30.mp4 | 00:03:07.87 | 5636 | 5207556 | PASS |
| Optimization_Algorithms_speaker_de_1080p30.mp4 | 00:04:53.07 | 8792 | 9328949 | PASS |

| Topic | Matching Master / Distribution AAC Payload SHA256 |
| --- | --- |
| Extrema | 67e6aeafca50aad76b2c116c8032ed41d56f85b9a280fabeeef96ee0b60bbaef |
| Sampling | 5eb8ed3dedc0ccb9d0fa8abb8cd7b634b059761940693b9f53b1e056c4c1432c |
| Experiments | 795a8d92dc1d0f1d9ca794c4a90bc322b2c08ec286f3b8bb41d6dc8e0c8947fe |
| Index | 293efa40af93ebdb34d179c075fb1766d2ecbd86584bf0721d84cb3de4e5463b |
| Pareto | e4337f2780f5ecadad186095809d72759e0712b8a971e739e6070ccfe9316c3a |
| Algorithms | 520f9b4a2286dfd1ded68f872abba3444a33da789065dc0996d06bc2c7303bc6 |

Input script retained: audit-distribution-encoding.cjs. Temporary output/logs remain under 🗑️generated/encoding for parent cleanup. Distribution files were audited in 🗑️generated/delivery before parent relocation.
