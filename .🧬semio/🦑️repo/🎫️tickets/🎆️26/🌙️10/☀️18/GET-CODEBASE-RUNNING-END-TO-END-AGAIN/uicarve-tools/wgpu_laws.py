"""🥣️ uicarve wgpu laws: the orphaned root tests of the old wgpu root become the cross-crate laws crate."""
import json, os, re, sys
sys.path.insert(0, 'lib')
import carve, rs, peel
from carve import R, UI, rd, wr
W = UI + '/🎯️targets/🧊️wgpu'
spec = carve.Spec('specs/wgpu.json')


def rel(frm_file, to_file):
    return os.path.relpath(to_file, os.path.dirname(frm_file))


c = spec.crates['laws']
root = spec.root_file(c)
old = rd('lists/wgpu-old-root.rs.txt')
want = ['text_renderer_tests', 'theme_token_tests', 'document_tree_tests', 'conformance_corpus_tests', 'prepared_engine_tests']
mounts_ = []
for m in carve.MOUNT.finditer(old):
    if m.group(3) in want:
        pm = re.search(r'#\[path\s*=\s*"([^"]+)"\]', m.group(1) or '')
        mounts_.append((m.group(3), os.path.normpath(os.path.join(W, pm.group(1)))))
assert len(mounts_) == len(want), mounts_
rootnames = {}
for _, r_ in spec.rules.rules:
    fr = r_['from'].split('::')
    if len(fr) == 2 and fr[0] == 'wgpu':
        rootnames[fr[1]] = r_
needed = {}
for n, p in mounts_:
    t = rd(p)
    if not re.search(r'(?m)^use super::\*;', t):
        continue
    ids = peel.idents_in(t)
    if n == 'theme_token_tests':
        ids |= peel.idents_in(rd(os.path.join(os.path.dirname(p), '../🔬️targets-wgpu-theme-engine-unit/🦀️.rs')))
    for nm in ids:
        if nm in rootnames and nm not in needed:
            needed[nm] = rootnames[nm]
lines = []
for nm, r_ in sorted(needed.items()):
    segs = spec.render('laws', r_, [x for x in r_['to'].split('::') if x])
    path = '::'.join(segs)
    lines.append('use %s;' % path if segs[-1] == nm else 'use %s as %s;' % (path, nm))
txt = '//! 🥣️ ' + c['desc'] + '.\n\n#![cfg(test)]\n\n' + '\n'.join(lines) + '\n\n'
for n, p in mounts_:
    txt += '#[path = "%s"]\nmod %s;\n\n' % (rel(root, p), n)
wr(root, txt)
cnt = 0
for f in carve.reachable(root):
    if f == root or not f.endswith('.rs'):
        continue
    old_t = rd(f)
    new_t = carve.rewrite_text(old_t, spec, 'laws', {})
    new_t = new_t.replace('#[cfg(feature = "wgpu-engine")]\n', '')
    if new_t != old_t:
        wr(f, new_t)
        cnt += 1
print('laws mounts', [n for n, _ in mounts_], 'root names', sorted(needed), 'files', cnt)
