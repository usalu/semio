# GH-ENGINE report

Final signatures: `api.md` section 3 (Status: ready).

## Files

- `G/🧱️model/🐍️.py` engine (stdlib only), `G/🧱️model/🔣️.json` (catalogue of 68 components, casting rules, five canvases, 45 named views), `G/🧱️model/🧬️schema/🔣️.schema.json`
- `G/🧪️cases/🔣️.json` (25 invalid mutations, 5 path, 21 tree, 9 Path Mapper, 49 cast, 20 synthetic matching and 48 canvas cases), `G/🧪️tests/🐍️.py`, `G/🧪️tests/🟨️.cjs`
- Authoring aid that emits the JSON: `T/gh-engine/build_model.py` (plus `smoke.py`, `print_outputs.py`, `probe.py`)

## Decisions

- Capsule types, tower storeys, facades, core, capital, podium, bolts and the 70-branch / 140-capsule tree come from `🏢️nakagin` (sliders carry `bind` paths; literals `{"nakagin": path}`, `{"slot": …}`, `{"capsule": "A"}`, `{"aggregate": true}`). Only the window wall depth 0.3 m (`capsule_wall`) is an engine assumption.
- Casting follows `research-casting.md` and the coordinator: Number to Integer rounds ties to even (Rutten 2010) but is marked `basis: "conflicting"` with `sources` [rutten-2010, rutten-2012: truncation]; Point to Plane is world XY; Line to Plane has the line as Z; the failure message is the real GH text, German keeps it and explains. Rules without a source (Number to Domain 0 To x, Circle to Curve, Box to Brep, Integer to Boolean, Text to Boolean, Text to Point, Line to Vector, Domain to Number, Text to Number success) have `basis: "unverified"`; Point to Plane, Line to Plane, Point to Number are `community`.
- Open `[CHECK]` items kept in the data: Entwine path order (`{p;k}`), Simplify Tree both-ends default, Center Box takes sizes as Numbers (centred), failed cast turns the component red here (research says red, bubble text unverified).
- The tower canvas reproduces the exact Nakagin slots: planes per facade with the item stagger, copied to every storey, Cull Index removes the west planes of tower B's top storey, then yaw by type and clearance to the facade. Default is tower B (86); `tower_overrides("A")` gives tower A (54); together they equal `nakagin.slots()` (140) within 1e-9 (test `TestTowerSlots`). Per-slot data (yaw, cull indices, capsule per type) are parameter trees read from Nakagin.

## Test result

```
Ran 57 tests ... OK            (Python: tree/mapper/matching oracles, NumPy voxel volumes, rhino3dm PlaneToPlane, Rotation, BoundingBox, Plane)
Grasshopper schema: authored model accepted; 25 invalid models rejected.
Grasshopper canvases: wires, parameters and views consistent in 5 canvases.
Grasshopper trees: 23 tree and Path Mapper cases reproduced by an independent JavaScript oracle.
Grasshopper casting: 14 rounding, number and domain casts reproduced; 49 cases name a documented rule.
```

Commands: `.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe 🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🐍️.py` and `node 🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🟨️.cjs` from the repo root.

## Adding a view for a topic (scene agents)

Never edit `🔣️.json` by hand; it is regenerated from `T/gh-engine/build_model.py`. Add one function per topic that registers itself on a canvas, placed above the `model = {...}` line:

```python
@topic("capsule")
def window_teaser(c):
    c.slider("wt", "Wall depth", 0.1, 0.6, 0.3)
    c.obj("wtv", "Volume")
    c.panel("wtp", [""])
    c.wire("cut.Result>wtv.Geometry", "wtv.Volume>wtp.Stream")
    c.view("window-teaser", "Window volume", "Fenstervolumen", [("cut", 0, 0), ("wtv", 1, 0), ("wtp", 2, 0)])
```

Rules: reuse existing object ids (they stay valid; see the view lists in `api.md` section 3) and add new ids lowercase (`^[a-z][a-z0-9_]*$`); wire names are `id.OutputName>id.InputName` with the real parameter names from the catalogue; a view has at most 12 objects, view-local coordinates are `(column, row)` multiplied by 260 and 140, wires must run left to right, positions must not coincide; a new component goes into the catalogue section of `build_model.py` (`comp(...)`) plus an implementation `IMPL["Name"]` or `@component("Name")` in `🧱️model/🐍️.py`. Then regenerate and test:

```text
cd 🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -I <T>/gh-engine/build_model.py "$PWD/🔣️.json"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe 🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🐍️.py
node 🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🟨️.cjs
```

The tests check wire endpoints, the 12-object limit, left-to-right flow and schema validity for every view automatically. Several agents editing `build_model.py` at once: keep each topic function self-contained and append it; never reorder existing ones.
