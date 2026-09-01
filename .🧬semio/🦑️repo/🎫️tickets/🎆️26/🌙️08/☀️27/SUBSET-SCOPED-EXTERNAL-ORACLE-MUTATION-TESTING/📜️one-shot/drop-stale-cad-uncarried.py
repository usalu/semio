#!/usr/bin/env python3
# 🧵️ One-shot: drops the STALE `-uncarried` requirement from cad's two layer kinds. Both already name
# `dxf-crate-cad-r12-read` as a qualifying oracle AND carry committed before/after fixtures, and the
# dxf reader was measured witnessing each one by name:
#   set-entity-layer       -> entity[3] layer differs: "0" vs "ANNOTATIONS"
#   set-block-entity-layer -> block[0].entity[0] layer differs: "0" vs "LEAF"
# Both compare readingsEqual:true against themselves and readingsEqual:false against their own after.
# The marker understated the coverage; it is removed, not re-labelled.
import json

ORACLE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧪️oracle/🔣️.json"
TARGETS = {"set-entity-layer", "set-block-entity-layer"}

d = json.load(open(ORACLE))
dropped = 0
for m in d["mutationManifests"][0]["mutations"]:
    if m["id"] not in TARGETS:
        continue
    before = len(m.get("oracleRequirements", []))
    m["oracleRequirements"] = [r for r in m.get("oracleRequirements", []) if not str(r.get("capability", "")).endswith("-uncarried")]
    if not any(r.get("oracle") for r in m["oracleRequirements"]):
        raise SystemExit(f"refusing to strip {m['id']}: no named qualifying oracle would remain")
    dropped += before - len(m["oracleRequirements"])
json.dump(d, open(ORACLE, "w"), ensure_ascii=False, indent=2)
open(ORACLE, "a").write("\n")
print(f"dropped {dropped} stale -uncarried requirement(s)")
