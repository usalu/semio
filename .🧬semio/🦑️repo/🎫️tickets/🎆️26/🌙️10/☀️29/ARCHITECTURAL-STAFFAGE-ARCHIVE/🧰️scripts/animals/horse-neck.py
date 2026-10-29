import sys, math; sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/animals")
exec(open(sys.path[0] + "/horse-ik.py").read())
res = []
for rootp in (0, -3):
  for n1 in range(-120, -60, 5):
    for n2 in range(-30, 6, 5):
      for n3 in range(-20, 11, 5):
        for hd in range(-20, 80, 5):
            b = {"neck-1": n1, "neck-2": n2, "neck-3": n3, "head": hd}
            w = a.fk(b, (0, 0, rootp)); nx, ny = a.anchor(w, "nose")
            res.append((abs(ny - (-H0 + 0.05)), rootp, b, round(nx, 2), round(ny, 2), round(kin.ang(w["head"]))))
res.sort(key=lambda r: r[0])
for r in [x for x in res if x[3] > 1.75 and 62 > abs(x[5] + 0) > 0 or True][:0]: pass
sel = [r for r in res if r[0] < 0.01 and -100 < r[5] < -70]
for r in sel[:25]: print(r)
