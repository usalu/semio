"""🧷️ The stage 80 of the generator chain of `tennis-player` (HW-07, `REP-A/ops/tennis-player.80-silhouette.json`, replacements of drawn shapes only, so the stage is idempotent) applied to the catalog file with the applier of the chain itself and written as the chain writes it (one space of indentation). `python3 apply_chain.py tennis-player`"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, *[".."] * 9))
REP_A = os.path.join(ROOT, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️11", "☀️22", "STAFFAGE-HUMAN-COLLECTION-WAVE", "🧰️scripts", "REP-A")
spec = importlib.util.spec_from_file_location("rep_a_apply", os.path.join(REP_A, "apply.py"))
chain = importlib.util.module_from_spec(spec)
spec.loader.exec_module(chain)

for slug in sys.argv[1:]:
    path = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", slug, "🔣️.json")
    before = open(path, encoding="utf8").read()
    doc = json.loads(before)
    ops = json.load(open(os.path.join(REP_A, "ops", f"{slug}.80-silhouette.json"), encoding="utf8"))
    if any(op["op"] != "shape" for op in ops):
        raise SystemExit(f"{slug}: the stage holds an op that is not a replacement of a drawn shape")
    for op in ops:
        chain.apply(doc["asset"], op)
    if open(path, encoding="utf8").read() != before:
        raise SystemExit(f"{path} changed on disk since it was read: nothing written, run again")
    with open(path, "w", encoding="utf8") as handle:
        handle.write(json.dumps(doc, indent=1, ensure_ascii=False) + "\n")
    print(f"{slug}: {len(ops)} ops of the stage 80 applied")
