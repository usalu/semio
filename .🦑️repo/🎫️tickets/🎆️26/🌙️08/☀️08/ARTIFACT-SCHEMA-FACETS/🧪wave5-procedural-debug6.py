#!/usr/bin/env python3
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
gumball = next(root.glob("**/🌀️procedural/**/🧭️gumball/🦀️component.rs"))
t = gumball.read_text()
# Full translate_selection module
start = t.find("pub mod translate_selection")
end = t.find("pub mod rotate_selection")
print("=== translate_selection ===")
print(t[start:end])
print("=== ensure_gumball usage in file ===")
for i, line in enumerate(t.splitlines(), 1):
    if "ensure_gumball" in line or "commit_fixture" in line or "Emit::" in line:
        print(f"{i}:{line}")

engine = next(root.glob("**/🧊️procedural3d/⚙️engine/🦀️component.rs"))
et = engine.read_text()
print("\n=== ensure_linked_flow_extensions ===")
i = et.find("fn ensure_linked_flow_extensions")
print(et[i : i + 900] if i >= 0 else "MISSING")
print("\n=== tessellate path in mesh_data ===")
i = et.find("fn mesh_data_for_preview_handle")
print(et[i : i + 1200])
