import json
import sys
from pathlib import Path

P = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\📊️demand\❓️quiz\🔣️.json")
s = P.read_text(encoding="utf-8")
for old, new in (
    ("Passivwohnung mit sehr geringem Nutzwärmebedarf", "Passivwohnung mit sehr geringem Heizwärmebedarf"),
    ("liefert Nutzwärme mit weniger bezogenem Strom", "liefert Wärme (Nutzenergie) mit weniger bezogenem Strom (Endenergie)"),
    ("supplies useful heat using less purchased electricity", "supplies heat (useful energy) using less purchased electricity (final energy)"),
    ("seasonal performance factor", "seasonal performance factor (JAZ)"),
    ("Jahresarbeitszahl", "Jahresarbeitszahl (JAZ)"),
    ("(the standard value of DIN V 4108-6)", "(flat rate for residential buildings, DIN V 4108-6:2003-06, Annex D)"),
    ("der Standardwert der DIN V 4108-6", "der Pauschalwert für Wohngebäude nach DIN V 4108-6:2003-06, Anhang D"),
):
    if old not in s:
        sys.exit(f"missing {old}")
    s = s.replace(old, new)
json.loads(s)
P.write_text(s, encoding="utf-8")
print("ok")
