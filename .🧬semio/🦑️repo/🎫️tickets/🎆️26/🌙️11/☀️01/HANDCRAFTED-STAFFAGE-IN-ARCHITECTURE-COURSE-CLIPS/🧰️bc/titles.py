import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent.parent / "🧰️bl"))
import audit_bilingual as a
for f in sorted(a.CATALOG.rglob("🔣️.json")):
    d=a.load(f)
    t=d.get("asset",{}).get("title") if isinstance(d.get("asset"),dict) else None
    if t and "humans" in str(f): print(d["asset"]["id"],"|",t["en"],"|",t["de"])
