
from pathlib import Path
import json
root = Path("/Users/ueli/Documents/semio")
for p in list((root/"✏️s/🔌️plugins/🪐️space").rglob("project.json")) + list((root/"🧰️framework/🛍️products/💻️os").rglob("project.json"))[:30]:
    try:
        d=json.loads(p.read_text())
    except Exception:
        continue
    name=d.get("name")
    if name and ("space" in name or "os" in name or "home" in name):
        print(name, "->", p)
