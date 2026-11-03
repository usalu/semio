s=open("human.py",encoding="utf8").read()
for nm,sub_ in (('upper-arm','a1'),('forearm','f1')):
    old=f'- (max(P["{sub_}"] - 0.03, 0) if sc == "500" else 0), lat,'
    assert old in s
s=s.replace('R.t(arm)[0] - (max(P["a1"] - 0.03, 0) if sc == "500" else 0), lat,','R.t(arm)[0] - (max(P["a1"] - 0.03, 0) if sc == "500" else 0) * (1 if R.t(arm)[0] >= R.o(arm)[0] else -1), lat,')
s=s.replace('R.t(fa)[0] - (max(P["f1"] - 0.03, 0) if sc == "500" else 0), lat,','R.t(fa)[0] - (max(P["f1"] - 0.03, 0) if sc == "500" else 0) * (1 if R.t(fa)[0] >= R.o(fa)[0] else -1), lat,')
s=s.replace("            xf = skull[2]; xb = hair[0]","            xf = skull[2] - P.get(\"head_trim\", 0.0); xb = hair[0]")
open("human.py","w",encoding="utf8").write(s)
t=open("p_child.py",encoding="utf8").read().replace("p500=dict(trim=.1)","p500=dict(trim=.1, head_trim=.04)")
open("p_child.py","w",encoding="utf8").write(t)
