import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
a = t.index("## Table")
b = t.index("## Fixed")
new = open(sys.argv[2], encoding="utf-8").read()
open(p, "w", encoding="utf-8").write(t[:a] + new + t[b:])
