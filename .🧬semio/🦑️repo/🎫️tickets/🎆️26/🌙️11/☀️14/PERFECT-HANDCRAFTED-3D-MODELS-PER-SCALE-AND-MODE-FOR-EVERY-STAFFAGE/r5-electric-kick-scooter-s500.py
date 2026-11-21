"""🛴 Round 5, 1:500 of vehicles/electric-kick-scooter (digital scale, the 2D band is unavailable): derived by hand from the 1:200 lists with the merges of `equipment-digital.py` (flat deck nose with the split tail, one inclined neck, three-section wheel tread, two-station stem, the battery mass as one block) and the two mudguard arcs of the thermal list drawn as smooth bands of 8 and 6 segments instead of the faceted 6 and 4. Writes `volumes["500"]` of the source."""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(T, "volumes", "vehicles", "electric-kick-scooter.json")


def load(name, path=None):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), path or os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


digital = load("equipment-digital")
s200 = load("r3-electric-kick-scooter-s200")
pt, kit = s200.pt, s200.kit


def fenders():
    S = kit.Scale(500, s200.PENS)
    g = s200.GUARD_R
    rear = S.prism("th-fender-r", "deck", pt.band(pt.AR, g[0], g[1], pt.GUARD_REAR[0], pt.GUARD_REAR[1], 8), -0.03, 0.03, "plastic", 0.3)
    front = S.prism("th-fender-f", "fork", pt.band(pt.AF, g[0], g[1], pt.GUARD_FRONT[0], pt.GUARD_FRONT[1], 6), -0.03, 0.03, "plastic", 0.3)
    return {"th-fender-r": rear, "th-fender-f": front}


if __name__ == "__main__":
    source = json.load(open(SRC, encoding="utf8"))
    v = digital.Volume(source["volumes"]["200"], 500)
    digital.scooter(v)
    for name, solid in fenders().items():
        part = v.part(name, "thermal")
        part["profile"] = solid["profile"]
    v.note("The mudguards of the thermal list are smooth arcs of 8 (rear) and 6 (front) segments over the tyres.", "Die Schutzbleche der Thermikliste sind glatte Bögen aus 8 (hinten) und 6 (vorn) Segmenten über den Reifen.")
    source["volumes"]["500"] = v.finish()
    with open(SRC, "w", encoding="utf8") as fh:
        json.dump(source, fh, ensure_ascii=False)
        fh.write("\n")
    print("500", *(len(source["volumes"]["500"][k]) for k in ("body", "thermal", "structure")))
