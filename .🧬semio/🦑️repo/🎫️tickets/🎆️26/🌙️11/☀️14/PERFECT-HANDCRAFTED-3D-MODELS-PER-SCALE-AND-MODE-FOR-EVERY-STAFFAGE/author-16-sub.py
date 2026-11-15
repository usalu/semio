import sys
f, old, new = sys.argv[1:4]
s = open(f).read()
assert old in s, "pattern not found: " + old
open(f, "w").write(s.replace(old, new))
