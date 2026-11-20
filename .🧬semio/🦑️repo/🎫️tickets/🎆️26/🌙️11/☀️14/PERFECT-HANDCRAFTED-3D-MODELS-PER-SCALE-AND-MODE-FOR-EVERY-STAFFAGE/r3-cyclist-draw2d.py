"""✏️ Round 3 2D of humans/cyclist (ruling E1): every scale and view redrawn as the orthographic view of that scale's handcrafted body, one shape per drawn part with the same id in all three views (so front and plan sit at the depth of their elevation twin), the paint order taken from the depth of the default pose.

Filled shapes are the projected outlines of their solids (`r3-cyclist-project2d.ts`): limbs of the rider, whose bones leave the sagittal plane, are solids swept through their real, tapering depth (elevation: across, front and plan: through the sagittal depth); crank, pedal and wheel parts, which turn in the sagittal plane, are their side profiles extruded across their lateral span; frame, pelvis, trunk, neck and head are plain outlines. Slender and texture parts are strokes along their centre lines, wheels are drawn by hand (tyre and rim rings, hub, spokes). Reads the candidate document and rewrites its `representations[*].geometry` and decision texts in place.
Usage: python3 r3-cyclist-draw2d.py
"""
import glob, importlib.util, json, math, os, subprocess

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated", "r3-cyclist")


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-cyclist-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


B = mod("body50")
C = mod("coarse")
FK = mod("fk")
TX = mod("texts")
R, K = B.R, B.K
VIEWS = ("elevation", "front", "plan")
SWEPT = ("arm-", "forearm-", "hand-", "thigh-", "shank-", "foot-", "toe-")
TURNING = ("crank-", "pedal-")
WHEELS = ("rear-wheel", "front-wheel")
PEN_FLOOR = {50: 0.13, 100: 0.13, 200: 0.13, 500: 0.13}
DIMENSIONS = {"widthM": 1.785, "depthM": 0.63}
AREAS = {"bike-metal": 1.62, "tyres": 0.49, "saddle": 0.086}
FAR = {"elevation": ("sleeve-l", "forearm-l", "thigh-l", "shank-l"), "front": ("thigh-l", "shank-l"), "plan": ("thigh-l", "shank-l")}


def bodies():
    return {50: B.build(), 100: C.body100(), 200: C.body200(), 500: C.body500()}


def centroid(s):
    k = s["kind"]
    if k == "loft":
        pts = [q["at"] for q in s["sections"]]
        c = [sum(p[i] for p in pts) / len(pts) for i in range(3)]
    elif k == "capsule":
        c = [(s["p0"][i] + s["p1"][i]) / 2 for i in range(3)]
    elif k == "revolve":
        a = sum(p[0] for p in s["profile"]) / len(s["profile"])
        c = [s["origin"][i] + s["axis"][i] * a for i in range(3)]
    elif k == "extrude":
        px = sum(p[0] for p in s["profile"]) / len(s["profile"])
        py = sum(p[1] for p in s["profile"]) / len(s["profile"])
        m = (s["from"] + s["to"]) / 2
        c = {"xy": [px, py, m], "zx": [py, m, px], "yz": [m, px, py]}[s["plane"]]
    else:
        c = list(s.get("transform", {}).get("translation", [0, 0, 0]))
    if s["kind"] == "extrude" and "transform" in s and "rotation" in s["transform"]:
        q = s["transform"]["rotation"]
        ang = 2 * math.atan2(q[2], q[3])
        c = [c[0] * math.cos(ang) - c[1] * math.sin(ang), c[0] * math.sin(ang) + c[1] * math.cos(ang), c[2]]
    return c


def centre_line(s):
    if s["kind"] == "capsule":
        return [s["p0"], s["p1"]]
    if s["kind"] == "loft":
        return [q["at"] for q in s["sections"]]
    if s["kind"] == "ellipsoid":
        return [s["transform"]["translation"]]
    if s["kind"] == "revolve" and s["bone"] == "frame":
        r = sum(p[1] for p in s["profile"]) / len(s["profile"])
        q = s["transform"]["rotation"]
        a0 = 2 * math.atan2(q[2], q[3])
        sw = s.get("sweepRad", 2 * math.pi)
        o = s["origin"]
        return [[o[0] + r * math.cos(a0 + sw * k / 24), o[1] + r * math.sin(a0 + sw * k / 24), o[2]] for k in range(25)]
    raise ValueError(s["id"])


