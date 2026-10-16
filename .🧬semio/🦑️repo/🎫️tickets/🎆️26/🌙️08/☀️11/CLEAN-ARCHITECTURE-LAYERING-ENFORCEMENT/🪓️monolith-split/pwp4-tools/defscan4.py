# 🔎️ Definition scan on the applied tree: a private item, field or method of a piece that is named outside the piece's module subtree is a visibility hole. --seed removes one widening and must be caught.
import sys, os, re
from spec4 import *
import widen4
seed = sys.argv[sys.argv.index('--seed') + 1] if '--seed' in sys.argv else None
def read_real(p): return open(p, encoding='utf-8').read() if os.path.exists(p) else None
total = 0; viol = []; members = 0
for src, pieces, rules in CUTS:
    resid_path = os.path.normpath(PLUG + '/' + src)
    texts = {os.path.normpath(PLUG + '/' + p.path): read_real(PLUG + '/' + p.path) for p in pieces if not p.mounted_elsewhere}
    texts = {k: v for k, v in texts.items() if v}
    viamap = {os.path.normpath(PLUG + '/' + p.path): p.unwrap for p in pieces if p.unwrap}
    k = None
    if seed:
        ks = [x for x in texts if seed in x]
        if not ks: ks = []
        k = ks[0] if ks else None
    if seed and k:
        t = texts[k]
        m = re.search(r'^(\s*)pub\(super\) (fn|struct|enum|const|static|type) ', t, re.M)
        texts[k] = t[:m.start()] + m.group(1) + m.group(2) + ' ' + t[m.end():]
        print('seeded: dropped one pub(super) in', os.path.relpath(k, PLUG), '->', t[m.start():m.end()].strip())
    def read(p):
        if p in texts: return texts[p]
        return read_real(p)
    for pp, text in texts.items():
        own = widen4.subtree(pp, read)
        outside = widen4.subtree(resid_path, read, {})
        for qq in texts:
            if qq != pp: outside.update(widen4.subtree(qq, read, {}))
        outside = {k: v for k, v in outside.items() if k not in own}
        for kind, name, li, vis, c in widen4.decl_names(text):
            members += 1
            if vis == '':
                via = viamap.get(pp)
                hit = None if (via and kind != 'item') else widen4.used_in(name, kind, outside, via if kind == 'item' else None)
                if hit: viol.append((os.path.relpath(pp, PLUG).split('/')[-2], kind, name, os.path.relpath(hit, PLUG)[-60:]))
print('members checked', members, 'private-but-used-outside', len(viol))
for v in viol[:40]: print('  ', v)
sys.exit(0 if (bool(viol) == bool(seed)) else 1)
