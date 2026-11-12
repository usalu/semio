"""Surgical edits of catalog JSON: replaces only the text span of the selected objects by their re-serialised form (same indentation, same style); the rest of the file is never touched and the file is re-read immediately before the write.
edit(path, select, fn): for every object whose text `select(text)` accepts, `fn(obj)` returns the new object (or None to keep it). Returns the number of replaced objects. Roundtrip of unmodified objects is asserted."""
import json
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
def spans(raw):
    out, stack, instr, esc = [], [], False, False
    for i, c in enumerate(raw):
        if instr:
            if esc: esc = False
            elif c == "\\": esc = True
            elif c == '"': instr = False
        elif c == '"': instr = True
        elif c == "{": stack.append(i)
        elif c == "}":
            s = stack.pop(); out.append((s, i + 1))
    return out
def dump(obj, raw, start, text=None):
    ls = raw.rfind("\n", 0, start) + 1
    base = len(raw[ls:]) - len(raw[ls:].lstrip(" "))
    unit = 2
    if text is not None and "\n" in text:
        second = text.split("\n")[1]
        unit = max(1, len(second) - len(second.lstrip(" ")) - base)
    return json.dumps(obj, indent=unit, ensure_ascii=False).replace("\n", "\n" + " " * base)
def edit(path, select, fn):
    raw = open(path, encoding="utf-8").read()
    chosen = []
    for s, e in sorted(spans(raw), key=lambda t: t[1] - t[0]):
        text = raw[s:e]
        if not select(text) or any(not (e <= a or s >= b) for a, b in [(c[0], c[1]) for c in chosen]): continue
        obj = json.loads(text)
        new = fn(json.loads(text))
        if new is None: continue
        assert dump(obj, raw, s, text) == text, f"roundtrip differs at {s}: {text[:80]!r}"
        chosen.append((s, e, dump(new, raw, s, text)))
    if not chosen: return 0
    again = open(path, encoding="utf-8").read()
    assert again == raw, "file changed meanwhile"
    for s, e, t in sorted(chosen, reverse=True): raw = raw[:s] + t + raw[e:]
    json.loads(raw)
    open(path, "w", encoding="utf-8").write(raw)
    return len(chosen)
