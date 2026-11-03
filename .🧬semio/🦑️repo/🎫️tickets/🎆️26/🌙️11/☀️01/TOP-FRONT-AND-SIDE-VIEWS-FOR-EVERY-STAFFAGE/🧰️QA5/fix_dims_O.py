import sys, os
sys.path.insert(0, "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️O1")
from lib import load, save


def shapes(doc, sc, view, sid):
    return [s for s in doc["asset"]["representations"][sc]["geometry"][view] if s["id"] == sid]


d = load("playground-swing")
for v, (a, b) in (("front", ("x", "w")), ("plan", ("y", "h"))):
    for s in shapes(d, "50", v, "beam-cap-left"):
        s[a], s[b] = 0.60, 0.02
    for s in shapes(d, "50", v, "beam-cap-right"):
        s[a], s[b] = -0.62, 0.02
for s in shapes(d, "500", "front", "beam"):
    s["x"], s["w"] = -0.62, 1.24
for s in shapes(d, "500", "plan", "beam"):
    s["y"], s["h"] = -0.62, 1.24
save("playground-swing", d)

d = load("stepladder")
for sc in ("50", "100", "200", "500"):
    for s in shapes(d, sc, "elevation", "front-rail"):
        p = s["points"]
        p[1][1] = 1.5
        p[2][1] = 1.5
        if sc == "200":
            p[3][1] = 0.0
        if sc == "500":
            p[0][0] = -0.41
    for s in shapes(d, sc, "elevation", "top-cap"):
        s["d"] = s["d"].replace("1.52", "1.5033")
    for s in shapes(d, sc, "front", "top-cap"):
        s["d"] = s["d"].replace("1.52", "1.5033")
for s in shapes(d, "500", "elevation", "foot-front"):
    for p in s["points"]:
        p[1] = 0.0
for sid in ("front-rail-l", "front-rail-r"):
    for s in shapes(d, "500", "plan", sid):
        s["y0"] = round(0.185 if s["y0"] > 0 else -0.185, 4)
save("stepladder", d)

d = load("shopping-trolley")
for sc in ("50", "100", "200", "500"):
    for s in shapes(d, sc, "plan", "grip"):
        if s["kind"] == "capsule":
            s["x0"], s["x1"] = -0.51, -0.53
    for sid in ("handle-arm-l", "handle-arm-r"):
        for s in shapes(d, sc, "plan", sid):
            s["x0"] = -0.51
for s in shapes(d, "500", "elevation", "handle"):
    for p in s["points"]:
        if p[1] == 0.11:
            p[1] = 0.12
save("shopping-trolley", d)

d = load("cooking-pot")
for s in shapes(d, "50", "elevation", "handle-left"):
    s["x"], s["w"] = -0.125, 0.041
for s in shapes(d, "50", "elevation", "handle-right"):
    s["x"], s["w"] = 0.084, 0.041
for v in ("plan",):
    for s in shapes(d, "50", v, "handle-left"):
        s["x"], s["w"] = -0.125, 0.041
    for s in shapes(d, "50", v, "handle-right"):
        s["x"], s["w"] = 0.084, 0.041
for v in ("elevation", "plan"):
    for s in shapes(d, "100", v, "handle-left"):
        s["x"], s["w"] = -0.125, 0.041
    for s in shapes(d, "100", v, "handle-right"):
        s["x"], s["w"] = 0.084, 0.041
for s in shapes(d, "100", "plan", "pot-top"):
    s["rx"] = s["ry"] = 0.086
for s in shapes(d, "100", "front", "pot-end"):
    s["x"], s["w"] = -0.086, 0.172
for s in shapes(d, "100", "elevation", "pot"):
    for p in s["points"]:
        if abs(p[0]) == 0.088:
            p[0] = 0.086 if p[0] > 0 else -0.086
for s in shapes(d, "200", "elevation", "pot"):
    for p in s["points"]:
        if abs(p[0]) == 0.1:
            p[0] = 0.086 if p[0] > 0 else -0.086
        elif abs(p[0]) == 0.085:
            p[0] = 0.078 if p[0] > 0 else -0.078
save("cooking-pot", d)
print("dims fixed")
