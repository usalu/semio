# Distribution Provenance Audit

All six published compact delivery files pass the delivery.json provenance chain audit. Each filename and video identity matches its distinct canonical curriculum entry. Independent SHA-256 recomputation matches the final delivery file, canonical master and canonical render certificate recorded in the manifest; final byte counts also match. All master certificates identify the correct video and pass current nested authored-source hash validation through the runtime interface.

| Video | Delivery → Master → Certificate | Delivery Bytes |
|---|---|---:|
| optimization_extrema | Pass | 4285844 |
| optimization_sampling | Pass | 4998097 |
| optimization_experiments | Pass | 6065509 |
| optimization_index | Pass | 6334447 |
| optimization_pareto | Pass | 5207556 |
| optimization_algorithms | Pass | 9328949 |

The manifest declares German speaker delivery, 1920×1080 at 30 fps, H.264 High level 4.1, yuv420p, CRF 18, fast preset, four encoder threads and faststart. It declares copied AAC LC stereo at 48 kHz with no spoken narration. Manifest frame counts are positive and agree with their stated durations at 30 fps. These are manifest checks; independent stream decode/profile and copied-AAC verification are covered by the encoding audit.

All master and certificate paths resolve to the matching rendered filenames, and every delivery file stays inside the published delivery directory. No duplicate or missing video entry was accepted. Canonical movie certificate validation remains recorded separately in final-provenance-audit.md.

Read-only verification; no media or source edits. distribution-provenance-audit.py is the retained audit input.
