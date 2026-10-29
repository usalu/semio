import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
a = "Ein Parameter hält Daten eines Typs. Frei auf dem Canvas steht er allein; als Eingang X einer Komponente gehört er zu einem Port. Beide sind Number."
b = "Ein Parameter hält Daten eines Typs. Frei steht er allein; als Eingang X einer Komponente gehört er zu einem Port. Beide sind Number."
assert a in s
open(p, "w", encoding="utf-8").write(s.replace(a, b))
