import ast, json, sys
from pathlib import Path
ROOT = Path("C:/git/semio/🎓️teaching")
OUT = Path(sys.argv[1])
man = json.loads((ROOT / "🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json").read_text(encoding="utf-8"))["casts"]
calls = {"staffage","fitted_staffage","staffage_motion","staffage_height","staffage_parts","_staffage","_staffage_parts","_staffage_motion","_fitted_staffage","_cast","_use_cast","_figure","build","pendant_lamp","watt_anchor"}
res = {}
for p in sorted(ROOT.rglob("🐍️.py")):
    if "🎬️clip" not in p.parts or {"🧪️tests",".venv","node_modules","__pycache__"} & set(p.parts):
        continue
    src = p.read_text(encoding="utf-8")
    names = set()
    for n in ast.walk(ast.parse(src)):
        if isinstance(n, ast.Constant) and isinstance(n.value, str) and n.value in man:
            names.add(n.value)
    if names:
        res[p.relative_to(ROOT / "🏛️architecture").as_posix()] = sorted(names)
used = {c for v in res.values() for c in v}
OUT.write_text(json.dumps({"clips": res, "unused": sorted(set(man) - used)}, ensure_ascii=False, indent=1), encoding="utf-8")
print(len(res), len(used), len(man))
