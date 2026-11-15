import json, re
ROOT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/%s/🔣️.json"
def scalar(v): return not isinstance(v, (dict, list))
def inline(v):
    if isinstance(v, list):
        return all(scalar(i) for i in v) or (all(isinstance(i, list) and all(scalar(l) for l in i) for i in v) and len(json.dumps(v)) <= 160)
    return False
def dump(v, depth):
    return json.dumps(v, indent=2, ensure_ascii=False).replace("\n", "\n" + "  " * depth)
def dump_old(v, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(v, dict):
        if not v: return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + dump(i, depth + 1) for k, i in v.items()) + "\n" + pad + "}"
    if isinstance(v, list):
        if not v: return "[]"
        if inline(v): return json.dumps(v, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(i, depth + 1) for i in v) + "\n" + pad + "]"
    return json.dumps(v, ensure_ascii=False)
def member_end(text, start):
    depth, index, quoted = 0, start, False
    while index < len(text):
        ch = text[index]
        if quoted:
            if ch == "\\": index += 1
            elif ch == '"': quoted = False
        elif ch == '"': quoted = True
        elif ch in "{[": depth += 1
        elif ch in "}]":
            depth -= 1
            if depth == 0: return index + 1
        index += 1
    raise SystemExit("unbalanced")
def edit_member(slug, key, fn, dry=False):
    path = ROOT % slug
    text = open(path, encoding="utf-8").read()
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    f = re.search(r'\n' + indent + r'"' + key + r'"\s*:\s*', text)
    s, e = f.start() + 1, member_end(text, f.end())
    old = json.loads(text[f.end():e])
    assert dump(old, len(indent) // 2) == text[f.end():e], "round trip differs for " + key
    new = fn(json.loads(json.dumps(old)))
    result = text[:f.end()] + dump(new, len(indent) // 2) + text[e:]
    json.loads(result)
    if not dry: open(path, "w", encoding="utf-8").write(result)
    return old, new
