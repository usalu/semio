# Cooling Demand — Full Video

Plays the **Cooling demand series** in curriculum order:

1. Teil 1 — Heizen vs. Kühlen
2. Teil 2 — Interne Wärmegewinne
3. Teil 3 — Transmission & Feuchte
4. Teil 4 — Solarstrahlung
5. Teil 5 — Systemauslegung
6. Teil 6 — Lüftungssysteme

Entry file: [`full_cooling_video.py`](./full_cooling_video.py)

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
.venv/bin/python tutorial/energy/demand/Cooling/full_cooling_video.py -q l
```

Or from this folder with the venv already active (`(.venv)` in the prompt):

```bash
python full_cooling_video.py -q l
```

```bash
# medium / high
.venv/bin/python tutorial/energy/demand/Cooling/full_cooling_video.py -q m
.venv/bin/python tutorial/energy/demand/Cooling/full_cooling_video.py -q h

# render only, do not open the player
.venv/bin/python tutorial/energy/demand/Cooling/full_cooling_video.py -q l --no-play
```

VS Code / Cursor launch: **🛠️dev🎬manim cooling full**

---

## Where the video is written

Default media root is next to this file: `tutorial/energy/demand/Cooling/media/`

| Quality | Output |
|---------|--------|
| `-q l` | `media/videos/full_cooling_video/480p15/FullCoolingDemandVideo.mp4` |
| `-q m` | `media/videos/full_cooling_video/720p30/FullCoolingDemandVideo.mp4` |
| `-q h` | `media/videos/full_cooling_video/1080p60/FullCoolingDemandVideo.mp4` |

Open that `.mp4` in any player.

---

## Optional: single Manim scene (fragile)

```bash
.venv/bin/manim -pql tutorial/energy/demand/Cooling/full_cooling_video.py FullCoolingDemandVideo
```

This one scene can render all animations and then **fail while combining** partials. Prefer the Python script above.

Section scenes (for Sideview / debugging one chapter):

| Scene class | Content |
|-------------|---------|
| `Cooling_01_HeatingVsCooling` | Teil 1 |
| `Cooling_02_InternalGains` | Teil 2 |
| `Cooling_03_TransmissionHumidity` | Teil 3 |
| `Cooling_04_SolarRadiation` | Teil 4 |
| `Cooling_05_Systemauslegung` | Teil 5 |
| `Cooling_06_Lueftungssysteme` | Teil 6 |

Example:

```bash
.venv/bin/manim -pql tutorial/energy/demand/Cooling/full_cooling_video.py Cooling_01_HeatingVsCooling
```

---

## Notes

- The full series is **long** — start with `-q l`.
- Captions come from each beat’s `NARRATION`.
- This runner does not mux external VO audio; module `build_full_video.py` scripts do that separately.
- Do not mix Heating scenes into this playlist.
