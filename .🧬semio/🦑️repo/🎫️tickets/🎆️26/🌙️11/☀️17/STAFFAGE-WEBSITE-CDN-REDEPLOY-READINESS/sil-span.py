"""Span-preserving JSON editor for the catalog assets: a parse that records the span of every value, so one value is replaced in place and every other byte (hand formatting, number spelling) stays."""
import json, re, subprocess
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
ROOT = "/Users/ueli/Documents/semio"
WS = " \t\r\n"
class P:
    def __init__(s, t): s.t = t; s.i = 0
    def ws(s):
        while s.t[s.i] in WS: s.i += 1
    def val(s):
        s.ws(); c = s.t[s.i]; st = s.i
        if c == "{":
            s.i += 1; pairs = []; s.ws()
            if s.t[s.i] == "}": s.i += 1; return {"k": "obj", "s": st, "e": s.i, "pairs": pairs}
            while True:
                s.ws(); key = s.str(); s.ws(); s.i += 1
                v = s.val(); pairs.append((key, v)); s.ws()
                if s.t[s.i] == ",": s.i += 1; continue
                s.i += 1; break
            return {"k": "obj", "s": st, "e": s.i, "pairs": pairs}
        if c == "[":
            s.i += 1; items = []; s.ws()
            if s.t[s.i] == "]": s.i += 1; return {"k": "arr", "s": st, "e": s.i, "items": items}
            while True:
                items.append(s.val()); s.ws()
                if s.t[s.i] == ",": s.i += 1; continue
                s.i += 1; break
            return {"k": "arr", "s": st, "e": s.i, "items": items}
        if c == '"': s.str(); return {"k": "val", "s": st, "e": s.i}
        m = re.compile(r"-?[0-9.eE+\-]+|true|false|null").match(s.t, s.i); s.i = m.end(); return {"k": "val", "s": st, "e": s.i}
    def str(s):
        j = s.i + 1
        while s.t[j] != '"': j += 2 if s.t[j] == "\\" else 1
        k = json.loads(s.t[s.i:j + 1]); s.i = j + 1; return k
def parse(t): return P(t).val()
def child(n, key):
    if n["k"] == "obj":
        for k, v in n["pairs"]:
            if k == key: return v
        raise KeyError(key)
    return n["items"][int(key)]
def walk(n, path):
    for p in path: n = child(n, p)
    return n
def solid(node, scale, sid, list_="body"):
    for i, it in enumerate(walk(node, ["asset", "volumes", scale, list_])["items"]):
        for k, v in it["pairs"]:
            if k == "id" and json.loads(node_text(it_text[0], v)) == sid: return it
    raise KeyError(sid)
it_text = [""]
def node_text(t, n): return t[n["s"]:n["e"]]
def path_of(slug): return CAT + slug + "/🔣️.json"
class Doc:
    def __init__(s, slug):
        s.slug = slug; s.path = path_of(slug); s.t = open(s.path, encoding="utf8").read(); s.edits = []; s.root = parse(s.t)
    def at(s, *path): return walk(s.root, list(path))
    def val(s, *path): n = s.at(*path); return json.loads(s.t[n["s"]:n["e"]])
    def find(s, scale, sid, list_="body", section="volumes"):
        for it in walk(s.root, ["asset", section, scale, list_])["items"]:
            for k, v in it["pairs"]:
                if k == "id" and json.loads(s.t[v["s"]:v["e"]]) == sid: return it
        raise KeyError(sid)
    def rep(s, node, text): s.edits.append((node["s"], node["e"], text))
    def setv(s, node, value): s.rep(node, flat(value))
    def commit(s):
        t = s.t
        for a, b, x in sorted(s.edits, reverse=True): t = t[:a] + x + t[b:]
        json.loads(t)
        open(s.path, "w", encoding="utf8").write(t)
def flat(v):
    if isinstance(v, list): return "[" + ", ".join(flat(x) for x in v) + "]"
    if isinstance(v, float): return repr(round(v, 6))
    return json.dumps(v, ensure_ascii=False)
def head_text(slug):
    return subprocess.run(["git", "-C", ROOT, "show", "HEAD:" + path_of(slug)[len(ROOT) + 1:]], capture_output=True, text=True, encoding="utf8").stdout
def restore_volumes(slug):
    """Puts the volumes block of HEAD back in the file (the span of `asset.volumes`), keeping every other byte."""
    d = Doc(slug); h = head_text(slug); hr = parse(h)
    vn = walk(d.root, ["asset", "volumes"]); hn = walk(hr, ["asset", "volumes"])
    new = d.t[:vn["s"]] + h[hn["s"]:hn["e"]] + d.t[vn["e"]:]
    json.loads(new)
    return d.t, new
