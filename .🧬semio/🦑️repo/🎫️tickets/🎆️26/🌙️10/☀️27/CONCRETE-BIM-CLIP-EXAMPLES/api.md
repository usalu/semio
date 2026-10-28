# Shared Interfaces

Contract between the model agent, the toolkit agent and the six scene agents. The model agent owns section 1 and 2, the toolkit agent owns section 3; each updates its section when the implementation is final (signatures, return keys, examples) and appends a line to `T/api-changes.md`. Scenes never compute geometry or numbers themselves; they call these functions.

Units: metres, m², m³, kN, kN/m². Plan coordinates: x along the 21.5 m box length (west → east), y along the 19 m width (south → north), z up from ground floor ±0.00. One teaching orientation: the box is aligned with the compass; the entrance (curved glass wall) faces north, the salon faces south. This simplification is stated in the model JSON.

## 1. Model JSON `C/🧱️model/🔣️.json` (schema-first, `🧬️schema/🔣️.schema.json`, JSON Schema 2020-12, `additionalProperties: false`; final, see also `T/model-milestone-1.md`)

Every authored quantity is `{"value", "unit", "basis": "sourced"|"assumed", "source"?}`. `sourced`: box 21.5 × 19 m, height 9.4 m, grid 4.75 m (initially 5.0 m), roof garden 110 m², solarium 70 m², dates, LoadBearing of the frame (research-villa-savoye); everything else is `assumed` (piloti Ø 0.30 m, storey elevations, slab thickness, wall and roof build-up, ribbon window sill/head, car, loads). Tags are unique across all objects; GlobalId = `global_id(tag)` = `compress(uuid5(NAMESPACE_URL, "semio:villa-savoye:" + tag))`, 22 characters.

| Key | Content |
| --- | --- |
| `version` 2, `languages` ["en","de"], `revision` (int, starts 1), `schema_id` "IFC4X3_ADD2" | |
| `orientation_note`, `notes[{id, text{en,de}}]` | compass simplification; teaching assumptions (piloti count disputed, ribbon windows on all sides, infill walls, ramp slope, areas fit, tributary estimate …) |
| `classes` | `{IfcX: {supertype, adds[attribute names]}}` for every class used (IfcRoot … IfcColumn, IfcWindow, IfcRamp, structural analysis classes); chains follow IFC 4.3 |
| `property_names` | `{id: {name{en,de}, unit, meaning?{en,de}}}` labels and meanings used by `information()` |
| `project`, `site`, `building` | `IfcProject` / `IfcSite` (Poissy, address, lat/lon) / `IfcBuilding` (architects, clients, `years`, `box {length,width,height}`) with `guid`, `tag`, `name` |
| `storeys` | `{guid, tag = id: ground|first|roof, class, name, elevation q}` 0.00 / 3.50 / 6.70 |
| `grid` | `{x[5], y[5], labels_x "1".."5", labels_y "A".."E", spacing q 4.75, initial_spacing q 5.0}`; pilotis flush with the long façades (y = 0 and 19), set back 1.25 m from the short ones |
| `types` | `TY-PILOTI` (`IfcColumnType`, `profile {shape circle, diameter q}`, Pset_ColumnCommon LoadBearing TRUE, IsExternal, FireRating), `TY-RIBBON` (`IfcWindowType`), `TY-WALL` (`IfcWallType`, material `LS-WALL`, Pset_WallCommon LoadBearing FALSE) |
| `materials` | `IfcMaterial` (`density q kN/m³`) and `IfcMaterialLayerSet`: `LS-ROOF` layers bottom to top `concrete 0.20, waterproofing 0.02, screed 0.06, soil 0.10` (each `{id, material, name{en,de}, thickness q, zones}`; zones `planted` and/or `solarium`), `LS-WALL` `render 0.03, brick 0.24, plaster 0.03` |
| `elements` | 60 occurrences, see below |
| `spaces` | 6 `IfcSpace` with `outline` (points, or `{"arc": …}` / `{"arc_of": "CW-1", "reverse": true}` items) and `reference_area q` where sourced |
| `relations` | 52 explicit relations `{guid, tag, class, relating tag, related [tags], name{en,de}}`: `IfcRelAggregates`, `IfcRelContainedInSpatialStructure`, `IfcRelDefinesByType`, `IfcRelVoidsElement`, `IfcRelFillsElement`, `IfcRelAssociatesMaterial`, `IfcRelConnectsElements` (each piloti `relating` → `SL-1` "carries the slab") |
| `models` | `architecture`, `structure`, `services`: `{id, schema_id, author, name{en,de}, revision, element_tags[]}`; slabs and ramps are the same objects (same GlobalIds) in architecture and structure |
| `loads` | `finishes {first q kN/m²}`, `imposed {first, roof_planted, solarium}`; densities are on the materials |
| `car` | `length, width, turning_radius, wall_clearance, min_side_margin` (assumed, 1929 car) |
| `rules` | `clearance[{id, name, a_class, a_system, b_class, required q}]` (rainwater pipe keeps 0.25 m from a piloti), `ids[{id, name, entity, pset, property, cardinality}]` |
| `demo_events` | authored scene payloads: `sill` (OpeningResized RW-S: sill 0.70, head 2.00), `roof_soil` (LayerThicknessChanged SL-R soil 0.30 m), `roof_soil_alt` (soil 0.20 m); read with `demo_event(name)` |
| `translation` | spec for the structural analysis translation: `source_schema`, `target_schema`, `target_domain`, `mesh {x_lines, y_lines "grid", surface_source, node_level}`, `boundary {id fixed, class, en, de, basis}`, `mapping[…]`, `assumptions[{id, en, de, basis}]`, `losses[{applies_to {class, predefined_type?}, property, en, de}]` |
| `links` | `{id, model "analysis", kind "slab-load"|"roof-load"|"piloti-section", object_guid, source_revision, used {…}}`; `used` is produced by `link_values(model, kind)` of the analysis submodule |

