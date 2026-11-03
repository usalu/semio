"""EN 1995 feature-rows fixtures: an overloaded timber beam/connection/case variant, panel and floor overload variants of the mutation before-snapshots and a roomy deck pass. Run: nice -n 19 .venv/bin/python -I gen1995.py [--write]"""
import copy, json, pathlib, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIX = ROOT / "🧪️tests/🧫️fixtures"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
load = lambda p: json.load(open(FIX / p, encoding="utf8"))
cases = load("🚦️check-outcome/design-cases-fail/🔣️.json")
deck = load("🚦️check-outcome/deck-pass/🔣️.json")
panel = load("🧬️mutations/➖️remove-panel-layer/✅apply/🔣️.before.json")
floor = load("🧬️mutations/🏠️change-floor-damping/✅apply/🔣️.before.json")


def overload():
    d = copy.deepcopy(cases)
    m = d["members"][0]
    m.update(bM=0.08, hM=0.16, spanM=7.0, massKgPerM2=3000.0, massKgPerM=400.0, dampingXi=0.01)
    for a in m["actions"]:
        a["qLineNPerM"] *= 4
        a["fC90KN"] = 40000.0
    c = d["connections"][0]
    c.update(spacingM=0.03, edgeDistanceM=0.012, endDistanceM=0.02)
    for a in c["actions"]:
        a["fKN"] *= 4
    for case in d["cases"]:
        n = {x["id"]: x for x in case["numbers"]}
        if case["entryId"] == "T-18":
            n["hole_d"]["value"] = 90
            n["n_t"]["value"] = 9e6
        if case["entryId"] == "T-30":
            n["alpha_deg"]["value"] = 15
    return d


def panels():
    d = copy.deepcopy(panel)
    d["panels"][0]["loadPa"] = 90000.0
    d["panels"][0]["layers"][1]["thicknessM"] = 0.01
    return d


def floors():
    d = copy.deepcopy(floor)
    f = d["floors"][0]
    f.update(eiLNM2PerM=20000.0, eiBNM2PerM=4000.0, massKgPerM2=900.0, imposedLoadPa=9000.0)
    return d


def roomy():
    d = copy.deepcopy(deck)
    d["decks"][0]["plateSideM"] = 0.5
    return d


def wheel():
    d = copy.deepcopy(deck)
    d["decks"][0]["wheelLoadN"] *= 40
    return d


def vibration():
    d = copy.deepcopy(cases)
    d["members"][0].update(bM=0.08, hM=0.2, spanM=6.0, massKgPerM2=1.0, massKgPerM=1.0, dampingXi=0.0001)
    return d


def column():
    d = copy.deepcopy(load("🚦️check-outcome/column-fail/🔣️.json"))
    for a in d["members"][0]["actions"]:
        a["nKN"] *= 3
    d["members"][0]["bucklingLengthYM"] *= 2
    d["members"][0]["bucklingLengthZM"] *= 2
    return d


FIXTURES = {"vibration-fail": vibration(), "column-overload-fail": column(), "overload-fail": overload(), "panel-overload-fail": panels(), "floor-overload-fail": floors(), "deck-roomy-pass": roomy(), "deck-wheel-fail": wheel()}

if __name__ == "__main__":
    for name, doc in FIXTURES.items():
        path = FIX / "🚦️check-outcome" / name / "🔣️.json"
        if "--write" in sys.argv:
            path.parent.mkdir(exist_ok=True)
            path.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        else:
            path = pathlib.Path("/tmp/claude-1000/s") / (name + ".json")
            path.write_text(json.dumps(doc), encoding="utf8")
        out = subprocess.run([sys.executable, "-I", str(ORACLE), str(path)], capture_output=True, text=True)
        print(name, out.stderr[-200:])
        for c in json.loads(out.stdout)["checks"]:
            print("  ", c["id"], round(c["utilization"], 3))
