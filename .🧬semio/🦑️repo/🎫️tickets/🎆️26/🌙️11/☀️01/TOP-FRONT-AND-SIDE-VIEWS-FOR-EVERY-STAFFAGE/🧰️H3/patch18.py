s=open("human.py",encoding="utf8").read()
s=s.replace('                tx1 -= P.get("trim", 0.0)\n','')
s=s.replace(' - P.get("arm_trim", 0.0)','')
s=s.replace('xf = skull[2] - P.get("head_trim", 0.0); xb = hair[0]','xf = skull[2]; xb = hair[0]')
open("human.py","w",encoding="utf8").write(s)
print(s.count("trim"))
for f,a,b in (("p_pregnant.py",", p500=dict(trim=.1)",""),("p_child.py",", p500=dict(trim=.1, head_trim=.04, arm_trim=.05)","")):
    t=open(f,encoding="utf8").read(); assert a in t,f; open(f,"w",encoding="utf8").write(t.replace(a,b))
