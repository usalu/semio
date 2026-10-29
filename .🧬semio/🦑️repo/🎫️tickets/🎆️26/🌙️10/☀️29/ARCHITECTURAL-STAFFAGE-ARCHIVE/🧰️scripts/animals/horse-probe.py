import sys; sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/animals")
exec(open(sys.path[0] + "/horse-ik.py").read())
base = {}
for s in "nf":
    o, e = hind(s, HX, -H0, base, (0,0,0)); print("hind", s, {k: round(v,1) for k,v in o.items()}, round(e,4))
    o, e = fore(s, FX, -H0, base, (0,0,0)); print("fore", s, {k: round(v,1) for k,v in o.items()}, round(e,4))
    break
# gait reach test: hind and fore at excursions
for H in (1.27, 1.29):
    for dx in (-0.45, -0.3, 0, 0.3, 0.45):
        oh, eh = hind("n", HX+dx, -H, base, (0,0,0), (-80+40*dx, -58, -50))
        of, ef = fore("n", FX+dx, -H, base, (0,0,0), (-90+40*dx, -55, -52), sc=max(-20, min(20, dx*50)))
        print(H, dx, round(eh,4), round(ef,4), {k: round(v) for k,v in oh.items() if k.startswith(("thigh","shank"))}, {k: round(v) for k,v in of.items() if k.startswith(("hum","rad"))})
