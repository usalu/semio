import ast
import glob
from pathlib import Path

for path in sorted(glob.glob("🎓️teaching/🏛️architecture/⚡️energy/*/🎬️clip/*/🐍️.py")):
    tree = ast.parse(Path(path).read_text(encoding="utf-8"))
    for n in ast.walk(tree):
        if isinstance(n, (ast.Assign, ast.AnnAssign)):
            tg = n.targets if isinstance(n, ast.Assign) else [n.target]
            if any(isinstance(t, ast.Name) and t.id == "NARRATION" for t in tg):
                for c in ast.walk(n):
                    if isinstance(c, ast.Call) and isinstance(c.func, ast.Name) and c.func.id == "tr":
                        print(path.split("clip")[-1], c.lineno)
