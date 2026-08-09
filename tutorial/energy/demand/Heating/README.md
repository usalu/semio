# Heating Demand — Full Video

Plays the **Heating demand series** in curriculum order (no institute intro card):

1. Modul 1 — Einführung
2. Modul 2 — Transmission / Leitung
3. Modul 3 — Konvektion / Lüftung
4. Modul 4 — Interne Wärmegewinne
5. Modul 5 — Solarer Wärmegewinn
6. Final calculation

Entry file: [`full_heating_video.py`](./full_heating_video.py)

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
.venv/bin/python tutorial/energy/demand/Heating/full_heating_video.py -q l

# medium / high
.venv/bin/python tutorial/energy/demand/Heating/full_heating_video.py -q m
.venv/bin/python tutorial/energy/demand/Heating/full_heating_video.py -q h

# render only, do not open the player
.venv/bin/python tutorial/energy/demand/Heating/full_heating_video.py -q l --no-play
```

VS Code / Cursor launch: **🛠️dev🎬manim heating full**

---

## Where the video is written

Default media root is next to this file: `tutorial/energy/demand/Heating/media/`

| Quality | Output |
|---------|--------|
| `-q l` | `media/videos/full_heating_video/480p15/FullHeatingDemandVideo.mp4` |
| `-q m` | `media/videos/full_heating_video/720p30/FullHeatingDemandVideo.mp4` |
| `-q h` | `media/videos/full_heating_video/1080p60/FullHeatingDemandVideo.mp4` |

Open that `.mp4` in any player.

If you already finished a long high-quality render whose Manim combine step failed, a recovered file may already be at:

`tutorial/energy/demand/Heating/media/videos/full_heating_video/1080p60/FullHeatingDemandVideo.mp4`

---

## Optional: single Manim scene (fragile)

```bash
.venv/bin/manim -pql tutorial/energy/demand/Heating/full_heating_video.py FullHeatingDemandVideo
```

This one scene can render all animations and then **fail while combining** partials. Prefer the Python script above.

Section scenes (for Sideview / debugging one chapter):

| Scene class | Content |
|-------------|---------|
| `Heating_01_Introduction` | Modul 1 |
| `Heating_02_Conduction` | Modul 2 |
| `Heating_03_Convection` | Modul 3 |
| `Heating_04_InternalGains` | Modul 4 |
| `Heating_05_Solar` | Modul 5 |
| `Heating_06_FinalCalculation` | Final calculation |

Example:

```bash
.venv/bin/manim -pql tutorial/energy/demand/Heating/full_heating_video.py Heating_01_Introduction
```

---

## Notes

- The full series is **long** — start with `-q l`.
- Captions come from each beat’s `NARRATION`.
- This runner does not mux external VO audio; module `build_full_video.py` scripts do that separately.
- Do not mix Cooling scenes into this playlist.
