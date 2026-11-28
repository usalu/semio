import json, math, sys
P = sys.argv[1]
d = json.load(open(P, encoding="utf-8"))
def wrap(a): return a - 2 * math.pi * round(a / (2 * math.pi))
def total(case, kind, st, x, y, prev):
    v, cam, g = case["viewport"], case["camera"], case["layer"]["gumball"]
    pl = g["pivotLayer"]
    px = (pl[0] - cam["x"]) * cam["zoom"] + v["width"] / 2
    py = (pl[1] - cam["y"]) * cam["zoom"] + v["height"] / 2
    if kind in ("moveX", "moveY"):
        m = g.get("modelToLayer", {"scale": [1, 1], "offset": [0, 0]})
        def model(sx, sy):
            wx = (sx - v["width"] / 2) / cam["zoom"] + cam["x"]
            wy = (sy - v["height"] / 2) / cam["zoom"] + cam["y"]
            return ((wx - m["offset"][0]) / m["scale"][0], (wy - m["offset"][1]) / m["scale"][1])
        b, a = model(*st), model(x, y)
        dx = a[0] - b[0] if kind == "moveX" else 0.0
        dy = a[1] - b[1] if kind == "moveY" else 0.0
        return None if abs(dx) < 1e-9 and abs(dy) < 1e-9 else {"dx": dx, "dy": dy}
    if kind == "rotate":
        raw = math.atan2(y - py, x - px) - math.atan2(st[1] - py, st[0] - px)
        p = prev["angle"] if prev else 0.0
        ang = p + wrap(raw - p)
        return None if abs(ang) < 1e-6 else {"angle": ang}
    r0 = math.hypot(st[0] - px, st[1] - py); r1 = math.hypot(x - px, y - py)
    ratio = r1 / r0 if r0 > 1e-6 else 1.0
    if abs(ratio - 1) < 1e-6: return None
    return {"sx": ratio if kind != "scaleY" else 1.0, "sy": ratio if kind != "scaleX" else 1.0}
IDENT = {"moveX": {"dx": 0.0, "dy": 0.0}, "moveY": {"dx": 0.0, "dy": 0.0}, "rotate": {"angle": 0.0}, "scaleX": {"sx": 1.0, "sy": 1.0}, "scaleY": {"sx": 1.0, "sy": 1.0}, "scaleUniform": {"sx": 1.0, "sy": 1.0}}
for case in d["cases"]:
    g = case["layer"]["gumball"]
    if not g.get("liveDispatch"): continue
    kind = case["expect"]["handle"]
    old = case["expect"]["dispatches"]
    if not kind or not old or old[0]["args"].get("phase") == "abort" and False: continue
    if not kind: continue
    st = (case["press"]["x"], case["press"]["y"])
    new, prev, streamed = [], None, False
    templ = next((o for o in old), None)
    if templ is None: continue
    def mk(vals, phase):
        a = json.loads(json.dumps(templ["args"])); a.pop("reason", None)
        for k in ("dx", "dy", "angle", "sx", "sy"):
            if k in a: a[k] = vals.get(k, a[k])
        a["phase"] = phase
        return {"action": templ["action"], "args": a}
    for e in case["events"]:
        if e["kind"] == "cancel":
            ab = json.loads(json.dumps(old[-1])); new.append(ab); break
        cur = total(case, kind, st, e["x"], e["y"], prev)
        if e["kind"] == "move":
            if cur == prev: continue
            prev = cur; streamed = True
            new.append(mk(cur or IDENT[kind], "stream"))
        else:
            if not streamed and not cur: break
            new.append(mk(cur or IDENT[kind], "commit"))
    print(case["name"][:60], len(old), "->", len(new))
    case["expect"]["dispatches"] = new
    if "returning to its press point" in case["name"]:
        case["name"] = "a live drag returning to its press point streams the identity for that move and commits the whole offset"
    if "streams every move as its increment" in case["name"]:
        case["name"] = case["name"].replace("streams every move as its increment and commits the release tail", "streams every move as its cumulative offset from the press and commits the whole offset")
    case["name"] = case["name"].replace("streams angle increments about the pivot", "streams cumulative angles about the pivot").replace("multiplies its increments to the release factor", "streams its cumulative factor and commits the release factor").replace("a live release without a streamed move commits its whole motion as the tail", "a live release without a streamed move commits its whole motion")
json.dump(d, open(P, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
open(P, "a", encoding="utf-8").write("\n")
