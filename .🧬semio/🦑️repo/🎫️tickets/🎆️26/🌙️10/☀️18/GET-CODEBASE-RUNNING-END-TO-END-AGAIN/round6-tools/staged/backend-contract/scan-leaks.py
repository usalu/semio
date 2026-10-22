#!/usr/bin/env python3
"""Lists every literal naming a backend in neutral build code after the staged edits (wire.EDITS) are applied in memory. Excludes adapters, tests and the new modules. Usage: scan-leaks.py [--lines]"""
import os,re,sys
from collections import OrderedDict
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import wire,names
B=names.R+'/'+names.B
groups=OrderedDict()
for rel,fn in wire.EDITS:groups.setdefault(rel,[]).append(fn)
edited={}
for rel,fns in groups.items():
    t=open(names.R+'/'+rel,encoding='utf8').read()
    for fn in fns:t=fn(t)
    edited[names.R+'/'+rel]=t
pat=re.compile(r"""(["'`])(nx|bazel)\1|\b(?:tool|selected\.tool)\s*[!=]==\s*["'](nx|bazel)["']|"nx"\|"bazel"|\["nx", *"bazel"\]""")
OWN=(names.T['TOOLS']+'/'+names.T['BAZEL'],names.T['TOOLS']+'/'+names.T['NX'],names.T['TOOLS']+'/'+names.T['SEMIO'],names.T['BACKENDS'],names.T['CONF'],names.T['TASKMODEL'])
rows=[]
for dp,dn,fn in os.walk(B):
    dn[:]=[d for d in dn if d!='node_modules' and not d.startswith('.')]
    rel=dp[len(B)+1:]
    if '🧪️tests' in rel.split('/') or any(rel==o or rel.startswith(o+'/') for o in OWN):continue
    for f in fn:
        if f.startswith('.'):continue
        if not f.endswith(('.ts','.mjs','.json','.rs','.py','.mts','.go')):continue
        p=os.path.join(dp,f)
        t=edited.get(p) or open(p,encoding='utf8',errors='ignore').read()
        for i,line in enumerate(t.split('\n'),1):
            for m in pat.finditer(line):rows.append((p[len(B)+1:],i,line[max(0,m.start()-70):m.end()+60].strip()))
c=__import__('collections').Counter(r[0] for r in rows)
for f,n in c.most_common():print(n,f)
print(len(rows),'hits',len(c),'files')
if '--lines' in sys.argv:
    for f,i,t in rows:print(f"{f}:{i}: {t}")
