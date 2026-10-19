import sys
p, old, new = sys.argv[1], sys.argv[2], sys.argv[3]
s = open(p, encoding="utf-8").read()
n = s.count(old)
if n != 1 and "--all" not in sys.argv[4:]:
    sys.exit(f"match count {n}")
open(p, "w", encoding="utf-8").write(s.replace(old, new))
print("ok", n)
