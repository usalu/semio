import json, sys
from pathlib import Path
man = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))["casts"]
root = Path(sys.argv[2])
skip = {".venv", "node_modules", "__pycache__", ".git"}
texts = []
for p in root.rglob("*"):
    if p.suffix not in (".py", ".ts", ".tsx", ".mjs", ".json", ".md") or set(p.parts) & skip or not p.is_file():
        continue
    if p.name == "🔣️.json" and "🧍️staffage" in p.parts and p.parent.name == "🧍️staffage":
        continue
    try:
        texts.append((p, p.read_text(encoding="utf-8", errors="ignore")))
    except OSError:
        pass
for cast in sorted(man):
    hits = [str(p.relative_to(root)) for p, t in texts if f'"{cast}"' in t or f"'{cast}'" in t]
    if not hits:
        print("UNUSED-LITERAL", cast, man[cast]["asset"])
    else:
        print("USED", cast, len(hits))
