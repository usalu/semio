"""EN 1996 v12: derives the fixture `detail-chase-warning` from `detail-rich-pass` (chases beyond Tables 8.1 and 8.2: width, distance to an opening, horizontal length, recess width and cumulative width) and appends its check-outcome rows, idempotently. Usage: gen1996-v12.py --write"""
import copy, glob, json, pathlib, re, subprocess, sys

sub = glob.glob("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/*en1996/🏅️standards/🔖️1/🪆️subsets/✳️any")[0]
root = pathlib.Path(sub) / "🧪️tests/🧫️fixtures/🚦️check-outcome"
feature = pathlib.Path(sub) / "🧪️tests/🚦️check-outcome-en1996-1/🥒️.feature"
oracle = sub + "/🔮️oracles/🚦️check-outcome/🐍️.py"
doc = copy.deepcopy(json.load(open(root / "detail-rich-pass/🔣️.json", encoding="utf8")))
base = doc["chases"][0]
make = lambda **kw: {**base, **kw}
doc["chases"] = [
    base,
    make(id="chase-2", xM=2.1, yM=0.0, widthM=0.35, lengthM=0.8, depthM=0.08),
    make(id="chase-3", kind="Horizontal", xM=3.5, yM=1.0, widthM=0.05, lengthM=1.5, depthM=0.02),
    make(id="chase-4", kind="Recess", xM=4.0, yM=0.2, widthM=0.3, lengthM=0.4, depthM=0.03),
]
text = json.dumps(doc, indent=1, ensure_ascii=False) + "\n"
if "--write" not in sys.argv:
    print(text[:200])
    sys.exit()
(root / "detail-chase-warning").mkdir(exist_ok=True)
(root / "detail-chase-warning/🔣️.json").write_text(text, encoding="utf8")
out = subprocess.run([sys.executable, "-I", oracle, str(root / "detail-chase-warning/🔣️.json")], capture_output=True, text=True)
utilisation = {c["id"]: c["utilization"] for c in json.loads(out.stdout)["checks"]}
ids = [i for i in sorted(utilisation) if ".8.6.chase-" in i]
body = feature.read_text(encoding="utf8")
n = max(int(x) for x in re.findall(r"\|\s*(?:pass|fail)-(\d+)\s*\|", body))
uri = "shared://🚦️check-outcome/detail-chase-warning/🔣️.json"
rows = []
for i in ids:
    if f"| {uri} | {i} |" in body:
        continue
    n += 1
    u = utilisation[i]
    status = "pass" if u <= 1 + 1e-9 else "fail" if i.endswith(("extent",)) or "chase-extent" in i else "warning"
    rows.append(f"      | {'pass' if status == 'pass' else 'fail'}-{n:02d} | {uri} | {i} | {status} | {u:.9g} | 1e-6 |")
if not body.endswith("\n"):
    body += "\n"
feature.write_text(body + "\n".join(rows) + ("\n" if rows else ""), encoding="utf8")
print(len(rows), "rows")