Element `{guid, tag, class, predefined_type, name{en,de}, storey, type?, material?, system?, geometry, attributes{name: q}, psets{Pset: {Prop: {value, ifc_type, basis, source?}}}, metadata{author, revision, domains[]}}`. Geometry by class: `IfcColumn {axis, base_level, top_level "soffit:SL-1"}`; `IfcSlab {outline, voids[{tag, outline}], level, zones?}`; `IfcWall {path[2], base_level, top_level "soffit:SL-R"|"overall"}`; `IfcOpeningElement {wall}` with attributes `start/end/sill/head`; `IfcWindow {opening}`; `IfcCurtainWall {centre, south_y, radius_from "car", …}`; `IfcRamp {from_level, to_level, landings[{outline, portion}]}`; `IfcRampFlight {ramp, path, from_level, to_level, portion[2]}`; services `{points[[x,y,z|level]], size?}`. Required elements are all present (25 pilotis, 2 slabs, 4 walls + openings + windows, curtain wall, 2 half-turn ramps with 4 flights, solarium screen, boiler, radiator, 4 roof drains, 4 downpipes, 3 heating pipes).

## 2. Model functions `C/🧱️model/🐍️.py` (stdlib only; every function accepts an optional `model` dict as last argument, defaults to a fresh `load_model()`; nothing mutates its input)

Core (written by the model agent). Details and example outputs: `T/model-milestone-1.md`.

```python
load_model() -> dict
element(key, model=None) -> dict               # tag or GlobalId; psets merged from type; material resolved (layer set: layers[{id,name,thickness q,density q,zones}], total_thickness)
quantity(element_or_key, name, model=None) -> float   # authored attribute, layer id ("soil"), derived quantity, type diameter
information(tag, property_id, language, model=None) -> dict   # {value, unit, basis, tag, guid, class, storey, property_id, label, meaning}
clearance(model=None, spacing=None) -> dict    # spacing override recomputes; {spacing, grid_spacing, initial_spacing, diameter, clear_width, clear_width_initial, car_width, car_length, margin, margin_initial, min_margin, turning_radius, curve_radius, wall_clearance, centre_radius, fits, fits_initial, gaps[…], entry_pair, min_path_clearance}
car_path(model=None, samples=120) -> dict      # {poses[[x,y,heading_deg]], length, entry_pair, pairs, crossings, gap_progress, gap{start,end}, centre_radius, curve_centre, min_clearance, car{length,width}}
building_geometry(model=None) -> dict          # everything the drawings need (metres), keys in T/model-milestone-1.md
class_chain(ifc_class, model=None) -> list     # [{"class","adds"}] IfcRoot first
type_occurrences(type_id, model=None) -> list  # 25 tags
quantities(key, model=None) -> dict            # per class: piloti {diameter, radius, cross_section_area, height, volume, outer_surface_area}; TY-PILOTI {count, volume, …, occurrences}; wall {length, height, thickness, gross_side_area, opening_area, net_side_area, glazing_share, volume, gross_volume}; TY-WALL {thickness, layers}; slab {gross_area, void_area, area, perimeter, thickness, volume, gross_volume, total_thickness?}; window/opening {width, height, area, sill, head}; curtain wall; ramp; flight; space {area, perimeter, reference_area?, fit?}; pipe
relations_of(key, model=None) -> list          # [{guid, tag, class, role, name, relating{tag,guid,class}, related[…], others[…]}]
represent(key, kind, model=None) -> dict       # kind plan|section|elevation|3d|axis|footprint|body: {key, guid, kind, identifier "Axis"|"FootPrint"|"Body", type "Curve2D"|"Curve3D"|"SweptSolid", primitives[…], derived{… cut_z, profile, solid, depth}}
partial_model(model_id, model=None) -> dict    # {id, schema_id, author, name, revision, element_tags, element_guids, counts_by_class}
compress_guid(int) -> str; decompress_guid(str) -> int; global_id(tag) -> str
demo_event(name, model=None) -> dict           # authored payloads: "sill", "roof_soil", "roof_soil_alt"
apply_event(event, model=None) -> dict         # new model (deep copy), revision + 1, GlobalIds unchanged; revisions of touched objects and of the discipline models containing them follow
consistency_issues(model=None) -> list         # [{code, id, object_guid, expected, actual, key?}]
resolve_link(model, link_id) -> dict           # new model, link regenerated from the current model (source_revision = model revision, used = link_values), revision unchanged
```

