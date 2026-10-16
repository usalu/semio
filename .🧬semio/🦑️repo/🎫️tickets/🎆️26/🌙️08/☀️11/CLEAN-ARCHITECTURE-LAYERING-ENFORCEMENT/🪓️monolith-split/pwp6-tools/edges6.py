# first-party edges the instance crates need (name evidence) against the declared dependency sets of Appendix H.
import sys; sys.path.insert(0, '.')
from lib6 import *
import orphan6 as O, defscan6 as D
CR = json.load(open(GEN + '/pwp0-crates.json', encoding='utf-8'))
ROOTS = {k: v['root'] for k, v in CR.items() if 'root' in v}
def crate_by_path(rel):
    best = None
    for k, root in ROOTS.items():
        if rel == root + '/🦀️.rs' or rel.startswith(root + '/'):
            if best is None or len(root) > len(ROOTS[best]): best = k
    return best

def index():
    idx = collections.defaultdict(set)
    files = [f.replace(PLUGP, '') for f in plug_files()]
    t = json.load(open(GEN + '/pwp6-touched.json', encoding='utf-8'))
    files = sorted(set(files) | set(t['created']))
    for f in files:
        p = PLUG + f
        if not os.path.exists(p) or '/🧪️tests/' in f or f.startswith(('🧪️tests/', '🧪️testing/')): continue
        c = O.crate_of_file(f) or crate_by_path(f)
        if c is None: continue
        raw = open(p, encoding='utf-8').read(); M = mask(raw)
        for m in re.finditer(r'(?m)^(?:pub(?:\([^)]*\))?\s+)?(?:(?:async|const|unsafe|extern\s+"[^"]*")\s+)*(?:fn|struct|enum|trait|type|static|const|union)\s+(\w+)', M):
            idx[m.group(1)].add(c)
    return idx

if __name__ == '__main__':
    idx = index()
    fam = D.family_files()
    texts = collections.defaultdict(str)
    for f in fam:
        c = O.crate_of_file(f)
        if c: texts[c] += mask(open(PLUG + f, encoding='utf-8').read()) + '\n'
    for c, tx in sorted(texts.items()):
        words = set(re.findall(r'(?<![\w.])([A-Za-z_]\w{3,})\b', tx))
        ev = collections.Counter()
        for w_ in words:
            cs = idx.get(w_)
            if cs and len(cs) == 1:
                d = next(iter(cs))
                if d != c: ev[d] += 1
        declared = set(CR['instance-' + c.split('instance-')[-1]]['deps']) if False else set(CR.get(c.replace('instance-', 'instance-'), {}).get('deps', []))
        decl = set(CR[c]['deps']) if c in CR else set()
        miss = sorted(k for k in ev if k not in decl and k in CR)
        print(c.ljust(20), 'declared', len(decl), 'evidenced', len(ev), 'MISSING', [(m, ev[m]) for m in miss])
