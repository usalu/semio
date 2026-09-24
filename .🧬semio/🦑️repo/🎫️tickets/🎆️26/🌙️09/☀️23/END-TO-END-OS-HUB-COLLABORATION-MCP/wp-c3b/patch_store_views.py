#!/usr/bin/env python3
from pathlib import Path
import re

# Resolve via rg-known relative walk from replication link
rep = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-c3b/links/replication").resolve()
# framework root is parents: replication -> modules -> framework? 
# replication is at framework/modules/replication
framework = rep.parents[1]
candidates = list(framework.glob("**/store/**/sync/**/unit/*.rs"))
if not candidates:
    # emoji folder names: walk products
    products = list(framework.glob("*products/*"))
    print("products", products)
    for prod in products:
        for p in prod.rglob("*.rs"):
            if "sync" in str(p) and "unit" in str(p) and "PresenceWindowView" in p.read_text():
                candidates.append(p)
print("candidates", candidates)
for target in candidates:
    t = target.read_text()
    if "PresenceWindowView" not in t:
        continue
    if "ray_origin" in t and t.count("PresenceWindowView") <= t.count("ray_origin") + 2:
        print("already ok", target)
        continue
    newt = t.replace("pointer: None },", "pointer: None, ray_origin: None },")
    newt = newt.replace("pointer: None }", "pointer: None, ray_origin: None }")
    # multiline with pointer: Some(...)
    newt = re.sub(
        r"(pointer:\s*(?:None|Some\([^)]+\)))(\s*)\}",
        lambda m: m.group(1) + ", ray_origin: None" + m.group(2) + "}"
        if "ray_origin" not in m.group(0)
        else m.group(0),
        newt,
    )
    missing = []
    for m in re.finditer(r"PresenceWindowView \{.{0,600}?\}", newt, re.S):
        block = m.group(0)
        if "window_id" in block and "ray_origin" not in block:
            missing.append(block[:120])
    if missing:
        print("STILL MISSING in", target)
        for m in missing:
            print(" ", m)
    else:
        target.write_text(newt)
        print("patched", target)
