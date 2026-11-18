"""Rewrites the main filled drawn shape of every vegetation bone whose drawing disagrees with the outline of its volume (trace of vegetation-trace.ts), only in the views and scales that fail the guard.
python3 vegetation-apply.py <slug> <trace.json> <iou> <hf> [--dry] [--force=all|--force=<scale>/<view>] [--only=<bone,...>]
python3 vegetation-apply.py <slug> --corrections <corrections.json>"""
import json, re, sys, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib")
slug = sys.argv[1]
HERE = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET/vegetation-work/"
LIMIT = {"50": 0.015, "100": 0.03, "200": 0.06, "500": 0.15}
KEYS = {"path": ("d",), "poly": ("points", "closed"), "rect": ("x", "y", "w", "h", "r"), "ellipse": ("cx", "cy", "rx", "ry"), "capsule": ("x0", "y0", "x1", "y1", "r0", "r1")}
GEOM = sum((KEYS[x] for x in KEYS), ()) + ("rotationDeg",)
d = mv.D("🌳️vegetation/" + slug)
a = d.val("asset")
def bbox(s):
    k = s["kind"]
    if k == "path":
        n = [float(x) for x in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", s["d"])]; xs, ys = n[0::2], n[1::2]
    elif k == "poly": xs = [p[0] for p in s["points"]]; ys = [p[1] for p in s["points"]]
    elif k == "rect": xs = [s["x"], s["x"] + s["w"]]; ys = [s["y"], s["y"] + s["h"]]
    elif k == "ellipse": xs = [s["cx"] - s["rx"], s["cx"] + s["rx"]]; ys = [s["cy"] - s["ry"], s["cy"] + s["ry"]]
    else: return 0
    return (max(xs) - min(xs)) * (max(ys) - min(ys))
def parse(rings): return [[tuple(float(v) for v in p.split(" ")) for p in r.split(";")] for r in rings]
def dstr(rings):
    f = lambda v: ("%.4f" % v).rstrip("0").rstrip(".") if abs(v) > 5e-5 else "0"
    return " ".join("M " + " L ".join(f(x) + " " + f(y) for x, y in r) + " Z" for r in rings)
def pts(r): return [[round(x, 4), round(y, 4)] for x, y in r]
def area(r): return abs(sum(r[i][0] * r[(i + 1) % len(r)][1] - r[(i + 1) % len(r)][0] * r[i][1] for i in range(len(r)))) / 2
def write(sc, view, main, rings):
    node = d.shape(sc, view, main["id"])
    kind = main["kind"]

    if "solid" in main:
        o = {k: v for k, v in main.items() if k not in GEOM and k != "solid"}
        if len(rings) == 1: o["kind"] = "poly"; o["points"] = pts(rings[0]); o["closed"] = True
        else: o["kind"] = "path"; o["d"] = dstr(rings)
        order = [k for k in main if k in o] + [k for k in o if k not in main]
        d.replace_obj(node, {k: o[k] for k in order})
        return
    if kind == "path": d.set(node, "d", dstr(rings))
    elif kind == "poly" and len(rings) == 1 and main.get("closed") is True: d.set(node, "points", pts(rings[0]))
    else:
        o = {k: v for k, v in main.items() if k not in GEOM}
        if len(rings) == 1: o["kind"] = "poly"; o["points"] = pts(rings[0]); o["closed"] = True
        else: o["kind"] = "path"; o["d"] = dstr(rings)
        order = [k for k in main if k in o] + [k for k in o if k not in main]
        d.replace_obj(node, {k: o[k] for k in order})
def prep(shape, rings, target):
    if shape.get("followTip") is True and len(rings) > 1:
        k = max(range(len(rings)), key=lambda i: area(rings[i])); return [rings[k]], [target[k]]
    return rings, target
def spill(sc, view, shape, rings, target, edits, report):
    if shape.get("followTip") is not True or len(rings) < 2: return
    k = max(range(len(rings)), key=lambda i: area(rings[i]))
    last = d.shape(sc, view, shape["id"])
    n = 0
    for i, r in enumerate(rings):
        if i == k or area(r) < 1e-9: continue
        n += 1
        o = {x: v for x, v in shape.items() if x not in GEOM and x not in ("followTip", "solid", "id")}
        pid = "%s-part-%d" % (shape["id"], n)
        have = next((s for s in a["representations"][sc]["geometry"][view] if s["id"] == pid), None)
        if have is not None:
            if (sc, view, pid) in globals().setdefault("TOUCHED", set()): continue
            globals()["TOUCHED"].add((sc, view, pid))
            write(sc, view, have, [r])
            edits.append({"scale": sc, "view": view, "id": pid, "bone": shape["bone"], "target": [target[i]]})
            continue
        new = {"id": pid}; new.update(o); new["kind"] = "poly"; new["points"] = pts(r); new["closed"] = True
        d.insert_after(last, new)
        edits.append({"scale": sc, "view": view, "id": new["id"], "bone": shape["bone"], "target": [target[i]]})
        report.append((sc, view, shape["bone"], "SPILL", new["id"], "pts %d" % len(r)))
def shapeof(sc, view, sid): return next(s for s in a["representations"][sc]["geometry"][view] if s["id"] == sid)
if sys.argv[2] == "--corrections":
    for c in json.load(open(sys.argv[3])):
        write(c["scale"], c["view"], shapeof(c["scale"], c["view"], c["id"]), parse(c["rings"]))
    d.commit(); print("corrected", len(d.edits)); sys.exit()
T = json.load(open(sys.argv[2]))
IOU = float(sys.argv[3]); HF = float(sys.argv[4])
DRY = "--dry" in sys.argv
THIN = float(([x[7:] for x in sys.argv if x.startswith("--thin=")] or ["1.5"])[0])
FORCE = [x for x in sys.argv if x.startswith("--force=")]
ONLY = [x[7:].split(",") for x in sys.argv if x.startswith("--only=")]
VIEWS = [x[8:].split(",") for x in sys.argv if x.startswith("--views=")]
DROP = False
report, edits = [], []
for sc in [s for s in ("50", "100", "200", "500") if s in T]:
    for view in ("elevation", "front", "plan"):
        bones = T[sc][view]
        if "section" in bones: continue
        if VIEWS and sc + "/" + view not in VIEWS[0]: continue
        info = bones["viewFail"]
        if not info["fail"] and not any(f in ("--force=" + sc + "/" + view, "--force=all") for f in FORCE): continue
        geo = a["representations"][sc]["geometry"][view]
        done = set()
        touched = globals().setdefault("TOUCHED", set())
        for bone, t in bones.items():
            if bone in ("viewFail", "clips", "strokes", "slenders") or not t["rings"]: continue
            if ONLY and bone not in ONLY[0]: continue
            if t.get("thick", 1) < THIN * LIMIT[sc] / 2:
                continue
            h = max(t["miss"], t["extra"])
            if h <= HF * LIMIT[sc] and t["iou"] >= IOU: continue
            cands = [s for s in geo if s["bone"] == bone and s.get("filled") is True and s["role"] in ("body", "detail", "glazing") and s["kind"] in KEYS]
            if not DROP and (any("solid" in s for s in cands) or any("solid" in s for s in geo if s["bone"] == bone)):
                report.append((sc, view, bone, "SOLID-DRAWN SKIP", "iou %s miss %s extra %s" % (t["iou"], t["miss"], t["extra"]))); continue
            rings = parse(t["rings"])
            if not cands:
                if h <= HF * LIMIT[sc]: continue
                solid = next(s for s in a["volumes"][sc]["body"] if s["bone"] == bone)
                zs = [s["z"] for s in geo]
                new = {"id": bone + "-mass", "bone": bone, "role": solid["role"] if solid["role"] in ("body", "detail") else "body", "z": round(min(zs) - 0.1, 2), "filled": True, "tone": solid.get("tone", 0.5)}
                if len(rings) == 1: new.update({"kind": "poly", "points": pts(rings[0]), "closed": True})
                else: new.update({"kind": "path", "d": dstr(rings)})
                last = [s for s in geo if s["bone"] == bone] or geo
                d.insert_after(d.shape(sc, view, last[-1]["id"]), new)
                edits.append({"scale": sc, "view": view, "id": new["id"], "bone": bone, "target": t["target"]})
                report.append((sc, view, bone, "INSERT", new["id"], "miss %s extra %s" % (t["miss"], t["extra"]), "pts %d" % sum(len(r) for r in rings)))
                continue
            main = max(cands, key=bbox)
            done.add(main["id"]); touched.add((sc, view, main["id"]))
            spill(sc, view, main, rings, t["target"], edits, report)
            rings, tgt = prep(main, rings, t["target"])
            write(sc, view, main, rings)
            edits.append({"scale": sc, "view": view, "id": main["id"], "bone": bone, "target": tgt})
            report.append((sc, view, bone, main["id"], main["kind"], "iou %s miss %s extra %s" % (t["iou"], t["miss"], t["extra"]), "rings %d pts %d" % (len(rings), sum(len(r) for r in rings))))
        for c in bones.get("clips", []):
            if c["id"] in done or (sc, view, c["id"]) in touched or ONLY and c["bone"] not in ONLY[0]: continue
            if c.get("thick", 1) < THIN * LIMIT[sc] / 2: continue
            shape = next((s for s in geo if s["id"] == c["id"] and s["bone"] == c["bone"]), None)
            if shape is None: continue
            if not c["rings"]:
                report.append((sc, view, c["bone"], "CLIP-EMPTY", c["id"], "extra %s" % c["extra"])); continue
            touched.add((sc, view, c["id"]))
            rings0 = parse(c["rings"])
            spill(sc, view, shape, rings0, c["target"], edits, report)
            rings, tgt = prep(shape, rings0, c["target"])
            write(sc, view, shape, rings)
            edits.append({"scale": sc, "view": view, "id": c["id"], "bone": c["bone"], "target": tgt})
            report.append((sc, view, c["bone"], "CLIP", c["id"], shape["kind"], "extra %s" % c["extra"]))
        for sl in bones.get("slenders", []):
            if ONLY and sl["bone"] not in ONLY[0]: continue
            if (sc, view, sl["id"]) in touched: continue
            touched.add((sc, view, sl["id"]))
            runs = parse(sl["runs"]); tgt = sl["target"]
            dd = " ".join("M " + " L ".join(("%.4f %.4f" % (x, y)).replace("-0.0000", "0") for x, y in r) for r in runs)
            have = next((s for s in geo if s["id"] == sl["id"]), None)
            if have is not None:
                d.set(d.shape(sc, view, sl["id"]), "d", dd)
            else:
                solid = next(s for s in a["volumes"][sc]["body"] if s["id"] == sl["solid"])
                zs = [s["z"] for s in geo]
                new = {"id": sl["id"], "bone": sl["bone"], "role": "body", "z": round(max(zs) + 0.1, 2), "filled": False, "tone": solid.get("tone", 0.5), "kind": "path", "d": dd}
                last = [s for s in geo if s["bone"] == sl["bone"]] or geo
                d.insert_after(d.shape(sc, view, last[-1]["id"]), new)
            edits.append({"scale": sc, "view": view, "id": sl["id"], "bone": sl["bone"], "target": tgt})
            report.append((sc, view, sl["bone"], "SLENDER-LINE", sl["id"], "runs %d" % len(runs)))
        for st in bones.get("strokes", []):
            if ONLY and st["bone"] not in ONLY[0]: continue
            shape = next((s for s in geo if s["id"] == st["id"] and s["bone"] == st["bone"]), None)
            if shape is None or (sc, view, st["id"]) in touched: continue
            touched.add((sc, view, st["id"]))
            runs = parse(st["runs"])
            node = d.shape(sc, view, shape["id"])
            if not runs:
                d.delete_item(node); report.append((sc, view, st["bone"], "STROKE-DELETE", st["id"])); continue
            if shape["kind"] == "path": d.set(node, "d", " ".join("M " + " L ".join(("%.4f %.4f" % (x, y)).replace("-0.0000", "0") for x, y in r) for r in runs))
            elif shape["kind"] == "poly" and len(runs) == 1 and shape.get("closed") is False: d.set(node, "points", pts(runs[0]))
            elif shape["kind"] == "poly" and len(runs) == 1:
                o = {k: v for k, v in shape.items() if k not in GEOM}
                o["kind"] = "poly"; o["points"] = pts(runs[0]); o["closed"] = False
                order = [k for k in shape if k in o] + [k for k in o if k not in shape]
                d.replace_obj(node, {k: o[k] for k in order})
            else:
                o = {k: v for k, v in shape.items() if k not in GEOM}
                o["kind"] = "path"; o["d"] = " ".join("M " + " L ".join("%.4f %.4f" % (x, y) for x, y in r) for r in runs)
                order = [k for k in shape if k in o] + [k for k in o if k not in shape]
                d.replace_obj(node, {k: o[k] for k in order})
            edits.append({"scale": sc, "view": view, "id": st["id"], "bone": st["bone"], "target": st["target"]})
            report.append((sc, view, st["bone"], "STROKE", st["id"], shape["kind"], "runs %d" % len(runs)))
for r in report: print(*r)
if not DRY:
    try: d.commit()
    except Exception as e:
        import json as _j
        txt = d.t
        for a_, b_, x_ in sorted(d.edits + [(k[0], k[1], v) for k, v in getattr(d, "pend", {}).items()], reverse=True): txt = txt[:a_] + x_ + txt[b_:]
        pos = int(str(e).split("(char ")[1].rstrip(")")) if "(char " in str(e) else 0
        open(HERE + "bad-" + slug + ".json", "w").write(txt)
        print("INVALID JSON", e, repr(txt[max(0, pos - 300):pos + 100]))
        raise
    print("committed", len(d.edits), "edits")
    json.dump(edits, open(HERE + "edits-" + slug + ".json", "w"))
