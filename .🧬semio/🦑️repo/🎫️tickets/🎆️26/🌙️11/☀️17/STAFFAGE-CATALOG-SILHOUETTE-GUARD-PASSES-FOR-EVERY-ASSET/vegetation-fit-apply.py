"""Applies the profile scales of vegetation-fit.ts: python3 vegetation-fit-apply.py <slug> <fit.json> [min gain]"""
import sys, json, re, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib")
slug = sys.argv[1]; GAIN = float(sys.argv[3]) if len(sys.argv) > 3 else 0.002
d = mv.D("🌳️vegetation/" + slug); a = d.val("asset"); n = 0
def scale_path(p, su, sv):
    k = [0]
    def f(m):
        tok = m.group(0)
        if tok.isalpha(): k[0] = 0; return tok
        v = float(tok) * (su if k[0] % 2 == 0 else sv); k[0] += 1
        return repr(round(v, 5)).rstrip("0").rstrip(".") if "." in repr(round(v, 5)) else repr(round(v, 5))
    return re.sub(r"[MLCQSTHVZ]|-?\d+\.?\d*(?:e-?\d+)?", f, p)
for r in json.load(open(sys.argv[2])):
    if r["iou1"] - r["iou0"] < GAIN or (r["su"] == 1 and r["sv"] == 1): continue
    s = next(x for x in a["representations"][str(r["scale"])]["geometry"][r["view"]] if x["id"] == r["id"] and x["bone"] == r["bone"])
    node = d.shape(str(r["scale"]), r["view"], r["id"])
    if s["kind"] == "poly": d.set(node, "points", [[round(x * r["su"], 5), round(y * r["sv"], 5)] for x, y in s["points"]])
    else: d.set(node, "d", scale_path(s["d"], r["su"], r["sv"]))
    n += 1
d.commit(); print("fitted", n)
