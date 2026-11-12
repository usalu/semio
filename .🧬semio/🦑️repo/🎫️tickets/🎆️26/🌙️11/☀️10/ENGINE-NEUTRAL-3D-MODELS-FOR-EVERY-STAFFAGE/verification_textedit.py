"""Exact-string edits inside the text span of selected catalog objects: object start is found by its id line, its end by brace matching; `fn(text)` returns the new text of the span."""
import json
def span(raw, marker):
    i = raw.index(marker)
    s = raw.rfind("{", 0, i)
    depth, instr, esc = 0, False, False
    for j in range(s, len(raw)):
        c = raw[j]
        if instr:
            if esc: esc = False
            elif c == "\\": esc = True
            elif c == '"': instr = False
        elif c == '"': instr = True
        elif c == "{": depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0: return s, j + 1
    raise ValueError(marker)
def edit_spans(path, edits):
    raw = open(path, encoding="utf-8").read()
    out = raw
    plan = []
    for marker, fn in edits:
        assert raw.count(marker) == 1, (marker, raw.count(marker))
        s, e = span(raw, marker)
        plan.append((s, e, fn(raw[s:e])))
    again = open(path, encoding="utf-8").read()
    assert again == raw, "file changed meanwhile"
    for s, e, t in sorted(plan, reverse=True): out = out[:s] + t + out[e:]
    json.loads(out)
    open(path, "w", encoding="utf-8").write(out)
    return len(plan)
