"""EN 1994 feature-rows for the tension stiffening check (`en1994.7.4.3.tension-stiffening.<beam>`, v11): one derived fixture (end span with 35 % of the hogging reinforcement) and the rows of the committed oracle documents. Run: .venv/bin/python -I gen1994v11.py [--write]"""
import copy, json, pathlib, re, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any")
DOCS = ROOT / "🧪️tests/🧫️fixtures/📄️oracle-documents"
FEATURE = ROOT / "🧪️tests/🚦️check-outcome-en1994-1/🥒️.feature"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
CHECK = "en1994.7.4.3.tension-stiffening.beam-B1"

derived = json.load(open(DOCS / "beam-end-span/🔣️.json"))
derived["beams"][0]["asHoggingM2PerM"] *= 0.35
DOCUMENTS = {"beam-end-span": None, "beam-interior-span": None, "beam-cantilever": None, "beam-end-span-hogging-fail": None, "beam-cantilever-fail": None, "beam-tension-stiffening-fail": derived}

text = FEATURE.read_text(encoding="utf8")
n = max(int(x) for x in re.findall(r"\|\s*(?:pass|fail)-(?:v\d+-)?(\d+)\s*\|", text))
rows = []
for name, doc in DOCUMENTS.items():
    if doc is not None and "--write" in sys.argv:
        (DOCS / name).mkdir(exist_ok=True)
        (DOCS / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    doc = doc or json.load(open(DOCS / name / "🔣️.json"))
    out = json.loads(subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True, check=True).stdout)
    c = next(x for x in out["checks"] if x["id"] == CHECK)
    status = c["status"]
    n += 1
    print(name, status, round(c["utilization"], 4))
    if f"| shared://📄️oracle-documents/{name}/🔣️.json | {CHECK} |" not in text:
        rows.append(f"      | {'fail' if status == 'fail' else 'pass'}-v2-{n:02d} | shared://📄️oracle-documents/{name}/🔣️.json | {CHECK} | {status} | {c['utilization']:.9g} | 1e-3 |")
print(len(rows), "new rows")
if "--write" in sys.argv and rows:
    FEATURE.write_text(text.rstrip("\n") + "\n" + "\n".join(rows) + "\n", encoding="utf8")
