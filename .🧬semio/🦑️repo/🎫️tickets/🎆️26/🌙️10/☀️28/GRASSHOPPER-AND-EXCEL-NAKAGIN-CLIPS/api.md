# API Contract

Each foundation agent owns one section below, implements it, and **replaces its section with the final signatures** when done (keep the heading, mark it `Status: ready`). Scene agents code only against sections marked ready. Additions are welcome; renames must be written here. Module paths follow the BIM pattern and are imported with literal `importlib.import_module("🎓️teaching.🏛️architecture.…🐍️")`.

## 1. Nakagin Teaching Model — `🏢️nakagin/🐍️.py` (owner NAKAGIN-MODEL) · Status: ready

Data `🏢️nakagin/🔣️.json` + `🧬️schema/🔣️.schema.json`. Every leaf is `{"value","unit","basis":"sourced|assumed|derived","source"?,"note"?}`; sources once under `sources`. Names bilingual `{"en","de"}`. Load the module with `importlib.util.spec_from_file_location` (folder name has no importable form beyond the literal `importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`). All geometry in metres, angles in degrees, X east, Y north, Z up, world origin at the centre of Tower A at ground.

```python
load() -> dict ; sources() -> dict ; value(path) -> Value(value, unit, basis, source)   # "capsule.width", "types.A.window_diameter", "energy.u_window"
Plane(origin, x, y, z) ; Circle(center, normal, radius) ; Solid(vertices, faces, edges, window, bolts, meta)  # faces: index loops, outward winding
CapsuleType(key, name, use, width, length, height, corner_radius, corner_segments, window_diameter, window_u, window_v, rotation,
            occupants, fittings[{key,name,width,length,count}], interior_width/length/height, count)   # count derived from slots()
capsule_types() -> tuple[CapsuleType]            # A Residence, B Residence with desk, C Studio (window 1.6), D Splayed residence (window u=-0.45, rotation 5°)
capsule_type(key) ; capsule_solid(t: CapsuleType | dict of overrides + "key") -> Solid
    # local frame: x width centred, y length, z up from 0; back face (bolted) y=0, front face y=length; window Circle on front; bolts 4 points on back; meta profile/bolt_radius
Tower(key, name, index, center(x,y), storeys, capsule_storeys, first_capsule_storey, core_side, shaft_side, storey_height, base_elevation, height,
      capital_height, capital_overhang, capital_taper) ; towers() ; facade_storeys(tower) -> per capsule storey the tuple of facade indices
core_solid(tower) ; capital_solid(tower) ; podium_solid() ; podium_height()         # tower = Tower | index | key
Slot(building, tower, storey, facade, item, type, plane, facade_plane, rotation) ; .path = (b,t,s,f)
slots() -> 140 Slots in tree order ; tree() -> {(b,t,s,f): (Slot item0, Slot item1)} (70 branches)
place(solid, plane) -> Solid ; slot_solid(slot) -> world-placed capsule Solid
program(t) -> {key, net_floor_area, volume, occupants, fittings[{key,name,count,area}], fitting_area, free_area, area_per_person}
envelope(t, variant=None) -> {key, variant, surfaces[{key,name,area,u_value,exposed,factor,ua}] (roof, floor, wall_left, wall_right, front, window, back), window_area,
                exposed_area, attached_area, air_change, volume, h_t, h_v, h}
climate() -> {station, indoor_temperature, design_temperature, heating_base, months[{key,name,mean,days,hdd}], hdd, source (key), period{en,de}, method{en,de}, hdd_value (Value, 1585.7 K·d derived), sources}
variants() -> keys ; variant(name) -> {key, name{en,de}, description{en,de}, overrides{energy key: Value}}   # window_upgrade (u_window 1.1), back_wall_exposed (attached_factor 1.0), shared_roof_floor (u_roof = u_floor = 0)
heating(t, variant=None) -> {key, variant, h_t, h_v, h, load (W), hdd, annual (kWh/a), annual_per_area (kWh/(m²·a)), net_floor_area, count} ; totals() -> {count, area, annual, annual_per_area, per_type, per_tower}
```
Variants: `variant` is a name or a tuple of names applied in order (e.g. `('window_upgrade','shared_roof_floor')`); the data stay untouched. Type A annual demand: base 1295.5, window_upgrade 1233.4, back_wall_exposed 1366.9, shared_roof_floor 876.9 kWh/a.
Facts: slot plane = facade plane (origin on the core facade, x along facade = outward normal × Z, y outward, z up; item 0 left at -1.375 m, item 1 right at +1.375 m and 0.917 m higher) yawed by the type rotation (+ for item 0, − for item 1) with the origin pushed outward by width/2·sin|yaw| so the back face touches the core. Facades 0..3 = N,E,S,W. Tower A (centre 0,0): facades 0,1,2 only (west, towards B and the bridges, has none) × 9 storeys; Tower B (centre -11.1,0): 4 facades × 10 storeys + 3 facades (no W) on storey 10 → 27+43 = 70 branches × 2 = 140. Storey index counts from the first capsule storey (elevation 5.5 m = 2 podium core storeys × 2.75 m). Type per slot = order[(tower + storey + 2·facade + item) mod 4]. Counts A 36, B 35, C 34, D 35.

## 2. Nakagin Drawings — `🏢️nakagin/🎨️visuals/🐍️.py` (owner NAKAGIN-DRAW) · Status: ready

