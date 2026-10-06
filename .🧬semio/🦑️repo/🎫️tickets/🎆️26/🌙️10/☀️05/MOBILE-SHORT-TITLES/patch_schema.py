#!/usr/bin/env python3
import json
from pathlib import Path

p = Path(__file__).resolve().parents[7] / "🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json"
data = json.loads(p.read_text(encoding="utf-8"))
defs = data["$defs"]
defs["Quiz"]["properties"].setdefault("short", {"$ref": "#/$defs/ShortText"})
defs["CatalogView"]["properties"].setdefault("short", {"$ref": "#/$defs/ShortText"})
p.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("patched", p)
