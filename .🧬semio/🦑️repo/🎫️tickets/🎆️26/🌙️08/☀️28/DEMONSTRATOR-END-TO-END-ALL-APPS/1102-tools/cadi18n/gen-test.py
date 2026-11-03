import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from mk import create
ED = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
py = '''"""🌐️ Language-agnostic check of the cad interaction specs: every viewer-facing text (`label`, `prompt`, label-item `text`) is a complete locale matrix, English and German both non-blank, and no plain string remains.
Independent oracle for the Rust test `every_interaction_text_carries_both_languages_and_no_blank_cell`: standard-library `json` only, no code of the cad crates.
Run by path (`python3 🐍️.py`); exits non-zero with every defect listed. See `LocalizedLabel` (`🧰️framework/🔨️modules/🖱️ui/🌐️locale`)."""
import json
import pathlib
import sys

LOCALES = {"en", "de"}
TERMINOLOGIES = {"native", "reuse"}
TEXT_KEYS = {"label", "prompt", "text"}
ASSETS = next(pathlib.Path(__file__).resolve().parents[5].glob("*/*/*modelDefinitions"))
defects = []
counts = {"texts": 0, "translated": 0, "files": 0}


def check_text(where, value):
    if not isinstance(value, dict) or set(value) != TERMINOLOGIES:
        defects.append(f"{where}: not a complete terminology matrix: {value!r}")
        return
    cells = {}
    for terminology, row in value.items():
        if not isinstance(row, dict) or set(row) != LOCALES:
            defects.append(f"{where}.{terminology}: not exactly {sorted(LOCALES)}: {row!r}")
            return
        for locale, text in row.items():
            if not isinstance(text, str) or not text.strip():
                defects.append(f"{where}.{terminology}.{locale}: blank text")
            cells[(terminology, locale)] = text
    counts["texts"] += 1
    counts["translated"] += cells[("native", "en")] != cells[("native", "de")]


def walk(where, node):
    if isinstance(node, dict):
        for key, child in node.items():
            if key in TEXT_KEYS:
                check_text(f"{where}.{key}", child)
            else:
                walk(f"{where}.{key}", child)
    elif isinstance(node, list):
        for index, child in enumerate(node):
            walk(f"{where}[{index}]", child)


for path in sorted(ASSETS.glob("*/*interactions/*.json")):
    counts["files"] += 1
    spec = json.loads(path.read_text(encoding="utf8"))
    if "label" not in spec:
        defects.append(f"{path.name}: the spec has no label")
    walk(path.name, spec)
if counts["files"] != 60:
    defects.append(f"{counts['files']} interaction specs found, expected 60")
if counts["texts"] < 600 or counts["translated"] * 100 < counts["texts"] * 85:
    defects.append(f"{counts['translated']} of {counts['texts']} texts differ between en and de")
print(f"{counts['files']} specs, {counts['texts']} texts, {counts['translated']} translated, {len(defects)} defects")
for defect in defects:
    print(defect)
sys.exit(1 if defects else 0)
'''
create(ED + "⚙️engine/🕹️interaction/🧪️tests/🌐️languages/🐍️.py", py, "cad-test-languages.patch")
