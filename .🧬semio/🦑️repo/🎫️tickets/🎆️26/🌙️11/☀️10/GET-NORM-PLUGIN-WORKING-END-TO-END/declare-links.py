"""🔗️ Declares the proven links of the insert-row fields as `reference` inputs with their `ref.kind` (compact snapshot schemas: the property object is rewritten in place)."""
import json, sys
A = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/"
S = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json"
EDITS = [
    ("⚖️en1990", "actionId", ["permanent", "variable", "accidental", "seismic"]),
    ("🌍️en1997", "layerId", "layer"), ("🌍️en1997", "pileId", "pile"), ("🌍️en1997", "wallId", "embedded-wall"), ("🌍️en1997", "governingLayerId", "layer"),
    ("🏋️en1991", "roofId", "roof"),
    ("🔩️en1993", "targetId", ["frame-node", "frame-bar"]), ("🔩️en1993", "startNodeId", "frame-node"), ("🔩️en1993", "endNodeId", "frame-node"), ("🔩️en1993", "memberId", "member"),
]
def span(text, start):
    depth, i, instr = 0, start, False
    while True:
        c = text[i]
        if instr:
            if c == "\\": i += 1
            elif c == '"': instr = False
        elif c == '"': instr = True
        elif c == "{": depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0: return i + 1
        i += 1
count = {}
for fam, prop, kind in EDITS:
    path = A + fam + "/" + S
    text = open(path).read()
    key = '"' + prop + '":{'
    pos, out, n = 0, [], 0
    while True:
        i = text.find(key, pos)
        if i < 0: out.append(text[pos:]); break
        j = span(text, i + len(key) - 1)
        obj = json.loads(text[i + len(key) - 1:j])
        out.append(text[pos:i + len(key) - 1])
        if obj.get("type") == "string" and "enum" not in obj:
            ui = obj.setdefault("x-semio-ui", {})
            ui.setdefault("label", {"en": prop, "de": prop})
            ui["widget"] = "reference"; ui["role"] = "value"; ui["ref"] = {"kind": kind}
            n += 1
        out.append(json.dumps(obj, ensure_ascii=False, separators=(",", ":")))
        pos = j
    open(path, "w").write("".join(out))
    count[(fam, prop)] = n
print(count)
