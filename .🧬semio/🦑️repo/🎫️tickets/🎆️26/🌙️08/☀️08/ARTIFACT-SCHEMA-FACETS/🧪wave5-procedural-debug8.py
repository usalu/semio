#!/usr/bin/env python3
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
engine = next(root.glob("**/🧊️procedural3d/⚙️engine/🦀️component.rs"))
et = engine.read_text()
print("=== test_support ===")
i = et.find("mod test_support")
print(et[i : i + 900] if i >= 0 else "no test_support")
print("\n=== tessellate ===")
for needle in ["tessellate_geometry", "use .*tessellate", "fn test_serial", "TEST_SERIAL"]:
    print(needle, et.find(needle))

for p in sorted(root.glob("**/🌀️procedural/**/*.rs")):
    t = p.read_text(errors="ignore")
    hits = []
    for i, line in enumerate(t.splitlines(), 1):
        if any(x in line for x in ["ensure_linked_flow_extensions", "test_support::lock", "test_serial()", "TEST_SERIAL"]):
            hits.append(f"{i}:{line.strip()}")
    if hits:
        print(f"\nFILE {p.relative_to(next(root.glob('**/🌀️procedural')))}")
        print("\n".join(hits))

print("\n=== flow window tests ===")
for p in root.glob("**/🌀️procedural/**/🕸️flow/🦀️component.rs"):
    print("FLOW", p)
    ft = p.read_text()
    i = ft.find("fn main_graph_scene")
    print(ft[i : i + 1000] if i >= 0 else "no test")
