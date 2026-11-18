"""Replaces a drawn shape by the traced outline of volume solids: python3 retrace.py <slug> <scales,> <view> <shapeId> <solids> [tol] ; the shape becomes a closed polygon in its local coordinates."""
import sys, os, json, subprocess, importlib.util
H = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET/"
s = importlib.util.spec_from_file_location("l", H + "animals-appliances-components-b-lib.py"); l = importlib.util.module_from_spec(s); s.loader.exec_module(l)
B = "/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
CWD = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
def trace(slug, scale, view, sid, solids, tol):
    r = subprocess.run([B, H + "animals-appliances-components-b-trace.ts", slug, scale, view, sid, solids, tol], cwd=CWD, capture_output=True, text=True)
    lines = r.stdout.strip().split("\n")
    if r.returncode != 0 or not lines[-1].startswith("[["): raise LookupError(r.stdout[-300:] + r.stderr[-300:])
    return json.loads(lines[-1]), lines[-2]
def retrace(d, slug, scale, view, sid, solids, tol="1.2"):
    pts, info = trace(slug, scale, view, sid, solids, tol)
    n = d.shape(scale, view, sid); o = d.obj(n); new = {}
    for k, v in o.items():
        if k in ("cx", "cy", "rx", "ry", "rotationDeg", "points", "closed", "x0", "y0", "x1", "y1", "r0", "r1", "solid", "d"): continue
        new[k] = "poly" if k == "kind" else v
    new["points"] = pts; new["closed"] = True
    d.replace_obj(n, new)
    print(scale, view, sid, info)
if __name__ == "__main__":
    slug, scales, view, sid, solids = sys.argv[1:6]; tol = sys.argv[6] if len(sys.argv) > 6 else "1.2"
    d = l.doc(slug)
    for sc in scales.split(","): retrace(d, slug, sc, view, sid, solids, tol)
    d.commit()

def trace_batch(slug, scale, view, jobs, tol=None):
    tol = tol or os.environ.get("TOL", "0.6")
    f = H + "🗑️generated/aac-b-jobs.json"
    json.dump(jobs, open(f, "w"))
    r = subprocess.run([B, H + "animals-appliances-components-b-trace.ts", slug, scale, view, "@", f, tol], cwd=CWD, capture_output=True, text=True)
    for ln in r.stdout.split("\n"):
        if ln.startswith("@@"): return json.loads(ln[2:])
    raise LookupError(r.stdout[-300:] + r.stderr[-300:])
def apply_trace(d, scale, view, sid, res):
    pts = res["pts"]
    n = d.shape(scale, view, sid); o = d.obj(n); new = {}
    for k, v in o.items():
        if k in ("cx", "cy", "rx", "ry", "rotationDeg", "points", "closed", "x0", "y0", "x1", "y1", "r0", "r1", "solid", "d"): continue
        new[k] = "poly" if k == "kind" else v
    new["points"] = pts; new["closed"] = True
    d.replace_obj(n, new)
