"""✏️ Round 3 2D of humans/skateboarder (ruling E1): every scale and view redrawn as the orthographic view of that scale's handcrafted body, one shape per drawn part with the same id in all three views, the paint order taken from the depth of the default pose.

The rider stands across the board, so his bones turn out of the rig plane: arm and leg segments are revolved solids (their round section seen the same from every side), trunk, seat, head, beanie, hands and shoes are their outlines swept through their real depth along the bone axis that looks most towards the viewer in the default pose (`w`: the side profile swept across, `v`: the front outline swept through the body), so every pose and clip carries them in all three views; the board, trucks and wheels stay in the rig plane and are plain outlines. Reads the candidate document and rewrites its `representations[*].geometry`, decisions, dimensions and texts in place.
Usage: python3 r3-skateboarder-draw2d.py
"""
import glob, importlib.util, json, math, os, subprocess

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated", "r3-skateboarder")


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-skateboarder-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


B = mod("body50")
C = mod("coarse")
FK = mod("fk")
TX = mod("texts")
R, K = B.R, B.K
VIEWS = ("elevation", "front", "plan")
DIR = {"elevation": (0.0, 0.0, -1.0), "front": (1.0, 0.0, 0.0), "plan": (0.0, 1.0, 0.0)}
BOARD = ("board", "wheel-f", "wheel-r")
LIMB = ("upper-arm-", "forearm-", "thigh-", "shank-")
DIMENSIONS = {"heightM": 1.846, "widthM": 0.813, "depthM": 0.345}
TRUNK = ("pelvis", "torso", "head")
BEANIE_FRONT = -0.07
PARENT = {R.GIRDLE: "torso", R.GAZE: "head"}
ACROSS = {"torso": (R.GIRDLE, R.GIRDLE_X), "head": (R.GAZE, R.GAZE_X)}
CUTS = {"hoodie": (0.04, 0.17, 0.30, 0.38, 0.45), "head": (0.06, 0.12, 0.17, 0.21), "beanie": (0.18, 0.21, 0.24)}
PRIORITY = {"beanie": 0.6, "eye": 0.7, "hood": -0.2, "wrist-l": -0.05, "wrist-r": -0.05}


def bodies():
    return {50: B.build(), 100: C.body100(), 200: C.body200(), 500: C.body500()}


def centroid(s):
    k = s["kind"]
    if k == "loft":
        pts = [q["at"] for q in s["sections"]]
        return [sum(p[i] for p in pts) / len(pts) for i in range(3)]
    if k == "capsule":
        return [(s["p0"][i] + s["p1"][i]) / 2 for i in range(3)]
    if k == "revolve":
        a = sum(p[0] for p in s["profile"]) / len(s["profile"])
        return [s["origin"][i] + s["axis"][i] * a for i in range(3)]
    if k == "extrude":
        px = sum(p[0] for p in s["profile"]) / len(s["profile"])
        py = sum(p[1] for p in s["profile"]) / len(s["profile"])
        m = (s["from"] + s["to"]) / 2
        return {"xy": [px, py, m], "zx": [py, m, px], "yz": [m, px, py]}[s["plane"]]
    return list(s.get("transform", {}).get("translation", [0, 0, 0]))


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def shape(i, bone, role, z, tone, region, material, **geometry):
    s = {"id": i, "bone": bone, "role": role, "z": round(z, 3), "filled": geometry.pop("filled", True), "tone": round(tone, 4), "region": region, "material": material}
    s.update(geometry)
    return s


def limb_profile(solids, offsets=True):
    """🦵 The half profile of a limb group revolved about its bone axis: the envelope over every section of its lofts of the mean section radius plus half the section's distance from the axis (the mean section centre)."""
    secs = [q for s in solids if s["kind"] == "loft" for q in s["sections"]]
    av = sum(q["at"][1] for q in secs) / len(secs)
    aw = sum(q["at"][2] for q in secs) / len(secs)
    us = sorted({round(q["at"][0], 4) for q in secs})
    prof = []
    for u in us:
        best = 0.0
        for s in solids:
            ss = s["sections"]
            if not ss[0]["at"][0] - 1e-9 <= u <= ss[-1]["at"][0] + 1e-9:
                continue
            for a, b in zip(ss, ss[1:]):
                if a["at"][0] - 1e-9 <= u <= b["at"][0] + 1e-9:
                    f = 0.0 if b["at"][0] == a["at"][0] else (u - a["at"][0]) / (b["at"][0] - a["at"][0])
                    r = (a["rx"] + a["ry"]) / 2 * (1 - f) + (b["rx"] + b["ry"]) / 2 * f
                    cv = a["at"][1] * (1 - f) + b["at"][1] * f
                    cw = a["at"][2] * (1 - f) + b["at"][2] * f
                    best = max(best, r + (0.5 * math.hypot(cv - av, cw - aw) if offsets else 0.0))
        prof.append((u, best))
    pts = [[us[0], round(av, 4)]] + [[round(u, 4), round(av + r, 4)] for u, r in prof] + [[us[-1], round(av, 4)]]
    return pts, round(av, 4), round(aw, 4)


