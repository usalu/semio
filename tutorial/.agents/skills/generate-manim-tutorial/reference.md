# Manim Tutorial — Reference

Companion to [SKILL.md](SKILL.md). Load only when generating or extending scenes.

## Canonical paths

### semio tutorial

| Path | Role |
|------|------|
| `tutorial/manim_fonts.py` | Body font (cached in `tutorial/.manim_font_cache` — see below), fixed type scale (`TITLE_FONT_SIZE`…`CAPTION_FONT_SIZE`), shared `BODY_LINE_SPACING` / `centered_body_text`, `apply_scene_style`, `scene_title`, `beat_subtitle` |
| `tutorial/manim_visuals.py` | Palette (single source, includes `P_DEEP_DARK`), pedagogy helpers, formula panel (`equation_row`/`formula_panel`/`highlight_param`), caption bar (`caption_bar`/`swap_caption`), narration timing (`narration_seconds`/`narration_text`/`subtitle_text`/`hold_for`) |
| `tutorial/energy/demand/Cooling/3_transmission_humidity/scene_3.py` | Best beat-split example |
| `tutorial/energy/demand/Cooling/3_transmission_humidity/generate_audio.py` | Imports `NARRATION` from `scene_3.py` → TTS |
| `tutorial/energy/demand/Cooling/3_transmission_humidity/build_full_video.py` | Render → mux → compose |
| `tutorial/intro/intro_scene.py` | Reusable NGS/IEK/LUH intro card (`NGSIntro`) |
| `tutorial/energy/demand/Heating/1_introduction/scene_1.py` | First Heating topic on the five-rule template, with German subtitles |
| `tutorial/energy/demand/Heating/{conduction,convection,final_calculation,internal_heat_gain,solar_heat_gain}/merged_scenes*.py` | Remaining legacy Heating topics — `SceneN` + `Full*Video`, not yet migrated |
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

## Palette — import, don't copy

```python
from manim_visuals import (
    P_DEEP_DARK,  # #0B0C10 — background, every scene
    P_WHITE,      # #E0E6ED
    P_CYAN,       # #66FCF1 — accents / windows
    P_TEAL,       # #45A29E — structure / equation boxes
    P_ORANGE,     # #FFAAA5
    P_YELLOW,     # #FFE66D — sun
    P_RED,        # #FF6B6B — heat / transmission
    P_BLUE,       # #4D96FF — humidity / cool air
    P_GREEN,      # #CAFFBF
)
```

`P_DEEP_DARK` lives in `manim_visuals.py` alongside the rest — every scene imports the same nine constants instead of re-declaring its own copy of the block (older scenes that still hardcode this block are the reason two of them drifted: one is missing `P_GREEN`, another invented an unshared `P_MUTED`).

## Type scale — the fixed sizes

```python
from manim_fonts import (
    TITLE_FONT_SIZE,     # 34 — 5.3 % of frame height — top-center chapter heading
    SUBTITLE_FONT_SIZE,  # 23 — 3.6 % — beat subtitle under the heading
    BODY_FONT_SIZE,       # 20 — 3.2 % — standard on-screen label
    LABEL_FONT_SIZE,      # 17 — 2.7 % — small callouts, unit tags, legend rows
    FORMULA_FONT_SIZE,    # 30 — 4.7 % — the dedicated formula panel
    CAPTION_FONT_SIZE,    # 25 — 4.0 % — the dedicated German subtitle bar
)
```

Every `Text(..., font_size=...)` in a scene uses one of these six — never a number typed by eye. `equation_row()` defaults to `FORMULA_FONT_SIZE` and `caption_bar()` defaults to `CAPTION_FONT_SIZE` if you don't pass one.

**Cold-start hang:** `resolve_serif_font()` asks `manimpango.list_fonts()` for the installed fonts, which asks fontconfig — and on a cold fontconfig cache (fresh devcontainer, fresh clone, right after installing a font) that first enumeration can hang for minutes with the process sitting nearly idle, not crashed. Every scene calls this at import time, so every scene paid that cost once. It's now cached to `tutorial/.manim_font_cache` (gitignored, machine-local) after the first successful resolution — reruns skip `manimpango` entirely. If a render seems to hang with near-zero CPU, check whether that file exists yet; if you must force re-resolution (e.g. a newly installed font should now win), delete it.

## The fixed bottom-of-frame layout

