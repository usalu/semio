import json, re
from decimal import Decimal

S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
EXP = re.compile(r"(?<=[ \[])(-?\d+(?:\.\d+)?)e-(\d+)\b(?=[,\n\]])")


def path(cat, aid):
    return f"{S}/{cat}/{aid}/🔣️.json"


def read(cat, aid):
    raw = open(path(cat, aid), "rb").read().decode("utf8")
    return json.loads(raw), raw


def dump(doc, raw):
    line = raw.split("\n")[1]
    text = json.dumps(doc, ensure_ascii=False, indent=len(line) - len(line.lstrip(" ")))
    if not EXP.search(raw):
        text = EXP.sub(lambda m: format(Decimal(m.group(0)), "f"), text)
    return text + "\n"


def write(cat, aid, doc, raw):
    open(path(cat, aid), "wb").write(dump(doc, raw).encode("utf8"))


if __name__ == "__main__":
    import sys

    doc, raw = read(sys.argv[1], sys.argv[2])
    print("roundtrip identical:", dump(doc, raw) == raw)
