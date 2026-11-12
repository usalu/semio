"""Imports every course module that uses `_manim_module.staffage` (run with the course Python environment from the workspace root)."""
import importlib, subprocess, sys, traceback
from pathlib import Path
root = Path.cwd()
sys.path.insert(0, str(root))
files = subprocess.run(["grep", "-rlE", r"_manim_module\.staffage", "🎓️teaching", "--include=*.py"], capture_output=True, text=True, cwd=root).stdout.split("\n")
failed = 0
for name in sorted(filter(None, files)):
    dotted = ".".join(Path(name).with_suffix("").parts)
    try:
        importlib.import_module(dotted)
        print("ok  ", name)
    except BaseException:
        failed += 1
        print("FAIL", name)
        traceback.print_exc(limit=3)
print(f"{len(files) - 1 - failed} imported, {failed} failed")
sys.exit(1 if failed else 0)
