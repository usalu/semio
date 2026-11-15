from pathlib import Path

path = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🧲️physics\🎬️clip\🧲️fundamentals\🐍️.py")
lines = path.read_text(encoding="utf-8").split("\n")
count = 0
for i, line in enumerate(lines):
    stripped = line.lstrip()
    if stripped.startswith('"') and "$" in line and "\\" in line:
        indent = line[: len(line) - len(stripped)]
        lines[i] = indent + "r" + stripped
        count += 1
path.write_text("\n".join(lines), encoding="utf-8")
print(count)
