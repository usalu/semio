
from pathlib import Path
repo = Path("/Users/ueli/Documents/semio")
wgpu = next(p for p in repo.rglob("📦️lib.rs") if "renderer" in str(p) and "wgpu" in str(p) and "engine" in str(p) and p.parent.name.endswith("rust"))
lines = wgpu.read_text().splitlines()

def dump(needle, after=90):
    for i, line in enumerate(lines):
        if needle in line:
            print(f"\n===== {needle} @ {i+1} =====")
            for j in range(i, min(len(lines), i + after)):
                print(f"{j+1}:{lines[j]}")
            return
    print("MISSING", needle)

dump("fn is_flow_graph", 35)
dump("fn sync_flow_host", 120)
dump("fn graph_scene_json", 30)

engine = next(p for p in repo.rglob("📦️lib.rs") if "procedural" in str(p) and "engine" in str(p) and "3d" in str(p) and "module" in str(p))
print("ENGINE", engine)
etext = engine.read_text().splitlines()
for i, line in enumerate(etext):
    if "struct Procedural3dRuntime" in line or "impl Default for Procedural3dRuntime" in line:
        print(f"\n===== runtime @ {i+1} =====")
        for j in range(i, min(len(etext), i+70)):
            print(f"{j+1}:{etext[j]}")
