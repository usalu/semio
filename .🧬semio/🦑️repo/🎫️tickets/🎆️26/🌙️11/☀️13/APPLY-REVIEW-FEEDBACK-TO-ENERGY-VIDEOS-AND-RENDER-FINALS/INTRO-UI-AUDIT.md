# UI Introduction Audit (read-only)

Scope: owner remark in `FEEDBACK.md` (this ticket, line 9): "the UI introduction is not using proper frames but ad hoc placeholders." Read-only audit, no source or media changed.

## Short answer

- There is no file named `intro`, `ui-introduction`, `walkthrough`, `tour` or `onboarding` that renders a UI introduction video. The only `intro` directory is an empty stub (see A3).
- The "UI" that every energy video shows is the presentation wrapper `🪟presentation`: the lesson is shown inside the real semio UI chrome (navbar, footer, five-window canvas, zoom in/out, cursor, buttons). The zoom-in, zoom-out and cursor sequence is the closest candidate for the "UI introduction" the owner means. Every energy README states it: "The navbar and footer stay visible while its lesson window zooms in from the five-window canvas" (e.g. `🌬️ventilation/🎬️clip/README.md:3`).
- Chrome frames (navbar, footer, cards, windows) are real browser captures (Playwright screenshots of the shared React UI). The interactive controls and the cursor are not: they are redrawn with Manim primitives (`Rectangle`, `Text`, `SVGMobject`, `Polygon`). Those redrawn parts are the ad hoc placeholders, and they are the most likely source of the owner's complaint. This is an inference from code; no rendered frame was inspected.

## A1. Where the UI sequence is produced

| Role | File | Lines | Notes |
|---|---|---|---|
| Lesson wrapper, entry of the presentation | `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🐍️.py` | 1-26 | Imports browser, geometry, typography, interaction, hierarchy, profile, export. |
| Overview frame (`t=0` of first chapter) | same | 123-126 | `render_overview` returns the first chapter's focus frame. |
| Zoom transition frames | same | 129-139 | `_transition(..., phase="focus"|"restore")` renders `count` frames through `_hierarchy_render.frame_batch`. |
| Still frames used for the zoom | same | 221-230 | `first.png` / `last.png` are the first and last frame of the authored chapter; they become the zoom's media. |
| Composition of zoom-in, body, zoom-out | same | 236-253 | ffmpeg `concat=n=3` labelled `[intro]`, `[body]`, `[outro]`. The label `intro` here means the zoom-in transition, not an institutional intro. |
| Whole-lesson wrapper | same | 268-301 | `render_presentation`; chapters go through `_render_chapter`, lessons through `📚️collection.compose_lesson`. |
| Browser capture driver | `🪟presentation/🌐️browser/🐍️.py` | 44-87 | `_call` runs `repo tools exec --light node 🟨️render.mjs` (line 57). Frames whose size is off by at most 1 px are resized with LANCZOS (lines 71-78). |
| Real capture | `🪟presentation/🌐️browser/🟨️render.mjs` | 97-126, 207 | Launches Chromium through the framework Playwright adapter, `page.goto("http://presentation.local/")` (line 119), `page.screenshot({path, animations:"allow"})` (line 207). The bundle is built from the shared React components (lines 56-64, 69). |
| Layout and hit-geometry layer | `🪟presentation/🥞️hierarchy/🖌️render/🐍️.py` | 18-22, 56-85, 213-277 | `_probes` measures real DOM control rectangles from browser metadata (lines 72-80). `frame_batch` (213-277) requests browser frames and writes them. Line 250 passes the authored chapter's first frame as the body image. |
| Interaction layer (controls, cursor) | `🪟presentation/🖱️interaction/🖌️render/🐍️.py` | 18-35, 38-56, 59-70 | See A2. |
| Cursor geometry | `🪟presentation/🖱️interaction/🎬️scene/🐍️.py` | 19-27 | `cursor_shape` builds a Manim `Polygon` per path from `_core.cursor_outline(kind)`. |
| Primitive helpers | `🪟presentation/🐍️.py` | 100-112 | `_text` (typography `_native_text`) and `_box` (Manim `Rectangle`). |

Measured geometry exists, so the frame data is not missing: `_probes` (hierarchy render, lines 56-85) already reads real card and chrome rectangles from the browser and then discards the browser pixels for controls (see A2).

## A2. Quoted placeholder code

`🪟presentation/🖱️interaction/🖌️render/🐍️.py`, lines 38-56 (controls are redrawn in Manim, not captured):

```
def _button(rect, style, *, icon=None, label=None, text_height=11.2):
    """🔘 Draw a real canonical control cell and its state-dependent ink."""
    ...
    frame = p._box(rect, color=style["stroke"], fill=style["fill"], opacity=style["fill_opacity"], stroke=style["stroke_width"])
    parts = [frame]
    if icon:
        glyph = _icon(icon).copy()
        ...
    if label:
        parts.append(p._text(label, rect["x"] + p._geometry.geometry_metrics()["spacing"], rect["y"] + rect["height"] / 2, font_size=text_height, line_height=text_height, color=style["text"]))
```

