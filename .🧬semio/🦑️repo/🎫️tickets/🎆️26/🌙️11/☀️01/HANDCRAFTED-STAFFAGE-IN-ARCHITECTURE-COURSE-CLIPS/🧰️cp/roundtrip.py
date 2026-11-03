"""🔁 Checks that json load and dump with the file's indent reproduces the vehicle files byte for byte."""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[8] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles"
for name in sys.argv[1:]:
    raw = (ROOT / name / "🔣️.json").read_bytes().decode("utf-8")
    second = raw.replace("\r\n", "\n").split("\n")[1]
    indent = len(second) - len(second.lstrip())
    text = json.dumps(json.loads(raw), ensure_ascii=False, indent=indent) + "\n"
    if "\r\n" in raw:
        text = text.replace("\n", "\r\n")
    print(name, indent, text == raw)
    import difflib
    a = raw.split("\n"); b = text.split("\n")
    d = [l for l in difflib.unified_diff(a, b, lineterm="", n=0)][:14]
    print("\n".join(x[:120] for x in d))
