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

**Every scene follows the same standardized template — five non-negotiable rules, enforced by shared helpers in `manim_fonts.py` / `manim_visuals.py`, not left to convention:**

1. **Consistent styling.** First line of every `construct()` is `apply_scene_style(self)` — one call, dark background + resolved body font. Every `font_size=` comes from the fixed type scale in `manim_fonts.py` (`TITLE_FONT_SIZE` 34 / `SUBTITLE_FONT_SIZE` 23 / `BODY_FONT_SIZE` 20 / `LABEL_FONT_SIZE` 17 / `FORMULA_FONT_SIZE` 30 / `CAPTION_FONT_SIZE` 25) — never a bespoke number picked by eye. Those values are set against the 8-unit frame height (a Text cap height is ≈ `0.0126 × font_size` units): the earlier scale put labels at 2.2 % and body at 2.5 % of frame height and read as too small in finished videos, so anything a viewer reads while a diagram moves is now ≈ 3 % or more. Raising the scale widens every label, so re-run the layout check afterwards. Palette colors are **imported** from `manim_visuals.py` (`P_DEEP_DARK`, `P_WHITE`, …) — never copy-pasted into the scene file.
2. **Clear heading, top-center.** `scene_title(text)` + `play_scene_title(self, title)` from `manim_fonts.py` — same face, size, and position every time. Animate (`Write`) only on the first beat of a topic; later beats in the same topic `self.add(title)` the identical text instantly (no repeated intro).
3. **Dedicated formula section.** Any beat that shows a formula uses `equation_row()` → `formula_panel()` from `manim_visuals.py` — one fixed bottom-center boxed slot, Text-only fragments. Ring the specific parameter being explained with `highlight_param(items, key)`, in the order the narration names it. Never `MathTex`/`Tex`, never a raw equation string sliced by character index.
4. **Narration embedded for timing.** Every `Beat*(Scene)` class declares `NARRATION: list[tuple[str, str, str]]` — ordered `(section_key, narration_en, subtitle_de)` clauses — as a class attribute. `hold_for(self, self.NARRATION, key, used=...)` from `manim_visuals.py` computes exactly how long that section's hold needs from the embedded English text, replacing hand-typed "wait this many seconds" guesses. `generate_audio.py` imports the same `NARRATION` (via `narration_text()`) for TTS — one source of text, not two files drifting apart.
5. **German on-screen subtitles.** Every clause's German translation lives right there in `NARRATION` (the third tuple element) and is shown via `caption_bar(text_de)` / `swap_caption(self, old, text_de)` from `manim_visuals.py` — one fixed bottom-edge boxed slot, always German, never the spoken English, always `CAPTION_FONT_SIZE` (no auto-shrink), always center-aligned lines with the shared vertical gap from `centered_body_text`. It sits in its own reserved zone below `formula_panel()`, so the two fixed slots never overlap; nothing else may be placed in either zone.

**Canonical style sources (read when unsure):**
- `tutorial/energy/demand/Cooling/3_transmission_humidity/scene_3.py`
- `tutorial/energy/demand/Cooling/3_transmission_humidity/generate_audio.py`
- `tutorial/energy/demand/Cooling/3_transmission_humidity/build_full_video.py`
- `tutorial/manim_fonts.py` / `tutorial/manim_visuals.py` — the template's actual implementation, not just its description
- Now I Get It prompts: `~/Nowgetit/NowIGetIt/backend/pipeline/{planner,scene_generator,tts,compose}.py`

For palette, templates, and file stubs see [reference.md](reference.md).

## When Invoked

1. Parse the user's description into a **scene plan** (beats), each with its `NARRATION` clauses (English VO + German subtitle per clause).
2. Create or extend a topic folder under `tutorial/energy/demand/{Heating|Cooling}/`.
3. Write Manim code, narrations, and pipeline scripts using the five rules above.
4. Prefer editing existing scene files when extending a topic; otherwise add a new numbered topic folder.

Do **not** invent a new visual language, a second formula-box style, or a second title mechanism. Reuse the shared palette, house helpers, and beat = Scene pattern.

## Workflow

Copy and track:

