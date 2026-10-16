"""🕸️ Reference graph of the top-level items of a Rust test file (token level). Impls attach to their self type."""
import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
from okrs import items, tokenize
IMPL_HEAD = re.compile(r'^\s*(?:unsafe\s+)?impl\b')
def split_generics(toks, i):
    d = 0
    while i < len(toks):
        if toks[i][1] == '<': d += 1
        elif toks[i][1] == '>':
            d -= 1
            if d == 0: return i + 1
        i += 1
    return i
def impl_parts(it_text):
    toks = [t for t in tokenize(it_text) if t[0] not in ('ws', 'comment')]
    # find 'impl'
    i = 0
    while i < len(toks) and toks[i][1] != 'impl': i += 1
    i += 1
    if i < len(toks) and toks[i][1] == '<': i = split_generics(toks, i)
    head = []
    d = 0
    while i < len(toks):
        t = toks[i][1]
        if t == '<': d += 1
        elif t == '>': d -= 1
        elif d == 0 and (t == '{' or t == 'where'): break
        head.append(toks[i])
        i += 1
    # split at top-level 'for'
    d = 0; fi = None
    for k, t in enumerate(head):
        if t[1] == '<': d += 1
        elif t[1] == '>': d -= 1
        elif d == 0 and t[1] == 'for': fi = k
    def lead(ts):
        ids = [x[1] for x in ts if x[0] == 'id' and x[1] not in ('dyn', 'mut', 'const')]
        # last path segment before '<'
        out = None
        d = 0
        for x in ts:
            if x[1] == '<': d += 1
            elif x[1] == '>': d -= 1
            elif d == 0 and x[0] == 'id' and x[1] not in ('dyn', 'mut', 'const'): out = x[1]
        return out
    if fi is None: return None, lead(head)
    return lead(head[:fi]), lead(head[fi + 1:])
def parse(path):
    src = open(path, encoding='utf8').read()
    its = items(src)
    out = []
    for idx, it in enumerate(its):
        a = src.count('\n', 0, it['start']) + 1
        b = src.count('\n', 0, it['end']) + 1
        toks = [t for t in tokenize(it['text']) if t[0] == 'id']
        ids = {t[1] for t in toks}
        rec = dict(idx=idx, kind=it['kind'], name=it['name'], vis=it['vis'], a=a, b=b, text=it['text'], ids=ids, attrs=it['attrs'], bytes=len(it['text'].encode()))
        if it['kind'] == 'impl':
            tr, st = impl_parts(it['text'])
            rec['trait'] = tr; rec['selfty'] = st
        rec['test'] = it['kind'] == 'fn' and any(re.search(r'#\s*\[\s*(?:[A-Za-z_0-9]+::)*(?:test|async_test)\b', x) for x in it['attrs'])
        out.append(rec)
    return src, out
if __name__ == '__main__':
    src, its = parse(sys.argv[1])
    for r in its[:40]: print(r['a'], r['kind'], r['name'], r.get('trait'), r.get('selfty'))
