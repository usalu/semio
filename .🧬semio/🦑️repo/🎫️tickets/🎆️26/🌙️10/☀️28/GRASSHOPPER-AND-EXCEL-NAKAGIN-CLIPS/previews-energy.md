# Energy Course Preview Renders (English, Speaker, 480p15)

Route: (a) energy script, `render --video <id> --language en --audience speaker --quality l --fps 15 --no-play --log-output --jobs 4`, one video after another via `render/energy-chain.sh`. No guard aborts and no retries were needed, so the direct Manim route (b) was not used.
The script writes its masters to `.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/rendered/`; each file was copied unchanged to `.../en/speaker/preview/`. The masters stay in `rendered/`.
Frame size is 854x480 at 15 fps as reported by the video stream.

| Video id | Segments | Duration | Wall clock | Size |
|---|---|---|---|---|
| physical_fundamentals | 8 | 16:52 | about 34 min (20:15 to 20:49) | 46 MB |
| ventilation | 22 | 27:06 | about 55 min (to 21:45) | 106 MB |
| cooling | 22 | 16:51 | about 28 min (to 22:13) | 67 MB |
| energy_balance | 26 | 35:36 | about 59 min (to 23:12) | 126 MB |
| heating | 35 | 28:54 | about 35 min (to 23:47) | 126 MB |

Files: `C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/preview/` containing `Full_Physical_Fundamentals_speaker_en_480p15.mp4`, `Full_Ventilation_speaker_en_480p15.mp4`, `Full_Cooling_Demand_speaker_en_480p15.mp4`, `Full_EnergyBalance_speaker_en_480p15.mp4`, `Full_Heating_Demand_speaker_en_480p15.mp4`.

Scripts: `render/energy-chain.sh` (serial chain), `render/probe.py` (duration probe). Logs: `🗑️generated/render/energy-*.log` (delete at ticket close).
