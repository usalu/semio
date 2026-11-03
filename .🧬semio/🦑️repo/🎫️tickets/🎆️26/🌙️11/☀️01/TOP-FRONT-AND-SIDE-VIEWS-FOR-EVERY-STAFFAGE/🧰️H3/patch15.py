import re
s=open("human.py",encoding="utf8").read()
s=re.sub(r' - \(max\(P\["a1"\] - 0.03, 0\) if sc == "500" else 0\) \* \(1 if R.t\(arm\)\[0\] >= R.o\(arm\)\[0\] - 1e-6 else -1\)','',s)
s=re.sub(r' - \(max\(P\["f1"\] - 0.03, 0\) if sc == "500" else 0\) \* \(1 if R.t\(fa\)\[0\] >= R.o\(fa\)\[0\] - 1e-6 else -1\)','',s)
open("human.py","w",encoding="utf8").write(s)
print(s.count("0.03, 0"))
