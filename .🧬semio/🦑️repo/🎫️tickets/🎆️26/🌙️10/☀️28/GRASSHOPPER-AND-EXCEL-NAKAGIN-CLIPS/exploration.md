# Nakagin Model And Consumer Exploration

Read-only audit on 2026-10-29. Source files were not changed.

## Current Teaching Model

`🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json` is the teaching source; Python derivations and separate visual mesh code sit beside it. Capsule outer dimensions are 2.5 × 4 × 2.5 m with assumed radius 0.25 m. `capsule_solid` extrudes a 20-point XZ profile to 40 vertices: the window is Circle metadata and bolts are four point tuples. It does not construct a boolean window opening, frame or volumetric bolts. A/B/C/D are invented teaching room programs, not archival tower identities. The JSON explicitly calls C Studio and D a 5-degree splayed teaching variant.

Tower centres (0,0) and (-11.1,0), core side 5.5 m, pitch 2.75 m, two podium storeys. Tower A has 9 capsule storeys on N/E/S; B has 11 on all facades except top W. Two items per branch, along-facade pitch 2.75 m, vertical stagger 0.917 m. This produces 140 but is an assumed assignment. Podium is a rectangular 31.4 × 20.7 × 5.5 m box centred (-5.55,0); capital is an assumed 2.75 m high taper with 0.25 m overhang.

## Independent Existing GLB Bounds

Read directly from GLB JSON POSITION accessor minima/maxima. Every selected GLB has no node transforms. Axes are asset axes (Y is vertical in these GLBs); these are bounding extents including attachments, not exact nominal shell dimensions.

| Asset under `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation` | Minimum | Maximum | Extent | Meshes | Vertices |
|---|---|---|---|---|---|
| 🌉️bridge/🧊️bridge.glb | 0, -0.2, 0 | 5.25, 2.477819, 4.8 | 5.25 × 2.677819 × 4.8 | 286 | 4740 |
| 🏛️capitals/▫️square/🧊️capital.glb | -2.75, 0, -2.75 | 2.75, 11.5, 2.75 | 5.5 × 11.5 × 5.5 | 26 | 106 |
| 🏛️capitals/🔘️cylindric/🧊️cylindric-capital.glb | -3.25, -0.01, -3.25 | 3.25, 9.909224, 3.25 | 6.5 × 9.919224 × 6.5 | 5 | 1534 |
| 💊️capsules/↗️slash/🧊️capsule_slash.glb | -1.314313, -0.2137625, -2.123121 | 1.314313, 2.3137624, 2.123121 | 2.628626 × 2.527525 × 4.246242 | 68 | 2196 |
| 💊️capsules/↘️backslash/🧊️capsule_backslash.glb | -1.314313, -0.2137625, -2.123121 | 1.314313, 2.3137624, 2.123121 | 2.628626 × 2.527525 × 4.246242 | 68 | 2170 |
| 💊️capsules/⚡️z/🧊️capsule_z.glb | -1.314313, -0.2137625, -2.123121 | 1.3178923, 2.3137624, 2.123121 | 2.632205 × 2.527525 × 4.246242 | 69 | 2192 |
| 💊️capsules/🅿️p/🧊️capsule_p.glb | -1.314313, -0.2137625, -2.123121 | 1.314313, 2.3137624, 2.123121 | 2.628626 × 2.527525 × 4.246242 | 68 | 2192 |
| 💊️capsules/🐍️s/🧊️capsule_s.glb | -1.3178923, -0.2137625, -2.123121 | 1.314313, 2.3137624, 2.123121 | 2.632205 × 2.527525 × 4.246242 | 69 | 2190 |
| 💊️capsules/📐️l/🧊️capsule_L.glb | -1.314313, -0.2137625, -2.123121 | 1.3178923, 2.3137624, 2.123121 | 2.632205 × 2.527525 × 4.246242 | 69 | 2196 |
| 💊️capsules/🔎️q/🧊️capsule_q.glb | -1.314313, -0.2137625, -2.123121 | 1.314313, 2.3137624, 2.123121 | 2.628626 × 2.527525 × 4.246242 | 68 | 2190 |
| 💊️capsules/🪝️j/🧊️capsule_J.glb | -1.3178923, -0.2137625, -2.123121 | 1.314313, 2.3137624, 2.123121 | 2.632205 × 2.527525 × 4.246242 | 69 | 2194 |
| 🥁️tambours/🌱️first-storey/🧊️tambour_first-storey.glb | -2.75275, -0.0022916666, -2.75275 | 2.75275, 4.585625, 2.75275 | 5.5055 × 4.587917 × 5.5055 | 33 | 136 |
| 🥁️tambours/🏁️last-storey/🧊️tambour_last-storey.glb | -2.75275, -0.0022916666, -2.75275 | 2.75275, 4.585625, 2.75275 | 5.5055 × 4.587917 × 5.5055 | 26 | 102 |
| 🥁️tambours/🏠️single-storey/🧊️tambour_single-storey.glb | -2.75275, -0.0022916666, -2.75275 | 2.75275, 4.585625, 2.75275 | 5.5055 × 4.587917 × 5.5055 | 15 | 82 |
| 🥁️tambours/🧱️standard/🧊️tambour.glb | -2.75275, -0.0022916666, -2.75275 | 2.75275, 4.585625, 2.75275 | 5.5055 × 4.587917 × 5.5055 | 46 | 190 |
| 🧱️bases/⬛️plain-shape/🧊️base.glb | -27.349054, 0, 0 | 4.0467787, 7.5, 20.723515 | 31.395833 × 7.5 × 20.723515 | 271 | 1544 |
| 🧱️bases/🫧️blob/🧊️base_blob.glb | -27.751875, -0.00375, -0.35011855 | 4.3731604, 7.50375, 20.8503 | 32.125035 × 7.5075 × 21.200419 | 94 | 2426 |

