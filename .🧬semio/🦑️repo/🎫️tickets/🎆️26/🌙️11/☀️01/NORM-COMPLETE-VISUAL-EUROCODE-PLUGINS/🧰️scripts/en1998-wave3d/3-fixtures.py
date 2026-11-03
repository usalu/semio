"""EN 1998 wave 3d: writes the check-outcome fixtures isolation-pass, isolation-fail, hazard-pass and hazard-fail from families-pass (isolation system, pier base, liquefiable layers with a steep slope, pile axial load) and prints the oracle rows of the new check ids.
Usage: /home/deck/semio/.venv/bin/python -I 3-fixtures.py [--write]"""
import copy, json, pathlib, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIX = ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
base = json.load(open(FIX / "families-pass/🔣️.json", encoding="utf8"))


def make(name, iso, bridge_period, layers, axial, slope=None, foundation=None):
    d = copy.deepcopy(base)
    d["isolationSystems"] = [dict(id="iso-1", bridgeId="br-1", buildingId="", **iso)]
    d["pierBases"] = [dict(id="pb-1", bridgeId="br-1", pierId="pier-1", radiusM=2.5, soilVsMS=250.0, soilUnitWeightNM3=19000.0, poissonRatio=0.35)]
    d["bridges"][0]["fundamentalPeriodS"] = bridge_period
    for f in d["foundations"]:
        f["pileAxialLoadN"] = 0.0
    d["foundations"][0]["pileAxialLoadN"] = axial
    for layer_id, n in layers.items():
        next(l for l in d["soilLayers"] if l["id"] == layer_id)["n160"] = n
    if foundation:
        d["foundations"][0].update(foundation)
    if slope:
        d["slopes"][0].update(slope)
    return name, d


CASES = [
    make("isolation-pass", dict(strengthN=3.0e5, stiffnessNPerM=1.5e7, yieldDisplacementM=0.01, capacityM=0.5), 0.25, {}, 3.0e6),
    make("isolation-fail", dict(strengthN=1.0e5, stiffnessNPerM=1.0e6, yieldDisplacementM=0.01, capacityM=0.1), 0.25, {}, 3.0e6),
    make("hazard-pass", dict(strengthN=3.0e5, stiffnessNPerM=1.5e7, yieldDisplacementM=0.01, capacityM=0.5), 0.25, {"l2": 1.0}, 2.0e6, {"governingLayerId": "l2", "angle": 2.0}, {"pileHeadShearN": 50000.0}),
    make("hazard-fail", dict(strengthN=3.0e5, stiffnessNPerM=1.5e7, yieldDisplacementM=0.01, capacityM=0.5), 0.25, {"l1": 0.5, "l2": 2.0}, 6.0e7),
]
for name, d in CASES:
    out = subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(d), capture_output=True, text=True, check=True).stdout
    rows = [r for r in json.loads(out)["checks"] if any(k in r["id"] for k in ("isolation", ".base.", "poreRatio", "spreading", "liquefied", "shearModulus"))]
    print("==", name)
    for r in rows:
        print(f"  {r['id']} {r['status']} {r['utilization']:.9g} {r.get('value')}")
    if "--write" in sys.argv:
        target = FIX / name
        target.mkdir(exist_ok=True)
        json.dump(d, open(target / "🔣️.json", "w", encoding="utf8"), ensure_ascii=False, indent=2)
        open(target / "🔣️.json", "a").write("\n")
