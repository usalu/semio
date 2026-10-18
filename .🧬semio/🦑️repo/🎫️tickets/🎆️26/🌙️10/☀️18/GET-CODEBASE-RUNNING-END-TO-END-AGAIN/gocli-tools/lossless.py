import sys,re,collections,glob,os
def body(path):
    L=open(path,encoding='utf-8').read().split('\n')
    i=0
    # skip through package line
    while not L[i].startswith('package '): i+=1
    i+=1
    while i<len(L) and L[i].strip()=='' : i+=1
    if i<len(L) and L[i].startswith('import ('):
        while L[i]!=')': i+=1
        i+=1
    elif i<len(L) and L[i].startswith('import '):
        i+=1
    out=[]
    for l in L[i:]:
        t=l.strip()
        if not t or re.match(r'//\s*#(end)?region',t): continue
        out.append(t)
    return out
orig=collections.Counter()
for p in sys.argv[1].split(','): orig.update(body(p))
new=collections.Counter()
for p in glob.glob(sys.argv[2]+'/*'): new.update(body(p))
d1=orig-new; d2=new-orig
print('missing',sum(d1.values()),'extra',sum(d2.values()))
for k,v in list(d1.items())[:20]: print('MISSING',v,k)
for k,v in list(d2.items())[:20]: print('EXTRA',v,k)
