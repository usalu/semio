import os, time
from pathlib import Path
skip = {"🧪️tests", ".venv", "node_modules", "__pycache__", "site-packages", ".🧬semio"}
t = time.time(); n = 0
for d, f, fs in os.walk(r"C:\git\semio\🎓️teaching"):
    f[:] = [x for x in f if x not in skip]
    n += len(fs)
print(n, time.time() - t)
