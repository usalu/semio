import importlib, os, sys, filecmp
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p48 as P
tags = sys.argv[1:] or 'sfm sfmunit surface mesh repair editor session algebra'.split()
bad = 0
for t in tags:
    sp = importlib.import_module('specs.' + t).SPEC
    live = os.path.join(P.REPO, os.path.dirname(sp['file']))
    stage = os.path.join(P.SCR, 'stage', t)
    same = filecmp.cmp(os.path.join(live, '🦀️.rs'), os.path.join(stage, '🦀️.rs'), shallow=False)
    for c in sp['children']:
        same = same and filecmp.cmp(os.path.join(live, c['folder'], '🦀️.rs'), os.path.join(stage, c['folder'], '🦀️.rs'), shallow=False)
    sys.argv = ['c4p48.py', 'lossless', t, live]
    print('LIVE', t, 'bytes-equal-to-stage' if same else 'DIFFERS-FROM-STAGE')
    ok = P.lossless(t, importlib.import_module('specs.' + t))
    bad += (not ok) or (not same)
sys.exit(1 if bad else 0)
