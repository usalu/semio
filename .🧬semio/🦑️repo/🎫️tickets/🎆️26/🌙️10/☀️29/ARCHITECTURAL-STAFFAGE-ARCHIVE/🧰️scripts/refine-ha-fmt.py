import json
def expanded(v):
    if isinstance(v, list): return any(isinstance(x, dict) for x in v)
    if isinstance(v, dict): return any(expanded(x) for x in v.values())
    return False
def inline(v):
    return json.dumps(v, ensure_ascii=False, separators=(", ", ": ")).replace("{", "{ ").replace("}", " }") if isinstance(v, dict) and v else json.dumps(v, ensure_ascii=False, separators=(", ", ": "))
def _inline(v):
    if isinstance(v, dict):
        return "{}" if not v else "{ " + ", ".join(json.dumps(k, ensure_ascii=False) + ": " + _inline(x) for k, x in v.items()) + " }"
    if isinstance(v, list):
        return "[" + ", ".join(_inline(x) for x in v) + "]"
    return json.dumps(v, ensure_ascii=False)
def dump(v, ind=0):
    pad = "  " * (ind + 1)
    if not expanded(v): return _inline(v)
    if isinstance(v, list):
        return "[\n" + ",\n".join(pad + dump(x, ind + 1) for x in v) + "\n" + "  " * ind + "]"
    return "{\n" + ",\n".join(pad + json.dumps(k, ensure_ascii=False) + ": " + dump(x, ind + 1) for k, x in v.items()) + "\n" + "  " * ind + "}"
if __name__ == "__main__":
    import sys, difflib
    t = open(sys.argv[1]).read()
    s = dump(json.loads(t)) + "\n"
    print(s == t)
    n = 0
    for l in difflib.unified_diff(t.splitlines(), s.splitlines(), lineterm="", n=0):
        print(l[:200]); n += 1
        if n > 40: break
