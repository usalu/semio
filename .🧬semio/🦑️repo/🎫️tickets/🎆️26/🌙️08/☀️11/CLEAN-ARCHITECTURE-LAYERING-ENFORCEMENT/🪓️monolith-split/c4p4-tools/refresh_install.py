import os, sys, importlib, shutil, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa

tag = sys.argv[1]
do_install = len(sys.argv) > 2 and sys.argv[2] == 'install'
mod = importlib.import_module('specs.' + tag)
sp = mod.SPEC
repo_file = os.path.join(pfa.REPO, sp['file'])
orig_file = os.path.join(pfa.T, 'orig', sp['orig'])
cur = open(repo_file, 'rb').read()
old = open(orig_file, 'rb').read()
if cur != old:
    snap = os.path.join(pfa.T, 'orig', 'history')
    os.makedirs(snap, exist_ok=True)
    shutil.copyfile(orig_file, os.path.join(snap, sp['orig'] + '.' + str(int(os.path.getmtime(orig_file))) + '.prev'))
    open(orig_file, 'wb').write(cur)
    print('orig refreshed for', tag, '(peer edit); previous kept in orig/history')
pfa.stage(tag, mod)
ok = pfa.lossless(tag, mod)
assert ok, 'lossless failed'
assert pfa.paths_check(tag, mod), 'paths'
if do_install:
    cur2 = open(repo_file, 'rb').read()
    assert cur2 == open(orig_file, 'rb').read(), 'repo file changed during staging; rerun'
    pfa.install(tag, mod)
