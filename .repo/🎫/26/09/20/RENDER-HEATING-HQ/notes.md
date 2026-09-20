# Render Heating HQ silent

Goal: `🎯r2602🎯updateddocs🎯updateduserdocs🎯updatedtutorials`

Repo MCP was unavailable. Work logged here.

Render intro `Demo_Intro_Heizlast` plus sections 1–6 at `-qh` (1080p60), German captions on screen, no VO, ffmpeg concat with `-an`.

Order:

1. Demo_Intro_Heizlast
2. Heating_01_Introduction
3. Heating_02_Conduction
4. Heating_03_Convection
5. Heating_04_InternalGains
6. Heating_05_Solar
7. Heating_06_FinalCalculation

Command:

```
.venv/bin/python tutorial/energy/demand/Heating/full_heating_video.py -q h --no-play --force
```

Finished 2026-09-20, ~20 min, exit 0.

- 1920×1080 @ 60 fps, H.264, no audio stream
- 1522.9 s (~25:23), 7 clips: intro + Modul 1–5 + final
- Deliverable: `tutorial/energy/demand/Heating/rendered/Full_Heating_Demand_NoAudio_1080p60.mp4`
