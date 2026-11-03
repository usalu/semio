"""🖨️ Print the extracted claim rows as compact lines for review."""
import json
import sys
from pathlib import Path

rows = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
start, stop = int(sys.argv[2]), int(sys.argv[3])
for index, row in enumerate(rows[start:stop], start):
    name = row["file"].split("/clip/", 1)[-1] if "/clip/" in row["file"] else row["file"]
    print(f'{index} {name} :{row["line"]} | {row["claim"]} | {row["text"][:150]!r}')
