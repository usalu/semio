import copy
from jio import read, write


def ids(items):
    return {s["id"] for s in items}


def find(items, sid):
    return next(s for s in items if s["id"] == sid)


def add(items, new, after=None):
    if new["id"] in ids(items):
        return
    items.append(new)


def line(sid, bone, pts, z=3.8, tone=0.3):
    return {"id": sid, "bone": bone, "role": "detail", "z": z, "filled": False, "tone": tone, "kind": "poly", "points": pts, "closed": False}


def like(src, sid, **kw):
    o = copy.deepcopy(src)
    o["id"] = sid
    o.update(kw)
    return o


def chalkboard():
    doc, raw = read("🪑️objects", "chalkboard")
    reps = doc["asset"]["representations"]
    e = reps["50"]["geometry"]["elevation"]
    frame = find(e, "frame")
    leaf = find(e, "leaf-frame")
    add(e, like(frame, "top-rail", bone="frame", z=3.7, tone=0.5, kind="rect", x=-1.988, y=1.2, w=3.976, h=0.048))
    add(e, like(leaf, "leaf-top-rail", bone="leaf", z=3.75, tone=0.55, kind="rect", x=-1.988, y=1.145, w=1.976, h=0.043))
    add(e, line("rail-track", "frame", [[-1.988, 1.224], [1.988, 1.224]]))
    add(e, like(leaf, "hanger-a", bone="leaf", z=3.9, tone=0.2, kind="ellipse", cx=-1.7, cy=1.166, rx=0.022, ry=0.022))
    add(e, like(leaf, "hanger-b", bone="leaf", z=3.9, tone=0.2, kind="ellipse", cx=-0.3, cy=1.166, rx=0.022, ry=0.022))
    add(e, like(find(e, "chalk-a"), "ruler", x=-0.7, y=0.022, w=0.9, h=0.012, tone=0.55))
    add(e, like(find(e, "chalk-a"), "chalk-c", x=-1.05, y=0.022, w=0.08, h=0.014, tone=0.8))
    for sc in ("100",):
        e1 = reps[sc]["geometry"]["elevation"]
        fr, lf = find(e1, "frame"), find(e1, "leaf-frame")
        add(e1, like(fr, "top-rail", bone="frame", z=3.7, tone=0.5, kind="rect", x=-1.98, y=1.19, w=3.96, h=0.05))
        add(e1, like(lf, "leaf-top-rail", bone="leaf", z=3.75, tone=0.55, kind="rect", x=-1.98, y=1.135, w=1.96, h=0.045))
    write("🪑️objects", "chalkboard", doc, raw)


def hob():
    doc, raw = read("🔌️appliances", "gas-hob")
    reps = doc["asset"]["representations"]
    e = reps["50"]["geometry"]["elevation"]
    add(e, line("vent-slots", "hob", [[-0.1 + 0.02 * i, 0.006 + 0.01 * (i % 2)] for i in range(11)], z=3.8))
    add(e, line("front-panel-seam", "hob", [[-0.14, 0.021], [0.14, 0.021]], z=3.8))
    add(e, line("burner-ports", "burner", [[-0.046 + 0.0092 * i, 0.0035 + 0.0015 * (i % 2)] for i in range(11)], z=3.8))
    f = reps["50"]["geometry"]["front"]
    add(f, line("vent-slots", "hob", [[-0.22 + 0.04 * i, 0.006 + 0.01 * (i % 2)] for i in range(12)], z=3.8))
    add(f, line("front-panel-seam", "hob", [[-0.25, 0.021], [0.25, 0.021]], z=3.8))
    write("🔌️appliances", "gas-hob", doc, raw)


def laptop():
    doc, raw = read("🔌️appliances", "laptop")
    reps = doc["asset"]["representations"]
    e = reps["50"]["geometry"]["elevation"]
    add(e, line("vent-slot", "base", [[-0.103, 0.0098], [-0.083, 0.0098]], z=3.8, tone=0.1))
    add(e, line("port-strip", "base", [[0.05, 0.005], [0.068, 0.005]], z=3.8, tone=0.1))
    add(e, line("lid-bezel", "lid", [[0.0, 0.0054], [0.213, 0.0054]], z=3.8))
    write("🔌️appliances", "laptop", doc, raw)


if __name__ == "__main__":
    import sys

    globals()[sys.argv[1]]()
