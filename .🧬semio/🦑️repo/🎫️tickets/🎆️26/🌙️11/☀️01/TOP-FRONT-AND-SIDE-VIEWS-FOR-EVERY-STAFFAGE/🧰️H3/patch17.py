s=open("human.py",encoding="utf8").read()
old='    for k in ("t0", "t1", "s0", "s1", "fw", "a0", "a1", "f0", "f1", "hr", "nw"): q[k] = max(P[k], fl)\n'
assert old in s
s=s.replace(old,'    for k in ("t0", "t1", "s0", "s1", "fw", "a0", "a1", "f0", "f1", "hr", "nw"): q[k] = max(P[k], fl + (.01 if k in ("fw", "a0", "a1", "f0", "f1", "hr") and sc == "500" else 0))\n')
s=s.replace("FIST = {0: (.09, .09)","FIST = {0: (.1, .1)")
open("human.py","w",encoding="utf8").write(s)
t=open("p_runner.py",encoding="utf8").read()
t=t.replace("bw = .07 if t == 0 else .03","bw = .09 if t == 0 else .03").replace(".036 if t else .06, .016 if t else .045",".036 if t else .08, .016 if t else .06")
open("p_runner.py","w",encoding="utf8").write(t)
g=open("p_guitarist.py",encoding="utf8").read()
g=g.replace('(.31, .024)], 3.85, gn); sol(out, -.024, .024)','(.31, .024)], 3.85, gn); sol(out, -.024 if t else -.05, .024 if t else .05)')
open("p_guitarist.py","w",encoding="utf8").write(g)
