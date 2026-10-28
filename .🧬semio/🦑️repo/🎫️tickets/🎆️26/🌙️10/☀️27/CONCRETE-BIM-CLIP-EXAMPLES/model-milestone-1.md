# Model milestone 1: JSON and core functions are in

Files (all exist, all run): `C/🧱️model/🔣️.json` (84 KB, 60 elements, 52 relations), `C/🧱️model/🐍️.py` (core), builder script `T/model-build-json.py`, smoke script `T/model-smoke.py` (`python T/model-smoke.py` prints sizes and examples). The schema file and the submodules (`🔌️exchange`, `🧮️analysis`) follow; the core re-exports `step_excerpt`, `mesh`, `ids_check`, `translate_structure`, `tributary`, `clash_check`, `link_values` as soon as those submodules exist (guarded import), so `import` of the core never breaks.

Import: `importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")`. Every function takes an optional `model` dict (default fresh `load_model()`); nothing mutates its input.

## What exists

| Function | Returns |
| --- | --- |
| `load_model()` | the JSON as a dict |
| `element(key)` | copy of one object (tag or GlobalId) with: `psets` merged from its type; `material` replaced by the resolved material (`{guid, tag, class, name{en,de}, ...}`; a layer set has `layers: [{id, name{en,de}, material, thickness{value,unit,basis}, density{...}, zones}]` and `total_thickness`); `material_tag` |
| `quantity(element_or_key, name)` | plain float: authored attribute, layer id (`"soil"` gives its thickness), derived quantity, type diameter |
| `information(tag, property_id, language)` | `{value, unit, basis, tag, guid, class, storey, property_id, label, meaning}`; ids: `diameter`, `height`, `volume`, `thickness`, `length`, `material`, `GlobalId`, `PredefinedType`, `LoadBearing` / `IsExternal` / `FireRating` (also as `Pset_ColumnCommon.LoadBearing`), `grid.spacing`, `grid.initial_spacing`, `piloti.diameter`, `car.width`, `car.length`, `car.turning_radius` |
| `clearance(model=None, spacing=None)` | see below |
| `car_path(model=None, samples=120)` | see below |
| `building_geometry()` | see below |
| `class_chain(ifc_class)` | `[{"class","adds":[...]}]` IfcRoot first |
| `type_occurrences("TY-PILOTI")` | 25 tags `A1`..`E5` (row letter = y, number = x) |
| `quantities(key)` | per class, see below |
| `relations_of(key)` | `[{guid, tag, class, role:"relating"|"related", name{en,de}, relating:{tag,guid,class}, related:[...], others:[...]}]` |
| `represent(key, kind)` | `{key, guid, kind, identifier, type, primitives, derived}`; keys: pilotis, `W-*`, `SL-*`, `RW-*`, `OP-*` |
| `partial_model(id)` | `{id, schema_id, author, name{en,de}, revision, element_tags, element_guids, counts_by_class}` |
| `apply_event(event)` | new model; `demo_event(name)` returns the authored payloads `sill`, `roof_soil`, `roof_soil_alt` |
| `consistency_issues(model)`, `resolve_link(model, link_id)` | work as soon as `link_values` of the analysis submodule exists |
| `class GlobalId` | `compress_guid(int)`, `decompress_guid(str)`, `global_id(tag)` |

## Key conventions (read this)

- Units: m, m², m³, kN, kN/m². Plan x = west to east (0..21.5), y = south to north (0..19), z up from `±0.00`.
- Tags are unique across everything (pilotis `A1`..`E5`, walls `W-S/E/N/W`, openings `OP-*`, windows `RW-*`, slabs `SL-1` (first floor) and `SL-R` (roof), curtain wall `CW-1`, ramps `RM-1` (ground to first) and `RM-2` (first to roof), flights `RF-1A/1B/2A/2B`, screen `SC-1`, boiler `BL-1`, radiator `RAD-1`, drains `DR-1..4`, downpipes `DP-1..4`, heating pipes `PS-1..3`, spaces `SP-HALL/GARAGE/BOILER/SALON/TERRACE/SOLARIUM`, types `TY-PILOTI/TY-RIBBON/TY-WALL`, materials `MAT-*`, `LS-ROOF`, `LS-WALL`, storeys `ground/first/roof`, `PROJECT`, `SITE`, `BUILDING`). `element()` and `quantities()` accept tag or GlobalId.
- Every authored number is `{value, unit, basis: "sourced"|"assumed", source?}`; `model["notes"]` lists the teaching assumptions (bilingual).
- Storey elevations: ground 0.00, first 3.50, roof 6.70 (assumed); first-floor slab 0.25 (assumed), piloti top 3.25 = 3.50 - 0.25; piloti diameter 0.30 (assumed).
- Walls `W-*`: base 3.50, top 6.50 (roof soffit, 3.00 high), thickness 0.30 from the type's layer set (render 0.03 + brick 0.24 + plaster 0.03). Wall axis is inset 0.15 so the outer face is flush with the box.
- Window/opening: `sill`/`head` are heights above the storey floor (same meaning in events); `sill_z`/`head_z` are absolute elevations; `start`/`end` are distances along the wall axis from `start`; `p0`/`p1` the plan points.

