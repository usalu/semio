import sys, math
exec(open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/animals/horse-ik.py").read())
exec(open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/animals/horse-emit.py").read().split("def pose(")[0].split("R0 = ")[1].join(["R0 = ", ""]) if False else "")
D, EXC, LIFT, HG, N = 0.64, 0.80, 0.09, 1.28, 8
def foot(phase, D, exc, lift):
    if phase < D: return exc/2 - exc*phase/D, 0.0, 0.0, True
    s = (phase - D)/(1-D); return -exc/2 + exc*s, lift*math.sin(math.pi*s), math.sin(math.pi*s), False
for k in range(N):
    for ft, off in {"hn": 0.0, "fn": 0.25, "hf": 0.5, "ff": 0.75}.items():
        dx, rise, sw, st = foot((k/N - off) % 1, D, EXC, LIFT)
        try:
            if ft[0] == "h": o, e = hind(ft[1], 0.043+dx, -HG+rise, {}, (0,0,0), (-80+16*dx+50*sw, -58-40*sw, -50-35*sw))
            else: o, e = fore(ft[1], 1.08+dx, -HG+rise, {}, (0,0,0), (-90+15*dx-60*sw, -55-30*sw, -52-30*sw), sc=-9+32*dx)
            print(k, ft, round(dx,2), round(sw,2), round(e,4), {a_: round(v) for a_, v in o.items()})
        except Exception as ex:
            print(k, ft, round(dx,2), round(sw,2), "FAIL")