Events: `{"type":"OpeningResized","object_guid":<window guid>,"sill":m,"head":m}` (heights above the storey floor; revision + 1 on the window, its opening and its wall, plus the architecture model) and `{"type":"LayerThicknessChanged","object_guid":<slab guid>,"layer":"soil"|"concrete"|"waterproofing"|"screed","value":m,"unit":"m"}` (revision + 1 on the slab and on the architecture and structure models). Optional extra keys `basis`, `note` are ignored. Invalid events raise `ValueError`.

`consistency_issues`: for each link, `stale-link` when the revision of the linked object is higher than `source_revision` (`expected` = object revision, `actual` = link revision); `stale-value` for each differing `used` value (`key`, `expected` = value from the current model, `actual` = value the analysis used); `unknown-object` for links or models naming a missing object. A fresh model has no issues. `OpeningResized` produces none (the analysis never used the window); `LayerThicknessChanged` on `SL-R` produces `stale-link` + `stale-value` on `LK-ROOF`.

Link values (`link_values(model, kind)`, analysis submodule; the keys below are the `used` objects stored in the JSON, plain numbers only): `slab-load` → `{thickness, area_load, total_reaction}` (first-floor slab SL-1); `roof-load` → `{area_load (mean), planted_area_load, solarium_area_load, total_reaction, reaction_<tag> for each of the 25 pilotis}` (roof slab SL-R, zoned area load); `piloti-section` → `{diameter, height, cross_section_area}` (type TY-PILOTI). Details: `T/analysis-model.md`.

Submodules (written by other agents, re-exported by the core when present; they receive the model dict and reach the core lazily):

