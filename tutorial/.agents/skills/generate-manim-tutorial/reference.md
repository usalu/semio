# Manim Tutorial — Reference

Companion to [SKILL.md](SKILL.md). Load only when generating or extending scenes.

## Canonical paths

### semio tutorial

| Path | Role |
|------|------|
| `tutorial/manim_fonts.py` | Cross-platform body font (`BODY_FONT` / `apply_body_font`) |
| `tutorial/energy/demand/Cooling/2_transmission_humidity/scene_3.py` | Best beat-split example |
| `tutorial/energy/demand/Cooling/2_transmission_humidity/generate_audio.py` | `NARRATIONS` + TTS loop |
| `tutorial/energy/demand/Cooling/2_transmission_humidity/build_full_video.py` | Render → mux → compose |
| `tutorial/intro/intro_scene.py` | Reusable NGS/IEK/LUH intro card (`NGSIntro`) |
| `tutorial/energy/demand/Heating/introduction/merged_scenes.py` | Legacy `SceneN` + `Full*Video` |
| `tutorial/pyproject.toml` | `manim`, `gtts`, `openai` |

### Now I Get It (pipeline source of truth)

| Path | Role |
|------|------|
| `~/Nowgetit/NowIGetIt/backend/pipeline/planner.py` | `PLANNER_SYSTEM` — scene plan JSON |
| `~/Nowgetit/NowIGetIt/backend/pipeline/scene_generator.py` | `MANIM_SYSTEM` — codegen rules |
| `~/Nowgetit/NowIGetIt/backend/pipeline/tts.py` | `synthesize_narration` |
| `~/Nowgetit/NowIGetIt/backend/pipeline/compose.py` | `mux_scene_audio`, `compose_final_video` |
| `~/Nowgetit/NowIGetIt/backend/pipeline/orchestrator.py` | TTS-first order |
| `~/Nowgetit/NowIGetIt/heating_demands/scene_3_transmission_humidity/` | Handcrafted twin of tutorial Cooling scene 3 |

Pipeline order: **Plan → TTS → Manim codegen → render → (optional VLM) → mux → compose**.

## Shared palette

```python
P_DEEP_DARK = "#0B0C10"
P_WHITE     = "#E0E6ED"
P_CYAN      = "#66FCF1"
P_TEAL      = "#45A29E"
P_ORANGE    = "#FFAAA5"
P_YELLOW    = "#FFE66D"
P_RED       = "#FF6B6B"
P_BLUE      = "#4D96FF"
# optional
P_GREEN     = "#CAFFBF"
```

Semantic hints: cyan accents / windows; yellow sun; red heat / transmission; blue humidity / cool air; teal structure / equation boxes.

## Scene class skeleton

```python
import numpy as np
from manim import *

from pathlib import Path as _Path
import sys as _sys

_TUTORIAL_ROOT = next(
    p for p in _Path(__file__).resolve().parents
    if (p / "manim_fonts.py").is_file()
)
if str(_TUTORIAL_ROOT) not in _sys.path:
    _sys.path.insert(0, str(_TUTORIAL_ROOT))
from manim_fonts import apply_body_font

P_DEEP_DARK = "#0B0C10"
P_WHITE = "#E0E6ED"
P_CYAN = "#66FCF1"
P_TEAL = "#45A29E"
P_ORANGE = "#FFAAA5"
P_YELLOW = "#FFE66D"
P_RED = "#FF6B6B"
P_BLUE = "#4D96FF"


def _build_house(center=ORIGIN):
    """Line-art house exterior (shared across beats)."""
    # … geometry → return dict of mobjects/points …
    ...


class Beat1_TopicName(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        apply_body_font()

        title = Text("Deutscher Titel", font_size=30, color=P_WHITE)
        title.to_edge(UP, buff=0.4)
        self.play(FadeIn(title))

        parts = _build_house(ORIGIN + DOWN * 0.3)
        self.play(Create(parts["house"]), run_time=1.5)

        # … GrowArrow / LaggedStart / equation box …

        self.wait(0.5)
```

