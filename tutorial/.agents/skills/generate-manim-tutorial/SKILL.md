---
name: generate-manim-tutorial
description: >-
  Generate educational Manim Community Edition tutorial animations (beat-based
  Scene classes, German on-screen text, English TTS narrations, generate_audio.py,
  build_full_video.py) for building-energy content under tutorial/energy/demand.
  Use when the user describes a new tutorial scene/beat, asks for Manim code,
  audio/TTS, Sideview-ready scenes, heating/cooling animations, or to extend
  Cooling/Heating tutorial folders. Distills the Now I Get It pipeline patterns.
---

# Generate Manim Tutorial

Turn a user description into a Cooling-style Manim topic folder: scenes + narrations + audio + compose scripts.

**Canonical style sources (read when unsure):**
- `tutorial/energy/demand/Cooling/2_transmission_humidity/scene_3.py`
- `tutorial/energy/demand/Cooling/2_transmission_humidity/generate_audio.py`
- `tutorial/energy/demand/Cooling/2_transmission_humidity/build_full_video.py`
- Now I Get It prompts: `~/Nowgetit/NowIGetIt/backend/pipeline/{planner,scene_generator,tts,compose}.py`

For palette, templates, and file stubs see [reference.md](reference.md).

## When Invoked

1. Parse the user’s description into a **scene plan** (beats).
2. Create or extend a topic folder under `tutorial/energy/demand/{Heating|Cooling}/`.
3. Write Manim code, narrations, and pipeline scripts.
4. Prefer editing existing scene files when extending a topic; otherwise add a new numbered topic folder.

Do **not** invent a new visual language. Reuse the shared palette, house helpers, and beat = Scene pattern.

## Workflow

Copy and track:

```
Progress:
- [ ] 1. Plan beats from description
- [ ] 2. Choose / create topic folder
- [ ] 3. Write scene_N.py (or extend)
- [ ] 4. Write / update generate_audio.py NARRATIONS
- [ ] 5. Write / update build_full_video.py scene list
- [ ] 6. Add manim.cfg if missing (Sideview)
- [ ] 7. Smoke-check: manim -ql <file> <BeatClass>
```

### 1. Plan (from description)

Produce an internal plan before coding:

| Field | Rule |
|-------|------|
| Topic slug | `N_snake_case` under Heating or Cooling |
| Through-line | One recurring metaphor (usually line-art house) |
| Beats | 3–6; each = one clear idea + real motion |
| Language | On-screen: German titles/labels; Narration: English (unless user asks DE VO) |
| Timing | ~2.5 English words/sec; animation runtime ≈ TTS ±0.5s; end each Scene with `self.wait(0.5)` |

Each beat needs:
- `class_name`: `Beat{N}_{PascalTopic}`
- `title_de`: short German title
- `visual_action`: motion verb (Create / GrowArrow / Transform / LaggedStart…)
- `narration_en`: spoken VO only (long explanation stays here, not on screen)
- `on_screen_labels`: ≤3-word German labels the narration names

Visual devices (pick one per beat): `house_section` | `particle_flow` | `equation_reveal` | `comparison_split` | `before_after` | `axes_graph` | `annotated_diagram` | `labeled_box_flow` | `morph_transform`

### 2. Topic folder layout

```
tutorial/energy/demand/Cooling/<N_topic>/
  scene_N.py              # shared palette + helpers + Beat* Scene classes
  generate_audio.py       # NARRATIONS → beat_N_audio.mp3|.wav
  build_full_video.py     # manim -qh → mux → compose
  manim.cfg               # low quality for Manim Sideview
  beat_N_audio.mp3        # generated
  rendered/               # HQ outputs (gitignored if project does so)
```

Heating legacy may still use `merged_scenes.py` + `Full*Video` + `add_sound`. **New work uses the Cooling trio above.** Do not mix both patterns in one topic.

### 3. Manim hard rules (Community Edition)

