import sys
f, a, b = sys.argv[1:4]
s = open(f).read()
if a not in s: raise SystemExit("not found: " + a)
open(f, "w").write(s.replace(a, b))
