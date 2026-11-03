s=open("human.py",encoding="utf8").read()
# hand cap ends at the fingertip, not tip + radius
s=s.replace('R.t(hd)[0] + 0.012, lat, P["hr"], P["hr"] * 0.8, 3.1','R.t(hd)[0] + 0.012 - P["hr"] * 0.8, lat, P["hr"], P["hr"] * 0.8, 3.1')
# head as a solid at 1:500 (no elevation twin there)
a='            ell(o, "head", hb, 0.0, cy, hx, ry, 5.0, h.S(sc, "skin"))\n'
assert a in s
s=s.replace(a,'''            if t == 0:
                sk_ = h.st.bbox(sc, h.KEY["skull"]); hr_ = h.st.bbox(sc, h.KEY["hair"])
                ellsol(o, "head", hb, sk_[0], max(sk_[2], hr_[2]), sk_[1], sk_[3], hx, 5.0, h.S(sc, "skin"))
            else:
                ell(o, "head", hb, 0.0, cy, hx, ry, 5.0, h.S(sc, "skin"))
''')
s=s.replace('            poly(o, "hair", hb, cap_pts','            if t > 0: poly(o, "hair", hb, cap_pts')
s+='''

def ellsol(o, id, bone, x0, x1, y0, y1, half, z, style):
    """An ellipse given by its bone-local box, swept laterally to +-half: a solid with its depth built in."""
    cx, cy, rx, ry = (x0 + x1) / 2, (y0 + y1) / 2, (x1 - x0) / 2, (y1 - y0) / 2
    poly(o, id, bone, [(cx + rx * math.cos(a * math.pi / 12), cy + ry * math.sin(a * math.pi / 12)) for a in range(24)], z, style)
    sol(o, -half, half)
'''
open("human.py","w",encoding="utf8").write(s)
t=open("p_reader.py",encoding="utf8").read()
i=t.index("    if True:\n        bb_ = wbox")
j=t.index("\n\nNOTES")
t=t[:i]+'''    if True:
        bb_ = wbox(R, h.st, sc, "head", "hair-bun")
        lb = h.st.bbox(sc, "hair-bun")
        if t == 0: ellsol(of, "hair-bun", "head", lb[0], lb[2], lb[1], lb[3], max(.035, (lb[3] - lb[1]) / 2), 5.6, s("hair-bun"))
        else: ell(of, "hair-bun", "head", 0.0, (bb_[1] + bb_[3]) / 2, max(.035, (bb_[3] - bb_[1]) / 2), (bb_[3] - bb_[1]) / 2, 5.6, s("hair-bun"))
        ell(op, "hair-bun", "head", (bb_[0] + bb_[2]) / 2, 0.0, (bb_[2] - bb_[0]) / 2, (bb_[2] - bb_[0]) / 2, 6.2, s("hair-bun"))
'''+t[j:]
open("p_reader.py","w",encoding="utf8").write(t)
