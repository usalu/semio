import os, sys, importlib, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa
root_mode = sys.argv[1]  # stage | live
bad = 0
n = 0
for tag in sys.argv[2:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    if root_mode == 'stage':
        base = os.path.join(pfa.SCR, 'stage', tag)
    else:
        base = os.path.join(pfa.REPO, os.path.dirname(sp['file']))
    files = [os.path.join(base, '🦀️.rs')] if root_mode == 'stage' else [os.path.join(pfa.REPO, sp['file'])]
    files += [os.path.join(base, c['folder'], '🦀️.rs') for c in sp['children']]
    for f in files:
        r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--config', 'skip_children=true', '--emit', 'stdout', f], capture_output=True, text=True)
        n += 1
        if r.returncode != 0:
            bad += 1
            print('FMT FAIL', tag, os.path.relpath(f, base)[-60:], r.stderr[:300])
print('rustfmt parsed', n, 'files, failures', bad)
sys.exit(1 if bad else 0)