```
Progress:
- [ ] 1. Plan beats from description
- [ ] 2. Choose / create topic folder
- [ ] 3. Write NARRATION clauses per beat — (key, narration_en, subtitle_de), drives TTS, on-screen timing, and subtitles
- [ ] 4. Write scene_N.py (or extend) with apply_scene_style / scene_title / formula_panel / caption_bar / hold_for
- [ ] 5. Write / update generate_audio.py to import NARRATION from scene_N.py
- [ ] 6. Write / update build_full_video.py scene list
- [ ] 7. Add manim.cfg if missing (Sideview)
- [ ] 8. Smoke-check: manim -ql <file> <BeatClass>
```

### 1. Plan (from description)

Produce an internal plan before coding:

| Field | Rule |
|-------|------|
| Topic slug | `N_snake_case` under Heating or Cooling |
| Through-line | One recurring metaphor (usually line-art house) |
| Beats | 3–6; each = one clear idea + real motion |
| Language | On-screen titles/labels/subtitles: German; Narration (VO/TTS): English (unless user asks DE VO) |
| Narration | Ordered `(section_key, narration_en, subtitle_de)` clauses per beat — one clause per distinct visual moment, not one paragraph |
| Subtitles | Every clause's `subtitle_de` is a natural German rendering of `narration_en`, not a stiff word-for-word gloss — short enough for `caption_bar` (≤ ~2 short lines) |
| Timing | `hold_for()` reads seconds straight from the embedded clauses; end each Scene with `self.wait(0.5)` |

Each beat needs:
- `class_name`: `Beat{N}_{PascalTopic}`
- `title_de`: short German title
- `visual_action`: motion verb (Create / GrowArrow / Transform / LaggedStart…)
- `NARRATION`: ordered `[(section_key, text_en, text_de), …]` — `text_en` is spoken VO only (long explanation lives here, never on screen), `text_de` is what `caption_bar` shows while it plays
- `on_screen_labels`: ≤3-word German labels the narration names

Visual devices (pick one per beat): `house_section` | `particle_flow` | `equation_reveal` | `comparison_split` | `before_after` | `axes_graph` | `annotated_diagram` | `labeled_box_flow` | `morph_transform`

### 2. Topic folder layout

```
tutorial/energy/demand/Cooling/<N_topic>/
  scene_N.py              # shared palette + helpers + Beat* Scene classes, each carrying its own NARRATION
  generate_audio.py       # imports NARRATION from scene_N.py → beat_N_audio.mp3|.wav
  build_full_video.py     # manim -qh → mux → compose
  manim.cfg               # low quality for Manim Sideview
  beat_N_audio.mp3        # generated
  rendered/               # HQ outputs (gitignored if project does so)
```

Heating legacy may still use `merged_scenes.py` + `Full*Video` + `add_sound`. **New work uses the Cooling trio above.** Do not mix both patterns in one topic — `Cooling/2_internal_gains/merged_scenes.py` is a known example of this drift (legacy `SceneN` classes and a monolithic `FullHiddenHeatVideo` sitting inside a Cooling-numbered folder, pipeline-generated and never migrated). Do not copy its pattern for new work; migrating it to the four-rule template is a separate future pass, not something to fix incidentally while touching something else.

### 3. Manim hard rules (Community Edition)

