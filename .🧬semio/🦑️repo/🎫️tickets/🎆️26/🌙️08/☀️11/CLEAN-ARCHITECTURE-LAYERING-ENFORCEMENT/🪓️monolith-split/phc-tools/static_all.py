import os, sys, json, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import state, plan_host, verify_static, hcmod, ws as wsm
from carve import REPO
ws, model, roots = state.load()
files = [f for f in ws.changed() if f.endswith('.rs') and f not in ws.deleted]
bad = verify_static.rustfmt_parse(ws, files)
print('rustfmt parse', len(files), 'files; failures', len(bad), [(os.path.relpath(f, plan_host.H), e[:120]) for f, e in bad][:5])
# the original files that already failed rustfmt before the carve
base_bad = []
for f in files:
    if f in ws.orig:
        p = subprocess.run(['rustfmt', '--edition', '2021', '--emit', 'stdout'], input=ws.orig[f], capture_output=True, text=True)
        if p.returncode != 0:
            base_bad.append(f)
print('of which already failing in the original:', len([f for f, e in bad if f in base_bad]))
print('longest paths', verify_static.longest_paths(ws))
names = sorted({os.path.basename(os.path.dirname(f)) for f in ws.changed() if '/' in f}) 
folders = set()
for f in ws.changed():
    rel = os.path.relpath(f, plan_host.H)
    if rel.startswith('..'):
        continue
    if f in ws.orig:
        continue
    parts = rel.split('/')[:-1]
    for p in parts:
        folders.add(p)
bad_e = [(p, verify_static.emoji_ok(p)) for p in sorted(folders) if verify_static.emoji_ok(p) != 'ok' and not p.isascii()]
print('new folder names', sorted(folders), 'emoji problems', bad_e)
# every mount resolves
hcmod.FS = ws
print('missing mounts', [m.path for m in model.missing])
