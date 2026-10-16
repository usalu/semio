#!/usr/bin/env python3
# 🔒️ Stage 9: moves the third-party users of the former single contract manifest (base64, json-patch, serde_json) onto the producer manifests that declare them now.
import json, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *
path = ROOT + "/🔒️dependencies.json"
text = read(path)
data = json.loads(text)
old = C[len(ROOT) + 1:] + "/📦️packages/🦀️rust/Cargo.toml"
manifests = {c["package"]: c["pkgdir"][len(ROOT) + 1:] + "/Cargo.toml" for c in CRATES}
manifests[FACADE] = old
changed = 0
for entry in data["entries"]:
    if entry["ecosystem"] != "rust" or entry["name"] not in ("base64", "json-patch", "serde_json") or old not in entry["users"]:
        continue
    declared = [m for m in manifests.values() if re.search(r"(?m)^" + re.escape(entry["name"]) + r"(\.workspace)?\s*=", read(ROOT + "/" + m))]
    users = [u for u in entry["users"] if u != old] + declared
    entry["users"] = sorted(set(users))
    changed += 1
write(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")
print("entries updated", changed)
