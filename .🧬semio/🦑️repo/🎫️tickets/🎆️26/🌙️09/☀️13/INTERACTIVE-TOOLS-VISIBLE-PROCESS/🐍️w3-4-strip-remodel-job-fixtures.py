"""🧹️ W3-4: removes the retired `job` lane from every remodeling snapshot and diff fixture (top-level objects
that carry `results`), keeping the committed 2-space canonical JSON formatting byte for byte otherwise."""
import glob
import json
import os

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures"
changed = 0
for path in glob.glob(os.path.join(ROOT, "**", "*.json"), recursive=True):
    raw = open(path, encoding="utf-8").read()
    if '"job"' not in raw:
        continue
    document = json.loads(raw)
    if not (isinstance(document, dict) and "job" in document and "results" in document):
        continue
    del document["job"]
    text = json.dumps(document, indent=2, ensure_ascii=False)
    open(path, "w", encoding="utf-8").write(text + ("\n" if raw.endswith("\n") else ""))
    changed += 1
print(f"stripped job from {changed} fixtures")