`formula_panel()` (`edge_buff=1.7`) and `caption_bar()` (`edge_buff=0.35`) are two separate, non-overlapping reserved zones, always in this order top-to-bottom: title → (scene content) → formula panel → subtitle. A beat may use either, both, or neither, but never repositions them and never puts anything else in either zone. Verify with a real render (`manim -ql`, pull a frame) whenever a beat combines both, the same way `reference.md`'s own example below was checked.

## Scene class skeleton

The full five-rule template — consistent styling, top-center heading, one dedicated formula panel, embedded narration driving the timing, and a German subtitle for every clause:

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

from manim_fonts import apply_scene_style, scene_title, play_scene_title, TITLE_RUN_TIME
from manim_visuals import (
    P_WHITE, P_CYAN, P_ORANGE, P_BLUE,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption,
    hold_for, subtitle_text,
)


def _build_house(center=ORIGIN):
    """Line-art house exterior (shared across beats)."""
    # … geometry → return dict of mobjects/points …
    ...


class Beat2_TransmissionFormula(Scene):
    # 📝 Ordered narration clauses — the single source of truth for the spoken
    # VO (generate_audio.py imports this), the German subtitle, and this
    # Scene's own timing. One clause per distinct visual moment.
    NARRATION = [
        ("intro", "Heat loss through a wall follows one formula.",
         "Wärmeverlust durch eine Wand folgt einer einzigen Formel."),
        ("formula", "Q dot T equals U times A times the temperature difference.",
         "Q-Punkt-T ist gleich U mal A mal der Temperaturdifferenz."),
        ("u", "U is the wall's insulation quality — lower is better.",
         "U ist die Dämmqualität der Wand — niedriger ist besser."),
        ("a", "A is the wall's area — bigger walls lose more heat.",
         "A ist die Wandfläche — größere Wände verlieren mehr Wärme."),
        ("dt", "And delta T is how much colder it is outside than in.",
         "Und Delta-T ist, wie viel kälter es draußen ist als drinnen."),
    ]

    def construct(self):
        apply_scene_style(self)  # same background + font, every scene — rule 1

        title = scene_title("Transmissionswärmeverlust")  # top-center, fixed size — rule 2
        play_scene_title(self, title)
        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))  # rule 5
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + 0.3)

        row, items = equation_row([
            ("qt", "Q̇_T", P_WHITE), (None, "=", P_WHITE),
            ("u", "U", P_ORANGE), (None, "·", P_WHITE),
            ("a", "A", P_CYAN), (None, "·", P_WHITE),
            ("dt", "ΔT_eq", P_BLUE),
        ])
        row, box = formula_panel(row)  # one fixed formula slot, boxed — rule 3
        self.play(Create(row), Create(box), run_time=1.2)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        hold_for(self, self.NARRATION, "formula", used=1.2 + 0.35)

        for key, color in (("u", P_ORANGE), ("a", P_CYAN), ("dt", P_BLUE)):
            ring = highlight_param(items, key, color=color)
            self.play(Create(ring), run_time=0.5)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.5 + 0.35)  # exactly this clause's need — rule 4
            self.play(FadeOut(ring), run_time=0.3)

        self.play(FadeOut(caption), run_time=0.3)
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

Narration text is **not** re-typed here — it's imported from the beat classes in `scene_N.py`:

