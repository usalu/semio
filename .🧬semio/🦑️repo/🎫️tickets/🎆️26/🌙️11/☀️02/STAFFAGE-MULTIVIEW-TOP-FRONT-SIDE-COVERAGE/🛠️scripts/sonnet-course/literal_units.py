import ast, sys
from pathlib import Path
R = Path("C:/git/semio/🎓️teaching")
calls = {"staffage","fitted_staffage","staffage_motion","staffage_parts","_staffage","_staffage_parts","_staffage_motion","_fitted_staffage","_cast","_use_cast","_figure","build","pendant_lamp","watt_anchor"}
out = []
for p in sorted(R.rglob("🐍️.py")):
    if "🎬️clip" not in p.parts or {"🧪️tests",".venv","node_modules","__pycache__"} & set(p.parts):
        continue
    src = p.read_text(encoding="utf-8")
    for n in ast.walk(ast.parse(src)):
        if isinstance(n, ast.Call):
            f = n.func
            name = f.id if isinstance(f, ast.Name) else f.attr if isinstance(f, ast.Attribute) else None
            if name in {"staffage","staffage_motion","_staffage","_staffage_motion","_cast","_use_cast","_figure"}:
                for k in n.keywords:
                    if k.arg in ("unit",) and any(isinstance(x, ast.Constant) and isinstance(x.value, (int, float)) for x in ast.walk(k.value)):
                        out.append(f"{p.relative_to(R).as_posix()}:{n.lineno}: {ast.unparse(k.value)}  <- {src.splitlines()[n.lineno-1].strip()[:120]}")
sys.stdout.buffer.write(("\n".join(out) + "\n" + str(len(out)) + "\n").encode("utf-8"))
