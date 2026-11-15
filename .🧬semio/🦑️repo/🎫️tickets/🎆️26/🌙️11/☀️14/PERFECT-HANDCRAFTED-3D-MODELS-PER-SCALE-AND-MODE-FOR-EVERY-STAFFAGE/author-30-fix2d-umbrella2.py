import json, re, glob
F = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*umans*/person-with-umbrella/🔣️.json")[0]
text = open(F, encoding="utf-8").read()
rep0 = text.index('"representations"')
for sc, hw in (("200", 0.012), ("500", 0.025)):
    sc_pos = text.index(f'"denominator": {sc}', rep0)
    el = text.index('"elevation"', sc_pos)
    i = text.index('"id": "shaft"', el)
    ls = text.rfind("\n", 0, i) + 1
    indent = re.match(r" *", text[ls:]).group(0)
    start = text.rfind("{", 0, i)
    start_line = text.rfind("\n", 0, start) + 1
    obj_indent = re.match(r" *", text[start_line:]).group(0)
    depth, q = 0, start
    while True:
        ch = text[q]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                break
        q += 1
    if '"id": "ferrule"' in text[start:text.index('"front"', el)]:
        continue
    shape = {"id": "ferrule", "bone": "umbrella", "role": "body", "z": 6.2, "filled": True, "tone": 0.1, "kind": "poly", "points": [[0.78, -hw], [0.78, hw], [0.88, round(0.6 * hw, 4)], [0.88, round(-0.6 * hw, 4)]], "closed": True, "region": "umbrella-frame", "penMm": 0.13}
    block = json.dumps(shape, indent=2, ensure_ascii=False)
    block = "\n".join(obj_indent + line if k else line for k, line in enumerate(block.split("\n")))
    text = text[: q + 1] + ",\n" + obj_indent + block + text[q + 1 :]
    print("inserted at", sc)
json.loads(text)
open(F, "w", encoding="utf-8").write(text)
