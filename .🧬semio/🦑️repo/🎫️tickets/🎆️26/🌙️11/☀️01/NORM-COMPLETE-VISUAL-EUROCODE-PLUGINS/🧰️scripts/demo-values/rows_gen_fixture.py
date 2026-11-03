"""Catalogue-fixture-case rows from the demo cases (EN 1995): demo cases of entries the independent oracle evaluates and the fixture does not hold yet are appended to the committed catalogue fixture and to the Examples of the feature (indices are positions). Run with the repo venv python -I: rows_gen_fixture.py <code> [--write]."""
import glob, re, sys
sys.path.insert(0, "."); from dv_lib import *
import oracle_probe as P
code = sys.argv[1]; write = "--write" in sys.argv
A = art(code)
fxp = A / "🧪️tests/🧫️fixtures/📇️catalogue/🔣️.json"
raw = fxp.read_text(encoding="utf-8"); fx = json.loads(raw)
print("fixture roundtrip", json.dumps(fx, indent=2, ensure_ascii=False) + "\n" == raw or json.dumps(fx, ensure_ascii=False) == raw.strip())
feature = next(pathlib.Path(p) for p in glob.glob(str(A / "🧪️tests/*atalogue*/🥒️.feature")))
body = feature.read_text(encoding="utf8")
have = {c["entry"] for c in fx["cases"]}
doc = json.loads(demo_doc_path(code).read_text(encoding="utf-8"))
new, rows, skipped = [], [], {}
for case in doc["cases"]:
    e = case["entryId"]
    if e in have: continue
    status, _ = P.res.get(e, ("?", None))
    if status != "OK": skipped[e] = status; continue
    index = len(fx["cases"]) + len(new)
    new.append({"entry": e, "case": {"annex": "De", "num": {n["id"]: n["value"] for n in case["numbers"]}, "text": {t["id"]: t["value"] for t in case["texts"]}}})
    rows.append("      | %s-%d | %d | %s |" % (e.lower().replace(".", "-"), index, index, e))
print(code, len(new), "new fixture cases;", len(skipped), "without independent formulation")
for e, why in skipped.items(): print("  skip", e, why)
if write and new:
    fx["cases"].extend(new)
    fxp.write_text(json.dumps(fx, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    lines = body.rstrip("\n").split("\n")
    last = max(i for i, line in enumerate(lines) if re.match(r"\s+\|.*\|\s*$", line))
    lines[last + 1 : last + 1] = rows
    feature.write_text("\n".join(lines) + "\n", encoding="utf8")
    print("written")
