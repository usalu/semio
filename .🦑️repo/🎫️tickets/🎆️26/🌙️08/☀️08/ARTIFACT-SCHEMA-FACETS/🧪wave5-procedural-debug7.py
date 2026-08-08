#!/usr/bin/env python3
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
gumball = next(root.glob("**/🌀️procedural/**/🧭️gumball/🦀️component.rs"))
t = gumball.read_text()
print("=== gumball_transform + mesh_selection ===")
i = t.find("fn gumball_transform")
print(t[i : i + 1200])
i = t.find("fn mesh_selection_ids")
print(t[i : i + 500])

engine = next(root.glob("**/🧊️procedural3d/⚙️engine/🦀️component.rs"))
et = engine.read_text()
print("\n=== ensure_gumball_node ===")
i = et.find("fn ensure_gumball_node")
print(et[i : i + 1500] if i >= 0 else "MISSING")
print("\n=== tessellate_geometry ===")
i = et.find("fn tessellate_geometry")
print(et[i : i + 800] if i >= 0 else "MISSING")
