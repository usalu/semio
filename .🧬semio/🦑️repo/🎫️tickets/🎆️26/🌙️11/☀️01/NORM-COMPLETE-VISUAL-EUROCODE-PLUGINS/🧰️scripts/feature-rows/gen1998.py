"""EN 1998 feature-rows fixtures derived from seismic-rich-pass and seismic-rayleigh-cracked: pass/fail for the families without a pair (qLimit, t1Given, torsion, drift, overturning, plan.*, nonstructural, noSteelClass, steel.class, omegaWdBeam, cracked stiffness). Run: nice -n 19 .venv/bin/python -I gen1998.py [--write]"""
import copy, json, pathlib, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIX = ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
rich = json.load(open(FIX / "seismic-rich-pass/🔣️.json"))
cracked = json.load(open(FIX / "seismic-rayleigh-cracked/🔣️.json"))


def derive(base, **edits):
    d = copy.deepcopy(base)
    b = d["buildings"][0]
    for key, value in edits.items():
        if key == "q0":
            for s in b["systems"]:
                s["q0"] = value
        elif key == "omega":
            b["members"][1]["omegaWd"] = value
        elif key == "steel":
            b["members"].append({**b["members"][0], "id": "steel-c2", "material": "steel", "role": "column", "rho": 0.0, "steelSectionClass": value})
        elif key == "rcclass":
            b["members"][0]["steelSectionClass"] = value
        elif key == "cos_x":
            for s in b["storeys"]:
                s["centreOfStiffnessXM"] = value
        elif key == "resistance":
            for s in b["systems"]:
                s["baseShearResistanceN"] = value
        elif key == "drift":
            b["storeys"][1]["driftXM"] = value
            b["storeys"][1]["driftYM"] = value
        else:
            b[key] = value
    return d


FIXTURES = {
    "seismic-detail-pass": derive(rich, q0=3.0, t1Method="given", t1GivenS=0.4, steel=1, planEnvelopeGapRatio=0.02),
    "seismic-detail-fail": derive(rich, q0=7.0, t1Method="given", t1GivenS=1.5, steel=3, rcclass=1, omega=0.5, planWidthM=24.0, planLengthM=4.0, planEnvelopeGapRatio=0.08, nonstructuralResistanceN=2000.0, overturningBaseXM=0.1, overturningBaseYM=0.1, tieDownMomentNm=0.0, drift=0.06, cos_x=-20.0),
    "seismic-cracked-fail": derive(cracked, resistance=200000.0),
}

if __name__ == "__main__":
    for name, doc in FIXTURES.items():
        if "--write" in sys.argv:
            (FIX / name).mkdir(exist_ok=True)
            (FIX / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        out = subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True)
        rows = json.loads(out.stdout)["checks"] if out.stdout else []
        print(name, out.stderr[-300:])
        for c in rows:
            if c["id"].startswith("en1998.1.bldg-office"):
                print("  ", c["id"][12:], round(c["utilization"], 4))
