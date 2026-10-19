"""📚 c7w literal-list expander: rsexpand2.py <file> — every quoted path literal of the file that ends with the repo path of a split root gets the child files appended right after it (same prefix, same list)."""
import json, os, sys, importlib, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import modsplit as P
tags = json.load(open(os.path.dirname(os.path.abspath(__file__)) + '/tags.json', encoding='utf-8'))
def children(tag):
    sp = importlib.import_module('specs.' + tag).SPEC
    return [f + '/🦀️.rs' for f in P.rel_folders(sp)]
roots = {}
for tag, p in tags.items():
    if tag == 'bimvalues' or not p.endswith('.rs'):
        continue
    try:
        roots[p] = (os.path.dirname(p), children(tag))
    except ModuleNotFoundError:
        pass
f = sys.argv[1]
t = open(f, encoding='utf-8').read()
n = 0
def sub(m):
    global n
    lit = m.group(1)
    for full, (d, kids) in roots.items():
        for k in range(len(full.split('/')) - 1, 2, -1):
            tail = '/'.join(full.split('/')[-k:])
            if lit.endswith('/' + tail) or lit == tail:
                prefix = lit[:len(lit) - len(tail)]
                dirtail = '/'.join(d.split('/')[-(k - 1):])
                extra = ''.join(f', "{prefix}{dirtail}/{c}"' for c in kids)
                n += len(kids)
                return m.group(0) + extra
        pass
    return m.group(0)
t2 = re.sub(r'"([^"\n]*?🦀️\.rs)"(?=[,\]\)])', sub, t)
open(f, 'w', encoding='utf-8').write(t2)
print('added', n)
