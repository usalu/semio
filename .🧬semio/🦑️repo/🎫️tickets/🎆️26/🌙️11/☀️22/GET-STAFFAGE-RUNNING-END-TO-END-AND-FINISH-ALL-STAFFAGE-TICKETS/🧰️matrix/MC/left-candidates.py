"""🧪️ Candidate drawings of the two prismatic leftovers of the coordinator, to decide with numbers whether `derived-view-in-band` can hold: writes trial copies of the catalog documents into $GEN/left (never the catalog) and prints the measure of each (`derived.py`). `steel-column`: 1:100 with the base and head plates at 30 mm (the thinnest plate that prints over the whole band 1:75 to 1:150 at the 0.2 mm feature minimum) and at their true 20 mm; 1:200 with the gussets drawn in the side elevation. `glulam-beam`: 1:500 with the two bearing plates as true steps of the outline of the side elevation (120 x 20 mm, 0.24 x 0.04 mm on paper). `python3 left-candidates.py`, then `derived.ts` and `prints.ts` on the copies confirm measure and print check."""
import copy, glob, json, os
from derived import CATALOG, distance, points, report
GEN = os.environ["GEN"] + "/left"
os.makedirs(GEN, exist_ok=True)


def load(slug):
    """📖️ The catalog document of an id."""
    return json.load(open(glob.glob(f"{CATALOG}/*/{slug}/🔣️.json")[0]))


def save(doc, name):
    """💾️ Writes a trial copy."""
    json.dump(doc, open(f"{GEN}/{name}.json", "w"), ensure_ascii=False, indent=2)


def shape(g, view, sid):
    """🔎️ A shape of a view by id."""
    return next(s for s in g[view] if s["id"] == sid)


def plates(doc, t):
    """🧱️ The 1:100 column with base and head plate drawn t metres thick and the bar between them."""
    g = doc["asset"]["representations"]["100"]["geometry"]
    for p in shape(g, "elevation", "column-web-bar")["points"]:
        p[0] = t if p[0] < 1 else 3 - t
    for p in shape(g, "elevation", "base-plate-block")["points"]:
        p[0] = t if p[0] > 0 else 0
    for p in shape(g, "elevation", "cap-plate-block")["points"]:
        p[0] = 3 - t if p[0] < 3 else 3
    for sid in ("stiffener-left", "stiffener-right", "anchor-a", "anchor-b"):
        for p in shape(g, "elevation", sid)["points"]:
            if p[0] == 0.035:
                p[0] = t
    shape(g, "front", "column-flange-bar").update(y=t, h=3 - 2 * t)
    shape(g, "front", "base-plate-block").update(h=t)
    shape(g, "front", "stiffener-edge").update(y=t, h=0.11 - t)
    shape(g, "front", "cap-plate-block").update(y=3 - t, h=t)
    for sid in ("anchor-a", "anchor-b"):
        shape(g, "front", sid).update(y=t, h=0.075 - t)
    return doc


for t in (0.03, 0.02):
    doc = plates(load("steel-column"), t)
    save(doc, f"steel-column-plate-{round(t * 1000)}")
    report(f"steel-column 1:100, plates {round(t * 1000)} mm", doc["asset"]["representations"]["100"]["geometry"])

doc = load("steel-column")
g = doc["asset"]["representations"]["200"]["geometry"]
for sign, name in ((1, "left"), (-1, "right")):
    g["elevation"].append({**{k: v for k, v in shape(g, "elevation", "base-plate-slab").items() if k != "points"}, "id": f"stiffener-{name}", "points": [[0.05, sign * 0.1], [0.05, sign * 0.15], [0.07, sign * 0.15], [0.11, sign * 0.115], [0.11, sign * 0.1]]})
save(doc, "steel-column-gussets-200")
report("steel-column 1:200, gussets 50 x 60 mm in the side elevation", g)
report("steel-column 1:500, as it is", load("steel-column")["asset"]["representations"]["500"]["geometry"])

doc = load("glulam-beam")
g = doc["asset"]["representations"]["500"]["geometry"]
beam = shape(g, "elevation", "beam")
for key in ("x", "y", "w", "h"):
    del beam[key]
beam.update(kind="poly", closed=True, points=[[0, -0.32], [0.12, -0.32], [0.12, -0.3], [7.88, -0.3], [7.88, -0.32], [8, -0.32], [8, 0.3], [0, 0.3]])
save(doc, "glulam-beam-steps-500")
report("glulam-beam 1:500, bearing plates as steps of 120 x 20 mm", g)
