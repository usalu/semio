"""🧫 Writes the EN 1994 fixtures `beam-studs-nonuniform` (per-stud layout: dense ends, sparse middle, a two-stud rib, a thin 16 mm stud, a stud on the span middle and two studs outside the span) and `beam-studs-nonuniform-fail` (same layout, the sparse middle thinned out)."""
import copy, json, pathlib
root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/📄️oracle-documents")
base = json.load(open(root / "beam-connection-pass/🔣️.json", encoding="utf8"))


def stud(i, x, d=0.019, h=0.095):
    return {"id": f"beam-B1-stud-{i}", "beamId": "beam-B1", "xM": x, "diameterM": d, "heightM": h, "fUPa": 450e6}


def make(xs):
    doc = copy.deepcopy(base)
    rows = []
    for x in xs:
        d = 0.016 if x == 0.3125 else 0.022 if x == 7.4375 else 0.019
        rows.append(stud(len(rows), x, d))
    doc["studs"] = [s for s in doc["studs"] if s["beamId"] != "beam-B1"] + rows
    return doc


grid = [round(0.0625 + 0.125 * k, 4) for k in range(80)]
extras = [0.0625, 5.0, 10.2, 10.3]
dense = sorted([x for x in grid if not 2.3 < x < 2.8] + extras)
sparse = sorted([x for x in grid if not 2.3 < x < 3.1 and not 6.1 < x < 6.7] + extras)
for name, xs in (("beam-studs-nonuniform", dense), ("beam-studs-nonuniform-fail", sparse)):
    out = root / name
    out.mkdir(exist_ok=True)
    with open(out / "🔣️.json", "w", encoding="utf8") as f:
        json.dump(make(xs), f, ensure_ascii=False, indent=2)
        f.write("\n")