The eight capsule assets are design catalog alternatives. Their existence does not establish historical tower membership. Native 3dm full-detail files and 1:200/1:500 files sit alongside them. The repository IFC research recorded six used variants, 154 capsules, 2 cores, 16 tambours, 2 capitals, 3 bridges and one base. That IFC is generated from a kit and cannot independently authenticate historical assembly. Its raw vertical conventions conflict with GLB conversion; explicit coordinate conversion is required.

## All Teaching Python Consumers

### `🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py`

- `_model = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`

### `🎓️teaching/🏛️architecture/🏢️nakagin/🐍️.py`

No direct import line; model/test source.

### `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🐍️.py`

- `spec = importlib.util.spec_from_file_location("architecture_nakagin_model", NAKAGIN / "🐍️.py")`
- `sys.modules["architecture_nakagin_model"] = module`

### `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py`

- `_nakagin = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`
- `_drawing = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")`
- `self.cap = _nakagin.capsule_type("A")`
- `for key, override in _nakagin.variant(name)["overrides"].items():`
- `return {slot.path + (slot.item,): slot.type for slot in _nakagin.slots()}`
- `basis = {"window": _nakagin.value("energy.u_window").basis, "wall": _nakagin.value("energy.u_wall").basis}`
- `months = _nakagin.climate()["months"]`
- `for i, t in enumerate(_nakagin.capsule_types()):`
- `climate = _nakagin.climate()`

### `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🎨️visuals/🐍️.py`

- `_NAKAGIN = "🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️"`
- `def _nakagin_drawing() -> type:`
- `"""🏢 The Nakagin drawing mixin; an empty mixin until 🏢️nakagin/🎨️visuals exists."""`
- `nakagin = clip.parents[1] / "🏢️nakagin"`
- `nakagin / "🐍️.py", nakagin / "🔣️.json", nakagin / "🧬️schema/🔣️.schema.json", nakagin / "🎨️visuals/🐍️.py")`
- `class ClipScene(_drafting.Drafting, _nakagin_drawing(), _workbook.Workbook, _engine.Scene):`

### `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📋️program/🐍️.py`

- `_nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`
- `_draw = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")`
- `self.types = {t.key: t for t in _nk.capsule_types()}`
- `self.slots = _nk.slots()`

### `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py`

- `_nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`
- `_nd = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")`
- `return {(s.building, s.tower, s.storey, s.facade, s.item) for s in _nk.slots() if s.type == key and (tower is None or s.tower == tower)}`
- `index = next(t.index for t in _nk.towers() if t.key == tower_key)`

### `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py`

- `_nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`
- `_draw = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")`
- `ctype = _nk.capsule_type("A")`
- `typemap = {(s.tower, s.storey, s.facade, s.item): s.type for s in _nk.slots()}`
- `one = next(slot for slot in _nk.slots() if slot.type == "A")`
- `chip = self._chip(self.tr("capsules in the building: ", "Kapseln im Gebäude: ") + self.number(len(_nk.slots()), 0), (0.0, -2.42, 0.0))`

### `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧪️tests/🎭️presentation/🐍️.py`

- `def test_window_over_the_nakagin_workbook(self):`

### `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧪️tests/🐍️.py`

- `NAKAGIN = ROOT / "🎓️teaching/🏛️architecture/🏢️nakagin/🐍️.py"`
- `cls.nk = load_module("nakagin_model_for_excel", NAKAGIN)`

### `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🐍️.py`

No direct import line; model/test source.

### `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py`

- `_nakagin = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`
- `slots = _nakagin.slots()`
- `towers = {tower.index: tower for tower in _nakagin.towers()}`

### `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🎨️visuals/🐍️.py`

- `_nakagin = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")`
- `nakagin = root.parents[1] / "🏢️nakagin"`
- `nakagin / "🐍️.py", nakagin / "🔣️.json", nakagin / "🧬️schema/🔣️.schema.json", nakagin / "🎨️visuals/🐍️.py")`
- `class ClipScene(_drafting.Drafting, _nakagin.NakaginDrawing, _canvas.Canvas, _engine.Scene):`

### `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py`

- `_nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`
- `_nkd = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")`
- `PARAM_COLOR = {"w": _nkd.AXIS_X, "l": _nkd.AXIS_Y, "h": _nkd.AXIS_Z, "r": "#9B6FD6", "d": "#E08A1E", "wu": "#14A3A8"}`
- `group.add(_draft.segments(clip([(view(-reach, 0, 0), view(reach, 0, 0))]), _nkd.AXIS_X, 0.3, 0.6), _draft.segments(clip([(view(0, -reach, 0), view(0, reach, 0))]), _nkd.AXIS_Y, 0.3, 0.6))`
- `v = _nkd.make_view(view, origin, SCALE, (0, 0, 0))`
- `axes = self.plane_axes(_nk.Plane((0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)), at=origin, scale=SCALE, view=view, length=1.7, labels=rv["body"] < 0.5)`
- `lifted = self.plane_axes(_nk.Plane(centre, (1, 0, 0), (0, 1, 0), (0, 0, 1)), at=v(*centre), scale=SCALE, view=view, length=1.1, labels=False, square=False)`
- `plane = self.plane_axes(_nk.Plane((u, l, vv), (1, 0, 0), (0, 0, 1), (0, -1, 0)), at=centre, scale=SCALE, view=view, length=1.3, labels=True, square=False)`
- `t = _nk.capsule_type(type_key)`
- `drawing = self.capsule_axon({"key": type_key}, at=core.at(*centre), scale=SCALE_WRAP, view=(AZIMUTH - yaw, ELEVATION), base=color or _nkd.type_color(type_key))`
- `plane = _nk.Plane(tuple(self._plane_a["origin"]), (1, 0, 0), (0, 1, 0), (0, 0, 1))`
- `plane = _nk.Plane(tuple(target.origin.t), tuple(target.x.t), tuple(target.y.t), tuple(target.z.t))`
- `strip.add(_fade(self.capsule_axon({"key": pick.tag["type"]}, at=(x, y, 0), scale=0.13, view=view, base=_nkd.type_color(pick.tag["type"])), wv["caps"]))`
- `group.add(self.plane_axes(_nk.Plane((0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)), at=(x - 0.05, y - 0.12, 0), scale=0.16, length=1.5, labels=False, square=False))`
- `types = _nk.capsule_types()`
- `cells = _engine.VGroup(_engine.Rectangle(width=width - 0.2, height=pitch - 0.04, stroke_width=0, fill_color=_nkd.type_color(t.key), fill_opacity=0.0).move_to((at[0], y, 0)))`
- `cells.add(_engine.Dot((xs[0] - 0.72, y, 0), radius=0.08, color=_nkd.type_color(t.key)))`
- `types = _nk.capsule_types()`
- `slots = _nk.slots()`
- `tower_b = _nk.towers()[1]`

