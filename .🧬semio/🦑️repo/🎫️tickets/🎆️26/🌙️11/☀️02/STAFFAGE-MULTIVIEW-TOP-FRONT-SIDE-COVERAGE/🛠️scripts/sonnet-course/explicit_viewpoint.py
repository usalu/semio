import re, sys
from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json")
raw = p.read_bytes().decode("utf-8")
out = []
for line in raw.split("\n"):
    if '"asset"' in line and '"viewpoint"' not in line:
        line = line.replace('"theme"', '"viewpoint": "elevation", "theme"', 1)
    out.append(line)
p.write_bytes("\n".join(out).encode("utf-8"))
