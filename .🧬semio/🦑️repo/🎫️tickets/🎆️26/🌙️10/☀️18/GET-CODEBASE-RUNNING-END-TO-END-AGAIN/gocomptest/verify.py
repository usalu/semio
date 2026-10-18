import json,glob,collections,re
d=json.load(open('decls.json'));g=json.load(open('assignment.json'))
exec(open('table.py').read())
L=open('orig.go.txt').read().split('\n')
def isreg(l): return l.startswith('// #region') or l.startswith('// #endregion')
cache={}
bad=0
for x in d:
    body=L[x['start']-1:x['end']]
    j=0
    while j<len(body) and body[j].startswith('//'): j+=1
    head=[l for k,l in enumerate(body[:j]) if not isreg(l) and not (k>0 and body[k-1].startswith('// #region'))]
    txt='\n'.join(head+body[j:])
    s=g[str(x['idx'])]
    e=[e for e,ss,_,_ in T if ss==s][0]
    p=f"out/{e}{FE}{s}/\U0001F439️.go"
    if p not in cache: cache[p]=open(p).read()
    if cache[p].count(txt)!=1: bad+=1;print('COUNT',cache[p].count(txt),x['name'])
print('decls',len(d),'bad',bad)
# multiset of non-blank non-region lines
orig=collections.Counter(l for l in L if l.strip() and not isreg(l))
new=collections.Counter()
for p,c in cache.items():
    for l in c.split('\n'):
        if l.strip() and not isreg(l): new[l]+=1
import_lines=set(l for l in L[L.index('import ('):L.index(')')+1])
miss=orig-new; extra=new-orig
miss={k:v for k,v in miss.items() if k not in import_lines}
extra={k:v for k,v in extra.items() if k not in import_lines and not k.startswith('// 2025-2026') and not k.startswith('package client')}
print('missing',len(miss),'extra',len(extra))
for k,v in list(miss.items())[:20]: print('MISS',v,k[:100])
for k,v in list(extra.items())[:40]: print('EXTRA',v,k[:100])
