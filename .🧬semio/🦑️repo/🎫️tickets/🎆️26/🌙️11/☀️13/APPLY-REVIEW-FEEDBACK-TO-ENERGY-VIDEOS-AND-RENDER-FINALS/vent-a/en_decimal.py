import re
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
base = root / "🎓️teaching" / "🏛️architecture" / "⚡️energy" / "🌬️ventilation" / "🎬️clip"
pattern = re.compile(r'(\(\s*"[a-z_0-9]+",\s*\n\s*r?")((?:[^"\\]|\\.)*)(",\s*\n\s*r?")')
for child in base.iterdir():
    path = child / "🐍️.py"
    if child.name not in ("❓️need", "📦️volume", "🌡️heat-flow") or not path.is_file():
        continue
    text = path.read_bytes().decode("utf-8")
    out = pattern.sub(lambda m: m.group(1) + m.group(2).replace("{,}", ".") + m.group(3), text)
    path.write_bytes(out.encode("utf-8"))
    print(child.name, text.count("{,}") - out.count("{,}"))
