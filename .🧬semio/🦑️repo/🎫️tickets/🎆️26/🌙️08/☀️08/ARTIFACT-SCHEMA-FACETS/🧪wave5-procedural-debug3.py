#!/usr/bin/env python3
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
paths = list(root.glob("**/🖍️draw/**/🦀️component.rs")) + list(
    root.glob("**/🖍️drawing/**/🦀️component.rs")
)
for p in paths:
    print("FILE", p)
    for i, line in enumerate(p.read_text(errors="ignore").splitlines(), 1):
        low = line.lower()
        if "rect" in low or "draw." in line:
            print(f"  {i}:{line.strip()[:200]}")
    print()