Import `importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")`; mix `NakaginDrawing` into the ClipScene next to BIM `Drafting` (needs `text`, `dimension`, `quantity`, `number`). Every widget is a pure function of its arguments (live: wrap in `self.live(lambda: ...)` or `always_redraw`), returns a `NakaginDrawing3D` (a `VGroup`) with `.view`, `.parts`, `.at(x, y, z)` (model metres to screen) and named parts. `view` is `"axon"` (front-right, az 55 / el 24; windows and +x walls visible), `"axon_back"` (back face and bolts), `"axon_left"`, `"axon_high"`, an `(azimuth, elevation)` tuple, `"plan"`, `"north"|"south"|"east"|"west"` or a BIM `View`. Use the same view and scale for drawings that must align (`plane_axes` + `tower_axon`: pass `at=drawing.at(*plane.origin)`). `at` is the screen position of the drawing's bounding-box centre (capsule: centre of the box; `plane_axes`: the plane origin). Text labels are cached per string, so slider-driven dimension strings cost ~40 ms the first time and ~0 afterwards; geometry is 4-15 ms per capsule, 0.2 s for the full tower, 0.13 s for the elevation.

```python
NakaginDrawing.capsule_axon(t, *, at, scale=0.62, view="axon", window=True, bolts=False, dims=(), highlight=(), hatch=(), base=CAPSULE, segments=None, dim_offset=0.42) -> NakaginDrawing3D
    # t: CapsuleType or dict (keys width, length, height, corner_radius, corner_segments, window_diameter, window_u, window_v, key; missing keys come from type "key")
    # dims ⊂ ("width","length","height","window"); hatch ⊂ ("exposed","attached","front","back","top","bottom","left","right"); highlight: tuple of face names or {face: colour}
    # parts: .body, .faces[name], .window, .bolts (ghosted when the back is hidden), .dims[name], .hatches[tag]
NakaginDrawing.capsule_views(t, *, at, scale=0.58, dims=True, base=CAPSULE, gap=1.0)   # parts plan (front down, hatched walls), front, side; each .dims, .window
NakaginDrawing.plane_axes(plane, *, at, scale=0.62, view="axon", length=2.0, labels=True, square=True)  # .axes["x"|"y"|"z"] red/green/blue, .origin, .square
NakaginDrawing.core_axon(tower, *, at, scale=0.2, view="axon", storeys=None, side=None, storey_height=None)  # live parametric core with storey lines and ground door
NakaginDrawing.capital_axon(tower, *, at, scale=0.2, view="axon") ; NakaginDrawing.podium_axon(*, at, scale=0.1, view="axon")
NakaginDrawing.tower_axon(*, at, scale=0.1, view="axon", lit=None, tint=None, types=True, capital=True, podium=True, core=True, ground=False,
                          where=None, towers=(0, 1), storeys=None, segments=2, base=CAPSULE) -> NakaginDrawing3D
    # lit: None | collection of path prefixes | predicate(key); lit capsules get a yellow outline, the others are dimmed. tint: {prefix: colour} | predicate -> colour | None.
    # key = (b, t, s, f, i); a prefix such as (0, 1, 5, 0) matches the branch {0;1;5;0} (both items). where(slot) filters slots; towers/storeys build the tower up step by step.
    # parts: .capsules[(b,t,s,f,i)] (VGroup), .cores[tower], .capitals[tower], .podium
NakaginDrawing.tower_elevation(*, at, scale=0.1, face="north", lit=None, tint=None, types=True, capital=True, podium=True, where=None, towers=(0, 1), storeys=None, segments=2, base=CAPSULE)
module: make_view(view, at, scale, centre) -> View ; project(point, view) -> np.ndarray ; type_color(key) -> str (A teal, B orange, C green, D red)
        capsule_mesh(w, l, h, r, segments) / box_mesh(...) / solid_mesh(solid) -> Mesh(V, N, faces, names) ; shade_of(base, tone) ; AXIS_X/AXIS_Y/AXIS_Z ; CAPSULE
```

Stills: `T/🗑️generated/nakagin-draw/*.png` from `T/nakagin-draw/showcase.py` (en and de). Capsules and the tower use the model's local frame (x centred, back at y=0, front at y=length, window on the front); planes come from `slots()`.

## 3. Grasshopper Engine — `🦗️grasshopper/🎬️clip/🧱️model/🐍️.py` (owner GH-ENGINE) · Status: ready

Data `🧱️model/🔣️.json` (+ `🧬️schema/🔣️.schema.json`, AJV-validated): `catalogue` (real Grasshopper names, nicknames, category/subcategory, inputs/outputs with type and access `item|list|tree`), `casting` (rule table with `basis` `verified|community|conflicting|unverified`, sources, bilingual explanations), `capsule_wall`, and five `canvases` (`anatomy`, `capsule`, `tower`, `casting`, `trees`). Capsule types, towers and the aggregated tree are **read from `🏢️nakagin`** (slider `bind` paths, literals `{"nakagin": path}`, `{"slot": …}`, `{"capsule": "A"}`, `{"aggregate": true}`); nothing building-specific is typed in the engine. Import with `importlib` like the BIM model (register the module in `sys.modules` before `exec_module`).

