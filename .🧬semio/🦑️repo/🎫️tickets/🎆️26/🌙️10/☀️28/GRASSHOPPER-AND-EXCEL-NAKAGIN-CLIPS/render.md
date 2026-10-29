# Render Report (quick previews, 480p15, speaker)

All 18 previews exist: nine videos in en and de, speaker only, `--quality l --fps 15`, delivered with provenance. No render process is left running.

Scripts in `render/`: `run-loop.sh <lang> <tag> <ids>` (one video at a time, up to 6 retries while sources change), `deliver.py` (env `DELIVER_PROFILE=480p15`, optional video ids merge into `delivery.json`), `newest.py` (names the newest fingerprinted sources of a scene). Logs: `🗑️generated/render/`.

Files: `.🧬semio/🎓️teaching/⚡️cache/videos/<lang>/speaker/delivery/<Output>_speaker_<lang>_480p15.mp4`, with `delivery.json` (schema `semio.architecture.delivery/v1`) in the same folder.

| Video | File stem | en | de |
|---|---|---|---|
| grasshopper_anatomy | Grasshopper_anatomy | 354 s | 324 s |
| grasshopper_capsule | Grasshopper_capsule | 305 s | 274 s |
| grasshopper_tower | Grasshopper_tower | 296 s | 271 s |
| grasshopper_casting | Grasshopper_casting | 301 s | 278 s |
| grasshopper_trees | Grasshopper_trees | 308 s | 299 s |
| excel_anatomy | Excel_anatomy | 265 s | 239 s |
| excel_references | Excel_references | 323 s | 297 s |
| excel_program | Excel_program | 315 s | 286 s |
| excel_energy | Excel_energy | 321 s | 286 s |

Frame check: three frames per file (20%, 50%, 80%), luma mean and std computed; none blank. Grasshopper trees is darker (mean about 65), Excel files about 80 to 120, the rest 130 to 155.

Pipeline notes: the renderer refuses segments whose sources changed during the render (fixed by retries and by the coordinator removing the BIM model import); the shared drafting `P()` tuple regression twice broke all Excel and slider renders until fixed at the drafting boundary.
