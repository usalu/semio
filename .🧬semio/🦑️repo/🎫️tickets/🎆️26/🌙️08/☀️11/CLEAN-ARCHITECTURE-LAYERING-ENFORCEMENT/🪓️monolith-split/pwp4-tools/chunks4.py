# 🧵️ Stitch proof: every top-level chunk of the pre-cut files appears exactly once among the chunks of the residuals and pieces (visibility prefixes and path literals normalised); the surplus is exactly the mount lines, the glob imports and the piece docstrings.
import os, re, sys, collections
from seg import *
from spec4 import *
def norm(c, file):
    txt = '\n'.join(l.strip() for l in c.lines if l.strip())
    txt = re.sub(r'(?m)^(pub\(in super::super\)|pub\(super\)) ', '', txt)
    txt = re.sub(r'(?<![\w])pub\(in super::super\) ', '', txt)
    txt = re.sub(r'(?<![\w])pub\(super\) ', '', txt)
    d = os.path.dirname(file)
    def fix(m): return m.group(1) + '"' + os.path.normpath(os.path.join(d, m.group(2))) + '"'
    txt = PATH_ATTR.sub(fix, txt); txt = INCL.sub(fix, txt)
    return txt
before = collections.Counter(); after = collections.Counter(); surplus = collections.Counter()
for src, pieces, rules in CUTS:
    fb = SCR + '/precut/' + src
    chunks, R = segment(path=fb)
    for c in chunks:
        if c.kind in ('filler',) or c.kind.startswith('region') and False: continue
        if c.kind == 'filler' and not any(l.strip() for l in c.lines): continue
        before[norm(c, os.path.normpath(PLUG + '/' + src))] += 1
    files = [src] + [p.path for p in pieces]
    for f in files:
        full = os.path.normpath(PLUG + '/' + f)
        if not os.path.exists(full): continue
        ch, R = segment(path=full)
        for c in ch:
            if c.kind == 'filler' and not any(l.strip() for l in c.lines): continue
            after[norm(c, full)] += 1
lost = before - after; gained = after - before
print('chunks before', sum(before.values()), 'after', sum(after.values()), '| lost', sum(lost.values()), '| gained', sum(gained.values()))
kinds = collections.Counter()
for k, v in gained.items():
    if k.startswith('use super::*;'): kinds['use super::*'] += v
    elif k.startswith('//!'): kinds['docstring'] += v
    elif re.match(r'^(#\[cfg[^\n]*\n)?#\[path = "[^"]+"\]\nmod \w+;\n(pub )?use \w+::\*;$', k): kinds['mount block'] += v
    elif re.match(r'^(#\[cfg[^\n]*\n)?#\[path', k): kinds['mount'] += v
    elif re.match(r'^(pub )?use \w+::\*;$', k): kinds['reexport'] += v
    else: kinds['OTHER'] += v; print('  OTHER gained:', k[:140].replace('\n', ' | '))
print('gained by kind:', dict(kinds))
for k, v in list(lost.items())[:12]: print('  LOST:', k[:160].replace('\n', ' | '))
