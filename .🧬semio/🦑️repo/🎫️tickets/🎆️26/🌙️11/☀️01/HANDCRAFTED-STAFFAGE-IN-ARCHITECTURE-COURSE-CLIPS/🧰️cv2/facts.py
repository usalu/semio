import json
from pathlib import Path

W = Path("C:/git/semio")
m = json.loads((W / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json").read_text(encoding="utf-8"))["casts"]
print(len(m))
for k, v in m.items():
    if "transport" in v or "timeline" in v or "environment" in v:
        print(k, json.dumps(v, ensure_ascii=False))
print("--- pose casts without env/clip (cheap):")
for k, v in list(m.items()):
    if "pose" in v and "environment" not in v and "timeline" not in v and "transport" not in v:
        print(k, json.dumps(v, ensure_ascii=False))
        break
