## 3. Toolkit `C/🎨️visuals/` (ClipScene, drafting, panels, Villa Savoye drawings, analysis views)

### 3.1 Module pattern (read this before adding or changing a widget module)

Every widget module is a folder `C/🎨️visuals/<emoji><name>/🐍️.py` that defines **one mixin class** (plain class, no base besides `object`) whose methods are the widgets. `ClipScene` (`C/🎨️visuals/🐍️.py`) inherits all mixins, so scenes call `self.villa_plan(...)`, `self.class_tree(...)`; nothing is imported by scenes.

| Module | Mixin class | Owner | Contains |
| --- | --- | --- | --- |
| `📐️drafting/🐍️.py` | `Drafting` + free functions | toolkit | palette, line weights, `View`, `Drawing`, `Live`, `segments`, `polyline`, `polygon`, `region`, `hatch`, `dimension`, `level_mark`, `leader`, `badge`, `north_arrow`, `scale_bar`, `label`, `text`, `mono` |
| `🧾️panels/🐍️.py` | `Panels` | toolkit | `class_box`, `class_tree`, `property_table`, `step_panel`, `model_card`, `guid_chip` |
| `🏛️savoye/🐍️.py` | `Savoye` | ORTHO | `villa_plan`, `villa_section`, `villa_elevation`, `piloti_detail`, `bay_detail`, `car`, `swept_path`, `roof_detail`, `slab_strip`, `draw_primitives` |
| `🏛️savoye/🧊️axon/🐍️.py` | `Axon` | AXON | `axon_view`, `axon_point`, `villa_axon`, `element_axon` |
| `🏛️savoye/🧱️core/🐍️.py` | free functions | shared (additions only) | `geometry(model)` cache, `catalogue(model)`, `Selection` (highlight/ghost by tag, IFC class or kind), `wall_frame`, `scan`, `roof_layers`, `layer_color` |
| `🧮️analysis/🐍️.py` | `Analysis` | ANALYSIS-VIEW | `analysis_view`, `load_arrows`, `tributary_view`, `mesh_view` |

Rules every module follows:

