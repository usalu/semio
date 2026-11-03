"""EN 1999 (W3-EN1999-NORM) fixtures derived from web-stiffener-pass: a staggered bolt pattern, a profiled sheet with a thin gauge, and a protected fire member; every row value comes from the independent oracle. `--write` stores the fixtures."""
import copy, json, pathlib, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIX = ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
base = json.load(open(FIX / "web-stiffener-pass/🔣️.json"))


def staggered(stagger):
    d = copy.deepcopy(base)
    c = d["connections"][0]
    c["bolts"].update(boltsPerRow=3, gauge=0.040, edgeDistance=0.030)
    c["holeStagger"] = stagger
    return d


def profiled():
    d = copy.deepcopy(base)
    sheet = d["coldFormed"][0]
    sheet.update(thickness=0.001, width=0.4)
    d["sheetProfiles"] = [{"id": "prof-1", "sheetId": sheet["id"], "pitch": 0.200, "topFlange": 0.060, "bottomFlange": 0.060, "webHeight": 0.040}]
    return d


def protected(thickness):
    d = copy.deepcopy(base)
    fire = d["fireScenarios"][0]
    fire.update(thetaA=20.0, durationS=900.0)
    d["fireInsulations"] = [{"id": "ins-1", "fireId": fire["id"], "thickness": thickness, "conductivity": 0.20, "specificHeat": 1200.0, "density": 800.0}]
    return d


FIXTURES = {"net-stagger-pass": staggered(0.030), "net-aligned-pass": staggered(0.0), "sheet-profile-pass": profiled(), "fire-protected-pass": protected(0.020), "fire-bare-pass": protected(0.0)}

if __name__ == "__main__":
    for name, doc in FIXTURES.items():
        if "--write" in sys.argv:
            (FIX / name).mkdir(exist_ok=True)
            (FIX / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        out = subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True)
        rows = json.loads(out.stdout)["checks"] if out.stdout else []
        keep = ("8.5.net", "1-4.", "1-2.fire", "1-2.insulation")
        print(name, out.stderr[-300:], " ".join(f"{c['id'][7:]}={c['utilization']:.8g}" for c in rows if any(k in c["id"] for k in keep)))
