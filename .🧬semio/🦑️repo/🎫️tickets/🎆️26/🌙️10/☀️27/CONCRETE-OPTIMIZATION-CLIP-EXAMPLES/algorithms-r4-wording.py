"""✏️ Round 4 caption wording edits of the algorithms scene (written through a scratch copy, then copied into place)."""
import sys
from pathlib import Path
src = Path(sys.argv[1]).read_text(encoding="utf-8")
for old, new in (
    ('"Die Mustersuche kommt als erste auf 1 % ans Minimum heran, nach 16 Simulationen, ganz ohne Steigung."', '"Die Mustersuche kommt nach 16 Simulationen als erste auf 1 % ans Minimum, ganz ohne Steigung."'),
    ('"Die Toleranz greift zuerst: Der genetische Lauf stoppte schon nach 15 Simulationen, noch weit vom Besten."', '"Die Toleranz greift zuerst: Schon nach 15 Simulationen stoppte der genetische Lauf, noch weit vom Besten."'),
    ('"None of them proves the global optimum; only the full map with 1,681 simulations could show it."', '"None of them proves the global optimum; for that we need the full map, 1,681 simulations."'),
    ('"Keines beweist das globale Optimum; nur die komplette Karte (1.681 Simulationen) könnte es zeigen."', '"Keines beweist das globale Optimum; dafür brauchen wir die volle Karte, 1.681 Simulationen."'),
):
    assert src.count(old) == 1, old[:60]
    src = src.replace(old, new)
Path(sys.argv[2]).write_text(src, encoding="utf-8")
