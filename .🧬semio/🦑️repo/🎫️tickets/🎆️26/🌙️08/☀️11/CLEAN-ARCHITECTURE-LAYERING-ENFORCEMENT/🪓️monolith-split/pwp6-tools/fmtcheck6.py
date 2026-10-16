import sys, subprocess; sys.path.insert(0, '.')
from common6 import *
t = json.load(open(GEN + '/pwp6-touched.json', encoding='utf-8'))
files = sorted(set(t['created'] + t['modified']) - set(t['deleted']))
files = [f for f in files if f.endswith('.rs')]
bad = []
for f in files:
    p = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--config', 'skip_children=true', '--emit', 'stdout', PLUG + f], capture_output=True, text=True)
    if p.returncode != 0:
        bad.append((f, p.stderr.strip().split('\n')[0][:160]))
print('rustfmt parsed', len(files), 'files; failures', len(bad))
for b in bad: print(b)
