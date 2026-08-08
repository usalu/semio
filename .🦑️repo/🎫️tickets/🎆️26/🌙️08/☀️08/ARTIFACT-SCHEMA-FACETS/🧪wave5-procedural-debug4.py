#!/usr/bin/env python3
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
gumball = next(root.glob("**/🌀️procedural/**/🧭️gumball/🦀️component.rs"))
print("GUMBALL", gumball)
text = gumball.read_text()
# print tests region
idx = text.find("#[cfg(test)]")
print(text[idx : idx + 3500])

engine = next(root.glob("**/🧊️procedural3d/⚙️engine/🦀️component.rs"))
print("\nENGINE", engine)
# find mesh_data_for_preview_handle and preview_payload helpers
for key in [
    "fn mesh_data_for_preview_handle",
    "fn preview_payload_from_evaluated_fixture",
    "fn geometry_handles_for_widget",
    "fn host_from_fixture",
]:
    i = text.find(key) if False else engine.read_text().find(key)
    et = engine.read_text()
    i = et.find(key)
    print("\n====", key, "====")
    print(et[i : i + 700] if i >= 0 else "MISSING")
