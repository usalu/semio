import sys,json,re
from pathlib import Path
sys.path.insert(0,str(Path(__file__).parent.parent/"🧰️bl"))
import audit_bilingual as a
aid,pat=sys.argv[1],re.compile(sys.argv[2])
for f in a.CATALOG.rglob("🔣️.json"):
    d=a.load(f)
    if d.get("asset",{}).get("id")!=aid: continue
    s=[]
    a.walk(d,"",aid,s,[])
    for p,en,de in s:
        if pat.search(p): print(p,"\n  EN:",en,"\n  DE:",de)
