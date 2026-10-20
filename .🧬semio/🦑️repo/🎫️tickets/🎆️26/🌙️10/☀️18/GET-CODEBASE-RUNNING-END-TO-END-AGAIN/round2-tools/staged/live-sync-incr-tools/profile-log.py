import sys,re,json,collections
log=sys.argv[1]
fam=collections.OrderedDict()
pat=re.compile(r'"resource":"([^"]+)","mode":"(\w+)","elapsedMs":([0-9.]+)')
for line in open(log,errors='replace'):
    if not line.startswith('[script] {'): 
        continue
    m=pat.search(line)
    if not m: continue
    r,mode,el=m.group(1),m.group(2),float(m.group(3))
    if r.startswith('lease-queue') : k='lease-queue'
    else:
        k=re.sub(r':\d+$','',r)
        k=re.sub(r'/Users/[^ ]*','<path>',k)
    d=fam.setdefault(k,{'n':0,'first':el,'last':el,'max':0,'mode':mode,'maxn':0})
    d['n']+=1; d['last']=el; d['max']=max(d['max'],el)
    mm=re.search(r':(\d+)$',r)
    if mm: d['maxn']=max(d['maxn'],int(mm.group(1)))
print(f"{'family':72} {'events':>8} {'first_s':>9} {'last_s':>9} {'maxcounter':>11} mode")
for k,d in fam.items():
    print(f"{k[:72]:72} {d['n']:8d} {d['first']/1000:9.1f} {d['last']/1000:9.1f} {d['maxn']:11d} {d['mode']}")
