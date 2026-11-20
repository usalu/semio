"""🧾 The catalog layout of the sliding door: finds a top-level asset member in the file text and dumps a value in that member's layout (two-space JSON, decimals without exponents)."""
import json
import re

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/sliding-door/🔣️.json"


def member(source, key):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', source)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = source[index]
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
                return found.end(), index + 1
        index += 1


def decimal(match):
    text = match.group(0)
    value = float(text)
    out = ("%.12f" % value).rstrip("0").rstrip(".")
    return out if float(out) == value else repr(value)


def dumped(value):
    text = json.dumps(value, indent=2, ensure_ascii=False)
    text = re.sub(r"(?<![\w.\"])-?\d+(?:\.\d+)?e[-+]?\d+(?![\w\"])", decimal, text)
    lines = text.split("\n")
    return "\n".join(line if i == 0 else "    " + line for i, line in enumerate(lines))


if __name__ == "__main__":
    source = open(CATALOG, encoding="utf-8").read()
    asset = json.loads(source)["asset"]
    for key in asset:
        start, end = member(source, key)
        print(key, dumped(asset[key]) == source[start:end])
