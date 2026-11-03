"""🙈 Declares the interior region hidden below 1:50 and stages the file for copying."""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[8] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles"
OUT = Path(__file__).resolve().parents[1] / "🗑️generated" / "cp"
HIDDEN = {
    "en": "The interior fittings are only drawn at 1:50; at 1:100 and coarser they are too small to print and the glazing and body panels carry the cabin.",
    "de": "Die Innenausstattung wird nur bei 1:50 gezeichnet; bei 1:100 und gröber ist sie zu klein zum Drucken und Verglasung und Karosserieblech tragen die Kabine.",
}
for name in sys.argv[1:]:
    raw = (ROOT / name / "🔣️.json").read_bytes().decode("utf-8")
    second = raw.replace("\r\n", "\n").split("\n")[1]
    indent = len(second) - len(second.lstrip())
    doc = json.loads(raw)
    for region in doc["asset"]["thermal"]["regions"]:
        if region["id"] == "interior":
            region["hidden"] = HIDDEN
    text = json.dumps(doc, ensure_ascii=False, indent=indent) + "\n"
    if "\r\n" in raw:
        text = text.replace("\n", "\r\n")
    (OUT / f"{name}.json").write_bytes(text.encode("utf-8"))
    print("staged", name)
