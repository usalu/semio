"""🧱 Builds the round 3 candidate of `machinery/concrete-mixer-truck`: the catalog asset re-read, its root fixes (dimensions, rig, anchors, poses, clips, provenance, 2D drawings of every scale) and the handcrafted volumes of every scale. Writes the candidate document and the volumes source; `--apply` writes the root fixes into the catalog file (member by member, the volumes stay for `volumes-insert.py`)."""

import copy
import json
import os
import re
import sys
from importlib import util as _u

T = os.path.dirname(os.path.abspath(__file__))


def mod(name):
    s = _u.spec_from_file_location(name, os.path.join(T, f"r3-concrete-mixer-truck-{name}.py"))
    m = _u.module_from_spec(s)
    s.loader.exec_module(m)
    return m


K = mod("kit")
ROOTS = mod("roots")
DEC = mod("decisions").DEC
SCALES = {n: (mod(f"d{n}"), mod(f"s{n}")) for n in (50, 100, 200, 500) if os.path.exists(os.path.join(T, f"r3-concrete-mixer-truck-s{n}.py"))}
ONLY = [int(a) for a in sys.argv[1:] if a.isdigit()]

text = open(K.CATALOG, encoding="utf8").read()
doc = json.loads(text)
asset = doc["asset"]
before = copy.deepcopy(asset)
ROOTS.apply(asset)
V = K.Kit()
for scale, (D, S) in SCALES.items():
    rep = asset["representations"][str(scale)]
    rep["geometry"] = {"elevation": D.elevation(), "front": D.front(), "plan": D.plan()}
    for shapes in rep["geometry"].values():
        for sh in shapes:
            if "region" in sh:
                sh["region"] = K.region_of(sh["id"], sh["region"])
    rep["decisions"] = DEC[scale]
    V.scale = scale
    S.build(V)
    V.finish(scale)
asset["volumes"] = {k: V.all.get(k, before["volumes"][k]) for k in ["50", "100", "200", "500"]}
import runpy
flights = runpy.run_path(os.path.join(T, "mixer-flights.py"))
flights["update_volumes"](asset["volumes"])
flights["update_rig"](asset)
flights["update_text"](asset)
flights["update_ink"](asset)
os.makedirs(os.path.dirname(K.CANDIDATE), exist_ok=True)
json.dump(doc, open(K.CANDIDATE, "w", encoding="utf8"), ensure_ascii=False)
json.dump({"volumes": asset["volumes"]}, open(K.SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
print({k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in asset["volumes"].items()})
print({k: {w: len(s) for w, s in asset["representations"][k]["geometry"].items()} for k in asset["volumes"]})

if "--apply" in sys.argv:
    fresh = open(K.CATALOG, encoding="utf8").read()
    if fresh != text:
        raise SystemExit("catalog file changed while building; run again")
    indent = "  "
    out = fresh
    changed = [k for k in asset if k != "volumes" and asset[k] != before[k]]
    for key in changed:
        found = re.search(r"\n" + indent + '"' + key + r'"\s*:\s*', out)
        start = found.end()
        depth, i, quoted = 0, start, False
        if out[start] not in "{[":
            end = min(out.index(",\n", start), out.index("\n", start))
        else:
            while True:
                ch = out[i]
                if quoted:
                    if ch == "\\":
                        i += 1
                    elif ch == '"':
                        quoted = False
                elif ch == '"':
                    quoted = True
                elif ch in "{[":
                    depth += 1
                elif ch in "}]":
                    depth -= 1
                    if depth == 0:
                        break
                i += 1
            end = i + 1
        out = out[:start] + K.dump(asset[key], 2) + out[end:]
    check = json.loads(out)
    want = {k: v for k, v in asset.items() if k != "volumes"}
    got = {k: v for k, v in check["asset"].items() if k != "volumes"}
    if want != got or list(check["asset"]) != list(before) or check["asset"]["volumes"] != before["volumes"]:
        raise SystemExit("root edit did not round-trip")
    open(K.CATALOG, "w", encoding="utf8").write(out)
    print("catalog roots written:", ", ".join(changed))
