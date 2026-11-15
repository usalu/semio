"""Reviewer 3 copy (reads volumes from T/volumes when the asset file has none) of the reviewer 2 helper: prints dimensions, regions (committed and working areaM2), default-pose exposure/contact and, per region, the 1:50 thermal solids with the closed analytic surface of each solid.
Usage: python3 areareview2-dump.py <slug> [scale]"""
import glob, json, math, os, subprocess, sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
ROOT = os.path.join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")


def per(pts):
    return sum(math.dist(pts[i], pts[(i + 1) % len(pts)]) for i in range(len(pts)))


def parea(pts):
    return abs(sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))) / 2


def area(s):
    k = s.get("kind")
    n = (s.get("repeat") or {}).get("count", 1)
    if k == "box":
        a, b, c = s["size"]
        return n * 2 * (a * b + b * c + c * a)
    if k == "extrude":
        p = s["profile"]
        L = abs(s["to"] - s["from"])
        h = sum(per(x) for x in (s.get("holes") or []))
        ha = sum(parea(x) for x in (s.get("holes") or []))
        return n * ((per(p) + h) * L + 2 * (parea(p) - ha))
    if k == "revolve":
        p = s["profile"]
        sw = s.get("sweepRad") or 2 * math.pi
        t = 0
        for i in range(len(p) - 1):
            a, b = p[i], p[i + 1]
            t += sw * (a[1] + b[1]) / 2 * math.dist(a, b)
        if p[0] != p[-1]:
            t += sw / 2 * (p[0][1] ** 2 + p[-1][1] ** 2)
        return n * t
    if k == "capsule":
        r0, r1 = s["r0"], s["r1"]
        L = math.dist(s["p0"], s["p1"])
        sl = math.hypot(L, r0 - r1)
        return n * (math.pi * (r0 + r1) * sl + 2 * math.pi * (r0 * r0 + r1 * r1))
    if k == "ellipsoid":
        a, b, c = s["radii"]
        p = 1.6075
        return n * 4 * math.pi * (((a * b) ** p + (a * c) ** p + (b * c) ** p) / 3) ** (1 / p)
    if k == "loft":
        t = 0
        sec = s["sections"]
        for i in range(len(sec) - 1):
            p0 = math.pi * (3 * (sec[i]["rx"] + sec[i]["ry"]) - math.sqrt((3 * sec[i]["rx"] + sec[i]["ry"]) * (sec[i]["rx"] + 3 * sec[i]["ry"])))
            p1 = math.pi * (3 * (sec[i + 1]["rx"] + sec[i + 1]["ry"]) - math.sqrt((3 * sec[i + 1]["rx"] + sec[i + 1]["ry"]) * (sec[i + 1]["rx"] + 3 * sec[i + 1]["ry"])))
            t += (p0 + p1) / 2 * math.dist(sec[i]["at"], sec[i + 1]["at"])
        return n * t
    return float("nan")


def desc(s):
    k = s.get("kind")
    r = s.get("repeat")
    rp = f" x{r['count']}" if r else ""
    if k == "box":
        return f"box {s['size']}{rp}"
    if k == "extrude":
        xs = [q[0] for q in s["profile"]]
        ys = [q[1] for q in s["profile"]]
        return f"extrude {s['plane']} prof {max(xs)-min(xs):.3f}x{max(ys)-min(ys):.3f} per {per(s['profile']):.3f} len {abs(s['to']-s['from']):.3f}{rp}"
    if k == "revolve":
        rr = max(q[1] for q in s["profile"])
        hh = max(q[0] for q in s["profile"]) - min(q[0] for q in s["profile"])
        return f"revolve r{rr:.3f} len{hh:.3f}{rp}"
    if k == "capsule":
        return f"capsule r{s['r0']}-{s['r1']} L{math.dist(s['p0'], s['p1']):.3f}{rp}"
    if k == "ellipsoid":
        return f"ellipsoid {s['radii']}{rp}"
    if k == "loft":
        return f"loft {len(s['sections'])} sections{rp}"
    return str(k)


def main():
    slug = sys.argv[1]
    sc = sys.argv[2] if len(sys.argv) > 2 else "50"
    path = glob.glob(f"{ROOT}/*/{slug}/🔣️.json")[0]
    a = json.load(open(path))["asset"]
    if not a.get("volumes"):
        sp = os.path.join(T, "volumes", path.split("/")[-3].split("️")[-1] if False else "", "")
        import glob as _g
        fs = _g.glob(os.path.join(T, "volumes", "*", slug + ".json"))
        if fs:
            a["volumes"] = json.load(open(fs[0]))["volumes"]
    rel = os.path.relpath(path, REPO)
    try:
        com = {r["id"]: r for r in json.loads(subprocess.check_output(["git", "-C", REPO, "show", "HEAD:" + rel]))["asset"]["thermal"]["regions"]}
    except Exception:
        com = {}
    print(slug, a["dimensions"])
    dp = a["presentation"]["defaultPose"]
    pose = next(p for p in a["poses"] if p["id"] == dp)
    pr = pose.get("thermal", {}).get("regions", {})
    print("poses:", [p["id"] for p in a["poses"]], "default", dp)
    body = {s["id"]: s for s in a["volumes"][sc]["body"]} if a.get("volumes") else {}
    only = sys.argv[3].split(",") if len(sys.argv) > 3 else None
    for r in a["thermal"]["regions"]:
        if only and r["id"] not in only:
            continue
        op = (r.get("operating") or {}).get("heatW")
        ol = r["layers"][0] if r.get("layers") else {}
        print(f"\n== {r['id']} [{r['kind']}/{r['model']}] areaM2 {r['areaM2']} (HEAD {com.get(r['id'], {}).get('areaM2')}) basis {r.get('areaBasis')} geom {r.get('geometry')} heatW {op} L {r.get('characteristicLengthM')} layers {[(l['id'], l.get('thicknessM')) for l in r.get('layers', [])]}")
        print("   pose:", pr.get(r["id"]))
        if not a.get("volumes"):
            continue
        tot = 0
        outer = 0
        rowsx = []
        for s in a["volumes"][sc]["thermal"]:
            if s.get("region") != r["id"]:
                continue
            b = body.get(s.get("of")) if s.get("of") else s
            if b is None:
                print("   ?", s)
                continue
            ar = area(b)
            lay = s.get("layer")
            tot += ar
            if s.get("of") or lay is None:
                outer += ar
            rowsx.append((ar, f"   {s['id']:34s} layer {str(lay):12s} {desc(b):80s} A={ar:.4f} sec={s.get('sectionFaces') or b.get('sectionFaces')}"))
        top = int(os.environ.get("TOP", "0"))
        for ar, line in (sorted(rowsx, reverse=True)[:top] if top else rowsx):
            print(line)
        if top and len(rowsx) > top:
            print(f"   ... {len(rowsx) - top} more solids")
        print(f"   SUM all layers {tot:.4f}   SUM of-refs/no-layer {outer:.4f}")


main()
