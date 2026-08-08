#!/usr/bin/env python3
from pathlib import Path
import re

root = Path("/Users/ueli/Documents/semio")
proc = next(p for p in root.glob("**/🌀️procedural") if p.is_dir())

hits = []
for p in proc.rglob("*"):
    if not p.is_file():
        continue
    if p.suffix not in {".rs", ".ts", ".semio", ".md", ".json"}:
        continue
    try:
        text = p.read_text(errors="ignore")
    except Exception:
        continue
    if re.search(r"\brect\b", text):
        for i, line in enumerate(text.splitlines(), 1):
            if re.search(r"\brect\b", line) and (
                "widget" in line.lower()
                or "id" in line
                or "neuron" in line
                or "drawing" in line
            ):
                hits.append(f"{p.relative_to(root)}:{i}:{line.strip()[:160]}")

print("\n".join(hits[:100]))
print("--- total", len(hits))

for p in root.glob("**/🌊️flow/**/*.rs"):
    try:
        t = p.read_text(errors="ignore")
    except Exception:
        continue
    if "drawing.rect" in t or "curve.rect" in t or 'neuron_kind: "rect"' in t:
        print("KINDFILE", p)
        for i, line in enumerate(t.splitlines(), 1):
            if "rect" in line:
                print(f"  {i}:{line.strip()[:160]}")
