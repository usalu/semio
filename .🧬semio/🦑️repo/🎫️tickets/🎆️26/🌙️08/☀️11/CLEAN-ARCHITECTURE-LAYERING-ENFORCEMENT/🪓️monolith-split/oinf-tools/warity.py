"""Arity check: every call of a world fn/method passes as many arguments as the definition declares."""
import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wengine import *
from stdnames import STD

def param_count(toks, i):
    po, pc, params = find_params(toks, i)
    n = len(params)
    has_self = False
    if params:
        seg = toks[params[0][0]:params[0][1]]
        if any(t.s == 'self' for t in seg[:4]): has_self = True
    return n, has_self

def scan(files):
    free = collections.defaultdict(set); meth = collections.defaultdict(set); typemeth = collections.defaultdict(lambda: collections.defaultdict(set))
    texts = {}
    for p in files:
        text = open(p, encoding='utf-8').read()
        fns, toks = collect_fns(p, text)
        texts[p] = (text, toks)
        for fn, i, pc in fns:
            n, hs = param_count(toks, i)
            if fn.parent is None: free[fn.name].add(n)
            else:
                meth[fn.name].add(n - (1 if hs else 0))
                typemeth[fn.parent][fn.name].add(n)
                if hs: typemeth[fn.parent][fn.name].add(n - 1)
    return free, meth, typemeth, texts

def count_args(toks, open_idx):
    d = 0; n = 0; seen = False; i = open_idx
    while True:
        t = toks[i]
        if t.k == 'p':
            if t.s in '([{': d += 1
            elif t.s in ')]}':
                d -= 1
                if d == 0: break
            elif t.s == ',' and d == 1: n += 1; seen = False; i += 1; continue
        if d >= 1 and not (i == open_idx): seen = True
        i += 1
    return n + (1 if seen else 0)

def check(files, report_limit=40):
    free, meth, typemeth, texts = scan(files)
    types = set(typemeth)
    bad = []
    for p, (text, toks) in texts.items():
        for i, t in enumerate(toks):
            if t.k != 'id' or i + 1 >= len(toks) or toks[i+1].s != '(': continue
            prev = toks[i-1].s
            if prev == 'fn': continue
            name = t.s
            if prev == '.':
                if name in STD or name not in meth: continue
                a = count_args(toks, i + 1)
                if a not in meth[name]: bad.append((p, text.count('\n', 0, t.a) + 1, name, a, sorted(meth[name])))
            elif prev == ':' and toks[i-2].s == ':' and toks[i-3].k == 'id' and toks[i-3].s in types:
                ty = toks[i-3].s
                if name in typemeth[ty]:
                    a = count_args(toks, i + 1)
                    if a not in typemeth[ty][name]: bad.append((p, text.count('\n', 0, t.a) + 1, ty + '::' + name, a, sorted(typemeth[ty][name])))
            elif prev not in ('.', ':') and name in free:
                a = count_args(toks, i + 1)
                if a not in free[name]: bad.append((p, text.count('\n', 0, t.a) + 1, name, a, sorted(free[name])))
    return bad

if __name__ == '__main__':
    root = sys.argv[1]
    files = []
    for dp, dn, fn in os.walk(root):
        if 'node_modules' in dp: continue
        for f in fn:
            if f.endswith('.rs'): files.append(os.path.join(dp, f))
    bad = check(files)
    print('arity mismatches:', len(bad))
    for p, line, name, a, exp in bad[:60]: print(p[len(root):], line, name, 'args', a, 'expected', exp)

def diff_against(orig_root, new_root):
    def walk(root):
        fs = []
        for dp, dn, fn in os.walk(root):
            if 'node_modules' in dp: continue
            for f in fn:
                if f.endswith('.rs'): fs.append(os.path.join(dp, f))
        return fs
    a = collections.Counter((p[len(orig_root):], n) for p, l, n, x, e in check(walk(orig_root)))
    bad = check(walk(new_root))
    b = collections.Counter((p[len(new_root):], n) for p, l, n, x, e in bad)
    out = []
    for k, v in b.items():
        if v > a.get(k, 0): out.append((k, v, a.get(k, 0)))
    return out, bad
