"""🧹 Normalises ops before they are applied to the file: a `shape` replacement keeps the key order of the shape in the file (same kind and keys: a `shape-set` of the geometry only; other: keys in the original order, solid and rotation dropped when the kind changes); ops on shapes added by the same batch are folded into the add. python3 humansb-A-norm.py <asset> <in.json> <out.json>"""
import json, sys
a, fin, fout = sys.argv[1:4]
d = json.load(open(f'/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/{a}/🔣️.json'))['asset']
GEO = {"kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "d", "points", "closed", "followTip", "rotationDeg"}
out = []
anchor = {}
added = {}
def merge(ops):
    seen = {}
    res = []
    for o in ops:
        if o["op"] in ("shape", "shape-set", "solid", "solid-set"):
            k = (o["scale"], o.get("view"), o["id"], o["op"].split("-")[0])
            if k in seen:
                p = res[seen[k]]
                if o["op"] in ("shape", "solid"):
                    res[seen[k]] = o
                elif p["op"] in ("shape", "solid"):
                    p[o["op"].split("-")[0]].update(o["set"])
                else:
                    p["set"].update(o["set"])
                continue
            seen[k] = len(res)
        res.append(o)
    return res
for o in merge(json.load(open(fin))):
    if o["op"] in ("shape-add", "solid-add"):
        kind = "shape" if o["op"] == "shape-add" else "solid"
        key = (o["scale"], o.get("view"), o["after"])
        base = anchor.get(key, o["after"])
        newid = o[kind]["id"]
        anchor[(o["scale"], o.get("view"), newid)] = base
        o = {**o, "after": base, kind: dict(o[kind])}
        out.append(o)
        added[(o["scale"], o.get("view"), newid)] = o
        continue
    if o["op"] in ("shape", "shape-set", "solid", "solid-set"):
        k = (o["scale"], o.get("view"), o["id"])
        if k in added:
            tgt = added[k]
            kind = "shape" if o["op"].startswith("shape") else "solid"
            if o["op"] in ("shape", "solid"):
                tgt[kind] = dict(o[kind])
            else:
                tgt[kind].update(o["set"])
            continue
    if o["op"] != "shape":
        out.append(o); continue
    old = next((s for s in d["representations"][o["scale"]]["geometry"][o["view"]] if s["id"] == o["id"]), None)
    new = o["shape"]
    if old is None:
        out.append(o); continue
    if old["kind"] == new["kind"] and new["kind"] in ("path", "poly") and {k for k in old if k not in GEO} == {k for k in new if k not in GEO} and old.get("followTip") == new.get("followTip"):
        key = "d" if new["kind"] == "path" else "points"
        out.append({"op": "shape-set", "scale": o["scale"], "view": o["view"], "id": o["id"], "set": {key: new[key]}})
        continue
    ordered = {}
    for k, v in old.items():
        if k == "kind":
            ordered["kind"] = new["kind"]
            for g in ("d", "points", "closed"):
                if g in new: ordered[g] = new[g]
        elif k in GEO or (k == "solid" and "solid" not in new):
            continue
        else:
            ordered[k] = v
    out.append({**o, "shape": ordered})
json.dump(out, open(fout, "w"))
