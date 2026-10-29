# 🧰 Audit: Authoring Toolkit For The Grasshopper And Excel Clip Series

Read-only audit of the current code (2026-10-28). Verified against files, not against the prior ticket notes. Prior ticket (`CONCRETE-BIM-CLIP-EXAMPLES/audit-toolkit.md`) was used as a starting point; its deltas are in section 7.

Path keys used below (all relative to `C:/git/semio`):

- `TEACH` = `🎓️teaching/🏛️architecture`
- `BIM` = `TEACH/🏗️bim/🎬️clip`
- `OPT` = `TEACH/🎯️optimization/🎬️clip`
- `ENERGY` = `TEACH/⚡️energy`
- `AUTH` = `ENERGY/🏗️authoring`
- `VIS` = `AUTH/🎨️visuals/🐍️.py` (2,590 lines)
- `THEME` = `AUTH/🎨️theme/🐍️.py` (61 lines)
- `FONTS` = `AUTH/🔤️fonts/🐍️.py` (386 lines)
- `RUNTIME` = `AUTH/🎬️clip/⚙️runtime/🐍️.py` (842 lines)
- `RENDER` = `AUTH/🎬️clip/🖌️render/🐍️.py` (77 lines)
- `MANIFEST` = `ENERGY/🎬️clip/🔣️.json` (curriculum registry)

Python for everything: `C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe` (run files, never `python -` on stdin).

---

## 0. Bottom Line For Authors

1. A scene is a Manim `Scene` subclass that inherits the series `ClipScene` base. The base owns language, localized labels, numbers, timing (`step`), and chapter dispatch (`present` to `present_<topic>`).
2. Narration is a class attribute `NARRATION = [[key, en, de], ...]`. `self.step(key, ...)` shows the clause caption and holds the frame for the clause duration. There is no `self.say`.
3. Every chapter is registered in `MANIFEST`. The series playlist and each standalone video are thin wrappers around `RUNTIME`.
4. Keyframes come from `capture.py` (one PNG per clause plus a layout report). It lives in the prior ticket, not in the clip folders, and must be copied into the new ticket folder.
5. Tests are unittest modules under each series' `🧪️tests/`, plus a Node `.cjs` schema oracle and, for BIM, a presentation contract.

---

## 1. Scene Declaration And ClipScene Bases

### 1.1 BIM base: `BIM/🎨️visuals/🐍️.py`

Class (line 60):

```python
class ClipScene(_drafting.Drafting, _Savoye, _Axon, _panels.Panels, _Analysis, _engine.Scene):
```

Imports and module-level handles (all via `importlib.import_module`, literal emoji paths):

- `_fonts` = `AUTH/🔤️fonts/🐍️`
- `_art` = `AUTH/🎨️visuals/🐍️` (energy visuals)
- `_theme` = `AUTH/🎨️theme/🐍️`
- `_drafting` = `BIM/🎨️visuals/📐️drafting/🐍️`
- `_panels` = `BIM/🎨️visuals/🧾️panels/🐍️`
- `_engine` = `manim`
- `_Savoye` from `BIM/🎨️visuals/🏛️savoye/🐍️`, `_Axon` from `.../🏛️savoye/🧊️axon/🐍️`, `_Analysis` from `BIM/🎨️visuals/🧮️analysis/🐍️`

Module constants re-exported from drafting: `INK, MUTED, ARCH, STRUCT, SERV, DATA, GOOD, WARN, GLASS, CONCRETE, GREEN_GROUND`, plus `MINUS = "−"`.

Methods on `ClipScene` itself (the rest comes from the mixins):

| Method | Signature | Notes |
| --- | --- | --- |
| `language` | `language(self) -> str` | Reads `SEMIO_TEACHING_LANGUAGE`; raises `ValueError` unless `en` or `de`. No host default. |
| `tr` | `tr(self, en: str, de: str) -> str` | Authored label in the active language. |
| `number` | `number(self, value: float, digits=2) -> str` | `en 1,250.5` / `de 1.250,5`; true minus `−` for negatives. |
| `step` | `step(self, key: str, *animations, seconds=1.2) -> None` | Look up `key` in `NARRATION`, swap the caption, play animations, then `_art.hold_for`. |
| `present` | `present(self, topic: str) -> None` | `set_vo_language`, `apply_scene_style`, then `getattr(self, "present_" + topic)()`, then fade the caption out. |

Mixin methods used by scenes (signatures from the source):

- Drafting (`BIM/🎨️visuals/📐️drafting/🐍️.py`, class `Drafting` line 318): `label(en, de=None, point, *, size=S_LABEL, color=INK, width=None, anchor="c")`, `text(string, point, *, size=S_LABEL, color=INK, width=None, anchor="c")`, `mono(string, point, *, size=S_SMALL, color=INK, anchor="c")`, `quantity(value, unit="", digits=2)`, `live(builder, key=None)`, `live_label(fn, point, *, size, color, anchor)`, `dimension(start, end, text=None, *, offset=0.3, color=DATA, size=S_SMALL, tick=0.07, ...)`, `level_mark(point, text, *, color, size, length=0.9, side=1)`, `hatch(rings, color=MUTED, spacing=0.09, angle=45.0, width=W_HAIR, opacity=0.9)`, `leader(source, target, text=None, *, color=DATA, size=S_SMALL, dot=True, anchor=None)`, `badge(text, point, *, color=GOOD, size=S_SMALL, filled=True, anchor="c")`, `north_arrow(point, size=0.5, *, color=INK, label="N")`, `scale_bar(point, scale, metres=10.0, *, color=INK, steps=2)`.
- Panels (`BIM/🎨️visuals/🧾️panels/🐍️.py`, class `Panels` line 33): `class_box(name, rows, point, *, color=ARCH, width=3.2, grey=(), size=S_SMALL, anchor="c")`, `class_tree(chains, point, *, highlight=(), width=6.4, expand=None, color=ARCH, leaf_attrs=2, pitch=0.58)`, `property_table(rows, point, *, width=6.8, title=None, basis=True, size=S_SMALL, color=GOOD)`, `step_panel(excerpt, point, *, width=6.6, wrap=None, highlight_roles=(), only=None, size=S_SMALL, color=MUTED)`, `model_card(model_id, point, *, model=None, width=3.9, thumbnail=True, scale=0.08)`, `guid_chip(key, point, *, color=DATA, model=None, anchor="c", label=True)`.
- Villa Savoye and axon (`BIM/🎨️visuals/🏛️savoye/🧊️axon/🐍️.py`, class `Axon` line 706): `axon_view(point, scale=0.26, *, model=None, view="sw")`, `axon_point(x, y, z, point, scale=0.26, *, discipline=None, explode=0.0, model=None, view="sw")`, `villa_axon(point, scale=0.26, *, model=None, highlight=(), disciplines=(...), explode=0.0, ...)`, `element_axon(tag, point, scale=0.26, *, model=None, extrude=1.0, opening=1.0, window=1.0, highlight=(), ...)`. `Savoye` (`🏛️savoye/🐍️.py` line 18) is `Savoye(_Axon, SavoyeOrtho)` where `SavoyeOrtho(_Plan, _Section, _Detail)`; plan, section and detail widgets are its methods (not enumerated here).
- Analysis (`BIM/🎨️visuals/🧮️analysis/🐍️.py`, class `Analysis` line 72): `analysis_view(analysis, point, scale=0.26, *, projection="plan", loads=None, stale=False, highlight=(), ...)`, `load_arrows(values, points, scale=ARROW_KN, *, unit="kN", per=None, direction=(0, -1), anchor="tail", ...)`, `tributary_view(tributary, point, scale, *, projection="plan", highlight=(), level=None, ...)`, `tributary_legend(point, *, kinds=KINDS, direction="row", size=S_SMALL, gap=0.3)`, `mesh_view(mesh, point, scale=0.75, *, view="axon", segments=None, highlight=(), vertices=True, ...)`.