`_box` (`🪟presentation/🐍️.py`, lines 106-112) is a plain `Rectangle` with `fill_color`/`stroke_width`. `_text` (lines 100-103) is a Manim text line. The docstring says "Draw a real canonical control cell", but the pixels are Manim geometry, not the DOM.

Cursor, `🖱️interaction/🎬️scene/🐍️.py`, lines 19-27: the cursor is a stack of hand-built `Polygon`s taken from path data (`_core.cursor_outline`), rendered by a separate Manim `Camera` in `cursor_layer` (`🖌️render/🐍️.py`, lines 59-70).

Icons, `🖌️render/🐍️.py`, lines 18-35: `SVGMobject` of the authored icon, plus a `Rectangle` viewport. The icon is drawn in Manim, not captured from the UI.

## A3. The institutional intro and its leftovers

Path (Unicode names): `🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/👋️intro/`

- `🏰️institution/📕️.pdf` (648,725 bytes). An institution PDF; nothing in the authoring tree reads it (grep for `📕`, `institution`, `🏰` in `🏗️authoring` returns only the README line below).
- `🔊️speech/` is an empty directory.
- No `🐍️.py` exists in the intro folder, so no `Intro` scene can be rendered from it.

Wiring status:

- Manifest `🎬️clip/🔣️.json` (energy) has no intro entry. The only match for "intro" in that file is the chapter scene `Heating_01_Introduction` (line 178), which is a heating chapter, not a UI intro.
- The test `🎬️clip/🪟️presentation/🧪️tests/🐍️.py` lines 46-54 asserts that the energy curriculum has no intro segments (`test_energy_curriculum_has_five_topic_owned_videos_and_no_institute_intro_segments`).
- The clip tests `🎬️clip/🧪️tests/🐍️.py` lines 649-698 create a synthetic `👋️intro/🐍️.py` fixture only inside a temp folder.
- Stale documentation: `🎬️clip/README.md:28` says "Omit chapter selection for the complete lesson with its institutional intro." The institutional intro has been removed from the energy curriculum, so the sentence is wrong.

Conclusion for A3: the institutional intro is not used anywhere. If the owner expects an intro video, it does not exist in the render path today.

## A4. Existing cache of UI frames (`.🧬semio/🎓️teaching/⚡️cache/videos/ui/`)

- `ui/h/`: 305 JSON sidecars and 228 PNG frames, content-addressed by hash. Each sidecar holds `hierarchy`, `footer_ancestors`, `layers`, `windows`, `chrome`, `cursor`, `content_camera`, and in some cases `border_compositor` and `motion_policy`. These are the browser's measured UI states (about 36 MB total). Example sidecar `03dcbb3b...json` records `opened_path: [architecture, optimization, optimization_sampling]`.
- `ui/b-0cf7768d`, `b-5ee90439`, `b-617b8a98`, `b-ccab8cc7`: each holds only `in.json` (the browser request). No `run.log` or `out.json`, so these attempts did not complete and were left behind.
- Writers: `🌐️browser/🐍️.py` line 188 (`output_port.output_root() / "ui/h"`) and line 262 (default `ui` directory).

Frames exist that were captured from the real UI. Nothing in `ui/h` is a placeholder.

## A5. What "proper frames" could mean, and what is missing

1. Capture controls and cursor the same way the chrome is captured. Option: extend `render.mjs` (line 207) to screenshot the control elements per state (hover, press, selected) and feed those PNGs to `_button` / `cursor_layer` instead of Manim `Rectangle`s. The DOM rectangles are already measured by `_probes` (`🥞️hierarchy/🖌️render/🐍️.py` 56-85), so the placement data needed exists.
2. Replace `_text` labels in controls with the browser's own text rendering (the label is in the captured frame).
3. Rewrite the institutional intro question explicitly: decide whether the owner wants an intro video (then author it: there is no `🐍️.py`, no narration, no manifest entry) or only the UI zoom. The clip README (line 28) and the test name at `🪟presentation/🧪️tests/🐍️.py:46` disagree on this.

Name clash to avoid in the fix: `Heating_01_Introduction` (chapter id) and `_transition` labels named `intro` (`🪟presentation/🐍️.py` 236-240) do not refer to a UI intro video.

## Files consulted

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🐍️.py`
- `.../🪟️presentation/🌐️browser/🐍️.py`, `🟨️render.mjs`
- `.../🪟️presentation/🥞️hierarchy/🖌️render/🐍️.py`
- `.../🪟️presentation/🖱️interaction/🖌️render/🐍️.py`, `🎬️scene/🐍️.py`
- `.../🪟️presentation/🧪️tests/🐍️.py`, `.../🎬️clip/🧪️tests/🐍️.py`
- `.../🎬️clip/🔣️.json`, `.../🎬️clip/README.md`, energy clip READMEs (ventilation, heating, cooling, demand, energy balance)
- `.🧬semio/🎓️teaching/⚡️cache/videos/ui/`
- Ticket `☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS/FEEDBACK.md` (line 9)
