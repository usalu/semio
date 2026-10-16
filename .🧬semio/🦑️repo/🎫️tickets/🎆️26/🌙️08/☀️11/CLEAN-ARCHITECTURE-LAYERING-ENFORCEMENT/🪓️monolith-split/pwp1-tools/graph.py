#!/usr/bin/env python3
# 🕸️ First-party edges of the nine WP1 crates from their live manifests (+ external dependency counts).
import json
from lib import *
import deps
from crates_wp1 import ORDER

C = json.load(open(GEN + "/pwp0-crates.json"))
names = {C[k]["name"]: k for k in C}
for k in ORDER:
    m = deps.toml_json(PLUG + "/" + C[k]["root"] + "/" + PKGDIR + "/Cargo.toml")
    fp = sorted(names[d] for d in m.get("dependencies", {}) if d in names)
    ext = [d for d in m.get("dependencies", {}) if d not in names]
    dev = list(m.get("dev-dependencies", {}))
    print(f"{k:18s} rank {C[k]['rank']:2d}  -> {', '.join(fp) or '-'}   | external {len(ext)}, dev {len(dev)}, lib path {m['lib']['path']}")
