import importlib, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p48 as P
tags = 'reg gol rsc fsc crd cbs'.split()
dirs = []
names = []
emo = {}
for t in tags:
    sp = importlib.import_module('specs.' + t).SPEC
    base = os.path.join(P.REPO, os.path.dirname(sp['file']))
    sib = {n.split('️')[0] for n in os.listdir(base) if '️' in n}
    seen = set()
    for c in sp['children']:
        d = os.path.join(base, c['folder'])
        dirs.append(d)
        names.append(c['folder'])
        e = c['folder'].split('️')[0]
        if e in sib: print('SIBLING CLASH', t, c['folder'])
        if e in seen: print('DUP', t, c['folder'])
        seen.add(e)
        emo.setdefault(e, 0)
        rel = os.path.relpath(os.path.join(d, '🦀️.rs'), P.REPO)
        if len(rel.encode()) > 240: print('LONG', len(rel.encode()), rel)
open('newdirs.txt', 'w', encoding='utf-8').write('\n'.join(os.path.relpath(d, P.REPO) for d in dirs) + '\n')
open('newnames.txt', 'w', encoding='utf-8').write('\n'.join(names) + '\n')
counts = {}
for line in open('/Users/ueli/.cache/c4p48/emojis.txt', encoding='utf-8'):
    k, v = line.split()
    counts[k] = int(v)
for e in emo:
    if counts.get(e, 0) < 2: print('EMOJI RARE', e, counts.get(e, 0))
print('dirs', len(dirs), 'emoji', len(emo))
