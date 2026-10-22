import re,sys,json
# usage: phases.py <label.err> [<label.cpu.jsonl>]  -> phase timeline (+cpu seconds per phase when sampled)
rows=[];last=None
for line in open(sys.argv[1],errors="replace"):
    try:t=float(line[:8])
    except ValueError:continue
    rest=line[9:]
    m=re.search(r'"command":"([^"]+)"(?:,"stage":"[^"]*")?(?:,"activity":\{"resource":"([^"]+)")?',rest)
    if rest.startswith("[script] {") and m:
        res=re.sub(r":\d+(:\d+)?$","",m[2] or "")
        res=re.sub(r"^build:graph-phase:([a-z]+).*",r"graph-phase:\1",res)
        key=m[1]+" "+res
        if key!=last:rows.append((t,key));last=key
    elif rest.startswith("[graph-phase]") or rest.startswith("[graph") or rest.startswith("[capacity] admitted") or rest.startswith("graph:") or rest.startswith("error:"):
        rows.append((t,rest.strip()[:240]))
cpu=[]
if len(sys.argv)>2:
    cpu=[json.loads(l) for l in open(sys.argv[2])]
def cpuat(t):
    best=0
    for r in cpu:
        if r["t"]<=t:best=r["cpu"]
    return best
end=rows[-1][0] if rows else 0
for i,(t,k) in enumerate(rows):
    nt=rows[i+1][0] if i+1<len(rows) else end
    extra=" cpu+%.1fs"%(cpuat(nt)-cpuat(t)) if cpu else ""
    print("%7.1f +%6.1fs%s %s"%(t,nt-t,extra,k))
if cpu:print("peak tree RSS MB",max(r["rssMB"] for r in cpu),"total cpu",cpu[-1]["cpu"])
