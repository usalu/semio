import sys,re,collections
log=sys.argv[1];lo=int(sys.argv[2]);hi=int(sys.argv[3])
pat=re.compile(r'"resource":"([^"]+)","mode":"(\w+)","elapsedMs":([0-9.]+)')
def fam(r):
    r=re.sub(r':\d+$','',r)
    if r.startswith('dependencies:store-ownership'): return 'ownership'
    return re.sub(r'/Users/[^ ]*','<path>',r)
blocks=[]  # (fam, ln0, ln1, first_el, last_el)
cur=None
for ln,line in enumerate(open(log,errors='replace'),1):
    if ln<lo: continue
    if ln>hi: break
    if not line.startswith('[script] {'): continue
    m=pat.search(line)
    if not m: continue
    r,mode,el=m.group(1),m.group(2),float(m.group(3));f=fam(r)
    if r.startswith('lease-queue') or f.startswith('native-fixed') or f.startswith('resource-dir'): continue
    ex = mode=='exclusive' and f in('ownership','dependencies:retire-obsolete','dependencies:retired-obsolete-generations','dependencies:generation-verify','dependencies:workspace-edges')
    if cur is None or cur['f']!=f:
        cur={'f':f,'ln0':ln,'ln1':ln,'first':el if ex else None,'last':el if ex else None,'n':0};blocks.append(cur)
    cur['ln1']=ln;cur['n']+=1
    if ex:
        if cur['first'] is None: cur['first']=el
        cur['last']=el
# derive phase times
last_known=0.0
rows=[]
for i,b in enumerate(blocks):
    nxt=None
    for c in blocks[i+1:]:
        if c['first'] is not None: nxt=c['first'];break
    start=b['first'] if b['first'] is not None else last_known
    end=b['last'] if b['last'] is not None else (nxt if nxt is not None else last_known)
    if b['last'] is not None: last_known=b['last']
    rows.append((b['f'],b['ln0'],b['n'],start/1000,end/1000,b['first'] is not None))
agg=collections.defaultdict(lambda:[0,0.0])
# gap-based attribution: time between last known and first known of next
prev_end=0.0
for i,b in enumerate(blocks):
    if b['first'] is not None:
        dur=(b['last']-b['first'])/1000
        agg[b['f']+' (inside)'][0]+=1;agg[b['f']+' (inside)'][1]+=dur
        prev_end=b['last']/1000
        pending=[]
    else:
        pass
# attribute unknown-elapsed blocks via gap between neighbours
i=0
while i<len(blocks):
    if blocks[i]['first'] is None:
        j=i
        while j<len(blocks) and blocks[j]['first'] is None: j+=1
        a=None
        for k in range(i-1,-1,-1):
            if blocks[k]['last'] is not None: a=blocks[k]['last']/1000;break
        b2=blocks[j]['first']/1000 if j<len(blocks) else None
        gap=(b2-a) if (a is not None and b2 is not None) else 0
        names='+'.join(sorted(set(x['f'] for x in blocks[i:j])))
        agg['GAP['+names+']'][0]+=1;agg['GAP['+names+']'][1]+=gap
        i=j
    else: i+=1
tot=0
for k,v in sorted(agg.items(),key=lambda kv:-kv[1][1]):
    print(f"{v[1]:9.1f}s  n={v[0]:3d}  {k}")
