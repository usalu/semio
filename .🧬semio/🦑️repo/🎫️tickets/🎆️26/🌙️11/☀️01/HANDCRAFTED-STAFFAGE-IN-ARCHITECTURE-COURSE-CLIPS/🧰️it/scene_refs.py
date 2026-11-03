import json
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
bad = 0
total = 0


def walk(node, path):
    global bad, total
    if isinstance(node, dict):
        if "script" in node and "scene" in node:
            total += 1
            script = root / "🎓️teaching" / node["script"]
            if not script.exists():
                script = root / "🎓️teaching" / "🏛️architecture" / node["script"]
            if not script.exists():
                print("MISSING SCRIPT", path, node["script"])
                bad += 1
            elif not re.search(r"^class %s\b" % re.escape(node["scene"]), script.read_text(encoding="utf-8"), re.M):
                print("MISSING SCENE", path, node["script"], node["scene"])
                bad += 1
        for k, v in node.items():
            walk(v, path)
    elif isinstance(node, list):
        for v in node:
            walk(v, path)


for p in (root / "🎓️teaching" / "🏛️architecture").rglob("*.json"):
    if "tests" not in str(p) and "🎬️clip" not in str(p):
        continue
    try:
        walk(json.loads(p.read_text(encoding="utf-8")), p.name)
    except Exception:
        pass
print("checked", total, "bad", bad)
