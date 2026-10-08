# 🧱️ API of `wall-layout` and `curtain-layout` (r4, label `i-walls`)

Consumers: `opening-frames`, `element-solids`, `spaces`, `quantities`, `plan-linework`, `diagnostics`. Paths: `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, leaf dirs `S/🧬️schema/💡️inferences/🧱️wall-layout` (with `🔗️joins/`) and `S/🧬️schema/💡️inferences/🪞️curtain-layout`.
Read the value from `ModelInference.wall_layout[wall_id]` / `ModelInference.curtain_layout[curtain_id]` (both `BTreeMap<String, _>`), or call `compute_wall_layout(&snapshot)` / `compute_curtain_layout(&snapshot)`. Units: metres, square metres, cubic metres, radians. All geometry is plan-view in the building coordinate frame of the snapshot (no building origin or rotation applied); z values are building-relative, the same datum as `storey-levels` (`elevation`, not `absolute_elevation`).

## Conventions (normative)

- **Left / right** are taken along the axis direction (`start` to `end`). A face curve is the axis offset: lines shift parallel, arcs get a concentric arc (radius `r - d * sign(sweep)`, same bulge).
- **Layers** of a wall type run from the **left (interior) face to the right (exterior) face**: `layers[0]` is on the left face.
- **Location line** names where the axis sits: `Center` mid plane (`left = right = t/2`), `Interior` on the left face (`left = 0`, `right = t`), `Exterior` on the right face (`left = t`, `right = 0`), `CoreCenter` on the centre of the `Core` layers (else the `Structure` layers, else mid plane) measured from the left face (`left = c`, `right = t - c`).
- **Bulge** = `tan(sweep / 4)`, positive counter-clockwise (the snapshot `Axis::Arc` and the loop `Vertex.bulge`).
- **Footprint loop** is counter-clockwise: `[right.start, right.end, left.end, left.start]`, each vertex carries the bulge of the edge to the next vertex (the last closes to the first). Side edges carry the face bulge, end edges are straight except where a wall butts against a curved face (then the end edge is that face's arc).

## `WallLayout`

| Field | Type | Meaning |
|---|---|---|
| `base_z`, `top_z`, `height` | f64 | resolved vertical extent (unchanged f1 semantics: `base = storey elevation + base_offset`; `top` from `TopConstraint`: `Unconnected` base + height, `StoreyTop` own storey top + offset, `Storey` target storey elevation + offset) |
| `thickness` | f64 | sum of the layer thicknesses of the wall type (0 when the type is missing) |
| `length` | f64 | centreline (axis) length, arc length for arcs |
| `offset_left`, `offset_right` | f64 | distance axis to left / right face, both `>= 0`, sum = `thickness` |
| `layer_offsets` | Vec<f64> | `layers + 1` signed offsets of the layer interfaces from the axis (left positive), `[0] = offset_left`, `[last] = -offset_right` |
| `left_face`, `right_face` | `FaceCurve {start, end, bulge}` | join-trimmed face curves (zero curve when the wall is degenerate) |
| `left_length`, `right_length` | f64 | exact lengths of the trimmed faces |
| `left_area`, `right_area` | f64 | gross side areas before openings: face length times `height` |
| `side_area` | f64 | centreline length times `height` (f1 value, kept) |
| `footprint` | Vec<`Vertex {point, bulge}`> | the join-trimmed footprint loop, empty when the axis has no length or a face collapses |
| `footprint_area` | f64 | exact area of the loop (circular-segment terms included) |
| `volume` | f64 | `footprint_area * height` (gross, before openings) |
| `joins` | Vec<`WallJoin`> | join graph edges seen from this wall |

`WallJoin {kind, end, other, other_end, point, overlap_area}`; `JoinEnd = Start | End | Along`.

| `kind` | meaning | `end` (on this wall) | `other_end` (on `other`) |
|---|---|---|---|
| `Miter` | axis ends coincide (within 1 micrometre); 2 or more walls form a node and are mitered pairwise around it | `Start` / `End` | `Start` / `End` |
| `Butt` | this wall's end lies on the interior of the axis of `other` and butts against its near face | `Start` / `End` | `Along` |
| `Through` | the mirror of `Butt`: `other` ends against this wall (this wall is not trimmed) | `Along` | `Start` / `End` of `other` |
| `Cross` | both axes cross in their interiors; nothing is trimmed, `overlap_area` is the area both bodies cover | `Along` | `Along` |

Every join appears on both walls (`Miter`/`Miter`, `Butt`/`Through`, `Cross`/`Cross` with equal `point` and `overlap_area`). Order inside `joins`: start-tip contacts, end-tip contacts (each by other id, then other tip), then for every other wall by id its `Through` contacts then its `Cross` crossings. The join graph is therefore the union of the `joins` of all walls; the neighbours of a wall are `{j.other}`.

Rules: only walls of the **same storey** join; an axis end is "on" another axis within `JOIN_TOLERANCE = 1e-6` m; a miter point farther than `MITER_LIMIT = 4` thicknesses from the node falls back to a square end; parallel faces (collinear walls) stay square; a T end that hits several through walls picks the lowest id; zero-length axes have no band. Area law for consumers: the footprints of `Miter`/`Butt`/`Through` pairs never overlap; `sum(footprint_area) - sum over distinct Cross pairs of overlap_area` equals the union area; for equal thickness and a mitered L, `area(a) + area(b) = thickness * (length(a) + length(b))`.

Helpers (pub in the leaf): `segment_of(&Axis) -> BulgeSeg`, `axis_length`, `thickness_of`, `offsets_of(&snapshot, &wall) -> Offsets {left, right, layers}`, `storey_bands(&snapshot, storey) -> BTreeMap<String, joins::Band>`, `top_of(&TopConstraint, base_z, &own, target) -> f64`, `constraint_storeys(storey, &top) -> Vec<String>` (own storey, then the top target), `layout_of(&snapshot, id, &wall, &own, target)`. `joins::{join, footprint, Band, Trim, Joined, Footprint, JOIN_TOLERANCE, MITER_LIMIT}` is pure geometry over bands (no snapshot).

## `CurtainLayout` (field `curtain-layout`, `ModelInference.curtain_layout`)

`{base_z, top_z, height, length, area, u_panels, v_panels, panel_width, panel_height}`: same vertical resolution as walls (`top_of`), `length` = axis length, `area = length * height` (gross, before mullions), `u_panels = max(1, ceil(length / u_spacing))`, `v_panels = max(1, ceil(height / v_spacing))` (1 when the spacing is not positive), `panel_width = length / u_panels`, `panel_height = height / v_panels`. DAG: key `CurtainKey::{Storey, Curtain}`, parents = own storey (+ target storey of a `Storey` top). Curtain walls have no layers and no joins.

## DAG and honesty contract

`LayoutKey::Wall(id)` parents are **only storeys** (own storey, then the `TopConstraint::Storey` target): joined neighbours are never parents (joins are symmetric, parents would form cycles). Their authored axes, locations, types and layers are part of `dep_input`: `{own: {wall, layers}, neighbours: {id: {wall, layers}}}` where the neighbour set is the same function that `compute` uses (`joins::join(...).neighbours`), so a wall that starts or stops joining, or a neighbour edit, changes the dependency hash while an unrelated wall does not. `FIELD_ID = s.bim.model.inference.wall-layout`, `SCHEMA_VERSION = 2`; `reads = [walls, wall_types, storeys, buildings, sites]`. Curtain: `s.bim.model.inference.curtain-layout`, `reads = [curtain_walls, storeys, buildings, sites]`.

## Wire / facets

JSON, TS, GraphQL, proto of `ModelInference` and of the leaf `🟦️.ts` carry `WallLayout`, `WallJoin`, `FaceCurve`, `Vertex`, `Point2`, `JoinEnd`, `JoinKind`, `CurtainLayout`. JSON encoding: enums as bare strings (`"Miter"`), structs as objects, no `Option` anywhere in the layout, so tables round-trip losslessly. Oracle tables: `S/🧫️fixtures/💡️inferences/{🏠️house,🧱️wall-joins}/💡️inference/🧱️wall-layout/🔣️.json` (written by `S/🧪️tests/🪜️infer-bim-1-levels-and-wall-heights/🐍️.py write`).
