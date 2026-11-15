import sys, json, math; sys.path.insert(0, ".")
import importlib; L = importlib.import_module("author-13-lib")
P = "volumes/appliances/toaster.json"
cur = json.load(open(P))
def rect(hz, hx): return [(-hz, -hx), (hz, -hx), (hz, hx), (-hz, hx)]
def sup(rz, rx, e=9, n=48):
    out = []
    for k in range(n):
        a = 2 * math.pi * k / n; c, s = math.cos(a), math.sin(a)
        out.append((rz * math.copysign(abs(c) ** (2 / e), c), rx * math.copysign(abs(s) ** (2 / e), s)))
    return out
def fix(sc):
    v = cur["volumes"][str(sc)]
    t = 0.0075 if sc == 50 else (0.015 if sc == 100 else 0.03)
    oz, ox = 0.07, 0.13
    ring = L.ext("member-base-frame", "base", "chrome-steel", "body", 0.5, rect(oz, ox), "zx", 0.02, 0.16, holes=[rect(oz - max(t, 0.01), ox - max(t, 0.01))])
    new = []
    for s in v["structure"]:
        if s["id"] == "member-base-frame": s.clear(); s.update(L.st(ring, "base", "member"))
        elif s["id"] == "mass-base": s.clear(); s.update(L.st(L.box("mass-base", "base", "chrome-steel", "detail", 0.5, (0.2, 0.04, 0.1), (0, 0.045, 0)), "base", "mass"))
    if not any(s["id"] == "pan" for s in v["body"]):
        y0, y1 = (0.012, 0.0195) if sc == 50 else ((0.012, 0.027) if sc == 100 else (0.012, 0.042))
        v["body"].append(L.ext("pan", "base", "chrome-steel", "body", 0.45, sup(0.077, 0.137), "zx", y0, y1))
        v["thermal"].append(L.th("th-pan", "pan", "casing", "sheet"))
    if sc == 100:
        for s in v["body"]:
            if s["id"] == "slice-rear": s["from"], s["to"] = 0.019, 0.041
            if s["id"] == "slice-front": s["from"], s["to"] = -0.041, -0.019
    if sc < 200 and not any(s["id"] == "th-lever" for s in v["thermal"]):
        v["thermal"] += [L.th("th-lever", "lever", "control", "knob")] + ([L.th("th-cancel-button", "cancel-button", "control", "knob")] if sc == 50 else [])
    for s in v["structure"]:
        if s["id"] == "joint-slices":
            s.clear(); s.update(L.st(L.ell("joint-slices", "slices", "chrome-steel", "detail", 0.5, (0.025, 0.012, 0.025) if sc == 50 else (0.025, 0.015, 0.025), (0, 0.065, 0)), "slices", "joint"))
    for s in v["thermal"]:
        if s["id"] == "th-slot-air":
            s["size"] = [0.08, 0.045, 0.04]; s["transform"] = {"translation": [0, 0.1425, 0]}
    return v
if __name__ == "__main__":
    for a in sys.argv[1:]: fix(int(a))
    json.dump(cur, open(P, "w"), ensure_ascii=False, indent=1)
