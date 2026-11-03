import re
from pathlib import Path

root = Path("C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals")
manifest = root / "🧍️staffage/🔣️.json"
lines = manifest.read_text(encoding="utf-8").split("\n")
drop = ('"ventilation-need-occupant-mirrored"', '"heating-calculation-radiator-mirrored"', '"optimization-room-radiator"', '"ventilation-volume-student"')
out = []
for line in lines:
    key = line.strip().split(":")[0]
    if key in drop:
        continue
    if key == '"ventilation-volume-office-worker"':
        line = line.replace("ventilation-volume-office-worker", "shared-office-worker-pathlines")
    if key == '"heating-calculation-radiator"':
        line = line.replace("heating-calculation-radiator", "shared-radiator-thermostat")
    out.append(line)
text = "\n".join(out)
manifest.write_text(text, encoding="utf-8", newline="")
print(len(lines) - len(out))

guard = root / "🧪️tests/🔣️staffage_only.json"
g = guard.read_text(encoding="utf-8")
g = g.replace('"forbidden_casts": [', '"forbidden_casts": ["ventilation-need-occupant-mirrored", "ventilation-volume-office-worker", "ventilation-volume-student", "heating-calculation-radiator", "heating-calculation-radiator-mirrored", "optimization-room-radiator", ', 1)
guard.write_text(g, encoding="utf-8", newline="")