def main():
    path = os.path.join(OUT, "🔣️.json")
    doc = json.load(open(path))
    a = doc["asset"]
    pose = next(p for p in a["poses"] if p["id"] == a["presentation"]["defaultPose"])
    posed = FK.chain(a["rig"], pose["bones"], pose.get("root", {}))
    rest = FK.chain(a["rig"], {}, {})
    old = json.load(open(os.path.join(OUT, "asset.before.json")))["asset"]["representations"]
    jobs, plan, shapes, columns = [], [], {}, []
    for den, body in bodies().items():
        rep = a["representations"][str(den)]
        pens = rep["pens"]
        fills = [s for s in body if not s.get("slender") and not s.get("texture")]
        groups = {}
        for s in fills:
            groups.setdefault((PARENT.get(s["bone"], s["bone"]), s["_g"]), []).append(s)
        for view in VIEWS:
            out = shapes.setdefault((den, view), [])
            d = DIR[view]

            def depth(solids):
                ws = [FK.pt(posed[s["bone"]], *centroid(s)) for s in solids]
                c = [sum(w[i] for w in ws) / len(ws) for i in range(3)]
                return dot(c, d)

            for (bone, g), solids in groups.items():
                rs = {s["role"] for s in solids}
                role = "body" if "body" in rs or not pens.get("detail") else "detail"
                zz = 5 + 12 * depth(solids) + (0.004 if role == "detail" else 0.0) + PRIORITY.get(g.rsplit("-", 1)[0] if g.startswith("eye-") else g, 0.0)
                base = dict(i=g, bone=bone, role=role, z=zz, tone=solids[0]["tone"], region=solids[0]["_region"], material=solids[0]["material"])
                key = f"{den}|{view}|{bone}|{g}"
                ids = [s["id"] for s in solids]
                if bone in BOARD:
                    job = {"key": key, "scale": den, "view": view, "solids": ids}
                    if view == "elevation":
                        job["bone"] = bone
                    jobs.append(job)
                    plan.append((key, base, "plain"))
                elif bone.startswith(LIMB):
                    pts, av, aw = limb_profile(solids, g != "beanie")
                    rv, rw = rest[bone]["v"], rest[bone]["w"]
                    sh = shape(g, bone, role, zz, base["tone"], base["region"], base["material"], kind="poly", points=pts, closed=True)
                    sh["solid"] = {"kind": "revolved", "axisBone": bone, "axis": "u", "axisX": round(av * rv[0] + aw * rw[0], 4), "axisZ": round(av * rv[2] + aw * rw[2], 4), "mirror": True}
                    out.append(sh)
                    continue
                elif view == "plan" and g.startswith("eye-"):
                    continue
                elif view == "plan" and den <= 100 and bone in ACROSS:
                    helper, x0 = ACROSS[bone]
                    cuts = [-1.0] + [c - x0 for c in CUTS.get(g, ())] + [1.0]
                    n = len(cuts) - 1
                    for k in range(n):
                        i = g if k == n - 1 else f"{g}-under-{k + 1}"
                        kk = f"{key}|{i}"
                        jobs.append({"key": kk, "scale": den, "view": "lateral", "solids": ids, "bone": helper, "sweep": True, "vRange": [cuts[k], cuts[k + 1]]})
                        plan.append((kk, {**base, "i": i, "bone": helper, "z": zz - 0.03 * (n - 1 - k)}, "swept-v"))
                        out.append(kk)
                    continue
                elif view == "plan" and bone in TRUNK:
                    jobs.append({"key": f"{key}|uv", "scale": den, "view": "elevation", "solids": ids, "bone": bone, "sweep": True, "tol": 0.8})
                    jobs.append({"key": f"{key}|uw", "scale": den, "view": "lateral", "solids": ids, "bone": bone, "tol": 0.8})
                    columns.append((key, base, rest[bone], 0.03 * den / 50))
                    out.append(key)
                    continue
                else:
                    draw = "torso" if bone == R.GIRDLE else bone
                    f = posed[draw]
                    how = "swept-w" if abs(dot(f["w"], d)) >= abs(dot(f["v"], d)) else "swept-v"
                    job = {"key": key, "scale": den, "view": "elevation" if how == "swept-w" else "lateral", "solids": ids, "bone": draw, "sweep": True}
                    if g == "beanie" and how == "swept-v":
                        job["vRange"] = [-1.0, BEANIE_FRONT]
                    jobs.append(job)
                    plan.append((key, {**base, "bone": draw}, how))
                    out.append(key)
                    continue
                out.append(key)
    outlines, spans, split = project(path, jobs, "outlines")
    if split:
        byjob = {j["key"]: j for j in jobs}
        more, plan2 = [], []
        for key, base, how in plan:
            if key not in split:
                plan2.append((key, base, how))
                continue
            subs = []
            for sid in byjob[key]["solids"]:
                k2 = f"{key}|{sid}"
                more.append({**byjob[key], "key": k2, "solids": [sid]})
                plan2.append((k2, {**base, "i": sid}, how))
                subs.append(k2)
            for lst in shapes.values():
                if key in lst:
                    i = lst.index(key)
                    lst[i:i + 1] = subs
        o2, s2, sp2 = project(path, more, "outlines-split")
        outlines.update(o2)
        spans.update(s2)
        for k in sp2:
            print("SPLIT AGAIN", k)
        plan = plan2
    made = {}
    for key, base, how in plan:
        sh = shape(base["i"], base["bone"], base["role"], base["z"], base["tone"], base["region"], base["material"], kind="poly", points=outlines[key], closed=True)
        if how.startswith("swept"):
            lo, hi, lo1, hi1 = spans[key][0:4]
            if lo is None or lo > hi:
                lo, hi = spans[key][4], spans[key][5]
            if lo1 is None or lo1 > hi1:
                lo1, hi1 = spans[key][4], spans[key][5]
            lo, hi, lo1, hi1 = [round(q, 4) for q in (lo, hi, lo1, hi1)]
            if abs(lo1 - lo) < 0.002:
                lo1 = round(lo + 0.0021, 4)
            if abs(hi1 - hi) < 0.002:
                hi1 = round(hi - 0.0023, 4)
            sh["solid"] = {"kind": "extruded", "sweep": "w" if how == "swept-w" else "v", "z0": lo, "z1": hi, "zTip": [lo1, hi1]}
        made[key] = sh
    for key, base, frame, step in columns:
        made[key] = trunk_columns(base, outlines[f"{key}|uv"], outlines[f"{key}|uw"], frame, step)
    for (den, view), lst in shapes.items():
        rep = a["representations"][str(den)]
        new = []
        for k in lst:
            m = made[k] if isinstance(k, str) else k
            new.extend(m if isinstance(m, list) else [m])
        for sh in new:
            if den == 50 and view == "elevation" and sh["id"].startswith("beanie"):
                sh["hatch"] = "knit-rib"
            if den == 500 and sh["id"] == "deck":
                sh.pop("region")
                sh["regions"] = ["deck", "trucks", "wheels"]
        ground = [s for s in old[str(den)]["geometry"][view] if s.get("role") == "ground"]
        rep["geometry"][view] = ground + sorted(new, key=lambda s: s["z"])
    TX.apply(a)
    a["dimensions"].update(DIMENSIONS)
    json.dump(doc, open(path, "w"), ensure_ascii=False, indent=1)
    print("shapes", {f"{d}|{v}": len(l) for (d, v), l in shapes.items()})


