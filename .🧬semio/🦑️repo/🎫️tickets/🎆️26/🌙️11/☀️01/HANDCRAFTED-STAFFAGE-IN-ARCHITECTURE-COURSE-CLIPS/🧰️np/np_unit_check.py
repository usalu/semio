"""🧭 List en/de pairs whose numbers match but whose adjacent units conflict (both sides carry a known unit)."""
import sys
from collections import Counter
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent))
CATALOG, MANIFEST = Path(sys.argv[1]), Path(sys.argv[2])
sys.argv = sys.argv[:2] + [sys.argv[2], str(Path(__file__))]
import np_number_check as n
files = [MANIFEST] + [Path(r) / f for r, _, fs in __import__('os').walk(CATALOG) for f in fs if f.endswith('.json') and '🔣' in f]
found = []
for f in sorted(set(files)):
    n.walk(__import__('json').loads(f.read_text(encoding='utf-8')), '', str(f), found)
for f, p, en, de in found:
    ce, cd = Counter(n.extract(en, 'en')), Counter(n.extract(de, 'de'))
    ve = Counter(v for v, _ in ce.elements())
    vd = Counter(v for v, _ in cd.elements())
    for v in set(ve) & set(vd):
        if ve[v] != 1 or vd[v] != 1:
            continue
        ue = [u for (vv, u) in ce if vv == v and u]
        ud = [u for (vv, u) in cd if vv == v and u]
        if ue and ud and ue[0] != ud[0]:
            print(f.split('catalog')[-1], p, v, 'en', ue[0], 'de', ud[0])
