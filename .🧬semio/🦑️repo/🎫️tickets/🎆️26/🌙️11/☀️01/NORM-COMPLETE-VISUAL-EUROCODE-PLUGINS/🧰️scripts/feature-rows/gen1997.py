"""EN 1997 feature-rows fixtures derived from geotechnical-rich-pass: an overloaded variant crossing every geotechnical limit. Run: nice -n 19 .venv/bin/python -I gen1997.py [--write]"""
import copy, json, pathlib, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIX = ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
base = json.load(open(FIX / "geotechnical-rich-pass/🔣️.json", encoding="utf8"))


def overload():
    d = copy.deepcopy(base)
    d["investigationDepth"] = 1.0
    sand = d["layers"][0]
    sand.update(phiPrimeDeg=45.0, cptQc=2e6, sptN=5.0)
    f1, f2 = d["footings"]
    f1["loadCases"][0].update(verticalPermanent=4.2e6, horizontalPermanent=2.6e6, momentPermanent=1.5e6)
    f2["loadCases"][0].update(horizontalPermanent=150000.0, momentPermanent=3e5)
    d["upliftCases"][0].update(porePressure=400000.0)
    w = d["layerWaters"][0]
    w.update(headDifference=5.0, flowLength=3.0)
    g = d["pileGroups"][0]
    g.update(capVertical=9e6, shaftCohesionSum=1000.0, baseCohesion=1000.0)
    e = d["embeddedWalls"][0]
    e.update(embedment=0.5, retainedHeight=6.0)
    return d


FIXTURES = {"geo-overload-fail": overload()}

if __name__ == "__main__":
    for name, doc in FIXTURES.items():
        path = FIX / name / "🔣️.json"
        if "--write" in sys.argv:
            path.parent.mkdir(exist_ok=True)
            path.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        else:
            path = pathlib.Path("/tmp/claude-1000/s") / (name + ".json")
            path.write_text(json.dumps(doc), encoding="utf8")
        out = subprocess.run([sys.executable, "-I", str(ORACLE), str(path)], capture_output=True, text=True)
        print(name, out.stderr[-300:])
        print(" ".join(c["id"].replace("en1997.", "") + "=" + format(c["utilization"], ".2f") for c in json.loads(out.stdout)["checks"]))
