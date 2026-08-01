from pathlib import Path

repo = Path("/Users/ueli/Documents/semio")
hits = [p for p in repo.rglob("📦️lib.rs") if "flow" in str(p) and "core" in str(p) and "implementation" in str(p) and "rust" in str(p)]
print("hits:")
for h in hits:
    print(h)

flow = None
for h in hits:
    t = h.read_text()
    if "fn flow_backed_node_graph_extras" in t and "fn from_fixture_with_cache" in t:
        flow = h
        break
print("chosen", flow)
if not flow:
    raise SystemExit(1)

text = flow.read_text().splitlines()

def dump_around(needle, before=5, after=80):
    for i, line in enumerate(text):
        if needle in line:
            print(f"\n===== {needle} @ {i+1} =====")
            for j in range(max(0, i - before), min(len(text), i + after)):
                print(f"{j+1}:{text[j]}")
            return
    print(f"MISSING {needle}")

dump_around("pub fn from_fixture_with_cache", 2, 50)
dump_around("pub fn flow_backed_node_graph_extras", 2, 60)
dump_around("fn rebuild_dag", 2, 40)
dump_around("fn build_dag_fixture", 2, 50)
