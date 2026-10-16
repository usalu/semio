"""use-declaration flattening."""
from oinf_rs import *

def parse_use_item(text):
    """text of a `use` item (with attrs). Returns (attrs, vis, entries) entries: list of dict(path=[...], local=name|None, glob=bool)"""
    toks = [t for t in lex(text) if t.k != 'comment']
    i = 0; attrs = []
    while toks[i].s == '#':
        d = 0; j = i + 1
        while True:
            if toks[j].s == '[': d += 1
            elif toks[j].s == ']':
                d -= 1
                if d == 0: break
            j += 1
        attrs.append(text[toks[i].a:toks[j].b]); i = j + 1
    vis = ''
    if toks[i].s == 'pub':
        a = toks[i].a; i += 1
        if toks[i].s == '(':
            d = 0
            while True:
                if toks[i].s == '(': d += 1
                elif toks[i].s == ')':
                    d -= 1
                    if d == 0: break
                i += 1
            i += 1
        vis = text[a:toks[i-1].b]
    assert toks[i].s == 'use'
    i += 1
    entries = []
    def tree(i, prefix):
        # returns index after tree
        path = list(prefix)
        while True:
            t = toks[i]
            if t.s == '{':
                i += 1
                while True:
                    i = tree(i, path)
                    if toks[i].s == ',': i += 1
                    if toks[i].s == '}': return i + 1
            elif t.s == '*':
                entries.append(dict(path=path, local=None, glob=True)); return i + 1
            elif t.k == 'id':
                if toks[i+1].s == ':' and toks[i+2].s == ':':
                    path.append(t.s); i += 3; continue
                # leaf
                local = t.s
                if toks[i+1].s == 'as':
                    local = toks[i+2].s; i += 3
                else: i += 1
                if t.s == 'self':
                    entries.append(dict(path=path, local=(local if local != 'self' else path[-1]), glob=False, self=True))
                else:
                    entries.append(dict(path=path + [t.s], local=local, glob=False))
                return i
            elif t.s == ':' and toks[i+1].s == ':':
                i += 2; continue
            else:
                raise SyntaxError(text)
    tree(i, [])
    return attrs, vis, entries
