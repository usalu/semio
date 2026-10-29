import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep('at=_at(2.15, 0.35))', 'at=_at(2.0, 0.35))')
rep('at=_at(3.75, 0.35), width=1.15)', 'at=_at(3.7, 0.35), width=1.5)')
open(p, "w", encoding="utf-8").write(s)
