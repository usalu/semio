"""🪚 Crate-budget F: add `steel.rM`, `steelRM` (diff patch) and `columns[].reinforcementProduct` to every committed EN 1994 JSON fixture in place (text insertion, formatting preserved). Usage: f_en1994_fixtures.py [--dry]"""
import os, re, sys

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994"
SKIP = {"target", "node_modules", "🧬️schema", "🔮️oracles", "📦️packages"}
DRY = "--dry" in sys.argv
NUM = r"-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?"
STEEL = re.compile(r'"aVM2"\s*:\s*(%s)' % NUM)
PATCH_STEEL = re.compile(r'"steelAVM2"\s*:\s*(null|%s)' % NUM)
COVER = re.compile(r'"rebarCoverM"\s*:\s*(null|%s)' % NUM)


def radius(text, pos):
    window = text[max(0, pos - 1500):pos]
    found = list(re.finditer(r'"designation"\s*:\s*"([^"]*)"', window))
    name = found[-1].group(1) if found else ""
    return "0.027" if re.fullmatch(r"HEB\s?\d+", name) else ("0.015" if name == "IPE300" else "0.0")


def enclosing_has_key(text, pos, key):
    depth, i = 0, pos - 1
    while i >= 0:
        c = text[i]
        if c in "}]":
            depth += 1
        elif c in "{[":
            if depth == 0:
                break
            depth -= 1
        i -= 1
    start = i
    depth, j, flat = 0, start + 1, []
    while j < pos:
        c = text[j]
        if c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
        elif depth == 0:
            flat.append(c)
        j += 1
    return re.search(r'"%s"\s*:' % key, "".join(flat)) is not None


def insert(text, regex, make):
    out, last = [], 0
    for m in regex.finditer(text):
        value = make(text, m)
        if value is None:
            continue
        key, val = value
        line_start = text.rfind("\n", 0, m.start()) + 1
        indent = text[line_start:m.start()]
        rest = text[m.end():m.end() + 3]
        pretty = indent.strip() == "" and (rest.startswith("\n") or rest.startswith(",\n"))
        spaced = ": " in text[m.start():m.end()]
        out.append(text[last:m.end()])
        out.append(',\n%s"%s": %s' % (indent, key, val) if pretty else (', "%s": %s' if spaced else ',"%s":%s') % (key, val))
        last = m.end()
    out.append(text[last:])
    return "".join(out)


def process(text):
    new = text
    if "aVM2" in new and '"rM"' not in new:
        new = insert(new, STEEL, lambda t, m: ("rM", radius(t, m.start())))
    if "steelAVM2" in new and '"steelRM"' not in new:
        new = insert(new, PATCH_STEEL, lambda t, m: ("steelRM", "null"))
    if "rebarCoverM" in new and '"reinforcementProduct"' not in new:
        new = insert(new, COVER, lambda t, m: ("reinforcementProduct", '"hot_rolled"') if enclosing_has_key(t, m.start(), "id") else ("reinforcementProduct", "null"))
    return new


count = 0
for dirpath, dirs, files in os.walk(ROOT):
    dirs[:] = [d for d in dirs if d not in SKIP or (d == "🧬️schema" and "🧫️fixtures" in dirpath)]
    for name in files:
        if not name.endswith(".json"):
            continue
        path = os.path.join(dirpath, name)
        text = open(path, encoding="utf-8").read()
        if "aVM2" not in text and "rebarCoverM" not in text and "steelAVM2" not in text:
            continue
        new = process(text)
        if new != text:
            count += 1
            if not DRY:
                open(path, "w", encoding="utf-8").write(new)
print("fixtures", count)
