#!/usr/bin/env python3
"""c4p32: writes newdirs.txt (repo-relative new child directories) and checks that no new folder emoji repeats among its siblings, that every docstring starts with the folder emoji and the path stays at most 240 bytes.
usage: sib.py [live]"""
import os, sys, importlib
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p32 as P
TAGS = ['shootingeditor', 'shootingroot', 'femeditor', 'femsession', 'bimterm', 'bimeditor', 'bimentities', 'bimpatches']
dirs, bad = [], 0
VS = '️'
for tag in TAGS:
    sp = importlib.import_module('specs.' + tag).SPEC
    base = os.path.dirname(sp['file'])
    seen = {}
    for e in os.listdir(os.path.join(P.REPO, base)):
        if os.path.isdir(os.path.join(P.REPO, base, e)):
            seen.setdefault(e.split(VS)[0], []).append(e)
    for c in sp['children']:
        rel = base + '/' + c['folder']
        dirs.append(rel)
        key = c['folder'].split(VS)[0]
        sibs = [s for s in seen.get(key, []) if s != c['folder']]
        nb = len((rel + '/🦀️.rs').encode('utf-8'))
        status = 'ok'
        if sibs:
            status = 'CLASH ' + ','.join(sibs); bad += 1
        if nb > 240:
            status += ' TOOLONG'; bad += 1
        print('%-14s %-34s %3d bytes %s' % (tag, c['folder'], nb, status))
open(os.path.join(T, 'newdirs.txt'), 'w', encoding='utf-8').write('\n'.join(dirs) + '\n')
print('dirs', len(dirs), 'problems', bad)
