"""🎨 c4p45 rustfmt parse check over roots and children (stage or live). usage: fmt.py [live|stage] [tag...]"""
import importlib, os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p45 as P
mode = sys.argv[1] if len(sys.argv) > 1 else 'live'
tags = sys.argv[2:] or 'pix gdt rnd dfl drv sch pnt'.split()
n = bad = 0
for t in tags:
    sp = importlib.import_module('specs.' + t).SPEC
    base = os.path.join(P.REPO, os.path.dirname(sp['file'])) if mode == 'live' else os.path.join(P.SCR, 'stage', t)
    files = [os.path.join(base, '🦀️.rs')] + [os.path.join(base, f, '🦀️.rs') for f in P.rel_folders(sp)]
    for f in files:
        n += 1
        if os.path.basename(os.path.dirname(f)) and any(x in f for x in ()):
            pass
        p = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--config', 'skip_children=true', '--emit', 'stdout', f], capture_output=True, text=True)
        if p.returncode != 0 or 'error' in p.stderr.lower():
            bad += 1
            print('PARSE FAIL', t, os.path.relpath(f, base), p.stderr.strip().split('\n')[0][:200])
print('rustfmt parsed', n, 'files, failures', bad)
sys.exit(1 if bad else 0)