## `building_geometry()` keys

```text
box {length 21.5, width 19.0, height 9.4}; storeys {ground 0.0, first 3.5, roof 6.7}
grid {x[5], y[5], labels_x "1".."5", labels_y "A".."E", spacing 4.75}
pilotis[25] {tag, guid, x, y, radius 0.15, base 0.0, top 3.25}
slabs[2] {tag, guid, kind "FLOOR"|"ROOF", outline[[x,y]], voids[[[x,y]]], void_tags, top, thickness,
          layers? [{id, name{en,de}, thickness, zones, density}] (roof only, bottom to top), total_thickness?, zones? {solarium: polygon}}
walls[4] {tag, guid, start, end, length, thickness, base 3.5, top 6.5,
          openings[1] {tag, guid, window_tag, window_guid, start 1.5, end 20.0, sill 0.9, head 2.0, sill_z 4.4, head_z 5.5, p0, p1}}
curtain_wall {tag "CW-1", guid, path[27 pts, west leg, 24-segment arc, east leg], base 0, top 3.25, radius 3.5, centre [8.5, 12.0]}
ramp {tag "RM-1", guid, width, ramps[2] {tag, guid, predefined_type "HALF_TURN_RAMP", width, flights[2] {tag, guid, ramp, start, end, width, z0, z1, run, slope}, landings[{outline, z}]},
      flights[4] (all, flattened), landings (all), voids {"V-RAMP": polygon, "V-RAMP-R": polygon}}
screens[1] {tag "SC-1", guid, path[28 pts incl. two rounded corners], base 6.7, top 9.4, thickness}
spaces[6] {id (= tag), tag, guid, name{en,de}, outline, storey, area, predefined_type}
services[13] {tag, guid, class, kind "boiler"|"radiator"|"drain"|"downpipe"|"pipe", system "heating"|"rainwater", points[[x,y,z]], size?, diameter?}
```

## `clearance(model=None, spacing=None)`

Pure function; `spacing` recomputes for a hypothetical grid spacing (`clearance(model, 5.0)`; a leading number also works).

```json
{"spacing": 4.75, "grid_spacing": 4.75, "initial_spacing": 5.0, "diameter": 0.3, "clear_width": 4.45, "clear_width_initial": 4.7,
 "car_width": 1.75, "car_length": 4.6, "margin": 1.35, "margin_initial": 1.475, "min_margin": 0.5, "turning_radius": 5.5,
 "curve_radius": 3.5, "wall_clearance": 0.25, "centre_radius": 4.625, "fits": true, "fits_initial": true,
 "gaps": [{"row": "E", "x": 12.5408, "y": 19.0, "between": ["E3","E4"], "clear_left": 0.418, "clear_right": 1.2412, "progress": 0.162177, "half_extent": 1.879}, ...],
 "entry_pair": ["E3","E4"], "min_path_clearance": 0.2366}
```

`margin` = (clear_width - car_width) / 2 per side (square-on). `curve_radius` = turning_radius - car_width - wall_clearance = 3.5 (this is the glass wall radius). `gaps[*].clear_left/right` are the real minimum distances between the oblique car rectangle and the two pilotis along the whole path.

## `car_path(model=None, samples=120)`

```json
{"poses": [[x, y, heading_deg] x 120], "length": 25.28, "entry_pair": ["E3","E4"], "pairs": [["E3","E4"],["D1","D2"],["C1","C2"],["B1","B2"]],
 "crossings": [...same as clearance gaps...], "gap_progress": 0.162177, "gap": {"start": [10.9, 19.0], "end": [15.35, 19.0]},
 "centre_radius": 4.625, "curve_centre": [8.5, 12.0], "min_clearance": 0.2366, "car": {"length": 4.6, "width": 1.75}}
```

Route: start outside the north façade at (13.125, 23.0) heading south (270 deg), enter between pilotis E3/E4, right turn (radius 4.625) to head west north of the glass wall, left turn (radius 4.625, concentric with the glass wall radius 3.5 plus 0.25 clearance plus half the car width) along the curved wall, then south at x = 3.875 down to y = 3.0 under the house. Headings are degrees, 0 = east, 90 = north, 270 = south. The car rectangle is `length` along the heading, `width` across. `gap_progress` is the fraction (0..1) of the path length where the car crosses the row-E line.

