"""📝 Print the en and de summary, title and provenance notes of the named catalog assets, sentence by sentence, when they carry a number."""
import json
import re
import sys
from pathlib import Path

CATALOG = Path(sys.argv[1])
WANTED = set(sys.argv[2:])
NUM = re.compile(r"\d")
for path in sorted(CATALOG.rglob("🔣️.json")):
    doc = json.loads(path.read_text(encoding="utf-8"))
    asset = doc.get("asset", {})
    if asset.get("id") not in WANTED:
        continue
    print("==", asset["id"])
    texts = {"summary": asset.get("summary", {}), "notes": asset.get("provenance", {}).get("notes", {}), "title": asset.get("title", {})}
    for field, pair in texts.items():
        for lang in ("en", "de"):
            value = pair.get(lang, "")
            for sentence in re.split(r"(?<=[.;])\s+", value):
                if NUM.search(sentence):
                    print(f"  {field}.{lang}: {sentence}")
