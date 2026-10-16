# 🪞 Surface parity of the store root: every public top-level name of the pre-A1 root (items and `pub use` leaves) must still resolve in the live root namespace (items, re-exports through the carved crates). usage: surface56.py [--seed name]
import sys, os, re, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools'); sys.path.insert(0, HERE)
import ns, registry, a2lib, em
from okrs import items, tokenize
import okusetree as U

def public_names(text):
    out = {}
    for it in items(text):
        k = it['kind']
        if k in ('fn', 'struct', 'enum', 'trait', 'const', 'static', 'type', 'union', 'mod') and it['vis'] == 'pub' and it['name']:
            out[it['name']] = k
        elif k == 'macro_rules' and any('macro_export' in a for a in it['attrs']) and it['name']:
            out[it['name']] = 'macro'
        elif k == 'use' and it['vis'] == 'pub':
            body = re.search(r'\buse\b(.*);', ''.join(t[1] for t in tokenize(it['text']) if t[0] != 'comment'), re.S)
            if not body: continue
            for path, alias, glob in U.parse(body.group(1)):
                if glob: continue
                out[U.local_name((path, alias, glob))] = 'use'
    return out

def main():
    before = public_names(open(HERE + '/pre-a1/root-pre-a1.rs', encoding='utf8').read())
    a2lib.register_all()
    w = ns.world(); root = w.files[em.ROOT]
    missing = []
    for name, kind in sorted(before.items()):
        r = ns.lookup(root, name)
        if r is None and kind != 'macro': missing.append((name, kind))
        elif r is None: pass
    print('public top-level names before:', len(before), '| unresolved now:', len(missing))
    for m in missing[:80]: print('  MISSING', m)
    return missing
if __name__ == '__main__': main()