1. `from manim import *` — CE only (`Create`, not `ShowCreation`).
2. Plain `Scene` only — no `MovingCameraScene` / `ThreeDScene`.
3. **Always `Text(...)`** for titles, labels, equations — never `MathTex` / `Tex` / `TexText`. No exceptions; every topic folder is now on `equation_row()`. The trap that remains is indirect: CE's `DecimalNumber` defaults to `mob_class=MathTex`, so `Axes(..., include_numbers=True)` and `axes.add_coordinates()` pull LaTeX back in — hand-place tick labels as `Text` positioned with `axes.c2p(...)` instead (see `6_lueftungssysteme/scene_6.py`'s `Beat5_Nachtlueftung`).
4. Palette: `from manim_visuals import P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN` — import it, never copy-paste the hex block into the scene file (copies drift: some existing files are missing `P_GREEN`, one invented an unshared `P_MUTED`).
5. First line of every `construct()`: `apply_scene_style(self)`. Do not set `self.camera.background_color` and call `apply_body_font()` as two separate hand-typed steps.
6. Title: `scene_title(text)` + `play_scene_title(self, title)` — top-center, `TITLE_FONT_SIZE`, `TITLE_EDGE_BUFF`. Animate on the topic's first beat only; later beats `self.add(title)` the same text so it's present without a repeated intro. Never a raw `Text(...).to_edge(UP)` bypass, never `to_corner`.
7. **`Write()` is for the title only.** For any other `Text` — subtitles, callout labels, door/wall/material labels — use `FadeIn()`. `Write()` on a small label that follows another mobject's `Create()`/animation earlier in the same beat has produced corrupted, ghosted trailing characters (reproduced in `Heating/1_introduction/scene_1.py`'s party-scene labels: `Write()` left "warm)" and "kalt)" doubled/garbled even at rest, while `FadeIn()` on the identical `Text` rendered clean). The one proven-safe `Write()` call is the title's, as the scene's first animation with nothing preceding it — do not generalize from it.
8. All other `font_size=` values come from `manim_fonts`: `SUBTITLE_FONT_SIZE` (20) for beat subtitles, `BODY_FONT_SIZE` (16) for standard labels, `LABEL_FONT_SIZE` (14) for small callouts, `FORMULA_FONT_SIZE` (26) for the formula panel. Do not pick a font size by eye.
9. Layout: build near `ORIGIN` with `arrange`/`next_to`, then place the group. No overlapping text/arrows. At most one dense formula box visible at a time — and it is always the one from `formula_panel()`.
10. Prefer real motion (`Create`, `GrowArrow`, `LaggedStart`, `Transform`, `ValueTracker`) over FadeIn-and-wait posters — but see rule 7 for `Text` specifically.
11. Continuity: later beats `self.add(...)` prior visual state when the story continues.
12. Shared helpers (`_build_house`, `_build_house_section`) at file top; return dicts of mobjects/geometry.
13. Formulas: `equation_row()` → `formula_panel()` from `manim_visuals.py`. Highlight a parameter with `highlight_param(items, key)` while `hold_for` waits out that clause's narration — never character-index slicing (`eq_text[5]`, fragile the moment the string changes), never a second, differently-positioned formula box.
14. Subtitles: `caption_bar(text_de)` from `manim_visuals.py` for the first clause's German subtitle, `swap_caption(self, old, text_de)` to cross-fade to each later clause's. Always German, always this one fixed bottom-edge slot — never a second caption position, never English on screen.
15. No filler: no breathing scale loops, no timing-arithmetic comments, no hand-typed "hold N seconds" magic numbers — `hold_for()` computes that from `NARRATION`.
16. Sparse on-screen text; VO carries the lecture — the subtitle repeats it in German, it does not add new information the VO didn't say.
17. **Meaningful physics:** Strahlung = wavy plumes/rays (`radiation_waves` / `solar_wave_ray`); Konvektion = air-stream ribbons (`convection_stream`); Atmung = fühlbar + latent (`respiration_parts`). Two adjacent zones (e.g. warm vs. cold air) merging or exchanging state read better as a shared boundary blending color (a gradient `Rectangle` widening across the seam) than as a directional arrow hopping the gap.
18. **Formula morph:** never FadeIn a raw equation alone — `ReplacementTransform` a physical object into its symbol (walls→`U`, air volume→`n` / `q_v,R`) via `symbol_token` from `tutorial/manim_visuals.py`, then feed it into an `equation_row`.
19. **Watt anchors:** every power number gets an everyday device compare (`watt_anchor`, e.g. laptop / Glühbirne / Toaster) so absurd loads are obvious.

### 4. Narration, timing & subtitles (`NARRATION`)

This is the mechanism that makes "how many seconds does this section need" a calculation instead of a guess, and keeps the German subtitle from ever drifting away from what's actually said:

