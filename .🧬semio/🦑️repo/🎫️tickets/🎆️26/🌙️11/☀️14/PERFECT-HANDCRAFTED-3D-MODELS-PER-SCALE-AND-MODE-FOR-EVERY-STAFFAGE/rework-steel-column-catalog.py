"""✍️ Hand edits of the catalog file of components/steel-column outside `volumes`, as exact text replacements so that every other byte stays: carrier travel ±0.1 m, one web shape in the 1:50 elevation, and the 1:100 drawing decision that names the true model thickness."""
import json
import re
from pathlib import Path

CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/steel-column/🔣️.json")
EN = " The model keeps the true 20 mm plates and 16 mm gussets; the drawn plates are 15 mm thicker, one model feature of this scale."
DE = " Das Modell behält die wahren 20-mm-Platten und 16-mm-Rippen; die gezeichneten Platten sind 15 mm dicker, ein Modellmerkmal dieses Maßstabs."


def once(text, pattern, replacement):
    result, count = re.subn(pattern, replacement, text, flags=re.S)
    assert count == 1, (pattern, count)
    return result


def main():
    text = CATALOG.read_text()
    text = once(text, r'"minM": -0\.2,(\s+)"maxM": 0\.2', r'"minM": -0.1,\1"maxM": 0.1')
    text = once(text, r',\s+\{\s+"id": "web-foot",.*?"closed": true\s+\}', "")
    text = once(text, r'("id": "web",.*?"points": \[\s+\[\s+)0\.14(,\s+0\.085\s+\],\s+\[\s+)0\.14(,)', r"\g<1>0.02\g<2>0.02\g<3>")
    text = once(text, r'(ein 24 mm stud\.)"', r"\1" + EN + '"') if False else text
    text = once(text, r'(each anchor are one 24 mm stud\.)"', r"\1" + EN + '"')
    text = once(text, r'(jedes Ankers sind ein 24-mm-Bolzen\.)"', r"\1" + DE + '"')
    json.loads(text)
    CATALOG.write_text(text)


if __name__ == "__main__":
    main()
