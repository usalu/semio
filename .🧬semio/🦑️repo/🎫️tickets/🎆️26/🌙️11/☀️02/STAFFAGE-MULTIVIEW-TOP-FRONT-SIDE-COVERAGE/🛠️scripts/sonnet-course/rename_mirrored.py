import re, subprocess, sys
from pathlib import Path
root = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip())
ren = {"optimization-room-cooler": "optimization-room-cooler-mirrored", "ventilation-natural-radiator-on": "ventilation-natural-radiator-on-mirrored", "ventilation-natural-radiator-off": "ventilation-natural-radiator-off-mirrored", "ventilation-volume-hood": "ventilation-volume-hood-mirrored", "shared-blind-lowered": "shared-blind-lowered-mirrored"}
files = subprocess.check_output(["git", "-c", "core.quotepath=false", "ls-files", "-z", "--", "🎓️teaching"], cwd=root).decode("utf-8").split("\0")
for name in files:
    p = root / name
    if not name or p.suffix not in (".py", ".json", ".md", ".ts", ".tsx") or not p.is_file():
        continue
    try:
        s = p.read_bytes().decode("utf-8")
    except UnicodeDecodeError:
        continue
    o = s
    for a, b in ren.items():
        s = re.sub(r"(?<![\w-])" + re.escape(a) + r"(?![\w-])", b, s)
    if s != o:
        p.write_bytes(s.encode("utf-8"))
        sys.stdout.buffer.write((name + "\n").encode("utf-8"))
