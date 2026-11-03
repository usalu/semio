"""🔎 Extract every string literal of the energy clip modules that carries a quantitative unit claim."""
import ast
import json
import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
OUT = Path(sys.argv[2])
UNIT = re.compile(r"(\d[\d .,]*)\s?(kWh|kW|MJ|kJ|kg|mL|ml|L|W|g|min|°C|degC|K|m²|m³|m/s|m|h|s)(?![A-Za-zäöüÄÖÜ])")
rows = []
for path in sorted(ROOT.rglob("🐍️.py")):
    if "🎬️clip" not in path.parts:
        continue
    rel = "🎓️teaching" + str(path).replace("\\", "/").split("🎓️teaching", 1)[1]
    try:
        tree = ast.parse(path.read_text(encoding="utf-8"))
    except SyntaxError as exc:
        rows.append({"file": rel, "line": exc.lineno, "text": "SYNTAX", "claim": ""})
        continue
    for node in ast.walk(tree):
        if isinstance(node, ast.Constant) and isinstance(node.value, str):
            text = node.value
            for m in UNIT.finditer(text):
                rows.append({"file": rel, "line": node.lineno, "text": text, "claim": m.group(0)})
                break
OUT.write_text(json.dumps(rows, ensure_ascii=False, indent=1), encoding="utf-8")
print(len(rows))
