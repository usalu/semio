# Lüftungssysteme continuous Seraphina VO

- Voice: `de-DE-SeraphinaMultilingualNeural` @ `-5%`
- Continuous whole-scene narrations (empty beat narrations)
- Manim holds in `scene_6.py` tuned so each HQ clip ≈ audio (±0.25s)
- Output: `tutorial/energy/demand/Cooling/6_lueftungssysteme/Full_Lueftungssysteme_vo_de.mp4`
- Per-beat WAV: `beat_N_audio.wav`
- Job: `.repo/🎫/26/08/04/CREATE-SYSTEMAUSLEGUNG-COOLING-VIDEO/lueftungssysteme-audio/`

| Beat | Video | Audio | Δ |
|------|------:|------:|--:|
| 1 Systemübersicht | 28.80 | 28.94 | +0.14 |
| 2 Freie Lüftung | 63.50 | 63.65 | +0.15 |
| 3 Mechanische Grundtypen | 37.60 | 37.82 | +0.22 |
| 4 Wärmerückgewinnung | 40.90 | 40.94 | +0.04 |
| 5 Luftführung | 42.70 | 42.74 | +0.04 |
| 6 RLT-Funktionen | 38.40 | 38.52 | +0.12 |

## Internal beat sync (2026-08-04 later)

Measured Seraphina segment durations and redistributed Manim `_pad(...)` holds so each visual phase lands with its spoken line (not only total duration).
Segment map: `audio_work/segment_timings.json`
