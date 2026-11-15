import ast
from pathlib import Path

energy = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
bad = 0
count = 0
for path in energy.rglob("*.py"):
    s = path.as_posix()
    if ".venv" in s or "site-packages" in s or "__pycache__" in s or "/.🧬semio/" in s:
        continue
    count += 1
    try:
        ast.parse(path.read_text(encoding="utf-8"))
    except Exception as error:
        bad += 1
        print("BAD", path.relative_to(energy).as_posix().encode("ascii", "backslashreplace").decode(), str(error)[:80])
print("parsed", count, "bad", bad)
