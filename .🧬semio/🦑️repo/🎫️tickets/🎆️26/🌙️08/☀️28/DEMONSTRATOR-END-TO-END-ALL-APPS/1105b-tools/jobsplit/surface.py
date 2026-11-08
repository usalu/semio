import re,glob,collections,os
R='/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation'
files=[f for f in glob.glob(R+'/**/🦀️.rs',recursive=True) if '🧪️' not in f]
cx=collections.Counter();crate=collections.Counter();par=collections.Counter();size=0
for f in files:
    t=open(f,encoding='utf8').read();size+=len(t.encode())
    for x in re.findall(r'\bcx\.(\w+)',t): cx[x]+=1
    for blk in re.findall(r'use crate::\{?([^;]*?)\}?;',t):
        for n in re.split(r'[,\s{}]+',blk):
            if n and n[0].isupper() or (n and n[0].islower() and '_' in n): crate[n]+=1
    for x in re.findall(r'crate::(\w+)',t): crate[x]+=1
    for x in re.findall(r'\bparent\.(\w+)',t): par[x]+=1
print('bytes',size,'files',len(files))
print('cx methods',len(cx),dict(cx))
print('crate items',len(crate),sorted(crate))
print('parent methods',dict(par))
