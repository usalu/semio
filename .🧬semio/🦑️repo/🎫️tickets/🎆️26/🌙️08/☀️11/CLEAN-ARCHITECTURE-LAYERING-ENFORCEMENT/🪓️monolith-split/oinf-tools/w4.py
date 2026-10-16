import sys, re, collections, os
sys.path.insert(0, '.')
from oinf_rs import *
INF = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/"
files = []
for dp, dn, fn in os.walk(INF + '🌍️world'):
    for f in fn:
        if f.endswith('.rs'): files.append(os.path.join(dp, f))
st_methods = set()
forms = collections.Counter(); ex = {}
for p in files:
    text = open(p, encoding='utf-8').read()
    toks = [t for t in lex(text) if t.k != 'comment']
    for i, t in enumerate(toks):
        if t.k == 'id' and t.s == 'state':
            prev = toks[i-1].s if i else ''
            prev2 = toks[i-2].s if i > 1 else ''
            nxt = toks[i+1].s if i+1 < len(toks) else ''
            nxt2 = toks[i+2].s if i+2 < len(toks) else ''
            nxt3 = toks[i+3].s if i+3 < len(toks) else ''
            if nxt == '.':
                form = 'state.' + ('call' if nxt3 == '(' else 'field')
            else:
                form = f'{prev2} {prev} STATE {nxt}'
            if prev == ':' : form = 'param-decl'
            forms[form] += 1
            ex.setdefault(form, (p[len(INF):], text.count('\n', 0, t.a)+1))
for f, c in forms.most_common(60): print(c, repr(f), ex[f])