def reach(poly, u):
    """📏 The lowest and highest crossing of a closed outline with the line of constant u."""
    ys = []
    for (x0, y0), (x1, y1) in zip(poly, poly[1:] + poly[:1]):
        if (x0 - u) * (x1 - u) <= 0 and x0 != x1:
            ys.append(y0 + (y1 - y0) * (u - x0) / (x1 - x0))
    return (min(ys), max(ys)) if ys else None


def trunk_columns(base, uv, uw, frame, step):
    """🏛️ A trunk part for the plan as bodies of revolution about axes parallel to its bone: one column on the bone axis and, where the part is clearly wider than deep or deeper than wide, one more column on each side or before and behind it that carries the round end out to the part's reach; stated from the part's side and front outlines, so the plan is round where the body is round and follows the bone in every pose."""
    us = [p[0] for p in uv]
    lo, hi = min(us), max(us)
    n = max(4, min(14, round((hi - lo) / step)))
    rows = []
    for k in range(n + 1):
        u = lo + (hi - lo) * (0.02 + 0.96 * k / n)
        a, b = reach(uv, u), reach(uw, u)
        if a and b:
            rows.append((u, a, b))
    v0, v1 = min(r[1][0] for r in rows), max(r[1][1] for r in rows)
    w0, w1 = min(r[2][0] for r in rows), max(r[2][1] for r in rows)
    return rows_columns(base, rows, frame, True)