### `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py`

- `_nak = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`
- `_nakd = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")`

### `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py`

- `_nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`
- `shape, diameter = {"width": x, "length": y, "height": z, "corner_radius": 0.0, "key": "A"}, _nk.capsule_type("A").window_diameter`
- `shape = _nk.capsule_type(state["type"])`
- `slots = {(c.building, c.tower, c.storey, c.facade, c.item): c for c in _nk.slots()}`
- `t = _nk.capsule_type(state["type"])`
- `slots_n = len(_nk.slots())`
- `kinds = _nk.capsule_types()`
- `names = [f"{self.tr('Type', 'Typ')} {t.key}" for t in _nk.capsule_types()]`

### `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🐍️.py`

- `def test_bound_sliders_read_the_nakagin_model(self):`
- `nak = GH._nakagin()`
- `def test_aggregated_tree_equals_nakagin_slots_by_an_independent_walk(self):`
- `nak = GH._nakagin()`
- `raw = json.loads((CLIP.parents[1] / "🏢️nakagin/🔣️.json").read_text(encoding="utf-8"))`
- `def test_capsule_types_match_nakagin(self):`
- `nak = GH._nakagin()`
- `def test_canvas_reproduces_every_nakagin_slot_plane(self):`
- `nak = GH._nakagin()`
- `nak = GH._nakagin()`

### `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🐍️.py`

- `NAKAGIN = File(__file__).resolve().parents[3] / "🏢️nakagin" / "🐍️.py"`
- `if k == "nakagin":`
- `return nakagin_value(v)`
- `def _nakagin():`
- `spec = importlib.util.spec_from_file_location("nakagin_model_for_grasshopper", NAKAGIN)`
- `def nakagin_value(path: str):`
- `v = _nakagin().value(path).value`
- `for t in _nakagin().capsule_types())`
- `slot = next(s for s in _nakagin().slots() if (s.building, s.tower, s.storey, s.facade, s.item) == tuple(index))`
- `cx, cy = _nakagin().towers()[slot.tower].center`
- `for path, slots in _nakagin().tree().items():`
- `return next(t for t in _nakagin().towers() if t.key == key)`
- `nak, t = _nakagin(), _tower(tower)`
- `nak, t = _nakagin(), _tower(tower)`
- `out["value"] = nakagin_value(out["bind"])`
- `out["items"] = [{**i, "value": nakagin_value(i["bind"])} if "bind" in i else i for i in out["items"]]`
- `return format_number(nakagin_value(v) if k == "nakagin" else _model()[v]["value"])`

### `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py`

- `_nakagin = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`
- `_draw = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")`
- `return _nakagin.Plane(origin=engine_plane.origin.t, x=engine_plane.x.t, y=engine_plane.y.t, z=engine_plane.z.t)`
- `tower = _nakagin.towers()[1]`
- `"""💊 Capsule type A in the viewport with the width taken from the nakagin model."""`
- `slot = next(s for s in _nakagin.slots() if (s.building, s.tower, s.storey, s.facade, s.item) == SLOT)`
- `cap = _draw._cap(_nakagin.capsule_type(slot.type))`
- `moving = _nakagin.Plane(origin=slot.plane.origin, x=(math.cos(theta), math.sin(theta), 0.0), y=(-math.sin(theta), math.cos(theta), 0.0), z=(0.0, 0.0, 1.0))`
- `slot = next(s for s in _nakagin.slots() if (s.building, s.tower, s.storey, s.facade, s.item) == SLOT)`

## Scope Implications

Non-Grasshopper clips using Nakagin are Excel anatomy, references, program and energy. They inherit shared drawings and invented A/B/C/D program data. Excel program and energy use model types/slots, so changing canonical shapes/type identities requires updating workbook derivations, narration, fixtures and both Python/JS expectations. Excel visuals has a dynamic Nakagin mixin loader. No other teaching Python consumers were found by recursively searching all .py files under 🎓️teaching. Read-only search includes names and source content, not compiled/rendered outputs.