Module-level helper convention seen in every BIM scene (copy it): `_fade(group, alpha)` (information scene line 49) and `_put(shape, point, edge="c")` and `_say(string, point, *, size, color, edge, width)` defined on the scene class.

`source_files()` (BIM base line 33): returns the visuals file, widget files, model files and theme sources for render provenance. The runtime consumes provenance through `source_dependencies(script)` (`RUNTIME` line 391). I did not read that function in full, so treat "copy this pattern" as a convention, not a verified contract.

### 1.2 Optimization base: `OPT/🎨️visuals/🐍️.py`

Class (line 150ish, verified by grep): `class ClipScene(_room.RoomVisuals, _engine.Scene):`

Module-level: `_fonts`, `_art`, `_theme`, `_model` (`OPT/🧮️model/🐍️`), `_room` (`OPT/🎨️visuals/🏠️room/🐍️`), `_engine` = manim. Colours: `INK = P_WHITE`, `MUTED = PASTEL_GREY`, `DESIGN = P_CYAN`, `DATA = P_YELLOW`, `GOOD = P_GREEN`, `WARN = P_ORANGE`. Also `Y_TICK_GAP = 0.14`. `CornerAxes(_engine.Axes)` (axes whose lines meet at the range minimum). `source_files()` as in BIM.

Methods on `ClipScene`:

| Method | Signature | Notes |
| --- | --- | --- |
| `language` | `language(self) -> str` | Same contract as BIM (`SEMIO_TEACHING_LANGUAGE`, en or de). |
| `tr` | `tr(self, en, de) -> str` | Same. |
| `number` | `number(self, value, digits=0) -> str` | Delegates to `_room.number(value, digits, german)`. Default 0 digits (BIM default is 2). |
| `quantity` | `quantity(self, value, unit, digits=0) -> str` | Number plus unit joined by narrow no-break space (`_room.unit_text`). |
| `parts` | `parts(self, values, digits=0, total=None) -> list` | Largest-remainder apportionment so printed parts add to the printed total (`_room.apportion`). |
| `label` | `label(self, en, de, point=(0,0,0), *, size=24, color=INK, width=6)` | Localized label, bundled sans. |
| `step` | `step(self, key, *animations, seconds=1.2)` | Same behaviour as BIM `step`, but the `_art.hold_for` localized list uses `self.tr`. |
| `explain` | `explain(self, key, *animations, seconds=1.2)` | Plays `step(key)` plus any `key_2`, `key_3`, ... clauses that exist in `NARRATION`. This is the sub-clause convention. |
| `clear_stage` | `clear_stage(self)` | Fades every mobject except the active caption (0.5 s). |
| `present` | `present(self, topic)` | Same dispatch as BIM (`present_<topic>`). |
| `plot_axes` | `plot_axes(self, *, x_range, y_range, x_label=None, y_label=None, point=(-.8,.25,0), width=7.5, height=3.6, x_digits=0, y_digits=0, x_ticks=None, y_ticks=None, titles=True, corner=False) -> (axes, labels)` | Physical-unit axes with localized titles and numeric ticks. |
| `axes` | `axes(self, *, x_range=(0,1,.5), y_range=(0,1,.5), point=(0,.15,0), width=7, height=3.3)` | Plain axes without tick labels. |
| `live_marker` | `live_marker(self, axes, trackers, x_key, value_fn, *, x_label="", x_unit="", x_digits=0, y_label="", y_unit="", y_digits=0, color=INK, drops=True, readout=True, x_scale=1.0, avoid=())` | Dot on the curve that follows trackers, dashed drop lines, and a readout chip that avoids `avoid` geometry. |
| `study_trackers` | `study_trackers(self, design=None) -> dict` | (`RoomVisuals`) ValueTrackers for the four design keys. |
| `set_design` | `set_design(self, trackers, design, run_time=1.0)` | (`RoomVisuals`) Animates trackers to a design. |

Shared keys in the optimization room: `KEYS = ("orientation", "wwr", "overhang", "insulation")`. Model access: `_room.result(design)` returns evaluated outputs.

### 1.3 Scene class declaration, NARRATION and TITLE

Shape (verified in `BIM/📊️information/🐍️.py` and `OPT/📉️extrema/🐍️.py`):

```python
class Beat1_DataToInformation(_visuals.ClipScene):
    """📚 Docstring starting with an emoji, English and German narration."""

    TITLE = {"en": "Data Become Information", "de": "Aus Daten wird Information"}
    NARRATION = [
        ["raw", "English clause, one sentence.", "Deutscher Satz."],
        ...
    ]

    def construct(self):
        """▶️ Present this chapter through the native BIM visual interface."""
        self.present("information")

    def present_information(self):
        """🎞️ The whole animation ..."""
        lang = self.language()
        ...
```

Rules that the tests enforce:

