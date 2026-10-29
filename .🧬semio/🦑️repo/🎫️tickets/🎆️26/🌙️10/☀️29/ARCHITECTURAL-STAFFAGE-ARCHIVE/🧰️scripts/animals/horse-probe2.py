import sys; sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/animals")
exec(open(sys.path[0] + "/horse-ik.py").read())
for H in (1.28, 1.30):
  for dx in (-0.4, -0.2, 0, 0.2, 0.4):
      for fx in (0.0, 0.03):
        S = -9 + 32*dx
        of, ef = fore("n", FX+dx+fx, -H, {}, (0,0,0), (-90+15*dx, -55, -52), sc=S)
        print(H, dx, fx, round(S), round(ef,4), {k: round(v) for k,v in of.items() if k[:3] in "scahumrad"})
