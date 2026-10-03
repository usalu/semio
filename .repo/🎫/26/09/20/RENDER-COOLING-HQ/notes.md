# Render Cooling HQ

Goal: `🎯r2602🎯updateddocs🎯updateduserdocs🎯updatedtutorials`

Repo MCP was unavailable (`repo://goals`, `ticket_open`, `ticket_close`). Work logged here.

## Deliverable

One 1080p60 series file, intro first, then Teile 1–6 in curriculum order:

`tutorial/energy/demand/Cooling/rendered/Full_Cooling_Demand_1080p60.mp4`

Also copied as:

- `tutorial/energy/demand/Cooling/rendered/Full_Cooling_Demand_WithAudio_1080p60.mp4`
- `tutorial/energy/demand/Cooling/media/videos/full_cooling_video/1080p60/FullCoolingDemandVideo.mp4`

Specs: 1920×1080, 60 fps, H.264 High, **no audio**, **1648.7 s (~27:29)**, 81 MB.

Silent copies:

- `tutorial/energy/demand/Cooling/rendered/Full_Cooling_Demand_NoAudio_1080p60.mp4`
- `tutorial/energy/demand/Cooling/media/videos/full_cooling_video/1080p60/FullCoolingDemandVideo.mp4`

VO copy kept separately as `Full_Cooling_Demand_WithAudio_1080p60.mp4`.

Order and durations:

1. Demo_Intro_Kuehllast — 14.8 s
2. Cooling_01_HeatingVsCooling — 103.0 s
3. Cooling_02_InternalGains — 243.9 s
4. Cooling_03_TransmissionHumidity — 167.0 s
5. Cooling_04_SolarRadiation — 181.7 s
6. Cooling_05_Systemauslegung — 397.8 s
7. Cooling_06_Lueftungssysteme — 540.5 s

## How it was built

A fresh `full_cooling_video.py -q h --force` Manim pass could not start: this machine currently wedges on reads of several `.venv` inodes (`defusedxml` pyc, `Cython/Compiler/__init__.py`, SciPy `array_api_compat`). PIL/Manim imports hang at 0% CPU.

Merged the existing 1080p60 HQ clips (libx264 `slow` / CRF 15 + AAC, from 2026-09-12) with ffmpeg concat copy:

```
ffmpeg -y -f concat -safe 0 -i concat_with_audio.txt -c copy Full_Cooling_Demand_WithAudio_1080p60.mp4
```

`full_cooling_video.py` compose now matches Heating: quality-folder lookup, silent concat `-c:v copy -an`, NoAudio copy path. That path was not used for this deliverable because a live Manim re-render is blocked.

2026-09-20 follow-up: stripped AAC from the merged file (`ffmpeg -c:v copy -an`) so the main deliverable matches Heating / EnergyBalance (silent, captions only).
