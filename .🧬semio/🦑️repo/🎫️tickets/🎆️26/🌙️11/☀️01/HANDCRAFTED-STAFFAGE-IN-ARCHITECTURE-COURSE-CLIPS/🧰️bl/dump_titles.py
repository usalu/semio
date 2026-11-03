"""📜 Dump every en/de title pair of referenced staffage assets for manual review."""
import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS\🧰️bl")
import audit_bilingual as a
from pathlib import Path
manifest = a.load(a.MANIFEST)
index = {}
for f in a.CATALOG.rglob("🔣️.json"):
    d = a.load(f)
    if isinstance(d.get("asset"), dict) and "id" in d["asset"]:
        index[d["asset"]["id"]] = f
ids = sorted({c["asset"] for c in manifest["casts"].values()})
rows = []
for aid in ids:
    f = index[aid]
    sink = []
    a.walk(a.load(f), "", str(f.relative_to(a.CATALOG)), sink, [])
    for path, en, de in sink:
        if path.endswith(".title") or path.endswith(".description") and len(en or "") < 60:
            rows.append(f"{aid}\t{path}\t{en}\t{de}")
(a.OUT / "titles.tsv").write_text("\n".join(rows) + "\n", encoding="utf-8")
print(len(rows))