```python
# C/🧱️model/🔌️exchange/🐍️.py  (EXCHANGE-MODEL)
step_excerpt(key, model=None, *, first_id=101, language="en", spatial=True, properties=("LoadBearing","IsExternal")) -> dict
#   {key, guid, class, schema, language, first_id, lines:[str], spans:[{line,start,end,role}], ids:{name:"#n"}, entities:{name:line_index}}
#   key: piloti (IfcColumn), slab (IfcSlab FLOOR/ROOF, rectangular outline) or standard wall (IfcWall); other classes raise ValueError.
#   lines = header (7 lines: ISO-10303-21; HEADER; FILE_DESCRIPTION; FILE_NAME; FILE_SCHEMA(('IFC4X3_ADD2')); ENDSEC; DATA;) + DATA entities + ENDSEC; END-ISO-10303-21;
#   piloti: 22 DATA lines (18 with spatial=False: without IFCCOLUMNTYPE/IFCRELDEFINESBYTYPE/IFCBUILDINGSTOREY/IFCRELCONTAINEDINSPATIALSTRUCTURE).
#   ids names: element placement axis point [refdir] shape representation solid profile [profile_axis profile_point] direction context world_axis
#   world_point material|layerset layer<i> layer_material<i> rel_material prop_<Name> pset rel_pset [type rel_type] storey rel_storey.
#   span roles (string tokens exclude their quotes, `.T.`/`.COLUMN.` keep their dots): number (`#n` before `=`, every DATA line), class (IFCCOLUMN|IFCSLAB|IFCWALL, element line only),
#   guid (element GlobalId only), name, tag, predefined (`.COLUMN.`), placement (element's ObjectPlacement `#n` and the IFCCARTESIANPOINT coordinates),
#   representation (element's Representation `#n`, 'Body', 'SweptSolid'), profile (circle radius; rectangle XDim and YDim), depth (extrusion depth),
#   material (IFCMATERIAL / IFCMATERIALLAYERSET name), pset, property, value (`.T.`, `.F.` or the label), reference (every other `#n` in an attribute list).
#   Names follow `language` (en|de; non-ASCII becomes \X2\00FC\X0\); numbers always STEP real syntax (`0.15`, `3.`, `1.E-05`), never localised.
mesh(key, segments=16, model=None) -> dict               # piloti only: {tag, guid, segments, radius, height, centre, vertices, faces, vertex_count (2n+2), triangle_count (4n),
#   kept[{id: vertices|triangles, en, de}], lost[{id: class|guid|property|material|type, en, de}],
#   layout {bottom:[i..], top:[i..], bottom_centre, top_centre, side_faces:[a,b), top_faces:[a,b), bottom_faces:[a,b)}}; closed, outward-oriented, caps fan-triangulated from the axis point
ids_check(model=None, spec=None, *, source="semantic"|"mesh") -> dict
#   spec: None (= model["rules"]["ids"][0], boolean datatype: IfcColumn Pset_ColumnCommon.LoadBearing required), a string "<Entity> [<Pset>.]<Property> exists|optional|prohibited"
#   (e.g. "IfcWall IsExternal exists"), or a dict {entity, property, pset?, predefined?, datatype? "boolean"|"text"|"number", value?, cardinality? "required"|"optional"|"prohibited", id?, name?}.
#   Returns {spec, source, applicability {entity, predefined, en, de}, requirement {property, pset, cardinality, datatype, en, de}, cardinality {id, en, de},
#   expected (objects of the class in the model), found (objects the source exposes: expected for "semantic", 0 for "mesh"), checked, passed (int), failed:[tags], applicable:[tags], passed_tags:[tags], items:[{tag, ok, value}]}.
#   The check never forces TRUE: a value only has to exist (and match datatype/value when the spec says so).
# C/🧱️model/🧮️analysis/🐍️.py  (ANALYSIS-MODEL)
translate_structure(model=None) -> dict                  # {source_schema, target_schema, target_domain, model_class "IfcStructuralAnalysisModel", source_revision (structure model), mid_surface{z,thickness}, mesh{x_lines,y_lines,columns,rows}, slab{tag,guid,gross_area,void_area,net_area,thickness},
                                                         #  members[{id,kind curve|surface,ifc_class,source_guid,source_tag,nodes,…}] (25 curve + 24 surface), nodes[{id,role slab|base,position}] (35 + 25), shells[{id,nodes[4],thickness,source_guid,area,bounds}] (24),
                                                         #  supports[{node,ifc_class IfcStructuralPointConnection,condition IfcBoundaryNodeCondition,fixity,basis,label{en,de}}] (25), loads{self_weight,finishes,imposed,total,area_load}, point_loads (roof reactions at slab nodes),
                                                         #  mapping, assumptions[{id,en,de,basis}], losses[{source_tag,property,en,de,group}], loss_groups, counts, used{link kind: values}}
tributary(model=None, level="first") -> dict             # {level "first"|"roof", method, slab_tag, area_load (area-weighted mean kN/m²), load_breakdown{name:{value,unit,basis,en,de}} (mean), slab_area, total_area, total_reaction, balanced, cantilever, cuts,
                                                         #  zones[{id "slab" | "planted","solarium", name{en,de}, area, area_load, load_breakdown (exact per zone, soil only in "planted"), share, reaction, outline, outlines, holes, pieces}],
                                                         #  pilotis[{tag,kind interior|edge|corner,row,column,area m²,reaction kN,share,cell[[x,y]…],pieces,zones{zone id:{area,share,reaction,pieces}}}]}  (zoned tributary-area estimate, not FEM)
clash_check(model=None, include_passed=False) -> list   # [{id, a_tag, a_guid, b_tag, b_guid, kind "clearance", rule, required, distance (= clearance), deficit, point[x,y,z], level, status}]; one deliberate violation DP-1 / A1
link_values(model, kind) -> dict                         # see above
```

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

