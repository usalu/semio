s=open("human.py",encoding="utf8").read()
i=s.index("def coarse(P, sc):"); j=s.index("def over(")
s=s[:i]+'''def coarse(P, sc):
    """Coarse scales need thicker limbs (print minimum feature); the outer arm edge and the lateral extent stay where they are."""
    if sc not in ("500", "200"): return P
    P = dict(P, **P.get("p500", {})) if sc == "500" else dict(P)
    fl = .058 if sc == "500" else .024
    q = dict(P)
    for k in ("t0", "t1", "s0", "s1", "fw", "a0", "a1", "f0", "f1", "hr", "nw"): q[k] = max(P[k], fl)
    q["ac"] = P["ac"] + P["a0"] - q["a0"]
    return q


'''+s[j:]
s=s.replace('ell(o, f"knee-{sd}", sk, lat, knee_y, P["t1"] * 1.02, P["t1"] * 0.95, 1.01,','ell(o, f"knee-{sd}", sk, lat, knee_y, P["t1"] * 1.02, P["t1"] * 0.95, 1.03,')
s=s.replace('P["a0"], P["a0"], 4.5, h.S(sc, "sleeve", sd))','P["a0"], P["a0"], 4.62, h.S(sc, "sleeve", sd))')
s=s.replace('P["a1"], P["a1"], 4.62, h.S(sc, "sleeve", sd))','P["a1"], P["a1"], 4.65, h.S(sc, "sleeve", sd))')
s=s.replace('if (low and "shoe-toe" in h.KEY) else R.t(tb)[0] + 0.02\n','if (low and "shoe-toe" in h.KEY) else R.t(tb)[0] + 0.02\n                tx1 -= P.get("trim", 0.0)\n')
open("human.py","w",encoding="utf8").write(s)

def sub(f,a,b):
    t=open(f,encoding="utf8").read()
    assert a in t,(f,a)
    open(f,"w",encoding="utf8").write(t.replace(a,b,1))
sub("p_pregnant.py","pd=.115, tl=.055, cd=.115, sd=.1)","pd=.115, tl=.055, cd=.115, sd=.1, p500=dict(trim=.05))")
sub("p_child.py","pd=.08, tl=.04, cd=.075, sd=.06)","pd=.08, tl=.04, cd=.075, sd=.06, p500=dict(trim=.04))")
# runner blade at 500
sub("p_runner.py","    ell(of, \"blade-toe\", \"blade-tip-r\", -hl, cy1 + 0.014, .036, .016, 2.6, s(\"blade-toe\"))","    bw = .055 if t == 0 else .03\n    ell(of, \"blade-toe\", \"blade-tip-r\", -hl, cy1 + 0.014, .036 if t else .06, .016 if t else .045, 2.6, s(\"blade-toe\"))")
t=open("p_runner.py",encoding="utf8").read()
t=t.replace("limb(-hl, by0, by1, .03, .03)","limb(-hl, by0, by1, bw, bw)").replace("limb(-hl, cy0, cy1, .03, .03)","limb(-hl, cy0, cy1, bw, bw)")
t=t.replace("R.t(\"blade-r\")[0], -hl, .03, .03,","R.t(\"blade-r\")[0], -hl, bw, bw,").replace("R.t(\"blade-tip-r\")[0], -hl, .03, .03,","R.t(\"blade-tip-r\")[0], -hl, bw, bw,")
t=t.replace("    bw = .055 if t == 0 else .03\n","")
t=t.replace("    s = lambda i: h.st.get(sc, i)\n","    s = lambda i: h.st.get(sc, i)\n    bw = .055 if t == 0 else .03\n",1)
open("p_runner.py","w",encoding="utf8").write(t)
# guitarist tuners
t=open("p_guitarist.py",encoding="utf8").read()
t=t.replace("for i, ly in enumerate((.02, 0.0, -.02)):","for i, ly in enumerate((.03, 0.0, -.03)):").replace("gp(.74 - i * .02)[0]","gp(.76 - i * .035)[0]")
open("p_guitarist.py","w",encoding="utf8").write(t)
# reader: pad/page bigger at 500, bun from elevation
t=open("p_reader.py",encoding="utf8").read()
t=t.replace("    lat = P[\"ac\"]\n","    lat = P[\"ac\"]\n    pwid, gwid = (.11, .105) if t == 0 else (.075, .07)\n")
t=t.replace("lat - .075, pw[0][1]","lat - pwid, pw[0][1]").replace("lat - .075, pw[1][1]","lat - pwid, pw[1][1]").replace("(lat + .075, pw[1][1]), (lat + .075, pw[0][1])","(lat + pwid, pw[1][1]), (lat + pwid, pw[0][1])").replace(".075, .075, 6.4",  "pwid, pwid, 6.4")
t=t.replace("lat - .07, gw[0][1]","lat - gwid, gw[0][1]").replace("lat - .07, gw[1][1]","lat - gwid, gw[1][1]").replace("(lat + .07, gw[1][1]), (lat + .07, gw[0][1])","(lat + gwid, gw[1][1]), (lat + gwid, gw[0][1])").replace(".068, .068, 6.5","gwid - .002, gwid - .002, 6.5")
i=t.index("    ell(of, \"hair-bun\"")
t=t[:i]+'''    if t >= 1:
        bb_ = wbox(R, h.st, sc, "head", "hair-bun")
        ell(of, "hair-bun", "head", 0.0, (bb_[1] + bb_[3]) / 2, max(.035, (bb_[3] - bb_[1]) / 2), (bb_[3] - bb_[1]) / 2, 5.6, s("hair-bun"))
        ell(op, "hair-bun", "head", (bb_[0] + bb_[2]) / 2, 0.0, (bb_[2] - bb_[0]) / 2, (bb_[2] - bb_[0]) / 2, 6.2, s("hair-bun"))
'''
open("p_reader.py","w",encoding="utf8").write(t)
print("ok")
