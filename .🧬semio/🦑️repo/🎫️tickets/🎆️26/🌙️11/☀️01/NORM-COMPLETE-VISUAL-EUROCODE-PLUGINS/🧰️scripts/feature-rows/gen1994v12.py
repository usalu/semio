"""EN 1994 feature-rows for the families of the v12 oracle twin (class, stud spacing and layout, slab longitudinal shear, bending-shear interaction, SLS stress, slab punching): derived fixtures and one row per (document, id). Run: .venv/bin/python -I gen1994v12.py [--write]"""
import copy, json, pathlib, re, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any")
DOCS = ROOT / "🧪️tests/🧫️fixtures/📄️oracle-documents"
FEATURE = ROOT / "🧪️tests/🚦️check-outcome-en1994-1/🥒️.feature"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
load = lambda name: json.load(open(DOCS / name / "🔣️.json"))


def run(doc):
    return {c["id"]: c for c in json.loads(subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True, check=True).stdout)["checks"]}


class4 = load("beam-connection-pass")
class4["beams"][0]["steel"] = {"designation": "CUSTOM-SLENDER", "heightM": 0.6, "widthM": 0.2, "twM": 0.004, "tfM": 0.01, "aM2": 0.00632, "wPlYM3": 1.62e-3, "iYM4": 3.9e-4, "aVM2": 0.00232, "rM": 0.0}
spacing = load("beam-connection-pass")
spacing["beams"][0]["studs"]["spacingM"] = 0.9
hogging = load("beam-end-span")
hogging["beams"][0]["actions"] = [a for a in hogging["beams"][0]["actions"]]
for a in hogging["beams"][0]["actions"]:
    if a["id"] == "Q-office":
        a["qAreaPa"] = 22000.0
punch_pass = load("slab-continuous")
punch_pass["slabs"][0]["actions"].append({"id": "F-wheel", "kind": "imposed", "category": "B", "stage": "composite", "qAreaPa": 0.0, "fKN": 60000.0, "deltaSigmaKPa": 0.0, "deltaTauKPa": 0.0, "xM": 1.5, "yM": 0.0})
punch_fail = copy.deepcopy(punch_pass)
punch_fail["slabs"][0]["actions"][-1]["fKN"] = 200000.0
DERIVED = {"beam-class4-fail": class4, "beam-stud-spacing-fail": spacing, "beam-hogging-interaction-fail": hogging, "slab-punching-pass": punch_pass, "slab-punching-fail": punch_fail}
ROWS = [("beam-connection-pass", "5.5.class"), ("beam-class4-fail", "5.5.class"), ("beam-connection-pass", "6.6.5.5.spacing"), ("beam-stud-spacing-fail", "6.6.5.5.spacing"), ("beam-end-span", "6.6.5.5.spacing"),
        ("beam-end-span", "6.6.1.1.layout"), ("beam-connection-pass", "6.6.1.1.layout"), ("beam-cantilever", "6.6.1.1.layout"), ("beam-partial-connection", "6.6.1.1.layout"),
        ("beam-connection-pass", "6.6.6.vlrd"), ("beam-bending-fail", "6.6.6.vlrd"), ("beam-shear-fail", "6.6.6.vlrd"),
        ("beam-connection-pass", "6.2.2.4.vm-sagging"), ("beam-bending-fail", "6.2.2.4.vm-sagging"), ("beam-partial-connection", "6.2.2.4.vm-sagging"),
        ("beam-end-span", "6.2.2.4.vm"), ("beam-interior-span", "6.2.2.4.vm"), ("beam-cantilever", "6.2.2.4.vm"), ("beam-hogging-interaction-fail", "6.2.2.4.vm"),
        ("beam-connection-pass", "7.2.2.stress"), ("beam-end-span", "7.2.2.stress"), ("beam-interior-span", "7.2.2.stress"), ("beam-shear-fail", "7.2.2.stress"),
        ("slab-punching-pass", "9.7.6.punching"), ("slab-punching-fail", "9.7.6.punching")]
text = FEATURE.read_text(encoding="utf8")
n = max(int(x) for x in re.findall(r"\|\s*(?:pass|fail)-(?:v\d+-)?(\d+)\s*\|", text))
rows = []
for name, doc in DERIVED.items():
    if "--write" in sys.argv:
        (DOCS / name).mkdir(exist_ok=True)
        (DOCS / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
for name, family in ROWS:
    doc = DERIVED.get(name) or load(name)
    out = run(doc)
    cid = next(i for i in out if i.startswith(f"en1994.{family}."))
    c = out[cid]
    status = c.get("status") or ("pass" if c["utilization"] <= 1 else "fail")
    n += 1
    print(name, cid, status, round(c["utilization"], 5))
    if f"| shared://📄️oracle-documents/{name}/🔣️.json | {cid} |" not in text:
        rows.append(f"      | {'fail' if status == 'fail' else 'pass'}-v2-{n:02d} | shared://📄️oracle-documents/{name}/🔣️.json | {cid} | {status} | {c['utilization']:.9g} | 1e-2 |")
print(len(rows), "new rows")
if "--write" in sys.argv and rows:
    FEATURE.write_text(text.rstrip("\n") + "\n" + "\n".join(rows) + "\n", encoding="utf8")