1. `from manim import *` — CE only (`Create`, not `ShowCreation`).
2. Plain `Scene` only — no `MovingCameraScene` / `ThreeDScene`.
3. **Always `Text(...)`** for titles, labels, equations — never `MathTex` / `Tex` / `TexText`.
4. Shared module palette (`P_DEEP_DARK`, …) — see [reference.md](reference.md).
5. Every Scene: `self.camera.background_color = P_DEEP_DARK` and `Text.set_default(font="Serif")`.
6. Title: `Text(...).to_edge(UP, buff=0.4)` + `FadeIn`. Side labels: `next_to` / `move_to` — do not `to_edge(UP)` then `shift(LEFT/RIGHT)`.
7. Layout: build near `ORIGIN` with `arrange`/`next_to`, then place the group. No overlapping text/arrows. At most one dense formula box visible at a time.
8. Prefer real motion (`Create`, `GrowArrow`, `LaggedStart`, `Transform`, `ValueTracker`) over FadeIn-and-wait posters.
9. Continuity: later beats `self.add(...)` prior visual state when the story continues.
10. Shared helpers (`_build_house`, `_build_house_section`) at file top; return dicts of mobjects/geometry.
11. Equations: `Text("Q̇_T = …")` + `SurroundingRectangle(..., color=P_TEAL)`. Highlight variables with rectangles on character ranges when teaching symbols.
12. No filler: no breathing scale loops, no timing-arithmetic comments, no trailing waits beyond the final `0.5`.
13. Sparse on-screen text; VO carries the lecture.

### 4. Audio (`generate_audio.py`)

- Key narrations as `"beat_N_audio"` → English (or requested language) strings.
- Write `{stem}.mp3` (accept `.wav` from Gemini TTS) next to the scene file.
- Prefer Now I Get It TTS when available:

```python
# Point at NowIGetIt so backend.pipeline.tts resolves
NOWIGETIT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT))
from backend.pipeline.tts import synthesize_narration
```

- If NowIGetIt is unavailable, use OpenAI-compatible `/audio/speech` with the same signature, or `gTTS` from `tutorial` deps — still write `beat_N_audio.mp3`.
- Do **not** rely on broken `sys.path` to `tutorial/energy` (there is no `backend` there).

### 5. Full video (`build_full_video.py`)

For each beat:

1. `manim -qh --media_dir rendered/media scene_N.py BeatClass`
2. Resolve audio `.mp3` or `.wav`
3. `mux_scene_audio(video, audio, rendered/beat_N_with_audio.mp4)` — keep full narration (no `-shortest` as primary)
4. `compose_final_video(clips, Full_<Topic>.mp4)`

Import compose helpers from NowIGetIt the same way as TTS. Scene list must match class names in `scene_N.py`.

### 6. Sideview

- Ensure `manim.cfg` with low quality exists in the topic folder (see [reference.md](reference.md)).
- User opens `scene_N.py` → Manim Sideview rotation icon → pick `Beat*` class.

## Output checklist

Before finishing:

- [ ] Every beat is its own `Beat*_…(Scene)` class
- [ ] German titles; English `NARRATIONS` (unless asked otherwise)
- [ ] Palette + Serif + dark background
- [ ] `Text` only (no TeX)
- [ ] `generate_audio.py` + `build_full_video.py` updated
- [ ] Class names in build script match scene file
- [ ] Final `self.wait(0.5)` on each Scene
- [ ] No new pattern invented when Cooling already covers it

## Heating vs Cooling

| | Cooling (default for new work) | Heating (legacy) |
|--|-------------------------------|------------------|
| Scenes | `scene_N.py` + `Beat*` classes | `merged_scenes.py` + `SceneN` |
| Audio | External mux via `build_full_video.py` | Often `Full*Video` + `add_sound` |
| When | Always for new Cooling topics; preferred for new Heating too | Only when extending an existing Heating chapter in-place |
