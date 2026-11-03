"""EN 1991 feature-rows fixtures for the row collections of v12 (craneHoists, footbridges, railDynamics, loadDispersals, bulkSolids, rotatingMachines), derived from `actions-all-pass`, and their check-outcome rows from the committed oracle. Run: .venv/bin/python -I gen1991.py [--write]"""
import copy, json, pathlib, re, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIX = ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome"
FEATURE = ROOT / "🧪️tests/🚦️check-outcome-en1991-1/🥒️.feature"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
base = json.load(open(FIX / "actions-all-pass/🔣️.json"))

PASS = {
    "craneHoists": [{"id": "hoist-ok", "ratedLoad": 100000.0, "releaseFraction": 0.5, "releaseDevice": "grab"}],
    "footbridges": [{"id": "fb-ok", "loadedLength": 40.0, "deckWidth": 3.0, "naturalFrequency": 1.9}],
    "railDynamics": [{"id": "rd-ok", "determinantLength": 12.0, "maintenance": "careful"}],
    "loadDispersals": [{"id": "ld-ok", "wheelLength": 0.4, "wheelWidth": 0.4, "pavementThickness": 0.1, "slabThickness": 0.25}],
    "bulkSolids": [{"id": "bs-ok", "internalFriction": 0.5236}],
    "rotatingMachines": [{"id": "rm-ok", "mass": 500.0, "eccentricity": 0.002, "frequency": 25.0}],
}
FAIL = {
    "craneHoists": [{"id": "hoist-bad", "ratedLoad": 0.0, "releaseFraction": 1.5, "releaseDevice": "hook"}],
    "footbridges": [{"id": "fb-bad", "loadedLength": 0.0, "deckWidth": 3.0, "naturalFrequency": 1.9}, {"id": "fb-narrow", "loadedLength": 40.0, "deckWidth": -1.0, "naturalFrequency": 0.0}],
    "railDynamics": [{"id": "rd-bad", "determinantLength": -1.0, "maintenance": "rough"}],
    "loadDispersals": [{"id": "ld-bad", "wheelLength": 0.0, "wheelWidth": 0.4, "pavementThickness": 0.1, "slabThickness": 0.25}],
    "bulkSolids": [{"id": "bs-bad", "internalFriction": 2.0}],
    "rotatingMachines": [{"id": "rm-bad", "mass": -5.0, "eccentricity": 0.002, "frequency": 25.0}],
}
FIXTURES = {"collections-pass": PASS, "collections-fail": FAIL}
EXPECT = {
    "collections-pass": ["en1991.3.hoist-release.hoist-ok", "en1991.3.hoist-test-static.hoist-ok", "en1991.3.hoist-test-dynamic.hoist-ok", "en1991.2.footbridge-load.fb-ok", "en1991.2.footbridge-horizontal.fb-ok", "en1991.2.footbridge-walking.fb-ok", "en1991.2.rail-dynamic.rd-ok", "en1991.2.dispersal-footprint.ld-ok", "en1991.4.bulk-k-mean.bs-ok", "en1991.3.machine-force.rm-ok"],
    "collections-fail": ["en1991.input.craneHoists.hoist-bad.releaseDevice", "en1991.input.craneHoists.hoist-bad.releaseFraction", "en1991.input.craneHoists.hoist-bad.ratedLoad", "en1991.input.footbridges.fb-bad.loadedLength", "en1991.input.footbridges.fb-narrow.deckWidth", "en1991.input.railDynamics.rd-bad.determinantLength", "en1991.input.railDynamics.rd-bad.maintenance", "en1991.input.loadDispersals.ld-bad.wheelLength", "en1991.input.bulkSolids.bs-bad.internalFriction", "en1991.input.rotatingMachines.rm-bad.mass"],
}

text = FEATURE.read_text(encoding="utf8")
n = max(int(x) for x in re.findall(r"\|\s*(?:pass|fail)-(?:v\d+-)?(\d+)\s*\|", text))
rows = []
for name, collections in FIXTURES.items():
    doc = {**copy.deepcopy(base), **copy.deepcopy(collections)}
    if "--write" in sys.argv:
        (FIX / name).mkdir(exist_ok=True)
        (FIX / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    out = json.loads(subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True, check=True).stdout)
    answers = {c["id"]: c for c in out["checks"]}
    for check in EXPECT[name]:
        c = answers[check]
        status = c.get("status") or ("pass" if c["utilization"] <= 1 else "fail")
        n += 1
        kind = "fail" if status == "fail" else "pass"
        row = f"      | {kind}-v2-{n:02d} | shared://🚦️check-outcome/{name}/🔣️.json | {check} | {status} | {c['utilization']:.9g} | 1e-3 |"
        if f"| shared://🚦️check-outcome/{name}/🔣️.json | {check} |" not in text:
            rows.append(row)
print(len(rows), "new rows")
if "--write" in sys.argv and rows:
    FEATURE.write_text(text.rstrip("\n") + "\n" + "\n".join(rows) + "\n", encoding="utf8")