- `NARRATION` is a list of 3-element lists `[key, en_text, de_text]` (optimization `Clause` alias is a tuple in `VIS` line 2121: `Clause = tuple[str, str, str]  # (section_key, narration_en, subtitle_de)`; the comment is stale, the element order is `(key, en, de)`).
- Keys are unique per scene. Optimization sub-clauses use the suffix `_2`, `_3`, ... (`explain`).
- `TITLE` is a dict with `en` and `de`. Chapter titles for the cases file live in `OPT/🧪️cases/🔣️.json` (`title: {en, de}`).
- The `construct` method calls `self.present("<topic>")`, and `present_<topic>` must exist on the class (`getattr` dispatch in `present`).
- Both series already use Python's `RUNTIME` calls; a scene never reads the audience or chapter selection directly. Audience is controlled by `SEMIO_TEACHING_AUDIENCE` (`speaker` shows captions, `spectator` does not), set by the runtime.

### 1.4 Clause timing: `step`, `hold_for`, caption

- `step(key, *animations, seconds=1.2)` (both series): fetch the row for `key`, show the caption (`caption_bar` on first use, `swap_caption` after that), play the animations for `seconds`, then call `_art.hold_for(self, localized, key)`.
- `_art.hold_for(scene, narration, key, *, used=0.0, min_wait=0.3, during=None) -> float` (`VIS` line 2203). Holds the frame for the time the clause needs, minus the time already visible. `during=lambda rt: [...]` runs a continuous update during the hold (used by sweeps in the energy solar-gains beat).
- `_art.narration_seconds(narration, key=None) -> float` (line 2159) and `_art.set_vo_language(lang)` (line 2126) pick the language used for timing estimates.
- Timing budget (from prior audit, consistent with `hold_for` docstring): max(measured TTS, word estimate, reading floor). Not re-derived here.

### 1.5 Language selection, numbers, live values

- Language: only `SEMIO_TEACHING_LANGUAGE` in `en`/`de`. The runtime sets it per segment (`RUNTIME` `render_segment` env block, line ~505). Never default.
- Numbers: BIM `self.number(value, digits=2)`, optimization `self.number(value, digits=0)`, `self.quantity(value, unit, digits)`, `self.parts(values, digits, total)`. Energy `de_num(value, digits=0)` (`VIS` line 1963) and `_room.number(value, digits, german)`.
- Live values: BIM `self.live(builder, key=None)` (rebuild on `key()` change) and `self.live_label(fn, point, ...)` (re-typeset only when the string changes). Optimization `live_marker` plus `RoomVisuals._live`/`_widget`. Energy `math_readout(fn, at, *, size=None, color=P_WHITE, edge="left")` (`VIS` line 1999) for math labels that update each frame.

---

## 2. Shared Energy Authoring Toolkit

### 2.1 Palette (`THEME` + `VIS` constants)

Tokens come from `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json` through `THEME.color(token)` (line 23).

| Token | Hex | `VIS` constant(s) |
| --- | --- | --- |
| primary | `#ff344f` | `P_PRIMARY`, `P_RED`, `PASTEL_RED`, `PASTEL_PINK` |
| secondary | `#34d1bf` | `P_SECONDARY`, `P_CYAN`, `P_TEAL`, `P_BLUE`, `PASTEL_TEAL`, `PASTEL_CYAN`, `PASTEL_BLUE` |
| tertiary | `#fa9500` | `P_ORANGE`, `PASTEL_ORANGE` |
| warning | `#fccf05` | `P_YELLOW`, `PASTEL_YELLOW` |
| success | `#7eb77f` | `P_GREEN`, `PASTEL_GREEN` |
| info | `#dbbea1` | `PASTEL_PURPLE` (beige, not purple) |
| dark | `#001117` | `P_DEEP_DARK` (background) |
| light | `#f7f3e3` | `P_WHITE`, `PASTEL_WHITE` (ink) |
| gray | `#7b827d` | `PASTEL_GREY` |

Note: `P_BLUE` is teal (secondary); there is no true blue token.

