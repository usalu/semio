"""EN 1999 feature-rows fixtures derived from web-stiffener-pass: an overloaded variant (members, connection, sheet, shell, fatigue), a web-panel patch-load pair and a short-fire pass variant, then the family coverage spec. Run: nice -n 19 .venv/bin/python -I gen1999.py [--write]"""
import copy, json, pathlib, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIX = ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
base = json.load(open(FIX / "web-stiffener-pass/🔣️.json"))
LOADS = ("gKLine", "qKLine", "nK", "vYK", "vZK", "mYK", "mZK", "torsionK", "localLoadK")
PATCH = {"id": "patch-1", "subjectId": "purlin-1", "position": "interior", "x": 1.0, "bearingLength": 0.05, "webHeight": 0.0, "webAngle": 0.0, "cornerRadius": 0.004, "load": 12000.0}


def scaled(doc, factor):
    d = copy.deepcopy(doc)
    owners = d["members"] + d["connections"] + d["coldFormed"] + d["shells"]
    for owner in owners:
        for action in owner["actions"]:
            for key in LOADS:
                action[key] *= factor
    return d


def overload():
    d = scaled(base, 30.0)
    d["fatigueDetails"][0]["deltaSigmaEd"] = 90e6
    d["connections"][0]["bolts"].update(edgeDistance=0.012, pitch=0.02)
    d["fireScenarios"][0].update(thetaA=250.0, durationS=1800.0)
    d["members"][0]["actions"][1].update(vYK=1e5, mZK=3000.0, torsionK=3e4)
    d["coldFormed"][0].update(thickness=0.001)
    d["coldFormed"][0]["actions"][0].update(vZK=1e5)
    d["sections"][0]["elements"][0].update(thickness=0.0005)
    d["connections"][0]["welds"].update(length=0.02)
    d["ringStiffeners"] = [{"id": "ring-1", "shellId": "shell-pass", "x": 1.0, "depth": 0.06, "thickness": 0.006, "internal": False}]
    return d


def sparse():
    d = copy.deepcopy(base)
    d["connections"][0]["bolts"].update(edgeDistance=0.5, pitch=0.5)
    return d


def patch(load):
    d = copy.deepcopy(base)
    d["patchLoads"] = [{**PATCH, "load": load}]
    return d


def calm():
    d = scaled(base, 0.1)
    d["fireScenarios"][0].update(thetaA=40.0, durationS=60.0)
    d["connections"][0]["bolts"].update(gauge=0.036)
    return d


FIXTURES = {"overload-fail": overload(), "web-panel-patch-pass": patch(12000.0), "web-panel-patch-fail": patch(900000.0), "calm-pass": calm(), "bolts-spread-fail": sparse()}

if __name__ == "__main__":
    for name, doc in FIXTURES.items():
        if "--write" in sys.argv:
            (FIX / name).mkdir(exist_ok=True)
            (FIX / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        out = subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True)
        rows = json.loads(out.stdout)["checks"] if out.stdout else []
        print(name, out.stderr[-300:], " ".join(f"{c['id'][7:]}={c['utilization']:.3g}" for c in rows if c["id"].split(".")[-1] not in ("post-chs",) and "post-chs" not in c["id"]))
