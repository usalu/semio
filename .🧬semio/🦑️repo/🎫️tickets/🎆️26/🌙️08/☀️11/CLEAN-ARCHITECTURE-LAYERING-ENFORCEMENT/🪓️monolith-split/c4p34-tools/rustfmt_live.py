import importlib, os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p34 as P
where = sys.argv[1] if len(sys.argv) > 1 else 'live'
n = bad = 0
hunks = 0
for t in 'png jpg avi pptx dxfbin dxfschema zlib'.split():
    sp = importlib.import_module('specs.' + t).SPEC
    base = os.path.dirname(os.path.join(P.REPO, sp['file'])) if where == 'live' else os.path.join(P.SCR, 'stage', t)
    files = [os.path.join(base, '🦀️.rs')] + [os.path.join(base, f, '🦀️.rs') for f in P.rel_folders(sp)]
    for f in files:
        n += 1
        r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--emit', 'stdout', '--config', 'skip_children=true', f], capture_output=True, text=True)
        if r.returncode != 0:
            bad += 1
            print('FAIL', f[-90:], r.stderr[:200])
print('rustfmt parse: files', n, 'parse failures', bad)
sys.exit(1 if bad else 0)
