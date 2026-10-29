import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep('at=_at(CB_AT[0], -0.2), state=sol.state(key), nickname=True)', 'at=_at(1.1, -0.2), state=sol.state(key), nickname=True)')
rep('at=_at(2.6, -0.2), max_lines=4)', 'at=_at(3.1, -0.2), max_lines=4)')
open(p, "w", encoding="utf-8").write(s)