## Examples

```python
>>> g = building_geometry(); g["pilotis"][0]
{'tag': 'A1', 'guid': '3KWwk33UXPqR9ieWnIoNgh', 'x': 1.25, 'y': 0.0, 'radius': 0.15, 'base': 0.0, 'top': 3.25}
>>> quantities("B3")   # piloti
{'diameter': 0.3, 'radius': 0.15, 'cross_section_area': 0.0706858, 'height': 3.25, 'volume': 0.229729, 'outer_surface_area': 3.063053}
>>> quantities("TY-PILOTI")   # type total
{'count': 25, 'diameter': 0.3, 'cross_section_area': 0.0706858, 'height': 3.25, 'volume': 5.743224, 'occurrences': [...25 tags]}
>>> quantities("W-S")
{'length': 21.5, 'height': 3.0, 'thickness': 0.3, 'gross_side_area': 64.5, 'opening_area': 20.35, 'net_side_area': 44.15,
 'glazing_share': 0.3155, 'volume': 13.245, 'gross_volume': 19.35}
>>> quantities("SL-1")  {'gross_area': 408.5, 'void_area': 28.08, 'area': 380.42, 'perimeter': 81.0, 'thickness': 0.25, 'volume': 95.105, 'gross_volume': 102.125}
>>> quantities("SL-R")  {... 'area': 270.21, 'thickness': 0.2, 'total_thickness': 0.38}      # concrete 0.20 + waterproofing 0.02 + screed 0.06 + soil 0.10
>>> quantities("TY-WALL")  {'thickness': 0.3, 'layers': [{'id':'render','name':{...},'thickness':0.03}, {'id':'brick',...0.24}, {'id':'plaster',...0.03}]}
>>> quantities("SP-TERRACE")  {'area': 110.21, 'perimeter': ..., 'reference_area': 110.0, 'fit': 0.001909}   # solarium 70.0, fit 0.0
>>> information("B3", "Pset_ColumnCommon.LoadBearing", "de")
{'value': True, 'unit': '', 'basis': 'sourced', 'tag': 'B3', 'guid': '0yoQlyTGDVUBsRS$cAaoWq', 'class': 'IfcColumn', 'storey': 'ground',
 'property_id': 'Pset_ColumnCommon.LoadBearing', 'label': 'Tragend', 'meaning': 'Ob das Bauteil Lasten trägt'}
>>> relations_of("B3")   # contained in 'ground'; defined by type 'TY-PILOTI'; IfcRelConnectsElements B3 -> SL-1 ("Piloti carries the slab")
>>> relations_of("W-S")  # also IfcRelVoidsElement W-S -> OP-S; relations_of("OP-S") adds IfcRelFillsElement OP-S -> RW-S
>>> ev = demo_event("sill")      # {"type":"OpeningResized","object_guid":<RW-S guid>,"sill":0.7,"head":2.0}
>>> apply_event(ev)["revision"]  # 2; quantities("W-S", new)["glazing_share"] -> 0.3729 (was 0.3155); GlobalIds unchanged
>>> apply_event(demo_event("roof_soil"))   # soil 0.10 -> 0.30; revision 2; SL-R metadata revision 2; models architecture and structure revision 2
```

Assumptions and fits (also in `model["notes"]`): terrace 110.21 m² against the sourced 110 m² (+0.2 %), solarium 70.0 m² (exact); enclosed ground floor about 96 m² (the 190 m² of one source is not reproduced); ramp flights are 10.5 m long, rise 1.75 m, slope 1:6 (assumed).

## Division of labour (coordinator message)

Core, JSON, schema and api.md 1-2: me. `🔌️exchange` (`step_excerpt`, `mesh`, `ids_check`) and `🧮️analysis` (`translate_structure`, `tributary`, `clash_check`, `link_values(model, kind)`) are written by the other agents against the JSON: translation spec in `model["translation"]`, rules in `model["rules"]` (`clearance` rule 0.25 m for rainwater pipes near a piloti; `ids` rule), loads in `model["loads"]` + material densities, links in `model["links"]` with `kind` in `slab-load | roof-load | piloti-section` (the JSON `used` objects are filled by `link_values`: `slab-load` {thickness, area_load, total_reaction}, `roof-load` {area_load, total_reaction, reaction_<tag>}, `piloti-section` {diameter, height, cross_section_area}); `consistency_issues`, `resolve_link` and `demo_event` already work end to end (soil 0.10 to 0.30: `stale-link` + 26 `stale-value` entries on `LK-ROOF`, `resolve_link` clears them, revision 2 stays). The deliberate teaching violation is `DP-1` (downpipe at (1.40, 0.35)) next to piloti `A1`: surface distance 0.18 m against the required 0.25 m.