def rows_columns(base, rows, frame, split):
    """🧱 The columns of one run of stations; a part that drifts across its bone axis by more than 10 mm (a slouching beanie, a leaning trunk) is stated as a lower and an upper run, each about its own axis."""
    cv = [(r[1][0] + r[1][1]) / 2 for r in rows]
    cw = [(r[2][0] + r[2][1]) / 2 for r in rows]
    if split and len(rows) >= 6 and max(cv) - min(cv) > 0.02:
        h = len(rows) // 2
        return rows_columns({**base, "i": f"{base['i']}-low", "z": base["z"] - 0.004}, rows[: h + 1], frame, False) + rows_columns(base, rows[h:], frame, False)
    av, aw = sum(cv) / len(cv), sum(cw) / len(cw)
    hv = [(r[1][1] - r[1][0]) / 2 + abs(c - av) / 2 for r, c in zip(rows, cv)]
    hw = [(r[2][1] - r[2][0]) / 2 + abs(c - aw) / 2 for r, c in zip(rows, cw)]

    def column(i, z, dv, dw, radii):
        keep = [(r[0], q) for r, q in zip(rows, radii) if q > 0.006]
        pts = [[round(keep[0][0], 4), round(av + dv, 4)]] + [[round(u, 4), round(av + dv + q, 4)] for u, q in keep] + [[round(keep[-1][0], 4), round(av + dv, 4)]]
        sh = shape(i, base["bone"], base["role"], z, base["tone"], base["region"], base["material"], kind="poly", points=pts, closed=True)
        sh["solid"] = {"kind": "revolved", "axisBone": base["bone"], "axis": "u", "axisX": round((av + dv) / frame["v"][0], 4), "axisZ": round(aw + dw, 4), "mirror": True}
        return sh

    i, z = base["i"], base["z"]
    across = max(hw) - 0.8 * max(hv)
    along = max(hv) - 0.8 * max(hw)
    if across > 0.25 * max(hv):
        ends = [min(w - across, v) for v, w in zip(hv, hw)]
        return [column(f"{i}-l", z - 0.002, 0.0, across, ends), column(f"{i}-r", z - 0.002, 0.0, -across, ends), column(i, z, 0.0, 0.0, [min(v, w) for v, w in zip(hv, hw)])]
    if along > 0.25 * max(hw):
        ends = [min(v - along, w) for v, w in zip(hv, hw)]
        return [column(f"{i}-front", z - 0.002, -along, 0.0, ends), column(f"{i}-back", z - 0.002, along, 0.0, ends), column(i, z, 0.0, 0.0, [min(v, w) for v, w in zip(hv, hw)])]
    return [column(i, z, 0.0, 0.0, [(v + w) / 2 for v, w in zip(hv, hw)])]


def project(path, jobs, name):
    """🖨️ Runs the outline projector on a job list; returns the outlines, the swept spans and the keys whose solids fell apart into several outlines."""
    json.dump(jobs, open(os.path.join(OUT, f"{name}.jobs.json"), "w"))
    bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
    out = os.path.join(OUT, f"{name}.json")
    subprocess.run([bun, os.path.join(T, "r3-skateboarder-project2d.ts"), path, os.path.join(OUT, f"{name}.jobs.json"), out], check=True, cwd=T, stdout=subprocess.DEVNULL)
    return json.load(open(out)), json.load(open(f"{out}.spans.json")), set(json.load(open(f"{out}.split.json")))


if __name__ == "__main__":
    main()
