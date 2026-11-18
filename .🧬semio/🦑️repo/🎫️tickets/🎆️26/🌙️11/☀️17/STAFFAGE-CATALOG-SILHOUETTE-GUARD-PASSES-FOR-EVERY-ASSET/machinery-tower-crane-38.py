"""Tower crane: the validator wants no flat shape on the rotating bones jib, counter-jib, slewing, skip and hoist-block in the elevation and front. The generated strokes of the tower head, pendants, jib, counter-jib and slings (1:200, 1:100, 1:50) become `member` shapes computed from the volume bars (machinery-bars.ts: principal axis of each bar copy in bone coordinates, projected duplicates of a view dropped; end-on bars take a diameter that keeps their round hull within the guard limit of the volume's square dot), the front skip body becomes a stack of extruded slices following its taper, the elevation hook gets its extruded solid, and the structure lists get one member entry per extra member shape."""
import sys, json, subprocess, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
SPREAD = ("jib", "counter-jib", "slewing", "skip")
REGEX = "^(jib-(chord|rail|diag|brace|end)|counter|tower-head|pendant|sling)-"
DOT = {50: 0.065, 100: 0.13, 200: 0.26}
tmpl = d.obj(d.shape(500, "elevation", "tower-head-leg-1"))
counts = {}
def bars(scale):
    out = f"{lib.T}/🗑️generated/tc-bars{scale}.json"
    subprocess.run([lib.B, f"{lib.T}/machinery-bars.ts", "tower-crane", str(scale), out, REGEX], cwd=lib.PK, check=True, capture_output=True)
    return json.load(open(out))
def proj(view, b, p):
    sgn = -1 if b["bone"] == "counter-jib" else 1
    return (p[0] * sgn, p[1] * sgn) if view == "elevation" else (p[2], p[1] * sgn)
def members(scale, view, bl):
    seen = {}
    for b in bl:
        a, c = proj(view, b, b["from"]), proj(view, b, b["to"])
        if b["id"].startswith("jib-tip"): continue
        length = math.dist(a, c)
        dd = b["d"]
        if length < 3 * dd: dd = min(dd, DOT[scale])
        dd = min(dd, 2.0 * scale / 1000)
        key = (b["bone"], tuple(sorted([(round(a[0], 3), round(a[1], 3)), (round(c[0], 3), round(c[1], 3))])), round(dd, 3))
        if key in seen: continue
        solid, k = b["id"].split("#")
        o = dict(tmpl); o["id"] = f"{solid}-{int(k) + 1}"; o["bone"] = b["bone"]
        o["from"] = {"x": b["from"][0], "y": b["from"][1], "z": b["from"][2]}; o["to"] = {"x": b["to"][0], "y": b["to"][1], "z": b["to"][2]}
        o["section"] = {"kind": "circle", "d": round(dd, 4)}; o["penMm"] = round(dd * 1000 / scale, 4)
        seen[key] = o
    ids = set(); out = []
    for o in seen.values():
        while o["id"] in ids: o["id"] += "x"
        ids.add(o["id"]); out.append(o)
    return out
def flat(x): return "solid" not in x and x["kind"] != "member" and x["bone"] in SPREAD
for scale in (200, 100, 50):
    bl = bars(scale)
    for view in ("elevation", "front"):
        geo = d.val("asset", "representations", str(scale), "geometry", view)
        anchor = d.shape(scale, view, "slew-platform")
        for x in geo:
            if flat(x) and x["id"] != "skip-body" and x["id"] != "hook": d.delete_item(d.shape(scale, view, x["id"]))
        ms = members(scale, view, bl)
        for m in reversed(ms): d.insert_after(anchor, m)
        for m in ms: counts.setdefault(scale, {}).setdefault(m["bone"], {}).setdefault(view, 0); counts[scale][m["bone"]][view] += 1
    n = d.shape(scale, "elevation", "hook"); o = d.obj(n)
    o["solid"] = {"kind": "extruded", "sweep": "w", "z0": -0.04, "z1": 0.04}
    d.replace_obj(n, o)
for scale in (500, 200, 100, 50):
    n = d.shape(scale, "front", "skip-body"); o = d.obj(n)
    N = {500: 3, 200: 6, 100: 12, 50: 24}[scale]
    sl = []
    for i in range(N):
        yb, yt = -1 + i / N, -1 + (i + 1) / N
        hw = 0.45 + 0.25 * ((yb + yt) / 2 + 1)
        s = {k: v for k, v in o.items() if k not in ("kind", "points", "closed")}
        s.update({"id": f"skip-body-{i + 1}", "kind": "rect", "x": round(-hw, 4), "y": round(yb, 4), "w": round(2 * hw, 4), "h": round(yt - yb, 4), "solid": {"kind": "extruded", "sweep": "w", "z0": round(-hw, 4), "z1": round(hw, 4)}})
        sl.append({k: s[k] for k in ("id", "bone", "role", "z", "filled", "tone", "kind", "x", "y", "w", "h", "region", "material", "solid") if k in s})
    d.replace_obj(n, sl[0])
    for s in reversed(sl[1:]): d.insert_after(n, s)
for scale in (500, 200, 100, 50):
    for view in ("elevation", "front", "plan"):
        per = {}
        for x in d.val("asset", "representations", str(scale), "geometry", view):
            if x["kind"] == "member": per[x["bone"]] = per.get(x["bone"], 0) + 1
        for bone, n_ in per.items(): counts.setdefault(scale, {}).setdefault(bone, {})[view + "_all"] = n_
for scale, per in counts.items():
    arr = d.at("asset", "volumes", str(scale), "structure")
    items = d.val("asset", "volumes", str(scale), "structure")
    for bone, v in per.items():
        need = max(v.values())
        have = [i for i, s in enumerate(items) if s.get("part") == "member" and s.get("member") == bone]
        if len(have) >= need or not have: print(scale, bone, "need", need, "have", len(have)); continue
        base = items[have[-1]]; node = arr["items"][have[-1]]
        for k in range(need - len(have)):
            e = dict(base); e["id"] = f"member-{bone}-extra-{k + 1}"
            d.insert_after(node, e)
        print(scale, bone, "need", need, "have", len(have), "added", need - len(have))
d.commit()
