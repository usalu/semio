"""🏚️ Builds the irregular-in-elevation foundation sliding fixtures (the demand needs the 0.8 q factor that the pre-v18 Rust demand omitted; the friction angle is solved so the utilisation is 0.9 and 1.1) and prints the feature values. Run: /home/deck/semio/.venv/bin/python -I fixtures.py"""
import copy, json, math, pathlib, subprocess

ROOT = next(pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts").glob("*en1998")) / "🏅️standards/🔖️1/🪆️subsets/✳️any"
FIX = ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome"
ORACLE = next((ROOT / "🔮️oracles/🚦️check-outcome").glob("*.py"))
PY = "/home/deck/semio/.venv/bin/python"


def run(doc):
    out = subprocess.run([PY, "-I", str(ORACLE)], input=json.dumps(doc), capture_output=True, text=True, check=True).stdout
    return {c["id"]: c["utilization"] for c in json.loads(out)["checks"]}


def build(base, regular, angle):
    doc = copy.deepcopy(base)
    for b in doc["buildings"]:
        b["elevationRegular"] = regular
    doc["foundations"][0].update({"baseFrictionAngleDeg": angle, "passiveResistanceN": 0.0})
    return doc


def save(name, doc):
    path = FIX / name / "🔣️.json"
    path.parent.mkdir(exist_ok=True)
    path.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")


base = json.load(open(FIX / "seismic-walls-pass/🔣️.json", encoding="utf8"))
key = "en1998.5.fd-1.sliding"
u30 = run(build(base, False, 30.0))[key]
for name, target in (("seismic-sliding-irregular-pass", 0.9), ("seismic-sliding-irregular-fail", 1.1)):
    angle = round(math.degrees(math.atan(math.tan(math.radians(30.0)) * u30 / target)), 4)
    save(name, build(base, False, angle))
    print(name, "angle", angle, "new", run(build(base, False, angle))[key], "old demand", run(build(base, True, angle))[key])