def world(F, bone, p):
    return FK.pt(F[bone], p[0], p[1], p[2])


def rnd(p):
    return [round(p[0], 4) + 0.0, round(p[1], 4) + 0.0]


def circle(r, reverse=False):
    k = 0.5523 * r
    if not reverse:
        return f"M {r} 0 C {r} {k} {k} {r} 0 {r} C {-k} {r} {-r} {k} {-r} 0 C {-r} {-k} {-k} {-r} 0 {-r} C {k} {-r} {r} {-k} {r} 0 Z"
    return f"M {r} 0 C {r} {-k} {k} {-r} 0 {-r} C {-k} {-r} {-r} {-k} {-r} 0 C {-r} {k} {-k} {r} 0 {r} C {k} {r} {r} {k} {r} 0 Z"


def ring(r0, r1):
    return f"{circle(round(r0, 4))} {circle(round(r1, 4), True)}"


def shape(i, bone, role, z, tone, region, material, **geometry):
    s = {"id": i, "bone": bone, "role": role, "z": round(z, 3), "filled": geometry.pop("filled", True), "tone": round(tone, 4), "region": region, "material": material}
    s.update(geometry)
    return s


def main():
    path = os.path.join(OUT, "🔣️.json")
    doc = json.load(open(path))
    a = doc["asset"]
    rest = FK.chain(a["rig"], {}, {})
    pose = next(p for p in a["poses"] if p["id"] == a["presentation"]["defaultPose"])
    posed = FK.chain(a["rig"], pose["bones"], pose.get("root", {}))
    old = json.load(open(os.path.join(OUT, "asset.before.json")))["asset"]["representations"]
    jobs, plan, notes = [], [], []
    shapes = {}
    for den, body in bodies().items():
        rep = a["representations"][str(den)]
        pens = rep["pens"]
        fills = [s for s in body if not s.get("slender") and not s.get("texture") and s["bone"] not in WHEELS]
        groups = {}
        for s in fills:
            groups.setdefault((s["bone"], s["_g"]), []).append(s)
        for view in VIEWS:
            out = shapes.setdefault((den, view), [])

            def depth(solids):
                ws = [world(posed, s["bone"], centroid(s)) for s in solids]
                d = [sum(w[i] for w in ws) / len(ws) for i in range(3)]
                return -d[2] if view == "elevation" else d[0] if view == "front" else d[1]

            def zof(d, role):
                return 5 + 12 * d + {"body": 0.0, "detail": 0.004, "texture": 0.008}.get(role, 0.0)

            def role_of(solids):
                rs = {s["role"] for s in solids}
                r = "body" if "body" in rs else sorted(rs)[0]
                return r if pens.get(r) else "body"

            for (bone, g), solids in groups.items():
                key = f"{den}|{view}|{bone}|{g}"
                role = role_of(solids)
                base = dict(i=g, bone=bone, role=role, z=zof(depth(solids), role), tone=solids[0]["tone"], region=solids[0]["_region"], material=solids[0]["material"])
                ids = [s["id"] for s in solids]
                if bone.startswith(SWEPT):
                    jobs.append({"key": key, "scale": den, "view": "elevation" if view == "elevation" else "lateral", "solids": ids, "bone": bone, "sweep": True})
                    plan.append((key, base, "swept-w" if view == "elevation" else "swept-v"))
                elif bone.startswith(TURNING):
                    jobs.append({"key": key, "scale": den, "view": "elevation", "solids": ids, "bone": bone})
                    plan.append((key, base, "plain" if view == "elevation" else "extruded"))
                else:
                    job = {"key": key, "scale": den, "view": view, "solids": ids}
                    if view == "elevation":
                        job["bone"] = bone
                    jobs.append(job)
                    plan.append((key, base, "plain"))
                out.append(key)
            for s in body:
                if not (s.get("slender") or s.get("texture")) or s["bone"] in WHEELS:
                    continue
                if view != "elevation" and s["bone"] != "frame" and not s["bone"].startswith(("head", "torso", "neck")):
                    continue
                pts = centre_line(s)
                if view == "elevation":
                    line = [rnd(p) for p in pts]
                else:
                    f = rest[s["bone"]]
                    ws = [FK.pt(f, *p) for p in pts]
                    line = [rnd([w[2], w[1]] if view == "front" else [w[0], w[2]]) for w in ws]
                if len(line) < 2:
                    continue
                role = "texture" if s.get("texture") and pens.get("texture") else ("detail" if pens.get("detail") and s["role"] != "body" else "body")
                thick = 2 * max(s.get("r0", 0), s.get("r1", 0), *(max(q["rx"], q["ry"]) for q in s.get("sections", [])))
                sh = shape(s["id"], s["bone"], role, zof(depth([s]), role) + 0.01, s["tone"], s["_region"], s["material"], kind="poly", points=line, closed=False, filled=False)
                if role != "texture":
                    sh["penMm"] = round(max(thick / den * 1000, PEN_FLOOR[den]), 3)
                out.append(sh)
            out.extend(wheel_shapes(den, view, body, pens, zof))
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
        notes += [f"SPLIT AGAIN {k}" for k in sp2]
        plan = plan2
    made = {}
    for key, base, how in plan:
        pts = outlines[key]
        sh = shape(base["i"], base["bone"], base["role"], base["z"], base["tone"], base["region"], base["material"], kind="poly", points=pts, closed=True)
        sp = spans.get(key)
        if how.startswith("swept"):
            lo, hi, lo1, hi1 = sp[0:4]
            if lo is None or lo > hi:
                lo, hi = lo1, hi1
            if lo1 is None or lo1 > hi1:
                lo1, hi1 = lo, hi
            lo, hi, lo1, hi1 = [round(q, 4) for q in (lo, hi, lo1, hi1)]
            if abs(lo1 - lo) < 0.002:
                lo1 = round(lo + 0.0021, 4)
            if abs(hi1 - hi) < 0.002:
                hi1 = round(hi - 0.0023, 4)
            sh["solid"] = {"kind": "extruded", "sweep": "w" if how == "swept-w" else "v", "z0": lo, "z1": hi, "zTip": [lo1, hi1]}
        elif how == "extruded":
            sh["solid"] = {"kind": "extruded", "sweep": "w", "z0": round(sp[4], 4), "z1": round(sp[5], 4)}
        made[key] = sh
    for (den, view), lst in shapes.items():
        rep = a["representations"][str(den)]
        new = [made[k] if isinstance(k, str) else k for k in lst]
        for sh in new:
            if den == 100 and sh["id"] in FAR[view]:
                sh["hatch"] = "far-limb"
            if den in (50, 100) and view == "elevation" and sh["id"] == "jacket":
                sh["hatch"] = "softshell"
            if den == 500 and sh["id"] == "head-helmet":
                sh.pop("region")
                sh["regions"] = ["face-skin", "helmet", "hair"]
        ground = [s for s in old[str(den)]["geometry"][view] if s.get("role") == "ground"]
        rep["geometry"][view] = ground + sorted(new, key=lambda s: s["z"])
    TX.apply(a)
    a["dimensions"].update(DIMENSIONS)
    for r in a["thermal"]["regions"]:
        if r["id"] in AREAS:
            r["areaM2"] = AREAS[r["id"]]
    json.dump(doc, open(path, "w"), ensure_ascii=False, indent=1)
    for n in notes:
        print(n)
    print("shapes", {f"{d}|{v}": len(l) for (d, v), l in shapes.items()})


