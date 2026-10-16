import os, sys, importlib, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa

def active(t):
    n = 0
    for l in t.split('\n'):
        s = l.strip()
        if s and not s.startswith('//'):
            n += 1
    return n

rows = []
for tag in ['bim', 'process', 'raster', 'vdi', 'en1999', 'en1993']:
    sp = importlib.import_module('specs.' + tag).SPEC
    orig = open(os.path.join(pfa.T, 'orig', sp['orig']), encoding='utf-8').read()
    base = os.path.join(pfa.REPO, os.path.dirname(sp['file']))
    par = open(os.path.join(base, '🦀️.rs'), encoding='utf-8').read()
    print('## %s: original %d raw / %d active' % (tag, orig.count('\n'), active(orig)))
    print('| parent | %d | %d |' % (par.count('\n'), active(par)))
    for c in sp['children']:
        t = open(os.path.join(base, c['folder'], '🦀️.rs'), encoding='utf-8').read()
        print('| %s | %d | %d |' % (c['folder'], t.count('\n'), active(t)))
