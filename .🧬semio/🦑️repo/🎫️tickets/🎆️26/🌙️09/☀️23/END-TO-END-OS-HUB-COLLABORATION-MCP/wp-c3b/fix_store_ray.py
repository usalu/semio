#!/usr/bin/env python3
from pathlib import Path

target = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-c3b/links/replication").resolve()
# Walk to the store sync unit test found earlier via relative discovery
framework = target.parents[1]
hits = []
for p in framework.rglob("*.rs"):
    try:
        text = p.read_text()
    except Exception:
        continue
    if "sample_presence_peer_with_interaction" in text and "PresenceWindowView" in text:
        hits.append(p)
print("hits", hits)
if not hits:
    raise SystemExit("not found")
path = hits[0]
t = path.read_text()
old = """                pointer: Some([0.5, 0.5, 0.5]),
            },"""
new = """                pointer: Some([0.5, 0.5, 0.5]),
                ray_origin: None,
            },"""
if old not in t:
    if "ray_origin: None" in t[t.find("pointer: Some([0.5") : t.find("pointer: Some([0.5") + 80]:
        print("already has ray_origin on Some pointer")
    else:
        print("OLD NOT FOUND")
        i = t.find("pointer: Some([0.5")
        print(repr(t[i : i + 120]))
        raise SystemExit(1)
else:
    path.write_text(t.replace(old, new, 1))
    print("patched", path)

# verify all constructions have ray_origin
t = path.read_text()
missing = 0
idx = 0
while True:
    i = t.find("PresenceWindowView", idx)
    if i < 0:
        break
    chunk = t[i : i + 500]
    if "window_id" in chunk and "ray_origin" not in chunk:
        missing += 1
        print("MISSING near", t[:i].count("\n") + 1)
    idx = i + 20
print("missing", missing)