## `manim.cfg` (Sideview)

```ini
[CLI]
quality = low_quality
preview = False
write_to_movie = True
format = mp4
media_dir = ./media
frame_rate = 15
pixel_height = 480
pixel_width = 854
```

## `generate_audio.py` stub

```python
"""Generate TTS audio for this topic using Now I Get It backend TTS."""

import sys
from pathlib import Path

NOWIGETIT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT))

from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent

NARRATIONS = {
    "beat_1_audio": (
        "English narration matching Beat1 visuals…"
    ),
}


def main():
    for name, text in NARRATIONS.items():
        out_path = BASE_DIR / f"{name}.mp3"
        result_path, skipped = synthesize_narration(text, out_path)
        print(name, "skipped" if skipped else result_path)


if __name__ == "__main__":
    main()
```

Requires NowIGetIt `.env` TTS keys (`tts_api_key` / OpenRouter). Gemini TTS may write `.wav` instead of `.mp3`.

## `build_full_video.py` stub

```python
import subprocess
import sys
from pathlib import Path

NOWIGETIT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
SEMIO = Path(__file__).resolve().parents[4]  # adjust to monorepo root
sys.path.insert(0, str(NOWIGETIT))

from backend.pipeline.compose import compose_final_video, mux_scene_audio

def main():
    base = Path(__file__).resolve().parent
    script = base / "scene_N.py"
    out = base / "rendered"
    out.mkdir(exist_ok=True)
    manim = SEMIO / ".venv" / "bin" / "manim"

    scenes = [
        ("Beat1_TopicName", base / "beat_1_audio.mp3"),
    ]
    clips = []
    for i, (cls, audio) in enumerate(scenes, 1):
        subprocess.run(
            [str(manim), "-qh", "--media_dir", str(out / "media"), str(script), cls],
            check=False,
        )
        mp4 = next((out / "media").rglob(f"{cls}.mp4"), None)
        audio = audio if audio.exists() else audio.with_suffix(".wav")
        muxed = out / f"beat_{i}_with_audio.mp4"
        path = mux_scene_audio(str(mp4), str(audio) if audio.exists() else None, muxed)
        clips.append(path or str(mp4))

    compose_final_video(clips, base / "Full_Topic_HQ.mp4")


if __name__ == "__main__":
    main()
```

Adjust `SEMIO` / `parents[N]` so `.venv/bin/manim` resolves. Prefer workspace `.venv` over system `manim`.

## Naming

| Kind | Pattern | Example |
|------|---------|---------|
| Topic folder | `N_snake_case` | `2_transmission_humidity` |
| Scene file | `scene_N.py` | `scene_3.py` |
| Beat class | `Beat{N}_{Pascal}` | `Beat1_TransmissionOpaque` |
| Audio stem | `beat_N_audio` | `beat_2_audio.mp3` |
| Muxed clip | `beat_N_with_audio.mp4` | |
| Final | `Full_<Topic>_….mp4` | `Full_Scene3_Transmission_Humidity.mp4` |

## Timing heuristics

- English VO ≈ 2.5 words/sec → estimate `duration_seconds`
- Map each narration clause to a `self.play(...)` with real motion
- If VO is longer than visuals, add labeled reveals — never pad with empty waits
- Always end Scene with exactly `self.wait(0.5)`

## Anti-patterns

- `MathTex` / LaTeX dependency
- Packing a whole chapter into one Scene when Sideview needs beat isolation
- German VO by default when on-screen is already German (double load) — keep VO English unless asked
- `sys.path` to `tutorial/energy` expecting `backend` (missing) — use NowIGetIt path
- Quality flags in `manim-sideview.commandLineArgs` — put quality in `manim.cfg`
- Mixing Heating `add_sound` master Scene with Cooling external mux in the same topic
