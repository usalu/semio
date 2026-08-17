from pathlib import Path
import re
p = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")
puzzle = next(x for x in p.iterdir() if x.name.endswith("puzzle"))
apps = next(x for x in puzzle.iterdir() if "apps" in x.name)
twod = next(x for x in apps.iterdir() if "2d" in x.name)
comp = next(x for x in twod.iterdir() if "component" in x.name)
t = comp.read_text()

# print ingest_operations test fully
idx = t.find("fn ingest_operations_is_idempotent")
print("==== ingest ====")
print(t[idx:idx+1800])

# find AddNode command handling
for pat in [r"AddNode\s*\{", r"add_node\(", r'"addNode"', r"fn add_node"]:
    for m in re.finditer(pat, t):
        print("\n====", pat, "at", m.start())
        print(t[max(0,m.start()-200):m.start()+800])
        break

# fixture_nodes
idx = t.find("fn fixture_nodes")
print("\n==== fixture_nodes ====")
print(t[idx:idx+600] if idx>=0 else 'missing in this file')
# maybe in artifacts
arts = next(x for x in puzzle.iterdir() if 'artifact' in x.name)
for f in arts.rglob('*.rs'):
    tt=f.read_text(errors='ignore')
    if 'fn fixture_nodes' in tt or 'pub fn fixture_nodes' in tt:
        i=tt.find('fn fixture_nodes')
        print(f, tt[i:i+500])
