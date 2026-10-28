#!/usr/bin/env python3
"""list.py <root> [after-substring]: lists the test files (`*.ts` (`*.tsx` too) that import bun:test or node:test or vitest; with ALL=1 anywhere, else under a `🧪️tests` path part) below <root> in codepoint path order; with <after-substring> only those sorting after the last file whose path contains it."""
import os,re,sys
root=sys.argv[1];after=sys.argv[2] if len(sys.argv)>2 else None
R=re.compile(r'''from\s*["'](bun:test|node:test|vitest)["']''')
fs=[]
for d,ds,fn in os.walk(root):
    ds[:]=sorted(x for x in ds if x not in ('node_modules','.git','target','dist'))
    if not os.environ.get('ALL') and '🧪️tests' not in d.split(os.sep): continue
    for f in fn:
        if not f.endswith(('.ts','.tsx')): continue
        p=os.path.join(d,f)
        with open(p,errors='replace') as h:
            if R.search(h.read(200000)): fs.append(p)
fs.sort()
if after:
    k=[i for i,f in enumerate(fs) if after in f]
    fs=fs[k[-1]+1:] if k else fs
print('\n'.join(fs))
