"""📦 Author 12: publishes a validated candidate tree: writes its volumes source `volumes/vegetation/<slug>.json`, splices the volumes with `volumes-insert.py` into a copy of the candidate document and replaces the catalog file with it in one write.

Usage: python3 rework-12-publish.py <slug>
The catalog file is re-read immediately before the write; when it changed since the candidate was built from it (another author wrote it), nothing is written.
"""
import json
import os
import subprocess
import sys

T = os.path.dirname(os.path.abspath(__file__))
slug = sys.argv[1]
catalog = os.path.join(os.path.abspath(os.path.join(T, *[".."] * 7)), "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🌳️vegetation", slug, "🔣️.json")
out = os.path.join(T, "🗑️generated", "rework-12", slug)
candidate = json.load(open(os.path.join(out, "🔣️.json"), encoding="utf-8"))
before = open(catalog, encoding="utf-8").read()
volumes = candidate["asset"]["volumes"]
source = os.path.join(T, "volumes", "vegetation", slug + ".json")
with open(source, "w", encoding="utf-8") as f:
    json.dump({"volumes": volumes}, f, ensure_ascii=False, separators=(",", ":"))
    f.write("\n")
shell = json.loads(json.dumps(candidate))
shell["asset"]["volumes"] = {str(s): {"status": "unavailable", "denominator": s} for s in (50, 100, 200, 500)}
staged = os.path.join(out, "catalog.json")
with open(staged, "w", encoding="utf-8") as f:
    f.write(json.dumps(shell, ensure_ascii=False, indent=2) + "\n")
subprocess.run([sys.executable, os.path.join(T, "volumes-insert.py"), source, staged], check=True)
text = open(staged, encoding="utf-8").read()
if json.loads(text) != candidate:
    raise SystemExit("staged document differs from the candidate")
if open(catalog, encoding="utf-8").read() != before:
    raise SystemExit("catalog file changed meanwhile; rebuild the candidate")
with open(catalog, "w", encoding="utf-8") as f:
    f.write(text)
os.remove(staged)
print(f"published {slug}: catalog {len(text)} bytes, source {os.path.getsize(source)} bytes")
