import gzip
import re
from pathlib import Path

base = Path(r"C:\git\semio")
teaching = next(path for path in base.iterdir() if path.name.endswith("teaching"))
dist = next(path for path in teaching.rglob("dist") if path.parent.name.endswith("typescript") and "quiz" in str(path))
files = [path for path in dist.rglob("*") if path.is_file() and "pages" not in path.parts]
total = sum(path.stat().st_size for path in files)
print("total", total, "files", len(files))
html = (dist / "index.html").read_text(encoding="utf-8")
linked = re.findall(r'<(?:script|link)\b[^>]*\s(?:src|href)="/(assets/[^"]+)"', html)
script = 0
style = 0
for rel in linked:
    data = (dist / rel).read_bytes()
    weight = len(gzip.compress(data, compresslevel=6))
    print(("js" if rel.endswith(".js") else "css" if rel.endswith(".css") else "other"), weight, len(data), rel)
    if rel.endswith(".js"):
        script += weight
    if rel.endswith(".css"):
        style += weight
print("script", script)
print("style", style)
print("pdf", any("pdf" in path.name for path in files))
