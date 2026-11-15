"""🩹 Restores committed `areaM2` values of exchange-area regions (C16) and of body-model regions (C18).

Walks the text of `asset.thermal.regions` by bracket matching, takes the depth-one `areaM2` token of the named region and verifies that nothing else changed.
"""
import json
import re
import sys


def close(text, start):
    depth, index, quoted = 0, start, False
    while True:
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return index
        index += 1


def members(text, start, end):
    index, depth, quoted, spans, begin = start + 1, 0, False, [], None
    while index < end:
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            if depth == 0 and char == "{":
                begin = index
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0 and begin is not None:
                spans.append((begin, index))
                begin = None
        index += 1
    return spans


audit = json.load(open(sys.argv[1], encoding="utf-8"))
for row in audit["areas"]:
    if row["class"] != "a" and row["model"] != "body":
        continue
    text = open(row["path"], encoding="utf-8").read()
    before = json.loads(text)
    regions = before["asset"]["thermal"]["regions"]
    position = next(i for i, r in enumerate(regions) if r["id"] == row["region"])
    if regions[position]["areaM2"] == row["old"]:
        continue
    found = None
    for thermal in re.finditer(r'"thermal"\s*:\s*\{', text):
        end = close(text, thermal.end() - 1)
        key = re.compile(r'"regions"\s*:\s*\[').search(text, thermal.end(), end)
        if not key:
            continue
        spans = members(text, key.end() - 1, close(text, key.end() - 1))
        if len(spans) != len(regions):
            continue
        a, b = spans[position]
        depth, quoted, index = 0, False, a
        while index < b:
            char = text[index]
            if quoted:
                if char == "\\":
                    index += 1
                elif char == '"':
                    quoted = False
            elif char == '"':
                token = re.compile(r'"areaM2"\s*:\s*(-?[0-9.eE+-]+)').match(text, index)
                if token and depth == 1:
                    found = token.span(1)
                    break
                quoted = True
            elif char in "{[":
                depth += 1
            elif char in "}]":
                depth -= 1
            index += 1
        if found:
            break
    if not found:
        print("SKIP", row["slug"], row["region"])
        continue
    text = text[: found[0]] + json.dumps(row["old"]) + text[found[1] :]
    after = json.loads(text)
    regions[position]["areaM2"] = row["old"]
    if before != after:
        print("SKIP other change", row["slug"], row["region"])
        continue
    open(row["path"], "w", encoding="utf-8").write(text)
    print("restored", row["slug"], row["region"], row["old"])
