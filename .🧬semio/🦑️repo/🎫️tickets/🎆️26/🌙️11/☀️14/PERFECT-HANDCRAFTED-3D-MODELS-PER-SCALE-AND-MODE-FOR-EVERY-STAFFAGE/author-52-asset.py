"""🧵 Author 52: surgical edits of one catalog asset file. patch(slug, fn) parses the asset, lets fn mutate it, and rewrites ONLY the members whose parsed value changed (found by path in the text, re-encoded with indent 2 and the file's indentation); every other byte stays."""
import json, os, copy

CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/%s/🔣️.json"


class Scan:
    def __init__(self, t):
        self.t, self.i = t, 0

    def ws(self):
        while self.t[self.i] in " \n\r\t":
            self.i += 1

    def string(self):
        j = self.i + 1
        while self.t[j] != '"':
            j += 2 if self.t[j] == "\\" else 1
        s = self.t[self.i:j + 1]
        self.i = j + 1
        return json.loads(s)

    def value(self, path, out, cur=()):
        self.ws()
        c = self.t[self.i]
        start = self.i
        if c == "{":
            self.i += 1
            self.ws()
            if self.t[self.i] == "}":
                self.i += 1
            else:
                while True:
                    self.ws()
                    k = self.string()
                    self.ws()
                    self.i += 1
                    ks = self.i
                    self.ws()
                    ind = self.line_indent(self.i)
                    self.value(path, out, cur + (k,))
                    self.ws()
                    if self.t[self.i] == ",":
                        self.i += 1
                        continue
                    self.i += 1
                    break
        elif c == "[":
            self.i += 1
            self.ws()
            if self.t[self.i] == "]":
                self.i += 1
            else:
                n = 0
                while True:
                    self.value(path, out, cur + (n,))
                    n += 1
                    self.ws()
                    if self.t[self.i] == ",":
                        self.i += 1
                        continue
                    self.i += 1
                    break
        elif c == '"':
            self.string()
        else:
            while self.t[self.i] not in ",}] \n\r\t":
                self.i += 1
        if cur in path:
            out[cur] = (start, self.i, self.line_indent(start))

    def line_indent(self, pos):
        s = self.t.rfind("\n", 0, pos) + 1
        n = 0
        while self.t[s + n] == " ":
            n += 1
        return " " * n


def enc(v, ind):
    return json.dumps(v, indent=2, ensure_ascii=False).replace("\n", "\n" + ind)


def diff_paths(a, b, cur=()):
    if isinstance(a, dict) and isinstance(b, dict) and set(a) == set(b):
        r = []
        for k in a:
            r += diff_paths(a[k], b[k], cur + (k,))
        return r
    return [] if a == b else [cur]


def patch(slug, fn, dry=False):
    p = CAT % slug
    text = open(p).read()
    doc = json.loads(text)
    new = copy.deepcopy(doc)
    fn(new["asset"])
    ds = diff_paths(doc, new)
    paths = []
    for d in ds:
        # keep granularity at the first list level below geometry/regions/sections to stay surgical but simple
        paths.append(d)
    ds2 = []
    for d in ds:
        if len(d) >= 2 and d[-2] in ("geometry", "regions", "sections", "supports") and False:
            pass
        ds2.append(d)
    # collapse: re-encode the parent member of the deepest-but-one level so that inserted/removed keys and list-length changes work
    top = set()
    for d in ds:
        dd = d
        while len(dd) > 0:
            node = doc
            ok = True
            for k in dd:
                try:
                    node = node[k]
                except (KeyError, IndexError, TypeError):
                    ok = False
                    break
            if ok:
                break
            dd = dd[:-1]
        # re-encode the nearest ancestor that exists in the old doc and is a dict/list at depth >= 3
        top.add(dd[:max(3, len(dd) - 0)] if False else dd)
    edits = []
    sc = Scan(text)
    want = set(top)
    out = {}
    sc.i = 0
    sc.value(want, out)
    for pth in want:
        st, en, ind = out[pth]
        nv = new
        for k in pth:
            nv = nv[k]
        edits.append((st, en, enc(nv, ind)))
    res = text
    for st, en, s in sorted(edits, reverse=True):
        res = res[:st] + s + res[en:]
    assert json.loads(res) == new, "patched file must equal the mutated asset"
    if dry:
        print("dry", [list(w) for w in want])
        return
    cur = open(p).read()
    assert cur == text, "asset changed meanwhile; rerun"
    open(p, "w").write(res)
    print("patched", slug, [list(w) for w in sorted(want, key=str)])
