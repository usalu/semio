s=open("human.py",encoding="utf8").read()
old='    fl = .08 if sc == "500" else .04\n'
assert old in s
s=s.replace(old,'    fl = .08 if sc == "500" else .04\n    fl5 = P.get("fl5", (.08, .09, .09))\n')
s=s.replace('q[k] = max(P[k], fl + (.01 if k in ("fw", "a0", "a1", "f0", "f1", "hr") and sc == "500" else 0))','q[k] = max(P[k], (fl5[2] if k == "fw" else fl5[1] if k in ("a0", "a1", "f0", "f1", "hr") else fl5[0]) if sc == "500" else fl)')
open("human.py","w",encoding="utf8").write(s)
t=open("p_child.py",encoding="utf8").read().replace("sd=.06)","sd=.06, fl5=(.05, .055, .05))")
open("p_child.py","w",encoding="utf8").write(t)
e=open("elev500.py",encoding="utf8").read()
e=e.replace('TARGET = {"arm": .09, "leg": .08}','TARGET = {"arm": .09, "leg": .08}\nOVERRIDE = {"running-child": {"arm": .055, "leg": .05}}')
e=e.replace("        k = TARGET[f] / cur\n        if k <= 1.001: continue","        tgt = OVERRIDE.get(aid, TARGET)[f]\n        k = tgt / cur\n        if (k <= 1.001 and aid not in OVERRIDE) or abs(k - 1) < .001: continue")
open("elev500.py","w",encoding="utf8").write(e)
c=open("child500.py",encoding="utf8").read()
c=c.replace('("arm-skin-r", .09), "sil-forearm-r": ("forearm-r", .09), "sil-shank-l": ("shank-l", .08), "sil-shank-r": ("shank-r", .08)','("arm-skin-r", .055), "sil-forearm-r": ("forearm-r", .055), "sil-shank-l": ("shank-l", .05), "sil-shank-r": ("shank-r", .05)')
c=c.replace('"region": m100[src]["region"]})','**({"regions": ["leg-skin", "socks"]} if "shank" in s["id"] else {"region": m100[src]["region"]})})')
c=c.replace("    if s[\"id\"] in MAP and s[\"kind\"] == \"poly\":","    if s[\"id\"] in MAP and s.get(\"kind\") in (\"poly\", \"capsule\"):\n        if s[\"kind\"] == \"capsule\": s.update(kind=\"poly\", points=[[0, 0], [s[\"x1\"] + s[\"r1\"], 0]])")
open("child500.py","w",encoding="utf8").write(c)
d=open("dims.py",encoding="utf8").read()
d+='''
doc = load("pregnant-person")
doc["asset"]["dimensions"]["widthM"] = 0.69
doc["asset"]["dimensions"]["depthM"] = 0.5
save("pregnant-person", doc)
print("pregnant dims")
'''
open("dims.py","w",encoding="utf8").write(d)
