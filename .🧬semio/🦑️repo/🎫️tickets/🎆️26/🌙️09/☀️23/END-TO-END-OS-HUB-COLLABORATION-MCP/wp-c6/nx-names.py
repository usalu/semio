from pathlib import Path
import json
root = Path("/Users/ueli/Documents/semio")
for base in [root/"✏️s/🔌️plugins/🪐️space", root/"🧰️framework/🛍️products/💻️os"]:
  for p in base.rglob("project.json"):
    if "node_modules" in p.parts or "target" in p.parts or "generated" in p.parts: continue
    try: d=json.loads(p.read_text())
    except Exception: continue
    name=d.get("name")
    if not name: continue
    targets=list((d.get("targets") or {}).keys())
    if any(t.startswith("test") or t=="typecheck" for t in targets):
      if "space" in name or name in ("os","framework-os","@semio-tech/framework-os","os-ts","framework-os-ts"):
        print(name, targets[:12], "->", p.relative_to(root))
