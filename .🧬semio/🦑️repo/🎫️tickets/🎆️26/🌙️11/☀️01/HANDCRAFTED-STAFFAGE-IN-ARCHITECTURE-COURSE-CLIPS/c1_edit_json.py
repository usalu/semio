import json
from pathlib import Path

v = Path("C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals")
d = json.loads((v / "🔣️.json").read_text(encoding="utf-8"))
for k in ("standing", "seated"):
    d.pop(k)
for k in ("standing_height", "seated_height"):
    d["dimensions"].pop(k)
(v / "🔣️.json").write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")

sp = v / "🧬️schema/🔣️.schema.json"
s = json.loads(sp.read_text(encoding="utf-8"))
s["required"] = [r for r in s["required"] if r not in ("standing", "seated")]
s["properties"]["dimensions"]["required"] = [r for r in s["properties"]["dimensions"]["required"] if r not in ("standing_height", "seated_height")]
for k in ("standing", "seated"):
    s["properties"].pop(k)
s["$defs"].pop("outline")
s["$defs"].pop("point")
if not s["$defs"]:
    s.pop("$defs")
sp.write_text(json.dumps(s, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
