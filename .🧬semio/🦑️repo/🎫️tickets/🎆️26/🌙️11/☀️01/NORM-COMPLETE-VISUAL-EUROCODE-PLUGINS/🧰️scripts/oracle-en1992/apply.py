import importlib.util, json, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SUBSET = next(Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts").glob("*en1992/🏅️standards/🔖️1/🪆️subsets/✳️any"))
VECTORS = SUBSET / "🧬️schema/🧪️tests/🧫️vectors/catalogue.json"
ORACLE = SUBSET / "🔮️oracles/⚖️compliance/🐍️.py"

spec = importlib.util.spec_from_file_location("oracle", ORACLE)
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)

new = []
for path in sorted(HERE.glob("cases_*.py")):
    s = importlib.util.spec_from_file_location(path.stem, path)
    m = importlib.util.module_from_spec(s)
    s.loader.exec_module(m)
    new += m.CASES

doc = json.loads(VECTORS.read_text(encoding="utf-8"))
tag = lambda c: json.dumps(c, sort_keys=True)
fresh = {tag(c) for c in new}
ids = {c["id"] for c in new}
base = [c for c in doc["cases"] if c["id"] not in ids or c.get("_base", False)]
base = [c for c in doc["cases"] if c["id"] not in ids]
for c in new:
    c.setdefault("text", {}); c.setdefault("list", {}); c.setdefault("num", {}); c.setdefault("tol", 1e-6)
    out = oracle.HANDLERS[c["id"]](c)
    assert out, c["id"]
    print(c["id"], {k: round(float(v), 6) for k, v in out.items()})
doc["cases"] = base + new
if "--write" in sys.argv:
    VECTORS.write_text(json.dumps(doc, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
    print("written", len(doc["cases"]))
