"""🔎 C6PLUG2 owner finder: who mounts a file (#[path] / include!) up to the crate lib root.
usage: owner.py <plugin-dir-or-tree> <file>..."""
import os, re, sys
REPO = '/Users/ueli/Documents/semio'
PATH = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INC = re.compile(r'include!\(\s*"([^"]+)"\s*\)')
def index(tree):
    m = {}
    for dp, dn, fn in os.walk(os.path.join(REPO, tree)):
        dn[:] = [d for d in dn if d not in ('node_modules', 'target', '.git')]
        for f in fn:
            if not f.endswith('.rs'):
                continue
            p = os.path.join(dp, f)
            try:
                t = open(p, encoding='utf-8').read()
            except Exception:
                continue
            for rx, kind in ((PATH, 'path'), (INC, 'include')):
                for mm in rx.finditer(t):
                    tgt = os.path.normpath(os.path.join(os.path.dirname(p), mm.group(1)))
                    m.setdefault(tgt, []).append((p, kind))
    return m
if __name__ == '__main__':
    tree = sys.argv[1]
    idx = index(tree)
    for f in sys.argv[2:]:
        cur = os.path.join(REPO, f)
        chain = []
        seen = set()
        while cur in idx and cur not in seen:
            seen.add(cur)
            par = idx[cur]
            chain.append([(os.path.relpath(a, REPO)[-110:], k) for a, k in par])
            cur = par[0][0]
        print(f[-80:])
        for c in chain:
            print('   <-', c)
        d = os.path.dirname(cur)
        print('   root', os.path.relpath(cur, REPO)[-110:], 'cargo' if os.path.exists(os.path.join(d, 'Cargo.toml')) else 'no-cargo')