```python
"""Generate TTS audio for this topic. Narration text lives in scene_N.py — this
file only reads it (via each Beat's NARRATION) and sends it to TTS."""

import sys
from pathlib import Path

NOWIGETIT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT))
from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(BASE_DIR))

# Importing scene_N runs its own _TUTORIAL_ROOT sys.path bootstrap, which is
# what makes the next import (manim_visuals) resolve.
from scene_N import Beat1_TopicName, Beat2_TopicName  # noqa: E402
from manim_visuals import narration_text  # noqa: E402

BEATS = [Beat1_TopicName, Beat2_TopicName]


def main():
    for i, cls in enumerate(BEATS, start=1):
        text = narration_text(cls.NARRATION)
        out_path = BASE_DIR / f"beat_{i}_audio.mp3"
        result_path, skipped = synthesize_narration(text, out_path)
        print(cls.__name__, "skipped" if skipped else result_path)


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
SEMIO = Path(__file__).resolve().parents[5]  # tutorial/energy/demand/<Cat>/<N_topic>/this_file.py → semio/
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
| Topic folder | `N_snake_case` | `3_transmission_humidity` |
| Scene file | `scene_N.py` | `scene_3.py` |
| Beat class | `Beat{N}_{Pascal}` | `Beat1_TransmissionOpaque` |
| Narration clause key | short `snake_case` slug | `"u"`, `"formula"`, `"outro"` |
| Audio stem | `beat_N_audio` | `beat_2_audio.mp3` |
| Muxed clip | `beat_N_with_audio.mp4` | |
| Final | `Full_<Topic>_….mp4` | `Full_Scene3_Transmission_Humidity.mp4` |

## Timing — computed, not guessed

- `narration_seconds(NARRATION, key)` — words in that clause's English ÷ `NARRATION_WPS` (2.5 words/sec English VO). `key=None` sums the whole beat.
- `hold_for(self, NARRATION, key, used=<run_time already spent on this clause's animation>)` — waits the remainder and returns it. Use this instead of any hand-typed `self.wait(N)`. If a clause shows a subtitle via `swap_caption`, add that cross-fade's `run_time` to `used` too (see the skeleton above) — the caption's own entrance is part of what "already spent" means.
- `narration_text(NARRATION, key=None)` — joins clauses' English back into one string for TTS; this is what `generate_audio.py` imports and sends to `synthesize_narration`.
- `subtitle_text(NARRATION, key)` — the one clause's German subtitle; feed it straight to `caption_bar()` / `swap_caption()`.
- If a clause's animation already runs longer than its narration needs, `hold_for` still returns at least `min_wait` (default 0.3s) — never a zero or negative wait.
- Always end the Scene with exactly `self.wait(0.5)` after the last `hold_for` (and after fading out the final caption) — a fixed breath before the cut, independent of narration length.

## Anti-patterns

- `Write()` on anything other than the title — small labels animated with `Write()` after another mobject's `Create()` earlier in the same beat have rendered with corrupted, ghosted trailing characters (reproduced and fixed in `Heating/1_introduction/scene_1.py`). Use `FadeIn()` for subtitles, callouts, and every other label.
- A simultaneous `FadeOut(old_text), FadeIn(new_text)` cross-fade between two *different* strings — the two render overlapping mid-transition, reading as doubled/ghosted text. `swap_caption()` already does this correctly (sequential fade-out-then-in); don't hand-roll a simultaneous version elsewhere.
- A directional arrow (`CurvedArrow`, `GrowArrow`) standing in for two things merging or exchanging state — a gradient `Rectangle` (`fill_color=[color_a, color_b]`) widening across the shared boundary reads more clearly and is the established fix for the Beat1 warm/cold transition.
- `MathTex` / LaTeX dependency — including the indirect route: `DecimalNumber` defaults to `mob_class=MathTex`, so `Axes(..., include_numbers=True)` / `axes.add_coordinates()` drag LaTeX in. Hand-place axis labels as `Text` at `axes.c2p(...)`.
- A raw equation `Text` sliced by character index to highlight a variable (`eq_text[5]`) — use `equation_row()`'s named fragments instead
- A second, differently-positioned formula box in the same beat — there is exactly one `formula_panel()` slot
- A second, differently-positioned subtitle — there is exactly one `caption_bar()` slot, always the bottom edge, always below the formula panel
- English text in `caption_bar()` — subtitles are always German, even when explaining an English-loanword term
- A subtitle line so long it exceeds `CAPTION_MAX_WIDTH` — `caption_bar` keeps `CAPTION_FONT_SIZE` and **word-wraps** (plus hand `\\n`) via `centered_body_text(..., max_width=...)`; never shrink. Lines are center-aligned with a fixed vertical gap. Always go through `body_text` / `apply_body_font` so `disable_ligatures=True` is on — Pango ligatures on Georgia make German clusters look broken
- `Write()` on formula/`Text` labels — use `FadeIn()`; `Write()` leaves spaced/ghosted glyphs mid- and post-animation
- Copy-pasting the palette hex block into a scene file instead of `from manim_visuals import P_DEEP_DARK, …`
- A hand-typed `self.wait(8.96)` "narration budget" magic number instead of `hold_for(...)`
- Narration or subtitle text that exists only in `generate_audio.py`, with no `NARRATION` attribute on the `Beat*` class it belongs to
- Packing a whole chapter into one Scene when Sideview needs beat isolation
- German VO by default when on-screen is already German (double load) — keep VO English unless asked
- `sys.path` to `tutorial/energy` expecting `backend` (missing) — use NowIGetIt path
- Quality flags in `manim-sideview.commandLineArgs` — put quality in `manim.cfg`
- Mixing Heating `add_sound` master Scene with Cooling external mux in the same topic (see `2_internal_gains/merged_scenes.py`)
