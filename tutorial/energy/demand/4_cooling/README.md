# Cooling Load — Full Video

Plays the **Cooling load series** (Kühllast, VDI 2078 design power — not annual
Kühlenergiebedarf) in curriculum order, prepended with the series intro card
``Demo_Intro_Kuehllast`` from ``tutorial/intro/intro_scene.py``:

0. Intro — Kühllast
1. Teil 1 — Heizwärmebedarf vs. Kühllast
2. Teil 2 — Interne Wärmegewinne
3. Teil 3 — Transmission & Lüftungslast
4. Teil 4 — Solare Einstrahlung
5. Teil 5 — Systemauslegung
6. Teil 6 — Sommerstrategie

Entry file: [`full_cooling_video.py`](./full_cooling_video.py)

This is the fourth video in the curriculum: Physical Fundamentals → **Lüftung** (`../2_ventilation/`) → Heizung →
**Kühlung** → Energiebilanz. Ventilation is not explained here any more — see the note at the end.

---

## Prerequisites

- Working directory: **semio repo root** (or any cwd — the script resolves paths)
- Repo `.venv` with Manim
- `ffmpeg` on PATH (needed for the reliable concat step)

```bash
source .venv/bin/activate          # macOS / Linux
.venv/bin/manim --version
ffmpeg -version
```

---

## How to run (recommended)

Render **each section separately**, then **ffmpeg-concat**. This avoids Manim’s mega-combine bug (`InvalidDataError` on `partial_movie_file_list.txt` when hundreds of partials / paths with spaces).

From the **semio repo root**:

```bash
# low quality + open the finished video when done
.venv/bin/python tutorial/energy/demand/4_cooling/full_cooling_video.py -q l
```

Or from this folder with the venv already active (`(.venv)` in the prompt):

```bash
python full_cooling_video.py -q l
```

```bash
# medium / high
.venv/bin/python tutorial/energy/demand/4_cooling/full_cooling_video.py -q m
.venv/bin/python tutorial/energy/demand/4_cooling/full_cooling_video.py -q h

# render only, do not open the player
.venv/bin/python tutorial/energy/demand/4_cooling/full_cooling_video.py -q l --no-play
```

VS Code / Cursor launch: **🛠️dev🎬manim cooling full**

---

## Where the video is written

Default media root is next to this file: `tutorial/energy/demand/4_cooling/media/`

| Quality | Output |
|---------|--------|
| `-q l` | `media/videos/full_cooling_video/480p15/FullCoolingDemandVideo.mp4` |
| `-q m` | `media/videos/full_cooling_video/720p30/FullCoolingDemandVideo.mp4` |
| `-q h` | `media/videos/full_cooling_video/1080p60/FullCoolingDemandVideo.mp4` |

Open that `.mp4` in any player.

---

## Optional: single Manim scene (fragile)

```bash
.venv/bin/manim -pql tutorial/energy/demand/4_cooling/full_cooling_video.py FullCoolingDemandVideo
```

This one scene can render all animations and then **fail while combining** partials. Prefer the Python script above.

Section scenes (for Sideview / debugging one chapter):

| Scene class | Content |
|-------------|---------|
| `Cooling_01_HeatingVsCooling` | Teil 1 |
| `Cooling_02_InternalGains` | Teil 2 |
| `Cooling_03_TransmissionHumidity` | Teil 3 — opake Transmission, Time Lag, Brückenbeat `Beat3_LueftungslastImSommer` |
| `Cooling_04_SolarRadiation` | Teil 4 |
| `Cooling_05_Systemauslegung` | Teil 5 — `Beat2_VolumeFlowEquation`, `Beat3_IsolateAirflow` (Last → Luftstrom → Kanal, gegen den Hygieneluftstrom) |
| `Cooling_06_Sommerstrategie` | Teil 6 — `Beat1_LastZuerstSenken`, `Beat2_SorptionsKuehlung`, `Beat3_KomfortStrategie` |

Example:

```bash
.venv/bin/manim -pql tutorial/energy/demand/4_cooling/full_cooling_video.py Cooling_01_HeatingVsCooling
```

---

## Notes

- The full series is **long** — start with `-q l`.
- Captions come from each beat’s `NARRATION`.
- Synced German VO: run each part’s `generate_audio.py` (synth), then re-render with
  `VO_TRACE=1` so `hold_for` uses measured clause lengths; align speech onto traced
  subtitle starts and mux. The series deliverable with audio is
  `rendered/Full_Cooling_Demand_1080p60.mp4` (also copied as
  `Full_Cooling_Demand_WithAudio_1080p60.mp4`).
- Do not mix Heating scenes into this playlist.
- Ventilation is explained once, in the **Lüftung** video (`../2_ventilation/`, second video of the curriculum, before
  Heating and Cooling). This series only plugs the summer into it: Teil 3 ends with the bridge beat
  `Beat3_LueftungslastImSommer` (`Q̇_V,sens` + `Q̇_V,lat` next to the opaque transmission), Teil 5 sizes the airflow of the
  solar load against the hygiene airflow of Lüftung Kapitel 2, and Teil 6 is the summer strategy that points to
  Lüftung Kapitel 4 (window, cross, stack, night) and 5 (supply/exhaust, heat recovery, duct).
