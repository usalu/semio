"""🏖️ c4p43 census sandbox: copies the three module dirs that hold the six crates into /Users/ueli/.cache/c4p43/cen/<label> and, for `after`, overlays the staged files. Writes only below the sandbox root (asserted). usage: sandbox.py before|after"""
import os, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import oracle as O
label = sys.argv[1]
root = os.path.join(O.SCR, 'cen', label)
real = os.path.realpath(os.path.join(O.SCR, 'cen'))
assert os.path.realpath(root).startswith(real + os.sep) or os.path.realpath(root) == real
shutil.rmtree(root, ignore_errors=True)
os.makedirs(root)
for d in [O.O + '/💡️inference', O.O + '/📇️directory', O.SPR, O.O + '/🎒️pack/🚦️control/🧪️tests/🧫️fixtures', '🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures', O.O + '/🧑‍💻dev/🔖️channel-version']:
    shutil.copytree(os.path.join(O.REPO, d), os.path.join(root, d), symlinks=True, ignore=O.IGNORE)
stage_root = os.path.join(O.SCR, 'stage')
tags = ['inf', 'dir', 'hcd', 'plw', 'ccd', 'cdc', 'cut']
for t in tags:
    sp = O.spec_of(t)
    td = os.path.join(root, os.path.dirname(sp['file']))
    assert os.path.realpath(td).startswith(os.path.realpath(root) + os.sep)
    if label == 'before':
        shutil.copyfile(os.path.join(O.T, 'orig', sp['orig']), os.path.join(root, sp['file']))
        for f in O.folders_of(sp):
            shutil.rmtree(os.path.join(td, f), ignore_errors=True)
    else:
        shutil.copyfile(O.stage_root_file(t, sp, stage_root), os.path.join(root, sp['file']))
        sd = O.stage_dir_of(t, sp, stage_root)
        for f in O.folders_of(sp):
            shutil.rmtree(os.path.join(td, f), ignore_errors=True)
            os.makedirs(os.path.join(td, f))
            shutil.copyfile(os.path.join(sd, f, '🦀️.rs'), os.path.join(td, f, '🦀️.rs'))
print('sandbox', root)
