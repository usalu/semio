s=open("human.py",encoding="utf8").read()
a='cap(o, f"upper-arm-{sd}", arm, R.o(arm)[0], lat, R.t(arm)[0], lat,'
b='cap(o, f"forearm-{sd}", fa, R.o(fa)[0], lat, R.t(fa)[0], lat,'
assert a in s and b in s
s=s.replace(a,'cap(o, f"upper-arm-{sd}", arm, R.o(arm)[0], lat, R.t(arm)[0] - (max(P["a1"] - 0.03, 0) if sc == "500" else 0), lat,')
s=s.replace(b,'cap(o, f"forearm-{sd}", fa, R.o(fa)[0], lat, R.t(fa)[0] - (max(P["f1"] - 0.03, 0) if sc == "500" else 0), lat,')
open("human.py","w",encoding="utf8").write(s)
