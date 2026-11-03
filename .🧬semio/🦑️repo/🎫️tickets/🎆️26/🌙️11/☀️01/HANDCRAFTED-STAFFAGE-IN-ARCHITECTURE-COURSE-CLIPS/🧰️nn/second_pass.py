"""🔁 Second pass: every clip string that names a staffage device or a unit word, digits or not, for manual review."""
import ast
import json
import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
OUT = Path(sys.argv[2])
DEVICE = re.compile(
    r"Teelicht|tealight|Mensch|Person|Personen|Toaster|toaster|Lampe|lamp|Leuchte|Laptop|Notebook|Server|Wasserkocher|kettle|Kochtopf|Topf|pot\b|"
    r"Herd|Kochfeld|hob|Dusche|shower|Staubsauger|vacuum|Heizk|radiator|Thermostat|thermostat|Pumpe|pump|Pumpjack|Pumpenheber|Boiler|Kessel|boiler|"
    r"Ventilator|fan|Lüfter|MVHR|WRG|Gerät|appliance|Klima|split|Verdunst|humidif|Wärmepumpe|heat pump|Speicher|cylinder|Durchlauf|Haarf|Fön|"
    r"Mensch|sleep|schlaf|Schlaf|walker|jogger|Büro|office|Kühlschrank|fridge|Ofen|Heizlüfter|heater|Kompressor|compressor|Ventil|valve|Steckdose|Monitor|Bildschirm|"
    r"Watt|Kilowatt|Liter|Litre|Minute|Stunde|Sekunde|Grad|Gramm|Kilogramm|kilogram|gram|minute|hour|second|watt|litre|liter|degree|Kelvin|Joule|joule|Prozent|percent|Meter|metre|meter|Zentimeter"
)
UNIT = re.compile(r"\d")
rows = []
for path in sorted(ROOT.rglob("🐍️.py")):
    if "🎬️clip" not in path.parts:
        continue
    rel = "🎓️teaching" + str(path).replace("\\", "/").split("🎓️teaching", 1)[1]
    tree = ast.parse(path.read_text(encoding="utf-8"))
    for node in ast.walk(tree):
        if isinstance(node, ast.Constant) and isinstance(node.value, str):
            text = node.value
            if DEVICE.search(text) and (UNIT.search(text) or DEVICE.search(text)):
                rows.append({"file": rel, "line": node.lineno, "text": text})
OUT.write_text(json.dumps(rows, ensure_ascii=False, indent=1), encoding="utf-8")
print(len(rows))
