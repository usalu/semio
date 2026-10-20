"""Helpers for the group generators: slice exact old text from the live tree between markers and emit a groups/<name>.py with literal anchors."""
import os, pprint
LIVE = "/Users/ueli/Documents/semio"
def live(path):
    with open(os.path.join(LIVE, path), encoding="utf-8") as handle:
        return handle.read()
def text_span(text, start, end, include_end=False):
    a = text.index(start)
    if text.count(start) != 1:
        raise SystemExit('start not unique: ' + start[:60])
    b = text.index(end, a)
    return text[a:b + (len(end) if include_end else 0)]
def read_abs(path):
    with open(path, encoding='utf-8') as handle:
        return handle.read()
def span(path, start, end, include_end=False):
    text = live(path)
    a = text.index(start)
    if text.count(start) != 1:
        raise SystemExit("start not unique: " + start[:60])
    b = text.index(end, a)
    return text[a:b + (len(end) if include_end else 0)]
def emit(name, title, requires, edits, new, folder):
    lines = ["NAME = %r" % name, "TITLE = %r" % title, "REQUIRES = %s" % pprint.pformat(requires, width=200), "EDITS = ["]
    for item in edits:
        lines.append("    (" + ", ".join(repr(part) for part in item) + "),")
    lines.append("]")
    lines.append("NEW = " + pprint.pformat(new, width=200))
    with open(os.path.join(folder, name + ".py"), "w", encoding="utf-8") as handle:
        handle.write("\n".join(lines) + "\n")
