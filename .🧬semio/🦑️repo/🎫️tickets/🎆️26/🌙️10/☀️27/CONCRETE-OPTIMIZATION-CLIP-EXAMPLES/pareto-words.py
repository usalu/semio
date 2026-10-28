"""Word counts and reading-floor seconds of the pareto narration."""
import ast
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
tree = ast.parse((ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🏆️pareto/🐍️.py").read_text(encoding="utf-8"))
rows = next(ast.literal_eval(node.value) for cls in tree.body if isinstance(cls, ast.ClassDef) for node in cls.body
            if isinstance(node, ast.Assign) and getattr(node.targets[0], "id", "") == "NARRATION")
total = 0.0
for key, en, de in rows:
    words = len(de.split())
    seconds = max(words / 1.6, 3.0)
    total += seconds
    print(f"{key:14s} de_words={words:2d} floor={seconds:4.1f}")
print("total floor", round(total, 1))