def project(path, jobs, name):
    """🖨️ Runs the outline projector on a job list; returns the outlines, the swept spans and the keys whose solids fell apart into several outlines."""
    json.dump(jobs, open(os.path.join(OUT, f"{name}.jobs.json"), "w"))
    bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
    out = os.path.join(OUT, f"{name}.json")
    subprocess.run([bun, os.path.join(T, "r3-cyclist-project2d.ts"), path, os.path.join(OUT, f"{name}.jobs.json"), out], check=True, cwd=T, stdout=subprocess.DEVNULL)
    return json.load(open(out)), json.load(open(f"{out}.spans.json")), set(json.load(open(f"{out}.split.json")))


def wheel_shapes(den, view, body, pens, zof):
    out = []
    by = {s["id"]: s for s in body}
    for n in ("rear", "front"):
        b = f"{n}-wheel"
        x = 0.0 if n == "rear" else R.FRONT[0]
        d = -0.0 if view == "elevation" else (x if view == "front" else 0.0)
        tyre = by[f"{n}-tyre"]
        hub = by.get(f"{n}-hub")
        hub_r = max(p[1] for p in hub["profile"]) if hub else None
        hub_w = max(abs(p[0]) for p in hub["profile"]) if hub else None
        if view == "elevation":
            if den == 500:
                out.append(shape(f"{n}-tyre", b, "body", zof(d, "body"), tyre["tone"], "tyres", "rubber", kind="ellipse", cx=0, cy=0, rx=round(R.WHEEL_R - 0.020, 4), ry=round(R.WHEEL_R - 0.020, 4), filled=False, penMm=0.13))
                continue
            inner = R.RIM_IN if den == 200 else R.RIM_OUT
            out.append(shape(f"{n}-tyre", b, "body", zof(d, "body"), tyre["tone"], "tyres", "rubber", kind="path", d=ring(R.WHEEL_R, inner)))
            if f"{n}-rim" in by:
                rim = by[f"{n}-rim"]
                out.append(shape(f"{n}-rim", b, "detail" if pens.get("detail") else "body", zof(d, "detail") + 0.001, rim["tone"], "bike-metal", "steel", kind="path", d=ring(R.RIM_OUT, R.RIM_IN)))
            out.append(shape(f"{n}-hub", b, "body", zof(d, "body") + 0.03, hub["tone"], "bike-metal", "steel", kind="ellipse", cx=0, cy=0, rx=round(hub_r, 4), ry=round(hub_r, 4)))
            if n == "rear" and "rear-cog" in by:
                cog = by["rear-cog"]
                rr = max(p[1] for p in cog["profile"])
                out.append(shape("rear-cog", b, "detail" if pens.get("detail") else "body", zof(0.05, "detail"), cog["tone"], "bike-metal", "steel", kind="ellipse", cx=0, cy=0, rx=round(rr, 4), ry=round(rr, 4)))
            for s in body:
                if s["bone"] == b and s.get("texture"):
                    cnt = s.get("repeat", {}).get("count", 1)
                    step = s.get("repeat", {}).get("rotationRad", 0.0)
                    for k in range(cnt):
                        c, sn = math.cos(step * k), math.sin(step * k)
                        p0 = [s["p0"][0] * c - s["p0"][1] * sn, s["p0"][0] * sn + s["p0"][1] * c]
                        p1 = [s["p1"][0] * c - s["p1"][1] * sn, s["p1"][0] * sn + s["p1"][1] * c]
                        out.append(shape(f"{s['id']}-{k}", b, "texture" if pens.get("texture") else "body", zof(d, "texture") + 0.01, s["tone"], "bike-metal", "steel", kind="poly", points=[rnd(p0), rnd(p1)], closed=False, filled=False))
        else:
            w = 0.020
            out.append(shape(f"{n}-tyre", b, "body", zof(d, "body"), tyre["tone"], "tyres", "rubber", kind="ellipse", cx=0, cy=0, rx=R.WHEEL_R, ry=R.WHEEL_R, solid={"kind": "extruded", "sweep": "w", "z0": -w, "z1": w}))
            if hub:
                out.append(shape(f"{n}-hub", b, "body", zof(d, "body") + 0.03, hub["tone"], "bike-metal", "steel", kind="ellipse", cx=0, cy=0, rx=round(hub_r, 4), ry=round(hub_r, 4), solid={"kind": "extruded", "sweep": "w", "z0": round(-hub_w, 4), "z1": round(hub_w, 4)}))
    return out


if __name__ == "__main__":
    main()
