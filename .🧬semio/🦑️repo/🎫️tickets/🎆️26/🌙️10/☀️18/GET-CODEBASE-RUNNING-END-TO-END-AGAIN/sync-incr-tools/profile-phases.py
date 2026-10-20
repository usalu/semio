import sys,re,collections
log=sys.argv[1]
pat=re.compile(r'"resource":"([^"]+)","mode":"(\w+)","elapsedMs":([0-9.]+)')
def fam(r):
    if r.startswith('lease-queue'): return 'lease-queue'
    r=re.sub(r':\d+$','',r)
    return re.sub(r'/Users/[^ ]*','<path>',r)
def group(f):
    if f.startswith('dependencies:store-ownership'): return 'ownership'
    return f
cur=None;start=0;n=0;maxel=0;ln0=0;first=None;counts=collections.Counter();maxc=collections.Counter()
rows=[]
def flush(ln):
    if cur is not None: rows.append((ln0,ln-1,cur,n,maxel,dict(counts),dict(maxc)))
for ln,line in enumerate(open(log,errors='replace'),1):
    if not line.startswith('[script] {'): continue
    m=pat.search(line)
    if not m: continue
    r=m.group(1);f=fam(r);g=group(f);el=float(m.group(3))
    if g=='lease-queue':continue
    if g!=cur:
        flush(ln);cur=g;ln0=ln;n=0;maxel=0;counts=collections.Counter();maxc=collections.Counter()
    n+=1;maxel=max(maxel,el);counts[f.split(':')[-1] if g=='ownership' else f]+=1
    mm=re.search(r':(\d+)$',r)
    if mm: maxc[f.split(':')[-1] if g=='ownership' else f]=max(maxc[f.split(':')[-1] if g=='ownership' else f],int(mm.group(1)))
flush(ln+1)
for ln0,ln1,g,n,maxel,c,mc in rows:
    print(f"{ln0:7d}-{ln1:7d} {g:40} events={n:6d} maxelapsed={maxel/1000:8.1f}s counters={mc}")
