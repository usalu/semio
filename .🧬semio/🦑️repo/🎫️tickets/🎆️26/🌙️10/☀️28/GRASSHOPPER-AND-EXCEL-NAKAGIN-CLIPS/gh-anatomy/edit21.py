import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a[:70]
    s = s.replace(a, b, 1)
rep('(f"{i} → {self.number(values[i], 2)} m", (0, tower_index, i, 1, 0), 0.7, 0.0)', '(f"{i} → {self.number(values[i], 1)} m", (0, tower_index, i, 1, 0), -0.6, 0.0)')
rep('_at(0.7, -0.5))]', '_at(1.3, -0.55))]')
rep('(self.tr("storey 3", "Geschoss 3"), (0, 0, 3, 1, 0), 0.75, 0.22), (self.tr("facade 1 · east", "Fassade 1 · Ost"), (0, 0, 3, 1, 0), 0.75, -0.22)', '(self.tr("storey 3", "Geschoss 3"), (0, 0, 3, 1, 0), -0.6, 0.22), (self.tr("facade 1 (E)", "Fassade 1 (O)"), (0, 0, 3, 1, 0), -0.6, -0.22)')
rep('(self.tr("path {0;0;3;1} · index 1", "Pfad {0;0;3;1} · Index 1"), (0, 0, 3, 1, 1), 0.75, 0.0)', '("{0;0;3;1} [1]", (0, 0, 3, 1, 1), -0.6, 0.0)')
rep('scale=0.05, lit=lambda k: k == (0, 0, 3, 1, 1)', 'scale=0.065, lit=lambda k: k == (0, 0, 3, 1, 1)')
open(p, "w", encoding="utf-8").write(s)
s = open(p, encoding="utf-8").read()
a = "baut aus Breite, Länge, Höhe eine Box. Der Viewport zeigt sie."
assert a in s
open(p, "w", encoding="utf-8").write(s.replace(a, "baut aus Breite, Länge und Höhe eine Box."))