BIM drafting roles (`BIM/🎨️visuals/📐️drafting/🐍️.py` lines 21-34): `INK=P_WHITE`, `MUTED=PASTEL_GREY` (the prior audit's `MUTED=P_PRIMARY` is fixed), `ARCH=P_CYAN`, `STRUCT=PASTEL_PURPLE`, `SERV=P_RED`, `DATA=P_YELLOW`, `GOOD=P_GREEN`, `WARN=P_ORANGE`, `GLASS=P_CYAN`, `BACKGROUND=P_DEEP_DARK`, `CONCRETE=color("light-gray")`, `GREEN_GROUND=mix(P_GREEN, BACKGROUND, 0.52)`, `GROUND_PLANE=mix(P_GREEN, BACKGROUND, 0.82)`, `DISCIPLINES={"architecture": ARCH, "structure": STRUCT, "services": SERV}`. `mix(a, b, amount)` and `shade(colour, factor)` are in drafting.

Optimization roles (`OPT/🎨️visuals/🐍️.py` lines 15-27): `INK=P_WHITE`, `MUTED=PASTEL_GREY`, `DESIGN=P_CYAN`, `DATA=P_YELLOW`, `GOOD=P_GREEN`, `WARN=P_ORANGE`. The extrema beat adds `HEAT=P_RED`, `COOL=P_BLUE`, `LIGHT=P_YELLOW`, `LOCAL=PASTEL_PURPLE`.

### 2.2 Fonts (`FONTS` + `THEME`)

- `THEME.FONT_SANS`, `FONT_SERIF`, `FONT_MONO` come from the token file's `fontStacks`. `FONT_MATH = "Libertinus Math"` (font file `🧰️framework/.../📚️compiler/🌍️world/🔤️fonts/🧮️LibertinusMath-Regular.otf`).
- Anta (body face) is the bundled sans used by `body_text`. Share Tech Mono (mono) is bundled and used by `Drafting.mono(...)` (BIM). Verified: the prior audit's "monospace never drawn" is no longer true for BIM.
- `THEME.font_size(role) -> float` (line 37) reads the type scale; ladder used by drafting: `S_SMALL=16.2`, `S_LABEL=19.2`, `S_BODY=21.6`; caption default `CAPTION` in energy visuals.
- Fonts functions (`FONTS`):
  - `register_fonts(font: str | None = None) -> None` (line 55)
  - `apply_scene_style(scene) -> str` (line 276): applies the body font to a scene (called by every `present`)
  - `body_text(text, *, font_size, color, **kwargs)` (line 285): returns a styled `Text`
  - `wrap_body_lines(text, *, font_size, max_width) -> list[str]` (line 299)
  - `centered_body_text(...)` (line 319)
  - `section_heading(text)` (line 365)
  - `_font_supports(font, character) -> bool` (line 139, private) and `_glyph_fallbacks` (line 150) substitute missing glyphs silently; check coverage before using `#`, `;`, `'`, `*`, `$` in mono text.

### 2.3 Stage geometry, title, caption

Frame: 16:9 Manim frame, height 8, width about 14.22. Verified constants (`VIS`):

- Content band: `SAFE_TOP = 2.58`, `SAFE_BOTTOM = -2.60`, `SAFE_LEFT = -6.80`, `SAFE_RIGHT = 6.80` (line 169-176). `SAFE_BOTTOM_FORMULA = -1.66` is defined; it is the content floor when a formula panel is visible (check each beat's own use before relying on it).
- `fit_band(group, *, top=SAFE_TOP, bottom=SAFE_BOTTOM)` (line 179): scales and shifts a scaffold into the band.
- Note band: `NOTE_BAND_Y: float = 2.30` (line 1335); `note_line(text, *, color=P_PRIMARY, font_size=None, y=NOTE_BAND_Y)` (line 1340).
- Formula band: `FORMULA_BAND = (-2.80, -1.84)` (line 2373). `formula_panel(row, *, color=P_PRIMARY, edge_buff=FORMULA_PANEL_EDGE_BUFF)` (line 1611); `FORMULA_PANEL_EDGE_BUFF = 1.7`.
- Caption band: `CAPTION_BAND = (-4.00, -2.62)` (line 2374). `CAPTION_MAX_WIDTH = 11.5`, `CAPTION_MAX_LINES = 2`, `CAPTION_EDGE_BUFF = 4 * UI_SPACING`, `CAPTION_LINE_BUFF = UI_SPACING` (lines 2026-2031).
- Caption function: `caption_bar(text_de: str, *, font_size=None, color=P_WHITE)` (line 2034). `swap_caption(scene, old, text_de, *, run_time=0.35, **kwargs)` (line 2084).
- Title: no shared title function was found in the energy visuals. The scene's `TITLE` dict is authored text; the presentation layer (`AUTH/🪟️presentation`) renders the chapter title. Do not put a title in the content band.

Layout zones are tags on mobjects: `_layout_zone` with values `formula`, `formula_box`, `caption`, `caption_box` (read by `_zone(mob)`, line 2377). Tagged text skips the band checks, so it must not be used to dodge the guard. The prior audit's claim that BIM abuses the tag is not re-verified in this pass.

### 2.4 Layout guard

- `layout_conflicts(scene) -> list[str]` (`VIS` line 2460): more than two caption lines; untagged text in the caption band; text in the formula band when a panel is shown; text-text overlaps; graphics touching a caption or formula frame; opaque graphic under text; strokes through text. Only `Text` instances are considered; live labels whose own fill opacity is 0 are invisible to it (this is why `layout_strict.py` exists, see section 4).
- `check_layout(scene, label="") -> list[str]` (line 2556) wraps it with a label.

### 2.5 Top 30 reusable functions and constants (energy `VIS` unless marked)

Signatures are as in the source; `...` marks keyword arguments not listed in full.

1. `color(token: str) -> str` (THEME 23)
2. `caption_bar(text_de: str, *, font_size=None, color=P_WHITE)` (2034)
3. `swap_caption(scene, old, text_de: str, *, run_time=0.35, **kwargs)` (2084)
4. `set_vo_language(lang: str) -> None` (2126)
5. `hold_for(scene, narration, key: str, *, used=0.0, min_wait=0.3, during=None) -> float` (2203)
6. `narration_seconds(narration, key=None) -> float` (2159)
7. `fit_band(group, *, top=SAFE_TOP, bottom=SAFE_BOTTOM)` (179)
8. `note_line(text, *, color=P_PRIMARY, font_size=None, y=NOTE_BAND_Y)` (1340)
9. `formula_panel(row, *, color=P_PRIMARY, edge_buff=1.7)` (1611)
10. `equation_row(parts, *, font_size=None, color=P_WHITE, buff=0.15)` (1575)
11. `math_label(src: str, at=None, *, size=None, color=P_WHITE, edge="center")` (1976)
12. `math_row(parts, *, font_size=None, color=P_WHITE, buff=0.15)` (1935)
13. `math_text(src: str, *, font_size=None, color=P_WHITE)` (1927)
14. `math_panel(parts, *, color=P_PRIMARY, size=None, ...)` (2010)
15. `math_readout(fn, at, *, size=None, color=P_WHITE, edge="left")` (1999)
16. `highlight_param(items: dict, key: str, *, color=P_PRIMARY)` (1626)
17. `symbol_token(text, *, color=P_WHITE, font_size=28)` (1558)
18. `chip(text, color=P_PRIMARY, *, font_size=None)` (744); `window_chip(content, color=P_PRIMARY)` (735)
19. `window_card(heading, reading, *, color=P_PRIMARY, width=None, height=None)` (1363)
20. `stat_card(...)` (1382) and `card_grid(entries, *, rows=1, cols=None, color=P_PRIMARY, ...)` (1403)
21. `side_labels(anchors, *, x, font_size=None, min_gap=0.40, ...)` (1433) dodged callouts with leaders
22. `meter(label, *, length=2.3, thickness=0.52, ...)` (672); `set_meter(gauge, frac)` (702); `bind_meter(gauge, tracker)` (716); `recolor_meter(gauge, color, **kwargs)` (723)
23. `stacked_bar(segments, *, base, width=1.0, unit=1.0, opacity=0.78)` (1487)
24. `labeled_axes(origin, *, x_len, y_len, x_label=None, ...)` (1520)
25. `energy_tank(center, *, height=2.2, width=0.9, color=P_YELLOW, level=1.0)` (1272); `thermometer_glyph(bottom, *, height=1.6, color=P_RED, level=0.0)` (1253)
26. `room_section(center, *, w=5.0, h=2.5, wall=0.2, slab=0.26, ...)` (1100); `house_section(center=None, scale=1.0)` (1048, gable only)
27. `window_glyph(x, sill, lintel, *, depth=0.42, color=P_CYAN, ...)` (987); `open_window(window, *, run_time=0.9, fraction=1.0, tilt=False, ...)` (1039)
28. `person_glyph(pos, color=P_ORANGE, scale=1.0)` (1195); `radiator(center, color=P_RED, *, height=None, width=None)` (1134); `cloud_glyph(content, *, pad=(0.42,0.36), color=P_WHITE)` (1223)
29. `dim_arrow(start, end, color=P_YELLOW)` (767); `cross_mark(color=P_RED, size=0.13)` (757); `dashed(a, b, color=GRAY, ...)` (`OPT/🎨️visuals/🏠️room/🐍️.py` 478)
30. `layout_conflicts(scene)` (2460); `place_math(mob, at, edge="center")` (1968); `de_num(value, digits=0)` (1963)

BIM-side functions that reach the same ground: `Drafting.dimension`, `Drafting.hatch`, `Drafting.leader`, `Drafting.badge`, `Drafting.mono`, `Panels.class_tree`, `Panels.property_table`, `Panels.step_panel`, `Analysis.load_arrows`, `Axon.villa_axon`, `Axon.element_axon`.

### 2.6 Tables and axes

- Energy has no table primitive; use `card_grid` or `property_table` (BIM).
- Axes: `labeled_axes` (VIS 1520), optimization `plot_axes` and `axes` (section 1.2), BIM `analysis_view` for plans.

---

## 3. Complete Example Skeletons

### 3.1 BIM information scene (`BIM/📊️information/🐍️.py`, 851 lines)

Verified skeleton of the first 110 lines:

```python
"""🎬 Data Become Information on the Villa Savoye ground floor: ..."""

import importlib
import math
import sys
from pathlib import Path
from types import SimpleNamespace

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
_draft = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_panels = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🧾️panels.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_sheet = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📏️sheet.🐍️")
_plan_widget = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📋️plan.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, ARCH, STRUCT, SERV, DATA, GOOD, WARN, GLASS, CONCRETE, GREEN_GROUND = (
    getattr(_visuals, name) for name in ("INK", "MUTED", "ARCH", "STRUCT", "SERV", "DATA", "GOOD", "WARN", "GLASS", "CONCRETE", "GREEN_GROUND"))
BACKGROUND = _draft.BACKGROUND
LABEL, SMALL = 19.2, 18.0
ORIGIN = np.array([-3.0, 0.0, 0.0])
...

def _fade(group, alpha): ...

class Beat1_DataToInformation(_visuals.ClipScene):
    """📚 Animate data become information on the Villa Savoye ground floor with explicit English and German narration."""

    TITLE = {"en": "Data Become Information", "de": "Aus Daten wird Information"}
    NARRATION = [
        ["raw", "This number is data. Alone, it does not say what was measured.",
         "Diese Zahl ist ein Datenwert. Allein sagt sie nicht, was gemessen wurde."],
        ["guess", "Metres, kilograms, degrees: without a unit it could be anything.",
         "Meter, Kilogramm, Grad: ohne Einheit könnte es alles sein."],
        ... (21 clauses in total: raw ... same)
    ]

    def construct(self):
        """▶️ Present this chapter through the native BIM visual interface."""
        self.present("information")

    def _put(self, shape, point, edge="c"): ...
    def _say(self, string, point, *, size=LABEL, color=INK, edge="c", width=None): ...

    def present_information(self):
        """🎞️ The whole animation: ..."""
        lang = self.language()
        m = _model.load_model()
        self._m, self._g, self._c, self._path = m, _model.building_geometry(m), _model.clearance(m), _model.car_path(m)
        ...
        # later: self.step("raw", FadeIn(...)), self.step("guess", ...), etc.
```

Note the information scene is the only one in this pass that imports `_sheet` and `_plan_widget` directly; most beats reach widgets through `ClipScene`.

### 3.2 Optimization extrema scene (`OPT/📉️extrema/🐍️.py`)

```python
"""📉 Formulate a feasible optimization problem and distinguish local from global extrema."""

import importlib
import math
import sys
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
_room = _visuals._room
_art = _visuals._art
_fonts = _visuals._fonts
_mn = _visuals._engine

INK, MUTED, DESIGN, DATA, GOOD, WARN = _visuals.INK, _visuals.MUTED, _visuals.DESIGN, _visuals.DATA, _visuals.GOOD, _visuals.WARN
HEAT, COOL, LIGHT = _art.P_RED, _art.P_BLUE, _art.P_YELLOW
...
PARTS = (("heating", ("Heating", "Heizstrom"), HEAT), ...)
...

class Beat1_LocalGlobalExtrema(_visuals.ClipScene):
    """🏔️ Teach direction, feasible domain and extrema ..."""

    TITLE = {"en":"Minimization and Maximization", "de":"Minimierung und Maximierung"}
    NARRATION = [
        ["formulate", "This seminar room has one glazed façade. Which compass direction should it face?", "Dieser Seminarraum hat ..."],
        ["formulate_2", "...", "..."],
        ...   # 27 clauses; sub-clauses use the _2, _3 suffix convention
    ]

    def construct(self):
        """▶️ Animate the full formulation through the shared local rendering interface."""
        self.present("extrema")

    def present_extrema(self):
        """🎞️ Run the phases: formulate, fix, sweep, local and global minimum, maximize, constrain, scope."""
        study = self.study = _model.orientation_study()
        table = dict(study["samples"])
        self.low, self.high = study["range"]
        self.energy = lambda theta: self._interpolate(table, theta)
        self.reference_energy = study["reference_electricity"]
        self.saving = lambda theta: self.reference_energy - self.energy(theta)
        self.fixed = dict(study["design"])
        self.peak, self.peak_energy = study["local_maxima"][0], study["local_maxima_electricity"][0]
        self.dials = self.study_trackers({**self.fixed, "orientation": 90.0})
        # later: self.step("formulate", ...), self.explain("sweep", ...), self.clear_stage()
```

Rule of thumb from both: the scene reads numbers only from the model (`_model.*`, `_room.result(...)`), never typed by hand.

### 3.3 Standalone and playlist wrappers (copy these)

Standalone (`OPT/../📉️extrema/🎬️clip/🐍️.py`, 31 lines), same shape for every BIM and optimization standalone:

```python
"""🎬 Render the standalone optimization extrema video through the shared curriculum interface."""
from __future__ import annotations
import importlib
import sys
from pathlib import Path
sys.dont_write_bytecode = True
_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))
_runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
VIDEO_ID = "optimization_extrema"
def compose(*, language: str, chapters: list[str] | None = None, **settings) -> dict[str, Path]:
    return _runtime.compose_videos([VIDEO_ID], language=language, chapter_scenes=chapters, **settings)
def main(argv: list[str] | None = None) -> int:
    return _runtime.video_main(VIDEO_ID, argv)
if __name__ == "__main__":
    raise SystemExit(main())
```

Series playlist (`BIM/🐍️.py` and `OPT/🐍️.py`, 31 lines): `VIDEO_IDS = (...)` tuple of six ids; `compose(...)` calls `compose_videos(list(VIDEO_IDS), ...)`; `main(argv)` builds a `--video <id>` selection and calls `importlib.import_module("...🖌️render.🐍️").main(...)`.

---

## 4. Keyframe Capture And Layout Checker

### 4.1 `capture.py` (prior ticket: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/capture.py`, 3,401 bytes)

What it does, verified:

- Takes `topic name language [keys...]`. `topic` is a BIM topic folder name (for example `📊️information`); the module path is built as `"🎓️teaching.🏛️architecture.🏗️bim.🎬️clip." + topic + ".🐍️"` and the scene class is fetched by name from it.
- Sets `SEMIO_TEACHING_AUDIENCE=speaker`, `SEMIO_TEACHING_LANGUAGE=<lang>`, `SEMIO_TEACHING_CACHE_DIR=<T>/🗑️generated/fonts/<Scene>-<lang>`.
- Subclasses the scene as `Capture`, overriding `step(key, *animations, seconds=1.2)`: it removes the old caption, builds the new one, skips animation time (`run_time=.01`), and for each requested key saves `<T>/🗑️generated/frames/<Scene>/<Scene>-<lang>-<key>.png` at 1280x720.
- For each frame it writes the `art.layout_conflicts(self)` result and the `layout_strict.strict_conflicts(self)` result into `<Scene>-<lang>-layout.md`.
- Output summary line: `strict findings: N in M of K frames`.
- Renders with `manim.tempconfig({... "skip_animations": True, "disable_caching": True, ...})` and `Capture().render()`.
- Requires `layout_strict.py` in the same folder (`sys.path.insert(0, TICKET)`).

Important constraints for the new series:

- `capture.py` is hard-wired to BIM. Change the module-path prefix (`"🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."`) to the new series clip folder (for example `🎓️teaching.🏛️architecture.<grasshopper-root>.🎬️clip.`), and the `topic` argument then means that series' topic folder.
- `capture.py` relies on `NARRATION`, `self._caption`, `self.renderer`, and `self.language()`. Both BIM and optimization scenes provide these (`ClipScene.step`), so the same override works for optimization scenes. The optimization `explain` calls `step`, so it is captured too.
- It imports `art = …energy.🏗️authoring.🎨️visuals.🐍️` and expects `caption_bar`, `layout_conflicts`. Both exist.

### 4.2 `layout_strict.py` (prior ticket: `…/CONCRETE-BIM-CLIP-EXAMPLES/layout_strict.py`, 16,046 bytes)

Public functions (verified): `strict_conflicts(scene) -> list[str]` (line 324) and `strict_section(conflicts: list[str]) -> str` (line 344).

Thresholds (module constants): `MIN_TEXT_PX = 11.0`, `EDGE_PX = 12.0`, `OVERLAP_PX = 3.0`, `TOUCH_PX = 4.0`, `TOUCH_RUN = 4`, `CAPTION_TOP = -2.62`, `GRID = (9, 3)`, `MAX_PER_LABEL = 3`, `CHIP_PAD = 0.45`, `CHIP_AREA = 14.0`, `BADGE_CHARS = 2`, `FRAME_ZONES = frozenset({"caption_box", "formula_box"})`, `GLYPH_OPACITY = 0.05`, `BACKING_OPACITY = 0.85`.

Why it exists: `LiveText` (a `Text` with fill opacity 0) is invisible to `layout_conflicts`. `layout_strict` reads opacity from the glyphs, so live chips and value tags are checked too. The module docstring says identical copies must sit next to both `capture.py` scripts.

### 4.3 Exact command lines for a new folder

Assumed new ticket folder `T` (the grasshopper/excel ticket): `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS`. Before the first run, copy `capture.py` and `layout_strict.py` into `T` (they are ticket scripts, not checked-in source, so this is allowed). Adjust the module path inside `capture.py` for the new series first.

```
cd /c/git/semio
C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/capture.py" <topic-folder> <SceneClass> en
C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/capture.py" <topic-folder> <SceneClass> de <key1> <key2>
```

Example that works today for BIM (from the prior ticket): `python T_bim/capture.py 📊️information Beat1_DataToInformation en`. Output: `T/🗑️generated/frames/Beat1_DataToInformation/...png` and `…-layout.md`. Read the PNGs with the Read tool, and fix every overlap, clipped label or unreadable text.

Output rules: everything goes under `T/🗑️generated/` (PNGs, layout reports, fonts cache). Keep the path under 256 characters; the emoji-heavy path plus the scene name can get long.

### 4.4 Changes needed for a new series

1. `capture.py`: change the module prefix in `importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip." + topic + ".🐍️")` to the new clip root.
2. Copy `layout_strict.py` next to it.
3. Nothing else: `capture.py` already imports energy visuals (`art`) and uses `ClipScene` through the scene class.

---

## 5. Imports And Test Layout

### 5.1 How scenes import shared code

- Literal `importlib.import_module("🎓️teaching.🏛️architecture.<folder>…🐍️")` calls, with emoji folder names. Module names are not Python identifiers but `importlib` accepts them.
- Every scene file first inserts the workspace root (the folder containing `🧰️framework`) into `sys.path`.
- Manim is always `importlib.import_module("manim")` (BIM visuals line ~18, optimization visuals line ~24).
- The renderer also sets `SEMIO_TEACHING_LANGUAGE`, `SEMIO_TEACHING_AUDIENCE`, `SEMIO_TEACHING_OUTPUT_ROOT`, `SEMIO_TEACHING_SPEECH_ROOT`, `VO_TRACE=1` per segment (`RUNTIME` `render_segment`).
- The manifest references source files with the relative prefix `🏛️architecture/...` (see section 6), resolved by `teaching_root(path)` (`RUNTIME` line 36), which searches parents for `🏛️architecture/⚡️energy`.

### 5.2 Test files, what they check

BIM (`BIM/🧪️tests/`):

- `🐍️.py` (2,266 lines, unittest). Test classes: `BimGlobalIdContract` (GlobalId oracle vs uuid library, random round-trips, invalid inputs, guid derivation and uniqueness); `BimModelStructureContract` (bilingual text, sourced or assumed quantities with units, research facts, element completeness, opening chains, IFC class registry, demo events); `BimGridContract` (25 pilotis on the 4.75 m grid, first-floor slab, piloti type count); `BimGeometryContract` (box, slabs, walls, curtain wall curve, ramp slope, screens and drains, areas); `BimClearanceLimits` and `BimClearanceContract` (bisection of clearance, pinch distances, fit flip at required margin, car path); `BimInformationContract` (language required, portable cases, values vs model); `BimQuantityContract` (volumes, areas, glazing share, layer totals); `BimClassChainContract`; `BimRelationContract`; `BimRepresentationContract`; `BimPartialModelContract`; `BimStepExchangeContract` (STEP header, entity attribute counts, spans, parser oracle); `BimMeshExchangeContract` (Euler characteristic, watertightness, volume against prism and hull); `BimIdsContract` (IDS requirement checks). Oracles: `uuid`, NumPy, SciPy. The module also loads the model without importing drawing code (`model_api()`).
- `🟨️.cjs` (`BIM/🧪️tests/🟨️.cjs`): Node oracle. Validates `🧱️model/🔣️.json` against `🧱️model/🧬️schema/🔣️.schema.json` with AJV (2020-12, strict), rejects the `invalid_models` cases, and checks GlobalId encoding vectors with `crypto` SHA-1 UUIDv5.
- `🟨️metadata.mjs`: checks the browser metadata renderer source imports (Babel parser, reads `browser_metadata` from `🧪️cases/🔣️.json`). The Python test invokes it via `repo tools exec --light node -- <metadata.mjs>` (line ~2262), so it needs the `repo` binary.
- `🎭️presentation/🐍️.py` (28 KB, unittest `BimPresentationContract`): checks every BIM leaf implements its declared presentation, per localized chapter and language (`CASES` = `🎭️presentation/🔣️.cases.json`, `chapters[]` with `id`, `script`, `scene`, `topic`, `keys`). Hashes the authored Python and JSON inputs for each run. Focused runs: env `SEMIO_TEST_BIM_TOPICS=<comma-separated topic folder names>`.

Optimization (`OPT/🧪️tests/`):

- `🐍️.py` (1,295 lines, unittest). Classes: `StudyModel` (vectorized NumPy re-implementation, documented coefficients in both languages, shading geometry, energy balance closure, direct gain continuity); `BearingContinuity`; `OrientationExtrema` (local and global minima vs SciPy bounded and Brent searches, courtyard constraint, greedy descent from 315°); `SurfaceAndSampling` (41x41 surface, Nelder-Mead cross-check, random and Latin hypercube sampling); `FactorialExperiments` (2^k contrasts vs least squares); `IndexOfVariants` (min-max normalization, weights, reweigh, invalid weights); `ParetoFront` (dominance matrix, payback crossing, knee vs tariff); `AlgorithmPaths` (gradient, pattern search, genetic algorithm, budgets); `SchemaFirstStudy` (schema is 2020-12 and forbids extras); `OptimizationClipContract` (narration extraction via AST, clause fit in both languages and audiences, six independent videos in the curriculum, entry points route only their own scenes, playlist routes six videos).
- `🟨️.cjs`: AJV validation of `🧮️model/🧬️schema/🔣️.schema.json` against `🧮️model/🔣️.json`, plus `invalidModels` cases from `🧪️cases/🔣️.json` (note the camelCase key `invalidModels`, unlike BIM's `invalid_models`).
- `🧪️cases/🔣️.json`: `version`, `languages`, `audiences` (`speaker` captions true, `spectator` false), `chapters[]` with `folder`, `scene`, `video_id`, `entry`, `title{en,de}`, `output`, and six chapters. Folder values: `📉️extrema`, `🎲️sampling`, `🧪️experiments`, `⚖️index`, `🏆️pareto`, `🧬️algorithms`.

Energy shared (`AUTH/🎨️visuals/🧪️tests/`, `AUTH/🔤️fonts/🧪️tests/`, `AUTH/🎬️clip/🧪️tests/`, `AUTH/🎬️clip/🪟️presentation/🧪️tests/`): shared visuals, fonts, render and presentation contracts. The render test `AUTH/🎬️clip/🧪️tests/🐍️.py` (51 KB) and `🔣️.cases.json` cover the curriculum runner; `🪟️presentation/🧪️tests/` covers the presentation layer. Not inspected in detail in this pass.

### 5.3 How to run

From any cwd (the tests resolve paths from `__file__`):

```
C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py"
C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🐍️.py"
C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️tests/🐍️.py"
node "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️tests/🟨️.cjs"
node "C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🟨️.cjs"
```

The `.cjs` files `require("ajv/dist/2020")` and resolve it from the file's location upward. I did not confirm that AJV is installed on this host. The BIM `🟨️metadata.mjs` needs `repo tools exec`, which requires the `repo` binary.

---

## 6. Standalone Video And Series Playlist Pattern

### 6.1 Curriculum registry: `MANIFEST` = `ENERGY/🎬️clip/🔣️.json`

Top-level: `{"$schema": "../🏗️authoring/🎬️clip/🧬️schema/🔣️.schema.json", "videos": [ ... ]}`.

Each video (verified contract in `RUNTIME.load_manifest` line 45):

```json
{
  "id": "bim_information",
  "script": "🏛️architecture/🏗️bim/📊️information/🎬️clip/🐍️.py",
  "chapters": [
    {
      "scene": "BIM_01_Information",
      "segments": [
        {"script": "🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py", "scene": "Beat1_DataToInformation"}
      ]
    }
  ],
  "output": "BIM_information"
}
```

Rules enforced by `load_manifest`:

- Video keys must be exactly `id`, `script`, `chapters`, `output`. Ids are distinct.
- `script` is the standalone wrapper (section 6.2). Its path must be relative to the teaching root (`🏛️architecture/...`), use `/` not `\`, be a file, and stay inside the root.
- Chapter keys exactly `scene`, `segments`. Chapter `scene` names are distinct. `segments` are non-empty.
- Segment keys: `script`, `scene` and optional `clauses: {"first": key, "last": key}`. `scene` must be an ASCII identifier. `clauses` must be in narration order and both keys must exist in the scene's `NARRATION` (checked by AST when `source_ids` is validated).
- Scene class name (segment `scene`) must be the `ClipScene` subclass name in `script`.

Optimization videos follow the same shape: `script` `🏛️architecture/🎯️optimization/📉️extrema/🎬️clip/🐍️.py`, scene `Optimization_Extrema` (chapter), segment script `🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py`, scene `Beat1_LocalGlobalExtrema`, output `Optimization_Extrema`.

Current registry, grouped (ids): `physical_fundamentals`, energy topics (`…`), `bim_information`, `bim_objects`, `bim_representations`, `bim_models`, `bim_exchange`, `bim_consistency`, `optimization_extrema`, `optimization_sampling`, `optimization_experiments`, `optimization_index`, `optimization_pareto`, `optimization_algorithms`, plus the energy ones. Full list: grep `"id"` in `MANIFEST`.

### 6.2 Per-video standalone wrapper

Path pattern: `<series-root>/<topic>/🎬️clip/🐍️.py`, where `<series-root>` is for example `🏗️bim` or `🎯️optimization`. Example: `BIM/../📊️information/🎬️clip/🐍️.py` is `TEACH/🏗️bim/📊️information/🎬️clip/🐍️.py`. Its `VIDEO_ID` must equal the manifest id. Template in section 3.3.

Note the mapping: `🏗️bim/<topic>/🎬️clip/🐍️.py` is the standalone (outside the clip folder). The canonical scenes are inside the clip folder: `🏗️bim/🎬️clip/<topic>/🐍️.py`. The optimization folders follow the same split (`🎯️optimization/<topic>/🎬️clip/🐍️.py` standalone, `🎯️optimization/🎬️clip/<topic>/🐍️.py` canonical).

### 6.3 Series playlist entry

- BIM: `BIM/🐍️.py` (31 lines, `VIDEO_IDS` of six ids: `bim_information`, `bim_objects`, `bim_representations`, `bim_models`, `bim_exchange`, `bim_consistency`).
- Optimization: `OPT/🐍️.py` (31 lines, six ids `optimization_extrema`, `optimization_sampling`, `optimization_experiments`, `optimization_index`, `optimization_pareto`, `optimization_algorithms`).

Both call `_runtime.compose_videos(list(VIDEO_IDS), language=..., chapter_scenes=..., **settings)` and `main` calls the render CLI with `--video <id>` for each id.

### 6.4 Render CLI (`RENDER` = `AUTH/🎬️clip/🖌️render/🐍️.py`)

```
repo <script.ts> render --video <id> --language en|de --audience speaker|spectator
```

Operator invocation shown in the READMEs is `repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render …` (the `repo` binary; the `repo` MCP server is not connected in this session, so use the CLI). Direct Python: `python RENDER --video <id> --language en --audience speaker`. Options (argparse, verified): `--language` (required), `--audience` (repeatable, default both), `--video` (repeatable, choices from `MANIFEST`), `--chapter`, `--output-root`, `--jobs`, `--log-output`, `-q/--quality {l,m,h}`, `--resolution w,h`, `--fps`, `--force`, `--concat-only`, `--no-play`, `--play`.

---

## 7. Deltas Versus The Prior Audit (2026-10-27)

- Prior: "MUTED = P_PRIMARY" in BIM. Now `MUTED = PASTEL_GREY` (`BIM drafting` line 22). Fixed.
- Prior: "Share Tech Mono never drawn". Now `Drafting.mono` uses `FONT_MONO` (BIM drafting line 334). Glyph coverage must still be checked per string (`_font_supports`).
- Prior: formula band y −2.80…−1.84, caption band. Confirmed (`FORMULA_BAND`, `CAPTION_BAND`). The prior audit's "caption bottom ≈ −3.6" is not in the code as such; use `CAPTION_BAND`.
- Prior: `layout_conflicts` at `VIS:2460`, `hold_for` at 2203, `caption_bar` at 2034, `swap_caption` at 2084, `set_vo_language` at 2126. All confirmed.
- Prior: `room_section` (452), `facade`, `plan`, `_hatch` in the optimization room. The room module now defines `room_section` as an energy-visuals function (VIS 1100) and the optimization room has its own `room_section` at `OPT/…/🏠️room` (line ~452 in that file; not re-verified this pass).
- Not verified this pass: the `PhysicalFundamentals` and energy topic chapters, `🪟️presentation` internals, `RUNTIME.source_dependencies`.

---

## 8. Open Items For The Grasshopper And Excel Authors

1. Decide the new series root (for example `🏗️grasshopper` and `🏗️excel`, or under `🏗️bim`-like sibling folders). Every path in `MANIFEST` must start with `🏛️architecture/`.
2. Create the series `ClipScene` base in the series clip root (`🎨️visuals/🐍️.py`). Mirror the BIM or optimization pattern: mixins, `language`, `tr`, `number`, `step`, `present`, `source_files()`.
3. Add videos to `MANIFEST` (`ENERGY/🎬️clip/🔣️.json`) with ids, standalone script, chapter scene and segments. Keep keys exactly as the loader expects.
4. Add the standalone wrappers and the series playlist wrapper (section 6).
5. Put `capture.py` and `layout_strict.py` in the ticket folder and change the module prefix in `capture.py`.
6. Add test contracts: `🧪️cases/🔣️.json` (chapters with `folder`, `scene`, `video_id`, `entry`, `title`, `output`), `🧪️tests/🐍️.py` (unittest, `CLIP = Path(__file__).resolve().parent.parent`), and `🧪️tests/🟨️.cjs` if a JSON Schema exists.
7. Confirm whether the energy presentation tests (`AUTH/🎬️clip/🪟️presentation/🔣️.json` and `…/🧪️tests/🔣️.cases.json`) enumerate every video id. They list the BIM and optimization ids, so new ids may need entries there too. Not verified in this pass.
8. Every visible string and every narration clause exists in `en` and `de`; numbers come from the model; no default language.

---

## Verification Log

- Read in full: `BIM/🎨️visuals/🐍️.py` (86 lines), `BIM/🐍️.py`, `OPT/🐍️.py`, `OPT/📉️extrema/🎬️clip/🐍️.py`, `BIM/📊️information/🎬️clip/🐍️.py`, `RUNTIME` lines 1-104, 483-545, 687-842, `RENDER`, prior `capture.py`, prior `agent-rules.md`, prior `toolkit.md`, `OPT/🎨️visuals/🐍️.py`, the head of `BIM/📊️information/🐍️.py`, `MANIFEST` excerpts.
- Grep-verified signatures: `VIS`, `THEME`, `FONTS`, `BIM` drafting, panels, Savoye, axon, analysis; optimization room; `layout_strict.py`; both `🧪️tests/🐍️.py` class lists; both `.cjs` headers.
- Not read in full: `VIS` (2,590 lines), `FONTS` body, `RUNTIME` `source_dependencies`, `🪟️presentation` internals, `analysis` and `savoye` widget bodies, the `OPT` room module body.
