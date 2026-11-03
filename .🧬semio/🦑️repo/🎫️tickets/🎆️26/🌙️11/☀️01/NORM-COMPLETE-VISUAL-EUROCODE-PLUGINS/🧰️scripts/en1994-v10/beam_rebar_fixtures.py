"""🪚 EN 1994 v10: add `beams[].reinforcementProduct` (snapshot rows `"hot_rolled"`, diff patches `null`) to every committed EN 1994 JSON fixture in place (text insertion, formatting preserved). Usage: beam_rebar_fixtures.py [--dry]"""
import os, re, sys

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994"
SKIP = {"target", "node_modules", "🧬️schema", "🔮️oracles", "📦️packages"}
DRY = "--dry" in sys.argv
NUM = r"-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?"
ZONE = re.compile(r'"endZoneLengthM"\s*:\s*(null|%s)' % NUM)


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


def insert_all(text):
    return insert(text, ZONE, lambda t, m: None if re.match(r',\s*"reinforcementProduct"', t[m.end():]) else ("reinforcementProduct", '"hot_rolled"') if enclosing_has_key(t, m.start(), "id") else ("reinforcementProduct", "null"))


count = 0
for dirpath, dirs, files in os.walk(ROOT):
    dirs[:] = [d for d in dirs if d not in SKIP or (d == "🧬️schema" and "🧫️fixtures" in dirpath)]
    for name in files:
        if not name.endswith(".json"):
            continue
        path = os.path.join(dirpath, name)
        text = open(path, encoding="utf-8").read()
        if "endZoneLengthM" not in text or "🧫️fixtures" not in path:
            continue
        new = insert_all(text)
        if new != text:
            count += 1
            if not DRY:
                open(path, "w", encoding="utf-8").write(new)
print("fixtures", count)
