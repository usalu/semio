import sys,re
log=sys.argv[1]
pat=re.compile(r'"resource":"([^"]+)","mode":"(\w+)","elapsedMs":([0-9.]+)')
prev=None;start=None;cnt=0;ln=0;lastel=0
def fam(r):
    if r.startswith('lease-queue'): return 'lease-queue'
    r=re.sub(r':\d+$','',r)
    return re.sub(r'/Users/[^ ]*','<path>',r)
other=[]
for ln,line in enumerate(open(log,errors='replace'),1):
    if line.startswith('==='): print(ln,line.strip()[:160]); continue
    if not line.startswith('[script] {'):
        if not line.startswith('[script] bun:dependency-storage'):
            other.append((ln,line.strip()[:240]))
        continue
    m=pat.search(line)
    if not m: continue
    f=fam(m.group(1))
    if f!=prev:
        if prev is not None: print(f"{start:7d}-{ln-1:7d} n={cnt:6d} el_last={lastel/1000:8.1f}s {prev}")
        prev=f;start=ln;cnt=0
    cnt+=1;lastel=float(m.group(3))
print(f"{start:7d}-{ln:7d} n={cnt:6d} el_last={lastel/1000:8.1f}s {prev}")
print('--- non-activity lines',len(other))
for o in other[:60]: print(o)
