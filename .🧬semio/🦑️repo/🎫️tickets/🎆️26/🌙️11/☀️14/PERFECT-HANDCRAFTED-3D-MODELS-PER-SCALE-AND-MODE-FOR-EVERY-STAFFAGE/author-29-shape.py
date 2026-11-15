"""Textual patch of 2D shape members (leaves every other byte of the asset file alone), shared by author 29.
Usage: python3 author-29-shape.py <asset 🔣️.json> <patch.json> [--apply]
patch.json = [ { "scale": "200", "view": "elevation", "id": "near-foot", "set": { "penMm": 0.15, "points": [[..]] } }, ... ]
Keys that do not exist yet are appended after `role`. Values are written in the same expanded style as the rest of the file."""
import json, re, sys

def member_end(text, start):
    depth, index, quoted = 0, start, False
    while index < len(text):
        char = text[index]
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
                return index + 1
        index += 1
    raise SystemExit("unbalanced")

def scalar(v): return not isinstance(v, (dict, list))

def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value: return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + dump(v, depth + 1) for k, v in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value: return "[]"
        return "[\n" + ",\n".join(inner + dump(v, depth + 1) for v in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)

def locate(text, scale, view, sid):
    m = re.search(r'\n(\s*)"representations"\s*:\s*', text)
    rs, re_ = m.start() + 1, member_end(text, m.end())
    m2 = re.compile(r'\n\s*"%s"\s*:\s*\{' % scale).search(text, rs, re_)
    s0 = m2.start() + 1
    s1 = member_end(text, m2.end() - 1)
    m3 = re.compile(r'\n\s*"geometry"\s*:\s*').search(text, s0, s1)
    g0, g1 = m3.start() + 1, member_end(text, m3.end())
    m4 = re.compile(r'\n\s*"%s"\s*:\s*' % view).search(text, g0, g1)
    v0, v1 = m4.start() + 1, member_end(text, m4.end())
    pos = text.find('"id": "%s"' % sid, v0, v1)
    if pos < 0: raise SystemExit("not found %s %s %s" % (scale, view, sid))
    start = text.rfind("{", v0, pos)
    return start, member_end(text, start)

def main():
    f, pf = sys.argv[1], sys.argv[2]
    patches = json.load(open(pf, encoding="utf-8"))
    text = open(f, encoding="utf-8").read()
    for p in patches:
        if "clone" in p:
            start, end = locate(text, p["scale"], p["view"], p["clone"])
            obj = text[start:end]
            ls = text.rfind("\n", 0, start) + 1
            indent = text[ls:start]
            text = text[:end] + ",\n" + indent + obj + text[end:]
            ns, ne = locate(text, p["scale"], p["view"], p["clone"])
            ns2 = text.find(obj, ne)
            clone_obj = text[ns2:ns2 + len(obj)]
            text = text[:ns2] + clone_obj.replace('"id": "%s"' % p["clone"], '"id": "%s"' % p["id"], 1) + text[ns2 + len(obj):]
        for k, val in p["set"].items():
            start, end = locate(text, p["scale"], p["view"], p["id"])
            obj = text[start:end]
            km = re.search(r'\n(\s*)"%s"\s*:\s*' % re.escape(k), obj)
            if km:
                ind = len(km.group(1)) // 2
                vs = km.end()
                ve = member_end(obj, vs) if obj[vs] in "[{" else re.compile(r'[,\n]').search(obj, vs).start()
                obj = obj[:vs] + dump(val, ind) + obj[ve:]
            else:
                rm = re.search(r'\n(\s*)"role"\s*:\s*[^\n]*\n', obj)
                ind = len(rm.group(1)) // 2
                ins = rm.end()
                line = "  " * ind + json.dumps(k) + ": " + dump(val, ind)
                before = obj[:rm.end() - 1]
                if not before.rstrip().endswith(","): before = before + ","
                obj = before + "\n" + line + "," + obj[rm.end() - 1:]
            text = text[:start] + obj + text[end:]
    json.loads(text)
    if "--apply" in sys.argv:
        open(f, "w", encoding="utf-8").write(text)
        print("patched", len(patches), "shapes")
    else:
        print("dry run ok", len(patches))

main()
