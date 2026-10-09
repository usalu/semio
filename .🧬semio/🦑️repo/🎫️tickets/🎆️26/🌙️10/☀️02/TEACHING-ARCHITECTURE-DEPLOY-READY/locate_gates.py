import json
import pathlib

root = pathlib.Path(__file__).resolve().parents[7]
teaching = next(path for path in root.iterdir() if path.name.endswith("teaching"))
print("teaching", teaching)
count = 0
for path in teaching.rglob("project.json"):
    count += 1
    print("project", path.relative_to(root))
print("project.json count", count)
for path in teaching.rglob("package.json"):
    text = path.read_text(encoding="utf-8")
    if "architecture-quiz" in text or "proctor" in text:
        print("pkg", path.relative_to(root))
