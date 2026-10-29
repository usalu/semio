import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
a = "fügt dem Ribbon einen Tab mit neuen Komponenten hinzu. Ladybug"
assert a in s
open(p, "w", encoding="utf-8").write(s.replace(a, "fügt dem Ribbon einen Tab hinzu. Ladybug"))