## Canonical Puzzle Assembly Audit

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio`. This is a DSL scene, not JSON. Catalog names are misleading: objects are labelled “Capsule With Balcony” but reference the plain capsule GLBs. Base ports c0 and c1 both incorrectly advertise (-7.5,-7.7,7.5); do not use that catalog port duplication as an actual placement oracle. Tambour bottom/top are z=0.9166667 and 3.6666667, giving 2.75 pitch although geometry occupies 4.5879 m vertically. Last tambour roof top z=4.5833335. Capsule door coordinates include back x=±0.45,y=-2.1 and side x=±1.3,y=-1.25; these match the detailed asset local-frame convention rather than teaching back y=0.

Parsed all 180 object-kind rows (quoted and unquoted identifiers accepted):

- Capsule With Balcony J: 33
- Capsule With Balcony Backslash: 31
- Capsule With Balcony L: 26
- Capsule With Balcony Slash: 40
- Capsule With Balcony P: 11
- Capsule With Balcony S: 13
- First Storey Tambour: 2
- Tambour: 16
- Bridge: 3
- Capital: 2
- Last Storey Tambour: 2
- Base: 1

Exact structural object origins and orientation quaternions in scene Z-up frame:

| Label | Kind | Origin | Quaternion |
|---|---|---|---|
| First Storey Tambour · t_f0_b_c1 | First Storey Tambour | -18.6, -7.699999999999999, 7.500000000000001 | 0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,-0.7071067811865475 |
| Tambour · t_f7_b_c0 | Tambour | -7.5, -7.699999999999998, 26.75 | -0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,0.7071067811865475 |
| Tambour · t_f5_b_c1 | Tambour | -18.6, -7.699999999999999, 21.25 | 0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,-0.7071067811865475 |
| Bridge · br_sl0_d2_t_f2_b_c0 | Bridge | -15.85, -8.1, 13.2 | 0,0,0,1 |
| Tambour · t_f1_b_c1 | Tambour | -18.6, -7.699999999999999, 10.25 | 0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,-0.7071067811865475 |
| Tambour · t_f9_b_c1 | Tambour | -18.6, -7.699999999999998, 32.25 | 0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,-0.7071067811865475 |
| Tambour · t_f1_b_c0 | Tambour | -7.5, -7.699999999999999, 10.25 | -0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,0.7071067811865475 |
| First Storey Tambour · t_f0_b_c0 | First Storey Tambour | -7.5, -7.699999999999999, 7.500000000000001 | -0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,0.7071067811865475 |
| Bridge · br_sl0_d2_t_f5_b_c0 | Bridge | -15.85, -8.099999999999998, 21.45 | 0,0,0,1 |
| Tambour · t_f3_b_c0 | Tambour | -7.5, -7.699999999999999, 15.75 | -0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,0.7071067811865475 |
| Tambour · t_f6_b_c0 | Tambour | -7.5, -7.699999999999998, 24 | -0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,0.7071067811865475 |
| Capital · ci_t_f8_b_c0 | Capital | -7.5, -7.699999999999998, 34.0833 | 0,0,0,1 |
| Last Storey Tambour · t_f10_b_c1 | Last Storey Tambour | -18.6, -7.699999999999998, 35 | 0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,-0.7071067811865475 |
| Tambour · t_f8_b_c1 | Tambour | -18.6, -7.699999999999998, 29.5 | 0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,-0.7071067811865475 |
| Bridge · br_sl0_d2_t_f8_b_c0 | Bridge | -15.85, -8.099999999999998, 29.7 | 0,0,0,1 |
| Tambour · t_f7_b_c1 | Tambour | -18.6, -7.699999999999998, 26.75 | 0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,-0.7071067811865475 |
| Base · b | Base | 0, 0, 0 | 0,0,0,1 |
| Tambour · t_f2_b_c0 | Tambour | -7.5, -7.699999999999999, 13 | -0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,0.7071067811865475 |
| Tambour · t_f4_b_c0 | Tambour | -7.5, -7.699999999999999, 18.5 | -0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,0.7071067811865475 |
| Tambour · t_f6_b_c1 | Tambour | -18.6, -7.699999999999998, 24 | 0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,-0.7071067811865475 |
| Tambour · t_f5_b_c0 | Tambour | -7.5, -7.699999999999999, 21.25 | -0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,0.7071067811865475 |
| Tambour · t_f2_b_c1 | Tambour | -18.6, -7.699999999999999, 13 | 0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,-0.7071067811865475 |
| Capital · ci_t_f10_b_c1 | Capital | -18.6, -7.699999999999997, 39.5833 | 0,0,0,1 |
| Tambour · t_f3_b_c1 | Tambour | -18.6, -7.699999999999999, 15.75 | 0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,-0.7071067811865475 |
| Tambour · t_f4_b_c1 | Tambour | -18.6, -7.699999999999999, 18.5 | 0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,-0.7071067811865475 |
| Last Storey Tambour · t_f8_b_c0 | Last Storey Tambour | -7.5, -7.699999999999998, 29.5 | -0.00000000000000005551115123125783,0.00000000000000005551115123125783,0.7071067811865475,0.7071067811865475 |

GLB bounds need axis conversion before applying DSL transforms. This canonical fixture has 154 capsules; it is an existing design example, not historical evidence. The independently researched older IFC has 154. Do not confuse these differing inventories or substitute either for archival assembly research.

## Ongoing Stale Consumer Audit

Audit snapshot before execution changes. These are exact current line numbers and will move as sources change. Counts must be derived from revised model per tower rather than blindly swapped. Narration claiming “the number the building really has” must distinguish schematic teaching definition from surveyed assembly.

| File and line | Existing text | Suggested update |
|---|---|---|
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py:87` | ["pertower", "Mapping to {A;B} gives one branch per tower: 54 capsules in tower A, 86 in tower B.", | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py:88` | "Das Abbilden auf {A;B} ergibt einen Ast je Turm: 54 Kapseln in Turm A, 86 in Turm B."], | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py:99` | ["result", "With matching paths, each tower's colour reaches all its capsules: 54 teal in tower A, 86 orange in tower B.", | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py:100` | "Mit passenden Pfaden erreicht die Farbe jedes Turms alle seine Kapseln: 54 türkise in Turm A, 86 orange in Turm B."], | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py:82` | ["fix", "Cull Index removes exactly those two planes, indices 6 and 7. Now the definition gives 86, the number the building really has.", | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py:83` | "Cull Index entfernt genau diese zwei Ebenen, die Indizes 6 und 7. Nun liefert die Definition 86, die Zahl des Gebäudes."], | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🐍️.py:659` | self.assertEqual(sol.value("pts", "Length"), 54) | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🐍️.py:677` | self.assertEqual(sum(len(GH.evaluate(GH.canvas("tower"), GH.tower_overrides(k)).items("slots", "Geometry")) for k in "AB"), 54 + 86) | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/README.md:10` | - **Four capsule types** as parameter variants of that one capsule: A Residence, B Residence with desk, C Studio (window ⌀ 1.6 m), D Splayed residence (side window, 5° yaw on its plane). Counts per type are derived from a modular pattern in the data (A 36, B 35, C 34, D 35). | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/README.md:45` | - **Vier Kapseltypen** als Parametervarianten dieser einen Kapsel: A Wohnkapsel, B Wohnkapsel mit Schreibtisch, C Atelier (Fenster ⌀ 1,6 m), D Gefächerte Wohnkapsel (Seitenfenster, 5° Drehung auf ihrer Ebene). Die Anzahl je Typ wird aus einem Modulo-Muster in den Daten abgeleitet (A 36, B 35, C 34, D 35). | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json:403` | "en": "Studio", | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json:404` | "de": "Atelier" | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json:407` | "en": "Small studio for two with large window, no bed", | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json:408` | "de": "Kleines Atelier für zwei mit grossem Fenster, ohne Bett" | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json:529` | "en": "Splayed residence", | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json:530` | "de": "Gefächerte Wohnkapsel" | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json:533` | "en": "Residence with side window, turned 5 degrees on its plane (teaching variant)", | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json:534` | "de": "Wohnkapsel mit seitlichem Fenster, 5 Grad auf ihrer Ebene gedreht (Lehrvariante)" | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🔣️.json:8` | "A": 54, | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🔣️.json:9` | "B": 86 | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/README.md:13` | \| grasshopper_tower \| The core with square plan, storey height and storey count, a series of planes for every storey and facade, capsule orientation with Orient and List Item, the capital on top of each core and the lower building as simple massing; counts read from the definition (A 9 × 3 × 2 = 54, B 11 × 4 × 2 = 88 minus 2 culled planes = 86), including the typical mistake of a forgotten Cull In | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/README.md:75` | \| grasshopper_tower \| Der Kern mit quadratischem Grundriss, Geschosshöhe und Geschosszahl, eine Serie von Ebenen für jedes Geschoss und jede Fassade, Kapselausrichtung mit Orient und List Item, das Kapitell auf jedem Kern und das Sockelgebäude als einfache Kubatur; Anzahlen aus der Definition gelesen (A 9 × 3 × 2 = 54, B 11 × 4 × 2 = 88 minus 2 entfernte Ebenen = 86), samt typischem Fehler eines v | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗣️transcript.de.md:145` | Cull Index entfernt genau diese zwei Ebenen, die Indizes 6 und 7. Nun liefert die Definition 86, die Zahl des Gebäudes. | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗣️transcript.de.md:247` | Das Abbilden auf {A;B} ergibt einen Ast je Turm: 54 Kapseln in Turm A, 86 in Turm B. | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗣️transcript.de.md:259` | Mit passenden Pfaden erreicht die Farbe jedes Turms alle seine Kapseln: 54 türkise in Turm A, 86 orange in Turm B. | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗣️transcript.en.md:145` | Cull Index removes exactly those two planes, indices 6 and 7. Now the definition gives 86, the number the building really has. | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗣️transcript.en.md:247` | Mapping to {A;B} gives one branch per tower: 54 capsules in tower A, 86 in tower B. | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗣️transcript.en.md:259` | With matching paths, each tower's colour reaches all its capsules: 54 teal in tower A, 86 orange in tower B. | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️cases/🔣️.json:195` | {"id": "tower-b-capsules", "canvas": "tower", "object": "ori", "output": "Geometry", "items": 86}, | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️cases/🔣️.json:196` | {"id": "tower-a-capsules", "canvas": "tower", "tower": "A", "object": "ori", "output": "Geometry", "items": 54}, | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️cases/🔣️.json:218` | {"id": "trees-mapper-per-tower", "canvas": "trees", "object": "ptl", "tree": {"{0}": [54, 86]}}, | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️cases/🔣️.json:222` | {"id": "trees-fix-materials", "canvas": "trees", "object": "cp2", "output": "Preview", "materials": {"teal": 54, "orange": 86}} | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🎨️visuals/📗️workbook/🔣️.json:94` | "en": "Displayed", | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🔣️.json:2651` | "en": "Studio", | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🔣️.json:2652` | "de": "Atelier" | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🔣️.json:2657` | "en": "Small studio for two with large window, no bed", | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🔣️.json:2658` | "de": "Kleines Atelier für zwei mit grossem Fenster, ohne Bett" | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🔣️.json:3171` | "en": "Splayed residence", | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🔣️.json:3172` | "de": "Gefächerte Wohnkapsel" | Re-derive from revised canonical assembly and synchronize narration, transcripts, fixtures and docs. |
| `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🔣️.json:3177` | "en": "Residence with side window, turned 5 degrees on its plane (teaching variant)", | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |
| `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🔣️.json:3178` | "de": "Wohnkapsel mit seitlichem Fenster, 5 Grad auf ihrer Ebene gedreht (Lehrvariante)" | Replace invented program/type description with canonical sourced capsule variants; preserve teaching programs only as explicitly separate scenarios. |

Excel quiz JSON also contains Studio/Atelier and 54 in many answer choices (for example lines 223-224 and 253-254). Those may be distractors; review question meaning before changing. No direct non-Grasshopper Python hardcoded 54/86 count narrative was found. Most Excel consumers derive types/slots dynamically, so model changes flow into visuals but potentially contradict their workbook schema or question content.

Additional remodel narrative correction: `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py:80-83` describes invented top-west culling as historical fact; lines 88-91 deliberately justify a simple podium extrusion. Both require rewriting with the detailed architectural model scope, not only numerical replacement.

## Revision 2 Execution Audit

Read after model writes at 2026-10-29T12:43Z. Independently summed raw JSON facade exceptions: A11 capsule storeys uses 9×3×2 +2×4×2 =70; B9 uses 8×4×2 +1×3×2 =70. D rotation is now zero. C remains Studio with a 1.6 m window and no-bed program.

- `🏢️nakagin/🐍️.py:631` unconditionally adds bed-base for every full-detail type; line 641 unconditionally adds one desk. This conflicts with C no-bed/two-desk program and type fitting quantities. Either make interior geometry consume program fittings or separate historical canonical geometry from explicitly illustrative programs.
- `🏢️nakagin/🐍️.py:654-663` makes four continuous core walls, no capsule entry apertures. Capsule doors lead directly into concrete. Build door openings from placement/access levels and validate circulation.
- `🏢️nakagin/🐍️.py:736` builds bridge levels3/6/9 without checking capsule collision. B east facade is populated on most corresponding levels and faces A across the bridge gap; validate bridge and capsule bounding/solid intersections before accepting.
- New hardware/shell/fittings are detailed assumed geometry; their presence does not establish documentary correctness. Keep provenance clear in labels and README.

Revision 2 render audit: `🏢️nakagin/🎨️visuals/🐍️.py:557-598` `_tower_items` never consumes connecting bridges from `structure_assembly`, so all tower axonometric/elevation clip views omit the new bridges. Line 585 filters tower capsule parts to shell/back/rim/frame/glazing and omits entry doors, bolts and services. Add bridge render items and suitable exterior hardware LOD to make shared detail visible consistently.

## Follow-Up Verification After Circulation Fixes

Later source snapshot resolves previous continuous-wall and missing-bridge findings: core walls now split around openings and add capsule landings; visuals cache bridge parts and expose drawing.bridges. The JSON makes assumed circulation and proposed A–D energy/program studies explicit. B east centres 3.875 m apart give 1.375 m gap for 1.25 m bridge; nominal lateral clearance is 0.0625 m per side. Earlier collision finding no longer applies to revised placement. Tower exterior LOD still filters out doors/bolts/services; use that deliberate reduction only where full-detail export retains them. Full assembly interior/program agreement remains to be verified independently.

## Authentic Scan And Survey Evidence

The public [GLUON scan](https://gluon.tokyo/nakagin/index_en.html) embeds `models/NakaginCapsuleTower.glb` directly. Downloaded into memory solely for geometric inspection, not saved: 16,527,260 bytes, 329,124 POSITION vertices, 291,056 triangles, a single merged textured mesh without semantic capsule nodes or node transforms. Bounds X[-0.3849164,0.3138013], Y[-0.0101108,1.1445266], Z[-0.2718509,0.1558762]. Asset axes are Y-up. Units/scale are not documented in model metadata; do not infer metre dimensions from its normalized coordinates without calibration. JPEG texture8192 pixels was decoded with existing jpeg-js solely for independent visual inspection. Two opposing elevations show mixed capsule end/long-wall orientations, irregular gaps occupied by bridges, two tall diagonal-cut metal roof enclosures, and a recessed/irregular podium. A regular all-end-window grid is visually false. Exterior scan cannot expose hidden capsule-room identity or per-storey plans by metadata because all surfaces are merged.

The [A606 survey publication announcement](https://readyfor.jp/projects/time_capsule_2072/announcements/241581) establishes A76/B64 and identifies measured floorplans on book pages8–13. Its public preview is linked at https://youtu.be/ff7y7J4CbDs. Full book is400 pages with all-room survey; it is not an online downloadable plan set found in this audit. READYFOR main project images117–120 point to numbered preview assets983867–983870; direct image requests returned HTTP202 empty bodies with an AWS WAF challenge. No bypass attempted.

[Primary A606 skeleton survey report](https://readyfor.jp/projects/time_capsule_2072/announcements/265901),2023-05-07, says original structural drawings and surveyed original capsules disagree in member positions/sections. It identifies A606 as C_R and B-type as a long-wall porthole with short-wall entry. Therefore brochure drawings should be compared with as-built scan, not promoted silently to exact as-built geometry. [Current publisher project](https://capsule1972.com/crowdfunding/) describes six capsule varieties in its parts list; archival eight combinations and six built varieties must not be conflated.

[Shibaura Institute Matsushita Lab survey participation](https://www.paes.shibaura-it.ac.jp/k-matsu/2021/07/14/%E4%B8%AD%E9%8A%80%E3%82%AB%E3%83%97%E3%82%BB%E3%83%AB%E3%80%80%E5%AE%9F%E6%B8%AC%E8%AA%BF%E6%9F%BB/) confirms direct university survey participation but indexed publication contains no complete measured floorplan drawings. [Jusoken library record](https://www.jusoken.or.jp/library/search/view/60610) confirms the400-page survey publication exists in its reference collection. No openly accessible complete measured plan PDF found.

The coordinator obtained a floorchart matching A76/B64: A floors3–13 counts2,7,7,7,8,8,7,8,8,7,7; B floors3–11 counts2,8,8,7,8,8,7,8,8. This supersedes the earlier70/70 model. Suffixes must not be counted as room totals because numbering skips numbers. Counts alone do not establish orientations or position.

## Closest Repository Slot Frames

Original DSL wraps cores with three vertical sublevels per2.75mstorey, not pairedfacades. Four capsules atsl0, twoatsl1,twoatsl2; zoffsets .2,1.1167,2.0333 relativecorebase. The secondcore rotates slotgeometry180°. This compact wrap is why corecentre spacing11.1m can work without opposed4mprojections colliding. Capsule origins are centred localassetorigins, not the new teachingmodel backface.

Existing fixture floorcounts (f0based):

- Core0: 8, 8, 7, 8, 8, 7, 8, 8, 7
- Core1: 8, 8, 7, 8, 8, 7, 8, 8, 7, 8, 8

Exactfirstfloor slotframes (relativecorecentreandbase; worldZup; yawfromquaternion):

| Core | Sublevel | Door | Type | Relativeorigin | Yaw |
|---|---|---|---|---|---|
| 0 | 0 | 0 | Capsule With Balcony Backslash | -1.35, 4.85, 0.2 | 0 |
| 0 | 0 | 1 | Capsule With Balcony L | -4.05, 2.15, 0.2 | 0 |
| 0 | 0 | 2 | Capsule With Balcony J | -4.05, -2.15, 0.2 | 180 |
| 0 | 0 | 3 | Capsule With Balcony Slash | -1.35, -4.85, 0.2 | 180 |
| 0 | 1 | 0 | Capsule With Balcony J | 2.15, -4.05, 1.1167 | 270 |
| 0 | 1 | 1 | Capsule With Balcony Slash | 4.85, -1.35, 1.1167 | 270 |
| 0 | 2 | 0 | Capsule With Balcony J | 4.05, 2.15, 2.0333 | 0 |
| 0 | 2 | 1 | Capsule With Balcony Slash | 1.35, 4.85, 2.0333 | 0 |
| 1 | 0 | 0 | Capsule With Balcony Backslash | 1.35, -4.85, 0.2 | 180 |
| 1 | 0 | 1 | Capsule With Balcony L | 4.05, -2.15, 0.2 | 180 |
| 1 | 0 | 2 | Capsule With Balcony J | 4.05, 2.15, 0.2 | 0 |
| 1 | 0 | 3 | Capsule With Balcony Slash | 1.35, 4.85, 0.2 | 0 |
| 1 | 1 | 0 | Capsule With Balcony J | -2.15, 4.05, 1.1167 | 90 |
| 1 | 1 | 1 | Capsule With Balcony Slash | -4.85, 1.35, 1.1167 | 90 |
| 1 | 2 | 0 | Capsule With Balcony S | -4.05, -2.15, 2.0333 | 180 |
| 1 | 2 | 1 | Capsule With Balcony P | -1.35, -4.85, 2.0333 | 180 |

Use frames only as closest existing designreference and apply genuinelyverified per-flooromissions/orientations; they do not authenticate assembly. Original fixture has154 not140.

## Fixture Floor Alignment Hypothesis

The existing c0 floorcounts8,8,7,8,8,7,8,8,7 become measured B2,8,8,7,8,8,7,8,8 by inserting2capsules at floor3 and droppinglast7capsulegroup at floor12. c1 can become measuredA76 by mappingf0→floor4, droppinglast8capsulegroup(floor14), removingone onfloors4/5/13 and inserting2capsules onfloor3. This mapping aligns fixturemissing-slot bridge intervals withhistoricalfloors6/9/12. It is a useful alignment hypothesis, not proof ofidentity or exact omittedroom.

Fixturemissing slots:

- Core0 floorindex2: sl0d2
- Core0 floorindex5: sl0d2
- Core0 floorindex8: sl0d2
- Core1 floorindex2: sl0d1
- Core1 floorindex5: sl0d1
- Core1 floorindex8: sl0d1

Do not manually invent which singlepositions disappear fromactualfloors4/5/13; inspect plans/photos. The fitting tree should describe sublevels, not falsely callthemfacades:4/2/2 slotsacross3sublevels.

## Measured Native Capsule Porthole Geometry

Independent GLB primitive POSITION accessor bounds identify circular components by two dimensionsnear1.3m and thirdthin dimension. In CADZ-upframe (GLBx,-GLBz,GLBy), slash/backslash/J/L front outercirclecentre(0,+2.123121,1.05),outerdiameter1.314313;glazingcentre(0,+2.0343332,1.05),diameter1.3. P/S longwall outercirclecentre(+1.3,+.95,1.05),normal+X,outerdiameter1.314313;glazingx1.2348763,diameter1.3. Capsule nominalfloor isapproximatelylocalz-.2, so centredwindowheightabovefloor1.25matchesnominalteachingheight.

Doorports fromDSL in sameCADframe: Backslash(+.45,-2.1,0),normal-Y;Slash/P(-.45,-2.1,0),normal-Y;J/S(-1.3,-1.25,0),normal-X;L(+1.3,-1.25,0),normal+X. Thus therearefourdoor/windowwallrelationships acrosssixusedvariants. Allsixgeometryboundsincludehardware; shellnominal2.5×4×2.5isnotidenticaltofullassetbbox.

## Measured Native Capital Wedge

Square capital GLB actual POSITION vertices (CADZ-up):

```json
[
  [
    -2.45,
    2.45,
    0
  ],
  [
    2.45,
    2.45,
    0
  ],
  [
    2.75,
    -2.75,
    0
  ],
  [
    2.45,
    -2.45,
    0
  ],
  [
    -2.45,
    -2.45,
    6.025785
  ],
  [
    -2.45,
    -2.45,
    0
  ],
  [
    2.45,
    -2.45,
    1.546942
  ],
  [
    2.45,
    2.45,
    6.025785
  ],
  [
    -2.45,
    2.45,
    10.901487
  ],
  [
    2.75,
    2.75,
    6
  ],
  [
    2.75,
    2.75,
    0
  ],
  [
    2.75,
    -2.75,
    1
  ],
  [
    -2.75,
    -2.75,
    6
  ],
  [
    -2.75,
    -2.75,
    0
  ],
  [
    -2.75,
    2.75,
    0
  ],
  [
    -2.75,
    2.75,
    11.5
  ]
]
```

Outerbasecorners±2.75. Topheightsat(x,y):(+2.75,+2.75)=6;(+2.75,-2.75)=1;(-2.75,-2.75)=6;(-2.75,+2.75)=11.5. This is an oblique cut with5.5mfall acrossbothplanaxes, not a smalltaperedfrustum. Innerfacesuse±2.45 andcorrespondingoffsetheights. Nativefixturecapitaloriginz34.0833/39.5833; geometryreusemustrespect actualtowerheight/layoutmapping.

## Native Podium Roof Extent

Independent topz7.5POSITIONvertices produce convexhullCADXY

```json
[
  [
    -26.88884,
    -3.93783
  ],
  [
    -20.46338,
    -20.47763
  ],
  [
    -20.42413,
    -20.57867
  ],
  [
    3.6739,
    -10.56773
  ],
  [
    -0.27379,
    -0.4
  ],
  [
    -23.81793,
    -0.4
  ],
  [
    -23.87919,
    -0.47057
  ]
]
```

Convexhullarea 416.8027m². This is measuredroofextent, not a claimthathull equals exactfootprint; slab/openingtopologyhasnotbeen fullyreconstructed. It neverthelessrefutesusingfull31.4×20.7bboxasrectangularfootprint (~650m²),whereprimaryprojectgivesbuildingarea429.51m². Nativeassetalsohas5.5mcoreholesatcentres(-7.5,-7.7),(-18.6,-7.7).

## Final Audit Status — Authoritative Corrections And Limits

This section supersedes earlier count and layout assumptions recorded chronologically above.

### Evidence hierarchy

- The A606 measured survey publication explicitly inventories **A76 / B64**, total140. Its measured floor plans occupy pages8–13. The coordinator independently read a floor chart with counts matching those totals: **A floors3–13=[2,7,7,7,8,8,7,8,8,7,7]**, **B floors3–11=[2,8,8,7,8,8,7,8,8]**. The earlier70/70 and86/54 allocations are superseded. Apartment suffixes are identifiers rather than counts; missing unlucky numbers invalidate reasoning from maximum suffix.
- The canonical repository DSL has154 capsules and is an authored design fixture. Its 8 compact slot frames across 3 sublevels are valuable geometric references; they do not establish the actual omitted capsule on every floor. Fixture f0→actual floor4 aligns periodic bridge omissions with floors6/9/12, but that mapping remains a hypothesis until checked against room plans.
- The authentic GLUON photogrammetry scan was decoded and visually inspected in memory. It confirms heterogeneous long/end-wall windows, offset capsule levels, bridge gaps, tall diagonal-cut roof heads and an irregular podium. Its merged mesh has no semantic room IDs and no explicit unit scale. It cannot supply certified per-room transforms through metadata.
- The A606 team reports discrepancies between original structural drawings and measured originals. Historical design drawings, surviving museum objects and as-built scan have different measurement scopes and dates. Do not silently substitute one for another.

### Independent repository CAD measurements available to implementation

- Six used capsule variants have approximately2.629–2.632 m ×2.528 m ×4.246 m overall GLB bounds including hardware. CAD frame=(GLBx,-GLBz,GLBy); capsule footprint is centred locally and nominal floor is about z=-.2. Exact circle and door planes are recorded above. J/L use side doors and end windows; P/S have long-wall windows; slash/backslash use end doors/windows.
- Square capital outer plan±2.75 m has top corner heights1,6,6,11.5 m, not a low tapered frustum. Exact16 vertices are above.
- Podium roof z=7.5 m has an irregular convex hull~416.803 m²; upper slab z=4.0 m hull~416.916 m²; ground z=0 hull~323.790 m². Upper storey is3.5 m above a4.0 m ground level in the asset. This is recessed framing/pilotis geometry, not a31.4×20.7 m bounding rectangle extruded5.5 m high. Top/ground point lists are measurements of asset geometry, not proof of original survey dimensions.
- Original core centres(-7.5,-7.7) and(-18.6,-7.7) are11.1 m apart. The 3-sublevel wrap uses lateral/side-entry capsules to pack compactly. Simply opposing4 m deep capsules at those centres creates collisions and must not replace the slot-frame design.

### Remaining unmeasured limits

Exact historical per-room slot identities/orientations, door/window variant labels, precise staircase circulation, structure joint geometry, all podium recesses and historical materials remain incompletely surveyed in the implementation evidence. Added generic beds, plumbing, railings and assumed wall thicknesses increase visual detail without establishing archaeological accuracy. Keep those assumptions explicit; do not claim a fully measured reconstruction. The complete measured survey book was identified but no freely accessible complete plan PDF was found. No external messages were sent; no source files were edited by this audit agent. Only this retained audit Markdown was written.
