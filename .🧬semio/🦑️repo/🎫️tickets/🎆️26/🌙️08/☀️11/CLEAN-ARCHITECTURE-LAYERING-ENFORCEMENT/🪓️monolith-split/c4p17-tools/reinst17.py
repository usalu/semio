#!/usr/bin/env python3
"""🔁 c4p17 re-installer for a tag whose first install is still byte-equal to its previous stage (`prev-<tag>`): removes only the old child folders (each must hold exactly the previous stage file), installs the new stage.
usage: reinst17.py <tag>"""
import os, sys, shutil, hashlib
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p17 as P, mkspec as M
tag = sys.argv[1]
prev = os.path.join(P.SCR, 'stage', 'prev-' + tag)
new = os.path.join(P.SCR, 'stage', tag)
target = os.path.join(P.REPO, M.FILES[tag])
base = os.path.dirname(target)
sha = lambda p: hashlib.sha256(open(p, 'rb').read()).hexdigest()
old_kids = sorted(d for d in os.listdir(prev) if os.path.isdir(os.path.join(prev, d)))
assert sha(target) == sha(os.path.join(prev, '🦀️.rs')), 'live parent differs from the previous stage'
for k in old_kids:
    live = os.path.join(base, k)
    assert os.listdir(live) == ['🦀️.rs'], ('unexpected content', live)
    assert sha(os.path.join(live, '🦀️.rs')) == sha(os.path.join(prev, k, '🦀️.rs')), ('live child differs from the previous stage', k)
for k in old_kids:
    shutil.rmtree(os.path.join(base, k))
new_kids = sorted(d for d in os.listdir(new) if os.path.isdir(os.path.join(new, d)))
for k in new_kids:
    assert not os.path.exists(os.path.join(base, k)), k
    os.makedirs(os.path.join(base, k))
    shutil.copyfile(os.path.join(new, k, '🦀️.rs'), os.path.join(base, k, '🦀️.rs'))
tmp = target + '.c4p17-tmp'
shutil.copyfile(os.path.join(new, '🦀️.rs'), tmp)
os.replace(tmp, target)
print('reinstalled', tag, 'removed', len(old_kids), 'installed', len(new_kids))
