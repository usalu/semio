import os, sys, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa
OV = os.path.join(pfa.SCR, 'overlay')
import shutil
shutil.rmtree(OV, ignore_errors=True)
for tag in sys.argv[1:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    if not sp.get('pre'):
        continue
    orig = open(os.path.join(pfa.T, 'orig', sp['orig']), encoding='utf-8').read()
    dst = os.path.join(OV, sp['file'])
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    open(dst, 'w', encoding='utf-8').write(sp['pre'](orig))
    print('overlay', sp['file'][-70:])