```python
# paths and trees
Path = tuple[int, ...]; format_path(p) -> "{0;1;2}"; parse_path("{0;1}") -> (0, 1); format_number(x, language="en") -> "1250.5" | "1250,5"
class Tree:   # ordered branches path -> tuple of items (None = null)
    Tree(dict) ; Tree.from_list(items, path=(0,)) ; Tree.from_item(x) ; Tree.from_dict({"{0;1}": [...]}) ; .to_dict()
    .paths() .branch(p) .branches() .items() .item(p, i, wrap=False) .count .path_count .counts() .stats() -> {path_count,item_count,counts,depths,longest}
    .flatten(path=(0,)) .graft() .simplify(mode="all|front|back") .flip() .explode() .merge(other) .map_items(fn)
    .map(mapping) | .map(source, target)      # Path Mapper, lexical: "{A;B;C;D}(i) -> {A;B;C}", index spec "(D*2+i)", constants item_count path_count path_index, literal mask {0;B}
    Tree.entwine([t1, t2])                    # paths gain the input index at the END: {p;k}  [CHECK in Rhino 8]
panel_rows(tree, paths=True) -> ((path|None, index, text), …) ; describe(value) -> Panel text ; plain(value) -> JSON-friendly (Tree, Point, Plane, Box, Brep volume+tag …)
# values (frozen dataclasses): Point(x,y,z) Vector(x,y,z) Plane(origin,x,y,z) Domain(lo,hi) Circle(plane,radius) Line(a,b) Rectangle(plane,x,y) Curve(shape) Box(plane,x,y,z) Brep(solid, xform, tag)
Plane.from_axes(origin, x, y) ; Plane.world_xy/world_xz/world_yz(origin) ; Box.volume/.center ; Brep.volume() .root_box() .window() .cuts() .fillet_radius()
plane_to_plane(a, b) -> 4x4 (Orient) ; rotation(angle, axis, center) ; translation(v) ; invert(m) ; apply(m, geometry)
make_capsule(width, length, height, corner, window, wall, tag=None, window_u=0.0, window_v=None) -> Brep   # same construction tree as the capsule canvas (box → fillet → window cut)
slot_tree(tower, part) -> Tree  # part yaw|cull|capsule, tree {0;storey}, from Nakagin ; tower_overrides("A"|"B") -> dict for evaluate ; make_capsule_of("A") ; capsule_types() -> tuple[dict] ; capsule_type(key) ; aggregate_tree() -> Tree  # {0;T;S;F} → 2 oriented capsule Breps, tag {"type","slot":(b,t,s,f,i)}; 70 branches, 140 items
# casting
cast(value, target, culture="en") -> Cast(ok, value, message{"en","de"[,"gh"]}, rule, basis)   # targets Number Integer Text Boolean Point Vector Plane Domain Circle Line Curve Box Brep Geometry
    # rule ids "Number>Integer", failures "Text>Number!" / "*>*!"; message["en"] is the real GH text "Data conversion failed from Text to Number", message["de"] keeps it and explains in German
rules() -> {id: {from,to,ok,basis,source,en,de}} ; type_name(value)
# components and canvases
catalogue() -> dict[str, ComponentSpec]   # .name .nickname .kind(component|slider|toggle|valuelist|panel|viewer|param) .category .subcategory .description{en,de} .inputs/.outputs: ParamSpec(name,nickname,type,access,optional,default) ; .input(n) .output(n)
canvas_keys() ; canvas(key) -> Definition  # .objects{id: Obj(id, component, at, settings)} .wires[Wire(src,out,dst,inp)] .views{name: {title{en,de}, objects[≤12], layout{id:[x,y]}}}
    # .obj(id) .spec(id) .wires_into(id, param) .view(name) -> Obj tuple with view-local `at` ; .view_wires(name) ; .bounds(view=None) ; .order()
define(spec) -> Definition                 # canvas-shaped dict (objects, wires) for tests / ad-hoc definitions
evaluate(definition, overrides={}) -> Solution
    # overrides: {"w": 2.8} sets a slider/toggle value, Panel lines, floating-param data or Value List index; {"w.max": 5} sets any setting
    # Solution: .tree(id, output=None) -> Tree ; .items(id, out) ; .value(id, out) ; .state(id) -> "ok|warning|error" ; .msgs(id) -> ({"level","en","de"}, …) ; .trees ; .states
```

Semantics: item access iterates with longest-list matching (shorter list repeats its last item); branches match by equal path, else by position with the last branch repeating; list outputs of an iterated call gain the iteration index in the path when there is more than one iteration; a failed cast leaves a null and the message "Data conversion failed from X to Y" (orange); a required input without valid data is an error with "Input parameter X failed to collect data" (red), and everything downstream turns red; Number→Integer rounds ties to even (2.5→2, 3.5→4; Rutten 2010). Rules the research could not confirm carry `basis: "unverified"` (show "teaching assumption — check in Rhino 8"): Number/Integer→Domain (0 To x), Circle/Line→Curve, Box→Brep, Integer/Number→Boolean, Text→Boolean, Text→Point, Line→Vector, Domain→Number, Text→Number success; `community`: Point→Plane (world XY), Line→Plane (Z along the line), Point→Number (distance).

