import os
import sys
from pathlib import Path

root = Path(sys.argv[1])
target = next(p for p in root.rglob("🐍️.py") if p.parent.name == sys.argv[2] and "ventilation" in str(p) and "clip" in str(p))
snippet = Path(sys.argv[3])
start, end = sys.argv[4], sys.argv[5]
text = target.read_text(encoding="utf-8")
i = text.index(start)
j = text.index(end, i)
new = snippet.read_text(encoding="utf-8")
text = text[:i] + new.rstrip("\n") + "\n\n\n" + text[j:]
tmp = target.with_name("tmp_splice.txt")
tmp.write_text(text, encoding="utf-8")
os.replace(tmp, target)
print("spliced", target, i, j)
