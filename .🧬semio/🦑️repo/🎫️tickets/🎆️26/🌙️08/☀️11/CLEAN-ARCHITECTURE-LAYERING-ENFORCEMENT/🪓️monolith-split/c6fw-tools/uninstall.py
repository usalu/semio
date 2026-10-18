"""↩️ Reverts an installed module split of this slice (own files only): the live root and every child must equal the installed stage; then the root is restored from the snapshot and the child folders are removed. usage: uninstall.py <tag>"""
import importlib, os, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import modsplit as P
tag = sys.argv[1]
sp = importlib.import_module('specs.' + tag).SPEC
prev = os.path.join(P.SCR, 'installed', tag)
base = os.path.dirname(os.path.join(P.REPO, sp['file']))
names = []
for dp, dn, fn in os.walk(prev):
    for f in fn:
        names.append(os.path.relpath(os.path.join(dp, f), prev))
for r in names:
    assert open(os.path.join(base, r), 'rb').read() == open(os.path.join(prev, r), 'rb').read(), ('live differs from installed stage', r)
orig = open(os.path.join(P.T, 'orig', sp['orig']), 'rb').read()
for r in names:
    if r == '🦀️.rs':
        continue
    d = os.path.dirname(os.path.join(base, r))
    shutil.rmtree(d)
open(os.path.join(base, '🦀️.rs'), 'wb').write(orig)
shutil.rmtree(prev)
print('uninstalled', tag, len(names) - 1, 'children')
