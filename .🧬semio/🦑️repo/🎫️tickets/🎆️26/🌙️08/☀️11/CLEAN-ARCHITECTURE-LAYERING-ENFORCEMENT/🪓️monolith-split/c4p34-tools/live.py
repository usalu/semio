import importlib, os, sys, filecmp
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p34 as P
tags = sys.argv[1:] or 'png jpg avi pptx dxfbin dxfschema zlib'.split()
bad = 0
for t in tags:
    sp = importlib.import_module('specs.' + t).SPEC
    live = os.path.dirname(os.path.join(P.REPO, sp['file']))
    stage = os.path.join(P.SCR, 'stage', t)
    same = filecmp.cmp(os.path.join(live, '🦀️.rs'), os.path.join(stage, '🦀️.rs'), shallow=False)
    for f in P.rel_folders(sp):
        same = same and filecmp.cmp(os.path.join(live, f, '🦀️.rs'), os.path.join(stage, f, '🦀️.rs'), shallow=False)
    sys.argv = ['c4p34.py', 'lossless', t, live]
    print('LIVE', t, 'bytes-equal-to-stage' if same else 'DIFFERS-FROM-STAGE')
    ok = P.lossless(t, importlib.import_module('specs.' + t))
    bad += (not ok) or (not same)
sys.exit(1 if bad else 0)
