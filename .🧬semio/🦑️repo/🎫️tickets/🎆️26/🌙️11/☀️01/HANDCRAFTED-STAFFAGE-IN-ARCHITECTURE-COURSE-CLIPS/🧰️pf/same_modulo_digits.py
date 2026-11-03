"""🔍 Compares two cast folders byte for byte, except that `data-value` is compared at 6 significant digits (the only intended difference of the optimisation)."""
import re
import sys
from pathlib import Path

def norm(text):
    return re.sub(r'data-value="([^"]*)"', lambda m: 'data-value="%s"' % float(("%.6g" % float(m.group(1)))), text)

a, b = Path(sys.argv[1]), Path(sys.argv[2])
names_a = sorted(p.name for p in a.iterdir())
names_b = sorted(p.name for p in b.iterdir())
if names_a != names_b:
    print("  FILES DIFFER", len(names_a), len(names_b))
    sys.exit(1)
bad = [n for n in names_a if norm((a / n).read_text(encoding="utf-8")) != norm((b / n).read_text(encoding="utf-8"))]
print("  identical modulo data-value digits" if not bad else "  DIFFERENT %s" % bad[:3])
