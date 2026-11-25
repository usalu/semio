"""🗒️ Lists the drawn shapes of an asset per scale and view with bone, kind, flags and local extents: `python3 dump.py <asset.json> [scale] [view]`; with `FULL=1` the whole shape."""
import json, os, sys
a = json.load(open(sys.argv[1]))["asset"]
want_scale = sys.argv[2] if len(sys.argv) > 2 else None
want_view = sys.argv[3] if len(sys.argv) > 3 else None
print("dims", a["dimensions"], "default", a["presentation"]["defaultPose"], "poses", [p["id"] for p in a["poses"]])
print("bones", [(b["id"], b.get("parent")) for b in a["rig"]["bones"]])
def pts(s):
    out = []
    def walk(v):
        if isinstance(v, list):
            if len(v) == 2 and all(isinstance(x, (int, float)) for x in v):
                out.append(v)
            else:
                for x in v:
                    walk(x)
        elif isinstance(v, dict):
            for k, x in v.items():
                if k not in ("solid",):
                    walk(x)
    walk({k: v for k, v in s.items() if k in ("points", "rings", "path", "from", "to", "at", "center", "centre")})
    return out
for key, rep in a["representations"].items():
    if want_scale and key != want_scale:
        continue
    if "geometry" not in rep:
        print("==", key, {k: (v if k != "alternatives" else "...") for k, v in rep.items()})
        continue
    print("==", key, "validRange", rep.get("validRange"), {v: len(rep["geometry"][v]) for v in rep["geometry"]})
    for view, shapes in rep["geometry"].items():
        if want_view and view != want_view:
            continue
        print(" --", view)
        for s in shapes:
            if os.environ.get("FULL"):
                print("   ", json.dumps(s, ensure_ascii=False))
                continue
            p = pts(s)
            ext = f"x[{min(q[0] for q in p):.3f},{max(q[0] for q in p):.3f}] y[{min(q[1] for q in p):.3f},{max(q[1] for q in p):.3f}] n{len(p)}" if p else ""
            rest = {k: (v if not isinstance(v, (list, dict)) else "…") for k, v in s.items() if k not in ("id", "bone", "points", "kind")}
            print("   ", s["id"], "|", s.get("bone"), "|", s.get("kind"), ext, rest)
