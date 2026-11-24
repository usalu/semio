"""🚢️ Ships the three-views ops of a person of HW-01 … HW-09: writes them as the stage `REP-A/ops/<slug>.70-three-views.json` of the person's generator chain (`REP-A/apply.py` applies the stages in order after the generator) and applies that one stage to the catalog file, which already carries the earlier stages, with the chain's own `apply` and in its own layout.
python3 ship-a-3.py <slug> <ops json> --before <untouched copy> --cand <proven candidate>"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, *[".."] * 8))
REP_A = os.path.normpath(os.path.join(HERE, "..", "..", "STAFFAGE-HUMAN-COLLECTION-WAVE", "🧰️scripts", "REP-A"))
spec = importlib.util.spec_from_file_location("rep_a_apply", os.path.join(REP_A, "apply.py"))
chain = importlib.util.module_from_spec(spec)
spec.loader.exec_module(chain)

slug, ops_file = sys.argv[1:3]
before = sys.argv[sys.argv.index("--before") + 1]
cand = sys.argv[sys.argv.index("--cand") + 1]
catalog = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", slug, "🔣️.json")
text = open(before, encoding="utf-8").read()
if open(catalog, encoding="utf-8").read() != text:
    raise SystemExit(f"{slug}: the catalog file changed since the untouched copy was taken")
doc = json.loads(text)
if json.dumps(doc, indent=1, ensure_ascii=False) + "\n" != text:
    raise SystemExit(f"{slug}: the catalog file is not in the layout of the chain")
ops = json.load(open(ops_file, encoding="utf-8"))
for op in ops:
    chain.apply(doc["asset"], op)
if doc != json.load(open(cand, encoding="utf-8")):
    raise SystemExit(f"{slug}: the stage on the untouched copy does not give the candidate")
stage = os.path.join(REP_A, "ops", f"{slug}.70-three-views.json")
with open(stage, "w", encoding="utf-8") as handle:
    handle.write("[\n" + ",\n".join(json.dumps(op, ensure_ascii=False) for op in ops) + "\n]\n")
with open(catalog, "w", encoding="utf-8") as handle:
    handle.write(json.dumps(doc, indent=1, ensure_ascii=False) + "\n")
print(f"{slug}: {len(ops)} ops as stage 70, catalog file written")
