"""🪪 Find titles that start lowercase or hold raw kebab-case ids (read-only)."""
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "🧰️bl"))
import audit_bilingual as a

KEBAB = re.compile(r"\b[a-z]+(?:-[a-z0-9]+)+\b")
OK = re.compile(r"^(e-|x-|u-|t-|a-|v-|s-|y-)")
for f in sorted(a.CATALOG.rglob("🔣️.json")):
    d = a.load(f)
    if not isinstance(d.get("asset"), dict):
        continue
    s = []
    a.walk(d, "", "", s, [])
    for p, en, de in s:
        if not (isinstance(en, str) and isinstance(de, str)):
            continue
        if p.endswith(".title") and (en[:1].islower() or de[:1].islower()):
            print("LOWER", d["asset"]["id"], p, "|", en, "|", de)
        if p.endswith(".title") or "events" in p:
            for m in KEBAB.findall(en) + KEBAB.findall(de):
                print("KEBAB", d["asset"]["id"], p, "|", en, "|", de)
                break
