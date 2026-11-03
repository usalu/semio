import re
s=open("human.py",encoding="utf8").read()
# thigh: always solid
i=s.index('                if P.get("solid_thigh"):'); j=s.index('                ell(o, f"knee-{sd}"')
s=s[:i]+'                limbsol(h, o, f"thigh-{sd}", th, P["t0"], P["t1"], lat, 1.0, h.S(sc, "thigh", sd))\n'+s[j:]
s=s.replace('if low: poly(o, f"shank-{sd}", sk, limb(lat, knee_y, ank_y, P["s0"], P["s1"], 0.004), 1.02, h.S(sc, "shank", sd), follow=True)','if low: limbsol(h, o, f"shank-{sd}", sk, P["s0"], P["s1"], lat, 1.02, h.S(sc, "shank", sd))')
s=s.replace('poly(o, f"upper-arm-{sd}", arm, limb(lat, sh_y, el_y, P["a0"], P["a1"], 0.003), 4.6, h.S(sc, "sleeve", sd), follow=True)','limbsol(h, o, f"upper-arm-{sd}", arm, P["a0"], P["a1"], lat, 4.6, h.S(sc, "sleeve", sd))')
s=s.replace('poly(o, f"forearm-{sd}", fa, limb(lat, el_y, wr_y, P["f0"], P["f1"], 0.002), 4.64, h.S(sc, "forearm", sd), follow=True)','limbsol(h, o, f"forearm-{sd}", fa, P["f0"], P["f1"], lat, 4.64, h.S(sc, "forearm", sd))')
s=s.replace('poly(o, f"hand-{sd}", hd, limb(lat, wr_y, tip_y, P["hr"], P["hr"] * 0.82, 0.004), 4.7, h.S(sc, "hand", sd), follow=True)','limbsol(h, o, f"hand-{sd}", hd, P["hr"], P["hr"] * 0.82, lat, 4.7, h.S(sc, "hand", sd))')
s+='''

def limbsol(h, o, id, bone, ra, rb, lat, z, style):
    """A limb segment as an extruded solid: the tapered outline in the bone frame, swept across the limb thickness, so every pose projects it exactly (an arm pointing at the viewer shows its end-on width)."""
    L = h.R.b[bone]["lengthM"]
    poly(o, id, bone, [(0.0, -ra), (L, -rb), (L, rb), (0.0, ra)], z, style)
    r = (ra + rb) / 2
    sol(o, lat - r, lat + r)
'''
open("human.py","w",encoding="utf8").write(s)
print(s.count("limbsol("))
for f,a,b in [("p_pregnant.py","p500=dict(trim=.05)","p500=dict(trim=.1)"),("p_child.py","p500=dict(trim=.04)","p500=dict(trim=.1)")]:
    t=open(f,encoding="utf8").read(); open(f,"w",encoding="utf8").write(t.replace(a,b))
t=open("p_reader.py",encoding="utf8").read()
t=t.replace("    if t >= 1:\n        bb_ = wbox","    if True:\n        bb_ = wbox")
open("p_reader.py","w",encoding="utf8").write(t)
