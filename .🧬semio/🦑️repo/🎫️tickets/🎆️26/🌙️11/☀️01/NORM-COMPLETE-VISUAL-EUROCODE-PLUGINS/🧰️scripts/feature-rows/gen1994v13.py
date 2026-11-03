"""EN 1994 v13 rows: shear-reduced moment at rho = 1 (capped) and rho = 0.27 (hogging + sagging); original: v12 (class, stud spacing and layout, slab longitudinal shear, bending-shear interaction, SLS stress, slab punching): derived fixtures and one row per (document, id). Run: .venv/bin/python -I gen1994v12.py [--write]"""
import copy, json, pathlib, re, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any")
DOCS = ROOT / "🧪️tests/🧫️fixtures/📄️oracle-documents"
FEATURE = ROOT / "🧪️tests/🚦️check-outcome-en1994-1/🥒️.feature"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
load = lambda name: json.load(open(DOCS / name / "🔣️.json"))


def run(doc):
    return {c["id"]: c for c in json.loads(subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True, check=True).stdout)["checks"]}


DERIVED = {}
ROWS = [("beam-shear-fail", "6.2.2.4.vm"), ("beam-shear-fail", "6.2.2.4.vm-sagging"), ("beam-hogging-interaction-fail", "6.2.2.4.vm"), ("beam-hogging-interaction-fail", "6.2.2.4.vm-sagging")]
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
    cid = next((i for i in out if i.startswith(f"en1994.{family}.")), None)
    if cid is None:
        print("skip", name, family)
        continue
    c = out[cid]
    status = c.get("status") or ("pass" if c["utilization"] <= 1 else "fail")
    n += 1
    print(name, cid, status, round(c["utilization"], 5))
    if f"| shared://📄️oracle-documents/{name}/🔣️.json | {cid} |" not in text:
        rows.append(f"      | {'fail' if status == 'fail' else 'pass'}-v13-{n:02d} | shared://📄️oracle-documents/{name}/🔣️.json | {cid} | {status} | {c['utilization']:.9g} | 1e-2 |")
print(len(rows), "new rows")
if "--write" in sys.argv and rows:
    FEATURE.write_text(text.rstrip("\n") + "\n" + "\n".join(rows) + "\n", encoding="utf8")
