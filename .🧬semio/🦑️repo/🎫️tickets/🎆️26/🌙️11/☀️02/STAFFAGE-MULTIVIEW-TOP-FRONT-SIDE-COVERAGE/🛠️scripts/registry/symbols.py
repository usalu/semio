"""🔤 Read-only inventory of symbol ids in the staffage catalog that carry a view word, with their owning assets and every textual reference in the archive, course, fixtures and tests."""

import json
import re
import sys
from pathlib import Path

catalog_dir, library, output = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
VIEW_WORDS = re.compile(r"(^|-)(plan|front|elevation|side|top|true|extent)(-|$)")

symbols = []
for folder in sorted(catalog_dir.iterdir()):
    if not folder.is_dir():
        continue
    for asset_dir in sorted(folder.iterdir()):
        if not asset_dir.is_dir():
            continue
        files = [p for p in asset_dir.iterdir() if p.suffix == ".json"]
        if len(files) != 1:
            continue
        asset = json.loads(files[0].read_text(encoding="utf-8"))["asset"]
        for symbol in asset.get("symbols", []):
            symbols.append({"asset": asset["id"], "id": symbol["id"], "keys": sorted(symbol.keys())})

flagged = [s for s in symbols if VIEW_WORDS.search(s["id"])]
ids = sorted({s["id"] for s in flagged})

references: dict[str, list[str]] = {i: [] for i in ids}
search_roots = [catalog_dir, library / "🧪️tests", library / "🧫️fixtures", library / "🎯️targets", library / "🌐️site", library / "🔨️modules"]
repo = library.parents[2]
course = library.parent / "⚡️energy" / "🏗️authoring" / "🎨️visuals"
search_roots.append(course)
for root in search_roots:
    if not root.exists():
        continue
    for path in root.rglob("*"):
        if not path.is_file() or path.suffix not in (".json", ".py", ".ts", ".tsx", ".feature", ".mjs"):
            continue
        if "node_modules" in path.parts or ".venv" in path.parts:
            continue
        text = path.read_text(encoding="utf-8", errors="ignore")
        for i in ids:
            if re.search(r'"' + re.escape(i) + r'"|\'' + re.escape(i) + r"'", text):
                references[i].append(path.relative_to(repo).as_posix())

result = {
    "generatedBy": "haiku-registry",
    "note": "Read-only. Symbol ids containing a view word, their owners and every quoted reference found.",
    "symbolsTotal": len(symbols),
    "flaggedSymbols": len(flagged),
    "flaggedIds": ids,
    "owners": {i: sorted({s["asset"] for s in flagged if s["id"] == i}) for i in ids},
    "references": {i: sorted(set(references[i])) for i in ids},
}
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text(json.dumps(result, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(json.dumps({"symbolsTotal": len(symbols), "flaggedSymbols": len(flagged), "distinctFlaggedIds": len(ids)}, ensure_ascii=False))
for i in ids:
    print(f"{i}: owners {sorted({s['asset'] for s in flagged if s['id'] == i})}; refs {len(set(references[i]))}")
