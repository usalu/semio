"""🛣️ Path scan: every absolute first-party path in a text is checked against the resolver (public / private / missing / unknown)."""
import sys, os, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
from okrs import tokenize
import nsboot, ns
CAT = nsboot.boot()
IDENTS = set()
for name, c in CAT.items():
    IDENTS.add(c['lib_name'] or name.replace('-', '_'))
FIRST = {i for i in IDENTS if i.startswith('semio_framework') or i in ('protocol', 'pack')}
def status(segs):
    """('pub'|'private'|'missing'|'unknown', depth_resolved)"""
    w = ns.ext_world(segs[0])
    if w is None: return 'unknown', 0
    cur = ns.Res('mod', mod=w.root)
    priv = False
    for i, s in enumerate(segs[1:]):
        if cur.kind != 'mod': return ('private' if priv else 'pub'), i + 1
        nxt = ns.lookup(cur.mod, s, pub_only=True)
        if nxt is None:
            nxt2 = ns.lookup(cur.mod, s, pub_only=False)
            if nxt2 is None: return 'missing', i + 1
            priv = True; nxt = nxt2
        if nxt.kind == ns.EXTERN_GLOB:
            subs = []
            for c in nxt.extern:
                subs.append(status(c + list(segs[i + 2:])))
            for st in subs:
                if st[0] in ('pub',): return st
            return subs[0] if subs else ('unknown', i + 1)
        if nxt.kind == 'extern':
            return status(nxt.extern + list(segs[i + 2:]))
        cur = nxt
    return ('private' if priv else 'pub'), len(segs)
def paths_in(text):
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    out = []
    i = 0; n = len(toks)
    while i < n:
        t = toks[i]
        if t[0] == 'id' and t[1] in FIRST and not (i and toks[i - 1][1] == ':' and i > 1 and toks[i - 2][1] == ':' and not (i > 2 and toks[i - 3][0] != 'id' or False)) and not (i and toks[i - 1][1] == '.'):
            segs = [t[1]]; j = i
            while j + 3 < n and toks[j + 1][1] == ':' and toks[j + 2][1] == ':' and toks[j + 3][0] == 'id':
                segs.append(toks[j + 3][1]); j += 3
            if len(segs) > 1: out.append(segs)
            i = j + 1
        else: i += 1
    return out
def scan(texts):
    res = collections.OrderedDict()
    for p, t in texts.items():
        seen = set()
        for segs in paths_in(t):
            key = tuple(segs)
            if key in seen: continue
            seen.add(key)
            # try longest prefix that exists
            st = None
            for take in range(len(segs), 1, -1):
                s2, d = status(segs[:take])
                if s2 in ('pub', 'private'):
                    st = (s2, take); break
                if s2 == 'unknown': st = ('unknown', take); break
            if st is None: res.setdefault(p, []).append(('missing', segs))
            elif st[0] == 'private': res.setdefault(p, []).append(('private', segs[:st[1]]))
            elif st[0] == 'pub' and st[1] < 2 + 0 and len(segs) > 2 and False: pass
    return res
