import re,sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent.parent / "🧰️bl"))
import audit_bilingual as a
want=set(sys.argv[1:])
for f in a.CATALOG.rglob("🔣️.json"):
    d=a.load(f)
    if not isinstance(d.get("asset"),dict): continue
    s=[]; a.walk(d,"","",s,[])
    for p,en,de in s:
        if not(isinstance(en,str) and isinstance(de,str)): continue
        dt={t.lower() for t in a.TOKEN.findall(de)}&{t.lower() for t in a.TOKEN.findall(en)}&want
        if dt: print(d["asset"]["id"],p[-30:],sorted(dt),"|",en[:90],"|",de[:110])
