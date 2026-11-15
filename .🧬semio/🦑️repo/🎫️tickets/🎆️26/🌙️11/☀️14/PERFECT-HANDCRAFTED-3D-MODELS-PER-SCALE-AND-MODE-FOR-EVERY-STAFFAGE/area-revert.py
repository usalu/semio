"""🧯 Restores committed `areaM2` values of exchange-area regions (ruling C16 class a) in catalog assets.

Reads T/area-audit.json, re-reads each asset right before writing, replaces only the number token of
`areaM2` inside the named thermal region and verifies that nothing else changed.
"""
import json
import re
import sys

audit = json.load(open(sys.argv[1], encoding="utf-8"))
todo = {}
for row in audit["areas"]:
    if row["class"] == "a":
        todo.setdefault(row["path"], []).append(row)
for path, rows in todo.items():
    text = open(path, encoding="utf-8").read()
    before = json.loads(text)
    regions = before["asset"]["thermal"]["regions"]
    tokens = list(re.finditer(r'"areaM2"\s*:\s*(-?[0-9.eE+-]+)', text))
    if len(tokens) != len(regions):
        print("SKIP token count", path, len(tokens), len(regions))
        continue
    edits = []
    for row in rows:
        index = next(i for i, region in enumerate(regions) if region["id"] == row["region"])
        if regions[index]["areaM2"] == row["old"]:
            continue
        edits.append((tokens[index].span(1), repr(row["old"]) if not float(row["old"]).is_integer() else str(row["old"]), index, row))
    for span, value, index, row in sorted(edits, key=lambda e: -e[0][0]):
        text = text[: span[0]] + value + text[span[1] :]
    after = json.loads(text)
    for _, _, index, row in edits:
        before["asset"]["thermal"]["regions"][index]["areaM2"] = after["asset"]["thermal"]["regions"][index]["areaM2"]
        if after["asset"]["thermal"]["regions"][index]["areaM2"] != row["old"]:
            raise SystemExit("value mismatch " + path)
    if before != after:
        raise SystemExit("other change " + path)
    if edits:
        open(path, "w", encoding="utf-8").write(text)
    print(len(edits), "restored", path.split("/")[-2])
