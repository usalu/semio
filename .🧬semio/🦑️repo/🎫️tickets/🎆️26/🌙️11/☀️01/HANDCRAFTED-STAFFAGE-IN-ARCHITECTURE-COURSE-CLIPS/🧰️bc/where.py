import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent.parent / "🧰️bl"))
import audit_bilingual as a
maps=[l.rstrip("\n").split("\t") for l in open(Path(__file__).parent/"map.tsv",encoding="utf8") if l.strip()]
files=sorted(a.CATALOG.rglob("🔣️.json"))
texts={f:f.read_text(encoding="utf8") for f in files}
for old,new in maps:
    key=f'"de": "{old}"'
    hits=[f"{f.parent.name}:{t.count(key)}" for f,t in texts.items() if key in t]
    print(f"{old} -> {new} :: {' '.join(hits)}")
