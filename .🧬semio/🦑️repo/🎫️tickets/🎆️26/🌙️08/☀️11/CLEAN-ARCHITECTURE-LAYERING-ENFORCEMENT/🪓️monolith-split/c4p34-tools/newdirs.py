import importlib, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p34 as P
out = []
for t in 'png jpg avi pptx dxfbin dxfschema zlib'.split():
    sp = importlib.import_module('specs.' + t).SPEC
    base = os.path.dirname(sp['file'])
    for f in P.rel_folders(sp):
        out.append(base + '/' + f)
print('\n'.join(out))
