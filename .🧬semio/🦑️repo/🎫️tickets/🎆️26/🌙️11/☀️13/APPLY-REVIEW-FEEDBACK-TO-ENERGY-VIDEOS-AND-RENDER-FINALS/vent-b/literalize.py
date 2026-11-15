import os
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
target = next(p for p in root.rglob("🐍️.py") if p.parent.name == "🪟️natural" and "ventilation" in str(p) and "clip" in str(p))
values = {"re": "1.212", "ri": "1.180", "drho": "0.032", "dp": "0.8", "v": "1.2", "q": "0.069", "qh": "250",
          "dpt": "1.4", "dpw": "3.8", "ratio": "2.7"}
text = target.read_text(encoding="utf-8")


def fill(match):
    body, de = match.group(1), match.group(2) == "True"
    for key, value in values.items():
        body = body.replace(f"@{key}@", value.replace(".", "{,}") if de else value)
    return body


text, count = re.subn(r'_say\((r"[^\n]*?"), _SAY, (False|True)\)', fill, text)
tmp = target.with_name("tmp_literal.txt")
tmp.write_text(text, encoding="utf-8")
os.replace(tmp, target)
sys.stdout.buffer.write(f"replaced {count}\n".encode())
