import json
from pathlib import Path
V = Path("🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals")
KEEP = ["sun", "moon", "flame", "microbe", "factory"]
src = json.loads((V / "🫟️silhouettes/🔣️.json").read_text(encoding="utf-8"))
out = V / "🔣️icons"
(out / "🧬️schema").mkdir(parents=True, exist_ok=True)
doc = {"version": 1, "outlines": {k: src["outlines"][k] for k in KEEP}}
(out / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
schema = json.loads((V / "🫟️silhouettes/🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"))
schema["title"] = "Diagram Icon Outlines"
schema["properties"]["outlines"]["required"] = KEEP
(out / "🧬️schema/🔣️.schema.json").write_text(json.dumps(schema, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(sorted(out.rglob("*")))