Canvases (views in `canvas(k).views`, each ≤ 12 objects, left-to-right layout in canvas units, column 260 / row 140):
- `anatomy`: views `slider`, `box` (3 sliders → Center Box → Volume → Panel 25), `param` (floating Number param 2.3), `list` (Series → 0, 2.75 … 11), `tree` (Series with counts "2","3" → `{0;0}`,`{0;1}`, Param Viewer).
- `capsule`: `body`, `round` (Fillet Edge, volume 24.538 m³), `window`, `window-circle`, `bolts` (4 points by list matching), `types` (Value List → List Item), `planes` (4 facade planes by Rotate), `orient` (List Item picks types, Orient onto the planes). Object ids: sliders `w l h r d bu bvl bvh side`, result `cut` (capsule Brep), `bolt`, `rot`, `ori`.
- `tower`: views `core`, `levels`, `facades`, `stagger`, `rotate`, `centre`, `storeys`, `cull` (Cull Index removes the west planes of tower B's top storey), `yaw`, `clearance`, `orient` (Orient the type capsule onto every slot plane, Tree Statistics `stl`/`stc`), `capital`, `podium`. The default is tower B (86 slots: `stl` = `[8]*10 + [6]`); `tower_overrides("A")` turns the same definition into tower A (54, `[6]*9`). Evaluating both reproduces all 140 `🏢️nakagin` `slots()` planes within 1e-9 (final planes = tree `slots`, capsules = tree `ori`).
- `casting`: one view per rule (`text-number`, `storeys` with ties by overriding `sl` 2.5 / 3.5, `deliberate` Floor, `point-plane`, `explicit`, `line-plane`, `domain`, `circle-curve`, `box-brep`, `bool-int`, `int-bool`, `fail` red Center Box, `fix` Value List). Objects `*` source → Param → Panel.
- `trees`: source `src` (140 capsules, `stc` 70 branches), `flatten`, `graft`, `simplify`, `mapper` (per storey: 20 storeys, `[6]*9 + [8]*10 + [6]`, the uneven top storey of tower B), `tower-map` (54 / 86), `flip` (2 × 70), `item`, `mistake` (`cp1`: short colour list repeats, 70 teal / 70 orange), `why`, `fix` (`pt` Path Mapper `{A;B;C;D}(i) -> {A;B}` + `gc` Graft → `cp2`: 54 teal / 86 orange). Custom Preview tags each Brep with `tag["material"]`.

Tests: `🧪️tests/🐍️.py` (57 tests; oracles: independent tree/mapper/matching implementations, NumPy, `rhino3dm` PlaneToPlane/Rotation/BoundingBox/Plane), `🧪️tests/🟨️.cjs` (AJV + 25 mutations + JS tree/mapper/cast oracle), cases in `🧪️cases/🔣️.json`.

## 4. Grasshopper Canvas Toolkit — `🦗️grasshopper/🎬️clip/🎨️visuals/` (owner GH-TOOLKIT) · Status: ready

`🎨️visuals/🐍️.py` — `ClipScene(Drafting, NakaginDrawing, Canvas, manim.Scene)` (no BIM `Panels`: it pulled the BIM model into the render fingerprint): `language()`, `tr(en, de)`, `number(value, digits=2)`, `step(key, *animations, seconds=1.2)`, `present(topic)` (calls `present_<topic>`), `source_files()`; constants `INK MUTED ARCH STRUCT SERV DATA GOOD WARN GLASS CONCRETE GREEN_GROUND MINUS`. Scenes declare `TITLE = {"en","de"}`, `NARRATION = [[key, en, de], ...]`, `construct` → `self.present("topic")`.

`🎨️visuals/🦗️canvas/🐍️.py` — mixin `Canvas` (all methods return a `Widget(VGroup)` with `.body`, `.parts[name]`, `.grip_in(name|index=None)`, `.grip_out(name|index=None)`; grips are invisible anchor mobjects that move with the widget; `at` is the body centre in scene units; `scale` scales the whole widget; private helpers are `_gh_*` so scenes may define `_put`/`_say`):

```python
gh_component(spec, *, at, scale=1, state="ok|warning|error|disabled", nickname=False, selected=False, anchor="c")
    # spec = catalogue key or dict/obj {name, nickname, inputs:[{name,nickname,type}], outputs:[...]}; middle band shows the name (2 lines) or the nickname
    # parts: body, centre, inputs[nick], outputs[nick]; grips by nickname, full name or index
gh_param(type, *, at, label=None, state="ok", selected=False, scale=1, caption=True)           # hexagon capsule, type label below; grips in/out
gh_slider(name, value, lo, hi, *, at, decimals=2, tracker=None, width=None, scale=1, selected=False, unit="")  # tracker: ValueTracker moves knob + value live
gh_panel(lines | Tree | {path: items}, *, at, width=None, paths=True, scale=1, max_lines=14, selected=False, fmt=None, rows=None, max_width=3.6)  # tree mode: {0;1} headers + "0  item" rows; parts["rows"]
gh_toggle(value, *, at, name="Toggle", scale=1) ; gh_value_list(items, selected=0, *, at, name="Value List", is_open=False, scale=1)
gh_wire(out_grip, in_grip, *, kind="item|list|tree", color=None, selected=False)   # grips are anchors (follow movement) or points; item thin, list double, tree dashed double; .curve = sampled bezier
gh_bubble(text, *, level="warning|error|remark", target=widget|mobject|point, side="above|below", width=3.6, scale=1, at=None)
gh_ribbon(tabs=RIBBON_TABS, active="Params", *, at, width=12.6, groups=[(title, [component names])], highlight=None)   # parts: tabs[name], tiles[name]
gh_param_viewer(tree, *, at, scale=1, max_items=8, max_rows=6)       # parts["rows"][path]; grip_in()
gh_group(members, title, *, color=None, pad=0.22, index=0)   # coloured box behind members, title bottom left
gh_background(*, at, width=13, height=4.6, grid=0.5)       # grey canvas with subtle grid
gh_viewport(*, at, width=6, height=4, label="Perspective", active=True)  # .fit(content, margin) scales a NakaginDrawing group inside; .origin; .view (axon View)
gh_canvas(definition, solution, *, view=None, at=(0,0,0), frame=None, scale=1, only=None, gap=(0.45,0.3), max_lines=10, pixels=135.0) -> Widget
    # real engine data: definition = canvas(key), solution = evaluate(definition[, overrides]); view = a name from definition.views (None = whole canvas)
    # objects by catalogue kind: slider -> gh_slider (value from the solution, decimals, integer rounding), toggle, valuelist, param (type from the component name),
    # viewer -> gh_param_viewer(incoming tree, first 5 branches + "+n"), panel -> gh_panel(real tree: {path} headers + indices when path_count > 1, German decimal comma, cut at max_lines), everything else gh_component(spec, state=solution.state)
    # layout: engine columns (x) and order (y); pixels converts engine units, objects in a column are pushed down where they would touch; frame=(w, h) draws the grey sheet and scales content down (never up) to fit, retrying with fewer panel lines until factor >= 0.8
    # wires: solution.tree(src, out) -> item (one value) | list (count > 1) | tree (path_count > 1); grips by param name; bubbles: one per object, worst level (error > warning > remark), text self.tr(msg.en, msg.de), kept clear of other widgets
    # result: .objects{id: widget} .obj(id) .bubbles{id: bubble|None} .wires[list of wire groups, .kind] .factor .sheet
```

Every named view (51 over the 5 canvases) is rendered by `T/gh-toolkit/views.py [en|de] [canvas/view ...]` into `T/🗑️generated/gh-toolkit/views/` with a strict layout report (`report-<lang>.md`); `🧪️tests/🎭️presentation` builds all of them in both languages and asserts frame fit, factor >= 0.75, object/wire counts, wire kinds and bubbles.

Module helpers: `path_text((0,1)) -> "{0;1}"`, `branches(tree|dict|list) -> [(path, items)]`, `bezier(a, b)`, `offset_curve`, `clip_segment`, `semicircle`. Palette constants: `CANVAS FILL OUTLINE WIRE WIRE_SELECTED PANEL_FILL BUBBLE PREVIEW_FILL PREVIEW_EDGE` (use `base=PREVIEW_FILL` in `capsule_axon` for the green preview). Text is `S_SMALL` 16.2 (scale ≥ 1 keeps it ≥ 16 px at 1080p); Grasshopper UI names stay English in both languages, captions and messages go through `self.tr`.
Tests: `🧪️tests/🎭️presentation/🐍️.py` discovers every `ClipScene` under the series folders (bilingual NARRATION, title, 14–24 clauses, step keys exist and are all used, `present_<topic>`, literal imports, native lifecycle en+de); pin a scene in `🔣️.cases.json` `chapters` with `{folder, scene, topic, keys}`. Showcase: `T/gh-toolkit/showcase.py` → `T/🗑️generated/gh-toolkit/*.png`.

## 5. Excel Engine — `📊️excel/🎬️clip/🧱️model/🐍️.py` (owner XL-ENGINE) · Status: ready

Stdlib-only. Load with `importlib.util.spec_from_file_location` (like the BIM model). Data `🧱️model/🔣️.json` + `🧬️schema/🔣️.schema.json` (AJV-validated): workbook `Nakagin.xlsx`, sheets in order **Program, Type A, Type B, Type C, Type D, Climate, Energy** (German tabs: **Programm, Typ A, Typ B, Typ C, Typ D, Klima, Energie**, JSON `title` per sheet; renaming a sheet drops its labels); every input is bound to a `🏢️nakagin` path (`"from": "types.A.window_diameter"`, value + basis copied from it; test asserts equality, `workbook(resolver)` rebinds live); labels bilingual `{"en","de"}`; formulas canonical English with leading `=`; defined names `theta_i theta_e theta_base heating_degree_days attached_factor air_change air_heat_capacity` (Climate!B2:B8) and `u_roof u_floor u_wall u_window` (Energy!B11:B14).

```python
# values: float | str | bool | None | XlError ; errors never raise ; value_data(v) -> JSON ({"error": "#REF!"}, lists for Matrix)
workbook(resolver=None, language="en") -> Workbook ;  # language="de": text cells hold the German labels (Typ A, O for east) so criteria/look-ups match; formulas and sheet names stay canonical. Strings are never plain constants: every text cell is a {en,de} label (`Cell.labels`); Program list ids are formulas; build(data, resolver=None) ; load() -> dict      # Workbook.sheets (Sheet.name/.cells{addr: Cell(value, formula, format, labels)}/.widths), .names, .sheet(n), .order(), .cell(sheet, addr)
evaluate(wb) -> {(sheet, "B4"): value}                    # dependency order, circular cells = 0, multi-cell results spill or #SPILL!
calculate(formula, cells=None, language="en")             # scratch evaluation of one formula (cells: {addr: const | "=formula"})
recalculation_order(wb) / circular_cells(wb) / precedents(wb, (sheet, addr)) / dependents(wb, (sheet, addr)) -> [(sheet, addr)]   # precedents expand ranges and names to cells
references(formula) -> [Ref|Range] ; reference_spans(formula) -> [(start, end, Ref|Range)]   # text offsets for coloured reference frames
Ref(col, row, col_abs, row_abs, sheet, sheet_end) ; Range(start, end, sheet, sheet_end)    # 3D span = sheet..sheet_end ; whole column row=None, whole row col=None
parse_ref("'Type A:Type C'!$B$12") ; format_ref(ref) ; quote_sheet(name) ; cycle_reference("B2") -> "$B$2"   # F4: B2 -> $B$2 -> B$2 -> $B2 -> B2
column_number("AA") ; column_letters(27) ; split_address("B12") -> (2, 12) ; address(2, 12)
translate(formula, "C2", "D4") -> str                     # copy/fill; off grid -> #REF!; names, strings, Table[Col] untouched
tokenize(text, lang="en") ; parse(formula, lang="en") -> Node(kind, value, args).data()   # -2^2 = 4 (neg binds tighter than ^), 2^3^2 left-assoc, % postfix, & , comparisons, union (A1,B2)
localize(formula, "de", wb=None) ; canonicalize(text, "de", wb=None) ; localize_error("#REF!", "de") -> "#BEZUG!"   # de rewrites function names, `;`, decimal comma AND sheet tabs: `=SUMME('Typ A:Typ D'!B13)`, `=Programm!B4`; defined names unchanged; canonical English stays in JSON/engine
tab_name(wb, "Type A", "de") -> "Typ A" ; sheet_titles("de", wb=None) -> {"Program": "Programm", "Type A": "Typ A", ..., "Climate": "Klima", "Energy": "Energie"}   # Sheet.labels = {"en","de"} ; the canonical name (Sheet.name) is the English one and is what evaluate()/apply()/precedents() use
display(value, number_format, language) -> str            # `0.00 "m"`, `0.0 "m²"`, `#,##0 "kWh/a"`, `0.0%`, sections ; 0.5 -> "0.50" / "0,50"; errors, TRUE/WAHR
structured_range(wb, "Capsules[Type]", sheet=None, cell=None) -> Range   # Excel tables: Workbook.tables {name: Table(name, sheet, col, row, headers[{en,de}], nrows)}; Program has table `Capsules` = H1:N141 (canonical English name, German name `Kapseln`: JSON `table.title` {en, de}; headers Capsule/Kapsel, Tower/Turm, Storey/Geschoss, Facade/Fassade, Item/Platz, Type/Typ, Net area/Nettogrundfläche)
  # supported: Capsules[Col] · Capsules[@Col] · [@Col] (inside the table) · Capsules[[#This Row],[Col]] · Capsules[#Headers|#All|#Data] · Capsules[[A]:[B]] · bare `Capsules` (data); unknown table #NAME?, unknown column #REF!, this-row outside the table #VALUE!
  # works in every function (COUNTIF/COUNTIFS/SUMIF/SUMPRODUCT/XLOOKUP/ROWS/COLUMNS/INDEX…), dependency graph (precedents expand to A1 cells), reference_spans(formula, wb, sheet, cell) / references(...) resolve them to Ranges for frames
  # localize de: table name (Capsules to Kapseln, also bare `Kapseln`), column headers (Kapseln[Nettogrundfläche], Kapseln[Typ]), specials (#Kopfzeilen #Alle #Daten #Diese Zeile) and `;` inside [[…];[…]]; canonicalize is the inverse; structured_range / Workbook.table accept either language's table name and header text; the engine and JSON keep English; German specials unverified in a Microsoft source: check in the German UI
table_headers("de", wb=None) -> {"Type": "Typ", "Net area": "Nettogrundfläche", ...} ; table_titles("de", wb=None) -> {"Capsules": "Kapseln"} ; define-table {name, sheet, ref "A1:D4"} · delete-table {name} events ; insert/delete row/column grow, shrink or shift the table
shown(wb, values, sheet, addr, language) ; formula_text(wb, sheet, addr, language)   # cell text / formula-bar text in the UI language (formula bar uses German tab names); show the tab strip with tab_name(...)
apply(wb, events) -> Workbook                             # pure fold; events (dicts):
  {"type": "set-cell", "sheet", "cell", "value" | "formula", "format"?} · "clear-cell" {sheet, cell}
  "copy" {sheet?, source "B1" | "Data!B1:B2", target "B2:B9" | "Other!B5"}   # tiles/fills, formulas translated
  "paste-values" {sheet, range} · "insert-row"/"delete-row"/"insert-column"/"delete-column" {sheet, index, count=1}
  "rename-sheet" {sheet, name} · "delete-sheet" {sheet} (refs -> #REF!, 3D span shrinks) · "move-sheet" {sheet, index} (3D endpoint moves inwards when it leaves) · "add-sheet" {name, index?}
  "define-name" {name, ref} · "delete-name" {name}
functions: SUM SUMPRODUCT COUNT COUNTA COUNTIF COUNTIFS SUMIF SUMIFS AVERAGE MIN MAX ROUND IF IFERROR XLOOKUP INDEX MATCH PI AND OR ROWS COLUMNS
  German: SUMME SUMMENPRODUKT ANZAHL ANZAHL2 ZÄHLENWENN ZÄHLENWENNS SUMMEWENN SUMMEWENNS MITTELWERT MIN MAX RUNDEN WENN WENNFEHLER XVERWEIS INDEX VERGLEICH PI UND ODER ZEILEN SPALTEN
  Unknown names (SUMM, or English SUM inside German text) -> #NAME? ; Table[Column] tokens are kept verbatim and evaluate to #NAME?
errors: #DIV/0! #REF! #NAME? #VALUE! #N/A #SPILL! #NUM! #NULL! ; German #DIV/0! #BEZUG! #NAME? #WERT! #NV #ÜBERLAUF! #ZAHL! #NULL!   # #ÜBERLAUF! unconfirmed in a Microsoft source: check in the German UI
```

Workbook facts for scenes. **Block layout** (important block A:G × rows 1:16; capsule list in H:N rows 1..141): `Program` A1 title · A3:F3 header, A4:F7 per type (A "Type A".., B =COUNTIF(Capsules[Type],A4) count, C net area per capsule ='Type A'!B9, D total area, E total volume, F share), A8:F8 totals · A10:E10 tower/type header, A11:F12 per tower (COUNTIFS(Capsules[Tower],$A11,Capsules[Type],B$10) with mixed refs; F =SUMIF(Capsules[Tower],$A11,Capsules[Net area])), A13:F13 totals · A14:C15 look-up (A15 input "Type C", XLOOKUP) · A16:B16 SUMPRODUCT total area · list H1:N141 (H id like `A-01-N-1`, I tower, J storey 1-based, K facade N/E/S/W, L item 1/2, M "Type X", N area =XLOOKUP([@Type],$A$4:$A$7,$C$4:$C$7); H id formula from [@Tower] [@Storey] [@Facade] [@Item]). Number formats: totals from 1,000 use `#,##0` style (182,901 en / 182.901 de), areas keep one decimal (`#,##0.0 "m²"` for 1,223.6 m²). `Type X` (identical layout): A1 "Type", B1 name, C1 use · A2:B8 inputs (outer width/length/height, interior width/length/height, window diameter) · B9 net floor area, B10 volume, B11 H_T =SUM(F3:F9), B12 H_V, B13 heating load (W), B14 annual demand (kWh/a), B15 per m², B16 free area · C2:F9 envelope table (C name, D area, E U-value via names, F U·A; row 9 back wall × `attached_factor`) · C11:G16 fittings (width, length, count, area G; G16 total). `Climate`: B2 θi, B3 θe, B4 limit, B5 HDD (=SUMPRODUCT over D3:F14 monthly table), B6 b, B7 n, B8 air heat capacity. `Energy`: A3:F8 per type (count, kWh/a per capsule, total, net area, kWh/(m²·a)), B11:B14 what-if U-values, B16 `=SUM('Type A:Type D'!B13)`.
Key results: counts 36/35/34/35 = 140 · net area 8.74 m² per capsule, 1,223.6 m² total · HDD 1,585.7 K·d · Type A annual 1,295.5 kWh/a (148 kWh/(m²·a)) · building 182,901 kWh/a = 149.5 kWh/(m²·a). Cross-checked against `🏢️nakagin` `totals()`, `heating()`, `envelope()` and recalculated by `formulas` (3D `SUM('A:D'!B13)` is expanded for that oracle only).

## 6. Excel Workbook Toolkit — `📊️excel/🎬️clip/🎨️visuals/` (owner XL-TOOLKIT) · Status: ready

`🎨️visuals/🐍️.py`: `ClipScene(Drafting, NakaginDrawing, Workbook, Scene)` with the BIM interface (`language()`, `tr(en, de)`, `number(v, digits)`, `step(key, *animations, seconds)`, `present(topic)` → `present_<topic>`), constants `INK MUTED ARCH STRUCT SERV DATA GOOD WARN GLASS CONCRETE GREEN_GROUND MINUS`, and `source_files()` (widgets, JSON, Excel model + schema, Nakagin model + visuals, theme). `NARRATION` rows `[key, en, de]`, `TITLE = {"en", "de"}`, `construct()` calls `self.present("<topic>")`.
`🎨️visuals/📗️workbook/🐍️.py` (strings and palette in `🔣️.json`, schema `🧬️schema/🔣️.schema.json`): mixin `Workbook`. `workbook` is an XL-engine `Workbook` (or anything with `xl_view()`, or a view with `sheets`, `file_name`, `formula(sheet, a1)`, `value(sheet, a1)`, `number_format(sheet, a1)`, `column_width(sheet, col)`; optional `style(sheet, a1) -> {bold, fill, align, color}`). Language is `self.language()` unless `language=` is given. Formula strings are always canonical English with a leading `=`; German text comes from the engine's `localize` and `display` (overridable hooks `xl_localize(formula, language)` and `xl_display(value, number_format, language)`).

```python
xl_window(workbook, sheet, *, at=(0,0,0), width=12.4, height=5.1, cols="A:F", rows=(1, 10), selected="A1" | "B2:D4", show="values|formulas",
          language=None, ribbon="Home", editing=None, values=None) -> Window    # values: precomputed engine.evaluate(workbook)
    # Window is a VGroup. Parts: title, ribbon, name_box, formula_bar, headers, frames, overlay, tabs{sheet: group}, tabs_strip, status, cells{a1: group}, tint, active
    # .cell(a1) -> VGroup(box, label) with .text .string ; .range(a1) -> VGroup of cells with .box (invisible Rectangle)
    # .rect(a1) -> (left, top, right, bottom) · .point(a1, anchor="c|l|r|u|d|ul|…") · .visible(a1) · .active_address() · .text_of(a1, show=None) -> (string, align) | None
    # .select(a1, **kw) -> Animation        # border + tint glide; name box, headers, formula bar, status follow
    # .edit(formula | None) -> Window        # enter/leave edit mode: coloured formula in cell + formula bar, one colour frame per reference on this sheet
    # .bind(a1, fn, align="r") / .bind_value(a1, tracker, digits=2, fmt=None) -> live cell text (ValueTracker, re-typeset when the string changes)
    # .recalculate(new_workbook, values=None, **kw) -> Animation   # every changed cell morphs to its new text (engine.apply result); bound cells stay live
xl_ref_frame(window, a1, *, color=None, dashed=False) -> VGroup        # tint + border + 4 handles; dashed = copy marquee; color defaults to REFERENCE_COLORS[0]
xl_fill_handle(window, a1, *, to=None, color=GREEN) -> VGroup          # square at the bottom-right of a1; with to: dashed outline of the fill range + plus cursor (.handle .outline .cursor)
xl_trace(window, src, dst, *, kind="precedent|dependent", color=TRACE) -> VGroup   # blue arrow: dot at src (cell or range) → arrowhead at dst; src "'Program'!C2" on another sheet → dotted black arrow from a sheet icon; .line .head .kind
xl_error(window, a1, code) -> VGroup                                   # green corner triangle + yellow smart tag + tooltip (.triangle .tag .tip); code "#DIV/0!" "#REF!" "#NAME?" "#VALUE!" "#N/A" "#NUM!" "#NULL!" "#SPILL!"; the cell itself shows the localized code through the window
xl_tabs(sheets, active, *, at=(0,0,0), width=None, titles=None) -> VGroup   # .tabs[canonical sheet name] .plus ; titles {name: tab text} e.g. engine.sheet_titles(language, wb) ; compact 16.2 tabs, all 7 Nakagin sheets fit at 12.4 ; active tab white + green underline ; tabs that do not fit are dropped from the left
xl_bar_chart(rows, *, at, width=4.8, height=3.2, unit="", title=None, decimals=1, color="#4472C4") -> VGroup | Live   # rows (label, value[, color]); value = number | ValueTracker | callable → live
xl_formula_text(formula, *, language=None, colors_by_ref=None, at=(0,0,0), size=S_LABEL, ink, caret=False) -> VGroup   # at = left baseline point; .refs = [(token, colour, mobject)] ; same colours as the frames
xl_cell_pop(window, a1, *, at=None, width=5.4) -> VGroup               # card Address · Stored value · Displayed · Formula (coloured) · Number format; parts card, rows{address,stored,shown,formula,format}, leader, cell_ring ; default beside the window when it has room (window width ≈ 7), else over its upper right
```

Module helpers (`📗️workbook`): `column_index`, `column_letters`, `split_address`, `cell_address`, `parse_range`, `split_reference`, `reference_tokens(formula) -> [("text"|"ref", s)]`, `reference_colors(formula) -> {ref: hex}` (blue `#2F6FDE`, red `#D93025`, purple `#8E3FBF`, green `#1E9E4A`, orange, teal, magenta, brown), `error_name(code, language)`, `formula_group`, `EngineView`, `as_view`, `DATA`, `REFERENCE_COLORS`.
Notes: a window uses fixed chrome of 1.84 units; the grid rows share the rest (rows 1–10 at height 5.1 fit the stage band y ∈ [−2.55, 2.55]; ≥ 0.34 per row switches cell text to 19.2 else 16.2). Left-aligned text overflows into empty cells to its right and is cut with an ellipsis; numbers that do not fit show `###`. Narrow windows (< 9 units) drop the Share button and trailing ribbon tabs. The cell pop card and error tooltip are opaque overlays: dim or move the window content they cover. Proof: `T/xl-toolkit.md`, stills `T/🗑️generated/xl-toolkit/`, tests `🧪️tests/🎭️presentation/`. Window tabs are localized automatically through the engine's `tab_name` (`window.tabs` stays keyed by canonical sheet name; no wrapper view needed). `Window.scene` is a weak-reference property, so `FadeIn`/`FadeOut`/`copy.deepcopy` of a window work without scene overrides. Text placement: runs without letters or digits (`-`, `*`, `=`, `,`) are placed by the real baseline, never by their ink bottom (the old `-` → `_` bug); `(` and `)` are drawn with the theme serif face because the body face renders them as square brackets. Overflow: left-aligned text and centred text (error codes such as `#BEZUG!`, `#ÜBERLAUF!`) spread over empty neighbour cells (centred: equally on both sides, on an opaque cell-coloured backing so gridlines do not cross it); centred text that still does not fit is scaled down, never cut; left text that does not fit ends in an ellipsis; numbers show `###`. Formula bar: text that does not fit one line at 19.2 shrinks to 16.2, and if it still does not fit it wraps at operators and separators (never inside a reference) into an expanded bar that grows downward over the grid with Excel's chevron (`up`); coloured tokens span the lines, nothing leaves the window (`window.formula_bar[1].current.lines` lists the lines). A formula edited in a cell that is too long for the window is shown only in the bar. Draw order inside a window is fill, selection tint, gridlines, cell text, headers, reference frames, active border, overlays, chrome, and it survives animation: `select` and `recalculate` return a `Glide` animation whose mobject is the window itself, so the scene never lifts the tint or the texts out of the window (the old AnimationGroup did, which greyed values under the tint).
