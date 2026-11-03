"""EN 1993 feature-rows fixtures from the committed class1-beam-column oracle document: tension (6.2.3), shear (6.2.6) and bolted joint (1-8 3.6) pass/fail. Run: nice -n 19 .venv/bin/python -I gen1993.py [--write]"""
import copy, json, pathlib, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
DOCS = ROOT / "🧪️tests/🧫️fixtures/📄️oracle-documents"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
base = json.load(open(DOCS / "class1-beam-column/🔣️.json"))


def make(**action):
    d = copy.deepcopy(base)
    d["memberActions"][0]["action"].update(action)
    return d


def joint(shear):
    d = copy.deepcopy(base)
    d["joints"][0]["actions"][0]["shear"] = shear
    return d


FIXTURES = {
    "tension-member": make(n=-700000.0, my=0.0, vz=0.0),
    "tension-member-fail": make(n=-9000000.0, my=0.0, vz=0.0),
    "shear-member-fail": make(vz=900000.0),
    "joint-bolt-fail": joint(1500000.0),
}

if __name__ == "__main__":
    for name, doc in FIXTURES.items():
        if "--write" in sys.argv:
            (DOCS / name).mkdir(exist_ok=True)
            (DOCS / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        out = subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True)
        print(name, [(c["id"][7:], round(c["utilization"], 3)) for c in json.loads(out.stdout)["checks"]] if out.stdout else out.stderr[-300:])