1. **Imports** at module top, by `importlib.import_module` like the existing clip modules: drafting (`_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")`), model (`🧱️model.🐍️`, accessed as `_model.fn(...)` at call time so a replaced module works), `manim` as `_engine`, energy art (`_art`) only for palette/`window_frame`/`CHROME_FILL`. Never import `🎨️visuals/🐍️.py` (circular) and never import another clip's visuals.
2. **Colours** come from drafting: `INK MUTED ARCH STRUCT SERV DATA GOOD WARN GLASS CONCRETE GREEN_GROUND GROUND_PLANE BACKGROUND`, `DISCIPLINES[name]`, `mix(a, b, t)`, `shade(colour, factor)`. No hex literals in widgets except through `mix`.
3. **Language**: widgets are methods; strings go through `self.tr(en, de)`; numbers through `self.number(v, digits)` / `self.quantity(v, unit, digits)` (en `1,250.5`, de `1.250,5`, unit after a narrow no-break space). Model names that carry `{en, de}` are read with `self.language()`.
4. **Type ladder** (never off-ladder): `S_SMALL = 16.2`, `S_LABEL = 19.2`, `S_BODY = 21.6`. Line weights `W_CUT 3.2`, `W_EDGE 1.9`, `W_THIN 1.2`, `W_HAIR 0.7`.
5. **Return values**: drawings return `Drawing` (a `VGroup`) with `.view` (a `View`), `.parts[tag]` (one `VGroup` per addressable element, tags are model tags such as `"B3"`, `"W-S"`, `"RW-S"`, `"SL-1"`), `.layers[discipline]`, `.anchors`, `.at("B3")` (centre of a part) and `.at(x, y, z)` (screen position of a model coordinate with the drawing's own projection). Panels return `VGroup`s with documented attributes. Parts are registered in painter's order; a part may be faded, indicated or transformed without redrawing.
6. **Live drawings**: for values that move, wrap the builder with `self.live(lambda: self.villa_xyz(..., model=current_model()), key=lambda: <hashable snapshot>)`; `Live` rebuilds only when `key()` changes (or every frame when `key` is omitted) and is safe for `FadeIn`, `FadeOut`, `Transform` and `Indicate`. Text that follows a number: `self.live_label(lambda: self.quantity(t.get_value(), "m"), point)`.
7. **Cropping and zooming**: every drawing takes `point` (screen position of the drawing's centre, documented per widget) and `scale` (screen units per metre). Zoom by raising `scale` and moving `point`; drawings accept `window=(x0, x1, y0, y1)` in model metres to skip elements outside the crop.
8. **No comments in bodies, docstrings start with an emoji**, no layout-zone tags.

Shared projection: `View(kind, point, scale, centre, *, azimuth, elevation, face, cut, axis)` with `kind` in `plan | axon | elevation | section`. `view(x, y, z)` returns the screen point; `view.depth(x, y, z)` larger is closer; `view.facing(normal)` is positive for faces looking at the viewer. Every drawing exposes its view as `drawing.view`.

### 3.2 ClipScene

`ClipScene` keeps `language()`, `tr(en, de)`, `number(value, digits=2)` (language punctuation, true minus), `step(key, *animations, seconds)` and `present(topic)` (title, caption bootstrap, then `self.present_<topic>()` defined by the scene class, then caption fade). It also provides `quantity(value, unit, digits)`, `label(en, de, point, ...)`, `text(string, point, ...)`, `mono(string, point, ...)`, `live(builder, key)` and `live_label(fn, point, ...)`. Module-level colours of `C/🎨️visuals/🐍️.py`: `INK MUTED ARCH STRUCT SERV DATA GOOD WARN GLASS CONCRETE GREEN_GROUND`. A widget module that does not exist yet contributes nothing (guarded import), any other failure is raised.

### 3.3 Drafting (toolkit, final)

All points are screen coordinates unless stated; `anchor` is a combination of `l r u d` (left edge at point, top edge at point, …) or `c`.

- `label(en, de, point, *, size=S_LABEL, color=INK, width=None, anchor="c")`, `text(string, point, ...)`, `mono(string, point, *, size=S_SMALL, color, anchor)` (bundled Share Tech Mono).
- `dimension(start, end, text=None, *, offset=0.3, color=DATA, size=S_SMALL, text_at=0.5, flip=False)` — extension lines, dimension line with oblique ticks, horizontal label next to the line. Positive `offset` puts the line to the left of `start → end`. Returns a `VGroup` with `.label` and `.line`.
- `level_mark(point, text, *, color, size, length=0.9, side=1)` — inverted triangle on a rule with the elevation text.
- `leader(source, target, text=None, *, color=DATA, dot=True)` — `source` is a mobject or point, `target` a point; returns `VGroup` (+ `.label`).
- `hatch(rings, color, spacing=0.09, angle=45, width, opacity)` — parallel lines clipped to a screen polygon (or polygon with holes, even-odd), one `VMobject`. Free functions in drafting: `hatch_pairs`, `segments(pairs, color, width, opacity)`, `dashes`, `polyline(points, color, width, opacity, closed, dash)`, `polygon(points, fill, opacity, stroke, width)`, `region(rings, ...)` (holes wind against the outer ring), `circle_points`, `signed_area`.
- `badge(text, point, *, color=GOOD, filled=True, anchor)`, `north_arrow(point, size)`, `scale_bar(point, scale, metres, steps)`.
- `live(builder, key=None)`, `live_label(fn, point, ...)`.

### 3.4 Panels (toolkit, final)

- `class_box(name, rows, point, *, color=ARCH, width=3.2, grey=())` → `.header`, `.rows` (list of labels), `.frame`; rows in `grey` are drawn as inherited.
- `class_tree(chains, point, *, highlight=(), width=6.4, expand=None)` — `chains` is one `class_chain(...)` result or a list of them (leaf classes `IfcColumn`, `IfcSlab`, `IfcWall`, `IfcWindow` share the trunk and sit side by side on the last row). Parts: `.chips[class]` (box + name), `.rows[class]` (chip + own attributes), `.attr[(class, attribute)]` (one label per attribute), `.arrows`, and with `expand="IfcColumn"` a full attribute box `.box` with `.box_rows[attribute]` (inherited grey, own bright).
- `property_table(rows, point, *, width=6.2, title=None, basis=True)` — rows `(name, value, unit, basis)` or dicts; numbers are formatted by the language rules, strings pass through; basis column reads `sourced` (green) / `assumed` (orange) (de `belegt` / `angenommen`). `.rows[i].name / .value / .basis`, `.cell(i, "value")`, `.frame`.
- `step_panel(excerpt, point, *, width=6.6, wrap=None, highlight_roles=(), only=None, size=S_SMALL)` — `excerpt` is `step_excerpt(...)` (`{"lines", "spans"}`) or a plain list of lines. Monospace; tokens coloured by role (`guid` yellow, `class` teal, `predefined` orange, `pset`/`property` green, `profile`/`extrusion`/`material` beige, `placement`/`representation` grey). Lines wrap after a comma at `wrap` columns (default: as many as fit `width`; at width 12.4 whole lines stay on one row). `only=[0, 2]` keeps source lines by index, `only=["guid", "class"]` keeps lines containing those roles. `.rows[i]` display rows, `.runs`, `.token(match, role=None, nth=0)` the sub-mobject of one token (`match` is a substring of the source line or its index) so leaders, rings and strike-outs can attach.
- `model_card(model_id, point, *, model=None, width=3.9, thumbnail=True)` → `.header .rev .schema .total .thumbnail .frame .info`.
- `guid_chip(key, point, *, color=DATA, model=None, anchor)` → `.id` (the mono text), `.head`, `.guid`, `.frame`.

### 3.5 Villa drawings, axonometric and analysis (owned by ORTHO, AXON, ANALYSIS-VIEW)

Their final signatures are written by their owners below this line (see `ortho.md`, `axon.md`, `analysis-views.md`); the requested features are collected in this list so nothing is lost:

- **Plan / section / elevation (ORTHO)**: `villa_plan(level, point, scale, *, model, highlight, dims, grid, labels, layers, window)` with services layer; `villa_section(point, scale, *, model, cut, highlight, layers, labels)` with pilotis of the cut row as parts and live roof build-up layers; `villa_elevation(face, point, scale, *, model, highlight)`; `roof_detail(point, scale, *, model)` with layer dimension lines and hatches; `slab_strip(point, scale, *, row, bays, collapse, supports, model)`; `piloti_detail(point, scale, *, model, fill, grow, area_fill, dims)` with parts `elevation section dim_height dim_diameter dim_radius`; `bay_detail(point, scale, *, model, spacing, car_progress)`; `car(point, scale, angle)`; `swept_path(path, width, progress, point, scale)`; `draw_primitives(primitives, point, scale)`; thumbnail mode (small scale, no labels); `.at` and `.parts` everywhere; one highlight colour per tag across all views.
- **Axonometric (AXON)**: `villa_axon(point, scale, *, model, highlight, ghost, disciplines, explode, grid, ...)` with `.layers[discipline]`, `.parts[tag]`, ghost at 12 %, grid with origin marker that does not explode; `axon_point(x, y, z, point, scale, *, discipline, explode, model)`; `element_axon(tag, point, scale, *, model, extrude, opening, window)`.
- **Analysis (ANALYSIS-VIEW)**: `analysis_view(analysis, point, scale, *, projection="plan"|"axon", loads, stale, highlight)` with `.parts[member_id]`, `.nodes`, `.supports`; `load_arrows(values, points, scale, *, colors)`; `tributary_view(tributary, point, scale, *, projection, highlight)` with `.parts[tag]`; `mesh_view(mesh, point, scale, *, view="axon"|"side"|"top", highlight)`.
