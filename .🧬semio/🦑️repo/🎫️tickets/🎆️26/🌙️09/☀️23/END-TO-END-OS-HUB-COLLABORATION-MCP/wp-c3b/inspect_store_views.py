#!/usr/bin/env python3
from pathlib import Path

target = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-c3b/links/replication").resolve().parents[1]
# find store sync unit via known candidates from prior run
candidates = list(Path("/Users/ueli/Documents/semio").glob("*framework/*products/*/️*modules/*/️*store/*/️*sync/*/️*tests/*/️*unit/*.rs"))
if not candidates:
    # walk from os product
    os_prod = list(Path("/Users/ueli/Documents/semio").glob("*framework/*products/*/️os"))
    print("os_prod", os_prod)
    for root in Path("/Users/ueli/Documents/semio").glob("*framework/*products/*"):
        if not root.is_dir():
            continue
        for p in root.rglob("*.rs"):
            if "/sync/" in str(p).replace("\\", "/") and p.name.endswith(".rs") and "PresenceWindowView" in p.read_text():
                if "unit" in str(p):
                    candidates.append(p)
print("cands", candidates)
target = candidates[0]
t = target.read_text()
# show constructions
idx = 0
while True:
    i = t.find("PresenceWindowView", idx)
    if i < 0:
        break
    print("---", i)
    print(t[i : i + 350])
    print()
    idx = i + 20
