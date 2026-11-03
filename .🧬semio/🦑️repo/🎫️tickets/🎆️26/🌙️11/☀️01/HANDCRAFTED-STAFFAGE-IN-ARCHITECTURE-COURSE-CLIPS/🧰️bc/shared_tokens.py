"""🔤 List tokens shared verbatim between en and de of a pair, by frequency (read-only)."""
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "🧰️bl"))
import audit_bilingual as a

c = Counter()
ex = defaultdict(list)
for f in a.CATALOG.rglob("🔣️.json"):
    d = a.load(f)
    if not isinstance(d.get("asset"), dict):
        continue
    s = []
    a.walk(d, "", "", s, [])
    for p, en, de in s:
        if not (isinstance(en, str) and isinstance(de, str)):
            continue
        et = {t.lower() for t in a.TOKEN.findall(en)}
        for t in {t.lower() for t in a.TOKEN.findall(de)} & et:
            if len(t) > 3 and not t.isupper():
                c[t] += 1
                if len(ex[t]) < 1:
                    ex[t].append(f"{d['asset']['id']}{p}: {de[:70]}")
for t, n in c.most_common(400):
    print(n, t, "|", ex[t][0])