- Every `Beat*(Scene)` class declares `NARRATION: list[tuple[str, str, str]]` as its first class attribute — ordered `(section_key, narration_en, subtitle_de)` clauses. One clause per distinct visual moment: an intro sentence, the formula statement, one clause per parameter you'll ring, an outro sentence. Keep a clause to what's said while *one* animation plays or *one* thing is highlighted — not a whole paragraph, and short enough to read comfortably as a subtitle.
- Compute holds from it, never by hand: `hold_for(self, self.NARRATION, "u", used=<run_time already spent animating this clause's visuals>)`. It waits `max(min_wait, narration_seconds(clause) - used)` and returns the seconds it waited.
- Show the matching German subtitle for the same clause: `caption_bar(subtitle_text(self.NARRATION, "u"))` for the beat's first clause, `swap_caption(self, current_caption, subtitle_text(self.NARRATION, "a"))` for every clause after — one bottom-edge slot, cross-fading from clause to clause.
- `narration_text(NARRATION)` joins every clause's English back into one string — what `generate_audio.py` sends to TTS for the whole beat. `narration_text(NARRATION, key="u")` isolates one clause if per-clause audio splicing is ever needed.
- `generate_audio.py` **imports** `NARRATION` from the beat classes in `scene_N.py` — it must not re-type the narration or subtitle text a second time. That single import is what keeps spoken text, subtitle text, and on-screen timing from drifting apart from each other.

See the full worked example in [reference.md](reference.md).

### 5. Audio (`generate_audio.py`)

- Import each `Beat*` class from `scene_N.py`; call `narration_text(cls.NARRATION)` to get its TTS string — do not hand-type narration a second time in this file.
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

### 6. Full video (`build_full_video.py`)

For each beat:

1. `manim -qh --media_dir rendered/media scene_N.py BeatClass`
2. Resolve audio `.mp3` or `.wav`
3. `mux_scene_audio(video, audio, rendered/beat_N_with_audio.mp4)` — keep full narration (no `-shortest` as primary)
4. `compose_final_video(clips, Full_<Topic>.mp4)`

Import compose helpers from NowIGetIt the same way as TTS. Scene list must match class names in `scene_N.py`.

### 7. Sideview

- Ensure `manim.cfg` with low quality exists in the topic folder (see [reference.md](reference.md)).
- User opens `scene_N.py` → Manim Sideview rotation icon → pick `Beat*` class.

## Output checklist

Before finishing:

- [ ] Every beat is its own `Beat*_…(Scene)` class with a `NARRATION` class attribute of `(key, narration_en, subtitle_de)` triples
- [ ] German titles/labels/subtitles; English narration (unless asked otherwise) — the narration and subtitle text both live in `scene_N.py`, not only in `generate_audio.py`
- [ ] `apply_scene_style(self)` is the first line of every `construct()`
- [ ] Heading via `scene_title()` / `play_scene_title()` — top-center, animated once per topic, `self.add()` on later beats
- [ ] Any formula uses `equation_row()` + `formula_panel()`; parameters ringed with `highlight_param()` in the order the narration names them
- [ ] Every clause shows its German subtitle via `caption_bar()` / `swap_caption()` — checked by eye (render a still or a low-quality pass) that it never overlaps the title, the formula panel, or any other visual
- [ ] Every hold uses `hold_for(...)` — no hand-typed wait constant
- [ ] `Text` only (no TeX); palette imported from `manim_visuals`, not copy-pasted
- [ ] `generate_audio.py` imports `NARRATION` from `scene_N.py`
- [ ] Class names in `build_full_video.py` match `scene_N.py`
- [ ] Final `self.wait(0.5)` on each Scene
- [ ] No new pattern invented when Cooling already covers it

## Heating vs Cooling

| | Cooling (default for new work) | Heating (legacy) |
|--|-------------------------------|------------------|
| Scenes | `scene_N.py` + `Beat*` classes, each with `NARRATION` | Often `merged_scenes.py` + `SceneN` |
| Audio | External mux via `build_full_video.py`, text imported from `scene_N.py` | Often `Full*Video` + `add_sound`, narration untracked |
| When | Always for new Cooling topics; preferred for new Heating too | Only when extending an existing Heating chapter in-place |

`Cooling/2_internal_gains/merged_scenes.py` currently follows the Heating column despite living under `Cooling/` — a known gap, not a pattern to copy.
