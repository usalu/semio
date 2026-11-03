import sys
from collections import defaultdict
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent.parent / "🧰️bl"))
import audit_bilingual as a
maps=[l.rstrip("\n").split("\t") for l in open(Path(__file__).parent/"map.tsv",encoding="utf8") if l.strip()]
per=defaultdict(list)
for f in sorted(a.CATALOG.rglob("🔣️.json")):
    t=f.read_text(encoding="utf8")
    for old,new in maps:
        if f'"de": "{old}"' in t: per[f].append((old,new))
for f,l in per.items():
    print(f"{f.relative_to(a.CATALOG)}: "+"; ".join(o+" >> "+n for o,n in l))
print(len(per), sum(len(l) for l in per.values()))
